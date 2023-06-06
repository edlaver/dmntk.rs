/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2023 Dariusz Depta, Engos Software
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2018-2023 Dariusz Depta, Engos Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! # Container for DMN models

use crate::errors::*;
use dmntk_common::{color_blue, color_green, color_red, color_reset, to_rdnn, ColorMode, Result};
use dmntk_feel::context::FeelContext;
use dmntk_feel::value_null;
use dmntk_feel::values::Value;
use dmntk_model::{Definitions, NamedElement};
use dmntk_model_evaluator::ModelEvaluator;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use walkdir::WalkDir;

const ERROR_TAG: &str = "error";

/// Structure representing the container for DMN models.
pub struct Workspace {
  ///
  evaluator: Arc<ModelEvaluator>,
  /// Namespaces indexed by RDNN.
  namespaces: HashMap<String, String>,
}

impl Workspace {
  /// Creates a new [Workspace] and loads DMN models from specified directory (recursive).
  pub fn new(opt_dir: Option<PathBuf>, color_mode: ColorMode, _verbose: bool) -> Result<Self> {
    if let Some(dir) = opt_dir {
      let loaded_definitions = Self::load(dir, color_mode);
      return match ModelEvaluator::new(&loaded_definitions) {
        Ok(evaluator) => {
          let mut namespaces: HashMap<String, String> = HashMap::new();
          let mut loaded_namespaces: HashMap<String, String> = HashMap::new();
          for definitions in loaded_definitions {
            let name = definitions.name();
            let namespace = definitions.namespace();
            if let Some(existing_name) = loaded_namespaces.get(namespace) {
              return Err(err_duplicated_namespace(namespace, name, existing_name));
            }
            loaded_namespaces.insert(namespace.to_string(), name.to_string());
            let Some(rdnn) = to_rdnn(&namespace) else {
              return Err(err_invalid_namespace(&namespace));
            };
            namespaces.insert(rdnn, namespace.to_string());
          }
          Ok(Self { evaluator, namespaces })
        }
        Err(reason) => Err(reason),
      };
    }
    Err(err_x("y"))
  }

  /// Evaluates invocable identified by its name in model namespace.
  pub fn evaluate_invocable_by_name(&self, rdnn: &str, invocable_name: &str, input_data: &FeelContext) -> Result<Value> {
    if let Some(namespace) = self.namespaces.get(rdnn) {
      Ok(self.evaluator.evaluate_invocable_by_name(namespace, invocable_name, input_data))
    } else {
      Err(err_evaluator_rdnn_not_found(rdnn))
    }
  }

  /// Evaluates invocable identified by its identifier in model namespace.
  pub fn evaluate_invocable_by_id(&self, _rdnn: String, _invocable_id: String, _input_data: FeelContext) -> Result<Value> {
    Ok(value_null!("not implemented yet"))
  }

  /// Loads DMN models from specified directory (recursive).
  fn load(dir: PathBuf, color_mode: ColorMode) -> Vec<Definitions> {
    let color_blue = color_blue!(color_mode);
    let color_green = color_green!(color_mode);
    let color_red = color_red!(color_mode);
    let color_reset = color_reset!(color_mode);
    let mut file_count = 0_usize;
    let mut loaded_count = 0_usize;
    let mut failed_count = 0_usize;
    let mut loaded_definitions = vec![];
    for file in &search_models_recursive(&dir) {
      file_count += 1;
      match fs::read_to_string(file) {
        Ok(xml) => match dmntk_model::parse(&xml) {
          Ok(definitions) => {
            loaded_definitions.push(definitions);
            loaded_count += 1;
          }
          Err(reason) => {
            eprintln!("[{1}{ERROR_TAG}{0}][{2}{3}{0}] {1}{4}{0}", color_reset, color_red, color_blue, file.display(), reason);
            failed_count += 1;
          }
        },
        Err(reason) => {
          eprintln!("[{1}{ERROR_TAG}{0}][{2}{3}{0}] {1}{4}{0}", color_reset, color_red, color_blue, file.display(), reason);
          failed_count += 1;
        }
      }
    }
    let color = if file_count > 0 { &color_green } else { &color_red };
    println!("{1}Found {file_count} {2}.{0}", color_reset, color, plural("model", file_count),);
    if loaded_count > 0 {
      println!("{1}Loaded {loaded_count} {2}.{0}", color_reset, color_green, plural("model", loaded_count));
    }
    if failed_count > 0 {
      println!("{1}Failed to load {failed_count} {2}.{0}", color_reset, color_red, plural("model", failed_count));
    }
    loaded_definitions
  }
}

/// Searches all subdirectories starting from specified directory
/// and searches for files that have `.dmn` extension.
fn search_models_recursive(dir: &PathBuf) -> Vec<PathBuf> {
  let mut paths = vec![];
  for entry in WalkDir::new(dir).into_iter().filter_map(|entry| entry.ok()) {
    let path = entry.path();
    if path.is_file() {
      if let Some(extension) = path.extension() {
        if extension == "dmn" {
          paths.push(entry.path().into());
        }
      }
    }
  }
  paths
}

/// Utility function to make a noun plural.
fn plural(noun: &str, number: usize) -> String {
  if number == 1 {
    noun.to_string()
  } else {
    format!("{}s", noun)
  }
}

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
use dmntk_common::{color_blue, color_green, color_magenta, color_red, color_reset, to_rdnn, ColorMode, Result};
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_model::model::{Definitions, NamedElement};
use dmntk_model_evaluator::ModelEvaluator;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use urlencoding::encode;
use walkdir::WalkDir;

const ERROR_TAG: &str = "error";

/// Type alias defining a map of definitions indexed by model name.
type DefinitionsByName = HashMap<String, Definitions>;

/// Type alias defining a map of evaluators indexed by model name.
type EvaluatorsByName = HashMap<String, Arc<ModelEvaluator>>;

/// Type alias defining a map of file names indexed by model name.
type FileByName = HashMap<String, PathBuf>;

/// Structure representing the container for DMN models.
pub struct Workspace {
  /// Map of [Definitions] indexed by _RDNN_.
  definitions: HashMap<String, DefinitionsByName>,
  /// Map of [ModelEvaluators](ModelEvaluator) indexed by [Definitions]' _RDNN_.
  evaluators: HashMap<String, EvaluatorsByName>,
  /// Map of file names indexed by _RDNN_.
  files: HashMap<String, FileByName>,
}

impl Workspace {
  /// Creates a new [Workspace] and loads DMN models from specified directory (recursive).
  pub fn new(opt_dir: Option<PathBuf>, color_mode: ColorMode, verbose: bool) -> Self {
    let mut workspace = Self {
      definitions: HashMap::new(),
      evaluators: HashMap::new(),
      files: HashMap::new(),
    };
    if let Some(dir) = opt_dir {
      workspace.load(dir, color_mode);
      workspace.deploy(color_mode);
    };
    if verbose {
      workspace.display_deployed(color_mode);
    }
    workspace
  }

  /// Evaluates invocable deployed in workspace.
  pub fn evaluate_invocable(&self, model_rdnn: &str, model_name: &str, invocable_name: &str, input_data: &FeelContext) -> Result<Value> {
    if let Some(evaluators_by_name) = self.evaluators.get(model_rdnn) {
      if let Some(model_evaluator) = evaluators_by_name.get(model_name) {
        Ok(model_evaluator.evaluate_invocable(invocable_name, input_data))
      } else {
        Err(err_evaluator_name_not_found(model_rdnn, model_name))
      }
    } else {
      Err(err_evaluator_rdnn_not_found(model_rdnn))
    }
  }

  /// Loads and deploys DMN models placed in specified directory (recursive).
  fn load(&mut self, dir: PathBuf, color_mode: ColorMode) {
    let color_blue = color_blue!(color_mode);
    let color_green = color_green!(color_mode);
    let color_red = color_red!(color_mode);
    let color_reset = color_reset!(color_mode);
    let mut file_count = 0_usize;
    let mut loaded_count = 0_usize;
    let mut failed_count = 0_usize;
    for file in &search_models_recursive(&dir) {
      file_count += 1;
      match fs::read_to_string(file) {
        Ok(xml) => match dmntk_model::parse(&xml) {
          Ok(definitions) => match self.add(definitions) {
            Ok((rdnn, _, name)) => {
              self
                .files
                .entry(rdnn.clone())
                .and_modify(|files_by_name| {
                  files_by_name.insert(name.clone(), file.to_owned());
                })
                .or_insert({
                  let mut files_by_name = HashMap::new();
                  files_by_name.insert(name.clone(), file.to_owned());
                  files_by_name
                });
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
  }

  /// Adds new definitions to workspace.
  fn add(&mut self, definitions: Definitions) -> Result<(String, String, String)> {
    let namespace = definitions.namespace().to_string();
    // create the rdnn from definitions namespace
    let Some(rdnn) = to_rdnn(&namespace) else {
      return Err(err_invalid_namespace(&namespace));
    };
    // get the name from definitions (always provided)
    let name = definitions.name().to_string();
    // check if the specified name already exists in the requested namespace
    if let Some(entry) = self.definitions.get(&rdnn) {
      if entry.contains_key(&name) {
        return Err(err_definitions_already_exists(&rdnn, &name));
      }
    }
    // save definitions by namespace and name
    self
      .definitions
      .entry(rdnn.clone())
      .and_modify(|definitions_by_name| {
        definitions_by_name.insert(name.clone(), definitions.clone());
      })
      .or_insert({
        let mut definitions_by_name = HashMap::new();
        definitions_by_name.insert(name.clone(), definitions);
        definitions_by_name
      });
    Ok((rdnn, namespace, name))
  }

  /// Creates evaluators for all definitions in workspace.
  fn deploy(&mut self, color_mode: ColorMode) {
    let color_blue = color_blue!(color_mode);
    let color_green = color_green!(color_mode);
    let color_red = color_red!(color_mode);
    let color_reset = color_reset!(color_mode);
    let mut deployed_count = 0;
    let mut failed_count = 0;
    for (rdnn, definitions_by_name) in &self.definitions {
      for (name, definitions) in definitions_by_name {
        match ModelEvaluator::new(definitions) {
          Ok(model_evaluator) => {
            self
              .evaluators
              .entry(rdnn.clone())
              .and_modify(|evaluators_by_name| {
                evaluators_by_name.insert(name.clone(), Arc::clone(&model_evaluator));
              })
              .or_insert({
                let mut evaluators_by_name: EvaluatorsByName = HashMap::new();
                evaluators_by_name.insert(name.clone(), model_evaluator);
                evaluators_by_name
              });
            deployed_count += 1;
          }
          Err(reason) => {
            let file = self.files.get(rdnn).unwrap().get(name).unwrap();
            eprintln!("[{1}{ERROR_TAG}{0}][{2}{3}{0}] {1}{4}{0}", color_reset, color_red, color_blue, file.display(), reason);
            failed_count += 1;
          }
        }
      }
    }
    if deployed_count > 0 {
      println!("{1}Deployed {deployed_count} {2}.{0}", color_reset, color_green, plural("model", deployed_count));
    }
    if failed_count > 0 {
      println!("{1}Failed to deploy {failed_count} {2}.{0}", color_reset, color_red, plural("model", failed_count));
    }
  }

  /// Displays URLs of deployed invocables.
  fn display_deployed(&self, color_mode: ColorMode) {
    let color_green = color_green!(color_mode);
    let color_blue = color_blue!(color_mode);
    let color_magenta = color_magenta!(color_mode);
    let color_reset = color_reset!(color_mode);
    for (model_rdnn, evaluators) in &self.evaluators {
      for (model_name, model_evaluator) in evaluators {
        for invocable_name in model_evaluator.invocable_names() {
          let url = format!(
            "  {1}{4}{0}/{2}{5}{0}/{3}{6}{0}",
            color_reset,
            color_blue,
            color_green,
            color_magenta,
            encode(model_rdnn),
            encode(model_name),
            encode(invocable_name)
          );
          println!("{}", url);
        }
      }
    }
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

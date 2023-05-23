/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2023 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2023 Dariusz Depta Engos Software
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
use dmntk_common::{to_rdnn, Result};
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_model::model::{Definitions, NamedElement};
use dmntk_model_evaluator::ModelEvaluator;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use walkdir::WalkDir;

const ERROR_TAG: &str = "[error]";

/// Type alias defining a map of definitions indexed by its name.
type DefinitionsByName = HashMap<String, Definitions>;

/// Type alias defining a map of evaluators indexed by its name.
type EvaluatorsByName = HashMap<String, Arc<ModelEvaluator>>;

/// Structure representing the container for DMN models.
pub struct Workspace {
  /// Map of [Definitions] indexed by **rdnn** created from [Definitions]' _namespace_ attribute.
  definitions: HashMap<String, DefinitionsByName>,
  /// Map of [ModelEvaluators](ModelEvaluator) indexed by [Definitions]' _name_ attribute.
  evaluators: HashMap<String, EvaluatorsByName>,
}

impl Workspace {
  /// Creates a new [Workspace] and loads DMN models from specified directory (recursive).
  pub fn new(opt_dir: Option<PathBuf>) -> Self {
    let mut workspace = Self {
      definitions: HashMap::new(),
      evaluators: HashMap::new(),
    };
    if let Some(dir) = opt_dir {
      workspace.load(dir);
      workspace.deploy();
    };
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
  fn load(&mut self, dir: PathBuf) {
    let mut file_count = 0_usize;
    let mut loaded_count = 0_usize;
    let mut failed_count = 0_usize;
    for file in &search_models_recursive(dir) {
      file_count += 1;
      match fs::read_to_string(file) {
        Ok(xml) => match dmntk_model::parse(&xml) {
          Ok(definitions) => match self.add(definitions) {
            Ok(_) => {
              loaded_count += 1;
            }
            Err(reason) => {
              eprintln!("{ERROR_TAG}[{}] {}", file.display(), reason);
              failed_count += 1;
            }
          },
          Err(reason) => {
            eprintln!("{ERROR_TAG}[{}] {}", file.display(), reason);
            failed_count += 1;
          }
        },
        Err(reason) => {
          eprintln!("{ERROR_TAG}[{}] {}", file.display(), reason);
          failed_count += 1;
        }
      }
    }
    println!("Found {file_count} {}.", plural("model", file_count));
    if loaded_count > 0 {
      println!("Loaded {loaded_count} {}.", plural("model", loaded_count));
    }
    if failed_count > 0 {
      println!("Failed to load {failed_count} {}.", plural("model", failed_count));
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
        // add definitions with specified name to existing namespace
        definitions_by_name.insert(name.clone(), definitions.clone());
      })
      .or_insert({
        // add definitions with specified name to namespace that will be created
        let mut definitions_by_name = HashMap::new();
        definitions_by_name.insert(name.clone(), definitions);
        definitions_by_name
      });
    Ok((rdnn, namespace, name))
  }

  /// Creates evaluators for all definitions in workspace.
  fn deploy(&mut self) {
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
            failed_count += 1;
            eprintln!("{ERROR_TAG}[{}/{}] {}", rdnn, name, reason);
          }
        }
      }
    }
    if deployed_count > 0 {
      println!("Deployed {deployed_count} {}.", plural("model", deployed_count));
    }
    if failed_count > 0 {
      println!("Failed to deploy {failed_count} {}.", plural("model", failed_count));
    }
  }
}

/// Searches all subdirectories starting from specified directory
/// and searches for files that have `.dmn` extension.
fn search_models_recursive(dir: PathBuf) -> Vec<PathBuf> {
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

/// Utility function to make plurals from noun.
fn plural(noun: &str, number: usize) -> String {
  if number == 1 {
    noun.to_string()
  } else {
    format!("{}s", noun)
  }
}

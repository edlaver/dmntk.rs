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
//!
//! Workspace has two _virtual_ states:
//! - `STASHING`: all model evaluators are deleted, model definitions can be freely modified,
//! - `DEPLOYED`: model evaluators are deployed, model definitions should remain unmodified.

use crate::errors::*;
use dmntk_common::{to_rdnn, Result};
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_model::model::{Definitions, NamedElement};
use dmntk_model_evaluator::ModelEvaluator;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;

/// Type alias defining a map of definitions indexed by its name.
type DefinitionsByName = HashMap<String, Arc<Definitions>>;

/// Type alias defining a map of evaluators indexed by its name.
type EvaluatorsByName = HashMap<String, Arc<ModelEvaluator>>;

/// Structure representing the container for DMN models.
pub struct Workspace {
  /// Map of [Definitions] indexed by [Definitions]' **namespace** attribute.
  definitions: HashMap<String, DefinitionsByName>,
  /// Map of [ModelEvaluators](ModelEvaluator) indexed by [Definitions]' **name** attribute.
  evaluators: HashMap<String, EvaluatorsByName>,
}

impl Workspace {
  /// Creates a new [Workspace] and loads DMN models from specified directory (recursive).
  pub fn new(opt_dir: Option<PathBuf>) -> Self {
    // create an empty workspace
    let mut workspace = Self {
      definitions: HashMap::new(),
      evaluators: HashMap::new(),
    };
    // load and deploy all DMN models (when optional directory specified)
    if let Some(dir) = opt_dir {
      let count = workspace.load_and_deploy_models(&dir);
      println!("Loaded {} file(s) from directory: {}", count, dir.to_string_lossy());
    }
    // workspace is now ready to use
    workspace
  }

  /// Deletes all definitions and model evaluators,
  /// and switches a workspace to state `STASHING`.
  pub fn clear(&mut self) {
    self.clear_definitions();
    self.clear_evaluators();
  }

  /// Adds a definition to workspace, deletes all model evaluators,
  /// and switches a workspace to state `STASHING`.
  pub fn add(&mut self, definitions: Definitions) -> Result<()> {
    // get the namespace from definitions (always provided)
    let Some(namespace) = to_rdnn(definitions.namespace()) else {
      return Err(err_invalid_namespace(definitions.namespace()));
    };
    // get the name from definitions (always provided)
    let name = definitions.name().to_string();
    // check if the specified name already exists in the requested namespace
    if let Some(entry) = self.definitions.get(&namespace) {
      if entry.contains_key(&name) {
        return Err(err_definitions_with_name_already_exists(&namespace, &name));
      }
    }
    // save definitions by namespace and name
    let definitions_arc = Arc::new(definitions);
    self
      .definitions
      .entry(namespace)
      .and_modify(|definitions_by_name| {
        // add definitions with specified name to existing namespace
        definitions_by_name.insert(name.clone(), Arc::clone(&definitions_arc));
      })
      .or_insert({
        // add definitions with specified name to namespace that will be created
        let mut definitions_by_name = HashMap::new();
        definitions_by_name.insert(name, Arc::clone(&definitions_arc));
        definitions_by_name
      });
    // delete all evaluators
    self.clear_evaluators();
    Ok(())
  }

  /// Removes a definition from workspace, deletes all model evaluators,
  /// switches a workspace to state `STASHING`.
  pub fn remove(&mut self, namespace: &str, name: &str) {
    if let Some(definitions_by_name) = self.definitions.get_mut(namespace) {
      definitions_by_name.remove(name);
    }
    self.clear_evaluators();
  }

  /// Replaces a definition in workspace, deletes all model evaluators,
  /// switches a workspace to state `STASHING`.
  pub fn replace(&mut self, definitions: Definitions) -> Result<()> {
    if let Some(namespace) = to_rdnn(definitions.namespace()) {
      self.remove(&namespace, definitions.name());
    }
    self.add(definitions)
  }

  /// Creates model evaluators for all definitions in workspace,
  /// switches a workspace to state `DEPLOYED`.
  pub fn deploy(&mut self) -> Result<()> {
    self.clear_evaluators();
    for (namespace, definitions_by_name) in &self.definitions {
      for (name, definitions) in definitions_by_name {
        match ModelEvaluator::new(definitions) {
          Ok(model_evaluator_arc) => {
            self
              .evaluators
              .entry(namespace.clone())
              .and_modify(|evaluators_by_name| {
                //
                evaluators_by_name.insert(name.clone(), Arc::clone(&model_evaluator_arc));
              })
              .or_insert({
                //
                let mut evaluators_by_name: EvaluatorsByName = HashMap::new();
                evaluators_by_name.insert(name.clone(), Arc::clone(&model_evaluator_arc));
                evaluators_by_name
              });
          }
          Err(reason) => return Err(reason),
        }
      }
    }
    Ok(())
  }

  /// Evaluates invocable (decision, business knowledge model or decision service) deployed in a workspace.
  pub fn evaluate_invocable(&self, model_namespace: &str, model_name: &str, invocable_name: &str, input_data: &FeelContext) -> Result<Value> {
    if let Some(evaluators_by_name) = self.evaluators.get(model_namespace) {
      if let Some(model_evaluator) = evaluators_by_name.get(model_name) {
        return Ok(model_evaluator.evaluate_invocable(invocable_name, input_data));
      }
    }
    Err(err_model_evaluator_is_not_deployed(model_name))
  }

  /// Utility function that deletes all definitions in a workspace.
  fn clear_definitions(&mut self) {
    self.definitions.clear();
  }

  /// Utility function that deletes all model evaluators in a workspace.
  fn clear_evaluators(&mut self) {
    self.evaluators.clear();
  }

  /// Utility function that loads and deploys DMN models from specified directory (recursive).
  fn load_and_deploy_models(&mut self, dir: &Path) -> usize {
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
      if entry.file_type().is_file() {
        let file_name = entry.file_name().to_string_lossy();
        if file_name.ends_with(".dmn") {
          if let Ok(xml) = fs::read_to_string(entry.path()) {
            match dmntk_model::parse(&xml) {
              Ok(definitions) => {
                match self.add(definitions) {
                  Ok(()) => {
                    // TODO update status report
                  }
                  Err(reason) => {
                    // TODO update status report
                    eprintln!("{reason}");
                  }
                }
              }
              Err(reason) => {
                // TODO update status report
                eprintln!("{reason}");
              }
            }
          }
        }
      }
    }
    match self.deploy() {
      Ok(()) => {
        // TODO update status report
      }
      Err(reason) => {
        // TODO update status report
        eprintln!("{reason}");
      }
    }
    self.evaluators.len()
  }
}

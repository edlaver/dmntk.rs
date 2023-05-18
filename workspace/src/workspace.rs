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
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fmt, fs};
use walkdir::WalkDir;

/// Type alias defining a map of definitions indexed by its name.
type DefinitionsByName = HashMap<String, Arc<Definitions>>;

/// Type alias defining a map of evaluators indexed by its name.
type EvaluatorsByName = HashMap<String, Arc<ModelEvaluator>>;

#[derive(Serialize, Default)]
pub enum StatusMessage {
  #[default]
  Ok,
  Failure,
}

#[derive(Serialize, Default)]
pub struct DefinitionsStatus {
  #[serde(rename = "file")]
  model_file: String,
  #[serde(rename = "rdnn", skip_serializing_if = "Option::is_none")]
  model_rdnn: Option<String>,
  #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
  model_namespace: Option<String>,
  #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
  model_name: Option<String>,
  #[serde(rename = "status")]
  status: StatusMessage,
  #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
  reason: Option<String>,
}

#[derive(Serialize, Default)]
pub struct EvaluatorStatus {
  #[serde(rename = "rdnn")]
  model_rdnn: String,
  #[serde(rename = "name")]
  model_name: String,
  #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
  deployment_path: Option<String>,
  #[serde(rename = "status")]
  status: StatusMessage,
  #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
  reason: Option<String>,
}

#[derive(Serialize, Default)]
pub struct WorkspaceStatus {
  #[serde(rename = "models")]
  definitions: Vec<DefinitionsStatus>,
  #[serde(rename = "evaluators")]
  evaluators: Vec<EvaluatorStatus>,
}

impl Display for WorkspaceStatus {
  ///
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "a")
  }
}

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
    // create an empty workspace
    let mut workspace = Self {
      definitions: HashMap::new(),
      evaluators: HashMap::new(),
    };
    // load and deploy all DMN models (when optional directory specified)
    if let Some(dir) = opt_dir {
      let workspace_status = workspace.load_and_deploy_models(&dir);
      println!("Deployed {} model(s) from directory: {}", workspace_status.evaluators.len(), dir.to_string_lossy());
    }
    // workspace is now ready to use
    workspace
  }

  /// Deletes all definitions and model evaluators.
  pub fn clear(&mut self) {
    self.definitions.clear();
    self.evaluators.clear();
  }

  /// Adds new definitions to workspace, deletes all model evaluators.
  pub fn add(&mut self, definitions: Definitions) -> Result<(String, String, String)> {
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
        return Err(err_definitions_with_name_already_exists(&rdnn, &name));
      }
    }
    // save definitions by namespace and name
    let definitions_arc = Arc::new(definitions);
    self
      .definitions
      .entry(rdnn.clone())
      .and_modify(|definitions_by_name| {
        // add definitions with specified name to existing namespace
        definitions_by_name.insert(name.clone(), Arc::clone(&definitions_arc));
      })
      .or_insert({
        // add definitions with specified name to namespace that will be created
        let mut definitions_by_name = HashMap::new();
        definitions_by_name.insert(name.clone(), Arc::clone(&definitions_arc));
        definitions_by_name
      });
    // delete all evaluators
    self.evaluators.clear();
    Ok((rdnn, namespace, name))
  }

  /// Creates model evaluators for all definitions in workspace.
  pub fn deploy(&mut self) -> Vec<EvaluatorStatus> {
    self.evaluators.clear();
    let mut evaluator_status_list = vec![];
    for (rdnn, definitions_by_name) in &self.definitions {
      for (name, definitions) in definitions_by_name {
        let mut evaluator_status = EvaluatorStatus {
          model_rdnn: rdnn.to_string(),
          model_name: name.to_string(),
          ..Default::default()
        };
        match ModelEvaluator::new(definitions) {
          Ok(model_evaluator_arc) => {
            self
              .evaluators
              .entry(rdnn.clone())
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
            evaluator_status.deployment_path = Some(format!("{}/{}", rdnn, name));
            evaluator_status.status = StatusMessage::Ok;
          }
          Err(reason) => {
            evaluator_status.status = StatusMessage::Failure;
            evaluator_status.reason = Some(reason.to_string());
          }
        }
        evaluator_status_list.push(evaluator_status);
      }
    }
    evaluator_status_list.sort_by_key(|k| (k.model_rdnn.clone(), k.model_name.clone()));
    evaluator_status_list
  }

  /// Evaluates invocable deployed in workspace.
  pub fn evaluate_invocable(&self, model_namespace: &str, model_name: &str, invocable_name: &str, input_data: &FeelContext) -> Result<Value> {
    if let Some(evaluators_by_name) = self.evaluators.get(model_namespace) {
      if let Some(model_evaluator) = evaluators_by_name.get(model_name) {
        return Ok(model_evaluator.evaluate_invocable(invocable_name, input_data));
      }
    }
    Err(err_model_evaluator_is_not_deployed(model_name))
  }

  /// Utility function that loads and deploys DMN models from specified directory (recursive).
  fn load_and_deploy_models(&mut self, dir: &Path) -> WorkspaceStatus {
    let mut definitions_status_list = vec![];
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
      if entry.file_type().is_file() {
        let file_name = entry.file_name().to_string_lossy();
        if file_name.ends_with(".dmn") {
          let mut definitions_status = DefinitionsStatus {
            model_file: file_name.to_string(),
            ..Default::default()
          };
          match fs::read_to_string(entry.path()) {
            Ok(xml) => match dmntk_model::parse(&xml) {
              Ok(definitions) => {
                definitions_status.model_namespace = Some(definitions.namespace().to_string());
                definitions_status.model_name = Some(definitions.name().to_string());
                match self.add(definitions) {
                  Ok((rdnn, _, _)) => {
                    definitions_status.model_rdnn = Some(rdnn);
                    definitions_status.status = StatusMessage::Ok;
                    definitions_status.reason = None;
                  }
                  Err(reason) => {
                    definitions_status.status = StatusMessage::Failure;
                    definitions_status.reason = Some(reason.to_string());
                  }
                }
              }
              Err(reason) => {
                definitions_status.status = StatusMessage::Failure;
                definitions_status.reason = Some(reason.to_string());
              }
            },
            Err(reason) => {
              definitions_status.status = StatusMessage::Failure;
              definitions_status.reason = Some(reason.to_string());
            }
          }
          definitions_status_list.push(definitions_status);
        }
      }
    }
    definitions_status_list.sort_by_key(|k| k.model_file.to_string());
    let evaluator_status_list = self.deploy();
    WorkspaceStatus {
      definitions: definitions_status_list,
      evaluators: evaluator_status_list,
    }
  }
}

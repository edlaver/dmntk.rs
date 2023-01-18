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

//! ???

use crate::model::*;
use dmntk_feel::Name;
use std::collections::HashMap;
use uuid::Uuid;

/// All definitions needed to build complete model evaluator from DMN models (with imports).
pub struct ModelDefinitions {
  /// Item definitions.
  item_definitions: Vec<ItemDefinition>,
  /// Map of input data definitions indexed by identifier.
  input_data: HashMap<String, InputData>,
  /// Map of business_knowledge models indexed by identifier.
  business_knowledge_models: HashMap<String, BusinessKnowledgeModel>,
  /// Map of decisions indexed by identifier.
  decisions: HashMap<String, Decision>,
  /// Map of decision services indexed by identifier.
  decision_services: HashMap<String, DecisionService>,
  /// Map of knowledge sources indexed by identifier.
  knowledge_sources: HashMap<String, KnowledgeSource>,
}

impl From<Definitions> for ModelDefinitions {
  ///
  fn from(definitions: Definitions) -> Self {
    Self::from(&definitions)
  }
}

impl From<&Definitions> for ModelDefinitions {
  ///
  fn from(definitions: &Definitions) -> Self {
    Self::from(&vec![(None, definitions)])
  }
}

impl From<&Vec<(Option<Name>, &Definitions)>> for ModelDefinitions {
  ///
  fn from(defs: &Vec<(Option<Name>, &Definitions)>) -> Self {
    let mut item_definitions = vec![];
    let mut input_data = HashMap::new();
    let mut business_knowledge_models = HashMap::new();
    let mut decisions = HashMap::new();
    let mut decision_services = HashMap::new();
    let mut knowledge_sources = HashMap::new();
    for (opt_import_name, definitions) in defs {
      println!("DDD: {}", definitions.name());
      let namespace = if opt_import_name.is_some() { Some(definitions.namespace()) } else { None };
      item_definitions.append(&mut definitions.item_definitions().clone());
      for drg_element in definitions.drg_elements() {
        match drg_element {
          DrgElement::InputData(inner) => {
            input_data.insert(prepare_id(namespace, inner.id()), inner.clone());
          }
          DrgElement::BusinessKnowledgeModel(inner) => {
            business_knowledge_models.insert(prepare_id(namespace, inner.id()), inner.clone());
          }
          DrgElement::Decision(inner) => {
            if let Some(id) = inner.id() {
              decisions.insert(id.clone(), inner.clone());
            }
          }
          DrgElement::DecisionService(inner) => {
            if let Some(id) = inner.id() {
              decision_services.insert(id.clone(), inner.clone());
            }
          }
          DrgElement::KnowledgeSource(inner) => {
            if let Some(id) = inner.id() {
              knowledge_sources.insert(id.clone(), inner.clone());
            }
          }
        }
      }
    }
    println!("DDD: bkm(s) = {business_knowledge_models:?}");
    Self {
      item_definitions,
      input_data,
      business_knowledge_models,
      decisions,
      decision_services,
      knowledge_sources,
    }
  }
}

impl ModelDefinitions {
  /// Returns item definitions.
  pub fn item_definitions(&self) -> &Vec<ItemDefinition> {
    &self.item_definitions
  }

  /// Returns references to decisions contained in the model.
  pub fn decisions(&self) -> Vec<&Decision> {
    self.decisions.values().collect()
  }

  /// Returns an optional reference to [Decision] with specified identifier
  /// or [None] when such [Decision] was not found among instances of [DrgElement].
  pub fn decision_by_id(&self, id: &str) -> Option<&Decision> {
    self.decisions.get(id)
  }

  /// Returns references to business knowledge models contained in the model.
  pub fn business_knowledge_models(&self) -> Vec<&BusinessKnowledgeModel> {
    self.business_knowledge_models.values().collect()
  }

  /// Returns an optional reference to [BusinessKnowledgeModel] with specified identifier
  /// or [None] when such [BusinessKnowledgeModel] was not found among instances of [DrgElement].
  pub fn business_knowledge_model_by_id(&self, id: &str) -> Option<&BusinessKnowledgeModel> {
    self.business_knowledge_models.get(id)
  }

  /// Returns a vector of references to decision services.
  pub fn decision_services(&self) -> Vec<&DecisionService> {
    self.decision_services.values().collect()
  }

  /// Returns an optional reference to [DecisionService] with specified identifier
  /// or [None] when such [DecisionService] was not found among instances of [DrgElement].
  pub fn decision_service_by_id(&self, id: &str) -> Option<&DecisionService> {
    self.decision_services.get(id)
  }

  ///
  pub fn input_data(&self) -> Vec<&InputData> {
    self.input_data.values().collect()
  }

  /// Returns an optional reference to [InputData] with specified identifier
  /// or [None] when such [InputData] was not found among
  /// instances of [DrgElement]).
  pub fn input_data_by_id(&self, id: &str) -> Option<&InputData> {
    self.input_data.get(id)
  }

  /// Returns an optional reference to [KnowledgeSource] with specified identifier
  /// or [None] when such [KnowledgeSource] was not found among instances of [DrgElements](DrgElement)).
  pub fn knowledge_source_by_id(&self, id: &str) -> Option<&KnowledgeSource> {
    self.knowledge_sources.get(id)
  }
}

///
fn prepare_id(opt_namespace: Option<&str>, opt_id: &Option<String>) -> String {
  let id = opt_id.as_ref().unwrap_or(&Uuid::new_v4().to_string()).clone();
  if let Some(namespace) = opt_namespace {
    format!("{namespace}#{id}")
  } else {
    id
  }
}

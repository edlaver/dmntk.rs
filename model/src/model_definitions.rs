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
use std::collections::HashMap;

/// All definitions needed to build complete model evaluator from DMN models (with imports).
pub struct ModelDefinitions {
  /// Item definitions.
  item_definitions: Vec<ItemDefinition>,
  /// Map of diagram elements indexed by element identifier.
  drg_elements: HashMap<String, DrgElement>,
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
    let item_definitions = definitions.item_definitions().clone();
    let mut drg_elements = HashMap::new();
    for drg_element in definitions.drg_elements() {
      if let Some(id) = drg_element.get_id() {
        let element = drg_element.clone();
        drg_elements.insert(id, element);
      }
    }
    Self { item_definitions, drg_elements }
  }
}

impl ModelDefinitions {
  /// Returns item definitions.
  pub fn item_definitions(&self) -> &Vec<ItemDefinition> {
    &self.item_definitions
  }

  ///
  pub fn input_data(&self) -> Vec<&InputData> {
    self
      .drg_elements
      .iter()
      .filter_map(|(_, v)| if let DrgElement::InputData(input_data) = v { Some(input_data) } else { None })
      .collect::<Vec<&InputData>>()
  }

  /// Returns references to decisions contained in the model.
  pub fn decisions(&self) -> Vec<&Decision> {
    self
      .drg_elements
      .iter()
      .filter_map(|(_, v)| {
        if v.is_decision() {
          if let DrgElement::Decision(inner) = &v {
            return Some(inner);
          }
        }
        None
      })
      .collect()
  }

  /// Returns references to business knowledge models contained in the model.
  pub fn business_knowledge_models(&self) -> Vec<&BusinessKnowledgeModel> {
    self
      .drg_elements
      .iter()
      .filter_map(|(_, v)| {
        if v.is_business_knowledge_model() {
          if let DrgElement::BusinessKnowledgeModel(inner) = &v {
            return Some(inner);
          }
        }
        None
      })
      .collect()
  }

  /// Returns an optional reference to [BusinessKnowledgeModel] with specified identifier
  /// or [None] when such [BusinessKnowledgeModel] was not found among instances of [DrgElement].
  pub fn business_knowledge_model_by_id(&self, id: &str) -> Option<&BusinessKnowledgeModel> {
    self.drg_elements.iter().find_map(|(_, v)| {
      if v.is_business_knowledge_model() && v.has_id(id) {
        if let DrgElement::BusinessKnowledgeModel(inner) = &v {
          return Some(inner);
        }
      }
      None
    })
  }

  /// Returns an optional reference to [Decision] with specified identifier
  /// or [None] when such [Decision] was not found among instances of [DrgElement].
  pub fn decision_by_id(&self, id: &str) -> Option<&Decision> {
    self.drg_elements.iter().find_map(|(_, v)| {
      if v.is_decision() && v.has_id(id) {
        if let DrgElement::Decision(inner) = &v {
          return Some(inner);
        }
      }
      None
    })
  }

  /// Returns a vector of references to decision services.
  pub fn decision_services(&self) -> Vec<&DecisionService> {
    self
      .drg_elements
      .iter()
      .filter_map(|(_, v)| {
        if v.is_decision_service() {
          if let DrgElement::DecisionService(inner) = &v {
            return Some(inner);
          }
        }
        None
      })
      .collect()
  }

  /// Returns an optional reference to [DecisionService] with specified identifier
  /// or [None] when such [DecisionService] was not found among instances of [DrgElement].
  pub fn decision_service_by_id(&self, id: &str) -> Option<&DecisionService> {
    self.drg_elements.iter().find_map(|(_, v)| {
      if v.is_decision_service() && v.has_id(id) {
        if let DrgElement::DecisionService(inner) = &v {
          return Some(inner);
        }
      }
      None
    })
  }

  /// Returns an optional reference to [InputData] with specified identifier
  /// or [None] when such [InputData] was not found among
  /// instances of [DrgElement]).
  pub fn input_data_by_id(&self, id: &str) -> Option<&InputData> {
    self.drg_elements.iter().find_map(|(_, v)| {
      if v.is_input_data() && v.has_id(id) {
        if let DrgElement::InputData(inner) = &v {
          return Some(inner);
        }
      }
      None
    })
  }

  /// Returns an optional reference to [KnowledgeSource] with specified identifier
  /// or [None] when such [KnowledgeSource] was not found among instances of [DrgElements](DrgElement)).
  pub fn knowledge_source_by_id(&self, id: &str) -> Option<&KnowledgeSource> {
    self.drg_elements.iter().find_map(|(_, v)| {
      if v.is_knowledge_source() && v.has_id(id) {
        if let DrgElement::KnowledgeSource(inner) = &v {
          return Some(inner);
        }
      }
      None
    })
  }
}

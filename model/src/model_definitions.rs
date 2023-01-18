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
use dmntk_common::HRef;
use dmntk_feel::{Name, QualifiedName};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct DefInformationItem {
  feel_name: Name,
  qname: QualifiedName,
  type_ref: Option<String>,
}

impl From<(&Option<Name>, &InformationItem)> for DefInformationItem {
  ///
  fn from((opt_import_name, information_item): (&Option<Name>, &InformationItem)) -> Self {
    Self {
      feel_name: information_item.feel_name().clone(),
      qname: if let Some(import_name) = opt_import_name {
        QualifiedName::new(&[import_name, information_item.feel_name()])
      } else {
        QualifiedName::new(&[information_item.feel_name()])
      },
      type_ref: information_item.type_ref().clone(),
    }
  }
}

impl DefInformationItem {
  /// Returns a qualified name.
  pub fn qname(&self) -> &QualifiedName {
    &self.qname
  }

  /// Returns a reference to optional type reference.
  pub fn type_ref(&self) -> &Option<String> {
    &self.type_ref
  }
}

#[derive(Debug)]
pub struct DefInputData {
  id: String,
  name: String,
  variable: DefInformationItem,
}

impl From<(&Option<Name>, &InputData)> for DefInputData {
  ///
  fn from((opt_import_name, input_data): (&Option<Name>, &InputData)) -> Self {
    Self {
      id: generate_id(input_data.id()),
      name: input_data.name().to_string(),
      variable: (opt_import_name, input_data.variable()).into(),
    }
  }
}

impl DefInputData {
  /// Returns a reference to identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a reference to name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns reference to a variable.
  pub fn variable(&self) -> &DefInformationItem {
    &self.variable
  }
}

#[derive(Debug)]
pub struct DefItemDefinition {
  id: String,
  name: String,
  feel_name: Name,
  type_ref: Option<String>,
  allowed_values: Option<UnaryTests>,
  item_components: Vec<DefItemDefinition>,
  function_item: Option<FunctionItem>,
  is_collection: bool,
}

impl From<(&Option<Name>, &ItemDefinition)> for DefItemDefinition {
  ///
  fn from((opt_import_name, input_data): (&Option<Name>, &ItemDefinition)) -> Self {
    Self {
      id: generate_id(input_data.id()),
      name: input_data.name().to_string(),
      feel_name: input_data.feel_name().clone(),
      type_ref: input_data.type_ref().clone(),
      allowed_values: input_data.allowed_values().clone(),
      item_components: input_data.item_components().iter().map(|a| (opt_import_name, a).into()).collect(),
      function_item: input_data.function_item().clone(),
      is_collection: input_data.is_collection(),
    }
  }
}

impl DefItemDefinition {
  /// Returns a reference to identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a reference to name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns a reference to `FEEL` name.
  pub fn feel_name(&self) -> &Name {
    &self.feel_name
  }

  pub fn type_ref(&self) -> &Option<String> {
    &self.type_ref
  }

  /// Returns reference to possible values or ranges of values
  /// in the base type that are allowed in this [ItemDefinition].
  pub fn allowed_values(&self) -> &Option<UnaryTests> {
    &self.allowed_values
  }

  /// Returns reference to nested [ItemDefinitions](ItemDefinition) that compose this [ItemDefinition].
  pub fn item_components(&self) -> &Vec<DefItemDefinition> {
    &self.item_components
  }

  /// Returns a reference to an optional [FunctionItem] that compose this [ItemDefinition].
  pub fn function_item(&self) -> &Option<FunctionItem> {
    &self.function_item
  }

  /// Returns flag indicating if the actual values are collections of allowed values.
  pub fn is_collection(&self) -> bool {
    self.is_collection
  }
}

#[derive(Debug)]
pub struct DefBusinessKnowledgeModel {
  id: String,
  name: String,
  variable: DefInformationItem,
  encapsulated_logic: Option<FunctionDefinition>,
  knowledge_requirements: Vec<KnowledgeRequirement>,
}

impl From<(&Option<Name>, &BusinessKnowledgeModel)> for DefBusinessKnowledgeModel {
  ///
  fn from(value: (&Option<Name>, &BusinessKnowledgeModel)) -> Self {
    let opt_import_name = value.0;
    let business_knowledge_model = value.1;
    Self {
      id: generate_id(business_knowledge_model.id()),
      name: business_knowledge_model.name().to_string(),
      variable: (opt_import_name, business_knowledge_model.variable()).into(),
      encapsulated_logic: business_knowledge_model.encapsulated_logic().clone(),
      knowledge_requirements: business_knowledge_model.knowledge_requirements().clone(),
    }
  }
}

impl DefBusinessKnowledgeModel {
  /// Returns a reference to identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a reference to name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns reference to a variable.
  pub fn variable(&self) -> &DefInformationItem {
    &self.variable
  }

  /// Returns reference to a variable for this [BusinessKnowledgeModel].
  pub fn encapsulated_logic(&self) -> &Option<FunctionDefinition> {
    &self.encapsulated_logic
  }

  /// Returns reference to the collection of instances of [KnowledgeRequirement] that compose this [BusinessKnowledgeModel].
  pub fn knowledge_requirements(&self) -> &Vec<KnowledgeRequirement> {
    &self.knowledge_requirements
  }
}

#[derive(Debug)]
pub struct DefDecision {
  id: String,
  name: String,
  variable: DefInformationItem,
  decision_logic: Option<ExpressionInstance>,
  information_requirements: Vec<InformationRequirement>,
  knowledge_requirements: Vec<KnowledgeRequirement>,
}

impl From<(&Option<Name>, &Decision)> for DefDecision {
  ///
  fn from((opt_import_name, decision): (&Option<Name>, &Decision)) -> Self {
    Self {
      id: generate_id(decision.id()),
      name: decision.name().to_string(),
      variable: (opt_import_name, decision.variable()).into(),
      decision_logic: decision.decision_logic().clone(),
      information_requirements: decision.information_requirements().clone(),
      knowledge_requirements: decision.knowledge_requirements().clone(),
    }
  }
}

impl DefDecision {
  /// Returns a reference to identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a reference to name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns reference to a variable.
  pub fn variable(&self) -> &DefInformationItem {
    &self.variable
  }

  /// Returns a reference to optional [Expression].
  pub fn decision_logic(&self) -> &Option<ExpressionInstance> {
    &self.decision_logic
  }

  /// Returns a reference to collection of [InformationRequirement].
  pub fn information_requirements(&self) -> &Vec<InformationRequirement> {
    &self.information_requirements
  }

  /// Returns reference to the collection of instances of [KnowledgeRequirement] that compose this [BusinessKnowledgeModel].
  pub fn knowledge_requirements(&self) -> &Vec<KnowledgeRequirement> {
    &self.knowledge_requirements
  }
}

#[derive(Debug)]
pub struct DefDecisionService {
  id: String,
  name: String,
  variable: DefInformationItem,
  input_decisions: Vec<HRef>,
  output_decisions: Vec<HRef>,
  encapsulated_decisions: Vec<HRef>,
  input_data: Vec<HRef>,
}

impl From<(&Option<Name>, &DecisionService)> for DefDecisionService {
  ///
  fn from((opt_import_name, decision_service): (&Option<Name>, &DecisionService)) -> Self {
    Self {
      id: generate_id(decision_service.id()),
      name: decision_service.name().to_string(),
      variable: (opt_import_name, decision_service.variable()).into(),
      input_decisions: decision_service.input_decisions().clone(),
      output_decisions: decision_service.output_decisions().clone(),
      encapsulated_decisions: decision_service.encapsulated_decisions().clone(),
      input_data: decision_service.input_data().clone(),
    }
  }
}

impl DefDecisionService {
  /// Returns a reference to identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a reference to name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns reference to a variable.
  pub fn variable(&self) -> &DefInformationItem {
    &self.variable
  }

  /// Returns a reference to collection of references to input [Decision]s for this [DecisionService].
  pub fn input_decisions(&self) -> &Vec<HRef> {
    &self.input_decisions
  }

  /// Returns a reference to collection of references to encapsulated [Decision]s for this [DecisionService].
  pub fn encapsulated_decisions(&self) -> &Vec<HRef> {
    &self.encapsulated_decisions
  }
  /// Returns a reference to collection of references to output [Decision]s for this [DecisionService].
  pub fn output_decisions(&self) -> &Vec<HRef> {
    &self.output_decisions
  }

  /// Returns a reference to collection of references to [InputData] for this [DecisionService].
  pub fn input_data(&self) -> &Vec<HRef> {
    &self.input_data
  }
}

/// All definitions needed to build complete model evaluator from DMN models (with imports).
pub struct DefDefinitions {
  /// Item definitions.
  item_definitions: Vec<DefItemDefinition>,
  /// Map of input data definitions indexed by identifier.
  input_data: HashMap<String, DefInputData>,
  /// Map of business_knowledge models indexed by identifier.
  business_knowledge_models: HashMap<String, DefBusinessKnowledgeModel>,
  /// Map of decisions indexed by identifier.
  decisions: HashMap<String, DefDecision>,
  /// Map of decision services indexed by identifier.
  decision_services: HashMap<String, DefDecisionService>,
  /// Map of knowledge sources indexed by identifier.
  knowledge_sources: HashMap<String, KnowledgeSource>,
}

impl From<Definitions> for DefDefinitions {
  ///
  fn from(definitions: Definitions) -> Self {
    Self::from(&definitions)
  }
}

impl From<&Definitions> for DefDefinitions {
  ///
  fn from(definitions: &Definitions) -> Self {
    Self::from(&vec![(None, definitions)])
  }
}

impl From<&Vec<(Option<Name>, &Definitions)>> for DefDefinitions {
  ///
  fn from(defs: &Vec<(Option<Name>, &Definitions)>) -> Self {
    let mut item_definitions = vec![];
    let mut input_data = HashMap::new();
    let mut business_knowledge_models = HashMap::new();
    let mut decisions = HashMap::new();
    let mut decision_services = HashMap::new();
    let mut knowledge_sources = HashMap::new();
    for (opt_import_name, definitions) in defs {
      let namespace = if opt_import_name.is_some() { Some(definitions.namespace()) } else { None };
      item_definitions.append(&mut definitions.item_definitions().iter().map(|a| (opt_import_name, a).into()).collect());
      for drg_element in definitions.drg_elements() {
        match drg_element {
          DrgElement::InputData(inner) => {
            input_data.insert(prepare_id(namespace, inner.id()), (opt_import_name, inner).into());
          }
          DrgElement::BusinessKnowledgeModel(inner) => {
            business_knowledge_models.insert(prepare_id(namespace, inner.id()), (opt_import_name, inner).into());
          }
          DrgElement::Decision(inner) => {
            decisions.insert(prepare_id(namespace, inner.id()), (opt_import_name, inner).into());
          }
          DrgElement::DecisionService(inner) => {
            decision_services.insert(prepare_id(namespace, inner.id()), (opt_import_name, inner).into());
          }
          DrgElement::KnowledgeSource(inner) => {
            if let Some(id) = inner.id() {
              knowledge_sources.insert(id.clone(), inner.clone());
            }
          }
        }
      }
    }
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

impl DefDefinitions {
  /// Returns item definitions.
  pub fn item_definitions(&self) -> &Vec<DefItemDefinition> {
    &self.item_definitions
  }

  /// Returns references to decisions contained in the model.
  pub fn decisions(&self) -> Vec<&DefDecision> {
    self.decisions.values().collect()
  }

  /// Returns an optional reference to [Decision] with specified identifier
  /// or [None] when such [Decision] was not found among instances of [DrgElement].
  pub fn decision_by_id(&self, id: &str) -> Option<&DefDecision> {
    self.decisions.get(id)
  }

  /// Returns references to business knowledge models contained in the model.
  pub fn business_knowledge_models(&self) -> Vec<&DefBusinessKnowledgeModel> {
    self.business_knowledge_models.values().collect()
  }

  /// Returns an optional reference to [BusinessKnowledgeModel] with specified identifier
  /// or [None] when such [BusinessKnowledgeModel] was not found among instances of [DrgElement].
  pub fn business_knowledge_model_by_id(&self, id: &str) -> Option<&DefBusinessKnowledgeModel> {
    self.business_knowledge_models.get(id)
  }

  /// Returns a vector of references to decision services.
  pub fn decision_services(&self) -> Vec<&DefDecisionService> {
    self.decision_services.values().collect()
  }

  /// Returns an optional reference to [DecisionService] with specified identifier
  /// or [None] when such [DecisionService] was not found among instances of [DrgElement].
  pub fn decision_service_by_id(&self, id: &str) -> Option<&DefDecisionService> {
    self.decision_services.get(id)
  }

  ///
  pub fn input_data(&self) -> Vec<&DefInputData> {
    self.input_data.values().collect()
  }

  /// Returns an optional reference to [InputData] with specified identifier
  /// or [None] when such [InputData] was not found among
  /// instances of [DrgElement]).
  pub fn input_data_by_id(&self, id: &str) -> Option<&DefInputData> {
    self.input_data.get(id)
  }

  /// Returns an optional reference to [KnowledgeSource] with specified identifier
  /// or [None] when such [KnowledgeSource] was not found among instances of [DrgElements](DrgElement)).
  pub fn knowledge_source_by_id(&self, id: &str) -> Option<&KnowledgeSource> {
    self.knowledge_sources.get(id)
  }
}

/// Generates a new identifier when not provided..
fn generate_id(opt_id: &Option<String>) -> String {
  opt_id.as_ref().unwrap_or(&Uuid::new_v4().to_string()).clone()
}

/// Prepares UUID based on provided optional namespace and optional identifier.
fn prepare_id(opt_namespace: Option<&str>, opt_id: &Option<String>) -> String {
  let id = opt_id.as_ref().unwrap_or(&Uuid::new_v4().to_string()).clone();
  if let Some(namespace) = opt_namespace {
    format!("{namespace}#{id}")
  } else {
    id
  }
}

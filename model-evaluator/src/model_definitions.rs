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

//! # Model definitions needed for building model evaluator

use crate::errors::err_invalid_item_definition_type;
use crate::type_ref::type_ref_to_feel_type;
use dmntk_common::{HRef, Result};
use dmntk_feel::Name;
use dmntk_model::*;
use std::collections::HashMap;
use std::fmt;

/// The key in hash maps for indexing definition artefacts by namespace and identifier.
///
/// [DefKey].0 = namespace
/// [DefKey].1 = identifier
///
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct DefKey(String, String);

impl DefKey {
  /// Creates new definition key based on namespace and identifier.
  pub fn new(namespace: &str, id: &str) -> Self {
    Self(namespace.to_string(), id.to_string())
  }
}

impl fmt::Display for DefKey {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}#{}", self.0, self.1)
  }
}

impl From<&DefHRef> for DefKey {
  ///
  fn from(value: &DefHRef) -> Self {
    Self(value.namespace.to_string(), value.id.to_string())
  }
}

/// Information item definition (variable properties).
#[derive(Debug)]
pub struct DefInformationItem {
  /// Variable namespace.
  namespace: String,
  /// Variable name.
  name: Name,
  /// Variable type reference.
  type_ref: String,
}

impl DefInformationItem {
  /// Creates [DefInformationItem] from [InformationItem].
  pub fn new(namespace: &str, information_item: &InformationItem) -> Self {
    Self {
      namespace: namespace.to_string(),
      name: information_item.feel_name().clone(),
      type_ref: information_item.type_ref().clone(),
    }
  }
}

impl DefInformationItem {
  /// Returns information item's namespace.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  /// Returns information item's name.
  pub fn name(&self) -> &Name {
    &self.name
  }

  /// Returns a reference to optional type reference.
  pub fn type_ref(&self) -> &String {
    &self.type_ref
  }
}

#[derive(Debug)]
pub struct DefInputData {
  namespace: String,
  id: String,
  name: String,
  variable: DefInformationItem,
}

impl DefInputData {
  /// Creates [DefInputData] from [InputData].
  pub fn new(namespace: &str, input_data: &InputData) -> Self {
    Self {
      namespace: namespace.to_string(),
      id: input_data.id().to_string(),
      name: input_data.name().to_string(),
      variable: DefInformationItem::new(namespace, input_data.variable()),
    }
  }
}

impl DefInputData {
  /// Returns the namespace.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

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
  namespace: String,
  id: String,
  name: String,
  feel_name: Name,
  type_ref: Option<String>,
  allowed_values: Option<UnaryTests>,
  item_components: Vec<DefItemDefinition>,
  function_item: Option<FunctionItem>,
  is_collection: bool,
}

impl DefItemDefinition {
  pub fn new(namespace: &str, item_definition: &ItemDefinition) -> Self {
    Self {
      namespace: namespace.to_string(),
      id: item_definition.id().to_string(),
      name: item_definition.name().to_string(),
      feel_name: item_definition.feel_name().clone(),
      type_ref: item_definition.type_ref().clone(),
      allowed_values: item_definition.allowed_values().clone(),
      item_components: item_definition.item_components().iter().map(|inner| DefItemDefinition::new(namespace, inner)).collect(),
      function_item: item_definition.function_item().clone(),
      is_collection: item_definition.is_collection(),
    }
  }
}

impl DefItemDefinition {
  /// Returns the namespace.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  /// Returns the identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns a FEEL name.
  pub fn feel_name(&self) -> &Name {
    &self.feel_name
  }

  /// Returns type reference.
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

  /// Returns the item definition type.
  pub fn item_definition_type(&self) -> Result<ItemDefinitionType> {
    let feel_type = if let Some(type_ref) = self.type_ref() { type_ref_to_feel_type(type_ref) } else { None };
    let condition = (
      self.type_ref().is_some(),
      feel_type.is_some(),
      !self.item_components().is_empty(),
      self.is_collection(),
      self.function_item().is_some(),
    );
    match condition {
      (_, true, false, false, false) => Ok(ItemDefinitionType::SimpleType(feel_type.unwrap())),
      (true, false, false, false, false) => Ok(ItemDefinitionType::ReferencedType(self.namespace.clone(), self.type_ref().as_ref().unwrap().clone())),
      (false, false, true, false, false) => Ok(ItemDefinitionType::ComponentType),
      (_, true, false, true, false) => Ok(ItemDefinitionType::CollectionOfSimpleType(feel_type.unwrap())),
      (false, false, true, true, false) => Ok(ItemDefinitionType::CollectionOfComponentType),
      (true, false, false, true, false) => Ok(ItemDefinitionType::CollectionOfReferencedType(
        self.namespace.clone(),
        self.type_ref().as_ref().unwrap().clone(),
      )),
      (false, false, false, false, true) => Ok(ItemDefinitionType::FunctionType),
      _ => Err(err_invalid_item_definition_type(self.name())),
    }
  }
}

#[derive(Debug)]
pub struct DefBusinessKnowledgeModel {
  namespace: String,
  id: String,
  name: String,
  variable: DefInformationItem,
  encapsulated_logic: Option<FunctionDefinition>,
  knowledge_requirements: Vec<DefKnowledgeRequirement>,
}

impl DefBusinessKnowledgeModel {
  /// Create [DefBusinessKnowledgeModel] from [BusinessKnowledgeModel].
  pub fn new(namespace: &str, business_knowledge_model: &BusinessKnowledgeModel) -> Self {
    Self {
      namespace: namespace.to_string(),
      id: business_knowledge_model.id().to_string(),
      name: business_knowledge_model.name().to_string(),
      variable: DefInformationItem::new(namespace, business_knowledge_model.variable()),
      encapsulated_logic: business_knowledge_model.encapsulated_logic().clone(),
      knowledge_requirements: business_knowledge_model
        .knowledge_requirements()
        .iter()
        .map(|knowledge_requirement| DefKnowledgeRequirement::new(namespace, knowledge_requirement))
        .collect(),
    }
  }
}

impl DefBusinessKnowledgeModel {
  /// Returns the namespace.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  /// Returns an identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns an output variable.
  pub fn variable(&self) -> &DefInformationItem {
    &self.variable
  }

  /// Returns reference to a variable for this [BusinessKnowledgeModel].
  pub fn encapsulated_logic(&self) -> &Option<FunctionDefinition> {
    &self.encapsulated_logic
  }

  /// Returns reference to the collection of instances of [KnowledgeRequirement] that compose this [BusinessKnowledgeModel].
  pub fn knowledge_requirements(&self) -> &Vec<DefKnowledgeRequirement> {
    &self.knowledge_requirements
  }
}

#[derive(Debug)]
pub struct DefHRef {
  namespace: String,
  id: String,
}

impl DefHRef {
  pub fn new(namespace: &str, href: &HRef) -> Self {
    Self {
      namespace: href.namespace().cloned().unwrap_or(namespace.to_string()),
      id: href.id().to_string(),
    }
  }

  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  pub fn id(&self) -> &str {
    &self.id
  }
}

#[derive(Debug)]
pub struct DefInformationRequirement {
  required_decision: Option<DefHRef>,
  required_input: Option<DefHRef>,
}

impl DefInformationRequirement {
  pub fn new(namespace: &str, information_requirement: &InformationRequirement) -> Self {
    Self {
      required_decision: information_requirement.required_decision().as_ref().map(|href| DefHRef::new(namespace, href)),
      required_input: information_requirement.required_input().as_ref().map(|href| DefHRef::new(namespace, href)),
    }
  }

  pub fn required_decision(&self) -> Option<&DefHRef> {
    self.required_decision.as_ref()
  }

  pub fn required_input(&self) -> Option<&DefHRef> {
    self.required_input.as_ref()
  }
}

#[derive(Debug)]
pub struct DefKnowledgeRequirement {
  required_knowledge: DefHRef,
}

impl DefKnowledgeRequirement {
  pub fn new(namespace: &str, knowledge_requirement: &KnowledgeRequirement) -> Self {
    Self {
      required_knowledge: DefHRef::new(namespace, knowledge_requirement.required_knowledge()),
    }
  }

  pub fn required_knowledge(&self) -> &DefHRef {
    &self.required_knowledge
  }
}

#[derive(Debug)]
pub struct DefDecision {
  namespace: String,
  id: String,
  name: String,
  variable: DefInformationItem,
  decision_logic: Option<ExpressionInstance>,
  information_requirements: Vec<DefInformationRequirement>,
  knowledge_requirements: Vec<DefKnowledgeRequirement>,
}

impl DefDecision {
  /// Create [DefDecision] from [Decision].
  pub fn new(namespace: &str, decision: &Decision) -> Self {
    Self {
      namespace: namespace.to_string(),
      id: decision.id().to_string(),
      name: decision.name().to_string(),
      variable: DefInformationItem::new(namespace, decision.variable()),
      decision_logic: decision.decision_logic().clone(),
      information_requirements: decision
        .information_requirements()
        .iter()
        .map(|information_requirement| DefInformationRequirement::new(namespace, information_requirement))
        .collect(),
      knowledge_requirements: decision
        .knowledge_requirements()
        .iter()
        .map(|knowledge_requirement| DefKnowledgeRequirement::new(namespace, knowledge_requirement))
        .collect(),
    }
  }
}

impl DefDecision {
  /// Returns the namespace.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  /// Returns an identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns output variable.
  pub fn variable(&self) -> &DefInformationItem {
    &self.variable
  }

  /// Returns a reference to optional [Expression].
  pub fn decision_logic(&self) -> &Option<ExpressionInstance> {
    &self.decision_logic
  }

  /// Returns a reference to collection of [InformationRequirement].
  pub fn information_requirements(&self) -> &Vec<DefInformationRequirement> {
    &self.information_requirements
  }

  /// Returns reference to the collection of instances of [KnowledgeRequirement] that compose this [BusinessKnowledgeModel].
  pub fn knowledge_requirements(&self) -> &Vec<DefKnowledgeRequirement> {
    &self.knowledge_requirements
  }
}

#[derive(Debug)]
pub struct DefDecisionService {
  namespace: String,
  id: String,
  name: String,
  variable: DefInformationItem,
  input_decisions: Vec<DefHRef>,
  output_decisions: Vec<DefHRef>,
  encapsulated_decisions: Vec<DefHRef>,
  input_data: Vec<DefHRef>,
}

impl DefDecisionService {
  /// Creates [DefDecisionService] from [DecisionService].
  pub fn new(namespace: &str, decision_service: &DecisionService) -> Self {
    Self {
      namespace: namespace.to_string(),
      id: decision_service.id().to_string(),
      name: decision_service.name().to_string(),
      variable: DefInformationItem::new(namespace, decision_service.variable()),
      input_decisions: decision_service.input_decisions().iter().map(|href| DefHRef::new(namespace, href)).collect(),
      output_decisions: decision_service.output_decisions().iter().map(|href| DefHRef::new(namespace, href)).collect(),
      encapsulated_decisions: decision_service.encapsulated_decisions().iter().map(|href| DefHRef::new(namespace, href)).collect(),
      input_data: decision_service.input_data().iter().map(|href| DefHRef::new(namespace, href)).collect(),
    }
  }
}

impl DefDecisionService {
  /// Returns the namespace.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

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

  /// Returns a collection of input [Decision]s for this [DecisionService].
  pub fn input_decisions(&self) -> &Vec<DefHRef> {
    &self.input_decisions
  }

  /// Returns a collection of encapsulated [Decision]s for this [DecisionService].
  pub fn encapsulated_decisions(&self) -> &Vec<DefHRef> {
    &self.encapsulated_decisions
  }
  /// Returns a collection of output [Decision]s for this [DecisionService].
  pub fn output_decisions(&self) -> &Vec<DefHRef> {
    &self.output_decisions
  }

  /// Returns a collection of [InputData] for this [DecisionService].
  pub fn input_data(&self) -> &Vec<DefHRef> {
    &self.input_data
  }
}

/// All definitions needed to build complete model evaluator from DMN models.
pub struct DefDefinitions {
  /// Item definitions.
  item_definitions: Vec<DefItemDefinition>,
  /// Map of input data definitions indexed by identifier.
  input_data: HashMap<DefKey, DefInputData>,
  /// Map of business_knowledge models indexed by identifier.
  business_knowledge_models: HashMap<DefKey, DefBusinessKnowledgeModel>,
  /// Map of decisions indexed by identifier.
  decisions: HashMap<DefKey, DefDecision>,
  /// Map of decision services indexed by identifier.
  decision_services: HashMap<DefKey, DefDecisionService>,
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
    Self::from(&vec![definitions])
  }
}

impl From<&Vec<&Definitions>> for DefDefinitions {
  ///
  fn from(defs: &Vec<&Definitions>) -> Self {
    let mut item_definitions = vec![];
    let mut input_data = HashMap::new();
    let mut business_knowledge_models = HashMap::new();
    let mut decisions = HashMap::new();
    let mut decision_services = HashMap::new();
    for definitions in defs {
      let namespace = definitions.namespace();
      item_definitions.append(&mut definitions.item_definitions().iter().map(|inner| DefItemDefinition::new(namespace, inner)).collect());
      for drg_element in definitions.drg_elements() {
        match drg_element {
          DrgElement::InputData(inner) => {
            input_data.insert(DefKey::new(namespace, inner.id()), DefInputData::new(namespace, inner));
          }
          DrgElement::BusinessKnowledgeModel(inner) => {
            business_knowledge_models.insert(DefKey::new(namespace, inner.id()), DefBusinessKnowledgeModel::new(namespace, inner));
          }
          DrgElement::Decision(inner) => {
            decisions.insert(DefKey::new(namespace, inner.id()), DefDecision::new(namespace, inner));
          }
          DrgElement::DecisionService(inner) => {
            decision_services.insert(DefKey::new(namespace, inner.id()), DefDecisionService::new(namespace, inner));
          }
          _ => {}
        }
      }
    }
    Self {
      item_definitions,
      input_data,
      business_knowledge_models,
      decisions,
      decision_services,
    }
  }
}

impl DefDefinitions {
  /// Returns item definitions.
  pub fn item_definitions(&self) -> &Vec<DefItemDefinition> {
    &self.item_definitions
  }

  /// Returns references to decisions.
  pub fn decisions(&self) -> Vec<&DefDecision> {
    self.decisions.values().collect()
  }

  /// Returns an optional reference to [Decision] with specified identifier
  /// or [None] when such [Decision] was not found among instances of [DrgElement].
  pub fn decision_by_key(&self, namespace: &str, id: &str) -> Option<&DefDecision> {
    self.decisions.get(&DefKey::new(namespace, id))
  }

  /// Returns references to business knowledge models.
  pub fn business_knowledge_models(&self) -> Vec<&DefBusinessKnowledgeModel> {
    self.business_knowledge_models.values().collect()
  }

  /// Returns an optional reference to [BusinessKnowledgeModel] with specified identifier
  /// or [None] when such [BusinessKnowledgeModel] was not found among instances of [DrgElement].
  pub fn business_knowledge_model_by_key(&self, namespace: &str, id: &str) -> Option<&DefBusinessKnowledgeModel> {
    self.business_knowledge_models.get(&DefKey::new(namespace, id))
  }

  /// Returns references to decision services.
  pub fn decision_services(&self) -> Vec<&DefDecisionService> {
    self.decision_services.values().collect()
  }

  /// Returns an optional reference to [DecisionService] with specified identifier
  /// or [None] when such [DecisionService] was not found among instances of [DrgElement].
  pub fn decision_service_by_id(&self, namespace: &str, id: &str) -> Option<&DefDecisionService> {
    self.decision_services.get(&DefKey::new(namespace, id))
  }

  /// Returns references to input data instances.
  pub fn input_data(&self) -> Vec<&DefInputData> {
    self.input_data.values().collect()
  }

  /// Returns an optional reference to [InputData] with specified identifier
  /// or [None] when such [InputData] was not found among
  /// instances of [DrgElement]).
  pub fn input_data_by_key(&self, namespace: &str, id: &str) -> Option<&DefInputData> {
    self.input_data.get(&DefKey::new(namespace, id))
  }
}

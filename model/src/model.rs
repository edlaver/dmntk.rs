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

//! # DMN model
//!
//! Model for Decision Requirements Graph (DRG)
//! depicted in one or more Decision Requirements Diagrams (DRD).

use crate::errors::*;
use dmntk_common::{DmntkError, HRef, OptHRef, Result};
use dmntk_feel::{FeelType, Name};
use std::convert::TryFrom;
use std::fmt;
use std::slice::Iter;

pub const URI_FEEL: &str = "https://www.omg.org/spec/DMN/20191111/FEEL/";
pub const URI_MODEL: &str = "https://www.omg.org/spec/DMN/20191111/MODEL/";
pub const URI_UNINTERPRETED: &str = "http://www.omg.org/spec/DMN/uninterpreted/20140801";
pub const URI_XML_SCHEMA: &str = "http://www.w3.org/2001/XMLSchema";

/// [DmnElement] is the abstract superclass for the Decision Model elements.
/// It provides the optional attributes `id`, `description` and `label`,
/// which other elements will inherit.
pub trait DmnElement {
  /// Returns reference to optional identifier for this [DmnElement].
  /// This identifier SHALL be unique within its containing [Definitions] element.
  fn id(&self) -> &Option<String>;
  /// Returns reference to optional description of this [DmnElement].
  fn description(&self) -> &Option<String>;
  /// Returns reference to optional alternative short description of this [DmnElement].
  fn label(&self) -> &Option<String>;
  /// Returns reference to attached additional elements to any [DmnElement].
  fn extension_elements(&self) -> &Option<ExtensionElements>;
  /// Returns reference to attached named extended attributes and model associations to any [DmnElement].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute>;
}

/// [NamedElement] adds attribute `name` to [DmnElement].
/// `name` attribute is required for [NamedElement].
pub trait NamedElement: DmnElement {
  /// Returns the name of this [NamedElement].
  fn name(&self) -> &str;
  /// Returns the `FEEL` name for this element.
  fn feel_name(&self) -> &Name;
}

/// [Expression] is an abstract class that describes the logic
/// by which a modeled decision shall be made, or pieces of that logic.
pub trait Expression: DmnElement {
  /// Optional namespace-prefixed name of the base type of this [Expression].
  fn type_ref(&self) -> &Option<String>;
}

/// [FeelTypedElement] adds the `FEEL` type attributes to element.
pub trait FeelTypedElement {
  /// Returns the optional `FEEL` type for this element.
  fn feel_type(&self) -> &Option<FeelType>;
  /// Sets `FEEL` type for this element.
  fn set_feel_type(&mut self, feel_type: FeelType);
}

/// [RequiredTypeRef] adds the required type reference to element.
pub trait RequiredTypeRef {
  /// Namespace-prefixed name of the base type of the implementor.
  fn type_ref(&self) -> &str;
}

/// [RequiredVariable] adds the required reference to [InformationItem].
pub trait RequiredVariable {
  /// Returns the reference to [InformationItem].
  fn variable(&self) -> &InformationItem;
}

/// `Invocable` is used to model the inputs of a decision whose values
/// are defined outside of the decision model.
pub trait Invocable: DmnElement + NamedElement + RequiredVariable {}

/// The abstract class [BusinessContextElement], and its concrete specializations
/// [PerformanceIndicator] and [OrganizationUnit] are placeholders,
/// anticipating a definition to be adopted from other OMG meta-models,
/// such as OMG OSM when it is further developed.
pub trait BusinessContextElement: NamedElement {
  /// The URI of this [BusinessContextElement].
  fn uri(&self) -> &Option<String>;
}

/// [Element] represents an element from another metamodel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
  pub(crate) name: String,
}

impl Element {
  /// Returns element's name.
  pub fn name(&self) -> &str {
    &self.name
  }
}

/// The [ExtensionElements] element is a container to aggregate elements
/// from other metamodels inside any [DmnElement].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionElements {
  pub(crate) elements: Vec<Element>,
}

impl ExtensionElements {
  /// Returns extension elements.
  pub fn elements(&self) -> &Vec<Element> {
    &self.elements
  }
}

/// The [ExtensionAttribute] element contains an [Element] or a reference
/// to an [Element] from another metamodel. An [ExtensionAttribute] also has a name
/// to define the role or purpose of the associated element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionAttribute {
  /// The name of the extension attribute.
  pub name: String,
  /// The contained [Element].
  /// This attribute SHALL NOT be used together with `value_ref`.
  pub value: Option<Element>,
  /// A reference to the associated [Element].
  /// This attribute SHALL NOT be used together with `value`.
  pub value_ref: Option<Element>,
}

/// Enumeration of concrete instances of [BusinessContextElement].
#[derive(Debug, Clone)]
pub enum BusinessContextElementInstance {
  PerformanceIndicator(PerformanceIndicator),
  OrganizationUnit(OrganizationUnit),
}

///
#[derive(Debug, Clone)]
pub struct PerformanceIndicator {
  /// Optional identifier of this this [PerformanceIndicator].
  pub(crate) id: Option<String>,
  /// Optional description of this [PerformanceIndicator].
  pub(crate) description: Option<String>,
  /// An optional alternative short description of this [PerformanceIndicator].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [PerformanceIndicator].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [PerformanceIndicator].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Name of this [PerformanceIndicator].
  pub(crate) name: String,
  /// `FEEL` name of this [PerformanceIndicator].
  pub(crate) feel_name: Name,
  /// The URI of this [PerformanceIndicator].
  pub(crate) uri: Option<String>,
  /// Collection of [Decision] that impact this [Perform`anceIndicator].
  /// This attribute stores references
  pub(crate) impacting_decisions: Vec<HRef>,
}

impl DmnElement for PerformanceIndicator {
  /// Returns reference to optional identifier for this [PerformanceIndicator].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [PerformanceIndicator].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [PerformanceIndicator].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [PerformanceIndicator].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [PerformanceIndicator].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl NamedElement for PerformanceIndicator {
  /// Returns a reference to the name of this element.
  fn name(&self) -> &str {
    &self.name
  }
  /// Returns a reference to `FEEL` name of this element.
  fn feel_name(&self) -> &Name {
    &self.feel_name
  }
}

impl BusinessContextElement for PerformanceIndicator {
  fn uri(&self) -> &Option<String> {
    &self.uri
  }
}

impl PerformanceIndicator {
  pub fn impacting_decisions(&self) -> &Vec<HRef> {
    &self.impacting_decisions
  }
}

///
#[derive(Debug, Clone)]
pub struct OrganizationUnit {
  /// Optional identifier of this this [OrganizationUnit].
  pub(crate) id: Option<String>,
  /// Optional description of this [OrganizationUnit].
  pub(crate) description: Option<String>,
  /// An optional alternative short description of this [OrganizationUnit].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [OrganizationUnit].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [OrganizationUnit].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Name of this [OrganizationUnit].
  pub(crate) name: String,
  /// `FEEL` name of this [OrganizationUnit].
  pub(crate) feel_name: Name,
  /// The URI of this [OrganizationUnit].
  pub(crate) uri: Option<String>,
  /// Collection of [Decision] that are made by this [OrganizationUnit].
  pub(crate) decisions_made: Vec<HRef>,
  /// Collection of [Decision] that are owned by this [OrganizationUnit].
  pub(crate) decisions_owned: Vec<HRef>,
}

impl DmnElement for OrganizationUnit {
  /// Returns reference to optional identifier for this [OrganizationUnit].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [OrganizationUnit].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [OrganizationUnit].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [OrganizationUnit].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [OrganizationUnit].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl NamedElement for OrganizationUnit {
  /// Returns a reference to the name of this element.
  fn name(&self) -> &str {
    &self.name
  }
  /// Returns a reference to `FEEL` name of this element.
  fn feel_name(&self) -> &Name {
    &self.feel_name
  }
}

impl BusinessContextElement for OrganizationUnit {
  fn uri(&self) -> &Option<String> {
    &self.uri
  }
}

impl OrganizationUnit {
  pub fn decisions_made(&self) -> &Vec<HRef> {
    &self.decisions_made
  }
  pub fn decisions_owned(&self) -> &Vec<HRef> {
    &self.decisions_owned
  }
}

/// In DMN™ model [DrgElement] is the abstract superclass for all DMN™ elements
/// that are contained within [Definitions] and that have a graphical representation in a DRD.
/// This enumeration specifies the list of [DRGElements](DrgElement) contained in [Definitions].
#[derive(Debug, Clone)]
pub enum DrgElement {
  Decision(Decision),
  InputData(InputData),
  BusinessKnowledgeModel(BusinessKnowledgeModel),
  DecisionService(DecisionService),
  KnowledgeSource(KnowledgeSource),
}

#[derive(Debug, Clone)]
pub enum Requirement {
  Information(InformationRequirement),
  Knowledge(KnowledgeRequirement),
  Authority(AuthorityRequirement),
}

/// [Definitions] element is the outermost containing object
/// for all elements of a DMN decision model.
/// It defines the scope of visibility and the namespace
/// for all contained elements.
#[derive(Debug, Clone)]
pub struct Definitions {
  /// Optional identifier for this [Definitions] derived from [DMNElement](DmnElement).
  pub(crate) id: Option<String>,
  /// Optional description of this [Definitions] derived from [DMNElement](DmnElement).
  pub(crate) description: Option<String>,
  /// Optional alternative short description of this [Definitions] derived from [DMNElement](DmnElement).
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [Definitions] derived from [DMNElement](DmnElement).
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [Definitions] derived from [DMNElement](DmnElement).
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Name of this [Definitions] derived from [NamedElement].
  pub(crate) name: String,
  /// `FEEL` name of this [ItemDefinition].
  pub(crate) feel_name: Name,
  /// Identifies the namespace associated with this [Definitions]
  /// and follows the convention established by XML Schema.
  pub(crate) namespace: String,
  /// This attribute identifies the expression language used in
  /// [LiteralExpressions](LiteralExpression) within the scope
  /// of this [Definitions]. The _Default_ is FEEL.
  /// This value **MAY** be overridden on each individual [LiteralExpression].
  /// The language **SHALL** be specified in a URI format.
  pub(crate) expression_language: Option<String>,
  /// This attribute identifies the type language used in
  /// [LiteralExpressions](LiteralExpression) within the scope
  /// of this [Definitions]. The _Default_ is FEEL.
  /// This value **MAY** be overridden on each individual [ItemDefinition].
  /// The language **SHALL** be specified in a URI format.
  pub(crate) type_language: Option<String>,
  /// Name of the tool used to export the XML serialization.
  pub(crate) exporter: Option<String>,
  /// Version of the tool used to export the XML serialization.
  pub(crate) exporter_version: Option<String>,
  /// Container for the instances of [ItemDefinition] that are contained in this [Definitions].
  pub(crate) item_definitions: Vec<ItemDefinition>,
  /// Container for the instances of [DrgElement] that are contained in this [Definitions].
  pub(crate) drg_elements: Vec<DrgElement>,
  /// Container for the instances of [BusinessContextElement] that are contained in this [Definitions].
  pub(crate) business_context_elements: Vec<BusinessContextElementInstance>,
  /// Container used to import externally defined elements and make them available for use by elements in this [Definitions].
  pub(crate) imports: Vec<Import>,
  /// Optional Diagram Interchange information contained within this [Definitions].
  pub(crate) dmndi: Option<Dmndi>,
}

impl Definitions {
  /// Returns the reference to the namespace associated with this [Definitions].
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  /// Returns the reference to optional expression language used within the scope of this [Definitions].
  pub fn expression_language(&self) -> &Option<String> {
    &self.expression_language
  }

  /// Returns reference to the type language used within the scope of this [Definitions].
  pub fn type_language(&self) -> &Option<String> {
    &self.type_language
  }

  /// Returns reference to the name of the tool used to export the XML serialization.
  pub fn exporter(&self) -> &Option<String> {
    &self.exporter
  }

  /// Returns reference to the version of the tool used to export the XML serialization.
  pub fn exporter_version(&self) -> &Option<String> {
    &self.exporter_version
  }

  /// Returns reference to the container of instances of [ItemDefinition] contained in this [Definitions].
  pub fn item_definitions(&self) -> &Vec<ItemDefinition> {
    &self.item_definitions
  }

  /// Returns reference to the container of instances of [Import] contained in this [Definitions].
  pub fn imports(&self) -> &Vec<Import> {
    &self.imports
  }

  /// Returns reference to optional [Dmndi] container.
  pub fn dmndi(&self) -> &Option<Dmndi> {
    &self.dmndi
  }

  /// Returns reference to [DrgElements](DrgElement) container.
  pub fn drg_elements(&self) -> Iter<DrgElement> {
    self.drg_elements.iter()
  }

  /// Returns all decision definitions.
  pub fn decisions(&self) -> Vec<Decision> {
    self
      .drg_elements
      .iter()
      .filter_map(|drg_element| {
        if let DrgElement::Decision(decision) = drg_element {
          Some(decision.clone())
        } else {
          None
        }
      })
      .collect()
  }

  /// Returns all business knowledge model definitions.
  pub fn business_knowledge_models(&self) -> Vec<BusinessKnowledgeModel> {
    self
      .drg_elements
      .iter()
      .filter_map(|drg_element| {
        if let DrgElement::BusinessKnowledgeModel(bkm) = drg_element {
          Some(bkm.clone())
        } else {
          None
        }
      })
      .collect()
  }

  /// Returns all decision services definitions.
  pub fn decision_services(&self) -> Vec<DecisionService> {
    self
      .drg_elements
      .iter()
      .filter_map(|drg_element| {
        if let DrgElement::DecisionService(decision_service) = drg_element {
          Some(decision_service.clone())
        } else {
          None
        }
      })
      .collect()
  }

  /// Returns all knowledge source definitions.
  pub fn knowledge_sources(&self) -> Vec<KnowledgeSource> {
    self
      .drg_elements
      .iter()
      .filter_map(|drg_element| {
        if let DrgElement::KnowledgeSource(knowledge_source) = drg_element {
          Some(knowledge_source.clone())
        } else {
          None
        }
      })
      .collect()
  }

  /// Returns all input data definitions.
  pub fn input_data(&self) -> Vec<InputData> {
    self
      .drg_elements
      .iter()
      .filter_map(|drg_element| {
        if let DrgElement::InputData(input_data) = drg_element {
          Some(input_data.clone())
        } else {
          None
        }
      })
      .collect()
  }

  /// Returns performance indicators.
  pub fn performance_indicators(&self) -> Vec<&PerformanceIndicator> {
    self
      .business_context_elements
      .iter()
      .filter_map(|item| match item {
        BusinessContextElementInstance::PerformanceIndicator(performance_indicator) => Some(performance_indicator),
        _ => None,
      })
      .collect()
  }

  /// Returns organisation units.
  pub fn organisation_units(&self) -> Vec<&OrganizationUnit> {
    self
      .business_context_elements
      .iter()
      .filter_map(|item| match item {
        BusinessContextElementInstance::OrganizationUnit(organisarion_unit) => Some(organisarion_unit),
        _ => None,
      })
      .collect()
  }

  /// Returns decision with specified identifier.
  pub fn get_decision(&self, id: &str) -> Option<Decision> {
    for drg_element in &self.drg_elements {
      if let DrgElement::Decision(decision) = drg_element {
        if let Some(decision_id) = decision.id() {
          if decision_id == id {
            return Some(decision.clone());
          }
        }
      }
    }
    None
  }

  /// Returns input data with specified identifier.
  pub fn get_input_data(&self, id: &str) -> Option<InputData> {
    for drg_element in &self.drg_elements {
      if let DrgElement::InputData(input_data) = drg_element {
        if let Some(decision_id) = input_data.id() {
          if decision_id == id {
            return Some(input_data.clone());
          }
        }
      }
    }
    None
  }

  /// Returns business knowledge model with specified identifier.
  pub fn get_business_knowledge_model(&self, id: &str) -> Option<BusinessKnowledgeModel> {
    for drg_element in &self.drg_elements {
      if let DrgElement::BusinessKnowledgeModel(business_knowledge_model) = drg_element {
        if let Some(decision_id) = business_knowledge_model.id() {
          if decision_id == id {
            return Some(business_knowledge_model.clone());
          }
        }
      }
    }
    None
  }

  /// Returns knowledge source with specified identifier.
  pub fn get_knowledge_source(&self, id: &str) -> Option<KnowledgeSource> {
    for drg_element in &self.drg_elements {
      if let DrgElement::KnowledgeSource(knowledge_source) = drg_element {
        if let Some(decision_id) = knowledge_source.id() {
          if decision_id == id {
            return Some(knowledge_source.clone());
          }
        }
      }
    }
    None
  }

  /// Returns a requirement with specified identifier.
  pub fn get_requirement(&self, id: &str) -> Option<Requirement> {
    for drg_element in &self.drg_elements {
      match drg_element {
        DrgElement::Decision(decision) => {
          for r in &decision.knowledge_requirements {
            if let Some(r_id) = r.id() {
              if r_id == id {
                return Some(Requirement::Knowledge(r.clone()));
              }
            }
          }
          for r in &decision.information_requirements {
            if let Some(r_id) = r.id() {
              if r_id == id {
                return Some(Requirement::Information(r.clone()));
              }
            }
          }
          for r in &decision.authority_requirements {
            if let Some(r_id) = r.id() {
              if r_id == id {
                return Some(Requirement::Authority(r.clone()));
              }
            }
          }
        }
        DrgElement::InputData(_) => {}
        DrgElement::BusinessKnowledgeModel(business_knowledge_model) => {
          for r in &business_knowledge_model.knowledge_requirements {
            if let Some(r_id) = r.id() {
              if r_id == id {
                return Some(Requirement::Knowledge(r.clone()));
              }
            }
          }
          for r in &business_knowledge_model.authority_requirements {
            if let Some(r_id) = r.id() {
              if r_id == id {
                return Some(Requirement::Authority(r.clone()));
              }
            }
          }
        }
        DrgElement::DecisionService(_) => {}
        DrgElement::KnowledgeSource(knowledge_source) => {
          for r in &knowledge_source.authority_requirements {
            if let Some(r_id) = r.id() {
              if r_id == id {
                return Some(Requirement::Authority(r.clone()));
              }
            }
          }
        }
      }
    }
    None
  }
}

impl DmnElement for Definitions {
  /// Returns reference to optional identifier for this [Definitions].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [Definitions].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [Definitions].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [Definitions].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [Definitions].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl NamedElement for Definitions {
  /// Returns reference to the name of this [Definitions].
  fn name(&self) -> &str {
    &self.name
  }
  /// Returns a reference to FEEL name of this element.
  fn feel_name(&self) -> &Name {
    &self.feel_name
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InformationItem {
  /// Optional identifier of this this [InformationItem].
  pub(crate) id: Option<String>,
  /// Optional description of this [InformationItem].
  pub(crate) description: Option<String>,
  /// Optional alternative short description of this [InformationItem].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [InformationItem].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [InformationItem].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Name of this [InformationItem].
  pub(crate) name: String,
  /// `FEEL` name of this [ItemDefinition].
  pub(crate) feel_name: Name,
  /// Optional qualified name of the type of this [InformationItem].
  pub(crate) type_ref: Option<String>,
  /// Optional `FEEL` type of this [InformationItem].
  pub(crate) feel_type: Option<FeelType>,
}

impl InformationItem {
  ///
  pub fn type_ref(&self) -> &Option<String> {
    &self.type_ref
  }
}

impl DmnElement for InformationItem {
  /// Returns reference to optional identifier for this [InformationItem].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [InformationItem].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [InformationItem].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [InformationItem].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [InformationItem].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl NamedElement for InformationItem {
  /// Returns reference to the name of this [InformationItem].
  fn name(&self) -> &str {
    &self.name
  }
  /// Returns a reference to `FEEL` name of this element.
  fn feel_name(&self) -> &Name {
    &self.feel_name
  }
}

impl FeelTypedElement for InformationItem {
  /// Returns a reference to optional `FEEL` type of this element.
  fn feel_type(&self) -> &Option<FeelType> {
    &self.feel_type
  }
  /// Sets the `FEEL` type for this element.
  fn set_feel_type(&mut self, feel_type: FeelType) {
    self.feel_type = Some(feel_type);
  }
}

/// [InputData] is used to model the inputs of a decision whose values
/// are defined outside of the decision model.
#[derive(Debug, Clone)]
pub struct InputData {
  /// Optional identifier of this this [InputData].
  pub(crate) id: Option<String>,
  /// Optional description of this [InputData].
  pub(crate) description: Option<String>,
  /// An optional alternative short description of this [InputData].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [InputData].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [InputData].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Name of this [InputData].
  pub(crate) name: String,
  /// `FEEL` name of this [ItemDefinition].
  pub(crate) feel_name: Name,
  /// The instance of [InformationItem] that stores the result of this [InputData].
  pub(crate) variable: InformationItem,
}

impl RequiredVariable for InputData {
  /// Returns reference to a variable for this [BusinessKnowledgeModel].  
  fn variable(&self) -> &InformationItem {
    &self.variable
  }
}

impl DmnElement for InputData {
  /// Returns reference to optional identifier for this [InputData].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [InputData].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [InputData].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [InputData].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [InputData].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl NamedElement for InputData {
  /// Returns reference to the name of this [InputData].
  fn name(&self) -> &str {
    &self.name
  }
  /// Returns a reference to `FEEL` name of this element.
  fn feel_name(&self) -> &Name {
    &self.feel_name
  }
}

/// `Import` class is used when referencing external elements,
/// either DMN [DRGElement](DrgElement) or [ItemDefinition] instances contained
/// in other [Definitions] elements, or non-DMN elements,
/// such as an XML Schema or a PMML file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
  /// Optional identifier of this this [Import].
  pub(crate) id: Option<String>,
  /// Optional description of this [Import].
  pub(crate) description: Option<String>,
  /// An optional alternative short description of this [Import].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [Import].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [Import].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Name of this [Import]. Serves as a prefix in namespace-qualified names,
  /// such as typeRefs specifying imported [ItemDefinitions](ItemDefinition)
  /// and expressions referencing imported [InformationItems](InformationItem).
  pub(crate) name: String,
  /// Optional `FEEL` name of this [Import].
  pub(crate) feel_name: Name,
  /// Specifies the style of import associated with this [Import].
  pub(crate) import_type: String,
  /// Identifies the location of the imported element.
  pub(crate) location_uri: Option<String>,
  /// Identifies the namespace of the imported element.
  pub(crate) namespace: String,
}

impl DmnElement for Import {
  /// Returns reference to optional identifier for this [InputData].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [InputData].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [InputData].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [InputData].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [InputData].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl NamedElement for Import {
  /// Returns reference to the name of this [Import].
  fn name(&self) -> &str {
    &self.name
  }
  /// Returns a reference to `FEEL` name of this [Import].
  fn feel_name(&self) -> &Name {
    &self.feel_name
  }
}

impl Import {
  /// Returns reference to the import type for this [Import].
  pub fn import_type(&self) -> &str {
    &self.import_type
  }
  /// Returns reference to the optional location URI for this [Import].
  pub fn location_uri(&self) -> &Option<String> {
    &self.location_uri
  }
  /// Returns reference to the namespace of this [Import].
  pub fn namespace(&self) -> &str {
    &self.namespace
  }
}

/// Enumeration of concrete instances of abstract [Expression], which are:
/// - [Context],
/// - [DecisionTable],
/// - [FunctionDefinition],
/// - [Invocation],
/// - [LiteralExpression],
/// - [Relation].
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionInstance {
  Context(Box<Context>),
  DecisionTable(Box<DecisionTable>),
  FunctionDefinition(Box<FunctionDefinition>),
  Invocation(Box<Invocation>),
  LiteralExpression(Box<LiteralExpression>),
  Relation(Box<Relation>),
}

/// A [Context] is composed of any number of model context entries, which are instances of [ContextEntry].
#[derive(Debug, Clone, PartialEq)]
pub struct Context {
  /// This attribute lists the instances of [ContextEntry] that compose this [Context].
  pub(crate) context_entries: Vec<ContextEntry>,
}

impl Context {
  /// Return a reference to context entries that compose this [Context].
  pub fn context_entries(&self) -> &Vec<ContextEntry> {
    &self.context_entries
  }
}

/// The class [ContextEntry] is used to model `FEEL` context entries when a context is modeled as a [Context] element.
#[derive(Debug, Clone, PartialEq)]
pub struct ContextEntry {
  /// The instance of [InformationItem] that is contained in this [ContextEntry],
  /// and whose name is the key in the modeled context entry.
  pub variable: Option<InformationItem>,
  /// The instance of [Expression] that is the expression in this [ContextEntry].
  pub value: ExpressionInstance,
}

/// [LiteralExpression] is used to model a value expression whose value
/// is specified by text in some specified expression language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralExpression {
  /// Optional identifier of this this [LiteralExpression].
  pub(crate) id: Option<String>,
  /// Optional description of this [LiteralExpression].
  pub(crate) description: Option<String>,
  /// An optional alternative short description of this [LiteralExpression].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [LiteralExpression].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [LiteralExpression].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Optional type definition for this [LiteralExpression].
  pub(crate) type_ref: Option<String>,
  /// The text of this [LiteralExpression].
  /// It SHALL be a valid expression in the `expression_language`.
  pub(crate) text: Option<String>,
  /// Identifies the expression language used in this [LiteralExpression].
  pub(crate) expression_language: Option<String>,
  /// The instance of [ImportedValue](Import) that specifies
  /// where the text of this [LiteralExpression] is located.
  pub(crate) imported_values: Option<Import>,
}

impl LiteralExpression {
  ///
  pub fn text(&self) -> &Option<String> {
    &self.text
  }
  ///
  pub fn expression_language(&self) -> Option<String> {
    self.expression_language.clone()
  }
  ///
  pub fn imported_values(&self) -> Option<Import> {
    self.imported_values.clone()
  }
}

impl DmnElement for LiteralExpression {
  /// Returns reference to optional identifier for this [LiteralExpression].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [LiteralExpression].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [LiteralExpression].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [LiteralExpression].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [LiteralExpression].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl Expression for LiteralExpression {
  /// Returns an optional type reference.
  fn type_ref(&self) -> &Option<String> {
    &self.type_ref
  }
}

/// [Invocation] is a mechanism that permits the evaluation of one value expression – the invoked expression – inside
/// another value expression – the invoking expression – by binding locally the input variables of the invoked
/// expression to values inside the invoking expression.
#[derive(Debug, Clone, PartialEq)]
pub struct Invocation {
  /// An expression whose value is a function.
  pub(crate) called_function: ExpressionInstance,
  /// Instances of [Binding] used to bind the formal parameters of the called function in this [Invocation].
  pub(crate) bindings: Vec<Binding>,
}

impl Invocation {
  /// Returns a reference to called function which is an instance of [Expression].
  pub fn called_function(&self) -> &ExpressionInstance {
    &self.called_function
  }
  /// Returns a reference to the collection of binding instances.
  pub fn bindings(&self) -> &Vec<Binding> {
    &self.bindings
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
  /// The [InformationItem] on which the `calledFunction` of the owning
  /// instance of [Invocation] depends that is bound by this [Binding].
  pub(crate) parameter: InformationItem,
  /// The instance of [Expression] to which the parameter in this [Binding] is
  /// bound when the owning instance of [Invocation] is evaluated.
  pub(crate) binding_formula: Option<ExpressionInstance>,
}

impl Binding {
  /// Returns a reference to parameter.
  pub fn parameter(&self) -> &InformationItem {
    &self.parameter
  }
  /// Returns a reference to binding formula.
  pub fn binding_formula(&self) -> &Option<ExpressionInstance> {
    &self.binding_formula
  }
}

/// [Decision]
#[derive(Debug, Clone)]
pub struct Decision {
  /// Identifier of the [Decision].
  pub(crate) id: Option<String>,
  /// Description of the [Decision].
  pub(crate) description: Option<String>,
  /// An alternative short description of the [Decision].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [Decision].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [Decision].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Name of the [Decision].
  pub(crate) name: String,
  /// `FEEL` name of this [ItemDefinition].
  pub(crate) feel_name: Name,
  /// A natural language question that characterizes the [Decision],
  /// such that the output of the [Decision] is an answer to the question.
  pub(crate) question: Option<String>,
  /// A natural language description of the answers allowed for the question
  /// such as `Yes/No`, a list of allowed values, a range of numeric values etc.
  pub(crate) allowed_answers: Option<String>,
  /// The instance of [InformationItem] that stores the result of this [Decision].
  pub(crate) variable: InformationItem,
  /// The instance of the [Expression] for the [Decision].
  pub(crate) decision_logic: Option<ExpressionInstance>,
  /// Collection of the instances of [InformationRequirement] that compose this [Decision].
  pub(crate) information_requirements: Vec<InformationRequirement>,
  /// Collection of the instances of [KnowledgeRequirement] that compose this [Decision].
  pub(crate) knowledge_requirements: Vec<KnowledgeRequirement>,
  /// Collection of the instances of [AuthorityRequirement] that compose this [Decision].
  pub(crate) authority_requirements: Vec<AuthorityRequirement>,
  // TODO add the following:
  // supported_objectives
  // impacted_performance_indicator
  // decision_maker
  // decision_owner
  // using_processes
  // using_tasks
}

impl Decision {
  /// Returns a reference to a natural language question that characterizes the [Decision].
  pub fn question(&self) -> &Option<String> {
    &self.question
  }
  /// Returns a reference to a natural language description of the answers allowed for the question defined in this [Decision].
  pub fn allowed_answers(&self) -> &Option<String> {
    &self.allowed_answers
  }
  /// Return a reference to a variable that stores the result of this [Decision].
  pub fn variable(&self) -> &InformationItem {
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
  /// Returns a reference to collection of [KnowledgeRequirement].
  pub fn knowledge_requirements(&self) -> &Vec<KnowledgeRequirement> {
    &self.knowledge_requirements
  }
  /// Returns a reference to collection of [AuthorityRequirement].
  pub fn authority_requirements(&self) -> &Vec<AuthorityRequirement> {
    &self.authority_requirements
  }
}

impl DmnElement for Decision {
  /// Returns reference to optional identifier for this [Decision].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [Decision].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [Decision].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [Decision].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [Decision].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl NamedElement for Decision {
  /// Returns reference to the name of this [Decision].
  fn name(&self) -> &str {
    &self.name
  }
  /// Returns a reference to `FEEL` name of this element.
  fn feel_name(&self) -> &Name {
    &self.feel_name
  }
}

/// The class [InformationRequirement] is used to model an information requirement,
/// as represented by a plain arrow in a DRD.
#[derive(Debug, Clone)]
pub struct InformationRequirement {
  /// Optional identifier of the [InformationRequirement].
  pub(crate) id: Option<String>,
  /// Optional description of the [InformationRequirement].
  pub(crate) description: Option<String>,
  /// An optional alternative short description of the [InformationRequirement].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [InformationRequirement].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [InformationRequirement].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Reference to [Decision] that this [InformationRequirement] associates
  /// with its containing  [Decision] element.
  pub(crate) required_decision: OptHRef,
  /// Reference to [InputData] that this [InformationRequirement] associates
  /// with its containing [Decision] element.
  pub(crate) required_input: OptHRef,
}

impl InformationRequirement {
  /// Returns reference to optional URI pointing [Decision].
  pub fn required_decision(&self) -> &OptHRef {
    &self.required_decision
  }
  /// Returns reference to optional URI pointing [InputData].
  pub fn required_input(&self) -> &OptHRef {
    &self.required_input
  }
}

impl DmnElement for InformationRequirement {
  /// Returns reference to optional identifier for this [InformationRequirement].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [InformationRequirement].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [InformationRequirement].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [InformationRequirement].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [InformationRequirement].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

/// The class [KnowledgeRequirement] is used to model a knowledge requirement,
/// as represented by a dashed arrow in a DRD.
#[derive(Debug, Clone)]
pub struct KnowledgeRequirement {
  /// Optional identifier of the [KnowledgeRequirement].
  pub(crate) id: Option<String>,
  /// Optional description of the [KnowledgeRequirement].
  pub(crate) description: Option<String>,
  /// An optional alternative short description of the [KnowledgeRequirement].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [KnowledgeRequirement].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [KnowledgeRequirement].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Reference to [Invocable] that this [KnowledgeRequirement] associates with
  /// its containing [Decision] or [BusinessKnowledgeModel] element.
  pub(crate) required_knowledge: OptHRef,
}

impl KnowledgeRequirement {
  /// Returns reference to the [Invocable].
  pub fn required_knowledge(&self) -> &OptHRef {
    &self.required_knowledge
  }
}

impl DmnElement for KnowledgeRequirement {
  /// Returns reference to optional identifier for this [KnowledgeRequirement].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [KnowledgeRequirement].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [KnowledgeRequirement].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [KnowledgeRequirement].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [KnowledgeRequirement].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

/// The class [AuthorityRequirement] is used to model an authority requirement,
/// as represented by an arrow drawn with a dashed line and a filled circular head in a DRD
#[derive(Debug, Clone)]
pub struct AuthorityRequirement {
  /// Optional identifier of the [AuthorityRequirement].
  pub(crate) id: Option<String>,
  /// Optional description of the [AuthorityRequirement].
  pub(crate) description: Option<String>,
  /// An optional alternative short description of the [AuthorityRequirement].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [AuthorityRequirement].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [AuthorityRequirement].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// The instance of [KnowledgeSource] that this [AuthorityRequirement] associates
  /// with its containing [KnowledgeSource], [Decision] or [BusinessKnowledgeModel] element.
  pub(crate) required_authority: OptHRef,
  /// The instance of [Decision] that this [AuthorityRequirement] associates
  /// with its containing [KnowledgeSource] element.
  pub(crate) required_decision: OptHRef,
  /// The instance of [InputData] that this [AuthorityRequirement] associates
  /// with its containing [KnowledgeSource] element.
  pub(crate) required_input: OptHRef,
}

impl AuthorityRequirement {
  /// Returns reference to optional [KnowledgeSource].
  pub fn required_authority(&self) -> &OptHRef {
    &self.required_authority
  }
  /// Returns reference to optional [Decision].
  pub fn required_decision(&self) -> &OptHRef {
    &self.required_decision
  }
  /// Returns reference to optional [InputData].
  pub fn required_input(&self) -> &OptHRef {
    &self.required_input
  }
}

impl DmnElement for AuthorityRequirement {
  /// Returns reference to optional identifier for this [AuthorityRequirement].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [AuthorityRequirement].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [AuthorityRequirement].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [AuthorityRequirement].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [AuthorityRequirement].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

/// The class [KnowledgeSource] is used to model authoritative knowledge sources in a decision model.
/// In a DRD, an instance of [KnowledgeSource] is represented by a `knowledge source` diagram element.
#[derive(Debug, Clone)]
pub struct KnowledgeSource {
  /// Optional identifier of this this [KnowledgeSource].
  pub(crate) id: Option<String>,
  /// Optional description of this [KnowledgeSource].
  pub(crate) description: Option<String>,
  /// Optional alternative short description of this [KnowledgeSource].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [KnowledgeSource].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [KnowledgeSource].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Name of this [KnowledgeSource].
  pub(crate) name: String,
  /// `FEEL` name of this [KnowledgeSource].
  pub(crate) feel_name: Name,
  /// Collection of the instances of [AuthorityRequirement] that compose this [Decision].
  pub(crate) authority_requirements: Vec<AuthorityRequirement>,
}

impl KnowledgeSource {
  /// Returns a reference to collection of [AuthorityRequirement].
  pub fn authority_requirements(&self) -> &Vec<AuthorityRequirement> {
    &self.authority_requirements
  }
}

impl DmnElement for KnowledgeSource {
  /// Returns reference to optional identifier for this [KnowledgeSource].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [KnowledgeSource].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [KnowledgeSource].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [KnowledgeSource].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [KnowledgeSource].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl NamedElement for KnowledgeSource {
  /// Returns reference to the name of this [KnowledgeSource].
  fn name(&self) -> &str {
    &self.name
  }
  /// Returns a reference to `FEEL` name of this [KnowledgeSource].
  fn feel_name(&self) -> &Name {
    &self.feel_name
  }
}

/// Instances of `Invocable` interface.
#[derive(Debug, Clone)]
pub enum InvocableInstance {
  BusinessKnowledgeModel(Box<BusinessKnowledgeModel>),
  DecisionService,
}

/// A business knowledge model has an abstract part, representing reusable,
/// invocable decision logic, and a concrete part, which mandates that the decision logic
/// must be a single FEEL boxed function definition.
#[derive(Debug, Clone)]
pub struct BusinessKnowledgeModel {
  /// Optional identifier of this this [BusinessKnowledgeModel].
  pub(crate) id: Option<String>,
  /// Optional description of this [BusinessKnowledgeModel].
  pub(crate) description: Option<String>,
  /// Optional alternative short description of this [BusinessKnowledgeModel].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [BusinessKnowledgeModel].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [BusinessKnowledgeModel].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Name of this [BusinessKnowledgeModel].
  pub(crate) name: String,
  /// `FEEL` name of this [ItemDefinition].
  pub(crate) feel_name: Name,
  /// Variable that is bound to the function defined by the [FunctionDefinition] for this [BusinessKnowledgeModel].
  pub(crate) variable: InformationItem,
  /// The function that encapsulates the logic encapsulated by this [BusinessKnowledgeModel].
  pub(crate) encapsulated_logic: Option<FunctionDefinition>,
  /// This attribute lists the instances of [KnowledgeRequirement] that compose this [BusinessKnowledgeModel].
  pub(crate) knowledge_requirements: Vec<KnowledgeRequirement>,
  /// This attribute lists the instances of [AuthorityRequirement] that compose this [BusinessKnowledgeModel].
  pub(crate) authority_requirements: Vec<AuthorityRequirement>,
}

impl BusinessKnowledgeModel {
  /// Returns reference to a variable for this [BusinessKnowledgeModel].
  pub fn encapsulated_logic(&self) -> &Option<FunctionDefinition> {
    &self.encapsulated_logic
  }
  /// Returns reference to the collection of instances of [KnowledgeRequirement] that compose this [BusinessKnowledgeModel].
  pub fn knowledge_requirements(&self) -> &Vec<KnowledgeRequirement> {
    &self.knowledge_requirements
  }
  /// Returns reference to the collection of instances of [AuthorityRequirement] that compose this [BusinessKnowledgeModel].
  pub fn authority_requirements(&self) -> &Vec<AuthorityRequirement> {
    &self.authority_requirements
  }
}

impl DmnElement for BusinessKnowledgeModel {
  /// Returns reference to optional identifier for this [BusinessKnowledgeModel].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [BusinessKnowledgeModel].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [BusinessKnowledgeModel].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [BusinessKnowledgeModel].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [BusinessKnowledgeModel].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl NamedElement for BusinessKnowledgeModel {
  /// Returns reference to the name of this [ItemDefinition].
  fn name(&self) -> &str {
    &self.name
  }
  /// Returns a reference to `FEEL` name of this element.
  fn feel_name(&self) -> &Name {
    &self.feel_name
  }
}

impl RequiredVariable for BusinessKnowledgeModel {
  /// Returns reference to a variable for this [BusinessKnowledgeModel].
  fn variable(&self) -> &InformationItem {
    &self.variable
  }
}

/// The [DecisionService] class is used to define named decision services
/// against the decision model contained in an instance of [Definitions].
#[derive(Debug, Clone)]
pub struct DecisionService {
  /// Optional identifier of this this [DecisionService].
  pub(crate) id: Option<String>,
  /// Optional description of this [DecisionService].
  pub(crate) description: Option<String>,
  /// Optional alternative short description of this [DecisionService].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [DecisionService].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [DecisionService].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Name of this [DecisionService].
  pub(crate) name: String,
  /// `FEEL` name of this [ItemDefinition].
  pub(crate) feel_name: Name,
  /// Variable for this [DecisionService].
  pub(crate) variable: InformationItem,
  /// Collection of references to the instances of [Decision] required to be output by this [DecisionService].
  pub(crate) output_decisions: Vec<HRef>,
  /// Collection of references to the instances of [Decision] to be encapsulated in this [DecisionService].
  pub(crate) encapsulated_decisions: Vec<HRef>,
  /// Collection of references to the instances of [Decision] required as input by this [DecisionService].
  pub(crate) input_decisions: Vec<HRef>,
  /// Collection of references to the instances of [InputData] required as input by this [DecisionService].
  pub(crate) input_data: Vec<HRef>,
}

impl DecisionService {
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

impl DmnElement for DecisionService {
  /// Returns reference to optional identifier for this [DecisionService].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [DecisionService].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [DecisionService].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [DecisionService].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [DecisionService].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl NamedElement for DecisionService {
  /// Returns reference to the name of this [ItemDefinition].
  fn name(&self) -> &str {
    &self.name
  }
  /// Returns a reference to `FEEL` name of this element.
  fn feel_name(&self) -> &Name {
    &self.feel_name
  }
}

impl RequiredVariable for DecisionService {
  /// Returns reference to a variable for this [DecisionService].
  fn variable(&self) -> &InformationItem {
    &self.variable
  }
}

/// Item definition types.
#[derive(Debug, Clone, PartialEq)]
pub enum ItemDefinitionType {
  SimpleType(FeelType),
  ReferencedType(String),
  ComponentType,
  CollectionOfSimpleType(FeelType),
  CollectionOfReferencedType(String),
  CollectionOfComponentType,
  FunctionType,
}

/// [ItemDefinition] is used to model the inputs of a decision,
/// whose values are defined outside of the decision model.
#[derive(Debug, Clone)]
pub struct ItemDefinition {
  /// Optional identifier of this this [ItemDefinition].
  pub(crate) id: Option<String>,
  /// Optional description of this [ItemDefinition].
  pub(crate) description: Option<String>,
  /// Optional alternative short description of this [ItemDefinition].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [ItemDefinition].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [ItemDefinition].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Name of this [ItemDefinition].
  pub(crate) name: String,
  /// `FEEL` name of this [ItemDefinition].
  pub(crate) feel_name: Name,
  /// Optional base type of this [ItemDefinition] identified by namespace-prefixed name.
  pub(crate) type_ref: Option<String>,
  /// This attribute identifies the type language used to specify the base
  /// type of this [ItemDefinition]. This value overrides the type
  /// language specified in the [Definitions] element. The default is `FEEL`.
  /// The language `SHALL` be specified in an URI format.
  pub(crate) type_language: Option<String>,
  /// This attribute contains `FEEL` built-in type only when the `type_language` attribute
  /// is `FEEL` and the `type_ref` attribute defines one of the built-in `FEEL` types.
  pub(crate) feel_type: Option<FeelType>,
  /// Possible values or ranges of values in the base type that are allowed in this [ItemDefinition].
  pub(crate) allowed_values: Option<UnaryTests>,
  /// Defines zero or more nested [ItemDefinitions](ItemDefinition) that compose this [ItemDefinition].
  pub(crate) item_components: Vec<ItemDefinition>,
  /// Setting this flag to true indicates that the actual values defined by
  /// this [ItemDefinition] are collections of allowed values.
  /// The default value is [false].
  pub(crate) is_collection: bool,
  /// Describes an optional [FunctionItem] that compose this [ItemDefinition].
  pub(crate) function_item: Option<FunctionItem>,
}

impl ItemDefinition {
  /// Returns reference to the type language used within the scope of this [ItemDefinition].
  pub fn type_language(&self) -> &Option<String> {
    &self.type_language
  }
  /// Returns reference to possible values or ranges of values
  /// in the base type that are allowed in this [ItemDefinition].
  pub fn allowed_values(&self) -> &Option<UnaryTests> {
    &self.allowed_values
  }
  /// Returns reference to nested [ItemDefinitions](ItemDefinition) that compose this [ItemDefinition].
  pub fn item_components(&self) -> &Vec<ItemDefinition> {
    &self.item_components
  }
  /// Returns mutable reference to nested [ItemDefinitions](ItemDefinition) that compose this [ItemDefinition].
  pub fn item_components_mut(&mut self) -> &mut Vec<ItemDefinition> {
    &mut self.item_components
  }
  /// Returns flag indicating if the actual values are collections of allowed values.
  pub fn is_collection(&self) -> bool {
    self.is_collection
  }
  /// Returns a reference to optional `FEEL` type.
  pub fn feel_type(&self) -> &Option<FeelType> {
    &self.feel_type
  }
  /// Sets the `FEEL` type for this element.
  pub fn set_feel_type(&mut self, feel_type: FeelType) {
    self.feel_type = Some(feel_type);
  }
  /// Returns a reference to an optional [FunctionItem] that compose this [ItemDefinition].
  pub fn function_item(&self) -> &Option<FunctionItem> {
    &self.function_item
  }
}

impl DmnElement for ItemDefinition {
  /// Returns reference to optional identifier for this [ItemDefinition].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [ItemDefinition].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [ItemDefinition].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [ItemDefinition].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [ItemDefinition].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl NamedElement for ItemDefinition {
  /// Returns reference to the name of this [ItemDefinition].
  fn name(&self) -> &str {
    &self.name
  }
  /// Returns a reference to `FEEL` name of this element.
  fn feel_name(&self) -> &Name {
    &self.feel_name
  }
}

impl Expression for ItemDefinition {
  fn type_ref(&self) -> &Option<String> {
    &self.type_ref
  }
}

/// [UnaryTests] is used to model a boolean test, where the argument
/// to be tested is implicit or denoted with a **?**.
/// Test is specified by text in some specified expression language.
#[derive(Debug, Clone)]
pub struct UnaryTests {
  /// The text of this [UnaryTests].
  /// It SHALL be a valid expression in the expressionLanguage.
  pub(crate) text: Option<String>,
  /// This attribute identifies the expression language used in this [UnaryTests].
  /// This value overrides the expression language specified for the containing
  /// instance of DecisionRequirementDiagram.
  /// The language SHALL be specified in a URI format.
  pub(crate) expression_language: Option<String>,
}

impl UnaryTests {
  /// Returns reference to optional text of this [UnaryTests].
  pub fn text(&self) -> &Option<String> {
    &self.text
  }
  /// Returns reference to optional expression language used in this [UnaryTests].
  pub fn expression_language(&self) -> &Option<String> {
    &self.expression_language
  }
}

/// [FunctionItem] defines the signature of a function:
/// the parameters and the output type of the function.
#[derive(Debug, Clone)]
pub struct FunctionItem {
  /// Reference to output type of the function.
  pub(crate) output_type_ref: Option<String>,
  /// Function parameters as [InformationItems](InformationItem).
  pub(crate) parameters: Vec<InformationItem>,
}

impl FunctionItem {
  /// Returns reference to output type of function.
  pub fn output_type_ref(&self) -> &Option<String> {
    &self.output_type_ref
  }
  /// Returns reference to function parameters defined
  /// as collection of [InformationItems](InformationItem).
  pub fn parameters(&self) -> &Vec<InformationItem> {
    &self.parameters
  }
}

/// Defines the type of the [FunctionDefinition].
/// The default value is `FEEL`. Supported values also include `Java` and `PMML`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionKind {
  Feel,
  Java,
  Pmml,
}

/// [FunctionItem] defines the signature of a function:
/// the parameters and the output type of the function.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
  /// Optional identifier of this this [FunctionDefinition].
  pub(crate) id: Option<String>,
  /// Optional description of this [FunctionDefinition].
  pub(crate) description: Option<String>,
  /// Optional alternative short description of this [FunctionDefinition].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [FunctionDefinition].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [FunctionDefinition].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Optional base type of this [FunctionDefinition] identified by namespace-prefixed name.
  pub(crate) type_ref: Option<String>,
  /// Container for instances of [InformationItem] that are the parameters of this [FunctionDefinition].
  pub(crate) formal_parameters: Vec<InformationItem>,
  /// The instance of [Expression] that is the body in this [FunctionDefinition].
  pub(crate) body: Option<ExpressionInstance>,
  /// Type of this [FunctionDefinition], the default value is FEEL.
  pub(crate) kind: FunctionKind,
}

impl FunctionDefinition {
  /// Returns reference to container of [InformationItem] of this [FunctionDefinition].
  pub fn formal_parameters(&self) -> &Vec<InformationItem> {
    &self.formal_parameters
  }
  /// Returns reference to [Expression] that is the body in this [FunctionDefinition].
  pub fn body(&self) -> &Option<ExpressionInstance> {
    &self.body
  }
  /// Returns the type of this [FunctionDefinition].
  pub fn kind(&self) -> &FunctionKind {
    &self.kind
  }
}

impl DmnElement for FunctionDefinition {
  /// Returns reference to optional identifier for this [FunctionDefinition].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [FunctionDefinition].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [FunctionDefinition].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [FunctionDefinition].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [FunctionDefinition].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl Expression for FunctionDefinition {
  fn type_ref(&self) -> &Option<String> {
    &self.type_ref
  }
}

/// A [Relation] is convenient shorthand for a list of similar contexts.
/// A [Relation] has a column instead of repeated `ContextEntry`s,
/// and a `List` is used for every row, with one of the `List`’s
/// expression for each column value.
#[derive(Debug, Clone, PartialEq)]
pub struct Relation {
  /// Optional identifier of this this [Relation].
  pub(crate) id: Option<String>,
  /// Optional description of this [Relation].
  pub(crate) description: Option<String>,
  /// Optional alternative short description of this [Relation].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [Relation].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [Relation].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Optional base type of this [Relation] identified by namespace-prefixed name.
  pub(crate) type_ref: Option<String>,
  /// This attribute lists the instances of [List] that are the rows in this [Relation].
  pub(crate) rows: Vec<List>,
  /// This attributes lists the instances of [InformationItem] that define the columns in this [Relation].
  pub(crate) columns: Vec<InformationItem>,
}

impl Relation {
  /// Returns a reference to collection of relation's rows.
  pub fn rows(&self) -> &Vec<List> {
    &self.rows
  }
  /// Returns a reference to collection of relation's columns.
  pub fn columns(&self) -> &Vec<InformationItem> {
    &self.columns
  }
}

impl DmnElement for Relation {
  /// Returns reference to optional identifier for this [FunctionDefinition].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [FunctionDefinition].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [FunctionDefinition].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [FunctionDefinition].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [FunctionDefinition].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl Expression for Relation {
  fn type_ref(&self) -> &Option<String> {
    &self.type_ref
  }
}

/// A [List] is simply a list of elements, which are instances of [Expression]s.
#[derive(Debug, Clone, PartialEq)]
pub struct List {
  /// Optional identifier of this this [List].
  pub(crate) id: Option<String>,
  /// Optional description of this [List].
  pub(crate) description: Option<String>,
  /// Optional alternative short description of this [List].
  pub(crate) label: Option<String>,
  /// Container to attach additional elements to any [List].
  pub(crate) extension_elements: Option<ExtensionElements>,
  /// Container to attach named extended attributes and model associations to any [List].
  pub(crate) extension_attributes: Vec<ExtensionAttribute>,
  /// Optional base type of this [List] identified by namespace-prefixed name.
  pub(crate) type_ref: Option<String>,
  /// This attribute lists the instances of [Expression] that are the elements in this [List].
  pub(crate) elements: Vec<ExpressionInstance>,
}

impl List {
  /// Returns a reference to collection of list's elements.
  pub fn elements(&self) -> &Vec<ExpressionInstance> {
    &self.elements
  }
}

impl DmnElement for List {
  /// Returns reference to optional identifier for this [FunctionDefinition].
  fn id(&self) -> &Option<String> {
    &self.id
  }
  /// Returns reference to optional description of this [FunctionDefinition].
  fn description(&self) -> &Option<String> {
    &self.description
  }
  /// Returns reference to optional alternative short description of this [FunctionDefinition].
  fn label(&self) -> &Option<String> {
    &self.label
  }
  /// Returns reference to attached additional elements to any [FunctionDefinition].
  fn extension_elements(&self) -> &Option<ExtensionElements> {
    &self.extension_elements
  }
  /// Returns reference to attached named extended attributes and model associations to any [FunctionDefinition].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
    &self.extension_attributes
  }
}

impl Expression for List {
  fn type_ref(&self) -> &Option<String> {
    &self.type_ref
  }
}

/// Decision table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionTable {
  /// Information item name, for which the decision table is its value expression.
  /// This is usually the name of the decision or the name of business knowledge model for
  /// which the decision table provides the decision logic.
  pub(crate) information_item_name: Option<String>,
  /// List of instances of input clause that compose this decision table.
  pub(crate) input_clauses: Vec<InputClause>,
  /// List of instances of output clause that compose this decision table.
  pub(crate) output_clauses: Vec<OutputClause>,
  /// List of instances of rule annotation clause that compose this decision table.
  pub(crate) annotations: Vec<RuleAnnotationClause>,
  /// List of instances of decision rule that compose this decision table.
  pub(crate) rules: Vec<DecisionRule>,
  /// Hit policy associated with the instance of the decision table.
  pub(crate) hit_policy: HitPolicy,
  /// Optional aggregation type when the hit policy is `COLLECT`.
  pub(crate) aggregation: Option<BuiltinAggregator>,
  /// Preferred representation of the instance of the decision table.
  pub(crate) preferred_orientation: DecisionTableOrientation,
  /// Optional output label for the description of the decision table output,
  /// may be the same as the name of the information item for which the
  /// decision table is the value expression.
  pub(crate) output_label: Option<String>,
}

/// Implementation of [Display](fmt::Display) for [DecisionTable].
impl fmt::Display for DecisionTable {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut buffer = String::new();
    buffer.push_str("Decision table:\n");
    buffer.push_str(format!(">> preferred orientation: {}\n", self.preferred_orientation).as_str());
    buffer.push_str(">> information item name: ");
    if let Some(text) = &self.information_item_name {
      buffer.push_str(format!("\n'{text}'\n").as_str());
    } else {
      buffer.push_str("none\n");
    }
    buffer.push_str(format!(">> hit policy: {}\n", self.hit_policy).as_str());
    buffer.push_str(">> aggregation: ");
    if let Some(aggregation) = &self.aggregation {
      buffer.push_str(format!("{aggregation}\n").as_str());
    } else {
      buffer.push_str("none\n");
    }
    buffer.push_str(">> output label: ");
    if let Some(text) = &self.output_label {
      buffer.push_str(format!("\n'{text}'\n").as_str());
    } else {
      buffer.push_str("none\n");
    }
    write!(f, "{buffer}")
  }
}

impl DecisionTable {
  /// Creates a new decision table.
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    information_item_name: Option<String>,
    input_clauses: Vec<InputClause>,
    output_clauses: Vec<OutputClause>,
    annotations: Vec<RuleAnnotationClause>,
    rules: Vec<DecisionRule>,
    hit_policy: HitPolicy,
    aggregation: Option<BuiltinAggregator>,
    preferred_orientation: DecisionTableOrientation,
    output_label: Option<String>,
  ) -> Self {
    Self {
      information_item_name,
      input_clauses,
      output_clauses,
      annotations,
      rules,
      hit_policy,
      aggregation,
      preferred_orientation,
      output_label,
    }
  }

  /// Returns the information item name.
  pub fn information_item_name(&self) -> &Option<String> {
    &self.information_item_name
  }

  /// Returns an iterator over input clauses.
  pub fn input_clauses(&self) -> Iter<InputClause> {
    self.input_clauses.iter()
  }

  /// Returns an iterator over output clauses.
  pub fn output_clauses(&self) -> Iter<OutputClause> {
    self.output_clauses.iter()
  }

  /// Returns an iterator over annotations.
  pub fn annotations(&self) -> Iter<RuleAnnotationClause> {
    self.annotations.iter()
  }

  /// Returns an iterator over rules.
  pub fn rules(&self) -> Iter<DecisionRule> {
    self.rules.iter()
  }

  /// Returns the [HitPolicy] of this decision table.
  pub fn hit_policy(&self) -> HitPolicy {
    self.hit_policy
  }

  /// Returns the aggregation when the [HitPolicy] is `COLLECT`.
  pub fn aggregation(&self) -> &Option<BuiltinAggregator> {
    &self.aggregation
  }

  /// Returns preferred orientation for this decision table.
  pub fn preferred_orientation(&self) -> &DecisionTableOrientation {
    &self.preferred_orientation
  }

  /// Return an output label.
  pub fn output_label(&self) -> &Option<String> {
    &self.output_label
  }

  /// Returns `true` when allowed input and/or allowed output values are present in decision table.
  pub fn allowed_values_present(&self) -> bool {
    for input_clause in &self.input_clauses {
      if input_clause.allowed_input_values.is_some() {
        return true;
      }
    }
    for output_clause in &self.output_clauses {
      if output_clause.allowed_output_values.is_some() {
        return true;
      }
    }
    false
  }
}

/// Orientation of the decision table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionTableOrientation {
  /// Decision table is presented horizontally, rules are presented as rows.
  RuleAsRow,
  /// Decision table is presented vertically, rules are presented as columns.
  RuleAsColumn,
  /// Decision table is presented as crosstab, rules are composed from two input dimensions.
  CrossTable,
}

/// Implementation of [Display](fmt::Display) for [DecisionTableOrientation].
impl fmt::Display for DecisionTableOrientation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DecisionTableOrientation::RuleAsRow => write!(f, "Rule-as-Row"),
      DecisionTableOrientation::RuleAsColumn => write!(f, "Rule-as-Column"),
      DecisionTableOrientation::CrossTable => write!(f, "CrossTable"),
    }
  }
}

impl TryFrom<&str> for DecisionTableOrientation {
  type Error = DmntkError;
  /// Tries to construct a decision table orientation from its text representation.
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value.trim() {
      "Rule-as-Row" => Ok(DecisionTableOrientation::RuleAsRow),
      "Rule-as-Column" => Ok(DecisionTableOrientation::RuleAsColumn),
      "CrossTable" => Ok(DecisionTableOrientation::CrossTable),
      other => Err(err_invalid_decision_table_orientation(other)),
    }
  }
}

/// Hit policy.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HitPolicy {
  /// `UNIQUE` hit policy. No overlapping rules are allowed, only single rule can be matched.
  /// This is the default value for hit policy. Crosstab decision tables may have only unique hit policy.
  Unique,
  /// `ANY` hit policy. Rules may overlap, but all matching rules show equal output entries.
  /// If matching rules have non-equal output entries, this policy is incorrect and the result is undefined.
  Any,
  /// `PRIORITY` hit policy. Multiple rules can match, with different output entries for each output.
  /// This policy returns matching rule with the highest priority. Output priorities are specified
  /// in the ordered list of output values, in decreasing order of priority.
  Priority,
  /// `FIRST` hit policy...
  First,
  /// `COLLECT` hit policy...
  Collect(BuiltinAggregator),
  /// `OUTPUT ORDER` hit policy...
  OutputOrder,
  /// `RULE ORDER` hit policy...
  RuleOrder,
}

/// Implementation of [Display](fmt::Display) for [HitPolicy].
impl fmt::Display for HitPolicy {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      HitPolicy::Unique => write!(f, "U"),
      HitPolicy::Any => write!(f, "A"),
      HitPolicy::Priority => write!(f, "P"),
      HitPolicy::First => write!(f, "F"),
      HitPolicy::Collect(aggregator) => write!(f, "{aggregator}"),
      HitPolicy::OutputOrder => write!(f, "O"),
      HitPolicy::RuleOrder => write!(f, "R"),
    }
  }
}

impl TryFrom<&str> for HitPolicy {
  type Error = DmntkError;
  /// Creates a hit policy from text.
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value.trim() {
      "U" => Ok(HitPolicy::Unique),
      "A" => Ok(HitPolicy::Any),
      "P" => Ok(HitPolicy::Priority),
      "F" => Ok(HitPolicy::First),
      "R" => Ok(HitPolicy::RuleOrder),
      "O" => Ok(HitPolicy::OutputOrder),
      "C" => Ok(HitPolicy::Collect(BuiltinAggregator::List)),
      "C+" => Ok(HitPolicy::Collect(BuiltinAggregator::Sum)),
      "C#" => Ok(HitPolicy::Collect(BuiltinAggregator::Count)),
      "C<" => Ok(HitPolicy::Collect(BuiltinAggregator::Min)),
      "C>" => Ok(HitPolicy::Collect(BuiltinAggregator::Max)),
      other => Err(err_invalid_decision_table_hit_policy(other)),
    }
  }
}

/// Aggregator function for `COLLECT` [hit policy](HitPolicy).
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BuiltinAggregator {
  /// The result of the decision table is a list of output entries.
  List,
  /// The result of the decision table is the number of outputs.
  Count,
  /// The result of the decision table is the sum of all the outputs.
  Sum,
  /// The result of the decision table is the smallest value of all the outputs.
  Min,
  /// The result of the decision table is the largest value of all the outputs.
  Max,
}

/// Implementation of [Display](fmt::Display) for [BuiltinAggregator].
impl fmt::Display for BuiltinAggregator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        BuiltinAggregator::List => "C",
        BuiltinAggregator::Count => "C#",
        BuiltinAggregator::Sum => "C+",
        BuiltinAggregator::Min => "C<",
        BuiltinAggregator::Max => "C>",
      }
    )
  }
}

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputClause {
  /// The subject of this input clause, text representation of unary tests.
  pub input_expression: String,
  /// Optional unary tests that constrain the result of input expression of this input clause.
  pub allowed_input_values: Option<String>,
}

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputClause {
  /// Type reference may specify the type to be used as decision table's output when more than one output clause is present.
  pub type_ref: Option<String>,
  /// The name of the output component when the decision table contains more than one output clause.
  pub name: Option<String>,
  /// Unary tests that constrain the result of output entries corresponding to this output clause.
  pub allowed_output_values: Option<String>,
  /// Default output expression, selected in incomplete table when no rules match for the decision table.
  pub default_output_entry: Option<String>,
}

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleAnnotationClause {
  /// Name that is used as the name of the rule annotation column of the containing decision table.
  pub name: String,
}

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRule {
  /// Ordered list of input entries that compose this decision rule.
  pub input_entries: Vec<InputEntry>,
  /// Ordered list of output entries that compose this decision rule.
  pub output_entries: Vec<OutputEntry>,
  /// Ordered list of rule annotations that compose this decision rule.
  pub annotation_entries: Vec<AnnotationEntry>,
}

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputEntry {
  /// Text representation of unary test that composes this input entry.
  pub text: String,
}

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputEntry {
  /// Text representation of literal expression that composes this output entry.
  pub text: String,
}

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationEntry {
  /// Text representing this rule annotation.
  pub text: String,
}

/// [Dmndi] is a container for the shared [DmnStyle](DmnStyle)s
/// and all [DmnDiagram](DmnDiagram)s defined in [Definitions].
#[derive(Debug, Clone)]
pub struct Dmndi {
  /// A list of shared [DmnStyle] that can be referenced
  /// by all [DmnDiagram] and [DmnDiagramElement].
  pub styles: Vec<DmnStyle>,
  /// A list of [DmnDiagram].
  pub diagrams: Vec<DmnDiagram>,
}

/// Defines possible elements of [DmnDiagramElement].
#[derive(Debug, Clone)]
pub enum DmnDiagramElement {
  DmnShape(DmnShape),
  DmnEdge(DmnEdge),
}

/// [DmnDiagram] is the container of [DmnDiagramElement] ([DmnShape] (s) and [DmnEdge] (s)).
/// [DmnDiagram] cannot include other [DmnDiagrams](DmnDiagram).
#[derive(Debug, Clone)]
pub struct DmnDiagram {
  /// [DmnDiagram] id.
  pub id: Option<String>,
  /// The name of the diagram. Default is empty [String].
  pub name: String,
  /// The documentation of the diagram. Default is empty [String].
  pub documentation: String,
  /// The resolution of the diagram expressed in user units per inch. Default is 300.
  pub resolution: f64,
  /// A list of [DmnDiagramElement] ([DmnShape] and [DmnEdge]) that are depicted in this diagram.
  pub diagram_elements: Vec<DmnDiagramElement>,
  /// A reference to a [DmnStyle] defined in the [Dmndi] that serves as the default styling
  /// of the [DmnDiagramElement] in this [DmnDiagram].
  pub shared_style: Option<String>,
  /// A [DmnStyle] that defines the default styling for this diagram.
  /// Properties defined in that style override the ones in the 'sharedStyle'.
  pub local_style: Option<DmnStyle>,
  /// The size of this diagram. If not specified, the [DmnDiagram] is unbounded.
  pub size: Option<DcDimension>,
}

/// [DmnShape] represents a [Decision], a [BusinessKnowledgeModel], an [InputData] element,
/// a [KnowledgeSource], a [DecisionService] or a [TextAnnotation] that is depicted on the diagram.
#[derive(Debug, Clone)]
pub struct DmnShape {
  /// [DmnShape] id.
  pub id: Option<String>,
  /// The [DcBounds] of the shape relative to the origin of its parent [DmnDiagram]. The [DcBounds] MUST be specified.
  pub bounds: DcBounds,
  /// A  reference  to  a  [Decision], a [BusinessKnowledgeModel], an [InputData] element,
  /// a [KnowledgeSource], a [DecisionService] or a [TextAnnotation] MUST be specified.
  pub dmn_element_ref: Option<String>,
  /// If the [DmnShape] depicts an [InputData] element then this attribute is used to determine
  /// if the [InputData] is listed on the [Decision] element (true) or drawn as separate notational elements in the DRD (false).
  pub is_listed_input_data: bool,
  /// If the [DmnShape] depicts a [DecisionService], this attribute references a [DmnDecisionServiceDividerLine] that defines,
  /// where the [DmnShape] is divided into two parts by a straight solid line.
  /// This can be the case when a [DmnShape] depicts a [DecisionService],
  /// where the set of output decisions is smaller than the set of encapsulated decisions.
  /// The start and end waypoints of the `decisionServiceDividerLine` **MUST** be on the border of the [DmnShape].
  pub decision_service_divider_line: Option<DmnDecisionServiceDividerLine>,
  /// If the [DmnShape] depicts a [DecisionService], this attribute indicates
  /// if it should be depicted expanded (`false`) or collapsed (`true`).
  /// Default is `false`.
  pub is_collapsed: bool,
  /// A reference to a [DmnStyle] defined in the [Dmndi].
  pub shared_style: Option<String>,
  /// A [DmnStyle] that defines the styling for this element.
  pub local_style: Option<DmnStyle>,
  /// An optional label when this [DmnElement] has a visible text label.
  pub label: Option<DmnLabel>,
}

/// Struct defines line inside [DecisionService].
#[derive(Debug, Clone)]
pub struct DmnDecisionServiceDividerLine {
  pub id: Option<String>,
  /// A list of points relative to the origin of its parent [DmnDiagram] that specifies
  /// the connected line segments of the edge. At least two (2) waypoint`s MUST be specified.
  /// Waypoint must be on the border of the [DmnShape].
  pub way_points: Vec<DcPoint>,
  /// A reference to a [DmnStyle] defined in the [Dmndi].
  pub shared_style: Option<String>,
  /// A [DmnStyle] that defines the styling for this element.
  pub local_style: Option<DmnStyle>,
}

#[derive(Debug, Clone)]
pub struct DmnEdge {
  pub id: Option<String>,
  /// A list of points relative to the origin of its parent [DmnDiagram] that specifies
  /// the connected line segments of the edge. At least two (2) waypoints MUST be specified.
  pub way_points: Vec<DcPoint>,
  /// A reference to a [InformationRequirement], [KnowledgeRequirement],
  /// [AuthorityRequirement] or an [Association], MUST be specified.
  pub dmn_element_ref: Option<String>,
  /// The actual [DmnDiagramElement] this [DmnEdge] is connecting from.
  /// MUST be specified when the [DmnEdge] has a source.
  pub source_element: Option<String>,
  /// The actual [DmnDiagramElement] this [DmnEdge] is connecting to.
  /// MUST be specified when the [DmnEdge] has a target.
  pub target_element: Option<String>,
  /// A reference to a [DmnStyle] defined in the [Dmndi].
  pub shared_style: Option<String>,
  /// A [DmnStyle] that defines the styling for this element.
  pub local_style: Option<DmnStyle>,
  /// An optional label when this [DmnElement] has a visible text label.
  pub label: Option<DmnLabel>,
}

//FIXME verify this struct
/// tdb
#[derive(Debug, Clone)]
pub struct Association {}

//FIXME verify this struct
/// tdb
#[derive(Debug, Clone)]
pub struct TextAnnotation {}

/// [DmnStyle] is used to keep some non-normative visual attributes such as color and font.
#[derive(Debug, Clone)]
pub struct DmnStyle {
  /// A unique id for this style so it can be referenced.
  /// Only styles defined in the [Dmndi] can be referenced by [DmnDiagramElement] and [DmnDiagram].
  pub id: Option<String>,
  /// The color use to fill the shape. Does not apply to [DmnEdge].
  /// Default is `white`.
  pub fill_color: Option<DcColor>,
  /// The color use to draw the shape borders.
  /// Default is `black`.
  pub stroke_color: Option<DcColor>,
  /// The color use to draw the label.
  /// Default is `black`.
  pub font_color: Option<DcColor>,
  /// A comma-separated list of Font Name that can be used to display the text.
  /// Default is `Arial`.
  pub font_family: String,
  /// The size in points of the font to use to display the text.
  /// Default is `8`.
  pub font_size: Option<f64>,
  /// If the text should be displayed in Italic.
  /// Default is `false`.
  pub font_italic: bool,
  /// If the text should be displayed in Bold.
  /// Default is `false`.
  pub font_bold: bool,
  /// If the text should be underlined.
  /// Default is `false`.
  pub font_underline: bool,
  /// If the text should be stroke through.
  /// Default is `false`.
  pub font_strike_through: bool,
  /// How text should be positioned horizontally within the Label bounds.
  /// Default depends of the [DmnDiagramElement] the label is attached to (see 13.5).
  pub label_horizontal_alignment: Option<DcAlignmentKind>,
  /// How  the  text  should  be  positioned  vertically  inside  the  Label  bounds.
  /// Default depends of the [DmnDiagramElement] the label is attached to (see 13.5).
  /// Start means `top` and end means `bottom`.
  pub label_vertical_alignment: Option<DcAlignmentKind>,
}

/// Struct represents the depiction of some textual information about a DMN element.
#[derive(Debug, Clone)]
pub struct DmnLabel {
  /// The bounds of the [DmnLabel]. When not specified, the label is positioned
  /// at its default position as determined in clause 13.5.
  pub bounds: Option<DcBounds>,
  /// An optional pretty printed text that MUST be displayed
  /// instead of the [DmnElement's](DmnElement) name if it is present.
  pub text: Option<String>,
  /// A reference to a [DmnStyle] defined in the [Dmndi].
  pub shared_style: Option<String>,
}

/// Defines RGB color.
#[derive(Debug, Copy, Clone)]
pub struct DcColor {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

/// Defines point.
#[derive(Debug, Copy, Clone)]
pub struct DcPoint {
  pub x: f64,
  pub y: f64,
}

/// Defines bounds.
#[derive(Debug, Copy, Clone)]
pub struct DcBounds {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

/// Defines dimensions.
#[derive(Debug, Copy, Clone)]
pub struct DcDimension {
  pub width: f64,
  pub height: f64,
}

/// Defines the king of element alignment.
#[derive(Debug, Copy, Clone)]
pub enum DcAlignmentKind {
  Start,
  End,
  Center,
}

/// Defines known colors.
#[derive(Debug, Copy, Clone)]
pub enum DcKnownColor {
  Maroon = 0x800000,
  Red = 0xFF0000,
  Orange = 0xFFA500,
  Yellow = 0xFFFF00,
  Olive = 0x808000,
  Purple = 0x800080,
  Fuchsia = 0xFF00FF,
  White = 0xFFFFFF,
  Lime = 0x00FF00,
  Green = 0x008000,
  Navy = 0x000080,
  Blue = 0x0000FF,
  Aqua = 0x00FFFF,
  Teal = 0x008080,
  Black = 0x000000,
  Silver = 0xC0C0C0,
  Gray = 0x808080,
}

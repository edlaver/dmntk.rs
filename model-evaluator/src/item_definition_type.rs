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

//! Builder for item definition type evaluators.

use crate::errors::*;
use crate::model_definitions::{DefDefinitions, DefItemDefinition};
use crate::type_ref::type_ref_to_feel_type;
use dmntk_common::Result;
use dmntk_feel::{FeelType, Name, FEEL_TYPE_NAME_ANY};
use dmntk_model::model::ItemDefinitionType;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};

/// Type of function that evaluates the item definition type.
type ItemDefinitionTypeEvaluatorFn = Box<dyn Fn(&ItemDefinitionTypeEvaluator) -> Option<FeelType> + Send + Sync>;

/// Item definition type evaluators.
#[derive(Default)]
pub struct ItemDefinitionTypeEvaluator {
  evaluators: RefCell<HashMap<String, ItemDefinitionTypeEvaluatorFn>>,
}

impl ItemDefinitionTypeEvaluator {
  /// Creates item definition type evaluators.
  pub fn build(&self, definitions: &DefDefinitions) -> Result<()> {
    for item_definition in definitions.item_definitions() {
      let evaluator = build_item_definition_type_evaluator(item_definition)?;
      let type_ref = item_definition.name().to_string();
      self.evaluators.borrow_mut().insert(type_ref, evaluator);
    }
    Ok(())
  }

  /// Evaluates a type of item definition with specified type reference name.
  pub fn eval(&self, type_ref: &str) -> Option<FeelType> {
    if let Some(evaluator) = self.evaluators.borrow().get(type_ref) {
      evaluator(self)
    } else {
      None
    }
  }

  /// Returns FEEL type for specified type reference.
  pub fn information_item_type(&self, type_ref: &str) -> Option<FeelType> {
    if let Some(feel_type) = type_ref_to_feel_type(type_ref) {
      match feel_type {
        FeelType::String => Some(FeelType::String),
        FeelType::Number => Some(FeelType::Number),
        FeelType::Boolean => Some(FeelType::Boolean),
        FeelType::Date => Some(FeelType::Date),
        FeelType::Time => Some(FeelType::Time),
        FeelType::DateTime => Some(FeelType::DateTime),
        FeelType::DaysAndTimeDuration => Some(FeelType::DaysAndTimeDuration),
        FeelType::YearsAndMonthsDuration => Some(FeelType::YearsAndMonthsDuration),
        _ => None,
      }
    } else {
      self.eval(type_ref)
    }
  }
}

///
pub fn build_item_definition_type_evaluator(item_definition: &DefItemDefinition) -> Result<ItemDefinitionTypeEvaluatorFn> {
  match item_definition.item_definition_type()? {
    ItemDefinitionType::SimpleType(feel_type) => simple_type(feel_type),
    ItemDefinitionType::ReferencedType(ref_type) => referenced_type(ref_type),
    ItemDefinitionType::ComponentType => component_type(item_definition),
    ItemDefinitionType::CollectionOfSimpleType(feel_type) => collection_of_simple_type(feel_type),
    ItemDefinitionType::CollectionOfReferencedType(ref_type) => collection_of_referenced_type(ref_type),
    ItemDefinitionType::CollectionOfComponentType => collection_of_component_type(item_definition),
    ItemDefinitionType::FunctionType => function_type(item_definition),
  }
}

///
fn simple_type(feel_type: FeelType) -> Result<ItemDefinitionTypeEvaluatorFn> {
  match feel_type {
    FeelType::String => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::String))),
    FeelType::Number => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::Number))),
    FeelType::Boolean => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::Boolean))),
    FeelType::Date => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::Date))),
    FeelType::Time => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::Time))),
    FeelType::DateTime => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::DateTime))),
    FeelType::DaysAndTimeDuration => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::DaysAndTimeDuration))),
    FeelType::YearsAndMonthsDuration => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::YearsAndMonthsDuration))),
    _ => Err(err_unsupported_feel_type(feel_type)),
  }
}

///
fn referenced_type(ref_type: String) -> Result<ItemDefinitionTypeEvaluatorFn> {
  Ok(Box::new(move |evaluators: &ItemDefinitionTypeEvaluator| evaluators.eval(&ref_type)))
}

///
fn component_type(item_definition: &DefItemDefinition) -> Result<ItemDefinitionTypeEvaluatorFn> {
  let mut type_evaluators: Vec<(Name, ItemDefinitionTypeEvaluatorFn)> = vec![];
  for component_item_definition in item_definition.item_components() {
    type_evaluators.push((
      component_item_definition.feel_name().clone(),
      build_item_definition_type_evaluator(component_item_definition)?,
    ));
  }
  Ok(Box::new(move |evaluators: &ItemDefinitionTypeEvaluator| {
    let mut entries = BTreeMap::new();
    for (component_name, component_evaluator) in &type_evaluators {
      if let Some(feel_type) = component_evaluator(evaluators) {
        entries.insert(component_name.clone(), feel_type);
      }
    }
    Some(FeelType::Context(entries))
  }))
}

///
fn collection_of_simple_type(feel_type: FeelType) -> Result<ItemDefinitionTypeEvaluatorFn> {
  match feel_type {
    FeelType::String => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::String)))),
    FeelType::Number => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::Number)))),
    FeelType::Boolean => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::Boolean)))),
    FeelType::Date => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::Date)))),
    FeelType::Time => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::Time)))),
    FeelType::DateTime => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::DateTime)))),
    FeelType::DaysAndTimeDuration => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::DaysAndTimeDuration)))),
    FeelType::YearsAndMonthsDuration => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::YearsAndMonthsDuration)))),
    _ => Err(err_unsupported_feel_type(feel_type)),
  }
}

///
fn collection_of_referenced_type(type_ref: String) -> Result<ItemDefinitionTypeEvaluatorFn> {
  Ok(Box::new(move |evaluators: &ItemDefinitionTypeEvaluator| {
    evaluators.eval(&type_ref).map(|feel_type| FeelType::List(Box::new(feel_type)))
  }))
}

///
fn collection_of_component_type(item_definition: &DefItemDefinition) -> Result<ItemDefinitionTypeEvaluatorFn> {
  let mut type_evaluators: Vec<(Name, ItemDefinitionTypeEvaluatorFn)> = vec![];
  for component_item_definition in item_definition.item_components() {
    type_evaluators.push((
      component_item_definition.feel_name().clone(),
      build_item_definition_type_evaluator(component_item_definition)?,
    ));
  }
  Ok(Box::new(move |evaluators: &ItemDefinitionTypeEvaluator| {
    let mut entries = BTreeMap::new();
    for (component_name, component_evaluator) in &type_evaluators {
      if let Some(feel_type) = component_evaluator(evaluators) {
        entries.insert(component_name.clone(), feel_type);
      }
    }
    Some(FeelType::List(Box::new(FeelType::Context(entries))))
  }))
}

///
fn function_type(item_definition: &DefItemDefinition) -> Result<ItemDefinitionTypeEvaluatorFn> {
  let mut output_type_ref = FEEL_TYPE_NAME_ANY.to_string();
  let mut parameters_type_ref = vec![];
  if let Some(function_item) = item_definition.function_item() {
    if let Some(type_ref) = function_item.output_type_ref() {
      output_type_ref = type_ref.to_owned();
    }
    for parameter_information_item in function_item.parameters() {
      parameters_type_ref.push(parameter_information_item.type_ref().as_ref().unwrap_or(&FEEL_TYPE_NAME_ANY.to_string()).clone());
    }
  }
  Ok(Box::new(move |evaluator: &ItemDefinitionTypeEvaluator| {
    let output_type = evaluator.information_item_type(&output_type_ref).unwrap_or(FeelType::Any);
    let parameter_types = parameters_type_ref
      .iter()
      .map(|parameter_type_ref| evaluator.information_item_type(parameter_type_ref).unwrap_or(FeelType::Any))
      .collect::<Vec<FeelType>>();
    Some(FeelType::function(&parameter_types, &output_type))
  }))
}

#[cfg(test)]
mod tests {
  use crate::item_definition_type::ItemDefinitionTypeEvaluator;
  use dmntk_examples::item_definition::*;
  use dmntk_feel::{FeelType, Name};

  /// Utility function for building item definition type evaluator from definitions.
  fn build_evaluator(xml: &str) -> ItemDefinitionTypeEvaluator {
    let item_definition_type_evaluator = ItemDefinitionTypeEvaluator::default();
    item_definition_type_evaluator.build(&dmntk_model::parse(xml).unwrap().into()).unwrap();
    item_definition_type_evaluator
  }

  #[test]
  fn simple_type_string() {
    let evaluator = build_evaluator(DMN_0101);
    assert_eq!(Some(FeelType::String), evaluator.eval("tCustomerName"));
  }

  #[test]
  fn simple_type_number() {
    let evaluator = build_evaluator(DMN_0102);
    assert_eq!(Some(FeelType::Number), evaluator.eval("tMonthlySalary"));
  }

  #[test]
  fn simple_type_boolean() {
    let evaluator = build_evaluator(DMN_0103);
    assert_eq!(Some(FeelType::Boolean), evaluator.eval("tIsAffordable"));
  }

  #[test]
  fn simple_type_date() {
    let evaluator = build_evaluator(DMN_0104);
    assert_eq!(Some(FeelType::Date), evaluator.eval("tBirthday"));
  }

  #[test]
  fn simple_type_time() {
    let evaluator = build_evaluator(DMN_0105);
    assert_eq!(Some(FeelType::Time), evaluator.eval("tDeliveryTime"));
  }

  #[test]
  fn simple_type_date_time() {
    let evaluator = build_evaluator(DMN_0106);
    assert_eq!(Some(FeelType::DateTime), evaluator.eval("tAppointment"));
  }

  #[test]
  fn simple_type_days_and_time_duration() {
    let evaluator = build_evaluator(DMN_0107);
    assert_eq!(Some(FeelType::DaysAndTimeDuration), evaluator.eval("tCourseDuration"));
  }

  #[test]
  fn simple_type_years_and_month_duration() {
    let evaluator = build_evaluator(DMN_0108);
    assert_eq!(Some(FeelType::YearsAndMonthsDuration), evaluator.eval("tGrowthDuration"));
  }

  #[test]
  fn referenced_type_string() {
    let evaluator = build_evaluator(DMN_0201);
    assert_eq!(Some(FeelType::String), evaluator.eval("tCustomerName"));
  }

  #[test]
  fn referenced_type_number() {
    let evaluator = build_evaluator(DMN_0202);
    assert_eq!(Some(FeelType::Number), evaluator.eval("tMonthlySalary"));
  }

  #[test]
  fn component_type() {
    let evaluator = build_evaluator(DMN_0301);
    let name_principal: Name = "principal".into();
    let name_rate: Name = "rate".into();
    let name_term_months: Name = "termMonths".into();
    let type_number = FeelType::Number;
    let component_type = FeelType::context(&[(&name_principal, &type_number), (&name_rate, &type_number), (&name_term_months, &type_number)]);
    assert_eq!(Some(component_type), evaluator.eval("tLoan"));
  }

  #[test]
  fn collection_of_simple_type_string() {
    let evaluator = build_evaluator(DMN_0401);
    assert_eq!(Some(FeelType::list(&FeelType::String)), evaluator.eval("tItems"));
  }

  #[test]
  fn collection_of_simple_type_number() {
    let evaluator = build_evaluator(DMN_0402);
    assert_eq!(Some(FeelType::list(&FeelType::Number)), evaluator.eval("tItems"));
  }

  #[test]
  fn collection_of_simple_type_boolean() {
    let evaluator = build_evaluator(DMN_0403);
    assert_eq!(Some(FeelType::list(&FeelType::Boolean)), evaluator.eval("tItems"));
  }

  #[test]
  fn collection_of_simple_type_date() {
    let evaluator = build_evaluator(DMN_0404);
    assert_eq!(Some(FeelType::list(&FeelType::Date)), evaluator.eval("tItems"));
  }

  #[test]
  fn collection_of_simple_type_time() {
    let evaluator = build_evaluator(DMN_0405);
    assert_eq!(Some(FeelType::list(&FeelType::Time)), evaluator.eval("tItems"));
  }

  #[test]
  fn collection_of_simple_type_date_time() {
    let evaluator = build_evaluator(DMN_0406);
    assert_eq!(Some(FeelType::list(&FeelType::DateTime)), evaluator.eval("tItems"));
  }

  #[test]
  fn collection_of_simple_type_days_and_time_duration() {
    let evaluator = build_evaluator(DMN_0407);
    assert_eq!(Some(FeelType::list(&FeelType::DaysAndTimeDuration)), evaluator.eval("tItems"));
  }

  #[test]
  fn collection_of_simple_type_years_and_months_duration() {
    let evaluator = build_evaluator(DMN_0408);
    assert_eq!(Some(FeelType::list(&FeelType::YearsAndMonthsDuration)), evaluator.eval("tItems"));
  }

  #[test]
  fn test_evaluate_input_data_0501_1() {
    let evaluator = build_evaluator(DMN_0501);
    assert_eq!(Some(FeelType::list(&FeelType::String)), evaluator.eval("tItems"));
  }

  #[test]
  fn test_evaluate_input_data_0601_1() {
    let evaluator = build_evaluator(DMN_0601);
    let name_number: Name = "number".into();
    let name_name: Name = "name".into();
    let name_manager: Name = "manager".into();
    let type_number = FeelType::Number;
    let type_string = FeelType::String;
    let component_type = FeelType::context(&[(&name_number, &type_number), (&name_name, &type_string), (&name_manager, &type_string)]);
    let list_type = FeelType::list(&component_type);
    assert_eq!(Some(list_type), evaluator.eval("tItems"));
  }
}
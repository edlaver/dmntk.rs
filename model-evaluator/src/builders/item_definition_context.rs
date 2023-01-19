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

//! Builder for item definition context evaluators.

use crate::errors::*;
use crate::model_definitions::{DefDefinitions, DefItemDefinition};
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::{Value, Values};
use dmntk_feel::{FeelType, Name};
use dmntk_model::model::ItemDefinitionType;
use std::collections::{BTreeMap, HashMap};

/// Type of closure that evaluates the item definition context.
type ItemDefinitionContextEvaluatorFn = Box<dyn Fn(&Name, &mut FeelContext, &ItemDefinitionContextEvaluator) -> FeelType + Send + Sync>;

/// Item definition type evaluators.
#[derive(Default)]
pub struct ItemDefinitionContextEvaluator {
  evaluators: HashMap<String, ItemDefinitionContextEvaluatorFn>,
}

impl ItemDefinitionContextEvaluator {
  /// Creates item definition type evaluators.
  pub fn build(&mut self, definitions: &DefDefinitions) -> Result<()> {
    for item_definition in definitions.item_definitions() {
      let evaluator = item_definition_context_evaluator(item_definition)?;
      let type_ref = item_definition.name().to_string();
      self.evaluators.insert(type_ref, evaluator);
    }
    Ok(())
  }
  /// Evaluates a context from item definition with specified type reference name.
  pub fn eval(&self, type_ref: &str, name: &Name, ctx: &mut FeelContext) -> FeelType {
    if let Some(evaluator) = self.evaluators.get(type_ref) {
      evaluator(name, ctx, self)
    } else {
      FeelType::Any
    }
  }
}

///
fn item_definition_context_evaluator(item_definition: &DefItemDefinition) -> Result<ItemDefinitionContextEvaluatorFn> {
  match item_definition.item_definition_type()? {
    ItemDefinitionType::SimpleType(feel_type) => simple_type_context_evaluator(feel_type),
    ItemDefinitionType::ReferencedType(ref_type) => referenced_type_context_evaluator(ref_type),
    ItemDefinitionType::ComponentType => component_type_context_evaluator(item_definition),
    ItemDefinitionType::CollectionOfSimpleType(feel_type) => collection_of_simple_type_context_evaluator(feel_type),
    ItemDefinitionType::CollectionOfReferencedType(ref_type) => collection_of_referenced_type_context_evaluator(ref_type),
    ItemDefinitionType::CollectionOfComponentType => collection_of_component_type_context_evaluator(item_definition),
    ItemDefinitionType::FunctionType => function_type_context_evaluator(item_definition),
  }
}

///
fn simple_type_context_evaluator(feel_type: FeelType) -> Result<ItemDefinitionContextEvaluatorFn> {
  if matches!(
    feel_type,
    FeelType::String
      | FeelType::Number
      | FeelType::Boolean
      | FeelType::Date
      | FeelType::Time
      | FeelType::DateTime
      | FeelType::DaysAndTimeDuration
      | FeelType::YearsAndMonthsDuration
  ) {
    Ok(Box::new(move |name: &Name, ctx: &mut FeelContext, _: &ItemDefinitionContextEvaluator| {
      ctx.set_entry(name, Value::FeelType(feel_type.clone()));
      feel_type.clone()
    }))
  } else {
    Err(err_unsupported_feel_type(feel_type))
  }
}

///
fn referenced_type_context_evaluator(ref_type: String) -> Result<ItemDefinitionContextEvaluatorFn> {
  Ok(Box::new(move |name: &Name, ctx: &mut FeelContext, evaluator: &ItemDefinitionContextEvaluator| {
    evaluator.eval(&ref_type, name, ctx)
  }))
}

///
fn component_type_context_evaluator(item_definition: &DefItemDefinition) -> Result<ItemDefinitionContextEvaluatorFn> {
  let mut context_evaluators = vec![];
  for component_item_definition in item_definition.item_components() {
    context_evaluators.push((component_item_definition.feel_name().clone(), item_definition_context_evaluator(component_item_definition)?));
  }
  Ok(Box::new(move |name: &Name, ctx: &mut FeelContext, evaluator: &ItemDefinitionContextEvaluator| {
    let mut entries = BTreeMap::new();
    let mut evaluated_ctx = FeelContext::default();
    for (component_name, component_evaluator) in &context_evaluators {
      let feel_type = component_evaluator(component_name, &mut evaluated_ctx, evaluator);
      entries.insert(component_name.clone(), feel_type);
    }
    ctx.set_entry(name, Value::Context(evaluated_ctx));
    FeelType::Context(entries)
  }))
}

///
fn collection_of_simple_type_context_evaluator(feel_type: FeelType) -> Result<ItemDefinitionContextEvaluatorFn> {
  if matches!(
    feel_type,
    FeelType::String
      | FeelType::Number
      | FeelType::Boolean
      | FeelType::Date
      | FeelType::Time
      | FeelType::DateTime
      | FeelType::DaysAndTimeDuration
      | FeelType::YearsAndMonthsDuration
  ) {
    Ok(Box::new(move |name: &Name, ctx: &mut FeelContext, _: &ItemDefinitionContextEvaluator| {
      let list_type = FeelType::List(Box::new(feel_type.clone()));
      let list = Value::List(Values::new(vec![Value::FeelType(feel_type.clone())]));
      ctx.set_entry(name, list);
      list_type
    }))
  } else {
    Err(err_unsupported_feel_type(feel_type))
  }
}

///
fn collection_of_referenced_type_context_evaluator(type_ref: String) -> Result<ItemDefinitionContextEvaluatorFn> {
  Ok(Box::new(move |name: &Name, ctx: &mut FeelContext, evaluator: &ItemDefinitionContextEvaluator| {
    let mut evaluated_ctx = FeelContext::default();
    let feel_type = evaluator.eval(&type_ref, name, &mut evaluated_ctx);
    let list_type = FeelType::List(Box::new(feel_type.clone()));
    let list = Value::List(Values::new(vec![Value::FeelType(feel_type)]));
    ctx.set_entry(name, list);
    list_type
  }))
}

///
fn collection_of_component_type_context_evaluator(item_definition: &DefItemDefinition) -> Result<ItemDefinitionContextEvaluatorFn> {
  let mut context_evaluators = vec![];
  for component_item_definition in item_definition.item_components() {
    context_evaluators.push((component_item_definition.feel_name().clone(), item_definition_context_evaluator(component_item_definition)?));
  }
  Ok(Box::new(move |name: &Name, ctx: &mut FeelContext, evaluator: &ItemDefinitionContextEvaluator| {
    let mut entries = BTreeMap::new();
    let mut evaluated_ctx = FeelContext::default();
    for (component_name, component_evaluator) in &context_evaluators {
      let feel_type = component_evaluator(component_name, &mut evaluated_ctx, evaluator);
      entries.insert(component_name.clone(), feel_type);
    }
    let list_type = FeelType::List(Box::new(FeelType::Context(entries)));
    let list = Value::List(Values::new(vec![Value::Context(evaluated_ctx)]));
    ctx.set_entry(name, list);
    list_type
  }))
}

///
fn function_type_context_evaluator(_item_definition: &DefItemDefinition) -> Result<ItemDefinitionContextEvaluatorFn> {
  Ok(Box::new(move |_name: &Name, _ctx: &mut FeelContext, _: &ItemDefinitionContextEvaluator| FeelType::Any))
  //TODO implement function type
}

#[cfg(test)]
mod tests {
  use crate::builders::item_definition_context::ItemDefinitionContextEvaluator;
  use dmntk_examples::item_definition::*;
  use dmntk_feel::context::FeelContext;
  use dmntk_feel::values::{Value, Values};
  use dmntk_feel::{FeelType, Name};

  /// Utility function for building item definition evaluator from definitions.
  fn build_evaluator(xml: &str) -> ItemDefinitionContextEvaluator {
    let mut evaluator = ItemDefinitionContextEvaluator::default();
    evaluator.build(&dmntk_model::parse(xml).unwrap().into()).unwrap();
    evaluator
  }

  #[test]
  fn simple_type_string() {
    let evaluator = build_evaluator(DMN_0101);
    let mut ctx = FeelContext::default();
    let expected_type = FeelType::String;
    let mut expected_context = FeelContext::default();
    let variable_name: Name = "Customer Name".into();
    expected_context.set_entry(&variable_name, Value::FeelType(FeelType::String));
    let actual_type = evaluator.eval("tCustomerName", &variable_name, &mut ctx);
    assert_eq!(expected_type, actual_type);
    assert_eq!(expected_context, ctx);
    assert_eq!("{Customer Name: type(string)}", ctx.to_string());
  }

  #[test]
  fn simple_type_number() {
    // let evaluator = build_evaluator(DMN_0102);
    // let mut ctx = FeelContext::default();
    // let expected_type = FeelType::Number;
    // let mut expected_context = FeelContext::default();
    // expected_context.set_entry(&"Monthly Salary".into(), Value::FeelType(FeelType::Number));
    // let actual_type = evaluator.eval("tMonthlySalary", &"Monthly Salary".into(), &mut ctx);
    // assert_eq!(expected_type, actual_type);
    // assert_eq!(expected_context, ctx);
    // assert_eq!("{Monthly Salary: type(number)}", ctx.to_string());
  }
  /*
      #[test]
      fn simple_type_boolean() {
        let definitions = &dmntk_model::parse(DMN_0103).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::Boolean), evaluators.eval("tIsAffordable"));
      }

      #[test]
      fn simple_type_date() {
        let definitions = &dmntk_model::parse(DMN_0104).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::Date), evaluators.eval("tBirthday"));
      }

      #[test]
      fn simple_type_time() {
        let definitions = &dmntk_model::parse(DMN_0105).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::Time), evaluators.eval("tDeliveryTime"));
      }

      #[test]
      fn simple_type_date_time() {
        let definitions = &dmntk_model::parse(DMN_0106).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::DateTime), evaluators.eval("tAppointment"));
      }

      #[test]
      fn simple_type_days_and_time_duration() {
        let definitions = &dmntk_model::parse(DMN_0107).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::DaysAndTimeDuration), evaluators.eval("tCourseDuration"));
      }

      #[test]
      fn simple_type_years_and_month_duration() {
        let definitions = &dmntk_model::parse(DMN_0108).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::YearsAndMonthsDuration), evaluators.eval("tGrowthDuration"));
      }
  */

  #[test]
  fn referenced_type_string() {
    // let evaluator = build_evaluator(DMN_0201);
    // let mut ctx = FeelContext::default();
    // let expected_type = FeelType::String;
    // let mut expected_context = FeelContext::default();
    // expected_context.set_entry(&"Customer Name".into(), Value::FeelType(FeelType::String));
    // let actual_type = evaluator.eval("tCustomerName", &"Customer Name".into(), &mut ctx);
    // assert_eq!(expected_type, actual_type);
    // assert_eq!(expected_context, ctx);
    // assert_eq!("{Customer Name: type(string)}", ctx.to_string());
  }

  /*
  #[test]
  fn referenced_type_number() {
    let definitions = &dmntk_model::parse(DMN_0202).unwrap();
    let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
    assert_eq!(Some(FeelType::Number), evaluators.eval("tMonthlySalary"));
  }
  */

  #[test]
  fn component_type() {
    // let evaluator = build_evaluator(DMN_0301);
    // let mut ctx = FeelContext::default();
    // let name_principal: Name = "principal".into();
    // let name_rate: Name = "rate".into();
    // let name_term_months: Name = "termMonths".into();
    // let type_number = FeelType::Number;
    // let expected_type = FeelType::context(&[(&name_principal, &type_number), (&name_rate, &type_number), (&name_term_months, &type_number)]);
    // let mut inner_context = FeelContext::default();
    // inner_context.set_entry(&name_principal, Value::FeelType(type_number.clone()));
    // inner_context.set_entry(&name_rate, Value::FeelType(type_number.clone()));
    // inner_context.set_entry(&name_term_months, Value::FeelType(type_number));
    // let mut expected_context = FeelContext::default();
    // expected_context.set_entry(&"Loan".into(), Value::Context(inner_context));
    // let actual_type = evaluator.eval("tLoan", &"Loan".into(), &mut ctx);
    // assert_eq!(expected_type, actual_type);
    // assert_eq!(expected_context, ctx);
    // assert_eq!("{Loan: {principal: type(number), rate: type(number), termMonths: type(number)}}", ctx.to_string());
  }

  #[test]
  fn collection_of_simple_type_string() {
    let evaluator = build_evaluator(DMN_0401);
    let mut ctx = FeelContext::default();
    let expected_type = FeelType::List(Box::new(FeelType::String));
    let mut expected_context = FeelContext::default();
    let variable_name: Name = "Items".into();
    expected_context.set_entry(&variable_name, Value::List(Values::new(vec![Value::FeelType(FeelType::String)])));
    let actual_type = evaluator.eval("tItems", &variable_name, &mut ctx);
    assert_eq!(expected_type, actual_type);
    assert_eq!(expected_context, ctx);
    assert_eq!("{Items: [type(string)]}", ctx.to_string());
  }

  /*
      #[test]
      fn collection_of_simple_type_number() {
        let definitions = &dmntk_model::parse(DMN_0402).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::Number)), evaluators.eval("tItems"));
      }

      #[test]
      fn collection_of_simple_type_boolean() {
        let definitions = &dmntk_model::parse(DMN_0403).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::Boolean)), evaluators.eval("tItems"));
      }

      #[test]
      fn collection_of_simple_type_date() {
        let definitions = &dmntk_model::parse(DMN_0404).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::Date)), evaluators.eval("tItems"));
      }

      #[test]
      fn collection_of_simple_type_time() {
        let definitions = &dmntk_model::parse(DMN_0405).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::Time)), evaluators.eval("tItems"));
      }

      #[test]
      fn collection_of_simple_type_date_time() {
        let definitions = &dmntk_model::parse(DMN_0406).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::DateTime)), evaluators.eval("tItems"));
      }

      #[test]
      fn collection_of_simple_type_days_and_time_duration() {
        let definitions = &dmntk_model::parse(DMN_0407).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::DaysAndTimeDuration)), evaluators.eval("tItems"));
      }

      #[test]
      fn collection_of_simple_type_years_and_months_duration() {
        let definitions = &dmntk_model::parse(DMN_0408).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::YearsAndMonthsDuration)), evaluators.eval("tItems"));
      }

  */
  #[test]
  fn collection_of_referenced_type_string() {
    // let evaluator = build_evaluator(DMN_0501);
    // let mut ctx = FeelContext::default();
    // let expected_type = FeelType::List(Box::new(FeelType::String));
    // let mut expected_context = FeelContext::default();
    // expected_context.set_entry(&"Items".into(), Value::List(Values::new(vec![Value::FeelType(FeelType::String)])));
    // let actual_type = evaluator.eval("tItems", &"Items".into(), &mut ctx);
    // assert_eq!(expected_type, actual_type);
    // assert_eq!(expected_context, ctx);
    // assert_eq!("{Items: [type(string)]}", ctx.to_string());
  }

  #[test]
  fn collection_of_component_type() {
    // let evaluator = build_evaluator(DMN_0601);
    // let mut ctx = FeelContext::default();
    // let name_manager: Name = "manager".into();
    // let name_name: Name = "name".into();
    // let name_number: Name = "number".into();
    // let type_number = FeelType::Number;
    // let type_string = FeelType::String;
    // let expected_type = FeelType::list(&FeelType::context(&[
    //   (&name_manager, &type_string),
    //   (&name_name, &type_string),
    //   (&name_number, &type_number),
    // ]));
    // let mut inner_context = FeelContext::default();
    // inner_context.set_entry(&name_manager, Value::FeelType(type_string.clone()));
    // inner_context.set_entry(&name_name, Value::FeelType(type_string));
    // inner_context.set_entry(&name_number, Value::FeelType(type_number));
    // let mut expected_context = FeelContext::default();
    // expected_context.set_entry(&"Items".into(), Value::List(Values::new(vec![Value::Context(inner_context)])));
    // let actual_type = evaluator.eval("tItems", &"Items".into(), &mut ctx);
    // assert_eq!("{Items: [{manager: type(string), name: type(string), number: type(number)}]}", ctx.to_string());
    // assert_eq!(expected_type, actual_type);
    // assert_eq!(expected_context, ctx);
  }
}

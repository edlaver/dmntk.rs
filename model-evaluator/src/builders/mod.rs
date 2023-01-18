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

mod business_knowledge_model;
mod decision;
mod decision_service;
pub(crate) mod decision_table;
mod input_data;
mod input_data_context;
mod item_definition;
mod item_definition_context;
mod item_definition_type;
mod variable;

use crate::errors::*;
use crate::ModelEvaluator;
pub use business_knowledge_model::BusinessKnowledgeModelEvaluator;
pub use decision::DecisionEvaluator;
pub use decision_service::DecisionServiceEvaluator;
use dmntk_common::Result;
use dmntk_feel::closure::Closure;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::{Value, Values};
use dmntk_feel::{value_null, Evaluator, FeelScope, FeelType, FunctionBody};
use dmntk_feel_parser::ClosureBuilder;
use dmntk_model::model::*;
use dmntk_model::{DefDefinitions, DefItemDefinition};
pub use input_data::InputDataEvaluator;
pub use input_data_context::InputDataContextEvaluator;
pub use item_definition::ItemDefinitionEvaluator;
pub use item_definition_context::ItemDefinitionContextEvaluator;
pub use item_definition_type::ItemDefinitionTypeEvaluator;
use std::sync::Arc;

///
pub fn information_item_type(type_ref: &str, evaluator: &ItemDefinitionTypeEvaluator) -> Option<FeelType> {
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
    evaluator.eval(type_ref)
  }
}

///
fn item_definition_type(item_definition: &DefItemDefinition) -> Result<ItemDefinitionType> {
  let feel_type = if let Some(type_ref) = item_definition.type_ref() {
    type_ref_to_feel_type(type_ref)
  } else {
    None
  };
  let condition = (
    item_definition.type_ref().is_some(),
    feel_type.is_some(),
    !item_definition.item_components().is_empty(),
    item_definition.is_collection(),
    item_definition.function_item().is_some(),
  );
  match condition {
    (_, true, false, false, false) => Ok(ItemDefinitionType::SimpleType(feel_type.unwrap())),
    (true, false, false, false, false) => Ok(ItemDefinitionType::ReferencedType(item_definition.type_ref().as_ref().unwrap().clone())),
    (false, false, true, false, false) => Ok(ItemDefinitionType::ComponentType),
    (_, true, false, true, false) => Ok(ItemDefinitionType::CollectionOfSimpleType(feel_type.unwrap())),
    (false, false, true, true, false) => Ok(ItemDefinitionType::CollectionOfComponentType),
    (true, false, false, true, false) => Ok(ItemDefinitionType::CollectionOfReferencedType(item_definition.type_ref().as_ref().unwrap().clone())),
    (false, false, false, false, true) => Ok(ItemDefinitionType::FunctionType),
    _ => Err(err_invalid_item_definition_type(item_definition.name())),
  }
}

///
fn type_ref_to_feel_type(type_ref: &str) -> Option<FeelType> {
  match type_ref.trim() {
    "string" => Some(FeelType::String),
    "number" => Some(FeelType::Number),
    "boolean" => Some(FeelType::Boolean),
    "date" => Some(FeelType::Date),
    "time" => Some(FeelType::Time),
    "dateTime" => Some(FeelType::DateTime),
    "dayTimeDuration" => Some(FeelType::DaysAndTimeDuration),
    "yearMonthDuration" => Some(FeelType::YearsAndMonthsDuration),
    _ => None,
  }
}

///
fn build_expression_instance_evaluator(scope: &FeelScope, expression_instance: &ExpressionInstance, model_evaluator: &ModelEvaluator) -> Result<(Evaluator, Closure)> {
  match expression_instance {
    ExpressionInstance::Context(context) => build_context_evaluator(scope, context, model_evaluator),
    ExpressionInstance::DecisionTable(decision_table) => build_decision_table_evaluator(scope, decision_table),
    ExpressionInstance::FunctionDefinition(function_definition) => build_function_definition_evaluator(scope, function_definition, model_evaluator),
    ExpressionInstance::Invocation(invocation) => build_invocation_evaluator(scope, invocation, model_evaluator),
    ExpressionInstance::LiteralExpression(literal_expression) => build_literal_expression_evaluator(scope, literal_expression),
    ExpressionInstance::Relation(relation) => build_relation_evaluator(scope, relation, model_evaluator),
  }
}

///
fn build_context_evaluator(scope: &FeelScope, context: &Context, model_evaluator: &ModelEvaluator) -> Result<(Evaluator, Closure)> {
  let item_definition_type_evaluator = model_evaluator.item_definition_type_evaluator()?;
  let mut entry_evaluators = vec![];
  scope.push(FeelContext::default());
  for context_entry in context.context_entries() {
    if let Some(variable) = &context_entry.variable {
      let variable_name = variable.feel_name();
      let variable_type = if let Some(type_ref) = variable.type_ref() {
        if type_ref == "Any" {
          FeelType::Any
        } else {
          information_item_type(type_ref, &item_definition_type_evaluator).ok_or_else(err_empty_feel_type)?
        }
      } else {
        FeelType::Any
      };
      let (evaluator, _) = build_expression_instance_evaluator(scope, &context_entry.value, model_evaluator)?;
      scope.set_name(variable_name.clone());
      entry_evaluators.push((Some(variable_name.clone()), variable_type, evaluator));
    } else {
      let (evaluator, _) = build_expression_instance_evaluator(scope, &context_entry.value, model_evaluator)?;
      entry_evaluators.push((None, FeelType::Any, evaluator));
    }
  }
  scope.pop();
  let context_evaluator = Box::new(move |scope: &FeelScope| {
    let mut evaluated_context = FeelContext::default();
    for (opt_variable_name, variable_type, evaluator) in &entry_evaluators {
      let value = evaluator(scope) as Value;
      let coerced_value = variable_type.coerced(&value);
      match opt_variable_name {
        Some(variable_name) => {
          scope.set_value(variable_name, coerced_value.clone());
          evaluated_context.set_entry(variable_name, coerced_value);
        }
        None => return coerced_value,
      }
    }
    Value::Context(evaluated_context)
  });
  Ok((context_evaluator, Closure::default()))
}

///
fn build_decision_table_evaluator(scope: &FeelScope, decision_table: &DecisionTable) -> Result<(Evaluator, Closure)> {
  let evaluator = decision_table::build_decision_table_evaluator(scope, decision_table)?;
  let decision_table_evaluator = Box::new(move |scope: &FeelScope| evaluator(scope));
  Ok((decision_table_evaluator, Closure::default()))
}

///
fn build_function_definition_evaluator(scope: &FeelScope, function_definition: &FunctionDefinition, model_evaluator: &ModelEvaluator) -> Result<(Evaluator, Closure)> {
  let item_definition_type_evaluator = model_evaluator.item_definition_type_evaluator()?;
  // resolve function definition's formal parameters
  let mut parameters = vec![];
  let mut parameters_ctx = FeelContext::default();
  for parameter in function_definition.formal_parameters() {
    let parameter_name = parameter.feel_name().clone();
    let parameter_type = if let Some(type_ref) = parameter.type_ref() {
      information_item_type(type_ref, &item_definition_type_evaluator).ok_or_else(err_empty_feel_type)?
    } else {
      FeelType::Any
    };
    parameters_ctx.set_entry(&parameter_name, Value::FeelType(parameter_type.clone()));
    parameters.push((parameter_name, parameter_type));
  }
  // resolve function definition's result type
  let result_type = if let Some(type_ref) = function_definition.type_ref() {
    information_item_type(type_ref, &item_definition_type_evaluator).ok_or_else(err_empty_feel_type)?
  } else {
    FeelType::Any
  };
  // check if the function is external
  match function_definition.kind() {
    FunctionKind::Feel => {
      // prepare function definition's body evaluator
      let body_expression_instance = function_definition.body().as_ref().ok_or_else(err_empty_function_body)?;
      scope.push(parameters_ctx);
      let (body_evaluator, mut closure) = build_expression_instance_evaluator(scope, body_expression_instance, model_evaluator)?;
      for (name, _) in &parameters {
        closure.remove(name.clone());
      }
      scope.pop();
      let function_body_evaluator = Arc::new(body_evaluator);
      let function_body = FunctionBody::LiteralExpression(function_body_evaluator);
      let function_definition_closure = closure.clone();
      // prepare the evaluator
      let function_definition_evaluator = Box::new(move |scope: &FeelScope| {
        let mut closure_ctx = FeelContext::default();
        for closure_name in closure.iter() {
          if let Some(closure_value) = scope.search_entry(closure_name) {
            closure_ctx.create_entry(closure_name, closure_value);
          }
        }
        Value::FunctionDefinition(parameters.clone(), function_body.clone(), false, closure.clone(), closure_ctx, result_type.clone())
      });
      Ok((function_definition_evaluator, function_definition_closure))
    }
    FunctionKind::Java => {
      let body_expression_instance = function_definition.body().as_ref().ok_or_else(err_empty_function_body)?;
      scope.push(parameters_ctx);
      let (body_evaluator, _) = build_expression_instance_evaluator(scope, body_expression_instance, model_evaluator)?;
      scope.pop();
      let function_definition_evaluator = Box::new(move |scope: &FeelScope| {
        if let Value::Context(java_mapping) = body_evaluator(scope) {
          if let Some(Value::String(class_name)) = java_mapping.get_entry(&"class".into()) {
            if let Some(Value::String(method_signature)) = java_mapping.get_entry(&"method signature".into()) {
              let java_class_name = class_name.to_owned();
              let java_method_signature = method_signature.to_owned();
              let java_evaluator = Box::new(move |_: &FeelScope| Value::ExternalJavaFunction(java_class_name.clone(), java_method_signature.clone())) as Evaluator;
              let function_body_evaluator = Arc::new(java_evaluator);
              let function_body = FunctionBody::External(function_body_evaluator);
              Value::FunctionDefinition(parameters.clone(), function_body, true, Closure::default(), FeelContext::default(), result_type.clone())
            } else {
              value_null!("invalid Java function mapping, no method signature entry in context {}", java_mapping)
            }
          } else {
            value_null!("invalid Java function mapping, no class name entry in context {}", java_mapping)
          }
        } else {
          value_null!("expected context as external function mapping")
        }
      });
      Ok((function_definition_evaluator, Closure::default()))
    }
    FunctionKind::Pmml => {
      let body_expression_instance = function_definition.body().as_ref().ok_or_else(err_empty_function_body)?;
      scope.push(parameters_ctx);
      let (body_evaluator, _) = build_expression_instance_evaluator(scope, body_expression_instance, model_evaluator)?;
      scope.pop();
      let function_definition_evaluator = Box::new(move |scope: &FeelScope| {
        if let Value::Context(pmml_mapping) = body_evaluator(scope) {
          if let Some(Value::String(document)) = pmml_mapping.get_entry(&"document".into()) {
            if let Some(Value::String(model_name)) = pmml_mapping.get_entry(&"model".into()) {
              let pmml_document = document.to_owned();
              let pmml_model_name = model_name.to_owned();
              let pmml_evaluator = Box::new(move |_: &FeelScope| Value::ExternalPmmlFunction(pmml_document.clone(), pmml_model_name.clone())) as Evaluator;
              let function_body_evaluator = Arc::new(pmml_evaluator);
              let function_body = FunctionBody::External(function_body_evaluator);
              Value::FunctionDefinition(parameters.clone(), function_body, true, Closure::default(), FeelContext::default(), result_type.clone())
            } else {
              value_null!("invalid PMML function mapping, no model name entry in context {}", pmml_mapping)
            }
          } else {
            value_null!("invalid PMML function mapping, no document entry in context {}", pmml_mapping)
          }
        } else {
          value_null!("expected context as external function mapping")
        }
      });
      Ok((function_definition_evaluator, Closure::default()))
    }
  }
}

///
fn build_invocation_evaluator(scope: &FeelScope, invocation: &Invocation, model_evaluator: &ModelEvaluator) -> Result<(Evaluator, Closure)> {
  let item_definition_type_evaluator = model_evaluator.item_definition_type_evaluator()?;
  let mut bindings = vec![];
  let (function_evaluator, _) = build_expression_instance_evaluator(scope, invocation.called_function(), model_evaluator)?;
  for binding in invocation.bindings() {
    if let Some(binding_formula) = binding.binding_formula() {
      let param_name = binding.parameter().feel_name().clone();
      let param_type = if let Some(type_ref) = binding.parameter().type_ref() {
        information_item_type(type_ref, &item_definition_type_evaluator).ok_or_else(err_empty_feel_type)?
      } else {
        FeelType::Any
      };
      let (evaluator, _) = build_expression_instance_evaluator(scope, binding_formula, model_evaluator)?;
      bindings.push((param_name, param_type, evaluator));
    }
  }
  let invocation_evaluator = Box::new(move |scope: &FeelScope| {
    let mut params_ctx = FeelContext::default();
    bindings.iter().for_each(|(param_name, param_type, evaluator)| {
      let param_value = evaluator(scope) as Value;
      params_ctx.set_entry(param_name, param_type.coerced(&param_value))
    });
    if let Value::FunctionDefinition(_, body, false, _, closure_ctx, result_type) = function_evaluator(scope) {
      scope.push(closure_ctx);
      scope.push(params_ctx);
      let value = body.evaluate(scope);
      scope.pop(); // params_ctx
      scope.pop(); // closure_ctx
      result_type.coerced(&value)
    } else {
      value_null!("expected Value::FunctionDefinition in invocation evaluator")
    }
  });
  Ok((invocation_evaluator, Closure::default()))
}

///
fn build_literal_expression_evaluator(scope: &FeelScope, literal_expression: &LiteralExpression) -> Result<(Evaluator, Closure)> {
  let text = literal_expression.text().as_ref().ok_or_else(err_empty_literal_expression)?;
  let node = dmntk_feel_parser::parse_expression(scope, text, false)?;
  let closure = ClosureBuilder::from_node(&node);
  let literal_expression_evaluator = dmntk_feel_evaluator::prepare(&node)?;
  Ok((literal_expression_evaluator, closure))
}

///
fn build_relation_evaluator(scope: &FeelScope, relation: &Relation, model_evaluator: &ModelEvaluator) -> Result<(Evaluator, Closure)> {
  let mut rows = vec![];
  for row in relation.rows() {
    let mut evaluators = vec![];
    for (i, element) in row.elements().iter().enumerate() {
      if let Some(column) = relation.columns().get(i) {
        let name = column.feel_name().clone();
        let (evaluator, _) = build_expression_instance_evaluator(scope, element, model_evaluator)?;
        evaluators.push((name, evaluator));
      }
    }
    rows.push(evaluators);
  }
  let relation_evaluator = Box::new(move |scope: &FeelScope| {
    let mut results = vec![];
    for row in &rows {
      let mut evaluated_context = FeelContext::default();
      for (name, evaluator) in row {
        evaluated_context.set_entry(name, evaluator(scope));
      }
      results.push(Value::Context(evaluated_context));
    }
    Value::List(Values::new(results))
  });
  Ok((relation_evaluator, Closure::default()))
}

///
fn bring_knowledge_requirements_into_context(definitions: &DefDefinitions, knowledge_requirements: &[KnowledgeRequirement], ctx: &mut FeelContext) -> Result<()> {
  for knowledge_requirement in knowledge_requirements {
    let href = knowledge_requirement.required_knowledge().as_ref().ok_or_else(err_empty_reference)?;
    let required_knowledge_id: &str = href.into();
    if let Some(business_knowledge_model) = definitions.business_knowledge_model_by_id(required_knowledge_id) {
      let output_variable_name = business_knowledge_model.variable().qname();
      ctx.create_entry(output_variable_name, value_null!());
      bring_knowledge_requirements_into_context(definitions, business_knowledge_model.knowledge_requirements(), ctx)?;
    } else if let Some(decision_service) = definitions.decision_service_by_id(required_knowledge_id) {
      let output_variable_name = decision_service.variable().qname();
      ctx.create_entry(output_variable_name, value_null!());
    } else {
      return Err(err_business_knowledge_model_with_reference_not_found(required_knowledge_id));
    }
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use dmntk_feel::FeelType;

  #[test]
  fn test_type_ref_to_feel_type() {
    assert_eq!(Some(FeelType::String), type_ref_to_feel_type("string"));
    assert_eq!(Some(FeelType::Number), type_ref_to_feel_type("number"));
    assert_eq!(Some(FeelType::Boolean), type_ref_to_feel_type("boolean"));
    assert_eq!(Some(FeelType::Date), type_ref_to_feel_type("date"));
    assert_eq!(Some(FeelType::Time), type_ref_to_feel_type("time"));
    assert_eq!(Some(FeelType::DateTime), type_ref_to_feel_type("dateTime"));
    assert_eq!(Some(FeelType::DaysAndTimeDuration), type_ref_to_feel_type("dayTimeDuration"));
    assert_eq!(Some(FeelType::YearsAndMonthsDuration), type_ref_to_feel_type("yearMonthDuration"));
    assert_eq!(None, type_ref_to_feel_type("text"));
  }
}

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

use crate::errors::*;
use crate::ModelEvaluator;
pub use business_knowledge_model::BusinessKnowledgeModelEvaluator;
pub use decision::DecisionEvaluator;
pub use decision_service::DecisionServiceEvaluator;
use dmntk_common::{DmntkError, Result};
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::{Value, Values};
use dmntk_feel::{value_null, Evaluator, FeelType, FunctionBody, Name, Scope};
use dmntk_model::model::*;
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
fn item_definition_type(item_definition: &ItemDefinition) -> Result<ItemDefinitionType> {
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

/// Type of closure that evaluates values from variable definition.
type VariableEvaluatorFn = Box<dyn Fn(&Value, &ItemDefinitionEvaluator) -> (Name, Value) + Send + Sync>;

///
pub struct Variable {
  pub name: Name,
  pub type_ref: Option<String>,
}

impl TryFrom<&InformationItem> for Variable {
  type Error = DmntkError;
  ///
  fn try_from(value: &InformationItem) -> Result<Self, Self::Error> {
    let name = value.feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone();
    let type_ref = value.type_ref().clone();
    Ok(Self { name, type_ref })
  }
}

impl Variable {
  ///
  fn feel_type(&self, item_definition_type_evaluator: &ItemDefinitionTypeEvaluator) -> FeelType {
    if let Some(type_ref) = &self.type_ref {
      information_item_type(type_ref, item_definition_type_evaluator).unwrap_or(FeelType::Any)
    } else {
      FeelType::Any
    }
  }
}

///
fn build_variable_evaluator(variable: &Variable) -> Result<VariableEvaluatorFn> {
  // prepare the variable name
  let variable_name = variable.name.clone();
  // if there is no type reference defined, the value is just returned as is
  if variable.type_ref.is_none() {
    return Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          return (variable_name.clone(), v.clone());
        }
      }
      (variable_name.clone(), value_null!())
    }));
  }
  // here the `variable.type_ref` must have some value, so unwrapping is safe
  // type_ref is either a simple type name or a name of an item definition,
  // both cases are handled below
  let type_ref = variable.type_ref.as_ref().unwrap().clone();
  Ok(match type_ref.as_str() {
    "Any" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          return (variable_name.clone(), v.clone());
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "Null" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          return if let Value::Null(_) = v {
            (variable_name.clone(), v.clone())
          } else {
            (variable_name.clone(), FeelType::Null.coerced(v))
          };
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "string" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          return if let Value::String(_) = v {
            (variable_name.clone(), v.clone())
          } else {
            (variable_name.clone(), FeelType::String.coerced(v))
          };
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "number" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          return if let Value::Number(_) = v {
            (variable_name.clone(), v.clone())
          } else {
            (variable_name.clone(), FeelType::Number.coerced(v))
          };
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "boolean" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          return if let Value::Boolean(_) = v {
            (variable_name.clone(), v.clone())
          } else {
            (variable_name.clone(), FeelType::Boolean.coerced(v))
          };
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "date" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          return if let Value::Date(_) = v {
            (variable_name.clone(), v.clone())
          } else {
            (variable_name.clone(), FeelType::Date.coerced(v))
          };
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "time" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          return if let Value::Time(_) = v {
            (variable_name.clone(), v.clone())
          } else {
            (variable_name.clone(), FeelType::Time.coerced(v))
          };
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "dateTime" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          return if let Value::DateTime(_) = v {
            (variable_name.clone(), v.clone())
          } else {
            (variable_name.clone(), FeelType::DateTime.coerced(v))
          };
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "dayTimeDuration" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          return if let Value::DaysAndTimeDuration(_) = v {
            (variable_name.clone(), v.clone())
          } else {
            (variable_name.clone(), FeelType::DaysAndTimeDuration.coerced(v))
          };
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "yearMonthDuration" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          return if let Value::YearsAndMonthsDuration(_) = v {
            (variable_name.clone(), v.clone())
          } else {
            (variable_name.clone(), FeelType::YearsAndMonthsDuration.coerced(v))
          };
        }
      }
      (variable_name.clone(), value_null!())
    }),
    _ => Box::new(move |value: &Value, item_definition_evaluator: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(entry_value) = ctx.get_entry(&variable_name) {
          let evaluated_value = item_definition_evaluator
            .eval(&type_ref, entry_value)
            .unwrap_or_else(|| value_null!("input data evaluator: item definition evaluator '{}' not found", type_ref));
          (variable_name.clone(), evaluated_value)
        } else {
          (variable_name.clone(), value_null!("no name {} in context {}", variable_name, ctx))
        }
      } else {
        (variable_name.clone(), value_null!("expected context, actual value is {}", value))
      }
    }),
  })
}

///
fn build_expression_instance_evaluator(scope: &Scope, expression_instance: &ExpressionInstance, model_evaluator: &ModelEvaluator) -> Result<Evaluator> {
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
fn build_context_evaluator(scope: &Scope, context: &Context, model_evaluator: &ModelEvaluator) -> Result<Evaluator> {
  let mut entry_evaluators = vec![];
  scope.push(FeelContext::default());
  for context_entry in context.context_entries() {
    if let Some(variable) = &context_entry.variable {
      let name = variable.feel_name().as_ref().ok_or_else(err_empty_feel_name)?;
      let evaluator = build_expression_instance_evaluator(scope, &context_entry.value, model_evaluator)?;
      scope.insert_null(name.clone());
      entry_evaluators.push((Some(name.clone()), evaluator));
    } else {
      let evaluator = build_expression_instance_evaluator(scope, &context_entry.value, model_evaluator)?;
      entry_evaluators.push((None, evaluator));
    }
  }
  scope.pop();
  Ok(Box::new(move |scope: &Scope| {
    let mut evaluated_context = FeelContext::default();
    for (opt_name, evaluator) in &entry_evaluators {
      match opt_name {
        Some(name) => {
          let value = evaluator(scope) as Value;
          scope.set_entry(name, value.clone());
          evaluated_context.set_entry(name, value);
        }
        None => {
          return evaluator(scope);
        }
      }
    }
    Value::Context(evaluated_context)
  }))
}

///
fn build_decision_table_evaluator(scope: &Scope, decision_table: &DecisionTable) -> Result<Evaluator> {
  let decision_table_evaluator = decision_table::build_decision_table_evaluator(scope, decision_table)?;
  Ok(Box::new(move |scope: &Scope| decision_table_evaluator(scope)))
}

///
fn build_function_definition_evaluator(scope: &Scope, function_definition: &FunctionDefinition, model_evaluator: &ModelEvaluator) -> Result<Evaluator> {
  let item_definition_type_evaluator = model_evaluator.item_definition_type_evaluator()?;
  // resolve function definition's formal parameters
  let mut parameters = vec![];
  let mut parameters_ctx = FeelContext::default();
  for parameter in function_definition.formal_parameters() {
    let parameter_name = parameter.feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone();
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
  let closure_ctx = scope.peek();
  // prepare function definition's body evaluator
  let body_expression_instance = function_definition.body().as_ref().ok_or_else(err_empty_function_body)?;
  scope.push(parameters_ctx);
  let body_evaluator = build_expression_instance_evaluator(scope, body_expression_instance, model_evaluator)?;
  scope.pop();
  let function_body_evaluator = Arc::new(body_evaluator);
  let function_body = FunctionBody::LiteralExpression(function_body_evaluator);
  //let closure_ctx = FeelContext::default();
  Ok(Box::new(move |scope: &Scope| {
    let mut a = FeelContext::default();
    for (name, _) in closure_ctx.get_entries() {
      if let Some(v) = scope.get_entry(name) {
        a.set_entry(name, v);
      }
    }
    Value::FunctionDefinition(parameters.clone(), function_body.clone(), a, result_type.clone())
  }))
}

///
fn build_invocation_evaluator(scope: &Scope, invocation: &Invocation, model_evaluator: &ModelEvaluator) -> Result<Evaluator> {
  let item_definition_type_evaluator = model_evaluator.item_definition_type_evaluator()?;
  let mut bindings = vec![];
  let function_evaluator = build_expression_instance_evaluator(scope, invocation.called_function(), model_evaluator)?;
  for binding in invocation.bindings() {
    if let Some(binding_formula) = binding.binding_formula() {
      let param_name = binding.parameter().feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone();
      let param_type = if let Some(type_ref) = binding.parameter().type_ref() {
        information_item_type(type_ref, &item_definition_type_evaluator).ok_or_else(err_empty_feel_type)?
      } else {
        FeelType::Any
      };
      let evaluator = build_expression_instance_evaluator(scope, binding_formula, model_evaluator)?;
      bindings.push((param_name, param_type, evaluator));
    }
  }
  Ok(Box::new(move |scope: &Scope| {
    let mut parameters_ctx = FeelContext::default();
    bindings.iter().for_each(|(param_name, param_type, evaluator)| {
      let param_value = evaluator(scope) as Value;
      parameters_ctx.set_entry(param_name, param_type.coerced(&param_value))
    });
    if let Value::FunctionDefinition(_, body, closure_ctx, result_type) = function_evaluator(scope) {
      scope.push(closure_ctx);
      scope.push(parameters_ctx);
      let value = body.evaluate(scope);
      scope.pop();
      scope.pop();
      result_type.coerced(&value)
    } else {
      value_null!("expected Value::FunctionDefinition in invocation evaluator")
    }
  }))
}

///
fn build_literal_expression_evaluator(scope: &Scope, literal_expression: &LiteralExpression) -> Result<Evaluator> {
  let text = literal_expression.text().as_ref().ok_or_else(err_empty_literal_expression)?;
  let node = dmntk_feel_parser::parse_expression(scope, text, false)?;
  dmntk_feel_evaluator::prepare(&node)
}

///
fn build_relation_evaluator(scope: &Scope, relation: &Relation, model_evaluator: &ModelEvaluator) -> Result<Evaluator> {
  let mut rows = vec![];
  for row in relation.rows() {
    let mut evaluators = vec![];
    for (i, element) in row.elements().iter().enumerate() {
      if let Some(column) = relation.columns().get(i) {
        let name = column.feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone();
        let evaluator = build_expression_instance_evaluator(scope, element, model_evaluator)?;
        evaluators.push((name, evaluator));
      }
    }
    rows.push(evaluators);
  }
  Ok(Box::new(move |scope: &Scope| {
    let mut results = vec![];
    for row in &rows {
      let mut evaluated_context = FeelContext::default();
      for (name, evaluator) in row {
        evaluated_context.set_entry(name, evaluator(scope));
      }
      results.push(Value::Context(evaluated_context));
    }
    Value::List(Values::new(results))
  }))
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

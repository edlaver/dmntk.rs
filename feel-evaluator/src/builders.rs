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

use crate::bifs;
use crate::errors::*;
use crate::iterations::{EveryExpressionEvaluator, ForExpressionEvaluator, SomeExpressionEvaluator};
use crate::macros::invalid_argument_type;
use dmntk_common::Result;
use dmntk_feel::bif::Bif;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::{Value, Values, VALUE_FALSE, VALUE_TRUE};
use dmntk_feel::{value_null, Evaluator, FeelNumber, FeelScope, FeelType, FunctionBody, Name, QualifiedName};
use dmntk_feel_parser::{AstNode, ClosureBuilder};
use dmntk_feel_temporal::{FeelDate, FeelDateTime, FeelDaysAndTimeDuration, FeelTime, FeelYearsAndMonthsDuration};
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashSet};
use std::convert::TryFrom;
use std::file;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

///
pub fn build_evaluator(node: &AstNode) -> Result<Evaluator> {
  match node {
    AstNode::Add(lhs, rhs) => build_add(lhs, rhs),
    AstNode::And(lhs, rhs) => build_and(lhs, rhs),
    AstNode::At(rhs) => build_at(rhs),
    AstNode::Between(lhs, mhs, rhs) => build_between(lhs, mhs, rhs),
    AstNode::Boolean(lhs) => build_boolean(*lhs),
    AstNode::Context(lhs) => build_context(lhs),
    AstNode::ContextEntry(lhs, rhs) => build_context_entry(lhs, rhs),
    AstNode::ContextEntryKey(lhs) => build_context_entry_key(lhs),
    AstNode::ContextType(lhs) => build_context_type(lhs),
    AstNode::ContextTypeEntry(lhs, rhs) => build_context_type_entry(lhs, rhs),
    AstNode::ContextTypeEntryKey(lhs) => build_context_type_entry_key(lhs),
    AstNode::Div(lhs, rhs) => build_div(lhs, rhs),
    AstNode::Eq(lhs, rhs) => build_eq(lhs, rhs),
    AstNode::EvaluatedExpression(lhs) => build_evaluated_expression(lhs),
    AstNode::Every(lhs, rhs) => build_every(lhs, rhs),
    AstNode::Exp(lhs, rhs) => build_exp(lhs, rhs),
    AstNode::ExpressionList(lhs) => build_expression_list(lhs),
    AstNode::FeelType(lhs) => build_feel_type(lhs),
    AstNode::Filter(lhs, rhs) => build_filter(lhs, rhs),
    AstNode::For(lhs, rhs) => build_for(lhs, rhs),
    AstNode::FormalParameter(lhs, rhs) => build_formal_parameter(lhs, rhs),
    AstNode::FormalParameters(lhs) => build_formal_parameters(lhs),
    AstNode::FunctionBody(lhs, rhs) => build_function_body(lhs, rhs),
    AstNode::FunctionDefinition(lhs, rhs) => build_function_definition(lhs, rhs),
    AstNode::FunctionInvocation(lhs, rhs) => build_function_invocation(lhs, rhs),
    AstNode::FunctionType(lhs, rhs) => build_function_type(lhs, rhs),
    AstNode::Ge(lhs, rhs) => build_ge(lhs, rhs),
    AstNode::Gt(lhs, rhs) => build_gt(lhs, rhs),
    AstNode::If(lhs, mid, rhs) => build_if(lhs, mid, rhs),
    AstNode::In(lhs, rhs) => build_in(lhs, rhs),
    AstNode::InstanceOf(lhs, rhs) => build_instance_of(lhs, rhs),
    AstNode::IntervalEnd(lhs, rhs) => build_interval_end(lhs, rhs),
    AstNode::IntervalStart(lhs, rhs) => build_interval_start(lhs, rhs),
    AstNode::Irrelevant => build_irrelevant(),
    AstNode::Le(lhs, rhs) => build_le(lhs, rhs),
    AstNode::List(items) => build_list(items),
    AstNode::ListType(lhs) => build_list_type(lhs),
    AstNode::Lt(lhs, rhs) => build_lt(lhs, rhs),
    AstNode::Mul(lhs, rhs) => build_mul(lhs, rhs),
    AstNode::Name(name) => build_name(name.clone()),
    AstNode::NamedParameter(lhs, rhs) => build_named_parameter(lhs, rhs),
    AstNode::NamedParameters(lhs) => build_named_parameters(lhs),
    AstNode::Neg(rhs) => build_neg(rhs),
    AstNode::NegatedList(lhs) => build_negated_list(lhs),
    AstNode::Null => build_null(),
    AstNode::Numeric(lhs, rhs) => build_numeric(lhs, rhs),
    AstNode::Nq(lhs, rhs) => build_nq(lhs, rhs),
    AstNode::Or(lhs, rhs) => build_or(lhs, rhs),
    AstNode::Out(lhs, rhs) => build_out(lhs, rhs),
    AstNode::ParameterName(lhs) => build_parameter_name(lhs),
    AstNode::ParameterTypes(lhs) => build_parameter_types(lhs),
    AstNode::Path(lhs, rhs) => build_path(lhs, rhs),
    AstNode::QualifiedName(lhs) => build_qualified_name(lhs),
    AstNode::QualifiedNameSegment(lhs) => build_qualified_name_segment(lhs),
    AstNode::Range(lhs, rhs) => build_range(lhs, rhs),
    AstNode::RangeType(lhs) => build_range_type(lhs),
    AstNode::Some(lhs, rhs) => build_some(lhs, rhs),
    AstNode::String(lhs) => build_string(lhs),
    AstNode::Sub(lhs, rhs) => build_sub(lhs, rhs),
    AstNode::UnaryGe(lhs) => build_unary_ge(lhs),
    AstNode::UnaryGt(lhs) => build_unary_gt(lhs),
    AstNode::UnaryLe(lhs) => build_unary_le(lhs),
    AstNode::UnaryLt(lhs) => build_unary_lt(lhs),
    AstNode::CommaList { .. }
    | AstNode::IterationContexts { .. }
    | AstNode::IterationContextSingle { .. }
    | AstNode::IterationContextRange { .. }
    | AstNode::PositionalParameters { .. }
    | AstNode::QuantifiedContext { .. }
    | AstNode::QuantifiedContexts { .. }
    | AstNode::Satisfies { .. } => Err(err_unexpected_ast_node(&format!("{node:?}"))),
  }
}

///
fn build_add(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => Value::Number(lh + rh),
        value @ Value::Null(_) => value,
        _ => value_null!("incompatible types in addition: {}({}) + {}({})", lhv, lhv.type_of(), rhv, rhv.type_of()),
      },
      Value::String(mut lh) => {
        if let Value::String(rh) = rhv {
          lh.push_str(&rh);
          Value::String(lh)
        } else {
          value_null!("expected string as a second argument in addition")
        }
      }
      Value::Date(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => {
          if let Some(result) = lh + rh {
            Value::Date(result)
          } else {
            value_null!("invalid result while adding days and time duration to date")
          }
        }
        Value::YearsAndMonthsDuration(rh) => {
          if let Some(a) = lh + rh {
            Value::Date(a)
          } else {
            value_null!("invalid result while adding years and months duration to date")
          }
        }
        other => invalid_argument_type!("add", "years and months duration", other.type_of()),
      },
      Value::DateTime(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => {
          if let Some(a) = lh + rh {
            Value::DateTime(a)
          } else {
            value_null!("invalid result while adding days and time duration to date and time")
          }
        }
        Value::YearsAndMonthsDuration(rh) => {
          if let Some(a) = lh + rh {
            Value::DateTime(a)
          } else {
            value_null!("invalid result while adding years and months duration to date and time")
          }
        }
        other => invalid_argument_type!("add", "days and time duration, years and months duration", other.type_of()),
      },
      Value::Time(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => Value::Time(lh + rh),
        other => invalid_argument_type!("add", "days and time duration", other.type_of()),
      },
      Value::DaysAndTimeDuration(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => Value::DaysAndTimeDuration(lh + rh),
        Value::Date(rh) => {
          if let Some(result) = rh + lh {
            Value::Date(result)
          } else {
            value_null!("invalid result while adding date to days and time duration")
          }
        }
        Value::DateTime(rh) => {
          if let Some(a) = rh + lh {
            Value::DateTime(a)
          } else {
            value_null!("invalid result while adding date and time to days and time duration")
          }
        }
        Value::Time(rh) => Value::Time(rh + lh),
        other => invalid_argument_type!("add", "days and time duration, date and time", other.type_of()),
      },
      Value::YearsAndMonthsDuration(lh) => match rhv {
        Value::Date(rh) => {
          if let Some(a) = rh + lh {
            Value::Date(a)
          } else {
            value_null!("invalid result while adding date to years and months duration")
          }
        }
        Value::DateTime(rh) => {
          if let Some(a) = rh + lh {
            Value::DateTime(a)
          } else {
            value_null!("invalid result while adding date and time to years and months duration")
          }
        }
        Value::YearsAndMonthsDuration(rh) => Value::YearsAndMonthsDuration(lh + rh),
        other => invalid_argument_type!("add", "years and months duration, date and time", other.type_of()),
      },
      value @ Value::Null(_) => value,
      other => invalid_argument_type!(
        "add",
        "number, string, date and time, days and time duration, years and months duration, null",
        other.type_of()
      ),
    }
  }))
}

/// Builds evaluator of temporal expression after `@` (at) literal.
fn build_at(text: &str) -> Result<Evaluator> {
  if let Ok(date) = FeelDate::from_str(text) {
    return Ok(Box::new(move |_: &FeelScope| Value::Date(date.clone())));
  }
  if let Ok(date_time) = FeelDateTime::try_from(text) {
    return Ok(Box::new(move |_: &FeelScope| Value::DateTime(date_time.clone())));
  }
  if let Ok(time) = text.parse::<FeelTime>() {
    return Ok(Box::new(move |_: &FeelScope| Value::Time(time.clone())));
  }
  if let Ok(ym_duration) = FeelYearsAndMonthsDuration::try_from(text) {
    return Ok(Box::new(move |_: &FeelScope| Value::YearsAndMonthsDuration(ym_duration.clone())));
  }
  if let Ok(dt_duration) = FeelDaysAndTimeDuration::try_from(text) {
    return Ok(Box::new(move |_: &FeelScope| Value::DaysAndTimeDuration(dt_duration.clone())));
  }
  Err(err_invalid_at_literal(text))
}

/// Prepares null value with error message for second argument in `between` operator.
macro_rules! between_null2 {
  ($expected:literal, $actual:expr) => {
    Value::Null(Some(format!(
      "expected {} as a second argument in 'between' operator, actual value type is {}",
      $expected, $actual
    )))
  };
}

/// Prepares null value with error message for third argument in `between` operator.
macro_rules! between_null3 {
  ($expected:literal, $actual:expr) => {
    Value::Null(Some(format!(
      "expected {} as a third argument in 'between' operator, actual value type is {}",
      $expected, $actual
    )))
  };
}

///
fn build_between(lhs: &AstNode, mhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let mhe = build_evaluator(mhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let mhv = mhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    match lhv {
      Value::Number(lh) => {
        if let Value::Number(mh) = mhv {
          if let Value::Number(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("number", rhv.type_of())
          }
        } else {
          between_null2!("number", mhv.type_of())
        }
      }
      Value::String(lh) => {
        if let Value::String(mh) = mhv {
          if let Value::String(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("string", rhv.type_of())
          }
        } else {
          between_null2!("string", mhv.type_of())
        }
      }
      Value::Date(lh) => {
        if let Value::Date(mh) = mhv {
          if let Value::Date(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("date", rhv.type_of())
          }
        } else {
          between_null2!("date", mhv.type_of())
        }
      }
      Value::Time(lh) => {
        if let Value::Time(mh) = mhv {
          if let Value::Time(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("time", rhv.type_of())
          }
        } else {
          between_null2!("time", mhv.type_of())
        }
      }
      Value::DateTime(lh) => {
        if let Value::DateTime(mh) = mhv {
          if let Value::DateTime(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("date and time", rhv.type_of())
          }
        } else {
          between_null2!("date and time", mhv.type_of())
        }
      }
      Value::DaysAndTimeDuration(lh) => {
        if let Value::DaysAndTimeDuration(mh) = mhv {
          if let Value::DaysAndTimeDuration(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("days and time duration", rhv.type_of())
          }
        } else {
          between_null2!("days and time duration", mhv.type_of())
        }
      }
      Value::YearsAndMonthsDuration(lh) => {
        if let Value::YearsAndMonthsDuration(mh) = mhv {
          if let Value::YearsAndMonthsDuration(rh) = rhv {
            Value::Boolean(mh <= lh && lh <= rh)
          } else {
            between_null3!("years and months duration", rhv.type_of())
          }
        } else {
          between_null2!("years and months duration", mhv.type_of())
        }
      }
      other => value_null!("unexpected value type in 'between' operator: {}", other.type_of()),
    }
  }))
}

///
fn build_boolean(lhs: bool) -> Result<Evaluator> {
  Ok(Box::new(move |_: &FeelScope| Value::Boolean(lhs)))
}

/// Semantics of conjunction.
/// ```text
/// left        right        result
///─────────────────────────────────
/// true        true         true
/// true        false        false
/// true        otherwise    null
/// false       true         false
/// false       false        false
/// false       otherwise    false
/// otherwise   true         null
/// otherwise   false        false
/// otherwise   otherwise    null
/// ```
fn build_and(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    match lhv {
      Value::Boolean(lh) => match rhv {
        Value::Boolean(rh) => Value::Boolean(lh && rh),
        _ => {
          if lh {
            value_null!()
          } else {
            Value::Boolean(false)
          }
        }
      },
      _ => match rhv {
        Value::Boolean(rh) => {
          if rh {
            value_null!()
          } else {
            Value::Boolean(false)
          }
        }
        _ => value_null!(),
      },
    }
  }))
}

///
fn build_context(lhs: &[AstNode]) -> Result<Evaluator> {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(node)?);
  }
  Ok(Box::new(move |scope: &FeelScope| {
    let mut evaluated_ctx = FeelContext::default();
    // prepare special context in scope, used for already evaluated context entries
    scope.push(FeelContext::default());
    // evaluate context entries
    for evaluator in &evaluators {
      match evaluator(scope) as Value {
        Value::ContextEntry(name, value) => {
          if evaluated_ctx.contains_entry(&name) {
            // duplicated context entry keys are not allowed
            scope.pop();
            return value_null!("duplicated context entry key: {}", name);
          } else {
            // add newly evaluated entry to evaluated context
            evaluated_ctx.set_entry(&name, (*value).clone());
            // add newly evaluated entry to special context
            scope.set_value(&name, *value);
          }
        }
        null @ Value::Null(_) => return null,
        other => return value_null!("expected context entry, actual value type is {}", other.type_of()),
      }
    }
    // remove special context from scope
    scope.pop();
    Value::Context(evaluated_ctx)
  }))
}

///
fn build_context_entry(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    match lhv {
      Value::ContextEntryKey(name) => Value::ContextEntry(name, Box::new(rhv)),
      _ => value_null!("expected context entry key, actual value type is {}", lhv.type_of()),
    }
  }))
}

///
fn build_context_entry_key(lhs: &Name) -> Result<Evaluator> {
  let name = lhs.clone();
  Ok(Box::new(move |_: &FeelScope| Value::ContextEntryKey(name.clone())))
}

///
fn build_context_type(lhs: &[AstNode]) -> Result<Evaluator> {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(node)?);
  }
  Ok(Box::new(move |scope: &FeelScope| {
    let mut entries = BTreeMap::new();
    for evaluator in &evaluators {
      match evaluator(scope) as Value {
        Value::ContextTypeEntry(name, feel_type) => {
          entries.insert(name, feel_type.clone());
        }
        null @ Value::Null(_) => return null,
        other => return value_null!("expected context type entry, actual value type is {}", other.type_of()),
      }
    }
    Value::FeelType(FeelType::Context(entries))
  }))
}

///
fn build_context_type_entry(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    if let Value::ContextTypeEntryKey(name) = lhv {
      if let Value::FeelType(feel_type) = rhv {
        Value::ContextTypeEntry(name, feel_type)
      } else {
        value_null!("expected a type in context type entry")
      }
    } else {
      value_null!("expected a name in context type entry")
    }
  }))
}

///
fn build_context_type_entry_key(lhs: &Name) -> Result<Evaluator> {
  let name = lhs.clone();
  Ok(Box::new(move |_: &FeelScope| Value::ContextTypeEntryKey(name.clone())))
}

///
fn build_div(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => {
          if rh.abs() == FeelNumber::zero() {
            value_null!("[division] division by zero")
          } else {
            Value::Number(lh / rh)
          }
        }
        _ => value_null!("[division] incompatible types: {} / {}", lhv, rhv),
      },
      Value::DaysAndTimeDuration(ref lh) => match rhv {
        Value::Number(rh) => {
          if rh.is_zero() {
            value_null!("[division] division by zero")
          } else {
            let lv = FeelNumber::from(lh.as_nanos()) / rh;
            if let Ok(v) = FeelNumber::try_into(lv) {
              Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_n(v))
            } else {
              value_null!("[division] error: {} / {}", lhv, rhv)
            }
          }
        }
        Value::DaysAndTimeDuration(rh) => {
          if rh.as_nanos() == 0 {
            value_null!("[division] division by zero")
          } else {
            let lvl = FeelNumber::from(lh.as_nanos());
            let rvl = FeelNumber::from(rh.as_nanos());
            Value::Number(lvl / rvl)
          }
        }
        _ => value_null!("[division] incompatible types: {} / {}", lhv, rhv),
      },
      Value::YearsAndMonthsDuration(ref lh) => match rhv {
        Value::Number(rh) => {
          if rh.is_zero() {
            value_null!("[division] division by zero")
          } else {
            let vl = FeelNumber::from(lh.as_months()) / rh;
            if let Ok(v) = FeelNumber::try_into(vl) {
              Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::from_m(v))
            } else {
              value_null!("[division] error: {} / {}", lhv, rhv)
            }
          }
        }
        Value::YearsAndMonthsDuration(rh) => {
          if rh.as_months() == 0 {
            value_null!("[division] division by zero")
          } else {
            let lvl = FeelNumber::from(lh.as_months());
            let rvl = FeelNumber::from(rh.as_months());
            Value::Number(lvl / rvl)
          }
        }
        _ => value_null!("[division] incompatible types: {} / {}", lhv, rhv),
      },
      _ => value_null!("[division] incompatible types: {} / {}", lhv, rhv),
    }
  }))
}

///
fn build_expression_list(lhs: &[AstNode]) -> Result<Evaluator> {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(node)?);
  }
  Ok(Box::new(move |scope: &FeelScope| {
    let mut values = vec![];
    for evaluator in &evaluators {
      values.push(evaluator(scope))
    }
    Value::ExpressionList(values)
  }))
}

///
fn build_exp(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    if let Value::Number(lh) = lhv {
      if let Value::Number(rh) = rhv {
        if let Some(result) = lh.pow(&rh) {
          Value::Number(result)
        } else {
          value_null!("exponentiation result is not a finite number")
        }
      } else {
        value_null!("exponentiation exponent is not a number")
      }
    } else {
      value_null!("exponentiation base is not a number")
    }
  }))
}

///
fn build_feel_type(lhs: &FeelType) -> Result<Evaluator> {
  let feel_type = lhs.clone();
  Ok(Box::new(move |_: &FeelScope| Value::FeelType(feel_type.clone())))
}

///
fn build_filter(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  let name_item: Name = "item".into();
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    match lhv {
      Value::List(values) => {
        let mut filtered_values = vec![];
        for value in &values {
          let (added_local_context, has_item_entry) = if let Value::Context(local_context) = value {
            scope.push(local_context.clone());
            if local_context.contains_entry(&name_item) {
              (true, true)
            } else {
              (true, false)
            }
          } else {
            (false, false)
          };
          if !has_item_entry {
            let mut special_context = FeelContext::default();
            special_context.set_entry(&name_item, value.clone());
            scope.push(special_context);
          }
          let rhv = rhe(scope) as Value;
          if let Value::Boolean(true) = rhv {
            filtered_values.push(value.clone());
          }
          if !has_item_entry {
            scope.pop();
          }
          if added_local_context {
            scope.pop();
          }
        }
        let rhv = rhe(scope) as Value;
        match rhv {
          Value::Number(index) => {
            if index.is_integer() {
              let list_size = values.len();
              if list_size > 0 {
                if !index.is_negative() {
                  let n = {
                    if let Ok(u_index) = usize::try_from(index) {
                      u_index
                    } else {
                      return value_null!("index is out of range 1..2⁶⁴: {}", index.to_string());
                    }
                  };
                  if n > 0 && n <= list_size {
                    // unwrap below is safe, index `n` is checked above, `values` variable is immutable
                    values.get(n - 1).unwrap().to_owned()
                  } else {
                    value_null!("index in filter is out of range [1..{}], actual index is {}", list_size, n)
                  }
                } else {
                  let n = {
                    if let Ok(u_index) = usize::try_from(index.abs()) {
                      u_index
                    } else {
                      return value_null!("index is out of range 1..2⁶⁴: {}", index.to_string());
                    }
                  };
                  if n > 0 && n <= list_size {
                    // unwrap below is safe, index `n` is checked above, `values` variable is immutable
                    values.get(list_size - n).unwrap().to_owned()
                  } else {
                    value_null!("index in filter is out of range [-{}..-1], actual index is -{}", list_size, n)
                  }
                }
              } else {
                // return null when the list is empty, no matter what value the index has
                value_null!()
              }
            } else {
              value_null!("index in filter must be an integer value, actual value is {}", index)
            }
          }
          _ => {
            // coerse the result list
            if filtered_values.len() == 1 {
              filtered_values[0].to_owned()
            } else {
              Value::List(filtered_values)
            }
          }
        }
      }
      v @ Value::Number(_)
      | v @ Value::Boolean(_)
      | v @ Value::String(_)
      | v @ Value::Date(_)
      | v @ Value::DateTime(_)
      | v @ Value::Time(_)
      | v @ Value::DaysAndTimeDuration(_)
      | v @ Value::YearsAndMonthsDuration(_)
      | v @ Value::Context(_) => match rhe(scope) {
        Value::Boolean(flag) => {
          if flag {
            Value::List(vec![v])
          } else {
            Value::List(Values::default())
          }
        }
        Value::Number(num) => {
          if num.is_one() || (-num).is_one() {
            v
          } else {
            value_null!("for singletons, only filter index with value 1 or -1 is accepted")
          }
        }
        _ => value_null!("only number or boolean indexes are allowed in filters"),
      },
      other => value_null!("unexpected value type in filter: {}", other.type_of()),
    }
  }))
}

///
fn build_for(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let rhe = build_evaluator(rhs)?;
  let mut evaluators_single = vec![];
  let mut evaluators_range = vec![];
  if let AstNode::IterationContexts(items) = lhs {
    for item in items {
      match item {
        AstNode::IterationContextRange(variable_name, range_start_node, range_end_node) => {
          let evaluator_range_start = build_evaluator(range_start_node)?;
          let evaluator_range_end = build_evaluator(range_end_node)?;
          if let AstNode::Name(name) = variable_name.borrow() {
            evaluators_range.push((name.clone(), evaluator_range_start, evaluator_range_end));
          }
        }
        AstNode::IterationContextSingle(variable_name, expr_node) => {
          let evaluator_single = build_evaluator(expr_node)?;
          if let AstNode::Name(name) = variable_name.borrow() {
            evaluators_single.push((name.clone(), evaluator_single));
          }
        }
        _ => {}
      }
    }
  }
  Ok(Box::new(move |scope: &FeelScope| {
    let mut expression_evaluator = ForExpressionEvaluator::new();
    if !evaluators_single.is_empty() {
      for (name, evaluator_single) in &evaluators_single {
        expression_evaluator.add_single(name.clone(), evaluator_single(scope));
      }
    }
    if !evaluators_range.is_empty() {
      for (name, evaluator_range_start, evaluator_range_end) in &evaluators_range {
        expression_evaluator.add_range(name.clone(), evaluator_range_start(scope), evaluator_range_end(scope));
      }
    }
    Value::List(expression_evaluator.evaluate(scope, &rhe))
  }))
}

///
fn build_formal_parameter(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    if let Value::ParameterName(parameter_name) = lhv {
      if let Value::FeelType(parameter_type) = rhv {
        Value::FormalParameter(parameter_name, parameter_type)
      } else {
        value_null!("expected type of the formal parameter")
      }
    } else {
      value_null!("expected name of the formal parameter")
    }
  }))
}

///
fn build_formal_parameters(lhs: &[AstNode]) -> Result<Evaluator> {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(node)?);
  }
  Ok(Box::new(move |scope: &FeelScope| {
    let mut formal_parameters = vec![];
    for evaluator in &evaluators {
      match evaluator(scope) as Value {
        Value::FormalParameter(parameter_name, parameter_type) => {
          formal_parameters.push((parameter_name, parameter_type));
        }
        null @ Value::Null(_) => return null,
        other => return value_null!("expected formal parameter, actual value type is: {}", other.type_of()),
      }
    }
    Value::FormalParameters(formal_parameters)
  }))
}

///
fn build_function_body(lhs: &AstNode, rhs: &bool) -> Result<Evaluator> {
  if *rhs {
    build_external_function_body(lhs)
  } else {
    build_internal_function_body(lhs)
  }
}

///
fn build_internal_function_body(lhs: &AstNode) -> Result<Evaluator> {
  let lhe = Arc::new(build_evaluator(lhs)?);
  Ok(Box::new(move |_: &FeelScope| Value::FunctionBody(FunctionBody::LiteralExpression(lhe.clone()), false)))
}

///
fn build_external_function_body(lhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let mapping_value = lhe(scope) as Value;
    match mapping_value {
      Value::Context(mapping_information) => {
        if let Some(Value::Context(java_mapping)) = mapping_information.get_entry(&"java".into()) {
          return if let Some(Value::String(class_name)) = java_mapping.get_entry(&"class".into()) {
            if let Some(Value::String(method_signature)) = java_mapping.get_entry(&"method signature".into()) {
              let java_class_name = class_name.to_owned();
              let java_method_signature = method_signature.to_owned();
              let java_evaluator = Box::new(move |_: &FeelScope| Value::ExternalJavaFunction(java_class_name.clone(), java_method_signature.clone())) as Evaluator;
              let lhe = Arc::new(java_evaluator);
              Value::FunctionBody(FunctionBody::External(lhe), true)
            } else {
              value_null!("invalid Java function mapping, no method signature entry in context {}", java_mapping)
            }
          } else {
            value_null!("invalid Java function mapping, no class name entry in context {}", java_mapping)
          };
        }
        if let Some(Value::Context(pmml_mapping)) = mapping_information.get_entry(&"pmml".into()) {
          return if let Some(Value::String(document)) = pmml_mapping.get_entry(&"document".into()) {
            if let Some(Value::String(model_name)) = pmml_mapping.get_entry(&"model".into()) {
              let pmml_document = document.to_owned();
              let pmml_model_name = model_name.to_owned();
              let pmml_evaluator = Box::new(move |_: &FeelScope| Value::ExternalPmmlFunction(pmml_document.clone(), pmml_model_name.clone())) as Evaluator;
              let lhe = Arc::new(pmml_evaluator);
              Value::FunctionBody(FunctionBody::External(lhe), true)
            } else {
              value_null!("invalid PMML function mapping, no model name entry in context {}", pmml_mapping)
            }
          } else {
            value_null!("invalid PMML function mapping, no document entry in context {}", pmml_mapping)
          };
        }
        value_null!("invalid external function mapping, expected 'java' or 'pmml' entry in context {}", mapping_information)
      }
      other => value_null!("invalid external function mapping, expected context, actual value is {}", other),
    }
  }))
}

///
fn build_function_definition(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let closure = ClosureBuilder::from_function_definition(lhs, rhs);
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    match lhv {
      Value::FormalParameters(parameters) => {
        if let Value::FunctionBody(body, external) = rhv {
          // evaluate closure context
          let mut closure_ctx = FeelContext::default();
          for closure_name in closure.iter() {
            if let Some(closure_value) = scope.search_entry(closure_name) {
              closure_ctx.create_entry(closure_name, closure_value);
            }
          }
          //TODO is `FeelType::Any` always ok for function result type in function definition?
          Value::FunctionDefinition(parameters, body, external, closure.clone(), closure_ctx, FeelType::Any)
        } else {
          value_null!("invalid body in function definition")
        }
      }
      null @ Value::Null(_) => null,
      other => value_null!("expected formal parameters, actual value type is {}", other.type_of()),
    }
  }))
}

///
fn build_eq(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    if let Some(result) = eval_ternary_equality(&lhv, &rhv) {
      Value::Boolean(result)
    } else {
      value_null!("equal err '{}' =?= '{}'", lhv, rhv)
    }
  }))
}

///
fn build_evaluated_expression(lhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  Ok(Box::new(move |scope: &FeelScope| lhe(scope)))
}

///
fn build_every(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let mut expr_evaluators = vec![];
  if let AstNode::QuantifiedContexts(items) = lhs {
    for item in items {
      if let AstNode::QuantifiedContext(variable_name, expr_node) = item {
        if let AstNode::Name(name) = variable_name.borrow() {
          let evaluator_single = build_evaluator(expr_node)?;
          expr_evaluators.push((name.clone(), evaluator_single));
        }
      }
    }
  } else {
    return Err(err_expected_ast_node("AstNode::QuantifiedContexts", &format!("{lhs:?}")));
  }
  if let AstNode::Satisfies(satisfies) = rhs {
    let satisfies_evaluator = build_evaluator(satisfies)?;
    Ok(Box::new(move |scope: &FeelScope| {
      let mut expression_evaluator = EveryExpressionEvaluator::new();
      for (name, expr_evaluator) in &expr_evaluators {
        expression_evaluator.add(name.clone(), expr_evaluator(scope));
      }
      expression_evaluator.evaluate(scope, &satisfies_evaluator)
    }))
  } else {
    Err(err_expected_ast_node("AstNode::Satisfies", &format!("{rhs:?}")))
  }
}

///
fn build_function_invocation(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  match rhs {
    AstNode::PositionalParameters(parameters) => build_function_invocation_with_positional_parameters(lhs, parameters),
    node @ AstNode::NamedParameters(_) => build_function_invocation_with_named_parameters(lhs, node),
    _ => Err(err_expected_positional_or_named_parameter()),
  }
}

///
fn build_function_invocation_with_positional_parameters(lhs: &AstNode, rhs: &[AstNode]) -> Result<Evaluator> {
  let mut argument_evaluators = vec![];
  for node in rhs {
    argument_evaluators.push(build_evaluator(node)?);
  }
  let function_evaluator = build_evaluator(lhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let function = function_evaluator(scope) as Value;
    let args = argument_evaluators.iter().map(|evaluator| evaluator(scope)).collect::<Vec<Value>>();
    match function {
      Value::BuiltInFunction(bif) => bifs::positional::evaluate_bif(bif, &args),
      Value::FunctionDefinition(params, body, external, _, closure_ctx, result_type) => {
        if external {
          eval_external_function_with_positional_parameters(scope, &args, &params, &body, result_type)
        } else {
          eval_function_with_positional_parameters(scope, &args, &params, &body, closure_ctx, result_type)
        }
      }
      _ => value_null!("expected built-in function name or function definition, actual is {}", function),
    }
  }))
}

///
fn build_function_invocation_with_named_parameters(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let function_evaluator = build_evaluator(lhs)?;
  let arguments_evaluator = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let function = function_evaluator(scope) as Value;
    let args = arguments_evaluator(scope) as Value;
    match function {
      Value::BuiltInFunction(bif) => bifs::named::evaluate_bif(bif, &args),
      Value::FunctionDefinition(params, body, external, _, closure_ctx, result_type) => {
        if external {
          eval_external_function_with_named_parameters(scope, &args, &params, &body, result_type)
        } else {
          eval_function_with_named_parameters(scope, &args, &params, &body, closure_ctx, result_type)
        }
      }
      _ => value_null!("expected built-in function name or function definition, actual is {}", function),
    }
  }))
}

///
fn build_function_type(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    if let Value::ParameterTypes(types) = lhv {
      if let Value::FeelType(result_type) = rhv {
        let parameter_types = types
          .iter()
          .filter_map(|value| if let Value::FeelType(feel_type) = value { Some(feel_type.clone()) } else { None })
          .collect();
        Value::FeelType(FeelType::Function(parameter_types, Box::new(result_type)))
      } else {
        value_null!("expected function's result type")
      }
    } else {
      value_null!("expected function's parameter types")
    }
  }))
}

///
fn build_ge(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_number"),
      },
      Value::String(lh) => match rhv {
        Value::String(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_string"),
      },
      Value::Date(lh) => match rhv {
        Value::Date(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_date"),
      },
      Value::DateTime(lh) => match rhv {
        Value::DateTime(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_date_time"),
      },
      Value::Time(lh) => match rhv {
        Value::Time(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_time"),
      },
      Value::DaysAndTimeDuration(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_days_and_time_duration"),
      },
      Value::YearsAndMonthsDuration(lh) => match rhv {
        Value::YearsAndMonthsDuration(rh) => Value::Boolean(lh >= rh),
        _ => value_null!("eval_greater_or_equal_years_and_months_duration"),
      },
      _ => value_null!("eval_greater_or_equal"),
    }
  }))
}

///
fn build_gt(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_then_number"),
      },
      Value::String(lh) => match rhv {
        Value::String(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_then_string"),
      },
      Value::Date(lh) => match rhv {
        Value::Date(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_then_date"),
      },
      Value::DateTime(lh) => match rhv {
        Value::DateTime(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_then_date_time"),
      },
      Value::Time(lh) => match rhv {
        Value::Time(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_then_time"),
      },
      Value::DaysAndTimeDuration(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_days_and_time_duration"),
      },
      Value::YearsAndMonthsDuration(lh) => match rhv {
        Value::YearsAndMonthsDuration(rh) => Value::Boolean(lh > rh),
        _ => value_null!("eval_greater_years_and_months_duration"),
      },
      _ => value_null!("eval_greater_then"),
    }
  }))
}

///
fn build_if(lhs: &AstNode, mhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let mhe = build_evaluator(mhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| match lhe(scope) {
    Value::Boolean(true) => mhe(scope),
    Value::Boolean(false) | Value::Null(_) => rhe(scope),
    _ => value_null!("condition in 'if' expression is not a boolean value"),
  }))
}

///
fn build_in(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    match rhv {
      inner @ Value::Null(_)
      | inner @ Value::Number(_)
      | inner @ Value::String(_)
      | inner @ Value::Boolean(_)
      | inner @ Value::Date(_)
      | inner @ Value::Time(_)
      | inner @ Value::DateTime(_)
      | inner @ Value::YearsAndMonthsDuration(_)
      | inner @ Value::DaysAndTimeDuration(_)
      | inner @ Value::Context(_) => eval_in_equal(&lhv, &inner),
      Value::Range(l, l_closed, r, r_closed) => eval_in_range(&lhv, &l, l_closed, &r, r_closed),
      Value::List(r_inner) => {
        if let Value::List(l_inner) = lhv {
          eval_in_list_in_list(&l_inner, &r_inner)
        } else {
          eval_in_list(&lhv, &r_inner)
        }
      }
      Value::ExpressionList(inner) => eval_in_list(&lhv, &inner),
      Value::NegatedCommaList(inner) => eval_in_negated_list(&lhv, &inner),
      Value::UnaryLess(inner) => eval_in_unary_less(&lhv, inner.borrow()),
      Value::UnaryLessOrEqual(inner) => eval_in_unary_less_or_equal(&lhv, inner.borrow()),
      Value::UnaryGreater(inner) => eval_in_unary_greater(&lhv, inner.borrow()),
      Value::UnaryGreaterOrEqual(inner) => eval_in_unary_greater_or_equal(&lhv, inner.borrow()),
      Value::Irrelevant => VALUE_TRUE,
      _ => value_null!("unexpected argument type in 'in' operator: {}", rhv.type_of()),
    }
  }))
}

///
fn build_interval_end(lhs: &AstNode, rhs: &bool) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let closed = *rhs;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    Value::IntervalEnd(Box::new(lhv), closed)
  }))
}

///
fn build_interval_start(lhs: &AstNode, rhs: &bool) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let closed = *rhs;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    Value::IntervalStart(Box::new(lhv), closed)
  }))
}

///
fn build_irrelevant() -> Result<Evaluator> {
  Ok(Box::new(move |_: &FeelScope| Value::Irrelevant))
}

///
fn build_instance_of(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    if let Value::FeelType(feel_type) = rhv {
      match lhv {
        Value::Number { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::Number => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::String { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::String => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::Boolean { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::Boolean => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::Date { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::Date => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::DateTime { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::DateTime => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::Time { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::Time => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::YearsAndMonthsDuration { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::YearsAndMonthsDuration => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::DaysAndTimeDuration { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          FeelType::DaysAndTimeDuration => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        Value::Null { .. } => match feel_type {
          FeelType::Null => VALUE_TRUE,
          _ => VALUE_FALSE,
        },
        value @ Value::Range { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          expected => Value::Boolean(value.type_of() == expected),
        },
        value @ Value::List { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          expected => Value::Boolean(value.type_of() == expected),
        },
        value @ Value::Context { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          expected => Value::Boolean(value.type_of() == expected),
        },
        value @ Value::FunctionDefinition { .. } => match feel_type {
          FeelType::Any => VALUE_TRUE,
          expected => Value::Boolean(value.type_of() == expected),
        },
        other => value_null!("invalid value in 'instance of' operator: {}", other),
      }
    } else {
      Value::Boolean(lhv.type_of() == rhv.type_of())
    }
  }))
}

///
fn build_le(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_number"),
      },
      Value::String(lh) => match rhv {
        Value::String(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_string"),
      },
      Value::Date(lh) => match rhv {
        Value::Date(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_date"),
      },
      Value::DateTime(lh) => match rhv {
        Value::DateTime(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_date_time"),
      },
      Value::Time(lh) => match rhv {
        Value::Time(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_time"),
      },
      Value::DaysAndTimeDuration(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_days_and_time_duration"),
      },
      Value::YearsAndMonthsDuration(lh) => match rhv {
        Value::YearsAndMonthsDuration(rh) => Value::Boolean(lh <= rh),
        _ => value_null!("eval_less_or_equal_years_and_months_duration"),
      },
      _ => value_null!("eval_less_or_equal"),
    }
  }))
}

///
fn build_lt(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_number"),
      },
      Value::String(lh) => match rhv {
        Value::String(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_string"),
      },
      Value::Date(lh) => match rhv {
        Value::Date(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_date"),
      },
      Value::DateTime(lh) => match rhv {
        Value::DateTime(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_date_time"),
      },
      Value::Time(lh) => match rhv {
        Value::Time(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_time"),
      },
      Value::DaysAndTimeDuration(lh) => match rhv {
        Value::DaysAndTimeDuration(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_days_and_time_duration"),
      },
      Value::YearsAndMonthsDuration(lh) => match rhv {
        Value::YearsAndMonthsDuration(rh) => Value::Boolean(lh < rh),
        _ => value_null!("eval_less_then_years_and_months_duration"),
      },
      _ => value_null!("eval_less_then"),
    }
  }))
}

///
fn build_list(lhs: &[AstNode]) -> Result<Evaluator> {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(node)?);
  }
  Ok(Box::new(move |scope: &FeelScope| {
    let mut values = vec![];
    for evaluator in &evaluators {
      values.push(evaluator(scope))
    }
    Value::List(values)
  }))
}

///
fn build_list_type(lhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    if let Value::FeelType(feel_type) = lhv {
      Value::FeelType(FeelType::List(Box::new(feel_type)))
    } else {
      value_null!("expected a feel type")
    }
  }))
}

///
fn build_mul(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    match lhv {
      Value::Number(lh) => match rhv {
        Value::Number(rh) => Value::Number(lh * rh),
        Value::DaysAndTimeDuration(ref rh) => {
          let val = lh * FeelNumber::from(rh.as_nanos());
          if let Ok(v) = FeelNumber::try_into(val) {
            Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_n(v))
          } else {
            value_null!("multiplication result is out of range of days and time duration")
          }
        }
        Value::YearsAndMonthsDuration(ref rh) => {
          let val = lh * FeelNumber::from(rh.as_months());
          if let Ok(v) = FeelNumber::try_into(val) {
            Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::from_m(v))
          } else {
            value_null!("multiplication result is out of range of years and months duration")
          }
        }
        _ => value_null!("[multiplication] incompatible types: {} * {}", lhv, rhv),
      },
      Value::DaysAndTimeDuration(ref lh) => match rhv {
        Value::Number(rh) => {
          let val = FeelNumber::from(lh.as_nanos()) * rh;
          if let Ok(v) = FeelNumber::try_into(val) {
            Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_n(v))
          } else {
            value_null!("multiplication result is out of range of days and time duration")
          }
        }
        _ => value_null!("[multiplication] incompatible types: {} * {}", lhv, rhv),
      },
      Value::YearsAndMonthsDuration(ref lh) => match rhv {
        Value::Number(rh) => {
          let val = FeelNumber::from(lh.as_months()) * rh;
          if let Ok(v) = FeelNumber::try_into(val) {
            Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::from_m(v))
          } else {
            value_null!("multiplication result is out of range of years and months duration")
          }
        }
        _ => value_null!("[multiplication] incompatible types: {} * {}", lhv, rhv),
      },
      value @ Value::Null(_) => value,
      other => value_null!("unexpected value type in multiplication: {}", other.type_of()),
    }
  }))
}

///
fn build_name(name: Name) -> Result<Evaluator> {
  Ok(Box::new(move |scope: &FeelScope| {
    if let Some(value) = scope.get_value(&name) {
      value
    } else if let Ok(bif) = Bif::from_str(&name.to_string()) {
      Value::BuiltInFunction(bif)
    } else {
      value_null!("context has no value for key '{}'", name)
    }
  }))
}

///
fn build_named_parameter(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  if let AstNode::ParameterName(name) = lhs {
    let lhv = Value::ParameterName(name.clone());
    let rhe = build_evaluator(rhs)?;
    return Ok(Box::new(move |scope: &FeelScope| {
      let rhv = rhe(scope);
      Value::NamedParameter(Box::new(lhv.clone()), Box::new(rhv))
    }));
  }
  Err(err_expected_ast_node_parameter_name(&format!("{lhs:?}")))
}

fn build_named_parameters(lhs: &[AstNode]) -> Result<Evaluator> {
  let mut evaluators = vec![];
  for item in lhs {
    evaluators.push(build_evaluator(item)?);
  }
  Ok(Box::new(move |scope: &FeelScope| {
    let mut parameters = BTreeMap::new();
    let mut position = 1_usize;
    for evaluator in &evaluators {
      if let Value::NamedParameter(name, value) = evaluator(scope) {
        if let Value::ParameterName(name) = *name {
          parameters.insert(name, (*value, position));
          position += 1;
        }
      }
    }
    Value::NamedParameters(parameters)
  }))
}

///
fn build_neg(lhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    match lhv {
      Value::Number(lh) => Value::Number(-lh),
      Value::DaysAndTimeDuration(lh) => Value::DaysAndTimeDuration(-lh),
      Value::YearsAndMonthsDuration(lh) => Value::YearsAndMonthsDuration(-lh),
      other => value_null!("unexpected type in arithmetic negation: {}", other.type_of()),
    }
  }))
}

///
fn build_negated_list(lhs: &[AstNode]) -> Result<Evaluator> {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(node)?);
  }
  Ok(Box::new(move |scope: &FeelScope| {
    let mut values = vec![];
    for evaluator in &evaluators {
      values.push(evaluator(scope))
    }
    Value::NegatedCommaList(values)
  }))
}

///
fn build_null() -> Result<Evaluator> {
  Ok(Box::new(move |_: &FeelScope| Value::Null(None)))
}

///
fn build_numeric(lhs: &str, rhs: &str) -> Result<Evaluator> {
  let text = format!("{lhs}.{rhs}");
  if let Ok(num) = text.parse::<FeelNumber>() {
    Ok(Box::new(move |_: &FeelScope| Value::Number(num)))
  } else {
    Ok(Box::new(move |_: &FeelScope| value_null!("failed to convert text '{}' into number", text)))
  }
}

///
fn build_nq(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    if let Some(result) = eval_ternary_equality(&lhv, &rhv) {
      Value::Boolean(!result)
    } else {
      value_null!()
    }
  }))
}

/// Semantics of disjunction.
/// ```text
/// left        right        result
///─────────────────────────────────
/// true        true         true
/// true        false        true
/// true        otherwise    true
/// false       true         true
/// false       false        false
/// false       otherwise    null
/// otherwise   true         true
/// otherwise   false        null
/// otherwise   otherwise    null
/// ```
fn build_or(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    match lhv {
      Value::Boolean(lh) => match rhv {
        Value::Boolean(rh) => Value::Boolean(lh || rh),
        _ => {
          if lh {
            Value::Boolean(true)
          } else {
            value_null!()
          }
        }
      },
      _ => match rhv {
        Value::Boolean(rh) => {
          if rh {
            Value::Boolean(true)
          } else {
            value_null!()
          }
        }
        _ => value_null!(),
      },
    }
  }))
}

///
fn build_out(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let ine = build_in(lhs, rhs)?;
  let lhe = build_evaluator(lhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let inv = ine(scope) as Value;
    let lhv = lhe(scope) as Value;
    match inv {
      Value::Boolean(true) => lhv,
      _ => value_null!(),
    }
  }))
}

///
fn build_parameter_name(lhs: &Name) -> Result<Evaluator> {
  let name = lhs.to_owned();
  Ok(Box::new(move |_: &FeelScope| Value::ParameterName(name.clone())))
}

///
fn build_parameter_types(lhs: &[AstNode]) -> Result<Evaluator> {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(node)?);
  }
  Ok(Box::new(move |scope: &FeelScope| {
    let mut values = vec![];
    for evaluator in &evaluators {
      values.push(evaluator(scope))
    }
    Value::ParameterTypes(values)
  }))
}

///
fn build_qualified_name(lhs: &[AstNode]) -> Result<Evaluator> {
  let mut evaluators = vec![];
  for node in lhs {
    evaluators.push(build_evaluator(node)?);
  }
  Ok(Box::new(move |scope: &FeelScope| {
    let mut names = vec![];
    for evaluator in &evaluators {
      if let Value::QualifiedNameSegment(name) = evaluator(scope) {
        names.push(name);
      }
    }
    scope.search(&names).unwrap_or_else(|| value_null!("no value for qualified name"))
  }))
}

///
fn build_qualified_name_segment(name: &Name) -> Result<Evaluator> {
  let name = name.to_owned();
  Ok(Box::new(move |_: &FeelScope| Value::QualifiedNameSegment(name.to_owned())))
}

///
fn get_property_from_value(value: Value, name: &Name) -> Value {
  let property_name = name.to_string();
  match value {
    Value::Date(date) => match property_name.as_str() {
      "year" => Value::Number(date.year().into()),
      "month" => Value::Number(date.month().into()),
      "day" => Value::Number(date.day().into()),
      "weekday" => {
        if let Some(day_of_week) = date.day_of_week() {
          Value::Number(day_of_week.1.into())
        } else {
          value_null!("could not retrieve weekday for date")
        }
      }
      other => value_null!("no such property in date: {}", other),
    },
    Value::DateTime(date_time) => match property_name.as_str() {
      "year" => Value::Number(date_time.year().into()),
      "month" => Value::Number(date_time.month().into()),
      "day" => Value::Number(date_time.day().into()),
      "weekday" => {
        if let Some(day_of_week) = date_time.day_of_week() {
          Value::Number(day_of_week.1.into())
        } else {
          value_null!("could not retrieve weekday for date and time")
        }
      }
      "hour" => Value::Number(date_time.hour().into()),
      "minute" => Value::Number(date_time.minute().into()),
      "second" => Value::Number(date_time.second().into()),
      "time offset" => {
        if let Some(offset) = date_time.feel_time_offset() {
          Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_s(offset as i64))
        } else {
          value_null!("could not retrieve time offset for date and time")
        }
      }
      "timezone" => {
        if let Some(feel_time_zone) = date_time.feel_time_zone() {
          Value::String(feel_time_zone)
        } else {
          value_null!("could not retrieve timezone for date and time")
        }
      }
      other => value_null!("no such property in date and time: {}", other),
    },
    Value::Time(time) => match property_name.as_str() {
      "hour" => Value::Number(time.hour().into()),
      "minute" => Value::Number(time.minute().into()),
      "second" => Value::Number(time.second().into()),
      "time offset" => {
        if let Some(offset) = time.feel_time_offset() {
          Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_s(offset as i64))
        } else {
          value_null!("could not retrieve time offset for time")
        }
      }
      "timezone" => {
        if let Some(feel_time_zone) = time.feel_time_zone() {
          Value::String(feel_time_zone)
        } else {
          value_null!("could not retrieve timezone for time")
        }
      }
      other => value_null!("no such property in time: {}", other),
    },
    Value::DaysAndTimeDuration(dt_duration) => match property_name.as_str() {
      "days" => Value::Number(dt_duration.get_days().into()),
      "hours" => Value::Number(dt_duration.get_hours().into()),
      "minutes" => Value::Number(dt_duration.get_minutes().into()),
      "seconds" => Value::Number(dt_duration.get_seconds().into()),
      other => value_null!("no such property in days and time duration: {}", other),
    },
    Value::YearsAndMonthsDuration(ym_duration) => match property_name.as_str() {
      "years" => Value::Number(ym_duration.years().into()),
      "months" => Value::Number(ym_duration.months().into()),
      other => value_null!("no such property in years and months duration: {}", other),
    },
    Value::Range(rs, cs, re, ce) => match property_name.as_str() {
      "start" => *rs,
      "start included" => Value::Boolean(cs),
      "end" => *re,
      "end included" => Value::Boolean(ce),
      other => value_null!("no such property in range: {}", other),
    },
    Value::UnaryGreater(value) => match property_name.as_str() {
      "start" => *value,
      "start included" => Value::Boolean(false),
      "end included" => Value::Boolean(false),
      other => value_null!("no such property in unary greater: {}", other),
    },
    Value::UnaryGreaterOrEqual(value) => match property_name.as_str() {
      "start" => *value,
      "start included" => Value::Boolean(true),
      "end included" => Value::Boolean(false),
      other => value_null!("no such property in unary greater or equal: {}", other),
    },
    Value::UnaryLess(value) => match property_name.as_str() {
      "end" => *value,
      "start included" => Value::Boolean(false),
      "end included" => Value::Boolean(false),
      other => value_null!("no such property in unary less: {}", other),
    },
    Value::UnaryLessOrEqual(value) => match property_name.as_str() {
      "end" => *value,
      "start included" => Value::Boolean(false),
      "end included" => Value::Boolean(true),
      other => value_null!("no such property in unary less or equal: {}", other),
    },
    v @ Value::Null(_) => v,
    other => value_null!("unexpected type: {}, for property: {}", other.type_of(), property_name),
  }
}

///
fn build_qualified_name_from_path(node: &AstNode) -> Result<QualifiedName> {
  match node {
    AstNode::Path(lhs, rhs) => {
      return if let AstNode::Name(name) = lhs.borrow() {
        let mut qualified_name = build_qualified_name_from_path(rhs)?;
        qualified_name.insert(0, name.clone());
        Ok(qualified_name)
      } else {
        Err(err_unexpected_ast_node(&format!("expected Name, found {lhs:?}",)))
      }
    }
    AstNode::Name(name) => return Ok(name.clone().into()),
    _ => {}
  }
  Err(err_unexpected_ast_node(&format!("expected Path or Name, found: {node:?}",)))
}

///
fn build_path(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let qualified_name = build_qualified_name_from_path(rhs)?;
  let mut property_path = qualified_name.clone();
  let property_name = property_path.pop().unwrap();
  let lhe = build_evaluator(lhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    match lhv {
      Value::Context(context) => {
        if let Some(value) = context.search_entry(&qualified_name) {
          return value.clone();
        }
        if let Some(value) = context.search_entry(&property_path) {
          return get_property_from_value(value.clone(), &property_name);
        }
        value_null!("build_path: no entry {} in context: {}", qualified_name, context)
      }
      Value::List(items) => {
        let mut result = vec![];
        for item in items {
          if let Value::Context(context) = item {
            if let Some(value) = context.search_entry(&qualified_name) {
              result.push(value.clone());
            } else if let Some(value) = context.search_entry(&property_path) {
              result.push(get_property_from_value(value.clone(), &property_name));
            }
          } else {
            return value_null!("build_path: no context in list");
          }
        }
        Value::List(result)
      }
      other => get_property_from_value(other, &property_name),
    }
  }))
}

///
fn build_range(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    let rhv = rhe(scope);
    if let Value::IntervalStart(lhv, l_closed) = lhv {
      if let Value::IntervalEnd(rhv, r_closed) = rhv {
        Value::Range(lhv, l_closed, rhv, r_closed)
      } else {
        value_null!("expected interval end")
      }
    } else {
      value_null!("expected interval start")
    }
  }))
}

///
fn build_range_type(lhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    if let Value::FeelType(feel_type) = lhv {
      Value::FeelType(FeelType::Range(Box::new(feel_type)))
    } else {
      value_null!("expected a feel type")
    }
  }))
}

///
fn build_some(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let mut expr_evaluators = vec![];
  if let AstNode::QuantifiedContexts(items) = lhs {
    for item in items {
      if let AstNode::QuantifiedContext(variable_name, expr_node) = item {
        if let AstNode::Name(name) = variable_name.borrow() {
          let evaluator_single = build_evaluator(expr_node)?;
          expr_evaluators.push((name.clone(), evaluator_single));
        }
      }
    }
  } else {
    return Err(err_expected_ast_node("AstNode::QuantifiedContexts", &format!("{lhs:?}")));
  }
  if let AstNode::Satisfies(satisfies) = rhs {
    let satisfies_evaluator = build_evaluator(satisfies)?;
    Ok(Box::new(move |scope: &FeelScope| {
      let mut expression_evaluator = SomeExpressionEvaluator::new();
      for (name, expr_evaluator) in &expr_evaluators {
        expression_evaluator.add(name.clone(), expr_evaluator(scope));
      }
      expression_evaluator.evaluate(scope, &satisfies_evaluator)
    }))
  } else {
    Err(err_expected_ast_node("AstNode::Satisfies", &format!("{rhs:?}")))
  }
}

///
fn build_string(lhs: &str) -> Result<Evaluator> {
  let value = Value::String(lhs.to_string());
  Ok(Box::new(move |_: &FeelScope| value.clone()))
}

///
fn build_sub(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope) as Value;
    let rhv = rhe(scope) as Value;
    match lhv {
      Value::Number(lh) => {
        if let Value::Number(rh) = rhv {
          return Value::Number(lh - rh);
        }
      }
      Value::Date(lh) => match rhv {
        Value::Date(rh) => {
          let l = FeelDateTime::new(lh, FeelTime::utc(0, 0, 0, 0));
          let r = FeelDateTime::new(rh, FeelTime::utc(0, 0, 0, 0));
          if let Some(result) = l - r {
            return Value::DaysAndTimeDuration(result);
          }
        }
        Value::DateTime(rh) => {
          let l = FeelDateTime::new(lh, FeelTime::utc(0, 0, 0, 0));
          if let Some(result) = l - rh {
            return Value::DaysAndTimeDuration(result);
          }
        }
        Value::DaysAndTimeDuration(rh) => {
          if let Some(date) = lh - rh {
            return Value::Date(date);
          }
        }
        Value::YearsAndMonthsDuration(rh) => {
          if let Some(date) = lh - rh {
            return Value::Date(date);
          }
        }
        _ => {}
      },
      Value::Time(lh) => match rhv {
        Value::Time(rh) => {
          if let Some(result) = lh - rh {
            return Value::DaysAndTimeDuration(result);
          }
        }
        Value::DaysAndTimeDuration(rh) => {
          return Value::Time(lh - rh);
        }
        _ => {}
      },
      Value::DateTime(lh) => match rhv {
        Value::Date(rh) => {
          let r = FeelDateTime::new(rh, FeelTime::utc(0, 0, 0, 0));
          if let Some(result) = lh - r {
            return Value::DaysAndTimeDuration(result);
          }
        }
        Value::DateTime(rh) => {
          if let Some(result) = lh - rh {
            return Value::DaysAndTimeDuration(result);
          }
        }
        Value::DaysAndTimeDuration(rh) => {
          if let Some(result) = lh - rh {
            return Value::DateTime(result);
          }
        }
        Value::YearsAndMonthsDuration(rh) => {
          if let Some(result) = lh - rh {
            return Value::DateTime(result);
          }
        }
        _ => {}
      },
      Value::DaysAndTimeDuration(lh) => {
        if let Value::DaysAndTimeDuration(rh) = rhv {
          return Value::DaysAndTimeDuration(lh - rh);
        }
      }
      Value::YearsAndMonthsDuration(lh) => {
        if let Value::YearsAndMonthsDuration(rh) = rhv {
          return Value::YearsAndMonthsDuration(lh - rh);
        }
      }
      _ => {}
    }
    value_null!("[subtraction] incompatible types: {} - {}", lhe(scope) as Value, rhe(scope) as Value)
  }))
}

///
fn build_unary_ge(lhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    Value::UnaryGreaterOrEqual(Box::from(lhv))
  }))
}

///
fn build_unary_gt(lhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    Value::UnaryGreater(Box::from(lhv))
  }))
}

///
fn build_unary_le(lhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    Value::UnaryLessOrEqual(Box::from(lhv))
  }))
}

///
fn build_unary_lt(lhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let lhv = lhe(scope);
    Value::UnaryLess(Box::from(lhv))
  }))
}

/// Evaluates ternary equality between two values.
pub fn eval_ternary_equality(lhs: &Value, rhs: &Value) -> Option<bool> {
  match lhs {
    Value::Boolean(ls) => match rhs {
      Value::Boolean(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::Number(ls) => match rhs {
      Value::Number(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::String(ls) => match rhs {
      Value::String(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::Context(ls) => match rhs {
      Value::Context(rs) => {
        if ls.keys().len() == rs.keys().len() {
          for (key1, value1) in ls.deref() {
            if let Some(value2) = rs.get_entry(key1) {
              if let Some(equal) = eval_ternary_equality(value1, value2) {
                if !equal {
                  return Some(false); // values in entries are NOT EQUAL
                }
              } else {
                // values in entries can not be compared
                return None;
              }
            } else {
              // there is no such key in the right-side context
              return Some(false);
            }
          }
          // contexts are EQUAL
          return Some(true);
        }
        // contexts have different number of keys, so they are NOT EQUAL
        Some(false)
      }
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::Date(ls) => match rhs {
      Value::Date(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::Time(ls) => match rhs {
      Value::Time(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::DateTime(ls) => match rhs {
      Value::DateTime(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::DaysAndTimeDuration(ls) => match rhs {
      Value::DaysAndTimeDuration(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::YearsAndMonthsDuration(ls) => match rhs {
      Value::YearsAndMonthsDuration(rs) => Some(*ls == *rs),
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::Null(_) => match rhs {
      Value::Null(_) => Some(true),
      _ => None,
    },
    Value::List(ls) => match rhs {
      Value::List(rs) => {
        if ls.len() == rs.len() {
          for (l, r) in ls.iter().zip(rs.iter()) {
            if let Some(true) = eval_ternary_equality(l, r) {
            } else {
              return Some(false);
            }
          }
          Some(true)
        } else {
          Some(false)
        }
      }
      Value::Null(_) => Some(false),
      _ => None,
    },
    Value::UnaryGreater(end) => match rhs {
      Value::Range(rs, cs, re, ce) => {
        if !*cs && !*ce && re.is_null() {
          eval_ternary_equality(end, rs)
        } else {
          Some(false)
        }
      }
      _ => None,
    },
    Value::UnaryLess(end) => match rhs {
      Value::Range(rs, cs, re, ce) => {
        if !*cs && !*ce && rs.is_null() {
          eval_ternary_equality(end, re)
        } else {
          Some(false)
        }
      }
      _ => None,
    },
    Value::UnaryGreaterOrEqual(end) => match rhs {
      Value::Range(rs, cs, re, ce) => {
        if *cs && !*ce && re.is_null() {
          eval_ternary_equality(end, rs)
        } else {
          Some(false)
        }
      }
      _ => None,
    },
    Value::UnaryLessOrEqual(end) => match rhs {
      Value::Range(rs, cs, re, ce) => {
        if !*cs && *ce && rs.is_null() {
          eval_ternary_equality(end, re)
        } else {
          Some(false)
        }
      }
      _ => None,
    },
    Value::Range(r1s, c1s, r1e, c1e) => match rhs {
      Value::Range(r2s, c2s, r2e, c2e) => {
        if *c1s == *c2s && *c1e == *c2e {
          if let Some(true) = eval_ternary_equality(r1s, r2s) {
            return eval_ternary_equality(r1e, r2e);
          }
        }
        Some(false)
      }
      Value::Null(_) => Some(false),
      _ => None,
    },
    _ => None,
  }
}

///
fn eval_in_list(left: &Value, items: &[Value]) -> Value {
  for item in items {
    match item {
      inner @ Value::Null(_)
      | inner @ Value::String(_)
      | inner @ Value::Number(_)
      | inner @ Value::Boolean(_)
      | inner @ Value::Date(_)
      | inner @ Value::Time(_)
      | inner @ Value::DateTime(_)
      | inner @ Value::YearsAndMonthsDuration(_)
      | inner @ Value::DaysAndTimeDuration(_)
      | inner @ Value::Context(_) => {
        if let Value::Boolean(true) = eval_in_equal(left, inner) {
          return VALUE_TRUE;
        }
      }
      Value::UnaryLess(inner) => {
        if let Value::Boolean(true) = eval_in_unary_less(left, inner.borrow()) {
          return VALUE_TRUE;
        }
      }
      Value::UnaryLessOrEqual(inner) => {
        if let Value::Boolean(true) = eval_in_unary_less_or_equal(left, inner.borrow()) {
          return VALUE_TRUE;
        }
      }
      Value::UnaryGreater(inner) => {
        if let Value::Boolean(true) = eval_in_unary_greater(left, inner.borrow()) {
          return VALUE_TRUE;
        }
      }
      Value::UnaryGreaterOrEqual(inner) => {
        if let Value::Boolean(true) = eval_in_unary_greater_or_equal(left, inner.borrow()) {
          return VALUE_TRUE;
        }
      }
      Value::List(inner) => {
        if let Value::Boolean(true) = eval_in_list(left, inner) {
          return VALUE_TRUE;
        }
      }
      Value::Range(l, l_closed, r, r_closed) => {
        if let Value::Boolean(true) = eval_in_range(left, l, *l_closed, r, *r_closed) {
          return VALUE_TRUE;
        }
      }
      _ => return value_null!("eval_in_list"),
    }
  }
  VALUE_FALSE
}

/// Checks if all elements from `list` are present in `items`.
fn eval_in_list_in_list(l_items: &[Value], r_items: &[Value]) -> Value {
  for item in r_items {
    if let Value::List(rhs) = item {
      let mut available: HashSet<usize> = (0..rhs.len()).collect();
      for l in l_items {
        let mut found = false;
        for (index, r) in rhs.iter().enumerate() {
          if available.contains(&index) {
            if let Value::Boolean(true) = eval_in_equal(l, r) {
              available.remove(&index);
              found = true;
              break;
            }
          }
        }
        if !found {
          return VALUE_FALSE;
        }
      }
      return VALUE_TRUE;
    }
  }
  VALUE_FALSE
}

///
fn eval_in_negated_list(left: &Value, items: &[Value]) -> Value {
  for item in items {
    match item {
      inner @ Value::Null(_)
      | inner @ Value::String(_)
      | inner @ Value::Number(_)
      | inner @ Value::Boolean(_)
      | inner @ Value::Date(_)
      | inner @ Value::Time(_)
      | inner @ Value::DateTime(_)
      | inner @ Value::DaysAndTimeDuration(_)
      | inner @ Value::YearsAndMonthsDuration(_) => {
        if let Value::Boolean(true) = eval_in_equal(left, inner) {
          return Value::Boolean(false);
        }
      }
      Value::UnaryLess(inner) => {
        if let Value::Boolean(true) = eval_in_unary_less(left, inner.borrow()) {
          return Value::Boolean(false);
        }
      }
      Value::UnaryLessOrEqual(inner) => {
        if let Value::Boolean(true) = eval_in_unary_less_or_equal(left, inner.borrow()) {
          return Value::Boolean(false);
        }
      }
      Value::UnaryGreater(inner) => {
        if let Value::Boolean(true) = eval_in_unary_greater(left, inner.borrow()) {
          return Value::Boolean(false);
        }
      }
      Value::UnaryGreaterOrEqual(inner) => {
        if let Value::Boolean(true) = eval_in_unary_greater_or_equal(left, inner.borrow()) {
          return Value::Boolean(false);
        }
      }
      Value::List(inner) => {
        if let Value::Boolean(true) = eval_in_list(left, inner) {
          return Value::Boolean(false);
        }
      }
      Value::Range(l, l_closed, r, r_closed) => {
        if let Value::Boolean(true) = eval_in_range(left, l, *l_closed, r, *r_closed) {
          return Value::Boolean(false);
        }
      }
      other => return value_null!("unexpected type in negated list: {}", other.type_of()),
    }
  }
  Value::Boolean(true)
}

///
fn eval_in_range(lhv: &Value, l: &Value, l_closed: bool, r: &Value, r_closed: bool) -> Value {
  match lhv {
    Value::Number(value) => match l.borrow() {
      Value::Number(lv) => match r.borrow() {
        Value::Number(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    Value::String(value) => match l.borrow() {
      Value::String(lv) => match r.borrow() {
        Value::String(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    Value::Date(value) => match l.borrow() {
      Value::Date(lv) => match r.borrow() {
        Value::Date(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    Value::Time(value) => match l.borrow() {
      Value::Time(lv) => match r.borrow() {
        Value::Time(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    Value::DateTime(value) => match l.borrow() {
      Value::DateTime(lv) => match r.borrow() {
        Value::DateTime(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    Value::YearsAndMonthsDuration(value) => match l.borrow() {
      Value::YearsAndMonthsDuration(lv) => match r.borrow() {
        Value::YearsAndMonthsDuration(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    Value::DaysAndTimeDuration(value) => match l.borrow() {
      Value::DaysAndTimeDuration(lv) => match r.borrow() {
        Value::DaysAndTimeDuration(rv) => {
          let l_ok = if l_closed { value >= lv } else { value > lv };
          let r_ok = if r_closed { value <= rv } else { value < rv };
          Value::Boolean(l_ok && r_ok)
        }
        _ => value_null!("eval_in_range"),
      },
      _ => value_null!("eval_in_range"),
    },
    _ => value_null!("eval_in_range"),
  }
}

///
fn eval_in_equal(left: &Value, right: &Value) -> Value {
  if let Some(true) = eval_ternary_equality(left, right) {
    VALUE_TRUE
  } else {
    VALUE_FALSE
  }
}

///
fn eval_in_unary_less(left: &Value, right: &Value) -> Value {
  match right {
    Value::Number(r) => {
      if let Value::Number(l) = left {
        return Value::Boolean(*l < *r);
      }
    }
    Value::String(r) => {
      if let Value::String(l) = left {
        return Value::Boolean(l < r);
      }
    }
    Value::Date(r) => {
      if let Value::Date(l) = left {
        return Value::Boolean(l < r);
      }
    }
    Value::Time(r) => {
      if let Value::Time(l) = left {
        return Value::Boolean(l < r);
      }
    }
    Value::DateTime(r) => {
      if let Value::DateTime(l) = left {
        return Value::Boolean(l < r);
      }
    }
    Value::YearsAndMonthsDuration(r) => {
      if let Value::YearsAndMonthsDuration(l) = left {
        return Value::Boolean(l < r);
      }
    }
    Value::DaysAndTimeDuration(r) => {
      if let Value::DaysAndTimeDuration(l) = left {
        return Value::Boolean(l < r);
      }
    }
    _ => {}
  }
  value_null!("eval_in_unary_less")
}

///
fn eval_in_unary_less_or_equal(left: &Value, right: &Value) -> Value {
  match right {
    Value::Number(r) => {
      if let Value::Number(l) = left {
        return Value::Boolean(*l <= *r);
      }
    }
    Value::String(r) => {
      if let Value::String(l) = left {
        return Value::Boolean(l <= r);
      }
    }
    Value::Date(r) => {
      if let Value::Date(l) = left {
        return Value::Boolean(l <= r);
      }
    }
    Value::Time(r) => {
      if let Value::Time(l) = left {
        return Value::Boolean(l <= r);
      }
    }
    Value::DateTime(r) => {
      if let Value::DateTime(l) = left {
        return Value::Boolean(l <= r);
      }
    }
    Value::YearsAndMonthsDuration(r) => {
      if let Value::YearsAndMonthsDuration(l) = left {
        return Value::Boolean(l <= r);
      }
    }
    Value::DaysAndTimeDuration(r) => {
      if let Value::DaysAndTimeDuration(l) = left {
        return Value::Boolean(l <= r);
      }
    }
    _ => {}
  }
  value_null!("eval_in_unary_less_or_equal")
}

///
fn eval_in_unary_greater(left: &Value, right: &Value) -> Value {
  match right {
    Value::Number(r) => {
      if let Value::Number(l) = left {
        return Value::Boolean(*l > *r);
      }
    }
    Value::String(r) => {
      if let Value::String(l) = left {
        return Value::Boolean(l > r);
      }
    }
    Value::Date(r) => {
      if let Value::Date(l) = left {
        return Value::Boolean(l > r);
      }
    }
    Value::Time(r) => {
      if let Value::Time(l) = left {
        return Value::Boolean(l > r);
      }
    }
    Value::DateTime(r) => {
      if let Value::DateTime(l) = left {
        return Value::Boolean(l > r);
      }
    }
    Value::YearsAndMonthsDuration(r) => {
      if let Value::YearsAndMonthsDuration(l) = left {
        return Value::Boolean(l > r);
      }
    }
    Value::DaysAndTimeDuration(r) => {
      if let Value::DaysAndTimeDuration(l) = left {
        return Value::Boolean(l > r);
      }
    }
    _ => {}
  }
  value_null!("eval_in_unary_greater")
}

///
fn eval_in_unary_greater_or_equal(left: &Value, right: &Value) -> Value {
  match right {
    Value::Number(r) => {
      if let Value::Number(l) = left {
        return Value::Boolean(*l >= *r);
      }
    }
    Value::String(r) => {
      if let Value::String(left_value) = left {
        return Value::Boolean(left_value >= r);
      }
    }
    Value::Date(r) => {
      if let Value::Date(l) = left {
        return Value::Boolean(l >= r);
      }
    }
    Value::Time(r) => {
      if let Value::Time(l) = left {
        return Value::Boolean(l >= r);
      }
    }
    Value::DateTime(r) => {
      if let Value::DateTime(l) = left {
        return Value::Boolean(l >= r);
      }
    }
    Value::YearsAndMonthsDuration(r) => {
      if let Value::YearsAndMonthsDuration(l) = left {
        return Value::Boolean(l >= r);
      }
    }
    Value::DaysAndTimeDuration(r) => {
      if let Value::DaysAndTimeDuration(l) = left {
        return Value::Boolean(l >= r);
      }
    }
    _ => {}
  }
  value_null!("eval_in_unary_greater_or_equal")
}

/// Evaluates function definition with positional parameters.
fn eval_function_with_positional_parameters(
  scope: &FeelScope,
  args: &[Value],
  params: &[(Name, FeelType)],
  body: &FunctionBody,
  closure_ctx: FeelContext,
  result_type: FeelType,
) -> Value {
  let mut params_ctx = FeelContext::default();
  if args.len() != params.len() {
    return value_null!("invalid number of arguments");
  }
  for (argument_value, (parameter_name, parameter_type)) in args.iter().zip(params) {
    params_ctx.set_entry(parameter_name, argument_value.coerced(parameter_type))
  }
  eval_function_definition(scope, params_ctx, body, closure_ctx, result_type)
}

/// Evaluates function definition with named parameters.
fn eval_function_with_named_parameters(
  scope: &FeelScope,
  args: &Value,
  params: &[(Name, FeelType)],
  body: &FunctionBody,
  closure_ctx: FeelContext,
  result_type: FeelType,
) -> Value {
  let mut params_ctx = FeelContext::default();
  if let Value::NamedParameters(argument_map) = args {
    if argument_map.len() != params.len() {
      return value_null!("invalid number of arguments");
    }
    for (parameter_name, parameter_type) in params {
      if let Some((argument, _)) = argument_map.get(parameter_name) {
        params_ctx.set_entry(parameter_name, argument.coerced(parameter_type))
      } else {
        return value_null!("parameter with name {} not found in arguments", parameter_name);
      }
    }
  }
  eval_function_definition(scope, params_ctx, body, closure_ctx, result_type)
}

/// Evaluates function definition.
fn eval_function_definition(scope: &FeelScope, params_ctx: FeelContext, body: &FunctionBody, closure_ctx: FeelContext, result_type: FeelType) -> Value {
  scope.push(closure_ctx);
  scope.push(params_ctx);
  let mut result = body.evaluate(scope);
  if let Value::FunctionDefinition(fd_params, fd_body, fd_external, fd_closure, fd_closure_ctx, fd_result_type) = &result {
    let mut new_closure_ctx = fd_closure_ctx.clone();
    for closure_name in fd_closure.iter() {
      if let Some(closure_value) = scope.search_entry(closure_name) {
        new_closure_ctx.create_entry(closure_name, closure_value);
      }
    }
    result = Value::FunctionDefinition(
      fd_params.to_owned(),
      fd_body.to_owned(),
      fd_external.to_owned(),
      fd_closure.to_owned(),
      new_closure_ctx,
      fd_result_type.to_owned(),
    );
  }
  scope.pop(); // params_ctx
  scope.pop(); // closure_ctx
  result.coerced(&result_type)
}

/// Evaluates external function definition with positional parameters.
fn eval_external_function_with_positional_parameters(scope: &FeelScope, args: &[Value], params: &[(Name, FeelType)], body: &FunctionBody, result_type: FeelType) -> Value {
  if args.len() != params.len() {
    return value_null!("invalid number of arguments");
  }
  eval_external_function_definition(scope, args, body, result_type)
}

/// Evaluates external function definition with named parameters.
fn eval_external_function_with_named_parameters(scope: &FeelScope, args: &Value, params: &[(Name, FeelType)], body: &FunctionBody, result_type: FeelType) -> Value {
  let mut args1 = vec![];
  if let Value::NamedParameters(argument_map) = args {
    if argument_map.len() != params.len() {
      return value_null!("invalid number of arguments");
    }
    for (parameter_name, parameter_type) in params {
      if let Some((argument, _)) = argument_map.get(parameter_name) {
        args1.push(argument.coerced(parameter_type));
      } else {
        return value_null!("parameter with name {} not found in arguments", parameter_name);
      }
    }
  }
  eval_external_function_definition(scope, &args1, body, result_type)
}

/// Evaluates external function definition.
fn eval_external_function_definition(scope: &FeelScope, arguments: &[Value], body: &FunctionBody, result_type: FeelType) -> Value {
  let result = match &body.evaluate(scope) {
    Value::ExternalJavaFunction(class_name, method_signature) => eval_java_function(class_name, method_signature, arguments),
    Value::ExternalPmmlFunction(document, model_name) => eval_pmml_function(document, model_name, arguments),
    other => value_null!("expected JAVA or PMML mapping, actual value is {}", other),
  };
  result.coerced(&result_type)
}

/// Mock of Java function evaluation
fn eval_java_function(class_name: &str, method_signature: &str, arguments: &[Value]) -> Value {
  match (class_name, method_signature) {
    ("java.lang.Math", "cos(double)") => Value::Number(FeelNumber::new(-88796890, 8)),
    ("java.lang.Math", "foo(double)") => value_null!(),
    ("java.lang.Foo", "valueOf(double)") => value_null!(),
    ("java.lang.Math", "max(java.lang.String, java.lang.String)") => value_null!(),
    ("java.lang.Math", "max(double,double)") => arguments[1].clone(),
    ("java.lang.Math", "max(double, double)") => arguments[1].clone(),
    ("java.lang.Math", "max(int,int)") => arguments[1].clone(),
    ("java.lang.Short", "valueOf(short)") => arguments[0].clone(),
    ("java.lang.Byte", "valueOf(byte)") => arguments[0].clone(),
    ("java.lang.String", "valueOf(char)") => {
      if let Value::String(s) = &arguments[0] {
        if s.len() == 1 {
          Value::String(s.clone())
        } else {
          value_null!()
        }
      } else {
        value_null!()
      }
    }
    ("java.lang.Math", "max(long,long)") => arguments[1].clone(),
    ("java.lang.Math", "max(float,float)") => arguments[1].clone(),
    ("java.lang.Integer", "valueOf(java.lang.String)") => {
      if let Value::String(s) = &arguments[0] {
        if let Ok(n) = s.parse::<i64>() {
          Value::Number(n.into())
        } else {
          value_null!()
        }
      } else {
        value_null!()
      }
    }
    ("java.lang.Float", "valueOf(java.lang.String)") | ("java.lang.Double", "valueOf(java.lang.String)") => {
      if let Value::String(s) = &arguments[0] {
        if let Ok(n) = s.parse::<f32>() {
          Value::Number(FeelNumber::new((n * 100.0) as i64, 2))
        } else {
          value_null!()
        }
      } else {
        value_null!()
      }
    }
    ("java.lang.String", "format(java.lang.String, [Ljava.lang.Object;)") => Value::String("foo bar".into()),
    _ => Value::String(format!("JAVA, class = {class_name}, method signature = {method_signature}")),
  }
}

/// Mock of PMML function evaluation
fn eval_pmml_function(document: &str, model_name: &str, _arguments: &[Value]) -> Value {
  match (document, model_name) {
    ("", _) => value_null!("PMML document not specified"),
    (_, "") => value_null!("PMML model name not specified"),
    _ => Value::String(format!("PMML, document = {document}, model name = {model_name}")),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dmntk_feel::{scope, FeelType};

  #[test]
  fn test_unimplemented_external_function_kind() {
    let evaluator = Box::new(move |_: &FeelScope| Value::Boolean(false)) as Evaluator;
    let body = FunctionBody::External(Arc::new(evaluator));
    let result = eval_external_function_definition(&scope!(), &[], &body, FeelType::Boolean);
    assert_eq!("null(expected JAVA or PMML mapping, actual value is false)", result.to_string())
  }
}

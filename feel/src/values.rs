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

//! `FEEL` values.

use self::errors::*;
use crate::bif::Bif;
use crate::context::FeelContext;
use crate::names::Name;
use crate::strings::ToFeelString;
use crate::types::FeelType;
use crate::FunctionBody;
use dmntk_common::{Jsonify, Result};
use dmntk_feel_number::FeelNumber;
use dmntk_feel_temporal::{FeelDate, FeelDateTime, FeelDaysAndTimeDuration, FeelTime, FeelYearsAndMonthsDuration};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::ops::Deref;
use std::str::FromStr;

/// Creates `Value::Null` with optional tracing message.
///
/// # Examples
///
/// ```
/// use crate::dmntk_feel::{value_null, values::Value};
///
/// let v = value_null!();
/// assert_eq!("null", v.to_string());
///
/// let v = value_null!("missing input parameter");
/// assert_eq!("null(missing input parameter)", v.to_string());
///
/// let v = value_null!("integer out of range {}..{}", 1, 100);
/// assert_eq!("null(integer out of range 1..100)", v.to_string());
/// ```
#[macro_export]
macro_rules! value_null {
  ($module:expr, $function:literal, $format:literal, $($arguments:tt)*) => {
    Value::Null(Some(format!("[{}::{}] {}", $module, $function, format!($format, $($arguments)*))))
  };
  ($format:literal, $($arguments:tt)*) => {
    Value::Null(Some(format!($format, $($arguments)*)))
  };
  ($argument:expr) => {
    Value::Null(Some(format!("{}", $argument)))
  };
  () => {
    Value::Null(None)
  };
}

#[macro_export]
macro_rules! value_number {
  ($n:expr) => {{
    Value::Number($n.into())
  }};
  ($n:expr, $s:expr) => {
    Value::Number(FeelNumber::new($n, $s))
  };
}

#[macro_export]
macro_rules! value_string {
  ($s:literal) => {{
    Value::String($s.to_string())
  }};
  ($s:expr) => {{
    Value::String($s)
  }};
}

/// Utility constant for value `true `of type `Boolean`.
pub const VALUE_TRUE: Value = Value::Boolean(true);

/// Utility constant for value `false` of type `Boolean`.
pub const VALUE_FALSE: Value = Value::Boolean(false);

/// `FEEL` value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  /// Value representing `FEEL` boolean type.
  Boolean(bool),

  /// Value for storing built-in function definition.
  BuiltInFunction(Bif),

  /// Value representing a collection of comma-separated list of expressions.
  ExpressionList(Values),

  /// Value representing a context.
  Context(FeelContext),

  /// Value representing a context entry.
  ContextEntry(Name, Box<Value>),

  /// Value representing a key of the context entry.
  ContextEntryKey(Name),

  /// Value representing the context type.
  ContextType(FeelType),

  /// Value representing a context entry in context type definition.
  ContextTypeEntry(Name, FeelType),

  /// Value representing a key of the context entry in context type definition.
  ContextTypeEntryKey(Name),

  /// Value for storing dates as [FeelDate].
  Date(FeelDate),

  /// Value for storing date and time as [FeelDateTime].
  DateTime(FeelDateTime),

  /// Value for days and time durations.
  DaysAndTimeDuration(FeelDaysAndTimeDuration),

  /// Value representing the `FEEL` type of a value.
  FeelType(FeelType),

  /// Value representing function's formal parameter with name and type.
  FormalParameter(Name, FeelType),

  /// List of formal parameters.
  FormalParameters(Vec<(Name, FeelType)>),

  /// Definition of the function body.
  /// Holds function body definition and the closure for lambdas.
  FunctionBody(FunctionBody, FeelContext),

  /// Value representing the function definition.
  /// This value holds the list of function's formal parameters, the function's body, closure for lambdas and expected result type.
  FunctionDefinition(Vec<(Name, FeelType)>, FunctionBody, FeelContext, FeelType),

  /// Value representing interval end.
  IntervalEnd(Box<Value>, bool),

  /// Value representing interval start.
  IntervalStart(Box<Value>, bool),

  /// Value representing `FEEL` `irrelevant` value.
  Irrelevant,

  /// Value representing a list of values.
  List(Values),

  /// Named parameter.
  NamedParameter(Box<Value>, Box<Value>),

  /// Value representing a collection of named parameters.
  NamedParameters(BTreeMap<Name, (Value, usize)>),

  /// Value representing a collection of values representing a negated comma-separated list of expressions.
  NegatedCommaList(Values),

  /// Null value with optional tracing message.
  Null(Option<String>),

  /// Value representing `FEEL` number type.
  Number(FeelNumber),

  /// Name of the parameter.
  ParameterName(Name),

  /// Value representing a list of function's parameter types.
  ParameterTypes(Vec<Value>),

  /// List of positional parameters.
  PositionalParameters(Values),

  /// Value representing a segment of a qualified name.
  QualifiedNameSegment(Name),

  /// Value representing a `range`.
  Range(Box<Value>, bool, Box<Value>, bool),

  /// `String` value...
  String(String),

  /// Value for storing time as [FeelTime].
  Time(FeelTime),

  /// **UnaryGreater** value...
  UnaryGreater(Box<Value>),

  /// **UnaryGreaterOrEqual** value...
  UnaryGreaterOrEqual(Box<Value>),

  /// **UnaryLess** value...
  UnaryLess(Box<Value>),

  /// **UnaryLessOrEqual** value...
  UnaryLessOrEqual(Box<Value>),

  /// Value for storing years and months duration.
  YearsAndMonthsDuration(FeelYearsAndMonthsDuration),
}

impl std::fmt::Display for Value {
  ///
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Boolean(value) => write!(f, "{value}"),
      Value::BuiltInFunction(_) => write!(f, "BuiltInFunction"),
      Value::ExpressionList(items) => write!(f, "{items}"),
      Value::Context(context) => write!(f, "{context}"),
      Value::ContextEntry(_, _) => write!(f, "ContextEntry"),
      Value::ContextEntryKey(name) => write!(f, "{name}"),
      Value::ContextType(_) => write!(f, "ContextType"),
      Value::ContextTypeEntry(name, feel_type) => write!(f, "{name}: {feel_type}"),
      Value::ContextTypeEntryKey(name) => write!(f, "{name}"),
      Value::Date(date) => write!(f, "{date}"),
      Value::DateTime(date_time) => write!(f, "{date_time}"),
      Value::DaysAndTimeDuration(dt_duration) => write!(f, "{dt_duration}"),
      Value::FeelType(feel_type) => write!(f, "type({feel_type})"),
      Value::FormalParameter(_, _) => write!(f, "FormalParameter"),
      Value::FormalParameters(_) => write!(f, "FormalParameters"),
      Value::FunctionBody(_, _) => write!(f, "FunctionBody"),
      Value::FunctionDefinition { .. } => write!(f, "FunctionDefinition"),
      Value::IntervalEnd(_, _) => write!(f, "IntervalEnd"),
      Value::IntervalStart(_, _) => write!(f, "IntervalStart"),
      Value::Irrelevant => write!(f, "Irrelevant"),
      Value::List(items) => write!(f, "{items}"),
      Value::NamedParameter(_, _) => write!(f, "NamedParameter"),
      Value::NamedParameters(_) => write!(f, "NamedParameters"),
      Value::NegatedCommaList(_) => write!(f, "NegatedCommaList"),
      Value::Number(value) => write!(f, "{value}"),
      Value::Null(trace) => write!(f, "null{}", trace.as_ref().map_or("".to_string(), |s| format!("({s})"))),
      Value::ParameterName(_) => write!(f, "ParameterName"),
      Value::ParameterTypes(_) => write!(f, "ParameterTypes"),
      Value::PositionalParameters(_) => write!(f, "PositionalParameters"),
      Value::QualifiedNameSegment(_) => write!(f, "QualifiedNameSegment"),
      Value::Range(v1, c1, v2, c2) => write!(f, "{}{}..{}{}", if *c1 { '[' } else { '(' }, v1, v2, if *c2 { ']' } else { ')' }),
      Value::String(s) => write!(f, "\"{s}\""),
      Value::Time(time) => write!(f, "{time}"),
      Value::UnaryGreater(_) => write!(f, "UnaryGreater"),
      Value::UnaryGreaterOrEqual(_) => write!(f, "UnaryGreaterOrEqual"),
      Value::UnaryLess(_) => write!(f, "UnaryLess"),
      Value::UnaryLessOrEqual(value) => write!(f, "UnaryLessOrEqual({value})"),
      Value::YearsAndMonthsDuration(ym_duration) => write!(f, "{ym_duration}"),
    }
  }
}

impl ToFeelString for Value {
  /// Converts [Value] into `FEEL` string.
  fn to_feel_string(&self) -> String {
    match self {
      Value::Context(context) => context.to_feel_string(),
      Value::List(items) => items.to_feel_string(),
      Value::String(value) => format!("\"{}\"", value.replace('"', "\\\"")),
      other => other.to_string(),
    }
  }
}

impl Jsonify for Value {
  /// Converts a [Value] to its `JSON` representation.
  fn jsonify(&self) -> String {
    match self {
      Value::Boolean(value) => format!("{value}"),
      Value::ExpressionList(items) => items.to_string(),
      Value::Context(ctx) => ctx.jsonify(),
      Value::ContextEntryKey(name) => name.to_string(),
      Value::List(items) => items.jsonify(),
      Value::Number(value) => value.jsonify(),
      Value::Null(_) => "null".to_string(),
      Value::String(s) => format!("\"{s}\""),
      _ => format!("jsonify not implemented for: {self}"),
    }
  }
}

impl Value {
  /// Returns `true` when the value is of type [Value::Null].
  pub fn is_null(&self) -> bool {
    matches!(self, Value::Null(_))
  }
  /// Returns `true` when the value is of type [Value::Boolean] and is equal to `true`.
  pub fn is_true(&self) -> bool {
    matches!(self, Value::Boolean(true))
  }
  /// Returns `true` when the value is of type [Value::Number].
  pub fn is_number(&self) -> bool {
    matches!(self, Value::Number(_))
  }
  /// Returns the type of this [Value].
  pub fn type_of(&self) -> FeelType {
    match self {
      Value::Boolean(_) => FeelType::Boolean,
      Value::BuiltInFunction(_) => FeelType::Any,
      Value::ExpressionList(_) => FeelType::Any,
      Value::Context(context) => {
        let mut entries = BTreeMap::new();
        for (name, value) in context.deref() {
          entries.insert(name.clone(), value.type_of());
        }
        FeelType::Context(entries)
      }
      Value::ContextEntry(_, _) => FeelType::Any,
      Value::ContextEntryKey(_) => FeelType::Any,
      Value::ContextType(feel_type) => feel_type.clone(),
      Value::ContextTypeEntry(_, feel_type) => feel_type.clone(),
      Value::ContextTypeEntryKey(_) => FeelType::Any,
      Value::Date(_) => FeelType::Date,
      Value::DateTime(_) => FeelType::DateTime,
      Value::DaysAndTimeDuration(_) => FeelType::DaysAndTimeDuration,
      Value::FeelType(feel_type) => feel_type.clone(),
      Value::FormalParameter(_, feel_type) => feel_type.clone(),
      Value::FormalParameters(_) => FeelType::Any,
      Value::FunctionBody(_, _) => FeelType::Any,
      Value::FunctionDefinition(parameters, _, _, result_type) => {
        let parameter_types = parameters.iter().map(|(_, feel_type)| feel_type.clone()).collect();
        FeelType::Function(parameter_types, Box::new(result_type.clone()))
      }
      Value::IntervalEnd(interval_end, _) => interval_end.type_of(),
      Value::IntervalStart(interval_start, _) => interval_start.type_of(),
      Value::Irrelevant => FeelType::Any,
      Value::List(values) => {
        if values.as_vec().is_empty() {
          FeelType::List(Box::new(FeelType::Null))
        } else {
          let item_type = values.as_vec()[0].type_of();
          for item in values.as_vec() {
            if !item.type_of().is_conformant(&item_type) {
              return FeelType::List(Box::new(FeelType::Any));
            }
          }
          FeelType::List(Box::new(item_type))
        }
      }
      Value::NamedParameter(_, _) => FeelType::Any,
      Value::NamedParameters(_) => FeelType::Any,
      Value::NegatedCommaList(_) => FeelType::Any,
      Value::Null(_) => FeelType::Null,
      Value::Number(_) => FeelType::Number,
      Value::ParameterName(_) => FeelType::Any,
      Value::ParameterTypes(_) => FeelType::Any,
      Value::PositionalParameters(_) => FeelType::Any,
      Value::QualifiedNameSegment(_) => FeelType::Any,
      Value::Range(range_start, _, range_end, _) => {
        let range_start_type = range_start.type_of();
        let range_end_type = range_end.type_of();
        if range_start_type == range_end_type {
          return FeelType::Range(Box::new(range_start_type));
        }
        FeelType::Range(Box::new(FeelType::Any))
      }
      Value::String(_) => FeelType::String,
      Value::Time(_) => FeelType::Time,
      Value::UnaryGreater(_) => FeelType::Boolean,
      Value::UnaryGreaterOrEqual(_) => FeelType::Boolean,
      Value::UnaryLess(_) => FeelType::Boolean,
      Value::UnaryLessOrEqual(_) => FeelType::Boolean,
      Value::YearsAndMonthsDuration(_) => FeelType::YearsAndMonthsDuration,
    }
  }
  /// Tries to convert `xsd:integer` string into valid [Value] representing a number.
  pub fn try_from_xsd_integer(text: &str) -> Result<Self> {
    let value = text.parse::<FeelNumber>().map_err(|_| err_invalid_xsd_integer(text))?;
    Ok(Value::Number(value))
  }
  /// Tries to convert `xsd:decimal` string into valid [Value] representing a number.
  pub fn try_from_xsd_decimal(text: &str) -> Result<Self> {
    let value = text.parse::<FeelNumber>().map_err(|_| err_invalid_xsd_decimal(text))?;
    Ok(Value::Number(value))
  }
  /// Tries to convert `xsd:double` string into valid [Value] representing a number.
  pub fn try_from_xsd_double(text: &str) -> Result<Self> {
    let value = text.parse::<FeelNumber>().map_err(|_| err_invalid_xsd_double(text))?;
    Ok(Value::Number(value))
  }
  /// Tries to convert `xsd:boolean` string into valid [Value] representing a boolean.
  pub fn try_from_xsd_boolean(text: &str) -> Result<Self> {
    match text {
      "true" | "1" => Ok(Value::Boolean(true)),
      "false" | "0" => Ok(Value::Boolean(false)),
      _ => Err(err_invalid_xsd_boolean(text)),
    }
  }
  /// Tries to convert `xsd:date` string into valid [Value] representing a date.
  /// FEEL date format is fully conformant with `xsd:date`.
  pub fn try_from_xsd_date(text: &str) -> Result<Self> {
    if let Ok(feel_date) = FeelDate::from_str(text) {
      return Ok(Value::Date(feel_date));
    }
    Err(err_invalid_xsd_date(text))
  }
  /// Tries to convert `xsd:time` string into valid [Value] representing a time.
  /// FEEL time format is fully conformant with `xsd:time`.
  pub fn try_from_xsd_time(text: &str) -> Result<Self> {
    if let Ok(feel_time) = FeelTime::from_str(text) {
      return Ok(Value::Time(feel_time));
    }
    Err(err_invalid_xsd_time(text))
  }
  /// Tries to convert `xsd:dateTime` string into valid [Value] representing a date and time.
  /// FEEL date and time format is fully conformant with `xsd:dateTime`.
  pub fn try_from_xsd_date_time(text: &str) -> Result<Self> {
    Ok(Value::DateTime(FeelDateTime::try_from(text).map_err(|_| err_invalid_xsd_date_time(text))?))
  }
  /// Tries to convert `xsd:duration` string into valid [Value] representing a date and time.
  /// FEEL durations are conformant with `xsd:duration` but spit into two ranges.
  pub fn try_from_xsd_duration(text: &str) -> Result<Self> {
    if let Ok(ym_duration) = FeelYearsAndMonthsDuration::try_from(text) {
      return Ok(Value::YearsAndMonthsDuration(ym_duration));
    }
    if let Ok(dt_duration) = FeelDaysAndTimeDuration::try_from(text) {
      return Ok(Value::DaysAndTimeDuration(dt_duration));
    }
    Err(err_invalid_xsd_duration(text))
  }
}

/// Collection of values.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Values(Vec<Value>);

impl Values {
  ///
  pub fn new(values: Vec<Value>) -> Self {
    Self(values)
  }
  ///
  pub fn add(&mut self, value: Value) {
    self.0.push(value);
  }
  ///
  pub fn insert(&mut self, index: usize, value: Value) {
    self.0.insert(index, value);
  }
  ///
  pub fn remove(&mut self, index: usize) {
    self.0.remove(index);
  }
  ///
  pub fn len(&self) -> usize {
    self.0.len()
  }
  ///
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
  ///
  pub fn reverse(&mut self) {
    self.0.reverse();
  }
  ///
  pub fn as_vec(&self) -> &Vec<Value> {
    &self.0
  }
}

impl std::fmt::Display for Values {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[{}]", self.0.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(", "))
  }
}

impl ToFeelString for Values {
  /// Converts [Values] into `FEEL` string.
  fn to_feel_string(&self) -> String {
    format!("[{}]", self.0.iter().map(|value| value.to_feel_string()).collect::<Vec<String>>().join(", "))
  }
}

impl Jsonify for Values {
  ///
  fn jsonify(&self) -> String {
    format!("[{}]", self.0.iter().map(|value| value.jsonify()).collect::<Vec<String>>().join(", "))
  }
}

/// Definitions of value errors.
mod errors {
  use dmntk_common::DmntkError;

  /// Value errors.
  struct ValueError(String);

  impl From<ValueError> for DmntkError {
    /// Converts [ValueError] into [DmntkError].
    fn from(e: ValueError) -> Self {
      DmntkError::new("ValueError", &e.0)
    }
  }

  /// Error used when parsed text is not acceptable `xsd:integer` representation.
  pub fn err_invalid_xsd_integer(text: &str) -> DmntkError {
    ValueError(format!("'{text}' is not valid xsd:integer representation")).into()
  }
  /// Error used when parsed text is not acceptable `xsd:decimal` representation.
  pub fn err_invalid_xsd_decimal(text: &str) -> DmntkError {
    ValueError(format!("'{text}' is not valid xsd:decimal representation")).into()
  }
  /// Error used when parsed text is not acceptable `xsd:double` representation.
  pub fn err_invalid_xsd_double(text: &str) -> DmntkError {
    ValueError(format!("'{text}' is not valid xsd:double representation")).into()
  }
  /// Error used when parsed text is not acceptable `xsd:boolean` representation.
  pub fn err_invalid_xsd_boolean(text: &str) -> DmntkError {
    ValueError(format!("'{text}' is not valid xsd:boolean representation")).into()
  }
  /// Error used when parsed text is not acceptable `xsd:date` representation.
  pub fn err_invalid_xsd_date(text: &str) -> DmntkError {
    ValueError(format!("'{text}' is not valid xsd:date representation")).into()
  }
  /// Error used when parsed text is not acceptable `xsd:time` representation.
  pub fn err_invalid_xsd_time(text: &str) -> DmntkError {
    ValueError(format!("'{text}' is not valid xsd:time representation")).into()
  }
  /// Error used when parsed text is not acceptable `xsd:dateTime` representation.
  pub fn err_invalid_xsd_date_time(text: &str) -> DmntkError {
    ValueError(format!("'{text}' is not valid xsd:dateTime representation")).into()
  }
  /// Error used when parsed text is not acceptable `xsd:duration` representation.
  pub fn err_invalid_xsd_duration(text: &str) -> DmntkError {
    ValueError(format!("'{text}' is not valid xsd:duration representation")).into()
  }
}

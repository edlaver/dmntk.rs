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

//! # Error definitions

use crate::values::Value;
use dmntk_common::DmntkError;

/// Definition of errors raised in `types` module.
struct TypesError(String);

impl From<TypesError> for DmntkError {
  /// Converts `TypesError` into [DmntkError].
  fn from(e: TypesError) -> Self {
    DmntkError::new("TypesError", &e.0)
  }
}

/// Creates an invalid `FEEL` type name error.
pub fn err_invalid_feel_type_name(s: &str) -> DmntkError {
  TypesError(format!("invalid FEEL type name: {s}")).into()
}

/// Creates an error indicating value non conformant with type.
pub fn err_invalid_value_for_retrieving_using_feel_type(s1: &str, s2: &str) -> DmntkError {
  TypesError(format!("invalid value for retrieving with type check, type = '{s1}', value = '{s2}'")).into()
}

/// Value errors.
struct ValueError(String);

impl From<ValueError> for DmntkError {
  /// Converts `ValueError` into [DmntkError].
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

/// Context errors.
struct ContextError(String);

impl From<ContextError> for DmntkError {
  fn from(e: ContextError) -> Self {
    DmntkError::new("ContextError", &e.0)
  }
}

/// Creates an instance of `value is not a context` error.
pub fn err_value_is_not_a_context(value: &Value) -> DmntkError {
  ContextError(format!("'{value}' is not a value containing context")).into()
}

/// Built-in functions errors.
struct BifError(String);

impl From<BifError> for DmntkError {
  /// Converts `BifError` into [DmntkError].
  fn from(e: BifError) -> Self {
    DmntkError::new("BifError", &e.0)
  }
}

/// Creates an instance of `UnknownFunctionName` error.
pub fn err_unknown_function_name(name: &str) -> DmntkError {
  BifError(format!("unknown built-in function name: {name}")).into()
}

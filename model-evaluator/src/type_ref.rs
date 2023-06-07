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

use dmntk_feel::FeelType;
use std::ops::Deref;

/// Type reference enumeration for simple FEEL types.
#[derive(Debug, PartialEq)]
pub enum TypeRef {
  String(FeelType),
  Number(FeelType),
  Boolean(FeelType),
  Date(FeelType),
  Time(FeelType),
  DateTime(FeelType),
  DayTimeDuration(FeelType),
  YearMonthDuration(FeelType),
}

impl Deref for TypeRef {
  type Target = FeelType;
  /// Unwraps inner [FeelType].
  fn deref(&self) -> &Self::Target {
    match self {
      TypeRef::String(feel_type) => feel_type,
      TypeRef::Number(feel_type) => feel_type,
      TypeRef::Boolean(feel_type) => feel_type,
      TypeRef::Date(feel_type) => feel_type,
      TypeRef::Time(feel_type) => feel_type,
      TypeRef::DateTime(feel_type) => feel_type,
      TypeRef::DayTimeDuration(feel_type) => feel_type,
      TypeRef::YearMonthDuration(feel_type) => feel_type,
    }
  }
}

/// Converts simple FEEL type reference into [FeelType] wrapped with [TypeRef].
pub fn type_ref_to_feel_type(type_ref: &str) -> Option<TypeRef> {
  match type_ref.trim() {
    "string" => Some(TypeRef::String(FeelType::String)),
    "number" => Some(TypeRef::Number(FeelType::Number)),
    "boolean" => Some(TypeRef::Boolean(FeelType::Boolean)),
    "date" => Some(TypeRef::Date(FeelType::Date)),
    "time" => Some(TypeRef::Time(FeelType::Time)),
    "dateTime" => Some(TypeRef::DateTime(FeelType::DateTime)),
    "dayTimeDuration" => Some(TypeRef::DayTimeDuration(FeelType::DaysAndTimeDuration)),
    "yearMonthDuration" => Some(TypeRef::YearMonthDuration(FeelType::YearsAndMonthsDuration)),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dmntk_feel::FeelType;

  #[test]
  fn test_type_ref_to_feel_type() {
    assert_eq!(Some(TypeRef::String(FeelType::String)), type_ref_to_feel_type("string"));
    assert_eq!(Some(TypeRef::Number(FeelType::Number)), type_ref_to_feel_type("number"));
    assert_eq!(Some(TypeRef::Boolean(FeelType::Boolean)), type_ref_to_feel_type("boolean"));
    assert_eq!(Some(TypeRef::Date(FeelType::Date)), type_ref_to_feel_type("date"));
    assert_eq!(Some(TypeRef::Time(FeelType::Time)), type_ref_to_feel_type("time"));
    assert_eq!(Some(TypeRef::DateTime(FeelType::DateTime)), type_ref_to_feel_type("dateTime"));
    assert_eq!(Some(TypeRef::DayTimeDuration(FeelType::DaysAndTimeDuration)), type_ref_to_feel_type("dayTimeDuration"));
    assert_eq!(
      Some(TypeRef::YearMonthDuration(FeelType::YearsAndMonthsDuration)),
      type_ref_to_feel_type("yearMonthDuration")
    );
    assert_eq!(None, type_ref_to_feel_type("date and time"));
    assert_eq!(None, type_ref_to_feel_type("days and time duration"));
    assert_eq!(None, type_ref_to_feel_type("years and months duration"));
    assert_eq!(None, type_ref_to_feel_type("text"));
  }
}

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

mod boxed_expressions;
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

pub use business_knowledge_model::BusinessKnowledgeModelEvaluator;
pub use decision::DecisionEvaluator;
pub use decision_service::DecisionServiceEvaluator;
pub use input_data::InputDataEvaluator;
pub use input_data_context::InputDataContextEvaluator;
pub use item_definition::ItemDefinitionEvaluator;
pub use item_definition_context::ItemDefinitionContextEvaluator;
pub use item_definition_type::ItemDefinitionTypeEvaluator;

use dmntk_feel::FeelType;

///
// TODO Create newtype named TypeRef, containing single string and implement conversion function.
pub fn type_ref_to_feel_type(type_ref: &str) -> Option<FeelType> {
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
    assert_eq!(None, type_ref_to_feel_type("date and time"));
    assert_eq!(None, type_ref_to_feel_type("days and time duration"));
    assert_eq!(None, type_ref_to_feel_type("years and months duration"));
    assert_eq!(None, type_ref_to_feel_type("text"));
  }
}

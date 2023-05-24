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

use dmntk_common::{DmntkError, ToErrorMessage};

/// Errors related to the DMN model.
#[derive(ToErrorMessage)]
struct ModelError(String);

pub fn err_invalid_decision_table_orientation(orientation: &str) -> DmntkError {
  ModelError(format!("invalid decision table orientation: {orientation}")).into()
}

pub fn err_invalid_decision_table_hit_policy(hit_policy: &str) -> DmntkError {
  ModelError(format!("invalid decision table hit policy: {hit_policy}")).into()
}

/// Errors related with parsing the decision model.
#[derive(ToErrorMessage)]
struct ModelParserError(String);

/// Raised when parsed text is not a valid function kind, accepted values are:
/// `FEEL`, `Java` or `PMML`.
pub fn err_invalid_function_kind(s: &str) -> DmntkError {
  ModelParserError(format!("'{s}' is not a valid function kind, accepted values are: 'FEEL', 'Java', 'PMML'")).into()
}

/// Raised when parsed text is not a valid hit policy, accepted values are:
/// `UNIQUE`, `FIRST`, `PRIORITY`, `ANY`, `COLLECT`, `RULE ORDER` or `OUTPUT ORDER`.
pub fn err_invalid_hit_policy(s: &str) -> DmntkError {
  ModelParserError(format!(
    "'{s}' is not a valid hit policy, allowed values are: 'UNIQUE', 'FIRST', 'PRIORITY', 'ANY', 'COLLECT', 'RULE ORDER', 'OUTPUT ORDER'"
  ))
  .into()
}

/// Raised when parsed text is not a valid aggregation for hit policy, accepted values are:
/// `COUNT`, `SUM`, `MIN`, or `MAX`.
pub fn err_invalid_aggregation(s: &str) -> DmntkError {
  ModelParserError(format!("'{s}' is not a valid aggregation, allowed values are: 'COUNT', 'SUM', 'MIN', 'MAX'")).into()
}

/// Invalid value for a color.
pub fn err_invalid_color_value(s: &str) -> DmntkError {
  ModelParserError(format!("conversion to valid color value failed with reason: {s}")).into()
}

/// Invalid value for a double.
pub fn err_invalid_double_value(reason: &str) -> DmntkError {
  ModelParserError(format!("conversion to valid double value failed with reason: {reason}")).into()
}

/// Raised when required child node is missing.
pub fn err_required_child_node_is_missing(s1: &str, s2: &str) -> DmntkError {
  ModelParserError(format!("required child node '{s2}' in parent node '{s1}' is missing")).into()
}

/// Raised when required `inputExpression` node is missing.
pub fn err_required_input_expression_is_missing() -> DmntkError {
  ModelParserError("required input expression in decision table's input clause is missing".to_string()).into()
}

/// Raised when required expression instance is missing.
pub fn err_required_expression_instance_is_missing() -> DmntkError {
  ModelParserError("required expression instance in context entry is missing".to_string()).into()
}

/// Raised when the number of elements in a row differs from the number of columns in relation.
pub fn err_number_of_elements_in_row_differs_from_number_of_columns() -> DmntkError {
  ModelParserError("number of elements in a row differs from the number of columns defined in a relation".to_string()).into()
}

pub fn err_xml_parsing_model_failed(s: &str) -> DmntkError {
  ModelParserError(format!("parsing model from XML failed with reason: {s}")).into()
}

pub fn err_xml_unexpected_node(s1: &str, s2: &str) -> DmntkError {
  ModelParserError(format!("unexpected XML node, expected: {s1}, actual: {s2}")).into()
}

pub fn err_xml_expected_mandatory_attribute(s1: &str, s2: &str) -> DmntkError {
  ModelParserError(format!("expected value for mandatory attribute '{s2}' in node {s1}")).into()
}

pub fn err_xml_expected_mandatory_child_node(s1: &str, s2: &str) -> DmntkError {
  ModelParserError(format!("expected mandatory child node '{s2}' in parent node {s1}")).into()
}

pub fn err_xml_expected_mandatory_text_content(s: &str) -> DmntkError {
  ModelParserError(format!("expected mandatory text content in node '{s}'")).into()
}

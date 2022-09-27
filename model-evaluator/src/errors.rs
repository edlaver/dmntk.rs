/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
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

use dmntk_common::DmntkError;
use dmntk_feel::FeelType;

/// Errors related to model evaluation.
pub struct ModelEvaluatorError(String);

impl From<ModelEvaluatorError> for DmntkError {
  fn from(e: ModelEvaluatorError) -> Self {
    DmntkError::new("ModelEvaluatorError", &e.0)
  }
}

pub fn err_business_knowledge_model_with_reference_not_found(s: &str) -> DmntkError {
  ModelEvaluatorError(format!("business knowledge model with reference '{}' was not found", s)).into()
}

pub fn err_input_data_without_type_reference(s: &str) -> DmntkError {
  ModelEvaluatorError(format!("input data with identifier '{}' has no type reference definition", s)).into()
}

pub fn err_empty_feel_name() -> DmntkError {
  ModelEvaluatorError("empty FEEL name".to_string()).into()
}

pub fn err_empty_identifier() -> DmntkError {
  ModelEvaluatorError("empty identifier".to_string()).into()
}

pub fn err_empty_literal_expression() -> DmntkError {
  ModelEvaluatorError("empty literal expression".to_string()).into()
}

pub fn err_empty_decision_logic() -> DmntkError {
  ModelEvaluatorError("empty decision logic in decision".to_string()).into()
}

pub fn err_empty_encapsulated_logic() -> DmntkError {
  ModelEvaluatorError("empty encapsulated logic in business knowledge model".to_string()).into()
}

pub fn err_invalid_item_definition_type(s: &str) -> DmntkError {
  ModelEvaluatorError(format!("invalid item definition type for '{}'", s)).into()
}

pub fn err_unsupported_feel_type(feel_type: FeelType) -> DmntkError {
  ModelEvaluatorError(format!("unsupported FEEL type: {}", feel_type)).into()
}

pub fn err_empty_feel_type() -> DmntkError {
  ModelEvaluatorError("empty FEEL type".to_string()).into()
}

pub fn err_empty_reference() -> DmntkError {
  ModelEvaluatorError("empty reference".to_string()).into()
}

pub fn err_empty_function_body() -> DmntkError {
  ModelEvaluatorError("empty function definition body".to_string()).into()
}

pub fn err_empty_value_expression() -> DmntkError {
  ModelEvaluatorError("empty value expression".to_string()).into()
}

pub fn err_read_lock_failed(reason: impl ToString) -> DmntkError {
  ModelEvaluatorError(format!("read lock failed with reason '{}'", reason.to_string())).into()
}

pub fn err_write_lock_failed(reason: impl ToString) -> DmntkError {
  ModelEvaluatorError(format!("write lock failed with reason '{}'", reason.to_string())).into()
}

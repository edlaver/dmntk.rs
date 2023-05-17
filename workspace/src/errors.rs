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

//! Errors reported by workspace.

use dmntk_common::DmntkError;

/// Errors reported by workspace.
struct WorkspaceError(String);

impl From<WorkspaceError> for DmntkError {
  /// Converts `WorkspaceError` into [DmntkError].
  fn from(e: WorkspaceError) -> Self {
    DmntkError::new("WorkspaceError", &e.0)
  }
}

pub fn err_model_evaluator_is_not_deployed(s: &str) -> DmntkError {
  WorkspaceError(format!("model evaluator for definitions '{s}' is not deployed")).into()
}

pub fn err_invalid_namespace(s: &str) -> DmntkError {
  WorkspaceError(format!("invalid namespace '{s}'")).into()
}

pub fn err_definitions_with_name_already_exists(namespace: &str, name: &str) -> DmntkError {
  WorkspaceError(format!("definitions with name '{name}' already exist in namespace '{namespace}'")).into()
}

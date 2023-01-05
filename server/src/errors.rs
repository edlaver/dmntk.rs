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

//! Errors reported by server.

use dmntk_common::DmntkError;

/// Server errors.
struct ServerError(String);

impl From<ServerError> for DmntkError {
  fn from(e: ServerError) -> Self {
    DmntkError::new("ServerError", &e.0)
  }
}

pub fn err_endpoint_not_found() -> DmntkError {
  ServerError("endpoint not found".to_string()).into()
}

pub fn err_missing_parameter(name: &str) -> DmntkError {
  ServerError(format!("missing parameter '{name}'")).into()
}

pub fn err_invalid_base64_encoding() -> DmntkError {
  ServerError("invalid Base64 encoding".to_string()).into()
}

pub fn err_invalid_utf8_content() -> DmntkError {
  ServerError("invalid UTF-8 content".to_string()).into()
}

pub fn err_workspace_read_lock_failed() -> DmntkError {
  ServerError("workspace read lock failed".to_string()).into()
}

pub fn err_workspace_write_lock_failed() -> DmntkError {
  ServerError("workspace write lock failed".to_string()).into()
}

pub fn err_internal_error(message: &str) -> DmntkError {
  ServerError(message.to_string()).into()
}

pub fn err_invalid_parameter(description: &str) -> DmntkError {
  ServerError(format!("invalid parameter '{description}'")).into()
}

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

use super::super::*;

static_model_evaluator_examples!(DMN_3_0085);

#[test]
fn _0001() {
  assert_decision_service(&MODEL_EVALUATOR, "decisionService_001", r#"{}"#, r#""foo""#);
}

#[test]
fn _0002() {
  assert_decision_service(&MODEL_EVALUATOR, "decisionService_002", r#"{decision_002_input: "baz"}"#, r#""foo baz""#);
}

#[test]
fn _0002_a() {
  assert_decision_service(
    &MODEL_EVALUATOR,
    "decisionService_002",
    r#"{}"#,
    r#"null(expected string as a second argument in addition)"#,
  );
}

#[test]
fn _0002_b() {
  assert_decision_service(
    &MODEL_EVALUATOR,
    "decisionService_002",
    r#"{decision_002_input: null}"#,
    r#"null(expected string as a second argument in addition)"#,
  );
}

#[test]
fn _0002_c() {
  assert_decision_service(
    &MODEL_EVALUATOR,
    "decisionService_002",
    r#"{decision_002_input: 1234}"#,
    r#"null(expected string as a second argument in addition)"#,
  );
}

#[test]
fn _0003() {
  assert_decision_service(
    &MODEL_EVALUATOR,
    "decisionService_003",
    r#"{decision_003_input_1: "B", decision_003_input_2: "C", inputData_003: "D"}"#,
    r#""A B C D""#,
  );
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "decision_004_1", &ctx, r#""foo""#);
}

#[test]
fn _0005() {
  assert_decision_service(&MODEL_EVALUATOR, "decisionService_005", r#"{}"#, r#""foo""#);
}

#[test]
fn _0006() {
  assert_decision_service(&MODEL_EVALUATOR, "decision_005_1", r#"{}"#, r#"null(invalid number of arguments)"#);
}

#[test]
fn _0007() {
  assert_decision_service(&MODEL_EVALUATOR, "decision_005_2", r#"{}"#, r#""foo""#);
}

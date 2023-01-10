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

use super::super::*;
use crate::model_evaluator::ModelEvaluator;
use std::sync::Arc;

lazy_static! {
  static ref CTX: FeelContext = context(r#"{}"#);
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0082);
}

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, "decision_001", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, "decision_003", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, "decision_004", &CTX, r#"{age: 10, name: "foo", surname: "bar"}"#);
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, "decision_005", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, "decision_006", &CTX, r#"["foo"]"#);
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, "decision_006_a", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0007() {
  assert_decision(&MODEL_EVALUATOR, "decision_007", &CTX, r#""foo""#);
}

#[test]
fn _0008() {
  assert_decision(&MODEL_EVALUATOR, "decision_007_a", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0009() {
  assert_decision(&MODEL_EVALUATOR, "decision_008", &CTX, r#"null"#);
}

#[test]
fn _0010() {
  assert_decision(&MODEL_EVALUATOR, "decision_bkm_001", &CTX, r#"true"#);
}

#[test]
fn _0011() {
  assert_decision(&MODEL_EVALUATOR, "decision_bkm_002", &CTX, r#"false"#);
}

#[test]
fn _0012() {
  assert_decision(&MODEL_EVALUATOR, "decision_bkm_003", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0013() {
  assert_decision(&MODEL_EVALUATOR, "decision_bkm_004_a", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0014() {
  assert_decision(&MODEL_EVALUATOR, "decision_bkm_004_b", &CTX, r#"null"#);
}

#[test]
fn _0015() {
  assert_decision(&MODEL_EVALUATOR, "decision_bkm_005", &CTX, r#"10"#);
}

#[test]
fn _0016() {
  assert_decision(&MODEL_EVALUATOR, "decision_bkm_005_a", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0017() {
  assert_decision(&MODEL_EVALUATOR, "invoke_001", &CTX, r#"false"#);
}

#[test]
fn _0018() {
  assert_decision(&MODEL_EVALUATOR, "invoke_002", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0019() {
  assert_decision(&MODEL_EVALUATOR, "invoke_004", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0020() {
  assert_decision(&MODEL_EVALUATOR, "invoke_005", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0021() {
  assert_decision(&MODEL_EVALUATOR, "invoke_006", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0022() {
  assert_decision(&MODEL_EVALUATOR, "fd_001", &CTX, r#"10"#);
}

#[test]
fn _0023() {
  assert_decision(&MODEL_EVALUATOR, "fd_002", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0024() {
  assert_decision(&MODEL_EVALUATOR, "literal_001", &CTX, r#"10"#);
}

#[test]
fn _0025() {
  assert_decision(&MODEL_EVALUATOR, "literal_002", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0026() {
  assert_decision(&MODEL_EVALUATOR, "literal_004", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0027() {
  assert_decision(&MODEL_EVALUATOR, "literal_005", &CTX, r#"10"#);
}

#[test]
fn _0028() {
  assert_decision(&MODEL_EVALUATOR, "literal_006", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0029() {
  assert_decision(&MODEL_EVALUATOR, "ds_invoke_002_with_number", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0030() {
  assert_decision(&MODEL_EVALUATOR, "ds_invoke_002_with_singleton_list", &CTX, r#""foo""#);
}

#[test]
fn _0031() {
  assert_decision(&MODEL_EVALUATOR, "decisionService_001", &CTX, r#"null(after coercion)"#);
}

#[test]
fn _0032() {
  let ctx = context(r#"{decisionService_002_input_1: 10}"#);
  assert_decision(&MODEL_EVALUATOR, "decisionService_002", &ctx, r#"null(after coercion)"#);
}

#[test]
fn _0033() {
  let ctx = context(r#"{decisionService_002_input_1: ["foo"]}"#);
  assert_decision(&MODEL_EVALUATOR, "decision_ds_002", &ctx, r#""foo""#);
}

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
use crate::ModelEvaluator;
use std::sync::Arc;

lazy_static! {
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0076);
  static ref CTX: FeelContext = context(r#"{}"#);
}

#[test]
fn _0001() {
  assert_decision(&MODEL_EVALUATOR, "boxed_001", &CTX, r#"456"#);
}

#[test]
fn _0002() {
  assert_decision(&MODEL_EVALUATOR, "incorrect_001", &CTX, r#"null"#);
}

#[test]
fn _0003() {
  assert_decision(&MODEL_EVALUATOR, "incorrect_002", &CTX, r#"null"#);
}

#[test]
fn _0004() {
  assert_decision(&MODEL_EVALUATOR, "incorrect_003", &CTX, r#"null"#);
}

#[test]
fn _0005() {
  assert_decision(&MODEL_EVALUATOR, "literal_001", &CTX, r#"-0.88796890"#);
}

#[test]
fn _0006() {
  assert_decision(&MODEL_EVALUATOR, "literal_002", &CTX, r#"456.78"#);
}

#[test]
fn _0007() {
  assert_decision(&MODEL_EVALUATOR, "literal_003", &CTX, r#"456"#);
}

#[test]
fn _0008() {
  assert_decision(&MODEL_EVALUATOR, "literal_004", &CTX, r#"456"#);
}

#[test]
fn _0009() {
  assert_decision(&MODEL_EVALUATOR, "literal_005", &CTX, r#"123"#);
}

#[test]
fn _0010() {
  assert_decision(&MODEL_EVALUATOR, "literal_006", &CTX, r#"3"#);
}

#[test]
fn _0011() {
  assert_decision(&MODEL_EVALUATOR, "literal_007", &CTX, r#""a""#);
}

#[test]
fn _0012() {
  assert_decision(&MODEL_EVALUATOR, "literal_007_a", &CTX, r#"null"#);
}

#[test]
fn _0013() {
  assert_decision(&MODEL_EVALUATOR, "literal_008", &CTX, r#"456"#);
}

#[test]
fn _0014() {
  assert_decision(&MODEL_EVALUATOR, "literal_009", &CTX, r#"456.78"#);
}

#[test]
fn _0015() {
  assert_decision(&MODEL_EVALUATOR, "literal_010", &CTX, r#"123"#);
}

#[test]
fn _0016() {
  assert_decision(&MODEL_EVALUATOR, "literal_011", &CTX, r#"1234.56"#);
}

#[test]
fn _0017() {
  assert_decision(&MODEL_EVALUATOR, "literal_012", &CTX, r#"1234.56"#);
}

#[test]
fn _0018() {
  assert_decision(&MODEL_EVALUATOR, "varargs_001", &CTX, r#""foo bar""#);
}

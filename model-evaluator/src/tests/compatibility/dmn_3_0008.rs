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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0008);
}

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "listGen1", &ctx, r#"["a", "b", "c"]"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{a: "a", b: "b", c: "c"}"#);
  assert_decision(&MODEL_EVALUATOR, "listGen2", &ctx, r#"["a", "b", "c"]"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{b: "b", c: "c"}"#);
  assert_decision(&MODEL_EVALUATOR, "listGen3", &ctx, r#"["a", "b", "c"]"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{c: "c"}"#);
  assert_decision(&MODEL_EVALUATOR, "listGen4", &ctx, r#"["a", "b", "c"]"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{a: "a", b: "b", c: "c"}"#);
  assert_decision(&MODEL_EVALUATOR, "listGen5", &ctx, r#"["a", "b", "c"]"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "listGen6", &ctx, r#"["w", "x", "y", "z"]"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{wx: ["w", "x"]}"#);
  assert_decision(&MODEL_EVALUATOR, "listGen7", &ctx, r#"["w", "x", "y", "z"]"#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{a: "a", b: "b"}"#);
  assert_decision(&MODEL_EVALUATOR, "listGen8", &ctx, r#"["a", "b", "w", "x", "y", "z"]"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{a: "a", b: "b", wx: ["w", "x"]}"#);
  assert_decision(&MODEL_EVALUATOR, "listGen9", &ctx, r#"["a", "b", "w", "x", "y", "z"]"#);
}

#[test]
fn _0010() {
  let ctx = context(r#"{c: "c", wx: ["w", "x"]}"#);
  assert_decision(&MODEL_EVALUATOR, "listGen10", &ctx, r#"["a", "b", "c", "w", "x", "y", "z"]"#);
}

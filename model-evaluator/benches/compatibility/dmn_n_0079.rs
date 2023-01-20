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

use super::build_model_evaluator;
use crate::compatibility::{assert_decision, context};
use dmntk_model_evaluator::ModelEvaluator;
use std::sync::Arc;
use test::Bencher;

lazy_static! {
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_N_0079);
}

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(expected 1 parameters, actual number of parameters is 0)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(expected 1 parameters, actual number of parameters is 2)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "decision_004";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(expected 1 parameters, actual number of parameters is 2)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "null_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "string_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "number_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""123.45""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "boolean_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""true""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "boolean_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""false""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""2018-12-10""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""2018-12-10T00:00:00""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""2018-12-10T10:30:00.0001""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0013(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""2018-12-10T10:30:00.0001+05:00:01""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0014(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_004";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""2018-12-10T10:30:00@Etc/UTC""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0015(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""10:30:00.0001""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0016(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""10:30:00.0001+05:00:01""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0017(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""10:30:00@Etc/UTC""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0018(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""P1D""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0019(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""-P1D""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0020(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""P0D""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0021(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_004";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""P1DT2H3M4.1234S""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0022(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_005";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""P2DT1H""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0023(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""P1Y""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0024(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""-P1Y""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0025(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""P0Y""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0026(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_004";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""P1Y2M""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0027(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_005";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""P2Y1M""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0028(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""[1, 2, 3, "foo"]""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0029(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""[1, 2, 3, [4, 5, "foo"]]""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0030(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "list_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""["\"foo\""]""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0031(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""{a: "foo"}""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0032(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""{a: "foo", b: {bar: "baz"}}""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0033(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""{"{": "foo"}""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0034(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_004";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""{":": "foo"}""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0035(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_005";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""{",": "foo"}""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0036(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_006";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""{"}": "foo"}""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0037(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_007";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""{"\"": "foo"}""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

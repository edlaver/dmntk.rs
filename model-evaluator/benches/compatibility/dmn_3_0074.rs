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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0074);
}

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "context_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""foo""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"2018"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"12"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"10"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "date_004";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"2018"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"12"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"10"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_004";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_005";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"10"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_005_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"0"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_006";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"30"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0013(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_006_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"0"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0014(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_007";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0015(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_007_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"0"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0016(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_008";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"PT5H"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0017(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_008_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(aaaa)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0018(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_009";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#""Etc/UTC""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0019(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dateTime_009_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(bbb)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0020(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"10"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0021(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"30"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0022(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0023(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_004";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"PT5H"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0024(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_004_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(ccc)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0025(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "time_005_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(ddd)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0026(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0027(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_001_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"0"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0028(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0029(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_002_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"0"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0030(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(no such property in years and months duration)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0031(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_004";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(no such property in years and months duration)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0032(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_005";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(no such property in years and months duration)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0033(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "ym_duration_006";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(no such property in years and months duration)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0034(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_001";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(no such property in days and time duration)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0035(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(no such property in days and time duration)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0036(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_003";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0037(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_003_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"0"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0038(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_004";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0039(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_004_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"0"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0040(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_005";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0041(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_005_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"0"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0042(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_006";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"2"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0043(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "dt_duration_006_a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"0"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

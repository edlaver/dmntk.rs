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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_1121);
}

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_ErrorCase_001_b24a0c91f2";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(expected 2 parameters, actual number of parameters is 1)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_ErrorCase_002_4e7651ae0e";
  assert_decision(
    &MODEL_EVALUATOR,
    invocable_name,
    &ctx,
    r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_ErrorCase_003_0886738d31";
  assert_decision(
    &MODEL_EVALUATOR,
    invocable_name,
    &ctx,
    r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_ErrorCase_004_1bdfef922b";
  assert_decision(
    &MODEL_EVALUATOR,
    invocable_name,
    &ctx,
    r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_ErrorCase_005_d0a077da4e";
  assert_decision(
    &MODEL_EVALUATOR,
    invocable_name,
    &ctx,
    r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_ErrorCase_006_f20de28d3f";
  assert_decision(
    &MODEL_EVALUATOR,
    invocable_name,
    &ctx,
    r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_ErrorCase_007_0921c3d61a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(expected 2 parameters, actual number of parameters is 0)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_008_015d35b442";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P1Y8M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_009_635028a5d8";
  let invocable_name = invocable_name;
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"-P1Y8M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_010_caaa2e5002";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P1Y"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_011_3fac022eb0";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"-P1Y"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_012_331ef38ce0";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P0Y"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0013(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_013_2f3cc46d9d";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P0Y"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0014(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_014_1fadbba7cd";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P1Y2M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0015(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_015_0e496f94fc";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P7Y6M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0016(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_016_b38662aa93";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P4Y9M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0017(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_017_86744b9a54";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"-P11M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0018(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_018_8a9ed1d66d";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"-P4033Y2M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0019(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_019_90c2084588";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"-P4035Y11M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0020(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_020_8ead9a0377";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P2Y"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0021(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_021_8a7d311ae9";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P11M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0022(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_022_87e369773b";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P5Y7M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0023(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_023_6385c7a83e";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P1Y"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0024(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_024_e96d1bd93a";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P4Y"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0025(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_025_161f6fca54";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P2Y9M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0026(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_026_fcc906b375";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P3Y"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0027(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_ErrorCase_027_3374dd86c6";
  assert_decision(
    &MODEL_EVALUATOR,
    invocable_name,
    &ctx,
    r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0028(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_ErrorCase_028_77600e7b35";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(expected 2 parameters, actual number of parameters is 1)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0029(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_ErrorCase_029_15a0d0d9c1";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"null(expected 2 parameters, actual number of parameters is 1)"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0030(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_ErrorCase_030_ec16878596";
  assert_decision(
    &MODEL_EVALUATOR,
    invocable_name,
    &ctx,
    r#"null([core::years and months duration] invalid argument type, expected date, date and time, actual type is list<Null>)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0031(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_031_4fd9c09d89";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P4Y3M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0032(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_032_2a09ac80d0";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P2Y4M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0033(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_033_7333eca866";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P1Y"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0034(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_034_c2cc06724c";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P2Y"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0035(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_035_dc05f9555d";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"P1Y8M"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0036(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-years-and-months-duration-function_036_f8c8b02ba3";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"-P1Y"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

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

use super::*;

from_examples!(DMN_3_1115);

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_001_e9ae035ab9";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid argument type, expected string, date or date and time, actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_002_9b9e6085ce";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null(expected 1,3 parameters, actual number of parameters is 2)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_003_e4b7918d8f";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid argument type, expected number (year), actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_004_f24ed41117";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid argument type, expected number (month), actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_005_3540a22062";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid argument type, expected number (day), actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_006_616e24dbb7";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid argument type, expected number (year), actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_007_cda82a5d01";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid argument type, expected number (year), actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_008_492649d3d0";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid argument type, expected number (month), actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_009_9e00bbdad3";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid argument type, expected number (year), actual type is Null)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_010_6d4d58d23a";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null(expected 1,3 parameters, actual number of parameters is 0)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_011_5f0b42b1f8";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-12-31"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_012_d9e4b97438";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-01-01"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0013(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_013_d7e901ee86";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"-2017-12-31"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0014(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_014_fad7e00633";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"-2017-01-01"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0015(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_015_1dd66594cf";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#""999999999-12-31""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0016(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_016_31f3fef4a0";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#""-999999999-12-31""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0017(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_017_887dfef005";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-08-14"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0018(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_018_fc0ef0c8cb";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-08-14"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0019(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_019_b2b82796ce";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-08-14"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0020(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_020_7d56b7bf63";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-09-03"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0021(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_021_95fb3d9984";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-09-06"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0022(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_022_4063db2d59";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2012-12-25"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0023(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_023_4a1f604006";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-08-03"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0024(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_024_3cb98a2bb8";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-10-11"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0025(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_025_cf0ad1313c";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-12-31"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0026(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_026_cedd7e5e5f";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-01-01"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0027(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_027_987c5be372";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"-2017-12-31"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0028(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_028_35ca79a6cd";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"-2017-01-01"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0029(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_029_88f5c7c90f";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#""999999999-12-31""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0030(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_030_9184a7bfc3";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#""-999999999-12-31""#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0031(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_031_4f5ec70669";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid date string '2012-12-25T')"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0032(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_032_fc66cc2fec";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"null([core::date] invalid date string '')"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0033(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_033_c3a5600c62";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid date string '2012/12/25')"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0034(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_034_7d2e18a10c";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid date string '0000-12-25T')"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0035(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_035_e6c1bb43fd";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid date string '9999999999-12-25')"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0036(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_036_b826a6b5f9";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid date string '2017-13-10')"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0037(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_037_cfd70896b6";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid date string '2017-12-32')"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0038(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_038_c26782f559";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid date string '998-12-31')"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0039(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_039_67a6eafa3f";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid date string '01211-12-31')"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0040(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_040_dd2a2ed4a2";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid date string '2012T-12-2511:00:00Z')"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0041(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_041_9e7e388146";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid date string '+2012-12-02')"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0042(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_042_8f5dd97588";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date_3] invalid date y=2017 m=13 d=31)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0043(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_043_8f82301fac";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date_3] invalid date y=2017 m=12 d=32)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0044(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_044_74893220b4";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date_3] invalid date y=2017 m=-8 d=2)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0045(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_045_969723fed5";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date_3] invalid date y=2017 m=8 d=-2)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0046(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_046_36bf30268a";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date_3] invalid date y=-1000999999 m=12 d=1)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0047(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_047_ba717eb672";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date_3] invalid date y=1000999999 m=12 d=32)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0048(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_048_25595a6420";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid argument type, expected string, date or date and time, actual type is number)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0049(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_ErrorCase_049_a1644ce710";
  assert_decision(
    &MODEL_EVALUATOR,
    &MODEL_NAMESPACE,
    invocable_name,
    &ctx,
    r#"null([core::date] invalid argument type, expected string, date or date and time, actual type is list<Null>)"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0050(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_050_8f1e299951";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2012-12-25"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0051(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_051_ad98079864";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-08-30"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

#[bench]
fn _0052(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "feel-date-function_052_63457d78b7";
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, invocable_name, &ctx, r#"2017-08-30"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(&MODEL_NAMESPACE, invocable_name, &ctx));
}

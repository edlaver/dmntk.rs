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

use super::build_model_evaluator;
use crate::compliance::{assert_decision, context};
use dmntk_model_evaluator::ModelEvaluator;
use std::sync::Arc;
use test::Bencher;

lazy_static! {
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0072);
}

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_001", &ctx, r#"true"#);
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_001_a", &ctx, r#"false"#);
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_002", &ctx, r#"true"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_002_a", &ctx, r#"false"#);
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_003", &ctx, r#"true"#);
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_003_1", &ctx, r#"true"#);
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_003_a", &ctx, r#"false"#);
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_004", &ctx, r#"true"#);
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_004_a", &ctx, r#"false"#);
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_005", &ctx, r#"true"#);
}

#[bench]
fn _0011(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_005_1", &ctx, r#"true"#);
}

#[bench]
fn _0012(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_005_a", &ctx, r#"false"#);
}

#[bench]
fn _0013(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_006", &ctx, r#"true"#);
}

#[bench]
fn _0014(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_006_a", &ctx, r#"false"#);
}

#[bench]
fn _0015(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_007_a", &ctx, r#"false"#);
}

#[bench]
fn _0016(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_007_b", &ctx, r#"false"#);
}

#[bench]
fn _0017(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_007_c", &ctx, r#"true"#);
}

#[bench]
fn _0018(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_007_d", &ctx, r#"false"#);
}

#[bench]
fn _0019(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_007_e", &ctx, r#"false"#);
}

#[bench]
fn _0020(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_008_a", &ctx, r#"false"#);
}

#[bench]
fn _0021(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_008_b", &ctx, r#"false"#);
}

#[bench]
fn _0022(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_008_c", &ctx, r#"true"#);
}

#[bench]
fn _0023(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_008_d", &ctx, r#"true"#);
}

#[bench]
fn _0024(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_008_e", &ctx, r#"false"#);
}

#[bench]
fn _0025(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_009_a", &ctx, r#"false"#);
}

#[bench]
fn _0026(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_009_b", &ctx, r#"true"#);
}

#[bench]
fn _0027(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_009_c", &ctx, r#"true"#);
}

#[bench]
fn _0028(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_009_d", &ctx, r#"false"#);
}

#[bench]
fn _0029(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_009_e", &ctx, r#"false"#);
}

#[bench]
fn _0030(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_010_a", &ctx, r#"false"#);
}

#[bench]
fn _0031(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_010_b", &ctx, r#"true"#);
}

#[bench]
fn _0032(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_010_c", &ctx, r#"true"#);
}

#[bench]
fn _0033(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_010_d", &ctx, r#"true"#);
}

#[bench]
fn _0034(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_010_e", &ctx, r#"false"#);
}

#[bench]
fn _0035(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_011", &ctx, r#"true"#);
}

#[bench]
fn _0036(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_014", &ctx, r#"true"#);
}

#[bench]
fn _0037(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "number_014_a", &ctx, r#"false"#);
}

#[bench]
fn _0038(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_001", &ctx, r#"true"#);
}

#[bench]
fn _0039(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_001_a", &ctx, r#"false"#);
}

#[bench]
fn _0040(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_002", &ctx, r#"true"#);
}

#[bench]
fn _0041(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_002_a", &ctx, r#"false"#);
}

#[bench]
fn _0042(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_003", &ctx, r#"true"#);
}

#[bench]
fn _0043(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_003_1", &ctx, r#"true"#);
}

#[bench]
fn _0044(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_003_a", &ctx, r#"false"#);
}

#[bench]
fn _0045(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_004", &ctx, r#"true"#);
}

#[bench]
fn _0046(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_004_a", &ctx, r#"false"#);
}

#[bench]
fn _0047(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_005", &ctx, r#"true"#);
}

#[bench]
fn _0048(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_005_1", &ctx, r#"true"#);
}

#[bench]
fn _0049(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_005_a", &ctx, r#"false"#);
}

#[bench]
fn _0050(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_006", &ctx, r#"true"#);
}

#[bench]
fn _0051(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_006_a", &ctx, r#"false"#);
}

#[bench]
fn _0052(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_007_a", &ctx, r#"false"#);
}

#[bench]
fn _0053(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_007_b", &ctx, r#"false"#);
}

#[bench]
fn _0054(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_007_c", &ctx, r#"true"#);
}

#[bench]
fn _0055(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_007_d", &ctx, r#"false"#);
}

#[bench]
fn _0056(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_007_e", &ctx, r#"false"#);
}

#[bench]
fn _0057(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_008_a", &ctx, r#"false"#);
}

#[bench]
fn _0058(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_008_b", &ctx, r#"false"#);
}

#[bench]
fn _0059(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_008_c", &ctx, r#"true"#);
}

#[bench]
fn _0060(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_008_d", &ctx, r#"true"#);
}

#[bench]
fn _0061(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_008_e", &ctx, r#"false"#);
}

#[bench]
fn _0062(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_009_a", &ctx, r#"false"#);
}

#[bench]
fn _0063(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_009_b", &ctx, r#"true"#);
}

#[bench]
fn _0064(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_009_c", &ctx, r#"true"#);
}

#[bench]
fn _0065(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_009_d", &ctx, r#"false"#);
}

#[bench]
fn _0066(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_009_e", &ctx, r#"false"#);
}

#[bench]
fn _0067(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_010_a", &ctx, r#"false"#);
}

#[bench]
fn _0068(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_010_b", &ctx, r#"true"#);
}

#[bench]
fn _0069(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_010_c", &ctx, r#"true"#);
}

#[bench]
fn _0070(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_010_d", &ctx, r#"true"#);
}

#[bench]
fn _0071(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_010_e", &ctx, r#"false"#);
}

#[bench]
fn _0072(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_011", &ctx, r#"true"#);
}

#[bench]
fn _0073(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_014", &ctx, r#"true"#);
}

#[bench]
fn _0074(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "string_014_a", &ctx, r#"false"#);
}

#[bench]
fn _0075(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_001", &ctx, r#"true"#);
}

#[bench]
fn _0076(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_001_a", &ctx, r#"false"#);
}

#[bench]
fn _0077(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_011", &ctx, r#"true"#);
}

#[bench]
fn _0078(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_014", &ctx, r#"true"#);
}

#[bench]
fn _0079(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "boolean_014_a", &ctx, r#"false"#);
}

#[bench]
fn _0080(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_001", &ctx, r#"true"#);
}

#[bench]
fn _0081(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_001_a", &ctx, r#"false"#);
}

#[bench]
fn _0082(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_002", &ctx, r#"true"#);
}

#[bench]
fn _0083(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_002_a", &ctx, r#"false"#);
}

#[bench]
fn _0084(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_003", &ctx, r#"true"#);
}

#[bench]
fn _0085(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_003_1", &ctx, r#"true"#);
}

#[bench]
fn _0086(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_003_a", &ctx, r#"false"#);
}

#[bench]
fn _0087(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_004", &ctx, r#"true"#);
}

#[bench]
fn _0088(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_004_a", &ctx, r#"false"#);
}

#[bench]
fn _0089(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_005", &ctx, r#"true"#);
}

#[bench]
fn _0090(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_005_1", &ctx, r#"true"#);
}

#[bench]
fn _0091(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_005_a", &ctx, r#"false"#);
}

#[bench]
fn _0092(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_006", &ctx, r#"true"#);
}

#[bench]
fn _0093(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_006_a", &ctx, r#"false"#);
}

#[bench]
fn _0094(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_007_a", &ctx, r#"false"#);
}

#[bench]
fn _0095(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_007_b", &ctx, r#"false"#);
}

#[bench]
fn _0096(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_007_c", &ctx, r#"true"#);
}

#[bench]
fn _0097(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_007_d", &ctx, r#"false"#);
}

#[bench]
fn _0098(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_007_e", &ctx, r#"false"#);
}

#[bench]
fn _0099(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_008_a", &ctx, r#"false"#);
}

#[bench]
fn _0100(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_008_b", &ctx, r#"false"#);
}

#[bench]
fn _0101(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_008_c", &ctx, r#"true"#);
}

#[bench]
fn _0102(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_008_d", &ctx, r#"true"#);
}

#[bench]
fn _0103(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_008_e", &ctx, r#"false"#);
}

#[bench]
fn _0104(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_009_a", &ctx, r#"false"#);
}

#[bench]
fn _0105(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_009_b", &ctx, r#"true"#);
}

#[bench]
fn _0106(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_009_c", &ctx, r#"true"#);
}

#[bench]
fn _0107(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_009_d", &ctx, r#"false"#);
}

#[bench]
fn _0108(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_009_e", &ctx, r#"false"#);
}

#[bench]
fn _0109(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_010_a", &ctx, r#"false"#);
}

#[bench]
fn _0110(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_010_b", &ctx, r#"true"#);
}

#[bench]
fn _0111(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_010_c", &ctx, r#"true"#);
}

#[bench]
fn _0112(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_010_d", &ctx, r#"true"#);
}

#[bench]
fn _0113(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_010_e", &ctx, r#"false"#);
}

#[bench]
fn _0114(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_011", &ctx, r#"true"#);
}

#[bench]
fn _0115(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_014", &ctx, r#"true"#);
}

#[bench]
fn _0116(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "date_014_a", &ctx, r#"false"#);
}

#[bench]
fn _0117(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_001", &ctx, r#"true"#);
}

#[bench]
fn _0118(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_001_a", &ctx, r#"false"#);
}

#[bench]
fn _0119(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_002", &ctx, r#"true"#);
}

#[bench]
fn _0120(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_002_a", &ctx, r#"false"#);
}

#[bench]
fn _0121(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_003", &ctx, r#"true"#);
}

#[bench]
fn _0122(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_003_1", &ctx, r#"true"#);
}

#[bench]
fn _0123(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_003_a", &ctx, r#"false"#);
}

#[bench]
fn _0124(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_004", &ctx, r#"true"#);
}

#[bench]
fn _0125(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_004_a", &ctx, r#"false"#);
}

#[bench]
fn _0126(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_005", &ctx, r#"true"#);
}

#[bench]
fn _0127(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_005_1", &ctx, r#"true"#);
}

#[bench]
fn _0128(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_005_a", &ctx, r#"false"#);
}

#[bench]
fn _0129(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_006", &ctx, r#"true"#);
}

#[bench]
fn _0130(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_006_a", &ctx, r#"false"#);
}

#[bench]
fn _0131(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_007_a", &ctx, r#"false"#);
}

#[bench]
fn _0132(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_007_b", &ctx, r#"false"#);
}

#[bench]
fn _0133(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_007_c", &ctx, r#"true"#);
}

#[bench]
fn _0134(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_007_d", &ctx, r#"false"#);
}

#[bench]
fn _0135(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_007_e", &ctx, r#"false"#);
}

#[bench]
fn _0136(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_008_a", &ctx, r#"false"#);
}

#[bench]
fn _0137(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_008_b", &ctx, r#"false"#);
}

#[bench]
fn _0138(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_008_c", &ctx, r#"true"#);
}

#[bench]
fn _0139(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_008_d", &ctx, r#"true"#);
}

#[bench]
fn _0140(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_008_e", &ctx, r#"false"#);
}

#[bench]
fn _0141(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_009_a", &ctx, r#"false"#);
}

#[bench]
fn _0142(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_009_b", &ctx, r#"true"#);
}

#[bench]
fn _0143(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_009_c", &ctx, r#"true"#);
}

#[bench]
fn _0144(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_009_d", &ctx, r#"false"#);
}

#[bench]
fn _0145(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_009_e", &ctx, r#"false"#);
}

#[bench]
fn _0146(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_010_a", &ctx, r#"false"#);
}

#[bench]
fn _0147(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_010_b", &ctx, r#"true"#);
}

#[bench]
fn _0148(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_010_c", &ctx, r#"true"#);
}

#[bench]
fn _0149(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_010_d", &ctx, r#"true"#);
}

#[bench]
fn _0150(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_010_e", &ctx, r#"false"#);
}

#[bench]
fn _0151(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_011", &ctx, r#"true"#);
}

#[bench]
fn _0152(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_014", &ctx, r#"true"#);
}

#[bench]
fn _0153(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "time_014_a", &ctx, r#"false"#);
}

#[bench]
fn _0154(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_001", &ctx, r#"true"#);
}

#[bench]
fn _0155(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_001_a", &ctx, r#"false"#);
}

#[bench]
fn _0156(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_002", &ctx, r#"true"#);
}

#[bench]
fn _0157(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_002_a", &ctx, r#"false"#);
}

#[bench]
fn _0158(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_003", &ctx, r#"true"#);
}

#[bench]
fn _0159(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_003_1", &ctx, r#"true"#);
}

#[bench]
fn _0160(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_003_a", &ctx, r#"false"#);
}

#[bench]
fn _0161(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_004", &ctx, r#"true"#);
}

#[bench]
fn _0162(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_004_a", &ctx, r#"false"#);
}

#[bench]
fn _0163(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_005", &ctx, r#"true"#);
}

#[bench]
fn _0164(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_005_1", &ctx, r#"true"#);
}

#[bench]
fn _0165(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_005_a", &ctx, r#"false"#);
}

#[bench]
fn _0166(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_006", &ctx, r#"true"#);
}

#[bench]
fn _0167(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_006_a", &ctx, r#"false"#);
}

#[bench]
fn _0168(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_007_a", &ctx, r#"false"#);
}

#[bench]
fn _0169(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_007_b", &ctx, r#"false"#);
}

#[bench]
fn _0170(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_007_c", &ctx, r#"true"#);
}

#[bench]
fn _0171(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_007_d", &ctx, r#"false"#);
}

#[bench]
fn _0172(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_007_e", &ctx, r#"false"#);
}

#[bench]
fn _0173(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_008_a", &ctx, r#"false"#);
}

#[bench]
fn _0174(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_008_b", &ctx, r#"false"#);
}

#[bench]
fn _0175(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_008_c", &ctx, r#"true"#);
}

#[bench]
fn _0176(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_008_d", &ctx, r#"true"#);
}

#[bench]
fn _0177(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_008_e", &ctx, r#"false"#);
}

#[bench]
fn _0178(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_009_a", &ctx, r#"false"#);
}

#[bench]
fn _0179(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_009_b", &ctx, r#"true"#);
}

#[bench]
fn _0180(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_009_c", &ctx, r#"true"#);
}

#[bench]
fn _0181(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_009_d", &ctx, r#"false"#);
}

#[bench]
fn _0182(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_009_e", &ctx, r#"false"#);
}

#[bench]
fn _0183(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_010_a", &ctx, r#"false"#);
}

#[bench]
fn _0184(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_010_b", &ctx, r#"true"#);
}

#[bench]
fn _0185(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_010_c", &ctx, r#"true"#);
}

#[bench]
fn _0186(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_010_d", &ctx, r#"true"#);
}

#[bench]
fn _0187(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_010_e", &ctx, r#"false"#);
}

#[bench]
fn _0188(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_011", &ctx, r#"true"#);
}

#[bench]
fn _0189(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_014", &ctx, r#"true"#);
}

#[bench]
fn _0190(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dateTime_014_a", &ctx, r#"false"#);
}

#[bench]
fn _0191(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_001", &ctx, r#"true"#);
}

#[bench]
fn _0192(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_001_a", &ctx, r#"false"#);
}

#[bench]
fn _0193(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_011_a", &ctx, r#"true"#);
}

#[bench]
fn _0194(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_011_b", &ctx, r#"false"#);
}

#[bench]
fn _0195(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "list_014_a", &ctx, r#"false"#);
}

#[bench]
fn _0196(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_001", &ctx, r#"true"#);
}

#[bench]
fn _0197(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_001_a", &ctx, r#"false"#);
}

#[bench]
fn _0198(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_011", &ctx, r#"true"#);
}

#[bench]
fn _0199(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_014", &ctx, r#"true"#);
}

#[bench]
fn _0200(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "context_014_a", &ctx, r#"false"#);
}

#[bench]
fn _0201(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_001", &ctx, r#"true"#);
}

#[bench]
fn _0202(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_001_a", &ctx, r#"false"#);
}

#[bench]
fn _0203(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_002", &ctx, r#"true"#);
}

#[bench]
fn _0204(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_002_a", &ctx, r#"false"#);
}

#[bench]
fn _0205(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_003", &ctx, r#"true"#);
}

#[bench]
fn _0206(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_003_1", &ctx, r#"true"#);
}

#[bench]
fn _0207(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_003_a", &ctx, r#"false"#);
}

#[bench]
fn _0208(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_004", &ctx, r#"true"#);
}

#[bench]
fn _0209(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_004_a", &ctx, r#"false"#);
}

#[bench]
fn _0210(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_005", &ctx, r#"true"#);
}

#[bench]
fn _0211(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_005_1", &ctx, r#"true"#);
}

#[bench]
fn _0212(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_005_a", &ctx, r#"false"#);
}

#[bench]
fn _0213(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_006", &ctx, r#"true"#);
}

#[bench]
fn _0214(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_006_a", &ctx, r#"false"#);
}

#[bench]
fn _0215(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_007_a", &ctx, r#"false"#);
}

#[bench]
fn _0216(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_007_b", &ctx, r#"false"#);
}

#[bench]
fn _0217(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_007_c", &ctx, r#"true"#);
}

#[bench]
fn _0218(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_007_d", &ctx, r#"false"#);
}

#[bench]
fn _0219(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_007_e", &ctx, r#"false"#);
}

#[bench]
fn _0220(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_008_a", &ctx, r#"false"#);
}

#[bench]
fn _0221(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_008_b", &ctx, r#"false"#);
}

#[bench]
fn _0222(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_008_c", &ctx, r#"true"#);
}

#[bench]
fn _0223(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_008_d", &ctx, r#"true"#);
}

#[bench]
fn _0224(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_008_e", &ctx, r#"false"#);
}

#[bench]
fn _0225(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_009_a", &ctx, r#"false"#);
}

#[bench]
fn _0226(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_009_b", &ctx, r#"true"#);
}

#[bench]
fn _0227(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_009_c", &ctx, r#"true"#);
}

#[bench]
fn _0228(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_009_d", &ctx, r#"false"#);
}

#[bench]
fn _0229(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_009_e", &ctx, r#"false"#);
}

#[bench]
fn _0230(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_010_a", &ctx, r#"false"#);
}

#[bench]
fn _0231(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_010_b", &ctx, r#"true"#);
}

#[bench]
fn _0232(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_010_c", &ctx, r#"true"#);
}

#[bench]
fn _0233(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_010_d", &ctx, r#"true"#);
}

#[bench]
fn _0234(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_010_e", &ctx, r#"false"#);
}

#[bench]
fn _0235(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_011", &ctx, r#"true"#);
}

#[bench]
fn _0236(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_014", &ctx, r#"true"#);
}

#[bench]
fn _0237(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "ym_duration_014_a", &ctx, r#"false"#);
}

#[bench]
fn _0238(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_001", &ctx, r#"true"#);
}

#[bench]
fn _0239(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_001_a", &ctx, r#"false"#);
}

#[bench]
fn _0240(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_002", &ctx, r#"true"#);
}

#[bench]
fn _0241(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_002_a", &ctx, r#"false"#);
}

#[bench]
fn _0242(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_003", &ctx, r#"true"#);
}

#[bench]
fn _0243(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_003_1", &ctx, r#"true"#);
}

#[bench]
fn _0244(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_003_a", &ctx, r#"false"#);
}

#[bench]
fn _0245(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_004", &ctx, r#"true"#);
}

#[bench]
fn _0246(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_004_a", &ctx, r#"false"#);
}

#[bench]
fn _0247(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_005", &ctx, r#"true"#);
}

#[bench]
fn _0248(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_005_1", &ctx, r#"true"#);
}

#[bench]
fn _0249(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_005_a", &ctx, r#"false"#);
}

#[bench]
fn _0250(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_006", &ctx, r#"true"#);
}

#[bench]
fn _0251(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_006_a", &ctx, r#"false"#);
}

#[bench]
fn _0252(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_007_a", &ctx, r#"false"#);
}

#[bench]
fn _0253(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_007_b", &ctx, r#"false"#);
}

#[bench]
fn _0254(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_007_c", &ctx, r#"true"#);
}

#[bench]
fn _0255(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_007_d", &ctx, r#"false"#);
}

#[bench]
fn _0256(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_007_e", &ctx, r#"false"#);
}

#[bench]
fn _0257(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_008_a", &ctx, r#"false"#);
}

#[bench]
fn _0258(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_008_b", &ctx, r#"false"#);
}

#[bench]
fn _0259(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_008_c", &ctx, r#"true"#);
}

#[bench]
fn _0260(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_008_d", &ctx, r#"true"#);
}

#[bench]
fn _0261(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_008_e", &ctx, r#"false"#);
}

#[bench]
fn _0262(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_009_a", &ctx, r#"false"#);
}

#[bench]
fn _0263(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_009_b", &ctx, r#"true"#);
}

#[bench]
fn _0264(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_009_c", &ctx, r#"true"#);
}

#[bench]
fn _0265(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_009_d", &ctx, r#"false"#);
}

#[bench]
fn _0266(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_009_e", &ctx, r#"false"#);
}

#[bench]
fn _0267(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_010_a", &ctx, r#"false"#);
}

#[bench]
fn _0268(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_010_b", &ctx, r#"true"#);
}

#[bench]
fn _0269(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_010_c", &ctx, r#"true"#);
}

#[bench]
fn _0270(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_010_d", &ctx, r#"true"#);
}

#[bench]
fn _0271(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_010_e", &ctx, r#"false"#);
}

#[bench]
fn _0272(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_011", &ctx, r#"true"#);
}

#[bench]
fn _0273(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_014", &ctx, r#"true"#);
}

#[bench]
fn _0274(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "dt_duration_014_a", &ctx, r#"false"#);
}

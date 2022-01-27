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
use crate::compliance::{assert_business_knowledge_model, assert_decision, context};
use dmntk_model_evaluator::ModelEvaluator;
use std::sync::Arc;
use test::Bencher;

lazy_static! {
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0014);
}

#[bench]
fn _0001(b: &mut Bencher) {
  let input_data = &context(r#"{p: 1, r: 1, n: 1, pmt: 1}"#);
  let invocable_name = "equity36Mo";
  assert_business_knowledge_model(&MODEL_EVALUATOR, invocable_name, input_data, r#"1"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, input_data));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let input_data = &context(r#"{p: 2, r: 1, n: 1, pmt: 1}"#);
  let invocable_name = "equity36Mo";
  assert_business_knowledge_model(&MODEL_EVALUATOR, invocable_name, input_data, r#"2.083333333333333333333333333333333"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, input_data));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let input_data = &context(r#"{p: 1, r: 1, n: 1}"#);
  let invocable_name = "monthlyPayment";
  assert_business_knowledge_model(&MODEL_EVALUATOR, invocable_name, input_data, r#"1.083333333333333333333333333333338"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, input_data));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let input_data = &context(r#"{p: 2, r: 1, n: 1}"#);
  let invocable_name = "monthlyPayment";
  assert_business_knowledge_model(&MODEL_EVALUATOR, invocable_name, input_data, r#"2.166666666666666666666666666666676"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, input_data));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let input_data = &context(r#"{requestedAmt: 330000, product: {fee: 0, lenderName: "Oceans Capital", points: 0, rate: 0.03500}}"#);
  let invocable_name = "FinancialMetrics";
  assert_business_knowledge_model(
    &MODEL_EVALUATOR,
    invocable_name,
    input_data,
    r#"{downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891032, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.847469769120902911415325410838, points: 0, rate: 0.03500}"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, input_data));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let input_data = &context(r#"{}"#);
  let invocable_name = "Bankrates";
  assert_decision(
    &MODEL_EVALUATOR,
    invocable_name,
    input_data,
    r#"[{fee: 0, lenderName: "Oceans Capital", points: 0, rate: 0.03500}, {fee: 2700, lenderName: "eClick Lending", points: 1.1, rate: 0.03200}, {fee: 1200, lenderName: "eClickLending", points: 0.1, rate: 0.03375}, {fee: 3966, lenderName: "AimLoan", points: 1.1, rate: 0.03000}, {fee: 285, lenderName: "Home Loans Today", points: 1.1, rate: 0.03125}, {fee: 4028, lenderName: "Sebonic", points: 0.1, rate: 0.03125}, {fee: 4317, lenderName: "AimLoan", points: 0.1, rate: 0.03125}, {fee: 2518, lenderName: "eRates Mortgage", points: 1.1, rate: 0.03125}, {fee: 822, lenderName: "Home Loans Today", points: 0.1, rate: 0.03250}, {fee: 1995, lenderName: "AimLoan", points: 0, rate: 0.03250}]"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, input_data));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let input_data = &context(r#"{RequestedAmt: 330000}"#);
  let invocable_name = "RankedProducts";
  assert_decision(
    &MODEL_EVALUATOR,
    invocable_name,
    input_data,
    r#"{metricsTable: [{downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891032, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.847469769120902911415325410838, points: 0, rate: 0.03500}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846162699216, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120545, points: 1.1, rate: 0.03200}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850624, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552193, points: 0.1, rate: 0.03375}, {downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225077008, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688247, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.120782970248151726934352923794468, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.307593257382315875048929769861, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607792, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911139, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233014543176, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323106, points: 1.1, rate: 0.03125}, {downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170865041632, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550485, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555808, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552276, points: 0, rate: 0.03250}], rankByDownPmt: [{downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891032, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.847469769120902911415325410838, points: 0, rate: 0.03500}, {downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170865041632, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550485, points: 0.1, rate: 0.03250}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850624, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552193, points: 0.1, rate: 0.03375}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555808, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552276, points: 0, rate: 0.03250}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225077008, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688247, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.120782970248151726934352923794468, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.307593257382315875048929769861, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607792, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911139, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233014543176, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323106, points: 1.1, rate: 0.03125}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846162699216, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120545, points: 1.1, rate: 0.03200}, {downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}], rankByEquityPct: [{downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170865041632, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550485, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555808, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552276, points: 0, rate: 0.03250}, {downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891032, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.847469769120902911415325410838, points: 0, rate: 0.03500}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850624, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552193, points: 0.1, rate: 0.03375}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225077008, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688247, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.120782970248151726934352923794468, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.307593257382315875048929769861, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607792, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911139, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233014543176, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323106, points: 1.1, rate: 0.03125}, {downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846162699216, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120545, points: 1.1, rate: 0.03200}], rankByMonthlyPmt: [{downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225077008, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688247, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.120782970248151726934352923794468, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.307593257382315875048929769861, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607792, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911139, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233014543176, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323106, points: 1.1, rate: 0.03125}, {downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170865041632, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550485, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555808, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552276, points: 0, rate: 0.03250}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846162699216, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120545, points: 1.1, rate: 0.03200}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850624, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552193, points: 0.1, rate: 0.03375}, {downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891032, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.847469769120902911415325410838, points: 0, rate: 0.03500}], rankByRate: [{downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225077008, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688247, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.120782970248151726934352923794468, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.307593257382315875048929769861, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607792, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911139, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.1160760438900092317382233014543176, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323106, points: 1.1, rate: 0.03125}, {downPmtAmt: 67266, equity36moPct: 0.1137029731874587934446846162699216, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120545, points: 1.1, rate: 0.03200}, {downPmtAmt: 66230.4, equity36moPct: 0.1261025270361081947340170865041632, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550485, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555808, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552276, points: 0, rate: 0.03250}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850624, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552193, points: 0.1, rate: 0.03375}, {downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891032, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.847469769120902911415325410838, points: 0, rate: 0.03500}]}"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, input_data));
}

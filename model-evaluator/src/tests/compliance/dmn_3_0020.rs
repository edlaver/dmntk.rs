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

use super::super::*;
use crate::model_evaluator::ModelEvaluator;
use std::sync::Arc;

lazy_static! {
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0020);
}

#[test]
fn _0001() {
  let ctx = context(r#"{Age: 16,Years of Service: 1}"#);
  assert_decision(&MODEL_EVALUATOR, "Total Vacation Days", &ctx, r#"27"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{Age: 25,Years of Service: 5}"#);
  assert_decision(&MODEL_EVALUATOR, "Total Vacation Days", &ctx, r#"22"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{Age: 25,Years of Service: 20}"#);
  assert_decision(&MODEL_EVALUATOR, "Total Vacation Days", &ctx, r#"24"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{Age: 44,Years of Service: 30}"#);
  assert_decision(&MODEL_EVALUATOR, "Total Vacation Days", &ctx, r#"30"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{Age: 50,Years of Service: 20}"#);
  assert_decision(&MODEL_EVALUATOR, "Total Vacation Days", &ctx, r#"24"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{Age: 50,Years of Service: 30}"#);
  assert_decision(&MODEL_EVALUATOR, "Total Vacation Days", &ctx, r#"30"#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{Age: 60,Years of Service: 20}"#);
  assert_decision(&MODEL_EVALUATOR, "Total Vacation Days", &ctx, r#"30"#);
}

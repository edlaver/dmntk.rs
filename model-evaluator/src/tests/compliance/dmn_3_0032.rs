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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0032);
}

#[test]
fn _0001() {
  let ctx = context(r#"{bool: true,num: 100}"#);
  assert_decision(&MODEL_EVALUATOR, "simpleIf", &ctx, r#"110"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{bool: false,num: 100}"#);
  assert_decision(&MODEL_EVALUATOR, "simpleIf", &ctx, r#"90"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{bool: null,num: 100}"#);
  assert_decision(&MODEL_EVALUATOR, "simpleIf", &ctx, r#"90"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{aDate: @"2017-01-02",aString: "Hello World"}"#);
  assert_decision(&MODEL_EVALUATOR, "conditionWithFunctions", &ctx, r#""Hello""#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{aDate: @"2017-01-01",aString: "Hello World"}"#);
  assert_decision(&MODEL_EVALUATOR, "conditionWithFunctions", &ctx, r#""World""#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{aDate: null,aString: "Hello World"}"#);
  assert_decision(&MODEL_EVALUATOR, "conditionWithFunctions", &ctx, r#""World""#);
}

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
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0080);
}

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision_001",
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 0)"#,
  );
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision_002",
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 1)"#,
  );
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision_003",
    &ctx,
    r#"null(expected 2 parameters, actual number of parameters is 3)"#,
  );
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "decision_004", &ctx, r#""foo""#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision_005",
    &ctx,
    r#"null([core::get value] invalid argument type, expected context, actual type is string)"#,
  );
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision_006",
    &ctx,
    r#"null([core::get value] invalid argument type, expected string, actual type is number)"#,
  );
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "decision_007", &ctx, r#""foo""#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "decision_008", &ctx, r#"null(parameter 'key' not found)"#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision_009",
    &ctx,
    r#"null([core::get value] invalid argument type, expected context, actual type is Null)"#,
  );
}

#[test]
fn _0010() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision_010",
    &ctx,
    r#"null([core::get value] invalid argument type, expected string, actual type is Null)"#,
  );
}

#[test]
fn _0011() {
  let ctx = context(r#"{}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "decision_011",
    &ctx,
    r#"null([core::get value] invalid argument type, expected context, actual type is Null)"#,
  );
}

#[test]
fn _0012() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, "decision_012", &ctx, r#"null"#);
}

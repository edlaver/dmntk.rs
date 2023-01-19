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
use crate::model_evaluator::ModelEvaluator;
use std::sync::Arc;

lazy_static! {
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0031);
  static ref CTX: FeelContext = context(r#"{inputA: 10, inputB: 5}"#);
}

#[test]
fn _0001() {
  assert_decision(
    &MODEL_EVALUATOR,
    "fn invocation positional parameters",
    &CTX,
    r#"{divisionResultPositional: 2, multiplicationResultPositional: 50, sumResult: 15}"#,
  );
}

#[test]
fn _0002() {
  assert_decision(
    &MODEL_EVALUATOR,
    "fn invocation named parameters",
    &CTX,
    r#"{divisionResultNamed: 2, multiplicationResultNamed: 50, subResult: 5, subResultMixed: -5}"#,
  );
}

#[test]
fn _0003() {
  assert_decision(
    &MODEL_EVALUATOR,
    "fn invocation complex parameters",
    &CTX,
    r#"{circumference: 94.24776, functionInvocationInParameter: 200, functionInvocationLiteralExpressionInParameter: 500}"#,
  );
}

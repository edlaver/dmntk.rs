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

use super::super::*;

from_examples!(DMN_3_1107);

#[test]
fn _0001() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "feel-substring-before-function_001_2e948ccdc7", &ctx, r#""foo""#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "feel-substring-before-function_002_f4de663db7", &ctx, r#""f""#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "feel-substring-before-function_003_60bc15bade", &ctx, r#""""#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "feel-substring-before-function_004_6f93aa7654", &ctx, r#""""#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "feel-substring-before-function_005_c8e030633c", &ctx, r#""""#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "feel-substring-before-function_006_1c3d39811d", &ctx, r#""""#);
}

#[test]
fn _0007() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "feel-substring-before-function_007_ea4e4a38b0", &ctx, r#""ab""#);
}

#[test]
fn _0008() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "feel-substring-before-function_008_501b0a5990", &ctx, r#""foo""#);
}

#[test]
fn _0009() {
  let ctx = context(r#"{}"#);
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, "feel-substring-before-function_009_a79c5a4111", &ctx, r#""foo""#);
}

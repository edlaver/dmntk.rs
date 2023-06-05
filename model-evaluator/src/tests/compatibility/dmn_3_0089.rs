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
use dmntk_examples::*;

static MODEL_EVALUATOR: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089, DMN_3_0089_MODEL_B1, DMN_3_0089_MODEL_A, DMN_3_0089_MODEL_B2]));
static MODEL_EVALUATOR_A: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_A]));
static MODEL_EVALUATOR_B1: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_A, DMN_3_0089_MODEL_B1]));
static MODEL_EVALUATOR_B2: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_A, DMN_3_0089_MODEL_B2]));

#[test]
fn _0001() {
  let ctx = context(r#" { Person name: "Jenny" } "#);
  assert_decision_1(
    &MODEL_EVALUATOR,
    "http://www.trisotech.com/definitions/_10435dcd-8774-4575-a338-49dd554a0928",
    "Model C Decision based on Bs",
    &ctx,
    r#""B: Evaluating Say Hello to: Hello, Jenny; B2: Evaluating Say Hello to: Hello, Jenny""#,
  );
}

#[test]
fn _0002() {
  let ctx = context(r#" { Person name: "Jenny" } "#);
  assert_decision_1(
    &MODEL_EVALUATOR_A,
    "http://www.trisotech.com/definitions/_ae5b3c17-1ac3-4e1d-b4f9-2cf861aec6d9",
    "Greet the Person",
    &ctx,
    r#""Hello, Jenny""#,
  );
}

#[test]
fn _0003() {
  let ctx = context(r#" { Person name: "Jenny" } "#);
  assert_decision_1(
    &MODEL_EVALUATOR_B1,
    "http://www.trisotech.com/definitions/_2a1d771a-a899-4fef-abd6-fc894332337c",
    "Evaluating Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, Jenny""#,
  );
}

#[test]
fn _0004() {
  let ctx = context(r#" { Person name: "Cecil" } "#);
  assert_decision_1(
    &MODEL_EVALUATOR_B2,
    "http://www.trisotech.com/definitions/_9d46ece4-a96c-4cb0-abc0-0ca121ac3768",
    "Evaluating B2 Say Hello",
    &ctx,
    r#""Evaluating Say Hello to: Hello, Cecil""#,
  );
}

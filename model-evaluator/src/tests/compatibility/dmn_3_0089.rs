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
static MODEL_EVALUATOR_A: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluator(DMN_3_0089_MODEL_A));
static MODEL_EVALUATOR_B: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluators(&[DMN_3_0089_MODEL_B1, DMN_3_0089_MODEL_A]));

#[test]
#[ignore]
fn _0001() {
  let ctx = context(
    r#"{
      Model B:  { modelA: { Person name: "B.A.John"   } },
      Model B2: { modelA: { Person name: "B2.A.John2" } }
    }"#,
  );
  assert_decision(
    &MODEL_EVALUATOR,
    "Model C Decision based on Bs",
    &ctx,
    r#""B: Evaluating Say Hello to: Hello, B.A.John; B2: Evaluating Say Hello to: Hello, B2.A.John2""#,
  );
}

#[test]
fn _0002() {
  let ctx = context(r#" { Person name: "Darius" } "#);
  assert_decision(&MODEL_EVALUATOR_A, "Greet the Person", &ctx, r#""Hello, Darius""#);
}

#[test]
#[ignore]
fn _0003() {
  let ctx = context(r#" { Person name: "Darius" } "#);
  assert_decision(&MODEL_EVALUATOR_B, "Evaluating Say Hello", &ctx, r#""Hello, Darius""#);
}

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

static MODEL_EVALUATOR: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluator(dmntk_examples::DMN_3_0008));

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "listGen1";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"["a", "b", "c"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(r#"{a: "a", b: "b", c: "c"}"#);
  let invocable_name = "listGen2";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"["a", "b", "c"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(r#"{b: "b", c: "c"}"#);
  let invocable_name = "listGen3";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"["a", "b", "c"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(invocable_name, &ctx));
}

#[bench]
fn _0004(b: &mut Bencher) {
  let ctx = context(r#"{c: "c"}"#);
  let invocable_name = "listGen4";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"["a", "b", "c"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(invocable_name, &ctx));
}

#[bench]
fn _0005(b: &mut Bencher) {
  let ctx = context(r#"{a: "a", b: "b", c: "c"}"#);
  let invocable_name = "listGen5";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"["a", "b", "c"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(invocable_name, &ctx));
}

#[bench]
fn _0006(b: &mut Bencher) {
  let ctx = context(r#"{}"#);
  let invocable_name = "listGen6";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"["w", "x", "y", "z"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(invocable_name, &ctx));
}

#[bench]
fn _0007(b: &mut Bencher) {
  let ctx = context(r#"{wx: ["w", "x"]}"#);
  let invocable_name = "listGen7";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"["w", "x", "y", "z"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(invocable_name, &ctx));
}

#[bench]
fn _0008(b: &mut Bencher) {
  let ctx = context(r#"{a: "a", b: "b"}"#);
  let invocable_name = "listGen8";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"["a", "b", "w", "x", "y", "z"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(invocable_name, &ctx));
}

#[bench]
fn _0009(b: &mut Bencher) {
  let ctx = context(r#"{a: "a", b: "b", wx: ["w", "x"]}"#);
  let invocable_name = "listGen9";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"["a", "b", "w", "x", "y", "z"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(invocable_name, &ctx));
}

#[bench]
fn _0010(b: &mut Bencher) {
  let ctx = context(r#"{c: "c", wx: ["w", "x"]}"#);
  let invocable_name = "listGen10";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"["a", "b", "c", "w", "x", "y", "z"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable_by_name(invocable_name, &ctx));
}

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

use super::*;

static MODEL_EVALUATOR: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluator(dmntk_examples::DMN_3_0013));

#[bench]
fn _0001(b: &mut Bencher) {
  let ctx = context(
    r#"{listA: [3, 1, 5, 4], stringList: ["a", "8", "Aa", "A", "10", "9"], tableB: [{col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}, {col1: 1, col2: 0, col3: 1, col4: 1}]}"#,
  );
  let invocable_name = "sort1";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"[5, 4, 3, 1]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0002(b: &mut Bencher) {
  let ctx = context(
    r#"{listA: [3, 1, 5, 4], stringList: ["a", "8", "Aa", "A", "10", "9"], tableB: [{col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}, {col1: 1, col2: 0, col3: 1, col4: 1}]}"#,
  );
  let invocable_name = "sort2";
  assert_decision(
    &MODEL_EVALUATOR,
    invocable_name,
    &ctx,
    r#"[{col1: 1, col2: 0, col3: 1, col4: 1}, {col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}]"#,
  );
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

#[bench]
fn _0003(b: &mut Bencher) {
  let ctx = context(
    r#"{listA: [3, 1, 5, 4], stringList: ["a", "8", "Aa", "A", "10", "9"], tableB: [{col1: 16, col2: 4, col3: 25, col4: 1}, {col1: 16, col2: 43, col3: 2, col4: 10}, {col1: 1, col2: 0, col3: 1, col4: 1}]}"#,
  );
  let invocable_name = "sort3";
  assert_decision(&MODEL_EVALUATOR, invocable_name, &ctx, r#"["10", "8", "9", "A", "Aa", "a"]"#);
  b.iter(|| MODEL_EVALUATOR.evaluate_invocable(invocable_name, &ctx));
}

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

static_model_evaluator_examples!(DMN_3_0002);

#[test]
fn _0001() {
  let ctx = context(r#"{A: "banana", B: "a", NumC: 2}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "Basic",
    &ctx,
    r#"{containsB: true, containsX: false, endsWithB: true, endsWithX: false, lowercase: "a", startsWithB: false, startsWithX: false, stringlength: 6, substringAfterB: "nana", substringBeforeB: "b", substringC1: "a", uppercase: "BANANA"}"#,
  );
}

#[test]
fn _0002() {
  let ctx = context(r#"{A: "banana"}"#);
  assert_decision(&MODEL_EVALUATOR, "Matches", &ctx, r#"true"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{A: "banana"}"#);
  assert_decision(
    &MODEL_EVALUATOR,
    "Replace",
    &ctx,
    r#"{AanplusStarstar: "b**a", Aao: "bonono", encloseVowels: "b[a]n[a]n[a]"}"#,
  );
}

#[test]
fn _0004() {
  let ctx = context(r#"{NumC: 2}"#);
  assert_decision(&MODEL_EVALUATOR, "Constructor", &ctx, r#""2""#);
}

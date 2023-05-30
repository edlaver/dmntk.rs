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

static_model_evaluator_examples!(DMN_3_0040);

#[test]
fn _0001() {
  let ctx = context(r#"{Principal: 600000,Term: 360}"#);
  assert_decision(&MODEL_EVALUATOR, "Boxed Context", &ctx, r#"2878.693549432766768088520383236288"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{Principal: 30000,Term: 60}"#);
  assert_decision(&MODEL_EVALUATOR, "Boxed Context", &ctx, r#"649.1175498364002934927000148859422"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{Principal: 600000,Term: 365}"#);
  assert_decision(&MODEL_EVALUATOR, "Boxed Context", &ctx, r#"2858.11609989659140087141889328903"#);
}

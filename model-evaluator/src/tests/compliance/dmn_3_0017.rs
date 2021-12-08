/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
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

use crate::tests::{assert_decision, context};

lazy_static! {
  static ref DEFINITIONS: dmntk_model::model::Definitions = dmntk_model::parse(dmntk_examples::DMN_3_0017, "file: ///3_0017.dmn").unwrap();
}

#[test]
fn _0001() {
  let ctx = context(r#"{structA: {name: "widget",price: 20}}"#);
  assert_decision(&DEFINITIONS, "priceGt10", &ctx, r#"true"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{numB: 9,numC: 10,structA: {name: "widget",price: 20}}"#);
  assert_decision(&DEFINITIONS, "priceInRange", &ctx, r#""Not in range""#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{dateD: @"2016-11-01"}"#);
  assert_decision(&DEFINITIONS, "dateCompare1", &ctx, r#"true"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{dateD: @"2016-11-01",dateE: @"2016-11-02"}"#);
  assert_decision(&DEFINITIONS, "dateCompare2", &ctx, r#"false"#);
}

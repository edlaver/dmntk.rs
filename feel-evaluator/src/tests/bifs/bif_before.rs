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

use super::super::*;

#[test]
fn _0001() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(1,10)", true);
}

#[test]
fn _0002() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(10,1)", false);
}

#[test]
fn _0003() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(10,10)", false);
}

#[test]
fn _0004() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(1,[1..10])", false);
}

#[test]
fn _0005() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(1,(1..10])", true);
}

#[test]
fn _0006() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(1,[5..10])", true);
}

#[test]
fn _0007() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before([1..10],10)", false);
}

#[test]
fn _0008() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before([1..10),10)", true);
}

#[test]
fn _0009() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before([1..10],15)", true);
}

#[test]
fn _0010() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before([1..10],[15..20])", true);
}

#[test]
fn _0011() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before([1..10],[10..20])", false);
}

#[test]
fn _0012() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before([1..10),[10..20])", true);
}

#[test]
fn _0013() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before([1..10],(10..20])", true);
}

#[test]
fn _0014() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before([1..10),(10..20])", true);
}

#[test]
fn _0015() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(point1:1,point2:10)", true);
}
#[test]
fn _0016() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(point2:10,point1:1)", true);
}

#[test]
fn _0017() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(point1:10,point2:1)", false);
}

#[test]
fn _0018() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(point1:10,point2:10)", false);
}

#[test]
fn _0019() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(range1:[1..10),range2:(10..20])", true);
}

#[test]
fn _0020() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(point:1,range:(1..10])", true);
}

#[test]
fn _0021() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "before(range:(1..10],point:1)", false);
}

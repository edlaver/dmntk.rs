/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
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
use dmntk_feel::scope;

#[test]
fn _0001() {
  te_be_value(false, &scope!(), r#"sublist([1,2,3,4,5,6,7,8,9,10],3,4)"#, r#"[3,4,5,6]"#);
}

#[test]
fn _0002() {
  te_be_value(false, &scope!(), r#"sublist([1,2,3,4,5,6,7,8,9,10],3)"#, r#"[3,4,5,6,7,8,9,10]"#);
}

#[test]
fn _0003() {
  te_be_value(false, &scope!(), r#"sublist([1,2,3,4,5,6,7,8,9,10],-1,1)"#, r#"[10]"#);
}

#[test]
fn _0004() {
  te_be_value(false, &scope!(), r#"sublist([1,2,3,4,5,6,7,8,9,10],-6,4)"#, r#"[5,6,7,8]"#);
}

#[test]
fn _0005() {
  te_be_value(false, &scope!(), r#"sublist(list: [1,2,3,4,5,6,7,8,9,10], start position: -6, length: 4)"#, r#"[5,6,7,8]"#);
}

#[test]
fn _0006() {
  te_be_value(false, &scope!(), r#"sublist(list: [1,2,3,4,5,6,7,8,9,10], start position: -6)"#, r#"[5,6,7,8,9,10]"#);
}

#[test]
fn _0007() {
  te_be_value(false, &scope!(), r#"sublist(list: [1,2,3,4,5,6,7,8,9,10], start position: 3)"#, r#"[3,4,5,6,7,8,9,10]"#);
}

#[test]
fn _0008() {
  te_null(false, &scope!(), r#"sublist()"#, r#"expected 2,3 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0009() {
  te_null(false, &scope!(), r#"sublist([])"#, r#"expected 2,3 parameters, actual number of parameters is 1"#);
}

#[test]
fn _0010() {
  te_null(false, &scope!(), r#"sublist([],1,1,1)"#, r#"expected 2,3 parameters, actual number of parameters is 4"#);
}

#[test]
fn _0011() {
  te_null(false, &scope!(), r#"sublist(l:[1,2,3], start position: 1, length: 1)"#, r#"parameter 'list' not found"#);
}

#[test]
fn _0012() {
  te_null(false, &scope!(), r#"sublist(list:[1,2,3], sp: 1, length: 1)"#, r#"parameter 'start position' not found"#);
}

#[test]
#[ignore]
fn _0013() {
  // TODO add checking if there are any other named parameters than allowed and report it as an error
  te_null(false, &scope!(), r#"sublist(list:[1,2,3], start position: 1, l: 1)"#, r#"parameter 'length' not found"#);
}

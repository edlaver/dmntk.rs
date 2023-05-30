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
use dmntk_feel::scope;

#[test]
fn test_0001() {
  te_number(false, &scope!(), "2**0", 1, 0);
}

#[test]
fn test_0002() {
  te_number(false, &scope!(), "1**1", 1, 0);
}

#[test]
fn test_0003() {
  te_number(false, &scope!(), " 1 ** 2 ", 1, 0);
}

#[test]
fn test_0004() {
  te_number(false, &scope!(), "3 ** 2", 9, 0);
}

#[test]
fn test_0005() {
  te_number(false, &scope!(), "-3 ** 2", 9, 0);
}

#[test]
fn test_0006() {
  te_number(false, &scope!(), " 5 **2** 3 ", 15625, 0);
}

#[test]
fn test_0007() {
  te_number(false, &scope!(), "10**2**-2", 1, 4);
}

#[test]
fn test_0008() {
  te_number(false, &scope!(), "( 1 ** 2 ) ** ( 3 ** 4 )", 1, 0);
}

#[test]
fn test_0009() {
  te_number(false, &scope!(), "( ( ( 4 ** 3 ) ) )", 64, 0);
}

#[test]
fn test_0010() {
  te_number(false, &scope!(), "2**(2+3)", 32, 0);
}

#[test]
fn test_0011() {
  te_number(false, &scope!(), "2**2+3", 7, 0);
}

#[test]
fn test_0012() {
  te_number(false, &scope!(), "1 + 3/2*2 - 2**3", -4, 0);
}

#[test]
fn test_0013() {
  te_number(false, &scope!(), "1+3/2*2-2**3", -4, 0);
}

#[test]
fn test_0014() {
  te_number(false, &scope!(), "3 ** 4 ** 5", 3486784401, 0);
}

#[test]
fn test_0015() {
  te_number_x(false, &scope!(), "3 ** (4 ** 5)", "3.733918487410200435329597541848665E+488");
}

#[test]
fn test_0016() {
  te_null(false, &scope!(), r#""foo" ** 4"#, r#"exponentiation base is not a number"#);
}

#[test]
fn test_0017() {
  te_null(false, &scope!(), "true ** 4", r#"exponentiation base is not a number"#);
}

#[test]
fn test_0018() {
  te_null(false, &scope!(), r#"date("2018-12-10") ** 4"#, r#"exponentiation base is not a number"#);
}

#[test]
fn test_0019() {
  te_null(false, &scope!(), r#"time("10:30:00") ** 4"#, r#"exponentiation base is not a number"#);
}

#[test]
fn test_0020() {
  te_null(false, &scope!(), r#"date and time("2018-12-10") ** 4"#, r#"exponentiation base is not a number"#);
}

#[test]
fn test_0021() {
  te_null(false, &scope!(), r#"duration("P2Y") ** 4"#, r#"exponentiation base is not a number"#);
}

#[test]
fn test_0022() {
  te_null(false, &scope!(), r#"4 ** "foo""#, r#"exponentiation exponent is not a number"#);
}

#[test]
fn test_0023() {
  te_null(
    false,
    &scope!(),
    r#"4 ** 10000000000000000000000000000000000000000"#,
    r#"exponentiation result is not a finite number"#,
  );
}

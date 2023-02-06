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
use dmntk_feel::scope;

#[test]
fn _0001() {
  te_bool(false, &scope!(), r#"2 in [1..5]"#, true);
}

#[test]
fn _0002() {
  te_null(false, &scope!(), r#"2 in ["a".."z"]"#, "eval_in_range");
}

#[test]
fn _0003() {
  te_null(false, &scope!(), r#"2 in [1.."z"]"#, "eval_in_range");
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"0 in [1..5]"#, false);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"6 in [1..5]"#, false);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"1 in [1..5]"#, true);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"5 in [1..5]"#, true);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#"1 in (1..5]"#, false);
}

#[test]
fn _0009() {
  te_bool(false, &scope!(), r#"5 in [1..5)"#, false);
}

#[test]
fn _0010() {
  te_bool(false, &scope!(), r#"1 in (1..5)"#, false);
}

#[test]
fn _0011() {
  te_bool(false, &scope!(), r#"1.01 in (1..5)"#, true);
}

#[test]
fn _0012() {
  te_bool(false, &scope!(), r#"5 in (1..5)"#, false);
}

#[test]
fn _0013() {
  te_bool(false, &scope!(), r#"4.99 in (1..5)"#, true);
}

#[test]
fn _0014() {
  te_bool(false, &scope!(), r#"1 in ]1..5]"#, false);
}

#[test]
fn _0015() {
  te_bool(false, &scope!(), r#"5 in [1..5["#, false);
}

#[test]
fn _0016() {
  te_bool(false, &scope!(), r#"1 in ]1..5["#, false);
}

#[test]
fn _0017() {
  te_bool(false, &scope!(), r#"5 in ]1..5["#, false);
}

#[test]
fn _0018() {
  te_bool(false, &scope!(), r#"1 in [1..5] and 5 in [1..5]"#, true);
}

#[test]
fn _0019() {
  te_bool(false, &scope!(), r#"1 in [1..5] or 6 in [1..5]"#, true);
}

#[test]
fn _0020() {
  te_bool(false, &scope!(), r#" @"P3D" in [@"P1D"..@"P5D"]"#, true);
}

#[test]
fn _0021() {
  te_bool(false, &scope!(), r#" @"P5D" in [@"P1D"..@"P5D")"#, false);
}

#[test]
fn _0022() {
  te_null(false, &scope!(), r#" "k" in [1.."z"]"#, "eval_in_range");
}

#[test]
fn _0023() {
  te_null(false, &scope!(), r#" "k" in ["a"..34]"#, "eval_in_range");
}

#[test]
fn _0024() {
  te_null(false, &scope!(), r#" @"2023-02-06" in ["2023-02-01"..@"2023-02-28"]"#, "eval_in_range");
}

#[test]
fn _0025() {
  te_null(false, &scope!(), r#" @"2023-02-06" in [@"2023-02-01".."2023-02-28"]"#, "eval_in_range");
}

#[test]
fn _0026() {
  te_null(
    false,
    &scope!(),
    r#" @"2023-02-06T10:11:12" in ["2023-02-01T00:00:00"..@"2023-02-28T23:59:59"]"#,
    "eval_in_range",
  );
}

#[test]
fn _0027() {
  te_null(
    false,
    &scope!(),
    r#" @"2023-02-06T10:11:12" in [@"2023-02-01T00:00:00".."2023-02-28T23:59:59"]"#,
    "eval_in_range",
  );
}

#[test]
fn _0028() {
  te_null(false, &scope!(), r#" @"10:11:12" in ["00:00:00"..@"23:59:59"]"#, "eval_in_range");
}

#[test]
fn _0029() {
  te_null(false, &scope!(), r#" @"10:11:12" in [@"00:00:00".."23:59:59"]"#, "eval_in_range");
}

#[test]
fn _0030() {
  te_null(false, &scope!(), r#" @"P5D" in [1..@"P5D"]"#, "eval_in_range");
}

#[test]
fn _0031() {
  te_null(false, &scope!(), r#" @"P5D" in [@"P1D"..5]"#, "eval_in_range");
}

#[test]
fn _0032() {
  te_null(false, &scope!(), r#" @"P5Y" in [1..@"P5Y"]"#, "eval_in_range");
}

#[test]
fn _0033() {
  te_null(false, &scope!(), r#" @"P5Y" in [@"P1Y"..5]"#, "eval_in_range");
}

#[test]
fn _0034() {
  te_null(false, &scope!(), r#" true in [false..true]"#, "eval_in_range");
}

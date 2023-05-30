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
fn _0001() {
  te_bool(false, &scope!(), r#"2 between 1 and 4"#, true);
}

#[test]
fn _0002() {
  te_bool(false, &scope!(), r#"1 between 1 and 4"#, true);
}

#[test]
fn _0003() {
  te_bool(false, &scope!(), r#"4 between 1 and 4"#, true);
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"0.99 between 1 and 4"#, false);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"4.01 between 1 and 4"#, false);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"6 between 1 and 4 + 2"#, true);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"2 between 1 + 1 and 4 + 2"#, true);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#"5 - 2 between 1 + 2 and 10.2/2"#, true);
}

#[test]
fn _0009() {
  te_bool(false, &scope!(), r#"5 - 2 - 0.1 between 1 + 2 and 10.2/2"#, false);
}

#[test]
fn _0010() {
  te_bool(false, &scope!(), r#"true = (2 between 1 and 4)"#, true);
}

#[test]
fn _0011() {
  te_bool(false, &scope!(), r#"(2 between 1 and 4) = true"#, true);
}

#[test]
fn _0012() {
  te_bool(false, &scope!(), r#"(2 between 1 and 4) = (5 between 3 and 8)"#, true);
}

#[test]
fn _0013() {
  te_null(
    false,
    &scope!(),
    r#"2 between "1" and 4"#,
    "expected number as a second argument in 'between' operator, actual value type is string",
  );
}

#[test]
fn _0014() {
  te_null(
    false,
    &scope!(),
    r#"2 between 1 and "4""#,
    "expected number as a third argument in 'between' operator, actual value type is string",
  );
}

#[test]
fn _0015() {
  te_bool(false, &scope!(), r#" "k" between "a" and "z" "#, true);
}

#[test]
fn _0016() {
  te_bool(false, &scope!(), r#" "k" between "m" and "z" "#, false);
}

#[test]
fn _0017() {
  te_null(
    false,
    &scope!(),
    r#" "k" between 1 and "z" "#,
    "expected string as a second argument in 'between' operator, actual value type is number",
  );
}

#[test]
fn _0018() {
  te_null(
    false,
    &scope!(),
    r#" "k" between "a" and 15 "#,
    "expected string as a third argument in 'between' operator, actual value type is number",
  );
}

#[test]
fn _0019() {
  te_bool(false, &scope!(), r#" @"2023-02-04" between @"2023-02-01" and @"2023-02-11" "#, true);
}

#[test]
fn _0020() {
  te_bool(false, &scope!(), r#" @"2023-02-14" between @"2023-02-01" and @"2023-02-11" "#, false);
}

#[test]
fn _0021() {
  te_null(
    false,
    &scope!(),
    r#" @"2023-02-04" between "2023-02-01" and @"2023-02-11" "#,
    "expected date as a second argument in 'between' operator, actual value type is string",
  );
}

#[test]
fn _0022() {
  te_null(
    false,
    &scope!(),
    r#" @"2023-02-04" between @"2023-02-01" and "2023-02-11" "#,
    "expected date as a third argument in 'between' operator, actual value type is string",
  );
}

#[test]
fn _0023() {
  te_bool(false, &scope!(), r#" @"10:11:12" between @"10:11:10" and @"10:11:15" "#, true);
}

#[test]
fn _0024() {
  te_bool(false, &scope!(), r#" @"10:11:32" between @"10:11:10" and @"10:11:15" "#, false);
}

#[test]
fn _0025() {
  te_null(
    false,
    &scope!(),
    r#" @"10:11:12" between "10:11:10" and @"10:11:15" "#,
    "expected time as a second argument in 'between' operator, actual value type is string",
  );
}

#[test]
fn _0026() {
  te_null(
    false,
    &scope!(),
    r#" @"10:11:12" between @"10:11:10" and "10:11:15" "#,
    "expected time as a third argument in 'between' operator, actual value type is string",
  );
}

#[test]
fn _0027() {
  te_bool(
    false,
    &scope!(),
    r#" @"2023-02-04T10:11:12" between @"2023-02-04T10:11:10" and @"2023-02-14T10:11:15" "#,
    true,
  );
}

#[test]
fn _0028() {
  te_bool(
    false,
    &scope!(),
    r#" @"2023-02-04T10:11:32" between @"2023-02-04T10:11:10" and @"2023-02-04T10:11:15" "#,
    false,
  );
}

#[test]
fn _0029() {
  te_null(
    false,
    &scope!(),
    r#" @"2023-02-04T10:11:12" between "2023-02-04T10:11:10" and @"2023-02-14T10:11:15" "#,
    "expected date and time as a second argument in 'between' operator, actual value type is string",
  );
}

#[test]
fn _0030() {
  te_null(
    false,
    &scope!(),
    r#" @"2023-02-04T10:11:12" between @"2023-02-04T10:11:10" and "2023-02-14T10:11:15" "#,
    "expected date and time as a third argument in 'between' operator, actual value type is string",
  );
}

#[test]
fn _0031() {
  te_bool(false, &scope!(), r#" @"P5D" between @"P1D" and @"P10D" "#, true);
}

#[test]
fn _0032() {
  te_bool(false, &scope!(), r#" @"P1D" between @"P2D" and @"P10D" "#, false);
}

#[test]
fn _0033() {
  te_null(
    false,
    &scope!(),
    r#" @"P5D" between "P1D" and @"P10D" "#,
    "expected days and time duration as a second argument in 'between' operator, actual value type is string",
  );
}

#[test]
fn _0034() {
  te_null(
    false,
    &scope!(),
    r#" @"P5D" between @"P1D" and "P10D" "#,
    "expected days and time duration as a third argument in 'between' operator, actual value type is string",
  );
}

#[test]
fn _0035() {
  te_bool(false, &scope!(), r#" @"P5Y" between @"P1Y" and @"P10Y" "#, true);
}

#[test]
fn _0036() {
  te_bool(false, &scope!(), r#" @"P1Y" between @"P2Y" and @"P10Y" "#, false);
}

#[test]
fn _0037() {
  te_null(
    false,
    &scope!(),
    r#" @"P5Y" between "P1Y" and @"P10Y" "#,
    "expected years and months duration as a second argument in 'between' operator, actual value type is string",
  );
}

#[test]
fn _0038() {
  te_null(
    false,
    &scope!(),
    r#" @"P5Y" between @"P1Y" and "P10Y" "#,
    "expected years and months duration as a third argument in 'between' operator, actual value type is string",
  );
}

#[test]
fn _0039() {
  te_null(
    false,
    &scope!(),
    r#" (function() 1) between 1 and 10 "#,
    "unexpected value type in 'between' operator: function<>->Any",
  );
}

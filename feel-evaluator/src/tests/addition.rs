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
  te_number(false, &scope!(), "1+1", 2, 0);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), " 1 + 2 ", 3, 0);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), " 5 +2 +1 ", 8, 0);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), "20+200+2", 222, 0);
}

#[test]
fn _0005() {
  te_number(false, &scope!(), "( 1 + 2 ) + ( 3 + 4 )", 10, 0);
}

#[test]
fn _0006() {
  te_number(false, &scope!(), "( ( ( 1 + 2 ) ) )", 3, 0);
}

#[test]
fn _0007() {
  te_number(false, &scope!(), "(1+2)*(3+2)", 15, 0);
}

#[test]
fn _0008() {
  te_number(false, &scope!(), "1+2*3+2", 9, 0);
}

#[test]
fn _0009() {
  te_number(false, &scope!(), ".25 + .2", 45, 2);
}

#[test]
fn _0010() {
  let scope = &te_scope(r#"{Full Name:"John Doe"}"#);
  te_string(false, scope, r#""Hello " + Full Name"#, "Hello John Doe");
}

#[test]
fn _0011() {
  let scope = &te_scope(r#"{Employment Status:"EMPLOYED"}"#);
  te_string(false, scope, r#""You are " + Employment Status"#, "You are EMPLOYED");
}

#[test]
fn _0012() {
  let scope = &te_scope(r#"{Full Name:"John Doe", Employment Status:"EMPLOYED"}"#);
  te_string(
    false,
    scope,
    r#""Hello " + Full Name + ", you are " + Employment Status"#,
    "Hello John Doe, you are EMPLOYED",
  );
}

#[test]
fn _0013() {
  te_null(false, &scope!(), r#"1.25 + "alfa""#, r#"incompatible types in addition: 1.25(number) + "alfa"(string)"#);
}

#[test]
fn _0014() {
  te_date_time_local(false, &scope!(), r#" @"2021-01-12T10:10:10" + @"P1DT1H" "#, (2021, 1, 13), (11, 10, 10, 0));
}

#[test]
fn _0015() {
  te_date_time_local(false, &scope!(), r#" @"P1DT1H" + @"2021-01-12T10:10:10" "#, (2021, 1, 13), (11, 10, 10, 0));
}

#[test]
fn _0016() {
  te_date_time_local(false, &scope!(), r#" @"2021-01-12T10:10:10" + @"P1Y1M" "#, (2022, 2, 12), (10, 10, 10, 0));
}

#[test]
fn _0017() {
  te_date_time_local(false, &scope!(), r#" @"P1Y1M" + @"2021-01-12T10:10:10" "#, (2022, 2, 12), (10, 10, 10, 0));
}

#[test]
fn _0018() {
  te_null(
    false,
    &scope!(),
    r#"@"2021-01-12T10:10:10" + 1"#,
    "[builders::add] invalid argument type, expected days and time duration, years and months duration, actual type is number",
  );
}

#[test]
fn _0019() {
  te_years_and_months_duration_x(false, &scope!(), r#" @"P1Y1M" + @"P2Y3M" "#, r#"P3Y4M"#);
}

#[test]
fn _0020() {
  te_days_and_time_duration_x(false, &scope!(), r#" @"P1DT2H" + @"P2DT3H" "#, r#"P3DT5H"#);
}

#[test]
fn _0021() {
  te_null(false, &scope!(), r#" null + @"P1DT2H" "#, r#""#);
}

#[test]
fn _0022() {
  te_null(
    false,
    &scope!(),
    r#" @"P1D" + 1 "#,
    r#"[builders::add] invalid argument type, expected days and time duration, date and time, actual type is number"#,
  );
}

#[test]
fn _0023() {
  te_null(
    false,
    &scope!(),
    r#" @"P1Y1M" + 1 "#,
    r#"[builders::add] invalid argument type, expected years and months duration, date and time, actual type is number"#,
  );
}

#[test]
fn _0024() {
  te_null(
    false,
    &scope!(),
    r#" true + @"P1DT2H" "#,
    r#"[builders::add] invalid argument type, expected number, string, date and time, days and time duration, years and months duration, null, actual type is boolean"#,
  );
}

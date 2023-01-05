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

use super::super::*;
use dmntk_feel::scope;

#[test]
fn _0001() {
  te_string(false, &scope!(), r#"month of year(date(2019,1,17))"#, "January");
}

#[test]
fn _0002() {
  te_string(false, &scope!(), r#"month of year(date:date(2019,1,17))"#, "January");
}

#[test]
fn _0003() {
  te_string(false, &scope!(), r#"month of year(date(2019,2,17))"#, "February");
}

#[test]
fn _0004() {
  te_string(false, &scope!(), r#"month of year(date:date(2019,2,17))"#, "February");
}

#[test]
fn _0005() {
  te_string(false, &scope!(), r#"month of year(date(2019,3,17))"#, "March");
}

#[test]
fn _0006() {
  te_string(false, &scope!(), r#"month of year(date:date(2019,3,17))"#, "March");
}

#[test]
fn _0007() {
  te_string(false, &scope!(), r#"month of year(date(2019,4,17))"#, "April");
}

#[test]
fn _0008() {
  te_string(false, &scope!(), r#"month of year(date:date(2019,4,17))"#, "April");
}

#[test]
fn _0009() {
  te_string(false, &scope!(), r#"month of year(date(2019,5,17))"#, "May");
}

#[test]
fn _0010() {
  te_string(false, &scope!(), r#"month of year(date:date(2019,5,17))"#, "May");
}

#[test]
fn _0011() {
  te_string(false, &scope!(), r#"month of year(date(2019,6,17))"#, "June");
}

#[test]
fn _0012() {
  te_string(false, &scope!(), r#"month of year(date:date(2019,6,17))"#, "June");
}

#[test]
fn _0013() {
  te_string(false, &scope!(), r#"month of year(date(2019,7,17))"#, "July");
}

#[test]
fn _0014() {
  te_string(false, &scope!(), r#"month of year(date:date(2019,7,17))"#, "July");
}

#[test]
fn _0015() {
  te_string(false, &scope!(), r#"month of year(date(2019,8,17))"#, "August");
}

#[test]
fn _0016() {
  te_string(false, &scope!(), r#"month of year(date:date(2019,8,17))"#, "August");
}

#[test]
fn _0017() {
  te_string(false, &scope!(), r#"month of year(date(2019,9,17))"#, "September");
}

#[test]
fn _0018() {
  te_string(false, &scope!(), r#"month of year(date:date(2019,9,17))"#, "September");
}

#[test]
fn _0019() {
  te_string(false, &scope!(), r#"month of year(date(2019,10,17))"#, "October");
}

#[test]
fn _0020() {
  te_string(false, &scope!(), r#"month of year(date:date(2019,10,17))"#, "October");
}

#[test]
fn _0021() {
  te_string(false, &scope!(), r#"month of year(date(2019,11,17))"#, "November");
}

#[test]
fn _0022() {
  te_string(false, &scope!(), r#"month of year(date:date(2019,11,17))"#, "November");
}

#[test]
fn _0023() {
  te_string(false, &scope!(), r#"month of year(date(2019,12,17))"#, "December");
}

#[test]
fn _0024() {
  te_string(false, &scope!(), r#"month of year(date:date(2019,12,17))"#, "December");
}

#[test]
fn _0025() {
  te_string(false, &scope!(), r#"month of year(date and time("2021-01-01T12:34:23"))"#, "January");
}

#[test]
fn _0026() {
  te_string(false, &scope!(), r#"month of year(date:date and time("2021-01-01T12:34:23"))"#, "January");
}

#[test]
fn _0027() {
  te_null(false, &scope!(), r#"month of year()"#, "expected 1 parameters, actual number of parameters is 0");
}

#[test]
fn _0028() {
  te_null(
    false,
    &scope!(),
    r#"month of year(date(2019,12,17),date(2019,12,17))"#,
    "expected 1 parameters, actual number of parameters is 2",
  );
}

#[test]
fn _0029() {
  te_null(false, &scope!(), r#"month of year(d:date(2019,12,17))"#, "parameter 'date' not found");
}

#[test]
fn _0030() {
  te_null(
    false,
    &scope!(),
    r#"month of year(10)"#,
    "[core::month of year] invalid argument type, expected date, date and time, actual type is number",
  );
}

#[test]
fn _0031() {
  te_null(
    false,
    &scope!(),
    r#"month of year(null)"#,
    "[core::month of year] invalid argument type, expected date, date and time, actual type is Null",
  );
}

#[test]
fn _0032() {
  te_null(
    false,
    &scope!(),
    r#"month of year(true)"#,
    "[core::month of year] invalid argument type, expected date, date and time, actual type is boolean",
  );
}

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
use dmntk_feel::scope;

#[test]
fn _0001() {
  let scope = &scope!();
  te_date(false, scope, r#"date("2012-12-25")"#, 2012, 12, 25);
}

#[test]
fn _0002() {
  let scope = &scope!();
  te_date(false, scope, r#"date(from: "2012-12-25")"#, 2012, 12, 25);
}

#[test]
fn _0003() {
  let scope = &scope!();
  te_date(false, scope, r#"date("2012-12-25")"#, 2012, 12, 25);
}

#[test]
fn _0004() {
  let scope = &scope!();
  te_date(false, scope, r#"date(2012,12,25)"#, 2012, 12, 25);
}

#[test]
fn _0005() {
  let scope = &scope!();
  te_date(false, scope, r#"date(year: 2012, month: 12, day: 25)"#, 2012, 12, 25);
}

#[test]
fn _0006() {
  let scope = &scope!();
  te_date(false, scope, r#"date(month: 12, day: 25, year: 2012)"#, 2012, 12, 25);
}

#[test]
fn _0007() {
  let scope = &scope!();
  te_date(false, scope, r#"date("262143-12-31")"#, 262143, 12, 31);
}

#[test]
fn _0008() {
  let scope = &scope!();
  te_date(false, scope, r#"date("999999999-12-31")"#, 999_999_999, 12, 31);
}

#[test]
fn _0009() {
  let scope = &scope!();
  te_date(false, scope, r#"date(999999999,12,31)"#, 999_999_999, 12, 31);
}

#[test]
fn _0010() {
  let scope = &scope!();
  te_date(false, scope, r#"date("-262144-01-01")"#, -262144, 1, 1);
}

#[test]
fn _0011() {
  let scope = &scope!();
  te_date(false, scope, r#"date("-999999999-01-01")"#, -999_999_999, 1, 1);
}

#[test]
fn _0012() {
  let scope = &scope!();
  te_date(false, scope, r#"date(-999999999,01,01)"#, -999_999_999, 1, 1);
}

#[test]
fn _0013() {
  let scope = &scope!();
  te_date(false, scope, r#"date(date and time("2012-12-25T12:23:18"))"#, 2012, 12, 25);
}

#[test]
fn _0014() {
  let scope = &scope!();
  te_date(false, scope, r#"date(from: date and time("2012-12-25T12:23:18"))"#, 2012, 12, 25);
}

#[test]
fn _0015() {
  let scope = &scope!();
  te_date(false, scope, r#"date(date and time("2012-12-25T12:23:18Z"))"#, 2012, 12, 25);
}

#[test]
fn _0016() {
  let scope = &scope!();
  te_date(false, scope, r#"date(date and time("2012-12-25T12:23:18z"))"#, 2012, 12, 25);
}

#[test]
fn _0017() {
  let scope = &scope!();
  te_bool(false, scope, r#"date("2012-12-25") in [date("2012-12-24")..date("2012-12-26")]"#, true);
}

#[test]
fn _0018() {
  let scope = &scope!();
  te_bool(false, scope, r#"date("2000-12-25") in [date("2012-12-24")..date("2012-12-26")]"#, false);
}

#[test]
fn _0019() {
  let scope = &scope!();
  te_bool(false, scope, r#"date("2020-12-25") in [date("2012-12-24")..date("2012-12-26")]"#, false);
}

#[test]
fn _0020() {
  let scope = &scope!();
  te_bool(false, scope, r#"date("2012-12-31") in (date("2012-12-25")..date("2013-02-14"))"#, true);
}

#[test]
fn _0021() {
  let scope = &scope!();
  te_null(false, scope, r#"date("2017-13-10")"#, r#"[core::date] invalid date string '2017-13-10'"#);
}

#[test]
fn _0022() {
  let scope = &scope!();
  te_null(false, scope, r#"date("2017/12/10")"#, r#"[core::date] invalid date string '2017/12/10'"#);
}

#[test]
fn _0023() {
  let scope = &scope!();
  te_null(false, scope, r#"date("2017,12,31")"#, r#"[core::date] invalid date string '2017,12,31'"#);
}

#[test]
fn _0024() {
  let scope = &scope!();
  te_date(false, scope, r#"date("2012-12-25")"#, 2012, 12, 25);
  te_number(false, scope, r#"date("2012-12-25").day"#, 25, 0);
  te_number(false, scope, r#"date("2012-12-25").month"#, 12, 0);
  te_number(false, scope, r#"date("2012-12-25").year"#, 2012, 0);
}

#[test]
fn _0025() {
  let scope = &te_scope(r#"{fromString: "2012-12-25"}"#);
  te_date(false, scope, r#"date(fromString)"#, 2012, 12, 25);
  te_number(false, scope, r#"date(fromString).day"#, 25, 0);
  te_number(false, scope, r#"date(fromString).month"#, 12, 0);
  te_number(false, scope, r#"date(fromString).year"#, 2012, 0);
}

#[test]
fn _0026() {
  te_null(
    false,
    &scope!(),
    r#"date(10)"#,
    r#"[core::date] invalid argument type, expected string, date or date and time, actual type is number"#,
  );
}

#[test]
fn _0027() {
  te_date(false, &scope!(), r#"date(date("2021-01-06"))"#, 2021, 1, 6);
}

#[test]
fn _0028() {
  let scope = &scope!();
  te_null(false, scope, r#"date(2020,2,30)"#, r#"[core::date_3] invalid date y=2020 m=2 d=30"#);
}

#[test]
fn _0029() {
  let scope = &scope!();
  te_null(
    false,
    scope,
    r#"date(384756328456345,790485703985734,45490654698475)"#,
    r#"[core::date_3] invalid date y=384756328456345 m=790485703985734 d=45490654698475"#,
  );
}

#[test]
fn _0030() {
  let scope = &scope!();
  te_null(
    false,
    scope,
    r#"date(true,2,30)"#,
    r#"[core::date] invalid argument type, expected number (year), actual type is boolean"#,
  );
}

#[test]
fn _0031() {
  let scope = &scope!();
  te_null(
    false,
    scope,
    r#"date(2020,"02",30)"#,
    r#"[core::date] invalid argument type, expected number (month), actual type is string"#,
  );
}

#[test]
fn _0032() {
  let scope = &scope!();
  te_null(
    false,
    scope,
    r#"date(2020,2,null)"#,
    r#"[core::date] invalid argument type, expected number (day), actual type is Null"#,
  );
}

#[test]
fn _0033() {
  let scope = &scope!();
  te_null(false, scope, r#"date()"#, r#"expected 1,3 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0034() {
  let scope = &scope!();
  te_null(false, scope, r#"date(2022,01)"#, r#"expected 1,3 parameters, actual number of parameters is 2"#);
}

#[test]
fn _0035() {
  let scope = &scope!();
  te_null(false, scope, r#"date(2022,1,1,1)"#, r#"expected 1,3 parameters, actual number of parameters is 4"#);
}

#[test]
fn _0036() {
  let scope = &scope!();
  te_null(false, scope, r#"date(f: "2021-01-26")"#, r#"invalid parameters in named::bif_date"#);
}

#[test]
fn _0037() {
  let scope = &scope!();
  te_null(false, scope, r#"date(f: date and time("2012-12-25T12:23:18"))"#, r#"invalid parameters in named::bif_date"#);
}

#[test]
fn _0038() {
  let scope = &scope!();
  te_null(false, scope, r#"date(m: 12, day: 25, year: 2012)"#, r#"invalid parameters in named::bif_date"#);
}

#[test]
fn _0039() {
  let scope = &scope!();
  te_null(false, scope, r#"date(month: 12, d: 25, year: 2012)"#, r#"invalid parameters in named::bif_date"#);
}

#[test]
fn _0040() {
  let scope = &scope!();
  te_null(false, scope, r#"date(month: 12, day: 25, y: 2012)"#, r#"invalid parameters in named::bif_date"#);
}

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
  te_bool(false, &scope!(), r#"before(1,10)"#, true);
}

#[test]
fn _0002() {
  te_bool(false, &scope!(), r#"before(date("2022-01-31"),date("2022-02-01"))"#, true);
}

#[test]
fn _0003() {
  te_bool(false, &scope!(), r#"before(10,1)"#, false);
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"before(date("2022-02-01"),date("2022-01-31"))"#, false);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"before(10,10)"#, false);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"before(date("1981-12-13"),date("1981-12-13"))"#, false);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"before(1,[1..10])"#, false);
}

#[test]
fn _0008() {
  te_bool(
    false,
    &scope!(),
    r#"before(date("1960-11-15"),[date("1960-11-15")..date("1960-12-31")])"#,
    false,
  );
}

#[test]
fn _0009() {
  te_bool(false, &scope!(), r#"before(1,(1..10])"#, true);
}

#[test]
fn _0010() {
  te_bool(false, &scope!(), r#"before(date("1960-11-15"),(date("1960-11-15")..date("1960-12-31")])"#, true);
}

#[test]
fn _0011() {
  te_bool(false, &scope!(), "before(1,[5..10])", true);
}

#[test]
fn _0012() {
  te_bool(false, &scope!(), r#"before(date("1960-11-15"),[date("1960-11-17")..date("1960-12-31")])"#, true);
}

#[test]
fn _0013() {
  te_bool(false, &scope!(), r#"before([1..10],10)"#, false);
}

#[test]
fn _0014() {
  te_bool(
    false,
    &scope!(),
    r#"before([date("1960-11-17")..date("1960-12-31")],date("1960-12-31"))"#,
    false,
  );
}

#[test]
fn _0015() {
  te_bool(false, &scope!(), r#"before([1..10),10)"#, true);
}

#[test]
fn _0016() {
  te_bool(false, &scope!(), r#"before([date("1960-11-17")..date("1960-12-31")),date("1960-12-31"))"#, true);
}

#[test]
fn _0017() {
  te_bool(false, &scope!(), r#"before([1..10],15)"#, true);
}

#[test]
fn _0018() {
  te_bool(false, &scope!(), r#"before([date("1960-11-17")..date("1960-12-31")],date("1961-02-12"))"#, true);
}

#[test]
fn _0019() {
  te_bool(false, &scope!(), r#"before([1..10],[15..20])"#, true);
}

#[test]
fn _0020() {
  te_bool(
    false,
    &scope!(),
    r#"before([date("1960-11-17")..date("1960-12-31")],[date("1961-01-01")..date("1960-05-31")])"#,
    true,
  );
}

#[test]
fn _0021() {
  te_bool(false, &scope!(), r#"before([1..10],[10..20])"#, false);
}

#[test]
fn _0022() {
  te_bool(
    false,
    &scope!(),
    r#"before([date("1960-11-17")..date("1960-12-31")],[date("1960-12-31")..date("1960-05-31")])"#,
    false,
  );
}

#[test]
fn _0023() {
  te_bool(false, &scope!(), r#"before([1..10),[10..20])"#, true);
}

#[test]
fn _0024() {
  te_bool(
    false,
    &scope!(),
    r#"before([date("1960-11-17")..date("1960-12-31")),[date("1960-12-31")..date("1960-05-31")])"#,
    true,
  );
}

#[test]
fn _0025() {
  te_bool(false, &scope!(), r#"before([1..10],(10..20])"#, true);
}

#[test]
fn _0026() {
  te_bool(
    false,
    &scope!(),
    r#"before([date("1960-11-17")..date("1960-12-31")],(date("1960-12-31")..date("1960-05-31")])"#,
    true,
  );
}

#[test]
fn _0027() {
  te_bool(false, &scope!(), r#"before([1..10),(10..20])"#, true);
}

#[test]
fn _0028() {
  te_bool(
    false,
    &scope!(),
    r#"before([date("1960-11-17")..date("1960-12-31")),(date("1960-12-31")..date("1960-05-31")])"#,
    true,
  );
}

#[test]
fn _0029() {
  te_bool(false, &scope!(), r#"before(point1:1,point2:10)"#, true);
}

#[test]
fn _0030() {
  te_bool(false, &scope!(), r#"before(point1:date("2022-01-31"),point2:date("2022-02-01"))"#, true);
}

#[test]
fn _0031() {
  te_bool(false, &scope!(), r#"before(point2:10,point1:1)"#, true);
}

#[test]
fn _0032() {
  te_bool(false, &scope!(), r#"before(point2:date("2022-02-01"),point1:date("2022-01-31"))"#, true);
}

#[test]
fn _0033() {
  te_bool(false, &scope!(), r#"before(point1:10,point2:1)"#, false);
}

#[test]
fn _0034() {
  te_bool(false, &scope!(), r#"before(point1:date("2022-02-01"),point2:date("2022-01-31"))"#, false);
}

#[test]
fn _0035() {
  te_bool(false, &scope!(), r#"before(point1:10,point2:10)"#, false);
}

#[test]
fn _0036() {
  te_bool(false, &scope!(), r#"before(point1:date("2022-02-01"),point2:date("2022-02-01"))"#, false);
}

#[test]
fn _0037() {
  te_bool(false, &scope!(), r#"before(range1:[1..10),range2:(10..20])"#, true);
}

#[test]
fn _0038() {
  te_bool(false, &scope!(), r#"before(point:1,range:(1..10])"#, true);
}

#[test]
fn _0039() {
  te_bool(false, &scope!(), r#"before(range:(1..10],point:1)"#, true);
}

#[test]
fn _0040() {
  te_null(false, &scope!(), r#"before()"#, r#"expected 2 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0041() {
  te_null(false, &scope!(), r#"before(1)"#, r#"expected 2 parameters, actual number of parameters is 1"#);
}

#[test]
fn _0042() {
  te_null(
    false,
    &scope!(),
    r#"before(1,2,3)"#,
    r#"expected 2 parameters, actual number of parameters is 3"#,
  );
}

#[test]
fn _0043() {
  te_null(false, &scope!(), r#"before(p1: 1, point2: 2)"#, r#"[named::before] invalid named parameters"#);
}

#[test]
fn _0044() {
  te_null(
    false,
    &scope!(),
    r#"before("abc","bcd")"#,
    r#"[core::before] invalid argument type, expected scalar or range of scalars, actual type is string"#,
  );
}

#[test]
fn _0045() {
  te_bool(false, &scope!(), r#"before(date("2022-01-31"),[date("2022-02-01")..date("2022-12-31")])"#, true);
}

#[test]
fn _0046() {
  te_bool(
    false,
    &scope!(),
    r#"before([date("2021-01-01")..date("2021-12-31")],[date("2022-01-01")..date("2022-12-31")])"#,
    true,
  );
}

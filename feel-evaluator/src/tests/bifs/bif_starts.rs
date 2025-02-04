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
  te_bool(false, &scope!(), r#"starts(1,[1..20])"#, true);
}

#[test]
fn _0002() {
  te_null(
    false,
    &scope!(),
    r#"starts(duration("P1D"),[1..20])"#,
    "[core::starts] invalid argument type, expected scalar or range of scalars, actual type is days and time duration",
  );
}

#[test]
fn _0003() {
  te_bool(false, &scope!(), r#"starts(2,[1..20])"#, false);
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"starts(1,(1..20])"#, false);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"starts(point: 1,range: [1..20])"#, true);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"starts(range: [1..20], point: 1)"#, true);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"starts(range1: [1..20],range2: [5..20])"#, false);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#"starts(date("2022-01-01"),[date("2022-01-01")..date("2022-06-01")])"#, true);
}

#[test]
fn _0009() {
  te_bool(
    false,
    &scope!(),
    r#"starts([date("2022-01-01")..date("2022-01-03")],[date("2022-01-01")..date("2022-06-01")])"#,
    true,
  );
}

#[test]
fn _0010() {
  te_bool(
    false,
    &scope!(),
    r#"starts(date and time("2022-01-01T00:00:00"),[date and time("2022-01-01T00:00:00")..date and time("2022-06-01T00:00:00")])"#,
    true,
  );
}

#[test]
fn _0011() {
  te_bool(
    false,
    &scope!(),
    r#"starts([date and time("2022-01-01T00:00:00")..date and time("2022-01-03T00:00:00")],[date and time("2022-01-01T00:00:00")..date and time("2022-06-01T00:00:00")])"#,
    true,
  );
}

#[test]
fn _0012() {
  te_bool(false, &scope!(), r#"starts(time("00:00:00"),[time("00:00:00")..time("02:23:57")])"#, true);
}

#[test]
fn _0013() {
  te_bool(
    false,
    &scope!(),
    r#"starts([time("00:00:00")..time("00:15:10")],[time("00:00:00")..time("19:38:23")])"#,
    true,
  );
}

#[test]
fn _0014() {
  te_bool(false, &scope!(), r#"starts(duration("P1D"),[duration("P1D")..duration("P10D")])"#, true);
}

#[test]
fn _0015() {
  te_bool(false, &scope!(), r#"starts([duration("P1D")..duration("P2D")],[duration("P1D")..duration("P10D")])"#, true);
}

#[test]
fn _0016() {
  te_bool(false, &scope!(), r#"starts(duration("P1Y"),[duration("P1Y")..duration("P10Y")])"#, true);
}

#[test]
fn _0017() {
  te_bool(false, &scope!(), r#"starts([duration("P1Y")..duration("P2Y")],[duration("P1Y")..duration("P10Y")])"#, true);
}

#[test]
fn _0018() {
  te_null(false, &scope!(), r#"starts()"#, "expected 2 parameters, actual number of parameters is 0");
}

#[test]
fn _0019() {
  te_null(false, &scope!(), r#"starts(20,[1..20],10)"#, "expected 2 parameters, actual number of parameters is 3");
}

#[test]
fn _0020() {
  te_null(false, &scope!(), r#"starts(p: 20, range: [1..20])"#, "[named::starts] invalid named parameters");
}

#[test]
fn _0021() {
  te_null(false, &scope!(), r#"starts(point: 20, r: [1..20])"#, "[named::starts] invalid named parameters");
}

#[test]
fn _0022() {
  te_null(false, &scope!(), r#"starts(p: 20, r: [1..20])"#, "[named::starts] invalid named parameters");
}

#[test]
fn _0023() {
  te_null(false, &scope!(), r#"starts(range1: [1..20], r2: [1..20])"#, "[named::starts] invalid named parameters");
}

#[test]
fn _0024() {
  te_null(false, &scope!(), r#"starts(r1: [1..20], range2: [1..20])"#, "[named::starts] invalid named parameters");
}

#[test]
fn _0025() {
  te_null(false, &scope!(), r#"starts(r1: [1..20], r2: [1..20])"#, "[named::starts] invalid named parameters");
}

#[test]
fn _0026() {
  te_null(
    false,
    &scope!(),
    r#"starts("a", [1..20])"#,
    "[core::starts] invalid argument type, expected scalar or range of scalars, actual type is string",
  );
}

#[test]
fn _0027() {
  te_null(
    false,
    &scope!(),
    r#"starts(["a".."z"], [1..20])"#,
    "[core::starts] invalid argument type, expected scalar or range of scalars, actual type is range<string>",
  );
}

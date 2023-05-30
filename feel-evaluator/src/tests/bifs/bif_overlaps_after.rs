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
  te_bool(false, &scope!(), r#"overlaps after([1..5],[3..8])"#, false);
}

#[test]
fn _0002() {
  te_bool(false, &scope!(), r#"overlaps after([6..12],[3..8])"#, true);
}

#[test]
fn _0003() {
  te_null(
    false,
    &scope!(),
    r#"overlaps after([6..12],[duration("P1D")..duration("P10D")])"#,
    "[core::overlaps after] invalid argument type, expected range<number>, actual type is range<days and time duration>",
  );
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"overlaps after(range1: [6..12], range2: [3..8])"#, true);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"overlaps after(range2: [3..8], range1: [6..12])"#, true);
}

#[test]
fn _0006() {
  te_bool(
    false,
    &scope!(),
    r#"overlaps after([date("2022-11-01")..date("2022-12-31")], [date("2022-06-01")..date("2022-12-06")])"#,
    true,
  );
}

#[test]
fn _0007() {
  te_null(
    false,
    &scope!(),
    r#"overlaps after([date("2022-11-01")..date("2022-12-31")], [date and time("2022-06-01T00:00:00")..date and time("2022-12-06T00:00:00")])"#,
    "[core::overlaps after] invalid argument type, expected range<date>, actual type is range<date and time>",
  );
}

#[test]
fn _0008() {
  te_bool(
    false,
    &scope!(),
    r#"overlaps after([date and time("2022-11-01T00:00:00")..date and time("2022-12-31T00:00:00")], [date and time("2022-06-01T00:00:00")..date and time("2022-12-06T00:00:00")])"#,
    true,
  );
}

#[test]
fn _0009() {
  te_null(
    false,
    &scope!(),
    r#"overlaps after([date and time("2022-11-01T00:00:00")..date and time("2022-12-31T00:00:00")], [date("2022-06-01")..date("2022-12-06")])"#,
    "[core::overlaps after] invalid argument type, expected range<date and time>, actual type is range<date>",
  );
}

#[test]
fn _0010() {
  te_bool(
    false,
    &scope!(),
    r#"overlaps after([time("13:15:03")..time("22:20:17")], [time("10:00:00")..time("21:59:11")])"#,
    true,
  );
}

#[test]
fn _0011() {
  te_null(
    false,
    &scope!(),
    r#"overlaps after([time("13:15:03")..time("22:20:17")], [date("2022-06-01")..date("2022-12-06")])"#,
    "[core::overlaps after] invalid argument type, expected range<time>, actual type is range<date>",
  );
}

#[test]
fn _0012() {
  te_bool(
    false,
    &scope!(),
    r#"overlaps after([duration("P10D")..duration("P20D")], [duration("P5D")..duration("P15D")])"#,
    true,
  );
}

#[test]
fn _0013() {
  te_null(
    false,
    &scope!(),
    r#"overlaps after([duration("P10D")..duration("P20D")], [duration("P5Y")..duration("P15Y")])"#,
    "[core::overlaps after] invalid argument type, expected range<days and time duration>, actual type is range<years and months duration>",
  );
}

#[test]
fn _0014() {
  te_bool(
    false,
    &scope!(),
    r#"overlaps after([duration("P10Y")..duration("P20Y")], [duration("P5Y")..duration("P15Y")])"#,
    true,
  );
}

#[test]
fn _0015() {
  te_null(
    false,
    &scope!(),
    r#"overlaps after([duration("P10Y")..duration("P20Y")], [duration("P5D")..duration("P15D")])"#,
    "[core::overlaps after] invalid argument type, expected range<years and months duration>, actual type is range<days and time duration>",
  );
}

#[test]
fn _0016() {
  te_null(false, &scope!(), r#"overlaps after()"#, "expected 2 parameters, actual number of parameters is 0");
}

#[test]
fn _0017() {
  te_null(
    false,
    &scope!(),
    r#"overlaps after([1..5],[3..8],[9..12])"#,
    "expected 2 parameters, actual number of parameters is 3",
  );
}

#[test]
fn _0018() {
  te_null(false, &scope!(), r#"overlaps after(range1: [1..5],r2: [3..8])"#, "parameter 'range2' not found");
}

#[test]
fn _0019() {
  te_null(false, &scope!(), r#"overlaps after(r1: [1..5],range2: [3..8])"#, "parameter 'range1' not found");
}

#[test]
fn _0020() {
  te_null(false, &scope!(), r#"overlaps after(r1: [1..5], r2: [3..8])"#, "parameter 'range1' not found");
}

#[test]
fn _0021() {
  te_null(
    false,
    &scope!(),
    r#"overlaps after(1, [3..8])"#,
    "[core::overlaps] invalid argument type, expected range<scalar>, actual type is number",
  );
}

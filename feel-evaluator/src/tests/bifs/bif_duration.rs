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
  te_years_and_months_duration(false, &scope!(), r#"duration("P1Y")"#, 1, 0);
}

#[test]
fn _0002() {
  te_years_and_months_duration(false, &scope!(), r#"duration("P4Y")"#, 4, 0);
}

#[test]
fn _0003() {
  te_years_and_months_duration(false, &scope!(), r#"duration("-P999999999Y")"#, -999_999_999, 0);
}

#[test]
fn _0004() {
  te_years_and_months_duration(false, &scope!(), r#"duration("P999999999Y")"#, 999_999_999, 0);
}

#[test]
fn _0005() {
  te_years_and_months_duration(false, &scope!(), r#"duration(from:"P26M")"#, 2, 2);
}

#[test]
fn _0006() {
  te_days_and_time_duration(false, &scope!(), r#"duration("P1D")"#, false, SECONDS_IN_DAY, 0);
}

#[test]
fn _0007() {
  te_days_and_time_duration(false, &scope!(), r#"duration("P4D")"#, false, 4 * SECONDS_IN_DAY, 0);
}

#[test]
fn _0008() {
  te_days_and_time_duration(false, &scope!(), r#"duration("PT2H")"#, false, 2 * SECONDS_IN_HOUR, 0);
}

#[test]
fn _0009() {
  te_days_and_time_duration(false, &scope!(), r#"duration("PT3M")"#, false, 3 * SECONDS_IN_MINUTE, 0);
}

#[test]
fn _0010() {
  te_days_and_time_duration(false, &scope!(), r#"duration("PT4S")"#, false, 4, 0);
}

#[test]
fn _0011() {
  te_days_and_time_duration(false, &scope!(), r#"duration(from:"PT24H")"#, false, SECONDS_IN_DAY, 0);
}

#[test]
fn _0012() {
  te_null(false, &scope!(), "duration(null)", "duration");
}

#[test]
fn _0013() {
  te_null(false, &scope!(), "duration()", "expected 1 parameters, actual number of parameters is 0");
}

#[test]
fn _0014() {
  te_null(false, &scope!(), "duration([])", "duration");
}

#[test]
fn _0015() {
  te_null(false, &scope!(), r#"duration("")"#, "duration");
}

#[test]
fn _0016() {
  te_null(false, &scope!(), "duration(2017)", "duration");
}

#[test]
fn _0017() {
  te_null(false, &scope!(), r#"duration("2012T-12-2511:00:00Z")"#, "duration");
}

#[test]
fn _0018() {
  te_null(false, &scope!(), r#"duration("P")"#, "duration");
}

#[test]
fn _0019() {
  te_null(false, &scope!(), r#"duration("P0")"#, "duration");
}

#[test]
fn _0020() {
  te_null(false, &scope!(), r#"duration("1Y")"#, "duration");
}

#[test]
fn _0021() {
  te_null(false, &scope!(), r#"duration("1D")"#, "duration");
}

#[test]
fn _0022() {
  te_null(false, &scope!(), r#"duration("P1H")"#, "duration");
}

#[test]
fn _0023() {
  te_null(false, &scope!(), r#"duration("P1S")"#, "duration");
}

#[test]
fn _0024() {
  te_null(false, &scope!(), r#"duration(f: "PT24H")"#, r#"parameter 'from' not found"#);
}

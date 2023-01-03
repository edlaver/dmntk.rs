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
 * you may not use this file except in compatibility with the License.
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
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "1-1", 0, 0);
}

#[test]
fn _0002() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, " 1 - 2 ", -1, 0);
}

#[test]
fn _0003() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, " 5 -2 -1 ", 2, 0);
}

#[test]
fn _0004() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "1000-200-2", 798, 0);
}

#[test]
fn _0005() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "( 1 - 2 ) - ( 3 - 4 )", 0, 0);
}

#[test]
fn _0006() {
  let scope = &te_scope(r#"{}"#);
  te_number(false, scope, "( ( ( 4 - 3 ) ) )", 1, 0);
}

#[test]
fn _0007() {
  let scope = &te_scope("{ a: 11.2, b: 0.2}");
  te_number(false, scope, "a-b", 11, 0);
}

#[test]
fn _0008() {
  let scope = &te_scope("{ a: 11.2, b: 0.2, c: 5.351 }");
  te_number(false, scope, "a-b-c", 5649, 3);
}

#[test]
fn _0009() {
  let scope = &te_scope("{ a: 11.2, b: 0.2, c: 5.351 }");
  te_number(false, scope, " a  \n -  b \n  -  c", 5649, 3);
}

#[test]
fn _0010() {
  te_time(false, &scope!(), r#"time("12:34:56") - duration("P0D")"#, FeelTime::new_hms_opt(12, 34, 56, 0).unwrap());
}

#[test]
fn _0011() {
  te_time(false, &scope!(), r#"time("12:34:56") - duration("P1D")"#, FeelTime::new_hms_opt(12, 34, 56, 0).unwrap());
}

#[test]
fn _0012() {
  te_time(false, &scope!(), r#"time("12:34:56") - duration("P11D")"#, FeelTime::new_hms_opt(12, 34, 56, 0).unwrap());
}

#[test]
fn _0013() {
  te_time(false, &scope!(), r#"time("12:34:56") - duration("PT10S")"#, FeelTime::new_hms_opt(12, 34, 46, 0).unwrap());
}

#[test]
fn _0014() {
  te_time(false, &scope!(), r#"time("12:34:23") - duration("PT25S")"#, FeelTime::new_hms_opt(12, 33, 58, 0).unwrap());
}

#[test]
fn _0015() {
  te_time(false, &scope!(), r#"time("12:34:23") - duration("PT40M")"#, FeelTime::new_hms_opt(11, 54, 23, 0).unwrap());
}

#[test]
fn _0016() {
  te_date(false, &scope!(), r#" @"2020-03-01" - @"PT24H" "#, 2020, 2, 29);
}

#[test]
fn _0017() {
  te_date(false, &scope!(), r#" @"2021-01-02" - @"PT25H" "#, 2020, 12, 31);
}

#[test]
/// Date converted to date and time has the timezone equal to UTC 00:00:00. Subtracted date and time has local timezone.
/// Such subtraction is not allowed (both date and times should have offset defined), that's why null value should be returned.
fn _0018() {
  te_null(
    false,
    &scope!(),
    r#" @"2021-01-02" - @"2021-01-01T10:10:10" "#,
    "[subtraction] incompatible types: 2021-01-02 - 2021-01-01T10:10:10",
  );
}

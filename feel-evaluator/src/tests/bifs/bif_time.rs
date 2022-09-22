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
  te_time(false, &scope!(), r#"time(11,59,45,null)"#, FeelTime::local(11, 59, 45, 0));
}

#[test]
fn _0002() {
  te_time(
    false,
    &scope!(),
    r#"time(hour: 11, minute: 59, second: 45, offset: null)"#,
    FeelTime::local(11, 59, 45, 0),
  );
}

#[test]
fn _0003() {
  te_time(false, &scope!(), r#"time(hour: 11, minute: 59, second: 45)"#, FeelTime::local(11, 59, 45, 0));
}

#[test]
fn _0004() {
  let scope = &te_scope(r#"{Hours:12,Minutes:59,Seconds:1.3,Timezone:@"-PT1H"}"#);
  te_time(
    false,
    scope,
    r#"time(Hours,Minutes,Seconds,Timezone)"#,
    FeelTime::new_hmso_opt(12, 59, 1, 300_000_000, -3600).unwrap(),
  );
}

#[test]
fn _0005() {
  te_time(false, &scope!(), r#"time("23:59:00")"#, FeelTime::local(23, 59, 0, 0));
}

#[test]
fn _0006() {
  te_time(false, &scope!(), r#"  time  (  "23:59:00"         )   "#, FeelTime::local(23, 59, 0, 0));
}

#[test]
fn _0007() {
  te_time(false, &scope!(), r#"time("23:59:00Z")"#, FeelTime::utc(23, 59, 0, 0));
}

#[test]
fn _0008() {
  te_time(false, &scope!(), r#"time("23:59:00z")"#, FeelTime::utc(23, 59, 0, 0));
}

#[test]
fn _0009() {
  te_time(false, &scope!(), r#"time("11:22:33-00:00")"#, FeelTime::utc(11, 22, 33, 0));
}

#[test]
fn _0010() {
  te_time(false, &scope!(), r#"time(from: "11:22:33-00:00")"#, FeelTime::utc(11, 22, 33, 0));
}

#[test]
fn _0011() {
  te_time(false, &scope!(), r#"time("11:22:33+00:00")"#, FeelTime::utc(11, 22, 33, 0));
}

#[test]
fn _0012() {
  te_time(false, &scope!(), r#"time(time("11:00:00"))"#, FeelTime::local(11, 0, 0, 0));
}

#[test]
fn _0013() {
  te_time(false, &scope!(), r#"time(from: time("11:00:00"))"#, FeelTime::local(11, 0, 0, 0));
}

#[test]
fn _0014() {
  te_time(
    false,
    &scope!(),
    r#"time(date and time("2019-12-06T18:34:12"))"#,
    FeelTime::local(18, 34, 12, 0),
  );
}

#[test]
fn _0015() {
  te_time(false, &scope!(), r#"time(date and time("2019-12-06T11:00:00Z"))"#, FeelTime::utc(11, 0, 0, 0));
}

#[test]
fn _0016() {
  te_time(
    false,
    &scope!(),
    r#"time(from: date and time("2019-12-06T11:00:00Z"))"#,
    FeelTime::utc(11, 0, 0, 0),
  );
}

#[test]
fn _0017() {
  te_time(false, &scope!(), r#"time(date and time("2019-12-06T11:00:00z"))"#, FeelTime::utc(11, 0, 0, 0));
}

#[test]
fn _0018() {
  te_time(false, &scope!(), r#"time(date("2019-12-06"))"#, FeelTime::utc(0, 0, 0, 0));
}

#[test]
fn _0019() {
  te_bool(false, &scope!(), r#"time("12:21:12") in [time("12:21:12")..time("12:21:12")]"#, true);
}

#[test]
fn _0020() {
  te_bool(false, &scope!(), r#"time("12:21:11") in [time("12:21:12")..time("12:21:12")]"#, false);
}

#[test]
fn _0021() {
  te_bool(false, &scope!(), r#"time("12:21:13") in [time("12:21:12")..time("12:21:12")]"#, false);
}

#[test]
fn _0022() {
  te_bool(false, &scope!(), r#"time("12:21:12") in (time("12:21:11")..time("12:21:13"))"#, true);
}

#[test]
fn _0023() {
  te_null(false, &scope!(), r#"time("22:63:12")"#, "time_1");
}

#[test]
fn _0024() {
  te_null(false, &scope!(), r#"time("22:10:12+15:00")"#, "time_1");
}

#[test]
fn _0025() {
  te_null(false, &scope!(), r#"time("22:10:12-15:00")"#, "time_1");
}

#[test]
fn _0026() {
  te_null(
    false,
    &scope!(),
    r#"time(24,59,45,null)"#,
    "[core::time_4] hour must be 0..23, current value is: 24",
  );
}

#[test]
fn _0027() {
  te_null(
    false,
    &scope!(),
    r#"time(23,60,45,null)"#,
    "[core::time_4] minute must be 0..59, current value is: 60",
  );
}

#[test]
fn _0028() {
  te_null(
    false,
    &scope!(),
    r#"time(24,59,45,null)"#,
    "[core::time_4] hour must be 0..23, current value is: 24",
  );
}

#[test]
fn _0029() {
  te_null(
    false,
    &scope!(),
    r#"time(23,60,45,null)"#,
    "[core::time_4] minute must be 0..59, current value is: 60",
  );
}

#[test]
fn _0030() {
  te_null(
    false,
    &scope!(),
    r#"time(23,59,60,null)"#,
    "[core::time_4] second must be 0..59, current value is: 60",
  );
}

#[test]
fn _0031() {
  te_null(
    false,
    &scope!(),
    r#"time(-12,12,12,null)"#,
    "[core::time_4] hour must be 0..23, current value is: -12",
  );
}

#[test]
fn _0032() {
  te_null(
    false,
    &scope!(),
    r#"time(12,-12,12,null)"#,
    "[core::time_4] minute must be 0..59, current value is: -12",
  );
}

#[test]
fn _0033() {
  te_null(
    false,
    &scope!(),
    r#"time(12,12,-12,null)"#,
    "[core::time_4] second must be 0..59, current value is: -12",
  );
}

#[test]
fn _0034() {
  te_time(false, &scope!(), r#"time(11,59,45)"#, FeelTime::local(11, 59, 45, 0));
}

#[test]
fn _0035() {
  te_null(false, &scope!(), r#"time()"#, r#"expected 1,3,4 parameters, actual number of parameters is 0"#);
}

#[test]
fn _0036() {
  te_null(
    false,
    &scope!(),
    r#"time(12,12)"#,
    r#"expected 1,3,4 parameters, actual number of parameters is 2"#,
  );
}

#[test]
fn _0037() {
  te_null(
    false,
    &scope!(),
    r#"time(12,12,12,12,12)"#,
    r#"expected 1,3,4 parameters, actual number of parameters is 5"#,
  );
}

#[test]
fn _0038() {
  te_null(false, &scope!(), r#"time(f: "11:22:33-00:00")"#, r#"invalid parameters in bif time"#);
}

#[test]
fn _0039() {
  te_null(
    false,
    &scope!(),
    r#"time(h: 11, minute: 59, second: 45, offset: null)"#,
    r#"invalid parameters in bif time"#,
  );
}

#[test]
fn _0040() {
  te_null(
    false,
    &scope!(),
    r#"time(hour: 11, m: 59, second: 45, offset: null)"#,
    r#"invalid parameters in bif time"#,
  );
}

#[test]
fn _0041() {
  te_null(
    false,
    &scope!(),
    r#"time(hour: 11, minute: 59, s: 45, offset: null)"#,
    r#"invalid parameters in bif time"#,
  );
}

#[test]
fn _0042() {
  te_null(
    false,
    &scope!(),
    r#"time("11", 59, 45, null)"#,
    r#"[core::time_4] hour must be a number, current type is: string"#,
  );
}

#[test]
fn _0043() {
  te_null(
    false,
    &scope!(),
    r#"time(11, "59", 45, null)"#,
    r#"[core::time_4] minutes must be a number, current type is: string"#,
  );
}

#[test]
fn _0044() {
  te_null(
    false,
    &scope!(),
    r#"time(11, 59, "45", null)"#,
    r#"[core::time_4] seconds must be a number, current type is: string"#,
  );
}

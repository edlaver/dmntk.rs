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
use crate::model_evaluator::ModelEvaluator;
use std::sync::Arc;

lazy_static! {
  static ref MODEL_EVALUATOR: Arc<ModelEvaluator> = build_model_evaluator(dmntk_examples::DMN_3_0036);
}

const CTX_1: &str = r#"
  {
    "Another Date": @"2018-07-31",
    "Another Date and Time": @"2018-07-31T17:13:00Z",
    "Another Days and Time Duration": @"PT12H",
    "Another String": "Hello",
    "Another Time": @"17:13:00",
    "Another Years and Months Duration": @"P8M",
    "Another boolean": false,
    "Another number": 15,
    Complex: {
      aBoolean: true,
      aDate: @"2018-07-30",
      aDateTime: @"2018-07-30T16:12:00Z",
      aDaysAndTimeDuration: @"PT10H",
      aNumber: 10,
      aString: "Hi",
      aTime: @"16:11:00",
      aYearsAndMonthsDuration: @"P5M"
    }
  }"#;

#[test]
fn _0001() {
  let ctx = context(CTX_1);
  assert_decision(&MODEL_EVALUATOR, "Compare Boolean", &ctx, r#""Not same boolean""#);
}

#[test]
fn _0002() {
  let ctx = context(CTX_1);
  assert_decision(&MODEL_EVALUATOR, "Compare String", &ctx, r#""Different String""#);
}

#[test]
fn _0003() {
  let ctx = context(CTX_1);
  assert_decision(&MODEL_EVALUATOR, "Compare Date", &ctx, r#""Future Date""#);
}
/*

#[test]
fn _0004() {

  let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Number", &ctx, r#""Bigger""#);
}


#[test]
fn _0005() {

  let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Date and Time", &ctx, r#""Future date time""#);
}


#[test]
fn _0006() {

  let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Days and Time Duration", &ctx, r#""Longer duration""#);
}


#[test]
fn _0007() {

  let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Years and Months Duration", &ctx, r#""Longer duration""#);
}


#[test]
fn _0008() {

  let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Time", &ctx, r#""Future Time""#);
}


#[test]
fn _0009() {

  let ctx = context(r#"{Another Date: 2018-07-31,Another Date and Time: 2018-07-31T17: 13: 00Z,Another Days and Time Duration: PT12H,Another String: ""Hello"",Another Time: 17: 13: 00,Another Years and Months Duration: P8M,Another boolean: false,Another number: 15,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Boolean", &ctx, r#"null(no rules matched, no output value defined)"#);
}


#[test]
fn _0010() {

  let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare String", &ctx, r#""Different String""#);
}


#[test]
fn _0011() {

  let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Date", &ctx, r#""Past Date""#);
}


#[test]
fn _0012() {

  let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Number", &ctx, r#""Smaller""#);
}


#[test]
fn _0013() {

  let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Date and Time", &ctx, r#""Past date time""#);
}


#[test]
fn _0014() {

  let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Days and Time Duration", &ctx, r#""Shorter duration""#);
}


#[test]
fn _0015() {

  let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Years and Months Duration", &ctx, r#""Shorter duration""#);
}


#[test]
fn _0016() {

  let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Time", &ctx, r#""Past Time""#);
}


#[test]
fn _0017() {

  let ctx = context(r#"{Another Date: 2018-07-29,Another Date and Time: 2018-07-29T15: 13: 00Z,Another Days and Time Duration: PT8H,Another String: ""Hello"",Another Time: 15: 13: 00,Another Years and Months Duration: P3M,Another boolean: false,Another number: 5,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Boolean", &ctx, r#"null(no rules matched, no output value defined)"#);
}


#[test]
fn _0018() {

  let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare String", &ctx, r#""Same String""#);
}


#[test]
fn _0019() {

  let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Date", &ctx, r#""Same Date""#);
}


#[test]
fn _0020() {

  let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Number", &ctx, r#""Equals""#);
}


#[test]
fn _0021() {

  let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Date and Time", &ctx, r#""Same date time""#);
}


#[test]
fn _0022() {

  let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Days and Time Duration", &ctx, r#""Same duration""#);
}


#[test]
fn _0023() {

  let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Years and Months Duration", &ctx, r#""Same duration""#);
}


#[test]
fn _0024() {

  let ctx = context(r#"{Another Date: 2018-07-30,Another Date and Time: 2018-07-30T16: 12: 00Z,Another Days and Time Duration: PT10H,Another String: ""Hi"",Another Time: 16: 11: 00,Another Years and Months Duration: P5M,Another boolean: true,Another number: 10,Complex: {aBoolean: true,aDate: 2018-07-30,aDateTime: 2018-07-30T16: 12: 00Z,aDaysAndTimeDuration: PT10H,aNumber: 10,aString: ""Hi"",aTime: 16: 11: 00,aYearsAndMonthsDuration: P5M}}"#);
  assert_decision(&MODEL_EVALUATOR, "Compare Time", &ctx, r#""Same Time""#);
}


*/

#[test]
fn _0025() {
  let ctx = context(
    r#"
    {
      "Another Date": "2018-07-30",
      "Another Date and Time": "2018-07-30T16:12:00Z",
       Another Days and Time Duration: "PT10H",
       Another String: "Hi",
       Another Time: "16:11:00",
       Another Years and Months Duration: "P5M",
       Another boolean: true,
       Another number: 10,
       Complex: {
         aBoolean: true,
         aDate: "2018-07-30",
         aDateTime: "2018-07-30T16:12:00Z",
         aDaysAndTimeDuration: "PT10H",
         aNumber: 10,
         aString: "Hi",
         aTime: "16:11:00",
         aYearsAndMonthsDuration: "P5M"
      }
    }
  "#,
  );
  assert_decision(&MODEL_EVALUATOR, "Compare Boolean", &ctx, r#""Same boolean""#);
}

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

use crate::context::ParsingContext;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::FeelType;

#[test]
fn test_display() {
  let mut pctx_a = ParsingContext::default();
  assert_eq!("{}", format!("{pctx_a}"));
  pctx_a.set_name("a".into());
  assert_eq!("{a: <v>}", format!("{pctx_a}"));
  pctx_a.set_name("b".into());
  assert_eq!("{a: <v>, b: <v>}", format!("{pctx_a}"));
  pctx_a.set_name("c".into());
  assert_eq!("{a: <v>, b: <v>, c: <v>}", format!("{pctx_a}"));
  let mut pctx_b = ParsingContext::default();
  pctx_b.set_name("x".into());
  pctx_b.set_name("y".into());
  pctx_b.set_name("z".into());
  pctx_a.set_context("d".into(), pctx_b);
  assert_eq!("{a: <v>, b: <v>, c: <v>, d: {x: <v>, y: <v>, z: <v>}}", format!("{pctx_a}"));
}

#[test]
fn test_from_feel_context_0001() {
  let fctx = FeelContext::default();
  let pctx: ParsingContext = fctx.into();
  assert_eq!("{}", format!("{pctx}"));
}

#[test]
fn test_from_feel_context_0002() {
  let mut fctx = FeelContext::default();
  fctx.set_null("a".into());
  let pctx: ParsingContext = fctx.into();
  assert_eq!("{a: <v>}", format!("{pctx}"));
}

#[test]
fn test_from_feel_context_0003() {
  let mut fctx = FeelContext::default();
  fctx.set_null("a".into());
  fctx.set_null("b".into());
  let pctx: ParsingContext = fctx.into();
  assert_eq!("{a: <v>, b: <v>}", format!("{pctx}"));
}

#[test]
fn test_from_feel_context_0004() {
  let mut fctx_a = FeelContext::default();
  fctx_a.set_null("a".into());
  fctx_a.set_null("b".into());
  let mut fctx_b = FeelContext::default();
  fctx_b.set_null("x".into());
  fctx_b.set_entry(&"y".into(), Value::Context(fctx_a));
  let pctx: ParsingContext = fctx_b.into();
  assert_eq!("{x: <v>, y: {a: <v>, b: <v>}}", format!("{pctx}"));
}

#[test]
fn test_from_feel_context_0005() {
  let mut fctx_a = FeelContext::default();
  fctx_a.set_null("a".into());
  fctx_a.set_entry(&"b".into(), Value::FeelType(FeelType::Any));
  let mut fctx_b = FeelContext::default();
  fctx_b.set_null("x".into());
  fctx_b.set_entry(&"y".into(), Value::Context(fctx_a));
  let pctx: ParsingContext = fctx_b.into();
  assert_eq!("{x: <v>, y: {a: <v>, b: <v>}}", format!("{pctx}"));
}

#[test]
fn test_from_feel_context_0006() {
  let mut fctx_a = FeelContext::default();
  fctx_a.set_null("a".into());
  fctx_a.set_entry(
    &"b".into(),
    Value::FeelType(FeelType::context(&[(&"i".into(), &FeelType::Number), (&"j".into(), &FeelType::Boolean)])),
  );
  let mut fctx_b = FeelContext::default();
  fctx_b.set_null("x".into());
  fctx_b.set_entry(&"y".into(), Value::Context(fctx_a));
  let pctx: ParsingContext = fctx_b.into();
  assert_eq!("{x: <v>, y: {a: <v>, b: {i: <v>, j: <v>}}}", format!("{pctx}"));
}

#[test]
fn test_from_feel_context_0007() {
  let mut fctx_a = FeelContext::default();
  fctx_a.set_null("a".into());
  fctx_a.set_entry(
    &"b".into(),
    Value::FeelType(FeelType::context(&[
      (&"i".into(), &FeelType::Number),
      (&"j".into(), &FeelType::context(&[(&"m".into(), &FeelType::Date), (&"n".into(), &FeelType::Time)])),
    ])),
  );
  let mut fctx_b = FeelContext::default();
  fctx_b.set_null("x".into());
  fctx_b.set_entry(&"y".into(), Value::Context(fctx_a));
  let pctx: ParsingContext = fctx_b.into();
  assert_eq!("{x: <v>, y: {a: <v>, b: {i: <v>, j: {m: <v>, n: <v>}}}}", format!("{pctx}"));
}

#[test]
fn test_from_feel_context_0008() {
  let mut fctx_a = FeelContext::default();
  fctx_a.set_null("a".into());
  fctx_a.set_entry(&"b".into(), Value::FeelType(FeelType::list(&FeelType::Boolean)));
  let mut fctx_b = FeelContext::default();
  fctx_b.set_null("x".into());
  fctx_b.set_entry(&"y".into(), Value::Context(fctx_a));
  let pctx: ParsingContext = fctx_b.into();
  assert_eq!("{x: <v>, y: {a: <v>, b: <v>}}", format!("{pctx}"));
}

#[test]
fn test_from_feel_context_0009() {
  let mut fctx_a = FeelContext::default();
  fctx_a.set_null("a".into());
  fctx_a.set_entry(
    &"b".into(),
    Value::FeelType(FeelType::list(&FeelType::context(&[
      (&"c".into(), &FeelType::DaysAndTimeDuration),
      (&"d".into(), &FeelType::YearsAndMonthsDuration),
    ]))),
  );
  let mut fctx_b = FeelContext::default();
  fctx_b.set_null("x".into());
  fctx_b.set_entry(&"y".into(), Value::Context(fctx_a));
  let pctx: ParsingContext = fctx_b.into();
  assert_eq!("{x: <v>, y: {a: <v>, b: {c: <v>, d: <v>}}}", format!("{pctx}"));
}

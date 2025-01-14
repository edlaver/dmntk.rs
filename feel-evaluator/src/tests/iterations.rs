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

use crate::iterations::{FeelIterator, ForExpressionEvaluator};
use crate::tests::te_scope;
use dmntk_feel::values::{values_to_string, Value, Values};
use dmntk_feel::{value_number, FeelNumber};

#[test]
fn _0001() {
  let mut iterator = FeelIterator::default();
  iterator.add_range("x".into(), 1, 3);
  let mut actual = vec![];
  iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(3, actual.len());
  assert_eq!(r#"[{x: 1}, {x: 2}, {x: 3}]"#, values_to_string(&actual));
}

#[test]
fn _0002() {
  let mut iterator = FeelIterator::default();
  iterator.add_range("x".into(), 1, 3);
  iterator.add_range("y".into(), 1, 5);
  let mut actual = vec![];
  iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(15, actual.len());
  assert_eq!(
    r#"[{x: 1, y: 1}, {x: 1, y: 2}, {x: 1, y: 3}, {x: 1, y: 4}, {x: 1, y: 5}, {x: 2, y: 1}, {x: 2, y: 2}, {x: 2, y: 3}, {x: 2, y: 4}, {x: 2, y: 5}, {x: 3, y: 1}, {x: 3, y: 2}, {x: 3, y: 3}, {x: 3, y: 4}, {x: 3, y: 5}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0003() {
  let mut iterator = FeelIterator::default();
  iterator.add_range("x".into(), 3, 1);
  iterator.add_range("y".into(), 5, 1);
  let mut actual = vec![];
  iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(15, actual.len());
  assert_eq!(
    r#"[{x: 3, y: 5}, {x: 3, y: 4}, {x: 3, y: 3}, {x: 3, y: 2}, {x: 3, y: 1}, {x: 2, y: 5}, {x: 2, y: 4}, {x: 2, y: 3}, {x: 2, y: 2}, {x: 2, y: 1}, {x: 1, y: 5}, {x: 1, y: 4}, {x: 1, y: 3}, {x: 1, y: 2}, {x: 1, y: 1}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0004() {
  let mut iterator = FeelIterator::default();
  iterator.add_range("x".into(), 1, 2);
  iterator.add_range("y".into(), 1, 3);
  iterator.add_range("z".into(), 1, 4);
  let mut actual = vec![];
  iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(24, actual.len());
  assert_eq!(
    r#"[{x: 1, y: 1, z: 1}, {x: 1, y: 1, z: 2}, {x: 1, y: 1, z: 3}, {x: 1, y: 1, z: 4}, {x: 1, y: 2, z: 1}, {x: 1, y: 2, z: 2}, {x: 1, y: 2, z: 3}, {x: 1, y: 2, z: 4}, {x: 1, y: 3, z: 1}, {x: 1, y: 3, z: 2}, {x: 1, y: 3, z: 3}, {x: 1, y: 3, z: 4}, {x: 2, y: 1, z: 1}, {x: 2, y: 1, z: 2}, {x: 2, y: 1, z: 3}, {x: 2, y: 1, z: 4}, {x: 2, y: 2, z: 1}, {x: 2, y: 2, z: 2}, {x: 2, y: 2, z: 3}, {x: 2, y: 2, z: 4}, {x: 2, y: 3, z: 1}, {x: 2, y: 3, z: 2}, {x: 2, y: 3, z: 3}, {x: 2, y: 3, z: 4}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0005() {
  let mut iterator = FeelIterator::default();
  let list = vec![Value::String("a".to_string()), Value::String("b".to_string()), Value::String("c".to_string())];
  iterator.add_list("x".into(), list);
  let mut actual = vec![];
  iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(3, actual.len());
  assert_eq!(r#"[{x: "a"}, {x: "b"}, {x: "c"}]"#, values_to_string(&actual));
}

#[test]
fn _0006() {
  let mut iterator = FeelIterator::default();
  let list1 = vec![Value::String("a".to_string()), Value::String("b".to_string()), Value::String("c".to_string())];
  iterator.add_list("x".into(), list1);
  let list2 = vec![value_number!(1, 0), value_number!(2, 0), value_number!(3, 0)];
  iterator.add_list("y".into(), list2);
  let mut actual = vec![];
  iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(9, actual.len());
  assert_eq!(
    r#"[{x: "a", y: 1}, {x: "a", y: 2}, {x: "a", y: 3}, {x: "b", y: 1}, {x: "b", y: 2}, {x: "b", y: 3}, {x: "c", y: 1}, {x: "c", y: 2}, {x: "c", y: 3}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0007() {
  let mut iterator = FeelIterator::default();
  let list_x = vec![Value::String("a".to_string()), Value::String("b".to_string())];
  let list_y = vec![value_number!(1, 0), value_number!(2, 0), value_number!(3, 0)];
  let list_z = vec![value_number!(1, 0), value_number!(2, 0), value_number!(3, 0), value_number!(4, 0)];
  iterator.add_list("x".into(), list_x);
  iterator.add_list("y".into(), list_y);
  iterator.add_list("z".into(), list_z);
  let mut actual = vec![];
  iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(24, actual.len());
  assert_eq!(
    r#"[{x: "a", y: 1, z: 1}, {x: "a", y: 1, z: 2}, {x: "a", y: 1, z: 3}, {x: "a", y: 1, z: 4}, {x: "a", y: 2, z: 1}, {x: "a", y: 2, z: 2}, {x: "a", y: 2, z: 3}, {x: "a", y: 2, z: 4}, {x: "a", y: 3, z: 1}, {x: "a", y: 3, z: 2}, {x: "a", y: 3, z: 3}, {x: "a", y: 3, z: 4}, {x: "b", y: 1, z: 1}, {x: "b", y: 1, z: 2}, {x: "b", y: 1, z: 3}, {x: "b", y: 1, z: 4}, {x: "b", y: 2, z: 1}, {x: "b", y: 2, z: 2}, {x: "b", y: 2, z: 3}, {x: "b", y: 2, z: 4}, {x: "b", y: 3, z: 1}, {x: "b", y: 3, z: 2}, {x: "b", y: 3, z: 3}, {x: "b", y: 3, z: 4}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0008() {
  let mut iterator = FeelIterator::default();
  iterator.add_range("x".into(), 1, 2);
  iterator.add_list("y".into(), vec![value_number!(1, 0), value_number!(2, 0), value_number!(3, 0)]);
  let mut actual = vec![];
  iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(6, actual.len());
  assert_eq!(
    r#"[{x: 1, y: 1}, {x: 1, y: 2}, {x: 1, y: 3}, {x: 2, y: 1}, {x: 2, y: 2}, {x: 2, y: 3}]"#,
    values_to_string(&actual)
  );
}

#[test]
fn _0009() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_range("x".into(), value_number!(1, 0), value_number!(3, 0));
  let scope = &te_scope(r#"{x:null}"#);
  let node = dmntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&node).unwrap();
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(3, actual.len());
  assert_eq!(r#"[2, 3, 4]"#, values_to_string(&actual));
}

#[test]
fn _0010() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_range("x".into(), value_number!(1, 0), value_number!(2, 0));
  iterator.add_single("y".into(), Value::List(vec![value_number!(5, 0), value_number!(6, 0), value_number!(7, 0)]));
  let scope = &te_scope(r#"{x:null,y:null}"#);
  let node = dmntk_feel_parser::parse_expression(scope, "x+y", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&node).unwrap();
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(6, actual.len());
  assert_eq!(r#"[6, 7, 8, 7, 8, 9]"#, values_to_string(&actual));
}

#[test]
fn _0011() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_single("x".into(), Value::List(Values::default()));
  let scope = &te_scope(r#"{x:null}"#);
  let node = dmntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&node).unwrap();
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(0, actual.len());
  assert_eq!(r#"[]"#, values_to_string(&actual));
}

#[test]
fn _0012() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_single("x".into(), value_number!(1, 0));
  let scope = &te_scope(r#"{x:null}"#);
  let node = dmntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&node).unwrap();
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(1, actual.len());
  assert_eq!(r#"[2]"#, values_to_string(&actual));
}

#[test]
fn _0013() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_range("x".into(), value_number!(1, 0), value_number!(2, 0));
  iterator.add_single("y".into(), Value::List(Values::default()));
  let scope = &te_scope(r#"{x:null,y:null}"#);
  let node = dmntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&node).unwrap();
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(2, actual.len());
  assert_eq!(r#"[2, 3]"#, values_to_string(&actual));
}

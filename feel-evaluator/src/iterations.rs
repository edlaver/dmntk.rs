/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
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

//!

use dmntk_feel::context::FeelContext;
use dmntk_feel::values::{Value, Values};
use dmntk_feel::{Evaluator, Name, Scope};

///
enum FeelIterationType {
  Range,
  List,
}

///
struct FeelIteratorState {
  /// Enumeration indicating if the iteration is over the range of values or a list of values.
  iteration_type: FeelIterationType,
  /// Name of the variable used in iteration state.
  name: Name,
  /// Current value of the looping index.
  index: isize,
  /// Iteration step.
  step: isize,
  /// Iteration start value.
  start: isize,
  /// Iteration end value.
  end: isize,
  /// Collection of `FEEL` values to iterate over (if not a range).
  values: Option<Values>,
}

///
#[derive(Default)]
struct FeelIterator {
  iteration_states: Vec<FeelIteratorState>,
}

impl FeelIterator {
  ///
  pub fn add_range(&mut self, name: Name, start: isize, end: isize) {
    self.iteration_states.push(FeelIteratorState {
      iteration_type: FeelIterationType::Range,
      name,
      index: start,
      step: if start <= end { 1 } else { -1 },
      start,
      end,
      values: None,
    });
  }
  ///
  pub fn add_list(&mut self, name: Name, values: Values) {
    self.iteration_states.push(FeelIteratorState {
      iteration_type: FeelIterationType::List,
      name,
      index: 0,
      step: 1,
      start: 0,
      end: (values.len() as isize) - 1,
      values: Some(values),
    });
  }
  ///
  pub fn run<F>(&mut self, mut handler: F)
  where
    F: FnMut(&FeelContext),
  {
    if !self.iteration_states.is_empty() {
      self.iteration_states.reverse();
      let mut iteration_context = FeelContext::default();
      'outer: loop {
        let mut is_empty_iteration = true;
        for iteration_state in &self.iteration_states {
          match iteration_state.iteration_type {
            FeelIterationType::Range => {
              let value = Value::Number(iteration_state.index.into());
              iteration_context.set_entry(&iteration_state.name, value);
              is_empty_iteration = false;
            }
            FeelIterationType::List => {
              if let Some(values) = &iteration_state.values {
                let index = iteration_state.index as usize;
                if let Some(value) = values.as_vec().get(index) {
                  iteration_context.set_entry(&iteration_state.name, value.clone());
                  is_empty_iteration = false;
                }
              }
            }
          }
        }
        if !is_empty_iteration {
          handler(&iteration_context);
        }
        let last_iteration_state_index = self.iteration_states.len() - 1;
        let mut overflow = true;
        'inner: for (x, iteration_state) in self.iteration_states.iter_mut().enumerate() {
          if overflow {
            if x == last_iteration_state_index {
              if iteration_state.step > 0 && iteration_state.index + iteration_state.step > iteration_state.end {
                break 'outer;
              }
              if iteration_state.step < 0 && iteration_state.index + iteration_state.step < iteration_state.end {
                break 'outer;
              }
            }
            if iteration_state.step > 0 {
              if iteration_state.index + iteration_state.step <= iteration_state.end {
                iteration_state.index += iteration_state.step;
                overflow = false;
              } else {
                iteration_state.index = iteration_state.start;
                overflow = true;
              }
            }
            if iteration_state.step < 0 {
              if iteration_state.index + iteration_state.step >= iteration_state.end {
                iteration_state.index += iteration_state.step;
                overflow = false;
              } else {
                iteration_state.index = iteration_state.start;
                overflow = true;
              }
            }
            if iteration_state.step == 0 {
              break 'outer;
            }
          } else {
            break 'inner;
          }
        }
      }
    }
  }
}

///
pub struct ForExpressionEvaluator {
  feel_iterator: FeelIterator,
  name_partial: Name,
}

impl ForExpressionEvaluator {
  ///
  pub fn new() -> Self {
    Self {
      feel_iterator: FeelIterator::default(),
      name_partial: "partial".into(),
    }
  }
  ///
  pub fn add_single(&mut self, name: Name, value: Value) {
    let values = match value {
      Value::List(values) => values,
      other => Values::new(vec![other]),
    };
    self.feel_iterator.add_list(name, values);
  }
  ///
  pub fn add_range(&mut self, name: Name, range_start: Value, range_end: Value) {
    if let Value::Number(start) = range_start {
      if let Value::Number(end) = range_end {
        self.feel_iterator.add_range(name, start.into(), end.into());
      }
    }
  }
  ///
  pub fn evaluate(&mut self, scope: &Scope, evaluator: &Evaluator) -> Values {
    let mut results = vec![];
    self.feel_iterator.run(|ctx| {
      let mut iteration_context = ctx.clone();
      iteration_context.set_entry(&self.name_partial, Value::List(Values::new(results.clone())));
      scope.push(iteration_context.clone());
      let iteration_value = evaluator(scope);
      scope.pop();
      results.push(iteration_value);
    });
    Values::new(results)
  }
}

///
pub struct SomeExpressionEvaluator {
  feel_iterator: FeelIterator,
}

impl SomeExpressionEvaluator {
  ///
  pub fn new() -> Self {
    Self {
      feel_iterator: FeelIterator::default(),
    }
  }
  ///
  pub fn add(&mut self, name: Name, value: Value) {
    let values = match value {
      Value::List(values) => values,
      other => Values::new(vec![other]),
    };
    self.feel_iterator.add_list(name, values);
  }
  ///
  pub fn evaluate(&mut self, scope: &Scope, evaluator: &Evaluator) -> Value {
    let mut result = false;
    self.feel_iterator.run(|ctx| {
      scope.push(ctx.clone());
      if let Value::Boolean(value) = evaluator(scope) {
        result = result || value;
      }
      scope.pop();
    });
    Value::Boolean(result)
  }
}

///
pub struct EveryExpressionEvaluator {
  feel_iterator: FeelIterator,
}

impl EveryExpressionEvaluator {
  ///
  pub fn new() -> Self {
    Self {
      feel_iterator: FeelIterator::default(),
    }
  }
  ///
  pub fn add(&mut self, name: Name, value: Value) {
    let values = match value {
      Value::List(values) => values,
      other => Values::new(vec![other]),
    };
    self.feel_iterator.add_list(name, values);
  }
  ///
  pub fn evaluate(&mut self, scope: &Scope, evaluator: &Evaluator) -> Value {
    let mut result = true;
    self.feel_iterator.run(|ctx| {
      scope.push(ctx.clone());
      if let Value::Boolean(value) = evaluator(scope) {
        result = result && value;
      }
      scope.pop();
    });
    Value::Boolean(result)
  }
}

#[cfg(test)]
mod tests {
  use crate::iterations::{FeelIterator, ForExpressionEvaluator};
  use crate::tests::te_scope;
  use dmntk_feel::values::{Value, Values};
  use dmntk_feel::{value_number, FeelNumber};

  #[test]
  fn test_range_iterator_1() {
    let mut iterator = FeelIterator::default();
    iterator.add_range("x".into(), 1, 3);
    let mut actual = vec![];
    iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
    assert_eq!(3, actual.len());
    assert_eq!(r#"[{x: 1}, {x: 2}, {x: 3}]"#, Values::new(actual).to_string());
  }

  #[test]
  fn test_range_iterator_2() {
    let mut iterator = FeelIterator::default();
    iterator.add_range("x".into(), 1, 3);
    iterator.add_range("y".into(), 1, 5);
    let mut actual = vec![];
    iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
    assert_eq!(15, actual.len());
    assert_eq!(
      r#"[{x: 1, y: 1}, {x: 1, y: 2}, {x: 1, y: 3}, {x: 1, y: 4}, {x: 1, y: 5}, {x: 2, y: 1}, {x: 2, y: 2}, {x: 2, y: 3}, {x: 2, y: 4}, {x: 2, y: 5}, {x: 3, y: 1}, {x: 3, y: 2}, {x: 3, y: 3}, {x: 3, y: 4}, {x: 3, y: 5}]"#,
      Values::new(actual).to_string()
    );
  }

  #[test]
  fn test_range_iterator_3() {
    let mut iterator = FeelIterator::default();
    iterator.add_range("x".into(), 1, 2);
    iterator.add_range("y".into(), 1, 3);
    iterator.add_range("z".into(), 1, 4);
    let mut actual = vec![];
    iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
    assert_eq!(24, actual.len());
    assert_eq!(
      r#"[{x: 1, y: 1, z: 1}, {x: 1, y: 1, z: 2}, {x: 1, y: 1, z: 3}, {x: 1, y: 1, z: 4}, {x: 1, y: 2, z: 1}, {x: 1, y: 2, z: 2}, {x: 1, y: 2, z: 3}, {x: 1, y: 2, z: 4}, {x: 1, y: 3, z: 1}, {x: 1, y: 3, z: 2}, {x: 1, y: 3, z: 3}, {x: 1, y: 3, z: 4}, {x: 2, y: 1, z: 1}, {x: 2, y: 1, z: 2}, {x: 2, y: 1, z: 3}, {x: 2, y: 1, z: 4}, {x: 2, y: 2, z: 1}, {x: 2, y: 2, z: 2}, {x: 2, y: 2, z: 3}, {x: 2, y: 2, z: 4}, {x: 2, y: 3, z: 1}, {x: 2, y: 3, z: 2}, {x: 2, y: 3, z: 3}, {x: 2, y: 3, z: 4}]"#,
      Values::new(actual).to_string()
    );
  }

  #[test]
  fn test_list_iterator_1() {
    let mut iterator = FeelIterator::default();
    let list = Values::new(vec![
      Value::String("a".to_string()),
      Value::String("b".to_string()),
      Value::String("c".to_string()),
    ]);
    iterator.add_list("x".into(), list);
    let mut actual = vec![];
    iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
    assert_eq!(3, actual.len());
    assert_eq!(r#"[{x: "a"}, {x: "b"}, {x: "c"}]"#, Values::new(actual).to_string());
  }

  #[test]
  fn test_list_iterator_2() {
    let mut iterator = FeelIterator::default();
    let list1 = Values::new(vec![
      Value::String("a".to_string()),
      Value::String("b".to_string()),
      Value::String("c".to_string()),
    ]);
    iterator.add_list("x".into(), list1);
    let list2 = Values::new(vec![value_number!(1, 0), value_number!(2, 0), value_number!(3, 0)]);
    iterator.add_list("y".into(), list2);
    let mut actual = vec![];
    iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
    assert_eq!(9, actual.len());
    assert_eq!(
      r#"[{x: "a", y: 1}, {x: "a", y: 2}, {x: "a", y: 3}, {x: "b", y: 1}, {x: "b", y: 2}, {x: "b", y: 3}, {x: "c", y: 1}, {x: "c", y: 2}, {x: "c", y: 3}]"#,
      Values::new(actual).to_string()
    );
  }

  #[test]
  fn test_list_iterator_3() {
    let mut iterator = FeelIterator::default();
    let list_x = Values::new(vec![Value::String("a".to_string()), Value::String("b".to_string())]);
    let list_y = Values::new(vec![value_number!(1, 0), value_number!(2, 0), value_number!(3, 0)]);
    let list_z = Values::new(vec![value_number!(1, 0), value_number!(2, 0), value_number!(3, 0), value_number!(4, 0)]);
    iterator.add_list("x".into(), list_x);
    iterator.add_list("y".into(), list_y);
    iterator.add_list("z".into(), list_z);
    let mut actual = vec![];
    iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
    assert_eq!(24, actual.len());
    assert_eq!(
      r#"[{x: "a", y: 1, z: 1}, {x: "a", y: 1, z: 2}, {x: "a", y: 1, z: 3}, {x: "a", y: 1, z: 4}, {x: "a", y: 2, z: 1}, {x: "a", y: 2, z: 2}, {x: "a", y: 2, z: 3}, {x: "a", y: 2, z: 4}, {x: "a", y: 3, z: 1}, {x: "a", y: 3, z: 2}, {x: "a", y: 3, z: 3}, {x: "a", y: 3, z: 4}, {x: "b", y: 1, z: 1}, {x: "b", y: 1, z: 2}, {x: "b", y: 1, z: 3}, {x: "b", y: 1, z: 4}, {x: "b", y: 2, z: 1}, {x: "b", y: 2, z: 2}, {x: "b", y: 2, z: 3}, {x: "b", y: 2, z: 4}, {x: "b", y: 3, z: 1}, {x: "b", y: 3, z: 2}, {x: "b", y: 3, z: 3}, {x: "b", y: 3, z: 4}]"#,
      Values::new(actual).to_string()
    );
  }

  #[test]
  fn test_range_list() {
    let mut iterator = FeelIterator::default();
    iterator.add_range("x".into(), 1, 2);
    iterator.add_list("y".into(), Values::new(vec![value_number!(1, 0), value_number!(2, 0), value_number!(3, 0)]));
    let mut actual = vec![];
    iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
    assert_eq!(6, actual.len());
    assert_eq!(
      r#"[{x: 1, y: 1}, {x: 1, y: 2}, {x: 1, y: 3}, {x: 2, y: 1}, {x: 2, y: 2}, {x: 2, y: 3}]"#,
      Values::new(actual).to_string()
    );
  }

  #[test]
  fn test_for_expression_evaluator_1() {
    let mut iterator = ForExpressionEvaluator::new();
    iterator.add_range("x".into(), value_number!(1, 0), value_number!(3, 0));
    let scope = &te_scope(r#"{x:null}"#);
    let node = dmntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
    let evaluator = crate::builders::build_evaluator(&node).unwrap();
    let actual = iterator.evaluate(scope, &evaluator);
    assert_eq!(3, actual.len());
    assert_eq!(r#"[2, 3, 4]"#, actual.to_string());
  }

  #[test]
  fn test_for_expression_evaluator_2() {
    let mut iterator = ForExpressionEvaluator::new();
    iterator.add_range("x".into(), value_number!(1, 0), value_number!(2, 0));
    iterator.add_single(
      "y".into(),
      Value::List(Values::new(vec![value_number!(5, 0), value_number!(6, 0), value_number!(7, 0)])),
    );
    let scope = &te_scope(r#"{x:null,y:null}"#);
    let node = dmntk_feel_parser::parse_expression(scope, "x+y", false).unwrap();
    let evaluator = crate::builders::build_evaluator(&node).unwrap();
    let actual = iterator.evaluate(scope, &evaluator);
    assert_eq!(6, actual.len());
    assert_eq!(r#"[6, 7, 8, 7, 8, 9]"#, actual.to_string());
  }

  #[test]
  fn test_for_expression_evaluator_empty_list_1() {
    let mut iterator = ForExpressionEvaluator::new();
    iterator.add_single("x".into(), Value::List(Values::default()));
    let scope = &te_scope(r#"{x:null}"#);
    let node = dmntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
    let evaluator = crate::builders::build_evaluator(&node).unwrap();
    let actual = iterator.evaluate(scope, &evaluator);
    assert_eq!(0, actual.len());
    assert_eq!(r#"[]"#, actual.to_string());
  }

  #[test]
  fn test_for_expression_evaluator_empty_list_2() {
    let mut iterator = ForExpressionEvaluator::new();
    iterator.add_range("x".into(), value_number!(1, 0), value_number!(2, 0));
    iterator.add_single("y".into(), Value::List(Values::default()));
    let scope = &te_scope(r#"{x:null,y:null}"#);
    let node = dmntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
    let evaluator = crate::builders::build_evaluator(&node).unwrap();
    let actual = iterator.evaluate(scope, &evaluator);
    assert_eq!(2, actual.len());
    assert_eq!(r#"[2, 3]"#, actual.to_string());
  }
}
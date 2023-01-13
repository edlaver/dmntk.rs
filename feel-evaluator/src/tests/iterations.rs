use crate::builders::BuilderContext;
use crate::iterations::{FeelIterator, ForExpressionEvaluator};
use crate::tests::te_scope;
use dmntk_feel::values::{Value, Values};
use dmntk_feel::{value_number, FeelNumber};

#[test]
fn _0001() {
  let mut iterator = FeelIterator::default();
  iterator.add_range("x".into(), 1, 3);
  let mut actual = vec![];
  iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(3, actual.len());
  assert_eq!(r#"[{x: 1}, {x: 2}, {x: 3}]"#, Values::new(actual).to_string());
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
    Values::new(actual).to_string()
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
    Values::new(actual).to_string()
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
    Values::new(actual).to_string()
  );
}

#[test]
fn _0005() {
  let mut iterator = FeelIterator::default();
  let list = Values::new(vec![Value::String("a".to_string()), Value::String("b".to_string()), Value::String("c".to_string())]);
  iterator.add_list("x".into(), list);
  let mut actual = vec![];
  iterator.run(|ctx| actual.push(Value::Context(ctx.clone())));
  assert_eq!(3, actual.len());
  assert_eq!(r#"[{x: "a"}, {x: "b"}, {x: "c"}]"#, Values::new(actual).to_string());
}

#[test]
fn _0006() {
  let mut iterator = FeelIterator::default();
  let list1 = Values::new(vec![Value::String("a".to_string()), Value::String("b".to_string()), Value::String("c".to_string())]);
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
fn _0007() {
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
fn _0008() {
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
fn _0009() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_range("x".into(), value_number!(1, 0), value_number!(3, 0));
  let scope = &te_scope(r#"{x:null}"#);
  let (node, _) = dmntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&mut BuilderContext::default(), &node).unwrap();
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(3, actual.len());
  assert_eq!(r#"[2, 3, 4]"#, actual.to_string());
}

#[test]
fn _0010() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_range("x".into(), value_number!(1, 0), value_number!(2, 0));
  iterator.add_single("y".into(), Value::List(Values::new(vec![value_number!(5, 0), value_number!(6, 0), value_number!(7, 0)])));
  let scope = &te_scope(r#"{x:null,y:null}"#);
  let (node, _) = dmntk_feel_parser::parse_expression(scope, "x+y", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&mut BuilderContext::default(), &node).unwrap();
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(6, actual.len());
  assert_eq!(r#"[6, 7, 8, 7, 8, 9]"#, actual.to_string());
}

#[test]
fn _0011() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_single("x".into(), Value::List(Values::default()));
  let scope = &te_scope(r#"{x:null}"#);
  let (node, _) = dmntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&mut BuilderContext::default(), &node).unwrap();
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(0, actual.len());
  assert_eq!(r#"[]"#, actual.to_string());
}

#[test]
fn _0012() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_single("x".into(), value_number!(1, 0));
  let scope = &te_scope(r#"{x:null}"#);
  let (node, _) = dmntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&mut BuilderContext::default(), &node).unwrap();
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(1, actual.len());
  assert_eq!(r#"[2]"#, actual.to_string());
}

#[test]
fn _0013() {
  let mut iterator = ForExpressionEvaluator::new();
  iterator.add_range("x".into(), value_number!(1, 0), value_number!(2, 0));
  iterator.add_single("y".into(), Value::List(Values::default()));
  let scope = &te_scope(r#"{x:null,y:null}"#);
  let (node, _) = dmntk_feel_parser::parse_expression(scope, "x+1", false).unwrap();
  let evaluator = crate::builders::build_evaluator(&mut BuilderContext::default(), &node).unwrap();
  let actual = iterator.evaluate(scope, &evaluator);
  assert_eq!(2, actual.len());
  assert_eq!(r#"[2, 3]"#, actual.to_string());
}

use crate::context::FeelContext;
use crate::values::Value;
use crate::{scope, value_number, FeelNumber, Name, Scope};
use dmntk_common::Jsonify;

#[test]
fn test_scope_default() {
  assert_eq!("[{}]", Scope::default().to_string());
  assert_eq!("[{}]", scope!().to_string());
  assert_eq!("Scope { contexts: RefCell { value: [FeelContext({})] } }", format!("{:?}", scope!()));
}

#[test]
fn test_scope_new() {
  assert_eq!("[]", Scope::new().to_string());
}

#[test]
fn test_scope_to_string() {
  let scope = Scope::default();
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  scope.set_entry(&name_a, value_number!(495, 1));
  scope.set_entry(&name_b, value_number!(50));
  assert_eq!("[{a: 49.5, b: 50}]", scope.to_string());
}

#[test]
fn test_scope_jsonify() {
  let scope = Scope::default();
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  scope.set_entry(&name_a, value_number!(495, 1));
  scope.set_entry(&name_b, value_number!(50));
  assert_eq!("[{a: 49.5, b: 50}]", scope.jsonify());
}

#[test]
fn test_scope_single_empty_context() {
  let scope = Scope::new();
  let ctx = FeelContext::default();
  scope.push(ctx);
  assert_eq!("[{}]", scope.to_string());
  let scope: Scope = FeelContext::default().into();
  assert_eq!("[{}]", scope.to_string());
}

#[test]
fn test_scope_multiple_empty_contexts() {
  let scope = Scope::new();
  scope.push(FeelContext::default());
  scope.push(FeelContext::default());
  scope.push(FeelContext::default());
  assert_eq!("[{}, {}, {}]", scope.to_string());
  let scope = Scope::default();
  scope.push(FeelContext::default());
  scope.push(FeelContext::default());
  scope.push(FeelContext::default());
  assert_eq!("[{}, {}, {}, {}]", scope.to_string());
}

#[test]
fn test_scope_single_context() {
  let scope = Scope::default();
  assert_eq!("[{}]", scope.to_string());
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  scope.set_entry(&name_a, value_number!(495, 1));
  assert_eq!("[{a: 49.5}]", scope.to_string());
  scope.set_entry(&name_b, value_number!(50));
  assert_eq!("[{a: 49.5, b: 50}]", scope.to_string());
  scope.pop();
  assert_eq!("[]", scope.to_string());
}

#[test]
fn test_scope_no_context() {
  let scope = Scope::new();
  assert_eq!("[]", scope.to_string());
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  scope.set_entry(&name_a, value_number!(125, 2));
  assert_eq!("[]", scope.to_string());
  scope.set_entry(&name_b, value_number!(175, 2));
  assert_eq!("[]", scope.to_string());
  scope.pop();
  assert_eq!("[]", scope.to_string());
}

#[test]
fn test_scope_push() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, value_number!(1));
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&name_b, value_number!(2));
  let mut ctx_c: FeelContext = Default::default();
  ctx_c.set_entry(&name_c, value_number!(3));
  let scope = Scope::new();
  scope.push(ctx_a);
  scope.push(ctx_b);
  scope.push(ctx_c);
  assert_eq!("[{a: 1}, {b: 2}, {c: 3}]", scope.to_string());
}

#[test]
fn test_scope_pop() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, value_number!(1));
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&name_b, value_number!(2));
  let mut ctx_c: FeelContext = Default::default();
  ctx_c.set_entry(&name_c, value_number!(3));
  let scope = Scope::new();
  scope.push(ctx_a);
  scope.push(ctx_b);
  scope.push(ctx_c);
  scope.pop();
  assert_eq!("[{a: 1}, {b: 2}]", scope.to_string());
}

#[test]
fn test_scope_peek() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, value_number!(1));
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&name_b, value_number!(2));
  let mut ctx_c: FeelContext = Default::default();
  ctx_c.set_entry(&name_c, value_number!(3));
  let scope = Scope::new();
  scope.push(ctx_a);
  scope.push(ctx_b);
  scope.push(ctx_c);
  let ctx = scope.peek();
  assert_eq!("{c: 3}", ctx.to_string());
  assert_eq!("[{a: 1}, {b: 2}, {c: 3}]", scope.to_string());
}

#[test]
fn test_flatten_keys() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, value_number!(1));
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&name_b, value_number!(2));
  let mut ctx_c: FeelContext = Default::default();
  ctx_c.set_entry(&name_c, value_number!(3));
  let scope = Scope::new();
  scope.push(ctx_a);
  scope.push(ctx_b);
  scope.push(ctx_c);
  let keys = scope.flatten_keys();
  assert_eq!(3, keys.len());
  assert!(keys.contains("a"));
  assert!(keys.contains("b"));
  assert!(keys.contains("c"));
}

#[test]
fn test_get_entry() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let name_d = Name::from("d");
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, value_number!(1));
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&name_b, value_number!(2));
  let mut ctx_c: FeelContext = Default::default();
  ctx_c.set_entry(&name_c, value_number!(3));
  let scope = Scope::new();
  scope.push(ctx_a);
  scope.push(ctx_b);
  scope.push(ctx_c);
  assert_eq!(value_number!(1), scope.get_entry(&name_a).unwrap());
  assert_eq!(None, scope.get_entry(&name_d));
}

#[test]
fn test_insert_null() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let name_d = Name::from("d");
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&name_a, value_number!(1));
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&name_b, value_number!(2));
  let mut ctx_c: FeelContext = Default::default();
  ctx_c.set_entry(&name_c, value_number!(3));
  let scope = Scope::new();
  scope.push(ctx_a);
  scope.push(ctx_b);
  scope.push(ctx_c);
  scope.insert_null(name_b);
  scope.insert_null(name_c);
  scope.insert_null(name_d);
  assert_eq!("[{a: 1}, {b: 2}, {b: null, c: null, d: null}]", scope.to_string());
}

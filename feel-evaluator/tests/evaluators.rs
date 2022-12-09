use dmntk_feel::values::Value;
use dmntk_feel::{AstNode, FeelNumber, FeelType, Scope};
use dmntk_feel_evaluator::{evaluate, evaluate_context, evaluate_context_node, evaluate_max, evaluate_min, evaluate_node_type, evaluate_sum, prepare};

#[test]
fn _0001() {
  let scope = Scope::default();
  assert_eq!("{}", evaluate_context(&scope, r#"{}"#).unwrap().to_string());
}

#[test]
fn _0002() {
  let scope = Scope::default();
  let node = AstNode::Context(vec![AstNode::ContextEntry(
    Box::new(AstNode::ContextEntryKey("alpha".into())),
    Box::new(AstNode::Boolean(true)),
  )]);
  assert_eq!("{alpha: true}", evaluate_context_node(&scope, &node).unwrap().to_string());
}

#[test]
fn _0003() {
  let scope = Scope::default();
  let node = AstNode::Boolean(true);
  assert_eq!(
    "FeelEvaluatorError: expected FEEL context as an input",
    evaluate_context_node(&scope, &node).err().unwrap().to_string()
  );
}

#[test]
fn _0004() {
  let scope = Scope::default();
  let node = AstNode::Boolean(true);
  assert_eq!(FeelType::Boolean, evaluate_node_type(&scope, &node));
}

#[test]
fn _0005() {
  let scope = Scope::default();
  let node = AstNode::Add(
    Box::new(AstNode::Numeric("1".to_string(), "23".to_string())),
    Box::new(AstNode::Numeric("1".to_string(), "77".to_string())),
  );
  let evaluator = prepare(&node).unwrap();
  let value = evaluator(&scope) as Value;
  assert_eq!("3", value.to_string());
}

#[test]
fn _0006() {
  let value = evaluate_sum(vec![Value::Number(FeelNumber::new(123, 2)), Value::Number(FeelNumber::new(177, 2))]);
  assert_eq!("3", value.to_string());
}

#[test]
fn _0007() {
  let value = evaluate_min(vec![Value::Number(FeelNumber::new(123, 2)), Value::Number(FeelNumber::new(177, 2))]);
  assert_eq!("1.23", value.to_string());
}

#[test]
fn _0008() {
  let value = evaluate_max(vec![Value::Number(FeelNumber::new(123, 2)), Value::Number(FeelNumber::new(177, 2))]);
  assert_eq!("1.77", value.to_string());
}

#[test]
fn _0009() {
  let scope = Scope::default();
  let node = AstNode::FunctionInvocation(Box::new(AstNode::Name("calculate".into())), Box::new(AstNode::Boolean(true)));
  assert_eq!(
    "FeelEvaluatorError: expected positional or named parameter",
    evaluate(&scope, &node).err().unwrap().to_string()
  );
}

#[test]
fn _0010() {
  let scope = Scope::default();
  let node = AstNode::FunctionInvocation(
    Box::new(AstNode::Name("calculate".into())),
    Box::new(AstNode::NamedParameters(vec![AstNode::NamedParameter(
      Box::new(AstNode::Boolean(true)),
      Box::new(AstNode::Boolean(false)),
    )])),
  );
  assert_eq!(
    "FeelEvaluatorError: expected AstNode::ParameterName, actual node is Boolean(true)",
    evaluate(&scope, &node).err().unwrap().to_string()
  );
}

#[test]
fn _0011() {
  let scope = Scope::default();
  let node = AstNode::Every(Box::new(AstNode::Boolean(true)), Box::new(AstNode::Boolean(false)));
  assert_eq!(
    "FeelEvaluatorError: expected AST node AstNode::QuantifiedContexts, actual AST node is Boolean(true)",
    evaluate(&scope, &node).err().unwrap().to_string()
  );
}

#[test]
fn _0012() {
  let scope = Scope::default();
  let node = AstNode::Some(Box::new(AstNode::Boolean(true)), Box::new(AstNode::Boolean(false)));
  assert_eq!(
    "FeelEvaluatorError: expected AST node AstNode::QuantifiedContexts, actual AST node is Boolean(true)",
    evaluate(&scope, &node).err().unwrap().to_string()
  );
}

#[test]
fn _0013() {
  let scope = Scope::default();
  let node = AstNode::CommaList(vec![]);
  assert_eq!(
    "FeelEvaluatorError: unexpected AST node in evaluator builder CommaList([])",
    evaluate(&scope, &node).err().unwrap().to_string()
  );
}

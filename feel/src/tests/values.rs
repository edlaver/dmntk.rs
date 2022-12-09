use crate::bif::Bif;
use crate::context::FeelContext;
use crate::function::FunctionBody;
use crate::values::{Value, Values};
use crate::{
  value_number, FeelDate, FeelDateTime, FeelDaysAndTimeDuration, FeelNumber, FeelTime, FeelType, FeelYearsAndMonthsDuration, Name, Scope, ToFeelString,
};
use dmntk_common::Jsonify;
use std::collections::BTreeMap;
use std::sync::Arc;

macro_rules! eq_dbg {
  ($expected:literal, $value: expr) => {
    assert_eq!($expected, format!("{:?}", $value));
  };
}

macro_rules! eq_dsp {
  ($expected:literal, $value: expr) => {
    assert_eq!($expected, format!("{}", $value));
  };
}

macro_rules! eq_typ {
  ($expected:expr, $value: expr) => {
    assert_eq!($expected, $value.type_of());
  };
}

#[test]
fn test_debug() {
  let name = Name::from("a");
  let t_number = FeelType::Number;
  let v_number = Value::Number(FeelNumber::new(1, 0));
  let b_number = Box::new(v_number);
  let v_date = FeelDate::new(2022, 9, 27);
  let v_time = FeelTime::new_hms_opt(12, 13, 23, 0).unwrap();
  let v_date_time = FeelDateTime::new(v_date.clone(), v_time.clone());
  let v_function_body = FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &Scope| value_number!(2))));
  let v_days_and_time_duration = FeelDaysAndTimeDuration::from_s(100);
  let v_years_and_months_duration = FeelYearsAndMonthsDuration::new_ym(3, 2);
  eq_dbg!(r#"Boolean(false)"#, Value::Boolean(false));
  eq_dbg!(r#"BuiltInFunction(Time)"#, Value::BuiltInFunction(Bif::Time));
  eq_dbg!(r#"ExpressionList(Values([]))"#, Value::ExpressionList(Values::default()));
  eq_dbg!(r#"Context(FeelContext({}))"#, Value::Context(FeelContext::default()));
  eq_dbg!(r#"ContextEntry(Name("a"), Number(+1E+0))"#, Value::ContextEntry(name.clone(), b_number.clone()));
  eq_dbg!(r#"ContextEntryKey(Name("a"))"#, Value::ContextEntryKey(name.clone()));
  eq_dbg!(r#"ContextType(Number)"#, Value::ContextType(t_number.clone()));
  eq_dbg!(
    r#"ContextTypeEntry(Name("a"), Number)"#,
    Value::ContextTypeEntry(name.clone(), t_number.clone())
  );
  eq_dbg!(r#"ContextTypeEntryKey(Name("a"))"#, Value::ContextTypeEntryKey(name.clone()));
  eq_dbg!(r#"Date(FeelDate(2022, 9, 27))"#, Value::Date(v_date));
  eq_dbg!(
    r#"DateTime(FeelDateTime(FeelDate(2022, 9, 27), FeelTime(12, 13, 23, 0, Local)))"#,
    Value::DateTime(v_date_time)
  );
  eq_dbg!(
    r#"DaysAndTimeDuration(FeelDaysAndTimeDuration(100000000000))"#,
    Value::DaysAndTimeDuration(v_days_and_time_duration)
  );
  eq_dbg!(r#"FeelType(Number)"#, Value::FeelType(t_number.clone()));
  eq_dbg!(r#"FormalParameter(Name("a"), Number)"#, Value::FormalParameter(name.clone(), t_number.clone()));
  eq_dbg!(r#"FormalParameters([])"#, Value::FormalParameters(vec![]));
  eq_dbg!(r#"FunctionBody(FunctionBodyLiteralExpression)"#, Value::FunctionBody(v_function_body.clone()));
  eq_dbg!(
    r#"FunctionDefinition([(Name("a"), Number)], FunctionBodyLiteralExpression, Number)"#,
    Value::FunctionDefinition(vec![(name.clone(), t_number.clone())], v_function_body, t_number)
  );
  eq_dbg!(r#"IntervalEnd(Number(+1E+0), false)"#, Value::IntervalEnd(b_number.clone(), false));
  eq_dbg!(r#"IntervalStart(Number(+1E+0), false)"#, Value::IntervalStart(b_number.clone(), false));
  eq_dbg!(r#"Irrelevant"#, Value::Irrelevant);
  eq_dbg!(r#"List(Values([]))"#, Value::List(Values::default()));
  eq_dbg!(
    r#"NamedParameter(ParameterName(Name("a")), Number(+1E+0))"#,
    Value::NamedParameter(Box::new(Value::ParameterName(name.clone())), b_number.clone())
  );
  eq_dbg!(r#"NamedParameters({})"#, Value::NamedParameters(BTreeMap::new()));
  eq_dbg!(r#"NegatedCommaList(Values([]))"#, Value::NegatedCommaList(Values::default()));
  eq_dbg!(r#"Number(+1E+0)"#, Value::Number(FeelNumber::new(1, 0)));
  eq_dbg!(r#"Null(None)"#, Value::Null(None));
  eq_dbg!(r#"ParameterName(Name("a"))"#, Value::ParameterName(name.clone()));
  eq_dbg!(r#"ParameterTypes([])"#, Value::ParameterTypes(vec![]));
  eq_dbg!(r#"PositionalParameters(Values([]))"#, Value::PositionalParameters(Values::default()));
  eq_dbg!(r#"QualifiedNameSegment(Name("a"))"#, Value::QualifiedNameSegment(name));
  eq_dbg!(
    r#"Range(Number(+1E+0), false, Number(+1E+0), true)"#,
    Value::Range(b_number.clone(), false, b_number.clone(), true)
  );
  eq_dbg!(r#"String("beta")"#, Value::String("beta".to_string()));
  eq_dbg!(r#"Time(FeelTime(12, 13, 23, 0, Local))"#, Value::Time(v_time));
  eq_dbg!(r#"UnaryGreater(Number(+1E+0))"#, Value::UnaryGreater(b_number.clone()));
  eq_dbg!(r#"UnaryGreaterOrEqual(Number(+1E+0))"#, Value::UnaryGreaterOrEqual(b_number.clone()));
  eq_dbg!(r#"UnaryLess(Number(+1E+0))"#, Value::UnaryLess(b_number.clone()));
  eq_dbg!(r#"UnaryLessOrEqual(Number(+1E+0))"#, Value::UnaryLessOrEqual(b_number));
  eq_dbg!(
    r#"YearsAndMonthsDuration(FeelYearsAndMonthsDuration(38))"#,
    Value::YearsAndMonthsDuration(v_years_and_months_duration)
  );
}

#[test]
fn test_display() {
  let name = Name::from("a");
  let t_number = FeelType::Number;
  let v_number = Value::Number(FeelNumber::new(1, 0));
  let b_number = Box::new(v_number);
  let v_date = FeelDate::new(2022, 9, 27);
  let v_time = FeelTime::new_hms_opt(12, 13, 23, 0).unwrap();
  let v_date_time = FeelDateTime::new(v_date.clone(), v_time.clone());
  let v_function_body = FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &Scope| value_number!(2))));
  let v_days_and_time_duration = FeelDaysAndTimeDuration::from_s(100);
  let v_years_and_months_duration = FeelYearsAndMonthsDuration::new_ym(3, 2);
  eq_dsp!(r#"false"#, Value::Boolean(false));
  eq_dsp!(r#"BuiltInFunction"#, Value::BuiltInFunction(Bif::Time));
  eq_dsp!(r#"[]"#, Value::ExpressionList(Values::default()));
  eq_dsp!(r#"{}"#, Value::Context(FeelContext::default()));
  eq_dsp!(r#"ContextEntry"#, Value::ContextEntry(name.clone(), b_number.clone()));
  eq_dsp!(r#"a"#, Value::ContextEntryKey(name.clone()));
  eq_dsp!(r#"ContextType"#, Value::ContextType(t_number.clone()));
  eq_dsp!(r#"a: number"#, Value::ContextTypeEntry(name.clone(), t_number.clone()));
  eq_dsp!(r#"a"#, Value::ContextTypeEntryKey(name.clone()));
  eq_dsp!(r#"2022-09-27"#, Value::Date(v_date));
  eq_dsp!(r#"2022-09-27T12:13:23"#, Value::DateTime(v_date_time));
  eq_dsp!(r#"PT1M40S"#, Value::DaysAndTimeDuration(v_days_and_time_duration));
  eq_dsp!(r#"type(number)"#, Value::FeelType(t_number.clone()));
  eq_dsp!(r#"FormalParameter"#, Value::FormalParameter(name.clone(), t_number.clone()));
  eq_dsp!(r#"FormalParameters"#, Value::FormalParameters(vec![]));
  eq_dsp!(r#"FunctionBody"#, Value::FunctionBody(v_function_body.clone()));
  eq_dsp!(
    r#"FunctionDefinition"#,
    Value::FunctionDefinition(vec![(name.clone(), t_number.clone())], v_function_body, t_number)
  );
  eq_dsp!(r#"IntervalEnd"#, Value::IntervalEnd(b_number.clone(), false));
  eq_dsp!(r#"IntervalStart"#, Value::IntervalStart(b_number.clone(), false));
  eq_dsp!(r#"Irrelevant"#, Value::Irrelevant);
  eq_dsp!(r#"[]"#, Value::List(Values::default()));
  eq_dsp!(
    r#"NamedParameter"#,
    Value::NamedParameter(Box::new(Value::ParameterName(name.clone())), b_number.clone())
  );
  eq_dsp!(r#"NamedParameters"#, Value::NamedParameters(BTreeMap::new()));
  eq_dsp!(r#"NegatedCommaList"#, Value::NegatedCommaList(Values::default()));
  eq_dsp!(r#"1"#, Value::Number(FeelNumber::new(1, 0)));
  eq_dsp!(r#"null"#, Value::Null(None));
  eq_dsp!(r#"ParameterName"#, Value::ParameterName(name.clone()));
  eq_dsp!(r#"ParameterTypes"#, Value::ParameterTypes(vec![]));
  eq_dsp!(r#"PositionalParameters"#, Value::PositionalParameters(Values::default()));
  eq_dsp!(r#"QualifiedNameSegment"#, Value::QualifiedNameSegment(name));
  eq_dsp!(r#"(1..1]"#, Value::Range(b_number.clone(), false, b_number.clone(), true));
  eq_dsp!(r#""beta""#, Value::String("beta".to_string()));
  eq_dsp!(r#"12:13:23"#, Value::Time(v_time));
  eq_dsp!(r#"UnaryGreater"#, Value::UnaryGreater(b_number.clone()));
  eq_dsp!(r#"UnaryGreaterOrEqual"#, Value::UnaryGreaterOrEqual(b_number.clone()));
  eq_dsp!(r#"UnaryLess"#, Value::UnaryLess(b_number.clone()));
  eq_dsp!(r#"UnaryLessOrEqual(1)"#, Value::UnaryLessOrEqual(b_number));
  eq_dsp!(r#"P3Y2M"#, Value::YearsAndMonthsDuration(v_years_and_months_duration));
}

#[test]
fn test_type_of() {
  let name = Name::from("a");
  let t_number = FeelType::Number;
  let v_number = Value::Number(FeelNumber::new(1, 0));
  let b_number = Box::new(v_number.clone());
  let v_boolean = Value::Boolean(false);
  let b_boolean = Box::new(v_boolean.clone());
  let v_date = FeelDate::new(2022, 9, 27);
  let v_time = FeelTime::new_hms_opt(12, 13, 23, 0).unwrap();
  let v_date_time = FeelDateTime::new(v_date.clone(), v_time.clone());
  let v_function_body = FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &Scope| value_number!(2))));
  let v_days_and_time_duration = FeelDaysAndTimeDuration::from_s(100);
  let v_years_and_months_duration = FeelYearsAndMonthsDuration::new_ym(3, 2);
  eq_typ!(FeelType::Boolean, Value::Boolean(false));
  eq_typ!(FeelType::Any, Value::BuiltInFunction(Bif::Time));
  eq_typ!(FeelType::Any, Value::ExpressionList(Values::default()));
  eq_typ!(FeelType::Context(BTreeMap::new()), Value::Context(FeelContext::default()));
  eq_typ!(FeelType::Any, Value::ContextEntry(name.clone(), b_number.clone()));
  eq_typ!(FeelType::Any, Value::ContextEntryKey(name.clone()));
  eq_typ!(FeelType::Number, Value::ContextType(t_number.clone()));
  eq_typ!(FeelType::Number, Value::ContextTypeEntry(name.clone(), t_number.clone()));
  eq_typ!(FeelType::Any, Value::ContextTypeEntryKey(name.clone()));
  eq_typ!(FeelType::Date, Value::Date(v_date));
  eq_typ!(FeelType::DateTime, Value::DateTime(v_date_time));
  eq_typ!(FeelType::DaysAndTimeDuration, Value::DaysAndTimeDuration(v_days_and_time_duration));
  eq_typ!(FeelType::Number, Value::FeelType(t_number.clone()));
  eq_typ!(FeelType::Number, Value::FormalParameter(name.clone(), t_number.clone()));
  eq_typ!(FeelType::Any, Value::FormalParameters(vec![]));
  eq_typ!(FeelType::Any, Value::FunctionBody(v_function_body.clone()));
  eq_typ!(
    FeelType::Function(vec![FeelType::Number], Box::new(FeelType::Number)),
    Value::FunctionDefinition(vec![(name.clone(), t_number.clone())], v_function_body, t_number.clone())
  );
  eq_typ!(FeelType::Number, Value::IntervalEnd(b_number.clone(), false));
  eq_typ!(FeelType::Number, Value::IntervalStart(b_number.clone(), false));
  eq_typ!(FeelType::Any, Value::Irrelevant);
  eq_typ!(FeelType::List(Box::new(FeelType::Null)), Value::List(Values::default()));
  eq_typ!(FeelType::List(Box::new(t_number)), Value::List(Values::new(vec![v_number.clone()])));
  eq_typ!(FeelType::List(Box::new(FeelType::Any)), Value::List(Values::new(vec![v_number, v_boolean])));
  eq_typ!(
    FeelType::Any,
    Value::NamedParameter(Box::new(Value::ParameterName(name.clone())), b_number.clone())
  );
  eq_typ!(FeelType::Any, Value::NamedParameters(BTreeMap::new()));
  eq_typ!(FeelType::Any, Value::NegatedCommaList(Values::default()));
  eq_typ!(FeelType::Number, Value::Number(FeelNumber::new(1, 0)));
  eq_typ!(FeelType::Null, Value::Null(None));
  eq_typ!(FeelType::Any, Value::ParameterName(name.clone()));
  eq_typ!(FeelType::Any, Value::ParameterTypes(vec![]));
  eq_typ!(FeelType::Any, Value::PositionalParameters(Values::default()));
  eq_typ!(FeelType::Any, Value::QualifiedNameSegment(name));
  eq_typ!(
    FeelType::Range(Box::new(FeelType::Number)),
    Value::Range(b_number.clone(), false, b_number.clone(), true)
  );
  eq_typ!(FeelType::Range(Box::new(FeelType::Any)), Value::Range(b_number.clone(), false, b_boolean, true));
  eq_typ!(FeelType::String, Value::String("beta".to_string()));
  eq_typ!(FeelType::Time, Value::Time(v_time));
  eq_typ!(FeelType::Boolean, Value::UnaryGreater(b_number.clone()));
  eq_typ!(FeelType::Boolean, Value::UnaryGreaterOrEqual(b_number.clone()));
  eq_typ!(FeelType::Boolean, Value::UnaryLess(b_number.clone()));
  eq_typ!(FeelType::Boolean, Value::UnaryLessOrEqual(b_number));
  eq_typ!(FeelType::YearsAndMonthsDuration, Value::YearsAndMonthsDuration(v_years_and_months_duration));
}

#[test]
fn test_is_number() {
  assert!(value_number!(10).is_number());
  assert!(value_number!(10123, 3).is_number());
  assert!(!Value::Boolean(true).is_number());
  assert!(!Value::Boolean(false).is_number());
}

#[test]
fn test_is_null() {
  assert!(Value::Null(None).is_null());
  assert!(Value::Null(Some("message".to_string())).is_null());
  assert!(!value_number!(10).is_null());
  assert!(!value_number!(10123, 3).is_null());
  assert!(!Value::Boolean(true).is_null());
  assert!(!Value::Boolean(false).is_null());
}

#[test]
fn test_is_true() {
  assert!(Value::Boolean(true).is_true());
  assert!(!Value::Boolean(false).is_true());
  assert!(!Value::Null(None).is_true());
  assert!(!value_number!(10).is_true());
}

#[test]
fn test_comparison() {
  assert_eq!(value_number!(10), value_number!(10));
  assert_ne!(value_number!(10, 0), value_number!(10, 1));
  assert_eq!(Value::Boolean(true), Value::Boolean(true));
  assert_eq!(Value::Boolean(false), Value::Boolean(false));
  assert_ne!(Value::Boolean(true), Value::Boolean(false));
  assert_ne!(Value::Boolean(false), Value::Boolean(true));
  assert_eq!(Value::Context(FeelContext::default()), Value::Context(FeelContext::default()));
  let fun_body_a = FunctionBody::Context(Arc::new(Box::new(|_: &Scope| value_number!(1))));
  let fun_body_b = FunctionBody::Context(Arc::new(Box::new(|_: &Scope| value_number!(2))));
  let fun_body_c = FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &Scope| value_number!(3))));
  let fun_body_d = FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &Scope| value_number!(4))));
  let fun_body_e = FunctionBody::DecisionTable(Arc::new(Box::new(|_: &Scope| value_number!(4))));
  let fun_body_f = FunctionBody::DecisionTable(Arc::new(Box::new(|_: &Scope| value_number!(5))));
  let fun_body_g = FunctionBody::External(Arc::new(Box::new(|_: &Scope| value_number!(6))));
  let fun_body_h = FunctionBody::External(Arc::new(Box::new(|_: &Scope| value_number!(7))));
  assert_eq!(fun_body_a, fun_body_b);
  assert_ne!(fun_body_a, fun_body_c);
  assert_ne!(fun_body_a, fun_body_e);
  assert_ne!(fun_body_a, fun_body_g);
  assert_eq!(fun_body_c, fun_body_d);
  assert_ne!(fun_body_c, fun_body_a);
  assert_ne!(fun_body_c, fun_body_e);
  assert_ne!(fun_body_c, fun_body_g);
  assert_eq!(fun_body_e, fun_body_f);
  assert_ne!(fun_body_e, fun_body_a);
  assert_ne!(fun_body_e, fun_body_c);
  assert_ne!(fun_body_e, fun_body_g);
  assert_eq!(fun_body_g, fun_body_h);
  assert_ne!(fun_body_g, fun_body_a);
  assert_ne!(fun_body_g, fun_body_c);
  assert_ne!(fun_body_g, fun_body_e);
}

#[test]
fn test_value_to_feel_string() {
  assert_eq!(r#""foo""#, Value::String("foo".to_string()).to_feel_string());
  assert_eq!(r#""\"bar\"""#, Value::String("\"bar\"".to_string()).to_feel_string());
  assert_eq!(r#"{}"#, Value::Context(FeelContext::default()).to_feel_string());
  assert_eq!(r#"[]"#, Value::List(Values::default()).to_feel_string());
  assert_eq!(r#"true"#, Value::Boolean(true).to_feel_string());
}

#[test]
fn test_jsonify() {
  assert_eq!(r#"true"#, Value::Boolean(true).jsonify());
  assert_eq!(r#"[]"#, Value::ExpressionList(Values::default()).jsonify());
  assert_eq!(r#"{}"#, Value::Context(FeelContext::default()).jsonify());
  assert_eq!(r#"alpha"#, Value::ContextEntryKey("alpha".into()).jsonify());
  assert_eq!(r#"[]"#, Value::List(Values::default()).jsonify());
  assert_eq!(r#"1.23"#, Value::Number(FeelNumber::new(123, 2)).jsonify());
  assert_eq!(r#"null"#, Value::Null(None).jsonify());
  assert_eq!(r#""beta""#, Value::String("beta".to_string()).jsonify());
  assert_eq!(
    r#"jsonify not implemented for: P3Y2M"#,
    Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::new_ym(3, 2)).jsonify()
  );
}

#[test]
fn test_try_from() {
  // xsd::integer
  assert!(Value::try_from_xsd_integer("1").is_ok());
  assert!(Value::try_from_xsd_integer("1a").is_err());
  assert_eq!(
    "ValueError: '1a' is not valid xsd:integer representation",
    Value::try_from_xsd_integer("1a").err().unwrap().to_string()
  );
  // xsd::decimal
  assert!(Value::try_from_xsd_decimal("1").is_ok());
  assert!(Value::try_from_xsd_decimal("1a").is_err());
  assert_eq!(
    "ValueError: '1a' is not valid xsd:decimal representation",
    Value::try_from_xsd_decimal("1a").err().unwrap().to_string()
  );
  // xsd::double
  assert!(Value::try_from_xsd_double("1.2").is_ok());
  assert!(Value::try_from_xsd_double("1.2a").is_err());
  assert_eq!(
    "ValueError: '1.2a' is not valid xsd:double representation",
    Value::try_from_xsd_double("1.2a").err().unwrap().to_string()
  );
  // xsd::boolean
  assert!(Value::try_from_xsd_boolean("true").is_ok());
  assert!(Value::try_from_xsd_boolean("false").is_ok());
  assert!(Value::try_from_xsd_boolean("1").is_ok());
  assert!(Value::try_from_xsd_boolean("0").is_ok());
  assert!(Value::try_from_xsd_boolean("5").is_err());
  assert!(Value::try_from_xsd_boolean("False").is_err());
  assert!(Value::try_from_xsd_boolean("True").is_err());
  assert_eq!(
    "ValueError: 'TRUE' is not valid xsd:boolean representation",
    Value::try_from_xsd_boolean("TRUE").err().unwrap().to_string()
  );
  // xsd::date
  assert!(Value::try_from_xsd_date("2022-09-27").is_ok());
  assert!(Value::try_from_xsd_date("2022-02-31").is_err());
  assert_eq!(
    "ValueError: '2022-02-31' is not valid xsd:date representation",
    Value::try_from_xsd_date("2022-02-31").err().unwrap().to_string()
  );
  // xsd::time
  assert!(Value::try_from_xsd_time("13:29:23").is_ok());
  assert!(Value::try_from_xsd_time("26:12:34").is_err());
  assert_eq!(
    "ValueError: '26:12:34' is not valid xsd:time representation",
    Value::try_from_xsd_time("26:12:34").err().unwrap().to_string()
  );
  // xsd::dateTime
  assert!(Value::try_from_xsd_date_time("2022-09-27T13:29:23").is_ok());
  assert!(Value::try_from_xsd_date_time("2022-02-31T23:12:34").is_err());
  assert_eq!(
    "ValueError: '2022-02-31T23:12:34' is not valid xsd:dateTime representation",
    Value::try_from_xsd_date_time("2022-02-31T23:12:34").err().unwrap().to_string()
  );
  // xsd::duration
  assert!(Value::try_from_xsd_duration("P1Y2M").is_ok());
  assert!(Value::try_from_xsd_duration("P2DT2H").is_ok());
  assert!(Value::try_from_xsd_duration("PYD").is_err());
  assert_eq!(
    "ValueError: 'PYD' is not valid xsd:duration representation",
    Value::try_from_xsd_duration("PYD").err().unwrap().to_string()
  );
}

#[test]
fn test_values_new() {
  assert_eq!("Values([])", format!("{:?}", Values::new(vec![])));
  assert_eq!("Values([])", format!("{:?}", Values::default()));
  assert!(Values::default().is_empty());
}

#[test]
fn test_values_add() {
  let mut v = Values::default();
  v.add(Value::Boolean(true));
  assert_eq!("Values([Boolean(true)])", format!("{v:?}"));
}

#[test]
fn test_values_insert() {
  let mut v = Values::default();
  v.add(Value::Boolean(true));
  v.insert(0, Value::Boolean(false));
  assert_eq!("Values([Boolean(false), Boolean(true)])", format!("{v:?}"));
}

#[test]
fn test_values_reverse() {
  let mut v = Values::default();
  v.add(Value::Boolean(true));
  v.insert(0, Value::Boolean(false));
  v.reverse();
  assert_eq!("Values([Boolean(true), Boolean(false)])", format!("{v:?}"));
}

#[test]
fn test_values_remove() {
  let mut v = Values::default();
  v.add(Value::Boolean(true));
  v.add(Value::Boolean(false));
  assert_eq!("Values([Boolean(true), Boolean(false)])", format!("{v:?}"));
  v.remove(0);
  assert_eq!("Values([Boolean(false)])", format!("{v:?}"));
  v.add(Value::Boolean(true));
  v.remove(1);
  assert_eq!("Values([Boolean(false)])", format!("{v:?}"));
}

#[test]
fn test_values_to_feel_string() {
  let mut v = Values::default();
  v.add(Value::Boolean(true));
  v.add(Value::Boolean(false));
  assert_eq!("[true, false]", v.to_feel_string());
}

#[test]
fn test_values_jsonify() {
  let mut v = Values::default();
  v.add(Value::Boolean(true));
  v.add(Value::Boolean(false));
  assert_eq!("[true, false]", v.jsonify());
}

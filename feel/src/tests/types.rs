use crate::closure::Closure;
use crate::context::FeelContext;
use crate::names::Name;
use crate::types::{is_built_in_type_name, FeelType};
use crate::values::{Value, Values};
use crate::{value_null, value_number, FeelNumber, FeelScope, FunctionBody};
use dmntk_feel_temporal::{FeelDate, FeelDateTime, FeelDaysAndTimeDuration, FeelTime, FeelYearsAndMonthsDuration};
use std::sync::Arc;

#[test]
fn test_get_value_checked() {
  let t_any = FeelType::Any;
  let t_boolean = FeelType::Boolean;
  let v_null = Value::Null(None);
  let v_boolean = Value::Boolean(true);
  let v_string = Value::String("hello".to_owned());
  assert_eq!("null", t_any.get_value_checked(&v_null).unwrap().to_string());
  assert_eq!("true", t_any.get_value_checked(&v_boolean).unwrap().to_string());
  assert!(t_boolean.get_value_checked(&v_string).is_err());
  assert_eq!(
    r#"TypesError: invalid value for retrieving with type check, type = 'boolean', value = '"hello"'"#,
    format!("{}", t_boolean.get_value_checked(&v_string).err().unwrap()).as_str()
  );
}

#[test]
fn test_type_equivalence() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let t_any = FeelType::Any;
  let t_boolean = FeelType::Boolean;
  let t_date = FeelType::Date;
  let t_date_time = FeelType::DateTime;
  let t_days_and_time_duration = FeelType::DaysAndTimeDuration;
  let t_null = FeelType::Null;
  let t_number = FeelType::Number;
  let t_string = FeelType::String;
  let t_time = FeelType::Time;
  let t_years_and_months_duration = FeelType::YearsAndMonthsDuration;
  let t_list_a = FeelType::list(&t_boolean);
  let t_list_b = FeelType::list(&t_number);
  let t_context_a = FeelType::context(&[(&name_a, &t_number)]);
  let t_context_b = FeelType::context(&[(&name_b, &t_boolean)]);
  let t_context_a_b = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean)]);
  let t_context_a_b_c = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean), (&name_c, &t_string)]);
  let t_function_a = FeelType::function(&[FeelType::Number, FeelType::Number], &t_number);
  let t_function_b = FeelType::function(&[FeelType::Number, FeelType::Number], &t_boolean);
  let t_function_c = FeelType::function(&[FeelType::Number], &t_string);
  let t_range_a = FeelType::range(&t_number);
  let t_range_b = FeelType::range(&t_date);
  // any
  assert!(t_any.is_equivalent(&t_any));
  assert!(!t_any.is_equivalent(&t_boolean));
  assert!(!t_any.is_equivalent(&t_context_a));
  assert!(!t_any.is_equivalent(&t_date));
  assert!(!t_any.is_equivalent(&t_date_time));
  assert!(!t_any.is_equivalent(&t_days_and_time_duration));
  assert!(!t_boolean.is_equivalent(&t_function_a));
  assert!(!t_boolean.is_equivalent(&t_list_a));
  assert!(!t_boolean.is_equivalent(&t_null));
  assert!(!t_boolean.is_equivalent(&t_number));
  assert!(!t_boolean.is_equivalent(&t_range_a));
  assert!(!t_boolean.is_equivalent(&t_string));
  assert!(!t_boolean.is_equivalent(&t_time));
  assert!(!t_boolean.is_equivalent(&t_years_and_months_duration));
  // boolean
  assert!(t_boolean.is_equivalent(&t_boolean));
  assert!(!t_boolean.is_equivalent(&t_any));
  assert!(!t_boolean.is_equivalent(&t_context_a));
  assert!(!t_boolean.is_equivalent(&t_date));
  assert!(!t_boolean.is_equivalent(&t_date_time));
  assert!(!t_boolean.is_equivalent(&t_days_and_time_duration));
  assert!(!t_boolean.is_equivalent(&t_function_a));
  assert!(!t_boolean.is_equivalent(&t_list_a));
  assert!(!t_boolean.is_equivalent(&t_null));
  assert!(!t_boolean.is_equivalent(&t_number));
  assert!(!t_boolean.is_equivalent(&t_range_a));
  assert!(!t_boolean.is_equivalent(&t_string));
  assert!(!t_boolean.is_equivalent(&t_time));
  assert!(!t_boolean.is_equivalent(&t_years_and_months_duration));
  // context
  assert!(t_context_a.is_equivalent(&t_context_a));
  assert!(t_context_a_b.is_equivalent(&t_context_a_b));
  assert!(t_context_a_b_c.is_equivalent(&t_context_a_b_c));
  assert!(!t_context_a.is_equivalent(&t_context_b));
  assert!(!t_context_a.is_equivalent(&t_context_a_b));
  assert!(!t_context_a_b.is_equivalent(&t_context_a));
  assert!(!t_context_a_b.is_equivalent(&t_context_a_b_c));
  assert!(!t_context_a_b.is_equivalent(&t_context_a_b_c));
  assert!(!t_context_a.is_equivalent(&t_any));
  assert!(!t_context_a.is_equivalent(&t_boolean));
  assert!(!t_context_a.is_equivalent(&t_date));
  assert!(!t_context_a.is_equivalent(&t_date_time));
  assert!(!t_context_a.is_equivalent(&t_days_and_time_duration));
  assert!(!t_context_a.is_equivalent(&t_function_a));
  assert!(!t_context_a.is_equivalent(&t_list_a));
  assert!(!t_context_a.is_equivalent(&t_null));
  assert!(!t_context_a.is_equivalent(&t_number));
  assert!(!t_context_a.is_equivalent(&t_range_a));
  assert!(!t_context_a.is_equivalent(&t_string));
  assert!(!t_context_a.is_equivalent(&t_time));
  assert!(!t_context_a.is_equivalent(&t_years_and_months_duration));
  // date
  assert!(t_date.is_equivalent(&t_date));
  assert!(!t_date.is_equivalent(&t_any));
  assert!(!t_date.is_equivalent(&t_boolean));
  assert!(!t_date.is_equivalent(&t_context_a));
  assert!(!t_date.is_equivalent(&t_date_time));
  assert!(!t_date.is_equivalent(&t_days_and_time_duration));
  assert!(!t_date.is_equivalent(&t_function_a));
  assert!(!t_date.is_equivalent(&t_list_a));
  assert!(!t_date.is_equivalent(&t_null));
  assert!(!t_date.is_equivalent(&t_number));
  assert!(!t_date.is_equivalent(&t_range_a));
  assert!(!t_date.is_equivalent(&t_string));
  assert!(!t_date.is_equivalent(&t_time));
  assert!(!t_date.is_equivalent(&t_years_and_months_duration));
  // date and time
  assert!(t_date_time.is_equivalent(&t_date_time));
  assert!(!t_date_time.is_equivalent(&t_any));
  assert!(!t_date_time.is_equivalent(&t_boolean));
  assert!(!t_date_time.is_equivalent(&t_context_a));
  assert!(!t_date_time.is_equivalent(&t_date));
  assert!(!t_date_time.is_equivalent(&t_function_a));
  assert!(!t_date_time.is_equivalent(&t_list_a));
  assert!(!t_date_time.is_equivalent(&t_null));
  assert!(!t_date_time.is_equivalent(&t_number));
  assert!(!t_date_time.is_equivalent(&t_range_a));
  assert!(!t_date_time.is_equivalent(&t_string));
  assert!(!t_date_time.is_equivalent(&t_time));
  assert!(!t_date_time.is_equivalent(&t_years_and_months_duration));
  // days and time duration
  assert!(t_days_and_time_duration.is_equivalent(&t_days_and_time_duration));
  assert!(!t_days_and_time_duration.is_equivalent(&t_any));
  assert!(!t_days_and_time_duration.is_equivalent(&t_boolean));
  assert!(!t_days_and_time_duration.is_equivalent(&t_context_a));
  assert!(!t_days_and_time_duration.is_equivalent(&t_date));
  assert!(!t_days_and_time_duration.is_equivalent(&t_date_time));
  assert!(!t_days_and_time_duration.is_equivalent(&t_function_a));
  assert!(!t_days_and_time_duration.is_equivalent(&t_list_a));
  assert!(!t_days_and_time_duration.is_equivalent(&t_null));
  assert!(!t_days_and_time_duration.is_equivalent(&t_number));
  assert!(!t_days_and_time_duration.is_equivalent(&t_range_a));
  assert!(!t_days_and_time_duration.is_equivalent(&t_string));
  assert!(!t_days_and_time_duration.is_equivalent(&t_time));
  assert!(!t_days_and_time_duration.is_equivalent(&t_years_and_months_duration));
  // function
  assert!(t_function_a.is_equivalent(&t_function_a));
  assert!(t_function_b.is_equivalent(&t_function_b));
  assert!(t_function_c.is_equivalent(&t_function_c));
  assert!(!t_function_a.is_equivalent(&t_function_b));
  assert!(!t_function_a.is_equivalent(&t_function_c));
  assert!(!t_function_b.is_equivalent(&t_function_a));
  assert!(!t_function_b.is_equivalent(&t_function_c));
  assert!(!t_function_c.is_equivalent(&t_function_a));
  assert!(!t_function_c.is_equivalent(&t_function_b));
  assert!(!t_function_a.is_equivalent(&t_any));
  assert!(!t_function_a.is_equivalent(&t_boolean));
  assert!(!t_function_a.is_equivalent(&t_context_a));
  assert!(!t_function_a.is_equivalent(&t_date));
  assert!(!t_function_a.is_equivalent(&t_date_time));
  assert!(!t_function_a.is_equivalent(&t_days_and_time_duration));
  assert!(!t_function_a.is_equivalent(&t_list_a));
  assert!(!t_function_a.is_equivalent(&t_null));
  assert!(!t_function_a.is_equivalent(&t_range_a));
  assert!(!t_function_a.is_equivalent(&t_string));
  assert!(!t_function_a.is_equivalent(&t_time));
  assert!(!t_function_a.is_equivalent(&t_years_and_months_duration));
  // list
  assert!(t_list_b.is_equivalent(&t_list_b));
  assert!(!t_list_b.is_equivalent(&t_list_a));
  assert!(!t_list_a.is_equivalent(&t_any));
  assert!(!t_list_a.is_equivalent(&t_boolean));
  assert!(!t_list_a.is_equivalent(&t_context_a));
  assert!(!t_list_a.is_equivalent(&t_date));
  assert!(!t_list_a.is_equivalent(&t_date_time));
  assert!(!t_list_a.is_equivalent(&t_days_and_time_duration));
  assert!(!t_list_a.is_equivalent(&t_function_a));
  assert!(!t_list_a.is_equivalent(&t_null));
  assert!(!t_list_a.is_equivalent(&t_range_a));
  assert!(!t_list_a.is_equivalent(&t_string));
  assert!(!t_list_a.is_equivalent(&t_time));
  assert!(!t_list_a.is_equivalent(&t_years_and_months_duration));
  // null
  assert!(t_null.is_equivalent(&t_null));
  assert!(!t_null.is_equivalent(&t_any));
  assert!(!t_null.is_equivalent(&t_boolean));
  assert!(!t_null.is_equivalent(&t_context_a));
  assert!(!t_null.is_equivalent(&t_date));
  assert!(!t_null.is_equivalent(&t_date_time));
  assert!(!t_null.is_equivalent(&t_days_and_time_duration));
  assert!(!t_null.is_equivalent(&t_function_a));
  assert!(!t_null.is_equivalent(&t_list_a));
  assert!(!t_null.is_equivalent(&t_range_a));
  assert!(!t_null.is_equivalent(&t_string));
  assert!(!t_null.is_equivalent(&t_time));
  assert!(!t_null.is_equivalent(&t_years_and_months_duration));
  // number
  assert!(t_number.is_equivalent(&t_number));
  assert!(!t_number.is_equivalent(&t_any));
  assert!(!t_number.is_equivalent(&t_boolean));
  assert!(!t_number.is_equivalent(&t_context_a));
  assert!(!t_number.is_equivalent(&t_date));
  assert!(!t_number.is_equivalent(&t_date_time));
  assert!(!t_number.is_equivalent(&t_days_and_time_duration));
  assert!(!t_number.is_equivalent(&t_function_a));
  assert!(!t_number.is_equivalent(&t_list_a));
  assert!(!t_number.is_equivalent(&t_null));
  assert!(!t_number.is_equivalent(&t_range_a));
  assert!(!t_number.is_equivalent(&t_string));
  assert!(!t_number.is_equivalent(&t_time));
  assert!(!t_number.is_equivalent(&t_years_and_months_duration));
  // range
  assert!(t_range_a.is_equivalent(&t_range_a));
  assert!(t_range_b.is_equivalent(&t_range_b));
  assert!(!t_range_a.is_equivalent(&t_any));
  assert!(!t_range_a.is_equivalent(&t_boolean));
  assert!(!t_range_a.is_equivalent(&t_context_a));
  assert!(!t_range_a.is_equivalent(&t_date));
  assert!(!t_range_a.is_equivalent(&t_date_time));
  assert!(!t_range_a.is_equivalent(&t_days_and_time_duration));
  assert!(!t_range_a.is_equivalent(&t_function_a));
  assert!(!t_range_a.is_equivalent(&t_list_a));
  assert!(!t_range_a.is_equivalent(&t_null));
  assert!(!t_range_a.is_equivalent(&t_number));
  assert!(!t_range_a.is_equivalent(&t_string));
  assert!(!t_range_a.is_equivalent(&t_time));
  assert!(!t_range_a.is_equivalent(&t_years_and_months_duration));
  // string
  assert!(t_string.is_equivalent(&t_string));
  assert!(!t_string.is_equivalent(&t_any));
  assert!(!t_string.is_equivalent(&t_boolean));
  assert!(!t_string.is_equivalent(&t_context_a));
  assert!(!t_string.is_equivalent(&t_date));
  assert!(!t_string.is_equivalent(&t_date_time));
  assert!(!t_string.is_equivalent(&t_days_and_time_duration));
  assert!(!t_string.is_equivalent(&t_function_a));
  assert!(!t_string.is_equivalent(&t_list_a));
  assert!(!t_string.is_equivalent(&t_null));
  assert!(!t_string.is_equivalent(&t_number));
  assert!(!t_string.is_equivalent(&t_range_a));
  assert!(!t_string.is_equivalent(&t_time));
  assert!(!t_string.is_equivalent(&t_years_and_months_duration));
  // time
  assert!(t_time.is_equivalent(&t_time));
  assert!(!t_time.is_equivalent(&t_any));
  assert!(!t_time.is_equivalent(&t_boolean));
  assert!(!t_time.is_equivalent(&t_context_a));
  assert!(!t_time.is_equivalent(&t_date));
  assert!(!t_time.is_equivalent(&t_date_time));
  assert!(!t_time.is_equivalent(&t_days_and_time_duration));
  assert!(!t_time.is_equivalent(&t_function_a));
  assert!(!t_time.is_equivalent(&t_list_a));
  assert!(!t_time.is_equivalent(&t_null));
  assert!(!t_time.is_equivalent(&t_number));
  assert!(!t_time.is_equivalent(&t_range_a));
  assert!(!t_time.is_equivalent(&t_string));
  assert!(!t_time.is_equivalent(&t_years_and_months_duration));
  // years and months duration
  assert!(t_years_and_months_duration.is_equivalent(&t_years_and_months_duration));
  assert!(!t_years_and_months_duration.is_equivalent(&t_any));
  assert!(!t_years_and_months_duration.is_equivalent(&t_boolean));
  assert!(!t_years_and_months_duration.is_equivalent(&t_context_a));
  assert!(!t_years_and_months_duration.is_equivalent(&t_date));
  assert!(!t_years_and_months_duration.is_equivalent(&t_date_time));
  assert!(!t_years_and_months_duration.is_equivalent(&t_days_and_time_duration));
  assert!(!t_years_and_months_duration.is_equivalent(&t_function_a));
  assert!(!t_years_and_months_duration.is_equivalent(&t_list_a));
  assert!(!t_years_and_months_duration.is_equivalent(&t_null));
  assert!(!t_years_and_months_duration.is_equivalent(&t_number));
  assert!(!t_years_and_months_duration.is_equivalent(&t_range_a));
  assert!(!t_years_and_months_duration.is_equivalent(&t_string));
  assert!(!t_years_and_months_duration.is_equivalent(&t_time));
}

#[test]
fn test_type_conformance() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let t_any = FeelType::Any;
  let t_boolean = FeelType::Boolean;
  let t_date = FeelType::Date;
  let t_date_time = FeelType::DateTime;
  let t_days_and_time_duration = FeelType::DaysAndTimeDuration;
  let t_null = FeelType::Null;
  let t_number = FeelType::Number;
  let t_string = FeelType::String;
  let t_time = FeelType::Time;
  let t_years_and_months_duration = FeelType::YearsAndMonthsDuration;
  let t_list_a = FeelType::list(&t_boolean);
  let t_list_b = FeelType::list(&t_number);
  let t_context_a = FeelType::context(&[(&name_a, &t_number)]);
  let t_context_b = FeelType::context(&[(&name_a, &t_string)]);
  let t_context_a_b = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean)]);
  let t_context_a_b_c = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean), (&name_c, &t_string)]);
  let t_function = FeelType::function(&[FeelType::Number, FeelType::Number], &t_number);
  let t_function_b = FeelType::function(&[FeelType::Number, FeelType::Number], &t_boolean);
  let t_function_c = FeelType::function(&[FeelType::Number], &t_string);
  let t_function_d = FeelType::function(&[FeelType::Any], &t_string);
  let t_range_a = FeelType::range(&t_number);
  let t_range_b = FeelType::range(&t_date);
  // any
  assert!(t_any.is_conformant(&t_any));
  assert!(!t_any.is_conformant(&t_boolean));
  assert!(!t_any.is_conformant(&t_context_a));
  assert!(!t_any.is_conformant(&t_date));
  assert!(!t_any.is_conformant(&t_date_time));
  assert!(!t_any.is_conformant(&t_days_and_time_duration));
  // boolean
  assert!(!t_boolean.is_conformant(&t_function));
  assert!(!t_boolean.is_conformant(&t_list_a));
  assert!(!t_boolean.is_conformant(&t_null));
  assert!(!t_boolean.is_conformant(&t_number));
  assert!(!t_boolean.is_conformant(&t_range_a));
  assert!(!t_boolean.is_conformant(&t_string));
  assert!(!t_boolean.is_conformant(&t_time));
  assert!(!t_boolean.is_conformant(&t_years_and_months_duration));
  assert!(t_boolean.is_conformant(&t_boolean));
  assert!(t_boolean.is_conformant(&t_any));
  assert!(!t_boolean.is_conformant(&t_context_a));
  assert!(!t_boolean.is_conformant(&t_date));
  assert!(!t_boolean.is_conformant(&t_date_time));
  assert!(!t_boolean.is_conformant(&t_days_and_time_duration));
  assert!(!t_boolean.is_conformant(&t_function));
  assert!(!t_boolean.is_conformant(&t_list_a));
  assert!(!t_boolean.is_conformant(&t_null));
  assert!(!t_boolean.is_conformant(&t_number));
  assert!(!t_boolean.is_conformant(&t_range_a));
  assert!(!t_boolean.is_conformant(&t_string));
  assert!(!t_boolean.is_conformant(&t_time));
  assert!(!t_boolean.is_conformant(&t_years_and_months_duration));
  // context
  assert!(t_context_a.is_conformant(&t_context_a));
  assert!(!t_context_a.is_conformant(&t_context_b));
  assert!(t_context_a_b.is_conformant(&t_context_a_b));
  assert!(t_context_a_b.is_conformant(&t_context_a));
  assert!(t_context_a_b_c.is_conformant(&t_context_a_b_c));
  assert!(t_context_a_b_c.is_conformant(&t_context_a_b));
  assert!(!t_context_a.is_conformant(&t_context_a_b));
  assert!(!t_context_a_b.is_conformant(&t_context_a_b_c));
  assert!(!t_context_a_b.is_conformant(&t_context_a_b_c));
  assert!(t_context_a.is_conformant(&t_any));
  assert!(!t_context_a.is_conformant(&t_boolean));
  assert!(!t_context_a.is_conformant(&t_date));
  assert!(!t_context_a.is_conformant(&t_date_time));
  assert!(!t_context_a.is_conformant(&t_days_and_time_duration));
  assert!(!t_context_a.is_conformant(&t_function));
  assert!(!t_context_a.is_conformant(&t_list_a));
  assert!(!t_context_a.is_conformant(&t_null));
  assert!(!t_context_a.is_conformant(&t_number));
  assert!(!t_context_a.is_conformant(&t_range_a));
  assert!(!t_context_a.is_conformant(&t_string));
  assert!(!t_context_a.is_conformant(&t_time));
  assert!(!t_context_a.is_conformant(&t_years_and_months_duration));
  // date
  assert!(t_date.is_conformant(&t_date));
  assert!(t_date.is_conformant(&t_any));
  assert!(!t_date.is_conformant(&t_boolean));
  assert!(!t_date.is_conformant(&t_context_a));
  assert!(!t_date.is_conformant(&t_date_time));
  assert!(!t_date.is_conformant(&t_days_and_time_duration));
  assert!(!t_date.is_conformant(&t_function));
  assert!(!t_date.is_conformant(&t_list_a));
  assert!(!t_date.is_conformant(&t_null));
  assert!(!t_date.is_conformant(&t_number));
  assert!(!t_date.is_conformant(&t_range_a));
  assert!(!t_date.is_conformant(&t_string));
  assert!(!t_date.is_conformant(&t_time));
  assert!(!t_date.is_conformant(&t_years_and_months_duration));
  // date and time
  assert!(t_date_time.is_conformant(&t_date_time));
  assert!(t_date_time.is_conformant(&t_any));
  assert!(!t_date_time.is_conformant(&t_boolean));
  assert!(!t_date_time.is_conformant(&t_context_a));
  assert!(!t_date_time.is_conformant(&t_date));
  assert!(!t_date_time.is_conformant(&t_function));
  assert!(!t_date_time.is_conformant(&t_list_a));
  assert!(!t_date_time.is_conformant(&t_null));
  assert!(!t_date_time.is_conformant(&t_number));
  assert!(!t_date_time.is_conformant(&t_range_a));
  assert!(!t_date_time.is_conformant(&t_string));
  assert!(!t_date_time.is_conformant(&t_time));
  assert!(!t_date_time.is_conformant(&t_years_and_months_duration));
  // days and time duration
  assert!(t_days_and_time_duration.is_conformant(&t_days_and_time_duration));
  assert!(t_days_and_time_duration.is_conformant(&t_any));
  assert!(!t_days_and_time_duration.is_conformant(&t_boolean));
  assert!(!t_days_and_time_duration.is_conformant(&t_context_a));
  assert!(!t_days_and_time_duration.is_conformant(&t_date));
  assert!(!t_days_and_time_duration.is_conformant(&t_date_time));
  assert!(!t_days_and_time_duration.is_conformant(&t_function));
  assert!(!t_days_and_time_duration.is_conformant(&t_list_a));
  assert!(!t_days_and_time_duration.is_conformant(&t_null));
  assert!(!t_days_and_time_duration.is_conformant(&t_number));
  assert!(!t_days_and_time_duration.is_conformant(&t_range_a));
  assert!(!t_days_and_time_duration.is_conformant(&t_string));
  assert!(!t_days_and_time_duration.is_conformant(&t_time));
  assert!(!t_days_and_time_duration.is_conformant(&t_years_and_months_duration));
  // function
  assert!(t_function.is_conformant(&t_function));
  assert!(t_function_b.is_conformant(&t_function_b));
  assert!(t_function_c.is_conformant(&t_function_c));
  assert!(!t_function.is_conformant(&t_function_b));
  assert!(!t_function.is_conformant(&t_function_c));
  assert!(!t_function_b.is_conformant(&t_function));
  assert!(!t_function_b.is_conformant(&t_function_c));
  assert!(!t_function_c.is_conformant(&t_function));
  assert!(!t_function_c.is_conformant(&t_function_b));
  assert!(t_function_d.is_conformant(&t_function_c));
  assert!(t_function.is_conformant(&t_any));
  assert!(!t_function.is_conformant(&t_boolean));
  assert!(!t_function.is_conformant(&t_context_a));
  assert!(!t_function.is_conformant(&t_date));
  assert!(!t_function.is_conformant(&t_date_time));
  assert!(!t_function.is_conformant(&t_days_and_time_duration));
  assert!(!t_function.is_conformant(&t_list_a));
  assert!(!t_function.is_conformant(&t_null));
  assert!(!t_function.is_conformant(&t_range_a));
  assert!(!t_function.is_conformant(&t_string));
  assert!(!t_function.is_conformant(&t_time));
  assert!(!t_function.is_conformant(&t_years_and_months_duration));
  // list
  assert!(t_list_b.is_conformant(&t_list_b));
  assert!(!t_list_b.is_conformant(&t_list_a));
  assert!(t_list_a.is_conformant(&t_any));
  assert!(!t_list_a.is_conformant(&t_boolean));
  assert!(!t_list_a.is_conformant(&t_context_a));
  assert!(!t_list_a.is_conformant(&t_date));
  assert!(!t_list_a.is_conformant(&t_date_time));
  assert!(!t_list_a.is_conformant(&t_days_and_time_duration));
  assert!(!t_list_a.is_conformant(&t_function));
  assert!(!t_list_a.is_conformant(&t_null));
  assert!(!t_list_a.is_conformant(&t_range_a));
  assert!(!t_list_a.is_conformant(&t_string));
  assert!(!t_list_a.is_conformant(&t_time));
  assert!(!t_list_a.is_conformant(&t_years_and_months_duration));
  // null
  assert!(t_null.is_conformant(&t_null));
  assert!(t_null.is_conformant(&t_any));
  assert!(t_null.is_conformant(&t_boolean));
  assert!(t_null.is_conformant(&t_context_a));
  assert!(t_null.is_conformant(&t_date));
  assert!(t_null.is_conformant(&t_date_time));
  assert!(t_null.is_conformant(&t_days_and_time_duration));
  assert!(t_null.is_conformant(&t_function));
  assert!(t_null.is_conformant(&t_list_a));
  assert!(t_null.is_conformant(&t_range_a));
  assert!(t_null.is_conformant(&t_string));
  assert!(t_null.is_conformant(&t_time));
  assert!(t_null.is_conformant(&t_years_and_months_duration));
  // number
  assert!(t_number.is_conformant(&t_number));
  assert!(t_number.is_conformant(&t_any));
  assert!(!t_number.is_conformant(&t_boolean));
  assert!(!t_number.is_conformant(&t_context_a));
  assert!(!t_number.is_conformant(&t_date));
  assert!(!t_number.is_conformant(&t_date_time));
  assert!(!t_number.is_conformant(&t_days_and_time_duration));
  assert!(!t_number.is_conformant(&t_function));
  assert!(!t_number.is_conformant(&t_list_a));
  assert!(!t_number.is_conformant(&t_null));
  assert!(!t_number.is_conformant(&t_range_a));
  assert!(!t_number.is_conformant(&t_string));
  assert!(!t_number.is_conformant(&t_time));
  assert!(!t_number.is_conformant(&t_years_and_months_duration));
  // range
  assert!(t_range_a.is_conformant(&t_range_a));
  assert!(t_range_a.is_conformant(&t_any));
  assert!(!t_range_a.is_conformant(&t_range_b));
  assert!(t_range_b.is_conformant(&t_range_b));
  assert!(t_range_b.is_conformant(&t_any));
  assert!(!t_range_b.is_conformant(&t_range_a));
  assert!(!t_range_a.is_conformant(&t_boolean));
  assert!(!t_range_a.is_conformant(&t_context_a));
  assert!(!t_range_a.is_conformant(&t_date));
  assert!(!t_range_a.is_conformant(&t_date_time));
  assert!(!t_range_a.is_conformant(&t_days_and_time_duration));
  assert!(!t_range_a.is_conformant(&t_function));
  assert!(!t_range_a.is_conformant(&t_list_a));
  assert!(!t_range_a.is_conformant(&t_null));
  assert!(!t_range_a.is_conformant(&t_number));
  assert!(!t_range_a.is_conformant(&t_string));
  assert!(!t_range_a.is_conformant(&t_time));
  assert!(!t_range_a.is_conformant(&t_years_and_months_duration));
  // string
  assert!(t_string.is_conformant(&t_string));
  assert!(t_string.is_conformant(&t_any));
  assert!(!t_string.is_conformant(&t_boolean));
  assert!(!t_string.is_conformant(&t_context_a));
  assert!(!t_string.is_conformant(&t_date));
  assert!(!t_string.is_conformant(&t_date_time));
  assert!(!t_string.is_conformant(&t_days_and_time_duration));
  assert!(!t_string.is_conformant(&t_function));
  assert!(!t_string.is_conformant(&t_list_a));
  assert!(!t_string.is_conformant(&t_null));
  assert!(!t_string.is_conformant(&t_number));
  assert!(!t_string.is_conformant(&t_range_a));
  assert!(!t_string.is_conformant(&t_time));
  assert!(!t_string.is_conformant(&t_years_and_months_duration));
  // time
  assert!(t_time.is_conformant(&t_time));
  assert!(t_time.is_conformant(&t_any));
  assert!(!t_time.is_conformant(&t_boolean));
  assert!(!t_time.is_conformant(&t_context_a));
  assert!(!t_time.is_conformant(&t_date));
  assert!(!t_time.is_conformant(&t_date_time));
  assert!(!t_time.is_conformant(&t_days_and_time_duration));
  assert!(!t_time.is_conformant(&t_function));
  assert!(!t_time.is_conformant(&t_list_a));
  assert!(!t_time.is_conformant(&t_null));
  assert!(!t_time.is_conformant(&t_number));
  assert!(!t_time.is_conformant(&t_range_a));
  assert!(!t_time.is_conformant(&t_string));
  assert!(!t_time.is_conformant(&t_years_and_months_duration));
  // years and months duration
  assert!(t_years_and_months_duration.is_conformant(&t_years_and_months_duration));
  assert!(t_years_and_months_duration.is_conformant(&t_any));
  assert!(!t_years_and_months_duration.is_conformant(&t_boolean));
  assert!(!t_years_and_months_duration.is_conformant(&t_context_a));
  assert!(!t_years_and_months_duration.is_conformant(&t_date));
  assert!(!t_years_and_months_duration.is_conformant(&t_date_time));
  assert!(!t_years_and_months_duration.is_conformant(&t_days_and_time_duration));
  assert!(!t_years_and_months_duration.is_conformant(&t_function));
  assert!(!t_years_and_months_duration.is_conformant(&t_list_a));
  assert!(!t_years_and_months_duration.is_conformant(&t_null));
  assert!(!t_years_and_months_duration.is_conformant(&t_number));
  assert!(!t_years_and_months_duration.is_conformant(&t_range_a));
  assert!(!t_years_and_months_duration.is_conformant(&t_string));
  assert!(!t_years_and_months_duration.is_conformant(&t_time));
}

#[test]
fn test_is_built_in_type_name() {
  assert!(is_built_in_type_name("Any"));
  assert!(is_built_in_type_name("boolean"));
  assert!(is_built_in_type_name("date"));
  assert!(is_built_in_type_name("date and time"));
  assert!(is_built_in_type_name("days and time duration"));
  assert!(is_built_in_type_name("Null"));
  assert!(is_built_in_type_name("number"));
  assert!(is_built_in_type_name("string"));
  assert!(is_built_in_type_name("time"));
  assert!(is_built_in_type_name("years and months duration"));
  assert!(!is_built_in_type_name("context"));
  assert!(!is_built_in_type_name("function"));
  assert!(!is_built_in_type_name("list"));
  assert!(!is_built_in_type_name("range"));
}

#[test]
fn test_is_simple_built_in_type() {
  let name_a = Name::from("a");
  let t_boolean = FeelType::Boolean;
  let t_number = FeelType::Number;
  let t_string = FeelType::String;
  assert!(FeelType::Any.is_simple_built_in_type());
  assert!(FeelType::Boolean.is_simple_built_in_type());
  assert!(FeelType::Date.is_simple_built_in_type());
  assert!(FeelType::DateTime.is_simple_built_in_type());
  assert!(FeelType::DaysAndTimeDuration.is_simple_built_in_type());
  assert!(FeelType::Null.is_simple_built_in_type());
  assert!(FeelType::Number.is_simple_built_in_type());
  assert!(FeelType::String.is_simple_built_in_type());
  assert!(FeelType::Time.is_simple_built_in_type());
  assert!(FeelType::YearsAndMonthsDuration.is_simple_built_in_type());
  assert!(!FeelType::context(&[(&name_a, &t_number)]).is_simple_built_in_type());
  assert!(!FeelType::function(&[], &t_string).is_simple_built_in_type());
  assert!(!FeelType::list(&t_boolean).is_simple_built_in_type());
  assert!(!FeelType::range(&t_number).is_simple_built_in_type());
}

#[test]
fn test_type_stringify() {
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  let t_any = FeelType::Any;
  let t_null = FeelType::Null;
  let t_string = FeelType::String;
  let t_number = FeelType::Number;
  let t_boolean = FeelType::Boolean;
  let t_date = FeelType::Date;
  let t_time = FeelType::Time;
  let t_date_time = FeelType::DateTime;
  let t_dt_duration = FeelType::DaysAndTimeDuration;
  let t_ym_duration = FeelType::YearsAndMonthsDuration;
  let t_function_a = FeelType::function(&[t_number.clone(), t_number.clone()], &t_number);
  let t_function_b = FeelType::function(&[], &t_any);
  let t_function_c = FeelType::function(&[], &t_string);
  let t_list_a = FeelType::list(&t_boolean);
  let t_list_b = FeelType::list(&t_number);
  let t_range_a = FeelType::range(&t_number);
  let t_range_b = FeelType::range(&t_date);
  let t_context_a = FeelType::context(&[(&name_a, &t_number)]);
  let t_context_b = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean)]);
  let t_context_c = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean), (&name_c, &t_string)]);
  assert_eq!("Any", t_any.to_string());
  assert_eq!("Null", t_null.to_string());
  assert_eq!("number", t_number.to_string());
  assert_eq!("boolean", t_boolean.to_string());
  assert_eq!("date", t_date.to_string());
  assert_eq!("time", t_time.to_string());
  assert_eq!("date and time", t_date_time.to_string());
  assert_eq!("days and time duration", t_dt_duration.to_string());
  assert_eq!("years and months duration", t_ym_duration.to_string());
  assert_eq!("string", t_string.to_string());
  assert_eq!("function<number, number>->number", t_function_a.to_string());
  assert_eq!("function<>->Any", t_function_b.to_string());
  assert_eq!("function<>->string", t_function_c.to_string());
  assert_eq!("list<boolean>", t_list_a.to_string());
  assert_eq!("list<number>", t_list_b.to_string());
  assert_eq!("range<number>", t_range_a.to_string());
  assert_eq!("range<date>", t_range_b.to_string());
  assert_eq!("context<a: number>", t_context_a.to_string());
  assert_eq!("context<a: number, b: boolean>", t_context_b.to_string());
  assert_eq!("context<a: number, b: boolean, c: string>", t_context_c.to_string());
}

#[test]
fn test_get_coerced() {
  let t_any = FeelType::Any;
  let t_number = FeelType::Number;
  let t_string = FeelType::String;
  let t_list_number = FeelType::List(Box::new(t_number.clone()));
  let t_list_string = FeelType::List(Box::new(t_string.clone()));
  let t_list_any = FeelType::List(Box::new(t_any));
  let v_number = value_number!(10, 0);
  let v_string = Value::String("a".into());
  let v_list_number_1 = Value::List(Values::new(vec![value_number!(1, 0)]));
  let v_list_number_2 = Value::List(Values::new(vec![value_number!(1, 0), value_number!(2, 0)]));
  let v_list_string_1 = Value::List(Values::new(vec![Value::String("A".to_string())]));
  assert_eq!(r#"[10]"#, t_list_number.coerced(&v_number).to_string());
  assert_eq!(r#"[10]"#, t_list_any.coerced(&v_number).to_string());
  assert_eq!(r#"1"#, t_number.coerced(&v_list_number_1).to_string());
  assert_eq!(r#"null(after coercion)"#, t_number.coerced(&v_list_number_2).to_string());
  assert_eq!(r#"null(after coercion)"#, t_number.coerced(&v_list_string_1).to_string());
  assert_eq!(r#"["a"]"#, t_list_string.coerced(&v_string).to_string());
  assert_eq!(r#"["a"]"#, t_list_any.coerced(&v_string).to_string());
  assert_eq!(r#""A""#, t_string.coerced(&v_list_string_1).to_string());
  assert_eq!(r#"10"#, t_number.coerced(&v_number).to_string());
}

macro_rules! gcv_ok {
  ($t:expr, $v1:expr, $v2:expr) => {
    assert_eq!($v1, $t.get_conformant_value(&$v2))
  };
  ($t:expr, $v:expr) => {
    assert_eq!($v, $t.get_conformant_value(&$v))
  };
}

macro_rules! gcv_err {
  ($typ:expr, $value:expr) => {
    assert_eq!(
      value_null!("type '{}' is not conformant with value of type '{}'", $typ, $value.type_of()),
      $typ.get_conformant_value(&$value)
    );
  };
}

#[test]
fn test_type_get_conformant_value() {
  // entry names
  let name_a = Name::from("a");
  let name_b = Name::from("b");
  let name_c = Name::from("c");
  // any
  let t_any = FeelType::Any;
  // boolean
  let t_boolean = FeelType::Boolean;
  let v_boolean_true = Value::Boolean(true);
  let v_boolean_false = Value::Boolean(false);
  // date
  let t_date = FeelType::Date;
  let v_date = Value::Date(FeelDate::new(2022, 9, 26));
  let v_date_b = Value::Date(FeelDate::new(2022, 11, 30));
  // date time
  let t_date_time = FeelType::DateTime;
  let v_date_time = Value::DateTime(FeelDateTime::new(FeelDate::new(2022, 9, 27), FeelTime::new_hms_opt(9, 2, 0, 0).unwrap()));
  // days and time duration
  let t_days_and_time_duration = FeelType::DaysAndTimeDuration;
  let v_days_and_time_duration = Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_s(100));
  // null
  let t_null = FeelType::Null;
  let v_null = value_null!();
  // number
  let t_number = FeelType::Number;
  let v_number_1 = value_number!(1);
  let v_number_2 = value_number!(2);
  // string
  let t_string = FeelType::String;
  let v_string = Value::String("alpha".to_string());
  // time
  let t_time = FeelType::Time;
  let v_time = Value::Time(FeelTime::new_hms_opt(9, 2, 0, 0).unwrap());
  // years and months duration
  let t_years_and_months_duration = FeelType::YearsAndMonthsDuration;
  let v_years_and_months_duration = Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::from_ym(2, 3));
  // list
  let t_list_a = FeelType::List(Box::new(FeelType::Boolean));
  let v_list_a = Value::List(Values::new(vec![v_boolean_true.clone(), v_boolean_false.clone()]));
  let t_list_b = FeelType::List(Box::new(FeelType::Number));
  let v_list_b = Value::List(Values::new(vec![v_number_1.clone(), v_number_2.clone()]));
  // context
  let t_context_a = FeelType::context(&[(&name_a, &t_number)]);
  let mut ctx_a = FeelContext::default();
  ctx_a.set_entry(&name_a, v_number_1.clone());
  let v_context_a = Value::Context(ctx_a);
  let t_context_a_b = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean)]);
  let mut ctx_a_b = FeelContext::default();
  ctx_a_b.set_entry(&name_a, v_number_1.clone());
  ctx_a_b.set_entry(&name_b, v_boolean_false.clone());
  let v_context_a_b = Value::Context(ctx_a_b);
  let t_context_a_b_c = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean), (&name_c, &t_string)]);
  let mut ctx_a_b_c = FeelContext::default();
  ctx_a_b_c.set_entry(&name_a, v_number_1.clone());
  ctx_a_b_c.set_entry(&name_b, v_boolean_false.clone());
  ctx_a_b_c.set_entry(&name_c, v_string.clone());
  let v_context_a_b_c = Value::Context(ctx_a_b_c);
  // function
  let t_function_a = FeelType::function(&[FeelType::Number, FeelType::Number], &t_number);
  let v_function_a = Value::FunctionDefinition(
    vec![(name_a.clone(), t_number.clone()), (name_b.clone(), t_number.clone())],
    FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &FeelScope| value_number!(1)))),
    Closure::default(),
    FeelContext::default(),
    t_number.clone(),
  );
  let t_function_b = FeelType::function(&[FeelType::Number, FeelType::Number], &t_boolean);
  let v_function_b = Value::FunctionDefinition(
    vec![(name_a.clone(), t_number.clone()), (name_b.clone(), t_number.clone())],
    FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &FeelScope| value_number!(2)))),
    Closure::default(),
    FeelContext::default(),
    t_boolean.clone(),
  );
  let t_function_c = FeelType::function(&[FeelType::Number], &t_string);
  let v_function_c = Value::FunctionDefinition(
    vec![(name_a.clone(), t_number.clone())],
    FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &FeelScope| value_number!(3)))),
    Closure::default(),
    FeelContext::default(),
    t_string.clone(),
  );
  let t_function_d = FeelType::function(&[FeelType::Any], &t_string);
  // range
  let t_range_a = FeelType::range(&t_number);
  let v_range_a = Value::Range(Box::new(v_number_1.clone()), false, Box::new(v_number_2), false);
  let t_range_b = FeelType::range(&t_date);
  let v_range_b = Value::Range(Box::new(v_date.clone()), false, Box::new(v_date_b), false);
  // any
  gcv_ok!(t_any, v_boolean_true);
  gcv_ok!(t_any, v_context_a);
  gcv_ok!(t_any, v_date);
  gcv_ok!(t_any, v_date_time);
  gcv_ok!(t_any, v_days_and_time_duration);
  gcv_ok!(t_any, v_function_a);
  gcv_ok!(t_any, v_list_a);
  gcv_ok!(t_any, v_null);
  gcv_ok!(t_any, v_number_1);
  gcv_ok!(t_any, v_range_a);
  gcv_ok!(t_any, v_string);
  gcv_ok!(t_any, v_time);
  gcv_ok!(t_any, v_years_and_months_duration);
  // boolean
  gcv_ok!(t_boolean, v_boolean_false);
  gcv_ok!(t_boolean, v_null);
  gcv_err!(t_boolean, v_context_a);
  gcv_err!(t_boolean, v_date);
  gcv_err!(t_boolean, v_date_time);
  gcv_err!(t_boolean, v_days_and_time_duration);
  gcv_err!(t_boolean, v_function_a);
  gcv_err!(t_boolean, v_list_a);
  gcv_err!(t_boolean, v_number_1);
  gcv_err!(t_boolean, v_range_a);
  gcv_err!(t_boolean, v_string);
  gcv_err!(t_boolean, v_time);
  gcv_err!(t_boolean, v_years_and_months_duration);
  // context
  gcv_ok!(t_context_a, v_context_a);
  gcv_ok!(t_context_a, v_context_a, v_context_a_b);
  gcv_ok!(t_context_a_b, v_context_a_b);
  gcv_ok!(t_context_a_b, v_context_a_b, v_context_a_b_c);
  gcv_ok!(t_context_a, v_null);
  gcv_ok!(t_context_a_b_c, v_context_a_b_c);
  gcv_err!(t_context_a_b_c, v_context_a_b);
  gcv_err!(t_context_a_b, v_context_a);
  gcv_err!(t_context_a, v_boolean_false);
  gcv_err!(t_context_a, v_date);
  gcv_err!(t_context_a, v_date_time);
  gcv_err!(t_context_a, v_days_and_time_duration);
  gcv_err!(t_context_a, v_function_a);
  gcv_err!(t_context_a, v_list_a);
  gcv_err!(t_context_a, v_number_1);
  gcv_err!(t_context_a, v_range_a);
  gcv_err!(t_context_a, v_string);
  gcv_err!(t_context_a, v_time);
  gcv_err!(t_context_a, v_years_and_months_duration);
  // date
  gcv_ok!(t_date, v_date);
  gcv_ok!(t_date, v_null);
  gcv_err!(t_date, v_boolean_true);
  gcv_err!(t_date, v_context_a);
  gcv_err!(t_date, v_date_time);
  gcv_err!(t_date, v_days_and_time_duration);
  gcv_err!(t_date, v_function_a);
  gcv_err!(t_date, v_list_a);
  gcv_err!(t_date, v_number_1);
  gcv_err!(t_date, v_range_a);
  gcv_err!(t_date, v_string);
  gcv_err!(t_date, v_time);
  gcv_err!(t_date, v_years_and_months_duration);
  // date and time
  gcv_ok!(t_date_time, v_date_time);
  gcv_ok!(t_date_time, v_null);
  gcv_err!(t_date_time, v_boolean_true);
  gcv_err!(t_date_time, v_context_a);
  gcv_err!(t_date_time, v_date);
  gcv_err!(t_date_time, v_function_a);
  gcv_err!(t_date_time, v_list_a);
  gcv_err!(t_date_time, v_number_1);
  gcv_err!(t_date_time, v_range_a);
  gcv_err!(t_date_time, v_string);
  gcv_err!(t_date_time, v_time);
  gcv_err!(t_date_time, v_years_and_months_duration);
  // days and time duration
  gcv_ok!(t_days_and_time_duration, v_days_and_time_duration);
  gcv_ok!(t_days_and_time_duration, v_null);
  gcv_err!(t_days_and_time_duration, v_boolean_true);
  gcv_err!(t_days_and_time_duration, v_context_a);
  gcv_err!(t_days_and_time_duration, v_date);
  gcv_err!(t_days_and_time_duration, v_date_time);
  gcv_err!(t_days_and_time_duration, v_function_a);
  gcv_err!(t_days_and_time_duration, v_list_a);
  gcv_err!(t_days_and_time_duration, v_number_1);
  gcv_err!(t_days_and_time_duration, v_range_a);
  gcv_err!(t_days_and_time_duration, v_string);
  gcv_err!(t_days_and_time_duration, v_time);
  gcv_err!(t_days_and_time_duration, v_years_and_months_duration);
  // function
  gcv_ok!(t_function_a, v_function_a);
  gcv_ok!(t_function_b, v_function_b);
  gcv_ok!(t_function_c, v_function_c);
  gcv_ok!(t_function_a, v_null);
  gcv_err!(t_function_a, v_function_b);
  gcv_err!(t_function_a, v_function_c);
  gcv_err!(t_function_b, v_function_a);
  gcv_err!(t_function_b, v_function_c);
  gcv_err!(t_function_c, v_function_a);
  gcv_err!(t_function_c, v_function_b);
  gcv_err!(t_function_d, v_function_c);
  gcv_err!(t_function_a, v_boolean_false);
  gcv_err!(t_function_a, v_context_a);
  gcv_err!(t_function_a, v_date);
  gcv_err!(t_function_a, v_date_time);
  gcv_err!(t_function_a, v_days_and_time_duration);
  gcv_err!(t_function_a, v_list_a);
  gcv_err!(t_function_a, v_number_1);
  gcv_err!(t_function_a, v_range_a);
  gcv_err!(t_function_a, v_string);
  gcv_err!(t_function_a, v_time);
  gcv_err!(t_function_a, v_years_and_months_duration);
  // list
  gcv_ok!(t_list_a, v_list_a);
  gcv_ok!(t_list_b, v_list_b);
  gcv_ok!(t_list_a, v_null);
  gcv_err!(t_list_a, v_list_b);
  gcv_err!(t_list_b, v_list_a);
  gcv_err!(t_list_a, v_boolean_true);
  gcv_err!(t_list_a, v_context_a);
  gcv_err!(t_list_a, v_date);
  gcv_err!(t_list_a, v_date_time);
  gcv_err!(t_list_a, v_days_and_time_duration);
  gcv_err!(t_list_a, v_function_a);
  gcv_err!(t_list_a, v_number_1);
  gcv_err!(t_list_a, v_range_a);
  gcv_err!(t_list_a, v_string);
  gcv_err!(t_list_a, v_time);
  gcv_err!(t_list_a, v_years_and_months_duration);
  // null
  gcv_ok!(t_null, v_null);
  gcv_err!(t_null, v_boolean_true);
  gcv_err!(t_null, v_context_a);
  gcv_err!(t_null, v_date);
  gcv_err!(t_null, v_date_time);
  gcv_err!(t_null, v_days_and_time_duration);
  gcv_err!(t_null, v_function_a);
  gcv_err!(t_null, v_list_a);
  gcv_err!(t_null, v_number_1);
  gcv_err!(t_null, v_range_a);
  gcv_err!(t_null, v_string);
  gcv_err!(t_null, v_time);
  gcv_err!(t_null, v_years_and_months_duration);
  // number
  gcv_ok!(t_number, v_number_1);
  gcv_ok!(t_number, v_null);
  gcv_err!(t_number, v_boolean_false);
  gcv_err!(t_number, v_context_a);
  gcv_err!(t_number, v_date);
  gcv_err!(t_number, v_date_time);
  gcv_err!(t_number, v_days_and_time_duration);
  gcv_err!(t_number, v_function_a);
  gcv_err!(t_number, v_list_a);
  gcv_err!(t_number, v_range_a);
  gcv_err!(t_number, v_string);
  gcv_err!(t_number, v_time);
  gcv_err!(t_number, v_years_and_months_duration);
  // range
  gcv_ok!(t_range_a, v_range_a);
  gcv_ok!(t_range_b, v_range_b);
  gcv_ok!(t_range_a, v_null);
  gcv_err!(t_range_a, v_boolean_true);
  gcv_err!(t_range_a, v_context_a);
  gcv_err!(t_range_a, v_date);
  gcv_err!(t_range_a, v_date_time);
  gcv_err!(t_range_a, v_days_and_time_duration);
  gcv_err!(t_range_a, v_function_a);
  gcv_err!(t_range_a, v_list_a);
  gcv_err!(t_range_a, v_number_1);
  gcv_err!(t_range_a, v_string);
  gcv_err!(t_range_a, v_time);
  gcv_err!(t_range_a, v_years_and_months_duration);
  // string
  gcv_ok!(t_string, v_string);
  gcv_ok!(t_string, v_null);
  gcv_err!(t_string, v_boolean_false);
  gcv_err!(t_string, v_context_a);
  gcv_err!(t_string, v_date);
  gcv_err!(t_string, v_date_time);
  gcv_err!(t_string, v_days_and_time_duration);
  gcv_err!(t_string, v_function_a);
  gcv_err!(t_string, v_list_a);
  gcv_err!(t_string, v_number_1);
  gcv_err!(t_string, v_range_a);
  gcv_err!(t_string, v_time);
  gcv_err!(t_string, v_years_and_months_duration);
  // time
  gcv_ok!(t_time, v_time);
  gcv_ok!(t_time, v_null);
  gcv_err!(t_time, v_boolean_true);
  gcv_err!(t_time, v_context_a);
  gcv_err!(t_time, v_date);
  gcv_err!(t_time, v_date_time);
  gcv_err!(t_time, v_days_and_time_duration);
  gcv_err!(t_time, v_function_a);
  gcv_err!(t_time, v_list_a);
  gcv_err!(t_time, v_number_1);
  gcv_err!(t_time, v_range_a);
  gcv_err!(t_time, v_string);
  gcv_err!(t_time, v_years_and_months_duration);
  // years and months duration
  gcv_ok!(t_years_and_months_duration, v_years_and_months_duration);
  gcv_ok!(t_years_and_months_duration, v_null);
  gcv_err!(t_years_and_months_duration, v_boolean_true);
  gcv_err!(t_years_and_months_duration, v_context_a);
  gcv_err!(t_years_and_months_duration, v_date);
  gcv_err!(t_years_and_months_duration, v_date_time);
  gcv_err!(t_years_and_months_duration, v_days_and_time_duration);
  gcv_err!(t_years_and_months_duration, v_function_a);
  gcv_err!(t_years_and_months_duration, v_list_a);
  gcv_err!(t_years_and_months_duration, v_number_1);
  gcv_err!(t_years_and_months_duration, v_range_a);
  gcv_err!(t_years_and_months_duration, v_string);
  gcv_err!(t_years_and_months_duration, v_time);
}

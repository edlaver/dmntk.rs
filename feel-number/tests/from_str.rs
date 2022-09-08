mod common;

use crate::common::eqs;
use dmntk_feel_number::FeelNumber;

#[test]
fn test_from_str_001() {
  eqs("12345.6789", "12345.6789".parse::<FeelNumber>().unwrap());
}

#[test]
fn test_from_str_002() {
  assert!("1234a5".parse::<FeelNumber>().is_err());
}

#[test]
fn test_from_str_003() {
  assert_eq!(
    "FeelNumberError: invalid number literal '1234a5'",
    "1234a5".parse::<FeelNumber>().unwrap_err().to_string()
  );
}

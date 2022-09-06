mod common;

use crate::common::eqs;
use dmntk_feel_number::FeelNumber;

#[test]
fn test_add_001() {
  eqs("0.4", FeelNumber::new(1, 1) + FeelNumber::new(3, 1));
}

#[test]
fn test_add_002() {
  eqs("123.8347847", FeelNumber::new(12345, 2) + FeelNumber::new(3847847, 7));
}

#[test]
fn test_add_003() {
  eqs("2.00", FeelNumber::new(123, 2) + FeelNumber::new(77, 2));
}

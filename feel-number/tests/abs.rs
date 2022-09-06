mod common;

use crate::common::eqs;
use dmntk_feel_number::FeelNumber;

#[test]
fn test_abs_001() {
  eqs("0", FeelNumber::new(0, 0).abs());
}

#[test]
fn test_abs_002() {
  eqs("0", FeelNumber::new(-0, 0).abs());
}

#[test]
fn test_abs_003() {
  eqs("1", FeelNumber::new(1, 0).abs());
}

#[test]
fn test_abs_004() {
  eqs("1", FeelNumber::new(-1, 0).abs());
}

#[test]
fn test_abs_005() {
  eqs("0.123456", FeelNumber::new(123456, 6).abs());
}

#[test]
fn test_abs_006() {
  eqs("0.123456", FeelNumber::new(-123456, 6).abs());
}

mod common;

use crate::common::eqs;
use dmntk_feel_number::FeelNumber;

#[test]
fn test_sub_001() {
  eqs("1", FeelNumber::new(123, 2) - FeelNumber::new(23, 2));
}

#[test]
fn test_sub_002() {
  let mut x = FeelNumber::new(123, 2);
  x -= FeelNumber::new(23, 2);
  eqs("1", x);
}

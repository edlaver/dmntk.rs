mod common;

use crate::common::eqs;
use dmntk_feel_number::FeelNumber;

#[test]
fn test_trunc_001() {
  eqs("12345", FeelNumber::new(123456789, 4).trunc());
}

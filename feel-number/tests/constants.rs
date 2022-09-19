mod common;

use crate::common::eqs;
use dmntk_feel_number::{FEEL_NUMBER_BILLION, FEEL_NUMBER_ONE, FEEL_NUMBER_TWO, FEEL_NUMBER_ZERO};

#[test]
fn test_constants_001() {
  eqs("0", FEEL_NUMBER_ZERO);
}

#[test]
fn test_constants_002() {
  eqs("1", FEEL_NUMBER_ONE);
}

#[test]
fn test_constants_003() {
  eqs("2", FEEL_NUMBER_TWO);
}

#[test]
fn test_constants_004() {
  eqs("1000000000", FEEL_NUMBER_BILLION);
}

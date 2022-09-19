mod common;

use crate::common::eqs;

#[test]
fn test_ceiling_001() {
  eqs("2", num!(1.5).ceiling());
}

#[test]
fn test_ceiling_002() {
  eqs("-1", num!(-1.5).ceiling());
}

#[test]
fn test_ceiling_003() {
  eqs("1", num!(0.3333).ceiling());
}

#[test]
fn test_ceiling_004() {
  eqs("0", num!(-0.3333).ceiling());
}

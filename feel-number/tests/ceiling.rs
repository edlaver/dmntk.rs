use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_ceiling_001() {
  assert_eq!("2", num!(1.5).ceiling());
}

#[test]
fn test_ceiling_002() {
  assert_eq!("-1", num!(-1.5).ceiling());
}

#[test]
fn test_ceiling_003() {
  assert_eq!("1", num!(0.3333).ceiling());
}

#[test]
fn test_ceiling_004() {
  assert_eq!("0", num!(-0.3333).ceiling());
}

use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_floor_001() {
  assert_eq!("1", num!(1.5).floor());
}

#[test]
fn test_floor_002() {
  assert_eq!("-2", num!(-1.5).floor());
}

#[test]
fn test_floor_003() {
  assert_eq!("0", num!(0.333).floor());
}

#[test]
fn test_floor_004() {
  assert_eq!("-1", num!(-0.3333).floor());
}

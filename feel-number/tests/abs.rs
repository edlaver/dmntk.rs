use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_abs_001() {
  assert_eq!("0", num!(0).abs());
}

#[test]
fn test_abs_002() {
  assert_eq!("0", num!(-0).abs());
}

#[test]
fn test_abs_003() {
  assert_eq!("1", num!(1).abs());
}

#[test]
fn test_abs_004() {
  assert_eq!("1", num!(-1).abs());
}

#[test]
fn test_abs_005() {
  assert_eq!("0.123456", num!(0.123456).abs());
}

#[test]
fn test_abs_006() {
  assert_eq!("0.123456", num!(-0.123456).abs());
}

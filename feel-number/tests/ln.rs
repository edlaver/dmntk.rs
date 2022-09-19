use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_ln_001() {
  assert!(num!(-1).ln().is_none());
}

#[test]
fn test_ln_002() {
  assert!(num!(0).ln().is_none());
}

#[test]
fn test_ln_003() {
  assert_eq!("0", num!(1).ln().unwrap());
}

#[test]
fn test_ln_004() {
  assert_eq!("1.386294361119890618834464242916353", num!(4).ln().unwrap());
}

#[test]
fn test_ln_005() {
  assert_eq!("2.302585092994045684017991454684364", num!(10).ln().unwrap());
}

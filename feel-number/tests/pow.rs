use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_pow_001() {
  assert!(num!(0).pow(&num!(0)).is_none());
}

#[test]
fn test_pow_002() {
  assert_eq!("41959.85737359436186095331070746801", num!(12.2384283).pow(&num!(4.25)).unwrap());
}

use dmntk_feel_number::FeelNumber;

#[test]
fn test_constants_001() {
  assert_eq!("0", FeelNumber::zero());
}

#[test]
fn test_constants_002() {
  assert_eq!("1", FeelNumber::one());
}

#[test]
fn test_constants_003() {
  assert_eq!("2", FeelNumber::two());
}

#[test]
fn test_constants_004() {
  assert_eq!("1000000000", FeelNumber::billion());
}

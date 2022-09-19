use dmntk_feel_number::FeelNumber;

#[test]
fn test_debug_001() {
  assert_eq!("49", format!("{:?}", FeelNumber::new(49, 0)));
}

#[test]
fn test_debug_002() {
  assert_eq!("1.23456789", format!("{:?}", FeelNumber::new(123456789, 8)));
}

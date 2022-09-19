use dmntk_feel_number::FeelNumber;

#[test]
fn test_sub_001() {
  assert_eq!("1", FeelNumber::new(123, 2) - FeelNumber::new(23, 2));
}

#[test]
fn test_sub_002() {
  let mut x = FeelNumber::new(123, 2);
  x -= FeelNumber::new(23, 2);
  assert_eq!("1", x);
}

use dmntk_feel_number::FeelNumber;

#[test]
fn test_minus_zero_001() {
  assert_eq!("0", FeelNumber::new(-0, 0).to_string());
}

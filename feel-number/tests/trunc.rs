use dmntk_feel_number::FeelNumber;

#[test]
fn test_trunc_001() {
  assert_eq!("12345", FeelNumber::new(123456789, 4).trunc());
}

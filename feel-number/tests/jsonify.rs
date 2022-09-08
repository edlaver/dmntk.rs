use dmntk_common::Jsonify;
use dmntk_feel_number::FeelNumber;

#[test]
fn test_jsonify_001() {
  assert_eq!("12345.6789", FeelNumber::new(123456789, 4).jsonify());
}

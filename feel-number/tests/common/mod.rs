use dmntk_feel_number::FeelNumber;

/// Utility function for testing the expected number value based on string representation.
pub fn eqs(expected: &str, actual: FeelNumber) {
  assert_eq!(expected, actual.to_string());
}

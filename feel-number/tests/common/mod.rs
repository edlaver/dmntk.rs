use dmntk_feel_number::FeelNumber;

#[macro_export]
macro_rules! num {
  ($n:expr) => {{
    use dmntk_feel_number::FeelNumber;
    stringify!($n).parse::<FeelNumber>().unwrap()
  }};
}

/// Utility function for testing the expected number value based on string representation.
pub fn eqs(expected: &str, actual: FeelNumber) {
  assert_eq!(expected, actual.to_string());
}

mod common;

use dmntk_feel_number::FeelNumber;

#[test]
fn test_to_string_001() {
  eqs!("49", num!(49));
}

#[cfg(feature = "dec")]
#[test]
fn test_to_string_002() {
  eqs!("49.0", FeelNumber::new(490, 1).to_string());
}

#[cfg(feature = "dfp")]
#[test]
fn test_to_string_002() {
  eqs!("49", FeelNumber::new(490, 1).to_string());
}

#[test]
fn test_to_string_003() {
  eqs!("4900", FeelNumber::new(4900, 0).to_string());
}

#[test]
fn test_to_string_004() {
  eqs!("50", FeelNumber::new(50, 0).to_string());
}

#[test]
fn test_to_string_005() {
  eqs!("50", num!(50));
}

#[test]
fn test_to_string_006() {
  eqs!("50.5", FeelNumber::new(505, 1).to_string());
}

#[cfg(feature = "dec")]
#[test]
fn test_to_string_007() {
  eqs!("50.50", FeelNumber::new(5050, 2).to_string());
}

#[cfg(feature = "dfp")]
#[test]
fn test_to_string_007() {
  eqs!("50.5", FeelNumber::new(5050, 2).to_string());
}

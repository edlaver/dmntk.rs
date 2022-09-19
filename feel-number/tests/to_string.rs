use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_to_string_001() {
  assert_eq!("49", num!(49));
}

#[test]
fn test_to_string_002() {
  assert_eq!("49.0", FeelNumber::new(490, 1).to_string());
}

#[test]
fn test_to_string_003() {
  assert_eq!("4900", FeelNumber::new(4900, 0).to_string());
}

#[test]
fn test_to_string_004() {
  assert_eq!("50", FeelNumber::new(50, 0).to_string());
}

#[test]
fn test_to_string_005() {
  assert_eq!("50", num!(50));
}

#[test]
fn test_to_string_006() {
  assert_eq!("50.5", FeelNumber::new(505, 1).to_string());
}

#[test]
fn test_to_string_007() {
  assert_eq!("50.50", FeelNumber::new(5050, 2).to_string());
}

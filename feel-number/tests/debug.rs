mod common;

use dmntk_feel_number::FeelNumber;

#[cfg(feature = "dec")]
#[test]
fn test_debug_001() {
  eqs!("49", format!("{:?}", FeelNumber::new(49, 0)));
}

#[cfg(feature = "dec")]
#[test]
fn test_debug_002() {
  eqs!("1.23456789", format!("{:?}", FeelNumber::new(123456789, 8)));
}

#[cfg(feature = "dfp")]
#[test]
fn test_debug_001() {
  eqs!("+49E+0", format!("{:?}", FeelNumber::new(49, 0)));
}

#[cfg(feature = "dfp")]
#[test]
fn test_debug_002() {
  eqs!("+123456789E-8", format!("{:?}", FeelNumber::new(123456789, 8)));
}

#[cfg(feature = "dfp")]
#[test]
fn test_debug_003() {
  eqs!("+5050E-2", format!("{:?}", num!(50.50)));
}

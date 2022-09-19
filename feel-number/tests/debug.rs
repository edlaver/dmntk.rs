use dmntk_feel_number::{num, FeelNumber};

#[cfg(feature = "dec")]
#[test]
fn test_debug_001() {
  assert_eq!("49", format!("{:?}", FeelNumber::new(49, 0)));
}

#[cfg(feature = "dec")]
#[test]
fn test_debug_002() {
  assert_eq!("1.23456789", format!("{:?}", FeelNumber::new(123456789, 8)));
}

#[cfg(feature = "dfp")]
#[test]
fn test_debug_001() {
  assert_eq!("+49E+0", format!("{:?}", FeelNumber::new(49, 0)));
}

#[cfg(feature = "dfp")]
#[test]
fn test_debug_002() {
  assert_eq!("+123456789E-8", format!("{:?}", FeelNumber::new(123456789, 8)));
}

#[cfg(feature = "dfp")]
#[test]
fn test_debug_003() {
  assert_eq!("+5050E-2", format!("{:?}", num!(50.50)));
}

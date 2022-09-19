use dmntk_feel_number::{num, FeelNumber};

#[cfg(feature = "dec")]
#[test]
fn test_pow_001() {
  assert!(num!(0).pow(&num!(0)).is_none());
}

#[cfg(feature = "dec")]
#[test]
fn test_pow_002() {
  assert_eq!("41959.85737359436186095331070746801", num!(12.2384283).pow(&num!(4.25)).unwrap());
}

#[cfg(feature = "dfp")]
#[test]
fn test_pow_001() {
  assert_eq!("+1E+0", num!(0).pow(&num!(0)).unwrap());
}

#[cfg(feature = "dfp")]
#[test]
fn test_pow_002() {
  assert_eq!("41959.85737359436186095331070746800", num!(12.2384283).pow(&num!(4.25)).unwrap());
}

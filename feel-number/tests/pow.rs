mod common;

use dmntk_feel_number::FeelNumber;

#[cfg(feature = "dec")]
#[test]
fn test_pow_001() {
  assert!(num!(0).pow(&num!(0)).is_none());
}

#[cfg(feature = "dec")]
#[test]
fn test_pow_002() {
  eqs!("41959.85737359436186095331070746801", num!(12.2384283).pow(&num!(4.25)).unwrap());
}

#[cfg(feature = "dfp")]
#[test]
fn test_pow_001() {
  eqs!("1", num!(0).pow(&num!(0)).unwrap());
}

#[cfg(feature = "dfp")]
#[test]
fn test_pow_002() {
  eqs!("41959.857373594361860953310707468", num!(12.2384283).pow(&num!(4.25)).unwrap());
}

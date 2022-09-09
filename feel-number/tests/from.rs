mod common;

use crate::common::eqs;
use dmntk_feel_number::FeelNumber;

#[test]
fn test_from_001() {
  let x: FeelNumber = u8::MAX.into();
  eqs("255", x);
}

#[test]
fn test_from_001_() {
  let x: FeelNumber = i8::MIN.into();
  eqs("-128", x);
}

#[test]
fn test_from_0011() {
  let x: FeelNumber = u16::MAX.into();
  eqs("65535", x);
}

#[test]
fn test_from_001_1() {
  let x: FeelNumber = i16::MIN.into();
  eqs("-32768", x);
}

#[test]
fn test_from_002() {
  let x: FeelNumber = i32::MIN.into();
  eqs("-2147483648", x);
}

#[test]
fn test_from_003() {
  let x: FeelNumber = u32::MAX.into();
  eqs("4294967295", x);
}

#[test]
fn test_from_004_() {
  let x: FeelNumber = u64::MAX.into();
  eqs("18446744073709551615", x);
}

#[test]
fn test_from_004() {
  let x: FeelNumber = i64::MIN.into();
  eqs("-9223372036854775808", x);
}

#[test]
fn test_from_005() {
  let x: FeelNumber = isize::MIN.into();
  eqs("-9223372036854775808", x);
}

#[test]
fn test_from_006() {
  let x: FeelNumber = usize::MAX.into();
  eqs("18446744073709551615", x);
}

#[test]
fn test_from_007() {
  let x: i32 = FeelNumber::new(-2147483648, 0).try_into().unwrap();
  assert_eq!(-2147483648_i32, x);
}

#[test]
fn test_from_008() {
  let x: u32 = FeelNumber::new(4294967295, 0).try_into().unwrap();
  assert_eq!(4294967295_u32, x);
}

#[test]
fn test_from_009() {
  let x: u8 = FeelNumber::new(255, 0).try_into().unwrap();
  assert_eq!(255_u8, x);
}

#[test]
fn test_from_010() {
  assert!(u8::try_from(FeelNumber::new(255, 0)).is_ok());
  assert!(u8::try_from(FeelNumber::new(256, 0)).is_err());
}

#[test]
fn test_from_011() {
  assert!(i32::try_from(FeelNumber::new(-2147483648, 0)).is_ok());
  assert!(i32::try_from(FeelNumber::new(-2147483649, 0)).is_err());
}

#[test]
fn test_from_012() {
  assert!(u32::try_from(FeelNumber::new(4294967295, 0)).is_ok());
  assert!(u32::try_from(FeelNumber::new(4294967296, 0)).is_err());
}

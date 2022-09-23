mod common;

use dmntk_feel_number::FeelNumber;

#[test]
fn test_from_001() {
  let x: FeelNumber = u8::MAX.into();
  eqs!("255", x);
}

#[test]
fn test_from_0002() {
  let x: FeelNumber = i8::MIN.into();
  eqs!("-128", x);
}

#[test]
fn test_from_0003() {
  let x: FeelNumber = u16::MAX.into();
  eqs!("65535", x);
}

#[test]
fn test_from_0004() {
  let x: FeelNumber = i16::MIN.into();
  eqs!("-32768", x);
}

#[test]
fn test_from_0005() {
  let x: FeelNumber = i32::MIN.into();
  eqs!("-2147483648", x);
}

#[test]
fn test_from_0006() {
  let x: FeelNumber = u32::MAX.into();
  eqs!("4294967295", x);
}

#[test]
fn test_from_0007() {
  let x: FeelNumber = u64::MAX.into();
  eqs!("18446744073709551615", x);
}

#[test]
fn test_from_0008() {
  let x: FeelNumber = i64::MIN.into();
  eqs!("-9223372036854775808", x);
}

#[test]
fn test_from_0009() {
  let x: FeelNumber = isize::MIN.into();
  eqs!("-9223372036854775808", x);
}

#[test]
fn test_from_0010() {
  let x: FeelNumber = usize::MAX.into();
  eqs!("18446744073709551615", x);
}

#[test]
fn test_from_0011() {
  let x: i32 = FeelNumber::new(-2147483648, 0).try_into().unwrap();
  assert_eq!(-2147483648_i32, x);
}

#[test]
fn test_from_0012() {
  let x: u32 = FeelNumber::new(4294967295, 0).try_into().unwrap();
  assert_eq!(4294967295_u32, x);
}

#[test]
fn test_from_0013() {
  let x: u8 = FeelNumber::new(255, 0).try_into().unwrap();
  assert_eq!(255_u8, x);
}

#[test]
fn test_from_0014() {
  assert!(u8::try_from(num!(0)).is_ok());
  assert!(u8::try_from(num!(255)).is_ok());
  assert!(u8::try_from(num!(256)).is_err());
  assert!(u8::try_from(num!(4294967296)).is_err());
}

#[test]
fn test_from_0015() {
  assert!(i32::try_from(FeelNumber::new(-2147483648, 0)).is_ok());
  assert!(i32::try_from(FeelNumber::new(-2147483649, 0)).is_err());
}

#[test]
fn test_from_0016() {
  assert!(u32::try_from(FeelNumber::new(4294967295, 0)).is_ok());
  assert!(u32::try_from(FeelNumber::new(4294967296, 0)).is_err());
}

#[test]
fn test_from_0017() {
  assert!(isize::try_from(num!(2)).is_ok());
  assert!(isize::try_from(<usize as Into<FeelNumber>>::into(usize::MAX)).is_err());
}

#[test]
fn test_from_0018() {
  assert!(usize::try_from(num!(0)).is_ok());
  assert!(usize::try_from(num!(2)).is_ok());
  assert!(usize::try_from(<usize as Into<FeelNumber>>::into(usize::MAX)).is_ok());
  assert!(usize::try_from(<usize as Into<FeelNumber>>::into(usize::MIN)).is_ok());
  assert!(usize::try_from(<isize as Into<FeelNumber>>::into(isize::MAX)).is_ok());
  assert!(usize::try_from(<isize as Into<FeelNumber>>::into(isize::MIN)).is_err());
  assert!(usize::try_from(num!(-1)).is_err());
  assert_eq!(0, usize::try_from(num!(0)).unwrap());
  assert_eq!(2, usize::try_from(num!(2)).unwrap());
  assert_eq!(usize::MAX, usize::try_from(<usize as Into<FeelNumber>>::into(usize::MAX)).unwrap());
  assert_eq!(usize::MIN, usize::try_from(<usize as Into<FeelNumber>>::into(usize::MIN)).unwrap());
  assert_eq!(usize::MAX / 2, usize::try_from(<isize as Into<FeelNumber>>::into(isize::MAX)).unwrap());
}

#[test]
fn test_from_0019() {
  assert!(u64::try_from(num!(0)).is_ok());
  assert!(u64::try_from(num!(2)).is_ok());
  assert!(u64::try_from(<usize as Into<FeelNumber>>::into(usize::MAX)).is_ok());
  assert!(u64::try_from(<usize as Into<FeelNumber>>::into(usize::MIN)).is_ok());
  assert!(u64::try_from(<isize as Into<FeelNumber>>::into(isize::MAX)).is_ok());
  assert!(u64::try_from(<isize as Into<FeelNumber>>::into(isize::MIN)).is_err());
  assert!(u64::try_from(num!(-1)).is_err());
  assert_eq!(300_000_000_u64, u64::try_from(num!(300000000)).unwrap());
}

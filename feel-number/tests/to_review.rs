mod common;

use crate::common::eqs;
use dmntk_feel_number::{FeelNumber, FEEL_NUMBER_BILLION, FEEL_NUMBER_ONE, FEEL_NUMBER_TWO, FEEL_NUMBER_ZERO};

#[test]
fn test_stringify() {
  eqs("49", num!(49));
  assert_eq!("49.0", FeelNumber::new(490, 1).to_string());
  assert_eq!("4900", FeelNumber::new(4900, 0).to_string());
  assert_eq!("50", FeelNumber::new(50, 0).to_string());
  eqs("50", num!(50));
  assert_eq!("50.5", FeelNumber::new(505, 1).to_string());
  assert_eq!("50.50", FeelNumber::new(5050, 2).to_string());
}

#[test]
fn test_debug() {
  assert_eq!("49", format!("{:?}", FeelNumber::new(49, 0)));
  assert_eq!("1.23456789", format!("{:?}", FeelNumber::new(123456789, 8)));
}

#[test]
fn test_ceiling() {
  assert_eq!("2", FeelNumber::new(15, 1).ceiling().to_string());
  assert_eq!("-1", FeelNumber::new(-15, 1).ceiling().to_string());
  assert_eq!("1", FeelNumber::new(3333, 4).ceiling().to_string());
  assert_eq!("0", FeelNumber::new(-3333, 4).ceiling().to_string());
}

#[test]
fn test_constants() {
  assert_eq!("0", FEEL_NUMBER_ZERO.to_string());
  assert_eq!("1", FEEL_NUMBER_ONE.to_string());
  assert_eq!("2", FEEL_NUMBER_TWO.to_string());
  assert_eq!("1000000000", FEEL_NUMBER_BILLION.to_string());
}

#[test]
fn test_div() {
  assert_eq!("2.5", (FeelNumber::new(20, 0) / FeelNumber::new(8, 0)).to_string());
}

#[test]
fn test_div_assign() {
  let mut x = FeelNumber::new(20, 0);
  x /= FeelNumber::new(8, 0);
  assert_eq!("2.5", x.to_string());
}

#[test]
fn test_exp() {
  assert_eq!("2.718281828459045235360287471352662", FeelNumber::new(1, 0).exp().to_string());
  assert_eq!("54.59815003314423907811026120286088", FeelNumber::new(4, 0).exp().to_string());
}

#[test]
fn test_from_string() {
  eqs("0", num!(0));
  assert_eq!("-0", num!(-0).to_string());
  assert_eq!("1", num!(1).to_string());
  assert_eq!("-1", num!(-1).to_string());
  assert_eq!("1.23456789", num!(1.23456789).to_string());
  assert_eq!("-1.23456789", num!(-1.23456789).to_string());
}

#[test]
fn test_floor() {
  assert_eq!("1", FeelNumber::new(15, 1).floor().to_string());
  assert_eq!("-2", FeelNumber::new(-15, 1).floor().to_string());
  assert_eq!("0", FeelNumber::new(3333, 4).floor().to_string());
  assert_eq!("-1", FeelNumber::new(-3333, 4).floor().to_string());
}

#[test]
fn test_is_integer() {
  assert!(FeelNumber::new(41, 0).is_integer());
  assert!(!FeelNumber::new(41, 1).is_integer());
}

#[test]
fn test_is_negative() {
  assert!(FeelNumber::new(-123, 2).is_negative());
  assert!(FeelNumber::new(-1, 0).is_negative());
  assert!(!FeelNumber::new(-0, 0).is_negative());
  assert!(!FeelNumber::new(0, 0).is_negative());
  assert!(!FeelNumber::new(1, 0).is_negative());
  assert!(!FeelNumber::new(123, 2).is_negative());
}

#[test]
fn test_is_positive() {
  assert!(!FeelNumber::new(-123, 2).is_positive());
  assert!(!FeelNumber::new(-1, 0).is_positive());
  assert!(!FeelNumber::new(-0, 0).is_positive());
  assert!(!FeelNumber::new(0, 0).is_positive());
  assert!(FeelNumber::new(1, 0).is_positive());
  assert!(FeelNumber::new(123, 2).is_positive());
}

#[test]
fn test_minus_zero() {
  assert_eq!("0", FeelNumber::new(-0, 0).to_string());
}

#[test]
fn test_mul() {
  assert_eq!("12", (FeelNumber::new(12, 1) * FeelNumber::new(10, 0)).to_string());
}

#[test]
fn test_mul_assign() {
  let mut x = FeelNumber::new(12, 1);
  x *= FeelNumber::new(10, 0);
  assert_eq!("12", x.to_string());
}

#[test]
fn test_neg() {
  assert_eq!("-1.23", (-FeelNumber::new(123, 2)).to_string());
  assert_eq!("1.23", (-FeelNumber::new(-123, 2)).to_string());
}

#[test]
fn test_odd() {
  assert!(!num!(-4).odd());
  assert!(num!(-3).odd());
  assert!(!num!(-2).odd());
  assert!(num!(-1).odd());
  assert!(!num!(-0).odd());
  assert!(!num!(0).odd());
  assert!(num!(1).odd());
  assert!(!num!(2).odd());
  assert!(num!(3).odd());
  assert!(!num!(4).odd());
  assert!(!FeelNumber::new(31, 1).odd());
}

#[test]
fn test_try_from_number_to_isize() {
  assert!(isize::try_from(num!(2)).is_ok());
  // assert!(isize::try_from(num!(isize::MAX as i128)).is_ok());
  // assert!(isize::try_from(num!(isize::MIN as i128)).is_ok());
  // assert!(isize::try_from(num!(i128::MAX)).is_err());
  // assert!(isize::try_from(num!(i128::MIN)).is_err());
  assert!(isize::try_from(<usize as Into<FeelNumber>>::into(usize::MAX)).is_err());
}

#[test]
fn test_try_from_number_to_usize() {
  assert!(usize::try_from(num!(0)).is_ok());
  assert!(usize::try_from(num!(2)).is_ok());
  assert!(usize::try_from(<usize as Into<FeelNumber>>::into(usize::MAX)).is_ok());
  assert!(usize::try_from(<usize as Into<FeelNumber>>::into(usize::MIN)).is_ok());
  assert!(usize::try_from(<isize as Into<FeelNumber>>::into(isize::MAX)).is_ok());
  assert!(usize::try_from(<isize as Into<FeelNumber>>::into(isize::MIN)).is_err());
  // assert!(usize::try_from(num!(i128::MAX)).is_err());
  // assert!(usize::try_from(num!(i128::MIN)).is_err());
  assert!(usize::try_from(num!(-1)).is_err());
  assert_eq!(0, usize::try_from(num!(0)).unwrap());
  assert_eq!(2, usize::try_from(num!(2)).unwrap());
  assert_eq!(usize::MAX, usize::try_from(<usize as Into<FeelNumber>>::into(usize::MAX)).unwrap());
  assert_eq!(usize::MIN, usize::try_from(<usize as Into<FeelNumber>>::into(usize::MIN)).unwrap());
  assert_eq!(usize::MAX / 2, usize::try_from(<isize as Into<FeelNumber>>::into(isize::MAX)).unwrap());
}

#[test]
fn test_try_from_number_to_u64() {
  assert!(u64::try_from(num!(0)).is_ok());
  assert!(u64::try_from(num!(2)).is_ok());
  assert!(u64::try_from(<usize as Into<FeelNumber>>::into(usize::MAX)).is_ok());
  assert!(u64::try_from(<usize as Into<FeelNumber>>::into(usize::MIN)).is_ok());
  assert!(u64::try_from(<isize as Into<FeelNumber>>::into(isize::MAX)).is_ok());
  assert!(u64::try_from(<isize as Into<FeelNumber>>::into(isize::MIN)).is_err());
  // assert!(u64::try_from(num!(i128::MAX)).is_err());
  // assert!(u64::try_from(num!(i128::MIN)).is_err());
  assert!(u64::try_from(num!(-1)).is_err());
  assert_eq!(300_000_000_u64, u64::try_from(num!(300000000)).unwrap());
}

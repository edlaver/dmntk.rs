use dmntk_feel_number::FeelNumber;

#[test]
fn test_stringify() {
  assert_eq!("49", FeelNumber::new(49, 0).to_string());
  assert_eq!("49", FeelNumber::from_i128(49).to_string());
  assert_eq!("49.0", FeelNumber::new(490, 1).to_string());
  assert_eq!("4900", FeelNumber::new(4900, 0).to_string());
  assert_eq!("50", FeelNumber::new(50, 0).to_string());
  assert_eq!("50", FeelNumber::from_i128(50).to_string());
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
fn test_comparison() {
  assert_eq!(FeelNumber::new(120000, 0), FeelNumber::from_i128(120000));
  assert!(!(FeelNumber::from_i128(0) > FeelNumber::from_i128(0)));
  assert!((FeelNumber::from_i128(0) >= FeelNumber::from_i128(0)));
  assert!((FeelNumber::new(123456, 2) > FeelNumber::new(123456, 3)));
  assert!((FeelNumber::new(123456, 3) < FeelNumber::new(123456, 2)));
  assert!((FeelNumber::new(123456, 2) <= FeelNumber::new(123456, 2)));
  assert!((FeelNumber::new(123456, 2) >= FeelNumber::new(123456, 2)));
  assert!((11_isize > FeelNumber::new(10, 0)));
  assert!((FeelNumber::new(11, 0) > 10));
  assert!((-6111..6176).contains(&FeelNumber::from_i128(0)));
  assert!((0..6176).contains(&FeelNumber::from_i128(6175)));
  assert!((-3..2).contains(&FeelNumber::from_i128(-3)));
  assert!(!(-3..2).contains(&FeelNumber::from_i128(-4)));
  assert!((0..60).contains(&FeelNumber::new(0, 0)));
  assert!((0..60).contains(&FeelNumber::new(59_999_999_999, 9)));
  assert!(!(0..60).contains(&FeelNumber::from_i128(60)));
}

#[test]
fn test_constants() {
  assert_eq!("0", FeelNumber::zero().to_string());
  assert_eq!("1", FeelNumber::one().to_string());
  assert_eq!("2", FeelNumber::two().to_string());
  assert_eq!("1000000000", FeelNumber::nano().to_string());
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
fn test_equal() {
  assert!((FeelNumber::from_i128(0) == FeelNumber::from_i128(0)));
  assert!(!(FeelNumber::from_i128(0) == FeelNumber::from_i128(1)));
  assert!(!(FeelNumber::from_i128(1) == FeelNumber::from_i128(0)));
  assert!((FeelNumber::new(123456, 2) == FeelNumber::new(123456, 2)));
  assert!(!(FeelNumber::new(123456, 2) == FeelNumber::new(-123456, 2)));
  assert!((FeelNumber::from_i128(0) == 0_isize));
  assert!((FeelNumber::from_i128(1) == 1_isize));
  assert!((FeelNumber::from_i128(-1) == -1_isize));
  assert!((0_isize == FeelNumber::from_i128(0)));
  assert!((1_isize == FeelNumber::from_i128(1)));
  assert!((-1_isize == FeelNumber::from_i128(-1)));
}

#[test]
fn test_even() {
  assert!(FeelNumber::from_i128(-4).even());
  assert!(!FeelNumber::from_i128(-3).even());
  assert!(FeelNumber::from_i128(-2).even());
  assert!(!FeelNumber::from_i128(-1).even());
  assert!(FeelNumber::from_i128(-0).even());
  assert!(FeelNumber::from_i128(0).even());
  assert!(!FeelNumber::from_i128(1).even());
  assert!(FeelNumber::from_i128(2).even());
  assert!(!FeelNumber::from_i128(3).even());
  assert!(FeelNumber::from_i128(4).even());
  assert!(!FeelNumber::new(41, 1).even());
}

#[test]
fn test_exp() {
  assert_eq!("2.718281828459045235360287471352662", FeelNumber::from_i128(1).exp().to_string());
  assert_eq!("54.59815003314423907811026120286088", FeelNumber::from_i128(4).exp().to_string());
}

#[test]
fn test_from_string() {
  assert_eq!("0", FeelNumber::from_string("0").to_string());
  assert_eq!("-0", FeelNumber::from_string("-0").to_string());
  assert_eq!("1", FeelNumber::from_string("1").to_string());
  assert_eq!("-1", FeelNumber::from_string("-1").to_string());
  assert_eq!("1.23456789", FeelNumber::from_string("1.23456789").to_string());
  assert_eq!("-1.23456789", FeelNumber::from_string("-1.23456789").to_string());
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
fn test_ln() {
  assert!(FeelNumber::from_i128(-1).ln().is_none());
  assert!(FeelNumber::from_i128(0).ln().is_none());
  assert_eq!("0", FeelNumber::from_i128(1).ln().unwrap().to_string());
  assert_eq!("1.386294361119890618834464242916353", FeelNumber::from_i128(4).ln().unwrap().to_string());
  assert_eq!("2.302585092994045684017991454684364", FeelNumber::from_i128(10).ln().unwrap().to_string());
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
  assert!(!FeelNumber::from_i128(-4).odd());
  assert!(FeelNumber::from_i128(-3).odd());
  assert!(!FeelNumber::from_i128(-2).odd());
  assert!(FeelNumber::from_i128(-1).odd());
  assert!(!FeelNumber::from_i128(-0).odd());
  assert!(!FeelNumber::from_i128(0).odd());
  assert!(FeelNumber::from_i128(1).odd());
  assert!(!FeelNumber::from_i128(2).odd());
  assert!(FeelNumber::from_i128(3).odd());
  assert!(!FeelNumber::from_i128(4).odd());
  assert!(!FeelNumber::new(31, 1).odd());
}

#[test]
fn test_pow() {
  assert!(FeelNumber::from_i128(0).pow(&FeelNumber::from_i128(0)).is_none());
  assert_eq!(
    "41959.85737359436186095331070746801",
    FeelNumber::new(122384283, 7).pow(&FeelNumber::new(425, 2)).unwrap().to_string()
  );
}

#[test]
fn test_rem() {
  assert_eq!("2", (FeelNumber::new(12, 0) % FeelNumber::new(5, 0)).to_string());
  assert_eq!("3", (FeelNumber::new(-12, 0) % FeelNumber::new(5, 0)).to_string());
  assert_eq!("-3", (FeelNumber::new(12, 0) % FeelNumber::new(-5, 0)).to_string());
  assert_eq!("-2", (FeelNumber::new(-12, 0) % FeelNumber::new(-5, 0)).to_string());
  assert_eq!("1.1", (FeelNumber::new(101, 1) % FeelNumber::new(45, 1)).to_string());
  assert_eq!("3.4", (FeelNumber::new(-101, 1) % FeelNumber::new(45, 1)).to_string());
  assert_eq!("-3.4", (FeelNumber::new(101, 1) % FeelNumber::new(-45, 1)).to_string());
  assert_eq!("-1.1", (FeelNumber::new(-101, 1) % FeelNumber::new(-45, 1)).to_string());
}

#[test]
fn test_rem_assign() {
  let mut x = FeelNumber::new(101, 1);
  x %= FeelNumber::new(-45, 1);
  assert_eq!("-3.4", x.to_string());
}

#[test]
fn test_round() {
  assert_eq!("123.46", FeelNumber::new(1234567, 4).round(&FeelNumber::from_i128(2)).to_string());
  assert_eq!("123.45", FeelNumber::new(1234547, 4).round(&FeelNumber::from_i128(2)).to_string());
  assert_eq!("100", FeelNumber::new(1234567, 4).round(&FeelNumber::from_i128(-2)).to_string());
  assert_eq!("200", FeelNumber::new(1634567, 4).round(&FeelNumber::from_i128(-2)).to_string());
}

#[test]
fn test_sqrt() {
  assert!(FeelNumber::from_i128(-1).sqrt().is_none());
  assert_eq!("0", FeelNumber::from_i128(0).sqrt().unwrap().to_string());
  assert_eq!("1", FeelNumber::from_i128(1).sqrt().unwrap().to_string());
  assert_eq!("1.414213562373095048801688724209698", FeelNumber::from_i128(2).sqrt().unwrap().to_string());
}

#[test]
fn test_square() {
  assert_eq!("4", FeelNumber::from_i128(2).square().unwrap().to_string());
  assert_eq!("25", FeelNumber::from_i128(5).square().unwrap().to_string());
  assert_eq!(None, FeelNumber::from_string("NaN").square());
  assert_eq!(None, FeelNumber::from_string("Inf").square());
  assert_eq!(None, FeelNumber::from_string("-Inf").square());
}

#[test]
fn test_sub() {
  assert_eq!("1", (FeelNumber::new(123, 2) - FeelNumber::new(23, 2)).to_string());
}

#[test]
fn test_sub_assign() {
  let mut x = FeelNumber::new(123, 2);
  x -= FeelNumber::new(23, 2);
  assert_eq!("1", x.to_string());
}

#[test]
fn test_try_from_number_to_isize() {
  assert!(isize::try_from(FeelNumber::from_i128(2)).is_ok());
  assert!(isize::try_from(FeelNumber::from_i128(isize::MAX as i128)).is_ok());
  assert!(isize::try_from(FeelNumber::from_i128(isize::MIN as i128)).is_ok());
  assert!(isize::try_from(FeelNumber::from_i128(i128::MAX)).is_err());
  assert!(isize::try_from(FeelNumber::from_i128(i128::MIN)).is_err());
}

#[test]
fn test_try_from_number_to_usize() {
  assert!(usize::try_from(FeelNumber::from_i128(0)).is_ok());
  assert!(usize::try_from(FeelNumber::from_i128(2)).is_ok());
  assert!(usize::try_from(FeelNumber::from_usize(usize::MAX)).is_ok());
  assert!(usize::try_from(FeelNumber::from_usize(usize::MIN)).is_ok());
  assert!(usize::try_from(FeelNumber::from_isize(isize::MAX)).is_ok());
  assert!(usize::try_from(FeelNumber::from_isize(isize::MIN)).is_err());
  assert!(usize::try_from(FeelNumber::from_i128(i128::MAX)).is_err());
  assert!(usize::try_from(FeelNumber::from_i128(i128::MIN)).is_err());
  assert!(usize::try_from(FeelNumber::from_i128(-1)).is_err());
  assert_eq!(0, usize::try_from(FeelNumber::from_i128(0)).unwrap());
  assert_eq!(2, usize::try_from(FeelNumber::from_i128(2)).unwrap());
  assert_eq!(usize::MAX, usize::try_from(FeelNumber::from_usize(usize::MAX)).unwrap());
  assert_eq!(usize::MIN, usize::try_from(FeelNumber::from_usize(usize::MIN)).unwrap());
  assert_eq!(usize::MAX / 2, usize::try_from(FeelNumber::from_isize(isize::MAX)).unwrap());
}

#[test]
fn test_try_from_number_to_u64() {
  assert!(u64::try_from(FeelNumber::from_i128(0)).is_ok());
  assert!(u64::try_from(FeelNumber::from_i128(2)).is_ok());
  assert!(u64::try_from(FeelNumber::from_usize(usize::MAX)).is_ok());
  assert!(u64::try_from(FeelNumber::from_usize(usize::MIN)).is_ok());
  assert!(u64::try_from(FeelNumber::from_isize(isize::MAX)).is_ok());
  assert!(u64::try_from(FeelNumber::from_isize(isize::MIN)).is_err());
  assert!(u64::try_from(FeelNumber::from_i128(i128::MAX)).is_err());
  assert!(u64::try_from(FeelNumber::from_i128(i128::MIN)).is_err());
  assert!(u64::try_from(FeelNumber::from_i128(-1)).is_err());
  assert_eq!(300_000_000_u64, u64::try_from(FeelNumber::from_i128(300_000_000)).unwrap());
}

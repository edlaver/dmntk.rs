mod common;

use dmntk_feel_number::FeelNumber;

#[test]
fn test_comparison_001() {
  assert!((num!(120000) == num!(120000)));
}

#[test]
fn test_comparison_002() {
  assert!(!(num!(0) > num!(0)));
}

#[test]
fn test_comparison_003() {
  assert!((num!(1234.56) == num!(1234.56)));
  assert!(!(num!(1234.56) == num!(-1234.56)));
  assert!((num!(1234.56) > num!(123.456)));
  assert!((num!(123.456) < num!(1234.56)));
  assert!((num!(1234.56) <= num!(1234.56)));
  assert!((num!(1234.56) >= num!(1234.56)));
  assert!((11_isize > num!(10)));
  assert!((num!(11) > 10));
  assert!((-6111..6176).contains(&num!(0)));
  assert!((0..6176).contains(&num!(6175)));
  assert!((-3..2).contains(&num!(-3)));
  assert!(!(-3..2).contains(&num!(-4)));
  assert!((0..60).contains(&num!(00)));
  assert!((0..60).contains(&num!(59.999999999)));
  assert!(!(0..60).contains(&num!(60)));
  assert!((num!(0) == num!(0)));
  assert!(!(num!(0) == num!(1)));
  assert!(!(num!(1) == num!(0)));
  assert!((num!(0) == 0_isize));
  assert!((num!(1) == 1_isize));
  assert!((num!(-1) == -1_isize));
  assert!((0_isize == num!(0)));
  assert!((1_isize == num!(1)));
  assert!((-1_isize == num!(-1)));
}

mod common;

use dmntk_feel_number::FeelNumber;

#[test]
fn test_mul_001() {
  eqs!("12", num!(1.2) * num!(10));
}

#[test]
fn test_mul_002() {
  let mut x = FeelNumber::new(12, 1);
  x *= FeelNumber::new(10, 0);
  eqs!("12", x);
}

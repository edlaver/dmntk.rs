use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_mul_001() {
  assert_eq!("12", num!(1.2) * num!(10));
}

#[test]
fn test_mul_002() {
  let mut x = FeelNumber::new(12, 1);
  x *= FeelNumber::new(10, 0);
  assert_eq!("12", x);
}

use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_div_001() {
  assert_eq!("2.5", num!(20) / num!(8));
}

#[test]
fn test_div_002() {
  let mut x = FeelNumber::new(20, 0);
  x /= FeelNumber::new(8, 0);
  assert_eq!("2.5", x);
}

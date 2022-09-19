use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_add_001() {
  assert_eq!("0.4", num!(0.1) + num!(0.3));
}

#[test]
fn test_add_002() {
  assert_eq!("123.8347847", FeelNumber::new(12345, 2) + FeelNumber::new(3847847, 7));
}

#[test]
fn test_add_003() {
  assert_eq!("8.93", FeelNumber::new(123, 2) + FeelNumber::new(77, 1));
}

#[test]
fn test_add_004() {
  let mut x = FeelNumber::new(1, 1);
  x += FeelNumber::new(3, 1);
  assert_eq!("0.4", x);
}

#[test]
fn test_add_005() {
  let mut x = FeelNumber::new(12345, 2);
  x += FeelNumber::new(3847847, 7);
  assert_eq!("123.8347847", x);
}

#[test]
fn test_add_006() {
  let mut x = FeelNumber::new(123, 2);
  x += FeelNumber::new(77, 1);
  assert_eq!("8.93", x);
}

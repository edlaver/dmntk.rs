use dmntk_feel_number::FeelNumber;

#[test]
fn test_rem_001() {
  assert_eq!("2", FeelNumber::new(12, 0) % FeelNumber::new(5, 0));
}

#[test]
fn test_rem_002() {
  assert_eq!("3", FeelNumber::new(-12, 0) % FeelNumber::new(5, 0));
}

#[test]
fn test_rem_003() {
  assert_eq!("-3", FeelNumber::new(12, 0) % FeelNumber::new(-5, 0));
}

#[test]
fn test_rem_004() {
  assert_eq!("-2", FeelNumber::new(-12, 0) % FeelNumber::new(-5, 0));
}

#[test]
fn test_rem_005() {
  assert_eq!("1.1", FeelNumber::new(101, 1) % FeelNumber::new(45, 1));
}

#[test]
fn test_rem_006() {
  assert_eq!("3.4", FeelNumber::new(-101, 1) % FeelNumber::new(45, 1));
}

#[test]
fn test_rem_007() {
  assert_eq!("-3.4", FeelNumber::new(101, 1) % FeelNumber::new(-45, 1));
}

#[test]
fn test_rem_008() {
  assert_eq!("-1.1", FeelNumber::new(-101, 1) % FeelNumber::new(-45, 1));
}

#[test]
fn test_rem_009() {
  let mut x = FeelNumber::new(101, 1);
  x %= FeelNumber::new(-45, 1);
  assert_eq!("-3.4", x);
}

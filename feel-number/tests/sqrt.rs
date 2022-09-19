use dmntk_feel_number::{num, FeelNumber};

#[test]
fn test_sqrt_001() {
  assert!(num!(-1).sqrt().is_none());
}

#[test]
fn test_sqrt_002() {
  assert_eq!("0", num!(0).sqrt().unwrap());
}

#[test]
fn test_sqrt_003() {
  assert_eq!("1", num!(1).sqrt().unwrap());
}

#[test]
fn test_sqrt_004() {
  assert_eq!("1.414213562373095048801688724209698", num!(2).sqrt().unwrap());
}

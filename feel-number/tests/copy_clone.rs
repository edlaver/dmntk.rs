use dmntk_feel_number::FeelNumber;

#[test]
fn test_copy_001() {
  let x = FeelNumber::new(12345, 2);
  let y = x;
  assert_eq!("123.45", y);
}

#[test]
#[allow(clippy::clone_on_copy)]
fn test_clone_001() {
  let x = FeelNumber::new(12345, 2);
  let y = x.clone();
  assert_eq!("123.45", y);
}

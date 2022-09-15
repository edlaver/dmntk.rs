#![feature(test)]

extern crate test;

use dmntk_feel_number::{FeelNumber, FEEL_NUMBER_ZERO};
use test::Bencher;

#[bench]
#[allow(clippy::clone_on_copy)]
fn bench_copy_001(b: &mut Bencher) {
  let x = FeelNumber::new(123456789, 4);
  let mut y = FEEL_NUMBER_ZERO;
  b.iter(|| {
    y = x;
    y
  })
}

#[bench]
#[allow(clippy::clone_on_copy)]
fn bench_clone_001(b: &mut Bencher) {
  let x = FeelNumber::new(123456789, 4);
  let mut y = FEEL_NUMBER_ZERO;
  b.iter(|| {
    y = x.clone();
    y
  })
}

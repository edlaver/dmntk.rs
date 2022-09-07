#![feature(test)]

extern crate test;

use dmntk_feel_number::FeelNumber;
use test::Bencher;

#[bench]
fn bench_copy_001(b: &mut Bencher) {
  let x = FeelNumber::new(123456789, 4);
  let mut y = FeelNumber::zero();
  b.iter(|| {
    y = x;
    y
  })
}

#[bench]
fn bench_clone_001(b: &mut Bencher) {
  let x = FeelNumber::new(123456789, 4);
  let mut y = FeelNumber::zero();
  b.iter(|| {
    y = x.clone();
    y
  })
}

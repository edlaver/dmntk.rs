/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! `FEEL` number type.

use crate::dec::*;
use crate::errors::*;
use dmntk_common::{DmntkError, Jsonify};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::str::FromStr;

macro_rules! try_from_feel_number {
  ($l:tt) => {
    impl TryFrom<FeelNumber> for $l {
      type Error = DmntkError;
      fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
        $l::try_from(&value)
      }
    }

    impl TryFrom<&FeelNumber> for $l {
      type Error = DmntkError;
      fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
        return value.to_string().parse::<$l>().map_err(|_| err_number_conversion_failed());
      }
    }
  };
}

/// FEEL number.
#[derive(Copy, Clone)]
pub struct FeelNumber(DecQuad);

impl FeelNumber {
  /// Creates a new [FeelNumber] from integer value and scale.
  pub fn new(n: i128, s: i32) -> Self {
    Self(dec_scale_b(&dec_quad_from_string(&format!("{}", n)), &dec_quad_from_string(&format!("{}", -s))))
  }
  /// Creates a new [FeelNumber] from [isize].
  fn from_isize(n: isize) -> Self {
    Self(dec_quad_from_string(&format!("{}", n)))
  }
  /// Creates a new [FeelNumber] from [i128].
  fn from_i128(n: i128) -> Self {
    Self(dec_quad_from_string(&format!("{}", n)))
  }
  ///
  pub fn zero() -> Self {
    Self(*DEC_ZERO)
  }
  ///
  pub fn one() -> Self {
    Self(*DEC_ONE)
  }
  ///
  pub fn two() -> Self {
    Self(*DEC_TWO)
  }
  ///
  pub fn nano() -> Self {
    Self(*DEC_NANO)
  }
  ///
  pub fn abs(&self) -> Self {
    Self(dec_quad_abs(&self.0))
  }
  ///
  pub fn ceiling(&self) -> Self {
    Self(dec_reduce(&dec_ceiling(&self.0)))
  }
  ///
  pub fn even(&self) -> bool {
    dec_is_zero(&dec_remainder(&self.0, &DEC_TWO))
  }
  ///
  pub fn exp(&self) -> Self {
    Self(dec_exp(&self.0))
  }
  ///
  pub fn floor(&self) -> Self {
    Self(dec_reduce(&dec_floor(&self.0)))
  }
  ///
  pub fn frac(&self) -> Self {
    Self(dec_fract(&self.0))
  }
  ///
  pub fn is_integer(&self) -> bool {
    dec_is_integer(&self.0)
  }
  ///
  pub fn is_one(&self) -> bool {
    dec_is_zero(&dec_compare(&self.0, &DEC_ONE))
  }
  ///
  pub fn is_negative(&self) -> bool {
    dec_is_negative(&self.0)
  }
  ///
  pub fn is_positive(&self) -> bool {
    dec_is_positive(&self.0)
  }
  ///
  pub fn ln(&self) -> Option<Self> {
    let n = dec_ln(&self.0);
    if dec_is_finite(&n) {
      Some(Self(dec_reduce(&n)))
    } else {
      None
    }
  }
  ///
  pub fn odd(&self) -> bool {
    dec_is_integer(&self.0) && !dec_is_zero(&dec_remainder(&self.0, &DEC_TWO))
  }
  ///
  pub fn pow(&self, rhs: &FeelNumber) -> Option<Self> {
    let n = dec_power(&self.0, &rhs.0);
    if dec_is_finite(&n) {
      Some(Self(dec_reduce(&n)))
    } else {
      None
    }
  }
  ///
  pub fn round(&self, rhs: &FeelNumber) -> Self {
    Self(dec_rescale(&self.0, &dec_minus(&rhs.0)))
  }
  ///
  pub fn sqrt(&self) -> Option<Self> {
    let n = dec_square_root(&self.0);
    if dec_is_finite(&n) {
      Some(Self(dec_reduce(&n)))
    } else {
      None
    }
  }
  ///
  pub fn square(&self) -> Option<Self> {
    let n = dec_power(&self.0, &DEC_TWO);
    if dec_is_finite(&n) {
      Some(Self(dec_reduce(&n)))
    } else {
      None
    }
  }
  ///
  pub fn trunc(&self) -> Self {
    Self(dec_trunc(&self.0))
  }
}

impl PartialEq<FeelNumber> for FeelNumber {
  fn eq(&self, rhs: &Self) -> bool {
    dec_is_zero(&dec_compare(&self.0, &rhs.0))
  }
}

impl PartialEq<FeelNumber> for isize {
  fn eq(&self, rhs: &FeelNumber) -> bool {
    dec_is_zero(&dec_compare(&FeelNumber::from_isize(*self).0, &rhs.0))
  }
}

impl PartialEq<isize> for FeelNumber {
  fn eq(&self, rhs: &isize) -> bool {
    dec_is_zero(&dec_compare(&self.0, &FeelNumber::from_isize(*rhs).0))
  }
}

impl PartialOrd<FeelNumber> for FeelNumber {
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    let flag = dec_compare(&self.0, &rhs.0);
    if dec_is_zero(&flag) {
      return Some(Ordering::Equal);
    }
    if dec_is_positive(&flag) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl PartialOrd<FeelNumber> for isize {
  fn partial_cmp(&self, rhs: &FeelNumber) -> Option<Ordering> {
    let flag = dec_compare(&FeelNumber::from_isize(*self).0, &rhs.0);
    if dec_is_zero(&flag) {
      return Some(Ordering::Equal);
    }
    if dec_is_positive(&flag) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl PartialOrd<isize> for FeelNumber {
  fn partial_cmp(&self, rhs: &isize) -> Option<Ordering> {
    let flag = dec_compare(&self.0, &FeelNumber::from_isize(*rhs).0);
    if dec_is_zero(&flag) {
      return Some(Ordering::Equal);
    }
    if dec_is_positive(&flag) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl Add<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn add(self, rhs: Self) -> Self::Output {
    Self(dec_add(&self.0, &rhs.0))
  }
}

impl AddAssign<FeelNumber> for FeelNumber {
  ///
  fn add_assign(&mut self, rhs: Self) {
    self.0 = dec_add(&self.0, &rhs.0);
  }
}

impl Sub<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn sub(self, rhs: Self) -> Self::Output {
    Self(dec_reduce(&dec_subtract(&self.0, &rhs.0)))
  }
}

impl SubAssign<FeelNumber> for FeelNumber {
  ///
  fn sub_assign(&mut self, rhs: Self) {
    self.0 = dec_reduce(&dec_subtract(&self.0, &rhs.0));
  }
}

impl Mul<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn mul(self, rhs: Self) -> Self::Output {
    Self(dec_reduce(&dec_multiply(&self.0, &rhs.0)))
  }
}

impl MulAssign<FeelNumber> for FeelNumber {
  ///
  fn mul_assign(&mut self, rhs: Self) {
    self.0 = dec_reduce(&dec_multiply(&self.0, &rhs.0));
  }
}

impl Div<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn div(self, rhs: Self) -> Self::Output {
    Self(dec_reduce(&dec_divide(&self.0, &rhs.0)))
  }
}

impl DivAssign<FeelNumber> for FeelNumber {
  ///
  fn div_assign(&mut self, rhs: Self) {
    self.0 = dec_reduce(&dec_divide(&self.0, &rhs.0));
  }
}

impl Rem<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn rem(self, rhs: Self) -> Self::Output {
    Self(dec_reduce(&dec_subtract(
      &self.0,
      &dec_multiply(&rhs.0, &dec_floor(&dec_divide(&self.0, &rhs.0))),
    )))
  }
}

impl RemAssign<FeelNumber> for FeelNumber {
  ///
  fn rem_assign(&mut self, rhs: Self) {
    self.0 = dec_reduce(&dec_subtract(&self.0, &dec_multiply(&rhs.0, &dec_floor(&dec_divide(&self.0, &rhs.0)))));
  }
}

impl Neg for FeelNumber {
  type Output = Self;
  ///
  fn neg(self) -> Self::Output {
    Self(dec_minus(&self.0))
  }
}

impl Debug for FeelNumber {
  ///
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", dec_quad_to_string(&self.0))
  }
}

impl Display for FeelNumber {
  /// Converts [FeelNumber] to string.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = dec_quad_to_string(&self.0);
    let zeroes: String;
    if s.contains("E+") {
      let mut split1 = s.split("E+");
      let before_exponent = split1.next().unwrap();
      let after_exponent = split1.next().unwrap();
      let exponent_digits = usize::from_str(after_exponent).unwrap();
      if before_exponent.contains('.') {
        let mut split2 = before_exponent.split('.');
        let before_decimal = split2.next().unwrap();
        let after_decimal = split2.next().unwrap();
        zeroes = (0..(exponent_digits - after_decimal.len())).map(|_| "0").collect();
        write!(f, "{}{}{}", before_decimal, after_decimal, zeroes)
      } else {
        zeroes = (0..exponent_digits).map(|_| "0").collect();
        write!(f, "{}{}", before_exponent, zeroes)
      }
    } else if s.contains("E-") {
      let mut split1 = s.split("E-");
      let before_exponent = split1.next().unwrap();
      let after_exponent = split1.next().unwrap();
      let exponent_digits = usize::from_str(after_exponent).unwrap();
      if before_exponent.contains('.') {
        let mut split2 = before_exponent.split('.');
        let before_decimal = split2.next().unwrap();
        let after_decimal = split2.next().unwrap();
        zeroes = (1..exponent_digits).map(|_| "0").collect();
        write!(f, "0.{}{}{}", zeroes, before_decimal, after_decimal)
      } else {
        zeroes = (1..exponent_digits).map(|_| "0").collect();
        write!(f, "0.{}{}", zeroes, before_exponent)
      }
    } else {
      write!(f, "{}", s)
    }
  }
}

impl Jsonify for FeelNumber {
  /// Converts [FeelNumber] to JSON string.
  fn jsonify(&self) -> String {
    format!("{}", self)
  }
}

impl FromStr for FeelNumber {
  type Err = DmntkError;
  ///
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let n = dec_quad_from_string(s);
    if dec_is_finite(&n) {
      Ok(Self(n))
    } else {
      Err(err_invalid_number_literal(s))
    }
  }
}

impl From<u8> for FeelNumber {
  ///
  fn from(value: u8) -> Self {
    Self(dec_quad_from_u32(value as u32))
  }
}

impl From<i8> for FeelNumber {
  ///
  fn from(value: i8) -> Self {
    Self(dec_quad_from_i32(value as i32))
  }
}

impl From<u16> for FeelNumber {
  ///
  fn from(value: u16) -> Self {
    Self(dec_quad_from_u32(value as u32))
  }
}

impl From<i16> for FeelNumber {
  ///
  fn from(value: i16) -> Self {
    Self(dec_quad_from_i32(value as i32))
  }
}

impl From<u32> for FeelNumber {
  ///
  fn from(value: u32) -> Self {
    Self(dec_quad_from_u32(value))
  }
}

impl From<i32> for FeelNumber {
  ///
  fn from(value: i32) -> Self {
    Self(dec_quad_from_i32(value))
  }
}

impl From<i64> for FeelNumber {
  ///
  fn from(value: i64) -> Self {
    Self::from_i128(value as i128)
  }
}

impl From<isize> for FeelNumber {
  ///
  fn from(value: isize) -> Self {
    Self::from_i128(value as i128)
  }
}

impl From<usize> for FeelNumber {
  ///
  fn from(value: usize) -> Self {
    Self::from_i128(value as i128)
  }
}

try_from_feel_number!(isize);
try_from_feel_number!(usize);
try_from_feel_number!(u64);
try_from_feel_number!(u32);
try_from_feel_number!(i32);
try_from_feel_number!(u8);

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
//!
//! Implementation of the `FEEL` number based on
//! **Intel(R) Decimal Floating-Point Math Library**.

use crate::errors::*;
use dfp_number_sys::*;
use dmntk_common::{DmntkError, Jsonify};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::str::FromStr;

/// Flags for operation status.
macro_rules! flags {
  () => {
    &mut 0_u32
  };
}

/// Rounding mode.
macro_rules! round {
  () => {
    RM_NEAREST_EVEN
  };
}

/// FEEL number.
#[derive(Copy, Clone)]
pub struct FeelNumber(BID128, bool);

impl FeelNumber {
  /// Returns [FeelNumber] value 0 (zero).
  pub fn zero() -> FeelNumber {
    Self(bid128_from_uint64(0), false)
  }
  /// Returns [FeelNumber] value 1 (one).
  pub fn one() -> FeelNumber {
    Self(bid128_from_uint64(1), false)
  }
  /// Returns [FeelNumber] value 2 (two).
  pub fn two() -> FeelNumber {
    Self(bid128_from_uint64(2), false)
  }
  /// Returns [FeelNumber] value 1000000000 (billion).
  pub fn billion() -> FeelNumber {
    Self(bid128_from_uint64(1_000_000_000), false)
  }
  /// Creates a new [FeelNumber] from integer value and a scale.
  pub fn new(n: i64, s: i32) -> Self {
    Self(bid128_scalbn(bid128_from_int64(n), -s), false)
  }
  /// Creates a new [FeelNumber] from [isize].
  fn from_isize(n: isize) -> Self {
    Self(bid128_from_string(&format!("{}", n), round!(), flags!()), false)
  }
  /// Creates a new [FeelNumber] from [i128].
  fn from_i128(n: i128) -> Self {
    Self(bid128_from_string(&format!("{}", n), round!(), flags!()), false)
  }
  /// Creates a new [FeelNumber] from [u128].
  fn from_u128(n: u128) -> Self {
    Self(bid128_from_string(&format!("{}", n), round!(), flags!()), false)
  }
  /// Returns an absolute value of this [FeelNumber].
  pub fn abs(&self) -> Self {
    Self(bid128_abs(self.0), false)
  }
  /// Returns a nearest integer greater than this [FeelNumber].
  pub fn ceiling(&self) -> Self {
    let bid = bid128_round_integral_positive(self.0, flags!());
    if bid128_is_zero(bid) {
      Self::zero()
    } else {
      Self(bid, false)
    }
  }
  ///
  pub fn even(&self) -> bool {
    bid128_is_zero(bid128_rem(self.0, Self::two().0, flags!()))
  }
  ///
  pub fn exp(&self) -> Self {
    Self(bid128_exp(self.0, round!(), flags!()), true)
  }
  ///
  pub fn floor(&self) -> Self {
    Self(bid128_round_integral_negative(self.0, flags!()), false)
  }
  ///
  pub fn frac(&self) -> Self {
    Self(bid128_sub(self.0, bid128_round_integral_zero(self.0, flags!()), round!(), flags!()), true)
  }
  ///
  pub fn is_integer(&self) -> bool {
    bid128_quiet_equal(self.0, bid128_round_integral_zero(self.0, flags!()), flags!())
  }
  ///
  pub fn is_one(&self) -> bool {
    bid128_quiet_equal(self.0, Self::one().0, flags!())
  }
  ///
  pub fn is_negative(&self) -> bool {
    bid128_quiet_less(self.0, Self::zero().0, flags!())
  }
  ///
  pub fn is_positive(&self) -> bool {
    bid128_quiet_greater(self.0, Self::zero().0, flags!())
  }
  ///
  pub fn ln(&self) -> Option<Self> {
    let n = bid128_log(self.0, round!(), flags!());
    if bid128_is_finite(n) {
      Some(Self(n, true))
    } else {
      None
    }
  }
  ///
  pub fn odd(&self) -> bool {
    if self.is_integer() {
      !bid128_is_zero(bid128_rem(self.0, Self::two().0, flags!()))
    } else {
      false
    }
  }
  ///
  pub fn pow(&self, rhs: &FeelNumber) -> Option<Self> {
    let n = bid128_pow(self.0, rhs.0, round!(), flags!());
    if bid128_is_finite(n) {
      Some(Self(n, true))
    } else {
      None
    }
  }
  ///
  pub fn round(&self, rhs: &FeelNumber) -> Self {
    let r = bid128_negate(rhs.0);
    let n = bid128_to_int32_int(r, flags!());
    let q = bid128_scalbn(Self::one().0, n);
    Self(bid128_quantize(self.0, q, round!(), flags!()), false)
  }
  ///
  pub fn sqrt(&self) -> Option<Self> {
    let n = bid128_sqrt(self.0, round!(), flags!());
    if bid128_is_finite(n) {
      Some(Self(n, true))
    } else {
      None
    }
  }
  ///
  pub fn square(&self) -> Option<Self> {
    let n = bid128_pow(self.0, Self::two().0, round!(), flags!());
    if bid128_is_finite(n) {
      Some(Self(n, true))
    } else {
      None
    }
  }
  ///
  pub fn trunc(&self) -> Self {
    Self(bid128_round_integral_zero(self.0, flags!()), false)
  }
  /// Calculates the remainder of the division.
  fn remainder(&self, rhs: BID128) -> BID128 {
    let mut n = bid128_div(self.0, rhs, round!(), flags!());
    n = bid128_round_integral_negative(n, flags!());
    n = bid128_mul(rhs, n, round!(), flags!());
    bid128_sub(self.0, n, round!(), flags!())
  }
}

impl PartialEq<FeelNumber> for FeelNumber {
  fn eq(&self, rhs: &Self) -> bool {
    bid128_quiet_equal(self.0, rhs.0, flags!())
  }
}

impl PartialEq<FeelNumber> for isize {
  fn eq(&self, rhs: &FeelNumber) -> bool {
    bid128_quiet_equal(FeelNumber::from_isize(*self).0, rhs.0, flags!())
  }
}

impl PartialEq<isize> for FeelNumber {
  fn eq(&self, rhs: &isize) -> bool {
    bid128_quiet_equal(self.0, FeelNumber::from_isize(*rhs).0, flags!())
  }
}

impl PartialOrd<FeelNumber> for FeelNumber {
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    if bid128_quiet_equal(self.0, rhs.0, flags!()) {
      return Some(Ordering::Equal);
    }
    if bid128_quiet_greater(self.0, rhs.0, flags!()) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl PartialOrd<FeelNumber> for isize {
  fn partial_cmp(&self, rhs: &FeelNumber) -> Option<Ordering> {
    let n = FeelNumber::from_isize(*self).0;
    if bid128_quiet_equal(n, rhs.0, flags!()) {
      return Some(Ordering::Equal);
    }
    if bid128_quiet_greater(n, rhs.0, flags!()) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl PartialOrd<isize> for FeelNumber {
  fn partial_cmp(&self, rhs: &isize) -> Option<Ordering> {
    let n = FeelNumber::from_isize(*rhs).0;
    if bid128_quiet_equal(self.0, n, flags!()) {
      return Some(Ordering::Equal);
    }
    if bid128_quiet_greater(self.0, n, flags!()) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl Add<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn add(self, rhs: Self) -> Self::Output {
    Self(bid128_add(self.0, rhs.0, round!(), flags!()), self.1 | rhs.1)
  }
}

impl AddAssign<FeelNumber> for FeelNumber {
  ///
  fn add_assign(&mut self, rhs: Self) {
    self.0 = bid128_add(self.0, rhs.0, round!(), flags!());
    self.1 |= rhs.1;
  }
}

impl Sub<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn sub(self, rhs: Self) -> Self::Output {
    Self(bid128_sub(self.0, rhs.0, round!(), flags!()), self.1 | rhs.1)
  }
}

impl SubAssign<FeelNumber> for FeelNumber {
  ///
  fn sub_assign(&mut self, rhs: Self) {
    self.0 = bid128_sub(self.0, rhs.0, round!(), flags!());
    self.1 |= rhs.1;
  }
}

impl Mul<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn mul(self, rhs: Self) -> Self::Output {
    Self(bid128_mul(self.0, rhs.0, round!(), flags!()), true)
  }
}

impl MulAssign<FeelNumber> for FeelNumber {
  ///
  fn mul_assign(&mut self, rhs: Self) {
    self.0 = bid128_mul(self.0, rhs.0, round!(), flags!());
    self.1 = true;
  }
}

impl Div<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn div(self, rhs: Self) -> Self::Output {
    Self(bid128_div(self.0, rhs.0, round!(), flags!()), true)
  }
}

impl DivAssign<FeelNumber> for FeelNumber {
  ///
  fn div_assign(&mut self, rhs: Self) {
    self.0 = bid128_div(self.0, rhs.0, round!(), flags!());
    self.1 = true;
  }
}

impl Rem<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn rem(self, rhs: Self) -> Self::Output {
    Self(self.remainder(rhs.0), false)
  }
}

impl RemAssign<FeelNumber> for FeelNumber {
  ///
  fn rem_assign(&mut self, rhs: Self) {
    self.0 = self.remainder(rhs.0)
  }
}

impl Neg for FeelNumber {
  type Output = Self;
  ///
  fn neg(self) -> Self::Output {
    Self(bid128_negate(self.0), false)
  }
}

impl Debug for FeelNumber {
  ///
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", bid128_to_string(self.0, flags!()))
  }
}

impl Display for FeelNumber {
  /// Converts [FeelNumber] to human readable string.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = bid128_to_string(self.0, flags!());
    let negative = s.starts_with('-');
    let mut split = s[1..].split('E');
    let (sb, sa) = split.next().zip(split.next()).unwrap(); // unwrap is ok, there is always E present
    let exponent = sa.parse::<isize>().unwrap(); // unwrap is ok, there is always correct exponent present
    let decimal_points = exponent.unsigned_abs();
    let (mut before, mut after) = if exponent < 0 {
      let digit_count = sb.len();
      if digit_count <= decimal_points {
        let before = "0".to_string();
        let mut after = "0".repeat(decimal_points - digit_count);
        if self.1 {
          after.push_str(sb.trim_end_matches('0'));
        } else {
          after.push_str(sb);
        }
        (before, after)
      } else {
        let before = sb[..digit_count - decimal_points].to_string();
        let after = if self.1 {
          sb[digit_count - decimal_points..].trim_end_matches('0').to_string()
        } else {
          sb[digit_count - decimal_points..].to_string()
        };
        (before, after)
      }
    } else {
      let mut before = sb.to_string();
      before.push_str(&"0".repeat(decimal_points));
      let after = "".to_string();
      (before, after)
    };
    if let Some(precision) = f.precision() {
      if after.len() < precision {
        after.push_str(&"0".repeat(precision - after.len()));
      } else {
        after = after[0..precision].to_string();
      }
    }
    if !after.is_empty() {
      before.push('.');
      before.push_str(&after);
    }
    return f.pad_integral(!negative, "", &before);
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
    let n = bid128_from_string(s, round!(), flags!());
    if bid128_is_finite(n) {
      Ok(Self(n, false))
    } else {
      Err(err_invalid_number_literal(s))
    }
  }
}

impl From<u8> for FeelNumber {
  ///
  fn from(value: u8) -> Self {
    Self(bid128_from_uint32(value as u32), false)
  }
}

impl From<i8> for FeelNumber {
  ///
  fn from(value: i8) -> Self {
    Self(bid128_from_int32(value as i32), false)
  }
}

impl From<u16> for FeelNumber {
  ///
  fn from(value: u16) -> Self {
    Self(bid128_from_uint32(value as u32), false)
  }
}

impl From<i16> for FeelNumber {
  ///
  fn from(value: i16) -> Self {
    Self(bid128_from_int32(value as i32), false)
  }
}

impl From<u32> for FeelNumber {
  ///
  fn from(value: u32) -> Self {
    Self(bid128_from_uint32(value), false)
  }
}

impl From<i32> for FeelNumber {
  ///
  fn from(value: i32) -> Self {
    Self(bid128_from_int32(value), false)
  }
}

impl From<u64> for FeelNumber {
  ///
  fn from(value: u64) -> Self {
    Self::from_u128(value as u128)
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
    Self::from_u128(value as u128)
  }
}

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

try_from_feel_number!(isize);
try_from_feel_number!(usize);
try_from_feel_number!(u64);
try_from_feel_number!(u32);
try_from_feel_number!(i32);
try_from_feel_number!(u8);

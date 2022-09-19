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

use crate::errors::*;
use dec_number_sys::*;
use dmntk_common::{DmntkError, Jsonify};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::str::FromStr;

macro_rules! ctx {
  () => {
    &mut dec_context_128()
  };
}

/// [FeelNumber] value 0 (zero).
const FEEL_NUMBER_ZERO: FeelNumber = FeelNumber(DEC_QUAD_ZERO);

/// [FeelNumber] value 1 (one).
const FEEL_NUMBER_ONE: FeelNumber = FeelNumber(DEC_QUAD_ONE);

/// [FeelNumber] value 2 (two).
const FEEL_NUMBER_TWO: FeelNumber = FeelNumber(DEC_QUAD_TWO);

/// [FeelNumber] value 1000000000 (billion).
const FEEL_NUMBER_BILLION: FeelNumber = FeelNumber(DEC_QUAD_BILLION);

/// FEEL number.
#[derive(Copy, Clone)]
pub struct FeelNumber(DecQuad);

impl FeelNumber {
  /// Returns [FeelNumber] value 0 (zero).
  pub fn zero() -> FeelNumber {
    FEEL_NUMBER_ZERO
  }
  /// Returns [FeelNumber] value 1 (one).
  pub fn one() -> FeelNumber {
    FEEL_NUMBER_ONE
  }
  /// Returns [FeelNumber] value 2 (two).
  pub fn two() -> FeelNumber {
    FEEL_NUMBER_TWO
  }
  /// Returns [FeelNumber] value 1000000000 (billion).
  pub fn billion() -> FeelNumber {
    FEEL_NUMBER_BILLION
  }
  /// Creates a new [FeelNumber] from integer value and scale.
  pub fn new(n: i128, s: i32) -> Self {
    Self(dec_quad_scale_b(
      &dec_quad_from_string(&format!("{}", n), ctx!()),
      &dec_quad_from_string(&format!("{}", -s), ctx!()),
      ctx!(),
    ))
  }
  /// Creates a new [FeelNumber] from [isize].
  fn from_isize(n: isize) -> Self {
    Self(dec_quad_from_string(&format!("{}", n), ctx!()))
  }
  /// Creates a new [FeelNumber] from [i128].
  fn from_i128(n: i128) -> Self {
    Self(dec_quad_from_string(&format!("{}", n), ctx!()))
  }
  /// Creates a new [FeelNumber] from [u128].
  fn from_u128(n: u128) -> Self {
    Self(dec_quad_from_string(&format!("{}", n), ctx!()))
  }
  /// Returns an absolute value of this [FeelNumber].
  pub fn abs(&self) -> Self {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadAbs(&mut dq, &self.0, ctx!());
    }
    Self(dq)
  }
  ///
  pub fn ceiling(&self) -> Self {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadToIntegralValue(&mut dq, &self.0, ctx!(), DEC_ROUND_CEILING);
      decQuadReduce(&mut dq, &dq, ctx!());
      decQuadPlus(&mut dq, &dq, ctx!());
    }
    Self(dq)
  }
  ///
  pub fn even(&self) -> bool {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadRemainder(&mut dq, &self.0, &DEC_QUAD_TWO, ctx!());
      decQuadIsZero(&dq) == 1
    }
  }
  ///
  pub fn exp(&self) -> Self {
    let mut dn = DecNumber::default();
    let mut dq = DecQuad::default();
    unsafe {
      decimal128ToNumber(&self.0, &mut dn);
      decNumberExp(&mut dn, &dn, ctx!());
      decimal128FromNumber(&mut dq, &dn, ctx!());
    }
    Self(dq)
  }
  ///
  pub fn floor(&self) -> Self {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadToIntegralValue(&mut dq, &self.0, ctx!(), DEC_ROUND_FLOOR);
      decQuadReduce(&mut dq, &dq, ctx!());
    }
    Self(dq)
  }
  ///
  pub fn frac(&self) -> Self {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadToIntegralValue(&mut dq, &self.0, ctx!(), DEC_ROUND_DOWN);
      decQuadSubtract(&mut dq, &self.0, &dq, ctx!());
    }
    Self(dq)
  }
  ///
  pub fn is_integer(&self) -> bool {
    unsafe { decQuadIsInteger(&self.0) == 1 }
  }
  ///
  pub fn is_one(&self) -> bool {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadCompare(&mut dq, &self.0, &DEC_QUAD_ONE, ctx!());
      decQuadIsZero(&dq) == 1
    }
  }
  ///
  pub fn is_negative(&self) -> bool {
    unsafe { decQuadIsNegative(&self.0) == 1 }
  }
  ///
  pub fn is_positive(&self) -> bool {
    unsafe { decQuadIsPositive(&self.0) == 1 }
  }
  ///
  pub fn ln(&self) -> Option<Self> {
    let mut dn = DecNumber::default();
    let mut dq = DecQuad::default();
    unsafe {
      decimal128ToNumber(&self.0, &mut dn);
      decNumberLn(&mut dn, &dn, ctx!());
      decimal128FromNumber(&mut dq, &dn, ctx!());
      if decQuadIsFinite(&dq) == 1 {
        decQuadReduce(&mut dq, &dq, ctx!());
        Some(Self(dq))
      } else {
        None
      }
    }
  }
  ///
  pub fn odd(&self) -> bool {
    unsafe {
      if decQuadIsInteger(&self.0) == 1 {
        let mut dq = DecQuad::default();
        decQuadRemainder(&mut dq, &self.0, &DEC_QUAD_TWO, ctx!());
        decQuadIsZero(&dq) == 0
      } else {
        false
      }
    }
  }
  ///
  pub fn pow(&self, rhs: &FeelNumber) -> Option<Self> {
    let mut n1 = DecNumber::default();
    let mut n2 = DecNumber::default();
    let mut dq = DecQuad::default();
    unsafe {
      decimal128ToNumber(&self.0, &mut n1);
      decimal128ToNumber(&rhs.0, &mut n2);
      decNumberPower(&mut n1, &n1, &n2, ctx!());
      decimal128FromNumber(&mut dq, &n1, ctx!());
      if decQuadIsFinite(&dq) == 1 {
        decQuadReduce(&mut dq, &dq, ctx!());
        Some(Self(dq))
      } else {
        None
      }
    }
  }
  ///
  pub fn round(&self, rhs: &FeelNumber) -> Self {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadMinus(&mut dq, &rhs.0, ctx!());
      decQuadScaleB(&mut dq, &DEC_QUAD_ONE, &dq, ctx!());
      decQuadQuantize(&mut dq, &self.0, &dq, ctx!());
    }
    Self(dq)
  }
  ///
  pub fn sqrt(&self) -> Option<Self> {
    let mut dn = DecNumber::default();
    let mut dq = DecQuad::default();
    unsafe {
      decimal128ToNumber(&self.0, &mut dn);
      decNumberSquareRoot(&mut dn, &dn, ctx!());
      decimal128FromNumber(&mut dq, &dn, ctx!());
      if decQuadIsFinite(&dq) == 1 {
        decQuadReduce(&mut dq, &dq, ctx!());
        Some(Self(dq))
      } else {
        None
      }
    }
  }
  ///
  pub fn square(&self) -> Option<Self> {
    let mut dn = DecNumber::default();
    let mut dq = DecQuad::default();
    unsafe {
      decimal128ToNumber(&self.0, &mut dn);
      decNumberPower(&mut dn, &dn, &DEC_NUMBER_TWO, ctx!());
      decimal128FromNumber(&mut dq, &dn, ctx!());
      if decQuadIsFinite(&dq) == 1 {
        decQuadReduce(&mut dq, &dq, ctx!());
        Some(Self(dq))
      } else {
        None
      }
    }
  }
  ///
  pub fn trunc(&self) -> Self {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadToIntegralValue(&mut dq, &self.0, ctx!(), DEC_ROUND_DOWN);
    }
    Self(dq)
  }
  /// Calculates the remainder of the division.
  fn remainder(&self, rhs: &DecQuad) -> DecQuad {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadDivide(&mut dq, &self.0, rhs, ctx!());
      decQuadToIntegralValue(&mut dq, &dq, ctx!(), DEC_ROUND_FLOOR);
      decQuadMultiply(&mut dq, rhs, &dq, ctx!());
      decQuadSubtract(&mut dq, &self.0, &dq, ctx!());
      decQuadReduce(&mut dq, &dq, ctx!());
    }
    dq
  }
}

impl PartialEq<FeelNumber> for FeelNumber {
  fn eq(&self, rhs: &Self) -> bool {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadCompare(&mut dq, &self.0, &rhs.0, ctx!());
      decQuadIsZero(&dq) == 1
    }
  }
}

impl PartialEq<FeelNumber> for isize {
  fn eq(&self, rhs: &FeelNumber) -> bool {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadCompare(&mut dq, &FeelNumber::from_isize(*self).0, &rhs.0, ctx!());
      decQuadIsZero(&dq) == 1
    }
  }
}

impl PartialEq<isize> for FeelNumber {
  fn eq(&self, rhs: &isize) -> bool {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadCompare(&mut dq, &self.0, &FeelNumber::from_isize(*rhs).0, ctx!());
      decQuadIsZero(&dq) == 1
    }
  }
}

impl PartialOrd<FeelNumber> for FeelNumber {
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadCompare(&mut dq, &self.0, &rhs.0, ctx!());
      if decQuadIsZero(&dq) == 1 {
        return Some(Ordering::Equal);
      }
      if decQuadIsPositive(&dq) == 1 {
        return Some(Ordering::Greater);
      }
    }
    Some(Ordering::Less)
  }
}

impl PartialOrd<FeelNumber> for isize {
  fn partial_cmp(&self, rhs: &FeelNumber) -> Option<Ordering> {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadCompare(&mut dq, &FeelNumber::from_isize(*self).0, &rhs.0, ctx!());
      if decQuadIsZero(&dq) == 1 {
        return Some(Ordering::Equal);
      }
      if decQuadIsPositive(&dq) == 1 {
        return Some(Ordering::Greater);
      }
    }
    Some(Ordering::Less)
  }
}

impl PartialOrd<isize> for FeelNumber {
  fn partial_cmp(&self, rhs: &isize) -> Option<Ordering> {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadCompare(&mut dq, &self.0, &FeelNumber::from_isize(*rhs).0, ctx!());
      if decQuadIsZero(&dq) == 1 {
        return Some(Ordering::Equal);
      }
      if decQuadIsPositive(&dq) == 1 {
        return Some(Ordering::Greater);
      }
    }
    Some(Ordering::Less)
  }
}

impl Add<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn add(self, rhs: Self) -> Self::Output {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadAdd(&mut dq, &self.0, &rhs.0, ctx!());
      decQuadReduce(&mut dq, &dq, ctx!());
    }
    Self(dq)
  }
}

impl AddAssign<FeelNumber> for FeelNumber {
  ///
  fn add_assign(&mut self, rhs: Self) {
    unsafe {
      decQuadAdd(&mut self.0, &self.0, &rhs.0, ctx!());
      decQuadReduce(&mut self.0, &self.0, ctx!());
    }
  }
}

impl Sub<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn sub(self, rhs: Self) -> Self::Output {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadSubtract(&mut dq, &self.0, &rhs.0, ctx!());
      decQuadReduce(&mut dq, &dq, ctx!());
    }
    Self(dq)
  }
}

impl SubAssign<FeelNumber> for FeelNumber {
  ///
  fn sub_assign(&mut self, rhs: Self) {
    unsafe {
      decQuadSubtract(&mut self.0, &self.0, &rhs.0, ctx!());
      decQuadReduce(&mut self.0, &self.0, ctx!());
    }
  }
}

impl Mul<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn mul(self, rhs: Self) -> Self::Output {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadMultiply(&mut dq, &self.0, &rhs.0, ctx!());
      decQuadReduce(&mut dq, &dq, ctx!());
    }
    Self(dq)
  }
}

impl MulAssign<FeelNumber> for FeelNumber {
  ///
  fn mul_assign(&mut self, rhs: Self) {
    unsafe {
      decQuadMultiply(&mut self.0, &self.0, &rhs.0, ctx!());
      decQuadReduce(&mut self.0, &self.0, ctx!());
    }
  }
}

impl Div<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn div(self, rhs: Self) -> Self::Output {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadDivide(&mut dq, &self.0, &rhs.0, ctx!());
      decQuadReduce(&mut dq, &dq, ctx!());
    }
    Self(dq)
  }
}

impl DivAssign<FeelNumber> for FeelNumber {
  ///
  fn div_assign(&mut self, rhs: Self) {
    unsafe {
      decQuadDivide(&mut self.0, &self.0, &rhs.0, ctx!());
      decQuadReduce(&mut self.0, &self.0, ctx!());
    }
  }
}

impl Rem<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn rem(self, rhs: Self) -> Self::Output {
    Self(self.remainder(&rhs.0))
  }
}

impl RemAssign<FeelNumber> for FeelNumber {
  ///
  fn rem_assign(&mut self, rhs: Self) {
    self.0 = self.remainder(&rhs.0)
  }
}

impl Neg for FeelNumber {
  type Output = Self;
  ///
  fn neg(self) -> Self::Output {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadMinus(&mut dq, &self.0, ctx!());
    }
    Self(dq)
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
    let n = dec_quad_from_string(s, ctx!());
    unsafe {
      if decQuadIsFinite(&n) == 1 {
        Ok(Self(n))
      } else {
        Err(err_invalid_number_literal(s))
      }
    }
  }
}

impl From<u8> for FeelNumber {
  ///
  fn from(value: u8) -> Self {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadFromUInt32(&mut dq, value as u32);
    }
    Self(dq)
  }
}

impl From<i8> for FeelNumber {
  ///
  fn from(value: i8) -> Self {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadFromInt32(&mut dq, value as i32);
    }
    Self(dq)
  }
}

impl From<u16> for FeelNumber {
  ///
  fn from(value: u16) -> Self {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadFromUInt32(&mut dq, value as u32);
    }
    Self(dq)
  }
}

impl From<i16> for FeelNumber {
  ///
  fn from(value: i16) -> Self {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadFromInt32(&mut dq, value as i32);
    }
    Self(dq)
  }
}

impl From<u32> for FeelNumber {
  ///
  fn from(value: u32) -> Self {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadFromUInt32(&mut dq, value as u32);
    }
    Self(dq)
  }
}

impl From<i32> for FeelNumber {
  ///
  fn from(value: i32) -> Self {
    let mut dq = DecQuad::default();
    unsafe {
      decQuadFromInt32(&mut dq, value as i32);
    }
    Self(dq)
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

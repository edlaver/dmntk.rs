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
pub const FEEL_NUMBER_ZERO: FeelNumber = FeelNumber(DEC_QUAD_ZERO);

/// [FeelNumber] value 1 (one).
pub const FEEL_NUMBER_ONE: FeelNumber = FeelNumber(DEC_QUAD_ONE);

/// [FeelNumber] value 2 (two).
pub const FEEL_NUMBER_TWO: FeelNumber = FeelNumber(DEC_QUAD_TWO);

/// [FeelNumber] value 1000000000 (billion).
pub const FEEL_NUMBER_BILLION: FeelNumber = FeelNumber(DEC_QUAD_BILLION);

/// FEEL number.
#[derive(Copy, Clone)]
pub struct FeelNumber(DecQuad);

impl FeelNumber {
  ///
  pub fn zero() -> FeelNumber {
    FEEL_NUMBER_ZERO
  }
  ///
  pub fn one() -> FeelNumber {
    FEEL_NUMBER_ONE
  }
  ///
  pub fn two() -> FeelNumber {
    FEEL_NUMBER_TWO
  }
  ///
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
  ///
  pub fn abs(&self) -> Self {
    Self(dec_quad_abs(&self.0, ctx!()))
  }
  ///
  pub fn ceiling(&self) -> Self {
    Self(dec_quad_plus(
      &dec_quad_reduce(&dec_quad_to_integral_value(&self.0, ctx!(), DEC_ROUND_CEILING), ctx!()),
      ctx!(),
    ))
  }
  ///
  pub fn even(&self) -> bool {
    unsafe {
      let mut dq = DecQuad::default();
      decQuadRemainder(&mut dq, &self.0, &DEC_QUAD_TWO, ctx!());
      decQuadIsZero(&dq) == 1
    }
  }
  ///
  pub fn exp(&self) -> Self {
    unsafe {
      let mut n_res = DecNumber::default();
      let mut dq_res = DecQuad::default();
      decimal128ToNumber(&self.0, &mut n_res);
      decNumberExp(&mut n_res, &n_res, ctx!());
      decimal128FromNumber(&mut dq_res, &n_res, ctx!());
      Self(dq_res)
    }
  }
  ///
  pub fn floor(&self) -> Self {
    Self(dec_quad_reduce(&dec_quad_to_integral_value(&self.0, ctx!(), DEC_ROUND_FLOOR), ctx!()))
  }
  ///
  pub fn frac(&self) -> Self {
    unsafe {
      let mut dq_res = DecQuad::default();
      decQuadToIntegralValue(&mut dq_res, &self.0, ctx!(), DEC_ROUND_DOWN);
      decQuadSubtract(&mut dq_res, &self.0, &dq_res, ctx!());
      Self(dq_res)
    }
  }
  ///
  pub fn is_integer(&self) -> bool {
    dec_quad_is_integer(&self.0)
  }
  ///
  pub fn is_one(&self) -> bool {
    dec_quad_is_zero(&dec_quad_compare(&self.0, &DEC_QUAD_ONE, ctx!()))
  }
  ///
  pub fn is_negative(&self) -> bool {
    dec_quad_is_negative(&self.0)
  }
  ///
  pub fn is_positive(&self) -> bool {
    unsafe { decQuadIsPositive(&self.0) == 1 }
  }
  ///
  pub fn ln(&self) -> Option<Self> {
    unsafe {
      let mut dn = DecNumber::default();
      let mut dq = DecQuad::default();
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
    unsafe {
      //TODO verify why in this case the library returns NaN
      if decQuadIsZero(&self.0) == 1 && decQuadIsZero(&rhs.0) == 1 {
        return Some(Self(DEC_QUAD_ONE));
      }
      let mut n1 = DecNumber::default();
      let mut n2 = DecNumber::default();
      let mut n3 = DecNumber::default();
      let mut dq = DecQuad::default();
      decimal128ToNumber(&self.0, &mut n1);
      decimal128ToNumber(&rhs.0, &mut n2);
      decNumberPower(&mut n3, &n1, &n2, ctx!());
      decimal128FromNumber(&mut dq, &n3, ctx!());
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
    unsafe {
      let mut dq = DecQuad::default();
      decQuadMinus(&mut dq, &rhs.0, ctx!());
      decQuadScaleB(&mut dq, &DEC_QUAD_ONE, &dq, ctx!());
      decQuadQuantize(&mut dq, &self.0, &dq, ctx!());
      Self(dq)
    }
  }
  ///
  pub fn sqrt(&self) -> Option<Self> {
    unsafe {
      let mut dn = DecNumber::default();
      let mut dq = DecQuad::default();
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
    unsafe {
      let mut dn = DecNumber::default();
      let mut dq = DecQuad::default();
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
    unsafe {
      let mut dq = DecQuad::default();
      decQuadToIntegralValue(&mut dq, &self.0, ctx!(), DEC_ROUND_DOWN);
      Self(dq)
    }
  }
  /// Calculates the remainder of the division.
  fn remainder(&self, rhs: &DecQuad) -> DecQuad {
    unsafe {
      let mut dq = DecQuad::default();
      decQuadDivide(&mut dq, &self.0, rhs, ctx!());
      decQuadToIntegralValue(&mut dq, &dq, ctx!(), DEC_ROUND_FLOOR);
      decQuadMultiply(&mut dq, rhs, &dq, ctx!());
      decQuadSubtract(&mut dq, &self.0, &dq, ctx!());
      decQuadReduce(&mut dq, &dq, ctx!());
      dq
    }
  }
}

impl PartialEq<FeelNumber> for FeelNumber {
  fn eq(&self, rhs: &Self) -> bool {
    dec_quad_is_zero(&dec_quad_compare(&self.0, &rhs.0, ctx!()))
  }
}

impl PartialEq<FeelNumber> for isize {
  fn eq(&self, rhs: &FeelNumber) -> bool {
    dec_quad_is_zero(&dec_quad_compare(&FeelNumber::from_isize(*self).0, &rhs.0, ctx!()))
  }
}

impl PartialEq<isize> for FeelNumber {
  fn eq(&self, rhs: &isize) -> bool {
    dec_quad_is_zero(&dec_quad_compare(&self.0, &FeelNumber::from_isize(*rhs).0, ctx!()))
  }
}

impl PartialOrd<FeelNumber> for FeelNumber {
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    let flag = dec_quad_compare(&self.0, &rhs.0, ctx!());
    if dec_quad_is_zero(&flag) {
      return Some(Ordering::Equal);
    }
    if dec_quad_is_positive(&flag) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl PartialOrd<FeelNumber> for isize {
  fn partial_cmp(&self, rhs: &FeelNumber) -> Option<Ordering> {
    let flag = dec_quad_compare(&FeelNumber::from_isize(*self).0, &rhs.0, ctx!());
    if dec_quad_is_zero(&flag) {
      return Some(Ordering::Equal);
    }
    if dec_quad_is_positive(&flag) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl PartialOrd<isize> for FeelNumber {
  fn partial_cmp(&self, rhs: &isize) -> Option<Ordering> {
    let flag = dec_quad_compare(&self.0, &FeelNumber::from_isize(*rhs).0, ctx!());
    if dec_quad_is_zero(&flag) {
      return Some(Ordering::Equal);
    }
    if dec_quad_is_positive(&flag) {
      return Some(Ordering::Greater);
    }
    Some(Ordering::Less)
  }
}

impl Add<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn add(self, rhs: Self) -> Self::Output {
    Self(dec_quad_add(&self.0, &rhs.0, ctx!()))
  }
}

impl AddAssign<FeelNumber> for FeelNumber {
  ///
  fn add_assign(&mut self, rhs: Self) {
    self.0 = dec_quad_add(&self.0, &rhs.0, ctx!());
  }
}

impl Sub<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn sub(self, rhs: Self) -> Self::Output {
    Self(dec_quad_reduce(&dec_quad_subtract(&self.0, &rhs.0, ctx!()), ctx!()))
  }
}

impl SubAssign<FeelNumber> for FeelNumber {
  ///
  fn sub_assign(&mut self, rhs: Self) {
    self.0 = dec_quad_reduce(&dec_quad_subtract(&self.0, &rhs.0, ctx!()), ctx!());
  }
}

impl Mul<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn mul(self, rhs: Self) -> Self::Output {
    Self(dec_quad_reduce(&dec_quad_multiply(&self.0, &rhs.0, ctx!()), ctx!()))
  }
}

impl MulAssign<FeelNumber> for FeelNumber {
  ///
  fn mul_assign(&mut self, rhs: Self) {
    self.0 = dec_quad_reduce(&dec_quad_multiply(&self.0, &rhs.0, ctx!()), ctx!());
  }
}

impl Div<FeelNumber> for FeelNumber {
  type Output = Self;
  ///
  fn div(self, rhs: Self) -> Self::Output {
    Self(dec_quad_reduce(&dec_quad_divide(&self.0, &rhs.0, ctx!()), ctx!()))
  }
}

impl DivAssign<FeelNumber> for FeelNumber {
  ///
  fn div_assign(&mut self, rhs: Self) {
    self.0 = dec_quad_reduce(&dec_quad_divide(&self.0, &rhs.0, ctx!()), ctx!());
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
    Self(dec_quad_minus(&self.0, ctx!()))
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
    if dec_quad_is_finite(&n) {
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

impl TryFrom<FeelNumber> for u8 {
  type Error = DmntkError;
  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    u8::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for u8 {
  type Error = DmntkError;
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let ctx = ctx!();
    let v = dec_quad_to_uint32(&value.0, ctx, DEC_ROUND_DOWN);
    if dec_context_get_status(ctx) != 0 {
      return Err(err_number_conversion_failed());
    }
    u8::try_from(v).map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for i8 {
  type Error = DmntkError;
  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    i8::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for i8 {
  type Error = DmntkError;
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let ctx = ctx!();
    let v = dec_quad_to_int32(&value.0, ctx, DEC_ROUND_DOWN);
    if dec_context_get_status(ctx) != 0 {
      return Err(err_number_conversion_failed());
    }
    i8::try_from(v).map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for u16 {
  type Error = DmntkError;
  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    u16::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for u16 {
  type Error = DmntkError;
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let ctx = ctx!();
    let v = dec_quad_to_uint32(&value.0, ctx, DEC_ROUND_DOWN);
    if dec_context_get_status(ctx) != 0 {
      return Err(err_number_conversion_failed());
    }
    u16::try_from(v).map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for i16 {
  type Error = DmntkError;
  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    i16::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for i16 {
  type Error = DmntkError;
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let ctx = ctx!();
    let v = dec_quad_to_int32(&value.0, ctx, DEC_ROUND_DOWN);
    if dec_context_get_status(ctx) != 0 {
      return Err(err_number_conversion_failed());
    }
    i16::try_from(v).map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for u32 {
  type Error = DmntkError;
  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    u32::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for u32 {
  type Error = DmntkError;
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let ctx = ctx!();
    let v = dec_quad_to_uint32(&value.0, ctx, DEC_ROUND_DOWN);
    if dec_context_get_status(ctx) != 0 {
      return Err(err_number_conversion_failed());
    }
    u32::try_from(v).map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for i32 {
  type Error = DmntkError;
  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    i32::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for i32 {
  type Error = DmntkError;
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let ctx = ctx!();
    let v = dec_quad_to_int32(&value.0, ctx, DEC_ROUND_DOWN);
    if dec_context_get_status(ctx) != 0 {
      return Err(err_number_conversion_failed());
    }
    i32::try_from(v).map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for u64 {
  type Error = DmntkError;
  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    u64::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for u64 {
  type Error = DmntkError;
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let ctx = ctx!();
    let v = dec_quad_to_integral_value(&value.0, ctx, DEC_ROUND_DOWN);
    if dec_context_get_status(ctx) != 0 {
      return Err(err_number_conversion_failed());
    }
    let s = dec_quad_to_string(&v);
    s.parse::<u64>().map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for i64 {
  type Error = DmntkError;
  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    i64::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for i64 {
  type Error = DmntkError;
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let ctx = ctx!();
    let v = dec_quad_to_integral_value(&value.0, ctx, DEC_ROUND_DOWN);
    if dec_context_get_status(ctx) != 0 {
      return Err(err_number_conversion_failed());
    }
    let s = dec_quad_to_string(&v);
    s.parse::<i64>().map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for usize {
  type Error = DmntkError;
  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    usize::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for usize {
  type Error = DmntkError;
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let ctx = ctx!();
    let v = dec_quad_to_integral_value(&value.0, ctx, DEC_ROUND_DOWN);
    if dec_context_get_status(ctx) != 0 {
      return Err(err_number_conversion_failed());
    }
    let s = dec_quad_to_string(&v);
    s.parse::<usize>().map_err(|_| err_number_conversion_failed())
  }
}

impl TryFrom<FeelNumber> for isize {
  type Error = DmntkError;
  fn try_from(value: FeelNumber) -> Result<Self, Self::Error> {
    isize::try_from(&value)
  }
}

impl TryFrom<&FeelNumber> for isize {
  type Error = DmntkError;
  fn try_from(value: &FeelNumber) -> Result<Self, Self::Error> {
    let ctx = ctx!();
    let v = dec_quad_to_integral_value(&value.0, ctx, DEC_ROUND_DOWN);
    if dec_context_get_status(ctx) != 0 {
      return Err(err_number_conversion_failed());
    }
    let s = dec_quad_to_string(&v);
    s.parse::<isize>().map_err(|_| err_number_conversion_failed())
  }
}

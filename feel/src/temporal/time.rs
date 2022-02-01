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

//! Implementation of FEEL time.

use super::date::FeelDate;
use super::nanos_to_string;
use super::zone::FeelZone;
use crate::temporal::errors::err_invalid_time_literal;
use crate::temporal::{after, after_or_equal, before, before_or_equal, between, equal, feel_time_offset, feel_time_zone, is_valid_time, RE_TIME};
use crate::FeelDateTime;
use chrono::{DateTime, FixedOffset};
use dmntk_common::{DmntkError, Result};
use std::str::FromStr;

/// FEEL time.
#[derive(Debug, Clone)]
pub struct FeelTime(pub u8, pub u8, pub u8, pub u64, pub FeelZone); //TODO make these fields private

impl std::fmt::Display for FeelTime {
  /// Converts [FeelTime] into [String].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.3 > 0 {
      write!(f, "{:02}:{:02}:{:02}.{}{}", self.0, self.1, self.2, nanos_to_string(self.3), self.4)
    } else {
      write!(f, "{:02}:{:02}:{:02}{}", self.0, self.1, self.2, self.4)
    }
  }
}

impl FromStr for FeelTime {
  type Err = DmntkError;
  /// Converts [String] into [FeelTime].
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Some(captures) = RE_TIME.captures(s) {
      if let Some(hour_match) = captures.name("hours") {
        if let Ok(hour) = hour_match.as_str().parse::<u8>() {
          if let Some(min_match) = captures.name("minutes") {
            if let Ok(min) = min_match.as_str().parse::<u8>() {
              if let Some(sec_match) = captures.name("seconds") {
                if let Ok(sec) = sec_match.as_str().parse::<u8>() {
                  let mut fractional = 0.0;
                  if let Some(frac_match) = captures.name("fractional") {
                    if let Ok(frac) = frac_match.as_str().parse::<f64>() {
                      fractional = frac;
                    }
                  }
                  let nanos = (fractional * 1e9).trunc() as u64;
                  if let Some(zone) = FeelZone::from_captures(&captures) {
                    let time = FeelTime(hour, min, sec, nanos, zone);
                    // even if parsing from string was successful, the time may still be invalid, so another check
                    let _: DateTime<FixedOffset> = time.clone().try_into().map_err(|_| err_invalid_time_literal(s))?;
                    return Ok(time);
                  }
                }
              }
            }
          }
        }
      }
    }
    Err(err_invalid_time_literal(s))
  }
}

impl std::cmp::PartialEq for FeelTime {
  fn eq(&self, other: &Self) -> bool {
    if let Some(true) = self.equal(other) {
      return true;
    }
    false
  }
}

impl TryFrom<FeelTime> for DateTime<FixedOffset> {
  type Error = DmntkError;
  /// Converts [FeelTime] into [DateTime] with [FixedOffset].
  fn try_from(me: FeelTime) -> Result<Self, Self::Error> {
    let result: DateTime<FixedOffset> = FeelDateTime(FeelDate::today_local(), me).try_into()?;
    Ok(result)
  }
}

impl FeelTime {
  pub fn new_hmso_opt(hour: u8, minute: u8, second: u8, nano: u64, offset: i32) -> Option<Self> {
    if is_valid_time(hour, minute, second) {
      Some(Self(hour, minute, second, nano, FeelZone::from_offset(offset)))
    } else {
      None
    }
  }

  pub fn new_hms_opt(hour: u8, minute: u8, second: u8, nano: u64) -> Option<Self> {
    if is_valid_time(hour, minute, second) {
      Some(Self(hour, minute, second, nano, FeelZone::Local))
    } else {
      None
    }
  }

  /// Creates UTC time from specified time values.
  pub fn utc(hour: u8, minute: u8, second: u8, nanos: u64) -> Self {
    Self(hour, minute, second, nanos, FeelZone::Utc)
  }

  /// Creates local time from specified time values.
  pub fn local(hour: u8, minute: u8, second: u8, nanos: u64) -> Self {
    Self(hour, minute, second, nanos, FeelZone::Local)
  }

  /// Creates a time from specified time and offset values.
  pub fn offset(hour: u8, minute: u8, second: u8, nanos: u64, offset: i32) -> Self {
    Self(hour, minute, second, nanos, FeelZone::Offset(offset))
  }

  /// Compares this time value with other time value and returns [Some] ([true])
  /// when both are equal. Otherwise returns [Some] ([false]).
  /// If any of compared values is not valid then [None] is returned.
  /// Times are compared using current date in local time zone.     
  pub fn equal(&self, other: &Self) -> Option<bool> {
    let today = FeelDate::today_local();
    equal(&FeelDateTime(today.clone(), self.clone()), &FeelDateTime(today, other.clone()))
  }

  pub fn before(&self, other: &Self) -> Option<bool> {
    let today = FeelDate::today_local();
    before(&FeelDateTime(today.clone(), self.clone()), &FeelDateTime(today, other.clone()))
  }

  pub fn before_or_equal(&self, other: &Self) -> Option<bool> {
    let today = FeelDate::today_local();
    before_or_equal(&FeelDateTime(today.clone(), self.clone()), &FeelDateTime(today, other.clone()))
  }

  pub fn after(&self, other: &Self) -> Option<bool> {
    let today = FeelDate::today_local();
    after(&FeelDateTime(today.clone(), self.clone()), &FeelDateTime(today, other.clone()))
  }

  pub fn after_or_equal(&self, other: &Self) -> Option<bool> {
    let today = FeelDate::today_local();
    after_or_equal(&FeelDateTime(today.clone(), self.clone()), &FeelDateTime(today, other.clone()))
  }

  pub fn between(&self, left: &Self, right: &Self, left_closed: bool, right_closed: bool) -> Option<bool> {
    let today = FeelDate::today_local();
    between(
      &FeelDateTime(today.clone(), self.clone()),
      &FeelDateTime(today.clone(), left.clone()),
      &FeelDateTime(today, right.clone()),
      left_closed,
      right_closed,
    )
  }

  pub fn hour(&self) -> u8 {
    self.0
  }

  pub fn minute(&self) -> u8 {
    self.1
  }

  pub fn second(&self) -> u8 {
    self.2
  }

  pub fn feel_time_offset(&self) -> Option<i32> {
    feel_time_offset(&FeelDateTime(FeelDate::today_local(), self.clone()))
  }

  pub fn feel_time_zone(&self) -> Option<String> {
    feel_time_zone(&FeelDateTime(FeelDate::today_local(), self.clone()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::FeelZone;

  fn eq_time_loc(hour: u8, min: u8, sec: u8, s: &str) {
    let expected = FeelTime(hour, min, sec, 0, FeelZone::Local);
    let actual = FeelTime::from_str(s).expect("should not fail");
    assert_eq!(Some(true), expected.equal(&actual));
  }

  fn eq_time_utc(hour: u8, min: u8, sec: u8, s: &str) {
    let expected = FeelTime(hour, min, sec, 0, FeelZone::Utc);
    let actual = FeelTime::from_str(s).expect("should not fail");
    assert_eq!(Some(true), expected.equal(&actual));
  }

  #[test]
  fn test_time_from_str() {
    eq_time_loc(18, 37, 9, "18:37:09");
    eq_time_utc(16, 37, 9, "16:37:09z");
    eq_time_utc(16, 37, 9, "16:37:09Z");
    eq_time_utc(16, 37, 9, "16:37:09@Etc/UTC");
    eq_time_utc(16, 37, 9, "18:37:09@Africa/Johannesburg");
    eq_time_utc(17, 37, 9, "17:37:09@Europe/London");

    // summer time in Vancouver
    // eq_time_utc(17, 37, 9, "10:37:09@America/Vancouver");

    // winter time in Vancouver
    eq_time_utc(18, 37, 9, "10:37:09@America/Vancouver");

    // summer time in New York
    // eq_time_utc(17, 37, 9, "13:37:09@America/New_York");

    // winter time in New York
    eq_time_utc(18, 37, 9, "13:37:09@America/New_York");

    eq_time_utc(17, 37, 9, "18:37:09@Europe/Warsaw");
  }

  #[test]
  fn test_time_from_str_errors() {
    assert_eq!(Err(err_invalid_time_literal("24:37:09")), FeelTime::from_str("24:37:09"));
    assert_eq!(Err(err_invalid_time_literal("18:60:09")), FeelTime::from_str("18:60:09"));
    assert_eq!(Err(err_invalid_time_literal("05:12:60")), FeelTime::from_str("05:12:60"));
  }
}

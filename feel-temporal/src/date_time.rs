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
 * you may not use this file except in compatibility with the License.
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

//! Implementation of FEEL date and time.

use super::date::{is_valid_date, FeelDate};
use super::errors::err_invalid_date_time_literal;
use super::time::FeelTime;
use super::zone::FeelZone;
use crate::defs::*;
use crate::ym_duration::FeelYearsAndMonthsDuration;
use crate::FeelDaysAndTimeDuration;
use chrono::{DateTime, Datelike, Duration, FixedOffset, Timelike};
use dmntk_common::{DmntkError, Result};
use std::cmp::Ordering;
use std::{fmt, ops};

/// FEEL date and time.
#[derive(Debug, Clone)]
pub struct FeelDateTime(pub FeelDate, pub FeelTime); //TODO make these fields private

/// Implements `Display` trait for date and time.
impl fmt::Display for FeelDateTime {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}T{}", self.0, self.1)
  }
}

impl TryFrom<&str> for FeelDateTime {
  type Error = DmntkError;
  /// Converts string into [FeelDateTime].
  fn try_from(s: &str) -> Result<Self, Self::Error> {
    if let Some(captures) = RE_DATE_AND_TIME.captures(s) {
      if let Some(year_match) = captures.name("year") {
        if let Ok(mut year) = year_match.as_str().parse::<Year>() {
          if captures.name("sign").is_some() {
            year = -year;
          }
          if let Some(month_match) = captures.name("month") {
            if let Ok(month) = month_match.as_str().parse::<Month>() {
              if let Some(day_match) = captures.name("day") {
                if let Ok(day) = day_match.as_str().parse::<Day>() {
                  if let Some(hour_match) = captures.name("hours") {
                    if let Ok(mut hour) = hour_match.as_str().parse::<u8>() {
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
                              if is_valid_date(year, month, day) {
                                let date = FeelDate::new(year, month, day);
                                if let Some(zone) = FeelZone::from_captures(&captures) {
                                  if is_valid_time(hour, min, sec) {
                                    if hour == 24 {
                                      if min != 0 || sec != 0 || nanos != 0 {
                                        // fix for hour == 24 //TODO make it more reasonably
                                        return Err(err_invalid_date_time_literal(s));
                                      }
                                      hour = 0;
                                    }
                                    let time = FeelTime(hour, min, sec, nanos, zone);
                                    return Ok(FeelDateTime(date, time));
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    Err(err_invalid_date_time_literal(s))
  }
}

impl PartialEq for FeelDateTime {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1
  }
}

impl PartialOrd for FeelDateTime {
  /// Returns the ordering of two date and times.
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if let Some(d) = self.0.partial_cmp(&other.0) {
      if let Some(t) = self.1.partial_cmp(&other.1) {
        return match (d, t) {
          (Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
          (Ordering::Equal, Ordering::Less) => Some(Ordering::Less),
          (Ordering::Equal, Ordering::Greater) => Some(Ordering::Greater),
          (Ordering::Less, _) => Some(Ordering::Less),
          (Ordering::Greater, _) => Some(Ordering::Greater),
        };
      }
    }
    None
  }
}

impl ops::Add<FeelYearsAndMonthsDuration> for FeelDateTime {
  type Output = Option<Self>;
  ///
  fn add(self, rhs: FeelYearsAndMonthsDuration) -> Self::Output {
    let mut m = rhs.as_months();
    if m > 0 {
      if let Ok(months) = m.try_into() {
        if let Some(date) = self.0.add_months(months) {
          return Some(FeelDateTime(date, self.1));
        }
      }
    } else {
      m = -m;
      if let Ok(months) = m.try_into() {
        if let Some(date) = self.0.sub_months(months) {
          return Some(FeelDateTime(date, self.1));
        }
      }
    }
    None
  }
}

impl ops::Add<FeelDaysAndTimeDuration> for FeelDateTime {
  type Output = Option<Self>;
  ///
  fn add(self, rhs: FeelDaysAndTimeDuration) -> Self::Output {
    let zone = self.1 .4.clone();
    if let Ok(mut date_time) = <FeelDateTime as TryInto<DateTime<FixedOffset>>>::try_into(self) {
      date_time += Duration::nanoseconds(rhs.as_nanos());
      return Some(FeelDateTime(
        FeelDate::new(date_time.year(), date_time.month(), date_time.day()),
        FeelTime(
          date_time.hour() as u8,
          date_time.minute() as u8,
          date_time.second() as u8,
          date_time.nanosecond() as u64,
          zone,
        ),
      ));
    }
    None
  }
}

impl ops::Sub<FeelDateTime> for FeelDateTime {
  type Output = Option<i64>;
  ///
  fn sub(self, other: FeelDateTime) -> Self::Output {
    let me_date_tuple = self.0.as_tuple();
    let me_time_tuple = ((self.1).0 as u32, (self.1).1 as u32, (self.1).2 as u32, (self.1).3 as u32);
    let me_offset_opt = match &(self.1).4 {
      FeelZone::Utc => Some(0),
      FeelZone::Local => get_local_offset_dt(me_date_tuple, me_time_tuple),
      FeelZone::Offset(offset) => Some(*offset),
      FeelZone::Zone(zone_name) => get_zone_offset_dt(zone_name, me_date_tuple, me_time_tuple),
    };
    let other_date_tuple = other.0.as_tuple();
    let other_time_tuple = ((other.1).0 as u32, (other.1).1 as u32, (other.1).2 as u32, (other.1).3 as u32);
    let other_offset_opt = match &(other.1).4 {
      FeelZone::Utc => Some(0),
      FeelZone::Local => get_local_offset_dt(other_date_tuple, other_time_tuple),
      FeelZone::Offset(offset) => Some(*offset),
      FeelZone::Zone(zone_name) => get_zone_offset_dt(zone_name, other_date_tuple, other_time_tuple),
    };
    if let Some((me_offset, other_offset)) = me_offset_opt.zip(other_offset_opt) {
      let me_date_opt = date_time_offset_dt(me_date_tuple, me_time_tuple, me_offset);
      let other_date_opt = date_time_offset_dt(other_date_tuple, other_time_tuple, other_offset);
      if let Some((me_date, other_date)) = me_date_opt.zip(other_date_opt) {
        return me_date.sub(other_date).num_nanoseconds();
      }
    }
    None
  }
}

impl TryFrom<FeelDateTime> for DateTime<FixedOffset> {
  type Error = DmntkError;
  ///
  fn try_from(value: FeelDateTime) -> Result<Self, Self::Error> {
    let me_date_tuple = value.0.as_tuple();
    let me_time_tuple = ((value.1).0 as u32, (value.1).1 as u32, (value.1).2 as u32, (value.1).3 as u32);
    let me_offset_opt = match &(value.1).4 {
      FeelZone::Utc => Some(0),
      FeelZone::Local => get_local_offset_dt(me_date_tuple, me_time_tuple),
      FeelZone::Offset(offset) => Some(*offset),
      FeelZone::Zone(zone_name) => get_zone_offset_dt(zone_name, me_date_tuple, me_time_tuple),
    };
    if let Some(me_offset) = me_offset_opt {
      if let Some(me_date) = date_time_offset_dt(me_date_tuple, me_time_tuple, me_offset) {
        return Ok(me_date);
      }
    }
    Err(err_invalid_date_time_literal("TDB"))
  }
}

impl FeelDateTime {
  /// Creates date and time from provided [FeelDate] and [FeelTime] values.  
  pub fn new(date: FeelDate, time: FeelTime) -> Self {
    Self(date, time)
  }

  /// Creates UTC date and time from specified date and time values.
  pub fn utc(year: Year, month: Month, day: Day, hour: u8, minute: u8, second: u8, nanosecond: u64) -> Self {
    Self(FeelDate::new(year, month, day), FeelTime::utc(hour, minute, second, nanosecond))
  }

  /// Creates local date and time from specified date and time values.
  pub fn local(year: Year, month: Month, day: Day, hour: u8, min: u8, sec: u8, nanos: u64) -> Self {
    Self(FeelDate::new(year, month, day), FeelTime::local(hour, min, sec, nanos))
  }

  /// Creates  date and time from specified date, time and offset values.
  pub fn offset(date: (Year, Month, Day), time: (u8, u8, u8, u64), offset: i32) -> Self {
    Self(FeelDate::new(date.0, date.1, date.2), FeelTime::offset(time.0, time.1, time.2, time.3, offset))
  }

  /// Returns the `Date` part from date and time value.
  pub fn date(&self) -> FeelDate {
    self.0.clone()
  }

  /// Returns the `Time` part from date and time value.
  pub fn time(&self) -> FeelTime {
    self.1.clone()
  }

  pub fn ym_duration(&self, other: &FeelDateTime) -> FeelYearsAndMonthsDuration {
    self.0.ym_duration(&other.0)
  }

  pub fn ym_duration_1(&self, other: &FeelDate) -> FeelYearsAndMonthsDuration {
    self.0.ym_duration(other)
  }

  pub fn year(&self) -> Year {
    self.0.year()
  }

  pub fn month(&self) -> Month {
    self.0.month()
  }

  pub fn day(&self) -> Day {
    self.0.day()
  }
  ///
  pub fn day_of_week(&self) -> Option<DayOfWeek> {
    self.0.day_of_week()
  }
  ///
  pub fn day_of_year(&self) -> Option<DayOfYear> {
    self.0.day_of_year()
  }
  ///
  pub fn week_of_year(&self) -> Option<WeekOfYear> {
    self.0.week_of_year()
  }
  ///
  pub fn month_of_year(&self) -> Option<MonthOfYear> {
    self.0.month_of_year()
  }
  ///
  pub fn hour(&self) -> u8 {
    self.1 .0
  }

  pub fn minute(&self) -> u8 {
    self.1 .1
  }

  pub fn second(&self) -> u8 {
    self.1 .2
  }

  pub fn feel_time_offset(&self) -> Option<i32> {
    feel_time_offset(self)
  }

  pub fn feel_time_zone(&self) -> Option<String> {
    feel_time_zone(self)
  }
}

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

//! Implementation of FEEL temporal artifacts.

use crate::feel_date_time::FeelDateTime;
use crate::feel_zone::FeelZone;
use chrono::{DateTime, Datelike, FixedOffset, Local, LocalResult, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
use lazy_static::lazy_static;
use regex::Regex;

/// Regular expression pattern for parsing dates.
const DATE_PATTERN: &str = r#"(?P<sign>-)?(?P<year>[1-9][0-9]{3,8})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})"#;
/// Regular expression pattern for parsing time.
const TIME_PATTERN: &str = r#"(?P<hours>[0-9]{2}):(?P<minutes>[0-9]{2}):(?P<seconds>[0-9]{2})(?P<fractional>\.[0-9]+)?"#;
/// Regular expression pattern for parsing time zones given as UTC.
const ZULU_PATTERN: &str = r#"(?P<zulu>[zZ])"#;
/// Regular expression pattern for parsing time zones given as zone name.
const ZONE_PATTERN: &str = r#"@(?P<zone>[a-zA-Z_/]+)"#;
/// Regular expression pattern for parsing time zones given as offset.
const OFFSET_PATTERN: &str = r#"(?P<offSign>[+-])(?P<offHours>[0-9]{2}):(?P<offMinutes>[0-9]{2})(:(?P<offSeconds>[0-9]{2}))?"#;

/// Type alias for year.
pub type Year = i32;
/// Type alias for month.
pub type Month = u32;
/// Type alias for day.
pub type Day = u32;
/// Type alias for weekday.
pub type DayOfWeek = (String, u8);
/// Type alias for ordinal number of a day in year.
pub type DayOfYear = u16;
/// Type alias for ordinal number of a week in year.
pub type WeekOfYear = u8;
/// Type alias for month in year.
pub type MonthOfYear = (String, u8);

lazy_static! {
  /// Regular expression pattern for parsing time zone.
  pub static ref TIME_ZONE_PATTERN: String = format!("{ZULU_PATTERN}|{ZONE_PATTERN}|{OFFSET_PATTERN}");
  /// Regular expression for parsing date.
  pub static ref RE_DATE: Regex = Regex::new(&format!("^{DATE_PATTERN}$")).unwrap();
  /// Regular expression for parsing time.
  pub static ref RE_TIME: Regex = Regex::new(&format!("^{TIME_PATTERN}({})?$", TIME_ZONE_PATTERN.as_str()) ).unwrap();
  /// Regular expression for parsing date and time.
  pub static ref RE_DATE_AND_TIME: Regex = Regex::new(&format!("^{DATE_PATTERN}T{TIME_PATTERN}({})?$",TIME_ZONE_PATTERN.as_str())).unwrap();
}

pub fn feel_time_offset(me: &FeelDateTime) -> Option<i32> {
  let me_date_tuple = me.date().as_tuple();
  let me_time_tuple = (
    (me.time()).hour() as u32,
    (me.time()).minute() as u32,
    (me.time()).second() as u32,
    (me.time()).nanos() as u32,
  );
  let me_offset_opt = match me.time().zone() {
    FeelZone::Utc => Some(0),
    FeelZone::Local => None, // in FEEL semantic domain the local offset is treated as none
    FeelZone::Offset(offset) => Some(*offset),
    FeelZone::Zone(zone_name) => get_zone_offset_dt(zone_name, me_date_tuple, me_time_tuple),
  };
  if let Some(me_offset) = me_offset_opt {
    return Some(me_offset);
  }
  None
}

pub fn feel_time_zone(me: &FeelDateTime) -> Option<String> {
  if let FeelZone::Zone(zone_name) = me.time().zone() {
    return Some(zone_name.clone());
  }
  None
}

///
pub fn date_time_offset_dt(date: (i32, u32, u32), time: (u32, u32, u32, u32), offset: i32) -> Option<DateTime<FixedOffset>> {
  if let Some(fixed_offset) = FixedOffset::east_opt(offset) {
    if let LocalResult::Single(offset_date_time) = fixed_offset.with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2) {
      if let Some(date_time) = offset_date_time.with_nanosecond(time.3) {
        return Some(date_time);
      }
    }
  }
  None
}

///
pub fn date_time_offset_t(time: (u32, u32, u32, u32), offset: i32) -> Option<DateTime<FixedOffset>> {
  let today = Local::now();
  date_time_offset_dt((today.year(), today.month(), today.day()), time, offset)
}

/// Returns the time offset (in seconds) between local time zone
/// and UTC time zone at specified **date and time**.
pub fn get_local_offset_dt(date: (i32, u32, u32), time: (u32, u32, u32, u32)) -> Option<i32> {
  if let Some(naive_date) = NaiveDate::from_ymd_opt(date.0, date.1, date.2) {
    if let Some(naive_time) = NaiveTime::from_hms_nano_opt(time.0, time.1, time.2, time.3) {
      let naive_date_time = NaiveDateTime::new(naive_date, naive_time);
      return Some(Local.offset_from_utc_datetime(&naive_date_time).local_minus_utc());
    }
  }
  None
}

/// Returns the time offset (in seconds) between local time zone
/// and UTC time zone at specified **time** today.
pub fn get_local_offset_t(time: (u32, u32, u32, u32)) -> Option<i32> {
  let today = Local::now();
  get_local_offset_dt((today.year(), today.month(), today.day()), time)
}

/// Returns time offset (in seconds) between named time zone
/// and UTC time zone at specified **date and time**.
pub fn get_zone_offset_dt(zone_name: &str, date: (i32, u32, u32), time: (u32, u32, u32, u32)) -> Option<i32> {
  // try to build UTC date and time from specified values
  if let LocalResult::Single(utc_date_time) = Utc.with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2) {
    if let Some(utc) = utc_date_time.with_nanosecond(time.3) {
      // try parse the time zone specified as text
      if let Ok(tz) = zone_name.parse::<chrono_tz::Tz>() {
        // build date and time in parsed time zone
        if let LocalResult::Single(z_date_time) = tz.with_ymd_and_hms(date.0, date.1, date.2, time.0, time.1, time.2) {
          if let Some(zdt) = z_date_time.with_nanosecond(time.3) {
            // calculate the time offset, the result is a chrono::Duration
            let offset: chrono::Duration = utc.with_timezone(&tz) - zdt;
            // return seconds
            return Some(offset.num_seconds() as i32);
          }
        }
      }
    }
  }
  None
}

/// Returns time offset (in seconds) between named time zone
/// and UTC time zone at specified **time** today.
pub fn get_zone_offset_t(zone_name: &str, time: (u32, u32, u32, u32)) -> Option<i32> {
  let today = Local::now();
  get_zone_offset_dt(zone_name, (today.year(), today.month(), today.day()), time)
}

/// Returns time offset (in seconds) of the named time zone and UTC time zone.
pub fn get_zone_offset(zone_name: &str) -> Option<i32> {
  let now = Utc::now();
  let time = now.time();
  let t = (time.hour(), time.minute(), time.second(), time.nanosecond());
  let d = (now.year(), now.month(), now.day());
  get_zone_offset_dt(zone_name, d, t)
}

/// Converts nanoseconds into string.
///
/// Calculates the remaining number of nanoseconds in a second (modulo),
/// converts the result into string prefixed with zeros when appropriate
/// and trims all zeros after last non-zero digit.
///
/// # Examples
///
/// ```no run
/// assert_eq!("", nanos_to_string(0));
/// assert_eq!("", nanos_to_string(1_000_000_000));
/// assert_eq!("00012", nanos_to_string(120_000));
/// assert_eq!("1", nanos_to_string(100_000_000));
/// ```
pub fn nanos_to_string(nano: u64) -> String {
  let chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
  let mut buffer = ['0', '0', '0', '0', '0', '0', '0', '0', '0'];
  let mut index = 9_usize;
  let mut count = 9_usize;
  let mut non_zero = false;
  let mut value = nano % 1_000_000_000;
  if value == 0 {
    return "".to_string();
  }
  while value > 0 {
    let remainder = (value % 10) as usize;
    value /= 10;
    if remainder > 0 {
      non_zero = true;
    }
    if remainder == 0 && !non_zero {
      count -= 1;
    }
    index -= 1;
    buffer[index] = chars[remainder];
  }
  buffer.iter().take(count).collect()
}

pub fn is_valid_time(hour: u8, minute: u8, second: u8) -> bool {
  (hour < 24 && minute < 60 && second < 60) || (hour == 24 && minute == 0 && second == 0)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::feel_date::FeelDate;
  use std::str::FromStr;

  const SECONDS_IN_HOUR: i32 = 3_600;
  const SECONDS_IN_MIN: i32 = 60;

  fn eq_date(year: Year, month: Month, day: Day, s: &str) {
    assert_eq!(Ok(FeelDate::new(year, month, day)), FeelDate::from_str(s));
  }

  #[test]
  fn test_parse_date() {
    eq_date(2020, 9, 28, "2020-09-28");
  }

  #[test]
  fn test_get_local_offset_dt() {
    let utc_offset = get_zone_offset_dt("Etc/UTC", (2020, 6, 12), (9, 12, 3, 0)).unwrap();
    let warsaw_offset = get_zone_offset_dt("Europe/Warsaw", (2020, 6, 12), (9, 12, 3, 0)).unwrap();
    let local_offset = get_local_offset_dt((2020, 6, 12), (9, 12, 3, 0)).unwrap();
    if local_offset == warsaw_offset {
      assert_eq!(Some(SECONDS_IN_HOUR), get_local_offset_dt((2020, 10, 29), (9, 12, 3, 0)));
      assert_eq!(
        get_zone_offset_dt("Europe/Warsaw", (2020, 6, 12), (9, 12, 3, 0)),
        get_local_offset_dt((2020, 6, 12), (9, 12, 3, 0))
      );
    }
    if local_offset == utc_offset {
      assert_eq!(Some(0), get_local_offset_dt((2020, 10, 29), (9, 12, 3, 0)));
      assert_eq!(
        get_zone_offset_dt("Etc/UTC", (2020, 6, 12), (9, 12, 3, 0)),
        get_local_offset_dt((2020, 6, 12), (9, 12, 3, 0))
      );
    }
  }

  #[test]
  fn test_get_zone_offset() {
    // winter time in Warsaw, offset = +01:00
    assert_eq!(Some(SECONDS_IN_HOUR), get_zone_offset_dt("Europe/Warsaw", (2020, 10, 29), (9, 12, 3, 0)));
    // summer time in Warsaw, offset = +02:00
    assert_eq!(Some(2 * SECONDS_IN_HOUR), get_zone_offset_dt("Europe/Warsaw", (2020, 6, 21), (11, 13, 49, 0)));
    // time in Moscow, offset = +03:00
    assert_eq!(Some(3 * SECONDS_IN_HOUR), get_zone_offset_dt("Europe/Moscow", (2020, 10, 29), (9, 12, 3, 0)));
    // summer time in New York, offset = -04:00
    assert_eq!(Some(-4 * SECONDS_IN_HOUR), get_zone_offset_dt("America/New_York", (2020, 6, 28), (12, 12, 3, 0)));
    // winter time in New York, offset = -05:00
    assert_eq!(Some(-5 * SECONDS_IN_HOUR), get_zone_offset_dt("America/New_York", (2020, 11, 12), (18, 4, 33, 0)));
    // time in Kolkata, offset = +05:30
    assert_eq!(
      Some(5 * SECONDS_IN_HOUR + 30 * SECONDS_IN_MIN),
      get_zone_offset_dt("Asia/Kolkata", (2020, 11, 12), (18, 4, 33, 0))
    );
    // no time change in Kolkata in summer, offset = +05:30
    assert_eq!(
      Some(5 * SECONDS_IN_HOUR + 30 * SECONDS_IN_MIN),
      get_zone_offset_dt("Asia/Kolkata", (2020, 6, 8), (8, 0, 0, 0))
    );
    // time in Honolulu, offset = -10:00
    assert_eq!(Some(-10 * SECONDS_IN_HOUR), get_zone_offset_dt("Pacific/Honolulu", (2020, 11, 12), (18, 4, 33, 0)));
    // no time change in Honolulu in summer, offset = -10:00
    assert_eq!(Some(-10 * SECONDS_IN_HOUR), get_zone_offset_dt("Pacific/Honolulu", (2020, 6, 8), (8, 0, 0, 0)));
  }

  #[test]
  fn test_nanos_to_string() {
    assert_eq!("", nanos_to_string(0));
    assert_eq!("000000001", nanos_to_string(1));
    assert_eq!("000000012", nanos_to_string(12));
    assert_eq!("00000012", nanos_to_string(120));
    assert_eq!("0000012", nanos_to_string(1_200));
    assert_eq!("000012", nanos_to_string(12_000));
    assert_eq!("00012", nanos_to_string(120_000));
    assert_eq!("0012", nanos_to_string(1_200_000));
    assert_eq!("012", nanos_to_string(12_000_000));
    assert_eq!("12", nanos_to_string(120_000_000));
    assert_eq!("1", nanos_to_string(100_000_000));
    assert_eq!("", nanos_to_string(1_000_000_000));
  }
}

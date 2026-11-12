//! Calendar dates for practice features (ADR 0019): review due dates, the daily problem,
//! streaks. A `Day` is a date without a time or a time zone — "2026-11-14" is due on
//! that date wherever the user is — stored as ISO text and counted as days since
//! 1970-01-01. The UI decides which local date "today" is.

use std::fmt;
use std::str::FromStr;

/// A calendar date. Ordered; `Day + n` moves `n` days.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Day(i64);

impl Day {
    /// Days since 1970-01-01.
    pub fn from_days(days: i64) -> Self {
        Day(days)
    }

    pub fn days(self) -> i64 {
        self.0
    }

    pub fn from_ymd(y: i64, m: u32, d: u32) -> Option<Self> {
        if !(1..=12).contains(&m) || d < 1 || d > days_in_month(y, m) {
            return None;
        }
        // Howard Hinnant's days_from_civil.
        let y = if m <= 2 { y - 1 } else { y };
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let mp = (i64::from(m) + 9) % 12;
        let doy = (153 * mp + 2) / 5 + i64::from(d) - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        Some(Day(era * 146_097 + doe - 719_468))
    }

    /// (year, month, day).
    pub fn ymd(self) -> (i64, u32, u32) {
        // Howard Hinnant's civil_from_days.
        let z = self.0 + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        let y = yoe + era * 400 + i64::from(m <= 2);
        (y, m, d)
    }

    pub fn add_days(self, n: i64) -> Self {
        Day(self.0 + n)
    }
}

fn days_in_month(y: i64, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 => 29,
        _ => 28,
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d) = self.ymd();
        write!(f, "{y:04}-{m:02}-{d:02}")
    }
}

impl FromStr for Day {
    type Err = String;

    /// Exactly `YYYY-MM-DD`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bad = || format!("not a date (YYYY-MM-DD): {s:?}");
        let b = s.as_bytes();
        if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
            return Err(bad());
        }
        let num = |r: std::ops::Range<usize>| -> Result<i64, String> {
            let part = &s[r];
            if part.bytes().all(|c| c.is_ascii_digit()) {
                part.parse().map_err(|_| bad())
            } else {
                Err(bad())
            }
        };
        let (y, m, d) = (num(0..4)?, num(5..7)?, num(8..10)?);
        Day::from_ymd(y, m as u32, d as u32).ok_or_else(bad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_and_epoch() {
        assert_eq!(Day::from_ymd(1970, 1, 1), Some(Day(0)));
        assert_eq!(Day(0).to_string(), "1970-01-01");
        for s in [
            "2026-11-14",
            "2000-02-29",
            "1999-12-31",
            "2100-03-01",
            "1969-07-20",
        ] {
            assert_eq!(s.parse::<Day>().unwrap().to_string(), s);
        }
        // Every day for 800 years round-trips through (y, m, d).
        let start = Day::from_ymd(1800, 1, 1).unwrap().days();
        for n in start..start + 800 * 366 {
            let (y, m, d) = Day(n).ymd();
            assert_eq!(Day::from_ymd(y, m, d), Some(Day(n)));
        }
    }

    #[test]
    fn adds_across_months_and_leap_years() {
        let d: Day = "2024-02-27".parse().unwrap();
        assert_eq!(d.add_days(2).to_string(), "2024-02-29");
        assert_eq!(d.add_days(3).to_string(), "2024-03-01");
        assert_eq!(
            "2026-12-30".parse::<Day>().unwrap().add_days(3).to_string(),
            "2027-01-02"
        );
        assert_eq!(
            "2026-03-01"
                .parse::<Day>()
                .unwrap()
                .add_days(-1)
                .to_string(),
            "2026-02-28"
        );
        assert!(d < d.add_days(1));
    }

    #[test]
    fn rejects_bad_text() {
        for s in [
            "2026-2-1",
            "2026-02-30",
            "2025-02-29",
            "1900-02-29",
            "2026/01/01",
            "abcd-ef-gh",
            "",
            "2026-13-01",
            " 2026-01-01",
        ] {
            assert!(s.parse::<Day>().is_err(), "{s}");
        }
    }
}

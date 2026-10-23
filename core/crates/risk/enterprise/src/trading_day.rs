//! The trading day (S13 AC-4): it ends at the firm's reset time (prop) or at the CME session close
//! (real accounts), Monday to Friday. A day is labelled by the date its reset closes it, in the
//! reset's time zone. Playback accounts pass their simulated clock as `at`.
//! Exchange holidays are not modelled yet (an open question for S13; a holiday is a normal day).

use chrono::{DateTime, Datelike, Days, NaiveDate, NaiveTime, TimeZone, Utc, Weekday};
use chrono_tz::Tz;

use crate::rules::ResetTime;

/// CME equity futures: the session closes at 16:00 Central (17:00 New York).
const CME_CLOSE_HOUR: u32 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResetSchedule {
    Firm(ResetTime),
    CmeSessionClose,
}

impl ResetSchedule {
    pub const fn source(&self) -> &'static str {
        match self {
            Self::Firm(_) => "firm",
            Self::CmeSessionClose => "cme_session_close",
        }
    }

    fn time(&self) -> (NaiveTime, Tz) {
        match self {
            Self::Firm(r) => (r.local_time, r.time_zone),
            Self::CmeSessionClose => (
                NaiveTime::from_hms_opt(CME_CLOSE_HOUR, 0, 0).unwrap_or(NaiveTime::MIN),
                chrono_tz::America::Chicago,
            ),
        }
    }

    /// The trading day that contains `at`: `[starts_at, ends_at)`.
    pub fn day_at(&self, at: DateTime<Utc>) -> TradingDay {
        let (time, tz) = self.time();
        let local = at.with_timezone(&tz);
        let mut date = local.date_naive();
        if local.time() >= time {
            date = next_day(date);
        }
        while is_weekend(date) {
            date = next_day(date);
        }
        let mut previous = prev_day(date);
        while is_weekend(previous) {
            previous = prev_day(previous);
        }
        TradingDay {
            date,
            starts_at: instant(tz, previous, time),
            ends_at: instant(tz, date, time),
        }
    }

    /// The start of the trading day after the one containing `at` (Q48 `effective_from`).
    pub fn next_day_start(&self, at: DateTime<Utc>) -> DateTime<Utc> {
        self.day_at(at).ends_at
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TradingDay {
    pub date: NaiveDate,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}

fn is_weekend(d: NaiveDate) -> bool {
    matches!(d.weekday(), Weekday::Sat | Weekday::Sun)
}

fn next_day(d: NaiveDate) -> NaiveDate {
    d.checked_add_days(Days::new(1)).unwrap_or(d)
}

fn prev_day(d: NaiveDate) -> NaiveDate {
    d.checked_sub_days(Days::new(1)).unwrap_or(d)
}

/// The local wall time as an instant. Ambiguous (DST fall back): the earlier one. Skipped (spring
/// forward gap): the first valid instant after it.
fn instant(tz: Tz, date: NaiveDate, time: NaiveTime) -> DateTime<Utc> {
    let mut naive = date.and_time(time);
    for _ in 0..4 {
        if let Some(t) = tz.from_local_datetime(&naive).earliest() {
            return t.with_timezone(&Utc);
        }
        naive += chrono::Duration::minutes(30);
    }
    Utc.from_utc_datetime(&naive)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::America::New_York;

    fn utc(s: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
    }

    fn firm(h: u32, m: u32) -> ResetSchedule {
        ResetSchedule::Firm(ResetTime {
            local_time: NaiveTime::from_hms_opt(h, m, 0).unwrap(),
            time_zone: New_York,
        })
    }

    #[test]
    fn a_day_runs_from_reset_to_reset() {
        // Tue 2026-11-03 10:00 New York (15:00Z, EST).
        let day = firm(17, 0).day_at(utc("2026-11-03T15:00:00Z"));
        assert_eq!(day.date, NaiveDate::from_ymd_opt(2026, 11, 3).unwrap());
        assert_eq!(day.starts_at, utc("2026-11-02T22:00:00Z"));
        assert_eq!(day.ends_at, utc("2026-11-03T22:00:00Z"));
        // At the reset itself the next day starts.
        let next = firm(17, 0).day_at(utc("2026-11-03T22:00:00Z"));
        assert_eq!(next.date, NaiveDate::from_ymd_opt(2026, 11, 4).unwrap());
    }

    #[test]
    fn friday_evening_and_the_weekend_belong_to_monday() {
        let s = firm(17, 0);
        for at in [
            "2026-11-06T23:00:00Z",
            "2026-11-07T15:00:00Z",
            "2026-11-08T23:30:00Z",
        ] {
            let day = s.day_at(utc(at));
            assert_eq!(
                day.date,
                NaiveDate::from_ymd_opt(2026, 11, 9).unwrap(),
                "{at}"
            );
            assert_eq!(day.starts_at, utc("2026-11-06T22:00:00Z"));
            assert_eq!(day.ends_at, utc("2026-11-09T22:00:00Z"));
        }
    }

    #[test]
    fn cme_close_is_four_pm_central_across_dst() {
        let s = ResetSchedule::CmeSessionClose;
        // CDT (UTC-5) before 1 Nov 2026, CST (UTC-6) after.
        assert_eq!(
            s.day_at(utc("2026-10-30T15:00:00Z")).ends_at,
            utc("2026-10-30T21:00:00Z")
        );
        let monday = s.day_at(utc("2026-11-02T15:00:00Z"));
        assert_eq!(monday.starts_at, utc("2026-10-30T21:00:00Z"));
        assert_eq!(monday.ends_at, utc("2026-11-02T22:00:00Z"));
        assert_eq!(s.source(), "cme_session_close");
    }

    #[test]
    fn a_reset_in_the_spring_forward_gap_moves_to_the_first_valid_time() {
        // 2027-03-14 02:30 does not exist in New York; 03:00 EDT = 07:00Z.
        let day = firm(2, 30).day_at(utc("2027-03-14T05:00:00Z"));
        assert_eq!(day.date, NaiveDate::from_ymd_opt(2027, 3, 15).unwrap());
        let sunday_reset = firm(2, 30).day_at(utc("2027-03-12T12:00:00Z")).ends_at;
        assert_eq!(sunday_reset, utc("2027-03-15T06:30:00Z"));
    }

    #[test]
    fn next_day_start_is_the_end_of_today() {
        let s = firm(17, 0);
        assert_eq!(
            s.next_day_start(utc("2026-11-03T15:00:00Z")),
            utc("2026-11-03T22:00:00Z")
        );
    }
}

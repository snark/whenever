//! # Whenever
//!
//! This crate provides basic natural language date processing, turning strings like
//! This crate provides basic natural language date processing, turning strings like
//! "today", "June 16", or "last Friday" into tractable datetime objects, provided by
//! the [chrono](https://crates.io/crates/chrono) crate.
//!
//! Whenever is in very early stages and its API is in flux.

extern crate chrono;
#[macro_use]
extern crate nom;
extern crate time;

use chrono::prelude::*;
use std::fmt;
use time::Duration;

pub mod parser;

#[derive(Debug)]
pub enum Period {
    Day,
    Month,
    Week,
    Year
}

/// A parsed date.
///
/// Currently this object is simply a struct wrapper around a chrono NaiveDate object
/// (that is, a date unaware of time zones). 

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParsedDate {
    pub date: chrono::naive::date::NaiveDate
}

impl ParsedDate {
    pub fn from_ymd(year: i32, month: u32, day: u32) -> ParsedDate {
        let d = NaiveDate::from_ymd_opt(year, month, day)
            .expect("invalid or out-of-range date");
        ParsedDate {
            date: d
        }
    }

    pub fn shift(&mut self, period: Period, count: i16) {
        let date = self.date;
        let new_date = match period {
            Period::Day => Some(date + Duration::days(count as i64)),
            Period::Week => Some(date + Duration::weeks(count as i64)),
            Period::Month => {
                let d = date.day() as i32;
                let y = date.year();
                let m_offset = count % 12;
                let mut m = date.month() as i32 + m_offset as i32;
                let mut y_offset = count / 12;
                if m < 12 {
                    y_offset = y_offset - 1;
                    m = m + 12;
                }
                if m > 12 {
                    y_offset = y_offset + 1;
                    m = m - 12;
                }
                let mut done = false;
                let mut day_offset: i32 = 0;
                let mut new_date = None;
                while !done {
                    let new_day = (d - day_offset) as i32;
                    new_date = date.clone()
                        .with_day(new_day as u32)
                        .and_then( |i| i.with_year(y - y_offset as i32) )
                        .and_then( |i| i.with_month(m as u32) );
                    day_offset = day_offset + 1;
                    done = new_date.is_some();
                }
                new_date
            },
            Period::Year => {
                let mut done = false;
                let y = date.year();
                let mut day_offset: i64 = 0;
                // Complicated because of leap year
                let mut new_date = None;
                while !done {
                    new_date = (date.clone() + Duration::days(day_offset))
                        .with_year(y + count as i32);
                    day_offset = day_offset + 1;
                    done = new_date.is_some();
                }
                new_date
            }
        };
        if new_date.is_some() {
            self.date = new_date.expect("date shift error");
        };
    }

}

impl fmt::Display for ParsedDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", &self.date.year(), &self.date.month(), &self.date.day())
    }
}

#[cfg(test)]
mod tests {
    use super::{ParsedDate, Period};
    #[test]
    fn date_shift() {
        let mut date = ParsedDate::from_ymd(2016, 12, 31);
        date.shift(Period::Day, 1);
        assert_eq!(date, ParsedDate::from_ymd(2017, 1, 1));
        date.shift(Period::Day, -1);
        assert_eq!(date, ParsedDate::from_ymd(2016, 12, 31));
    }
}

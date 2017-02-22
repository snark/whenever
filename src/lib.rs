extern crate chrono;
#[macro_use]
extern crate nom;
extern crate time;

use chrono::prelude::*;
use std::fmt;

pub mod parser;

#[derive(Debug)]
pub struct ParsedDate {
    pub date: chrono::naive::date::NaiveDate
}

impl ParsedDate {
    pub fn from_ymd(year: i32, month: u32, day: u32) -> ParsedDate {
        let d = NaiveDate::from_ymd_opt(year, month, day).expect("invalid or out-of-range date");
        ParsedDate {
            date: d
        }
    }
}

impl fmt::Display for ParsedDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", &self.date.year(), &self.date.month(), &self.date.day())
    }
}

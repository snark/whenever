use super::super::{ParsedDate, Period};
use chrono::prelude::*;
use time::Duration;

named!(pub indexical <&[u8], Option<ParsedDate>>,
    alt_complete!(
        tag_no_case!("today") => { |_|
            {
                let today = Local::today();
                Some(ParsedDate::from_ymd(today.year(), today.month(), today.day()))
            }
        } |
        tag_no_case!("tomorrow") => { |_|
            {
                let ts = Local::today() + Duration::days(1);
                Some(ParsedDate::from_ymd(ts.year(), ts.month(), ts.day()))
            }
        } |
        tag_no_case!("yesterday") => { |_|
            {
                let ts = Local::today() - Duration::days(1);
                Some(ParsedDate::from_ymd(ts.year(), ts.month(), ts.day()))
            }
        }
    )
);

#[derive(Debug)]
enum Weekday {
    Mon = 0,
    Tue = 1,
    Wed = 2,
    Thu = 3,
    Fri = 4,
    Sat = 5,
    Sun = 6,
}

fn weekday_calc(dow: Weekday, offset: i8) -> ParsedDate {
    let today = Local::today();
    let today_dow = today.weekday() as i8;
    let diff = match ( dow as i8 - today_dow + offset ) % 7 {
        x if x == 0 => 7,
        x if x < 7 => x,
        _ => unreachable!("Bad weekday diff"),
    };
    let ts = today + Duration::days(diff as i64);
    ParsedDate::from_ymd(ts.year(), ts.month(), ts.day())
}

named!(pub base_weekday <&[u8], Option<ParsedDate>>,
    alt_complete!(
        tag_no_case!("Monday") => { |_| Some(weekday_calc(Weekday::Mon, 7)) } |
        tag_no_case!("Tuesday") => { |_| Some(weekday_calc(Weekday::Tue, 7)) } |
        tag_no_case!("Wednesday") => { |_| Some(weekday_calc(Weekday::Wed, 7)) } |
        tag_no_case!("Thursday") => { |_| Some(weekday_calc(Weekday::Thu, 7)) } |
        tag_no_case!("Friday") => { |_| Some(weekday_calc(Weekday::Fri, 7)) } |
        tag_no_case!("Saturday") => { |_| Some(weekday_calc(Weekday::Sat, 7)) } |
        tag_no_case!("Sunday") => { |_| Some(weekday_calc(Weekday::Sun, 7)) }
    )
);

named!(pub last_weekday <&[u8], Option<ParsedDate>>,
    ws!(
        do_parse!(
            d: preceded!(tag_no_case!("last"), base_weekday) >>
            ({
                if d.is_some() {
                    let mut nd = d.expect("Unreachable failure in weekday");
                    nd.shift(Period::Day, -7);
                    Some(nd)
                } else {
                    None
                }
            })
        )
    )
);

// NB: when we have a parser for ambiguity, we should flag this; on a Monday,
// what does "next Sunday" mean -- six or thirteen days from now?
named!(pub next_weekday <&[u8], Option<ParsedDate>>,
    ws!(
        do_parse!(
            d: preceded!(tag_no_case!("next"), base_weekday) >>
            ({
                if d.is_some() {
                    let mut nd = d.expect("Unreachable failure in weekday");
                    nd.shift(Period::Day, 7);
                    Some(nd)
                } else {
                    None
                }
            })
        )
    )
);

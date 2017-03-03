use super::super::ParsedDate;
use chrono::prelude::{Datelike, Local};
use time::Duration;
use super::helper_macros::periods;

named!(indexical_unit <&[u8], ParsedDate>,
    alt_complete!(
        alt!(tag_no_case!("today") | tag_no_case!("now")) => { |_|
            {
                let today = Local::today();
                ParsedDate::from_ymd(today.year(), today.month(), today.day())
            }
        } |
        tag_no_case!("tomorrow") => { |_|
            {
                let ts = Local::today() + Duration::days(1);
                ParsedDate::from_ymd(ts.year(), ts.month(), ts.day())
            }
        } |
        tag_no_case!("yesterday") => { |_|
            {
                let ts = Local::today() - Duration::days(1);
                ParsedDate::from_ymd(ts.year(), ts.month(), ts.day())
            }
        }
    )
);

named!(next_last <&[u8], i16>,
    alt_complete!(
        tag_no_case!("next") => { |_| 1 } |
        tag_no_case!("last") => { |_| -1 }
    )
);

named!(indexical_phrase <&[u8], ParsedDate>,
    ws!(
        do_parse!(
            sign: next_last >>
            period: periods >>
            (
                {
                    let ts = Local::today();
                    let mut pd = ParsedDate::from_ymd(ts.year(), ts.month(), ts.day());
                    pd.shift(period, sign);
                    pd
                }
            )
        )
    )
);

named!(pub indexical <&[u8], ParsedDate>,
    alt_complete!(
        indexical_phrase | indexical_unit
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

fn weekday_calc(dow: Weekday, offset: i8, mut weekshift: i8) -> ParsedDate {
    let today = Local::today();
    let today_dow = today.weekday() as i8;
    let diff = match (dow as i8 - today_dow + offset) % 7 {
        x if x == 0 => 7,
        x if x < 7 => x,
        _ => unreachable!("Bad weekday diff"),
    };
    if diff == 7 && weekshift != 0 {
        weekshift = weekshift - 1;
    }
    let ts = today + Duration::days(diff as i64) + Duration::days(7 * weekshift as i64);
    ParsedDate::from_ymd(ts.year(), ts.month(), ts.day())
}

named!(which_weekday_full <&[u8], Weekday>,
    alt_complete!(
        tag_no_case!("Monday") => { |_| Weekday::Mon } |
        tag_no_case!("Tuesday") => { |_| Weekday::Tue } |
        tag_no_case!("Wednesday") => { |_| Weekday::Wed } |
        tag_no_case!("Thursday") => { |_| Weekday::Thu } |
        tag_no_case!("Friday") => { |_| Weekday::Fri } |
        tag_no_case!("Saturday") => { |_| Weekday::Sat } |
        tag_no_case!("Sunday") => { |_| Weekday::Sun }
    )
);

named!(which_weekday_abbr <&[u8], Weekday>,
    alt_complete!(
        tag_no_case!("Mon") => { |_| Weekday::Mon } |
        tag_no_case!("Tues") => { |_| Weekday::Tue } |
        tag_no_case!("Tue") => { |_| Weekday::Tue } |
        tag_no_case!("Wed") => { |_| Weekday::Wed } |
        tag_no_case!("Thurs") => { |_| Weekday::Thu } |
        tag_no_case!("Thu") => { |_| Weekday::Thu } |
        tag_no_case!("Fri") => { |_| Weekday::Fri } |
        tag_no_case!("Sat") => { |_| Weekday::Sat } |
        tag_no_case!("Sun") => { |_| Weekday::Sun }
    )
);

named!(which_weekday <&[u8], Weekday>,
    alt_complete!(
        terminated!(which_weekday_abbr, tag!(".")) |
        which_weekday_abbr |
        which_weekday_full
    )
);

named!(base_weekday <&[u8], ParsedDate>,
   ws!(
       do_parse!(
           opt!(tag!("this")) >>
           wd: which_weekday >> (
               weekday_calc(wd, 7, 0)
           )
        )
    )
);

// NB: when we have a parser for ambiguity, we should flag this; on a Monday,
// what does "next Sunday" mean -- six or thirteen days from now?
named!(next_weekday <&[u8], ParsedDate>,
    ws!(
        do_parse!(
            wd: preceded!(tag_no_case!("next"), which_weekday) >> (
                weekday_calc(wd, 7, 1)
            )
        )
    )
);

named!(last_weekday <&[u8], ParsedDate>,
    ws!(
        do_parse!(
            wd: preceded!(tag_no_case!("last"), which_weekday) >> (
                weekday_calc(wd, 7, -1)
            )
        )
    )
);

named!(pub weekday <&[u8], ParsedDate>,
    alt_complete!(
        last_weekday |
        next_weekday |
        base_weekday
    )
);

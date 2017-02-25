mod deictics;

use nom;
use self::deictics::{indexical, next_weekday, last_weekday, base_weekday};
use super::ParsedDate;

named!(pub date_token <&[u8], Option<ParsedDate>>,
    ws!(
        alt_complete!(
            indexical | base_weekday | last_weekday | next_weekday
        )
    )
);

pub fn parse_date(date_str: &str) -> Option<ParsedDate> {
    match date_token(date_str.as_bytes()) {
        nom::IResult::Done(_, d) => d,
        nom::IResult::Error(_) => None,
        nom::IResult::Incomplete(_) => unreachable!(
            "Incomplete parse received from date_token"
        )
    }
}


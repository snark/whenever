mod deictics;

use nom;
use super::ParsedDate;
use self::deictics::{indexical, weekday};

named!(pub date_token <&[u8], Option<ParsedDate>>,
    ws!(
        alt_complete!(
            indexical | weekday
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

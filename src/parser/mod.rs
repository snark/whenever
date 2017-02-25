#![macro_use]

mod deictics;
mod helper_macros;
mod offsets;

use nom;
use self::deictics::{indexical, weekday};
use self::offsets::offset;
use super::ParsedDate;

named!(pub date_token <&[u8], ParsedDate>,
    ws!(
        alt_complete!(
            indexical | weekday | offset
        )
    )
);

pub fn parse_date(date_str: &str) -> Option<ParsedDate> {
    match date_token(date_str.as_bytes()) {
        nom::IResult::Done(_, d) => Some(d),
        nom::IResult::Error(_) => None,
        nom::IResult::Incomplete(_) => unreachable!(
            "Incomplete parse received from date_token"
        )
    }
}


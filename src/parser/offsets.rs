use super::super::ParsedDate;
use super::helper_macros::{periods, numbers_longform};
use super::date_token;

named!(sign <&[u8], i8>,
    alt_complete!(
        tag_no_case!("before") => { |_| -1 } |
        tag_no_case!("ago") => { |_| -1 } |
        tag_no_case!("from") => { |_| 1 } |
        tag_no_case!("after") => { |_| 1 }
    )
);


named!(pub offset <&[u8], ParsedDate>,
    ws!(
        do_parse!(
            n: numbers_longform >>
            period: periods >>
            sign: sign >>
            dt: date_token >>
            (
                {
                    let mut new_dt = dt.clone();
                    new_dt.shift(period, (n * sign) as i16);
                    new_dt
                }
            )
        )
    )
);

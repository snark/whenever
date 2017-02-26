#![macro_use]

named!(pub numbers_longform <&[u8], i8>,
    alt_complete!(
        tag_no_case!("zero") => { |_| 0 } |
        tag_no_case!("one") => { |_| 1 } |
        tag_no_case!("a") => { |_| 1 } |
        tag_no_case!("the") => { |_| 1 } |
        tag_no_case!("two") => { |_| 2 } |
        tag_no_case!("three") => { |_| 3 } |
        tag_no_case!("four") => { |_| 4 } |
        tag_no_case!("five") => { |_| 5 } |
        tag_no_case!("six") => { |_| 6 } |
        tag_no_case!("seven") => { |_| 7 } |
        tag_no_case!("eight") => { |_| 8 } |
        tag_no_case!("nine") => { |_| 9 } |
        tag_no_case!("ten") => { |_| 10 } |
        tag_no_case!("eleven") => { |_| 11 } |
        tag_no_case!("twelve") => { |_| 12 } |
        tag_no_case!("thirteen") => { |_| 13 } |
        tag_no_case!("fourteen") => { |_| 14 } |
        tag_no_case!("fifteen") => { |_| 15 } |
        tag_no_case!("sixteen") => { |_| 16 } |
        tag_no_case!("seventeen") => { |_| 17 } |
        tag_no_case!("eighteen") => { |_| 18 } |
        tag_no_case!("nineteen") => { |_| 19 } |
        tag_no_case!("twenty") => { |_| 20 }
    )
);

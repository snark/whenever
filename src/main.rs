extern crate whenever;

use std::env;
use whenever::parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = match args.len() {
        2 => args[1].clone(),
        _ => "today".to_string(),
    };
    let dt = parser::parse_date(&arg);
    match dt {
        Some(d) => println!("{}", d),
        None => println!("Could not parse {}", arg),
    }
}

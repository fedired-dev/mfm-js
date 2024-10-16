mod api;
pub mod node;
pub mod parser;
mod util;

pub use api::{parse, parse_simple, parse_with_nest_limit};

pub type MfmParseError<'a> = nom::Err<nom::error::Error<&'a str>>;

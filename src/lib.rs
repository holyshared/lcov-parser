//! lcov-parser to provide an API to parse LCOV report.

#[macro_use]
extern crate lines;
extern crate combine;

pub use self::record::*;
pub use self::parser::*;

mod combinator;

pub mod record;
pub mod parser;

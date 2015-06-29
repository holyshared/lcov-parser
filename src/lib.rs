/// lcov-parser to provide an API to parse LCOV report.

#[macro_use]
extern crate nom;
extern crate lines;

mod combinator;

pub mod record;
pub mod parser;

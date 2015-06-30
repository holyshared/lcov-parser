//! lcov-parser to provide an API to parse LCOV report.

#[macro_use]
extern crate nom;
extern crate lines;

pub use self::record::*;
pub use self::parser::*;
pub use self::util::*;

mod combinator;

pub mod record;
pub mod parser;
pub mod util;

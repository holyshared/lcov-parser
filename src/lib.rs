//! lcov-parser to provide an API to parse LCOV report.

#[macro_use]
extern crate nom;
extern crate lines;
extern crate parser_combinators;

pub use self::record::*;
pub use self::parser::*;
pub use self::parser2::*;
pub use self::combinator2::*;
pub use self::util::*;

mod combinator;
//mod combinator2;

pub mod combinator2;
pub mod record;
pub mod parser;
pub mod parser2;
pub mod util;

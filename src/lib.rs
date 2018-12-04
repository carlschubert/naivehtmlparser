extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod dom;
pub mod html_parser;
use self::html_parser::parse_html_file;

use std::fs;

#[cfg(test)]
mod tests;

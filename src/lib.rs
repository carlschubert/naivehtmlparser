extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod dom;
pub mod errors;
pub mod html_parser;

#[cfg(test)]
mod tests;

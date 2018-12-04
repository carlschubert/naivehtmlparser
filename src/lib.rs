extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod dom;
pub mod html_parser;
pub mod errors;

#[cfg(test)]
mod tests;

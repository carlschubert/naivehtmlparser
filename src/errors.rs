use html_parser::Rule;
use std::convert::From;
use std::error;
use std::fmt;
use pest;

#[derive(Debug)]
pub enum Error {
    PestRuleError(pest::error::Error<Rule>),
    UnexpectedError(String),
    Empty,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Empty => "Nothing to parse",
            Error::UnexpectedError(ref e) => e,
            _ => self.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => Some(self),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(s: pest::error::Error<Rule>) -> Error {
        Error::PestRuleError(s)
    }
}

pub fn invalid_nesting(rule: &str) -> Error {
    Error::UnexpectedError(format!("Invalid nesting in {} rule", rule))
}

pub fn invalid_empty(rule: &str) -> Error {
    Error::UnexpectedError(format!("{} cannot be empty", rule))
}

pub fn invalid_html() -> Error {
    Error::UnexpectedError(format!("File is not valid html"))
}
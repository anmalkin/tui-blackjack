#![allow(dead_code, unused_imports)]

/// Error wrappers to handle various errors from user command line input.
use core::fmt;
use std::{
    io::{self},
    num::ParseFloatError,
};

#[derive(Debug)]
pub enum CliError {
    IoError(io::Error),
    ParseError(ParseFloatError),
    InvalidCommand,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CliError::IoError(ref e) => write!(f, "IO error: {}", e),
            CliError::ParseError(ref e) => write!(f, "IO error: {}", e),
            CliError::InvalidCommand => write!(f, "Invalid command"),
        }
    }
}

impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        CliError::IoError(value)
    }
}

impl From<ParseFloatError> for CliError {
    fn from(value: ParseFloatError) -> Self {
        CliError::ParseError(value)
    }
}

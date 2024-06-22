/// Error wrappers to handle various errors from user command line input.

use core::fmt;
use std::{io::{self, Error}, num::ParseFloatError};

#[derive(Debug)]
pub enum CliError {
    IoError(io::Error),
    ParseError(ParseFloatError),
    InvalidCommand,
    InvalidBet,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CliError::IoError(ref e) => e.fmt(f),
            CliError::ParseError(ref e) => e.fmt(f),
            CliError::InvalidCommand => {
                todo!()
            },
            CliError::InvalidBet => todo!(),
        }
    }
}

impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        todo!()
    }
}

impl From<ParseFloatError> for CliError {
    fn from(value: ParseFloatError) -> Self {
        todo!()
    }
}

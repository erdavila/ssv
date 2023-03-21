use std::error::Error;
use std::fmt::Display;

use self::position::Position;

pub(crate) mod domain;
mod fluent_writer;
mod options;
mod position;
mod reader;
pub mod tokenizer;
pub mod tokens;
mod writer;

#[derive(Clone, Copy, Debug)]
pub enum LineBreak {
    Lf,
    CrLf,
}

type ReadResult<T> = Result<T, ReadError>;
type WriteResult<T> = Result<T, WriteError>;

#[derive(Debug)]
pub enum ReadError {
    UnpairedQuote(Position),
    UnclosedQuotedValue(Position),
    IoError(std::io::Error),
}

impl Error for ReadError {}

impl Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadError::UnpairedQuote(position) => write!(
                f,
                "unpaired quote at {}:{}",
                position.line_number, position.column_number
            ),
            ReadError::UnclosedQuotedValue(position) => write!(
                f,
                "unclosed quoted value {}:{}",
                position.line_number, position.column_number
            ),
            ReadError::IoError(error) => write!(f, "IO Error: {error}"),
        }
    }
}

#[derive(Debug)]
pub enum WriteError {
    IoError(std::io::Error),
}

impl Error for WriteError {}

impl Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteError::IoError(io_error) => write!(f, "IO error: {io_error}"),
        }
    }
}

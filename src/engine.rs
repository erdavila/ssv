use std::error::Error;
use std::fmt::Display;

mod domain;
mod fluent_writer;
mod options;
mod position;
mod reader;
mod tokenizer;
mod tokens;
mod writer;

#[derive(Clone, Copy)]
pub enum LineBreak {
    Lf,
    CrLf,
}

type ReadResult<T> = Result<T, ReadError>;
type WriteResult<T> = Result<T, WriteError>;

#[derive(Debug)]
pub enum ReadError {
    IoError(std::io::Error),
}

impl Error for ReadError {}

impl Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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

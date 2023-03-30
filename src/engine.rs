use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use self::domain::Domain;
use self::position::Position;
use self::reader::Reader;
use self::writer::Writer;

pub mod domain;
pub mod fluent_writer;
pub mod options;
pub mod position;
pub mod reader;
pub mod tokenizer;
pub mod writer;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineBreak {
    Lf,
    CrLf,
}

pub type ReadResult<T> = Result<T, ReadError>;
pub type WriteResult<T> = Result<T, WriteError>;

pub fn read_file<D: Domain, P: AsRef<Path>>(path: P) -> ReadResult<Reader<D, File>> {
    let file = File::open(path)?;
    let reader = read(file);
    Ok(reader)
}

pub fn read<D: Domain, R: Read>(reader: R) -> Reader<D, R> {
    Reader::new(reader)
}

pub fn write_file<'a, D: Domain, P: AsRef<Path>>(
    path: P,
    rows: impl IntoIterator<Item = impl IntoIterator<Item = &'a D::StringSlice>>,
) -> WriteResult<()>
where
    D::StringSlice: 'a,
{
    let file = File::create(path)?;
    write::<'a, D, _>(file, rows)
}

pub fn write<'a, D: Domain, W: Write>(
    writer: W,
    rows: impl IntoIterator<Item = impl IntoIterator<Item = &'a D::StringSlice>>,
) -> WriteResult<()>
where
    D::StringSlice: 'a,
{
    let mut writer: Writer<D, _> = Writer::new(writer);
    writer.write_rows(rows)
}

#[derive(Debug)]
pub enum ReadError {
    UnpairedQuote(Position),
    UnclosedQuotedValue(Position),
    IoError(std::io::Error),
}

impl Error for ReadError {}

impl From<std::io::Error> for ReadError {
    fn from(io_error: std::io::Error) -> Self {
        ReadError::IoError(io_error)
    }
}

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
    InvalidSpacing,
    IoError(std::io::Error),
}

impl Error for WriteError {}

impl From<std::io::Error> for WriteError {
    fn from(io_error: std::io::Error) -> Self {
        WriteError::IoError(io_error)
    }
}

impl Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteError::InvalidSpacing => write!(f, "invalid spacing"),
            WriteError::IoError(io_error) => write!(f, "IO error: {io_error}"),
        }
    }
}

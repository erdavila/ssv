//! Domain-independent generic implementation.

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use self::domain::Domain;
use self::position::Position;
use self::reader::Reader;
use self::writer::Writer;

#[doc(hidden)]
pub mod domain;

pub mod fluent_writer;
pub mod options;
pub mod position;
pub mod reader;
pub mod tokenizer;
pub mod writer;

/// Line-break types.
#[doc = generic_item_warning_doc!("LineBreak")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineBreak {
    /// Line-break consisting only of LF (byte value/codepoint 10).
    Lf,

    /// Line-break consisting of CR (byte value/codepoint 13) and LF (byte value/codepoint 10).
    CrLf,
}

/// A specialized [`Result`] type for read operations.
#[doc = generic_item_warning_doc!("ReadResult")]
pub type ReadResult<T> = Result<T, ReadError>;

/// A specialized [`Result`] type for write operations.
#[doc = generic_item_warning_doc!("WriteResult")]
pub type WriteResult<T> = Result<T, WriteError>;

/// Reads SSV from a file.
#[doc = generic_item_warning_doc!("read_file")]
/// # Example
///
/// ```no_run
/// # let file_path = "";
/// # #[allow(unused_variables)]
/// for row in ssv::chars::read_file(file_path)? {
///     // Use `row`
/// }
/// # Ok::<_, ssv::chars::ReadError>(())
/// ```
pub fn read_file<D: Domain, P: AsRef<Path>>(path: P) -> ReadResult<Reader<D, File>> {
    let file = File::open(path)?;
    let reader = read(file);
    Ok(reader)
}

/// Reads SSV from a reader.
#[doc = generic_item_warning_doc!("read")]
/// It is exactly the same as calling the [`Reader::new`] method.
pub fn read<D: Domain, R: Read>(reader: R) -> Reader<D, R> {
    Reader::new(reader)
}

/// Writes SSV to a file.
#[doc = generic_item_warning_doc!("write_file")]
/// # Example
///
/// ```no_run
/// # let file_path = "";
/// ssv::chars::write_file(
///     file_path,
///     [
///         vec!["value", "another-value"],
///         vec!["yet another value"],
///     ]
/// )?;
/// # Ok::<(), ssv::chars::WriteError>(())
/// ```
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

/// Writes SSV to a writer.
#[doc = generic_item_warning_doc!("write")]
/// It is exactly the same as calling the [`Writer::new`] and
/// [`Writer::write_rows`] methods.
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

/// The error type for read operations.
#[doc = generic_item_warning_doc!("ReadError")]
#[derive(Debug)]
pub enum ReadError {
    /// A quote in a value was not duplicated (see the [rules](crate#rules)).
    UnpairedQuote(Position),

    /// The input ended before reaching the closing quote of a quoted value.
    UnclosedQuotedValue(Position),

    /// An [IO error](std::io::Error) happened when using the underlying reader.
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

/// The error type for write operations.
#[doc = generic_item_warning_doc!("ReadError")]
#[derive(Debug)]
pub enum WriteError {
    /// A value containing non-spacing elements was tried to be used as spacing.
    InvalidSpacing,

    /// An [IO error](std::io::Error) happened when using the underlying writer.
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

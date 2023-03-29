use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use crate::engine::domain::Domain;

type D = super::engine::domain::CharsDomain;

pub use crate::engine::LineBreak;

pub use crate::engine::position::Position;
pub use crate::engine::position::WithPosition;
pub use crate::engine::ReadError;
pub use crate::engine::ReadResult;

pub type Tokenizer<R> = super::engine::tokenizer::Tokenizer<D, R>;
pub type Token = super::engine::tokenizer::Token<D>;
pub type Reader<R> = super::engine::reader::Reader<D, R>;

#[inline]
pub fn read_file<P: AsRef<Path>>(path: P) -> ReadResult<Reader<File>> {
    crate::engine::read_file(path)
}

#[inline]
pub fn read<R: Read>(reader: R) -> Reader<R> {
    crate::engine::read(reader)
}

pub use crate::engine::WriteError;
pub use crate::engine::WriteResult;

pub type Options = super::engine::options::Options<D>;
pub type FluentWriter<W> = super::engine::fluent_writer::FluentWriter<D, W>;
pub type Writer<W> = super::engine::writer::Writer<D, W>;
pub type RowWriter<'a, W> = super::engine::writer::RowWriter<'a, D, W>;

#[inline]
pub fn write_file<'a, P: AsRef<Path>>(
    path: P,
    rows: impl IntoIterator<Item = impl IntoIterator<Item = &'a <D as Domain>::StringSlice>>,
) -> WriteResult<()>
where
    <D as Domain>::StringSlice: 'a,
{
    crate::engine::write_file::<'a, D, P>(path, rows)
}

#[inline]
pub fn write<'a, W: Write>(
    writer: W,
    rows: impl IntoIterator<Item = impl IntoIterator<Item = &'a <D as Domain>::StringSlice>>,
) -> WriteResult<()>
where
    <D as Domain>::StringSlice: 'a,
{
    crate::engine::write::<D, W>(writer, rows)
}

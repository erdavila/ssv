use std::io::Write;
use std::marker::PhantomData;

use crate::engine::domain::Domain;
use crate::engine::LineBreak;

use super::domain::{BytesDomain, DomainStringSlice};
use super::{WriteError, WriteResult};

#[derive(Debug)]
pub struct FluentWriter<D: Domain, W: Write> {
    inner: W,
    phantom: PhantomData<D>,
}

impl<D: Domain, W: Write> FluentWriter<D, W> {
    pub fn new(inner: W) -> Self {
        FluentWriter {
            inner,
            phantom: PhantomData,
        }
    }

    pub fn write_value(self, value: &D::StringSlice) -> WriteResult<Self> {
        self.write_value_raw(value, false)
    }

    pub fn write_quoted_value(self, value: &D::StringSlice) -> WriteResult<Self> {
        self.write_value_raw(value, true)
    }

    fn write_value_raw(mut self, value: &D::StringSlice, quoted: bool) -> WriteResult<Self> {
        let prepared_value = PreparedValue::from(value.as_bytes());
        let quoted = quoted || prepared_value.must_be_quoted;

        if quoted {
            self.write(&[BytesDomain::QUOTE])?;
        }
        self.write(&prepared_value.bytes)?;
        if quoted {
            self.write(&[BytesDomain::QUOTE])?;
        }

        Ok(self)
    }

    pub fn write_spacing(mut self, value: &D::StringSlice) -> WriteResult<Self> {
        let bytes = value.as_bytes();
        for byte in bytes {
            if !BytesDomain::is_spacing_element(*byte) {
                return Err(WriteError::InvalidSpacing);
            }
        }
        self.write(bytes)?;
        Ok(self)
    }

    pub fn write_line_break(self) -> WriteResult<Self> {
        self.write_this_line_break(LineBreak::Lf)
    }

    pub fn write_this_line_break(mut self, line_break: LineBreak) -> WriteResult<Self> {
        let bytes: &[u8] = match line_break {
            LineBreak::Lf => &[BytesDomain::LF],
            LineBreak::CrLf => &[BytesDomain::CR, BytesDomain::LF],
        };
        self.write(bytes)?;
        Ok(self)
    }

    pub fn write_comment(mut self, comment: &D::StringSlice) -> WriteResult<Self> {
        self.write(&[BytesDomain::HASH])?;
        self.write(comment.as_bytes())?;
        Ok(self)
    }

    fn write(&mut self, bytes: &[u8]) -> WriteResult<()> {
        self.inner.write_all(bytes)?;
        Ok(())
    }

    pub fn finish(mut self) -> WriteResult<()> {
        self.inner.flush()?;
        Ok(())
    }

    pub fn default_spacing(&self) -> &D::StringSlice {
        todo!()
    }

    pub fn set_default_spacing(self, _spacing: D::String) -> WriteResult<Self> {
        todo!()
    }

    pub fn default_line_break(&self) -> LineBreak {
        todo!()
    }

    pub fn set_default_line_break(self, _line_break: LineBreak) -> WriteResult<Self> {
        todo!()
    }

    pub fn always_quoted(&self) -> bool {
        todo!()
    }

    pub fn set_always_quoted(self, _always_quoted: bool) -> WriteResult<Self> {
        todo!()
    }
}

struct PreparedValue {
    bytes: Vec<u8>,
    must_be_quoted: bool,
}

impl PreparedValue {
    fn from(original_bytes: &[u8]) -> PreparedValue {
        let mut only_quotes = true;
        let mut spacing_or_line_break = false;

        let mut bytes = Vec::new();
        for byte in original_bytes {
            bytes.push(*byte);

            if *byte == BytesDomain::QUOTE {
                bytes.push(*byte);
            } else {
                only_quotes = false;

                if *byte == BytesDomain::LF || BytesDomain::is_spacing_element(*byte) {
                    spacing_or_line_break = true;
                }
            }
        }

        PreparedValue {
            bytes,
            must_be_quoted: only_quotes || spacing_or_line_break,
        }
    }
}

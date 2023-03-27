use std::io::Write;
use std::marker::PhantomData;

use crate::engine::domain::Domain;
use crate::engine::LineBreak;

use super::domain::{BytesDomain, DomainStringSlice};
use super::{WriteError, WriteResult};

#[derive(Debug)]
pub struct FluentWriter<D: Domain, W: Write> {
    inner: W,
    state: State,
    phantom: PhantomData<D>,
}

impl<D: Domain, W: Write> FluentWriter<D, W> {
    pub fn new(inner: W) -> Self {
        FluentWriter {
            inner,
            state: State::LineBegin,
            phantom: PhantomData,
        }
    }

    pub fn write_value(self, value: &D::StringSlice) -> WriteResult<Self> {
        self.write_value_raw(value, false)
    }

    pub fn write_quoted_value(self, value: &D::StringSlice) -> WriteResult<Self> {
        self.write_value_raw(value, true)
    }

    fn write_value_raw(self, value: &D::StringSlice, quoted: bool) -> WriteResult<Self> {
        let mut this = match self.state {
            State::Value => self.write_spacing_raw(&[b' '])?,
            State::Comment => self.write_line_break()?,
            _ => self,
        };

        let prepared_value = PreparedValue::from(value.as_bytes());
        let quoted = quoted
            || prepared_value.must_be_quoted
            || (this.state == State::LineBegin
                && prepared_value.bytes.first() == Some(&BytesDomain::HASH));

        if quoted {
            this.write(&[BytesDomain::QUOTE])?;
        }
        this.write(&prepared_value.bytes)?;
        if quoted {
            this.write(&[BytesDomain::QUOTE])?;
        }

        this.state = State::Value;
        Ok(this)
    }

    pub fn write_spacing(self, spacing: &D::StringSlice) -> WriteResult<Self> {
        if !D::is_valid_spacing(spacing) {
            return Err(WriteError::InvalidSpacing);
        }

        let this = match self.state {
            State::Comment => self.write_line_break()?,
            _ => self,
        };

        this.write_spacing_raw(spacing.as_bytes())
    }

    fn write_spacing_raw(mut self, spacing_bytes: &[u8]) -> WriteResult<Self> {
        self.write(spacing_bytes)?;

        self.state = State::Spacing;
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

        self.state = State::LineBegin;
        Ok(self)
    }

    pub fn write_comment(self, comment: &D::StringSlice) -> WriteResult<Self> {
        let mut this = match self.state {
            State::Value | State::Spacing | State::Comment => self.write_line_break()?,
            _ => self,
        };

        this.write(&[BytesDomain::HASH])?;
        this.write(comment.as_bytes())?;

        this.state = State::Comment;
        Ok(this)
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

#[derive(PartialEq, Eq, Debug)]
enum State {
    Value,
    Spacing,
    LineBegin,
    Comment,
}

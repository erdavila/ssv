//! Writes SSV using a fluent interface. Automatically inserts delimiters.

use std::io::Write;

use crate::engine::domain::Domain;
use crate::engine::LineBreak;

use super::domain::{BytesDomain, DomainStringSlice};
use super::options::Options;
use super::{WriteError, WriteResult};

/// Has a fluent interface to write SSV to a byte writer.
#[doc = generic_item_warning_doc!("FluentWriter")]
/// Spacing and line-breaks are automatically written as required.
///
/// The underlying byte writer is flushed when the [`FluentWriter`] is dropped.
/// Eventual flush errors will be ignored. Prefer to explicitly call the
/// [`finish`](FluentWriter::finish) method instead of letting the
/// [`FluentWriter`] being dropped.
///
/// # Example
///
/// ```
/// use ssv::chars::FluentWriter;
/// let mut output = Vec::new();
///
/// let fluent_writer = FluentWriter::new(&mut output);
///
/// fluent_writer
///     .write_value("value")?
///     .write_value("another value")? // automatic spacing
///     .write_line_break()?
///     .write_value("finalvalue")?
///     .finish()?;
/// # Ok::<(), ssv::chars::WriteError>(())
/// ```
#[derive(Debug)]
pub struct FluentWriter<D: Domain, W: Write> {
    inner: W,
    state: State,
    options: Options<D>,
}

impl<D: Domain, W: Write> FluentWriter<D, W> {
    /// Creates an instance that writes SSV to the given byte writer.
    pub fn new(inner: W) -> Self {
        FluentWriter {
            inner,
            state: State::LineBegin,
            options: Options::new(),
        }
    }

    /// Writes a value.
    ///
    /// The value is enclosed in quotes if required (check the [rules](crate#rules))
    /// or if the [`always_quoted`](FluentWriter::always_quoted) option is `true`.
    ///
    /// If the last wroten item was another value, then the
    /// [default spacing](FluentWriter::default_spacing) is automatically written
    /// before this value.
    ///
    /// If the last wroten item was a comment, then the
    /// [default line-break](FluentWriter::default_line_break) is automatically
    /// written before this value.
    pub fn write_value(self, value: &D::StringSlice) -> WriteResult<Self> {
        self.write_value_raw(value, false)
    }

    /// Writes a value enclosed in quotes.
    ///
    /// If the last wroten item was another value, then the
    /// [default spacing](FluentWriter::default_spacing) is automatically written
    /// before this value.
    ///
    /// If the last wroten item was a comment, then the
    /// [default line-break](FluentWriter::default_line_break) is automatically
    /// written before this value.
    pub fn write_quoted_value(self, value: &D::StringSlice) -> WriteResult<Self> {
        self.write_value_raw(value, true)
    }

    fn write_value_raw(mut self, value: &D::StringSlice, quoted: bool) -> WriteResult<Self> {
        let mut this = match self.state {
            State::Value => {
                Self::write_spacing_raw(
                    &mut self.inner,
                    self.options.default_spacing().as_bytes(),
                    &mut self.state,
                )?;
                self
            }
            State::Comment => self.write_line_break()?,
            _ => self,
        };

        let prepared_value = PreparedValue::from(value.as_bytes());
        let quoted = quoted
            || prepared_value.must_be_quoted
            || this.always_quoted()
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

    /// Writes the specified spacing.
    ///
    /// If the last wroten item was a comment, then the
    /// [default line-break](FluentWriter::default_line_break) is automatically
    /// written before this value.
    pub fn write_spacing(self, spacing: &D::StringSlice) -> WriteResult<Self> {
        if !D::is_valid_spacing(spacing) {
            return Err(WriteError::InvalidSpacing);
        }

        let mut this = match self.state {
            State::Comment => self.write_line_break()?,
            _ => self,
        };

        Self::write_spacing_raw(&mut this.inner, spacing.as_bytes(), &mut this.state)?;
        Ok(this)
    }

    fn write_spacing_raw(
        writer: &mut W,
        spacing_bytes: &[u8],
        state: &mut State,
    ) -> WriteResult<()> {
        Self::write_raw(writer, spacing_bytes)?;
        *state = State::Spacing;
        Ok(())
    }

    /// Writes the [default line-break](FluentWriter::default_line_break).
    pub fn write_line_break(self) -> WriteResult<Self> {
        let line_break = self.default_line_break();
        self.write_this_line_break(line_break)
    }

    /// Writes the specified line-break.
    pub fn write_this_line_break(mut self, line_break: LineBreak) -> WriteResult<Self> {
        let bytes: &[u8] = match line_break {
            LineBreak::Lf => &[BytesDomain::LF],
            LineBreak::CrLf => &[BytesDomain::CR, BytesDomain::LF],
        };
        self.write(bytes)?;

        self.state = State::LineBegin;
        Ok(self)
    }

    /// Writes the comment.
    ///
    /// The HASH sign (`#`) is written before the comment.
    ///
    /// If the last wroten item was a value, spacing, or another comment, then
    /// the [default line-break](FluentWriter::default_line_break) is automatically
    /// written before the HASH sign and the comment.
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
        Self::write_raw(&mut self.inner, bytes)
    }

    fn write_raw(writer: &mut W, bytes: &[u8]) -> WriteResult<()> {
        writer.write_all(bytes)?;
        Ok(())
    }

    /// Finalizes the object by flushing the underlying byte writer.
    ///
    /// Prefer to explicitly call this method instead of letting the [`FluentWriter`]
    /// being dropped.
    pub fn finish(mut self) -> WriteResult<()> {
        self.inner.flush()?;
        Ok(())
    }

    /// Returns the default spacing used by the methods
    /// [`write_value`](FluentWriter::write_value) and
    /// [`write_quoted_value`](FluentWriter::write_quoted_value).
    ///
    /// This is the same as `self.options().default_spacing()`.
    pub fn default_spacing(&self) -> &D::StringSlice {
        self.options.default_spacing()
    }

    /// Sets the [default spacing](FluentWriter::default_spacing).
    ///
    /// This has the same effect as `self.options_mut().set_default_spacing(spacing)`.
    pub fn set_default_spacing(mut self, spacing: D::String) -> WriteResult<Self> {
        self.options.set_default_spacing(spacing)?;
        Ok(self)
    }

    /// Returns the default line-break used by the methods
    /// [`write_value`](FluentWriter::write_value),
    /// [`write_quoted_value`](FluentWriter::write_quoted_value),
    /// [`write_spacing`](FluentWriter::write_spacing),
    /// [`write_line_break`](FluentWriter::write_line_break) and
    /// [`write_comment`](FluentWriter::write_comment).
    ///
    /// This is the same as `self.options().default_line_break()`.
    pub fn default_line_break(&self) -> LineBreak {
        self.options.default_line_break()
    }

    /// Sets the [default line-break](FluentWriter::default_line_break).
    ///
    /// This has the same effect as `self.options_mut().set_default_line_break(line_break)`.
    pub fn set_default_line_break(mut self, line_break: LineBreak) -> Self {
        self.options.set_default_line_break(line_break);
        self
    }

    /// Returns whether the [write_value](FluentWriter::write_value) should
    /// automatically quote the values.
    ///
    /// This is the same as `self.options().always_quoted()`.
    pub fn always_quoted(&self) -> bool {
        self.options.always_quoted()
    }

    /// Sets whether the [write_value](FluentWriter::write_value) should
    /// automatically quote the values.
    ///
    /// This is the same as `self.options_mut().set_always_quoted(always_quoted)`.
    pub fn set_always_quoted(mut self, always_quoted: bool) -> Self {
        self.options.set_always_quoted(always_quoted);
        self
    }

    /// Returns a reference to the associated [`Options`] object.
    pub fn options(&self) -> &Options<D> {
        &self.options
    }

    /// Returns a mutable reference to the associated [`Options`] object.
    pub fn options_mut(&mut self) -> &mut Options<D> {
        &mut self.options
    }

    /// Replaces the associated [`Options`] object.
    pub fn set_options(mut self, options: Options<D>) -> WriteResult<Self> {
        self.options = options;
        Ok(self)
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

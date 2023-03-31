//! Writes SSV following a row-oriented structure.
use std::io::Write;

use crate::engine::domain::Domain;

use super::fluent_writer::FluentWriter;
use super::options::Options;
use super::WriteResult;

/// Follows a row-oriented structure to write SSV to a byte writer.
#[doc = generic_item_warning_doc!("Writer")]
/// The methods ([`write_rows`](Writer::write_rows) and [`write_row`](Writer::write_row))
/// write entire rows at once. The associated [`options`](Writer::options) object
/// govern how the items are written:
/// * the values inside a row are separated by the [default spacing](Options::default_spacing);
/// * all rows are ended with the [default line-break](Options::default_line_break);
/// * if [`always_quoted`](Options::always_quoted) is `true`, all values are automatically quoted.
///
/// It also has the method [`new_row`](Writer::new_row), which returns an object
/// to write the values in the context of a row.
///
/// The underlying byte writer is flushed when the [`Writer`] is dropped.
/// Eventual flush errors will be ignored. Prefer to explicitly call the
/// [`finish`](Writer::finish) method instead of letting the [`Writer`] being
/// dropped.
///
/// # Invalid state after erroring
///
/// After an error is returned by any method of this object or an associated
/// [`RowWriter`], the [`Writer`] become unusable and will panic if any method
/// is called.
///
/// # Example
///
/// ```
/// use ssv::chars::Writer;
/// let mut output = Vec::new();
///
/// let mut writer = Writer::new(&mut output);
///
/// writer.write_rows([
///     vec!["value", "another value"],
///     vec!["value in another row"],
/// ])?;
/// writer.write_row(["third row"])?;
/// writer.finish()?;
/// # Ok::<_, ssv::chars::WriteError>(())
/// ```
pub struct Writer<D: Domain, W: Write> {
    fluent: Option<FluentWriter<D, W>>,
}

const INVALID_WRITER_MESSAGE: &str = "the Writer is invalid due to a previous error";

impl<D: Domain, W: Write> Writer<D, W> {
    /// Creates an instance that writes SSV to the given byte writer.
    pub fn new(writer: W) -> Self {
        Writer {
            fluent: Some(FluentWriter::new(writer)),
        }
    }

    /// Writes several rows.
    pub fn write_rows<'a>(
        &mut self,
        rows: impl IntoIterator<Item = impl IntoIterator<Item = &'a D::StringSlice>>,
    ) -> WriteResult<()>
    where
        D::StringSlice: 'a,
    {
        for row in rows {
            self.write_row(row)?;
        }
        Ok(())
    }

    /// Writes a single row.
    pub fn write_row<'a>(
        &mut self,
        row: impl IntoIterator<Item = &'a D::StringSlice>,
    ) -> WriteResult<()>
    where
        D::StringSlice: 'a,
    {
        let mut row_writer = self.new_row();
        row_writer.write_values(row)?;
        row_writer.finish()
    }

    /// Returns an object to write values in the context of a row.
    ///
    /// Only one instance of [`RowWriter`] can be obtained each time.
    ///
    /// # Example
    ///
    /// ```
    /// use ssv::chars::Writer;
    /// let mut output = Vec::new();
    ///
    /// let mut writer = Writer::new(&mut output);
    ///
    /// let mut row_writer = writer.new_row();
    /// row_writer.write_values(["value", "another value"])?;
    /// row_writer.finish()?;
    ///
    /// let mut row_writer = writer.new_row();
    /// row_writer.write_value("before custom spacing")?;
    /// row_writer.write_spacing(" \t ")?;
    /// row_writer.write_value("after custom spacing")?;
    /// row_writer.finish()?;
    ///
    /// writer.finish()?;
    /// # Ok::<_, ssv::chars::WriteError>(())
    /// ```
    pub fn new_row(&mut self) -> RowWriter<'_, D, W> {
        RowWriter { writer: self }
    }

    /// Writes a comment line.
    ///
    /// The comment is automatically preceded by the HASH sign (`#`) and followed
    /// by the [default line-break](Options::default_line_break).
    pub fn write_comment_line(&mut self, comment: &D::StringSlice) -> WriteResult<()> {
        let row_writer = self.new_row();
        row_writer
            .writer
            .use_fluent(|fluent| fluent.write_comment(comment))?;
        row_writer.finish()
    }

    /// Finalizes the object by flushing the underlying byte writer.
    ///
    /// Prefer to explicitly call this method instead of letting the [`Writer`]
    /// being dropped.
    pub fn finish(mut self) -> WriteResult<()> {
        self.take_fluent().finish()
    }

    fn use_fluent<F>(&mut self, f: F) -> WriteResult<()>
    where
        F: FnOnce(FluentWriter<D, W>) -> WriteResult<FluentWriter<D, W>>,
    {
        let fluent = self.take_fluent();
        let fluent = f(fluent)?;
        self.fluent = Some(fluent);
        Ok(())
    }

    fn take_fluent(&mut self) -> FluentWriter<D, W> {
        self.fluent.take().expect(INVALID_WRITER_MESSAGE)
    }

    /// Returns a reference to the associated [`Options`] object.
    pub fn options(&self) -> &Options<D> {
        self.fluent
            .as_ref()
            .expect(INVALID_WRITER_MESSAGE)
            .options()
    }

    /// Returns a mutable reference to the associated [`Options`] object.
    pub fn options_mut(&mut self) -> &mut Options<D> {
        self.fluent
            .as_mut()
            .expect(INVALID_WRITER_MESSAGE)
            .options_mut()
    }

    /// Replaces the associated [`Options`] object.
    pub fn set_options(&mut self, options: Options<D>) -> WriteResult<()> {
        self.use_fluent(|fluent| fluent.set_options(options))
    }
}

/// An object that writes values in the context of a row.
#[doc = generic_item_warning_doc!("RowWriter")]
/// This object is originated from a [`Writer`] and is governed by the associated
/// [`options`](Writer::options) object in the same way.
///
/// The [default line-break](Options::default_line_break) is automatically
/// written when [`RowWriter`] is dropped. Eventual writing errors will be
/// ignored. Prefer to explicitly call the [`finish`](Writer::finish) method
/// instead of letting the [`RowWriter`] being dropped.
///
/// # Invalid state after erroring
///
/// After an error is returned by any method of this object, this object and
/// the originating [`Writer`] become unusable and will panic if any method is
/// called.
pub struct RowWriter<'a, D: Domain, W: Write> {
    writer: &'a mut Writer<D, W>,
}

impl<'a, D: Domain, W: Write> RowWriter<'a, D, W> {
    /// Writes several values.
    pub fn write_values<'b>(
        &mut self,
        values: impl IntoIterator<Item = &'b D::StringSlice>,
    ) -> WriteResult<()>
    where
        D::StringSlice: 'b,
    {
        for value in values {
            self.write_value(value)?;
        }
        Ok(())
    }

    /// Writes a single value.
    pub fn write_value(&mut self, value: &D::StringSlice) -> WriteResult<()> {
        self.writer.use_fluent(|fluent| fluent.write_value(value))
    }

    /// Writes custom spacing.
    ///
    /// When this method is used, the [default spacing](Options::default_spacing)
    /// is *not* used to separate the values immediately before and after the
    /// custom spacing.
    pub fn write_spacing(&mut self, spacing: &D::StringSlice) -> WriteResult<()> {
        self.writer
            .use_fluent(|fluent| fluent.write_spacing(spacing))
    }

    /// Finalizes the row by writing the [default line-break](Options::default_line_break).
    ///
    /// Prefer to explicitly call this method instead of letting the [`RowWriter`]
    /// being dropped.
    pub fn finish(mut self) -> WriteResult<()> {
        self.finish_row()?;
        std::mem::forget(self);
        Ok(())
    }

    fn finish_row(&mut self) -> WriteResult<()> {
        self.writer.use_fluent(|fluent| fluent.write_line_break())
    }

    /// Returns a reference to the [`Options`] object associated to the
    /// originating [`Writer`].
    pub fn options(&self) -> &Options<D> {
        self.writer.options()
    }

    /// Returns a mutable reference to the [`Options`] object associated to the
    /// originating [`Writer`].
    pub fn options_mut(&mut self) -> &mut Options<D> {
        self.writer.options_mut()
    }

    /// Replaces the [`Options`] object associated to the originating [`Writer`].
    pub fn set_options(&mut self, options: Options<D>) -> WriteResult<()> {
        self.writer.set_options(options)
    }
}

impl<'a, D: Domain, W: Write> Drop for RowWriter<'a, D, W> {
    fn drop(&mut self) {
        self.finish_row().unwrap();
    }
}

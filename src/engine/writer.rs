use std::io::Write;

use crate::engine::domain::Domain;

use super::fluent_writer::FluentWriter;
use super::options::Options;
use super::WriteResult;

pub struct Writer<D: Domain, W: Write> {
    fluent: Option<FluentWriter<D, W>>,
}

impl<D: Domain, W: Write> Writer<D, W> {
    pub fn new(writer: W) -> Self {
        Writer {
            fluent: Some(FluentWriter::new(writer)),
        }
    }

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

    pub fn new_row(&mut self) -> RowWriter<'_, D, W> {
        RowWriter { writer: self }
    }

    pub fn write_comment_line(&mut self, comment: &D::StringSlice) -> WriteResult<()> {
        let row_writer = self.new_row();
        row_writer
            .writer
            .use_fluent(|fluent| fluent.write_comment(comment))?;
        row_writer.finish()
    }

    pub fn finish(mut self) -> WriteResult<()> {
        self.take_fluent()?.finish()
    }

    fn use_fluent<F>(&mut self, f: F) -> WriteResult<()>
    where
        F: FnOnce(FluentWriter<D, W>) -> WriteResult<FluentWriter<D, W>>,
    {
        let fluent = self.take_fluent()?;
        self.fluent = Some(f(fluent)?);
        Ok(())
    }

    fn take_fluent(&mut self) -> WriteResult<FluentWriter<D, W>> {
        match self.fluent.take() {
            Some(fluent) => Ok(fluent),
            None => todo!(),
        }
    }

    pub fn options(&self) -> &Options<D> {
        todo!()
    }

    pub fn options_mut(&mut self) -> &mut Options<D> {
        todo!()
    }

    pub fn set_options(&mut self, _options: Options<D>) {
        todo!()
    }
}

pub struct RowWriter<'a, D: Domain, W: Write> {
    writer: &'a mut Writer<D, W>,
}

impl<'a, D: Domain, W: Write> RowWriter<'a, D, W> {
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

    pub fn write_value(&mut self, value: &D::StringSlice) -> WriteResult<()> {
        self.writer.use_fluent(|fluent| fluent.write_value(value))
    }

    pub fn write_spacing(&mut self, spacing: &D::StringSlice) -> WriteResult<()> {
        self.writer
            .use_fluent(|fluent| fluent.write_spacing(spacing))
    }

    pub fn finish(mut self) -> WriteResult<()> {
        self.finish_row()?;
        std::mem::forget(self);
        Ok(())
    }

    fn finish_row(&mut self) -> WriteResult<()> {
        self.writer.use_fluent(|fluent| fluent.write_line_break())
    }

    pub fn options(&self) -> &Options<D> {
        todo!()
    }

    pub fn options_mut(&mut self) -> &mut Options<D> {
        todo!()
    }

    pub fn set_options(&mut self, _options: Options<D>) {
        todo!()
    }
}

impl<'a, D: Domain, W: Write> Drop for RowWriter<'a, D, W> {
    fn drop(&mut self) {
        self.finish_row().unwrap();
    }
}

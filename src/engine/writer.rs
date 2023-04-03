use std::io::Write;
use std::marker::PhantomData;

use crate::engine::domain::Domain;

use super::options::Options;
use super::WriteResult;

pub struct Writer<D: Domain, W: Write> {
    phantom: PhantomData<(D, W)>,
}

impl<D: Domain, W: Write> Writer<D, W> {
    pub fn new(_inner: W) -> Self {
        todo!()
    }

    pub fn write_rows<'a>(
        &mut self,
        _rows: impl IntoIterator<Item = impl IntoIterator<Item = &'a D::StringSlice>>,
    ) -> WriteResult<()>
    where
        D::StringSlice: 'a,
    {
        todo!()
    }

    pub fn write_row<'a>(
        &mut self,
        _row: impl IntoIterator<Item = &'a D::StringSlice>,
    ) -> WriteResult<()>
    where
        D::StringSlice: 'a,
    {
        todo!()
    }

    pub fn new_row(&mut self) -> RowWriter<D> {
        todo!()
    }

    pub fn write_comment_line(&mut self, _comment: &D::StringSlice) -> WriteResult<()> {
        todo!()
    }

    pub fn finish(self) -> WriteResult<()> {
        todo!()
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

pub struct RowWriter<D: Domain> {
    phantom: PhantomData<D>,
}

impl<D: Domain> RowWriter<D> {
    pub fn write_values<'a>(
        &mut self,
        _values: impl IntoIterator<Item = &'a D::StringSlice>,
    ) -> WriteResult<()>
    where
        D::StringSlice: 'a,
    {
        todo!()
    }

    pub fn write_value(&mut self, _value: &D::StringSlice) -> WriteResult<()> {
        todo!()
    }

    pub fn write_spacing(&mut self, _spacing: &D::StringSlice) -> WriteResult<()> {
        todo!()
    }

    pub fn finish(self) -> WriteResult<()> {
        todo!()
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

use std::io::Write;
use std::marker::PhantomData;

use crate::engine::domain::Domain;
use crate::engine::LineBreak;

use super::WriteResult;

pub struct FluentWriter<D: Domain, W: Write> {
    phantom: PhantomData<(D, W)>,
}

impl<D: Domain, W: Write> FluentWriter<D, W> {
    pub fn new(_inner: W) -> Self {
        todo!()
    }

    pub fn write_value(self, _value: &D::StringSlice) -> WriteResult<Self> {
        todo!()
    }

    pub fn write_quoted_value(self, _value: &D::StringSlice) -> WriteResult<Self> {
        todo!()
    }

    pub fn write_spacing(self, _value: &D::StringSlice) -> WriteResult<Self> {
        todo!()
    }

    pub fn write_line_break(self) -> WriteResult<Self> {
        todo!()
    }

    pub fn write_non_default_line_break(self, _line_break: LineBreak) -> WriteResult<Self> {
        todo!()
    }

    pub fn write_comment(self, _comment: &D::StringSlice) -> WriteResult<Self> {
        todo!()
    }

    pub fn finish(self) -> WriteResult<()> {
        todo!()
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

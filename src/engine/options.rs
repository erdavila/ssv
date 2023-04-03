use crate::engine::domain::Domain;
use crate::engine::LineBreak;

use super::WriteResult;

#[derive(Clone)]
pub struct Options<D: Domain> {
    default_spacing: D::String,
    default_line_break: LineBreak,
    always_quoted: bool,
}

impl<D: Domain> Options<D> {
    pub fn default_spacing(&self) -> &D::StringSlice {
        todo!()
    }

    pub fn set_default_spacing(&mut self, _spacing: D::String) -> WriteResult<()> {
        todo!()
    }

    pub fn default_line_break(&self) -> LineBreak {
        todo!()
    }

    pub fn set_default_line_break(&mut self, _line_break: LineBreak) {
        todo!()
    }

    pub fn always_quoted(&self) -> bool {
        todo!()
    }

    pub fn set_always_quoted(&mut self, _always_quoted: bool) {
        todo!()
    }
}

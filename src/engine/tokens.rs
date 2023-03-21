use crate::engine::domain::Domain;
use crate::engine::LineBreak;

#[derive(Debug)]
pub enum Token<D: Domain> {
    UnquotedValue(UnquotedValue<D>),
    QuotedValue(QuotedValue<D>),
    Spacing(Spacing<D>),
    LineBreak(LineBreak),
    Comment(Comment<D>),
}

#[derive(Debug)]
pub struct UnquotedValue<D: Domain>(pub D::String);

#[derive(Debug)]
pub struct QuotedValue<D: Domain>(pub D::String);

#[derive(Debug)]
pub struct Spacing<D: Domain>(pub D::String);

#[derive(Debug)]
pub struct Comment<D: Domain>(pub D::String);

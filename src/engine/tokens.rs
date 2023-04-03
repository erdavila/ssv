use crate::engine::domain::Domain;
use crate::engine::LineBreak;

pub enum Token<D: Domain> {
    UnquotedValue(UnquotedValue<D>),
    QuotedValue(QuotedValue<D>),
    Spacing(Spacing<D>),
    LineBreak(LineBreak),
    Comment(Comment<D>),
}

pub struct UnquotedValue<D: Domain>(D::String);

pub struct QuotedValue<D: Domain>(D::String);

pub struct Spacing<D: Domain>(D::String);

pub struct Comment<D: Domain>(D::String);

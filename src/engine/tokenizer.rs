use std::io::Read;
use std::iter::Peekable;

use crate::engine::domain::Domain;

use super::position::{Position, WithPosition};
use super::tokens::Token;
use super::ReadResult;

pub struct Tokenizer<D: Domain, R: Read> {
    elements: Peekable<D::ElementIterator<R>>,
    state: State,
    position: Position,
    current_token_position: Position,
}

impl<D: Domain, R: Read> Tokenizer<D, R> {
    pub fn new(inner: R) -> Self {
        Tokenizer {
            elements: D::element_iterator(inner).peekable(),
            state: State::Begin,
            position: Position {
                line_number: 1,
                column_number: 0,
            },
            current_token_position: Position {
                line_number: 0,
                column_number: 0,
            },
        }
    }
}

impl<D: Domain, R: Read> Iterator for Tokenizer<D, R> {
    type Item = ReadResult<WithPosition<Token<D>>>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

enum State {
    Begin,
}

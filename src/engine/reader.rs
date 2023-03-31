//! Reads SSV in a row-oriented way.

use std::io::Read;
use std::iter::FusedIterator;

use crate::engine::domain::Domain;

use super::tokenizer::{Token, Tokenizer};
use super::ReadResult;

/// Reads SSV rows from a byte reader.
#[doc = generic_item_warning_doc!("Reader")]
/// It is an iterator of SSV rows. Each row is [`Vec`] of values.
///
/// # Example
///
/// ```
/// use ssv::chars::Reader;
///
/// let input = "value\nvalue";
///
/// let mut reader = Reader::new(input.as_bytes());
///
/// while let Some(result) = reader.next() {
///     let row = result?;
///     println!("Values in row:");
///     for value in row {
///         println!("  {value:?}");
///     }
/// }
/// # Ok::<_, ssv::chars::ReadError>(())
/// ```
pub struct Reader<D: Domain, R: Read> {
    tokenizer: Tokenizer<D, R>,
    state: Option<State<D>>,
}

impl<D: Domain, R: Read> Reader<D, R> {
    /// Creates an instance that reads SSV from the given byte reader.
    pub fn new(inner: R) -> Self {
        Reader {
            tokenizer: Tokenizer::new(inner),
            state: Some(State::Begin),
        }
    }

    fn process(&mut self, token: Token<D>, state: State<D>) -> ProcessResult<D> {
        match state {
            State::Begin => match token {
                Token::UnquotedValue(value) | Token::QuotedValue(value) => {
                    ProcessResult::NextState(State::Row(vec![value]))
                }
                Token::Spacing(_) => ProcessResult::NextState(State::Row(Vec::new())),
                Token::LineBreak(_) => ProcessResult::ReturnRow(Vec::new()),
                Token::Comment(_) => ProcessResult::NextState(State::Comment),
            },
            State::Row(mut row) => match token {
                Token::UnquotedValue(value) | Token::QuotedValue(value) => {
                    row.push(value);
                    ProcessResult::NextState(State::Row(row))
                }
                Token::Spacing(_) => ProcessResult::NextState(State::Row(row)),
                Token::LineBreak(_) => ProcessResult::ReturnRow(row),
                Token::Comment(_) => unreachable!(),
            },
            State::Comment => {
                matches!(token, Token::LineBreak(_));
                ProcessResult::NextState(State::Begin)
            }
        }
    }

    fn finish(&mut self, state: State<D>) -> Option<Vec<D::String>> {
        match state {
            State::Row(row) => Some(row),
            State::Begin | State::Comment => None,
        }
    }
}

impl<D: Domain, R: Read> Iterator for Reader<D, R> {
    type Item = ReadResult<Vec<D::String>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = self.state.take()?;

        while let Some(result) = self.tokenizer.next() {
            match result {
                Ok(token) => match self.process(token.value, state) {
                    ProcessResult::ReturnRow(row) => {
                        self.state = Some(State::Begin);
                        return Some(Ok(row));
                    }
                    ProcessResult::NextState(next_state) => state = next_state,
                },
                Err(error) => {
                    self.state = None;
                    return Some(Err(error));
                }
            }
        }

        self.finish(state).map(Ok)
    }
}

impl<D: Domain, R: Read> FusedIterator for Reader<D, R> {}

enum State<D: Domain> {
    Begin,
    Row(Vec<D::String>),
    Comment,
}

enum ProcessResult<D: Domain> {
    ReturnRow(Vec<D::String>),
    NextState(State<D>),
}

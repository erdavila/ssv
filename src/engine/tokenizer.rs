use std::io::Read;
use std::iter::{FusedIterator, Peekable};

use crate::engine::domain::{Domain, DomainString};
use crate::engine::{LineBreak, ReadError};

use super::position::{Position, WithPosition};
use super::ReadResult;

#[derive(Debug)]
pub enum Token<D: Domain> {
    UnquotedValue(D::String),
    QuotedValue(D::String),
    Spacing(D::String),
    LineBreak(LineBreak),
    Comment(D::String),
}

pub struct Tokenizer<D: Domain, R: Read> {
    elements: Peekable<D::ElementIterator<R>>,
    state: Option<State<D>>,
    position: Position,
    current_token_position: Position,
}

impl<D: Domain, R: Read> Tokenizer<D, R> {
    pub fn new(inner: R) -> Self {
        Tokenizer {
            elements: D::element_iterator(inner).peekable(),
            state: Some(State::Begin),
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

    fn process(
        &mut self,
        element: D::Element,
        state: State<D>,
    ) -> ReadResult<(State<D>, Option<Token<D>>)> {
        macro_rules! next_element_is_lf {
            () => {
                if let Some(Ok(element)) = self.elements.peek() {
                    *element == D::LF
                } else {
                    false
                }
            };
        }

        let next_state = match state {
            State::Begin => {
                if D::is_spacing_element(element) {
                    State::Spacing(D::String::from_element(element))
                } else if element == D::QUOTE {
                    State::QuotesPrefix(1)
                } else if element == D::LF {
                    return Ok((state, Some(Token::LineBreak(LineBreak::Lf))));
                } else if element == D::CR && next_element_is_lf!() {
                    State::CrInLineBreak
                } else if element == D::HASH {
                    State::Comment(D::String::new())
                } else {
                    let value = D::String::from_element(element);
                    State::UnquotedValue(value)
                }
            }
            State::UnquotedValue(mut value) => {
                if element == D::QUOTE {
                    State::QuoteInUnquotedValue(value)
                } else {
                    let next_state = if D::is_spacing_element(element) {
                        Some(State::Spacing(D::String::from_element(element)))
                    } else if element == D::LF {
                        Some(State::LfLineBreak)
                    } else if element == D::CR && next_element_is_lf!() {
                        Some(State::CrInLineBreak)
                    } else {
                        None
                    };

                    if let Some(next_state) = next_state {
                        return Ok((next_state, Some(Token::UnquotedValue(value))));
                    } else {
                        value.push(element);
                        State::UnquotedValue(value)
                    }
                }
            }
            State::QuoteInUnquotedValue(mut value) => {
                if element == D::QUOTE {
                    value.push(element);
                    State::UnquotedValue(value)
                } else {
                    let mut position = self.position;
                    position.column_number -= 1;
                    return Err(ReadError::UnpairedQuote(position));
                }
            }
            State::QuotesPrefix(count) => {
                if element == D::QUOTE {
                    State::QuotesPrefix(count + 1)
                } else if count % 2 == 0 {
                    let next_state_after_quoted_value = if D::is_spacing_element(element) {
                        Some(State::Spacing(D::String::from_element(element)))
                    } else if element == D::LF {
                        Some(State::LfLineBreak)
                    } else if element == D::CR && next_element_is_lf!() {
                        Some(State::CrInLineBreak)
                    } else {
                        None
                    };

                    if let Some(next_state) = next_state_after_quoted_value {
                        let value = D::String::quotes((count - 2) / 2);
                        return Ok((next_state, Some(Token::QuotedValue(value))));
                    } else {
                        let mut value = D::String::quotes(count / 2);
                        value.push(element);
                        State::UnquotedValue(value)
                    }
                } else {
                    let mut value = D::String::quotes((count - 1) / 2);
                    value.push(element);
                    State::QuotedValue(value)
                }
            }
            State::QuotedValue(mut value) => {
                if element == D::QUOTE {
                    State::QuoteInQuotedValue(value)
                } else {
                    value.push(element);
                    State::QuotedValue(value)
                }
            }
            State::QuoteInQuotedValue(mut value) => {
                if element == D::QUOTE {
                    value.push(element);
                    State::QuotedValue(value)
                } else {
                    let next_state_after_quoted_value = if D::is_spacing_element(element) {
                        Some(State::Spacing(D::String::from_element(element)))
                    } else if element == D::LF {
                        Some(State::LfLineBreak)
                    } else if element == D::CR && next_element_is_lf!() {
                        Some(State::CrInLineBreak)
                    } else {
                        None
                    };

                    if let Some(next_state) = next_state_after_quoted_value {
                        return Ok((next_state, Some(Token::QuotedValue(value))));
                    } else {
                        let mut position = self.position;
                        position.column_number -= 1;
                        return Err(ReadError::UnpairedQuote(position));
                    }
                }
            }
            State::Spacing(mut spacing) => {
                if D::is_spacing_element(element) {
                    spacing.push(element);
                    State::Spacing(spacing)
                } else {
                    let next_state = if element == D::QUOTE {
                        State::QuotesPrefix(1)
                    } else if element == D::LF {
                        State::LfLineBreak
                    } else if element == D::CR && next_element_is_lf!() {
                        State::CrInLineBreak
                    } else {
                        State::UnquotedValue(D::String::from_element(element))
                    };
                    return Ok((next_state, Some(Token::Spacing(spacing))));
                }
            }
            State::LfLineBreak => {
                let next_state = if D::is_spacing_element(element) {
                    State::Spacing(D::String::from_element(element))
                } else if element == D::QUOTE {
                    State::QuotesPrefix(1)
                } else if element == D::LF {
                    State::LfLineBreak
                } else if element == D::CR && next_element_is_lf!() {
                    State::CrInLineBreak
                } else if element == D::HASH {
                    State::Comment(D::String::new())
                } else {
                    State::UnquotedValue(D::String::from_element(element))
                };
                return Ok((next_state, Some(Token::LineBreak(LineBreak::Lf))));
            }
            State::CrInLineBreak => {
                assert_eq!(element, D::LF);
                return Ok((State::Begin, Some(Token::LineBreak(LineBreak::CrLf))));
            }
            State::Comment(mut comment) => {
                let next_line_break_state = if element == D::LF {
                    Some(State::LfLineBreak)
                } else if element == D::CR && next_element_is_lf!() {
                    Some(State::CrInLineBreak)
                } else {
                    None
                };

                if let Some(next_state) = next_line_break_state {
                    return Ok((next_state, Some(Token::Comment(comment))));
                } else {
                    comment.push(element);
                    State::Comment(comment)
                }
            }
        };

        Ok((next_state, None))
    }

    fn finish(&mut self, state: State<D>) -> ReadResult<Option<Token<D>>> {
        match state {
            State::Begin => Ok(None),
            State::UnquotedValue(value) => Ok(Some(Token::UnquotedValue(value))),
            State::QuoteInUnquotedValue(_) => Err(ReadError::UnpairedQuote(self.position)),
            State::QuotesPrefix(count) => {
                if count % 2 == 0 {
                    let value = D::String::quotes((count - 2) / 2);
                    Ok(Some(Token::QuotedValue(value)))
                } else {
                    let mut position = self.position;
                    position.column_number += 1;
                    Err(ReadError::UnclosedQuotedValue(position))
                }
            }
            State::QuotedValue(_) => {
                let mut position = self.position;
                position.column_number += 1;
                Err(ReadError::UnclosedQuotedValue(position))
            }
            State::QuoteInQuotedValue(value) => Ok(Some(Token::QuotedValue(value))),
            State::Spacing(spacing) => Ok(Some(Token::Spacing(spacing))),
            State::LfLineBreak => Ok(Some(Token::LineBreak(LineBreak::Lf))),
            State::CrInLineBreak => unreachable!(),
            State::Comment(comment) => Ok(Some(Token::Comment(comment))),
        }
    }

    fn include_current_token_position(
        &self,
        token: Option<Token<D>>,
    ) -> Option<ReadResult<WithPosition<Token<D>>>> {
        token.map(|token| {
            Ok(WithPosition {
                value: token,
                position: self.current_token_position,
            })
        })
    }
}

impl<D: Domain, R: Read> Iterator for Tokenizer<D, R> {
    type Item = ReadResult<WithPosition<Token<D>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = self.state.take()?;

        while let Some(result) = self.elements.next() {
            let element = match result {
                Ok(element) => element,
                Err(io_error) => {
                    self.state = None;
                    return Some(Err(ReadError::IoError(io_error)));
                }
            };

            self.position.column_number += 1;
            if state == State::Begin {
                self.current_token_position = self.position;
            }

            match self.process(element, state) {
                Ok((next_state, token)) => {
                    let token = self.include_current_token_position(token);

                    if token.is_some() && next_state != State::Begin {
                        self.current_token_position = self.position;
                    }

                    if element == D::LF {
                        self.position.line_number += 1;
                        self.position.column_number = 0;
                    }

                    if token.is_some() {
                        self.state = Some(next_state);
                        return token;
                    } else {
                        state = next_state;
                    }
                }
                Err(error) => {
                    self.state = None;
                    return Some(Err(error));
                }
            }
        }

        self.state = None;
        match self.finish(state) {
            Ok(token_option) => self.include_current_token_position(token_option),
            Err(error) => Some(Err(error)),
        }
    }
}

impl<D: Domain, R: Read> FusedIterator for Tokenizer<D, R> {}

#[derive(PartialEq, Eq)]
enum State<D: Domain> {
    Begin,
    UnquotedValue(D::String),
    QuoteInUnquotedValue(D::String),
    QuotesPrefix(usize),
    QuotedValue(D::String),
    QuoteInQuotedValue(D::String),
    Spacing(D::String),
    LfLineBreak,
    CrInLineBreak,
    Comment(D::String),
}

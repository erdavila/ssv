use std::io::Read;
use std::iter::Peekable;
use std::mem::take;

use crate::engine::domain::{Domain, DomainString};
use crate::engine::tokens::UnquotedValue;
use crate::engine::ReadError;

use super::position::{Position, WithPosition};
use super::tokens::Token;
use super::ReadResult;

pub struct Tokenizer<D: Domain, R: Read> {
    elements: Peekable<D::ElementIterator<R>>,
    state: State<D>,
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

    fn process_element(&mut self, element: D::Element) -> Option<ReadResult<Token<D>>> {
        macro_rules! next_element_is_lf {
            () => {
                if let Some(Ok(element)) = self.elements.peek() {
                    *element == D::LF
                } else {
                    false
                }
            };
        }

        match &mut self.state {
            State::Begin => {
                if D::is_spacing_element(element) {
                    todo!()
                } else if element == D::QUOTE {
                    self.state = State::QuotesPrefix(1);
                } else if element == D::LF {
                    todo!()
                } else if element == D::CR && next_element_is_lf!() {
                    todo!()
                } else if element == D::HASH {
                    todo!()
                } else {
                    let value = D::String::from_element(element);
                    self.state = State::UnquotedValue(value)
                }
            }
            State::UnquotedValue(value) => {
                if element == D::QUOTE {
                    let value = take(value);
                    self.state = State::QuoteInUnquotedValue(value);
                } else {
                    let next_state = if D::is_spacing_element(element) {
                        Some(State::Spacing(D::String::from_element(element)))
                    } else if element == D::LF {
                        Some(State::LfLineBreak)
                    } else if element == D::CR && next_element_is_lf!() {
                        Some(State::CrInLineBreak)
                    } else {
                        value.push(element);
                        None
                    };

                    if let Some(next_state) = next_state {
                        let value = take(value);
                        self.state = next_state;
                        return Some(Ok(Token::UnquotedValue(UnquotedValue(value))));
                    }
                }
            }
            State::QuoteInUnquotedValue(value) => {
                if element == D::QUOTE {
                    value.push(element);
                    let value = take(value);
                    self.state = State::UnquotedValue(value);
                } else {
                    let mut position = self.position;
                    position.column_number -= 1;
                    return Some(Err(ReadError::UnpairedQuote(position)));
                }
            }
            State::QuotesPrefix(count) => {
                if D::is_spacing_element(element) {
                    todo!()
                } else if element == D::QUOTE {
                    *count += 1;
                } else if element == D::LF {
                    todo!()
                } else if element == D::CR && next_element_is_lf!() {
                    todo!()
                } else {
                    if *count % 2 == 0 {
                        let mut value = D::String::quotes(*count / 2);
                        value.push(element);
                        self.state = State::UnquotedValue(value);
                    } else {
                        todo!()
                    }
                }
            }
            State::Spacing(_) => todo!(),
            State::LfLineBreak => todo!(),
            State::CrInLineBreak => todo!(),
        }

        None
    }

    fn finish(&mut self) -> Option<ReadResult<Token<D>>> {
        match &mut self.state {
            State::Begin => None,
            State::UnquotedValue(value) => {
                let value = take(value);
                self.state = State::Begin;
                Some(Ok(Token::UnquotedValue(UnquotedValue(value))))
            }
            State::QuoteInUnquotedValue(_) => Some(Err(ReadError::UnpairedQuote(self.position))),
            State::QuotesPrefix(_) => todo!(),
            State::Spacing(_) => todo!(),
            State::LfLineBreak => todo!(),
            State::CrInLineBreak => todo!(),
        }
    }

    fn include_current_token_position(
        &self,
        token: Option<ReadResult<Token<D>>>,
    ) -> Option<ReadResult<WithPosition<Token<D>>>> {
        token.map(|result| {
            result.map(|token| WithPosition {
                value: token,
                position: self.current_token_position,
            })
        })
    }
}

impl<D: Domain, R: Read> Iterator for Tokenizer<D, R> {
    type Item = ReadResult<WithPosition<Token<D>>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(result) = self.elements.next() {
            let element = match result {
                Ok(element) => element,
                Err(_) => todo!(),
            };

            self.position.column_number += 1;
            if self.state == State::Begin {
                self.current_token_position = self.position;
            }

            let to_return = self.process_element(element);
            let to_return = self.include_current_token_position(to_return);
            if let Some(Ok(_)) = to_return {
                if self.state != State::Begin {
                    self.current_token_position = self.position;
                }
            }

            if element == D::LF {
                self.position.line_number += 1;
                self.position.column_number = 0;
            }

            if to_return.is_some() {
                return to_return;
            }
        }

        let to_return = self.finish();
        self.include_current_token_position(to_return)
    }
}

#[derive(PartialEq, Eq)]
enum State<D: Domain> {
    Begin,
    UnquotedValue(D::String),
    QuoteInUnquotedValue(D::String),
    QuotesPrefix(usize),
    Spacing(D::String),
    LfLineBreak,
    CrInLineBreak,
}

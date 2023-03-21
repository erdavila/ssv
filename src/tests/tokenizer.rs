use std::ops::Deref;

use crate::engine::domain::{BytesDomain, CharsDomain};
use crate::engine::tokenizer::Tokenizer;
use crate::engine::tokens::Token;
use crate::engine::ReadError;
use crate::tests::support::{CR, CRLF, LF, Q, TAB};
use crate::{bformat, domain_format};

macro_rules! assert_tokenization {
    ($input:expr, $($arg:tt)*) => {
        assert_tokenization_domain!(BytesDomain, $input, $($arg)* );
        assert_tokenization_domain!(CharsDomain, $input, $($arg)* );
    };
}

macro_rules! assert_tokenization_domain {
    ($domain:ident, $input:expr, [ $( $assertion:ident ( $($arg:expr),* ) ),+ $(,)? ]) => {
        let input_bytes = bformat!($input);

        let mut tokenizer: Tokenizer<$domain, _> = Tokenizer::new(input_bytes.deref());

        $(
            _assert_tokenization_domain_assertion!($domain, $assertion, tokenizer, $($arg),*);
        )+
    };
}

macro_rules! _assert_tokenization_domain_assertion {
    ($domain:ident, unquoted_value, $tokenizer:ident, $expected_value:literal, $expected_line_number:literal, $expected_column_number:literal) => {
        let token = $tokenizer.next().unwrap().unwrap();
        if let Token::UnquotedValue(value) = token.value {
            assert_eq!(value.0, domain_format!($domain, $expected_value));
            assert_eq!(token.position.line_number, $expected_line_number);
            assert_eq!(token.position.column_number, $expected_column_number);
        } else {
            panic!("wrong token: {:?}", token.value);
        }
    };
    ($_domain:ident, unpaired_quote_error, $tokenizer:ident, $expected_line_number:literal, $expected_column_number:literal) => {
        let error = $tokenizer.next().unwrap().unwrap_err();
        if let ReadError::UnpairedQuote(position) = error {
            assert_eq!(position.line_number, $expected_line_number);
            assert_eq!(position.column_number, $expected_column_number);
        } else {
            panic!("wrong error: {:?}", error);
        }
    };
    ($_domain:ident, end, $tokenizer:ident, ) => {
        assert!($tokenizer.next().is_none());
    };
}

#[test]
fn empty_input() {
    assert_tokenization!("", [end()]);
}

#[test]
fn unquoted_value() {
    assert_tokenization!("abc", [unquoted_value("abc", 1, 1), end()]);
}

#[test]
fn unquoted_value_containing_quote() {
    assert_tokenization!("abc{Q}{Q}def", [unquoted_value("abc{Q}def", 1, 1), end()]);
}

#[test]
fn unquoted_value_containing_cr() {
    assert_tokenization!("abc{CR}def", [unquoted_value("abc{CR}def", 1, 1), end()]);
}

#[test]
fn unquoted_value_starting_with_quotes() {
    assert_tokenization!("{Q}{Q}abc", [unquoted_value("{Q}abc", 1, 1)]);
}

#[test]
fn unquoted_value_followed_by_spacing() {
    assert_tokenization!(
        "abc {TAB} ",
        [
            unquoted_value("abc", 1, 1),
            // TODO: make it work
            // spacing(" {TAB} ", 1, 4),
            // end(),
        ]
    );
}

#[test]
fn unquoted_value_followed_by_line_break() {
    assert_tokenization!(
        "abc{LF}",
        [
            unquoted_value("abc", 1, 1),
            // TODO: make it work
            // line_break(LineBreak::Lf, 1, 4),
            // end(),
        ]
    );
    assert_tokenization!(
        "abc{CRLF}",
        [
            unquoted_value("abc", 1, 1),
            // TODO: make it work
            // line_break(LineBreak::CrLf, 1, 4),
            // end(),
        ]
    );
}

#[test]
fn unquoted_value_with_unpaired_quote() {
    assert_tokenization!("abc{Q}", [unpaired_quote_error(1, 4)]);
    assert_tokenization!("abc{Q}def", [unpaired_quote_error(1, 4)]);
}

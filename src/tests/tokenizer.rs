use std::ops::Deref;

use crate::engine::domain::{BytesDomain, CharsDomain};
use crate::engine::tokenizer::{Token, Tokenizer};
use crate::engine::{LineBreak, ReadError};
use crate::tests::support::{CR, CRLF, LF, Q, TAB};
use crate::{bformat, domain_format};

macro_rules! assert_tokenization {
    ($input:expr, $($arg:tt)*) => {
        assert_tokenization_domain!(BytesDomain, $input, $($arg)* );
        assert_tokenization_domain!(CharsDomain, $input, $($arg)* );
    };
}

macro_rules! assert_tokenization_domain {
    ($domain:ident, $input:expr, [ $( $assertion:ident ( $($arg:expr),* ) ),* $(,)? ]) => {
        let input_bytes = bformat!($input);

        let mut tokenizer: Tokenizer<$domain, _> = Tokenizer::new(input_bytes.deref());

        $(
            _assert_tokenization_domain_assertion!($domain, $assertion, tokenizer, $($arg),*);
        )*

        assert!(tokenizer.next().is_none());
    };
}

macro_rules! _assert_tokenization_domain_assertion {
    ($domain:ident, unquoted_value, $($arg:tt),*) => {
        _assert_tokenization_domain_assertion_token!($domain, UnquotedValue, $($arg),*);
    };
    ($domain:ident, quoted_value, $($arg:tt),*) => {
        _assert_tokenization_domain_assertion_token!($domain, QuotedValue, $($arg),*);
    };
    ($domain:ident, spacing, $($arg:tt),*) => {
        _assert_tokenization_domain_assertion_token!($domain, Spacing, $($arg),*);
    };
    ($_domain:ident, line_break, $($arg:tt),*) => {
        _assert_tokenization_domain_assertion_token!(_no_domain_, LineBreak, $($arg),*);
    };
    ($domain:ident, comment, $($arg:tt),*) => {
        _assert_tokenization_domain_assertion_token!($domain, Comment, $($arg),*);
    };
    ($_domain:ident, unpaired_quote_error, $($arg:tt),*) => {
        _assert_tokenization_domain_assertion_error!(UnpairedQuote, $($arg),*);
    };
    ($_domain:ident, unclosed_quoted_value_error, $($arg:tt),*) => {
        _assert_tokenization_domain_assertion_error!(UnclosedQuotedValue, $($arg),*);
    };
}

macro_rules! _assert_tokenization_domain_assertion_token {
    ($domain:ident, $expected_token:ident, $tokenizer:ident, $expected_value:expr, $expected_line_number:literal, $expected_column_number:literal) => {
        let token = $tokenizer.next().unwrap().unwrap();
        if let Token::$expected_token(value) = token.value {
            assert_eq!(
                value,
                _assert_tokenization_domain_value!($domain, $expected_value)
            );
            assert_eq!(token.position.line_number, $expected_line_number);
            assert_eq!(token.position.column_number, $expected_column_number);
        } else {
            panic!("wrong token: {:?}", token.value);
        }
    };
}

macro_rules! _assert_tokenization_domain_value {
    (_no_domain_, $string:expr) => {
        $string
    };
    ($domain:ident, $string:literal) => {
        domain_format!($domain, $string)
    };
}

macro_rules! _assert_tokenization_domain_assertion_error {
    ($expected_error:ident, $tokenizer:ident, $expected_line_number:literal, $expected_column_number:literal) => {
        let error = $tokenizer.next().unwrap().unwrap_err();
        if let ReadError::$expected_error(position) = error {
            assert_eq!(position.line_number, $expected_line_number);
            assert_eq!(position.column_number, $expected_column_number);
        } else {
            panic!("wrong error: {:?}", error);
        }
    };
}

#[test]
fn empty_input() {
    assert_tokenization!("", []);
}

#[test]
fn unquoted_value() {
    assert_tokenization!("abc", [unquoted_value("abc", 1, 1)]);
}

#[test]
fn unquoted_value_containing_quote() {
    assert_tokenization!("abc{Q}{Q}def", [unquoted_value("abc{Q}def", 1, 1)]);
}

#[test]
fn unquoted_value_containing_cr() {
    assert_tokenization!("abc{CR}def", [unquoted_value("abc{CR}def", 1, 1)]);
}

#[test]
fn unquoted_value_starting_with_quotes() {
    assert_tokenization!("{Q}{Q}abc", [unquoted_value("{Q}abc", 1, 1)]);
}

#[test]
fn unquoted_value_followed_by_spacing() {
    assert_tokenization!(
        "abc {TAB} ",
        [unquoted_value("abc", 1, 1), spacing(" {TAB} ", 1, 4)]
    );
}

#[test]
fn unquoted_value_followed_by_line_break() {
    assert_tokenization!(
        "abc{LF}",
        [unquoted_value("abc", 1, 1), line_break(LineBreak::Lf, 1, 4)]
    );
    assert_tokenization!(
        "abc{CRLF}",
        [
            unquoted_value("abc", 1, 1),
            line_break(LineBreak::CrLf, 1, 4),
        ]
    );
}

#[test]
fn unquoted_value_with_unpaired_quote() {
    assert_tokenization!("abc{Q}", [unpaired_quote_error(1, 4)]);
    assert_tokenization!("abc{Q}def", [unpaired_quote_error(1, 4)]);
}

#[test]
fn quoted_value() {
    assert_tokenization!("{Q}abc{Q}", [quoted_value("abc", 1, 1)]);
}

#[test]
fn quoted_value_containing_quote() {
    assert_tokenization!("{Q}abc{Q}{Q}def{Q}", [quoted_value("abc{Q}def", 1, 1)]);
}

#[test]
fn quoted_value_containing_only_quotes() {
    assert_tokenization!("{Q}{Q}{Q}{Q}{Q}{Q}", [quoted_value("{Q}{Q}", 1, 1)]);
}

#[test]
fn quoted_value_containing_spacing() {
    assert_tokenization!("{Q}abc def{Q}", [quoted_value("abc def", 1, 1)]);
}

#[test]
fn quoted_value_containing_line_break() {
    assert_tokenization!("{Q}abc{LF}def{Q}", [quoted_value("abc{LF}def", 1, 1)]);
    assert_tokenization!("{Q}abc{CRLF}def{Q}", [quoted_value("abc{CRLF}def", 1, 1)]);
}

#[test]
fn quoted_value_starting_with_quotes() {
    assert_tokenization!("{Q}{Q}{Q}abc{Q}", [quoted_value("{Q}abc", 1, 1)]);
}

#[test]
fn quoted_value_starting_with_quotes_and_spacing() {
    assert_tokenization!(
        "{Q}{Q}{Q} {TAB} abc{Q}",
        [quoted_value("{Q} {TAB} abc", 1, 1)]
    );
}

#[test]
fn quoted_value_starting_with_quotes_and_line_break() {
    assert_tokenization!("{Q}{Q}{Q}{LF}abc{Q}", [quoted_value("{Q}{LF}abc", 1, 1)]);
    assert_tokenization!(
        "{Q}{Q}{Q}{CRLF}abc{Q}",
        [quoted_value("{Q}{CRLF}abc", 1, 1)]
    );
}

#[test]
fn quoted_value_followed_by_spacing() {
    assert_tokenization!(
        "{Q}abc{Q} {TAB} ",
        [quoted_value("abc", 1, 1), spacing(" {TAB} ", 1, 6)]
    );
}

#[test]
fn quoted_value_followed_by_line_break() {
    assert_tokenization!(
        "{Q}abc{Q}{LF}",
        [quoted_value("abc", 1, 1), line_break(LineBreak::Lf, 1, 6)]
    );
    assert_tokenization!(
        "{Q}abc{Q}{CRLF}",
        [quoted_value("abc", 1, 1), line_break(LineBreak::CrLf, 1, 6)]
    );
}

#[test]
fn quoted_value_containing_only_quotes_followed_by_spacing() {
    assert_tokenization!(
        "{Q}{Q}{Q}{Q} {TAB} ",
        [quoted_value("{Q}", 1, 1), spacing(" {TAB} ", 1, 5)]
    );
}

#[test]
fn quoted_value_containing_only_quotes_followed_by_line_break() {
    assert_tokenization!(
        "{Q}{Q}{Q}{Q}{LF}",
        [quoted_value("{Q}", 1, 1), line_break(LineBreak::Lf, 1, 5)]
    );
    assert_tokenization!(
        "{Q}{Q}{Q}{Q}{CRLF}",
        [quoted_value("{Q}", 1, 1), line_break(LineBreak::CrLf, 1, 5)]
    );
}

#[test]
fn unclosed_quoted_value() {
    assert_tokenization!("{Q}abc", [unclosed_quoted_value_error(1, 5)]);
    assert_tokenization!("{Q}{Q}{Q}", [unclosed_quoted_value_error(1, 4)]);
}

#[test]
fn quoted_value_with_unpaired_quote() {
    assert_tokenization!("{Q}abc{Q}def", [unpaired_quote_error(1, 5)]);
}

#[test]
fn spacing() {
    assert_tokenization!(" {TAB} ", [spacing(" {TAB} ", 1, 1)]);
}

#[test]
fn spacing_followed_by_unquoted_value() {
    assert_tokenization!(
        " {TAB} abc",
        [spacing(" {TAB} ", 1, 1), unquoted_value("abc", 1, 4)]
    );
}

#[test]
fn spacing_followed_by_quoted_value() {
    assert_tokenization!(
        " {TAB} {Q}abc{Q}",
        [spacing(" {TAB} ", 1, 1), quoted_value("abc", 1, 4)]
    );
}

#[test]
fn spacing_followed_by_line_break() {
    assert_tokenization!(
        " {TAB} {LF}",
        [spacing(" {TAB} ", 1, 1), line_break(LineBreak::Lf, 1, 4)]
    );
    assert_tokenization!(
        " {TAB} {CRLF}",
        [spacing(" {TAB} ", 1, 1), line_break(LineBreak::CrLf, 1, 4)]
    );
}

#[test]
fn line_break() {
    assert_tokenization!("{LF}", [line_break(LineBreak::Lf, 1, 1)]);
    assert_tokenization!("{CRLF}", [line_break(LineBreak::CrLf, 1, 1)]);
}

#[test]
fn line_break_followed_by_unquoted_value() {
    assert_tokenization!(
        "{LF}abc",
        [line_break(LineBreak::Lf, 1, 1), unquoted_value("abc", 2, 1)]
    );
    assert_tokenization!(
        "{CRLF}abc",
        [
            line_break(LineBreak::CrLf, 1, 1),
            unquoted_value("abc", 2, 1),
        ]
    );
}

#[test]
fn lf_line_break_following_a_value_and_followed_by_unquoted_value() {
    assert_tokenization!(
        "abc{LF}def",
        [
            unquoted_value("abc", 1, 1),
            line_break(LineBreak::Lf, 1, 4),
            unquoted_value("def", 2, 1),
        ]
    );
}

#[test]
fn lf_line_break_following_a_value_and_followed_by_quoted_value() {
    assert_tokenization!(
        "abc{LF}{Q}def{Q}",
        [
            unquoted_value("abc", 1, 1),
            line_break(LineBreak::Lf, 1, 4),
            quoted_value("def", 2, 1),
        ]
    );
}

#[test]
fn lf_line_break_following_a_value_and_followed_by_spacing() {
    assert_tokenization!(
        "abc{LF} {TAB} ",
        [
            unquoted_value("abc", 1, 1),
            line_break(LineBreak::Lf, 1, 4),
            spacing(" {TAB} ", 2, 1),
        ]
    );
}

#[test]
fn lf_line_break_following_a_value_and_followed_by_line_break() {
    assert_tokenization!(
        "abc{LF}{LF}",
        [
            unquoted_value("abc", 1, 1),
            line_break(LineBreak::Lf, 1, 4),
            line_break(LineBreak::Lf, 2, 1),
        ]
    );
    assert_tokenization!(
        "abc{LF}{CRLF}",
        [
            unquoted_value("abc", 1, 1),
            line_break(LineBreak::Lf, 1, 4),
            line_break(LineBreak::CrLf, 2, 1),
        ]
    );
}

#[test]
fn comment() {
    assert_tokenization!("#comment", [comment("comment", 1, 1)]);
}

#[test]
fn comment_containing_quote() {
    assert_tokenization!("#com{Q}ment", [comment("com{Q}ment", 1, 1)]);
}

#[test]
fn comment_containing_spacing() {
    assert_tokenization!("#com ment", [comment("com ment", 1, 1)]);
}

#[test]
fn comment_followed_by_line_break() {
    assert_tokenization!(
        "#comment{LF}",
        [comment("comment", 1, 1), line_break(LineBreak::Lf, 1, 9)]
    );
    assert_tokenization!(
        "#comment{CRLF}",
        [comment("comment", 1, 1), line_break(LineBreak::CrLf, 1, 9)]
    );
}

#[test]
fn comment_following_line_break_following_a_value() {
    assert_tokenization!(
        "abc{LF}#comment",
        [
            unquoted_value("abc", 1, 1),
            line_break(LineBreak::Lf, 1, 4),
            comment("comment", 2, 1),
        ]
    );
    assert_tokenization!(
        "abc{CRLF}#comment",
        [
            unquoted_value("abc", 1, 1),
            line_break(LineBreak::CrLf, 1, 4),
            comment("comment", 2, 1),
        ]
    );
}

#[test]
fn hash_after_spacing_is_not_comment() {
    assert_tokenization!(
        " {TAB} #not-a-comment",
        [
            spacing(" {TAB} ", 1, 1),
            unquoted_value("#not-a-comment", 1, 4)
        ]
    );
}

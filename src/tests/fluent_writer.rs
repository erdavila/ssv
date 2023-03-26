use crate::engine::domain::{BytesDomain, CharsDomain};
use crate::engine::fluent_writer::FluentWriter;
use crate::engine::{LineBreak, WriteError};
use crate::tests::support::{CRLF, LF, Q, TAB};
use crate::{bformat, domain_format};

macro_rules! assert_fluent_write {
    ($($param:tt),+ $(,)?) => {
        _assert_fluent_write_domain!(BytesDomain, $($param),+);
        _assert_fluent_write_domain!(CharsDomain, $($param),+);
    };
}

macro_rules! _assert_fluent_write_domain {
    ($domain:ident, [ $( $method:ident ( $($arg:expr),* ) ),* ], $expected_output:literal ) => {
        let mut destination = Vec::new();

        let fluent_writer: FluentWriter<$domain, _> = FluentWriter::new(&mut destination);
        fluent_writer
        $(
            . $method ( $( _domain_arg!($domain, $arg) ),* ) .unwrap()
        )*  .finish().unwrap();

        assert_eq!(destination, domain_format!(BytesDomain, $expected_output));
    };
}

macro_rules! _domain_arg {
    ($domain:ident, $arg:literal) => {
        &domain_format!($domain, $arg)
    };
    ($domain:ident, $value:expr) => {
        $value
    };
}

macro_rules! assert_fluent_write_error {
    ($($arg:tt)*) => {
        _assert_fluent_write_error_domain!(BytesDomain, $($arg)*);
        _assert_fluent_write_error_domain!(CharsDomain, $($arg)*);
    };
}

macro_rules! _assert_fluent_write_error_domain {
    ($domain:ident, [ $( $method:ident ( $($arg:literal),* ) )* ], $error_method:ident ( $($error_arg:literal)* ) , $expected_error:ident $(,)? ) => {
        let mut destination = Vec::new();

        let fluent_writer: FluentWriter<$domain, _> = FluentWriter::new(&mut destination);
        let error = fluent_writer
        $(
            . $method ( $( &domain_format!($domain, $arg) ),* ) .unwrap()
        )*
            . $error_method ( $( &domain_format!($domain, $error_arg) )* ) .unwrap_err();

        if let WriteError::$expected_error = error {

        } else {
            panic!("wrong error: {:?}", error);
        }
    };
}

#[test]
fn empty() {
    assert_fluent_write!([], "");
}

#[test]
fn write_value() {
    assert_fluent_write!([write_value("abc")], "abc");
}

#[test]
fn write_quoted_value() {
    assert_fluent_write!([write_quoted_value("abc")], "{Q}abc{Q}");
}

#[test]
fn write_value_duplicates_quote() {
    assert_fluent_write!([write_value("abc{Q}def")], "abc{Q}{Q}def");

    assert_fluent_write!([write_quoted_value("abc{Q}def")], "{Q}abc{Q}{Q}def{Q}");
}

#[test]
fn write_value_automatically_quotes_values_containing_only_quotes() {
    assert_fluent_write!([write_value("{Q}{Q}")], "{Q}{Q}{Q}{Q}{Q}{Q}");
}

#[test]
fn write_value_automatically_quotes_value_with_spacing() {
    assert_fluent_write!([write_value("abc def")], "{Q}abc def{Q}");
}

#[test]
fn write_value_automatically_quotes_value_with_line_break() {
    assert_fluent_write!([write_value("abc{LF}def")], "{Q}abc{LF}def{Q}");

    assert_fluent_write!([write_value("abc{CRLF}def")], "{Q}abc{CRLF}def{Q}");
}

#[test]
fn write_spacing() {
    assert_fluent_write!([write_spacing(" {TAB} ")], " {TAB} ");
}

#[test]
fn write_spacing_invalid() {
    assert_fluent_write_error!([], write_spacing("abc"), InvalidSpacing);
}

#[test]
fn write_line_break() {
    assert_fluent_write!([write_line_break()], "{LF}");
}

#[test]
fn write_this_line_break() {
    assert_fluent_write!([write_this_line_break(LineBreak::Lf)], "{LF}");
    assert_fluent_write!([write_this_line_break(LineBreak::CrLf)], "{CRLF}");
}

#[test]
fn write_comment() {
    assert_fluent_write!([write_comment("comment")], "#comment");
}

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
    ($domain:ident, [ $( $method:ident ( $($arg:expr),* ) ),* $(,)? ], $expected_output:literal ) => {
        let mut destination = Vec::new();

        let fluent_writer: FluentWriter<$domain, _> = FluentWriter::new(&mut destination);
        $(
            let fluent_writer = _assert_fluent_write_unwrap!($method, fluent_writer.$method ( $( _domain_arg!($domain, $method, $arg) ),* ));
        )*
        fluent_writer.finish().unwrap();

        assert_eq!(destination, domain_format!(BytesDomain, $expected_output));
    };
}

macro_rules! _domain_arg {
    ($_:ident, true) => {
        true
    };
    ($_:ident, false) => {
        false
    };
    ($domain:ident, set_default_spacing, $arg:literal) => {
        domain_format!($domain, $arg)
    };
    ($domain:ident, $_method:ident, $arg:literal) => {
        &domain_format!($domain, $arg)
    };
    ($_domain:ident, $_method:ident, $value:expr) => {
        $value
    };
}

macro_rules! _assert_fluent_write_unwrap {
    (set_default_line_break, $e:expr) => {
        $e
    };
    (set_always_quoted, $e:expr) => {
        $e
    };
    ($_:ident, $e:expr) => {
        $e.unwrap()
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

#[test]
fn value_followed_by_value_has_auto_spacing_in_between() {
    assert_fluent_write!([write_value("abc"), write_value("def")], "abc def");
}

#[test]
fn value_followed_by_spacing() {
    assert_fluent_write!([write_value("abc"), write_spacing(" {TAB} ")], "abc {TAB} ");
}

#[test]
fn value_followed_by_line_break() {
    assert_fluent_write!([write_value("abc"), write_line_break()], "abc{LF}");
}

#[test]
fn value_followed_by_comment_has_auto_line_break_in_between() {
    assert_fluent_write!(
        [write_value("abc"), write_comment("comment")],
        "abc{LF}#comment",
    );
}

#[test]
fn spacing_followed_by_value() {
    assert_fluent_write!([write_spacing(" {TAB} "), write_value("abc")], " {TAB} abc");
}

#[test]
fn spacing_followed_by_spacing() {
    assert_fluent_write!(
        [write_spacing("  "), write_spacing("{TAB}{TAB}")],
        "  {TAB}{TAB}",
    );
}

#[test]
fn spacing_followed_by_line_break() {
    assert_fluent_write!(
        [write_spacing(" {TAB} "), write_line_break()],
        " {TAB} {LF}",
    );
}

#[test]
fn spacing_followed_by_comment_has_auto_line_break_in_between() {
    assert_fluent_write!(
        [write_spacing(" "), write_comment("comment")],
        " {LF}#comment",
    );
}

#[test]
fn line_break_followed_by_value() {
    assert_fluent_write!([write_line_break(), write_value("abc")], "{LF}abc");
}

#[test]
fn line_break_followed_by_spacing() {
    assert_fluent_write!(
        [write_line_break(), write_spacing(" {TAB} ")],
        "{LF} {TAB} ",
    );
}

#[test]
fn line_break_followed_by_line_break() {
    assert_fluent_write!([write_line_break(), write_line_break()], "{LF}{LF}");
}

#[test]
fn line_break_followed_by_comment() {
    assert_fluent_write!(
        [write_line_break(), write_comment("comment")],
        "{LF}#comment",
    );
}

#[test]
fn comment_followed_by_value_has_auto_line_break_in_between() {
    assert_fluent_write!(
        [write_comment("comment"), write_value("abc")],
        "#comment{LF}abc",
    );
}

#[test]
fn comment_followed_by_spacing_has_auto_line_break_in_between() {
    assert_fluent_write!(
        [write_comment("comment"), write_spacing(" ")],
        "#comment{LF} ",
    );
}

#[test]
fn comment_followed_by_line_break() {
    assert_fluent_write!(
        [write_comment("comment"), write_line_break()],
        "#comment{LF}",
    );
}

#[test]
fn comment_followed_by_comment_has_auto_line_break_in_between() {
    assert_fluent_write!(
        [write_comment("comment-1"), write_comment("comment-2")],
        "#comment-1{LF}#comment-2",
    );
}

#[test]
fn some_data() {
    assert_fluent_write!(
        [
            write_value("abc"),
            write_value("def ghi"),
            write_line_break(),
            write_value("123 456"),
            write_value("789"),
        ],
        "abc {Q}def ghi{Q}{LF}{Q}123 456{Q} 789",
    );
}
#[test]
fn value_automatically_quoted_to_disambiguate_against_comment() {
    assert_fluent_write!([write_value("#not-a-comment")], "{Q}#not-a-comment{Q}");

    assert_fluent_write!(
        [write_line_break(), write_value("#not-a-comment")],
        "{LF}{Q}#not-a-comment{Q}"
    );

    assert_fluent_write!(
        [write_comment("comment"), write_value("#not-a-comment")],
        "#comment{LF}{Q}#not-a-comment{Q}",
    );

    // Quoting is not needed in the following cases

    assert_fluent_write!(
        [write_value("abc"), write_value("#not-a-comment")],
        "abc #not-a-comment",
    );

    assert_fluent_write!(
        [write_spacing(" "), write_value("#not-a-comment")],
        " #not-a-comment",
    );

    assert_fluent_write!(
        [write_value("first-value-in-the-line")],
        "first-value-in-the-line",
    );

    assert_fluent_write!(
        [write_line_break(), write_value("first-value-in-the-line")],
        "{LF}first-value-in-the-line",
    );
}

#[test]
fn set_default_spacing() {
    macro_rules! test_domain {
        ($domain:ident) => {
            let fluent_writer: FluentWriter<$domain, _> = FluentWriter::new(Vec::new());

            let spacing = domain_format!($domain, " ");
            assert_eq!(fluent_writer.default_spacing(), spacing);
            assert_eq!(fluent_writer.options().default_spacing(), spacing);

            let spacing = domain_format!($domain, " {TAB} ");
            let mut fluent_writer = fluent_writer.set_default_spacing(spacing.clone()).unwrap();
            assert_eq!(fluent_writer.default_spacing(), spacing);
            assert_eq!(fluent_writer.options().default_spacing(), spacing);

            let spacing = domain_format!($domain, "{TAB}{TAB}");
            fluent_writer
                .options_mut()
                .set_default_spacing(spacing.clone())
                .unwrap();
            assert_eq!(fluent_writer.default_spacing(), spacing);
            assert_eq!(fluent_writer.options().default_spacing(), spacing);
        };
    }

    test_domain!(BytesDomain);
    test_domain!(CharsDomain);
}

#[test]
fn set_default_line_break() {
    macro_rules! test_domain {
        ($domain:ident) => {
            let fluent_writer: FluentWriter<$domain, _> = FluentWriter::new(Vec::new());

            assert_eq!(fluent_writer.default_line_break(), LineBreak::Lf);
            assert_eq!(fluent_writer.options().default_line_break(), LineBreak::Lf);

            let mut fluent_writer = fluent_writer.set_default_line_break(LineBreak::CrLf);
            assert_eq!(fluent_writer.default_line_break(), LineBreak::CrLf);
            assert_eq!(
                fluent_writer.options().default_line_break(),
                LineBreak::CrLf
            );

            fluent_writer
                .options_mut()
                .set_default_line_break(LineBreak::Lf);
            assert_eq!(fluent_writer.default_line_break(), LineBreak::Lf);
            assert_eq!(fluent_writer.options().default_line_break(), LineBreak::Lf);
        };
    }

    test_domain!(BytesDomain);
    test_domain!(CharsDomain);
}

#[test]
fn set_always_quoted() {
    macro_rules! test_domain {
        ($domain:ident) => {
            let fluent_writer: FluentWriter<$domain, _> = FluentWriter::new(Vec::new());

            assert_eq!(fluent_writer.always_quoted(), false);
            assert_eq!(fluent_writer.options().always_quoted(), false);

            let mut fluent_writer = fluent_writer.set_always_quoted(true);
            assert_eq!(fluent_writer.always_quoted(), true);
            assert_eq!(fluent_writer.options().always_quoted(), true);

            fluent_writer.options_mut().set_always_quoted(false);
            assert_eq!(fluent_writer.always_quoted(), false);
            assert_eq!(fluent_writer.options().always_quoted(), false);
        };
    }

    test_domain!(BytesDomain);
    test_domain!(CharsDomain);
}

#[test]
fn default_spacing() {
    assert_fluent_write!(
        [
            set_default_spacing(" {TAB} "),
            write_value("abc"),
            write_value("def"),
        ],
        "abc {TAB} def",
    );
}

#[test]
fn default_line_break() {
    assert_fluent_write!(
        [
            set_default_line_break(LineBreak::CrLf),
            write_value("abc"),
            write_comment("comment-1"),
            write_spacing(" {TAB} "),
            write_comment("comment-2"),
            write_comment("comment-3"),
            write_value("def"),
        ],
        "abc{CRLF}#comment-1{CRLF} {TAB} {CRLF}#comment-2{CRLF}#comment-3{CRLF}def",
    );
}

#[test]
fn always_quoted() {
    /*
       This code should work:

           assert_fluent_write!(
               [
                   set_always_quoted(true),
                   write_value("abc"),
               ],
               "{Q}abc{Q}",
           );

       For some reason, the compiler complains at the `true`, as if it had ignored the first rule in `_domain_arg`.

       The code below was obtained from the above by using the "Inline macro" command in VS Code, and it works.
    */

    let mut destination = Vec::new();
    let fluent_writer: FluentWriter<BytesDomain, _> = FluentWriter::new(&mut destination);
    fluent_writer
        .set_always_quoted(_domain_arg!(BytesDomain, true))
        .write_value(_domain_arg!(BytesDomain, write_value, "abc"))
        .unwrap()
        .finish()
        .unwrap();
    assert_eq!(destination, domain_format!(BytesDomain, "{Q}abc{Q}"));

    let mut destination = Vec::new();
    let fluent_writer: FluentWriter<CharsDomain, _> = FluentWriter::new(&mut destination);
    fluent_writer
        .set_always_quoted(_domain_arg!(CharsDomain, true))
        .write_value(_domain_arg!(CharsDomain, write_value, "abc"))
        .unwrap()
        .finish()
        .unwrap();
    assert_eq!(destination, domain_format!(BytesDomain, "{Q}abc{Q}"));
}

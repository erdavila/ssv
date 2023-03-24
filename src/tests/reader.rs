use std::ops::Deref;

use crate::engine::domain::{BytesDomain, CharsDomain, Domain};
use crate::engine::reader::Reader;
use crate::engine::ReadError;
use crate::tests::support::{CRLF, LF, Q};
use crate::{bformat, domain_format};

macro_rules! assert_reading {
    ($input:literal, $($arg:tt)*) => {
        let input = bformat!($input);
        _assert_reading_domain!(BytesDomain, input, $($arg)*);
        _assert_reading_domain!(CharsDomain, input, $($arg)*);
    };
}

macro_rules! _assert_reading_domain {
    ($domain:ident, $input:ident, $($arg:tt)*) => {
        let mut reader: Reader<$domain, _> = Reader::new($input.deref());

        let expected_rows: Vec<Vec<<$domain as Domain>::String>> = domain_format!($domain, $($arg)*);
        for expected_row in expected_rows {
            let row = reader.next().unwrap().unwrap();
            assert_eq!(row, expected_row);
        }
        assert!(reader.next().is_none());
    };
}

#[test]
fn empty_input() {
    assert_reading!("", []);
}

#[test]
fn some_input() {
    assert_reading!(
        "abc {Q}def{Q}{LF}# comment{LF}  {Q}123 456{Q}{LF}",
        [["abc", "def"], ["123 456"]],
    );
    assert_reading!(
        "abc {Q}def{Q}{CRLF}# comment{CRLF}  {Q}123 456{Q}{CRLF}",
        [["abc", "def"], ["123 456"]],
    );
}

#[test]
fn empty_line() {
    assert_reading!("{LF}abc{LF}", [[], ["abc"]]);
    assert_reading!("{CRLF}abc{CRLF}", [[], ["abc"]]);
}

#[test]
fn last_line_without_line_break() {
    assert_reading!("abc{LF}def", [["abc"], ["def"]]);
    assert_reading!("abc{CRLF}def", [["abc"], ["def"]]);
}

#[test]
fn last_line_is_comment_without_line_break() {
    assert_reading!("abc{LF}# comment", [["abc"]]);
    assert_reading!("abc{CRLF}# comment", [["abc"]]);
}

#[test]
fn error() {
    let input: &[u8] = b"a\nb\xFF cdef"; // Invalid UTF-8

    let reader: Reader<BytesDomain, _> = Reader::new(input);
    for row in reader {
        row.unwrap();
    }

    let reader: Reader<CharsDomain, _> = Reader::new(input);
    let mut remaining = reader.skip_while(|result| result.is_ok());

    let error = remaining.next().unwrap().unwrap_err();
    if let ReadError::IoError(io_error) = error {
        assert_eq!(io_error.kind(), std::io::ErrorKind::InvalidData);
    } else {
        panic!("wrong error: {:?}", error);
    }

    assert!(remaining.next().is_none());
}

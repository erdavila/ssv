use crate::engine::domain::CharsDomain;
use crate::engine::domain::{BytesDomain, DomainStringSlice};
use crate::engine::reader::Reader;
use crate::engine::ReadResult;
use crate::tests::support::LF;
use crate::{bformat, domain_format, domain_format_ref};

mod fluent_writer;
mod reader;
mod tokenizer;
mod writer;

pub mod support;

#[test]
fn read() {
    macro_rules! test_with_domain {
        ($domain:ident) => {
            let input = bformat!("abc def{LF}123 456{LF}");

            let result: Reader<$domain, _> = super::engine::read(input.as_bytes());

            let rows: ReadResult<Vec<_>> = result.collect();
            assert_eq!(
                rows.unwrap(),
                domain_format!($domain, [["abc", "def"], ["123", "456"]]),
            );
        };
    }

    test_with_domain!(BytesDomain);
    test_with_domain!(CharsDomain);
}

#[test]
fn write() {
    macro_rules! test_with_domain {
        ($domain:ident) => {
            let mut destination: Vec<u8> = Vec::new();

            super::engine::write::<$domain, _>(
                &mut destination,
                domain_format_ref!($domain, [["abc", "def"], ["123", "456"]]),
            )
            .unwrap();

            assert_eq!(destination, bformat!("abc def{LF}123 456{LF}"));
        };
    }

    test_with_domain!(BytesDomain);
    test_with_domain!(CharsDomain);
}

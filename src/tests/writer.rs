use crate::engine::domain::{BytesDomain, CharsDomain, Domain};
use crate::engine::writer::Writer;
use crate::tests::support::{LF, Q, TAB};
use crate::{bformat, domain_format, domain_format_ref};

fn assert_writer<D, F>(f: F, expected: Vec<u8>)
where
    D: Domain,
    F: FnOnce(&mut Writer<D, &mut Vec<u8>>),
{
    let mut destination = Vec::new();

    let mut writer: Writer<D, _> = Writer::new(&mut destination);
    f(&mut writer);
    writer.finish().unwrap();

    assert_eq!(destination, expected);
}

#[test]
fn write_rows() {
    macro_rules! test_domain {
        ($domain:ident) => {
            assert_writer::<$domain, _>(
                |writer| {
                    writer
                        .write_rows(domain_format_ref!(
                            $domain,
                            [["abc", "def ghi"], ["#not-a-comment", "123 456", "789"]],
                        ))
                        .unwrap();
                },
                bformat!("abc {Q}def ghi{Q}{LF}{Q}#not-a-comment{Q} {Q}123 456{Q} 789{LF}"),
            );
        };
    }

    test_domain!(BytesDomain);
    test_domain!(CharsDomain);
}

#[test]
fn write_row_and_write_comment_line() {
    macro_rules! test_domain {
        ($domain:ident) => {
            assert_writer::<$domain, _>(
                |writer| {
                    writer
                        .write_row(domain_format_ref!($domain, ["abc", "def ghi"]))
                        .unwrap();
                    writer
                        .write_comment_line(domain_format_ref!($domain, "this is a comment"))
                        .unwrap();
                    writer
                        .write_row(domain_format_ref!(
                            $domain,
                            ["#not-a-comment", "123 456", "789"],
                        ))
                        .unwrap();
                },
                bformat!("abc {Q}def ghi{Q}{LF}#this is a comment{LF}{Q}#not-a-comment{Q} {Q}123 456{Q} 789{LF}"),
            );
        };
    }

    test_domain!(BytesDomain);
    test_domain!(CharsDomain);
}

#[test]
fn row_writer_write_values() {
    macro_rules! test_domain {
        ($domain:ident) => {
            assert_writer::<$domain, _>(
                |writer| {
                    let mut row = writer.new_row();
                    row.write_values(domain_format_ref!($domain, ["abc", "def ghi"]))
                        .unwrap();
                    row.finish().unwrap();

                    let mut row = writer.new_row();
                    row.write_values(domain_format_ref!($domain, ["#not-a-comment", "123 456"]))
                        .unwrap();
                    row.write_values(domain_format_ref!($domain, ["789"]))
                        .unwrap();
                    row.finish().unwrap();
                },
                bformat!("abc {Q}def ghi{Q}{LF}{Q}#not-a-comment{Q} {Q}123 456{Q} 789{LF}"),
            );
        };
    }

    test_domain!(BytesDomain);
    test_domain!(CharsDomain);
}

#[test]
fn row_writer_write_value() {
    macro_rules! test_domain {
        ($domain:ident) => {
            assert_writer::<$domain, _>(
                |writer| {
                    let mut row = writer.new_row();
                    row.write_value(domain_format_ref!($domain, "abc")).unwrap();
                    row.write_value(domain_format_ref!($domain, "def ghi"))
                        .unwrap();
                    row.finish().unwrap();

                    let mut row = writer.new_row();
                    row.write_value(domain_format_ref!($domain, "#not-a-comment"))
                        .unwrap();
                    row.write_value(domain_format_ref!($domain, "123 456"))
                        .unwrap();
                    row.write_value(domain_format_ref!($domain, "789")).unwrap();
                    row.finish().unwrap();
                },
                bformat!("abc {Q}def ghi{Q}{LF}{Q}#not-a-comment{Q} {Q}123 456{Q} 789{LF}"),
            );
        };
    }

    test_domain!(BytesDomain);
    test_domain!(CharsDomain);
}

#[test]
fn row_writer_write_spacing() {
    macro_rules! test_domain {
        ($domain:ident) => {
            assert_writer::<$domain, _>(
                |writer| {
                    let mut row = writer.new_row();
                    row.write_value(domain_format_ref!($domain, "abc")).unwrap();
                    row.write_spacing(domain_format_ref!($domain, " {TAB} "))
                        .unwrap();
                    row.write_value(domain_format_ref!($domain, "def ghi"))
                        .unwrap();
                    row.finish().unwrap();

                    let mut row = writer.new_row();
                    row.write_spacing(domain_format_ref!($domain, "   "))
                        .unwrap();
                    row.write_value(domain_format_ref!($domain, "#not-a-comment"))
                        .unwrap();
                    row.write_value(domain_format_ref!($domain, "123 456"))
                        .unwrap();
                    row.write_value(domain_format_ref!($domain, "789")).unwrap();
                    row.write_spacing(domain_format_ref!($domain, "{TAB} {TAB}"))
                        .unwrap();
                    row.finish().unwrap();
                },
                bformat!(
                    "abc {TAB} {Q}def ghi{Q}{LF}   #not-a-comment {Q}123 456{Q} 789{TAB} {TAB}{LF}"
                ),
            );
        };
    }

    test_domain!(BytesDomain);
    test_domain!(CharsDomain);
}

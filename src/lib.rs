#![warn(missing_docs)]
#![doc(test(attr(deny(warnings))))]

//! SSV means Space-Separated Values, and is an alternative to CSV.
//!
//! It is meant to be a cleaner format for human-writen data without the hassles
//! of numbers containing commas, particularly on languages that use comma as
//! decimal separator.
//!
//! # Rules
//!
//! * Values are separated by a sequence of at least one spacing element.
//!   * A spacing element is either a SPACE (byte value/codepoint 32) or a TAB
//! (byte value/codepoint 9).
//!   * The first value in a row can be preceded by spacing, which is ignored.
//!   * The last value in a row can be succeeded by spacing, which is ignored.
//! * Rows of values are separated by line-breaks. A line-break is an LF (byte
//! value/codepoint 10) optionally preceded by CR (byte value/codepoint 13).
//! * Values may be enclosed in quotes (`"`).
//! * Values *must* be enclosed in quotes in the following cases:
//!   * the value is empty;
//!   * the value contains any spacing element;
//!   * the value contains a line-break;
//!   * the value contains only quotes;
//!   * the value is the first thing in a row and starts with a HASH sign (`#`).
//! * Values containing quotes are encoded by duplicating the quotes.
//! * A line starting with the HASH sign (`#`) is ignored until the next
//! line-break (or end of the content). Such line is considered a comment line.
//!
//!
//! ## Example
//! The following content:
//!
//! | #  | Name     | Age | Note         |
//! |---:|----------|----:|--------------|
//! |  1 | John Doe |  53 | a.k.a. "Joe" |
//! | 77 | Mary     |  23 |              |
//!
//! can be encoded as:
//! ```ssv
//! "#"  Name        Age  Note
//!  1   "John Doe"   53  "a.k.a. ""Joe"""
//! 77   Mary         23  ""
//! ```
//!
//! # Bytes and Chars - modules, imports
//!
//! This SSV lib has a single generic implementation that is specialized for the
//! "domains":
//!
//! | domain | Element  | String      | StringSlice |
//! |--------|----------|-------------|-------------|
//! | bytes  | [`u8`]   | [`Vec<u8>`] | `&[u8]`     |
//! | chars  | [`char`] | [`String`]  | `&str`      |
//!
//! The generic implementation is in the [`engine`] module.
//! The modules [`bytes`] and [`chars`] have specializations that are aliases
//! for types in the [`engine`] module. Code using this crate should not have
//! references to the [`engine`] module, only to the specializations modules.
//!
//! # Reading SSV
//!
//! Given a byte reader (a value implementing the [`std::io::Read`] trait),
//! SSV can be read with:
//!
//! * [`Tokenizer`](crate::engine::tokenizer::Tokenizer) - an iterator that
//! validates and returns tokens, including spacing, line-breaks and comments.
//! * [`Reader`](crate::engine::reader::Reader) - an iterator that returns rows.
//! Each row is a [`Vec`](std::vec::Vec) of values.
//! * [`read`](crate::engine::read) - a utility function that creates a
//! [`Reader`](crate::engine::reader::Reader) object.
//!
//! There is also the [`read_file`](crate::engine::read_file) function that reads
//! from a file given its path.
//!
//! # Writing SSV
//!
//! Given a byte writer (a value implementing the [`std::io::Write`] trait),
//! SSV can be written with:
//!
//! * [`FluentWriter`](crate::engine::fluent_writer::FluentWriter) - an object
//! that writes items with a fluent interface. Delimiters such as spacing and
//! line-breaks are automatically written when required.
//! * [`Writer`](crate::engine::writer::Writer) - an object that writes in a
//! row-oriented way.
//! * [`write`](crate::engine::write) - a utility function that uses a
//! [`Writer`](crate::engine::writer::Writer) object to write SSV content.
//!
//! There is also the [`write_file`](crate::engine::write_file) function that
//! writes to a file given its path.

macro_rules! generic_item_warning_doc {
    ($item_name:literal) => {
        concat!(
            "

 <div style=\"color: black; background-color:yellow\">
 ⚠️ This doc is about the generic implementation, but it applies to the
 domain-specific aliases too. You should use the domain-specific aliases only:
 </div>

 * [`crate::bytes::",
            $item_name,
            "`]
 * [`crate::chars::",
            $item_name,
            "`]
 ---

"
        )
    };
}

macro_rules! generic_item_link_doc {
    ($item_name:literal) => {
        concat!(
            "

See the doc for the generic item [`crate::engine::",
            $item_name,
            "`] for more info."
        )
    };
}

macro_rules! generic_item_delegation_doc {
    ($item_name:literal) => {
        concat!(
            "

It simply delegates to the generic item [`crate::engine::",
            $item_name,
            "`]. See its doc for more info."
        )
    };
}

pub mod engine;

macro_rules! domain_module {
    ($name:ident, $domain:ty, $doc:literal) => {
        #[doc = $doc]
        pub mod $name {
            use std::fs::File;
            use std::io::Read;
            use std::io::Write;
            use std::path::Path;

            use crate::engine::domain::Domain;

            pub use crate::engine::LineBreak;

            pub use crate::engine::position::Position;
            pub use crate::engine::position::WithPosition;
            pub use crate::engine::ReadError;
            pub use crate::engine::ReadResult;

            /// Reads SSV tokens from a value that implements the [`Read`] trait.
            #[doc = generic_item_link_doc!("tokenizer::Tokenizer")]
            pub type Tokenizer<R> = super::engine::tokenizer::Tokenizer<$domain, R>;

            /// An SSV token.
            #[doc = generic_item_link_doc!("tokenizer::Token")]
            pub type Token = super::engine::tokenizer::Token<$domain>;

            /// Reads SSV rows from a value that implements the [`Read`] trait.
            #[doc = generic_item_link_doc!("reader::Reader")]
            pub type Reader<R> = super::engine::reader::Reader<$domain, R>;

            /// Reads SSV from a file.
            #[doc = generic_item_delegation_doc!("read_file")]
            #[inline]
            pub fn read_file<P: AsRef<Path>>(path: P) -> ReadResult<Reader<File>> {
                crate::engine::read_file(path)
            }

            /// Reads SSV from a reader.
            #[doc = generic_item_delegation_doc!("read")]
            #[inline]
            pub fn read<R: Read>(reader: R) -> Reader<R> {
                crate::engine::read(reader)
            }

            pub use crate::engine::WriteError;
            pub use crate::engine::WriteResult;

            /// The options for writing SSV content.
            #[doc = generic_item_link_doc!("options::Options")]
            pub type Options = super::engine::options::Options<$domain>;

            /// Has a fluent interface to write SSV to a value that implements the [`Write`]
            #[doc = generic_item_link_doc!("fluent_writer::FluentWriter")]
            pub type FluentWriter<W> = super::engine::fluent_writer::FluentWriter<$domain, W>;

            /// Follows a row-oriented structure to write SSV to a value that implements
            /// the [`Write`] trait.
            #[doc = generic_item_link_doc!("writer::Writer")]
            pub type Writer<W> = super::engine::writer::Writer<$domain, W>;

            /// An object that writes values in the context of a row.
            #[doc = generic_item_link_doc!("writer::RowWriter")]
            pub type RowWriter<'a, W> = super::engine::writer::RowWriter<'a, $domain, W>;

            /// Writes SSV to a file.
            #[doc = generic_item_delegation_doc!("write_file")]
            #[inline]
            pub fn write_file<'a, P: AsRef<Path>>(
                path: P,
                rows: impl IntoIterator<
                    Item = impl IntoIterator<Item = &'a <$domain as Domain>::StringSlice>,
                >,
            ) -> WriteResult<()>
            where
                <$domain as Domain>::StringSlice: 'a,
            {
                crate::engine::write_file::<'a, $domain, P>(path, rows)
            }

            /// Writes SSV to a writer.
            #[doc = generic_item_delegation_doc!("write")]
            #[inline]
            pub fn write<'a, W: Write>(
                writer: W,
                rows: impl IntoIterator<
                    Item = impl IntoIterator<Item = &'a <$domain as Domain>::StringSlice>,
                >,
            ) -> WriteResult<()>
            where
                <$domain as Domain>::StringSlice: 'a,
            {
                crate::engine::write::<$domain, W>(writer, rows)
            }
        }
    };
}

domain_module!(
    chars,
    super::engine::domain::CharsDomain,
    "Types and functions to operate on `char`/`String`/`&str`."
);
domain_module!(
    bytes,
    super::engine::domain::BytesDomain,
    "Types and functions to operate on `u8`/`Vec<u8>`/`&[u8]`."
);

#[cfg(test)]
mod tests;

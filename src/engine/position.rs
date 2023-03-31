//! Position of tokens.

/// The position of a token.
#[doc = generic_item_warning_doc!("Position")]
#[derive(Clone, Copy, Debug)]
pub struct Position {
    /// The line number where the token is found, starting from 1.
    pub line_number: usize,

    /// The column number where the token is found, starting from 1.
    ///
    /// Columns may be counted in bytes or chars, depending on the domain.
    pub column_number: usize,
}

/// A value associated to a [`Position`].
#[doc = generic_item_warning_doc!("WithPosition")]
#[derive(Debug)]
pub struct WithPosition<T> {
    /// The value.
    pub value: T,

    /// The [`Position`].
    pub position: Position,
}

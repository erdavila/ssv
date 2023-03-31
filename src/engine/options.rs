//! Options for writing SSV.

use std::ops::Deref;

use crate::engine::domain::{Domain, DomainString};
use crate::engine::LineBreak;

use super::{WriteError, WriteResult};

/// The options for writing SSV content.
#[doc = generic_item_warning_doc!("Options")]
/// See the docs for [`FluentWriter`](crate::engine::fluent_writer::FluentWriter)
/// and [`Writer`](crate::engine::writer::Writer) on how they use the options.
#[derive(Clone, Copy, Debug)]
pub struct Options<D: Domain> {
    default_spacing: D::String,
    default_line_break: LineBreak,
    always_quoted: bool,
}

impl<D: Domain> Options<D> {
    /// Creates a new instance with default values.
    pub fn new() -> Self {
        Options {
            default_spacing: D::String::from_element(D::SPACE),
            default_line_break: LineBreak::Lf,
            always_quoted: false,
        }
    }

    /// Returns the default spacing.
    pub fn default_spacing(&self) -> &D::StringSlice {
        &self.default_spacing
    }

    /// Sets the default spacing.
    pub fn set_default_spacing(&mut self, spacing: D::String) -> WriteResult<()> {
        if !D::is_valid_spacing(spacing.deref()) {
            return Err(WriteError::InvalidSpacing);
        }

        self.default_spacing = spacing;
        Ok(())
    }

    /// Returns the default line-break.
    pub fn default_line_break(&self) -> LineBreak {
        self.default_line_break
    }

    /// Sets the default line-break.
    pub fn set_default_line_break(&mut self, line_break: LineBreak) {
        self.default_line_break = line_break;
    }

    /// Returns whether the values are always quoted.
    pub fn always_quoted(&self) -> bool {
        self.always_quoted
    }

    /// Sets whether the values are always quoted.
    pub fn set_always_quoted(&mut self, always_quoted: bool) {
        self.always_quoted = always_quoted;
    }
}

impl<D: Domain> Default for Options<D> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::bformat;
    use crate::engine::domain::{BytesDomain, CharsDomain};
    use crate::engine::{LineBreak, WriteError};
    use crate::tests::support::TAB;

    use super::Options;

    #[test]
    fn initial_values() {
        let options: Options<BytesDomain> = Options::new();
        assert_eq!(options.default_spacing(), b" ");
        assert_eq!(options.default_line_break(), LineBreak::Lf);
        assert_eq!(options.always_quoted(), false);

        let options: Options<CharsDomain> = Options::new();
        assert_eq!(options.default_spacing(), " ");
        assert_eq!(options.default_line_break(), LineBreak::Lf);
        assert_eq!(options.always_quoted(), false);
    }

    #[test]
    fn set_default_spacing() {
        let mut options: Options<BytesDomain> = Options::new();
        options.set_default_spacing(bformat!(" {TAB} ")).unwrap();
        assert_eq!(options.default_spacing(), bformat!(" {TAB} "));

        let mut options: Options<CharsDomain> = Options::new();
        options.set_default_spacing(format!(" {TAB} ")).unwrap();
        assert_eq!(options.default_spacing(), format!(" {TAB} "));
    }

    #[test]
    fn set_default_spacing_invalid() {
        macro_rules! assert_invalid_spacing {
            ($spacing:literal) => {
                let mut options: Options<BytesDomain> = Options::new();
                let error = options.set_default_spacing(bformat!($spacing)).unwrap_err();
                if let WriteError::InvalidSpacing = error {
                } else {
                    panic!("wrong error: {error:?}");
                }

                let mut options: Options<CharsDomain> = Options::new();
                let error = options.set_default_spacing(format!($spacing)).unwrap_err();
                if let WriteError::InvalidSpacing = error {
                } else {
                    panic!("wrong error: {error:?}");
                }
            };
        }

        assert_invalid_spacing!("abc");
        assert_invalid_spacing!("");
    }

    #[test]
    fn set_default_line_break() {
        let mut options: Options<BytesDomain> = Options::new();
        options.set_default_line_break(LineBreak::CrLf);
        assert_eq!(options.default_line_break(), LineBreak::CrLf);

        let mut options: Options<CharsDomain> = Options::new();
        options.set_default_line_break(LineBreak::CrLf);
        assert_eq!(options.default_line_break(), LineBreak::CrLf);
    }

    #[test]
    fn set_always_quoted() {
        let mut options: Options<BytesDomain> = Options::new();
        options.set_always_quoted(true);
        assert_eq!(options.always_quoted(), true);

        let mut options: Options<CharsDomain> = Options::new();
        options.set_always_quoted(true);
        assert_eq!(options.always_quoted(), true);
    }
}

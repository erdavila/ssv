use std::fmt::Debug;
use std::io::{BufRead, BufReader, Bytes, Read};

pub trait Domain: Eq + Debug {
    type Element: Copy + Eq + Debug;
    type ElementIterator<R: Read>: Iterator<Item = std::io::Result<Self::Element>>;
    type String: DomainString<Self::Element>;
    type StringSlice: DomainStringSlice<Self::Element> + ?Sized;

    const LF: Self::Element;
    const CR: Self::Element;
    const QUOTE: Self::Element;
    const HASH: Self::Element;

    fn is_spacing_element(element: Self::Element) -> bool;
    fn element_iterator<R: Read>(inner: R) -> Self::ElementIterator<R>;
}

pub trait DomainString<E>: Sized + Eq {
    fn new() -> Self;
    fn push(&mut self, element: E);
    fn quotes(length: usize) -> Self;

    fn from_element(element: E) -> Self {
        let mut string = Self::new();
        string.push(element);
        string
    }
}

pub trait DomainStringSlice<E> {
    fn as_bytes(&self) -> &[u8];
}

#[derive(PartialEq, Eq, Debug)]
pub struct BytesDomain;
impl Domain for BytesDomain {
    type Element = u8;
    type ElementIterator<R: Read> = Bytes<R>;
    type String = Vec<u8>;
    type StringSlice = [u8];

    const LF: Self::Element = b'\n';
    const CR: Self::Element = b'\r';
    const QUOTE: Self::Element = b'"';
    const HASH: Self::Element = b'#';

    fn is_spacing_element(element: Self::Element) -> bool {
        matches!(element, b' ' | b'\t')
    }

    fn element_iterator<R: Read>(inner: R) -> Self::ElementIterator<R> {
        inner.bytes()
    }
}

impl DomainString<u8> for Vec<u8> {
    fn new() -> Self {
        Vec::new()
    }

    fn push(&mut self, element: u8) {
        Vec::push(self, element);
    }

    fn quotes(length: usize) -> Self {
        [BytesDomain::QUOTE].repeat(length)
    }
}

impl DomainStringSlice<u8> for [u8] {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct CharsDomain;
impl Domain for CharsDomain {
    type Element = char;
    type ElementIterator<R: Read> = Chars<R>;
    type String = String;
    type StringSlice = str;

    const LF: Self::Element = '\n';
    const CR: Self::Element = '\r';
    const QUOTE: Self::Element = '"';
    const HASH: Self::Element = '#';

    fn is_spacing_element(element: Self::Element) -> bool {
        matches!(element, ' ' | '\t')
    }

    fn element_iterator<R: Read>(inner: R) -> Self::ElementIterator<R> {
        Chars {
            inner: BufReader::new(inner),
            chars: None,
        }
    }
}

impl DomainString<char> for String {
    fn new() -> Self {
        String::new()
    }

    fn push(&mut self, element: char) {
        String::push(self, element);
    }

    fn quotes(length: usize) -> Self {
        Self::from_element(CharsDomain::QUOTE).repeat(length)
    }
}

impl DomainStringSlice<char> for str {
    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

pub struct Chars<R: Read> {
    inner: BufReader<R>,
    chars: Option<std::vec::IntoIter<char>>,
}

impl<R: Read> Chars<R> {
    pub fn new(reader: R) -> Self {
        Chars {
            inner: BufReader::new(reader),
            chars: None,
        }
    }
}

impl<R: Read> Iterator for Chars<R> {
    type Item = Result<char, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut chars) = self.chars {
            if let Some(char) = chars.next() {
                return Some(Ok(char));
            }
        }

        let mut buf = String::new();
        if let Err(err) = self.inner.read_line(&mut buf) {
            return Some(Err(err));
        }

        let mut chars: std::vec::IntoIter<char> = buf.chars().collect::<Vec<_>>().into_iter();
        if let Some(char) = chars.next() {
            self.chars = Some(chars);
            Some(Ok(char))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chars() {
        let reader: &[u8] = b"abc def\n123 456";

        let chars = Chars::new(reader);

        let vec: Result<Vec<char>, std::io::Error> = chars.collect();
        assert_eq!(vec.unwrap(), "abc def\n123 456".chars().collect::<Vec<_>>(),);
    }

    #[test]
    fn chars_utf8() {
        let reader: &[u8] = b"\xC3\xB3rg\xC3\xA3o";

        let chars = Chars::new(reader);

        let vec: Result<Vec<char>, std::io::Error> = chars.collect();
        assert_eq!(vec.unwrap(), "órgão".chars().collect::<Vec<_>>(),);
    }

    #[test]
    fn chars_utf8_error() {
        let reader: &[u8] = b"a\nb\xFF "; // Invalid UTF-8

        let mut chars = Chars::new(reader);

        assert_eq!(chars.next().unwrap().unwrap(), 'a');
        assert_eq!(chars.next().unwrap().unwrap(), '\n');
        let error = chars.next().unwrap().unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    }
}

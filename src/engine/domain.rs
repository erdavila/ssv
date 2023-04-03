use std::io::{BufRead, BufReader, Bytes, Read};

pub trait Domain {
    type Element;
    type ElementIterator<R: Read>: Iterator<Item = std::io::Result<Self::Element>>;
    type String;
    type StringSlice: ?Sized;

    const LF: Self::Element;

    fn element_iterator<R: Read>(inner: R) -> Self::ElementIterator<R>;
}

pub struct BytesDomain;
impl Domain for BytesDomain {
    type Element = u8;
    type ElementIterator<R: Read> = Bytes<R>;
    type String = Vec<u8>;
    type StringSlice = [u8];

    const LF: Self::Element = b'\n';

    fn element_iterator<R: Read>(inner: R) -> Self::ElementIterator<R> {
        inner.bytes()
    }
}

pub struct CharsDomain;
impl Domain for CharsDomain {
    type Element = char;
    type ElementIterator<R: Read> = Chars<R>;
    type String = String;
    type StringSlice = str;

    const LF: Self::Element = '\n';

    fn element_iterator<R: Read>(inner: R) -> Self::ElementIterator<R> {
        Chars {
            inner: BufReader::new(inner),
            chars: None,
        }
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

use std::ops::Deref;

use ssv::engine::domain::{BytesDomain, CharsDomain, Domain};

use crate::combinations::CombinationsIterator;

macro_rules! assert_roundtrip {
    ($domain:ident, $regular_elem:literal, $spacing_elem:literal, $quote_elem:literal) => {
        use_domain_module!($domain);

        for combinations_size in 0..=5 {
            for elems in CombinationsIterator::new(
                [$regular_elem, $spacing_elem, $quote_elem],
                combinations_size,
            ) {
                let string: <$domain as Domain>::String = elems.into_iter().collect();
                let mut destination = Vec::new();

                write(&mut destination, [[string.deref()]]).unwrap();

                let values: ReadResult<Vec<_>> = read(destination.deref()).collect();

                assert_eq!(values.unwrap(), [[string]]);
            }
        }
    };
}

macro_rules! use_domain_module {
    (BytesDomain) => {
        use ssv::bytes::*;
    };
    (CharsDomain) => {
        use ssv::chars::*;
    };
}

#[test]
fn roundtrip_bytes() {
    assert_roundtrip!(BytesDomain, b'x', b' ', b'"');
}

#[test]
fn roundtrip_chars() {
    assert_roundtrip!(CharsDomain, 'x', ' ', '"');
}

mod combinations {
    pub struct RawCombinationsIterator {
        last_element: usize,
        next_combination: Option<Vec<usize>>,
    }

    impl RawCombinationsIterator {
        pub fn new(number_of_elements: usize, combinations_size: usize) -> Self {
            RawCombinationsIterator {
                last_element: number_of_elements - 1,
                next_combination: Some(std::iter::repeat(0).take(combinations_size).collect()),
            }
        }
    }

    impl Iterator for RawCombinationsIterator {
        type Item = Vec<usize>;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(ref mut next_combination) = self.next_combination {
                let to_return = next_combination.clone();

                if next_combination.is_empty() {
                    self.next_combination = None;
                } else {
                    for (i, element) in next_combination.iter_mut().enumerate().rev() {
                        if *element < self.last_element {
                            *element += 1;
                            break;
                        } else if i > 0 {
                            *element = 0;
                            continue;
                        } else {
                            self.next_combination = None;
                            break;
                        }
                    }
                }

                Some(to_return)
            } else {
                None
            }
        }
    }

    pub struct CombinationsIterator<T> {
        elements: Vec<T>,
        raw: RawCombinationsIterator,
    }

    impl<T> CombinationsIterator<T> {
        pub fn new(elements: impl IntoIterator<Item = T>, combinations_size: usize) -> Self {
            let elements: Vec<_> = elements.into_iter().collect();
            let len = elements.len();
            CombinationsIterator {
                elements,
                raw: RawCombinationsIterator::new(len, combinations_size),
            }
        }
    }

    impl<T: Clone> Iterator for CombinationsIterator<T> {
        type Item = Vec<T>;

        fn next(&mut self) -> Option<Self::Item> {
            self.raw.next().map(|indexes| {
                indexes
                    .iter()
                    .map(|index| self.elements[*index].clone())
                    .collect()
            })
        }
    }

    mod tests {
        use super::*;

        #[test]
        fn combinations_raw() {
            let values: Vec<_> = RawCombinationsIterator::new(3, 0).collect();
            assert_eq!(values, vec![vec![]]);

            let values: Vec<_> = RawCombinationsIterator::new(3, 3).collect();
            assert_eq!(
                values,
                vec![
                    vec![0, 0, 0],
                    vec![0, 0, 1],
                    vec![0, 0, 2],
                    vec![0, 1, 0],
                    vec![0, 1, 1],
                    vec![0, 1, 2],
                    vec![0, 2, 0],
                    vec![0, 2, 1],
                    vec![0, 2, 2],
                    vec![1, 0, 0],
                    vec![1, 0, 1],
                    vec![1, 0, 2],
                    vec![1, 1, 0],
                    vec![1, 1, 1],
                    vec![1, 1, 2],
                    vec![1, 2, 0],
                    vec![1, 2, 1],
                    vec![1, 2, 2],
                    vec![2, 0, 0],
                    vec![2, 0, 1],
                    vec![2, 0, 2],
                    vec![2, 1, 0],
                    vec![2, 1, 1],
                    vec![2, 1, 2],
                    vec![2, 2, 0],
                    vec![2, 2, 1],
                    vec![2, 2, 2],
                ]
            );
        }

        #[test]
        fn combinations() {
            let values: Vec<_> = CombinationsIterator::new(vec!['A', 'B', 'C'], 0).collect();
            assert_eq!(values, vec![vec![]]);

            let values: Vec<_> = CombinationsIterator::new(vec!['A', 'B', 'C'], 3).collect();
            assert_eq!(
                values,
                vec![
                    vec!['A', 'A', 'A'],
                    vec!['A', 'A', 'B'],
                    vec!['A', 'A', 'C'],
                    vec!['A', 'B', 'A'],
                    vec!['A', 'B', 'B'],
                    vec!['A', 'B', 'C'],
                    vec!['A', 'C', 'A'],
                    vec!['A', 'C', 'B'],
                    vec!['A', 'C', 'C'],
                    vec!['B', 'A', 'A'],
                    vec!['B', 'A', 'B'],
                    vec!['B', 'A', 'C'],
                    vec!['B', 'B', 'A'],
                    vec!['B', 'B', 'B'],
                    vec!['B', 'B', 'C'],
                    vec!['B', 'C', 'A'],
                    vec!['B', 'C', 'B'],
                    vec!['B', 'C', 'C'],
                    vec!['C', 'A', 'A'],
                    vec!['C', 'A', 'B'],
                    vec!['C', 'A', 'C'],
                    vec!['C', 'B', 'A'],
                    vec!['C', 'B', 'B'],
                    vec!['C', 'B', 'C'],
                    vec!['C', 'C', 'A'],
                    vec!['C', 'C', 'B'],
                    vec!['C', 'C', 'C'],
                ]
            );
        }
    }
}

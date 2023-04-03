use std::io::Read;
use std::marker::PhantomData;

use crate::engine::domain::Domain;

use super::ReadResult;

pub struct Reader<D: Domain, R: Read> {
    phantom: PhantomData<(D, R)>,
}

impl<D: Domain, R: Read> Reader<D, R> {
    pub fn new(_inner: R) -> Self {
        todo!()
    }
}

impl<D: Domain, R: Read> Iterator for Reader<D, R> {
    type Item = ReadResult<Vec<D::String>>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

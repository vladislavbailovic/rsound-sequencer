use crate::Mutator;

// TODO: global volume and envelope
// TODO: volumes track

pub struct Sequence<T> {
    seq: Vec<T>,
}

use std::ops::Deref;
impl<T> Deref for Sequence<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.seq
    }
}

impl<T> Sequence<T> {
    pub fn new(seq: Vec<T>) -> Self {
        Sequence { seq }
    }

    pub fn transform(&mut self, mutator: impl Mutator<Data = T>) -> &mut Self {
        self.seq = mutator.apply(&self.seq);
        self
    }
}

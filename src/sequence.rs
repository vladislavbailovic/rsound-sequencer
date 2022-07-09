use note::*;

use crate::Mutator;

pub struct Sequence {
    seq: Vec<Note>,
}

use std::ops::Deref;
impl Deref for Sequence {
    type Target = Vec<Note>;

    fn deref(&self) -> &Self::Target {
        &self.seq
    }
}

impl Sequence {
    pub fn new(seq: Vec<Note>) -> Self {
        Sequence { seq }
    }

    pub fn transform(&mut self, mutator: impl Mutator) -> &mut Self {
        self.seq = mutator.apply(&self.seq);
        self
    }
}

#[cfg(feature = "graph")]
pub mod graph;

use note::*;

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

    pub fn humanize(&mut self) -> &mut Self {
        let mut result = Vec::new();
        for &x in &self.seq {
            if let Note::Tone(p, o, val) = x {
                let offset = Value::from(1, 128, None);
                if offset.per_beat() > val.per_beat() {
                    result.push(Note::Rest(offset));
                    result.push(Note::Tone(p, o, val - offset));
                } else {
                    result.push(x);
                }
            } else {
                result.push(x);
            }
        }
        self.seq = result;
        self
    }
}

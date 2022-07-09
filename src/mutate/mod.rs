mod humanize;
pub use humanize::*;

use note::Note;

pub trait Mutator {
    fn apply(&self, sequence: &[Note]) -> Vec<Note>;
}

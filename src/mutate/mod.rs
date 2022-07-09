mod humanize;
pub use humanize::*;

// TODO: grid snapping mutator

use note::Note;

pub trait Mutator {
    fn apply(&self, sequence: &[Note]) -> Vec<Note>;
}

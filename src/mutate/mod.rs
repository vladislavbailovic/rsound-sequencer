mod humanize;
pub use humanize::*;

// TODO: grid snapping mutator

pub trait Mutator {
    type Data;
    fn apply(&self, sequence: &[Self::Data]) -> Vec<Self::Data>;
}

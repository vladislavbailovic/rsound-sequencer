mod humanize;
pub use humanize::*;
mod spaceout;
pub use spaceout::*;

// TODO: grid snapping mutator

pub trait Mutator {
    type Data;
    fn apply(&self, sequence: &[Self::Data]) -> Vec<Self::Data>;
}

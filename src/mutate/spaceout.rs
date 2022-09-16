use super::Mutator;
use crate::amount::Amount;
use note::*;

pub struct Spaceout {
    between: Value,
}
impl Spaceout {
    pub fn note(between: Value) -> SpaceoutNote {
        SpaceoutNote {
            space: Self { between },
        }
    }
    pub fn amount(between: Value) -> SpaceoutAmount {
        SpaceoutAmount {
            space: Self { between },
        }
    }
}

pub struct SpaceoutNote {
    space: Spaceout,
}

impl Mutator for SpaceoutNote {
    type Data = Note;

    fn apply(&self, sequence: &[Self::Data]) -> Vec<Self::Data> {
        let mut result = Vec::new();
        let pad = Note::Rest(self.space.between);
        for &x in sequence {
            result.push(x);
            result.push(pad);
        }
        result
    }
}

pub struct SpaceoutAmount {
    space: Spaceout,
}

impl Mutator for SpaceoutAmount {
    type Data = Amount;

    fn apply(&self, sequence: &[Self::Data]) -> Vec<Self::Data> {
        let mut result = Vec::new();
        let pad = Amount::zero(self.space.between);
        for &x in sequence {
            result.push(x);
            result.push(pad);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sequence;

    #[test]
    fn space_note() {
        let bpm = 60.0;
        let src = vec![
            note![A: C0, 1 / 4],
            note![C: C1, 1 / 4],
            note![A: C0, 1 / 4],
            note![B: C0, 1 / 4],
        ];
        let seq = Sequence::new(src.clone());
        let mut spaced = Sequence::new(src.clone());
        spaced.transform(Spaceout::note(val![1 / 4]));

        let total = seq.iter().map(|x| x.secs(bpm)).sum::<f64>();
        let equal = spaced.iter().map(|x| x.secs(bpm)).sum::<f64>();
        assert_eq!(total, equal / 2.0);
    }
}

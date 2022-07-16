use std::ops::Range;

use super::Mutator;
use crate::amount::Amount;
use crate::random::Random;
use note::*;

pub struct Humanize {
    resolution: usize,
    range: Range<usize>,
    frequency: f64,
}

impl Humanize {
    pub fn note() -> HumanizeNote {
        HumanizeNote {
            humanizer: Self {
                resolution: 256,
                range: Range { start: 1, end: 32 },
                frequency: 0.5,
            },
        }
    }
    pub fn amount() -> HumanizeAmount {
        HumanizeAmount {
            humanizer: Self {
                resolution: 100,
                range: Range {
                    start: 75,
                    end: 125,
                },
                frequency: 0.25,
            },
        }
    }

    fn is_showtime(&self) -> bool {
        let mut rng: Random = Default::default();
        let doit = rng.random(1, 10);
        doit as f64 > self.frequency * 10.0
    }

    fn range_value(&self) -> usize {
        let mut rng: Random = Default::default();
        rng.random(self.range.start, self.range.end)
    }
}

pub struct HumanizeNote {
    humanizer: Humanize,
}

impl Mutator for HumanizeNote {
    type Data = Note;

    fn apply(&self, sequence: &[Self::Data]) -> Vec<Self::Data> {
        let mut result = Vec::new();
        for &x in sequence {
            if !self.humanizer.is_showtime() {
                result.push(x);
                continue;
            }
            if let Note::Tone(p, o, val) = x {
                let offset = Value::from(
                    self.humanizer.range_value(),
                    self.humanizer.resolution,
                    None,
                );
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
        result
    }
}

pub struct HumanizeAmount {
    humanizer: Humanize,
}

impl Mutator for HumanizeAmount {
    type Data = Amount;

    fn apply(&self, sequence: &[Self::Data]) -> Vec<Self::Data> {
        sequence
            .iter()
            .map(|&x| {
                if !self.humanizer.is_showtime() {
                    return x;
                }
                if x.intensity() != 0.0 {
                    let ni = self.humanizer.range_value() as f64 / self.humanizer.resolution as f64;
                    return Amount::at(ni, x.duration());
                }
                x
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sequence;

    #[test]
    fn humanize_note() {
        let bpm = 60.0;
        let src = vec![
            note![A: C0, 1 / 4],
            pause![1 / 14],
            note![C: C1, 1 / 4 T],
            pause![1 / 14],
            note![A: C0, 1 / 8],
            pause![1 / 14],
            note![B: C0, 1 / 8 T],
        ];
        let seq = Sequence::new(src.clone());
        let mut humanized = Sequence::new(src.clone());
        humanized.transform(Humanize::note());

        let total = seq.iter().map(|x| x.secs(bpm)).sum::<f32>();
        let human_total = humanized.iter().map(|x| x.secs(bpm)).sum::<f32>();
        assert_eq!(total, human_total);

        let notes = seq
            .iter()
            .map(|x| if x.midi().is_some() { 1 } else { 0 })
            .sum::<i32>();
        let human_notes = humanized
            .iter()
            .map(|x| if x.midi().is_some() { 1 } else { 0 })
            .sum::<i32>();
        assert_eq!(notes, human_notes, "humanization preserves notes count");

        let playtime = seq
            .iter()
            .map(|x| if x.midi().is_some() { x.secs(bpm) } else { 0.0 })
            .sum::<f32>();
        let human_playtime = humanized
            .iter()
            .map(|x| if x.midi().is_some() { x.secs(bpm) } else { 0.0 })
            .sum::<f32>();
        assert!(
            playtime > human_playtime,
            "expected humanizing to reduce playtime. Original: {}, human: {}",
            playtime,
            human_playtime
        );

        let pausetime = seq
            .iter()
            .map(|x| if x.midi().is_none() { x.secs(bpm) } else { 0.0 })
            .sum::<f32>();
        let human_pausetime = humanized
            .iter()
            .map(|x| if x.midi().is_none() { x.secs(bpm) } else { 0.0 })
            .sum::<f32>();
        assert!(
            pausetime < human_pausetime,
            "expected humanizing to increase pausetime. Original: {}, human: {}",
            pausetime,
            human_pausetime
        );
    }

    #[test]
    fn humanize_amount() {
        let bpm = 60.0;
        let src = vec![
            Amount::at(0.75, val![1 / 4]),
            Amount::zero(val![1 / 14]),
            Amount::new(val![1/4 T]),
            Amount::zero(val![1 / 14]),
            Amount::at(0.5, val![1 / 8]),
            Amount::zero(val![1 / 14]),
            Amount::new(val![1/8 T]),
        ];
        let seq = Sequence::new(src.clone());
        let mut humanized = Sequence::new(src.clone());
        humanized.transform(Humanize::amount());

        let total = seq.iter().map(|x| x.duration().secs(bpm)).sum::<f32>();
        let human_total = humanized
            .iter()
            .map(|x| x.duration().secs(bpm))
            .sum::<f32>();
        assert_eq!(total, human_total);
    }
}

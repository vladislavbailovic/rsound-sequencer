use std::ops::Range;
use std::time::{SystemTime, UNIX_EPOCH};

use super::Mutator;
use note::*;

struct Random {
    a: usize,
    b: usize,
    prev: usize,
}

impl Default for Random {
    fn default() -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Self {
            prev: since_the_epoch.subsec_nanos() as usize,
            a: 15342,
            b: 45194,
        }
    }
}

impl Random {
    fn random(&mut self, min: usize, max: usize) -> usize {
        assert!(max > min, "reverse humanizing not possible");

        let max = max - min;
        let prev = (self.prev * self.a + self.b) % 1000000000;
        self.prev = prev;

        (prev as f32 % ((max - min) + 1) as f32) as usize + min
    }
}

pub struct Humanize {
    resolution: usize,
    range: Range<usize>,
    frequency: f32,
}

impl Default for Humanize {
    fn default() -> Self {
        Self {
            resolution: 256,
            range: Range { start: 1, end: 32 },
            frequency: 0.5,
        }
    }
}

impl Mutator for Humanize {
    type Data = Note;

    fn apply(&self, sequence: &[Self::Data]) -> Vec<Self::Data> {
        let mut rng: Random = Default::default();
        let mut result = Vec::new();
        for &x in sequence {
            let doit = rng.random(1, 10);
            if doit as f32 > self.frequency * 10.0 {
                eprintln!(
                    "not humanizing because {} > {}!",
                    doit,
                    self.frequency * 10.0
                );
                result.push(x);
                continue;
            }
            if let Note::Tone(p, o, val) = x {
                let offset = Value::from(
                    rng.random(self.range.start, self.range.end),
                    self.resolution,
                    None,
                );
                eprintln!("humanizing with offset: {:#?}", offset);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sequence;

    #[test]
    fn randomize() {
        let max = 1312;
        let min = 161;
        let mut rng: Random = Default::default();
        let mut prev: usize = 0;
        for _ in 1..256 {
            let rnd = rng.random(min, max);
            assert!(rnd >= min, "random is greater than minimum");
            assert!(rnd <= max, "random is lesser than maximum");
            assert!(rnd != prev, "random is different to previous");

            prev = rnd;
        }
    }

    #[test]
    fn humanize() {
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
        humanized.transform(Humanize::default());

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
}

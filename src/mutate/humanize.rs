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
            resolution: 128,
            range: Range { start: 1, end: 128 },
            frequency: 0.5,
        }
    }
}

impl Mutator for Humanize {
    fn apply(&self, sequence: &[Note]) -> Vec<Note> {
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
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

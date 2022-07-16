use std::time::{SystemTime, UNIX_EPOCH};

pub struct Random {
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
    pub fn random(&mut self, min: usize, max: usize) -> usize {
        assert!(max > min, "reverse humanizing not possible");

        let max = max - min;
        let prev = (self.prev * self.a + self.b) % 1000000000;
        self.prev = prev;

        (prev as f32 % ((max - min) + 1) as f32) as usize + min
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

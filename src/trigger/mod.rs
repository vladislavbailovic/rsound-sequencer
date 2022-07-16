// TODO: global volume and envelope
// TODO: volumes track

pub trait Trigger {
    fn intensity(&self) -> f64;
    fn set_intensity(&mut self, ni: f64);
}

struct Volume {
    value: f64,
}

impl Default for Volume {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Trigger for Volume {
    fn intensity(&self) -> f64 {
        self.value
    }
    fn set_intensity(&mut self, ni: f64) {
        self.value = ni;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let v: Volume = Default::default();
        assert_eq!(v.intensity(), 1.0, "full blast by default");
    }

    #[test]
    fn intensity() {
        let mut v: Volume = Default::default();
        v.set_intensity(0.25);
        assert_eq!(v.intensity(), 0.25);
    }
}

use note::Value;
// TODO: global volume and envelope
// TODO: volumes track

pub trait Trigger {
    fn intensity(&self) -> f64;
    fn set_intensity(&mut self, ni: f64);
    fn duration(&self) -> Value;
}

pub struct Amount {
    value: f64,
    duration: Value,
}

impl Trigger for Amount {
    fn intensity(&self) -> f64 {
        self.value
    }
    fn set_intensity(&mut self, ni: f64) {
        self.value = ni;
    }
    fn duration(&self) -> Value {
        self.duration
    }
}

impl Amount {
    pub fn new(duration: Value) -> Self {
        Self {
            value: 1.0,
            duration,
        }
    }

    pub fn at(value: f64, duration: Value) -> Self {
        Self { value, duration }
    }

    pub fn zero(duration: Value) -> Self {
        Self {
            value: 0.0,
            duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intensity() {
        let mut v = Amount::new(Value::Frac(1.0));
        v.set_intensity(0.25);
        assert_eq!(v.intensity(), 0.25);
    }
}

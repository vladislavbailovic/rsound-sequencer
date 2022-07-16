use note::Value;
// TODO: global volume and envelope
// TODO: volumes track

#[derive(Clone, Copy)]
pub struct Amount {
    value: f64,
    duration: Value,
}

impl Amount {
    pub fn intensity(&self) -> f64 {
        self.value
    }
    pub fn set_intensity(&mut self, ni: f64) {
        self.value = ni;
    }
    pub fn duration(&self) -> Value {
        self.duration
    }

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
        let v = Amount::new(Value::Frac(1.0));
        assert_eq!(v.intensity(), 1.0);
    }
}

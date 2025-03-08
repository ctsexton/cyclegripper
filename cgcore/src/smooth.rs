pub struct SmoothValue {
    current: f32,
    target: f32,
    samples_to_target: usize,
    steps: usize,
}

impl SmoothValue {
    pub fn new(initial: f32) -> Self {
        Self {
            current: initial,
            target: initial,
            samples_to_target: 0,
            steps: 0,
        }
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
        self.samples_to_target = self.steps;
    }

    pub fn set_distance(&mut self, sample_rate: f64, time_secs: f64) {
        self.steps = (sample_rate * time_secs).floor() as usize;
    }

    pub fn next(&mut self) -> f32 {
        let distance = self.target - self.current;
        let increment = if self.samples_to_target > 0 {
            distance / self.samples_to_target as f32
        } else {
            0.0
        };
        self.current = self.current + increment;
        if self.samples_to_target > 0 {
            self.samples_to_target = self.samples_to_target - 1;
        }
        self.current
    }
}

#[cfg(test)]
mod tests {
    use super::SmoothValue;

    #[test]
    fn test_smooth_value() {
        let mut sv = SmoothValue::new(0.0);
        assert_eq!(sv.next(), 0.0);
        sv.set_target(1.0);
        // Step size is still zero so we can't move to target yet
        assert_eq!(sv.next(), 0.0);
        assert_eq!(sv.next(), 0.0);

        sv.set_distance(10.0, 0.4);
        sv.set_target(1.0);
        assert_eq!(sv.next(), 0.25);
        assert_eq!(sv.next(), 0.5);
        assert_eq!(sv.next(), 0.75);
        assert_eq!(sv.next(), 1.0);
        assert_eq!(sv.next(), 1.0);
        assert_eq!(sv.next(), 1.0);
        assert_eq!(sv.next(), 1.0);
    }
}

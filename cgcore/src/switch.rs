pub struct TimedSwitch {
    samples: u32,
    sample_rate: f64,
}

impl TimedSwitch {
    pub fn new(sample_rate: f64) -> Self {
        Self { samples: 0, sample_rate }
    }

    pub fn tick(&mut self) -> bool {
        if self.samples > 0 {
            self.samples -= 1;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, time_secs: f64) {
        self.samples = (time_secs * self.sample_rate).floor() as u32;
    }
}

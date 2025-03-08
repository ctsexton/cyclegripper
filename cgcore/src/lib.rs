pub mod ringbuffer;
pub mod smooth;
pub mod switch;

use std::f32::consts::PI;

use ringbuffer::Ringbuffer;
use smooth::SmoothValue;
use switch::TimedSwitch;

#[derive(Debug)]
pub struct Trig {
    pub offset: f32,
    pub length: Option<f32>,
}

pub struct Processor {
    buffers: [Ringbuffer; 2],
    switch: TimedSwitch,
    delay_time_secs: f32,
    sample_rate: f64,
    bypass: bool,
    switch_status: bool,
    smooth_drywet: SmoothValue,
    switch_block: [bool; MAX_BLOCK_SIZE],
    mix_block: [f32; MAX_BLOCK_SIZE],
}

pub const MAX_BLOCK_SIZE: usize = 8192;

impl Processor {
    pub fn new(sample_rate: f64) -> Self {
        let mut smooth_drywet = SmoothValue::new(0.0); // 0.0 is full dry
        smooth_drywet.set_distance(sample_rate, 0.002);

        Self {
            buffers: [
                Ringbuffer::new((sample_rate * 2.0) as usize),
                Ringbuffer::new((sample_rate * 2.0) as usize),
            ],
            switch: TimedSwitch::new(sample_rate),
            delay_time_secs: 0.5,
            sample_rate,
            bypass: true,
            switch_status: false,
            smooth_drywet,
            switch_block: [false; MAX_BLOCK_SIZE],
            mix_block: [0.0; MAX_BLOCK_SIZE],
        }
    }

    pub fn process(
        &mut self,
        input: [&[f32]; 2],
        mut output: &mut [Vec<f32>],
        trigs: &[Trig],
        drywet: f32,
    ) {
        let n_samples = input.get(0).unwrap().len();

        if (drywet - self.smooth_drywet.current()).abs() > 0.0001 {
            self.smooth_drywet.set_target(drywet.clamp(0.0, 1.0));
        }

        for i in 0..n_samples {
            if let Some(trig) = trigs.get(0) {
                if trig.offset.floor() as usize == i {
                    if let Some(length) = trig.length {
                        self.switch.reset(length as f64);
                    }
                }
            }
            let item = self.switch_block.get_mut(i).unwrap();
            *item = self.switch.tick();

            *self.mix_block.get_mut(i).unwrap() = self.smooth_drywet.next();
        }

        for ch in 0..2 {
            for i in 0..n_samples {
                let outbuf_ch = if ch == 0 { 1 } else { 0 };
                let read_buffer = self.buffers.get_mut(ch).unwrap();
                if let Some(trig) = trigs.get(0) {
                    if let Some(length) = trig.length {
                        if (trig.offset).floor() as usize == i {
                            let offset_samples =
                                (length as f64 * self.sample_rate).floor() as usize;
                            read_buffer.set_read_offset(offset_samples);
                            self.bypass = false;
                        }
                    } else {
                        self.bypass = true;
                    }
                }
                let delayed_sample = read_buffer.read();
                let switch_on = self.bypass || *self.switch_block.get(i).unwrap();
                if self.switch_status != switch_on {
                    self.switch_status = switch_on;
                }
                let input_sample = *input.get(ch).unwrap().get(i).unwrap();
                let fx_signal = if switch_on {
                    input_sample
                } else {
                    delayed_sample
                };
                let buffer = self.buffers.get_mut(outbuf_ch).unwrap();
                buffer.write(fx_signal);
                let (dry, wet) = equal_power_fade(*self.mix_block.get(i).unwrap());
                let out = dry * input_sample + wet * fx_signal;
                *output.get_mut(ch).unwrap().get_mut(i).unwrap() = out;
            }
        }
    }

    fn write_to_buffers(&mut self, to_write: &[[f32; 2]]) {}
}

fn equal_power_fade(position: f32) -> (f32, f32) {
    let position = position.clamp(0.0, 1.0);
    let a = (0.5 + 0.5 * (position * PI).cos()).sqrt();
    let b = (0.5 - 0.5 * (position * PI).cos()).sqrt();
    (a, b)
}

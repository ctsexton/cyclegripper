pub mod switch;
pub mod ringbuffer;

use ringbuffer::Ringbuffer;
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
}

pub const MAX_BLOCK_SIZE: usize = 8192;

impl Processor {
    pub fn new(sample_rate: f64) -> Self {
        println!("Initializing");
        Self {
            buffers: [Ringbuffer::new((sample_rate * 2.0) as usize), Ringbuffer::new((sample_rate * 2.0) as usize)],
            switch: TimedSwitch::new(sample_rate),
            delay_time_secs: 0.5,
            sample_rate,
            bypass: true,
            switch_status: false,
        }
    }

    pub fn process(&mut self, input: [&[f32];2], mut output: &mut [Vec<f32>], trigs: &[Trig]) {
        let n_samples = input.get(0).unwrap().len();

        let mut switch_block = [false; MAX_BLOCK_SIZE];
        let mut bypass_block = [true; MAX_BLOCK_SIZE];
        for i in 0..n_samples {
            if let Some(trig) = trigs.get(0) {
                if trig.offset.floor() as usize == i {
                    if let Some(length) = trig.length {
                        println!("Trigger switch");
                        self.switch.reset(length as f64);
                    }
                }
            }
            let item = switch_block.get_mut(i).unwrap();
            *item = self.switch.tick();
        }
        

        for ch in 0..2 {
            for i in 0..n_samples {
                let outbuf_ch = if ch == 0 { 1 } else { 0 };
                let read_buffer = self.buffers.get_mut(ch).unwrap();
                if let Some(trig) = trigs.get(0) {
                    if let Some(length) = trig.length {
                        if (trig.offset).floor() as usize == i {
                            let offset_samples = (length as f64 * self.sample_rate).floor() as usize;
                            read_buffer.set_read_offset(offset_samples);
                            self.bypass = false;
                        }
                    } else {
                        self.bypass = true;
                    }
                }
                let delayed_sample = read_buffer.read();
                let switch_on = self.bypass || *switch_block.get(i).unwrap();
                if self.switch_status != switch_on {
                    println!("Switch changed: {:?}", switch_on);
                    self.switch_status = switch_on;
                }
                let input_sample = *input.get(ch).unwrap().get(i).unwrap();
                let to_write = if switch_on {input_sample} else {delayed_sample};
                let buffer = self.buffers.get_mut(outbuf_ch).unwrap();
                buffer.write(to_write);
                *output.get_mut(ch).unwrap().get_mut(i).unwrap() = to_write;
            }
        }
    }

    fn write_to_buffers(&mut self, to_write: &[[f32; 2]]) {}
}

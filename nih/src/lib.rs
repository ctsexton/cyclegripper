use nih_plug::prelude::*;
use std::sync::Arc;
use cgcore::{Trig, Processor, MAX_BLOCK_SIZE};
use dasp_signal::{noise, Noise};

struct CycleGripper {
    params: Arc<CycleGripperParams>,
    processor: Option<Processor>,
    inputs: Vec<Vec<f32>>,
    outputs: Vec<Vec<f32>>,
    trigs: Vec<Trig>,
    noise: Noise,
}

impl Default for CycleGripper {
    fn default() -> Self {
        Self { params: Arc::new(CycleGripperParams::default()), processor: None, inputs: vec![], outputs: vec![], trigs: vec![], noise: noise(0) }
    }
}

#[derive(Params)]
struct CycleGripperParams {
    #[id = "drywet"]
    pub drywet: FloatParam
}

impl Default for CycleGripperParams {
    fn default() -> Self {
        Self { drywet: FloatParam::new("DryWet", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 })}
    }
}

impl Plugin for CycleGripper {
    const NAME: &'static str = "CycleGripper";
    const VENDOR: &'static str = "CTS";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "cameron.t.sexton@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        aux_input_ports: &[],
        aux_output_ports: &[],
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(&mut self, _audio_io_layout: &AudioIOLayout, buffer_config: &BufferConfig, _context: &mut impl InitContext<Self>) -> bool {
        let processor = Processor::new(buffer_config.sample_rate as f64);
        self.processor = Some(processor);
        self.inputs = vec![vec![0_f32; MAX_BLOCK_SIZE]; 2];
        self.outputs = vec![vec![0_f32; MAX_BLOCK_SIZE]; 2];
        self.trigs = Vec::<Trig>::with_capacity(MAX_BLOCK_SIZE);
        true
    }

    fn reset(&mut self) {}

    fn process(&mut self, buffer: &mut Buffer, _aux: &mut AuxiliaryBuffers, context: &mut impl ProcessContext<Self>) -> ProcessStatus {
        for buf in self.inputs.iter_mut() {
            for item in buf.iter_mut() {
                *item = 0.0;
            }
        }
        for buf in self.outputs.iter_mut() {
            for item in buf.iter_mut() {
                *item = 0.0;
            }
        }
        // We're going to copy the mono input channel to all processor ins for now
        for (in_copy, in_proc) in self.inputs.as_mut_slice().iter_mut().zip(buffer.as_slice().iter()) {
            for (sample, copy) in in_proc.iter().zip(in_copy.iter_mut()) {
                *copy = *sample;
            }
        }


        let block_size = buffer.samples();
        self.trigs.clear();
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { note, .. } => {
                    println!("Note: {:?}", note);
                    if note == 60 {
                        let length = (self.noise.next_sample() * 0.5 + 0.5) * 0.15 + 0.02;
                        let trig = Trig {offset: event.timing().clamp(0, block_size as u32) as f32, length: Some(length as f32)};
                        self.trigs.push(trig);
                    } else {
                        let trig = Trig {offset: event.timing().clamp(0, block_size as u32) as f32, length: None};
                        self.trigs.push(trig);
                    }
                }
                _ => {}
            }
        }

        let inputs_to_process = [&self.inputs.get(0).unwrap()[0..block_size], &self.inputs.get(1).unwrap()[0..block_size]];
        let outputs_to_process = &mut self.outputs;
        self.processor.as_mut().unwrap().process(inputs_to_process, outputs_to_process, self.trigs.as_slice());

        for (processor_out, plugin_out) in self.outputs.iter().zip(buffer.as_slice().iter_mut()) {
            for i in 0..block_size {
                *plugin_out.get_mut(i).unwrap() = *processor_out.get(i).unwrap();
            }
        }


        ProcessStatus::Normal
    }
}

impl ClapPlugin for CycleGripper {
    const CLAP_ID: &'static str = "com.camsexton.cyclegripper";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Cycle gripping");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

nih_export_clap!(CycleGripper);

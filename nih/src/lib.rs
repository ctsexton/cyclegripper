use nih_plug::prelude::*;
use std::sync::Arc;

struct CycleGripper {
    params: Arc<CycleGripperParams>
}

impl Default for CycleGripper {
    fn default() -> Self {
        Self { params: Arc::new(CycleGripperParams::default()) }
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
        main_input_channels: NonZeroU32::new(1),
        main_output_channels: NonZeroU32::new(1),
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
        true
    }

    fn reset(&mut self) {}

    fn process(&mut self, buffer: &mut Buffer, _aux: &mut AuxiliaryBuffers, context: &mut impl ProcessContext<Self>) -> ProcessStatus {
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

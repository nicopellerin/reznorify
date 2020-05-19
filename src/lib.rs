#[macro_use] 
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

use std::sync::Arc;

struct Reznorify {
    params: Arc<ReznorifyParameters>
}

struct ReznorifyParameters {
    threshold: AtomicFloat,
    gain: AtomicFloat
}

impl Default for Reznorify {
    fn default() -> Reznorify {
        Reznorify {
            params: Arc::new(ReznorifyParameters::default()),
        }
    }
}

impl Default for ReznorifyParameters {
    fn default() -> ReznorifyParameters {
        ReznorifyParameters {
            threshold: AtomicFloat::new(0.5),
            gain: AtomicFloat::new(1.0),
        }
    }
}

impl Plugin for Reznorify {
    fn get_info(&self) -> Info {
        Info {
            name: "Reznorify".to_string(),
            vendor: "nicopellerin".to_string(),
            unique_id: 450845848,

            inputs: 2,
            outputs: 2,
            parameters: 2,

            ..Default::default()
        }
    }

    
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let threshold = self.params.threshold.get();
        let gain = self.params.gain.get();

        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {

                if *input_sample >= 0.0 {
                    *output_sample = input_sample.min(threshold) / threshold;
                    *output_sample = input_sample * gain;
                } else {
                    *output_sample = input_sample.max(-threshold) / threshold;
                    *output_sample = input_sample * gain;
                }
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

impl PluginParameters for ReznorifyParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold.get(),
            1 => self.gain.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        #[allow(clippy::single_match)]
        match index {
            0 => self.threshold.set(value.max(0.01)),
            1 => self.gain.set(value),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold".to_string(),
            1 => "Gain".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}", self.threshold.get()),
            1 => format!("{:.2}", self.gain.get()),
            _ => "".to_string(),
        }
    }
}

plugin_main!(Reznorify);
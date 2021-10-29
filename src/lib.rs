#[macro_use]
extern crate vst;

use std::sync::{ Arc, RwLock };

use vst::{
    buffer::AudioBuffer,
    plugin::{Category, Info, Plugin, PluginParameters},
};

struct TanhEffect {
    params: Arc<TanhParams>,
}

impl Default for TanhEffect {
    fn default() -> Self {
        Self {
            params: Arc::new(TanhParams::default()),
        }
    }
}

impl Plugin for TanhEffect {
    fn get_info(&self) -> Info {
        Info {
            name: "tanh".to_string(),
            vendor: "g-s-k".to_string(),
            unique_id: 1369,
            category: Category::Effect,
            parameters: 3,
            inputs: 2,
            outputs: 2,

            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        for (input, output) in buffer.zip() {
            for (in_sample, out_sample) in input.into_iter().zip(output.into_iter()) {
                let amplified_signal = *in_sample * *self.params.gain.read().unwrap();
                let ceiling = *self.params.ceiling.read().unwrap();
                let squished_signal = (amplified_signal / ceiling).tanh();
                let mix = *self.params.mix.read().unwrap();
                *out_sample = squished_signal * mix + *in_sample * (1.0 - mix);
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        self.params.clone()
    }
}

plugin_main!(TanhEffect);

struct TanhParams {
    gain: RwLock<f32>,
    ceiling: RwLock<f32>,
    mix: RwLock<f32>,
}

impl Default for TanhParams {
    fn default() -> Self {
        Self {
            gain: RwLock::new(1.0),
            ceiling: RwLock::new(1.0),
            mix: RwLock::new(1.0),
        }
    }
}

impl PluginParameters for TanhParams {
    fn can_be_automated(&self, _index: i32) -> bool {
        true
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Gain".to_string(),
            1 => "Ceiling".to_string(),
            2 => "Dry/Wet".to_string(),
            _ => unreachable!(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => String::new(),
            1 => String::new(),
            2 => "%".to_string(),
            _ => unreachable!(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => self.gain.read().unwrap().to_string(),
            1 => self.ceiling.read().unwrap().to_string(),
            2 => format!("{:.2}", *self.mix.read().unwrap() * 100.0),
            _ => unreachable!(),
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => *self.gain.read().unwrap(),
            1 => *self.ceiling.read().unwrap(),
            2 => *self.mix.read().unwrap(),
            _ => unreachable!(),
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => *self.gain.write().unwrap() = value,
            1 => *self.ceiling.write().unwrap() = value,
            2 => *self.mix.write().unwrap() = value,
            _ => unreachable!(),
        }
    }
}

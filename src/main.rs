use std::{
    thread,
    time::{Duration, Instant},
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use pitch::detect::PitchFrequencyDetector;
use ui::Ui;

const UI_RENDER_TICK_TIME: Duration = Duration::from_millis(1000 / 10);

mod pitch;
mod ui;

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Should have a default input device");
    let config = device
        .default_input_config()
        .expect("Should have a default input config")
        .config();

    let mut pitch_frequency_detector = PitchFrequencyDetector::new(config.clone());
    let mut ui = Ui::new();

    let mut last_render_timestamp = Instant::now();

    let stream = device
        .build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if last_render_timestamp.elapsed() < UI_RENDER_TICK_TIME {
                    return;
                }

                let frequency = pitch_frequency_detector.find_pitch_frequency(data);

                if let Some(frequency) = frequency {
                    ui.render(frequency).expect("Should be able to render UI");
                }

                last_render_timestamp = Instant::now();
            },
            move |_| {},
            None,
        )
        .expect("Should be able to build input stream");

    stream.play().expect("Should be able to play input stream");

    thread::park();
}

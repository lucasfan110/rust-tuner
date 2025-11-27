use cpal::StreamConfig;
use pitch_detection::{
    Pitch,
    detector::{PitchDetector, yin::YINDetector},
};

const TUNER_POWER_THRESHOLD: f32 = 0.1;
const TUNER_CLARITY_THRESHOLD: f32 = 0.8;

fn downmix_audio(buffer: &mut Vec<f32>, audio_data: &[f32], num_channels: usize) {
    buffer.extend(
        audio_data
            .chunks_exact(num_channels)
            .map(|d| d.iter().sum::<f32>() / num_channels as f32),
    );
}

/// Audio data is assumed to be mono (1 channel)
fn detect_pitch(audio_data: &[f32], sample_rate: usize) -> Option<Pitch<f32>> {
    let len = audio_data.len();
    let mut detector = YINDetector::new(len, len);

    detector.get_pitch(
        audio_data,
        sample_rate,
        TUNER_POWER_THRESHOLD,
        TUNER_CLARITY_THRESHOLD,
    )
}

#[derive(Debug, Clone)]
pub struct PitchFrequencyDetector {
    downmixed_audio_buffer: Vec<f32>,
    input_device_config: StreamConfig,
}

impl PitchFrequencyDetector {
    pub fn new(input_device_config: StreamConfig) -> Self {
        Self {
            downmixed_audio_buffer: Vec::with_capacity(1024),
            input_device_config,
        }
    }

    pub fn find_pitch_frequency(&mut self, audio_data: &[f32]) -> Option<f32> {
        downmix_audio(
            &mut self.downmixed_audio_buffer,
            audio_data,
            self.input_device_config.channels as usize,
        );

        let pitch = detect_pitch(
            &self.downmixed_audio_buffer,
            self.input_device_config.sample_rate.0 as usize,
        )
        .map(|p| p.frequency);

        self.downmixed_audio_buffer.clear();

        pitch
    }
}

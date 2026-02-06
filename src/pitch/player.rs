use std::f64::{self, consts::PI};

use anyhow::anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

const PITCH_VOLUME: f64 = 0.2;

pub struct PitchPlayer {
    output_device: cpal::Device,
    config: cpal::SupportedStreamConfig,
    stream: Option<cpal::Stream>,
}

impl PitchPlayer {
    pub fn new() -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let output_device = host
            .default_output_device()
            .ok_or(anyhow!("No output device"))?;
        let config = output_device.default_output_config()?;

        Ok(Self {
            output_device,
            config,
            stream: None,
        })
    }

    fn play<T>(&mut self, frequency: f64) -> anyhow::Result<()>
    where
        T: cpal::Sample + cpal::SizedSample + cpal::FromSample<f64>,
    {
        let sample_rate = self.config.sample_rate() as f64;
        let channels = self.config.channels() as usize;

        let mut sample_clock = 0f64;
        let frequency_rounded = frequency.round();

        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;

            let value = (sample_clock * frequency_rounded * 2.0 * PI / sample_rate).sin();

            value * PITCH_VOLUME * ((440.0 / frequency_rounded) * 0.5 + 0.5)
        };

        let err_fn = |err| eprintln!("An error occurred on stream: {}", err);

        let stream = self.output_device.build_output_stream(
            &self.config.clone().into(),
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let value: T = cpal::Sample::from_sample(next_value());

                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            err_fn,
            None,
        )?;

        stream.play()?;

        self.stream = Some(stream);

        Ok(())
    }

    pub fn play_pitch(&mut self, pitch: f64) -> anyhow::Result<()> {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.play::<f32>(pitch),
            cpal::SampleFormat::I16 => self.play::<i16>(pitch),
            cpal::SampleFormat::U16 => self.play::<u16>(pitch),
            _ => Err(anyhow!("Unsupported sample format")),
        }
    }
}

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    BuildStreamError, DefaultStreamConfigError, Device, PauseStreamError, PlayStreamError, Stream,
    StreamConfig,
};
use std::any::Any;
use std::fs::File;
use std::io::BufReader;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SoundError {
    #[error("No available audio output device")]
    NoOutputDeviceError,
    #[error(transparent)]
    DefaultStreamConfigError(#[from] DefaultStreamConfigError),
    #[error(transparent)]
    PauseStreamError(#[from] PauseStreamError),
    #[error(transparent)]
    PlayStreamError(#[from] PlayStreamError),
    #[error(transparent)]
    BuildStreamError(#[from] BuildStreamError),
}

pub struct Sound {
    device: Device,
    stream: Option<Stream>,
}

impl Sound {
    pub fn new() -> Result<Sound, SoundError> {
        let device = cpal::default_host()
            .default_output_device()
            .ok_or(SoundError::NoOutputDeviceError)?;

        Ok(Self {
            device,
            stream: None,
        })
    }

    pub fn beep(&mut self) -> Result<(), SoundError> {
        if self.stream.is_some() {
            return Ok(());
        }

        let config: &StreamConfig = &self.device.default_output_config()?.into();
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        let mut sample_clock = 0f32;
        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
        };

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = self.device.build_output_stream(
            config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value)
            },
            err_fn,
        )?;

        stream.play()?;
        self.stream = Some(stream);

        Ok(())
    }

    pub fn stop_beep(&mut self) -> Result<(), SoundError> {
        if let Some(_) = self.stream {
            self.stream = None
        }

        Ok(())
    }
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

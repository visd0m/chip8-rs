use crate::emulator;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BuildStreamError, Device, PlayStreamError, Sample, SampleFormat, Stream, StreamConfig};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("No available audio output device")]
    NoOutputDeviceError,
    #[error(transparent)]
    PlayStreamError(#[from] PlayStreamError),
    #[error(transparent)]
    BuildStreamError(#[from] BuildStreamError),
}

pub struct Audio {
    device: Device,
    stream: Option<Stream>,
}

impl Audio {
    pub fn new() -> Result<Audio, AudioError> {
        let device = cpal::default_host()
            .default_output_device()
            .ok_or(AudioError::NoOutputDeviceError)?;

        Ok(Self {
            device,
            stream: None,
        })
    }
}

impl emulator::audio::Audio for Audio {
    fn beep(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.stream.is_some() {
            return Ok(());
        }

        let config = self.device.default_output_config()?;
        let stream = match config.sample_format() {
            SampleFormat::F32 => beep_stream::<f32>(&self.device, &config.into()),
            SampleFormat::I16 => beep_stream::<i16>(&self.device, &config.into()),
            SampleFormat::U16 => beep_stream::<u16>(&self.device, &config.into()),
        }?;

        stream.play()?;
        self.stream = Some(stream);

        Ok(())
    }

    fn stop_beep(&mut self) {
        if self.stream.is_some() {
            self.stream = None
        }
    }
}

fn beep_stream<T>(device: &Device, config: &StreamConfig) -> Result<Stream, AudioError>
where
    T: Sample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
    )?;

    stream.play()?;
    Ok(stream)
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

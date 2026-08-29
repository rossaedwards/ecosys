//! Phase 5 — audio capture via `cpal`.
//!
//! Captures the system's default *output* device rather than an input
//! device: on Windows, cpal's WASAPI backend automatically enables loopback
//! capture (`AUDCLNT_STREAMFLAGS_LOOPBACK`) whenever an input stream is
//! built on a render (output) device. On Linux this resolves to whatever
//! PipeWire/PulseAudio treats as that device's monitor source. Falls back
//! to the default input device (e.g. a microphone) if no output device
//! capture is available.

use std::sync::mpsc::Sender;

use anyhow::Context;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};

pub struct AudioCapture {
    _stream: cpal::Stream,
    pub channels: usize,
    pub sample_rate: u32,
}

fn stream_error(err: cpal::StreamError) {
    eprintln!("[audio] stream error: {err}");
}

/// Start capturing audio, sending interleaved f32 PCM chunks to `tx` as
/// they arrive. Returns an [`AudioCapture`] handle that must be kept alive
/// for the duration of capture (dropping it stops the stream).
pub fn start_capture(tx: Sender<Vec<f32>>) -> anyhow::Result<AudioCapture> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .or_else(|| host.default_input_device())
        .context("no audio output or input device available")?;

    eprintln!(
        "[audio] capturing from: {}",
        device.name().unwrap_or_else(|_| "<unknown device>".to_string())
    );

    let supported_config = device
        .default_output_config()
        .or_else(|_| device.default_input_config())
        .context("device exposes no usable audio config")?;

    let sample_format = supported_config.sample_format();
    let stream_config: StreamConfig = supported_config.into();
    let channels = stream_config.channels as usize;
    let sample_rate = stream_config.sample_rate.0;

    let stream = match sample_format {
        SampleFormat::F32 => device.build_input_stream(
            &stream_config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let _ = tx.send(data.to_vec());
            },
            stream_error,
            None,
        )?,
        SampleFormat::I16 => device.build_input_stream(
            &stream_config,
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                let pcm: Vec<f32> = data.iter().map(|s| *s as f32 / i16::MAX as f32).collect();
                let _ = tx.send(pcm);
            },
            stream_error,
            None,
        )?,
        SampleFormat::U16 => device.build_input_stream(
            &stream_config,
            move |data: &[u16], _: &cpal::InputCallbackInfo| {
                let pcm: Vec<f32> = data
                    .iter()
                    .map(|s| (*s as f32 - 32768.0) / 32768.0)
                    .collect();
                let _ = tx.send(pcm);
            },
            stream_error,
            None,
        )?,
        other => anyhow::bail!("unsupported sample format: {other:?}"),
    };

    stream.play()?;

    Ok(AudioCapture {
        _stream: stream,
        channels,
        sample_rate,
    })
}

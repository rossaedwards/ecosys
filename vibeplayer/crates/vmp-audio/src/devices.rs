//! Audio device enumeration (Audacity-style I/O matrix).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDeviceInfo {
    pub id: String,
    pub name: String,
    pub is_input: bool,
    pub is_output: bool,
    pub is_default: bool,
    pub default_sample_rate: Option<u32>,
    pub max_channels: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInventory {
    pub host: String,
    pub inputs: Vec<AudioDeviceInfo>,
    pub outputs: Vec<AudioDeviceInfo>,
}

/// List host audio devices. Empty inventory when `playback` feature is off.
#[cfg(feature = "playback")]
pub fn list_devices() -> DeviceInventory {
    use cpal::traits::{DeviceTrait, HostTrait};

    let host = cpal::default_host();
    let host_name = format!("{:?}", host.id());

    let default_out = host
        .default_output_device()
        .and_then(|d| d.name().ok());
    let default_in = host
        .default_input_device()
        .and_then(|d| d.name().ok());

    let mut outputs = Vec::new();
    if let Ok(devices) = host.output_devices() {
        for (i, d) in devices.enumerate() {
            let name = d.name().unwrap_or_else(|_| format!("Output {i}"));
            let is_default = default_out.as_ref() == Some(&name);
            let (sr, ch) = d
                .default_output_config()
                .map(|c| (Some(c.sample_rate().0), Some(c.channels())))
                .unwrap_or((None, None));
            outputs.push(AudioDeviceInfo {
                id: format!("out:{i}"),
                name,
                is_input: false,
                is_output: true,
                is_default,
                default_sample_rate: sr,
                max_channels: ch,
            });
        }
    }

    let mut inputs = Vec::new();
    if let Ok(devices) = host.input_devices() {
        for (i, d) in devices.enumerate() {
            let name = d.name().unwrap_or_else(|_| format!("Input {i}"));
            let is_default = default_in.as_ref() == Some(&name);
            let (sr, ch) = d
                .default_input_config()
                .map(|c| (Some(c.sample_rate().0), Some(c.channels())))
                .unwrap_or((None, None));
            inputs.push(AudioDeviceInfo {
                id: format!("in:{i}"),
                name,
                is_input: true,
                is_output: false,
                is_default,
                default_sample_rate: sr,
                max_channels: ch,
            });
        }
    }

    DeviceInventory {
        host: host_name,
        inputs,
        outputs,
    }
}

#[cfg(not(feature = "playback"))]
pub fn list_devices() -> DeviceInventory {
    DeviceInventory {
        host: "none".into(),
        inputs: vec![],
        outputs: vec![AudioDeviceInfo {
            id: "null".into(),
            name: "Null (build with --features playback)".into(),
            is_input: false,
            is_output: true,
            is_default: true,
            default_sample_rate: Some(48000),
            max_channels: Some(2),
        }],
    }
}

use std::{borrow::Cow, collections::HashMap};

use libpulse_binding::{
    channelmap::Position,
    context::introspect::{
        SinkInfo as PulseSinkInfo, SinkInputInfo, SourceInfo as PulseSourceInfo, SourceOutputInfo,
    },
    def::{PortAvailable, SinkState, SourceState},
    sample::Format as PulseFormat,
    volume::{ChannelVolumes, Volume as PulseVolume},
};

use crate::{
    types::{
        device::{DeviceInfo, DevicePort, DeviceState, SinkInfo, SourceInfo},
        format::{AudioFormat, ChannelMap, ChannelPosition, SampleFormat, SampleSpec},
        stream::{MediaInfo, StreamInfo, StreamState},
    },
    volume::types::Volume,
};

pub(crate) fn convert_volume_to_pulse(volume: &Volume) -> ChannelVolumes {
    let channels = volume.channels();
    if channels == 0 {
        return ChannelVolumes::default();
    }

    let avg_level = volume.average();
    let pulse_vol = PulseVolume((avg_level * PulseVolume::NORMAL.0 as f64) as u32);

    let mut pulse_volume = ChannelVolumes::default();
    pulse_volume.set(channels as u8, pulse_vol);

    pulse_volume
}

pub(crate) fn convert_volume_from_pulse(pulse_volume: &ChannelVolumes) -> Volume {
    let volumes: Vec<f64> = (0..pulse_volume.len())
        .map(|i| {
            let pulse_vol = pulse_volume.get()[i as usize].0 as f64;
            pulse_vol / PulseVolume::NORMAL.0 as f64
        })
        .collect();

    Volume::new(volumes)
}

fn convert_single_volume_from_pulse(pulse_volume: PulseVolume) -> Volume {
    Volume::new(vec![pulse_volume.0 as f64 / PulseVolume::NORMAL.0 as f64])
}

fn convert_channel_position(position: Position) -> ChannelPosition {
    match position {
        Position::Mono => ChannelPosition::Mono,
        Position::FrontLeft => ChannelPosition::FrontLeft,
        Position::FrontRight => ChannelPosition::FrontRight,
        Position::FrontCenter => ChannelPosition::FrontCenter,
        Position::RearLeft => ChannelPosition::RearLeft,
        Position::RearRight => ChannelPosition::RearRight,
        Position::Lfe => ChannelPosition::LFE,
        Position::SideLeft => ChannelPosition::SideLeft,
        Position::SideRight => ChannelPosition::SideRight,
        _ => ChannelPosition::Unknown,
    }
}

pub(crate) fn convert_sample_format(format: PulseFormat) -> SampleFormat {
    match format {
        PulseFormat::U8 => SampleFormat::U8,
        PulseFormat::S16le => SampleFormat::S16LE,
        PulseFormat::S16be => SampleFormat::S16BE,
        PulseFormat::S24le => SampleFormat::S24LE,
        PulseFormat::S24be => SampleFormat::S24BE,
        PulseFormat::S32le => SampleFormat::S32LE,
        PulseFormat::S32be => SampleFormat::S32BE,
        PulseFormat::F32le => SampleFormat::F32LE,
        PulseFormat::F32be => SampleFormat::F32BE,
        _ => SampleFormat::Unknown,
    }
}

fn cow_str_to_string(cow_str: Option<&Cow<str>>) -> String {
    cow_str.map(|s| s.to_string()).unwrap_or_default()
}

pub(crate) fn create_device_info_from_sink(sink_info: &PulseSinkInfo) -> SinkInfo {
    let volume = convert_volume_from_pulse(&sink_info.volume);
    let name = cow_str_to_string(sink_info.name.as_ref());
    let description = cow_str_to_string(sink_info.description.as_ref());
    let ports: Vec<DevicePort> = sink_info
        .ports
        .iter()
        .map(|port| DevicePort {
            name: cow_str_to_string(port.name.as_ref()),
            description: cow_str_to_string(port.description.as_ref()),
            priority: port.priority,
            available: port.available == PortAvailable::Yes,
        })
        .collect();
    let active_port = sink_info
        .active_port
        .as_ref()
        .and_then(|p| p.name.as_ref().map(|s| s.to_string()));

    let mut properties = HashMap::new();
    for key in sink_info.proplist.iter() {
        if let Some(value) = sink_info.proplist.get_str(&key) {
            properties.insert(key.to_string(), value);
        }
    }

    SinkInfo {
        device: DeviceInfo {
            index: sink_info.index,
            name,
            description,
            card_index: sink_info.card,
            owner_module: sink_info.owner_module,
            driver: cow_str_to_string(sink_info.driver.as_ref()),
            state: if sink_info.state == SinkState::Running {
                DeviceState::Running
            } else if sink_info.state == SinkState::Idle {
                DeviceState::Idle
            } else {
                DeviceState::Suspended
            },
            volume,
            base_volume: convert_single_volume_from_pulse(sink_info.base_volume),
            n_volume_steps: sink_info.n_volume_steps,
            muted: sink_info.mute,
            properties,
            ports,
            active_port,
            formats: sink_info
                .formats
                .iter()
                .map(|f| AudioFormat {
                    encoding: format!("{:?}", f.get_encoding()),
                    properties: {
                        let mut props = HashMap::new();
                        for key in f.get_properties().iter() {
                            if let Some(value) = f.get_properties().get_str(&key) {
                                props.insert(key.to_string(), value);
                            }
                        }
                        props
                    },
                })
                .collect(),
            sample_spec: SampleSpec {
                format: convert_sample_format(sink_info.sample_spec.format),
                rate: sink_info.sample_spec.rate,
                channels: sink_info.sample_spec.channels,
            },
            channel_map: ChannelMap {
                channels: sink_info.channel_map.len(),
                positions: (0..sink_info.channel_map.len())
                    .map(|i| convert_channel_position(sink_info.channel_map.get()[i as usize]))
                    .collect(),
            },
            latency: sink_info.latency,
            configured_latency: sink_info.configured_latency,
            flags: sink_info.flags.bits(),
        },
        monitor_source: sink_info.monitor_source,
        monitor_source_name: cow_str_to_string(sink_info.monitor_source_name.as_ref()),
    }
}

pub(crate) fn create_device_info_from_source(source_info: &PulseSourceInfo) -> SourceInfo {
    let volume = convert_volume_from_pulse(&source_info.volume);
    let name = cow_str_to_string(source_info.name.as_ref());
    let description = cow_str_to_string(source_info.description.as_ref());
    let is_monitor = source_info.monitor_of_sink.is_some();

    let ports: Vec<DevicePort> = source_info
        .ports
        .iter()
        .map(|port| DevicePort {
            name: cow_str_to_string(port.name.as_ref()),
            description: cow_str_to_string(port.description.as_ref()),
            priority: port.priority,
            available: port.available == PortAvailable::Yes,
        })
        .collect();
    let active_port = source_info
        .active_port
        .as_ref()
        .and_then(|p| p.name.as_ref().map(|s| s.to_string()));

    let mut properties = HashMap::new();
    for key in source_info.proplist.iter() {
        if let Some(value) = source_info.proplist.get_str(&key) {
            properties.insert(key.to_string(), value);
        }
    }

    SourceInfo {
        device: DeviceInfo {
            index: source_info.index,
            name,
            description,
            card_index: source_info.card,
            owner_module: source_info.owner_module,
            driver: cow_str_to_string(source_info.driver.as_ref()),
            state: if source_info.state == SourceState::Running {
                DeviceState::Running
            } else if source_info.state == SourceState::Idle {
                DeviceState::Idle
            } else {
                DeviceState::Suspended
            },
            volume,
            base_volume: convert_single_volume_from_pulse(source_info.base_volume),
            n_volume_steps: source_info.n_volume_steps,
            muted: source_info.mute,
            properties,
            ports,
            active_port,
            formats: source_info
                .formats
                .iter()
                .map(|f| AudioFormat {
                    encoding: format!("{:?}", f.get_encoding()),
                    properties: {
                        let mut props = HashMap::new();
                        for key in f.get_properties().iter() {
                            if let Some(value) = f.get_properties().get_str(&key) {
                                props.insert(key.to_string(), value);
                            }
                        }
                        props
                    },
                })
                .collect(),
            sample_spec: SampleSpec {
                format: convert_sample_format(source_info.sample_spec.format),
                rate: source_info.sample_spec.rate,
                channels: source_info.sample_spec.channels,
            },
            channel_map: ChannelMap {
                channels: source_info.channel_map.len(),
                positions: (0..source_info.channel_map.len())
                    .map(|i| convert_channel_position(source_info.channel_map.get()[i as usize]))
                    .collect(),
            },
            latency: source_info.latency,
            configured_latency: source_info.configured_latency,
            flags: source_info.flags.bits(),
        },
        monitor_of_sink: source_info.monitor_of_sink,
        monitor_of_sink_name: if source_info.monitor_of_sink.is_some() {
            Some(cow_str_to_string(source_info.monitor_of_sink_name.as_ref()))
        } else {
            None
        },
        is_monitor,
    }
}

pub(crate) fn create_stream_info_from_sink_input(sink_input_info: &SinkInputInfo) -> StreamInfo {
    let volume = convert_volume_from_pulse(&sink_input_info.volume);
    let name = sink_input_info.name.clone().unwrap_or_default().to_string();

    let application_name = sink_input_info.proplist.get_str("application.name");
    let binary = sink_input_info.proplist.get_str("application.binary");
    let pid = sink_input_info
        .proplist
        .get_str("application.process.id")
        .and_then(|s| s.parse::<u32>().ok());

    let media = MediaInfo {
        title: sink_input_info.proplist.get_str("media.title"),
        artist: sink_input_info.proplist.get_str("media.artist"),
        album: sink_input_info.proplist.get_str("media.album"),
        icon_name: sink_input_info.proplist.get_str("application.icon_name"),
    };

    let mut properties = HashMap::new();
    for key in sink_input_info.proplist.iter() {
        if let Some(value) = sink_input_info.proplist.get_str(&key) {
            properties.insert(key.to_string(), value);
        }
    }

    StreamInfo {
        index: sink_input_info.index,
        name,
        application_name,
        binary,
        pid,
        owner_module: sink_input_info.owner_module,
        client: sink_input_info.client,
        device_index: sink_input_info.sink,
        volume,
        muted: sink_input_info.mute,
        corked: sink_input_info.corked,
        has_volume: sink_input_info.has_volume,
        volume_writable: sink_input_info.volume_writable,
        state: StreamState::Running,
        sample_spec: SampleSpec {
            format: convert_sample_format(sink_input_info.sample_spec.format),
            rate: sink_input_info.sample_spec.rate,
            channels: sink_input_info.sample_spec.channels,
        },
        channel_map: ChannelMap {
            channels: sink_input_info.channel_map.len(),
            positions: (0..sink_input_info.channel_map.len())
                .map(|i| convert_channel_position(sink_input_info.channel_map.get()[i as usize]))
                .collect(),
        },
        properties,
        media,
        buffer_latency: sink_input_info.buffer_usec.0,
        device_latency: sink_input_info.sink_usec.0,
        resample_method: sink_input_info
            .resample_method
            .as_ref()
            .map(|s| s.to_string()),
        driver: sink_input_info
            .driver
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_default(),
        format: Some(format!("{:?}", sink_input_info.format.get_encoding())),
    }
}

pub(crate) fn create_stream_info_from_source_output(
    source_output_info: &SourceOutputInfo,
) -> StreamInfo {
    let volume = convert_volume_from_pulse(&source_output_info.volume);
    let name = source_output_info
        .name
        .clone()
        .unwrap_or_default()
        .to_string();

    let application_name = source_output_info.proplist.get_str("application.name");
    let binary = source_output_info.proplist.get_str("application.binary");
    let pid = source_output_info
        .proplist
        .get_str("application.process.id")
        .and_then(|s| s.parse::<u32>().ok());

    let media = MediaInfo {
        title: source_output_info.proplist.get_str("media.title"),
        artist: source_output_info.proplist.get_str("media.artist"),
        album: source_output_info.proplist.get_str("media.album"),
        icon_name: source_output_info.proplist.get_str("application.icon_name"),
    };

    let mut properties = HashMap::new();
    for key in source_output_info.proplist.iter() {
        if let Some(value) = source_output_info.proplist.get_str(&key) {
            properties.insert(key.to_string(), value);
        }
    }

    StreamInfo {
        index: source_output_info.index,
        name,
        application_name,
        binary,
        pid,
        owner_module: source_output_info.owner_module,
        client: source_output_info.client,
        device_index: source_output_info.source,
        volume,
        muted: source_output_info.mute,
        corked: source_output_info.corked,
        has_volume: source_output_info.has_volume,
        volume_writable: source_output_info.volume_writable,
        state: StreamState::Running,
        sample_spec: SampleSpec {
            format: convert_sample_format(source_output_info.sample_spec.format),
            rate: source_output_info.sample_spec.rate,
            channels: source_output_info.sample_spec.channels,
        },
        channel_map: ChannelMap {
            channels: source_output_info.channel_map.len(),
            positions: (0..source_output_info.channel_map.len())
                .map(|i| convert_channel_position(source_output_info.channel_map.get()[i as usize]))
                .collect(),
        },
        properties,
        media,
        buffer_latency: source_output_info.buffer_usec.0,
        device_latency: source_output_info.source_usec.0,
        resample_method: source_output_info
            .resample_method
            .as_ref()
            .map(|s| s.to_string()),
        driver: source_output_info
            .driver
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_default(),
        format: Some(format!("{:?}", source_output_info.format.get_encoding())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_volume_to_pulse_with_empty_channels_returns_default() {
        let volume = Volume::new(vec![]);

        let result = convert_volume_to_pulse(&volume);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn convert_volume_to_pulse_converts_normal_volume_correctly() {
        let volume = Volume::mono(1.0);

        let result = convert_volume_to_pulse(&volume);

        assert_eq!(result.len(), 1);
        assert_eq!(result.avg(), PulseVolume::NORMAL);
    }

    #[test]
    fn convert_volume_to_pulse_converts_zero_volume_to_muted() {
        let volume = Volume::mono(0.0);

        let result = convert_volume_to_pulse(&volume);

        assert_eq!(result.len(), 1);
        assert_eq!(result.avg(), PulseVolume::MUTED);
    }

    #[test]
    fn convert_volume_to_pulse_converts_max_volume_correctly() {
        let volume = Volume::mono(4.0);

        let result = convert_volume_to_pulse(&volume);

        let expected_value = PulseVolume((4.0 * PulseVolume::NORMAL.0 as f64) as u32);
        assert_eq!(result.avg(), expected_value);
    }

    #[test]
    fn convert_volume_from_pulse_converts_normal_volume_correctly() {
        let mut pulse_volume = ChannelVolumes::default();
        pulse_volume.set(1, PulseVolume::NORMAL);

        let result = convert_volume_from_pulse(&pulse_volume);

        assert_eq!(result.channels(), 1);
        assert_eq!(result.channel(0), Some(1.0));
    }

    #[test]
    fn convert_volume_from_pulse_converts_muted_to_zero() {
        let mut pulse_volume = ChannelVolumes::default();
        pulse_volume.set(1, PulseVolume::MUTED);

        let result = convert_volume_from_pulse(&pulse_volume);

        assert_eq!(result.channels(), 1);
        assert_eq!(result.channel(0), Some(0.0));
    }

    #[test]
    fn convert_volume_from_pulse_converts_max_volume_correctly() {
        let mut pulse_volume = ChannelVolumes::default();
        let max_pulse_vol = PulseVolume((4.0 * PulseVolume::NORMAL.0 as f64) as u32);
        pulse_volume.set(1, max_pulse_vol);

        let result = convert_volume_from_pulse(&pulse_volume);

        assert_eq!(result.channels(), 1);
        let channel_vol = result.channel(0).unwrap();
        assert!((channel_vol - 4.0).abs() < 0.01);
    }

    #[test]
    fn convert_volume_from_pulse_handles_multi_channel() {
        let mut pulse_volume = ChannelVolumes::default();
        pulse_volume.set(2, PulseVolume::NORMAL);

        let result = convert_volume_from_pulse(&pulse_volume);

        assert_eq!(result.channels(), 2);
        assert_eq!(result.channel(0), Some(1.0));
        assert_eq!(result.channel(1), Some(1.0));
    }

    #[test]
    fn convert_sample_format_converts_u8_correctly() {
        let result = convert_sample_format(PulseFormat::U8);
        assert_eq!(result, SampleFormat::U8);
    }

    #[test]
    fn convert_sample_format_converts_s16le_correctly() {
        let result = convert_sample_format(PulseFormat::S16le);
        assert_eq!(result, SampleFormat::S16LE);
    }

    #[test]
    fn convert_sample_format_converts_s16be_correctly() {
        let result = convert_sample_format(PulseFormat::S16be);
        assert_eq!(result, SampleFormat::S16BE);
    }

    #[test]
    fn convert_sample_format_converts_unknown_to_unknown() {
        let result = convert_sample_format(PulseFormat::Invalid);
        assert_eq!(result, SampleFormat::Unknown);
    }

    #[test]
    fn create_device_info_from_sink_maps_running_state_correctly() {
        let sink = create_minimal_pulse_sink(SinkState::Running);

        let result = create_device_info_from_sink(&sink);

        assert_eq!(result.device.state, DeviceState::Running);
    }

    #[test]
    fn create_device_info_from_sink_maps_idle_state_correctly() {
        let sink = create_minimal_pulse_sink(SinkState::Idle);

        let result = create_device_info_from_sink(&sink);

        assert_eq!(result.device.state, DeviceState::Idle);
    }

    #[test]
    fn create_device_info_from_sink_maps_suspended_state_to_suspended() {
        let sink = create_minimal_pulse_sink(SinkState::Suspended);

        let result = create_device_info_from_sink(&sink);

        assert_eq!(result.device.state, DeviceState::Suspended);
    }

    fn create_minimal_pulse_sink(state: SinkState) -> PulseSinkInfo<'static> {
        let mut channel_map = libpulse_binding::channelmap::Map::default();
        channel_map.init_stereo();

        PulseSinkInfo {
            name: Some("test-sink".into()),
            index: 0,
            description: Some("Test Sink".into()),
            sample_spec: libpulse_binding::sample::Spec {
                format: PulseFormat::S16le,
                channels: 2,
                rate: 44100,
            },
            channel_map,
            owner_module: None,
            volume: ChannelVolumes::default(),
            mute: false,
            monitor_source: 1,
            monitor_source_name: Some("test-sink.monitor".into()),
            latency: libpulse_binding::time::MicroSeconds(0),
            driver: Some("test-driver".into()),
            flags: libpulse_binding::def::SinkFlagSet::empty(),
            proplist: libpulse_binding::proplist::Proplist::new().unwrap(),
            configured_latency: libpulse_binding::time::MicroSeconds(0),
            base_volume: PulseVolume::NORMAL,
            state,
            n_volume_steps: 65536,
            card: None,
            ports: vec![],
            active_port: None,
            formats: vec![],
        }
    }
}

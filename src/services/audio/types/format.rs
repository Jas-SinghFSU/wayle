use std::collections::HashMap;

/// Sample specification
#[derive(Debug, Clone, PartialEq)]
pub struct SampleSpec {
    /// Sample rate in Hz
    pub rate: u32,
    /// Number of channels
    pub channels: u8,
    /// Sample format
    pub format: SampleFormat,
}

/// Sample format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleFormat {
    /// Unsigned 8-bit samples.
    U8,
    /// Signed 16-bit little-endian samples.
    S16LE,
    /// Signed 16-bit big-endian samples.
    S16BE,
    /// Signed 24-bit little-endian samples.
    S24LE,
    /// Signed 24-bit big-endian samples.
    S24BE,
    /// Signed 32-bit little-endian samples.
    S32LE,
    /// Signed 32-bit big-endian samples.
    S32BE,
    /// Float 32-bit little-endian samples.
    F32LE,
    /// Float 32-bit big-endian samples.
    F32BE,
    /// Unknown format.
    Unknown,
}

/// Channel map for audio channels
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelMap {
    /// Number of channels
    pub channels: u8,
    /// Channel positions
    pub positions: Vec<ChannelPosition>,
}

/// Channel position enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelPosition {
    /// Mono channel.
    Mono,
    /// Front left channel.
    FrontLeft,
    /// Front right channel.
    FrontRight,
    /// Front center channel.
    FrontCenter,
    /// Rear left channel.
    RearLeft,
    /// Rear right channel.
    RearRight,
    /// Low frequency effects channel (subwoofer).
    LFE,
    /// Side left channel.
    SideLeft,
    /// Side right channel.
    SideRight,
    /// Unknown channel position.
    Unknown,
}

/// Audio format information
#[derive(Debug, Clone, PartialEq)]
pub struct AudioFormat {
    /// Encoding type
    pub encoding: String,
    /// Properties of the format
    pub properties: HashMap<String, String>,
}

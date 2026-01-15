use crate::ffi;

/// Audio input method for capturing system audio.
///
/// Specifies which audio backend CAVA should use to capture audio data for visualization.
/// Different input methods are available depending on the platform and installed audio systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethod {
    /// Read audio from a named pipe (FIFO).
    ///
    /// Requires an application to write PCM audio data to the FIFO.
    /// Common with MPD and other music players.
    Fifo,

    /// PortAudio cross-platform audio I/O library.
    ///
    /// Works on Linux, macOS, and Windows. Provides automatic device selection.
    PortAudio,

    /// PipeWire multimedia server (default).
    ///
    /// Modern Linux audio system that supersedes PulseAudio and JACK.
    /// The default input method for most Linux desktop systems.
    PipeWire,

    /// Advanced Linux Sound Architecture (ALSA).
    ///
    /// Low-level Linux audio API. Requires specific device configuration.
    Alsa,

    /// PulseAudio sound server.
    ///
    /// Common on older Linux desktop systems. Works with automatic source detection.
    Pulse,

    /// sndio audio subsystem.
    ///
    /// Primarily used on OpenBSD and other BSD systems.
    Sndio,

    /// Open Sound System.
    ///
    /// Legacy Unix audio API, less common on modern systems.
    Oss,

    /// JACK Audio Connection Kit.
    ///
    /// Professional audio system for low-latency audio routing.
    Jack,

    /// Read audio from shared memory.
    ///
    /// Used by applications like Squeezelite that write directly to shared memory.
    Shmem,

    /// Windows audio capture (WASAPI).
    ///
    /// Windows-specific audio capture method.
    Winscap,
}

impl From<InputMethod> for ffi::InputMethod {
    fn from(method: InputMethod) -> Self {
        match method {
            InputMethod::Fifo => ffi::InputMethod::Fifo,
            InputMethod::PortAudio => ffi::InputMethod::PortAudio,
            InputMethod::PipeWire => ffi::InputMethod::PipeWire,
            InputMethod::Alsa => ffi::InputMethod::Alsa,
            InputMethod::Pulse => ffi::InputMethod::Pulse,
            InputMethod::Sndio => ffi::InputMethod::Sndio,
            InputMethod::Oss => ffi::InputMethod::Oss,
            InputMethod::Jack => ffi::InputMethod::Jack,
            InputMethod::Shmem => ffi::InputMethod::Shmem,
            InputMethod::Winscap => ffi::InputMethod::Winscap,
        }
    }
}

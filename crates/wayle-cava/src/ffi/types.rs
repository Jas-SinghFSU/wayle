#![allow(unsafe_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(unsafe_op_in_unsafe_fn)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub(super) type CavaPlan = cava_plan;
pub(super) type ConfigParams = config_params;
pub(super) type AudioData = audio_data;
pub(super) type AudioRaw = audio_raw;
pub(super) type InputFn =
    unsafe extern "C" fn(arg1: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputMethod {
    Fifo,
    PortAudio,
    PipeWire,
    Alsa,
    Pulse,
    Sndio,
    Oss,
    Jack,
    Shmem,
    Winscap,
}

impl From<InputMethod> for input_method {
    fn from(method: InputMethod) -> Self {
        match method {
            InputMethod::Fifo => input_method_INPUT_FIFO,
            InputMethod::PortAudio => input_method_INPUT_PORTAUDIO,
            InputMethod::PipeWire => input_method_INPUT_PIPEWIRE,
            InputMethod::Alsa => input_method_INPUT_ALSA,
            InputMethod::Pulse => input_method_INPUT_PULSE,
            InputMethod::Sndio => input_method_INPUT_SNDIO,
            InputMethod::Oss => input_method_INPUT_OSS,
            InputMethod::Jack => input_method_INPUT_JACK,
            InputMethod::Shmem => input_method_INPUT_SHMEM,
            InputMethod::Winscap => input_method_INPUT_WINSCAP,
        }
    }
}

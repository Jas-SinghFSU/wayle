#![allow(dead_code)]
use std::os::raw::{c_char, c_double, c_int, c_uint};

#[repr(C)]
#[derive(Debug)]
pub(crate) struct FftwPlan {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct FftwComplex {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct CavaPlan {
    pub fft_bass_buffer_size: c_int,
    pub fft_mid_buffer_size: c_int,
    pub fft_treble_buffer_size: c_int,
    pub number_of_bars: c_int,
    pub audio_channels: c_int,
    pub input_buffer_size: c_int,
    pub rate: c_int,
    pub bass_cut_off_bar: c_int,
    pub treble_cut_off_bar: c_int,
    pub sens_init: c_int,
    pub autosens: c_int,
    pub frame_skip: c_int,
    pub status: c_int,
    pub error_message: [c_char; 1024],

    pub sens: c_double,
    pub framerate: c_double,
    pub noise_reduction: c_double,

    pub p_bass_l: *mut FftwPlan,
    pub p_bass_r: *mut FftwPlan,
    pub p_mid_l: *mut FftwPlan,
    pub p_mid_r: *mut FftwPlan,
    pub p_treble_l: *mut FftwPlan,
    pub p_treble_r: *mut FftwPlan,

    pub out_bass_l: *mut FftwComplex,
    pub out_bass_r: *mut FftwComplex,
    pub out_mid_l: *mut FftwComplex,
    pub out_mid_r: *mut FftwComplex,
    pub out_treble_l: *mut FftwComplex,
    pub out_treble_r: *mut FftwComplex,

    pub bass_multiplier: *mut c_double,
    pub mid_multiplier: *mut c_double,
    pub treble_multiplier: *mut c_double,

    pub in_bass_r_raw: *mut c_double,
    pub in_bass_l_raw: *mut c_double,
    pub in_mid_r_raw: *mut c_double,
    pub in_mid_l_raw: *mut c_double,
    pub in_treble_r_raw: *mut c_double,
    pub in_treble_l_raw: *mut c_double,
    pub in_bass_r: *mut c_double,
    pub in_bass_l: *mut c_double,
    pub in_mid_r: *mut c_double,
    pub in_mid_l: *mut c_double,
    pub in_treble_r: *mut c_double,
    pub in_treble_l: *mut c_double,
    pub prev_cava_out: *mut c_double,
    pub cava_mem: *mut c_double,
    pub input_buffer: *mut c_double,
    pub cava_peak: *mut c_double,

    pub eq: *mut c_double,

    pub cut_off_frequency: *mut f32,
    pub fft_buffer_lower_cut_off: *mut c_int,
    pub fft_buffer_upper_cut_off: *mut c_int,
    pub cava_fall: *mut c_double,
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct AudioData {
    pub cava_in: *mut c_double,

    pub input_buffer_size: c_int,
    pub cava_buffer_size: c_int,

    pub format: c_int,
    pub rate: c_uint,
    pub channels: c_uint,
    pub threadparams: c_int,
    pub source: *mut c_char,
    pub im: c_int,
    pub terminate: c_int,
    pub error_message: [c_char; 1024],
    pub samples_counter: c_int,
    pub ieee_float: c_int,
    pub autoconnect: c_int,
    pub lock: libc::pthread_mutex_t,
    pub resume_cond: libc::pthread_cond_t,
    pub suspend_flag: bool,
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct AudioRaw {
    pub bars: *mut c_int,
    pub previous_frame: *mut c_int,
    pub bars_left: *mut f32,
    pub bars_right: *mut f32,
    pub bars_raw: *mut f32,
    pub previous_bars_raw: *mut f32,
    pub cava_out: *mut c_double,
    pub dimension_bar: *mut c_int,
    pub dimension_value: *mut c_int,
    pub user_eq_keys_to_bars_ratio: c_double,
    pub channels: c_int,
    pub number_of_bars: c_int,
    pub output_channels: c_int,
    pub height: c_int,
    pub lines: c_int,
    pub width: c_int,
    pub remainder: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum InputMethod {
    Fifo = 0,
    PortAudio = 1,
    PipeWire = 2,
    Alsa = 3,
    Pulse = 4,
    Sndio = 5,
    Oss = 6,
    Jack = 7,
    Shmem = 8,
    Winscap = 9,
    Max = 10,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum OutputMethod {
    Ncurses = 0,
    Noncurses = 1,
    Raw = 2,
    Sdl = 3,
    SdlGlsl = 4,
    Noritake = 5,
    NotSupported = 6,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum MonoOption {
    Left = 0,
    Right = 1,
    Average = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum DataFormat {
    Ascii = 0,
    Binary = 1,
    Ntk3000 = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum XAxisScale {
    None = 0,
    Frequency = 1,
    Note = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum Orientation {
    Bottom = 0,
    Top = 1,
    Left = 2,
    Right = 3,
    SplitH = 4,
    SplitV = 5,
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct ConfigParams {
    pub color: *mut c_char,
    pub bcolor: *mut c_char,
    pub raw_target: *mut c_char,
    pub audio_source: *mut c_char,
    pub gradient_colors: *mut *mut c_char,
    pub horizontal_gradient_colors: *mut *mut c_char,
    pub data_format: *mut c_char,
    pub vertex_shader: *mut c_char,
    pub fragment_shader: *mut c_char,

    pub bar_delim: c_char,
    pub frame_delim: c_char,
    pub monstercat: c_double,
    pub integral: c_double,
    pub gravity: c_double,
    pub ignore: c_double,
    pub sens: c_double,
    pub noise_reduction: c_double,
    pub lower_cut_off: c_uint,
    pub upper_cut_off: c_uint,
    pub user_eq: *mut c_double,
    pub input: InputMethod,
    pub output: OutputMethod,
    pub xaxis: XAxisScale,
    pub mono_opt: MonoOption,
    pub orientation: Orientation,
    pub blend_direction: Orientation,
    pub user_eq_keys: c_int,
    pub user_eq_enabled: c_int,
    pub col: c_int,
    pub bgcol: c_int,
    pub autobars: c_int,
    pub stereo: c_int,
    pub raw_format: c_int,
    pub ascii_range: c_int,
    pub bit_format: c_int,
    pub gradient: c_int,
    pub gradient_count: c_int,
    pub horizontal_gradient: c_int,
    pub horizontal_gradient_count: c_int,
    pub fixedbars: c_int,
    pub framerate: c_int,
    pub bar_width: c_int,
    pub bar_spacing: c_int,
    pub bar_height: c_int,
    pub autosens: c_int,
    pub overshoot: c_int,
    pub waves: c_int,
    pub samplerate: c_int,
    pub samplebits: c_int,
    pub channels: c_int,
    pub autoconnect: c_int,
    pub sleep_timer: c_int,
    pub sdl_width: c_int,
    pub sdl_height: c_int,
    pub sdl_x: c_int,
    pub sdl_y: c_int,
    pub sdl_full_screen: c_int,
    pub draw_and_quit: c_int,
    pub zero_test: c_int,
    pub non_zero_test: c_int,
    pub reverse: c_int,
    pub sync_updates: c_int,
    pub continuous_rendering: c_int,
    pub disable_blanking: c_int,
    pub show_idle_bar_heads: c_int,
    pub waveform: c_int,
    pub in_atty: c_int,
    pub fp: c_int,
    pub x_axis_info: c_int,
}

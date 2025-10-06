#![allow(unsafe_code)]

use std::{ffi, pin::Pin, ptr};

use ffi::CString;

use super::super::types;
use crate::Result;

pub struct Config {
    inner: Pin<Box<types::ConfigParams>>,
    _raw_target: CString,
    _data_format: CString,
    _audio_source: CString,
}

impl Config {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bars: usize,
        autosens: bool,
        stereo: bool,
        noise_reduction: f64,
        framerate: u32,
        input: types::InputMethod,
        channels: u32,
        samplerate: u32,
        low_cutoff: u32,
        high_cutoff: u32,
        audio_source: &str,
    ) -> Result<Self> {
        let raw_target = CString::new("/dev/stdout")?;
        let data_format = CString::new("binary")?;
        let audio_source_str = CString::new(audio_source)?;

        let config = Box::new(types::ConfigParams {
            color: ptr::null_mut(),
            bcolor: ptr::null_mut(),
            raw_target: raw_target.as_ptr() as *mut _,
            audio_source: audio_source_str.as_ptr() as *mut _,
            gradient_colors: ptr::null_mut(),
            horizontal_gradient_colors: ptr::null_mut(),
            data_format: data_format.as_ptr() as *mut _,
            vertex_shader: ptr::null_mut(),
            fragment_shader: ptr::null_mut(),

            bar_delim: b';' as i8,
            frame_delim: b'\n' as i8,
            monstercat: 0.0,
            integral: 0.0,
            gravity: 0.0,
            ignore: 0.0,
            sens: 1.0,
            noise_reduction,
            lower_cut_off: low_cutoff,
            upper_cut_off: high_cutoff,
            user_eq: ptr::null_mut(),
            input,
            output: types::OutputMethod::Raw,
            xaxis: types::XAxisScale::None,
            mono_opt: types::MonoOption::Average,
            orientation: types::Orientation::Bottom,
            blend_direction: types::Orientation::Bottom,
            user_eq_keys: 0,
            user_eq_enabled: 0,
            col: 0,
            bgcol: 0,
            autobars: 0,
            stereo: stereo as i32,
            raw_format: 1,
            ascii_range: 1000,
            bit_format: 16,
            gradient: 0,
            gradient_count: 0,
            horizontal_gradient: 0,
            horizontal_gradient_count: 0,
            fixedbars: bars as i32,
            framerate: framerate as i32,
            bar_width: 2,
            bar_spacing: 1,
            bar_height: 32,
            autosens: autosens as i32,
            overshoot: 0,
            waves: 0,
            samplerate: samplerate as i32,
            samplebits: 16,
            channels: channels as i32,
            autoconnect: 2,
            sleep_timer: 0,
            sdl_width: 1000,
            sdl_height: 500,
            sdl_x: -1,
            sdl_y: -1,
            sdl_full_screen: 0,
            draw_and_quit: 0,
            zero_test: 0,
            non_zero_test: 0,
            reverse: 0,
            sync_updates: 0,
            continuous_rendering: 0,
            disable_blanking: 1,
            show_idle_bar_heads: 1,
            waveform: 0,
            in_atty: 0,
            fp: 0,
            x_axis_info: 0,
        });

        Ok(Self {
            inner: Pin::new(config),
            _raw_target: raw_target,
            _data_format: data_format,
            _audio_source: audio_source_str,
        })
    }

    pub(crate) fn as_ptr(&mut self) -> *mut types::ConfigParams {
        &mut *self.inner as *mut _
    }
}

unsafe impl Send for Config {}
unsafe impl Sync for Config {}

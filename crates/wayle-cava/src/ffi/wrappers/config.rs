#![allow(unsafe_code)]

use std::{ffi::CString, pin::Pin, ptr};

use super::super::types::{
    self, config_params, input_method, mono_option_AVERAGE, orientation_ORIENT_BOTTOM,
    output_method_OUTPUT_RAW, xaxis_scale_NONE,
};
use crate::Result;

pub(crate) struct Config {
    inner: Pin<Box<config_params>>,
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

        let config = Box::new(config_params {
            color: ptr::null_mut(),
            bcolor: ptr::null_mut(),
            raw_target: raw_target.as_ptr() as *mut _,
            audio_source: audio_source_str.as_ptr() as *mut _,
            gradient_colors: ptr::null_mut(),
            horizontal_gradient_colors: ptr::null_mut(),
            data_format: data_format.as_ptr() as *mut _,
            vertex_shader: ptr::null_mut(),
            fragment_shader: ptr::null_mut(),
            theme: ptr::null_mut(),

            bar_delim: b';' as i8,
            frame_delim: b'\n' as i8,
            monstercat: 0.0,
            integral: 0.0,
            gravity: 0.0,
            ignore: 0.0,
            sens: 1.0,
            noise_reduction,
            max_height: 0.0,
            lower_cut_off: low_cutoff,
            upper_cut_off: high_cutoff,
            userEQ: ptr::null_mut(),
            input: input_method::from(input),
            output: output_method_OUTPUT_RAW,
            xaxis: xaxis_scale_NONE,
            mono_opt: mono_option_AVERAGE,
            orientation: orientation_ORIENT_BOTTOM,
            blendDirection: orientation_ORIENT_BOTTOM,
            userEQ_keys: 0,
            userEQ_enabled: 0,
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
            active: 0,
            remix: 0,
            virtual_: 0,
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
            center_align: 0,
            inAtty: 0,
            inAterminal: 0,
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

    pub(crate) fn as_ptr(&mut self) -> *mut config_params {
        &mut *self.inner as *mut _
    }
}

unsafe impl Send for Config {}

unsafe impl Sync for Config {}

#![allow(unsafe_code)]

use std::{pin::Pin, ptr, slice};

use super::{
    super::{bindings, types},
    AudioInput, Config, Plan,
};
use crate::{Error, Result};

pub struct AudioOutput {
    pub(super) inner: Pin<Box<types::AudioRaw>>,
}

impl AudioOutput {
    pub fn new(bars: usize) -> Self {
        let audio_raw = Box::new(types::AudioRaw {
            bars: ptr::null_mut(),
            previous_frame: ptr::null_mut(),
            bars_left: ptr::null_mut(),
            bars_right: ptr::null_mut(),
            bars_raw: ptr::null_mut(),
            previous_bars_raw: ptr::null_mut(),
            cava_out: ptr::null_mut(),
            dimension_bar: ptr::null_mut(),
            dimension_value: ptr::null_mut(),
            user_eq_keys_to_bars_ratio: 0.0,
            channels: 0,
            number_of_bars: bars as i32,
            output_channels: 0,
            height: 0,
            lines: 0,
            width: 0,
            remainder: 0,
        });

        Self {
            inner: Pin::new(audio_raw),
        }
    }

    pub fn init(
        &mut self,
        audio_input: &mut AudioInput,
        config: &mut Config,
        plan: &Plan,
    ) -> Result<()> {
        let ret = unsafe {
            bindings::audio_raw_init(
                audio_input.as_ptr(),
                self.as_ptr(),
                config.as_ptr(),
                plan.as_ptr(),
            )
        };

        if ret != 0 {
            return Err(Error::AudioRawInitFailed(ret));
        }

        Ok(())
    }

    pub(crate) fn as_ptr(&mut self) -> *mut types::AudioRaw {
        &mut *self.inner as *mut _
    }

    pub fn values(&self) -> &[f64] {
        unsafe {
            let output_data = self.inner.as_ref().get_ref();
            slice::from_raw_parts(output_data.cava_out, output_data.number_of_bars as usize)
        }
    }
}

impl Drop for AudioOutput {
    fn drop(&mut self) {
        unsafe {
            bindings::audio_raw_clean(self.as_ptr());
        }
    }
}

unsafe impl Send for AudioOutput {}
unsafe impl Sync for AudioOutput {}

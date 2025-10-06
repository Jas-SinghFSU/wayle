#![allow(unsafe_code)]

use std::{ffi, ptr};

use ptr::NonNull;

use super::{
    super::{bindings, types},
    AudioInput, AudioOutput,
};
use crate::{Error, Result};

pub struct Plan {
    ptr: NonNull<types::CavaPlan>,
}

impl Plan {
    pub fn new(
        bars: usize,
        samplerate: u32,
        channels: u32,
        autosens: bool,
        noise_reduction: f64,
        low_cutoff: u32,
        high_cutoff: u32,
    ) -> Result<Self> {
        let ptr = unsafe {
            bindings::cava_init(
                bars as i32,
                samplerate,
                channels as i32,
                autosens as i32,
                noise_reduction,
                low_cutoff as i32,
                high_cutoff as i32,
            )
        };

        let ptr = NonNull::new(ptr).ok_or(Error::NullPlan)?;

        let wrapper = Self { ptr };

        if wrapper.status() != 0 {
            let msg = wrapper
                .error_message()
                .unwrap_or_else(|| "Unknown error".to_string());
            return Err(Error::InitFailed(msg));
        }

        Ok(wrapper)
    }

    pub fn status(&self) -> i32 {
        unsafe { (*self.ptr.as_ptr()).status }
    }

    pub fn error_message(&self) -> Option<String> {
        if self.status() != 0 {
            unsafe {
                let msg_ptr = (*self.ptr.as_ptr()).error_message.as_ptr();
                let c_str = ffi::CStr::from_ptr(msg_ptr);
                return Some(c_str.to_string_lossy().into_owned());
            }
        }

        None
    }

    pub(crate) fn as_ptr(&self) -> *mut types::CavaPlan {
        self.ptr.as_ptr()
    }

    pub fn execute(&self, audio_input: &AudioInput, audio_output: &AudioOutput) {
        unsafe {
            let input_data = audio_input.inner.as_ref().get_ref();
            let output_data = audio_output.inner.as_ref().get_ref();

            bindings::cava_execute(
                input_data.cava_in,
                input_data.samples_counter,
                output_data.cava_out,
                self.ptr.as_ptr(),
            );
        }
    }
}

impl Drop for Plan {
    fn drop(&mut self) {
        unsafe {
            bindings::cava_destroy(self.ptr.as_ptr());
        }
    }
}

unsafe impl Send for Plan {}
unsafe impl Sync for Plan {}

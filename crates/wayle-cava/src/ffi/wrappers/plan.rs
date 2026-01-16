#![allow(unsafe_code)]

use std::{ffi, ptr::NonNull};

use super::{
    super::types::{cava_destroy, cava_execute, cava_init, cava_plan},
    AudioInput, AudioOutput,
};
use crate::{Error, Result};

pub(crate) struct Plan {
    ptr: NonNull<cava_plan>,
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
        // SAFETY: cava_init allocates and initializes a cava_plan struct.
        // It returns a valid pointer on success, null on failure.
        let ptr = unsafe {
            cava_init(
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

    fn status(&self) -> i32 {
        // SAFETY: ptr is valid and points to an initialized cava_plan.
        unsafe { (*self.ptr.as_ptr()).status }
    }

    fn error_message(&self) -> Option<String> {
        if self.status() != 0 {
            // SAFETY: error_message is a fixed-size array initialized by cava_init.
            // It contains a null-terminated string on error.
            unsafe {
                let msg_ptr = (*self.ptr.as_ptr()).error_message.as_ptr();
                let c_str = ffi::CStr::from_ptr(msg_ptr);
                return Some(c_str.to_string_lossy().into_owned());
            }
        }

        None
    }

    pub(crate) fn as_ptr(&self) -> *mut cava_plan {
        self.ptr.as_ptr()
    }

    pub fn execute(&self, audio_input: &AudioInput, audio_output: &AudioOutput) {
        // SAFETY: All pointers are valid and point to initialized structs.
        // cava_execute reads from cava_in and writes to cava_out.
        unsafe {
            let input_data = audio_input.inner.as_ref().get_ref();
            let output_data = audio_output.inner.as_ref().get_ref();

            cava_execute(
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
        // SAFETY: ptr was created by cava_init and is valid.
        // cava_destroy frees all resources associated with the plan.
        unsafe {
            cava_destroy(self.ptr.as_ptr());
        }
    }
}

unsafe impl Send for Plan {}

unsafe impl Sync for Plan {}

#![allow(unsafe_code)]

use std::ptr::NonNull;

use super::{
    super::types::{cava_destroy, cava_execute, cava_plan},
    AudioInput, AudioOutput,
};

/// FFT execution plan wrapping `cava_plan`.
///
/// Created by [`AudioOutput::init`] which delegates to `audio_raw_init`
/// for correct bar count adjustment (stereo halves the per-channel count).
pub(crate) struct Plan {
    ptr: NonNull<cava_plan>,
}

impl Plan {
    /// Takes ownership of a `cava_plan` pointer allocated by `cava_init`.
    ///
    /// `cava_destroy` is called on drop.
    pub(crate) fn from_raw(ptr: NonNull<cava_plan>) -> Self {
        Self { ptr }
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

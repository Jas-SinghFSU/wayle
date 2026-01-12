#![allow(unsafe_code)]

use std::{ffi, ptr::NonNull};

use super::{
    super::types::{cava_destroy, cava_execute, cava_init, cava_plan},
    AudioInput, AudioOutput,
};
use crate::{Error, Result};

/// Safe wrapper around libcava's cava_plan struct.
///
/// This struct owns the FFT plan and processes audio data for visualization.
pub struct Plan {
    ptr: NonNull<cava_plan>,
}

impl Plan {
    /// Creates a new cava plan with the specified parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - cava_init returns a null pointer
    /// - cava_init reports an error via the status field
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

    /// Returns the plan's status code (0 = success).
    pub fn status(&self) -> i32 {
        // SAFETY: ptr is valid and points to an initialized cava_plan.
        unsafe { (*self.ptr.as_ptr()).status }
    }

    /// Returns the error message if status is non-zero.
    pub fn error_message(&self) -> Option<String> {
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

    /// Executes the FFT processing on the input audio data.
    ///
    /// This reads from audio_input's buffer and writes visualization data to audio_output.
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

/// # Safety
///
/// `Plan` is `Send` because:
/// - The inner `cava_plan` is self-contained with no thread-local state
/// - All internal buffers are allocated and owned by the plan
/// - libcava functions don't use global mutable state
unsafe impl Send for Plan {}

/// # Safety
///
/// `Plan` is `Sync` because:
/// - `execute()` only requires `&self` and operates on separate input/output buffers
/// - The FFT processing is stateless from the plan's perspective
/// - Multiple threads can safely read the same plan (though concurrent execute() calls
///   on the same input/output would be a logic error at the caller level)
unsafe impl Sync for Plan {}

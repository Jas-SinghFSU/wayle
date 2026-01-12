#![allow(unsafe_code)]

use std::{pin::Pin, ptr, slice};

use super::{
    super::types::{audio_raw, audio_raw_clean, audio_raw_init, cava_plan},
    AudioInput, Config, Plan,
};
use crate::{Error, Result};

/// Safe wrapper around libcava's audio_raw struct for output data.
///
/// This struct holds the processed audio visualization data after cava_execute.
pub struct AudioOutput {
    pub(super) inner: Pin<Box<audio_raw>>,
}

impl AudioOutput {
    /// Creates a new audio output handler for the specified number of bars.
    pub fn new(bars: usize) -> Self {
        let audio_raw = Box::new(audio_raw {
            bars: ptr::null_mut(),
            previous_frame: ptr::null_mut(),
            bars_left: ptr::null_mut(),
            bars_right: ptr::null_mut(),
            bars_raw: ptr::null_mut(),
            previous_bars_raw: ptr::null_mut(),
            cava_out: ptr::null_mut(),
            dimension_bar: ptr::null_mut(),
            dimension_value: ptr::null_mut(),
            userEQ_keys_to_bars_ratio: 0.0,
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

    /// Initializes the audio output with the given components.
    ///
    /// # Errors
    ///
    /// Returns an error if audio_raw_init fails.
    pub fn init(
        &mut self,
        audio_input: &mut AudioInput,
        config: &mut Config,
        plan: &Plan,
    ) -> Result<()> {
        let mut plan_ptr = plan.as_ptr();

        // SAFETY: All pointers are valid and point to initialized structs.
        // audio_raw_init takes a pointer-to-pointer for the plan.
        let ret = unsafe {
            audio_raw_init(
                audio_input.as_ptr(),
                self.as_ptr(),
                config.as_ptr(),
                &mut plan_ptr as *mut *mut cava_plan,
            )
        };

        if ret != 0 {
            return Err(Error::AudioRawInitFailed(ret));
        }

        Ok(())
    }

    pub(crate) fn as_ptr(&mut self) -> *mut audio_raw {
        &mut *self.inner as *mut _
    }

    /// Returns a slice of the current visualization values.
    ///
    /// The values are normalized between 0.0 and 1.0.
    pub fn values(&self) -> &[f64] {
        // SAFETY: After init(), cava_out points to valid memory with number_of_bars elements.
        // The data is valid for the lifetime of this struct.
        unsafe {
            let output_data = self.inner.as_ref().get_ref();
            slice::from_raw_parts(output_data.cava_out, output_data.number_of_bars as usize)
        }
    }
}

impl Drop for AudioOutput {
    fn drop(&mut self) {
        // SAFETY: audio_raw_clean frees memory allocated by audio_raw_init.
        // This is safe because we own the audio_raw struct.
        unsafe {
            audio_raw_clean(self.as_ptr());
        }
    }
}

/// # Safety
///
/// `AudioOutput` is `Send` because:
/// - The inner `audio_raw` contains only primitive types and raw pointers
/// - All raw pointers point to memory allocated by libcava and owned by this struct
/// - No thread-local state is accessed
unsafe impl Send for AudioOutput {}

/// # Safety
///
/// `AudioOutput` is `Sync` because:
/// - Read access to `values()` doesn't mutate state
/// - Write access requires `&mut self` through `init()` and `as_ptr()`
/// - The visualization data is only written by `cava_execute` which takes `&self` on Plan
unsafe impl Sync for AudioOutput {}

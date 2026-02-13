#![allow(unsafe_code)]

use std::{
    pin::Pin,
    ptr::{self, NonNull},
    slice,
};

use super::{
    super::types::{audio_raw, audio_raw_clean, audio_raw_init, cava_plan},
    AudioInput, Config, Plan,
};
use crate::{Error, Result};

pub(crate) struct AudioOutput {
    pub(super) inner: Pin<Box<audio_raw>>,
}

impl AudioOutput {
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

    /// Initializes the audio output buffers and creates the FFT plan.
    ///
    /// The plan is created internally by `audio_raw_init` with the correct
    /// bar count adjusted for stereo output channels.
    ///
    /// # Errors
    ///
    /// Returns error if initialization fails or the plan pointer is null.
    pub fn init(&mut self, audio_input: &mut AudioInput, config: &mut Config) -> Result<Plan> {
        let mut plan_ptr: *mut cava_plan = ptr::null_mut();

        // SAFETY: All pointers are valid and point to initialized structs.
        // audio_raw_init creates a cava_plan internally via cava_init and writes
        // its pointer to plan_ptr. We capture this pointer to construct a Plan.
        let ret = unsafe {
            audio_raw_init(
                audio_input.as_ptr(),
                self.as_ptr(),
                config.as_ptr(),
                &mut plan_ptr,
            )
        };

        if ret != 0 {
            return Err(Error::AudioRawInitFailed(ret));
        }

        let ptr = NonNull::new(plan_ptr).ok_or(Error::NullPlan)?;
        Ok(Plan::from_raw(ptr))
    }

    pub(crate) fn as_ptr(&mut self) -> *mut audio_raw {
        &mut *self.inner as *mut _
    }

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

unsafe impl Send for AudioOutput {}

unsafe impl Sync for AudioOutput {}

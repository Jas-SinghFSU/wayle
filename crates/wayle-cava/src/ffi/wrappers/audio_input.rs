#![allow(unsafe_code)]

use std::{ffi, mem::MaybeUninit, pin::Pin, ptr, thread};

use super::{
    super::types::{InputFn, audio_data, get_input},
    Config,
};
use crate::{Error, Result};

struct SendPtr(usize);

unsafe impl Send for SendPtr {}

pub(crate) struct AudioInput {
    pub(super) inner: Pin<Box<audio_data>>,
    input_thread: Option<thread::JoinHandle<()>>,
}

impl AudioInput {
    pub fn new(buffer_size: usize, channels: u32, samplerate: u32) -> Result<Self> {
        const PER_READ_CHUNK_SIZE: usize = 512;

        let mut audio = Box::new(audio_data {
            cava_in: ptr::null_mut(),
            input_buffer_size: (PER_READ_CHUNK_SIZE * channels as usize) as i32,
            cava_buffer_size: buffer_size as i32,
            format: 16,
            rate: samplerate,
            channels,
            threadparams: 0,
            source: ptr::null_mut(),
            im: 0,
            terminate: 0,
            error_message: [0; 1024],
            samples_counter: 0,
            IEEE_FLOAT: 0,
            autoconnect: 0,
            active: 0,
            remix: 0,
            virtual_: 0,
            lock: unsafe { MaybeUninit::zeroed().assume_init() },
            resumeCond: unsafe { MaybeUninit::zeroed().assume_init() },
            suspendFlag: false,
        });

        // SAFETY: `audio.lock` is uninitialized memory that we're initializing in place.
        // pthread_mutex_init returns 0 on success.
        let ret = unsafe {
            libc::pthread_mutex_init(
                ptr::addr_of_mut!(audio.lock) as *mut libc::pthread_mutex_t,
                ptr::null(),
            )
        };
        if ret != 0 {
            return Err(Error::MutexInit(ret));
        }

        // SAFETY: `audio.resumeCond` is uninitialized memory that we're initializing in place.
        // pthread_cond_init returns 0 on success.
        let ret = unsafe {
            libc::pthread_cond_init(
                ptr::addr_of_mut!(audio.resumeCond) as *mut libc::pthread_cond_t,
                ptr::null(),
            )
        };
        if ret != 0 {
            // SAFETY: We successfully initialized the mutex above, so we must destroy it.
            unsafe {
                libc::pthread_mutex_destroy(
                    ptr::addr_of_mut!(audio.lock) as *mut libc::pthread_mutex_t
                );
            }
            return Err(Error::CondInit(ret));
        }

        Ok(Self {
            inner: Pin::new(audio),
            input_thread: None,
        })
    }

    /// Calls `get_input` to configure audio buffers and obtain the input
    /// thread function. Must be called before [`AudioOutput::init`] to match
    /// the order expected by libcava.
    ///
    /// # Errors
    ///
    /// Returns error if `get_input` returns null (unsupported input method).
    pub fn setup_input(&mut self, config: &mut Config) -> Result<InputFn> {
        // SAFETY: Both pointers are valid and point to initialized structs.
        // get_input allocates `cava_in` and `source` buffers via malloc,
        // sets audio format/rate/channels, and returns a function pointer.
        unsafe { get_input(self.as_ptr(), config.as_ptr()) }.ok_or(Error::NoInputFunction)
    }

    /// Spawns the audio input thread using the function from [`setup_input`].
    pub fn spawn_input_thread(&mut self, input_fn: InputFn) {
        if self.input_thread.is_some() {
            return;
        }

        let audio_ptr = SendPtr(self.as_ptr() as usize);

        // SAFETY: The input function expects a void pointer to audio_data.
        // The pointer remains valid because:
        // 1. AudioInput owns the audio_data and is pinned
        // 2. The thread is joined in Drop before audio_data is deallocated
        let handle = thread::spawn(move || unsafe {
            input_fn(audio_ptr.0 as *mut ffi::c_void);
        });

        self.input_thread = Some(handle);
    }

    pub(crate) fn as_ptr(&mut self) -> *mut audio_data {
        &mut *self.inner as *mut _
    }

    pub fn lock(&self) -> Result<()> {
        // SAFETY: The mutex was initialized in `new()` and remains valid.
        let ret = unsafe {
            libc::pthread_mutex_lock(ptr::addr_of!(self.inner.lock) as *mut libc::pthread_mutex_t)
        };
        if ret != 0 {
            return Err(Error::MutexLock(ret));
        }

        Ok(())
    }

    pub fn unlock(&self) -> Result<()> {
        // SAFETY: The mutex was initialized in `new()` and is currently locked.
        let ret = unsafe {
            libc::pthread_mutex_unlock(ptr::addr_of!(self.inner.lock) as *mut libc::pthread_mutex_t)
        };
        if ret != 0 {
            return Err(Error::MutexUnlock(ret));
        }

        Ok(())
    }

    pub fn samples_counter(&self) -> i32 {
        self.inner.samples_counter
    }

    pub fn reset_samples_counter(&mut self) {
        self.inner.samples_counter = 0;
    }
}

impl Drop for AudioInput {
    fn drop(&mut self) {
        self.inner.terminate = 1;

        if let Some(handle) = self.input_thread.take() {
            let _ = handle.join();
        }

        // SAFETY: The condition variable and mutex were initialized in `new()`.
        // We're destroying them after the input thread has terminated.
        unsafe {
            libc::pthread_cond_destroy(
                ptr::addr_of_mut!(self.inner.resumeCond) as *mut libc::pthread_cond_t
            );
            libc::pthread_mutex_destroy(
                ptr::addr_of_mut!(self.inner.lock) as *mut libc::pthread_mutex_t
            );
        }

        // SAFETY: `cava_in` and `source` were allocated by C (get_input /
        // audio_raw_init) via malloc. free(NULL) is safe if setup_input was
        // never called.
        unsafe {
            libc::free(self.inner.cava_in as *mut ffi::c_void);
            libc::free(self.inner.source as *mut ffi::c_void);
        }
    }
}

unsafe impl Send for AudioInput {}

unsafe impl Sync for AudioInput {}

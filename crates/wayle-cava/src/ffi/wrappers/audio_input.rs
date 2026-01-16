#![allow(unsafe_code)]

use std::{
    ffi::{self, CString},
    mem::MaybeUninit,
    pin::Pin,
    ptr, thread,
};

use super::{
    super::types::{audio_data, get_input},
    Config,
};
use crate::{Error, Result};

struct SendPtr(usize);

unsafe impl Send for SendPtr {}

pub(crate) struct AudioInput {
    pub(super) inner: Pin<Box<audio_data>>,
    _cava_in_buffer: Vec<f64>,
    _source_string: CString,
    input_thread: Option<thread::JoinHandle<()>>,
}

impl AudioInput {
    pub fn new(buffer_size: usize, channels: u32, samplerate: u32, source: &str) -> Result<Self> {
        let source_string = CString::new(source)?;
        let mut cava_in_buffer = vec![0.0; buffer_size];

        const PER_READ_CHUNK_SIZE: usize = 512;

        let mut audio = Box::new(audio_data {
            cava_in: cava_in_buffer.as_mut_ptr(),
            input_buffer_size: (PER_READ_CHUNK_SIZE * channels as usize) as i32,
            cava_buffer_size: buffer_size as i32,
            format: -1,
            rate: samplerate,
            channels,
            threadparams: 0,
            source: source_string.as_ptr() as *mut _,
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
            _cava_in_buffer: cava_in_buffer,
            _source_string: source_string,
            input_thread: None,
        })
    }

    pub fn start_input(&mut self, mut config: Config) -> Result<()> {
        if self.input_thread.is_some() {
            return Ok(());
        }

        // SAFETY: Both pointers are valid and point to initialized structs.
        // get_input returns a function pointer or None.
        let input_fn =
            unsafe { get_input(self.as_ptr(), config.as_ptr()) }.ok_or(Error::NoInputFunction)?;

        let audio_ptr = SendPtr(self.as_ptr() as usize);

        // SAFETY: The input function expects a void pointer to audio_data.
        // The pointer remains valid because:
        // 1. AudioInput owns the audio_data and is pinned
        // 2. The thread is joined in Drop before audio_data is deallocated
        let handle = thread::spawn(move || unsafe {
            input_fn(audio_ptr.0 as *mut ffi::c_void);
        });

        self.input_thread = Some(handle);

        Ok(())
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
    }
}

unsafe impl Send for AudioInput {}

unsafe impl Sync for AudioInput {}

#![allow(unsafe_code)]

use std::{ffi, mem::MaybeUninit, pin::Pin, ptr, thread};

use ffi::CString;

use super::{
    super::{bindings, types},
    Config,
};
use crate::{Error, Result};

struct SendPtr(usize);
unsafe impl Send for SendPtr {}

pub struct AudioInput {
    pub(super) inner: Pin<Box<types::AudioData>>,
    _cava_in_buffer: Vec<f64>,
    _source_string: CString,
    input_thread: Option<thread::JoinHandle<()>>,
}

impl AudioInput {
    pub fn new(buffer_size: usize, channels: u32, samplerate: u32, source: &str) -> Result<Self> {
        let source_string = CString::new(source)?;
        let mut cava_in_buffer = vec![0.0; buffer_size];

        const PER_READ_CHUNK_SIZE: usize = 512;

        let mut audio_data = Box::new(types::AudioData {
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
            ieee_float: 0,
            autoconnect: 0,
            lock: unsafe { MaybeUninit::zeroed().assume_init() },
            resume_cond: unsafe { MaybeUninit::zeroed().assume_init() },
            suspend_flag: false,
        });

        let ret = unsafe { libc::pthread_mutex_init(&mut audio_data.lock, ptr::null()) };
        if ret != 0 {
            return Err(Error::MutexInit(ret));
        }

        let ret = unsafe { libc::pthread_cond_init(&mut audio_data.resume_cond, ptr::null()) };
        if ret != 0 {
            unsafe {
                libc::pthread_mutex_destroy(&mut audio_data.lock);
            }
            return Err(Error::CondInit(ret));
        }

        Ok(Self {
            inner: Pin::new(audio_data),
            _cava_in_buffer: cava_in_buffer,
            _source_string: source_string,
            input_thread: None,
        })
    }

    pub fn start_input(&mut self, mut config: Config) -> Result<()> {
        if self.input_thread.is_some() {
            return Ok(());
        }

        let input_fn = unsafe { bindings::get_input(self.as_ptr(), config.as_ptr()) }
            .ok_or(Error::NoInputFunction)?;

        let audio_ptr = SendPtr(self.as_ptr() as usize);

        let handle = thread::spawn(move || unsafe {
            input_fn(audio_ptr.0 as *mut ffi::c_void);
        });

        self.input_thread = Some(handle);

        Ok(())
    }

    pub(crate) fn as_ptr(&mut self) -> *mut types::AudioData {
        &mut *self.inner as *mut _
    }

    pub fn lock(&self) -> Result<()> {
        let ret = unsafe { libc::pthread_mutex_lock(&self.inner.lock as *const _ as *mut _) };
        if ret != 0 {
            return Err(Error::MutexLock(ret));
        }

        Ok(())
    }

    pub fn unlock(&self) -> Result<()> {
        let ret = unsafe { libc::pthread_mutex_unlock(&self.inner.lock as *const _ as *mut _) };
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

        unsafe {
            libc::pthread_cond_destroy(&mut self.inner.resume_cond);
            libc::pthread_mutex_destroy(&mut self.inner.lock);
        }
    }
}

unsafe impl Send for AudioInput {}
unsafe impl Sync for AudioInput {}

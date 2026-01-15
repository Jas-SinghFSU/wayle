use std::ffi::NulError;

use thiserror::Error;

/// Errors that can occur when working with the CAVA audio visualizer.
#[derive(Debug, Error)]
pub enum Error {
    /// CAVA plan initialization failed with the given error message.
    ///
    /// # Errors
    /// This typically occurs when CAVA configuration parameters are invalid or
    /// when system resources required for audio processing cannot be allocated.
    #[error("cannot initialize CAVA: {0}")]
    InitFailed(String),

    /// CAVA plan creation returned a null pointer.
    ///
    /// # Errors
    /// Indicates a critical failure in CAVA initialization where the library
    /// failed to allocate the plan structure.
    #[error("CAVA plan returned null pointer")]
    NullPlan,

    /// Failed to acquire mutex lock on audio input data.
    ///
    /// # Errors
    /// Returned when pthread_mutex_lock fails. The error code is the errno value.
    #[error("Mutex lock operation failed with error code: {0}")]
    MutexLock(i32),

    /// Failed to release mutex lock on audio input data.
    ///
    /// # Errors
    /// Returned when pthread_mutex_unlock fails. The error code is the errno value.
    #[error("Mutex unlock operation failed with error code: {0}")]
    MutexUnlock(i32),

    /// Failed to initialize pthread mutex.
    ///
    /// # Errors
    /// Returned when pthread_mutex_init fails during AudioInput construction.
    /// The error code is the errno value.
    #[error("Mutex initialization failed with error code: {0}")]
    MutexInit(i32),

    /// Failed to initialize pthread condition variable.
    ///
    /// # Errors
    /// Returned when pthread_cond_init fails during AudioInput construction.
    /// The error code is the errno value.
    #[error("Condition variable initialization failed with error code: {0}")]
    CondInit(i32),

    /// No audio input function was returned by CAVA.
    ///
    /// # Errors
    /// Indicates the selected input method is not supported or could not be loaded.
    #[error("No audio input function returned")]
    NoInputFunction,

    /// String parameter contains an interior null byte.
    ///
    /// # Errors
    /// Returned when converting Rust strings to C strings for FFI calls.
    /// The string must not contain null bytes except at the terminator.
    #[error("String contains null byte: {0}")]
    NullByte(#[from] NulError),

    /// Audio raw output initialization failed.
    ///
    /// # Errors
    /// Returned when audio_raw_init from libcava fails. The error code indicates
    /// the specific failure reason.
    #[error("Audio raw initialization failed with error code: {0}")]
    AudioRawInitFailed(i32),

    /// Invalid configuration parameter.
    ///
    /// # Errors
    /// Returned when a configuration parameter is out of valid range or has
    /// an invalid relationship with other parameters.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

/// A specialized Result type for CAVA operations.
///
/// This type alias uses the crate's [`Error`] type as the error variant.
pub type Result<T> = std::result::Result<T, Error>;

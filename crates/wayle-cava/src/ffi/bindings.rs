use std::os::raw::{c_double, c_int, c_uint, c_void};

use super::types::{AudioData, AudioRaw, CavaPlan, ConfigParams};

pub type ThreadFn = Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>;

#[allow(unsafe_code)]
unsafe extern "C" {
    pub fn cava_init(
        number_of_bars: c_int,
        rate: c_uint,
        channels: c_int,
        autosens: c_int,
        noise_reduction: c_double,
        low_cut_off: c_int,
        high_cut_off: c_int,
    ) -> *mut CavaPlan;

    pub fn cava_execute(
        cava_in: *mut c_double,
        new_samples: c_int,
        cava_out: *mut c_double,
        plan: *mut CavaPlan,
    );

    pub fn cava_destroy(plan: *mut CavaPlan);

    pub fn get_input(audio: *mut AudioData, prm: *mut ConfigParams) -> ThreadFn;

    pub fn audio_raw_init(
        audio: *mut AudioData,
        audio_raw: *mut AudioRaw,
        prm: *mut ConfigParams,
        plan: *mut CavaPlan,
    ) -> c_int;

    pub fn audio_raw_clean(audio_raw: *mut AudioRaw) -> c_int;
}

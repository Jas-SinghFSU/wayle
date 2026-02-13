/// Wrapper header for bindgen to generate Rust FFI bindings for libcava.
///
/// Keep `cava_plan` opaque here so bindgen does not need transitive FFTW headers.
#pragma once

#include <cava/config.h>
#include <cava/input/common.h>

#ifdef __cplusplus
extern "C" {
#endif

struct cava_plan;

struct audio_raw {
    int *bars;
    int *previous_frame;
    float *bars_left;
    float *bars_right;
    float *bars_raw;
    float *previous_bars_raw;
    double *cava_out;
    int *dimension_bar;
    int *dimension_value;
    double userEQ_keys_to_bars_ratio;
    int channels;
    int number_of_bars;
    int output_channels;
    int height;
    int lines;
    int width;
    int remainder;
};

struct cava_plan *cava_init(int number_of_bars, unsigned int rate, int channels, int autosens,
                            double noise_reduction, int low_cut_off, int high_cut_off);
void cava_execute(double *cava_in, int new_samples, double *cava_out, struct cava_plan *plan);
void cava_destroy(struct cava_plan *plan);

int audio_raw_init(struct audio_data *audio, struct audio_raw *audio_raw, struct config_params *prm,
                   struct cava_plan **plan);
int audio_raw_clean(struct audio_raw *audio_raw);
int audio_raw_destroy(struct audio_raw *audio_raw);

#ifdef __cplusplus
}
#endif

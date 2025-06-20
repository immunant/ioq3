use ::libc;

pub mod SigProc_FLP_h {
    /* *******************************************************************/
    /*                                MACROS                            */
    /* *******************************************************************/
    /* sigmoid function */
    #[inline]

    pub unsafe extern "C" fn silk_sigmoid(mut x: f32) -> f32 {
        return (1.0f64 / (1.0f64 + crate::stdlib::exp(-x as f64))) as f32;
    }
    /* using log2() helps the fixed-point conversion */
    #[inline]

    pub unsafe extern "C" fn silk_log2(mut x: f64) -> f32 {
        return (3.32192809488736f64 * crate::stdlib::log10(x)) as f32;
    }

    /* SILK_SIGPROC_FLP_H */
}

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;

pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;

pub use crate::structs_FLP_h::silk_encoder_control_FLP;
pub use crate::structs_FLP_h::silk_encoder_state_FLP;
pub use crate::structs_FLP_h::silk_shape_state_FLP;
pub use crate::structs_h::silk_LP_state;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_VAD_state;
pub use crate::structs_h::silk_encoder_state;
pub use crate::structs_h::silk_nsq_state;
pub use crate::structs_h::SideInfoIndices;

pub use crate::src::opus_1_2_1::silk::float::autocorrelation_FLP::silk_autocorrelation_FLP;
pub use crate::src::opus_1_2_1::silk::float::bwexpander_FLP::silk_bwexpander_FLP;
pub use crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP;
pub use crate::src::opus_1_2_1::silk::float::k2a_FLP::silk_k2a_FLP;
pub use crate::src::opus_1_2_1::silk::float::noise_shape_analysis_FLP::SigProc_FLP_h::silk_log2;
pub use crate::src::opus_1_2_1::silk::float::noise_shape_analysis_FLP::SigProc_FLP_h::silk_sigmoid;
pub use crate::src::opus_1_2_1::silk::float::schur_FLP::silk_schur_FLP;

/* **********************************************************************
Copyright (c) 2006-2011, Skype Limited. All rights reserved.
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
- Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.
- Redistributions in binary form must reproduce the above copyright
notice, this list of conditions and the following disclaimer in the
documentation and/or other materials provided with the distribution.
- Neither the name of Internet Society, IETF or IETF Trust, nor the
names of specific contributors, may be used to endorse or promote
products derived from this software without specific prior written
permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
***********************************************************************/
/* Compute gain to make warped filter coefficients have a zero mean log frequency response on a   */
/* non-warped frequency scale. (So that it can be implemented with a minimum-phase monic filter.) */
/* Note: A monic filter is one with the first coefficient equal to 1.0. In Silk we omit the first */
/* coefficient in an array of coefficients, for monic filters.                                    */
#[inline]

unsafe extern "C" fn warped_gain(mut coefs: *const f32, mut lambda: f32, mut order: i32) -> f32 {
    let mut i: i32 = 0;
    let mut gain: f32 = 0.;
    lambda = -lambda;
    gain = *coefs.offset((order - 1 as i32) as isize);
    i = order - 2 as i32;
    while i >= 0 as i32 {
        gain = lambda * gain + *coefs.offset(i as isize);
        i -= 1
    }
    return 1.0f32 / (1.0f32 - lambda * gain);
}
/* Convert warped filter coefficients to monic pseudo-warped coefficients and limit maximum     */
/* amplitude of monic warped coefficients by using bandwidth expansion on the true coefficients */
#[inline]

unsafe extern "C" fn warped_true2monic_coefs(
    mut coefs: *mut f32,
    mut lambda: f32,
    mut limit: f32,
    mut order: i32,
) {
    let mut i: i32 = 0;
    let mut iter: i32 = 0;
    let mut ind: i32 = 0 as i32;
    let mut tmp: f32 = 0.;
    let mut maxabs: f32 = 0.;
    let mut chirp: f32 = 0.;
    let mut gain: f32 = 0.;
    /* Convert to monic coefficients */
    i = order - 1 as i32;
    while i > 0 as i32 {
        *coefs.offset((i - 1 as i32) as isize) -= lambda * *coefs.offset(i as isize);
        i -= 1
    }
    gain = (1.0f32 - lambda * lambda) / (1.0f32 + lambda * *coefs.offset(0 as i32 as isize));
    i = 0 as i32;
    while i < order {
        *coefs.offset(i as isize) *= gain;
        i += 1
    }
    /* Limit */
    iter = 0 as i32;
    while iter < 10 as i32 {
        /* Find maximum absolute value */
        maxabs = -1.0f32;
        i = 0 as i32;
        while i < order {
            tmp = crate::stdlib::fabs(*coefs.offset(i as isize) as f64) as f32;
            if tmp > maxabs {
                maxabs = tmp;
                ind = i
            }
            i += 1
        }
        if maxabs <= limit {
            /* Coefficients are within range - done */
            return;
        }
        /* Convert back to true warped coefficients */
        i = 1 as i32;
        while i < order {
            *coefs.offset((i - 1 as i32) as isize) += lambda * *coefs.offset(i as isize);
            i += 1
        }
        gain = 1.0f32 / gain;
        i = 0 as i32;
        while i < order {
            *coefs.offset(i as isize) *= gain;
            i += 1
        }
        /* Apply bandwidth expansion */
        chirp = 0.99f32
            - (0.8f32 + 0.1f32 * iter as f32) * (maxabs - limit)
                / (maxabs * (ind + 1 as i32) as f32);
        silk_bwexpander_FLP(coefs, order, chirp);
        /* Convert to monic warped coefficients */
        i = order - 1 as i32;
        while i > 0 as i32 {
            *coefs.offset((i - 1 as i32) as isize) -= lambda * *coefs.offset(i as isize);
            i -= 1
        }
        gain = (1.0f32 - lambda * lambda) / (1.0f32 + lambda * *coefs.offset(0 as i32 as isize));
        i = 0 as i32;
        while i < order {
            *coefs.offset(i as isize) *= gain;
            i += 1
        }
        iter += 1
    }
}
#[inline]

unsafe extern "C" fn limit_coefs(mut coefs: *mut f32, mut limit: f32, mut order: i32) {
    let mut i: i32 = 0;
    let mut iter: i32 = 0;
    let mut ind: i32 = 0 as i32;
    let mut tmp: f32 = 0.;
    let mut maxabs: f32 = 0.;
    let mut chirp: f32 = 0.;
    iter = 0 as i32;
    while iter < 10 as i32 {
        /* Find maximum absolute value */
        maxabs = -1.0f32;
        i = 0 as i32;
        while i < order {
            tmp = crate::stdlib::fabs(*coefs.offset(i as isize) as f64) as f32;
            if tmp > maxabs {
                maxabs = tmp;
                ind = i
            }
            i += 1
        }
        if maxabs <= limit {
            /* Coefficients are within range - done */
            return;
        }
        /* Apply bandwidth expansion */
        chirp = 0.99f32
            - (0.8f32 + 0.1f32 * iter as f32) * (maxabs - limit)
                / (maxabs * (ind + 1 as i32) as f32);
        silk_bwexpander_FLP(coefs, order, chirp);
        iter += 1
    }
}
/* **********************************************************************
Copyright (c) 2006-2011, Skype Limited. All rights reserved.
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
- Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.
- Redistributions in binary form must reproduce the above copyright
notice, this list of conditions and the following disclaimer in the
documentation and/or other materials provided with the distribution.
- Neither the name of Internet Society, IETF or IETF Trust, nor the
names of specific contributors, may be used to endorse or promote
products derived from this software without specific prior written
permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
***********************************************************************/
/* ********************/
/* Encoder Functions */
/* ********************/
/* High-pass filter with cutoff frequency adaptation based on pitch lag statistics */
/* I/O  Encoder states                              */
/* Encoder main function */
/* I/O  Encoder state FLP                           */
/* Encoder main function */
/* I/O  Encoder state FLP                           */
/* O    Number of payload bytes;                    */
/* I/O  compressor data structure                   */
/* I    The type of conditional coding to use       */
/* I    If > 0: maximum number of output bits       */
/* I    Flag to force constant-bitrate operation    */
/* Initializes the Silk encoder state */
/* I/O  Encoder state FLP                           */
/* I    Run-tim architecture                        */
/* Control the Silk encoder */
/* I/O  Pointer to Silk encoder state FLP           */
/* I    Control structure                           */
/* I    Flag to allow switching audio bandwidth     */
/* I    Channel number                              */
/* *************************/
/* Noise shaping analysis */
/* *************************/
/* Compute noise shaping coefficients and initial gain values */
/* Compute noise shaping coefficients and initial gain values */
#[no_mangle]

pub unsafe extern "C" fn silk_noise_shape_analysis_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut psEncCtrl: *mut silk_encoder_control_FLP,
    mut pitch_res: *const f32,
    mut x: *const f32,
)
/* I    Input signal [frame_length + la_shape]      */
{
    let mut psShapeSt: *mut silk_shape_state_FLP = &mut (*psEnc).sShape;
    let mut k: i32 = 0;
    let mut nSamples: i32 = 0;
    let mut nSegs: i32 = 0;
    let mut SNR_adj_dB: f32 = 0.;
    let mut HarmShapeGain: f32 = 0.;
    let mut Tilt: f32 = 0.;
    let mut nrg: f32 = 0.;
    let mut log_energy: f32 = 0.;
    let mut log_energy_prev: f32 = 0.;
    let mut energy_variation: f32 = 0.;
    let mut BWExp: f32 = 0.;
    let mut gain_mult: f32 = 0.;
    let mut gain_add: f32 = 0.;
    let mut strength: f32 = 0.;
    let mut b: f32 = 0.;
    let mut warping: f32 = 0.;
    let mut x_windowed: [f32; 240] = [0.; 240];
    let mut auto_corr: [f32; 25] = [0.; 25];
    let mut rc: [f32; 25] = [0.; 25];
    let mut x_ptr: *const f32 = std::ptr::null();
    let mut pitch_res_ptr: *const f32 = std::ptr::null();
    /* Point to start of first LPC analysis block */
    x_ptr = x.offset(-((*psEnc).sCmn.la_shape as isize));
    /* ***************/
    /* GAIN CONTROL */
    /* ***************/
    SNR_adj_dB = (*psEnc).sCmn.SNR_dB_Q7 as f32 * (1 as i32 as f32 / 128.0f32);
    /* Input quality is the average of the quality in the lowest two VAD bands */
    (*psEncCtrl).input_quality = 0.5f32
        * ((*psEnc).sCmn.input_quality_bands_Q15[0 as i32 as usize]
            + (*psEnc).sCmn.input_quality_bands_Q15[1 as i32 as usize]) as f32
        * (1.0f32 / 32768.0f32);
    /* Coding quality level, between 0.0 and 1.0 */
    (*psEncCtrl).coding_quality = silk_sigmoid(0.25f32 * (SNR_adj_dB - 20.0f32));
    if (*psEnc).sCmn.useCBR == 0 as i32 {
        /* Reduce coding SNR during low speech activity */
        b = 1.0f32 - (*psEnc).sCmn.speech_activity_Q8 as f32 * (1.0f32 / 256.0f32);
        SNR_adj_dB -= 2.0f32
            * (*psEncCtrl).coding_quality
            * (0.5f32 + 0.5f32 * (*psEncCtrl).input_quality)
            * b
            * b
    }
    if (*psEnc).sCmn.indices.signalType as i32 == 2 as i32 {
        /* Reduce gains for periodic signals */
        SNR_adj_dB += 2.0f32 * (*psEnc).LTPCorr
    } else {
        /* For unvoiced signals and low-quality input, adjust the quality slower than SNR_dB setting */
        SNR_adj_dB += (-0.4f32 * (*psEnc).sCmn.SNR_dB_Q7 as f32 * (1 as i32 as f32 / 128.0f32)
            + 6.0f32)
            * (1.0f32 - (*psEncCtrl).input_quality)
    }
    /* ************************/
    /* SPARSENESS PROCESSING */
    /* ************************/
    /* Set quantizer offset */
    if (*psEnc).sCmn.indices.signalType as i32 == 2 as i32 {
        /* Initially set to 0; may be overruled in process_gains(..) */
        (*psEnc).sCmn.indices.quantOffsetType = 0 as i32 as i8
    } else {
        /* Sparseness measure, based on relative fluctuations of energy per 2 milliseconds */
        nSamples = 2 as i32 * (*psEnc).sCmn.fs_kHz;
        energy_variation = 0.0f32;
        log_energy_prev = 0.0f32;
        pitch_res_ptr = pitch_res;
        nSegs = 5 as i32 as opus_int16 as opus_int32
            * (*psEnc).sCmn.nb_subfr as opus_int16 as opus_int32
            / 2 as i32;
        k = 0 as i32;
        while k < nSegs {
            nrg = nSamples as f32 + silk_energy_FLP(pitch_res_ptr, nSamples) as f32;
            log_energy = silk_log2(nrg as f64);
            if k > 0 as i32 {
                energy_variation +=
                    crate::stdlib::fabs((log_energy - log_energy_prev) as f64) as f32
            }
            log_energy_prev = log_energy;
            pitch_res_ptr = pitch_res_ptr.offset(nSamples as isize);
            k += 1
        }
        /* Set quantization offset depending on sparseness measure */
        if energy_variation > 0.6f32 * (nSegs - 1 as i32) as f32 {
            (*psEnc).sCmn.indices.quantOffsetType = 0 as i32 as i8
        } else {
            (*psEnc).sCmn.indices.quantOffsetType = 1 as i32 as i8
        }
    }
    /* ******************************/
    /* Control bandwidth expansion */
    /* ******************************/
    /* More BWE for signals with high prediction gain */
    strength = 1e-3f32 * (*psEncCtrl).predGain; /* between 0.0 and 1.0 */
    BWExp = 0.94f32 / (1.0f32 + strength * strength);
    /* Slightly more warping in analysis will move quantization noise up in frequency, where it's better masked */
    warping = (*psEnc).sCmn.warping_Q16 as f32 / 65536.0f32 + 0.01f32 * (*psEncCtrl).coding_quality;
    /* *******************************************/
    /* Compute noise shaping AR coefs and gains */
    /* *******************************************/
    k = 0 as i32;
    while k < (*psEnc).sCmn.nb_subfr {
        /* Apply window: sine slope followed by flat part followed by cosine slope */
        let mut shift: i32 = 0;
        let mut slope_part: i32 = 0;
        let mut flat_part: i32 = 0;
        flat_part = (*psEnc).sCmn.fs_kHz * 3 as i32;
        slope_part = ((*psEnc).sCmn.shapeWinLength - flat_part) / 2 as i32;
        crate::src::opus_1_2_1::silk::float::apply_sine_window_FLP::silk_apply_sine_window_FLP(
            x_windowed.as_mut_ptr(),
            x_ptr,
            1 as i32,
            slope_part,
        );
        shift = slope_part;
        crate::stdlib::memcpy(
            x_windowed.as_mut_ptr().offset(shift as isize) as *mut libc::c_void,
            x_ptr.offset(shift as isize) as *const libc::c_void,
            (flat_part as usize).wrapping_mul(::std::mem::size_of::<f32>() as usize),
        );
        shift += flat_part;
        crate::src::opus_1_2_1::silk::float::apply_sine_window_FLP::silk_apply_sine_window_FLP(
            x_windowed.as_mut_ptr().offset(shift as isize),
            x_ptr.offset(shift as isize),
            2 as i32,
            slope_part,
        );
        /* Update pointer: next LPC analysis block */
        x_ptr = x_ptr.offset((*psEnc).sCmn.subfr_length as isize);
        if (*psEnc).sCmn.warping_Q16 > 0 as i32 {
            /* Calculate warped auto correlation */
            crate::src::opus_1_2_1::silk::float::warped_autocorrelation_FLP::silk_warped_autocorrelation_FLP(auto_corr.as_mut_ptr(),
                                            x_windowed.as_mut_ptr(), warping,
                                            (*psEnc).sCmn.shapeWinLength,
                                            (*psEnc).sCmn.shapingLPCOrder);
        } else {
            /* Calculate regular auto correlation */
            silk_autocorrelation_FLP(
                auto_corr.as_mut_ptr(),
                x_windowed.as_mut_ptr(),
                (*psEnc).sCmn.shapeWinLength,
                (*psEnc).sCmn.shapingLPCOrder + 1 as i32,
            );
        }
        /* Add white noise, as a fraction of energy */
        auto_corr[0 as i32 as usize] += auto_corr[0 as i32 as usize] * 3e-5f32 + 1.0f32;
        /* Convert correlations to prediction coefficients, and compute residual energy */
        nrg = silk_schur_FLP(
            rc.as_mut_ptr(),
            auto_corr.as_mut_ptr() as *const f32,
            (*psEnc).sCmn.shapingLPCOrder,
        );
        silk_k2a_FLP(
            &mut *(*psEncCtrl)
                .AR
                .as_mut_ptr()
                .offset((k * 24 as i32) as isize),
            rc.as_mut_ptr(),
            (*psEnc).sCmn.shapingLPCOrder,
        );
        (*psEncCtrl).Gains[k as usize] = crate::stdlib::sqrt(nrg as f64) as f32;
        if (*psEnc).sCmn.warping_Q16 > 0 as i32 {
            /* Adjust gain for warping */
            (*psEncCtrl).Gains[k as usize] *= warped_gain(
                &mut *(*psEncCtrl)
                    .AR
                    .as_mut_ptr()
                    .offset((k * 24 as i32) as isize),
                warping,
                (*psEnc).sCmn.shapingLPCOrder,
            )
        }
        /* Bandwidth expansion for synthesis filter shaping */
        silk_bwexpander_FLP(
            &mut *(*psEncCtrl)
                .AR
                .as_mut_ptr()
                .offset((k * 24 as i32) as isize),
            (*psEnc).sCmn.shapingLPCOrder,
            BWExp,
        );
        if (*psEnc).sCmn.warping_Q16 > 0 as i32 {
            /* Convert to monic warped prediction coefficients and limit absolute values */
            warped_true2monic_coefs(
                &mut *(*psEncCtrl)
                    .AR
                    .as_mut_ptr()
                    .offset((k * 24 as i32) as isize),
                warping,
                3.999f32,
                (*psEnc).sCmn.shapingLPCOrder,
            );
        } else {
            /* Limit absolute values */
            limit_coefs(
                &mut *(*psEncCtrl)
                    .AR
                    .as_mut_ptr()
                    .offset((k * 24 as i32) as isize),
                3.999f32,
                (*psEnc).sCmn.shapingLPCOrder,
            );
        }
        k += 1
    }
    /* ****************/
    /* Gain tweaking */
    /* ****************/
    /* Increase gains during low speech activity */
    gain_mult = crate::stdlib::pow(2.0f32 as f64, (-0.16f32 * SNR_adj_dB) as f64) as f32;
    gain_add = crate::stdlib::pow(2.0f32 as f64, (0.16f32 * 2 as i32 as f32) as f64) as f32;
    k = 0 as i32;
    while k < (*psEnc).sCmn.nb_subfr {
        (*psEncCtrl).Gains[k as usize] *= gain_mult;
        (*psEncCtrl).Gains[k as usize] += gain_add;
        k += 1
    }
    /* ***********************************************/
    /* Control low-frequency shaping and noise tilt */
    /* ***********************************************/
    /* Less low frequency shaping for noisy inputs */
    strength = 4.0f32
        * (1.0f32
            + 0.5f32
                * ((*psEnc).sCmn.input_quality_bands_Q15[0 as i32 as usize] as f32
                    * (1.0f32 / 32768.0f32)
                    - 1.0f32));
    strength *= (*psEnc).sCmn.speech_activity_Q8 as f32 * (1.0f32 / 256.0f32);
    if (*psEnc).sCmn.indices.signalType as i32 == 2 as i32 {
        /* Reduce low frequencies quantization noise for periodic signals, depending on pitch lag */
        /*f = 400; freqz([1, -0.98 + 2e-4 * f], [1, -0.97 + 7e-4 * f], 2^12, Fs); axis([0, 1000, -10, 1])*/
        k = 0 as i32;
        while k < (*psEnc).sCmn.nb_subfr {
            b = 0.2f32 / (*psEnc).sCmn.fs_kHz as f32
                + 3.0f32 / (*psEncCtrl).pitchL[k as usize] as f32;
            (*psEncCtrl).LF_MA_shp[k as usize] = -1.0f32 + b;
            (*psEncCtrl).LF_AR_shp[k as usize] = 1.0f32 - b - b * strength;
            k += 1
        }
        Tilt = -0.25f32
            - (1 as i32 as f32 - 0.25f32)
                * 0.35f32
                * (*psEnc).sCmn.speech_activity_Q8 as f32
                * (1.0f32 / 256.0f32)
    } else {
        b = 1.3f32 / (*psEnc).sCmn.fs_kHz as f32;
        (*psEncCtrl).LF_MA_shp[0 as i32 as usize] = -1.0f32 + b;
        (*psEncCtrl).LF_AR_shp[0 as i32 as usize] = 1.0f32 - b - b * strength * 0.6f32;
        k = 1 as i32;
        while k < (*psEnc).sCmn.nb_subfr {
            (*psEncCtrl).LF_MA_shp[k as usize] = (*psEncCtrl).LF_MA_shp[0 as i32 as usize];
            (*psEncCtrl).LF_AR_shp[k as usize] = (*psEncCtrl).LF_AR_shp[0 as i32 as usize];
            k += 1
        }
        Tilt = -0.25f32
    }
    /* ***************************/
    /* HARMONIC SHAPING CONTROL */
    /* ***************************/
    if 1 as i32 != 0 && (*psEnc).sCmn.indices.signalType as i32 == 2 as i32 {
        /* Harmonic noise shaping */
        HarmShapeGain = 0.3f32;
        /* More harmonic noise shaping for high bitrates or noisy input */
        HarmShapeGain +=
            0.2f32 * (1.0f32 - (1.0f32 - (*psEncCtrl).coding_quality) * (*psEncCtrl).input_quality);
        /* Less harmonic noise shaping for less periodic signals */
        HarmShapeGain *= crate::stdlib::sqrt((*psEnc).LTPCorr as f64) as f32
    } else {
        HarmShapeGain = 0.0f32
    }
    /* ************************/
    /* Smooth over subframes */
    /* ************************/
    k = 0 as i32;
    while k < (*psEnc).sCmn.nb_subfr {
        (*psShapeSt).HarmShapeGain_smth +=
            0.4f32 * (HarmShapeGain - (*psShapeSt).HarmShapeGain_smth);
        (*psEncCtrl).HarmShapeGain[k as usize] = (*psShapeSt).HarmShapeGain_smth;
        (*psShapeSt).Tilt_smth += 0.4f32 * (Tilt - (*psShapeSt).Tilt_smth);
        (*psEncCtrl).Tilt[k as usize] = (*psShapeSt).Tilt_smth;
        k += 1
    }
}

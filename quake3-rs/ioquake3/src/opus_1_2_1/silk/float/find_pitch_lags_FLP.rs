use ::libc;

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
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* I    LPC residual from pitch analysis            */
/* I    Input signal [frame_length + la_shape]      */
/* Autocorrelations for a warped frequency axis */
/* O    Result [order + 1]                          */
/* I    Input data to correlate                     */
/* I    Warping coefficient                         */
/* I    Length of input                             */
/* I    Correlation order (even)                    */
/* Calculation of LTP state scaling */
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* I    The type of conditional coding to use       */
/* *********************************************/
/* Prediction Analysis                        */
/* *********************************************/
/* Find pitch lags */
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
#[no_mangle]

pub unsafe extern "C" fn silk_find_pitch_lags_FLP(
    mut psEnc: *mut crate::structs_FLP_h::silk_encoder_state_FLP,
    mut psEncCtrl: *mut crate::structs_FLP_h::silk_encoder_control_FLP,
    mut res: *mut f32,
    mut x: *const f32,
    mut arch: i32,
)
/* I    Run-time architecture                       */
{
    let mut buf_len: i32 = 0;
    let mut thrhld: f32 = 0.;
    let mut res_nrg: f32 = 0.;
    let mut x_buf_ptr: *const f32 = 0 as *const f32;
    let mut x_buf: *const f32 = 0 as *const f32;
    let mut auto_corr: [f32; 17] = [0.; 17];
    let mut A: [f32; 16] = [0.; 16];
    let mut refl_coef: [f32; 16] = [0.; 16];
    let mut Wsig: [f32; 384] = [0.; 384];
    let mut Wsig_ptr: *mut f32 = 0 as *mut f32;
    /* *****************************************/
    /* Set up buffer lengths etc based on Fs  */
    /* *****************************************/
    buf_len = (*psEnc).sCmn.la_pitch + (*psEnc).sCmn.frame_length + (*psEnc).sCmn.ltp_mem_length;
    /* Safety check */
    x_buf = x.offset(-((*psEnc).sCmn.ltp_mem_length as isize));
    /* *****************************************/
    /* Estimate LPC AR coeficients            */
    /* *****************************************/
    /* Calculate windowed signal */
    /* First LA_LTP samples */
    x_buf_ptr = x_buf
        .offset(buf_len as isize)
        .offset(-((*psEnc).sCmn.pitch_LPC_win_length as isize));
    Wsig_ptr = Wsig.as_mut_ptr();
    crate::src::opus_1_2_1::silk::float::apply_sine_window_FLP::silk_apply_sine_window_FLP(
        Wsig_ptr,
        x_buf_ptr,
        1 as i32,
        (*psEnc).sCmn.la_pitch,
    );
    /* Middle non-windowed samples */
    Wsig_ptr = Wsig_ptr.offset((*psEnc).sCmn.la_pitch as isize);
    x_buf_ptr = x_buf_ptr.offset((*psEnc).sCmn.la_pitch as isize);
    crate::stdlib::memcpy(
        Wsig_ptr as *mut libc::c_void,
        x_buf_ptr as *const libc::c_void,
        (((*psEnc).sCmn.pitch_LPC_win_length - ((*psEnc).sCmn.la_pitch << 1 as i32))
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
    );
    /* Last LA_LTP samples */
    Wsig_ptr = Wsig_ptr.offset(
        ((*psEnc).sCmn.pitch_LPC_win_length - ((*psEnc).sCmn.la_pitch << 1 as i32))
            as isize,
    );
    x_buf_ptr = x_buf_ptr.offset(
        ((*psEnc).sCmn.pitch_LPC_win_length - ((*psEnc).sCmn.la_pitch << 1 as i32))
            as isize,
    );
    crate::src::opus_1_2_1::silk::float::apply_sine_window_FLP::silk_apply_sine_window_FLP(
        Wsig_ptr,
        x_buf_ptr,
        2 as i32,
        (*psEnc).sCmn.la_pitch,
    );
    /* Calculate autocorrelation sequence */
    crate::src::opus_1_2_1::silk::float::autocorrelation_FLP::silk_autocorrelation_FLP(
        auto_corr.as_mut_ptr(),
        Wsig.as_mut_ptr(),
        (*psEnc).sCmn.pitch_LPC_win_length,
        (*psEnc).sCmn.pitchEstimationLPCOrder + 1 as i32,
    );
    /* Add white noise, as a fraction of the energy */
    auto_corr[0 as i32 as usize] +=
        auto_corr[0 as i32 as usize] * 1e-3f32 + 1 as i32 as f32;
    /* Calculate the reflection coefficients using Schur */
    res_nrg = crate::src::opus_1_2_1::silk::float::schur_FLP::silk_schur_FLP(
        refl_coef.as_mut_ptr(),
        auto_corr.as_mut_ptr() as *const f32,
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    /* Prediction gain */
    (*psEncCtrl).predGain =
        auto_corr[0 as i32 as usize] / (if res_nrg > 1.0f32 { res_nrg } else { 1.0f32 });
    /* Convert reflection coefficients to prediction coefficients */
    crate::src::opus_1_2_1::silk::float::k2a_FLP::silk_k2a_FLP(
        A.as_mut_ptr(),
        refl_coef.as_mut_ptr(),
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    /* Bandwidth expansion */
    crate::src::opus_1_2_1::silk::float::bwexpander_FLP::silk_bwexpander_FLP(
        A.as_mut_ptr(),
        (*psEnc).sCmn.pitchEstimationLPCOrder,
        0.99f32,
    );
    /* ****************************************/
    /* LPC analysis filtering                */
    /* ****************************************/
    crate::src::opus_1_2_1::silk::float::LPC_analysis_filter_FLP::silk_LPC_analysis_filter_FLP(
        res,
        A.as_mut_ptr() as *const f32,
        x_buf,
        buf_len,
        (*psEnc).sCmn.pitchEstimationLPCOrder,
    );
    if (*psEnc).sCmn.indices.signalType as i32 != 0 as i32
        && (*psEnc).sCmn.first_frame_after_reset == 0 as i32
    {
        /* Threshold for pitch estimator */
        thrhld = 0.6f32;
        thrhld -= 0.004f32 * (*psEnc).sCmn.pitchEstimationLPCOrder as f32;
        thrhld -= 0.1f32 * (*psEnc).sCmn.speech_activity_Q8 as f32 * (1.0f32 / 256.0f32);
        thrhld -= 0.15f32
            * ((*psEnc).sCmn.prevSignalType as i32 >> 1 as i32) as f32;
        thrhld -= 0.1f32 * (*psEnc).sCmn.input_tilt_Q15 as f32 * (1.0f32 / 32768.0f32);
        /* ****************************************/
        /* Call Pitch estimator                  */
        /* ****************************************/
        if crate::src::opus_1_2_1::silk::float::pitch_analysis_core_FLP::silk_pitch_analysis_core_FLP(res as *const f32,
                                        (*psEncCtrl).pitchL.as_mut_ptr(),
                                        &mut (*psEnc).sCmn.indices.lagIndex,
                                        &mut (*psEnc).sCmn.indices.contourIndex,
                                        &mut (*psEnc).LTPCorr,
                                        (*psEnc).sCmn.prevLag,
                                        (*psEnc).sCmn.pitchEstimationThreshold_Q16
                                            as f32 / 65536.0f32,
                                        thrhld, (*psEnc).sCmn.fs_kHz,
                                        (*psEnc).sCmn.pitchEstimationComplexity,
                                        (*psEnc).sCmn.nb_subfr, arch) ==
               0 as i32 {
            (*psEnc).sCmn.indices.signalType =
                2 as i32 as i8
        } else {
            (*psEnc).sCmn.indices.signalType =
                1 as i32 as i8
        }
    } else {
        crate::stdlib::memset(
            (*psEncCtrl).pitchL.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<[i32; 4]>() as libc::c_ulong,
        );
        (*psEnc).sCmn.indices.lagIndex = 0 as i32 as crate::opus_types_h::opus_int16;
        (*psEnc).sCmn.indices.contourIndex = 0 as i32 as i8;
        (*psEnc).LTPCorr = 0 as i32 as f32
    };
}

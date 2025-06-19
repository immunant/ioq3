use ::libc;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;

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
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* O    Residual                                    */
/* I    Speech signal                               */
/* I    Run-time architecture                       */
/* Find LPC and LTP coefficients */
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
/* Find LPC and LTP coefficients */
#[no_mangle]

pub unsafe extern "C" fn silk_find_pred_coefs_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut psEncCtrl: *mut silk_encoder_control_FLP,
    mut res_pitch: *const f32,
    mut x: *const f32,
    mut condCoding: i32,
)
/* I    The type of conditional coding to use       */
{
    let mut i: i32 = 0;
    let mut XXLTP: [f32; 100] = [0.; 100];
    let mut xXLTP: [f32; 20] = [0.; 20];
    let mut invGains: [f32; 4] = [0.; 4];
    let mut NLSF_Q15: [opus_int16; 16] = [0; 16];
    let mut x_ptr: *const f32 = 0 as *const f32;
    let mut x_pre_ptr: *mut f32 = 0 as *mut f32;
    let mut LPC_in_pre: [f32; 384] = [0.; 384];
    let mut minInvGain: f32 = 0.;
    /* Weighting for weighted least squares */
    i = 0 as i32;
    while i < (*psEnc).sCmn.nb_subfr {
        invGains[i as usize] = 1.0f32 / (*psEncCtrl).Gains[i as usize];
        i += 1
    }
    if (*psEnc).sCmn.indices.signalType as i32 == 2 as i32 {
        /* *********/
        /* VOICED */
        /* *********/
        /* LTP analysis */
        crate::src::opus_1_2_1::silk::float::find_LTP_FLP::silk_find_LTP_FLP(
            XXLTP.as_mut_ptr(),
            xXLTP.as_mut_ptr(),
            res_pitch,
            (*psEncCtrl).pitchL.as_mut_ptr() as *const i32,
            (*psEnc).sCmn.subfr_length,
            (*psEnc).sCmn.nb_subfr,
        );
        crate::src::opus_1_2_1::silk::float::wrappers_FLP::silk_quant_LTP_gains_FLP(
            (*psEncCtrl).LTPCoef.as_mut_ptr(),
            (*psEnc).sCmn.indices.LTPIndex.as_mut_ptr(),
            &mut (*psEnc).sCmn.indices.PERIndex,
            &mut (*psEnc).sCmn.sum_log_gain_Q7,
            &mut (*psEncCtrl).LTPredCodGain,
            XXLTP.as_mut_ptr() as *const f32,
            xXLTP.as_mut_ptr() as *const f32,
            (*psEnc).sCmn.subfr_length,
            (*psEnc).sCmn.nb_subfr,
            (*psEnc).sCmn.arch,
        );
        crate::src::opus_1_2_1::silk::float::LTP_scale_ctrl_FLP::silk_LTP_scale_ctrl_FLP(
            psEnc as *mut silk_encoder_state_FLP,
            psEncCtrl as *mut silk_encoder_control_FLP,
            condCoding,
        );
        crate::src::opus_1_2_1::silk::float::LTP_analysis_filter_FLP::silk_LTP_analysis_filter_FLP(
            LPC_in_pre.as_mut_ptr(),
            x.offset(-((*psEnc).sCmn.predictLPCOrder as isize)),
            (*psEncCtrl).LTPCoef.as_mut_ptr() as *const f32,
            (*psEncCtrl).pitchL.as_mut_ptr() as *const i32,
            invGains.as_mut_ptr() as *const f32,
            (*psEnc).sCmn.subfr_length,
            (*psEnc).sCmn.nb_subfr,
            (*psEnc).sCmn.predictLPCOrder,
        );
    } else {
        /* Quantize LTP gain parameters */
        /* Control LTP scaling */
        /* Create LTP residual */
        /* ***********/
        /* UNVOICED */
        /* ***********/
        /* Create signal with prepended subframes, scaled by inverse gains */
        x_ptr = x.offset(-((*psEnc).sCmn.predictLPCOrder as isize));
        x_pre_ptr = LPC_in_pre.as_mut_ptr();
        i = 0 as i32;
        while i < (*psEnc).sCmn.nb_subfr {
            crate::src::opus_1_2_1::silk::float::scale_copy_vector_FLP::silk_scale_copy_vector_FLP(
                x_pre_ptr,
                x_ptr,
                invGains[i as usize],
                (*psEnc).sCmn.subfr_length + (*psEnc).sCmn.predictLPCOrder,
            );
            x_pre_ptr = x_pre_ptr
                .offset(((*psEnc).sCmn.subfr_length + (*psEnc).sCmn.predictLPCOrder) as isize);
            x_ptr = x_ptr.offset((*psEnc).sCmn.subfr_length as isize);
            i += 1
        }
        crate::stdlib::memset(
            (*psEncCtrl).LTPCoef.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            (((*psEnc).sCmn.nb_subfr * 5 as i32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        );
        (*psEncCtrl).LTPredCodGain = 0.0f32;
        (*psEnc).sCmn.sum_log_gain_Q7 = 0 as i32
    }
    /* Limit on total predictive coding gain */
    if (*psEnc).sCmn.first_frame_after_reset != 0 {
        minInvGain = 1.0f32 / 1e2f32
    } else {
        minInvGain = crate::stdlib::pow(
            2 as i32 as f64,
            ((*psEncCtrl).LTPredCodGain / 3 as i32 as f32) as f64,
        ) as f32
            / 1e4f32;
        minInvGain /= 0.25f32 + 0.75f32 * (*psEncCtrl).coding_quality
    }
    /* LPC_in_pre contains the LTP-filtered input for voiced, and the unfiltered input for unvoiced */
    crate::src::opus_1_2_1::silk::float::find_LPC_FLP::silk_find_LPC_FLP(
        &mut (*psEnc).sCmn as *mut _ as *mut silk_encoder_state,
        NLSF_Q15.as_mut_ptr(),
        LPC_in_pre.as_mut_ptr() as *const f32,
        minInvGain,
    );
    /* Quantize LSFs */
    crate::src::opus_1_2_1::silk::float::wrappers_FLP::silk_process_NLSFs_FLP(
        &mut (*psEnc).sCmn as *mut _ as *mut silk_encoder_state,
        (*psEncCtrl).PredCoef.as_mut_ptr(),
        NLSF_Q15.as_mut_ptr(),
        (*psEnc).sCmn.prev_NLSFq_Q15.as_mut_ptr() as *const opus_int16,
    );
    /* Calculate residual energy using quantized LPC coefficients */
    crate::src::opus_1_2_1::silk::float::residual_energy_FLP::silk_residual_energy_FLP(
        (*psEncCtrl).ResNrg.as_mut_ptr(),
        LPC_in_pre.as_mut_ptr() as *const f32,
        (*psEncCtrl).PredCoef.as_mut_ptr(),
        (*psEncCtrl).Gains.as_mut_ptr() as *const f32,
        (*psEnc).sCmn.subfr_length,
        (*psEnc).sCmn.nb_subfr,
        (*psEnc).sCmn.predictLPCOrder,
    );
    /* Copy to prediction struct for use in next frame for interpolation */
    crate::stdlib::memcpy(
        (*psEnc).sCmn.prev_NLSFq_Q15.as_mut_ptr() as *mut libc::c_void,
        NLSF_Q15.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[opus_int16; 16]>() as libc::c_ulong,
    );
}

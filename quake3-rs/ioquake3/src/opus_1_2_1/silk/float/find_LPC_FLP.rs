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
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* I    Residual from pitch analysis                */
/* I    Speech signal                               */
/* I    The type of conditional coding to use       */
/* LPC analysis */
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
/* LPC analysis */
#[no_mangle]

pub unsafe extern "C" fn silk_find_LPC_FLP(
    mut psEncC: *mut crate::structs_h::silk_encoder_state,
    mut NLSF_Q15: *mut crate::opus_types_h::opus_int16,
    mut x: *const libc::c_float,
    minInvGain: libc::c_float,
)
/* I    Inverse of max prediction gain              */
{
    let mut k: libc::c_int = 0;
    let mut subfr_length: libc::c_int = 0;
    let mut a: [libc::c_float; 16] = [0.; 16];
    /* Used only for NLSF interpolation */
    let mut res_nrg: libc::c_float = 0.;
    let mut res_nrg_2nd: libc::c_float = 0.;
    let mut res_nrg_interp: libc::c_float = 0.;
    let mut NLSF0_Q15: [crate::opus_types_h::opus_int16; 16] = [0; 16];
    let mut a_tmp: [libc::c_float; 16] = [0.; 16];
    let mut LPC_res: [libc::c_float; 384] = [0.; 384];
    subfr_length = (*psEncC).subfr_length + (*psEncC).predictLPCOrder;
    /* Default: No interpolation */
    (*psEncC).indices.NLSFInterpCoef_Q2 = 4 as libc::c_int as libc::c_schar;
    /* Burg AR analysis for the full frame */
    res_nrg = crate::src::opus_1_2_1::silk::float::burg_modified_FLP::silk_burg_modified_FLP(
        a.as_mut_ptr(),
        x,
        minInvGain,
        subfr_length,
        (*psEncC).nb_subfr,
        (*psEncC).predictLPCOrder,
    );
    if (*psEncC).useInterpolatedNLSFs != 0
        && (*psEncC).first_frame_after_reset == 0
        && (*psEncC).nb_subfr == 4 as libc::c_int
    {
        /* Optimal solution for last 10 ms; subtract residual energy here, as that's easier than        */
        /* adding it to the residual energy of the first 10 ms in each iteration of the search below    */
        res_nrg -= crate::src::opus_1_2_1::silk::float::burg_modified_FLP::silk_burg_modified_FLP(
            a_tmp.as_mut_ptr(),
            x.offset((4 as libc::c_int / 2 as libc::c_int * subfr_length) as isize),
            minInvGain,
            subfr_length,
            4 as libc::c_int / 2 as libc::c_int,
            (*psEncC).predictLPCOrder,
        );
        /* Convert to NLSFs */
        crate::src::opus_1_2_1::silk::float::wrappers_FLP::silk_A2NLSF_FLP(
            NLSF_Q15,
            a_tmp.as_mut_ptr(),
            (*psEncC).predictLPCOrder,
        );
        /* Search over interpolation indices to find the one with lowest residual energy */
        res_nrg_2nd = 3.40282347e+38f32;
        k = 3 as libc::c_int;
        while k >= 0 as libc::c_int {
            /* Interpolate NLSFs for first half */
            crate::src::opus_1_2_1::silk::interpolate::silk_interpolate(
                NLSF0_Q15.as_mut_ptr(),
                (*psEncC).prev_NLSFq_Q15.as_mut_ptr() as *const crate::opus_types_h::opus_int16,
                NLSF_Q15 as *const crate::opus_types_h::opus_int16,
                k,
                (*psEncC).predictLPCOrder,
            );
            /* Convert to LPC for residual energy evaluation */
            crate::src::opus_1_2_1::silk::float::wrappers_FLP::silk_NLSF2A_FLP(
                a_tmp.as_mut_ptr(),
                NLSF0_Q15.as_mut_ptr(),
                (*psEncC).predictLPCOrder,
                (*psEncC).arch,
            );
            /* Calculate residual energy with LSF interpolation */
            crate::src::opus_1_2_1::silk::float::LPC_analysis_filter_FLP::silk_LPC_analysis_filter_FLP(LPC_res.as_mut_ptr(),
                                         a_tmp.as_mut_ptr() as
                                             *const libc::c_float, x,
                                         2 as libc::c_int * subfr_length,
                                         (*psEncC).predictLPCOrder);
            res_nrg_interp = (crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
                LPC_res
                    .as_mut_ptr()
                    .offset((*psEncC).predictLPCOrder as isize),
                subfr_length - (*psEncC).predictLPCOrder,
            ) + crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
                LPC_res
                    .as_mut_ptr()
                    .offset((*psEncC).predictLPCOrder as isize)
                    .offset(subfr_length as isize),
                subfr_length - (*psEncC).predictLPCOrder,
            )) as libc::c_float;
            /* Determine whether current interpolated NLSFs are best so far */
            if res_nrg_interp < res_nrg {
                /* Interpolation has lower residual energy */
                res_nrg = res_nrg_interp;
                (*psEncC).indices.NLSFInterpCoef_Q2 = k as libc::c_schar
            } else if res_nrg_interp > res_nrg_2nd {
                break;
            }
            res_nrg_2nd = res_nrg_interp;
            k -= 1
        }
    }
    if (*psEncC).indices.NLSFInterpCoef_Q2 as libc::c_int == 4 as libc::c_int {
        /* NLSF interpolation is currently inactive, calculate NLSFs from full frame AR coefficients */
        crate::src::opus_1_2_1::silk::float::wrappers_FLP::silk_A2NLSF_FLP(
            NLSF_Q15,
            a.as_mut_ptr(),
            (*psEncC).predictLPCOrder,
        );
    };
}

use ::libc;

pub mod float_cast_h {
    /* Copyright (C) 2001 Erik de Castro Lopo <erikd AT mega-nerd DOT com> */
    /*
       Redistribution and use in source and binary forms, with or without
       modification, are permitted provided that the following conditions
       are met:

       - Redistributions of source code must retain the above copyright
       notice, this list of conditions and the following disclaimer.

       - Redistributions in binary form must reproduce the above copyright
       notice, this list of conditions and the following disclaimer in the
       documentation and/or other materials provided with the distribution.

       THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
       ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
       LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
       A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
       OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
       EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
       PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
       PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
       LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
       NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
       SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
    */
    /* Version 1.1 */
    /*============================================================================
     **      On Intel Pentium processors (especially PIII and probably P4), converting
     **      from float to int is very slow. To meet the C specs, the code produced by
     **      most C compilers targeting Pentium needs to change the FPU rounding mode
     **      before the float to int conversion is performed.
     **
     **      Changing the FPU rounding mode causes the FPU pipeline to be flushed. It
     **      is this flushing of the pipeline which is so slow.
     **
     **      Fortunately the ISO C99 specifications define the functions lrint, lrintf,
     **      llrint and llrintf which fix this problem as a side effect.
     **
     **      On Unix-like systems, the configure process should have detected the
     **      presence of these functions. If they weren't found we have to replace them
     **      here with a standard C cast.
     */
    /*
     **      The C99 prototypes for lrint and lrintf are as follows:
     **
     **              long int lrintf (float x) ;
     **              long int lrint  (double x) ;
     */
    /*      The presence of the required functions are detected during the configure
     **      process and the values HAVE_LRINT and HAVE_LRINTF are set accordingly in
     **      the config.h file.
     */
    /* With GCC, when SSE is available, the fastest conversion is cvtss2si. */
    #[inline]

    pub unsafe extern "C" fn float2int(mut x: f32) -> crate::opus_types_h::opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }

    use ::std::arch::x86_64::_mm_cvt_ss2si;
    use ::std::arch::x86_64::_mm_set_ss;
    /* FLOAT_CAST_H */
    /* DISABLE_FLOAT_API */
}

pub mod SigProc_FLP_h {
    /* floating-point to integer conversion (rounding) */
    #[inline]

    pub unsafe extern "C" fn silk_float2int(
        mut x: f32,
    ) -> crate::opus_types_h::opus_int32 {
        return float2int(x);
    }

    use crate::src::opus_1_2_1::silk::float::wrappers_FLP::float_cast_h::float2int;
    /* SILK_SIGPROC_FLP_H */
}

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint16;
pub use crate::opus_types_h::opus_uint32;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
pub use crate::src::opus_1_2_1::silk::float::wrappers_FLP::float_cast_h::float2int;
pub use crate::src::opus_1_2_1::silk::float::wrappers_FLP::SigProc_FLP_h::silk_float2int;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
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
/* Wrappers. Calls flp / fix code */
/* Convert AR filter coefficients to NLSF parameters */
#[no_mangle]

pub unsafe extern "C" fn silk_A2NLSF_FLP(
    mut NLSF_Q15: *mut crate::opus_types_h::opus_int16,
    mut pAR: *const f32,
    LPC_order: i32,
)
/* I    LPC order                                   */
{
    let mut i: i32 = 0;
    let mut a_fix_Q16: [crate::opus_types_h::opus_int32; 16] = [0; 16];
    i = 0 as i32;
    while i < LPC_order {
        a_fix_Q16[i as usize] = silk_float2int(*pAR.offset(i as isize) * 65536.0f32);
        i += 1
    }
    crate::src::opus_1_2_1::silk::A2NLSF::silk_A2NLSF(NLSF_Q15, a_fix_Q16.as_mut_ptr(), LPC_order);
}
/* Convert LSF parameters to AR prediction filter coefficients */
#[no_mangle]

pub unsafe extern "C" fn silk_NLSF2A_FLP(
    mut pAR: *mut f32,
    mut NLSF_Q15: *const crate::opus_types_h::opus_int16,
    LPC_order: i32,
    mut arch: i32,
)
/* I    Run-time architecture                       */
{
    let mut i: i32 = 0;
    let mut a_fix_Q12: [crate::opus_types_h::opus_int16; 16] = [0; 16];
    crate::src::opus_1_2_1::silk::NLSF2A::silk_NLSF2A(
        a_fix_Q12.as_mut_ptr(),
        NLSF_Q15,
        LPC_order,
        arch,
    );
    i = 0 as i32;
    while i < LPC_order {
        *pAR.offset(i as isize) = a_fix_Q12[i as usize] as f32 * (1.0f32 / 4096.0f32);
        i += 1
    }
}
/* *****************************************/
/* Floating-point NLSF processing wrapper */
/* *****************************************/
#[no_mangle]

pub unsafe extern "C" fn silk_process_NLSFs_FLP(
    mut psEncC: *mut crate::structs_h::silk_encoder_state,
    mut PredCoef: *mut [f32; 16],
    mut NLSF_Q15: *mut crate::opus_types_h::opus_int16,
    mut prev_NLSF_Q15: *const crate::opus_types_h::opus_int16,
)
/* I    Previous Normalized LSFs (0 - (2^15-1))     */
{
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut PredCoef_Q12: [[crate::opus_types_h::opus_int16; 16]; 2] = [[0; 16]; 2];
    crate::src::opus_1_2_1::silk::process_NLSFs::silk_process_NLSFs(
        psEncC as *mut crate::structs_h::silk_encoder_state,
        PredCoef_Q12.as_mut_ptr(),
        NLSF_Q15,
        prev_NLSF_Q15,
    );
    j = 0 as i32;
    while j < 2 as i32 {
        i = 0 as i32;
        while i < (*psEncC).predictLPCOrder {
            (*PredCoef.offset(j as isize))[i as usize] =
                PredCoef_Q12[j as usize][i as usize] as f32 * (1.0f32 / 4096.0f32);
            i += 1
        }
        j += 1
    }
}
/* Residual energy: nrg = wxx - 2 * wXx * c + c' * wXX * c */
/* O    Weighted residual energy                    */
/* I    Filter coefficients                         */
/* I/O  Weighted correlation matrix, reg. out       */
/* I    Weighted correlation vector                 */
/* I    Weighted correlation value                  */
/* I    Dimension                                   */
/* Processing of gains */
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* I    The type of conditional coding to use       */
/* *****************/
/* Linear Algebra */
/* *****************/
/* Calculates correlation matrix X'*X */
/* I    x vector [ L+order-1 ] used to create X     */
/* I    Length of vectors                           */
/* I    Max lag for correlation                     */
/* O    X'*X correlation matrix [order x order]     */
/* Calculates correlation vector X'*t */
/* I    x vector [L+order-1] used to create X       */
/* I    Target vector [L]                           */
/* I    Length of vecors                            */
/* I    Max lag for correlation                     */
/* O    X'*t correlation vector [order]             */
/* Apply sine window to signal vector.  */
/* Window types:                        */
/*  1 -> sine window from 0 to pi/2     */
/*  2 -> sine window from pi/2 to pi    */
/* O    Pointer to windowed signal                  */
/* I    Pointer to input signal                     */
/* I    Selects a window type                       */
/* I    Window length, multiple of 4                */
/* Wrapper functions. Call flp / fix code */
/* Convert AR filter coefficients to NLSF parameters */
/* O    NLSF vector      [ LPC_order ]              */
/* I    LPC coefficients [ LPC_order ]              */
/* I    LPC order                                   */
/* Convert NLSF parameters to AR prediction filter coefficients */
/* O    LPC coefficients [ LPC_order ]              */
/* I    NLSF vector      [ LPC_order ]              */
/* I    LPC order                                   */
/* I    Run-time architecture                       */
/* Limit, stabilize, and quantize NLSFs */
/* I/O  Encoder state                               */
/* O    Prediction coefficients                     */
/* I/O  Normalized LSFs (quant out) (0 - (2^15-1))  */
/* I    Previous Normalized LSFs (0 - (2^15-1))     */
/* Floating-point Silk NSQ wrapper      */
/* ***************************************/
/* Floating-point Silk NSQ wrapper      */
/* ***************************************/
#[no_mangle]

pub unsafe extern "C" fn silk_NSQ_wrapper_FLP(
    mut psEnc: *mut crate::structs_FLP_h::silk_encoder_state_FLP,
    mut psEncCtrl: *mut crate::structs_FLP_h::silk_encoder_control_FLP,
    mut psIndices: *mut crate::structs_h::SideInfoIndices,
    mut psNSQ: *mut crate::structs_h::silk_nsq_state,
    mut pulses: *mut i8,
    mut x: *const f32,
)
/* I    Prefiltered input signal                    */
{
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut x16: [crate::opus_types_h::opus_int16; 320] = [0; 320];
    let mut Gains_Q16: [crate::opus_types_h::opus_int32; 4] = [0; 4];
    let mut PredCoef_Q12: [[crate::opus_types_h::opus_int16; 16]; 2] = [[0; 16]; 2];
    let mut LTPCoef_Q14: [crate::opus_types_h::opus_int16; 20] = [0; 20];
    let mut LTP_scale_Q14: i32 = 0;
    /* Noise shaping parameters */
    let mut AR_Q13: [crate::opus_types_h::opus_int16; 96] = [0; 96]; /* Packs two int16 coefficients per int32 value             */
    let mut LF_shp_Q14: [crate::opus_types_h::opus_int32; 4] = [0; 4];
    let mut Lambda_Q10: i32 = 0;
    let mut Tilt_Q14: [i32; 4] = [0; 4];
    let mut HarmShapeGain_Q14: [i32; 4] = [0; 4];
    /* Convert control struct to fix control struct */
    /* Noise shape parameters */
    i = 0 as i32;
    while i < (*psEnc).sCmn.nb_subfr {
        j = 0 as i32;
        while j < (*psEnc).sCmn.shapingLPCOrder {
            AR_Q13[(i * 24 as i32 + j) as usize] =
                silk_float2int((*psEncCtrl).AR[(i * 24 as i32 + j) as usize] * 8192.0f32)
                    as crate::opus_types_h::opus_int16;
            j += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i < (*psEnc).sCmn.nb_subfr {
        LF_shp_Q14[i as usize] = ((silk_float2int((*psEncCtrl).LF_AR_shp[i as usize] * 16384.0f32)
            as crate::opus_types_h::opus_uint32)
            << 16 as i32)
            as crate::opus_types_h::opus_int32
            | silk_float2int((*psEncCtrl).LF_MA_shp[i as usize] * 16384.0f32)
                as crate::opus_types_h::opus_uint16 as i32;
        Tilt_Q14[i as usize] = silk_float2int((*psEncCtrl).Tilt[i as usize] * 16384.0f32);
        HarmShapeGain_Q14[i as usize] =
            silk_float2int((*psEncCtrl).HarmShapeGain[i as usize] * 16384.0f32);
        i += 1
    }
    Lambda_Q10 = silk_float2int((*psEncCtrl).Lambda * 1024.0f32);
    /* prediction and coding parameters */
    i = 0 as i32;
    while i < (*psEnc).sCmn.nb_subfr * 5 as i32 {
        LTPCoef_Q14[i as usize] = silk_float2int((*psEncCtrl).LTPCoef[i as usize] * 16384.0f32)
            as crate::opus_types_h::opus_int16;
        i += 1
    }
    j = 0 as i32;
    while j < 2 as i32 {
        i = 0 as i32;
        while i < (*psEnc).sCmn.predictLPCOrder {
            PredCoef_Q12[j as usize][i as usize] =
                silk_float2int((*psEncCtrl).PredCoef[j as usize][i as usize] * 4096.0f32)
                    as crate::opus_types_h::opus_int16;
            i += 1
        }
        j += 1
    }
    i = 0 as i32;
    while i < (*psEnc).sCmn.nb_subfr {
        Gains_Q16[i as usize] = silk_float2int((*psEncCtrl).Gains[i as usize] * 65536.0f32);
        i += 1
    }
    if (*psIndices).signalType as i32 == 2 as i32 {
        LTP_scale_Q14 = crate::src::opus_1_2_1::silk::tables_other::silk_LTPScales_table_Q14
            [(*psIndices).LTP_scaleIndex as usize] as i32
    } else {
        LTP_scale_Q14 = 0 as i32
    }
    /* Convert input to fix */
    i = 0 as i32;
    while i < (*psEnc).sCmn.frame_length {
        x16[i as usize] = silk_float2int(*x.offset(i as isize)) as crate::opus_types_h::opus_int16;
        i += 1
    }
    /* Call NSQ */
    if (*psEnc).sCmn.nStatesDelayedDecision > 1 as i32
        || (*psEnc).sCmn.warping_Q16 > 0 as i32
    {
        crate::src::opus_1_2_1::silk::NSQ_del_dec::silk_NSQ_del_dec_c(
            &mut (*psEnc).sCmn as *mut _ as *const crate::structs_h::silk_encoder_state,
            psNSQ as *mut crate::structs_h::silk_nsq_state,
            psIndices as *mut crate::structs_h::SideInfoIndices,
            x16.as_mut_ptr() as *const crate::opus_types_h::opus_int16,
            pulses,
            PredCoef_Q12[0 as i32 as usize].as_mut_ptr()
                as *const crate::opus_types_h::opus_int16,
            LTPCoef_Q14.as_mut_ptr() as *const crate::opus_types_h::opus_int16,
            AR_Q13.as_mut_ptr() as *const crate::opus_types_h::opus_int16,
            HarmShapeGain_Q14.as_mut_ptr() as *const i32,
            Tilt_Q14.as_mut_ptr() as *const i32,
            LF_shp_Q14.as_mut_ptr() as *const crate::opus_types_h::opus_int32,
            Gains_Q16.as_mut_ptr() as *const crate::opus_types_h::opus_int32,
            (*psEncCtrl).pitchL.as_mut_ptr() as *const i32,
            Lambda_Q10,
            LTP_scale_Q14,
        );
    } else {
        crate::src::opus_1_2_1::silk::NSQ::silk_NSQ_c(
            &mut (*psEnc).sCmn as *mut _ as *const crate::structs_h::silk_encoder_state,
            psNSQ as *mut crate::structs_h::silk_nsq_state,
            psIndices as *mut crate::structs_h::SideInfoIndices,
            x16.as_mut_ptr() as *const crate::opus_types_h::opus_int16,
            pulses,
            PredCoef_Q12[0 as i32 as usize].as_mut_ptr()
                as *const crate::opus_types_h::opus_int16,
            LTPCoef_Q14.as_mut_ptr() as *const crate::opus_types_h::opus_int16,
            AR_Q13.as_mut_ptr() as *const crate::opus_types_h::opus_int16,
            HarmShapeGain_Q14.as_mut_ptr() as *const i32,
            Tilt_Q14.as_mut_ptr() as *const i32,
            LF_shp_Q14.as_mut_ptr() as *const crate::opus_types_h::opus_int32,
            Gains_Q16.as_mut_ptr() as *const crate::opus_types_h::opus_int32,
            (*psEncCtrl).pitchL.as_mut_ptr() as *const i32,
            Lambda_Q10,
            LTP_scale_Q14,
        );
    };
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
/* I/O  Encoder state                               */
/* O    NLSFs                                       */
/* I    Input signal                                */
/* I    Prediction gain from LTP (dB)               */
/* LTP analysis */
/* O    Weight for LTP quantization         */
/* O    Weight for LTP quantization                 */
/* I    LPC residual                                */
/* I    LTP lags                                    */
/* I    Subframe length                             */
/* I    number of subframes                         */
/* O    LTP res MAX_NB_SUBFR*(pre_lgth+subfr_lngth) */
/* I    Input signal, with preceding samples        */
/* I    LTP coefficients for each subframe          */
/* I    Pitch lags                                  */
/* I    Inverse quantization gains                  */
/* I    Length of each subframe                     */
/* I    number of subframes                         */
/* I    Preceding samples for each subframe         */
/* Calculates residual energies of input subframes where all subframes have LPC_order   */
/* of preceding samples                                                                 */
/* O    Residual energy per subframe                */
/* I    Input signal                                */
/* I    AR coefs for each frame half                */
/* I    Quantization gains                          */
/* I    Subframe length                             */
/* I    number of subframes                         */
/* I    LPC order                                   */
/* 16th order LPC analysis filter */
/* O    LPC residual signal                         */
/* I    LPC coefficients                            */
/* I    Input signal                                */
/* I    Length of input signal                      */
/* I    LPC order                                   */
/* LTP tap quantizer */
/* **********************************************/
/* Floating-point Silk LTP quantiation wrapper */
/* **********************************************/
#[no_mangle]

pub unsafe extern "C" fn silk_quant_LTP_gains_FLP(
    mut B: *mut f32,
    mut cbk_index: *mut i8,
    mut periodicity_index: *mut i8,
    mut sum_log_gain_Q7: *mut crate::opus_types_h::opus_int32,
    mut pred_gain_dB: *mut f32,
    mut XX: *const f32,
    mut xX: *const f32,
    subfr_len: i32,
    nb_subfr: i32,
    mut arch: i32,
)
/* I    Run-time architecture                       */
{
    let mut i: i32 = 0;
    let mut pred_gain_dB_Q7: i32 = 0;
    let mut B_Q14: [crate::opus_types_h::opus_int16; 20] = [0; 20];
    let mut XX_Q17: [crate::opus_types_h::opus_int32; 100] = [0; 100];
    let mut xX_Q17: [crate::opus_types_h::opus_int32; 20] = [0; 20];
    i = 0 as i32;
    while i < nb_subfr * 5 as i32 * 5 as i32 {
        XX_Q17[i as usize] = silk_float2int(*XX.offset(i as isize) * 131072.0f32);
        i += 1
    }
    i = 0 as i32;
    while i < nb_subfr * 5 as i32 {
        xX_Q17[i as usize] = silk_float2int(*xX.offset(i as isize) * 131072.0f32);
        i += 1
    }
    crate::src::opus_1_2_1::silk::quant_LTP_gains::silk_quant_LTP_gains(
        B_Q14.as_mut_ptr(),
        cbk_index,
        periodicity_index,
        sum_log_gain_Q7,
        &mut pred_gain_dB_Q7,
        XX_Q17.as_mut_ptr() as *const crate::opus_types_h::opus_int32,
        xX_Q17.as_mut_ptr() as *const crate::opus_types_h::opus_int32,
        subfr_len,
        nb_subfr,
        arch,
    );
    i = 0 as i32;
    while i < nb_subfr * 5 as i32 {
        *B.offset(i as isize) = B_Q14[i as usize] as f32 * (1.0f32 / 16384.0f32);
        i += 1
    }
    *pred_gain_dB = pred_gain_dB_Q7 as f32 * (1.0f32 / 128.0f32);
}

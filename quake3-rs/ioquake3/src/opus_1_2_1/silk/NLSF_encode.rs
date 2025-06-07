use ::libc;

pub mod macros_h {
    #[inline]

    pub unsafe extern "C" fn silk_CLZ32(
        mut in32: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        return if in32 != 0 {
            (32 as libc::c_int)
                - (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }

    /* SILK_MACROS_H */
    /* Column based */
    /* Row based */
}

pub mod Inlines_h {
    /* Divide two int32 values and return result as int32 in a given Q-domain */
    #[inline]

    pub unsafe extern "C" fn silk_DIV32_varQ(
        a32: crate::opus_types_h::opus_int32,
        b32: crate::opus_types_h::opus_int32,
        Qres: libc::c_int,
    ) -> crate::opus_types_h::opus_int32
/* I    Q-domain of result (>= 0)       */ {
        let mut a_headrm: libc::c_int = 0;
        let mut b_headrm: libc::c_int = 0;
        let mut lshift: libc::c_int = 0;
        let mut b32_inv: crate::opus_types_h::opus_int32 = 0;
        let mut a32_nrm: crate::opus_types_h::opus_int32 = 0;
        let mut b32_nrm: crate::opus_types_h::opus_int32 = 0;
        let mut result: crate::opus_types_h::opus_int32 = 0;
        /* Compute number of bits head room and normalize inputs */
        a_headrm = silk_CLZ32(if a32 > 0 as libc::c_int { a32 } else { -a32 }) - 1 as libc::c_int; /* Q: a_headrm                  */
        a32_nrm = ((a32 as crate::opus_types_h::opus_uint32) << a_headrm)
            as crate::opus_types_h::opus_int32; /* Q: b_headrm                  */
        b_headrm = silk_CLZ32(if b32 > 0 as libc::c_int { b32 } else { -b32 }) - 1 as libc::c_int;
        b32_nrm = ((b32 as crate::opus_types_h::opus_uint32) << b_headrm)
            as crate::opus_types_h::opus_int32;
        /* Inverse of b32, with 14 bits of precision */
        b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int) / (b32_nrm >> 16 as libc::c_int); /* Q: 29 + 16 - b_headrm        */
        /* First approximation */
        result = (a32_nrm as libc::c_longlong
            * b32_inv as crate::opus_types_h::opus_int16 as libc::c_longlong
            >> 16 as libc::c_int) as crate::opus_types_h::opus_int32; /* Q: 29 + a_headrm - b_headrm  */
        /* Compute residual by subtracting product of denominator and first approximation */
        /* It's OK to overflow because the final value of a32_nrm should always be small */
        a32_nrm = (a32_nrm as crate::opus_types_h::opus_uint32).wrapping_sub(
            (((b32_nrm as libc::c_longlong * result as libc::c_longlong >> 32 as libc::c_int)
                as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_uint32)
                << 3 as libc::c_int) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_uint32,
        ) as crate::opus_types_h::opus_int32; /* Q: a_headrm   */
        /* Refinement */
        result = (result as libc::c_longlong
            + (a32_nrm as libc::c_longlong
                * b32_inv as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32; /* Q: 29 + a_headrm - b_headrm  */
        /* Convert to Qres domain */
        lshift = 29 as libc::c_int + a_headrm - b_headrm - Qres;
        if lshift < 0 as libc::c_int {
            return (((if 0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32 >> -lshift
                > 0x7fffffff as libc::c_int >> -lshift
            {
                (if result
                    > 0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32 >> -lshift
                {
                    (0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32) >> -lshift
                } else {
                    (if result < 0x7fffffff as libc::c_int >> -lshift {
                        (0x7fffffff as libc::c_int) >> -lshift
                    } else {
                        result
                    })
                })
            } else {
                (if result > 0x7fffffff as libc::c_int >> -lshift {
                    (0x7fffffff as libc::c_int) >> -lshift
                } else {
                    (if result
                        < 0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32 >> -lshift
                    {
                        (0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32) >> -lshift
                    } else {
                        result
                    })
                })
            }) as crate::opus_types_h::opus_uint32)
                << -lshift) as crate::opus_types_h::opus_int32;
        } else if lshift < 32 as libc::c_int {
            return result >> lshift;
        } else {
            /* Avoid undefined result */
            return 0 as libc::c_int;
        };
    }

    use crate::src::opus_1_2_1::silk::NLSF_encode::macros_h::silk_CLZ32;
    /* SILK_FIX_INLINES_H */
}

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint16;
pub use crate::opus_types_h::opus_uint32;

pub use crate::src::opus_1_2_1::silk::NLSF_encode::macros_h::silk_CLZ32;
pub use crate::src::opus_1_2_1::silk::NLSF_encode::Inlines_h::silk_DIV32_varQ;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;

pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::structs_h::silk_NLSF_CB_struct;
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
/* Convert Left/Right stereo signal to adaptive Mid/Side representation */
/* I/O  State                                       */
/* I/O  Left input signal, becomes mid signal       */
/* I/O  Right input signal, becomes side signal     */
/* O    Quantization indices                        */
/* O    Flag: only mid signal coded                 */
/* O    Bitrates for mid and side signals           */
/* I    Total bitrate                               */
/* I    Speech activity level in previous frame     */
/* I    Last frame before a stereo->mono transition */
/* I    Sample rate (kHz)                           */
/* I    Number of samples                           */
/* Convert adaptive Mid/Side representation to Left/Right stereo signal */
/* I/O  State                                       */
/* I/O  Left input signal, becomes mid signal       */
/* I/O  Right input signal, becomes side signal     */
/* I    Predictors                                  */
/* I    Samples rate (kHz)                          */
/* I    Number of samples                           */
/* Find least-squares prediction gain for one signal based on another and quantize it */
/* O    Returns predictor in Q13                    */
/* O    Ratio of residual and mid energies          */
/* I    Basis signal                                */
/* I    Target signal                               */
/* I/O  Smoothed mid, residual norms                */
/* I    Number of samples                           */
/* I    Smoothing coefficient                       */
/* Quantize mid/side predictors */
/* I/O  Predictors (out: quantized)                 */
/* O    Quantization indices                        */
/* Entropy code the mid/side quantization indices */
/* I/O  Compressor data structure                   */
/* I    Quantization indices                        */
/* Entropy code the mid-only flag */
/* I/O  Compressor data structure                   */
/* Decode mid/side predictors */
/* I/O  Compressor data structure                   */
/* O    Predictors                                  */
/* Decode mid-only flag */
/* I/O  Compressor data structure                   */
/* O    Flag that only mid channel has been coded   */
/* Encodes signs of excitation */
/* I/O  Compressor data structure               */
/* I    pulse signal                            */
/* I    length of input                         */
/* I    Signal type                             */
/* I    Quantization offset type                */
/* I    Sum of absolute pulses per block        */
/* Decodes signs of excitation */
/* I/O  Compressor data structure               */
/* I/O  pulse signal                            */
/* I    length of input                         */
/* I    Signal type                             */
/* I    Quantization offset type                */
/* I    Sum of absolute pulses per block        */
/* Check encoder control struct */
/* I    Control structure                           */
/* Control internal sampling rate */
/* I/O  Pointer to Silk encoder state               */
/* I    Control structure                           */
/* Control SNR of redidual quantizer */
/* I/O  Pointer to Silk encoder state               */
/* I    Target max bitrate (bps)                    */
/* **************/
/* Shell coder */
/* **************/
/* Encode quantization indices of excitation */
/* I/O  compressor data structure                   */
/* I    Signal type                                 */
/* I    quantOffsetType                             */
/* I    quantization indices                        */
/* I    Frame length                                */
/* Shell encoder, operates on one shell code frame of 16 pulses */
/* I/O  compressor data structure                   */
/* I    data: nonnegative pulse amplitudes          */
/* Shell decoder, operates on one shell code frame of 16 pulses */
/* O    data: nonnegative pulse amplitudes          */
/* I/O  Compressor data structure                   */
/* I    number of pulses per pulse-subframe         */
/* Gain scalar quantization with hysteresis, uniform on log scale */
/* O    gain indices                                */
/* I/O  gains (quantized out)                       */
/* I/O  last index in previous frame                */
/* I    first gain is delta coded if 1              */
/* I    number of subframes                         */
/* Gains scalar dequantization, uniform on log scale */
/* O    quantized gains                             */
/* I    gain indices                                */
/* I/O  last index in previous frame                */
/* I    first gain is delta coded if 1              */
/* I    number of subframes                          */
/* Compute unique identifier of gain indices vector */
/* O    returns unique identifier of gains          */
/* I    gain indices                                */
/* I    number of subframes                         */
/* Interpolate two vectors */
/* O    interpolated vector                         */
/* I    first vector                                */
/* I    second vector                               */
/* I    interp. factor, weight on 2nd vector        */
/* I    number of parameters                        */
/* LTP tap quantizer */
/* O    Quantized LTP gains             */
/* O    Codebook Index                  */
/* O    Periodicity Index               */
/* I/O  Cumulative max prediction gain  */
/* O    LTP prediction gain             */
/* I    Correlation matrix in Q18       */
/* I    Correlation vector in Q18       */
/* I    Number of samples per subframe  */
/* I    Number of subframes             */
/* I    Run-time architecture           */
/* Entropy constrained matrix-weighted VQ, for a single input data vector */
/* O    index of best codebook vector               */
/* O    best residual energy                        */
/* O    best total bitrate                          */
/* O    sum of absolute LTP coefficients            */
/* I    correlation matrix                          */
/* I    correlation vector                          */
/* I    codebook                                    */
/* I    codebook effective gain                     */
/* I    code length for each codebook vector        */
/* I    number of samples per subframe              */
/* I    maximum sum of absolute LTP coefficients    */
/* I    number of vectors in codebook               */
/* ***********************************/
/* Noise shaping quantization (NSQ) */
/* ***********************************/
/* I    Encoder State                   */
/* I/O  NSQ state                       */
/* I/O  Quantization Indices            */
/* I    Input                           */
/* O    Quantized pulse signal          */
/* I    Short term prediction coefs     */
/* I    Long term prediction coefs      */
/* I  Noise shaping coefs             */
/* I    Long term shaping coefs         */
/* I    Spectral tilt                   */
/* I    Low frequency shaping coefs     */
/* I    Quantization step sizes         */
/* I    Pitch lags                      */
/* I    Rate/distortion tradeoff        */
/* I    LTP state scaling               */
/* Noise shaping using delayed decision */
/* I    Encoder State                   */
/* I/O  NSQ state                       */
/* I/O  Quantization Indices            */
/* I    Input                           */
/* O    Quantized pulse signal          */
/* I    Short term prediction coefs     */
/* I    Long term prediction coefs      */
/* I  Noise shaping coefs             */
/* I    Long term shaping coefs         */
/* I    Spectral tilt                   */
/* I    Low frequency shaping coefs     */
/* I    Quantization step sizes         */
/* I    Pitch lags                      */
/* I    Rate/distortion tradeoff        */
/* I    LTP state scaling               */
/* ***********/
/* Silk VAD */
/* ***********/
/* Initialize the Silk VAD */
/* O    Return value, 0 if success                  */
/* I/O  Pointer to Silk VAD state                   */
/* Get speech activity level in Q8 */
/* O    Return value, 0 if success                  */
/* I/O  Encoder state                               */
/* I    PCM input                                   */
/* Low-pass filter with variable cutoff frequency based on  */
/* piece-wise linear interpolation between elliptic filters */
/* Start by setting transition_frame_no = 1;                */
/* I/O  LP filter state                             */
/* I/O  Low-pass filtered output signal             */
/* I    Frame length                                */
/* *****************/
/* NLSF Quantizer */
/* *****************/
/* Limit, stabilize, convert and quantize NLSFs */
/* I/O  Encoder state                               */
/* O    Prediction coefficients                     */
/* I/O  Normalized LSFs (quant out) (0 - (2^15-1))  */
/* I    Previous Normalized LSFs (0 - (2^15-1))     */
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
/* **********************/
/* NLSF vector encoder */
/* **********************/
#[no_mangle]

pub unsafe extern "C" fn silk_NLSF_encode(
    mut NLSFIndices: *mut libc::c_schar,
    mut pNLSF_Q15: *mut crate::opus_types_h::opus_int16,
    mut psNLSF_CB: *const crate::structs_h::silk_NLSF_CB_struct,
    mut pW_Q2: *const crate::opus_types_h::opus_int16,
    NLSF_mu_Q20: libc::c_int,
    nSurvivors: libc::c_int,
    signalType: libc::c_int,
) -> crate::opus_types_h::opus_int32
/* I    Signal type: 0/1/2                          */ {
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut ind1: libc::c_int = 0;
    let mut bestIndex: libc::c_int = 0;
    let mut prob_Q8: libc::c_int = 0;
    let mut bits_q7: libc::c_int = 0;
    let mut W_tmp_Q9: crate::opus_types_h::opus_int32 = 0;
    let mut ret: crate::opus_types_h::opus_int32 = 0;
    let mut err_Q24: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut RD_Q25: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut tempIndices1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tempIndices2: *mut libc::c_schar = 0 as *mut libc::c_schar;
    let mut res_Q10: [crate::opus_types_h::opus_int16; 16] = [0; 16];
    let mut NLSF_tmp_Q15: [crate::opus_types_h::opus_int16; 16] = [0; 16];
    let mut W_adj_Q5: [crate::opus_types_h::opus_int16; 16] = [0; 16];
    let mut pred_Q8: [libc::c_uchar; 16] = [0; 16];
    let mut ec_ix: [crate::opus_types_h::opus_int16; 16] = [0; 16];
    let mut pCB_element: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut iCDF_ptr: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut pCB_Wght_Q9: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    /* NLSF stabilization */
    crate::src::opus_1_2_1::silk::NLSF_stabilize::silk_NLSF_stabilize(
        pNLSF_Q15,
        (*psNLSF_CB).deltaMin_Q15,
        (*psNLSF_CB).order as libc::c_int,
    );
    /* First stage: VQ */
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int32>() as libc::c_ulong)
            .wrapping_mul((*psNLSF_CB).nVectors as libc::c_ulong) as usize,
    );
    err_Q24 = fresh0.as_mut_ptr() as *mut crate::opus_types_h::opus_int32;
    crate::src::opus_1_2_1::silk::NLSF_VQ::silk_NLSF_VQ(
        err_Q24,
        pNLSF_Q15 as *const crate::opus_types_h::opus_int16,
        (*psNLSF_CB).CB1_NLSF_Q8,
        (*psNLSF_CB).CB1_Wght_Q9,
        (*psNLSF_CB).nVectors as libc::c_int,
        (*psNLSF_CB).order as libc::c_int,
    );
    /* Sort the quantization errors */
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nSurvivors as libc::c_ulong) as usize,
    );
    tempIndices1 = fresh1.as_mut_ptr() as *mut libc::c_int;
    crate::src::opus_1_2_1::silk::sort::silk_insertion_sort_increasing(
        err_Q24,
        tempIndices1,
        (*psNLSF_CB).nVectors as libc::c_int,
        nSurvivors,
    );
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int32>() as libc::c_ulong)
            .wrapping_mul(nSurvivors as libc::c_ulong) as usize,
    );
    RD_Q25 = fresh2.as_mut_ptr() as *mut crate::opus_types_h::opus_int32;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_schar>() as libc::c_ulong)
            .wrapping_mul((nSurvivors * 16 as libc::c_int) as libc::c_ulong) as usize,
    );
    tempIndices2 = fresh3.as_mut_ptr() as *mut libc::c_schar;
    /* Loop over survivors */
    s = 0 as libc::c_int;
    while s < nSurvivors {
        ind1 = *tempIndices1.offset(s as isize);
        /* Residual after first stage */
        pCB_element = &*(*psNLSF_CB)
            .CB1_NLSF_Q8
            .offset((ind1 * (*psNLSF_CB).order as libc::c_int) as isize)
            as *const libc::c_uchar;
        pCB_Wght_Q9 = &*(*psNLSF_CB)
            .CB1_Wght_Q9
            .offset((ind1 * (*psNLSF_CB).order as libc::c_int) as isize)
            as *const crate::opus_types_h::opus_int16;
        i = 0 as libc::c_int;
        while i < (*psNLSF_CB).order as libc::c_int {
            NLSF_tmp_Q15[i as usize] =
                ((*pCB_element.offset(i as isize) as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_uint16 as libc::c_int)
                    << 7 as libc::c_int) as crate::opus_types_h::opus_int16;
            W_tmp_Q9 = *pCB_Wght_Q9.offset(i as isize) as crate::opus_types_h::opus_int32;
            res_Q10[i as usize] = ((*pNLSF_Q15.offset(i as isize) as libc::c_int
                - NLSF_tmp_Q15[i as usize] as libc::c_int)
                as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * W_tmp_Q9 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                >> 14 as libc::c_int)
                as crate::opus_types_h::opus_int16;
            W_adj_Q5[i as usize] = silk_DIV32_varQ(
                *pW_Q2.offset(i as isize) as crate::opus_types_h::opus_int32,
                W_tmp_Q9 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                    * W_tmp_Q9 as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32,
                21 as libc::c_int,
            ) as crate::opus_types_h::opus_int16;
            i += 1
        }
        /* Unpack entropy table indices and predictor for current CB1 index */
        crate::src::opus_1_2_1::silk::NLSF_unpack::silk_NLSF_unpack(
            ec_ix.as_mut_ptr(),
            pred_Q8.as_mut_ptr(),
            psNLSF_CB as *const crate::structs_h::silk_NLSF_CB_struct,
            ind1,
        );
        /* Trellis quantizer */
        *RD_Q25.offset(s as isize) =
            crate::src::opus_1_2_1::silk::NLSF_del_dec_quant::silk_NLSF_del_dec_quant(
                &mut *tempIndices2.offset((s * 16 as libc::c_int) as isize),
                res_Q10.as_mut_ptr() as *const crate::opus_types_h::opus_int16,
                W_adj_Q5.as_mut_ptr() as *const crate::opus_types_h::opus_int16,
                pred_Q8.as_mut_ptr() as *const libc::c_uchar,
                ec_ix.as_mut_ptr() as *const crate::opus_types_h::opus_int16,
                (*psNLSF_CB).ec_Rates_Q5,
                (*psNLSF_CB).quantStepSize_Q16 as libc::c_int,
                (*psNLSF_CB).invQuantStepSize_Q6,
                NLSF_mu_Q20,
                (*psNLSF_CB).order,
            );
        /* Add rate for first stage */
        iCDF_ptr = &*(*psNLSF_CB).CB1_iCDF.offset(
            ((signalType >> 1 as libc::c_int) * (*psNLSF_CB).nVectors as libc::c_int) as isize,
        ) as *const libc::c_uchar;
        if ind1 == 0 as libc::c_int {
            prob_Q8 = 256 as libc::c_int - *iCDF_ptr.offset(ind1 as isize) as libc::c_int
        } else {
            prob_Q8 = *iCDF_ptr.offset((ind1 - 1 as libc::c_int) as isize) as libc::c_int
                - *iCDF_ptr.offset(ind1 as isize) as libc::c_int
        }
        bits_q7 = ((8 as libc::c_int) << 7 as libc::c_int)
            - crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(prob_Q8);
        *RD_Q25.offset(s as isize) = *RD_Q25.offset(s as isize)
            + bits_q7 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                * (NLSF_mu_Q20 >> 2 as libc::c_int) as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32;
        s += 1
    }
    /* Find the lowest rate-distortion error */
    crate::src::opus_1_2_1::silk::sort::silk_insertion_sort_increasing(
        RD_Q25,
        &mut bestIndex,
        nSurvivors,
        1 as libc::c_int,
    );
    *NLSFIndices.offset(0 as libc::c_int as isize) =
        *tempIndices1.offset(bestIndex as isize) as libc::c_schar;
    crate::stdlib::memcpy(
        &mut *NLSFIndices.offset(1 as libc::c_int as isize) as *mut libc::c_schar
            as *mut libc::c_void,
        &mut *tempIndices2.offset((bestIndex * 16 as libc::c_int) as isize) as *mut libc::c_schar
            as *const libc::c_void,
        ((*psNLSF_CB).order as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_schar>() as libc::c_ulong),
    );
    /* Decode */
    crate::src::opus_1_2_1::silk::NLSF_decode::silk_NLSF_decode(
        pNLSF_Q15,
        NLSFIndices,
        psNLSF_CB as *const crate::structs_h::silk_NLSF_CB_struct,
    );
    ret = *RD_Q25.offset(0 as libc::c_int as isize);
    return ret;
}

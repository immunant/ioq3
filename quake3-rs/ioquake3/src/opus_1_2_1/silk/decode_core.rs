use ::libc;

pub mod macros_h {
    #[inline]

    pub unsafe extern "C" fn silk_CLZ32(
        mut in32: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        return if in32 != 0 {
            (32 as i32)
                - (::std::mem::size_of::<u32>() as libc::c_ulong as i32
                    * 8 as i32
                    - (in32 as u32).leading_zeros() as i32)
        } else {
            32 as i32
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
        Qres: i32,
    ) -> crate::opus_types_h::opus_int32
/* I    Q-domain of result (>= 0)       */ {
        let mut a_headrm: i32 = 0;
        let mut b_headrm: i32 = 0;
        let mut lshift: i32 = 0;
        let mut b32_inv: crate::opus_types_h::opus_int32 = 0;
        let mut a32_nrm: crate::opus_types_h::opus_int32 = 0;
        let mut b32_nrm: crate::opus_types_h::opus_int32 = 0;
        let mut result: crate::opus_types_h::opus_int32 = 0;
        /* Compute number of bits head room and normalize inputs */
        a_headrm = silk_CLZ32(if a32 > 0 as i32 { a32 } else { -a32 }) - 1 as i32; /* Q: a_headrm                  */
        a32_nrm = ((a32 as crate::opus_types_h::opus_uint32) << a_headrm)
            as crate::opus_types_h::opus_int32; /* Q: b_headrm                  */
        b_headrm = silk_CLZ32(if b32 > 0 as i32 { b32 } else { -b32 }) - 1 as i32;
        b32_nrm = ((b32 as crate::opus_types_h::opus_uint32) << b_headrm)
            as crate::opus_types_h::opus_int32;
        /* Inverse of b32, with 14 bits of precision */
        b32_inv = (0x7fffffff as i32 >> 2 as i32) / (b32_nrm >> 16 as i32); /* Q: 29 + 16 - b_headrm        */
        /* First approximation */
        result = (a32_nrm as i64
            * b32_inv as crate::opus_types_h::opus_int16 as i64
            >> 16 as i32) as crate::opus_types_h::opus_int32; /* Q: 29 + a_headrm - b_headrm  */
        /* Compute residual by subtracting product of denominator and first approximation */
        /* It's OK to overflow because the final value of a32_nrm should always be small */
        a32_nrm = (a32_nrm as crate::opus_types_h::opus_uint32).wrapping_sub(
            (((b32_nrm as i64 * result as i64 >> 32 as i32)
                as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_uint32)
                << 3 as i32) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_uint32,
        ) as crate::opus_types_h::opus_int32; /* Q: a_headrm   */
        /* Refinement */
        result = (result as i64
            + (a32_nrm as i64
                * b32_inv as crate::opus_types_h::opus_int16 as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32; /* Q: 29 + a_headrm - b_headrm  */
        /* Convert to Qres domain */
        lshift = 29 as i32 + a_headrm - b_headrm - Qres;
        if lshift < 0 as i32 {
            return (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32 >> -lshift
                > 0x7fffffff as i32 >> -lshift
            {
                (if result
                    > 0x80000000 as u32 as crate::opus_types_h::opus_int32 >> -lshift
                {
                    (0x80000000 as u32 as crate::opus_types_h::opus_int32) >> -lshift
                } else {
                    (if result < 0x7fffffff as i32 >> -lshift {
                        (0x7fffffff as i32) >> -lshift
                    } else {
                        result
                    })
                })
            } else {
                (if result > 0x7fffffff as i32 >> -lshift {
                    (0x7fffffff as i32) >> -lshift
                } else {
                    (if result
                        < 0x80000000 as u32 as crate::opus_types_h::opus_int32 >> -lshift
                    {
                        (0x80000000 as u32 as crate::opus_types_h::opus_int32) >> -lshift
                    } else {
                        result
                    })
                })
            }) as crate::opus_types_h::opus_uint32)
                << -lshift) as crate::opus_types_h::opus_int32;
        } else if lshift < 32 as i32 {
            return result >> lshift;
        } else {
            /* Avoid undefined result */
            return 0 as i32;
        };
    }
    /* Invert int32 value and return result as int32 in a given Q-domain */
    #[inline]

    pub unsafe extern "C" fn silk_INVERSE32_varQ(
        b32: crate::opus_types_h::opus_int32,
        Qres: i32,
    ) -> crate::opus_types_h::opus_int32
/* I    Q-domain of result (> 0)        */ {
        let mut b_headrm: i32 = 0;
        let mut lshift: i32 = 0;
        let mut b32_inv: crate::opus_types_h::opus_int32 = 0;
        let mut b32_nrm: crate::opus_types_h::opus_int32 = 0;
        let mut err_Q32: crate::opus_types_h::opus_int32 = 0;
        let mut result: crate::opus_types_h::opus_int32 = 0;
        /* Compute number of bits head room and normalize input */
        b_headrm = silk_CLZ32(if b32 > 0 as i32 { b32 } else { -b32 }) - 1 as i32; /* Q: b_headrm                */
        b32_nrm = ((b32 as crate::opus_types_h::opus_uint32) << b_headrm)
            as crate::opus_types_h::opus_int32;
        /* Inverse of b32, with 14 bits of precision */
        b32_inv = (0x7fffffff as i32 >> 2 as i32) / (b32_nrm >> 16 as i32); /* Q: 29 + 16 - b_headrm    */
        /* First approximation */
        result = ((b32_inv as crate::opus_types_h::opus_uint32) << 16 as i32)
            as crate::opus_types_h::opus_int32; /* Q: 61 - b_headrm            */
        /* Compute residual by subtracting product of denominator and first approximation from one */
        err_Q32 = (((((1 as i32) << 29 as i32)
            - (b32_nrm as i64
                * b32_inv as crate::opus_types_h::opus_int16 as i64
                >> 16 as i32) as crate::opus_types_h::opus_int32)
            as crate::opus_types_h::opus_uint32)
            << 3 as i32) as crate::opus_types_h::opus_int32; /* Q32                        */
        /* Refinement */
        result = (result as i64
            + (err_Q32 as i64 * b32_inv as i64 >> 16 as i32))
            as crate::opus_types_h::opus_int32; /* Q: 61 - b_headrm            */
        /* Convert to Qres domain */
        lshift = 61 as i32 - b_headrm - Qres;
        if lshift <= 0 as i32 {
            return (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32 >> -lshift
                > 0x7fffffff as i32 >> -lshift
            {
                (if result
                    > 0x80000000 as u32 as crate::opus_types_h::opus_int32 >> -lshift
                {
                    (0x80000000 as u32 as crate::opus_types_h::opus_int32) >> -lshift
                } else {
                    (if result < 0x7fffffff as i32 >> -lshift {
                        (0x7fffffff as i32) >> -lshift
                    } else {
                        result
                    })
                })
            } else {
                (if result > 0x7fffffff as i32 >> -lshift {
                    (0x7fffffff as i32) >> -lshift
                } else {
                    (if result
                        < 0x80000000 as u32 as crate::opus_types_h::opus_int32 >> -lshift
                    {
                        (0x80000000 as u32 as crate::opus_types_h::opus_int32) >> -lshift
                    } else {
                        result
                    })
                })
            }) as crate::opus_types_h::opus_uint32)
                << -lshift) as crate::opus_types_h::opus_int32;
        } else if lshift < 32 as i32 {
            return result >> lshift;
        } else {
            /* Avoid undefined result */
            return 0 as i32;
        };
    }

    use crate::src::opus_1_2_1::silk::decode_core::macros_h::silk_CLZ32;
    /* SILK_FIX_INLINES_H */
}

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
pub use crate::src::opus_1_2_1::silk::decode_core::macros_h::silk_CLZ32;
pub use crate::src::opus_1_2_1::silk::decode_core::Inlines_h::silk_DIV32_varQ;
pub use crate::src::opus_1_2_1::silk::decode_core::Inlines_h::silk_INVERSE32_varQ;

pub use crate::structs_h::silk_CNG_struct;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_PLC_struct;
pub use crate::structs_h::silk_decoder_control;
pub use crate::structs_h::silk_decoder_state;
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
/* O    Returns RD value in Q25                     */
/* I    Codebook path vector [ LPC_ORDER + 1 ]      */
/* I/O  Quantized NLSF vector [ LPC_ORDER ]         */
/* I    Codebook object                             */
/* I    NLSF weight vector [ LPC_ORDER ]            */
/* I    Rate weight for the RD optimization         */
/* I    Max survivors after first stage             */
/* I    Signal type: 0/1/2                          */
/* Compute quantization errors for an LPC_order element input vector for a VQ codebook */
/* O    Quantization errors [K]                     */
/* I    Input vectors to be quantized [LPC_order]   */
/* I    Codebook vectors [K*LPC_order]              */
/* I    Codebook weights [K*LPC_order]              */
/* I    Number of codebook vectors                  */
/* I    Number of LPCs                              */
/* Delayed-decision quantizer for NLSF residuals */
/* O    Returns RD value in Q25                     */
/* O    Quantization indices [ order ]              */
/* I    Input [ order ]                             */
/* I    Weights [ order ]                           */
/* I    Backward predictor coefs [ order ]          */
/* I    Indices to entropy coding tables [ order ]  */
/* I    Rates []                                    */
/* I    Quantization step size                      */
/* I    Inverse quantization step size              */
/* I    R/D tradeoff                                */
/* I    Number of input values                      */
/* Unpack predictor values and indices for entropy coding tables */
/* O    Indices to entropy tables [ LPC_ORDER ]     */
/* O    LSF predictor [ LPC_ORDER ]                 */
/* I    Codebook object                             */
/* I    Index of vector in first LSF codebook       */
/* **********************/
/* NLSF vector decoder */
/* **********************/
/* O    Quantized NLSF vector [ LPC_ORDER ]         */
/* I    Codebook path vector [ LPC_ORDER + 1 ]      */
/* I    Codebook object                             */
/* ***************************************************/
/* Decoder Functions                                */
/* ***************************************************/
/* I/O  Decoder state pointer                       */
/* Set decoder sampling rate */
/* I/O  Decoder state pointer                       */
/* I    Sampling frequency (kHz)                    */
/* I    API Sampling frequency (Hz)                 */
/* ***************/
/* Decode frame */
/* ***************/
/* I/O  Pointer to Silk decoder state               */
/* I/O  Compressor data structure                   */
/* O    Pointer to output speech frame              */
/* O    Pointer to size of output frame             */
/* I    0: no loss, 1 loss, 2 decode fec            */
/* I    The type of conditional coding to use       */
/* I    Run-time architecture                       */
/* Decode indices from bitstream */
/* I/O  State                                       */
/* I/O  Compressor data structure                   */
/* I    Frame number                                */
/* I    Flag indicating LBRR data is being decoded  */
/* I    The type of conditional coding to use       */
/* Decode parameters from payload */
/* I/O  State                                       */
/* I/O  Decoder control                             */
/* I    The type of conditional coding to use       */
/* Core decoder. Performs inverse NSQ operation LTP + LPC */
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
/* *********************************************************/
/* Core decoder. Performs inverse NSQ operation LTP + LPC */
/* *********************************************************/
#[no_mangle]

pub unsafe extern "C" fn silk_decode_core(
    mut psDec: *mut crate::structs_h::silk_decoder_state,
    mut psDecCtrl: *mut crate::structs_h::silk_decoder_control,
    mut xq: *mut crate::opus_types_h::opus_int16,
    mut pulses: *const crate::opus_types_h::opus_int16,
    mut arch: i32,
)
/* I    Run-time architecture                       */
{
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut lag: i32 = 0 as i32;
    let mut start_idx: i32 = 0;
    let mut sLTP_buf_idx: i32 = 0;
    let mut NLSF_interpolation_flag: i32 = 0;
    let mut signalType: i32 = 0;
    let mut A_Q12: *mut crate::opus_types_h::opus_int16 = 0 as *mut crate::opus_types_h::opus_int16;
    let mut B_Q14: *mut crate::opus_types_h::opus_int16 = 0 as *mut crate::opus_types_h::opus_int16;
    let mut pxq: *mut crate::opus_types_h::opus_int16 = 0 as *mut crate::opus_types_h::opus_int16;
    let mut A_Q12_tmp: [crate::opus_types_h::opus_int16; 16] = [0; 16];
    let mut sLTP: *mut crate::opus_types_h::opus_int16 = 0 as *mut crate::opus_types_h::opus_int16;
    let mut sLTP_Q15: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut LTP_pred_Q13: crate::opus_types_h::opus_int32 = 0;
    let mut LPC_pred_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut Gain_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut inv_gain_Q31: crate::opus_types_h::opus_int32 = 0;
    let mut gain_adj_Q16: crate::opus_types_h::opus_int32 = 0;
    let mut rand_seed: crate::opus_types_h::opus_int32 = 0;
    let mut offset_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut pred_lag_ptr: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut pexc_Q14: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut pres_Q14: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut res_Q14: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut sLPC_Q14: *mut crate::opus_types_h::opus_int32 =
        0 as *mut crate::opus_types_h::opus_int32;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong)
            .wrapping_mul((*psDec).ltp_mem_length as libc::c_ulong) as usize,
    );
    sLTP = fresh0.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int32>() as libc::c_ulong)
            .wrapping_mul(((*psDec).ltp_mem_length + (*psDec).frame_length) as libc::c_ulong)
            as usize,
    );
    sLTP_Q15 = fresh1.as_mut_ptr() as *mut crate::opus_types_h::opus_int32;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int32>() as libc::c_ulong)
            .wrapping_mul((*psDec).subfr_length as libc::c_ulong) as usize,
    );
    res_Q14 = fresh2.as_mut_ptr() as *mut crate::opus_types_h::opus_int32;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int32>() as libc::c_ulong)
            .wrapping_mul(((*psDec).subfr_length + 16 as i32) as libc::c_ulong)
            as usize,
    );
    sLPC_Q14 = fresh3.as_mut_ptr() as *mut crate::opus_types_h::opus_int32;
    offset_Q10 = crate::src::opus_1_2_1::silk::tables_other::silk_Quantization_Offsets_Q10
        [((*psDec).indices.signalType as i32 >> 1 as i32) as usize]
        [(*psDec).indices.quantOffsetType as usize]
        as crate::opus_types_h::opus_int32;
    if ((*psDec).indices.NLSFInterpCoef_Q2 as i32) < (1 as i32) << 2 as i32
    {
        NLSF_interpolation_flag = 1 as i32
    } else {
        NLSF_interpolation_flag = 0 as i32
    }
    /* Decode excitation */
    rand_seed = (*psDec).indices.Seed as crate::opus_types_h::opus_int32;
    i = 0 as i32;
    while i < (*psDec).frame_length {
        rand_seed = (907633515 as i32 as crate::opus_types_h::opus_uint32).wrapping_add(
            (rand_seed as crate::opus_types_h::opus_uint32)
                .wrapping_mul(196314165 as i32 as crate::opus_types_h::opus_uint32),
        ) as crate::opus_types_h::opus_int32;
        (*psDec).exc_Q14[i as usize] =
            ((*pulses.offset(i as isize) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_uint32)
                << 14 as i32) as crate::opus_types_h::opus_int32;
        if (*psDec).exc_Q14[i as usize] > 0 as i32 {
            (*psDec).exc_Q14[i as usize] -= (80 as i32) << 4 as i32
        } else if (*psDec).exc_Q14[i as usize] < 0 as i32 {
            (*psDec).exc_Q14[i as usize] += (80 as i32) << 4 as i32
        }
        (*psDec).exc_Q14[i as usize] += offset_Q10 << 4 as i32;
        if rand_seed < 0 as i32 {
            (*psDec).exc_Q14[i as usize] = -(*psDec).exc_Q14[i as usize]
        }
        rand_seed = (rand_seed as crate::opus_types_h::opus_uint32)
            .wrapping_add(*pulses.offset(i as isize) as crate::opus_types_h::opus_uint32)
            as crate::opus_types_h::opus_int32;
        i += 1
    }
    /* Copy LPC state */
    crate::stdlib::memcpy(sLPC_Q14 as *mut libc::c_void,
           (*psDec).sLPC_Q14_buf.as_mut_ptr() as *const libc::c_void,
           (16 as i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()
                                                as libc::c_ulong));
    pexc_Q14 = (*psDec).exc_Q14.as_mut_ptr();
    pxq = xq;
    sLTP_buf_idx = (*psDec).ltp_mem_length;
    /* Loop over subframes */
    k = 0 as i32;
    while k < (*psDec).nb_subfr {
        pres_Q14 = res_Q14;
        A_Q12 = (*psDecCtrl).PredCoef_Q12[(k >> 1 as i32) as usize].as_mut_ptr();
        /* Preload LPC coeficients to array on stack. Gives small performance gain */
        crate::stdlib::memcpy(
            A_Q12_tmp.as_mut_ptr() as *mut libc::c_void,
            A_Q12 as *const libc::c_void,
            ((*psDec).LPC_order as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::opus_types_h::opus_int16,
            >() as libc::c_ulong),
        );
        B_Q14 = &mut *(*psDecCtrl)
            .LTPCoef_Q14
            .as_mut_ptr()
            .offset((k * 5 as i32) as isize)
            as *mut crate::opus_types_h::opus_int16;
        signalType = (*psDec).indices.signalType as i32;
        Gain_Q10 = (*psDecCtrl).Gains_Q16[k as usize] >> 6 as i32;
        inv_gain_Q31 = silk_INVERSE32_varQ((*psDecCtrl).Gains_Q16[k as usize], 47 as i32);
        /* Calculate gain adjustment factor */
        if (*psDecCtrl).Gains_Q16[k as usize] != (*psDec).prev_gain_Q16 {
            gain_adj_Q16 = silk_DIV32_varQ(
                (*psDec).prev_gain_Q16,
                (*psDecCtrl).Gains_Q16[k as usize],
                16 as i32,
            );
            /* Scale short term state */
            i = 0 as i32;
            while i < 16 as i32 {
                *sLPC_Q14.offset(i as isize) = (gain_adj_Q16 as i64
                    * *sLPC_Q14.offset(i as isize) as i64
                    >> 16 as i32)
                    as crate::opus_types_h::opus_int32;
                i += 1
            }
        } else {
            gain_adj_Q16 = (1 as i32) << 16 as i32
        }
        /* Save inv_gain */
        (*psDec).prev_gain_Q16 = (*psDecCtrl).Gains_Q16[k as usize];
        /* Avoid abrupt transition from voiced PLC to unvoiced normal decoding */
        if (*psDec).lossCnt != 0
            && (*psDec).prevSignalType == 2 as i32
            && (*psDec).indices.signalType as i32 != 2 as i32
            && k < 4 as i32 / 2 as i32
        {
            crate::stdlib::memset(
                B_Q14 as *mut libc::c_void,
                0 as i32,
                (5 as i32 as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong
                    ),
            );
            *B_Q14.offset((5 as i32 / 2 as i32) as isize) = (0.25f64
                * ((1 as i32 as i64) << 14 as i32) as f64
                + 0.5f64)
                as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_int16;
            signalType = 2 as i32;
            (*psDecCtrl).pitchL[k as usize] = (*psDec).lagPrev
        }
        if signalType == 2 as i32 {
            /* Voiced */
            lag = (*psDecCtrl).pitchL[k as usize];
            /* Re-whitening */
            if k == 0 as i32 || k == 2 as i32 && NLSF_interpolation_flag != 0 {
                /* Rewhiten with new A coefs */
                start_idx = (*psDec).ltp_mem_length
                    - lag
                    - (*psDec).LPC_order
                    - 5 as i32 / 2 as i32;
                if k == 2 as i32 {
                    crate::stdlib::memcpy(
                        &mut *(*psDec)
                            .outBuf
                            .as_mut_ptr()
                            .offset((*psDec).ltp_mem_length as isize)
                            as *mut crate::opus_types_h::opus_int16
                            as *mut libc::c_void,
                        xq as *const libc::c_void,
                        ((2 as i32 * (*psDec).subfr_length) as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()
                                as libc::c_ulong),
                    );
                }
                crate::src::opus_1_2_1::silk::LPC_analysis_filter::silk_LPC_analysis_filter(
                    &mut *sLTP.offset(start_idx as isize),
                    &mut *(*psDec)
                        .outBuf
                        .as_mut_ptr()
                        .offset((start_idx + k * (*psDec).subfr_length) as isize),
                    A_Q12,
                    (*psDec).ltp_mem_length - start_idx,
                    (*psDec).LPC_order,
                    arch,
                );
                /* After rewhitening the LTP state is unscaled */
                if k == 0 as i32 {
                    /* Do LTP downscaling to reduce inter-packet dependency */
                    inv_gain_Q31 = (((inv_gain_Q31 as i64
                        * (*psDecCtrl).LTP_scale_Q14 as crate::opus_types_h::opus_int16
                            as i64
                        >> 16 as i32)
                        as crate::opus_types_h::opus_int32
                        as crate::opus_types_h::opus_uint32)
                        << 2 as i32)
                        as crate::opus_types_h::opus_int32
                }
                i = 0 as i32;
                while i < lag + 5 as i32 / 2 as i32 {
                    *sLTP_Q15.offset((sLTP_buf_idx - i - 1 as i32) as isize) = (inv_gain_Q31
                        as i64
                        * *sLTP.offset(((*psDec).ltp_mem_length - i - 1 as i32) as isize)
                            as i64
                        >> 16 as i32)
                        as crate::opus_types_h::opus_int32;
                    i += 1
                }
            } else if gain_adj_Q16 != (1 as i32) << 16 as i32 {
                i = 0 as i32;
                while i < lag + 5 as i32 / 2 as i32 {
                    *sLTP_Q15.offset((sLTP_buf_idx - i - 1 as i32) as isize) = (gain_adj_Q16
                        as i64
                        * *sLTP_Q15.offset((sLTP_buf_idx - i - 1 as i32) as isize)
                            as i64
                        >> 16 as i32)
                        as crate::opus_types_h::opus_int32;
                    i += 1
                }
            }
        }
        /* Update LTP state when Gain changes */
        /* Long-term prediction */
        if signalType == 2 as i32 {
            /* Set up pointer */
            pred_lag_ptr = &mut *sLTP_Q15
                .offset((sLTP_buf_idx - lag + 5 as i32 / 2 as i32) as isize)
                as *mut crate::opus_types_h::opus_int32;
            i = 0 as i32;
            while i < (*psDec).subfr_length {
                /* Unrolled loop */
                /* Avoids introducing a bias because silk_SMLAWB() always rounds to -inf */
                LTP_pred_Q13 = 2 as i32;
                LTP_pred_Q13 = (LTP_pred_Q13 as i64
                    + (*pred_lag_ptr.offset(0 as i32 as isize) as i64
                        * *B_Q14.offset(0 as i32 as isize) as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LTP_pred_Q13 = (LTP_pred_Q13 as i64
                    + (*pred_lag_ptr.offset(-(1 as i32) as isize) as i64
                        * *B_Q14.offset(1 as i32 as isize) as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LTP_pred_Q13 = (LTP_pred_Q13 as i64
                    + (*pred_lag_ptr.offset(-(2 as i32) as isize) as i64
                        * *B_Q14.offset(2 as i32 as isize) as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LTP_pred_Q13 = (LTP_pred_Q13 as i64
                    + (*pred_lag_ptr.offset(-(3 as i32) as isize) as i64
                        * *B_Q14.offset(3 as i32 as isize) as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LTP_pred_Q13 = (LTP_pred_Q13 as i64
                    + (*pred_lag_ptr.offset(-(4 as i32) as isize) as i64
                        * *B_Q14.offset(4 as i32 as isize) as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                pred_lag_ptr = pred_lag_ptr.offset(1);
                /* Generate LPC excitation */
                *pres_Q14.offset(i as isize) = *pexc_Q14.offset(i as isize)
                    + ((LTP_pred_Q13 as crate::opus_types_h::opus_uint32) << 1 as i32)
                        as crate::opus_types_h::opus_int32;
                /* Update states */
                *sLTP_Q15.offset(sLTP_buf_idx as isize) =
                    ((*pres_Q14.offset(i as isize) as crate::opus_types_h::opus_uint32)
                        << 1 as i32) as crate::opus_types_h::opus_int32;
                sLTP_buf_idx += 1;
                i += 1
            }
        } else {
            pres_Q14 = pexc_Q14
        }
        i = 0 as i32;
        while i < (*psDec).subfr_length {
            /* Short-term prediction */
            /* Avoids introducing a bias because silk_SMLAWB() always rounds to -inf */
            LPC_pred_Q10 = (*psDec).LPC_order >> 1 as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*sLPC_Q14.offset((16 as i32 + i - 1 as i32) as isize)
                    as i64
                    * A_Q12_tmp[0 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*sLPC_Q14.offset((16 as i32 + i - 2 as i32) as isize)
                    as i64
                    * A_Q12_tmp[1 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*sLPC_Q14.offset((16 as i32 + i - 3 as i32) as isize)
                    as i64
                    * A_Q12_tmp[2 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*sLPC_Q14.offset((16 as i32 + i - 4 as i32) as isize)
                    as i64
                    * A_Q12_tmp[3 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*sLPC_Q14.offset((16 as i32 + i - 5 as i32) as isize)
                    as i64
                    * A_Q12_tmp[4 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*sLPC_Q14.offset((16 as i32 + i - 6 as i32) as isize)
                    as i64
                    * A_Q12_tmp[5 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*sLPC_Q14.offset((16 as i32 + i - 7 as i32) as isize)
                    as i64
                    * A_Q12_tmp[6 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*sLPC_Q14.offset((16 as i32 + i - 8 as i32) as isize)
                    as i64
                    * A_Q12_tmp[7 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*sLPC_Q14.offset((16 as i32 + i - 9 as i32) as isize)
                    as i64
                    * A_Q12_tmp[8 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*sLPC_Q14.offset((16 as i32 + i - 10 as i32) as isize)
                    as i64
                    * A_Q12_tmp[9 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            if (*psDec).LPC_order == 16 as i32 {
                LPC_pred_Q10 = (LPC_pred_Q10 as i64
                    + (*sLPC_Q14.offset((16 as i32 + i - 11 as i32) as isize)
                        as i64
                        * A_Q12_tmp[10 as i32 as usize] as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LPC_pred_Q10 = (LPC_pred_Q10 as i64
                    + (*sLPC_Q14.offset((16 as i32 + i - 12 as i32) as isize)
                        as i64
                        * A_Q12_tmp[11 as i32 as usize] as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LPC_pred_Q10 = (LPC_pred_Q10 as i64
                    + (*sLPC_Q14.offset((16 as i32 + i - 13 as i32) as isize)
                        as i64
                        * A_Q12_tmp[12 as i32 as usize] as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LPC_pred_Q10 = (LPC_pred_Q10 as i64
                    + (*sLPC_Q14.offset((16 as i32 + i - 14 as i32) as isize)
                        as i64
                        * A_Q12_tmp[13 as i32 as usize] as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LPC_pred_Q10 = (LPC_pred_Q10 as i64
                    + (*sLPC_Q14.offset((16 as i32 + i - 15 as i32) as isize)
                        as i64
                        * A_Q12_tmp[14 as i32 as usize] as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LPC_pred_Q10 = (LPC_pred_Q10 as i64
                    + (*sLPC_Q14.offset((16 as i32 + i - 16 as i32) as isize)
                        as i64
                        * A_Q12_tmp[15 as i32 as usize] as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32
            }
            *sLPC_Q14.offset((16 as i32 + i) as isize) = if (*pres_Q14.offset(i as isize)
                as crate::opus_types_h::opus_uint32)
                .wrapping_add(
                    (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32
                        >> 4 as i32
                        > 0x7fffffff as i32 >> 4 as i32
                    {
                        (if LPC_pred_Q10
                            > 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                >> 4 as i32
                        {
                            (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                >> 4 as i32
                        } else {
                            (if LPC_pred_Q10 < 0x7fffffff as i32 >> 4 as i32 {
                                (0x7fffffff as i32) >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    } else {
                        (if LPC_pred_Q10 > 0x7fffffff as i32 >> 4 as i32 {
                            (0x7fffffff as i32) >> 4 as i32
                        } else {
                            (if LPC_pred_Q10
                                < 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                    >> 4 as i32
                            {
                                (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                    >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    }) as crate::opus_types_h::opus_uint32)
                        << 4 as i32) as crate::opus_types_h::opus_int32
                        as crate::opus_types_h::opus_uint32,
                )
                & 0x80000000 as u32
                == 0 as i32 as u32
            {
                if (*pres_Q14.offset(i as isize)
                    & (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32
                        >> 4 as i32
                        > 0x7fffffff as i32 >> 4 as i32
                    {
                        (if LPC_pred_Q10
                            > 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                >> 4 as i32
                        {
                            (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                >> 4 as i32
                        } else {
                            (if LPC_pred_Q10 < 0x7fffffff as i32 >> 4 as i32 {
                                (0x7fffffff as i32) >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    } else {
                        (if LPC_pred_Q10 > 0x7fffffff as i32 >> 4 as i32 {
                            (0x7fffffff as i32) >> 4 as i32
                        } else {
                            (if LPC_pred_Q10
                                < 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                    >> 4 as i32
                            {
                                (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                    >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    }) as crate::opus_types_h::opus_uint32)
                        << 4 as i32)
                        as crate::opus_types_h::opus_int32) as u32
                    & 0x80000000 as u32
                    != 0 as i32 as u32
                {
                    0x80000000 as u32 as crate::opus_types_h::opus_int32
                } else {
                    (*pres_Q14.offset(i as isize))
                        + (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32
                            >> 4 as i32
                            > 0x7fffffff as i32 >> 4 as i32
                        {
                            (if LPC_pred_Q10
                                > 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                    >> 4 as i32
                            {
                                (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                    >> 4 as i32
                            } else {
                                (if LPC_pred_Q10 < 0x7fffffff as i32 >> 4 as i32 {
                                    (0x7fffffff as i32) >> 4 as i32
                                } else {
                                    LPC_pred_Q10
                                })
                            })
                        } else {
                            (if LPC_pred_Q10 > 0x7fffffff as i32 >> 4 as i32 {
                                (0x7fffffff as i32) >> 4 as i32
                            } else {
                                (if LPC_pred_Q10
                                    < 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                        >> 4 as i32
                                {
                                    (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                        >> 4 as i32
                                } else {
                                    LPC_pred_Q10
                                })
                            })
                        }) as crate::opus_types_h::opus_uint32)
                            << 4 as i32)
                            as crate::opus_types_h::opus_int32
                }
            } else if (*pres_Q14.offset(i as isize)
                | (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32
                    >> 4 as i32
                    > 0x7fffffff as i32 >> 4 as i32
                {
                    (if LPC_pred_Q10
                        > 0x80000000 as u32 as crate::opus_types_h::opus_int32
                            >> 4 as i32
                    {
                        (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                            >> 4 as i32
                    } else {
                        (if LPC_pred_Q10 < 0x7fffffff as i32 >> 4 as i32 {
                            (0x7fffffff as i32) >> 4 as i32
                        } else {
                            LPC_pred_Q10
                        })
                    })
                } else {
                    (if LPC_pred_Q10 > 0x7fffffff as i32 >> 4 as i32 {
                        (0x7fffffff as i32) >> 4 as i32
                    } else {
                        (if LPC_pred_Q10
                            < 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                >> 4 as i32
                        {
                            (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                >> 4 as i32
                        } else {
                            LPC_pred_Q10
                        })
                    })
                }) as crate::opus_types_h::opus_uint32)
                    << 4 as i32) as crate::opus_types_h::opus_int32)
                as u32
                & 0x80000000 as u32
                == 0 as i32 as u32
            {
                0x7fffffff as i32
            } else {
                (*pres_Q14.offset(i as isize))
                    + (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32
                        >> 4 as i32
                        > 0x7fffffff as i32 >> 4 as i32
                    {
                        (if LPC_pred_Q10
                            > 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                >> 4 as i32
                        {
                            (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                >> 4 as i32
                        } else {
                            (if LPC_pred_Q10 < 0x7fffffff as i32 >> 4 as i32 {
                                (0x7fffffff as i32) >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    } else {
                        (if LPC_pred_Q10 > 0x7fffffff as i32 >> 4 as i32 {
                            (0x7fffffff as i32) >> 4 as i32
                        } else {
                            (if LPC_pred_Q10
                                < 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                    >> 4 as i32
                            {
                                (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                    >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    }) as crate::opus_types_h::opus_uint32)
                        << 4 as i32)
                        as crate::opus_types_h::opus_int32
            };
            *pxq.offset(i as isize) = if (if 8 as i32 == 1 as i32 {
                ((*sLPC_Q14.offset((16 as i32 + i) as isize) as i64
                    * Gain_Q10 as i64
                    >> 16 as i32) as crate::opus_types_h::opus_int32
                    >> 1 as i32)
                    + ((*sLPC_Q14.offset((16 as i32 + i) as isize) as i64
                        * Gain_Q10 as i64
                        >> 16 as i32)
                        as crate::opus_types_h::opus_int32
                        & 1 as i32)
            } else {
                (((*sLPC_Q14.offset((16 as i32 + i) as isize) as i64
                    * Gain_Q10 as i64
                    >> 16 as i32) as crate::opus_types_h::opus_int32
                    >> 8 as i32 - 1 as i32)
                    + 1 as i32)
                    >> 1 as i32
            }) > 0x7fff as i32
            {
                0x7fff as i32
            } else if (if 8 as i32 == 1 as i32 {
                ((*sLPC_Q14.offset((16 as i32 + i) as isize) as i64
                    * Gain_Q10 as i64
                    >> 16 as i32) as crate::opus_types_h::opus_int32
                    >> 1 as i32)
                    + ((*sLPC_Q14.offset((16 as i32 + i) as isize) as i64
                        * Gain_Q10 as i64
                        >> 16 as i32)
                        as crate::opus_types_h::opus_int32
                        & 1 as i32)
            } else {
                (((*sLPC_Q14.offset((16 as i32 + i) as isize) as i64
                    * Gain_Q10 as i64
                    >> 16 as i32) as crate::opus_types_h::opus_int32
                    >> 8 as i32 - 1 as i32)
                    + 1 as i32)
                    >> 1 as i32
            }) < 0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
            {
                0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
            } else if 8 as i32 == 1 as i32 {
                ((*sLPC_Q14.offset((16 as i32 + i) as isize) as i64
                    * Gain_Q10 as i64
                    >> 16 as i32) as crate::opus_types_h::opus_int32
                    >> 1 as i32)
                    + ((*sLPC_Q14.offset((16 as i32 + i) as isize) as i64
                        * Gain_Q10 as i64
                        >> 16 as i32)
                        as crate::opus_types_h::opus_int32
                        & 1 as i32)
            } else {
                (((*sLPC_Q14.offset((16 as i32 + i) as isize) as i64
                    * Gain_Q10 as i64
                    >> 16 as i32) as crate::opus_types_h::opus_int32
                    >> 8 as i32 - 1 as i32)
                    + 1 as i32)
                    >> 1 as i32
            } as crate::opus_types_h::opus_int16;
            i += 1
        }
        /* Add prediction to LPC excitation */
        /* Scale with gain */
        /* Update LPC filter state */
        crate::stdlib::memcpy(
            sLPC_Q14 as *mut libc::c_void,
            &mut *sLPC_Q14.offset((*psDec).subfr_length as isize)
                as *mut crate::opus_types_h::opus_int32 as *const libc::c_void,
            (16 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::opus_types_h::opus_int32,
            >() as libc::c_ulong),
        );
        pexc_Q14 = pexc_Q14.offset((*psDec).subfr_length as isize);
        pxq = pxq.offset((*psDec).subfr_length as isize);
        k += 1
    }
    /* Save LPC state */
    crate::stdlib::memcpy((*psDec).sLPC_Q14_buf.as_mut_ptr() as *mut libc::c_void,
           sLPC_Q14 as *const libc::c_void,
           (16 as i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()
                                                as libc::c_ulong));
}

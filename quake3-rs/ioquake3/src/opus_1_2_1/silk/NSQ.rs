use ::libc;

pub mod macros_h {
    #[inline]

    pub unsafe extern "C" fn silk_CLZ32(
        mut in32: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        return if in32 != 0 {
            (32 as i32)
                - (::std::mem::size_of::<u32>() as usize as i32 * 8 as i32
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
        result = (a32_nrm as i64 * b32_inv as crate::opus_types_h::opus_int16 as i64 >> 16 as i32)
            as crate::opus_types_h::opus_int32; /* Q: 29 + a_headrm - b_headrm  */
        /* Compute residual by subtracting product of denominator and first approximation */
        /* It's OK to overflow because the final value of a32_nrm should always be small */
        a32_nrm = (a32_nrm as crate::opus_types_h::opus_uint32).wrapping_sub(
            (((b32_nrm as i64 * result as i64 >> 32 as i32) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_uint32)
                << 3 as i32) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_uint32,
        ) as crate::opus_types_h::opus_int32; /* Q: a_headrm   */
        /* Refinement */
        result = (result as i64
            + (a32_nrm as i64 * b32_inv as crate::opus_types_h::opus_int16 as i64 >> 16 as i32))
            as crate::opus_types_h::opus_int32; /* Q: 29 + a_headrm - b_headrm  */
        /* Convert to Qres domain */
        lshift = 29 as i32 + a_headrm - b_headrm - Qres;
        if lshift < 0 as i32 {
            return (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32 >> -lshift
                > 0x7fffffff as i32 >> -lshift
            {
                (if result > 0x80000000 as u32 as crate::opus_types_h::opus_int32 >> -lshift {
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
                    (if result < 0x80000000 as u32 as crate::opus_types_h::opus_int32 >> -lshift {
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
            - (b32_nrm as i64 * b32_inv as crate::opus_types_h::opus_int16 as i64 >> 16 as i32)
                as crate::opus_types_h::opus_int32)
            as crate::opus_types_h::opus_uint32)
            << 3 as i32) as crate::opus_types_h::opus_int32; /* Q32                        */
        /* Refinement */
        result = (result as i64 + (err_Q32 as i64 * b32_inv as i64 >> 16 as i32))
            as crate::opus_types_h::opus_int32; /* Q: 61 - b_headrm            */
        /* Convert to Qres domain */
        lshift = 61 as i32 - b_headrm - Qres;
        if lshift <= 0 as i32 {
            return (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32 >> -lshift
                > 0x7fffffff as i32 >> -lshift
            {
                (if result > 0x80000000 as u32 as crate::opus_types_h::opus_int32 >> -lshift {
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
                    (if result < 0x80000000 as u32 as crate::opus_types_h::opus_int32 >> -lshift {
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

    use crate::src::opus_1_2_1::silk::NSQ::macros_h::silk_CLZ32;
    /* SILK_FIX_INLINES_H */
}

pub mod NSQ_h {
    /* **********************************************************************
    Copyright (c) 2014 Vidyo.
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
    #[inline]

    pub unsafe extern "C" fn silk_noise_shape_quantizer_short_prediction_c(
        mut buf32: *const crate::opus_types_h::opus_int32,
        mut coef16: *const crate::opus_types_h::opus_int16,
        mut order: i32,
    ) -> crate::opus_types_h::opus_int32 {
        let mut out: crate::opus_types_h::opus_int32 = 0;
        /* Avoids introducing a bias because silk_SMLAWB() always rounds to -inf */
        out = order >> 1 as i32;
        out = (out as i64
            + (*buf32.offset(0 as i32 as isize) as i64 * *coef16.offset(0 as i32 as isize) as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32;
        out = (out as i64
            + (*buf32.offset(-(1 as i32) as isize) as i64
                * *coef16.offset(1 as i32 as isize) as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32;
        out = (out as i64
            + (*buf32.offset(-(2 as i32) as isize) as i64
                * *coef16.offset(2 as i32 as isize) as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32;
        out = (out as i64
            + (*buf32.offset(-(3 as i32) as isize) as i64
                * *coef16.offset(3 as i32 as isize) as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32;
        out = (out as i64
            + (*buf32.offset(-(4 as i32) as isize) as i64
                * *coef16.offset(4 as i32 as isize) as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32;
        out = (out as i64
            + (*buf32.offset(-(5 as i32) as isize) as i64
                * *coef16.offset(5 as i32 as isize) as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32;
        out = (out as i64
            + (*buf32.offset(-(6 as i32) as isize) as i64
                * *coef16.offset(6 as i32 as isize) as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32;
        out = (out as i64
            + (*buf32.offset(-(7 as i32) as isize) as i64
                * *coef16.offset(7 as i32 as isize) as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32;
        out = (out as i64
            + (*buf32.offset(-(8 as i32) as isize) as i64
                * *coef16.offset(8 as i32 as isize) as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32;
        out = (out as i64
            + (*buf32.offset(-(9 as i32) as isize) as i64
                * *coef16.offset(9 as i32 as isize) as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32;
        if order == 16 as i32 {
            out = (out as i64
                + (*buf32.offset(-(10 as i32) as isize) as i64
                    * *coef16.offset(10 as i32 as isize) as i64
                    >> 16 as i32)) as crate::opus_types_h::opus_int32;
            out = (out as i64
                + (*buf32.offset(-(11 as i32) as isize) as i64
                    * *coef16.offset(11 as i32 as isize) as i64
                    >> 16 as i32)) as crate::opus_types_h::opus_int32;
            out = (out as i64
                + (*buf32.offset(-(12 as i32) as isize) as i64
                    * *coef16.offset(12 as i32 as isize) as i64
                    >> 16 as i32)) as crate::opus_types_h::opus_int32;
            out = (out as i64
                + (*buf32.offset(-(13 as i32) as isize) as i64
                    * *coef16.offset(13 as i32 as isize) as i64
                    >> 16 as i32)) as crate::opus_types_h::opus_int32;
            out = (out as i64
                + (*buf32.offset(-(14 as i32) as isize) as i64
                    * *coef16.offset(14 as i32 as isize) as i64
                    >> 16 as i32)) as crate::opus_types_h::opus_int32;
            out = (out as i64
                + (*buf32.offset(-(15 as i32) as isize) as i64
                    * *coef16.offset(15 as i32 as isize) as i64
                    >> 16 as i32)) as crate::opus_types_h::opus_int32
        }
        return out;
    }
    #[inline]

    pub unsafe extern "C" fn silk_NSQ_noise_shape_feedback_loop_c(
        mut data0: *const crate::opus_types_h::opus_int32,
        mut data1: *mut crate::opus_types_h::opus_int32,
        mut coef: *const crate::opus_types_h::opus_int16,
        mut order: i32,
    ) -> crate::opus_types_h::opus_int32 {
        let mut out: crate::opus_types_h::opus_int32 = 0;
        let mut tmp1: crate::opus_types_h::opus_int32 = 0;
        let mut tmp2: crate::opus_types_h::opus_int32 = 0;
        let mut j: i32 = 0;
        tmp2 = *data0.offset(0 as i32 as isize);
        tmp1 = *data1.offset(0 as i32 as isize);
        *data1.offset(0 as i32 as isize) = tmp2;
        out = order >> 1 as i32;
        out = (out as i64 + (tmp2 as i64 * *coef.offset(0 as i32 as isize) as i64 >> 16 as i32))
            as crate::opus_types_h::opus_int32;
        j = 2 as i32;
        while j < order {
            tmp2 = *data1.offset((j - 1 as i32) as isize);
            *data1.offset((j - 1 as i32) as isize) = tmp1;
            out = (out as i64
                + (tmp1 as i64 * *coef.offset((j - 1 as i32) as isize) as i64 >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            tmp1 = *data1.offset((j + 0 as i32) as isize);
            *data1.offset((j + 0 as i32) as isize) = tmp2;
            out = (out as i64 + (tmp2 as i64 * *coef.offset(j as isize) as i64 >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            j += 2 as i32
        }
        *data1.offset((order - 1 as i32) as isize) = tmp1;
        out = (out as i64
            + (tmp1 as i64 * *coef.offset((order - 1 as i32) as isize) as i64 >> 16 as i32))
            as crate::opus_types_h::opus_int32;
        /* Q11 -> Q12 */
        out = ((out as crate::opus_types_h::opus_uint32) << 1 as i32)
            as crate::opus_types_h::opus_int32;
        return out;
    }

    /* SILK_NSQ_H */
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

pub use crate::src::opus_1_2_1::silk::NSQ::macros_h::silk_CLZ32;
pub use crate::src::opus_1_2_1::silk::NSQ::Inlines_h::silk_DIV32_varQ;
pub use crate::src::opus_1_2_1::silk::NSQ::Inlines_h::silk_INVERSE32_varQ;
pub use crate::src::opus_1_2_1::silk::NSQ::NSQ_h::silk_NSQ_noise_shape_feedback_loop_c;
pub use crate::src::opus_1_2_1::silk::NSQ::NSQ_h::silk_noise_shape_quantizer_short_prediction_c;

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
#[no_mangle]

pub unsafe extern "C" fn silk_NSQ_c(
    mut psEncC: *const silk_encoder_state,
    mut NSQ: *mut silk_nsq_state,
    mut psIndices: *mut SideInfoIndices,
    mut x16: *const opus_int16,
    mut pulses: *mut i8,
    mut PredCoef_Q12: *const opus_int16,
    mut LTPCoef_Q14: *const opus_int16,
    mut AR_Q13: *const opus_int16,
    mut HarmShapeGain_Q14: *const i32,
    mut Tilt_Q14: *const i32,
    mut LF_shp_Q14: *const opus_int32,
    mut Gains_Q16: *const opus_int32,
    mut pitchL: *const i32,
    Lambda_Q10: i32,
    LTP_scale_Q14: i32,
)
/* I    LTP state scaling               */
{
    let mut k: i32 = 0;
    let mut lag: i32 = 0;
    let mut start_idx: i32 = 0;
    let mut LSF_interpolation_flag: i32 = 0;
    let mut A_Q12: *const opus_int16 = std::ptr::null();
    let mut B_Q14: *const opus_int16 = std::ptr::null();
    let mut AR_shp_Q13: *const opus_int16 = std::ptr::null();
    let mut pxq: *mut opus_int16 = std::ptr::null_mut();
    let mut sLTP_Q15: *mut opus_int32 = std::ptr::null_mut();
    let mut sLTP: *mut opus_int16 = std::ptr::null_mut();
    let mut HarmShapeFIRPacked_Q14: opus_int32 = 0;
    let mut offset_Q10: i32 = 0;
    let mut x_sc_Q10: *mut opus_int32 = std::ptr::null_mut();
    (*NSQ).rand_seed = (*psIndices).Seed as opus_int32;
    /* Set unvoiced lag to the previous one, overwrite later for voiced */
    lag = (*NSQ).lagPrev;
    offset_Q10 = crate::src::opus_1_2_1::silk::tables_other::silk_Quantization_Offsets_Q10
        [((*psIndices).signalType as i32 >> 1 as i32) as usize]
        [(*psIndices).quantOffsetType as usize] as i32;
    if (*psIndices).NLSFInterpCoef_Q2 as i32 == 4 as i32 {
        LSF_interpolation_flag = 0 as i32
    } else {
        LSF_interpolation_flag = 1 as i32
    }
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_int32>() as usize)
            .wrapping_mul(((*psEncC).ltp_mem_length + (*psEncC).frame_length) as usize)
            as usize,
    );
    sLTP_Q15 = fresh0.as_mut_ptr() as *mut opus_int32;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_int16>() as usize)
            .wrapping_mul(((*psEncC).ltp_mem_length + (*psEncC).frame_length) as usize)
            as usize,
    );
    sLTP = fresh1.as_mut_ptr() as *mut opus_int16;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_int32>() as usize).wrapping_mul((*psEncC).subfr_length as usize)
            as usize,
    );
    x_sc_Q10 = fresh2.as_mut_ptr() as *mut opus_int32;
    /* Set up pointers to start of sub frame */
    (*NSQ).sLTP_shp_buf_idx = (*psEncC).ltp_mem_length;
    (*NSQ).sLTP_buf_idx = (*psEncC).ltp_mem_length;
    pxq = &mut *(*NSQ)
        .xq
        .as_mut_ptr()
        .offset((*psEncC).ltp_mem_length as isize) as *mut opus_int16;
    k = 0 as i32;
    while k < (*psEncC).nb_subfr {
        A_Q12 = &*PredCoef_Q12
            .offset(((k >> 1 as i32 | 1 as i32 - LSF_interpolation_flag) * 16 as i32) as isize)
            as *const opus_int16;
        B_Q14 = &*LTPCoef_Q14.offset((k * 5 as i32) as isize) as *const opus_int16;
        AR_shp_Q13 = &*AR_Q13.offset((k * 24 as i32) as isize) as *const opus_int16;
        /* Noise shape parameters */
        HarmShapeFIRPacked_Q14 = *HarmShapeGain_Q14.offset(k as isize) >> 2 as i32;
        HarmShapeFIRPacked_Q14 |= (((*HarmShapeGain_Q14.offset(k as isize) >> 1 as i32)
            as opus_uint32)
            << 16 as i32) as opus_int32;
        (*NSQ).rewhite_flag = 0 as i32;
        if (*psIndices).signalType as i32 == 2 as i32 {
            /* Voiced */
            lag = *pitchL.offset(k as isize);
            /* Re-whitening */
            if k & 3 as i32 - ((LSF_interpolation_flag as opus_uint32) << 1 as i32) as opus_int32
                == 0 as i32
            {
                /* Rewhiten with new A coefs */
                start_idx = (*psEncC).ltp_mem_length
                    - lag
                    - (*psEncC).predictLPCOrder
                    - 5 as i32 / 2 as i32;
                crate::src::opus_1_2_1::silk::LPC_analysis_filter::silk_LPC_analysis_filter(
                    &mut *sLTP.offset(start_idx as isize),
                    &mut *(*NSQ)
                        .xq
                        .as_mut_ptr()
                        .offset((start_idx + k * (*psEncC).subfr_length) as isize),
                    A_Q12,
                    (*psEncC).ltp_mem_length - start_idx,
                    (*psEncC).predictLPCOrder,
                    (*psEncC).arch,
                );
                (*NSQ).rewhite_flag = 1 as i32;
                (*NSQ).sLTP_buf_idx = (*psEncC).ltp_mem_length
            }
        }
        silk_nsq_scale_states(
            psEncC,
            NSQ,
            x16,
            x_sc_Q10,
            sLTP as *const opus_int16,
            sLTP_Q15,
            k,
            LTP_scale_Q14,
            Gains_Q16,
            pitchL,
            (*psIndices).signalType as i32,
        );
        silk_noise_shape_quantizer(
            NSQ,
            (*psIndices).signalType as i32,
            x_sc_Q10 as *const opus_int32,
            pulses,
            pxq,
            sLTP_Q15,
            A_Q12,
            B_Q14,
            AR_shp_Q13,
            lag,
            HarmShapeFIRPacked_Q14,
            *Tilt_Q14.offset(k as isize),
            *LF_shp_Q14.offset(k as isize),
            *Gains_Q16.offset(k as isize),
            Lambda_Q10,
            offset_Q10,
            (*psEncC).subfr_length,
            (*psEncC).shapingLPCOrder,
            (*psEncC).predictLPCOrder,
            (*psEncC).arch,
        );
        x16 = x16.offset((*psEncC).subfr_length as isize);
        pulses = pulses.offset((*psEncC).subfr_length as isize);
        pxq = pxq.offset((*psEncC).subfr_length as isize);
        k += 1
    }
    /* Update lagPrev for next frame */
    (*NSQ).lagPrev = *pitchL.offset(((*psEncC).nb_subfr - 1 as i32) as isize);
    /* Save quantized speech and noise shaping signals */
    crate::stdlib::memmove(
        (*NSQ).xq.as_mut_ptr() as *mut libc::c_void,
        &mut *(*NSQ)
            .xq
            .as_mut_ptr()
            .offset((*psEncC).frame_length as isize) as *mut opus_int16
            as *const libc::c_void,
        ((*psEncC).ltp_mem_length as usize)
            .wrapping_mul(::std::mem::size_of::<opus_int16>() as usize),
    );
    crate::stdlib::memmove(
        (*NSQ).sLTP_shp_Q14.as_mut_ptr() as *mut libc::c_void,
        &mut *(*NSQ)
            .sLTP_shp_Q14
            .as_mut_ptr()
            .offset((*psEncC).frame_length as isize) as *mut opus_int32
            as *const libc::c_void,
        ((*psEncC).ltp_mem_length as usize)
            .wrapping_mul(::std::mem::size_of::<opus_int32>() as usize),
    );
}
/* **********************************/
/* silk_noise_shape_quantizer  */
/* **********************************/
#[inline]

unsafe extern "C" fn silk_noise_shape_quantizer(
    mut NSQ: *mut silk_nsq_state,
    mut signalType: i32,
    mut x_sc_Q10: *const opus_int32,
    mut pulses: *mut i8,
    mut xq: *mut opus_int16,
    mut sLTP_Q15: *mut opus_int32,
    mut a_Q12: *const opus_int16,
    mut b_Q14: *const opus_int16,
    mut AR_shp_Q13: *const opus_int16,
    mut lag: i32,
    mut HarmShapeFIRPacked_Q14: opus_int32,
    mut Tilt_Q14: i32,
    mut LF_shp_Q14: opus_int32,
    mut Gain_Q16: opus_int32,
    mut Lambda_Q10: i32,
    mut offset_Q10: i32,
    mut length: i32,
    mut shapingLPCOrder: i32,
    mut predictLPCOrder: i32,
    mut _arch: i32,
)
/* I    Architecture                    */
{
    let mut i: i32 = 0;
    let mut LTP_pred_Q13: opus_int32 = 0;
    let mut LPC_pred_Q10: opus_int32 = 0;
    let mut n_AR_Q12: opus_int32 = 0;
    let mut n_LTP_Q13: opus_int32 = 0;
    let mut n_LF_Q12: opus_int32 = 0;
    let mut r_Q10: opus_int32 = 0;
    let mut rr_Q10: opus_int32 = 0;
    let mut q1_Q0: opus_int32 = 0;
    let mut q1_Q10: opus_int32 = 0;
    let mut q2_Q10: opus_int32 = 0;
    let mut rd1_Q20: opus_int32 = 0;
    let mut rd2_Q20: opus_int32 = 0;
    let mut exc_Q14: opus_int32 = 0;
    let mut LPC_exc_Q14: opus_int32 = 0;
    let mut xq_Q14: opus_int32 = 0;
    let mut Gain_Q10: opus_int32 = 0;
    let mut tmp1: opus_int32 = 0;
    let mut tmp2: opus_int32 = 0;
    let mut sLF_AR_shp_Q14: opus_int32 = 0;
    let mut psLPC_Q14: *mut opus_int32 = std::ptr::null_mut();
    let mut shp_lag_ptr: *mut opus_int32 = std::ptr::null_mut();
    let mut pred_lag_ptr: *mut opus_int32 = std::ptr::null_mut();
    shp_lag_ptr = &mut *(*NSQ)
        .sLTP_shp_Q14
        .as_mut_ptr()
        .offset(((*NSQ).sLTP_shp_buf_idx - lag + 3 as i32 / 2 as i32) as isize)
        as *mut opus_int32;
    pred_lag_ptr = &mut *sLTP_Q15.offset(((*NSQ).sLTP_buf_idx - lag + 5 as i32 / 2 as i32) as isize)
        as *mut opus_int32;
    Gain_Q10 = Gain_Q16 >> 6 as i32;
    /* Set up short term AR state */
    psLPC_Q14 = &mut *(*NSQ)
        .sLPC_Q14
        .as_mut_ptr()
        .offset((16 as i32 - 1 as i32) as isize) as *mut opus_int32;
    i = 0 as i32;
    while i < length {
        /* Generate dither */
        (*NSQ).rand_seed = (907633515 as i32 as opus_uint32).wrapping_add(
            ((*NSQ).rand_seed as opus_uint32).wrapping_mul(196314165 as i32 as opus_uint32),
        ) as opus_int32;
        /* Short-term prediction */
        LPC_pred_Q10 =
            silk_noise_shape_quantizer_short_prediction_c(psLPC_Q14, a_Q12, predictLPCOrder);
        /* Long-term prediction */
        if signalType == 2 as i32 {
            /* Unrolled loop */
            /* Avoids introducing a bias because silk_SMLAWB() always rounds to -inf */
            LTP_pred_Q13 = 2 as i32;
            LTP_pred_Q13 = (LTP_pred_Q13 as i64
                + (*pred_lag_ptr.offset(0 as i32 as isize) as i64
                    * *b_Q14.offset(0 as i32 as isize) as i64
                    >> 16 as i32)) as opus_int32;
            LTP_pred_Q13 = (LTP_pred_Q13 as i64
                + (*pred_lag_ptr.offset(-(1 as i32) as isize) as i64
                    * *b_Q14.offset(1 as i32 as isize) as i64
                    >> 16 as i32)) as opus_int32;
            LTP_pred_Q13 = (LTP_pred_Q13 as i64
                + (*pred_lag_ptr.offset(-(2 as i32) as isize) as i64
                    * *b_Q14.offset(2 as i32 as isize) as i64
                    >> 16 as i32)) as opus_int32;
            LTP_pred_Q13 = (LTP_pred_Q13 as i64
                + (*pred_lag_ptr.offset(-(3 as i32) as isize) as i64
                    * *b_Q14.offset(3 as i32 as isize) as i64
                    >> 16 as i32)) as opus_int32;
            LTP_pred_Q13 = (LTP_pred_Q13 as i64
                + (*pred_lag_ptr.offset(-(4 as i32) as isize) as i64
                    * *b_Q14.offset(4 as i32 as isize) as i64
                    >> 16 as i32)) as opus_int32;
            pred_lag_ptr = pred_lag_ptr.offset(1)
        } else {
            LTP_pred_Q13 = 0 as i32
        }
        /* Noise shape feedback */
        /* check that order is even */
        n_AR_Q12 = silk_NSQ_noise_shape_feedback_loop_c(
            &mut (*NSQ).sDiff_shp_Q14,
            (*NSQ).sAR2_Q14.as_mut_ptr(),
            AR_shp_Q13,
            shapingLPCOrder,
        );
        n_AR_Q12 = (n_AR_Q12 as i64
            + ((*NSQ).sLF_AR_shp_Q14 as i64 * Tilt_Q14 as opus_int16 as i64 >> 16 as i32))
            as opus_int32;
        n_LF_Q12 = ((*NSQ).sLTP_shp_Q14[((*NSQ).sLTP_shp_buf_idx - 1 as i32) as usize] as i64
            * LF_shp_Q14 as opus_int16 as i64
            >> 16 as i32) as opus_int32;
        n_LF_Q12 = (n_LF_Q12 as i64
            + ((*NSQ).sLF_AR_shp_Q14 as i64 * (LF_shp_Q14 as i64 >> 16 as i32) >> 16 as i32))
            as opus_int32;
        /* Combine prediction and noise shaping signals */
        tmp1 = ((LPC_pred_Q10 as opus_uint32) << 2 as i32) as opus_int32 - n_AR_Q12; /* Q12 */
        tmp1 = tmp1 - n_LF_Q12; /* Q12 */
        if lag > 0 as i32 {
            /* Symmetric, packed FIR coefficients */
            n_LTP_Q13 = ((*shp_lag_ptr.offset(0 as i32 as isize)
                + *shp_lag_ptr.offset(-(2 as i32) as isize)) as i64
                * HarmShapeFIRPacked_Q14 as opus_int16 as i64
                >> 16 as i32) as opus_int32;
            n_LTP_Q13 = (n_LTP_Q13 as i64
                + (*shp_lag_ptr.offset(-(1 as i32) as isize) as i64
                    * (HarmShapeFIRPacked_Q14 as i64 >> 16 as i32)
                    >> 16 as i32)) as opus_int32;
            n_LTP_Q13 = ((n_LTP_Q13 as opus_uint32) << 1 as i32) as opus_int32;
            shp_lag_ptr = shp_lag_ptr.offset(1);
            /* Q10 */
            tmp2 = LTP_pred_Q13 - n_LTP_Q13; /* Q13 */
            tmp1 = tmp2 + ((tmp1 as opus_uint32) << 1 as i32) as opus_int32; /* Q13 */
            tmp1 = if 3 as i32 == 1 as i32 {
                (tmp1 >> 1 as i32) + (tmp1 & 1 as i32)
            } else {
                ((tmp1 >> 3 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
            }
        } else {
            tmp1 = if 2 as i32 == 1 as i32 {
                (tmp1 >> 1 as i32) + (tmp1 & 1 as i32)
            } else {
                ((tmp1 >> 2 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
            }
            /* Q10 */
        } /* residual error Q10 */
        r_Q10 = *x_sc_Q10.offset(i as isize) - tmp1;
        /* Flip sign depending on dither */
        if (*NSQ).rand_seed < 0 as i32 {
            r_Q10 = -r_Q10
        }
        r_Q10 = if -((31 as i32) << 10 as i32) > (30 as i32) << 10 as i32 {
            if r_Q10 > -((31 as i32) << 10 as i32) {
                -((31 as i32) << 10 as i32)
            } else if r_Q10 < (30 as i32) << 10 as i32 {
                (30 as i32) << 10 as i32
            } else {
                r_Q10
            }
        } else if r_Q10 > (30 as i32) << 10 as i32 {
            (30 as i32) << 10 as i32
        } else if r_Q10 < -((31 as i32) << 10 as i32) {
            -((31 as i32) << 10 as i32)
        } else {
            r_Q10
        };
        /* Find two quantization level candidates and measure their rate-distortion */
        q1_Q10 = r_Q10 - offset_Q10;
        q1_Q0 = q1_Q10 >> 10 as i32;
        if Lambda_Q10 > 2048 as i32 {
            /* For aggressive RDO, the bias becomes more than one pulse. */
            let mut rdo_offset: i32 = Lambda_Q10 / 2 as i32 - 512 as i32; /* Q1_Q0 < -1 */
            if q1_Q10 > rdo_offset {
                q1_Q0 = q1_Q10 - rdo_offset >> 10 as i32
            } else if q1_Q10 < -rdo_offset {
                q1_Q0 = q1_Q10 + rdo_offset >> 10 as i32
            } else if q1_Q10 < 0 as i32 {
                q1_Q0 = -(1 as i32)
            } else {
                q1_Q0 = 0 as i32
            }
        }
        if q1_Q0 > 0 as i32 {
            q1_Q10 = ((q1_Q0 as opus_uint32) << 10 as i32) as opus_int32 - 80 as i32;
            q1_Q10 = q1_Q10 + offset_Q10;
            q2_Q10 = q1_Q10 + 1024 as i32;
            rd1_Q20 = q1_Q10 as opus_int16 as opus_int32 * Lambda_Q10 as opus_int16 as opus_int32;
            rd2_Q20 = q2_Q10 as opus_int16 as opus_int32 * Lambda_Q10 as opus_int16 as opus_int32
        } else if q1_Q0 == 0 as i32 {
            q1_Q10 = offset_Q10;
            q2_Q10 = q1_Q10 + (1024 as i32 - 80 as i32);
            rd1_Q20 = q1_Q10 as opus_int16 as opus_int32 * Lambda_Q10 as opus_int16 as opus_int32;
            rd2_Q20 = q2_Q10 as opus_int16 as opus_int32 * Lambda_Q10 as opus_int16 as opus_int32
        } else if q1_Q0 == -(1 as i32) {
            q2_Q10 = offset_Q10;
            q1_Q10 = q2_Q10 - (1024 as i32 - 80 as i32);
            rd1_Q20 = -q1_Q10 as opus_int16 as opus_int32 * Lambda_Q10 as opus_int16 as opus_int32;
            rd2_Q20 = q2_Q10 as opus_int16 as opus_int32 * Lambda_Q10 as opus_int16 as opus_int32
        } else {
            q1_Q10 = ((q1_Q0 as opus_uint32) << 10 as i32) as opus_int32 + 80 as i32;
            q1_Q10 = q1_Q10 + offset_Q10;
            q2_Q10 = q1_Q10 + 1024 as i32;
            rd1_Q20 = -q1_Q10 as opus_int16 as opus_int32 * Lambda_Q10 as opus_int16 as opus_int32;
            rd2_Q20 = -q2_Q10 as opus_int16 as opus_int32 * Lambda_Q10 as opus_int16 as opus_int32
        }
        rr_Q10 = r_Q10 - q1_Q10;
        rd1_Q20 = rd1_Q20 + rr_Q10 as opus_int16 as opus_int32 * rr_Q10 as opus_int16 as opus_int32;
        rr_Q10 = r_Q10 - q2_Q10;
        rd2_Q20 = rd2_Q20 + rr_Q10 as opus_int16 as opus_int32 * rr_Q10 as opus_int16 as opus_int32;
        if rd2_Q20 < rd1_Q20 {
            q1_Q10 = q2_Q10
        }
        *pulses.offset(i as isize) = if 10 as i32 == 1 as i32 {
            (q1_Q10 >> 1 as i32) + (q1_Q10 & 1 as i32)
        } else {
            ((q1_Q10 >> 10 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        } as i8;
        /* Excitation */
        exc_Q14 = ((q1_Q10 as opus_uint32) << 4 as i32) as opus_int32;
        if (*NSQ).rand_seed < 0 as i32 {
            exc_Q14 = -exc_Q14
        }
        /* Add predictions */
        LPC_exc_Q14 = exc_Q14 + ((LTP_pred_Q13 as opus_uint32) << 1 as i32) as opus_int32;
        xq_Q14 = LPC_exc_Q14 + ((LPC_pred_Q10 as opus_uint32) << 4 as i32) as opus_int32;
        /* Scale XQ back to normal level before saving */
        *xq.offset(i as isize) = if (if 8 as i32 == 1 as i32 {
            ((xq_Q14 as i64 * Gain_Q10 as i64 >> 16 as i32) as opus_int32 >> 1 as i32)
                + ((xq_Q14 as i64 * Gain_Q10 as i64 >> 16 as i32) as opus_int32 & 1 as i32)
        } else {
            (((xq_Q14 as i64 * Gain_Q10 as i64 >> 16 as i32) as opus_int32 >> 8 as i32 - 1 as i32)
                + 1 as i32)
                >> 1 as i32
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else if (if 8 as i32 == 1 as i32 {
            ((xq_Q14 as i64 * Gain_Q10 as i64 >> 16 as i32) as opus_int32 >> 1 as i32)
                + ((xq_Q14 as i64 * Gain_Q10 as i64 >> 16 as i32) as opus_int32 & 1 as i32)
        } else {
            (((xq_Q14 as i64 * Gain_Q10 as i64 >> 16 as i32) as opus_int32 >> 8 as i32 - 1 as i32)
                + 1 as i32)
                >> 1 as i32
        }) < 0x8000 as i32 as opus_int16 as i32
        {
            0x8000 as i32 as opus_int16 as i32
        } else if 8 as i32 == 1 as i32 {
            ((xq_Q14 as i64 * Gain_Q10 as i64 >> 16 as i32) as opus_int32 >> 1 as i32)
                + ((xq_Q14 as i64 * Gain_Q10 as i64 >> 16 as i32) as opus_int32 & 1 as i32)
        } else {
            (((xq_Q14 as i64 * Gain_Q10 as i64 >> 16 as i32) as opus_int32 >> 8 as i32 - 1 as i32)
                + 1 as i32)
                >> 1 as i32
        } as opus_int16;
        /* Update states */
        psLPC_Q14 = psLPC_Q14.offset(1);
        *psLPC_Q14 = xq_Q14;
        (*NSQ).sDiff_shp_Q14 =
            xq_Q14 - ((*x_sc_Q10.offset(i as isize) as opus_uint32) << 4 as i32) as opus_int32;
        sLF_AR_shp_Q14 =
            (*NSQ).sDiff_shp_Q14 - ((n_AR_Q12 as opus_uint32) << 2 as i32) as opus_int32;
        (*NSQ).sLF_AR_shp_Q14 = sLF_AR_shp_Q14;
        (*NSQ).sLTP_shp_Q14[(*NSQ).sLTP_shp_buf_idx as usize] =
            sLF_AR_shp_Q14 - ((n_LF_Q12 as opus_uint32) << 2 as i32) as opus_int32;
        *sLTP_Q15.offset((*NSQ).sLTP_buf_idx as isize) =
            ((LPC_exc_Q14 as opus_uint32) << 1 as i32) as opus_int32;
        (*NSQ).sLTP_shp_buf_idx += 1;
        (*NSQ).sLTP_buf_idx += 1;
        /* Make dither dependent on quantized signal */
        (*NSQ).rand_seed = ((*NSQ).rand_seed as opus_uint32)
            .wrapping_add(*pulses.offset(i as isize) as opus_uint32)
            as opus_int32;
        i += 1
    }
    /* Update LPC synth buffer */
    crate::stdlib::memcpy(
        (*NSQ).sLPC_Q14.as_mut_ptr() as *mut libc::c_void,
        &mut *(*NSQ).sLPC_Q14.as_mut_ptr().offset(length as isize) as *mut opus_int32
            as *const libc::c_void,
        (16 as i32 as usize).wrapping_mul(::std::mem::size_of::<opus_int32>() as usize),
    );
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
#[inline]

unsafe extern "C" fn silk_nsq_scale_states(
    mut psEncC: *const silk_encoder_state,
    mut NSQ: *mut silk_nsq_state,
    mut x16: *const opus_int16,
    mut x_sc_Q10: *mut opus_int32,
    mut sLTP: *const opus_int16,
    mut sLTP_Q15: *mut opus_int32,
    mut subfr: i32,
    LTP_scale_Q14: i32,
    mut Gains_Q16: *const opus_int32,
    mut pitchL: *const i32,
    signal_type: i32,
)
/* I    Signal type                     */
{
    let mut i: i32 = 0;
    let mut lag: i32 = 0;
    let mut gain_adj_Q16: opus_int32 = 0;
    let mut inv_gain_Q31: opus_int32 = 0;
    let mut inv_gain_Q26: opus_int32 = 0;
    lag = *pitchL.offset(subfr as isize);
    inv_gain_Q31 = silk_INVERSE32_varQ(
        if *Gains_Q16.offset(subfr as isize) > 1 as i32 {
            *Gains_Q16.offset(subfr as isize)
        } else {
            1 as i32
        },
        47 as i32,
    );
    /* Scale input */
    inv_gain_Q26 = if 5 as i32 == 1 as i32 {
        (inv_gain_Q31 >> 1 as i32) + (inv_gain_Q31 & 1 as i32)
    } else {
        ((inv_gain_Q31 >> 5 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
    };
    i = 0 as i32;
    while i < (*psEncC).subfr_length {
        *x_sc_Q10.offset(i as isize) =
            (*x16.offset(i as isize) as i64 * inv_gain_Q26 as i64 >> 16 as i32) as opus_int32;
        i += 1
    }
    /* After rewhitening the LTP state is un-scaled, so scale with inv_gain_Q16 */
    if (*NSQ).rewhite_flag != 0 {
        if subfr == 0 as i32 {
            /* Do LTP downscaling */
            inv_gain_Q31 = (((inv_gain_Q31 as i64 * LTP_scale_Q14 as opus_int16 as i64 >> 16 as i32)
                as opus_int32 as opus_uint32)
                << 2 as i32) as opus_int32
        }
        i = (*NSQ).sLTP_buf_idx - lag - 5 as i32 / 2 as i32;
        while i < (*NSQ).sLTP_buf_idx {
            *sLTP_Q15.offset(i as isize) =
                (inv_gain_Q31 as i64 * *sLTP.offset(i as isize) as i64 >> 16 as i32) as opus_int32;
            i += 1
        }
    }
    /* Adjust for changing gain */
    if *Gains_Q16.offset(subfr as isize) != (*NSQ).prev_gain_Q16 {
        gain_adj_Q16 = silk_DIV32_varQ(
            (*NSQ).prev_gain_Q16,
            *Gains_Q16.offset(subfr as isize),
            16 as i32,
        );
        /* Scale long-term shaping state */
        i = (*NSQ).sLTP_shp_buf_idx - (*psEncC).ltp_mem_length;
        while i < (*NSQ).sLTP_shp_buf_idx {
            (*NSQ).sLTP_shp_Q14[i as usize] = (gain_adj_Q16 as i64
                * (*NSQ).sLTP_shp_Q14[i as usize] as i64
                >> 16 as i32) as opus_int32;
            i += 1
        }
        /* Scale long-term prediction state */
        if signal_type == 2 as i32 && (*NSQ).rewhite_flag == 0 as i32 {
            i = (*NSQ).sLTP_buf_idx - lag - 5 as i32 / 2 as i32;
            while i < (*NSQ).sLTP_buf_idx {
                *sLTP_Q15.offset(i as isize) = (gain_adj_Q16 as i64
                    * *sLTP_Q15.offset(i as isize) as i64
                    >> 16 as i32) as opus_int32;
                i += 1
            }
        }
        (*NSQ).sLF_AR_shp_Q14 =
            (gain_adj_Q16 as i64 * (*NSQ).sLF_AR_shp_Q14 as i64 >> 16 as i32) as opus_int32;
        (*NSQ).sDiff_shp_Q14 =
            (gain_adj_Q16 as i64 * (*NSQ).sDiff_shp_Q14 as i64 >> 16 as i32) as opus_int32;
        /* Scale short-term prediction and shaping states */
        i = 0 as i32;
        while i < 16 as i32 {
            (*NSQ).sLPC_Q14[i as usize] = (gain_adj_Q16 as i64 * (*NSQ).sLPC_Q14[i as usize] as i64
                >> 16 as i32) as opus_int32;
            i += 1
        }
        i = 0 as i32;
        while i < 24 as i32 {
            (*NSQ).sAR2_Q14[i as usize] = (gain_adj_Q16 as i64 * (*NSQ).sAR2_Q14[i as usize] as i64
                >> 16 as i32) as opus_int32;
            i += 1
        }
        /* Save inverse gain */
        (*NSQ).prev_gain_Q16 = *Gains_Q16.offset(subfr as isize)
    };
}

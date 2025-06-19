pub mod SigProc_FIX_h {
    /* Allocate opus_int16 aligned to 4-byte memory address */
    /* Useful Macros that can be adjusted to other platforms */
    /* Fixed point macros */
    /* (a32 * b32) output have to be 32bit int */
    /* (a32 * b32) output have to be 32bit uint */
    /* a32 + (b32 * c32) output have to be 32bit int */
    /* a32 + (b32 * c32) output have to be 32bit uint */
    /* ((a32 >> 16)  * (b32 >> 16)) output have to be 32bit int */
    /* a32 + ((a32 >> 16)  * (b32 >> 16)) output have to be 32bit int */
    /* (a32 * b32) */
    /*(opus_int64)*/
    /* Adds two signed 32-bit values in a way that can overflow, while not relying on undefined behaviour
    (just standard two's complement implementation-specific behaviour) */
    /* Subtractss two signed 32-bit values in a way that can overflow, while not relying on undefined behaviour
    (just standard two's complement implementation-specific behaviour) */
    /* Multiply-accumulate macros that allow overflow in the addition (ie, no asserts in debug mode) */
    /* These macros enables checking for overflow in silk_API_Debug.h*/
    /* Saturation for positive input values */
    /* Add with saturation for positive input values */
    /* shift >= 0, shift < 8  */
    /* shift >= 0, shift < 16 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 64 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 8  */
    /* shift >= 0, shift < 16 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 64 */
    /* shift >= 0, shift < 32 */
    /* saturates before shifting */
    /* shift >= 0, allowed to overflow */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* Requires that shift > 0 */
    /* Number of rightshift required to fit the multiplication */
    /* Macro to convert floating-point constants to fixed-point */
    /* silk_min() versions with typecast in the function call */
    #[inline]

    pub unsafe extern "C" fn silk_min_int(mut a: i32, mut b: i32) -> i32 {
        return if a < b { a } else { b };
    }
    #[inline]

    pub unsafe extern "C" fn silk_min_32(
        mut a: crate::opus_types_h::opus_int32,
        mut b: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        return if a < b { a } else { b };
    }
    /* silk_min() versions with typecast in the function call */
    #[inline]

    pub unsafe extern "C" fn silk_max_int(mut a: i32, mut b: i32) -> i32 {
        return if a > b { a } else { b };
    }

    /* SILK_SIGPROC_FIX_H */
    /*    silk_SMMUL: Signed top word multiply.
    ARMv6        2 instruction cycles.
    ARMv3M+      3 instruction cycles. use SMULL and ignore LSB registers.(except xM)*/
    /*#define silk_SMMUL(a32, b32)                (opus_int32)silk_RSHIFT(silk_SMLAL(silk_SMULWB((a32), (b32)), (a32), silk_RSHIFT_ROUND((b32), 16)), 16)*/
    /* the following seems faster on x86 */
    /*  Add some multiplication functions that can be easily mapped to ARM. */
    /* PSEUDO-RANDOM GENERATOR                                                          */
    /* Make sure to store the result as the seed for the next call (also in between     */
    /* frames), otherwise result won't be random at all. When only using some of the    */
    /* bits, take the most significant bits by right-shifting.                          */
    /* Be careful, silk_abs returns wrong when input equals to silk_intXX_MIN */
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
pub use crate::src::opus_1_2_1::silk::gain_quant::SigProc_FIX_h::silk_max_int;
pub use crate::src::opus_1_2_1::silk::gain_quant::SigProc_FIX_h::silk_min_32;
pub use crate::src::opus_1_2_1::silk::gain_quant::SigProc_FIX_h::silk_min_int;
pub use crate::src::opus_1_2_1::silk::lin2log::silk_lin2log;
pub use crate::src::opus_1_2_1::silk::log2lin::silk_log2lin;
/* Gain scalar quantization with hysteresis, uniform on log scale */
#[no_mangle]

pub unsafe extern "C" fn silk_gains_quant(
    mut ind: *mut i8,
    mut gain_Q16: *mut crate::opus_types_h::opus_int32,
    mut prev_ind: *mut i8,
    conditional: i32,
    nb_subfr: i32,
)
/* I    number of subframes                         */
{
    let mut k: i32 = 0;
    let mut double_step_size_threshold: i32 = 0;
    k = 0 as i32;
    while k < nb_subfr {
        /* Convert to log scale, scale, floor() */
        *ind.offset(k as isize) = ((65536 as i32 * (64 as i32 - 1 as i32)
            / ((88 as i32 - 2 as i32) * 128 as i32 / 6 as i32))
            as i64
            * (crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(*gain_Q16.offset(k as isize))
                - (2 as i32 * 128 as i32 / 6 as i32 + 16 as i32 * 128 as i32))
                as crate::opus_types_h::opus_int16 as i64
            >> 16 as i32) as crate::opus_types_h::opus_int32
            as i8;
        /* 3967 = 31 in Q7 */
        if (*ind.offset(k as isize) as i32) < *prev_ind as i32 {
            let ref mut fresh0 = *ind.offset(k as isize);
            *fresh0 += 1
        }
        *ind.offset(k as isize) = if 0 as i32 > 64 as i32 - 1 as i32 {
            if *ind.offset(k as isize) as i32 > 0 as i32 {
                0 as i32
            } else if (*ind.offset(k as isize) as i32) < 64 as i32 - 1 as i32 {
                (64 as i32) - 1 as i32
            } else {
                *ind.offset(k as isize) as i32
            }
        } else if *ind.offset(k as isize) as i32 > 64 as i32 - 1 as i32 {
            (64 as i32) - 1 as i32
        } else if (*ind.offset(k as isize) as i32) < 0 as i32 {
            0 as i32
        } else {
            *ind.offset(k as isize) as i32
        } as i8;
        if k == 0 as i32 && conditional == 0 as i32 {
            /* Round towards previous quantized gain (hysteresis) */
            /* Compute delta indices and limit */
            /* Full index */
            *ind.offset(k as isize) = if *prev_ind as i32 + -(4 as i32) > 64 as i32 - 1 as i32 {
                if *ind.offset(k as isize) as i32 > *prev_ind as i32 + -(4 as i32) {
                    (*prev_ind as i32) + -(4 as i32)
                } else if (*ind.offset(k as isize) as i32) < 64 as i32 - 1 as i32 {
                    (64 as i32) - 1 as i32
                } else {
                    *ind.offset(k as isize) as i32
                }
            } else if *ind.offset(k as isize) as i32 > 64 as i32 - 1 as i32 {
                (64 as i32) - 1 as i32
            } else if (*ind.offset(k as isize) as i32) < *prev_ind as i32 + -(4 as i32) {
                (*prev_ind as i32) + -(4 as i32)
            } else {
                *ind.offset(k as isize) as i32
            } as i8;
            *prev_ind = *ind.offset(k as isize)
        } else {
            /* Delta index */
            *ind.offset(k as isize) = (*ind.offset(k as isize) as i32 - *prev_ind as i32) as i8;
            /* Double the quantization step size for large gain increases, so that the max gain level can be reached */
            double_step_size_threshold = 2 as i32 * 36 as i32 - 64 as i32 + *prev_ind as i32;
            if *ind.offset(k as isize) as i32 > double_step_size_threshold {
                *ind.offset(k as isize) = (double_step_size_threshold
                    + (*ind.offset(k as isize) as i32 - double_step_size_threshold + 1 as i32
                        >> 1 as i32)) as i8
            }
            *ind.offset(k as isize) = if -(4 as i32) > 36 as i32 {
                if *ind.offset(k as isize) as i32 > -(4 as i32) {
                    -(4 as i32)
                } else if (*ind.offset(k as isize) as i32) < 36 as i32 {
                    36 as i32
                } else {
                    *ind.offset(k as isize) as i32
                }
            } else if *ind.offset(k as isize) as i32 > 36 as i32 {
                36 as i32
            } else if (*ind.offset(k as isize) as i32) < -(4 as i32) {
                -(4 as i32)
            } else {
                *ind.offset(k as isize) as i32
            } as i8;
            /* Accumulate deltas */
            if *ind.offset(k as isize) as i32 > double_step_size_threshold {
                *prev_ind = (*prev_ind as i32
                    + (((*ind.offset(k as isize) as crate::opus_types_h::opus_uint32) << 1 as i32)
                        as crate::opus_types_h::opus_int32
                        - double_step_size_threshold)) as i8;
                *prev_ind = silk_min_int(*prev_ind as i32, 64 as i32 - 1 as i32) as i8
            } else {
                *prev_ind = (*prev_ind as i32 + *ind.offset(k as isize) as i32) as i8
            }
            /* Shift to make non-negative */
            let ref mut fresh1 = *ind.offset(k as isize);
            *fresh1 = (*fresh1 as i32 - -(4 as i32)) as i8
        }
        *gain_Q16.offset(k as isize) =
            crate::src::opus_1_2_1::silk::log2lin::silk_log2lin(silk_min_32(
                ((65536 as i32 * ((88 as i32 - 2 as i32) * 128 as i32 / 6 as i32)
                    / (64 as i32 - 1 as i32)) as i64
                    * *prev_ind as crate::opus_types_h::opus_int16 as i64
                    >> 16 as i32) as crate::opus_types_h::opus_int32
                    + (2 as i32 * 128 as i32 / 6 as i32 + 16 as i32 * 128 as i32),
                3967 as i32,
            ));
        k += 1
    }
}
/* Scale and convert to linear scale */
/* Gains scalar dequantization, uniform on log scale */
#[no_mangle]

pub unsafe extern "C" fn silk_gains_dequant(
    mut gain_Q16: *mut crate::opus_types_h::opus_int32,
    mut ind: *const i8,
    mut prev_ind: *mut i8,
    conditional: i32,
    nb_subfr: i32,
)
/* I    number of subframes                          */
{
    let mut k: i32 = 0;
    let mut ind_tmp: i32 = 0;
    let mut double_step_size_threshold: i32 = 0;
    k = 0 as i32;
    while k < nb_subfr {
        if k == 0 as i32 && conditional == 0 as i32 {
            /* Gain index is not allowed to go down more than 16 steps (~21.8 dB) */
            *prev_ind =
                silk_max_int(*ind.offset(k as isize) as i32, *prev_ind as i32 - 16 as i32) as i8
        } else {
            /* Delta index */
            ind_tmp = *ind.offset(k as isize) as i32 + -(4 as i32);
            /* Accumulate deltas */
            double_step_size_threshold = 2 as i32 * 36 as i32 - 64 as i32 + *prev_ind as i32;
            if ind_tmp > double_step_size_threshold {
                *prev_ind = (*prev_ind as i32
                    + (((ind_tmp as crate::opus_types_h::opus_uint32) << 1 as i32)
                        as crate::opus_types_h::opus_int32
                        - double_step_size_threshold)) as i8
            } else {
                *prev_ind = (*prev_ind as i32 + ind_tmp) as i8
            }
        }
        *prev_ind = if 0 as i32 > 64 as i32 - 1 as i32 {
            if *prev_ind as i32 > 0 as i32 {
                0 as i32
            } else if (*prev_ind as i32) < 64 as i32 - 1 as i32 {
                (64 as i32) - 1 as i32
            } else {
                *prev_ind as i32
            }
        } else if *prev_ind as i32 > 64 as i32 - 1 as i32 {
            (64 as i32) - 1 as i32
        } else if (*prev_ind as i32) < 0 as i32 {
            0 as i32
        } else {
            *prev_ind as i32
        } as i8;
        /* 3967 = 31 in Q7 */
        *gain_Q16.offset(k as isize) =
            crate::src::opus_1_2_1::silk::log2lin::silk_log2lin(silk_min_32(
                ((65536 as i32 * ((88 as i32 - 2 as i32) * 128 as i32 / 6 as i32)
                    / (64 as i32 - 1 as i32)) as i64
                    * *prev_ind as crate::opus_types_h::opus_int16 as i64
                    >> 16 as i32) as crate::opus_types_h::opus_int32
                    + (2 as i32 * 128 as i32 / 6 as i32 + 16 as i32 * 128 as i32),
                3967 as i32,
            ));
        k += 1
    }
}
/* Scale and convert to linear scale */
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
/* Compute unique identifier of gain indices vector */
#[no_mangle]

pub unsafe extern "C" fn silk_gains_ID(
    mut ind: *const i8,
    nb_subfr: i32,
) -> crate::opus_types_h::opus_int32
/* I    number of subframes                         */ {
    let mut k: i32 = 0;
    let mut gainsID: crate::opus_types_h::opus_int32 = 0;
    gainsID = 0 as i32;
    k = 0 as i32;
    while k < nb_subfr {
        gainsID = *ind.offset(k as isize) as i32
            + ((gainsID as crate::opus_types_h::opus_uint32) << 8 as i32)
                as crate::opus_types_h::opus_int32;
        k += 1
    }
    return gainsID;
}

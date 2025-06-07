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

pub mod SigProc_FIX_h {
    /* *******************************************************************/
    /*                                MACROS                            */
    /* *******************************************************************/
    /* Rotate a32 right by 'rot' bits. Negative rot values result in rotating
    left. Output is 32bit int.
    Note: contemporary compilers recognize the C expression below and
    compile it into a 'ror' instruction if available. No need for OPUS_INLINE ASM! */
    #[inline]

    pub unsafe extern "C" fn silk_ROR32(
        mut a32: crate::opus_types_h::opus_int32,
        mut rot: libc::c_int,
    ) -> crate::opus_types_h::opus_int32 {
        let mut x: crate::opus_types_h::opus_uint32 = a32 as crate::opus_types_h::opus_uint32;
        let mut r: crate::opus_types_h::opus_uint32 = rot as crate::opus_types_h::opus_uint32;
        let mut m: crate::opus_types_h::opus_uint32 = -rot as crate::opus_types_h::opus_uint32;
        if rot == 0 as libc::c_int {
            return a32;
        } else if rot < 0 as libc::c_int {
            return (x << m | x >> (32 as libc::c_int as libc::c_uint).wrapping_sub(m))
                as crate::opus_types_h::opus_int32;
        } else {
            return (x << (32 as libc::c_int as libc::c_uint).wrapping_sub(r) | x >> r)
                as crate::opus_types_h::opus_int32;
        };
    }
    /* silk_min() versions with typecast in the function call */
    #[inline]

    pub unsafe extern "C" fn silk_max_int(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
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

pub mod Inlines_h {
    /* get number of leading zeros and fractional part (the bits right after the leading one */
    #[inline]

    pub unsafe extern "C" fn silk_CLZ_FRAC(
        mut in_0: crate::opus_types_h::opus_int32,
        mut lz: *mut crate::opus_types_h::opus_int32,
        mut frac_Q7: *mut crate::opus_types_h::opus_int32,
    )
    /* O  the 7 bits right after the leading one */
    {
        let mut lzeros: crate::opus_types_h::opus_int32 = silk_CLZ32(in_0);
        *lz = lzeros;
        *frac_Q7 = silk_ROR32(in_0, 24 as libc::c_int - lzeros) & 0x7f as libc::c_int;
    }
    /* Approximation of square root                                          */
    /* Accuracy: < +/- 10%  for output values > 15                           */
    /*           < +/- 2.5% for output values > 120                          */
    #[inline]

    pub unsafe extern "C" fn silk_SQRT_APPROX(
        mut x: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        let mut y: crate::opus_types_h::opus_int32 = 0;
        let mut lz: crate::opus_types_h::opus_int32 = 0;
        let mut frac_Q7: crate::opus_types_h::opus_int32 = 0;
        if x <= 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        silk_CLZ_FRAC(x, &mut lz, &mut frac_Q7);
        if lz & 1 as libc::c_int != 0 {
            y = 32768 as libc::c_int
        } else {
            y = 46214 as libc::c_int
            /* 46214 = sqrt(2) * 32768 */
        }
        /* get scaling right */
        y >>= lz >> 1 as libc::c_int;
        /* increment using fractional part of input */
        y = (y as libc::c_longlong
            + (y as libc::c_longlong
                * (213 as libc::c_int as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
                    * frac_Q7 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32)
                    as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32;
        return y;
    }
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

    use crate::src::opus_1_2_1::silk::stereo_find_predictor::macros_h::silk_CLZ32;
    use crate::src::opus_1_2_1::silk::stereo_find_predictor::SigProc_FIX_h::silk_ROR32;
    /* SILK_FIX_INLINES_H */
}

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::silk::stereo_find_predictor::macros_h::silk_CLZ32;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::src::opus_1_2_1::silk::inner_prod_aligned::silk_inner_prod_aligned_scale;
pub use crate::src::opus_1_2_1::silk::stereo_find_predictor::Inlines_h::silk_CLZ_FRAC;
pub use crate::src::opus_1_2_1::silk::stereo_find_predictor::Inlines_h::silk_DIV32_varQ;
pub use crate::src::opus_1_2_1::silk::stereo_find_predictor::Inlines_h::silk_SQRT_APPROX;
pub use crate::src::opus_1_2_1::silk::stereo_find_predictor::SigProc_FIX_h::silk_ROR32;
pub use crate::src::opus_1_2_1::silk::stereo_find_predictor::SigProc_FIX_h::silk_max_int;
pub use crate::src::opus_1_2_1::silk::sum_sqr_shift::silk_sum_sqr_shift;
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
/* Find least-squares prediction gain for one signal based on another and quantize it */
#[no_mangle]

pub unsafe extern "C" fn silk_stereo_find_predictor(
    mut ratio_Q14: *mut crate::opus_types_h::opus_int32,
    mut x: *const crate::opus_types_h::opus_int16,
    mut y: *const crate::opus_types_h::opus_int16,
    mut mid_res_amp_Q0: *mut crate::opus_types_h::opus_int32,
    mut length: libc::c_int,
    mut smooth_coef_Q16: libc::c_int,
) -> crate::opus_types_h::opus_int32
/* I    Smoothing coefficient                       */ {
    let mut scale: libc::c_int = 0;
    let mut scale1: libc::c_int = 0;
    let mut scale2: libc::c_int = 0;
    let mut nrgx: crate::opus_types_h::opus_int32 = 0;
    let mut nrgy: crate::opus_types_h::opus_int32 = 0;
    let mut corr: crate::opus_types_h::opus_int32 = 0;
    let mut pred_Q13: crate::opus_types_h::opus_int32 = 0;
    let mut pred2_Q10: crate::opus_types_h::opus_int32 = 0;
    /* Find  predictor */
    crate::src::opus_1_2_1::silk::sum_sqr_shift::silk_sum_sqr_shift(
        &mut nrgx,
        &mut scale1,
        x,
        length,
    ); /* make even */
    crate::src::opus_1_2_1::silk::sum_sqr_shift::silk_sum_sqr_shift(
        &mut nrgy,
        &mut scale2,
        y,
        length,
    );
    scale = silk_max_int(scale1, scale2);
    scale = scale + (scale & 1 as libc::c_int);
    nrgy = nrgy >> scale - scale2;
    nrgx = nrgx >> scale - scale1;
    nrgx = silk_max_int(nrgx, 1 as libc::c_int);
    corr = crate::src::opus_1_2_1::silk::inner_prod_aligned::silk_inner_prod_aligned_scale(
        x, y, scale, length,
    );
    pred_Q13 = silk_DIV32_varQ(corr, nrgx, 13 as libc::c_int);
    pred_Q13 =
        if -((1 as libc::c_int) << 14 as libc::c_int) > (1 as libc::c_int) << 14 as libc::c_int {
            if pred_Q13 > -((1 as libc::c_int) << 14 as libc::c_int) {
                -((1 as libc::c_int) << 14 as libc::c_int)
            } else if pred_Q13 < (1 as libc::c_int) << 14 as libc::c_int {
                (1 as libc::c_int) << 14 as libc::c_int
            } else {
                pred_Q13
            }
        } else if pred_Q13 > (1 as libc::c_int) << 14 as libc::c_int {
            (1 as libc::c_int) << 14 as libc::c_int
        } else if pred_Q13 < -((1 as libc::c_int) << 14 as libc::c_int) {
            -((1 as libc::c_int) << 14 as libc::c_int)
        } else {
            pred_Q13
        };
    pred2_Q10 = (pred_Q13 as libc::c_longlong
        * pred_Q13 as crate::opus_types_h::opus_int16 as libc::c_longlong
        >> 16 as libc::c_int) as crate::opus_types_h::opus_int32;
    /* Faster update for signals with large prediction parameters */
    smooth_coef_Q16 = silk_max_int(
        smooth_coef_Q16,
        if pred2_Q10 > 0 as libc::c_int {
            pred2_Q10
        } else {
            -pred2_Q10
        },
    );
    /* Smoothed mid and residual norms */
    scale = scale >> 1 as libc::c_int;
    *mid_res_amp_Q0.offset(0 as libc::c_int as isize) =
        (*mid_res_amp_Q0.offset(0 as libc::c_int as isize) as libc::c_longlong
            + ((((silk_SQRT_APPROX(nrgx) as crate::opus_types_h::opus_uint32) << scale)
                as crate::opus_types_h::opus_int32
                - *mid_res_amp_Q0.offset(0 as libc::c_int as isize))
                as libc::c_longlong
                * smooth_coef_Q16 as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32;
    /* Residual energy = nrgy - 2 * pred * corr + pred^2 * nrgx */
    nrgy = nrgy
        - (((corr as libc::c_longlong
            * pred_Q13 as crate::opus_types_h::opus_int16 as libc::c_longlong
            >> 16 as libc::c_int) as crate::opus_types_h::opus_int32
            as crate::opus_types_h::opus_uint32)
            << 3 as libc::c_int + 1 as libc::c_int) as crate::opus_types_h::opus_int32;
    nrgy = nrgy
        + (((nrgx as libc::c_longlong
            * pred2_Q10 as crate::opus_types_h::opus_int16 as libc::c_longlong
            >> 16 as libc::c_int) as crate::opus_types_h::opus_int32
            as crate::opus_types_h::opus_uint32)
            << 6 as libc::c_int) as crate::opus_types_h::opus_int32;
    *mid_res_amp_Q0.offset(1 as libc::c_int as isize) =
        (*mid_res_amp_Q0.offset(1 as libc::c_int as isize) as libc::c_longlong
            + ((((silk_SQRT_APPROX(nrgy) as crate::opus_types_h::opus_uint32) << scale)
                as crate::opus_types_h::opus_int32
                - *mid_res_amp_Q0.offset(1 as libc::c_int as isize))
                as libc::c_longlong
                * smooth_coef_Q16 as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32;
    /* Ratio of smoothed residual and mid norms */
    *ratio_Q14 = silk_DIV32_varQ(
        *mid_res_amp_Q0.offset(1 as libc::c_int as isize),
        if *mid_res_amp_Q0.offset(0 as libc::c_int as isize) > 1 as libc::c_int {
            *mid_res_amp_Q0.offset(0 as libc::c_int as isize)
        } else {
            1 as libc::c_int
        },
        14 as libc::c_int,
    );
    *ratio_Q14 = if 0 as libc::c_int > 32767 as libc::c_int {
        if *ratio_Q14 > 0 as libc::c_int {
            0 as libc::c_int
        } else if *ratio_Q14 < 32767 as libc::c_int {
            32767 as libc::c_int
        } else {
            *ratio_Q14
        }
    } else if *ratio_Q14 > 32767 as libc::c_int {
        32767 as libc::c_int
    } else if *ratio_Q14 < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        *ratio_Q14
    };
    return pred_Q13;
}

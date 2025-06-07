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

    use crate::src::opus_1_2_1::silk::stereo_LR_to_MS::macros_h::silk_CLZ32;
    /* SILK_FIX_INLINES_H */
}

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::silk::stereo_LR_to_MS::macros_h::silk_CLZ32;
pub use crate::src::opus_1_2_1::silk::stereo_LR_to_MS::Inlines_h::silk_DIV32_varQ;
pub use crate::src::opus_1_2_1::silk::stereo_LR_to_MS::SigProc_FIX_h::silk_max_int;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;

pub use crate::stdlib::uint32_t;
pub use crate::structs_h::stereo_enc_state;
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
#[no_mangle]

pub unsafe extern "C" fn silk_stereo_LR_to_MS(
    mut state: *mut crate::structs_h::stereo_enc_state,
    mut x1: *mut crate::opus_types_h::opus_int16,
    mut x2: *mut crate::opus_types_h::opus_int16,
    mut ix: *mut [libc::c_schar; 3],
    mut mid_only_flag: *mut libc::c_schar,
    mut mid_side_rates_bps: *mut crate::opus_types_h::opus_int32,
    mut total_rate_bps: crate::opus_types_h::opus_int32,
    mut prev_speech_act_Q8: libc::c_int,
    mut toMono: libc::c_int,
    mut fs_kHz: libc::c_int,
    mut frame_length: libc::c_int,
)
/* I    Number of samples                           */
{
    let mut n: libc::c_int = 0;
    let mut is10msFrame: libc::c_int = 0;
    let mut denom_Q16: libc::c_int = 0;
    let mut delta0_Q13: libc::c_int = 0;
    let mut delta1_Q13: libc::c_int = 0;
    let mut sum: crate::opus_types_h::opus_int32 = 0;
    let mut diff: crate::opus_types_h::opus_int32 = 0;
    let mut smooth_coef_Q16: crate::opus_types_h::opus_int32 = 0;
    let mut pred_Q13: [crate::opus_types_h::opus_int32; 2] = [0; 2];
    let mut pred0_Q13: crate::opus_types_h::opus_int32 = 0;
    let mut pred1_Q13: crate::opus_types_h::opus_int32 = 0;
    let mut LP_ratio_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut HP_ratio_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut frac_Q16: crate::opus_types_h::opus_int32 = 0;
    let mut frac_3_Q16: crate::opus_types_h::opus_int32 = 0;
    let mut min_mid_rate_bps: crate::opus_types_h::opus_int32 = 0;
    let mut width_Q14: crate::opus_types_h::opus_int32 = 0;
    let mut w_Q24: crate::opus_types_h::opus_int32 = 0;
    let mut deltaw_Q24: crate::opus_types_h::opus_int32 = 0;
    let mut side: *mut crate::opus_types_h::opus_int16 = 0 as *mut crate::opus_types_h::opus_int16;
    let mut LP_mid: *mut crate::opus_types_h::opus_int16 =
        0 as *mut crate::opus_types_h::opus_int16;
    let mut HP_mid: *mut crate::opus_types_h::opus_int16 =
        0 as *mut crate::opus_types_h::opus_int16;
    let mut LP_side: *mut crate::opus_types_h::opus_int16 =
        0 as *mut crate::opus_types_h::opus_int16;
    let mut HP_side: *mut crate::opus_types_h::opus_int16 =
        0 as *mut crate::opus_types_h::opus_int16;
    let mut mid: *mut crate::opus_types_h::opus_int16 =
        &mut *x1.offset(-(2 as libc::c_int) as isize) as *mut crate::opus_types_h::opus_int16;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong)
            .wrapping_mul((frame_length + 2 as libc::c_int) as libc::c_ulong) as usize,
    );
    side = fresh0.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    /* Convert to basic mid/side signals */
    n = 0 as libc::c_int;
    while n < frame_length + 2 as libc::c_int {
        sum = *x1.offset((n - 2 as libc::c_int) as isize) as libc::c_int
            + *x2.offset((n - 2 as libc::c_int) as isize) as crate::opus_types_h::opus_int32;
        diff = *x1.offset((n - 2 as libc::c_int) as isize) as libc::c_int
            - *x2.offset((n - 2 as libc::c_int) as isize) as crate::opus_types_h::opus_int32;
        *mid.offset(n as isize) = if 1 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            ((sum >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int) >> 1 as libc::c_int
        } as crate::opus_types_h::opus_int16;
        *side.offset(n as isize) = if (if 1 as libc::c_int == 1 as libc::c_int {
            (diff >> 1 as libc::c_int) + (diff & 1 as libc::c_int)
        } else {
            ((diff >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int) >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 1 as libc::c_int == 1 as libc::c_int {
            (diff >> 1 as libc::c_int) + (diff & 1 as libc::c_int)
        } else {
            ((diff >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int) >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as crate::opus_types_h::opus_int16 as libc::c_int
        {
            0x8000 as libc::c_int as crate::opus_types_h::opus_int16 as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (diff >> 1 as libc::c_int) + (diff & 1 as libc::c_int)
        } else {
            ((diff >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int) >> 1 as libc::c_int
        } as crate::opus_types_h::opus_int16;
        n += 1
    }
    /* Buffering */
    crate::stdlib::memcpy(mid as *mut libc::c_void,
           (*state).sMid.as_mut_ptr() as *const libc::c_void,
           (2 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()
                                                as libc::c_ulong));
    crate::stdlib::memcpy(side as *mut libc::c_void,
           (*state).sSide.as_mut_ptr() as *const libc::c_void,
           (2 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()
                                                as libc::c_ulong));
    crate::stdlib::memcpy((*state).sMid.as_mut_ptr() as *mut libc::c_void,
           &mut *mid.offset(frame_length as isize) as *mut crate::opus_types_h::opus_int16 as
               *const libc::c_void,
           (2 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()
                                                as libc::c_ulong));
    crate::stdlib::memcpy((*state).sSide.as_mut_ptr() as *mut libc::c_void,
           &mut *side.offset(frame_length as isize) as *mut crate::opus_types_h::opus_int16 as
               *const libc::c_void,
           (2 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()
                                                as libc::c_ulong));
    /* LP and HP filter mid signal */
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong)
            .wrapping_mul(frame_length as libc::c_ulong) as usize,
    );
    LP_mid = fresh1.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong)
            .wrapping_mul(frame_length as libc::c_ulong) as usize,
    );
    HP_mid = fresh2.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    n = 0 as libc::c_int;
    while n < frame_length {
        sum = if 2 as libc::c_int == 1 as libc::c_int {
            (*mid.offset(n as isize) as libc::c_int
                + *mid.offset((n + 2 as libc::c_int) as isize) as crate::opus_types_h::opus_int32
                + ((*mid.offset((n + 1 as libc::c_int) as isize)
                    as crate::opus_types_h::opus_uint32)
                    << 1 as libc::c_int) as crate::opus_types_h::opus_int32
                >> 1 as libc::c_int)
                + (*mid.offset(n as isize) as libc::c_int
                    + *mid.offset((n + 2 as libc::c_int) as isize)
                        as crate::opus_types_h::opus_int32
                    + ((*mid.offset((n + 1 as libc::c_int) as isize)
                        as crate::opus_types_h::opus_uint32)
                        << 1 as libc::c_int)
                        as crate::opus_types_h::opus_int32
                    & 1 as libc::c_int)
        } else {
            ((*mid.offset(n as isize) as libc::c_int
                + *mid.offset((n + 2 as libc::c_int) as isize) as crate::opus_types_h::opus_int32
                + ((*mid.offset((n + 1 as libc::c_int) as isize)
                    as crate::opus_types_h::opus_uint32)
                    << 1 as libc::c_int) as crate::opus_types_h::opus_int32
                >> 2 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int)
                >> 1 as libc::c_int
        };
        *LP_mid.offset(n as isize) = sum as crate::opus_types_h::opus_int16;
        *HP_mid.offset(n as isize) = (*mid.offset((n + 1 as libc::c_int) as isize) as libc::c_int
            - sum) as crate::opus_types_h::opus_int16;
        n += 1
    }
    /* LP and HP filter side signal */
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong)
            .wrapping_mul(frame_length as libc::c_ulong) as usize,
    );
    LP_side = fresh3.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong)
            .wrapping_mul(frame_length as libc::c_ulong) as usize,
    );
    HP_side = fresh4.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    n = 0 as libc::c_int;
    while n < frame_length {
        sum = if 2 as libc::c_int == 1 as libc::c_int {
            (*side.offset(n as isize) as libc::c_int
                + *side.offset((n + 2 as libc::c_int) as isize) as crate::opus_types_h::opus_int32
                + ((*side.offset((n + 1 as libc::c_int) as isize)
                    as crate::opus_types_h::opus_uint32)
                    << 1 as libc::c_int) as crate::opus_types_h::opus_int32
                >> 1 as libc::c_int)
                + (*side.offset(n as isize) as libc::c_int
                    + *side.offset((n + 2 as libc::c_int) as isize)
                        as crate::opus_types_h::opus_int32
                    + ((*side.offset((n + 1 as libc::c_int) as isize)
                        as crate::opus_types_h::opus_uint32)
                        << 1 as libc::c_int)
                        as crate::opus_types_h::opus_int32
                    & 1 as libc::c_int)
        } else {
            ((*side.offset(n as isize) as libc::c_int
                + *side.offset((n + 2 as libc::c_int) as isize) as crate::opus_types_h::opus_int32
                + ((*side.offset((n + 1 as libc::c_int) as isize)
                    as crate::opus_types_h::opus_uint32)
                    << 1 as libc::c_int) as crate::opus_types_h::opus_int32
                >> 2 as libc::c_int - 1 as libc::c_int)
                + 1 as libc::c_int)
                >> 1 as libc::c_int
        };
        *LP_side.offset(n as isize) = sum as crate::opus_types_h::opus_int16;
        *HP_side.offset(n as isize) = (*side.offset((n + 1 as libc::c_int) as isize) as libc::c_int
            - sum) as crate::opus_types_h::opus_int16;
        n += 1
    }
    /* Find energies and predictors */
    is10msFrame = (frame_length == 10 as libc::c_int * fs_kHz) as libc::c_int;
    smooth_coef_Q16 = if is10msFrame != 0 {
        (0.01f64 / 2 as libc::c_int as libc::c_double
            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
    } else {
        (0.01f64 * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
    };
    smooth_coef_Q16 = ((prev_speech_act_Q8 as crate::opus_types_h::opus_int16
        as crate::opus_types_h::opus_int32
        * prev_speech_act_Q8 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32)
        as libc::c_longlong
        * smooth_coef_Q16 as crate::opus_types_h::opus_int16 as libc::c_longlong
        >> 16 as libc::c_int) as crate::opus_types_h::opus_int32;
    pred_Q13[0 as libc::c_int as usize] =
        crate::src::opus_1_2_1::silk::stereo_find_predictor::silk_stereo_find_predictor(
            &mut LP_ratio_Q14,
            LP_mid as *const crate::opus_types_h::opus_int16,
            LP_side as *const crate::opus_types_h::opus_int16,
            &mut *(*state)
                .mid_side_amp_Q0
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            frame_length,
            smooth_coef_Q16,
        );
    pred_Q13[1 as libc::c_int as usize] =
        crate::src::opus_1_2_1::silk::stereo_find_predictor::silk_stereo_find_predictor(
            &mut HP_ratio_Q14,
            HP_mid as *const crate::opus_types_h::opus_int16,
            HP_side as *const crate::opus_types_h::opus_int16,
            &mut *(*state)
                .mid_side_amp_Q0
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize),
            frame_length,
            smooth_coef_Q16,
        );
    /* Ratio of the norms of residual and mid signals */
    frac_Q16 = HP_ratio_Q14
        + LP_ratio_Q14 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            * 3 as libc::c_int as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32;
    frac_Q16 = if frac_Q16
        < ((1 as libc::c_int as libc::c_longlong
            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int))
            as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
    {
        frac_Q16
    } else {
        ((1 as libc::c_int as libc::c_longlong
            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int))
            as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
    };
    /* Determine bitrate distribution between mid and side, and possibly reduce stereo width */
    total_rate_bps -= if is10msFrame != 0 {
        1200 as libc::c_int
    } else {
        600 as libc::c_int
    }; /* Subtract approximate bitrate for coding stereo parameters */
    if total_rate_bps < 1 as libc::c_int {
        total_rate_bps = 1 as libc::c_int
    }
    min_mid_rate_bps = 2000 as libc::c_int
        + fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            * 900 as libc::c_int as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32;
    /* Default bitrate distribution: 8 parts for Mid and (5+3*frac) parts for Side. so: mid_rate = ( 8 / ( 13 + 3 * frac ) ) * total_ rate */
    frac_3_Q16 = 3 as libc::c_int * frac_Q16;
    *mid_side_rates_bps.offset(0 as libc::c_int as isize) = silk_DIV32_varQ(
        total_rate_bps,
        (((8 as libc::c_int + 5 as libc::c_int) as libc::c_longlong
            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int))
            as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
            + frac_3_Q16,
        16 as libc::c_int + 3 as libc::c_int,
    );
    /* If Mid bitrate below minimum, reduce stereo width */
    if *mid_side_rates_bps.offset(0 as libc::c_int as isize) < min_mid_rate_bps {
        *mid_side_rates_bps.offset(0 as libc::c_int as isize) = min_mid_rate_bps;
        *mid_side_rates_bps.offset(1 as libc::c_int as isize) =
            total_rate_bps - *mid_side_rates_bps.offset(0 as libc::c_int as isize);
        /* width = 4 * ( 2 * side_rate - min_rate ) / ( ( 1 + 3 * frac ) * min_rate ) */
        width_Q14 = silk_DIV32_varQ(
            ((*mid_side_rates_bps.offset(1 as libc::c_int as isize)
                as crate::opus_types_h::opus_uint32)
                << 1 as libc::c_int) as crate::opus_types_h::opus_int32
                - min_mid_rate_bps,
            ((((1 as libc::c_int as libc::c_longlong
                * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int))
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
                + frac_3_Q16) as libc::c_longlong
                * min_mid_rate_bps as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int) as crate::opus_types_h::opus_int32,
            14 as libc::c_int + 2 as libc::c_int,
        );
        width_Q14 = if 0 as libc::c_int
            > ((1 as libc::c_int as libc::c_longlong
                * ((1 as libc::c_int as libc::c_longlong) << 14 as libc::c_int))
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
        {
            if width_Q14 > 0 as libc::c_int {
                0 as libc::c_int
            } else if width_Q14
                < ((1 as libc::c_int as libc::c_longlong
                    * ((1 as libc::c_int as libc::c_longlong) << 14 as libc::c_int))
                    as libc::c_double
                    + 0.5f64) as crate::opus_types_h::opus_int32
            {
                ((1 as libc::c_int as libc::c_longlong
                    * ((1 as libc::c_int as libc::c_longlong) << 14 as libc::c_int))
                    as libc::c_double
                    + 0.5f64) as crate::opus_types_h::opus_int32
            } else {
                width_Q14
            }
        } else if width_Q14
            > ((1 as libc::c_int as libc::c_longlong
                * ((1 as libc::c_int as libc::c_longlong) << 14 as libc::c_int))
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
        {
            ((1 as libc::c_int as libc::c_longlong
                * ((1 as libc::c_int as libc::c_longlong) << 14 as libc::c_int))
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
        } else if width_Q14 < 0 as libc::c_int {
            0 as libc::c_int
        } else {
            width_Q14
        }
    } else {
        *mid_side_rates_bps.offset(1 as libc::c_int as isize) =
            total_rate_bps - *mid_side_rates_bps.offset(0 as libc::c_int as isize);
        width_Q14 = ((1 as libc::c_int as libc::c_longlong
            * ((1 as libc::c_int as libc::c_longlong) << 14 as libc::c_int))
            as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
    }
    /* Smoother */
    (*state).smth_width_Q14 = ((*state).smth_width_Q14 as libc::c_longlong
        + ((width_Q14 - (*state).smth_width_Q14 as libc::c_int) as libc::c_longlong
            * smooth_coef_Q16 as crate::opus_types_h::opus_int16 as libc::c_longlong
            >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32
        as crate::opus_types_h::opus_int16;
    /* At very low bitrates or for inputs that are nearly amplitude panned, switch to panned-mono coding */
    *mid_only_flag = 0 as libc::c_int as libc::c_schar;
    if toMono != 0 {
        /* Last frame before stereo->mono transition; collapse stereo width */
        width_Q14 = 0 as libc::c_int;
        pred_Q13[0 as libc::c_int as usize] = 0 as libc::c_int;
        pred_Q13[1 as libc::c_int as usize] = 0 as libc::c_int;
        crate::src::opus_1_2_1::silk::stereo_quant_pred::silk_stereo_quant_pred(
            pred_Q13.as_mut_ptr(),
            ix,
        );
    } else if (*state).width_prev_Q14 as libc::c_int == 0 as libc::c_int
        && (8 as libc::c_int * total_rate_bps < 13 as libc::c_int * min_mid_rate_bps
            || ((frac_Q16 as libc::c_longlong * (*state).smth_width_Q14 as libc::c_longlong
                >> 16 as libc::c_int) as crate::opus_types_h::opus_int32)
                < (0.05f64
                    * ((1 as libc::c_int as libc::c_longlong) << 14 as libc::c_int)
                        as libc::c_double
                    + 0.5f64) as crate::opus_types_h::opus_int32)
    {
        /* Code as panned-mono; previous frame already had zero width */
        /* Scale down and quantize predictors */
        pred_Q13[0 as libc::c_int as usize] = (*state).smth_width_Q14
            as crate::opus_types_h::opus_int32
            * pred_Q13[0 as libc::c_int as usize] as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
            >> 14 as libc::c_int;
        pred_Q13[1 as libc::c_int as usize] = (*state).smth_width_Q14
            as crate::opus_types_h::opus_int32
            * pred_Q13[1 as libc::c_int as usize] as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
            >> 14 as libc::c_int;
        crate::src::opus_1_2_1::silk::stereo_quant_pred::silk_stereo_quant_pred(
            pred_Q13.as_mut_ptr(),
            ix,
        );
        /* Collapse stereo width */
        width_Q14 = 0 as libc::c_int;
        pred_Q13[0 as libc::c_int as usize] = 0 as libc::c_int;
        pred_Q13[1 as libc::c_int as usize] = 0 as libc::c_int;
        *mid_side_rates_bps.offset(0 as libc::c_int as isize) = total_rate_bps;
        *mid_side_rates_bps.offset(1 as libc::c_int as isize) = 0 as libc::c_int;
        *mid_only_flag = 1 as libc::c_int as libc::c_schar
    } else if (*state).width_prev_Q14 as libc::c_int != 0 as libc::c_int
        && (8 as libc::c_int * total_rate_bps < 11 as libc::c_int * min_mid_rate_bps
            || ((frac_Q16 as libc::c_longlong * (*state).smth_width_Q14 as libc::c_longlong
                >> 16 as libc::c_int) as crate::opus_types_h::opus_int32)
                < (0.02f64
                    * ((1 as libc::c_int as libc::c_longlong) << 14 as libc::c_int)
                        as libc::c_double
                    + 0.5f64) as crate::opus_types_h::opus_int32)
    {
        /* Transition to zero-width stereo */
        /* Scale down and quantize predictors */
        pred_Q13[0 as libc::c_int as usize] = (*state).smth_width_Q14
            as crate::opus_types_h::opus_int32
            * pred_Q13[0 as libc::c_int as usize] as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
            >> 14 as libc::c_int;
        pred_Q13[1 as libc::c_int as usize] = (*state).smth_width_Q14
            as crate::opus_types_h::opus_int32
            * pred_Q13[1 as libc::c_int as usize] as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
            >> 14 as libc::c_int;
        crate::src::opus_1_2_1::silk::stereo_quant_pred::silk_stereo_quant_pred(
            pred_Q13.as_mut_ptr(),
            ix,
        );
        /* Collapse stereo width */
        width_Q14 = 0 as libc::c_int;
        pred_Q13[0 as libc::c_int as usize] = 0 as libc::c_int;
        pred_Q13[1 as libc::c_int as usize] = 0 as libc::c_int
    } else if (*state).smth_width_Q14 as libc::c_int
        > (0.95f64
            * ((1 as libc::c_int as libc::c_longlong) << 14 as libc::c_int) as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
    {
        /* Full-width stereo coding */
        crate::src::opus_1_2_1::silk::stereo_quant_pred::silk_stereo_quant_pred(
            pred_Q13.as_mut_ptr(),
            ix,
        );
        width_Q14 = ((1 as libc::c_int as libc::c_longlong
            * ((1 as libc::c_int as libc::c_longlong) << 14 as libc::c_int))
            as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
    } else {
        /* Reduced-width stereo coding; scale down and quantize predictors */
        pred_Q13[0 as libc::c_int as usize] = (*state).smth_width_Q14
            as crate::opus_types_h::opus_int32
            * pred_Q13[0 as libc::c_int as usize] as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
            >> 14 as libc::c_int;
        pred_Q13[1 as libc::c_int as usize] = (*state).smth_width_Q14
            as crate::opus_types_h::opus_int32
            * pred_Q13[1 as libc::c_int as usize] as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
            >> 14 as libc::c_int;
        crate::src::opus_1_2_1::silk::stereo_quant_pred::silk_stereo_quant_pred(
            pred_Q13.as_mut_ptr(),
            ix,
        );
        width_Q14 = (*state).smth_width_Q14 as crate::opus_types_h::opus_int32
    }
    /* Make sure to keep on encoding until the tapered output has been transmitted */
    if *mid_only_flag as libc::c_int == 1 as libc::c_int {
        (*state).silent_side_len = ((*state).silent_side_len as libc::c_int
            + (frame_length - 8 as libc::c_int * fs_kHz))
            as crate::opus_types_h::opus_int16;
        if ((*state).silent_side_len as libc::c_int) < 5 as libc::c_int * fs_kHz {
            *mid_only_flag = 0 as libc::c_int as libc::c_schar
        } else {
            /* Limit to avoid wrapping around */
            (*state).silent_side_len = 10000 as libc::c_int as crate::opus_types_h::opus_int16
        }
    } else {
        (*state).silent_side_len = 0 as libc::c_int as crate::opus_types_h::opus_int16
    }
    if *mid_only_flag as libc::c_int == 0 as libc::c_int
        && *mid_side_rates_bps.offset(1 as libc::c_int as isize) < 1 as libc::c_int
    {
        *mid_side_rates_bps.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
        *mid_side_rates_bps.offset(0 as libc::c_int as isize) = silk_max_int(
            1 as libc::c_int,
            total_rate_bps - *mid_side_rates_bps.offset(1 as libc::c_int as isize),
        )
    }
    /* Interpolate predictors and subtract prediction from side channel */
    pred0_Q13 = -((*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int); /* Q11 */
    pred1_Q13 = -((*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int); /* Q8  */
    w_Q24 = (((*state).width_prev_Q14 as crate::opus_types_h::opus_uint32) << 10 as libc::c_int)
        as crate::opus_types_h::opus_int32; /* Q8  */
    denom_Q16 = ((1 as libc::c_int) << 16 as libc::c_int) / (8 as libc::c_int * fs_kHz); /* Q11 */
    delta0_Q13 = -if 16 as libc::c_int == 1 as libc::c_int {
        ((pred_Q13[0 as libc::c_int as usize]
            - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int)
            as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            * denom_Q16 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            >> 1 as libc::c_int)
            + ((pred_Q13[0 as libc::c_int as usize]
                - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int)
                as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * denom_Q16 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                & 1 as libc::c_int)
    } else {
        (((pred_Q13[0 as libc::c_int as usize]
            - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int)
            as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            * denom_Q16 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            >> 16 as libc::c_int - 1 as libc::c_int)
            + 1 as libc::c_int)
            >> 1 as libc::c_int
    }; /* Q8  */
    delta1_Q13 = -if 16 as libc::c_int == 1 as libc::c_int {
        ((pred_Q13[1 as libc::c_int as usize]
            - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int)
            as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            * denom_Q16 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            >> 1 as libc::c_int)
            + ((pred_Q13[1 as libc::c_int as usize]
                - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int)
                as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * denom_Q16 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                & 1 as libc::c_int)
    } else {
        (((pred_Q13[1 as libc::c_int as usize]
            - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int)
            as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            * denom_Q16 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            >> 16 as libc::c_int - 1 as libc::c_int)
            + 1 as libc::c_int)
            >> 1 as libc::c_int
    }; /* Q8  */
    deltaw_Q24 = ((((width_Q14 - (*state).width_prev_Q14 as libc::c_int) as libc::c_longlong
        * denom_Q16 as crate::opus_types_h::opus_int16 as libc::c_longlong
        >> 16 as libc::c_int) as crate::opus_types_h::opus_int32
        as crate::opus_types_h::opus_uint32)
        << 10 as libc::c_int) as crate::opus_types_h::opus_int32;
    n = 0 as libc::c_int;
    while n < 8 as libc::c_int * fs_kHz {
        pred0_Q13 += delta0_Q13;
        pred1_Q13 += delta1_Q13;
        w_Q24 += deltaw_Q24;
        sum = (((*mid.offset(n as isize) as libc::c_int
            + *mid.offset((n + 2 as libc::c_int) as isize) as crate::opus_types_h::opus_int32
            + ((*mid.offset((n + 1 as libc::c_int) as isize) as crate::opus_types_h::opus_uint32)
                << 1 as libc::c_int) as crate::opus_types_h::opus_int32)
            as crate::opus_types_h::opus_uint32)
            << 9 as libc::c_int) as crate::opus_types_h::opus_int32;
        sum = ((w_Q24 as libc::c_longlong
            * *side.offset((n + 1 as libc::c_int) as isize) as libc::c_longlong
            >> 16 as libc::c_int) as crate::opus_types_h::opus_int32
            as libc::c_longlong
            + (sum as libc::c_longlong
                * pred0_Q13 as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32;
        sum = (sum as libc::c_longlong
            + (((*mid.offset((n + 1 as libc::c_int) as isize) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_uint32)
                << 11 as libc::c_int) as crate::opus_types_h::opus_int32
                as libc::c_longlong
                * pred1_Q13 as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32;
        *x2.offset((n - 1 as libc::c_int) as isize) = if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            ((sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int) >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            ((sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int) >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as crate::opus_types_h::opus_int16 as libc::c_int
        {
            0x8000 as libc::c_int as crate::opus_types_h::opus_int16 as libc::c_int
        } else if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            ((sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int) >> 1 as libc::c_int
        } as crate::opus_types_h::opus_int16;
        n += 1
    }
    pred0_Q13 = -pred_Q13[0 as libc::c_int as usize];
    pred1_Q13 = -pred_Q13[1 as libc::c_int as usize];
    w_Q24 = ((width_Q14 as crate::opus_types_h::opus_uint32) << 10 as libc::c_int)
        as crate::opus_types_h::opus_int32;
    n = 8 as libc::c_int * fs_kHz;
    while n < frame_length {
        sum = (((*mid.offset(n as isize) as libc::c_int
            + *mid.offset((n + 2 as libc::c_int) as isize) as crate::opus_types_h::opus_int32
            + ((*mid.offset((n + 1 as libc::c_int) as isize) as crate::opus_types_h::opus_uint32)
                << 1 as libc::c_int) as crate::opus_types_h::opus_int32)
            as crate::opus_types_h::opus_uint32)
            << 9 as libc::c_int) as crate::opus_types_h::opus_int32;
        sum = ((w_Q24 as libc::c_longlong
            * *side.offset((n + 1 as libc::c_int) as isize) as libc::c_longlong
            >> 16 as libc::c_int) as crate::opus_types_h::opus_int32
            as libc::c_longlong
            + (sum as libc::c_longlong
                * pred0_Q13 as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32;
        sum = (sum as libc::c_longlong
            + (((*mid.offset((n + 1 as libc::c_int) as isize) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_uint32)
                << 11 as libc::c_int) as crate::opus_types_h::opus_int32
                as libc::c_longlong
                * pred1_Q13 as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32;
        *x2.offset((n - 1 as libc::c_int) as isize) = if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            ((sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int) >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            ((sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int) >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as crate::opus_types_h::opus_int16 as libc::c_int
        {
            0x8000 as libc::c_int as crate::opus_types_h::opus_int16 as libc::c_int
        } else if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            ((sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int) >> 1 as libc::c_int
        } as crate::opus_types_h::opus_int16;
        n += 1
    }
    (*state).pred_prev_Q13[0 as libc::c_int as usize] =
        pred_Q13[0 as libc::c_int as usize] as crate::opus_types_h::opus_int16;
    (*state).pred_prev_Q13[1 as libc::c_int as usize] =
        pred_Q13[1 as libc::c_int as usize] as crate::opus_types_h::opus_int16;
    (*state).width_prev_Q14 = width_Q14 as crate::opus_types_h::opus_int16;
}

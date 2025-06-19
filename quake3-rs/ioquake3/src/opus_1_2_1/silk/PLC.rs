use ::libc;

pub mod macros_h {
    #[inline]

    pub unsafe extern "C" fn silk_CLZ32(
        mut in32: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        return if in32 != 0 {
            (32 as i32)
                - (::std::mem::size_of::<u32>() as libc::c_ulong as i32 * 8 as i32
                    - (in32 as u32).leading_zeros() as i32)
        } else {
            32 as i32
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
        mut rot: i32,
    ) -> crate::opus_types_h::opus_int32 {
        let mut x: crate::opus_types_h::opus_uint32 = a32 as crate::opus_types_h::opus_uint32;
        let mut r: crate::opus_types_h::opus_uint32 = rot as crate::opus_types_h::opus_uint32;
        let mut m: crate::opus_types_h::opus_uint32 = -rot as crate::opus_types_h::opus_uint32;
        if rot == 0 as i32 {
            return a32;
        } else if rot < 0 as i32 {
            return (x << m | x >> (32 as i32 as u32).wrapping_sub(m))
                as crate::opus_types_h::opus_int32;
        } else {
            return (x << (32 as i32 as u32).wrapping_sub(r) | x >> r)
                as crate::opus_types_h::opus_int32;
        };
    }
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
    #[inline]

    pub unsafe extern "C" fn silk_max_16(
        mut a: crate::opus_types_h::opus_int16,
        mut b: crate::opus_types_h::opus_int16,
    ) -> crate::opus_types_h::opus_int16 {
        return if a as i32 > b as i32 {
            a as i32
        } else {
            b as i32
        } as crate::opus_types_h::opus_int16;
    }
    #[inline]

    pub unsafe extern "C" fn silk_max_32(
        mut a: crate::opus_types_h::opus_int32,
        mut b: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
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
        *frac_Q7 = silk_ROR32(in_0, 24 as i32 - lzeros) & 0x7f as i32;
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
        if x <= 0 as i32 {
            return 0 as i32;
        }
        silk_CLZ_FRAC(x, &mut lz, &mut frac_Q7);
        if lz & 1 as i32 != 0 {
            y = 32768 as i32
        } else {
            y = 46214 as i32
            /* 46214 = sqrt(2) * 32768 */
        }
        /* get scaling right */
        y >>= lz >> 1 as i32;
        /* increment using fractional part of input */
        y = (y as i64
            + (y as i64
                * (213 as i32 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                    * frac_Q7 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32)
                    as crate::opus_types_h::opus_int16 as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32;
        return y;
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

    use crate::src::opus_1_2_1::silk::PLC::macros_h::silk_CLZ32;
    use crate::src::opus_1_2_1::silk::PLC::SigProc_FIX_h::silk_ROR32;
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
pub use crate::src::opus_1_2_1::silk::PLC::macros_h::silk_CLZ32;

pub use crate::structs_h::silk_CNG_struct;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_PLC_struct;
pub use crate::structs_h::silk_decoder_control;
pub use crate::structs_h::silk_decoder_state;
pub use crate::structs_h::SideInfoIndices;

pub use crate::src::opus_1_2_1::silk::bwexpander::silk_bwexpander;
pub use crate::src::opus_1_2_1::silk::sum_sqr_shift::silk_sum_sqr_shift;
pub use crate::src::opus_1_2_1::silk::LPC_analysis_filter::silk_LPC_analysis_filter;
pub use crate::src::opus_1_2_1::silk::LPC_inv_pred_gain::silk_LPC_inverse_pred_gain_c;
pub use crate::src::opus_1_2_1::silk::PLC::Inlines_h::silk_CLZ_FRAC;
pub use crate::src::opus_1_2_1::silk::PLC::Inlines_h::silk_INVERSE32_varQ;
pub use crate::src::opus_1_2_1::silk::PLC::Inlines_h::silk_SQRT_APPROX;
pub use crate::src::opus_1_2_1::silk::PLC::SigProc_FIX_h::silk_ROR32;
pub use crate::src::opus_1_2_1::silk::PLC::SigProc_FIX_h::silk_max_16;
pub use crate::src::opus_1_2_1::silk::PLC::SigProc_FIX_h::silk_max_32;
pub use crate::src::opus_1_2_1::silk::PLC::SigProc_FIX_h::silk_max_int;
pub use crate::src::opus_1_2_1::silk::PLC::SigProc_FIX_h::silk_min_32;
pub use crate::src::opus_1_2_1::silk::PLC::SigProc_FIX_h::silk_min_int;

static mut HARM_ATT_Q15: [opus_int16; 2] = [32440 as i32 as opus_int16, 31130 as i32 as opus_int16];
/* 0.99, 0.95 */

static mut PLC_RAND_ATTENUATE_V_Q15: [opus_int16; 2] =
    [31130 as i32 as opus_int16, 26214 as i32 as opus_int16];
/* 0.95, 0.8 */

static mut PLC_RAND_ATTENUATE_UV_Q15: [opus_int16; 2] =
    [32440 as i32 as opus_int16, 29491 as i32 as opus_int16];
#[no_mangle]

pub unsafe extern "C" fn silk_PLC_Reset(mut psDec: *mut silk_decoder_state)
/* I/O Decoder state        */
{
    (*psDec).sPLC.pitchL_Q8 =
        (((*psDec).frame_length as opus_uint32) << 8 as i32 - 1 as i32) as opus_int32;
    (*psDec).sPLC.prevGain_Q16[0 as i32 as usize] =
        ((1 as i32 as i64 * ((1 as i32 as i64) << 16 as i32)) as f64 + 0.5f64) as opus_int32;
    (*psDec).sPLC.prevGain_Q16[1 as i32 as usize] =
        ((1 as i32 as i64 * ((1 as i32 as i64) << 16 as i32)) as f64 + 0.5f64) as opus_int32;
    (*psDec).sPLC.subfr_length = 20 as i32;
    (*psDec).sPLC.nb_subfr = 2 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn silk_PLC(
    mut psDec: *mut silk_decoder_state,
    mut psDecCtrl: *mut silk_decoder_control,
    mut frame: *mut opus_int16,
    mut lost: i32,
    mut arch: i32,
)
/* I Run-time architecture  */
{
    /* PLC control function */
    if (*psDec).fs_kHz != (*psDec).sPLC.fs_kHz {
        silk_PLC_Reset(psDec);
        (*psDec).sPLC.fs_kHz = (*psDec).fs_kHz
    }
    if lost != 0 {
        /* ***************************/
        /* Generate Signal          */
        /* ***************************/
        silk_PLC_conceal(psDec, psDecCtrl, frame, arch);
        (*psDec).lossCnt += 1
    } else {
        /* ***************************/
        /* Update state             */
        /* ***************************/
        silk_PLC_update(psDec, psDecCtrl);
    };
}
/* 0.99, 0.9 */
/* *************************************************/
/* Update state of PLC                            */
/* *************************************************/
#[inline]

unsafe extern "C" fn silk_PLC_update(
    mut psDec: *mut silk_decoder_state,
    mut psDecCtrl: *mut silk_decoder_control,
)
/* I/O Decoder control      */
{
    let mut LTP_Gain_Q14: opus_int32 = 0;
    let mut temp_LTP_Gain_Q14: opus_int32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut psPLC: *mut silk_PLC_struct = 0 as *mut silk_PLC_struct;
    psPLC = &mut (*psDec).sPLC;
    /* Update parameters used in case of packet loss */
    (*psDec).prevSignalType = (*psDec).indices.signalType as i32;
    LTP_Gain_Q14 = 0 as i32;
    if (*psDec).indices.signalType as i32 == 2 as i32 {
        /* Find the parameters for the last subframe which contains a pitch pulse */
        j = 0 as i32;
        while j * (*psDec).subfr_length
            < (*psDecCtrl).pitchL[((*psDec).nb_subfr - 1 as i32) as usize]
        {
            if j == (*psDec).nb_subfr {
                break;
            }
            temp_LTP_Gain_Q14 = 0 as i32;
            i = 0 as i32;
            while i < 5 as i32 {
                temp_LTP_Gain_Q14 += (*psDecCtrl).LTPCoef_Q14
                    [(((*psDec).nb_subfr - 1 as i32 - j) * 5 as i32 + i) as usize]
                    as i32;
                i += 1
            }
            if temp_LTP_Gain_Q14 > LTP_Gain_Q14 {
                LTP_Gain_Q14 = temp_LTP_Gain_Q14;
                crate::stdlib::memcpy(
                    (*psPLC).LTPCoef_Q14.as_mut_ptr() as *mut libc::c_void,
                    &mut *(*psDecCtrl).LTPCoef_Q14.as_mut_ptr().offset(
                        (((*psDec).nb_subfr - 1 as i32 - j) as opus_int16 as opus_int32
                            * 5 as i32 as opus_int16 as opus_int32)
                            as isize,
                    ) as *mut opus_int16 as *const libc::c_void,
                    (5 as i32 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<opus_int16>() as libc::c_ulong),
                );
                (*psPLC).pitchL_Q8 = (((*psDecCtrl).pitchL
                    [((*psDec).nb_subfr - 1 as i32 - j) as usize]
                    as opus_uint32)
                    << 8 as i32) as opus_int32
            }
            j += 1
        }
        crate::stdlib::memset(
            (*psPLC).LTPCoef_Q14.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            (5 as i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<opus_int16>() as libc::c_ulong),
        );
        (*psPLC).LTPCoef_Q14[(5 as i32 / 2 as i32) as usize] = LTP_Gain_Q14 as opus_int16;
        /* Limit LT coefs */
        if LTP_Gain_Q14 < 11469 as i32 {
            let mut scale_Q10: i32 = 0;
            let mut tmp: opus_int32 = 0;
            tmp = ((11469 as i32 as opus_uint32) << 10 as i32) as opus_int32;
            scale_Q10 = tmp
                / (if LTP_Gain_Q14 > 1 as i32 {
                    LTP_Gain_Q14
                } else {
                    1 as i32
                });
            i = 0 as i32;
            while i < 5 as i32 {
                (*psPLC).LTPCoef_Q14[i as usize] = ((*psPLC).LTPCoef_Q14[i as usize] as opus_int32
                    * scale_Q10 as opus_int16 as opus_int32
                    >> 10 as i32) as opus_int16;
                i += 1
            }
        } else if LTP_Gain_Q14 > 15565 as i32 {
            let mut scale_Q14: i32 = 0;
            let mut tmp_0: opus_int32 = 0;
            tmp_0 = ((15565 as i32 as opus_uint32) << 14 as i32) as opus_int32;
            scale_Q14 = tmp_0
                / (if LTP_Gain_Q14 > 1 as i32 {
                    LTP_Gain_Q14
                } else {
                    1 as i32
                });
            i = 0 as i32;
            while i < 5 as i32 {
                (*psPLC).LTPCoef_Q14[i as usize] = ((*psPLC).LTPCoef_Q14[i as usize] as opus_int32
                    * scale_Q14 as opus_int16 as opus_int32
                    >> 14 as i32) as opus_int16;
                i += 1
            }
        }
    } else {
        (*psPLC).pitchL_Q8 = ((((*psDec).fs_kHz as opus_int16 as opus_int32
            * 18 as i32 as opus_int16 as opus_int32) as opus_uint32)
            << 8 as i32) as opus_int32;
        crate::stdlib::memset(
            (*psPLC).LTPCoef_Q14.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            (5 as i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<opus_int16>() as libc::c_ulong),
        );
    }
    /* Save LPC coeficients */
    crate::stdlib::memcpy(
        (*psPLC).prevLPC_Q12.as_mut_ptr() as *mut libc::c_void,
        (*psDecCtrl).PredCoef_Q12[1 as i32 as usize].as_mut_ptr() as *const libc::c_void,
        ((*psDec).LPC_order as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
    (*psPLC).prevLTP_scale_Q14 = (*psDecCtrl).LTP_scale_Q14 as opus_int16;
    /* Save last two gains */
    crate::stdlib::memcpy(
        (*psPLC).prevGain_Q16.as_mut_ptr() as *mut libc::c_void,
        &mut *(*psDecCtrl)
            .Gains_Q16
            .as_mut_ptr()
            .offset(((*psDec).nb_subfr - 2 as i32) as isize) as *mut opus_int32
            as *const libc::c_void,
        (2 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<opus_int32>() as libc::c_ulong),
    );
    (*psPLC).subfr_length = (*psDec).subfr_length;
    (*psPLC).nb_subfr = (*psDec).nb_subfr;
}
#[inline]

unsafe extern "C" fn silk_PLC_energy(
    mut energy1: *mut opus_int32,
    mut shift1: *mut i32,
    mut energy2: *mut opus_int32,
    mut shift2: *mut i32,
    mut exc_Q14: *const opus_int32,
    mut prevGain_Q10: *const opus_int32,
    mut subfr_length: i32,
    mut nb_subfr: i32,
) {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut exc_buf: *mut opus_int16 = 0 as *mut opus_int16;
    let mut exc_buf_ptr: *mut opus_int16 = 0 as *mut opus_int16;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_int16>() as libc::c_ulong)
            .wrapping_mul((2 as i32 * subfr_length) as libc::c_ulong) as usize,
    );
    exc_buf = fresh0.as_mut_ptr() as *mut opus_int16;
    /* Find random noise component */
    /* Scale previous excitation signal */
    exc_buf_ptr = exc_buf;
    k = 0 as i32;
    while k < 2 as i32 {
        i = 0 as i32;
        while i < subfr_length {
            *exc_buf_ptr.offset(i as isize) = if (*exc_Q14
                .offset((i + (k + nb_subfr - 2 as i32) * subfr_length) as isize)
                as i64
                * *prevGain_Q10.offset(k as isize) as i64
                >> 16 as i32) as opus_int32
                >> 8 as i32
                > 0x7fff as i32
            {
                0x7fff as i32
            } else if ((*exc_Q14.offset((i + (k + nb_subfr - 2 as i32) * subfr_length) as isize)
                as i64
                * *prevGain_Q10.offset(k as isize) as i64
                >> 16 as i32) as opus_int32
                >> 8 as i32)
                < 0x8000 as i32 as opus_int16 as i32
            {
                0x8000 as i32 as opus_int16 as i32
            } else {
                ((*exc_Q14.offset((i + (k + nb_subfr - 2 as i32) * subfr_length) as isize) as i64
                    * *prevGain_Q10.offset(k as isize) as i64
                    >> 16 as i32) as opus_int32)
                    >> 8 as i32
            } as opus_int16;
            i += 1
        }
        exc_buf_ptr = exc_buf_ptr.offset(subfr_length as isize);
        k += 1
    }
    /* Find the subframe with lowest energy of the last two and use that as random noise generator */
    silk_sum_sqr_shift(energy1, shift1, exc_buf, subfr_length);
    silk_sum_sqr_shift(
        energy2,
        shift2,
        &mut *exc_buf.offset(subfr_length as isize),
        subfr_length,
    );
}
#[inline]

unsafe extern "C" fn silk_PLC_conceal(
    mut psDec: *mut silk_decoder_state,
    mut psDecCtrl: *mut silk_decoder_control,
    mut frame: *mut opus_int16,
    mut arch: i32,
)
/* I Run-time architecture  */
{
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut lag: i32 = 0;
    let mut idx: i32 = 0;
    let mut sLTP_buf_idx: i32 = 0;
    let mut shift1: i32 = 0;
    let mut shift2: i32 = 0;
    let mut rand_seed: opus_int32 = 0;
    let mut harm_Gain_Q15: opus_int32 = 0;
    let mut rand_Gain_Q15: opus_int32 = 0;
    let mut inv_gain_Q30: opus_int32 = 0;
    let mut energy1: opus_int32 = 0;
    let mut energy2: opus_int32 = 0;
    let mut rand_ptr: *mut opus_int32 = 0 as *mut opus_int32;
    let mut pred_lag_ptr: *mut opus_int32 = 0 as *mut opus_int32;
    let mut LPC_pred_Q10: opus_int32 = 0;
    let mut LTP_pred_Q12: opus_int32 = 0;
    let mut rand_scale_Q14: opus_int16 = 0;
    let mut B_Q14: *mut opus_int16 = 0 as *mut opus_int16;
    let mut sLPC_Q14_ptr: *mut opus_int32 = 0 as *mut opus_int32;
    let mut A_Q12: [opus_int16; 16] = [0; 16];
    let mut sLTP: *mut opus_int16 = 0 as *mut opus_int16;
    let mut sLTP_Q14: *mut opus_int32 = 0 as *mut opus_int32;
    let mut psPLC: *mut silk_PLC_struct = &mut (*psDec).sPLC;
    let mut prevGain_Q10: [opus_int32; 2] = [0; 2];
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_int32>() as libc::c_ulong)
            .wrapping_mul(((*psDec).ltp_mem_length + (*psDec).frame_length) as libc::c_ulong)
            as usize,
    );
    sLTP_Q14 = fresh1.as_mut_ptr() as *mut opus_int32;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_int16>() as libc::c_ulong)
            .wrapping_mul((*psDec).ltp_mem_length as libc::c_ulong) as usize,
    );
    sLTP = fresh2.as_mut_ptr() as *mut opus_int16;
    prevGain_Q10[0 as i32 as usize] = (*psPLC).prevGain_Q16[0 as i32 as usize] >> 6 as i32;
    prevGain_Q10[1 as i32 as usize] = (*psPLC).prevGain_Q16[1 as i32 as usize] >> 6 as i32;
    if (*psDec).first_frame_after_reset != 0 {
        crate::stdlib::memset(
            (*psPLC).prevLPC_Q12.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<[opus_int16; 16]>() as libc::c_ulong,
        );
    }
    silk_PLC_energy(
        &mut energy1,
        &mut shift1,
        &mut energy2,
        &mut shift2,
        (*psDec).exc_Q14.as_mut_ptr(),
        prevGain_Q10.as_mut_ptr(),
        (*psDec).subfr_length,
        (*psDec).nb_subfr,
    );
    if energy1 >> shift2 < energy2 >> shift1 {
        /* First sub-frame has lowest energy */
        rand_ptr = &mut *(*psDec).exc_Q14.as_mut_ptr().offset((silk_max_int
            as unsafe extern "C" fn(_: i32, _: i32) -> i32)(
            0 as i32,
            ((*psPLC).nb_subfr - 1 as i32) * (*psPLC).subfr_length - 128 as i32,
        ) as isize) as *mut opus_int32
    } else {
        /* Second sub-frame has lowest energy */
        rand_ptr = &mut *(*psDec).exc_Q14.as_mut_ptr().offset((silk_max_int
            as unsafe extern "C" fn(_: i32, _: i32) -> i32)(
            0 as i32,
            (*psPLC).nb_subfr * (*psPLC).subfr_length - 128 as i32,
        ) as isize) as *mut opus_int32
    }
    /* Set up Gain to random noise component */
    B_Q14 = (*psPLC).LTPCoef_Q14.as_mut_ptr();
    rand_scale_Q14 = (*psPLC).randScale_Q14;
    /* Set up attenuation gains */
    harm_Gain_Q15 =
        HARM_ATT_Q15[silk_min_int(2 as i32 - 1 as i32, (*psDec).lossCnt) as usize] as opus_int32;
    if (*psDec).prevSignalType == 2 as i32 {
        rand_Gain_Q15 = PLC_RAND_ATTENUATE_V_Q15
            [silk_min_int(2 as i32 - 1 as i32, (*psDec).lossCnt) as usize]
            as opus_int32
    } else {
        rand_Gain_Q15 = PLC_RAND_ATTENUATE_UV_Q15
            [silk_min_int(2 as i32 - 1 as i32, (*psDec).lossCnt) as usize]
            as opus_int32
    }
    /* LPC concealment. Apply BWE to previous LPC */
    silk_bwexpander(
        (*psPLC).prevLPC_Q12.as_mut_ptr(),
        (*psDec).LPC_order,
        (0.99f64 * ((1 as i32 as i64) << 16 as i32) as f64 + 0.5f64) as opus_int32,
    );
    /* Preload LPC coeficients to array on stack. Gives small performance gain */
    crate::stdlib::memcpy(
        A_Q12.as_mut_ptr() as *mut libc::c_void,
        (*psPLC).prevLPC_Q12.as_mut_ptr() as *const libc::c_void,
        ((*psDec).LPC_order as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
    /* First Lost frame */
    if (*psDec).lossCnt == 0 as i32 {
        rand_scale_Q14 = ((1 as i32) << 14 as i32) as opus_int16;
        /* Reduce random noise Gain for voiced frames */
        if (*psDec).prevSignalType == 2 as i32 {
            i = 0 as i32; /* 0.2 */
            while i < 5 as i32 {
                rand_scale_Q14 =
                    (rand_scale_Q14 as i32 - *B_Q14.offset(i as isize) as i32) as opus_int16;
                i += 1
            }
            rand_scale_Q14 = silk_max_16(3277 as i32 as opus_int16, rand_scale_Q14);
            rand_scale_Q14 = (rand_scale_Q14 as opus_int32
                * (*psPLC).prevLTP_scale_Q14 as opus_int32
                >> 14 as i32) as opus_int16
        } else {
            /* Reduce random noise for unvoiced frames with high LPC gain */
            let mut invGain_Q30: opus_int32 = 0;
            let mut down_scale_Q30: opus_int32 = 0;
            invGain_Q30 =
                silk_LPC_inverse_pred_gain_c((*psPLC).prevLPC_Q12.as_mut_ptr(), (*psDec).LPC_order);
            down_scale_Q30 = silk_min_32((1 as i32) << 30 as i32 >> 3 as i32, invGain_Q30);
            down_scale_Q30 = silk_max_32((1 as i32) << 30 as i32 >> 8 as i32, down_scale_Q30);
            down_scale_Q30 = ((down_scale_Q30 as opus_uint32) << 3 as i32) as opus_int32;
            rand_Gain_Q15 = (down_scale_Q30 as i64 * rand_Gain_Q15 as opus_int16 as i64
                >> 16 as i32) as opus_int32
                >> 14 as i32
        }
    }
    rand_seed = (*psPLC).rand_seed;
    lag = if 8 as i32 == 1 as i32 {
        ((*psPLC).pitchL_Q8 >> 1 as i32) + ((*psPLC).pitchL_Q8 & 1 as i32)
    } else {
        (((*psPLC).pitchL_Q8 >> 8 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
    };
    sLTP_buf_idx = (*psDec).ltp_mem_length;
    /* Rewhiten LTP state */
    idx = (*psDec).ltp_mem_length - lag - (*psDec).LPC_order - 5 as i32 / 2 as i32;
    silk_LPC_analysis_filter(
        &mut *sLTP.offset(idx as isize),
        &mut *(*psDec).outBuf.as_mut_ptr().offset(idx as isize),
        A_Q12.as_mut_ptr(),
        (*psDec).ltp_mem_length - idx,
        (*psDec).LPC_order,
        arch,
    );
    /* Scale LTP state */
    inv_gain_Q30 = silk_INVERSE32_varQ((*psPLC).prevGain_Q16[1 as i32 as usize], 46 as i32);
    inv_gain_Q30 = if inv_gain_Q30 < 0x7fffffff as i32 >> 1 as i32 {
        inv_gain_Q30
    } else {
        (0x7fffffff as i32) >> 1 as i32
    };
    i = idx + (*psDec).LPC_order;
    while i < (*psDec).ltp_mem_length {
        *sLTP_Q14.offset(i as isize) =
            (inv_gain_Q30 as i64 * *sLTP.offset(i as isize) as i64 >> 16 as i32) as opus_int32;
        i += 1
    }
    /* **************************/
    /* LTP synthesis filtering */
    /* **************************/
    k = 0 as i32;
    while k < (*psDec).nb_subfr {
        /* Set up pointer */
        pred_lag_ptr = &mut *sLTP_Q14.offset((sLTP_buf_idx - lag + 5 as i32 / 2 as i32) as isize)
            as *mut opus_int32;
        i = 0 as i32;
        while i < (*psDec).subfr_length {
            /* Unrolled loop */
            /* Avoids introducing a bias because silk_SMLAWB() always rounds to -inf */
            LTP_pred_Q12 = 2 as i32;
            LTP_pred_Q12 = (LTP_pred_Q12 as i64
                + (*pred_lag_ptr.offset(0 as i32 as isize) as i64
                    * *B_Q14.offset(0 as i32 as isize) as i64
                    >> 16 as i32)) as opus_int32;
            LTP_pred_Q12 = (LTP_pred_Q12 as i64
                + (*pred_lag_ptr.offset(-(1 as i32) as isize) as i64
                    * *B_Q14.offset(1 as i32 as isize) as i64
                    >> 16 as i32)) as opus_int32;
            LTP_pred_Q12 = (LTP_pred_Q12 as i64
                + (*pred_lag_ptr.offset(-(2 as i32) as isize) as i64
                    * *B_Q14.offset(2 as i32 as isize) as i64
                    >> 16 as i32)) as opus_int32;
            LTP_pred_Q12 = (LTP_pred_Q12 as i64
                + (*pred_lag_ptr.offset(-(3 as i32) as isize) as i64
                    * *B_Q14.offset(3 as i32 as isize) as i64
                    >> 16 as i32)) as opus_int32;
            LTP_pred_Q12 = (LTP_pred_Q12 as i64
                + (*pred_lag_ptr.offset(-(4 as i32) as isize) as i64
                    * *B_Q14.offset(4 as i32 as isize) as i64
                    >> 16 as i32)) as opus_int32;
            pred_lag_ptr = pred_lag_ptr.offset(1);
            /* Generate LPC excitation */
            rand_seed = (907633515 as i32 as opus_uint32).wrapping_add(
                (rand_seed as opus_uint32).wrapping_mul(196314165 as i32 as opus_uint32),
            ) as opus_int32;
            idx = rand_seed >> 25 as i32 & 128 as i32 - 1 as i32;
            *sLTP_Q14.offset(sLTP_buf_idx as isize) = (((LTP_pred_Q12 as i64
                + (*rand_ptr.offset(idx as isize) as i64 * rand_scale_Q14 as i64 >> 16 as i32))
                as opus_int32
                as opus_uint32)
                << 2 as i32) as opus_int32;
            sLTP_buf_idx += 1;
            i += 1
        }
        /* Gradually reduce LTP gain */
        j = 0 as i32;
        while j < 5 as i32 {
            *B_Q14.offset(j as isize) = (harm_Gain_Q15 as opus_int16 as opus_int32
                * *B_Q14.offset(j as isize) as opus_int32
                >> 15 as i32) as opus_int16;
            j += 1
        }
        if (*psDec).indices.signalType as i32 != 0 as i32 {
            /* Gradually reduce excitation gain */
            rand_scale_Q14 = (rand_scale_Q14 as opus_int32
                * rand_Gain_Q15 as opus_int16 as opus_int32
                >> 15 as i32) as opus_int16
        }
        /* Slowly increase pitch lag */
        (*psPLC).pitchL_Q8 = ((*psPLC).pitchL_Q8 as i64
            + ((*psPLC).pitchL_Q8 as i64 * 655 as i32 as opus_int16 as i64 >> 16 as i32))
            as opus_int32;
        (*psPLC).pitchL_Q8 = silk_min_32(
            (*psPLC).pitchL_Q8,
            (((18 as i32 as opus_int16 as opus_int32 * (*psDec).fs_kHz as opus_int16 as opus_int32)
                as opus_uint32)
                << 8 as i32) as opus_int32,
        );
        lag = if 8 as i32 == 1 as i32 {
            ((*psPLC).pitchL_Q8 >> 1 as i32) + ((*psPLC).pitchL_Q8 & 1 as i32)
        } else {
            (((*psPLC).pitchL_Q8 >> 8 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        };
        k += 1
    }
    /* **************************/
    /* LPC synthesis filtering */
    /* **************************/
    sLPC_Q14_ptr =
        &mut *sLTP_Q14.offset(((*psDec).ltp_mem_length - 16 as i32) as isize) as *mut opus_int32;
    /* Copy LPC state */
    crate::stdlib::memcpy(
        sLPC_Q14_ptr as *mut libc::c_void,
        (*psDec).sLPC_Q14_buf.as_mut_ptr() as *const libc::c_void,
        (16 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<opus_int32>() as libc::c_ulong),
    );
    /* check that unrolling works */
    i = 0 as i32;
    while i < (*psDec).frame_length {
        /* partly unrolled */
        /* Avoids introducing a bias because silk_SMLAWB() always rounds to -inf */
        LPC_pred_Q10 = (*psDec).LPC_order >> 1 as i32;
        LPC_pred_Q10 = (LPC_pred_Q10 as i64
            + (*sLPC_Q14_ptr.offset((16 as i32 + i - 1 as i32) as isize) as i64
                * A_Q12[0 as i32 as usize] as i64
                >> 16 as i32)) as opus_int32;
        LPC_pred_Q10 = (LPC_pred_Q10 as i64
            + (*sLPC_Q14_ptr.offset((16 as i32 + i - 2 as i32) as isize) as i64
                * A_Q12[1 as i32 as usize] as i64
                >> 16 as i32)) as opus_int32;
        LPC_pred_Q10 = (LPC_pred_Q10 as i64
            + (*sLPC_Q14_ptr.offset((16 as i32 + i - 3 as i32) as isize) as i64
                * A_Q12[2 as i32 as usize] as i64
                >> 16 as i32)) as opus_int32;
        LPC_pred_Q10 = (LPC_pred_Q10 as i64
            + (*sLPC_Q14_ptr.offset((16 as i32 + i - 4 as i32) as isize) as i64
                * A_Q12[3 as i32 as usize] as i64
                >> 16 as i32)) as opus_int32;
        LPC_pred_Q10 = (LPC_pred_Q10 as i64
            + (*sLPC_Q14_ptr.offset((16 as i32 + i - 5 as i32) as isize) as i64
                * A_Q12[4 as i32 as usize] as i64
                >> 16 as i32)) as opus_int32;
        LPC_pred_Q10 = (LPC_pred_Q10 as i64
            + (*sLPC_Q14_ptr.offset((16 as i32 + i - 6 as i32) as isize) as i64
                * A_Q12[5 as i32 as usize] as i64
                >> 16 as i32)) as opus_int32;
        LPC_pred_Q10 = (LPC_pred_Q10 as i64
            + (*sLPC_Q14_ptr.offset((16 as i32 + i - 7 as i32) as isize) as i64
                * A_Q12[6 as i32 as usize] as i64
                >> 16 as i32)) as opus_int32;
        LPC_pred_Q10 = (LPC_pred_Q10 as i64
            + (*sLPC_Q14_ptr.offset((16 as i32 + i - 8 as i32) as isize) as i64
                * A_Q12[7 as i32 as usize] as i64
                >> 16 as i32)) as opus_int32;
        LPC_pred_Q10 = (LPC_pred_Q10 as i64
            + (*sLPC_Q14_ptr.offset((16 as i32 + i - 9 as i32) as isize) as i64
                * A_Q12[8 as i32 as usize] as i64
                >> 16 as i32)) as opus_int32;
        LPC_pred_Q10 = (LPC_pred_Q10 as i64
            + (*sLPC_Q14_ptr.offset((16 as i32 + i - 10 as i32) as isize) as i64
                * A_Q12[9 as i32 as usize] as i64
                >> 16 as i32)) as opus_int32;
        j = 10 as i32;
        while j < (*psDec).LPC_order {
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*sLPC_Q14_ptr.offset((16 as i32 + i - j - 1 as i32) as isize) as i64
                    * A_Q12[j as usize] as i64
                    >> 16 as i32)) as opus_int32;
            j += 1
        }
        /* Add prediction to LPC excitation */
        *sLPC_Q14_ptr.offset((16 as i32 + i) as isize) = if (*sLPC_Q14_ptr
            .offset((16 as i32 + i) as isize)
            as opus_uint32)
            .wrapping_add(
                (((if 0x80000000 as u32 as opus_int32 >> 4 as i32 > 0x7fffffff as i32 >> 4 as i32 {
                    (if LPC_pred_Q10 > 0x80000000 as u32 as opus_int32 >> 4 as i32 {
                        (0x80000000 as u32 as opus_int32) >> 4 as i32
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
                        (if LPC_pred_Q10 < 0x80000000 as u32 as opus_int32 >> 4 as i32 {
                            (0x80000000 as u32 as opus_int32) >> 4 as i32
                        } else {
                            LPC_pred_Q10
                        })
                    })
                }) as opus_uint32)
                    << 4 as i32) as opus_int32 as opus_uint32,
            )
            & 0x80000000 as u32
            == 0 as i32 as u32
        {
            if (*sLPC_Q14_ptr.offset((16 as i32 + i) as isize)
                & (((if 0x80000000 as u32 as opus_int32 >> 4 as i32 > 0x7fffffff as i32 >> 4 as i32
                {
                    (if LPC_pred_Q10 > 0x80000000 as u32 as opus_int32 >> 4 as i32 {
                        (0x80000000 as u32 as opus_int32) >> 4 as i32
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
                        (if LPC_pred_Q10 < 0x80000000 as u32 as opus_int32 >> 4 as i32 {
                            (0x80000000 as u32 as opus_int32) >> 4 as i32
                        } else {
                            LPC_pred_Q10
                        })
                    })
                }) as opus_uint32)
                    << 4 as i32) as opus_int32) as u32
                & 0x80000000 as u32
                != 0 as i32 as u32
            {
                0x80000000 as u32 as opus_int32
            } else {
                (*sLPC_Q14_ptr.offset((16 as i32 + i) as isize))
                    + (((if 0x80000000 as u32 as opus_int32 >> 4 as i32
                        > 0x7fffffff as i32 >> 4 as i32
                    {
                        (if LPC_pred_Q10 > 0x80000000 as u32 as opus_int32 >> 4 as i32 {
                            (0x80000000 as u32 as opus_int32) >> 4 as i32
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
                            (if LPC_pred_Q10 < 0x80000000 as u32 as opus_int32 >> 4 as i32 {
                                (0x80000000 as u32 as opus_int32) >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    }) as opus_uint32)
                        << 4 as i32) as opus_int32
            }
        } else if (*sLPC_Q14_ptr.offset((16 as i32 + i) as isize)
            | (((if 0x80000000 as u32 as opus_int32 >> 4 as i32 > 0x7fffffff as i32 >> 4 as i32 {
                (if LPC_pred_Q10 > 0x80000000 as u32 as opus_int32 >> 4 as i32 {
                    (0x80000000 as u32 as opus_int32) >> 4 as i32
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
                    (if LPC_pred_Q10 < 0x80000000 as u32 as opus_int32 >> 4 as i32 {
                        (0x80000000 as u32 as opus_int32) >> 4 as i32
                    } else {
                        LPC_pred_Q10
                    })
                })
            }) as opus_uint32)
                << 4 as i32) as opus_int32) as u32
            & 0x80000000 as u32
            == 0 as i32 as u32
        {
            0x7fffffff as i32
        } else {
            (*sLPC_Q14_ptr.offset((16 as i32 + i) as isize))
                + (((if 0x80000000 as u32 as opus_int32 >> 4 as i32 > 0x7fffffff as i32 >> 4 as i32
                {
                    (if LPC_pred_Q10 > 0x80000000 as u32 as opus_int32 >> 4 as i32 {
                        (0x80000000 as u32 as opus_int32) >> 4 as i32
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
                        (if LPC_pred_Q10 < 0x80000000 as u32 as opus_int32 >> 4 as i32 {
                            (0x80000000 as u32 as opus_int32) >> 4 as i32
                        } else {
                            LPC_pred_Q10
                        })
                    })
                }) as opus_uint32)
                    << 4 as i32) as opus_int32
        };
        /* Scale with Gain */
        *frame.offset(i as isize) = if (if (if 8 as i32 == 1 as i32 {
            ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                * prevGain_Q10[1 as i32 as usize] as i64
                >> 16 as i32) as opus_int32
                >> 1 as i32)
                + ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                    * prevGain_Q10[1 as i32 as usize] as i64
                    >> 16 as i32) as opus_int32
                    & 1 as i32)
        } else {
            (((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                * prevGain_Q10[1 as i32 as usize] as i64
                >> 16 as i32) as opus_int32
                >> 8 as i32 - 1 as i32)
                + 1 as i32)
                >> 1 as i32
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else {
            (if (if 8 as i32 == 1 as i32 {
                ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                    * prevGain_Q10[1 as i32 as usize] as i64
                    >> 16 as i32) as opus_int32
                    >> 1 as i32)
                    + ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                        * prevGain_Q10[1 as i32 as usize] as i64
                        >> 16 as i32) as opus_int32
                        & 1 as i32)
            } else {
                (((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                    * prevGain_Q10[1 as i32 as usize] as i64
                    >> 16 as i32) as opus_int32
                    >> 8 as i32 - 1 as i32)
                    + 1 as i32)
                    >> 1 as i32
            }) < 0x8000 as i32 as opus_int16 as i32
            {
                0x8000 as i32 as opus_int16 as i32
            } else {
                (if 8 as i32 == 1 as i32 {
                    ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                        * prevGain_Q10[1 as i32 as usize] as i64
                        >> 16 as i32) as opus_int32
                        >> 1 as i32)
                        + ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                            * prevGain_Q10[1 as i32 as usize] as i64
                            >> 16 as i32) as opus_int32
                            & 1 as i32)
                } else {
                    (((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                        * prevGain_Q10[1 as i32 as usize] as i64
                        >> 16 as i32) as opus_int32
                        >> 8 as i32 - 1 as i32)
                        + 1 as i32)
                        >> 1 as i32
                })
            })
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else if (if (if 8 as i32 == 1 as i32 {
            ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                * prevGain_Q10[1 as i32 as usize] as i64
                >> 16 as i32) as opus_int32
                >> 1 as i32)
                + ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                    * prevGain_Q10[1 as i32 as usize] as i64
                    >> 16 as i32) as opus_int32
                    & 1 as i32)
        } else {
            (((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                * prevGain_Q10[1 as i32 as usize] as i64
                >> 16 as i32) as opus_int32
                >> 8 as i32 - 1 as i32)
                + 1 as i32)
                >> 1 as i32
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else {
            (if (if 8 as i32 == 1 as i32 {
                ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                    * prevGain_Q10[1 as i32 as usize] as i64
                    >> 16 as i32) as opus_int32
                    >> 1 as i32)
                    + ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                        * prevGain_Q10[1 as i32 as usize] as i64
                        >> 16 as i32) as opus_int32
                        & 1 as i32)
            } else {
                (((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                    * prevGain_Q10[1 as i32 as usize] as i64
                    >> 16 as i32) as opus_int32
                    >> 8 as i32 - 1 as i32)
                    + 1 as i32)
                    >> 1 as i32
            }) < 0x8000 as i32 as opus_int16 as i32
            {
                0x8000 as i32 as opus_int16 as i32
            } else {
                (if 8 as i32 == 1 as i32 {
                    ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                        * prevGain_Q10[1 as i32 as usize] as i64
                        >> 16 as i32) as opus_int32
                        >> 1 as i32)
                        + ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                            * prevGain_Q10[1 as i32 as usize] as i64
                            >> 16 as i32) as opus_int32
                            & 1 as i32)
                } else {
                    (((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                        * prevGain_Q10[1 as i32 as usize] as i64
                        >> 16 as i32) as opus_int32
                        >> 8 as i32 - 1 as i32)
                        + 1 as i32)
                        >> 1 as i32
                })
            })
        }) < 0x8000 as i32 as opus_int16 as i32
        {
            0x8000 as i32 as opus_int16 as i32
        } else if (if 8 as i32 == 1 as i32 {
            ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                * prevGain_Q10[1 as i32 as usize] as i64
                >> 16 as i32) as opus_int32
                >> 1 as i32)
                + ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                    * prevGain_Q10[1 as i32 as usize] as i64
                    >> 16 as i32) as opus_int32
                    & 1 as i32)
        } else {
            (((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                * prevGain_Q10[1 as i32 as usize] as i64
                >> 16 as i32) as opus_int32
                >> 8 as i32 - 1 as i32)
                + 1 as i32)
                >> 1 as i32
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else if (if 8 as i32 == 1 as i32 {
            ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                * prevGain_Q10[1 as i32 as usize] as i64
                >> 16 as i32) as opus_int32
                >> 1 as i32)
                + ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                    * prevGain_Q10[1 as i32 as usize] as i64
                    >> 16 as i32) as opus_int32
                    & 1 as i32)
        } else {
            (((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                * prevGain_Q10[1 as i32 as usize] as i64
                >> 16 as i32) as opus_int32
                >> 8 as i32 - 1 as i32)
                + 1 as i32)
                >> 1 as i32
        }) < 0x8000 as i32 as opus_int16 as i32
        {
            0x8000 as i32 as opus_int16 as i32
        } else if 8 as i32 == 1 as i32 {
            ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                * prevGain_Q10[1 as i32 as usize] as i64
                >> 16 as i32) as opus_int32
                >> 1 as i32)
                + ((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                    * prevGain_Q10[1 as i32 as usize] as i64
                    >> 16 as i32) as opus_int32
                    & 1 as i32)
        } else {
            (((*sLPC_Q14_ptr.offset((16 as i32 + i) as isize) as i64
                * prevGain_Q10[1 as i32 as usize] as i64
                >> 16 as i32) as opus_int32
                >> 8 as i32 - 1 as i32)
                + 1 as i32)
                >> 1 as i32
        } as opus_int16;
        i += 1
    }
    /* Save LPC state */
    crate::stdlib::memcpy(
        (*psDec).sLPC_Q14_buf.as_mut_ptr() as *mut libc::c_void,
        &mut *sLPC_Q14_ptr.offset((*psDec).frame_length as isize) as *mut opus_int32
            as *const libc::c_void,
        (16 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<opus_int32>() as libc::c_ulong),
    );
    /* *************************************/
    /* Update states                      */
    /* *************************************/
    (*psPLC).rand_seed = rand_seed;
    (*psPLC).randScale_Q14 = rand_scale_Q14;
    i = 0 as i32;
    while i < 4 as i32 {
        (*psDecCtrl).pitchL[i as usize] = lag;
        i += 1
    }
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
/* 0.7 in Q14               */
/* 0.95 in Q14              */
/* 2^3 = 8 dB LPC gain      */
/* 2^8 = 24 dB LPC gain     */
/* 0.01 in Q16              */
/* I/O Decoder state        */
/* I/O Decoder state        */
/* I/O Decoder control      */
/* I/O  signal              */
/* I Loss flag              */
/* I Run-time architecture  */
/* Glues concealed frames with new good received frames */
#[no_mangle]

pub unsafe extern "C" fn silk_PLC_glue_frames(
    mut psDec: *mut silk_decoder_state,
    mut frame: *mut opus_int16,
    mut length: i32,
)
/* I length of signal       */
{
    let mut i: i32 = 0;
    let mut energy_shift: i32 = 0;
    let mut energy: opus_int32 = 0;
    let mut psPLC: *mut silk_PLC_struct = 0 as *mut silk_PLC_struct;
    psPLC = &mut (*psDec).sPLC;
    if (*psDec).lossCnt != 0 {
        /* Calculate energy in concealed residual */
        silk_sum_sqr_shift(
            &mut (*psPLC).conc_energy,
            &mut (*psPLC).conc_energy_shift,
            frame as *const opus_int16,
            length,
        );
        (*psPLC).last_frame_lost = 1 as i32
    } else {
        if (*psDec).sPLC.last_frame_lost != 0 {
            /* Calculate residual in decoded signal if last frame was lost */
            silk_sum_sqr_shift(
                &mut energy,
                &mut energy_shift,
                frame as *const opus_int16,
                length,
            );
            /* Normalize energies */
            if energy_shift > (*psPLC).conc_energy_shift {
                (*psPLC).conc_energy =
                    (*psPLC).conc_energy >> energy_shift - (*psPLC).conc_energy_shift
            } else if energy_shift < (*psPLC).conc_energy_shift {
                energy = energy >> (*psPLC).conc_energy_shift - energy_shift
            }
            /* Fade in the energy difference */
            if energy > (*psPLC).conc_energy {
                let mut frac_Q24: opus_int32 = 0;
                let mut LZ: opus_int32 = 0;
                let mut gain_Q16: opus_int32 = 0;
                let mut slope_Q16: opus_int32 = 0;
                LZ = silk_CLZ32((*psPLC).conc_energy);
                LZ = LZ - 1 as i32;
                (*psPLC).conc_energy = (((*psPLC).conc_energy as opus_uint32) << LZ) as opus_int32;
                energy = energy >> silk_max_32(24 as i32 - LZ, 0 as i32);
                frac_Q24 =
                    (*psPLC).conc_energy / (if energy > 1 as i32 { energy } else { 1 as i32 });
                gain_Q16 = ((silk_SQRT_APPROX(frac_Q24) as opus_uint32) << 4 as i32) as opus_int32;
                slope_Q16 = (((1 as i32) << 16 as i32) - gain_Q16) / length;
                /* Make slope 4x steeper to avoid missing onsets after DTX */
                slope_Q16 = ((slope_Q16 as opus_uint32) << 2 as i32) as opus_int32;
                i = 0 as i32;
                while i < length {
                    *frame.offset(i as isize) = (gain_Q16 as i64 * *frame.offset(i as isize) as i64
                        >> 16 as i32) as opus_int32
                        as opus_int16;
                    gain_Q16 += slope_Q16;
                    if gain_Q16 > (1 as i32) << 16 as i32 {
                        break;
                    }
                    i += 1
                }
            }
        }
        (*psPLC).last_frame_lost = 0 as i32
    };
}

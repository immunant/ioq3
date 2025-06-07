use ::libc;

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
/*
 * Matrix of resampling methods used:
 *                                 Fs_out (kHz)
 *                        8      12     16     24     48
 *
 *               8        C      UF     U      UF     UF
 *              12        AF     C      UF     U      UF
 * Fs_in (kHz)  16        D      AF     C      UF     UF
 *              24        AF     D      AF     C      U
 *              48        AF     AF     AF     D      C
 *
 * C   -> Copy (no resampling)
 * D   -> Allpass-based 2x downsampling
 * U   -> Allpass-based 2x upsampling
 * UF  -> Allpass-based 2x upsampling followed by FIR interpolation
 * AF  -> AR2 filter followed by FIR interpolation
 */
/* Tables with delay compensation values to equalize total delay for different modes */

static mut delay_matrix_enc: [[libc::c_schar; 3]; 5] = [
    [
        6 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        10 as libc::c_int as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        6 as libc::c_int as libc::c_schar,
    ],
    [
        18 as libc::c_int as libc::c_schar,
        10 as libc::c_int as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
    ],
];

static mut delay_matrix_dec: [[libc::c_schar; 5]; 3] = [
    [
        4 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
    ],
];
/* Initialize/reset the resampler state for a given pair of input/output sampling rates */
#[no_mangle]

pub unsafe extern "C" fn silk_resampler_init(
    mut S: *mut crate::resampler_structs_h::silk_resampler_state_struct,
    mut Fs_Hz_in: crate::opus_types_h::opus_int32,
    mut Fs_Hz_out: crate::opus_types_h::opus_int32,
    mut forEnc: libc::c_int,
) -> libc::c_int
/* I    If 1: encoder; if 0: decoder                                */ {
    let mut up2x: libc::c_int = 0;
    /* Clear state */
    crate::stdlib::memset(
        S as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::resampler_structs_h::silk_resampler_state_struct>()
            as libc::c_ulong,
    );
    /* Input checking */
    if forEnc != 0 {
        if Fs_Hz_in != 8000 as libc::c_int
            && Fs_Hz_in != 12000 as libc::c_int
            && Fs_Hz_in != 16000 as libc::c_int
            && Fs_Hz_in != 24000 as libc::c_int
            && Fs_Hz_in != 48000 as libc::c_int
            || Fs_Hz_out != 8000 as libc::c_int
                && Fs_Hz_out != 12000 as libc::c_int
                && Fs_Hz_out != 16000 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        (*S).inputDelay = delay_matrix_enc[(((Fs_Hz_in >> 12 as libc::c_int)
            - (Fs_Hz_in > 16000 as libc::c_int) as libc::c_int
            >> (Fs_Hz_in > 24000 as libc::c_int) as libc::c_int)
            - 1 as libc::c_int) as usize][(((Fs_Hz_out >> 12 as libc::c_int)
            - (Fs_Hz_out > 16000 as libc::c_int) as libc::c_int
            >> (Fs_Hz_out > 24000 as libc::c_int) as libc::c_int)
            - 1 as libc::c_int) as usize] as libc::c_int
    } else {
        if Fs_Hz_in != 8000 as libc::c_int
            && Fs_Hz_in != 12000 as libc::c_int
            && Fs_Hz_in != 16000 as libc::c_int
            || Fs_Hz_out != 8000 as libc::c_int
                && Fs_Hz_out != 12000 as libc::c_int
                && Fs_Hz_out != 16000 as libc::c_int
                && Fs_Hz_out != 24000 as libc::c_int
                && Fs_Hz_out != 48000 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        (*S).inputDelay = delay_matrix_dec[(((Fs_Hz_in >> 12 as libc::c_int)
            - (Fs_Hz_in > 16000 as libc::c_int) as libc::c_int
            >> (Fs_Hz_in > 24000 as libc::c_int) as libc::c_int)
            - 1 as libc::c_int) as usize][(((Fs_Hz_out >> 12 as libc::c_int)
            - (Fs_Hz_out > 16000 as libc::c_int) as libc::c_int
            >> (Fs_Hz_out > 24000 as libc::c_int) as libc::c_int)
            - 1 as libc::c_int) as usize] as libc::c_int
    }
    (*S).Fs_in_kHz = Fs_Hz_in / 1000 as libc::c_int;
    (*S).Fs_out_kHz = Fs_Hz_out / 1000 as libc::c_int;
    /* Number of samples processed per batch */
    (*S).batchSize = (*S).Fs_in_kHz * 10 as libc::c_int;
    /* Find resampler with the right sampling ratio */
    up2x = 0 as libc::c_int;
    if Fs_Hz_out > Fs_Hz_in {
        /* Upsample */
        if Fs_Hz_out == Fs_Hz_in * 2 as libc::c_int {
            /* Fs_out : Fs_in = 2 : 1 */
            /* Special case: directly use 2x upsampler */
            (*S).resampler_function = 1 as libc::c_int
        } else {
            /* Default resampler */
            (*S).resampler_function = 2 as libc::c_int;
            up2x = 1 as libc::c_int
        }
    } else if Fs_Hz_out < Fs_Hz_in {
        /* Downsample */
        (*S).resampler_function = 3 as libc::c_int;
        if Fs_Hz_out * 4 as libc::c_int == Fs_Hz_in * 3 as libc::c_int {
            /* Fs_out : Fs_in = 3 : 4 */
            (*S).FIR_Fracs = 3 as libc::c_int;
            (*S).FIR_Order = 18 as libc::c_int;
            (*S).Coefs =
                crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_3_4_COEFS.as_ptr()
        } else if Fs_Hz_out * 3 as libc::c_int == Fs_Hz_in * 2 as libc::c_int {
            /* Fs_out : Fs_in = 2 : 3 */
            (*S).FIR_Fracs = 2 as libc::c_int;
            (*S).FIR_Order = 18 as libc::c_int;
            (*S).Coefs =
                crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_2_3_COEFS.as_ptr()
        } else if Fs_Hz_out * 2 as libc::c_int == Fs_Hz_in {
            /* Fs_out : Fs_in = 1 : 2 */
            (*S).FIR_Fracs = 1 as libc::c_int;
            (*S).FIR_Order = 24 as libc::c_int;
            (*S).Coefs =
                crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_1_2_COEFS.as_ptr()
        } else if Fs_Hz_out * 3 as libc::c_int == Fs_Hz_in {
            /* Fs_out : Fs_in = 1 : 3 */
            (*S).FIR_Fracs = 1 as libc::c_int;
            (*S).FIR_Order = 36 as libc::c_int;
            (*S).Coefs =
                crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_1_3_COEFS.as_ptr()
        } else if Fs_Hz_out * 4 as libc::c_int == Fs_Hz_in {
            /* Fs_out : Fs_in = 1 : 4 */
            (*S).FIR_Fracs = 1 as libc::c_int;
            (*S).FIR_Order = 36 as libc::c_int;
            (*S).Coefs =
                crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_1_4_COEFS.as_ptr()
        } else if Fs_Hz_out * 6 as libc::c_int == Fs_Hz_in {
            /* Fs_out : Fs_in = 1 : 6 */
            (*S).FIR_Fracs = 1 as libc::c_int;
            (*S).FIR_Order = 36 as libc::c_int;
            (*S).Coefs =
                crate::src::opus_1_2_1::silk::resampler_rom::silk_Resampler_1_6_COEFS.as_ptr()
        } else {
            /* None available */
            return -(1 as libc::c_int);
        }
    } else {
        /* Input and output sampling rates are equal: copy */
        (*S).resampler_function = 0 as libc::c_int
    }
    /* Ratio of input/output samples */
    (*S).invRatio_Q16 = (((((Fs_Hz_in as crate::opus_types_h::opus_uint32)
        << 14 as libc::c_int + up2x) as crate::opus_types_h::opus_int32
        / Fs_Hz_out) as crate::opus_types_h::opus_uint32)
        << 2 as libc::c_int) as crate::opus_types_h::opus_int32;
    /* Make sure the ratio is rounded up */
    while (((*S).invRatio_Q16 as libc::c_longlong * Fs_Hz_out as libc::c_longlong
        >> 16 as libc::c_int) as crate::opus_types_h::opus_int32)
        < ((Fs_Hz_in as crate::opus_types_h::opus_uint32) << up2x)
            as crate::opus_types_h::opus_int32
    {
        (*S).invRatio_Q16 += 1
    }
    return 0 as libc::c_int;
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
/*#define silk_MACRO_COUNT */
/* Used to enable WMOPS counting */
/* max order of the LPC analysis in schur() and k2a() */
/* for memset(), memcpy(), memmove() */
/* *******************************************************************/
/*                    SIGNAL PROCESSING FUNCTIONS                   */
/* *******************************************************************/
/* !
 * Initialize/reset the resampler state for a given pair of input/output sampling rates
*/
/* I/O  Resampler state                                             */
/* I    Input sampling rate (Hz)                                    */
/* I    Output sampling rate (Hz)                                   */
/* I    If 1: encoder; if 0: decoder                                */
/* !
 * Resampler: convert from one sampling rate to another
 */
/* Resampler: convert from one sampling rate to another */
/* Input and output sampling rate are at most 48000 Hz  */
#[no_mangle]

pub unsafe extern "C" fn silk_resampler(
    mut S: *mut crate::resampler_structs_h::silk_resampler_state_struct,
    mut out: *mut crate::opus_types_h::opus_int16,
    mut in_0: *const crate::opus_types_h::opus_int16,
    mut inLen: crate::opus_types_h::opus_int32,
) -> libc::c_int
/* I    Number of input samples                                     */ {
    let mut nSamples: libc::c_int = 0;
    /* Need at least 1 ms of input data */
    nSamples = (*S).Fs_in_kHz - (*S).inputDelay;
    /* Copy to delay buffer */
    crate::stdlib::memcpy(&mut *(*S).delayBuf.as_mut_ptr().offset((*S).inputDelay as isize)
               as *mut crate::opus_types_h::opus_int16 as *mut libc::c_void,
           in_0 as *const libc::c_void,
           (nSamples as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()
                                                as libc::c_ulong));
    match (*S).resampler_function {
        1 => {
            crate::src::opus_1_2_1::silk::resampler_private_up2_HQ::silk_resampler_private_up2_HQ_wrapper(S as *mut libc::c_void, out,
                                                  (*S).delayBuf.as_mut_ptr(),
                                                  (*S).Fs_in_kHz);
            crate::src::opus_1_2_1::silk::resampler_private_up2_HQ::silk_resampler_private_up2_HQ_wrapper(S as *mut libc::c_void,
                                                  &mut *out.offset((*S).Fs_out_kHz
                                                                       as
                                                                       isize),
                                                  &*in_0.offset(nSamples as
                                                                    isize),
                                                  inLen - (*S).Fs_in_kHz);
        }
        2 => {
            crate::src::opus_1_2_1::silk::resampler_private_IIR_FIR::silk_resampler_private_IIR_FIR(
                S as *mut libc::c_void,
                out,
                (*S).delayBuf.as_mut_ptr() as *const crate::opus_types_h::opus_int16,
                (*S).Fs_in_kHz,
            );
            crate::src::opus_1_2_1::silk::resampler_private_IIR_FIR::silk_resampler_private_IIR_FIR(
                S as *mut libc::c_void,
                &mut *out.offset((*S).Fs_out_kHz as isize),
                &*in_0.offset(nSamples as isize),
                inLen - (*S).Fs_in_kHz,
            );
        }
        3 => {
            crate::src::opus_1_2_1::silk::resampler_private_down_FIR::silk_resampler_private_down_FIR(S as *mut libc::c_void, out,
                                            (*S).delayBuf.as_mut_ptr() as
                                                *const crate::opus_types_h::opus_int16,
                                            (*S).Fs_in_kHz);
            crate::src::opus_1_2_1::silk::resampler_private_down_FIR::silk_resampler_private_down_FIR(S as *mut libc::c_void,
                                            &mut *out.offset((*S).Fs_out_kHz
                                                                 as isize),
                                            &*in_0.offset(nSamples as isize),
                                            inLen - (*S).Fs_in_kHz);
        }
        _ => {
            crate::stdlib::memcpy(
                out as *mut libc::c_void,
                (*S).delayBuf.as_mut_ptr() as *const libc::c_void,
                ((*S).Fs_in_kHz as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong
                    ),
            );
            crate::stdlib::memcpy(
                &mut *out.offset((*S).Fs_out_kHz as isize) as *mut crate::opus_types_h::opus_int16
                    as *mut libc::c_void,
                &*in_0.offset(nSamples as isize) as *const crate::opus_types_h::opus_int16
                    as *const libc::c_void,
                ((inLen - (*S).Fs_in_kHz) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong
                    ),
            );
        }
    }
    /* Copy to delay buffer */
    crate::stdlib::memcpy((*S).delayBuf.as_mut_ptr() as *mut libc::c_void,
           &*in_0.offset((inLen - (*S).inputDelay) as isize) as
               *const crate::opus_types_h::opus_int16 as *const libc::c_void,
           ((*S).inputDelay as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()
                                                as libc::c_ulong));
    return 0 as libc::c_int;
}

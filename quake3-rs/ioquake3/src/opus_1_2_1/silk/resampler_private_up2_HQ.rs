use ::libc;

pub mod resampler_rom_h {
    /* Tables for 2x upsampler, high quality */

    pub static mut silk_resampler_up2_hq_0: [crate::opus_types_h::opus_int16; 3] = [
        1746 as i32 as crate::opus_types_h::opus_int16,
        14986 as i32 as crate::opus_types_h::opus_int16,
        (39083 as i32 - 65536 as i32) as crate::opus_types_h::opus_int16,
    ];

    pub static mut silk_resampler_up2_hq_1: [crate::opus_types_h::opus_int16; 3] = [
        6854 as i32 as crate::opus_types_h::opus_int16,
        25769 as i32 as crate::opus_types_h::opus_int16,
        (55542 as i32 - 65536 as i32) as crate::opus_types_h::opus_int16,
    ];

    /* SILK_FIX_RESAMPLER_ROM_H */
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
pub use crate::src::opus_1_2_1::silk::resampler_private_up2_HQ::resampler_rom_h::silk_resampler_up2_hq_0;
pub use crate::src::opus_1_2_1::silk::resampler_private_up2_HQ::resampler_rom_h::silk_resampler_up2_hq_1;
/* Upsample by a factor 2, high quality */
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
/* Upsample by a factor 2, high quality */
/* Uses 2nd order allpass filters for the 2x upsampling, followed by a      */
/* notch filter just above Nyquist.                                         */
#[no_mangle]

pub unsafe extern "C" fn silk_resampler_private_up2_HQ(
    mut S: *mut opus_int32,
    mut out: *mut opus_int16,
    mut in_0: *const opus_int16,
    mut len: opus_int32,
)
/* I    Number of input samples     */
{
    let mut k: opus_int32 = 0;
    let mut in32: opus_int32 = 0;
    let mut out32_1: opus_int32 = 0;
    let mut out32_2: opus_int32 = 0;
    let mut Y: opus_int32 = 0;
    let mut X: opus_int32 = 0;
    /* Internal variables and state are in Q10 format */
    k = 0 as i32;
    while k < len {
        /* Convert to Q10 */
        in32 = ((*in_0.offset(k as isize) as opus_int32
            as opus_uint32)
            << 10 as i32) as opus_int32;
        /* First all-pass section for even output sample */
        Y = in32 - *S.offset(0 as i32 as isize);
        X = (Y as i64 * silk_resampler_up2_hq_0[0 as i32 as usize] as i64 >> 16 as i32)
            as opus_int32;
        out32_1 = *S.offset(0 as i32 as isize) + X;
        *S.offset(0 as i32 as isize) = in32 + X;
        /* Second all-pass section for even output sample */
        Y = out32_1 - *S.offset(1 as i32 as isize);
        X = (Y as i64 * silk_resampler_up2_hq_0[1 as i32 as usize] as i64 >> 16 as i32)
            as opus_int32;
        out32_2 = *S.offset(1 as i32 as isize) + X;
        *S.offset(1 as i32 as isize) = out32_1 + X;
        /* Third all-pass section for even output sample */
        Y = out32_2 - *S.offset(2 as i32 as isize);
        X = (Y as i64 + (Y as i64 * silk_resampler_up2_hq_0[2 as i32 as usize] as i64 >> 16 as i32))
            as opus_int32;
        out32_1 = *S.offset(2 as i32 as isize) + X;
        *S.offset(2 as i32 as isize) = out32_2 + X;
        /* Apply gain in Q15, convert back to int16 and store to output */
        *out.offset((2 as i32 * k) as isize) = if (if 10 as i32 == 1 as i32 {
            (out32_1 >> 1 as i32) + (out32_1 & 1 as i32)
        } else {
            ((out32_1 >> 10 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else if (if 10 as i32 == 1 as i32 {
            (out32_1 >> 1 as i32) + (out32_1 & 1 as i32)
        } else {
            ((out32_1 >> 10 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) < 0x8000 as i32 as opus_int16 as i32
        {
            0x8000 as i32 as opus_int16 as i32
        } else if 10 as i32 == 1 as i32 {
            (out32_1 >> 1 as i32) + (out32_1 & 1 as i32)
        } else {
            ((out32_1 >> 10 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        } as opus_int16;
        /* First all-pass section for odd output sample */
        Y = in32 - *S.offset(3 as i32 as isize);
        X = (Y as i64 * silk_resampler_up2_hq_1[0 as i32 as usize] as i64 >> 16 as i32)
            as opus_int32;
        out32_1 = *S.offset(3 as i32 as isize) + X;
        *S.offset(3 as i32 as isize) = in32 + X;
        /* Second all-pass section for odd output sample */
        Y = out32_1 - *S.offset(4 as i32 as isize);
        X = (Y as i64 * silk_resampler_up2_hq_1[1 as i32 as usize] as i64 >> 16 as i32)
            as opus_int32;
        out32_2 = *S.offset(4 as i32 as isize) + X;
        *S.offset(4 as i32 as isize) = out32_1 + X;
        /* Third all-pass section for odd output sample */
        Y = out32_2 - *S.offset(5 as i32 as isize);
        X = (Y as i64 + (Y as i64 * silk_resampler_up2_hq_1[2 as i32 as usize] as i64 >> 16 as i32))
            as opus_int32;
        out32_1 = *S.offset(5 as i32 as isize) + X;
        *S.offset(5 as i32 as isize) = out32_2 + X;
        /* Apply gain in Q15, convert back to int16 and store to output */
        *out.offset((2 as i32 * k + 1 as i32) as isize) = if (if 10 as i32 == 1 as i32 {
            (out32_1 >> 1 as i32) + (out32_1 & 1 as i32)
        } else {
            ((out32_1 >> 10 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else if (if 10 as i32 == 1 as i32 {
            (out32_1 >> 1 as i32) + (out32_1 & 1 as i32)
        } else {
            ((out32_1 >> 10 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) < 0x8000 as i32 as opus_int16 as i32
        {
            0x8000 as i32 as opus_int16 as i32
        } else if 10 as i32 == 1 as i32 {
            (out32_1 >> 1 as i32) + (out32_1 & 1 as i32)
        } else {
            ((out32_1 >> 10 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }
            as opus_int16;
        k += 1
    }
}
/* Upsample by a factor 2, high quality */
#[no_mangle]

pub unsafe extern "C" fn silk_resampler_private_up2_HQ_wrapper(
    mut SS: *mut libc::c_void,
    mut out: *mut opus_int16,
    mut in_0: *const opus_int16,
    mut len: opus_int32,
)
/* I    Number of input samples     */
{
    let mut S: *mut silk_resampler_state_struct =
        SS as *mut silk_resampler_state_struct;
    silk_resampler_private_up2_HQ((*S).sIIR.as_mut_ptr(), out, in_0, len);
}

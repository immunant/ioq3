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
#[inline]

unsafe extern "C" fn silk_resampler_private_IIR_FIR_INTERPOL(
    mut out: *mut opus_int16,
    mut buf: *mut opus_int16,
    mut max_index_Q16: opus_int32,
    mut index_increment_Q16: opus_int32,
) -> *mut opus_int16 {
    let mut index_Q16: opus_int32 = 0;
    let mut res_Q15: opus_int32 = 0;
    let mut buf_ptr: *mut opus_int16 = 0 as *mut opus_int16;
    let mut table_index: opus_int32 = 0;
    /* Interpolate upsampled signal and store in output array */
    index_Q16 = 0 as i32;
    while index_Q16 < max_index_Q16 {
        table_index = ((index_Q16 & 0xffff as i32) as i64 * 12 as i32 as opus_int16 as i64
            >> 16 as i32) as opus_int32;
        buf_ptr = &mut *buf.offset((index_Q16 >> 16 as i32) as isize) as *mut opus_int16;
        res_Q15 = *buf_ptr.offset(0 as i32 as isize) as opus_int32
            * crate::src::opus_1_2_1::silk::resampler_rom::silk_resampler_frac_FIR_12
                [table_index as usize][0 as i32 as usize] as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(1 as i32 as isize) as opus_int32
                * crate::src::opus_1_2_1::silk::resampler_rom::silk_resampler_frac_FIR_12
                    [table_index as usize][1 as i32 as usize] as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(2 as i32 as isize) as opus_int32
                * crate::src::opus_1_2_1::silk::resampler_rom::silk_resampler_frac_FIR_12
                    [table_index as usize][2 as i32 as usize] as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(3 as i32 as isize) as opus_int32
                * crate::src::opus_1_2_1::silk::resampler_rom::silk_resampler_frac_FIR_12
                    [table_index as usize][3 as i32 as usize] as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(4 as i32 as isize) as opus_int32
                * crate::src::opus_1_2_1::silk::resampler_rom::silk_resampler_frac_FIR_12
                    [(11 as i32 - table_index) as usize][3 as i32 as usize]
                    as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(5 as i32 as isize) as opus_int32
                * crate::src::opus_1_2_1::silk::resampler_rom::silk_resampler_frac_FIR_12
                    [(11 as i32 - table_index) as usize][2 as i32 as usize]
                    as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(6 as i32 as isize) as opus_int32
                * crate::src::opus_1_2_1::silk::resampler_rom::silk_resampler_frac_FIR_12
                    [(11 as i32 - table_index) as usize][1 as i32 as usize]
                    as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(7 as i32 as isize) as opus_int32
                * crate::src::opus_1_2_1::silk::resampler_rom::silk_resampler_frac_FIR_12
                    [(11 as i32 - table_index) as usize][0 as i32 as usize]
                    as opus_int32;
        let fresh0 = out;
        out = out.offset(1);
        *fresh0 = if (if 15 as i32 == 1 as i32 {
            (res_Q15 >> 1 as i32) + (res_Q15 & 1 as i32)
        } else {
            ((res_Q15 >> 15 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else if (if 15 as i32 == 1 as i32 {
            (res_Q15 >> 1 as i32) + (res_Q15 & 1 as i32)
        } else {
            ((res_Q15 >> 15 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) < 0x8000 as i32 as opus_int16 as i32
        {
            0x8000 as i32 as opus_int16 as i32
        } else if 15 as i32 == 1 as i32 {
            (res_Q15 >> 1 as i32) + (res_Q15 & 1 as i32)
        } else {
            ((res_Q15 >> 15 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        } as opus_int16;
        index_Q16 += index_increment_Q16
    }
    return out;
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
/* Number of input samples to process in the inner loop */
/* Description: Hybrid IIR/FIR polyphase implementation of resampling */
/* Upsample using a combination of allpass-based 2x upsampling and FIR interpolation */
#[no_mangle]

pub unsafe extern "C" fn silk_resampler_private_IIR_FIR(
    mut SS: *mut libc::c_void,
    mut out: *mut opus_int16,
    mut in_0: *const opus_int16,
    mut inLen: opus_int32,
)
/* I    Number of input samples     */
{
    let mut S: *mut silk_resampler_state_struct = SS as *mut silk_resampler_state_struct;
    let mut nSamplesIn: opus_int32 = 0;
    let mut max_index_Q16: opus_int32 = 0;
    let mut index_increment_Q16: opus_int32 = 0;
    let mut buf: *mut opus_int16 = 0 as *mut opus_int16;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_int16>() as libc::c_ulong)
            .wrapping_mul((2 as i32 * (*S).batchSize + 8 as i32) as libc::c_ulong) as usize,
    );
    buf = fresh1.as_mut_ptr() as *mut opus_int16;
    /* Copy buffered samples to start of buffer */
    crate::stdlib::memcpy(
        buf as *mut libc::c_void,
        (*S).sFIR.i16_0.as_mut_ptr() as *const libc::c_void,
        (8 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
    /* Iterate over blocks of frameSizeIn input samples */
    index_increment_Q16 = (*S).invRatio_Q16;
    loop {
        nSamplesIn = if inLen < (*S).batchSize {
            inLen
        } else {
            (*S).batchSize
        };
        /* Upsample 2x */
        crate::src::opus_1_2_1::silk::resampler_private_up2_HQ::silk_resampler_private_up2_HQ(
            (*S).sIIR.as_mut_ptr(),
            &mut *buf.offset(8 as i32 as isize),
            in_0,
            nSamplesIn,
        ); /* + 1 because 2x upsampling */
        max_index_Q16 = ((nSamplesIn as opus_uint32) << 16 as i32 + 1 as i32) as opus_int32;
        out = silk_resampler_private_IIR_FIR_INTERPOL(out, buf, max_index_Q16, index_increment_Q16);
        in_0 = in_0.offset(nSamplesIn as isize);
        inLen -= nSamplesIn;
        if !(inLen > 0 as i32) {
            break;
        }
        /* More iterations to do; copy last part of filtered signal to beginning of buffer */
        crate::stdlib::memcpy(
            buf as *mut libc::c_void,
            &mut *buf.offset((nSamplesIn << 1 as i32) as isize) as *mut opus_int16
                as *const libc::c_void,
            (8 as i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<opus_int16>() as libc::c_ulong),
        );
    }
    /* Copy last part of filtered signal to the state for the next call */
    crate::stdlib::memcpy(
        (*S).sFIR.i16_0.as_mut_ptr() as *mut libc::c_void,
        &mut *buf.offset((nSamplesIn << 1 as i32) as isize) as *mut opus_int16
            as *const libc::c_void,
        (8 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
}

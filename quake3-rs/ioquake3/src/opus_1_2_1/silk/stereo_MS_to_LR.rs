use ::libc;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;

pub use crate::stdlib::uint32_t;
pub use crate::structs_h::stereo_dec_state;
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
/* Convert adaptive Mid/Side representation to Left/Right stereo signal */
#[no_mangle]

pub unsafe extern "C" fn silk_stereo_MS_to_LR(
    mut state: *mut stereo_dec_state,
    mut x1: *mut opus_int16,
    mut x2: *mut opus_int16,
    mut pred_Q13: *const opus_int32,
    mut fs_kHz: i32,
    mut frame_length: i32,
)
/* I    Number of samples                           */
{
    let mut n: i32 = 0;
    let mut denom_Q16: i32 = 0;
    let mut delta0_Q13: i32 = 0;
    let mut delta1_Q13: i32 = 0;
    let mut sum: opus_int32 = 0;
    let mut diff: opus_int32 = 0;
    let mut pred0_Q13: opus_int32 = 0;
    let mut pred1_Q13: opus_int32 = 0;
    /* Buffering */
    crate::stdlib::memcpy(
        x1 as *mut libc::c_void,
        (*state).sMid.as_mut_ptr() as *const libc::c_void,
        (2 as i32 as usize)
            .wrapping_mul(::std::mem::size_of::<opus_int16>() as usize),
    );
    crate::stdlib::memcpy(
        x2 as *mut libc::c_void,
        (*state).sSide.as_mut_ptr() as *const libc::c_void,
        (2 as i32 as usize)
            .wrapping_mul(::std::mem::size_of::<opus_int16>() as usize),
    );
    crate::stdlib::memcpy(
        (*state).sMid.as_mut_ptr() as *mut libc::c_void,
        &mut *x1.offset(frame_length as isize) as *mut opus_int16 as *const libc::c_void,
        (2 as i32 as usize)
            .wrapping_mul(::std::mem::size_of::<opus_int16>() as usize),
    );
    crate::stdlib::memcpy(
        (*state).sSide.as_mut_ptr() as *mut libc::c_void,
        &mut *x2.offset(frame_length as isize) as *mut opus_int16 as *const libc::c_void,
        (2 as i32 as usize)
            .wrapping_mul(::std::mem::size_of::<opus_int16>() as usize),
    );
    /* Interpolate predictors and add prediction to side channel */
    pred0_Q13 = (*state).pred_prev_Q13[0 as i32 as usize] as opus_int32; /* Q11 */
    pred1_Q13 = (*state).pred_prev_Q13[1 as i32 as usize] as opus_int32; /* Q8  */
    denom_Q16 = ((1 as i32) << 16 as i32) / (8 as i32 * fs_kHz); /* Q8  */
    delta0_Q13 = if 16 as i32 == 1 as i32 {
        ((*pred_Q13.offset(0 as i32 as isize) - (*state).pred_prev_Q13[0 as i32 as usize] as i32)
            as opus_int16 as opus_int32
            * denom_Q16 as opus_int16 as opus_int32
            >> 1 as i32)
            + ((*pred_Q13.offset(0 as i32 as isize)
                - (*state).pred_prev_Q13[0 as i32 as usize] as i32) as opus_int16
                as opus_int32
                * denom_Q16 as opus_int16 as opus_int32
                & 1 as i32)
    } else {
        (((*pred_Q13.offset(0 as i32 as isize) - (*state).pred_prev_Q13[0 as i32 as usize] as i32)
            as opus_int16 as opus_int32
            * denom_Q16 as opus_int16 as opus_int32
            >> 16 as i32 - 1 as i32)
            + 1 as i32)
            >> 1 as i32
    }; /* Q11 */
    delta1_Q13 = if 16 as i32 == 1 as i32 {
        ((*pred_Q13.offset(1 as i32 as isize) - (*state).pred_prev_Q13[1 as i32 as usize] as i32)
            as opus_int16 as opus_int32
            * denom_Q16 as opus_int16 as opus_int32
            >> 1 as i32)
            + ((*pred_Q13.offset(1 as i32 as isize)
                - (*state).pred_prev_Q13[1 as i32 as usize] as i32) as opus_int16
                as opus_int32
                * denom_Q16 as opus_int16 as opus_int32
                & 1 as i32)
    } else {
        (((*pred_Q13.offset(1 as i32 as isize) - (*state).pred_prev_Q13[1 as i32 as usize] as i32)
            as opus_int16 as opus_int32
            * denom_Q16 as opus_int16 as opus_int32
            >> 16 as i32 - 1 as i32)
            + 1 as i32)
            >> 1 as i32
    }; /* Q8  */
    n = 0 as i32; /* Q8  */
    while n < 8 as i32 * fs_kHz {
        pred0_Q13 += delta0_Q13;
        pred1_Q13 += delta1_Q13;
        sum = (((*x1.offset(n as isize) as i32
            + *x1.offset((n + 2 as i32) as isize) as i32
            + ((*x1.offset((n + 1 as i32) as isize) as opus_uint32) << 1 as i32) as opus_int32)
            as opus_uint32)
            << 9 as i32) as opus_int32;
        sum = (((*x2.offset((n + 1 as i32) as isize) as opus_int32 as opus_uint32) << 8 as i32)
            as opus_int32 as i64
            + (sum as i64 * pred0_Q13 as opus_int16 as i64 >> 16 as i32))
            as opus_int32;
        sum = (sum as i64
            + (((*x1.offset((n + 1 as i32) as isize) as opus_int32 as opus_uint32) << 11 as i32)
                as opus_int32 as i64
                * pred1_Q13 as opus_int16 as i64
                >> 16 as i32)) as opus_int32;
        *x2.offset((n + 1 as i32) as isize) = if (if 8 as i32 == 1 as i32 {
            (sum >> 1 as i32) + (sum & 1 as i32)
        } else {
            ((sum >> 8 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else if (if 8 as i32 == 1 as i32 {
            (sum >> 1 as i32) + (sum & 1 as i32)
        } else {
            ((sum >> 8 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) < 0x8000 as i32 as opus_int16 as i32
        {
            0x8000 as i32 as opus_int16 as i32
        } else if 8 as i32 == 1 as i32 {
            (sum >> 1 as i32) + (sum & 1 as i32)
        } else {
            ((sum >> 8 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        } as opus_int16;
        n += 1
    }
    pred0_Q13 = *pred_Q13.offset(0 as i32 as isize);
    pred1_Q13 = *pred_Q13.offset(1 as i32 as isize);
    n = 8 as i32 * fs_kHz;
    while n < frame_length {
        sum = (((*x1.offset(n as isize) as i32
            + *x1.offset((n + 2 as i32) as isize) as i32
            + ((*x1.offset((n + 1 as i32) as isize) as opus_uint32) << 1 as i32) as opus_int32)
            as opus_uint32)
            << 9 as i32) as opus_int32;
        sum = (((*x2.offset((n + 1 as i32) as isize) as opus_int32 as opus_uint32) << 8 as i32)
            as opus_int32 as i64
            + (sum as i64 * pred0_Q13 as opus_int16 as i64 >> 16 as i32))
            as opus_int32;
        sum = (sum as i64
            + (((*x1.offset((n + 1 as i32) as isize) as opus_int32 as opus_uint32) << 11 as i32)
                as opus_int32 as i64
                * pred1_Q13 as opus_int16 as i64
                >> 16 as i32)) as opus_int32;
        *x2.offset((n + 1 as i32) as isize) = if (if 8 as i32 == 1 as i32 {
            (sum >> 1 as i32) + (sum & 1 as i32)
        } else {
            ((sum >> 8 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else if (if 8 as i32 == 1 as i32 {
            (sum >> 1 as i32) + (sum & 1 as i32)
        } else {
            ((sum >> 8 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) < 0x8000 as i32 as opus_int16 as i32
        {
            0x8000 as i32 as opus_int16 as i32
        } else if 8 as i32 == 1 as i32 {
            (sum >> 1 as i32) + (sum & 1 as i32)
        } else {
            ((sum >> 8 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        } as opus_int16;
        n += 1
    }
    (*state).pred_prev_Q13[0 as i32 as usize] = *pred_Q13.offset(0 as i32 as isize) as opus_int16;
    (*state).pred_prev_Q13[1 as i32 as usize] = *pred_Q13.offset(1 as i32 as isize) as opus_int16;
    /* Convert to left/right signals */
    n = 0 as i32;
    while n < frame_length {
        sum = *x1.offset((n + 1 as i32) as isize) as i32
            + *x2.offset((n + 1 as i32) as isize) as opus_int32;
        diff = *x1.offset((n + 1 as i32) as isize) as i32
            - *x2.offset((n + 1 as i32) as isize) as opus_int32;
        *x1.offset((n + 1 as i32) as isize) = if sum > 0x7fff as i32 {
            0x7fff as i32
        } else if sum < 0x8000 as i32 as opus_int16 as i32 {
            0x8000 as i32 as opus_int16 as i32
        } else {
            sum
        } as opus_int16;
        *x2.offset((n + 1 as i32) as isize) = if diff > 0x7fff as i32 {
            0x7fff as i32
        } else if diff < 0x8000 as i32 as opus_int16 as i32 {
            0x8000 as i32 as opus_int16 as i32
        } else {
            diff
        } as opus_int16;
        n += 1
    }
}

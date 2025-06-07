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
    mut state: *mut crate::structs_h::stereo_dec_state,
    mut x1: *mut crate::opus_types_h::opus_int16,
    mut x2: *mut crate::opus_types_h::opus_int16,
    mut pred_Q13: *const crate::opus_types_h::opus_int32,
    mut fs_kHz: libc::c_int,
    mut frame_length: libc::c_int,
)
/* I    Number of samples                           */
{
    let mut n: libc::c_int = 0;
    let mut denom_Q16: libc::c_int = 0;
    let mut delta0_Q13: libc::c_int = 0;
    let mut delta1_Q13: libc::c_int = 0;
    let mut sum: crate::opus_types_h::opus_int32 = 0;
    let mut diff: crate::opus_types_h::opus_int32 = 0;
    let mut pred0_Q13: crate::opus_types_h::opus_int32 = 0;
    let mut pred1_Q13: crate::opus_types_h::opus_int32 = 0;
    /* Buffering */
    crate::stdlib::memcpy(x1 as *mut libc::c_void,
           (*state).sMid.as_mut_ptr() as *const libc::c_void,
           (2 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()
                                                as libc::c_ulong));
    crate::stdlib::memcpy(x2 as *mut libc::c_void,
           (*state).sSide.as_mut_ptr() as *const libc::c_void,
           (2 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()
                                                as libc::c_ulong));
    crate::stdlib::memcpy((*state).sMid.as_mut_ptr() as *mut libc::c_void,
           &mut *x1.offset(frame_length as isize) as *mut crate::opus_types_h::opus_int16 as
               *const libc::c_void,
           (2 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()
                                                as libc::c_ulong));
    crate::stdlib::memcpy((*state).sSide.as_mut_ptr() as *mut libc::c_void,
           &mut *x2.offset(frame_length as isize) as *mut crate::opus_types_h::opus_int16 as
               *const libc::c_void,
           (2 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int16>()
                                                as libc::c_ulong));
    /* Interpolate predictors and add prediction to side channel */
    pred0_Q13 =
        (*state).pred_prev_Q13[0 as libc::c_int as usize] as crate::opus_types_h::opus_int32; /* Q11 */
    pred1_Q13 =
        (*state).pred_prev_Q13[1 as libc::c_int as usize] as crate::opus_types_h::opus_int32; /* Q8  */
    denom_Q16 = ((1 as libc::c_int) << 16 as libc::c_int) / (8 as libc::c_int * fs_kHz); /* Q8  */
    delta0_Q13 = if 16 as libc::c_int == 1 as libc::c_int {
        ((*pred_Q13.offset(0 as libc::c_int as isize)
            - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int)
            as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            * denom_Q16 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            >> 1 as libc::c_int)
            + ((*pred_Q13.offset(0 as libc::c_int as isize)
                - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int)
                as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * denom_Q16 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                & 1 as libc::c_int)
    } else {
        (((*pred_Q13.offset(0 as libc::c_int as isize)
            - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int)
            as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            * denom_Q16 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            >> 16 as libc::c_int - 1 as libc::c_int)
            + 1 as libc::c_int)
            >> 1 as libc::c_int
    }; /* Q11 */
    delta1_Q13 = if 16 as libc::c_int == 1 as libc::c_int {
        ((*pred_Q13.offset(1 as libc::c_int as isize)
            - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int)
            as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            * denom_Q16 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            >> 1 as libc::c_int)
            + ((*pred_Q13.offset(1 as libc::c_int as isize)
                - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int)
                as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * denom_Q16 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                & 1 as libc::c_int)
    } else {
        (((*pred_Q13.offset(1 as libc::c_int as isize)
            - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int)
            as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            * denom_Q16 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
            >> 16 as libc::c_int - 1 as libc::c_int)
            + 1 as libc::c_int)
            >> 1 as libc::c_int
    }; /* Q8  */
    n = 0 as libc::c_int; /* Q8  */
    while n < 8 as libc::c_int * fs_kHz {
        pred0_Q13 += delta0_Q13;
        pred1_Q13 += delta1_Q13;
        sum = (((*x1.offset(n as isize) as libc::c_int
            + *x1.offset((n + 2 as libc::c_int) as isize) as libc::c_int
            + ((*x1.offset((n + 1 as libc::c_int) as isize) as crate::opus_types_h::opus_uint32)
                << 1 as libc::c_int) as crate::opus_types_h::opus_int32)
            as crate::opus_types_h::opus_uint32)
            << 9 as libc::c_int) as crate::opus_types_h::opus_int32;
        sum = (((*x2.offset((n + 1 as libc::c_int) as isize) as crate::opus_types_h::opus_int32
            as crate::opus_types_h::opus_uint32)
            << 8 as libc::c_int) as crate::opus_types_h::opus_int32
            as libc::c_longlong
            + (sum as libc::c_longlong
                * pred0_Q13 as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32;
        sum = (sum as libc::c_longlong
            + (((*x1.offset((n + 1 as libc::c_int) as isize) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_uint32)
                << 11 as libc::c_int) as crate::opus_types_h::opus_int32
                as libc::c_longlong
                * pred1_Q13 as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32;
        *x2.offset((n + 1 as libc::c_int) as isize) = if (if 8 as libc::c_int == 1 as libc::c_int {
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
    pred0_Q13 = *pred_Q13.offset(0 as libc::c_int as isize);
    pred1_Q13 = *pred_Q13.offset(1 as libc::c_int as isize);
    n = 8 as libc::c_int * fs_kHz;
    while n < frame_length {
        sum = (((*x1.offset(n as isize) as libc::c_int
            + *x1.offset((n + 2 as libc::c_int) as isize) as libc::c_int
            + ((*x1.offset((n + 1 as libc::c_int) as isize) as crate::opus_types_h::opus_uint32)
                << 1 as libc::c_int) as crate::opus_types_h::opus_int32)
            as crate::opus_types_h::opus_uint32)
            << 9 as libc::c_int) as crate::opus_types_h::opus_int32;
        sum = (((*x2.offset((n + 1 as libc::c_int) as isize) as crate::opus_types_h::opus_int32
            as crate::opus_types_h::opus_uint32)
            << 8 as libc::c_int) as crate::opus_types_h::opus_int32
            as libc::c_longlong
            + (sum as libc::c_longlong
                * pred0_Q13 as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32;
        sum = (sum as libc::c_longlong
            + (((*x1.offset((n + 1 as libc::c_int) as isize) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_uint32)
                << 11 as libc::c_int) as crate::opus_types_h::opus_int32
                as libc::c_longlong
                * pred1_Q13 as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32;
        *x2.offset((n + 1 as libc::c_int) as isize) = if (if 8 as libc::c_int == 1 as libc::c_int {
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
        *pred_Q13.offset(0 as libc::c_int as isize) as crate::opus_types_h::opus_int16;
    (*state).pred_prev_Q13[1 as libc::c_int as usize] =
        *pred_Q13.offset(1 as libc::c_int as isize) as crate::opus_types_h::opus_int16;
    /* Convert to left/right signals */
    n = 0 as libc::c_int;
    while n < frame_length {
        sum = *x1.offset((n + 1 as libc::c_int) as isize) as libc::c_int
            + *x2.offset((n + 1 as libc::c_int) as isize) as crate::opus_types_h::opus_int32;
        diff = *x1.offset((n + 1 as libc::c_int) as isize) as libc::c_int
            - *x2.offset((n + 1 as libc::c_int) as isize) as crate::opus_types_h::opus_int32;
        *x1.offset((n + 1 as libc::c_int) as isize) = if sum > 0x7fff as libc::c_int {
            0x7fff as libc::c_int
        } else if sum < 0x8000 as libc::c_int as crate::opus_types_h::opus_int16 as libc::c_int {
            0x8000 as libc::c_int as crate::opus_types_h::opus_int16 as libc::c_int
        } else {
            sum
        } as crate::opus_types_h::opus_int16;
        *x2.offset((n + 1 as libc::c_int) as isize) = if diff > 0x7fff as libc::c_int {
            0x7fff as libc::c_int
        } else if diff < 0x8000 as libc::c_int as crate::opus_types_h::opus_int16 as libc::c_int {
            0x8000 as libc::c_int as crate::opus_types_h::opus_int16 as libc::c_int
        } else {
            diff
        } as crate::opus_types_h::opus_int16;
        n += 1
    }
}

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
/* Second order AR filter */
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
/* Second order AR filter with single delay elements */
#[no_mangle]

pub unsafe extern "C" fn silk_resampler_private_AR2(
    mut S: *mut crate::opus_types_h::opus_int32,
    mut out_Q8: *mut crate::opus_types_h::opus_int32,
    mut in_0: *const crate::opus_types_h::opus_int16,
    mut A_Q14: *const crate::opus_types_h::opus_int16,
    mut len: crate::opus_types_h::opus_int32,
)
/* I    Signal length               */
{
    let mut k: crate::opus_types_h::opus_int32 = 0;
    let mut out32: crate::opus_types_h::opus_int32 = 0;
    k = 0 as i32;
    while k < len {
        out32 = *S.offset(0 as i32 as isize)
            + ((*in_0.offset(k as isize) as crate::opus_types_h::opus_int32
                as crate::opus_types_h::opus_uint32)
                << 8 as i32) as crate::opus_types_h::opus_int32;
        *out_Q8.offset(k as isize) = out32;
        out32 = ((out32 as crate::opus_types_h::opus_uint32) << 2 as i32)
            as crate::opus_types_h::opus_int32;
        *S.offset(0 as i32 as isize) =
            (*S.offset(1 as i32 as isize) as i64
                + (out32 as i64
                    * *A_Q14.offset(0 as i32 as isize) as i64
                    >> 16 as i32)) as crate::opus_types_h::opus_int32;
        *S.offset(1 as i32 as isize) = (out32 as i64
            * *A_Q14.offset(1 as i32 as isize) as i64
            >> 16 as i32)
            as crate::opus_types_h::opus_int32;
        k += 1
    }
}

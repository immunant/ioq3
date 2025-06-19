pub mod resampler_rom_h {
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
    /* Tables for 2x downsampler */

    pub static mut silk_resampler_down2_0: crate::opus_types_h::opus_int16 =
        9872 as i32 as crate::opus_types_h::opus_int16;

    pub static mut silk_resampler_down2_1: crate::opus_types_h::opus_int16 =
        (39809 as i32 - 65536 as i32) as crate::opus_types_h::opus_int16;

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
pub use crate::src::opus_1_2_1::silk::resampler_down2::resampler_rom_h::silk_resampler_down2_0;
pub use crate::src::opus_1_2_1::silk::resampler_down2::resampler_rom_h::silk_resampler_down2_1;
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
/* I/O  Resampler state                                             */
/* O    Output signal                                               */
/* I    Input signal                                                */
/* I    Number of input samples                                     */
/* !
* Downsample 2x, mediocre quality
*/
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
/* Downsample by a factor 2 */
#[no_mangle]

pub unsafe extern "C" fn silk_resampler_down2(
    mut S: *mut opus_int32,
    mut out: *mut opus_int16,
    mut in_0: *const opus_int16,
    mut inLen: opus_int32,
)
/* I    Number of input samples                                     */
{
    let mut k: opus_int32 = 0;
    let mut len2: opus_int32 = inLen >> 1 as i32;
    let mut in32: opus_int32 = 0;
    let mut out32: opus_int32 = 0;
    let mut Y: opus_int32 = 0;
    let mut X: opus_int32 = 0;
    /* Internal variables and state are in Q10 format */
    k = 0 as i32;
    while k < len2 {
        /* Convert to Q10 */
        in32 = ((*in_0.offset((2 as i32 * k) as isize) as opus_int32
            as opus_uint32)
            << 10 as i32) as opus_int32;
        /* All-pass section for even input sample */
        Y = in32 - *S.offset(0 as i32 as isize);
        X = (Y as i64 + (Y as i64 * silk_resampler_down2_1 as i64 >> 16 as i32))
            as opus_int32;
        out32 = *S.offset(0 as i32 as isize) + X;
        *S.offset(0 as i32 as isize) = in32 + X;
        /* Convert to Q10 */
        in32 = ((*in_0.offset((2 as i32 * k + 1 as i32) as isize) as opus_int32
            as opus_uint32)
            << 10 as i32) as opus_int32;
        /* All-pass section for odd input sample, and add to output of previous section */
        Y = in32 - *S.offset(1 as i32 as isize);
        X = (Y as i64 * silk_resampler_down2_0 as i64 >> 16 as i32)
            as opus_int32;
        out32 = out32 + *S.offset(1 as i32 as isize);
        out32 = out32 + X;
        *S.offset(1 as i32 as isize) = in32 + X;
        /* Add, convert back to int16 and store to output */
        *out.offset(k as isize) = if (if 11 as i32 == 1 as i32 {
            (out32 >> 1 as i32) + (out32 & 1 as i32)
        } else {
            ((out32 >> 11 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else if (if 11 as i32 == 1 as i32 {
            (out32 >> 1 as i32) + (out32 & 1 as i32)
        } else {
            ((out32 >> 11 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) < 0x8000 as i32 as opus_int16 as i32
        {
            0x8000 as i32 as opus_int16 as i32
        } else if 11 as i32 == 1 as i32 {
            (out32 >> 1 as i32) + (out32 & 1 as i32)
        } else {
            ((out32 >> 11 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        } as opus_int16;
        k += 1
    }
}

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;
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
/*                                                                      *
 * silk_biquad_alt.c                                              *
 *                                                                      *
 * Second order ARMA filter                                             *
 * Can handle slowly varying filter coefficients                        *
 *                                                                      */
/* Second order ARMA filter, alternative implementation */
#[no_mangle]

pub unsafe extern "C" fn silk_biquad_alt_stride1(
    mut in_0: *const opus_int16,
    mut B_Q28: *const opus_int32,
    mut A_Q28: *const opus_int32,
    mut S: *mut opus_int32,
    mut out: *mut opus_int16,
    len: opus_int32,
)
/* I     signal length (must be even)                               */
{
    /* DIRECT FORM II TRANSPOSED (uses 2 element state vector) */
    let mut k: i32 = 0;
    let mut inval: opus_int32 = 0;
    let mut A0_U_Q28: opus_int32 = 0;
    let mut A0_L_Q28: opus_int32 = 0;
    let mut A1_U_Q28: opus_int32 = 0;
    let mut A1_L_Q28: opus_int32 = 0;
    let mut out32_Q14: opus_int32 = 0;
    /* Negate A_Q28 values and split in two parts */
    A0_L_Q28 = -*A_Q28.offset(0 as i32 as isize) & 0x3fff as i32; /* lower part */
    A0_U_Q28 = -*A_Q28.offset(0 as i32 as isize) >> 14 as i32; /* upper part */
    A1_L_Q28 = -*A_Q28.offset(1 as i32 as isize) & 0x3fff as i32; /* lower part */
    A1_U_Q28 = -*A_Q28.offset(1 as i32 as isize) >> 14 as i32; /* upper part */
    k = 0 as i32;
    while k < len {
        /* S[ 0 ], S[ 1 ]: Q12 */
        inval = *in_0.offset(k as isize) as opus_int32;
        out32_Q14 = (((*S.offset(0 as i32 as isize) as i64
            + (*B_Q28.offset(0 as i32 as isize) as i64
                * inval as opus_int16 as i64
                >> 16 as i32)) as opus_int32
            as opus_uint32)
            << 2 as i32) as opus_int32;
        *S.offset(0 as i32 as isize) = *S.offset(1 as i32 as isize)
            + (if 14 as i32 == 1 as i32 {
                ((out32_Q14 as i64 * A0_L_Q28 as opus_int16 as i64
                    >> 16 as i32) as opus_int32
                    >> 1 as i32)
                    + ((out32_Q14 as i64 * A0_L_Q28 as opus_int16 as i64
                        >> 16 as i32) as opus_int32
                        & 1 as i32)
            } else {
                (((out32_Q14 as i64 * A0_L_Q28 as opus_int16 as i64
                    >> 16 as i32) as opus_int32
                    >> 14 as i32 - 1 as i32)
                    + 1 as i32)
                    >> 1 as i32
            });
        *S.offset(0 as i32 as isize) = (*S.offset(0 as i32 as isize) as i64
            + (out32_Q14 as i64 * A0_U_Q28 as opus_int16 as i64 >> 16 as i32))
            as opus_int32;
        *S.offset(0 as i32 as isize) = (*S.offset(0 as i32 as isize) as i64
            + (*B_Q28.offset(1 as i32 as isize) as i64
                * inval as opus_int16 as i64
                >> 16 as i32))
            as opus_int32;
        *S.offset(1 as i32 as isize) = if 14 as i32 == 1 as i32 {
            ((out32_Q14 as i64 * A1_L_Q28 as opus_int16 as i64 >> 16 as i32)
                as opus_int32
                >> 1 as i32)
                + ((out32_Q14 as i64 * A1_L_Q28 as opus_int16 as i64
                    >> 16 as i32) as opus_int32
                    & 1 as i32)
        } else {
            (((out32_Q14 as i64 * A1_L_Q28 as opus_int16 as i64 >> 16 as i32)
                as opus_int32
                >> 14 as i32 - 1 as i32)
                + 1 as i32)
                >> 1 as i32
        };
        *S.offset(1 as i32 as isize) = (*S.offset(1 as i32 as isize) as i64
            + (out32_Q14 as i64 * A1_U_Q28 as opus_int16 as i64 >> 16 as i32))
            as opus_int32;
        *S.offset(1 as i32 as isize) = (*S.offset(1 as i32 as isize) as i64
            + (*B_Q28.offset(2 as i32 as isize) as i64
                * inval as opus_int16 as i64
                >> 16 as i32))
            as opus_int32;
        /* Scale back to Q0 and saturate */
        *out.offset(k as isize) =
            if out32_Q14 + ((1 as i32) << 14 as i32) - 1 as i32 >> 14 as i32 > 0x7fff as i32 {
                0x7fff as i32
            } else if (out32_Q14 + ((1 as i32) << 14 as i32) - 1 as i32 >> 14 as i32)
                < 0x8000 as i32 as opus_int16 as i32
            {
                0x8000 as i32 as opus_int16 as i32
            } else {
                (out32_Q14 + ((1 as i32) << 14 as i32) - 1 as i32) >> 14 as i32
            } as opus_int16;
        k += 1
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
/* I/O  State vector [ 2 ]                                          */
/* O    Output signal [ len ]                                       */
/* I    Input signal [ floor(len/2) ]                               */
/* I    Number of input samples                                     */
/* !
 * Downsample by a factor 2/3, low quality
*/
/* I/O  State vector [ 6 ]                                          */
/* O    Output signal [ floor(2*inLen/3) ]                          */
/* I    Input signal [ inLen ]                                      */
/* I    Number of input samples                                     */
/* !
 * second order ARMA filter;
 * slower than biquad() but uses more precise coefficients
 * can handle (slowly) varying coefficients
 */
/* I     input signal                                               */
/* I     MA coefficients [3]                                        */
/* I     AR coefficients [2]                                        */
/* I/O   State vector [2]                                           */
/* O     output signal                                              */
/* I     signal length (must be even)                               */
#[no_mangle]

pub unsafe extern "C" fn silk_biquad_alt_stride2_c(
    mut in_0: *const opus_int16,
    mut B_Q28: *const opus_int32,
    mut A_Q28: *const opus_int32,
    mut S: *mut opus_int32,
    mut out: *mut opus_int16,
    len: opus_int32,
)
/* I     signal length (must be even)                               */
{
    /* DIRECT FORM II TRANSPOSED (uses 2 element state vector) */
    let mut k: i32 = 0;
    let mut A0_U_Q28: opus_int32 = 0;
    let mut A0_L_Q28: opus_int32 = 0;
    let mut A1_U_Q28: opus_int32 = 0;
    let mut A1_L_Q28: opus_int32 = 0;
    let mut out32_Q14: [opus_int32; 2] = [0; 2];
    /* Negate A_Q28 values and split in two parts */
    A0_L_Q28 = -*A_Q28.offset(0 as i32 as isize) & 0x3fff as i32; /* lower part */
    A0_U_Q28 = -*A_Q28.offset(0 as i32 as isize) >> 14 as i32; /* upper part */
    A1_L_Q28 = -*A_Q28.offset(1 as i32 as isize) & 0x3fff as i32; /* lower part */
    A1_U_Q28 = -*A_Q28.offset(1 as i32 as isize) >> 14 as i32; /* upper part */
    k = 0 as i32;
    while k < len {
        /* S[ 0 ], S[ 1 ], S[ 2 ], S[ 3 ]: Q12 */
        out32_Q14[0 as i32 as usize] = (((*S.offset(0 as i32 as isize) as i64
            + (*B_Q28.offset(0 as i32 as isize) as i64
                * *in_0.offset((2 as i32 * k + 0 as i32) as isize) as i64
                >> 16 as i32))
            as opus_int32
            as opus_uint32)
            << 2 as i32) as opus_int32;
        out32_Q14[1 as i32 as usize] = (((*S.offset(2 as i32 as isize) as i64
            + (*B_Q28.offset(0 as i32 as isize) as i64
                * *in_0.offset((2 as i32 * k + 1 as i32) as isize) as i64
                >> 16 as i32))
            as opus_int32
            as opus_uint32)
            << 2 as i32) as opus_int32;
        *S.offset(0 as i32 as isize) = *S.offset(1 as i32 as isize)
            + (if 14 as i32 == 1 as i32 {
                ((out32_Q14[0 as i32 as usize] as i64
                    * A0_L_Q28 as opus_int16 as i64
                    >> 16 as i32) as opus_int32
                    >> 1 as i32)
                    + ((out32_Q14[0 as i32 as usize] as i64
                        * A0_L_Q28 as opus_int16 as i64
                        >> 16 as i32) as opus_int32
                        & 1 as i32)
            } else {
                (((out32_Q14[0 as i32 as usize] as i64
                    * A0_L_Q28 as opus_int16 as i64
                    >> 16 as i32) as opus_int32
                    >> 14 as i32 - 1 as i32)
                    + 1 as i32)
                    >> 1 as i32
            });
        *S.offset(2 as i32 as isize) = *S.offset(3 as i32 as isize)
            + (if 14 as i32 == 1 as i32 {
                ((out32_Q14[1 as i32 as usize] as i64
                    * A0_L_Q28 as opus_int16 as i64
                    >> 16 as i32) as opus_int32
                    >> 1 as i32)
                    + ((out32_Q14[1 as i32 as usize] as i64
                        * A0_L_Q28 as opus_int16 as i64
                        >> 16 as i32) as opus_int32
                        & 1 as i32)
            } else {
                (((out32_Q14[1 as i32 as usize] as i64
                    * A0_L_Q28 as opus_int16 as i64
                    >> 16 as i32) as opus_int32
                    >> 14 as i32 - 1 as i32)
                    + 1 as i32)
                    >> 1 as i32
            });
        *S.offset(0 as i32 as isize) = (*S.offset(0 as i32 as isize) as i64
            + (out32_Q14[0 as i32 as usize] as i64
                * A0_U_Q28 as opus_int16 as i64
                >> 16 as i32))
            as opus_int32;
        *S.offset(2 as i32 as isize) = (*S.offset(2 as i32 as isize) as i64
            + (out32_Q14[1 as i32 as usize] as i64
                * A0_U_Q28 as opus_int16 as i64
                >> 16 as i32))
            as opus_int32;
        *S.offset(0 as i32 as isize) = (*S.offset(0 as i32 as isize) as i64
            + (*B_Q28.offset(1 as i32 as isize) as i64
                * *in_0.offset((2 as i32 * k + 0 as i32) as isize) as i64
                >> 16 as i32))
            as opus_int32;
        *S.offset(2 as i32 as isize) = (*S.offset(2 as i32 as isize) as i64
            + (*B_Q28.offset(1 as i32 as isize) as i64
                * *in_0.offset((2 as i32 * k + 1 as i32) as isize) as i64
                >> 16 as i32))
            as opus_int32;
        *S.offset(1 as i32 as isize) = if 14 as i32 == 1 as i32 {
            ((out32_Q14[0 as i32 as usize] as i64
                * A1_L_Q28 as opus_int16 as i64
                >> 16 as i32) as opus_int32
                >> 1 as i32)
                + ((out32_Q14[0 as i32 as usize] as i64
                    * A1_L_Q28 as opus_int16 as i64
                    >> 16 as i32) as opus_int32
                    & 1 as i32)
        } else {
            (((out32_Q14[0 as i32 as usize] as i64
                * A1_L_Q28 as opus_int16 as i64
                >> 16 as i32) as opus_int32
                >> 14 as i32 - 1 as i32)
                + 1 as i32)
                >> 1 as i32
        };
        *S.offset(3 as i32 as isize) = if 14 as i32 == 1 as i32 {
            ((out32_Q14[1 as i32 as usize] as i64
                * A1_L_Q28 as opus_int16 as i64
                >> 16 as i32) as opus_int32
                >> 1 as i32)
                + ((out32_Q14[1 as i32 as usize] as i64
                    * A1_L_Q28 as opus_int16 as i64
                    >> 16 as i32) as opus_int32
                    & 1 as i32)
        } else {
            (((out32_Q14[1 as i32 as usize] as i64
                * A1_L_Q28 as opus_int16 as i64
                >> 16 as i32) as opus_int32
                >> 14 as i32 - 1 as i32)
                + 1 as i32)
                >> 1 as i32
        };
        *S.offset(1 as i32 as isize) = (*S.offset(1 as i32 as isize) as i64
            + (out32_Q14[0 as i32 as usize] as i64
                * A1_U_Q28 as opus_int16 as i64
                >> 16 as i32))
            as opus_int32;
        *S.offset(3 as i32 as isize) = (*S.offset(3 as i32 as isize) as i64
            + (out32_Q14[1 as i32 as usize] as i64
                * A1_U_Q28 as opus_int16 as i64
                >> 16 as i32))
            as opus_int32;
        *S.offset(1 as i32 as isize) = (*S.offset(1 as i32 as isize) as i64
            + (*B_Q28.offset(2 as i32 as isize) as i64
                * *in_0.offset((2 as i32 * k + 0 as i32) as isize) as i64
                >> 16 as i32))
            as opus_int32;
        *S.offset(3 as i32 as isize) = (*S.offset(3 as i32 as isize) as i64
            + (*B_Q28.offset(2 as i32 as isize) as i64
                * *in_0.offset((2 as i32 * k + 1 as i32) as isize) as i64
                >> 16 as i32))
            as opus_int32;
        /* Scale back to Q0 and saturate */
        *out.offset((2 as i32 * k + 0 as i32) as isize) =
            if out32_Q14[0 as i32 as usize] + ((1 as i32) << 14 as i32) - 1 as i32 >> 14 as i32
                > 0x7fff as i32
            {
                0x7fff as i32
            } else if (out32_Q14[0 as i32 as usize] + ((1 as i32) << 14 as i32) - 1 as i32
                >> 14 as i32)
                < 0x8000 as i32 as opus_int16 as i32
            {
                0x8000 as i32 as opus_int16 as i32
            } else {
                (out32_Q14[0 as i32 as usize] + ((1 as i32) << 14 as i32) - 1 as i32) >> 14 as i32
            } as opus_int16;
        *out.offset((2 as i32 * k + 1 as i32) as isize) =
            if out32_Q14[1 as i32 as usize] + ((1 as i32) << 14 as i32) - 1 as i32 >> 14 as i32
                > 0x7fff as i32
            {
                0x7fff as i32
            } else if (out32_Q14[1 as i32 as usize] + ((1 as i32) << 14 as i32) - 1 as i32
                >> 14 as i32)
                < 0x8000 as i32 as opus_int16 as i32
            {
                0x8000 as i32 as opus_int16 as i32
            } else {
                (out32_Q14[1 as i32 as usize] + ((1 as i32) << 14 as i32) - 1 as i32) >> 14 as i32
            } as opus_int16;
        k += 1
    }
}

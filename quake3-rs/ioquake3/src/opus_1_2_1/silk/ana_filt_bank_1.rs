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
/* Coefficients for 2-band filter bank based on first-order allpass filters */

static mut A_fb1_20: opus_int16 =
    ((5394 as i32) << 1 as i32) as opus_int16;

static mut A_fb1_21: opus_int16 =
    -(24290 as i32) as opus_int16;
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
/* I     input signal                                               */
/* I     MA coefficients [3]                                        */
/* I     AR coefficients [2]                                        */
/* I/O   State vector [4]                                           */
/* O     output signal                                              */
/* I     signal length (must be even)                               */
/* Variable order MA prediction error filter. */
/* O    Output signal                                               */
/* I    Input signal                                                */
/* I    MA prediction coefficients, Q12 [order]                     */
/* I    Signal length                                               */
/* I    Filter order                                                */
/* I    Run-time architecture                                       */
/* Chirp (bandwidth expand) LP AR filter */
/* I/O  AR filter to be expanded (without leading 1)                */
/* I    Length of ar                                                */
/* I    Chirp factor (typically in the range 0 to 1)                */
/* Chirp (bandwidth expand) LP AR filter */
/* I/O  AR filter to be expanded (without leading 1)                */
/* I    Length of ar                                                */
/* I    Chirp factor in Q16                                         */
/* Compute inverse of LPC prediction gain, and                           */
/* test if LPC coefficients are stable (all poles within unit circle)    */
/* O   Returns inverse prediction gain in energy domain, Q30        */
/* I   Prediction coefficients, Q12 [order]                         */
/* I   Prediction order                                             */
/* Split signal in two decimated bands using first-order allpass filters */
/* (opus_int16)(20623 << 1) */
/* Split signal into two decimated bands using first-order allpass filters */
#[no_mangle]

pub unsafe extern "C" fn silk_ana_filt_bank_1(
    mut in_0: *const opus_int16,
    mut S: *mut opus_int32,
    mut outL: *mut opus_int16,
    mut outH: *mut opus_int16,
    N: opus_int32,
)
/* I    Number of input samples                                     */
{
    let mut k: i32 = 0;
    let mut N2: i32 = N >> 1 as i32;
    let mut in32: opus_int32 = 0;
    let mut X: opus_int32 = 0;
    let mut Y: opus_int32 = 0;
    let mut out_1: opus_int32 = 0;
    let mut out_2: opus_int32 = 0;
    /* Internal variables and state are in Q10 format */
    k = 0 as i32;
    while k < N2 {
        /* Convert to Q10 */
        in32 = ((*in_0.offset((2 as i32 * k) as isize) as opus_int32
            as opus_uint32)
            << 10 as i32) as opus_int32;
        /* All-pass section for even input sample */
        Y = in32 - *S.offset(0 as i32 as isize);
        X = (Y as i64 + (Y as i64 * A_fb1_21 as i64 >> 16 as i32))
            as opus_int32;
        out_1 = *S.offset(0 as i32 as isize) + X;
        *S.offset(0 as i32 as isize) = in32 + X;
        /* Convert to Q10 */
        in32 = ((*in_0.offset((2 as i32 * k + 1 as i32) as isize) as opus_int32
            as opus_uint32)
            << 10 as i32) as opus_int32;
        /* All-pass section for odd input sample, and add to output of previous section */
        Y = in32 - *S.offset(1 as i32 as isize);
        X = (Y as i64 * A_fb1_20 as i64 >> 16 as i32) as opus_int32;
        out_2 = *S.offset(1 as i32 as isize) + X;
        *S.offset(1 as i32 as isize) = in32 + X;
        /* Add/subtract, convert back to int16 and store to output */
        *outL.offset(k as isize) = if (if 11 as i32 == 1 as i32 {
            (out_2 + out_1 >> 1 as i32) + (out_2 + out_1 & 1 as i32)
        } else {
            ((out_2 + out_1 >> 11 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else if (if 11 as i32 == 1 as i32 {
            (out_2 + out_1 >> 1 as i32) + (out_2 + out_1 & 1 as i32)
        } else {
            ((out_2 + out_1 >> 11 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) < 0x8000 as i32 as opus_int16 as i32
        {
            0x8000 as i32 as opus_int16 as i32
        } else if 11 as i32 == 1 as i32 {
            (out_2 + out_1 >> 1 as i32) + (out_2 + out_1 & 1 as i32)
        } else {
            ((out_2 + out_1 >> 11 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        } as opus_int16;
        *outH.offset(k as isize) = if (if 11 as i32 == 1 as i32 {
            (out_2 - out_1 >> 1 as i32) + (out_2 - out_1 & 1 as i32)
        } else {
            ((out_2 - out_1 >> 11 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) > 0x7fff as i32
        {
            0x7fff as i32
        } else if (if 11 as i32 == 1 as i32 {
            (out_2 - out_1 >> 1 as i32) + (out_2 - out_1 & 1 as i32)
        } else {
            ((out_2 - out_1 >> 11 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        }) < 0x8000 as i32 as opus_int16 as i32
        {
            0x8000 as i32 as opus_int16 as i32
        } else if 11 as i32 == 1 as i32 {
            (out_2 - out_1 >> 1 as i32) + (out_2 - out_1 & 1 as i32)
        } else {
            ((out_2 - out_1 >> 11 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
        } as opus_int16;
        k += 1
    }
}

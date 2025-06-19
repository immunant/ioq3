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
/* I    Input signal [N]                                            */
/* I/O  State vector [2]                                            */
/* O    Low band [N/2]                                              */
/* O    High band [N/2]                                             */
/* I    Number of input samples                                     */
/* *******************************************************************/
/*                        SCALAR FUNCTIONS                          */
/* *******************************************************************/
/* Approximation of 128 * log2() (exact inverse of approx 2^() below) */
/* Convert input to a log scale    */
/* I  input in linear scale                                         */
/* Approximation of a sigmoid function */
/* I                                                                */
/* Approximation of 2^() (exact inverse of approx log2() above) */
/* Convert input to a linear scale */
/* I  input on log scale                                            */
/* Compute number of bits to right shift the sum of squares of a vector    */
/* of int16s to make it fit in an int32                                    */
/* O   Energy of x, after shifting to the right                     */
/* O   Number of bits right shift applied to energy                 */
/* I   Input vector                                                 */
/* I   Length of input vector                                       */
/* Calculates the reflection coefficients from the correlation sequence    */
/* Faster than schur64(), but much less accurate.                          */
/* uses SMLAWB(), requiring armv5E and higher.                             */
/* O    Returns residual energy                                     */
/* O    reflection coefficients [order] Q15                         */
/* I    correlations [order+1]                                      */
/* I    prediction order                                            */
/* Calculates the reflection coefficients from the correlation sequence    */
/* Slower than schur(), but more accurate.                                 */
/* Uses SMULL(), available on armv4                                        */
/* O    returns residual energy                                     */
/* O    Reflection coefficients [order] Q16                         */
/* I    Correlations [order+1]                                      */
/* I    Prediction order                                            */
/* Step up function, converts reflection coefficients to prediction coefficients */
/* O    Prediction coefficients [order] Q24                         */
/* I    Reflection coefficients [order] Q15                         */
/* I    Prediction order                                            */
/* Step up function, converts reflection coefficients to prediction coefficients */
/* O    Prediction coefficients [order] Q24                         */
/* I    Reflection coefficients [order] Q16                         */
/* I    Prediction order                                            */
/* Apply sine window to signal vector.                              */
/* Window types:                                                    */
/*    1 -> sine window from 0 to pi/2                               */
/*    2 -> sine window from pi/2 to pi                              */
/* every other sample of window is linearly interpolated, for speed */
/* O    Pointer to windowed signal                                  */
/* I    Pointer to input signal                                     */
/* I    Selects a window type                                       */
/* I    Window length, multiple of 4                                */
/* Compute autocorrelation */
/* O    Result (length correlationCount)                            */
/* O    Scaling of the correlation vector                           */
/* I    Input data to correlate                                     */
/* I    Length of input                                             */
/* I    Number of correlation taps to compute                       */
/* I    Run-time architecture                                       */
/* I                                                                */
/* O                                                                */
/* O    4 pitch values                                              */
/* I    sampling frequency (kHz)                                    */
/* I    number of sub frames                                        */
/* O    Voicing estimate: 0 voiced, 1 unvoiced                      */
/* I    Signal of length PE_FRAME_LENGTH_MS*Fs_kHz                  */
/* O    4 pitch lag values                                          */
/* O    Lag Index                                                   */
/* O    Pitch contour Index                                         */
/* I/O  Normalized correlation; input: value from previous frame    */
/* I    Last lag of previous frame; set to zero is unvoiced         */
/* I    First stage threshold for lag candidates 0 - 1              */
/* I    Final threshold for lag candidates 0 - 1                    */
/* I    Sample frequency (kHz)                                      */
/* I    Complexity setting, 0-2, where 2 is highest                 */
/* I    number of 5 ms subframes                                    */
/* I    Run-time architecture                                       */
/* Compute Normalized Line Spectral Frequencies (NLSFs) from whitening filter coefficients      */
/* If not all roots are found, the a_Q16 coefficients are bandwidth expanded until convergence. */
/* O    Normalized Line Spectral Frequencies in Q15 (0..2^15-1) [d] */
/* I/O  Monic whitening filter coefficients in Q16 [d]              */
/* I    Filter order (must be even)                                 */
/* compute whitening filter coefficients from normalized line spectral frequencies */
/* O    monic whitening filter coefficients in Q12,  [ d ]          */
/* I    normalized line spectral frequencies in Q15, [ d ]          */
/* I    filter order (should be even)                               */
/* I    Run-time architecture                                       */
/* Convert int32 coefficients to int16 coefs and make sure there's no wrap-around */
/* **********************************************************************
Copyright (c) 2013, Koen Vos. All rights reserved.
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
/* Convert int32 coefficients to int16 coefs and make sure there's no wrap-around */
#[no_mangle]

pub unsafe extern "C" fn silk_LPC_fit(
    mut a_QOUT: *mut crate::opus_types_h::opus_int16,
    mut a_QIN: *mut crate::opus_types_h::opus_int32,
    QOUT: i32,
    QIN: i32,
    d: i32,
)
/* I    Filter order                                                */
{
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut idx: i32 = 0 as i32;
    let mut maxabs: crate::opus_types_h::opus_int32 = 0;
    let mut absval: crate::opus_types_h::opus_int32 = 0;
    let mut chirp_Q16: crate::opus_types_h::opus_int32 = 0;
    /* Limit the maximum absolute value of the prediction coefficients, so that they'll fit in int16 */
    i = 0 as i32;
    while i < 10 as i32 {
        /* Find maximum absolute value and its index */
        maxabs = 0 as i32;
        k = 0 as i32;
        while k < d {
            absval = if *a_QIN.offset(k as isize) > 0 as i32 {
                *a_QIN.offset(k as isize)
            } else {
                -*a_QIN.offset(k as isize)
            };
            if absval > maxabs {
                maxabs = absval;
                idx = k
            }
            k += 1
        }
        maxabs = if QIN - QOUT == 1 as i32 {
            (maxabs >> 1 as i32) + (maxabs & 1 as i32)
        } else {
            ((maxabs >> QIN - QOUT - 1 as i32) + 1 as i32) >> 1 as i32
        };
        if !(maxabs > 0x7fff as i32) {
            break;
        }
        /* Reduce magnitude of prediction coefficients */
        maxabs = if maxabs < 163838 as i32 {
            maxabs
        } else {
            163838 as i32
        }; /* ( silk_int32_MAX >> 14 ) + silk_int16_MAX = 163838 */
        chirp_Q16 = (0.999f64 * ((1 as i32 as i64) << 16 as i32) as f64 + 0.5f64)
            as crate::opus_types_h::opus_int32
            - (((maxabs - 0x7fff as i32) as crate::opus_types_h::opus_uint32) << 14 as i32)
                as crate::opus_types_h::opus_int32
                / (maxabs * (idx + 1 as i32) >> 2 as i32);
        crate::src::opus_1_2_1::silk::bwexpander_32::silk_bwexpander_32(a_QIN, d, chirp_Q16);
        i += 1
    }
    if i == 10 as i32 {
        /* Reached the last iteration, clip the coefficients */
        k = 0 as i32;
        while k < d {
            *a_QOUT.offset(k as isize) = if (if QIN - QOUT == 1 as i32 {
                (*a_QIN.offset(k as isize) >> 1 as i32) + (*a_QIN.offset(k as isize) & 1 as i32)
            } else {
                ((*a_QIN.offset(k as isize) >> QIN - QOUT - 1 as i32) + 1 as i32) >> 1 as i32
            }) > 0x7fff as i32
            {
                0x7fff as i32
            } else if (if QIN - QOUT == 1 as i32 {
                (*a_QIN.offset(k as isize) >> 1 as i32) + (*a_QIN.offset(k as isize) & 1 as i32)
            } else {
                ((*a_QIN.offset(k as isize) >> QIN - QOUT - 1 as i32) + 1 as i32) >> 1 as i32
            }) < 0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
            {
                0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
            } else if QIN - QOUT == 1 as i32 {
                (*a_QIN.offset(k as isize) >> 1 as i32) + (*a_QIN.offset(k as isize) & 1 as i32)
            } else {
                ((*a_QIN.offset(k as isize) >> QIN - QOUT - 1 as i32) + 1 as i32) >> 1 as i32
            } as crate::opus_types_h::opus_int16;
            *a_QIN.offset(k as isize) =
                ((*a_QOUT.offset(k as isize) as crate::opus_types_h::opus_int32
                    as crate::opus_types_h::opus_uint32)
                    << QIN - QOUT) as crate::opus_types_h::opus_int32;
            k += 1
        }
    } else {
        k = 0 as i32;
        while k < d {
            *a_QOUT.offset(k as isize) = if QIN - QOUT == 1 as i32 {
                (*a_QIN.offset(k as isize) >> 1 as i32) + (*a_QIN.offset(k as isize) & 1 as i32)
            } else {
                ((*a_QIN.offset(k as isize) >> QIN - QOUT - 1 as i32) + 1 as i32) >> 1 as i32
            } as crate::opus_types_h::opus_int16;
            k += 1
        }
    };
}

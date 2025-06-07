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

/* helper function for NLSF2A(..) */
#[inline]

unsafe extern "C" fn silk_NLSF2A_find_poly(
    mut out: *mut crate::opus_types_h::opus_int32,
    mut cLSF: *const crate::opus_types_h::opus_int32,
    mut dd: libc::c_int,
)
/* I    polynomial order (= 1/2 * filter order)   */
{
    let mut k: libc::c_int = 0; /* QA*/
    let mut n: libc::c_int = 0;
    let mut ftmp: crate::opus_types_h::opus_int32 = 0;
    *out.offset(0 as libc::c_int as isize) =
        ((1 as libc::c_int as crate::opus_types_h::opus_uint32) << 16 as libc::c_int)
            as crate::opus_types_h::opus_int32;
    *out.offset(1 as libc::c_int as isize) = -*cLSF.offset(0 as libc::c_int as isize);
    k = 1 as libc::c_int;
    while k < dd {
        ftmp = *cLSF.offset((2 as libc::c_int * k) as isize);
        *out.offset((k + 1 as libc::c_int) as isize) =
            ((*out.offset((k - 1 as libc::c_int) as isize) as crate::opus_types_h::opus_uint32)
                << 1 as libc::c_int) as crate::opus_types_h::opus_int32
                - (if 16 as libc::c_int == 1 as libc::c_int {
                    (ftmp as libc::c_longlong * *out.offset(k as isize) as libc::c_longlong
                        >> 1 as libc::c_int)
                        + (ftmp as libc::c_longlong * *out.offset(k as isize) as libc::c_longlong
                            & 1 as libc::c_int as libc::c_longlong)
                } else {
                    ((ftmp as libc::c_longlong * *out.offset(k as isize) as libc::c_longlong
                        >> 16 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int as libc::c_longlong)
                        >> 1 as libc::c_int
                }) as crate::opus_types_h::opus_int32;
        n = k;
        while n > 1 as libc::c_int {
            let ref mut fresh0 = *out.offset(n as isize);
            *fresh0 += *out.offset((n - 2 as libc::c_int) as isize)
                - (if 16 as libc::c_int == 1 as libc::c_int {
                    (ftmp as libc::c_longlong
                        * *out.offset((n - 1 as libc::c_int) as isize) as libc::c_longlong
                        >> 1 as libc::c_int)
                        + (ftmp as libc::c_longlong
                            * *out.offset((n - 1 as libc::c_int) as isize) as libc::c_longlong
                            & 1 as libc::c_int as libc::c_longlong)
                } else {
                    ((ftmp as libc::c_longlong
                        * *out.offset((n - 1 as libc::c_int) as isize) as libc::c_longlong
                        >> 16 as libc::c_int - 1 as libc::c_int)
                        + 1 as libc::c_int as libc::c_longlong)
                        >> 1 as libc::c_int
                }) as crate::opus_types_h::opus_int32;
            n -= 1
        }
        let ref mut fresh1 = *out.offset(1 as libc::c_int as isize);
        *fresh1 -= ftmp;
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
/* compute whitening filter coefficients from normalized line spectral frequencies */
#[no_mangle]

pub unsafe extern "C" fn silk_NLSF2A(
    mut a_Q12: *mut crate::opus_types_h::opus_int16,
    mut NLSF: *const crate::opus_types_h::opus_int16,
    d: libc::c_int,
    mut _arch: libc::c_int,
)
/* I    Run-time architecture                                       */
{
    /* This ordering was found to maximize quality. It improves numerical accuracy of
    silk_NLSF2A_find_poly() compared to "standard" ordering. */
    static mut ordering16: [libc::c_uchar; 16] = [
        0 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
    ];
    static mut ordering10: [libc::c_uchar; 10] = [
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
    ];
    let mut ordering: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut dd: libc::c_int = 0;
    let mut cos_LSF_QA: [crate::opus_types_h::opus_int32; 24] = [0; 24];
    let mut P: [crate::opus_types_h::opus_int32; 13] = [0; 13];
    let mut Q: [crate::opus_types_h::opus_int32; 13] = [0; 13];
    let mut Ptmp: crate::opus_types_h::opus_int32 = 0;
    let mut Qtmp: crate::opus_types_h::opus_int32 = 0;
    let mut f_int: crate::opus_types_h::opus_int32 = 0;
    let mut f_frac: crate::opus_types_h::opus_int32 = 0;
    let mut cos_val: crate::opus_types_h::opus_int32 = 0;
    let mut delta: crate::opus_types_h::opus_int32 = 0;
    let mut a32_QA1: [crate::opus_types_h::opus_int32; 24] = [0; 24];
    /* convert LSFs to 2*cos(LSF), using piecewise linear curve from table */
    ordering = if d == 16 as libc::c_int {
        ordering16.as_ptr()
    } else {
        ordering10.as_ptr()
    };
    k = 0 as libc::c_int;
    while k < d {
        /* f_int on a scale 0-127 (rounded down) */
        f_int = *NLSF.offset(k as isize) as libc::c_int >> 15 as libc::c_int - 7 as libc::c_int;
        /* QA */
        f_frac = *NLSF.offset(k as isize) as libc::c_int
            - ((f_int as crate::opus_types_h::opus_uint32) << 15 as libc::c_int - 7 as libc::c_int)
                as crate::opus_types_h::opus_int32;
        cos_val = crate::src::opus_1_2_1::silk::table_LSF_cos::silk_LSFCosTab_FIX_Q12
            [f_int as usize] as crate::opus_types_h::opus_int32;
        delta = crate::src::opus_1_2_1::silk::table_LSF_cos::silk_LSFCosTab_FIX_Q12
            [(f_int + 1 as libc::c_int) as usize] as libc::c_int
            - cos_val;
        cos_LSF_QA[*ordering.offset(k as isize) as usize] =
            if 20 as libc::c_int - 16 as libc::c_int == 1 as libc::c_int {
                (((cos_val as crate::opus_types_h::opus_uint32) << 8 as libc::c_int)
                    as crate::opus_types_h::opus_int32
                    + delta * f_frac
                    >> 1 as libc::c_int)
                    + (((cos_val as crate::opus_types_h::opus_uint32) << 8 as libc::c_int)
                        as crate::opus_types_h::opus_int32
                        + delta * f_frac
                        & 1 as libc::c_int)
            } else {
                ((((cos_val as crate::opus_types_h::opus_uint32) << 8 as libc::c_int)
                    as crate::opus_types_h::opus_int32
                    + delta * f_frac
                    >> 20 as libc::c_int - 16 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int)
                    >> 1 as libc::c_int
            };
        k += 1
    }
    dd = d >> 1 as libc::c_int;
    /* f_frac, range: 0..255 */
    /* Read start and end value from table */
    /* Q12 */
    /* Q12, with a range of 0..200 */
    /* Linear interpolation */
    /* generate even and odd polynomials using convolution */
    silk_NLSF2A_find_poly(
        P.as_mut_ptr(),
        &mut *cos_LSF_QA.as_mut_ptr().offset(0 as libc::c_int as isize),
        dd,
    );
    silk_NLSF2A_find_poly(
        Q.as_mut_ptr(),
        &mut *cos_LSF_QA.as_mut_ptr().offset(1 as libc::c_int as isize),
        dd,
    );
    /* convert even and odd polynomials to opus_int32 Q12 filter coefs */
    k = 0 as libc::c_int;
    while k < dd {
        Ptmp = P[(k + 1 as libc::c_int) as usize] + P[k as usize];
        Qtmp = Q[(k + 1 as libc::c_int) as usize] - Q[k as usize];
        /* QA+1 */
        a32_QA1[k as usize] = -Qtmp - Ptmp;
        a32_QA1[(d - k - 1 as libc::c_int) as usize] = Qtmp - Ptmp;
        k += 1
    }
    /* the Ptmp and Qtmp values at this stage need to fit in int32 */
    /* QA+1 */
    /* Convert int32 coefficients to Q12 int16 coefs */
    crate::src::opus_1_2_1::silk::LPC_fit::silk_LPC_fit(
        a_Q12,
        a32_QA1.as_mut_ptr(),
        12 as libc::c_int,
        16 as libc::c_int + 1 as libc::c_int,
        d,
    );
    i = 0 as libc::c_int;
    while crate::src::opus_1_2_1::silk::LPC_inv_pred_gain::silk_LPC_inverse_pred_gain_c(a_Q12, d)
        == 0 as libc::c_int
        && i < 16 as libc::c_int
    {
        /* Prediction coefficients are (too close to) unstable; apply bandwidth expansion   */
        /* on the unscaled coefficients, convert to Q12 and measure again                   */
        crate::src::opus_1_2_1::silk::bwexpander_32::silk_bwexpander_32(
            a32_QA1.as_mut_ptr(),
            d,
            65536 as libc::c_int
                - ((2 as libc::c_int as crate::opus_types_h::opus_uint32) << i)
                    as crate::opus_types_h::opus_int32,
        );
        k = 0 as libc::c_int;
        while k < d {
            *a_Q12.offset(k as isize) = if 16 as libc::c_int + 1 as libc::c_int - 12 as libc::c_int
                == 1 as libc::c_int
            {
                (a32_QA1[k as usize] >> 1 as libc::c_int) + (a32_QA1[k as usize] & 1 as libc::c_int)
            } else {
                ((a32_QA1[k as usize]
                    >> 16 as libc::c_int + 1 as libc::c_int - 12 as libc::c_int - 1 as libc::c_int)
                    + 1 as libc::c_int)
                    >> 1 as libc::c_int
            } as crate::opus_types_h::opus_int16;
            k += 1
            /* QA+1 -> Q12 */
        }
        i += 1
    }
}

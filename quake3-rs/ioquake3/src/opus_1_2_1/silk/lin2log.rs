use ::libc;

pub mod macros_h {
    #[inline]

    pub unsafe extern "C" fn silk_CLZ32(
        mut in32: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        return if in32 != 0 {
            (32 as i32)
                - (::std::mem::size_of::<u32>() as libc::c_ulong as i32
                    * 8 as i32
                    - (in32 as u32).leading_zeros() as i32)
        } else {
            32 as i32
        };
    }

    /* SILK_MACROS_H */
    /* Column based */
    /* Row based */
}

pub mod Inlines_h {
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
    /* ! \file silk_Inlines.h
     *  \brief silk_Inlines.h defines OPUS_INLINE signal processing functions.
     */
    /* count leading zeros of opus_int64 */
    /* Search in the lower 32 bits */
    /* Search in the upper 32 bits */
    /* get number of leading zeros and fractional part (the bits right after the leading one */
    #[inline]

    pub unsafe extern "C" fn silk_CLZ_FRAC(
        mut in_0: crate::opus_types_h::opus_int32,
        mut lz: *mut crate::opus_types_h::opus_int32,
        mut frac_Q7: *mut crate::opus_types_h::opus_int32,
    )
    /* O  the 7 bits right after the leading one */
    {
        let mut lzeros: crate::opus_types_h::opus_int32 = silk_CLZ32(in_0);
        *lz = lzeros;
        *frac_Q7 = silk_ROR32(in_0, 24 as i32 - lzeros) & 0x7f as i32;
    }

    use crate::src::opus_1_2_1::silk::lin2log::macros_h::silk_CLZ32;
    use crate::src::opus_1_2_1::silk::lin2log::SigProc_FIX_h::silk_ROR32;
    /* SILK_FIX_INLINES_H */
}

pub mod SigProc_FIX_h {
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
    /* O    Output signal                                               */
    /* I/O  Input signal                                                */
    /* I    Input Q domain                                              */
    /* I    Input Q domain                                              */
    /* I    Filter order                                                */
    /* I/O   Unsorted / Sorted vector                                   */
    /* O     Index vector for the sorted elements                       */
    /* I     Vector length                                              */
    /* I     Number of correctly sorted positions                       */
    /* I/O   Unsorted / Sorted vector                                   */
    /* O     Index vector for the sorted elements                       */
    /* I     Vector length                                              */
    /* I     Number of correctly sorted positions                       */
    /* I/O   Unsorted / Sorted vector                                   */
    /* I     Vector length                                              */
    /* NLSF stabilizer, for a single input data vector */
    /* I/O   Unstable/stabilized normalized LSF vector in Q15 [L]       */
    /* I     Min distance vector, NDeltaMin_Q15[L] must be >= 1 [L+1]   */
    /* I     Number of NLSF parameters in the input vector              */
    /* Laroia low complexity NLSF weights */
    /* O     Pointer to input vector weights [D]                        */
    /* I     Pointer to input vector         [D]                        */
    /* I     Input vector dimension (even)                              */
    /* Compute reflection coefficients from input signal */
    /* O    Residual energy                                             */
    /* O    Residual energy Q value                                     */
    /* O    Prediction coefficients (length order)                      */
    /* I    Input signal, length: nb_subfr * ( D + subfr_length )       */
    /* I    Inverse of max prediction gain                              */
    /* I    Input signal subframe length (incl. D preceding samples)    */
    /* I    Number of subframes stacked in x                            */
    /* I    Order                                                       */
    /* I    Run-time architecture                                       */
    /* Copy and multiply a vector by a constant */
    /* I    Gain in Q16                                                 */
    /* I    Length                                                      */
    /* Some for the LTP related function requires Q26 to work.*/
    /* I/O  Q0/Q18                                                      */
    /* I    Q26                                                         */
    /* I    length                                                      */
    /* *******************************************************************/
    /*                        INLINE ARM MATH                           */
    /* *******************************************************************/
    /*    return sum( inVec1[i] * inVec2[i] ) */
    /*    I input vector 1                                              */
    /*    I input vector 2                                              */
    /*    I vector lengths                                              */
    /*    I Run-time architecture                                       */
    /*    I input vector 1                                              */
    /*    I input vector 2                                              */
    /*    I number of bits to shift                                     */
    /*    I vector lengths                                              */
    /*    I input vector 1                                              */
    /*    I input vector 2                                              */
    /*    I vector lengths                                              */
    /* *******************************************************************/
    /*                                MACROS                            */
    /* *******************************************************************/
    /* Rotate a32 right by 'rot' bits. Negative rot values result in rotating
    left. Output is 32bit int.
    Note: contemporary compilers recognize the C expression below and
    compile it into a 'ror' instruction if available. No need for OPUS_INLINE ASM! */
    #[inline]

    pub unsafe extern "C" fn silk_ROR32(
        mut a32: crate::opus_types_h::opus_int32,
        mut rot: i32,
    ) -> crate::opus_types_h::opus_int32 {
        let mut x: crate::opus_types_h::opus_uint32 = a32 as crate::opus_types_h::opus_uint32;
        let mut r: crate::opus_types_h::opus_uint32 = rot as crate::opus_types_h::opus_uint32;
        let mut m: crate::opus_types_h::opus_uint32 = -rot as crate::opus_types_h::opus_uint32;
        if rot == 0 as i32 {
            return a32;
        } else if rot < 0 as i32 {
            return (x << m | x >> (32 as i32 as u32).wrapping_sub(m))
                as crate::opus_types_h::opus_int32;
        } else {
            return (x << (32 as i32 as u32).wrapping_sub(r) | x >> r)
                as crate::opus_types_h::opus_int32;
        };
    }

    /* SILK_SIGPROC_FIX_H */
    /*    silk_SMMUL: Signed top word multiply.
    ARMv6        2 instruction cycles.
    ARMv3M+      3 instruction cycles. use SMULL and ignore LSB registers.(except xM)*/
    /*#define silk_SMMUL(a32, b32)                (opus_int32)silk_RSHIFT(silk_SMLAL(silk_SMULWB((a32), (b32)), (a32), silk_RSHIFT_ROUND((b32), 16)), 16)*/
    /* the following seems faster on x86 */
    /*  Add some multiplication functions that can be easily mapped to ARM. */
    /* PSEUDO-RANDOM GENERATOR                                                          */
    /* Make sure to store the result as the seed for the next call (also in between     */
    /* frames), otherwise result won't be random at all. When only using some of the    */
    /* bits, take the most significant bits by right-shifting.                          */
    /* Be careful, silk_abs returns wrong when input equals to silk_intXX_MIN */
}

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::silk::lin2log::macros_h::silk_CLZ32;
pub use crate::src::opus_1_2_1::silk::lin2log::Inlines_h::silk_CLZ_FRAC;
pub use crate::src::opus_1_2_1::silk::lin2log::SigProc_FIX_h::silk_ROR32;
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
/* Approximation of 128 * log2() (very close inverse of silk_log2lin()) */
/* Convert input to a log scale    */
#[no_mangle]

pub unsafe extern "C" fn silk_lin2log(
    inLin: crate::opus_types_h::opus_int32,
) -> crate::opus_types_h::opus_int32
/* I  input in linear scale                                         */ {
    let mut lz: crate::opus_types_h::opus_int32 = 0;
    let mut frac_Q7: crate::opus_types_h::opus_int32 = 0;
    silk_CLZ_FRAC(inLin, &mut lz, &mut frac_Q7);
    /* Piece-wise parabolic approximation */
    return (frac_Q7 as i64
        + ((frac_Q7 * (128 as i32 - frac_Q7)) as i64
            * 179 as i32 as crate::opus_types_h::opus_int16 as i64
            >> 16 as i32)) as crate::opus_types_h::opus_int32
        + (((31 as i32 - lz) as crate::opus_types_h::opus_uint32) << 7 as i32)
            as crate::opus_types_h::opus_int32;
}

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

pub mod SigProc_FIX_h {
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
    /* Allocate opus_int16 aligned to 4-byte memory address */
    /* Useful Macros that can be adjusted to other platforms */
    /* Fixed point macros */
    /* (a32 * b32) output have to be 32bit int */
    /* (a32 * b32) output have to be 32bit uint */
    /* a32 + (b32 * c32) output have to be 32bit int */
    /* a32 + (b32 * c32) output have to be 32bit uint */
    /* ((a32 >> 16)  * (b32 >> 16)) output have to be 32bit int */
    /* a32 + ((a32 >> 16)  * (b32 >> 16)) output have to be 32bit int */
    /* (a32 * b32) */
    /*(opus_int64)*/
    /* Adds two signed 32-bit values in a way that can overflow, while not relying on undefined behaviour
    (just standard two's complement implementation-specific behaviour) */
    /* Subtractss two signed 32-bit values in a way that can overflow, while not relying on undefined behaviour
    (just standard two's complement implementation-specific behaviour) */
    /* Multiply-accumulate macros that allow overflow in the addition (ie, no asserts in debug mode) */
    /* These macros enables checking for overflow in silk_API_Debug.h*/
    /* Saturation for positive input values */
    /* Add with saturation for positive input values */
    /* shift >= 0, shift < 8  */
    /* shift >= 0, shift < 16 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 64 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 8  */
    /* shift >= 0, shift < 16 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 64 */
    /* shift >= 0, shift < 32 */
    /* saturates before shifting */
    /* shift >= 0, allowed to overflow */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* Requires that shift > 0 */
    /* Number of rightshift required to fit the multiplication */
    /* Macro to convert floating-point constants to fixed-point */
    /* silk_min() versions with typecast in the function call */
    #[inline]

    pub unsafe extern "C" fn silk_min_int(mut a: i32, mut b: i32) -> i32 {
        return if a < b { a } else { b };
    }
    /* silk_min() versions with typecast in the function call */
    #[inline]

    pub unsafe extern "C" fn silk_max_int(mut a: i32, mut b: i32) -> i32 {
        return if a > b { a } else { b };
    }
    #[inline]

    pub unsafe extern "C" fn silk_max_32(
        mut a: crate::opus_types_h::opus_int32,
        mut b: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        return if a > b { a } else { b };
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

pub mod Inlines_h {
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
    /* Approximation of square root                                          */
    /* Accuracy: < +/- 10%  for output values > 15                           */
    /*           < +/- 2.5% for output values > 120                          */
    #[inline]

    pub unsafe extern "C" fn silk_SQRT_APPROX(
        mut x: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        let mut y: crate::opus_types_h::opus_int32 = 0;
        let mut lz: crate::opus_types_h::opus_int32 = 0;
        let mut frac_Q7: crate::opus_types_h::opus_int32 = 0;
        if x <= 0 as i32 {
            return 0 as i32;
        }
        silk_CLZ_FRAC(x, &mut lz, &mut frac_Q7);
        if lz & 1 as i32 != 0 {
            y = 32768 as i32
        } else {
            y = 46214 as i32
            /* 46214 = sqrt(2) * 32768 */
        }
        /* get scaling right */
        y >>= lz >> 1 as i32;
        /* increment using fractional part of input */
        y = (y as i64
            + (y as i64
                * (213 as i32 as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
                    * frac_Q7 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32)
                    as crate::opus_types_h::opus_int16 as i64
                >> 16 as i32)) as crate::opus_types_h::opus_int32;
        return y;
    }

    use crate::src::opus_1_2_1::silk::VAD::macros_h::silk_CLZ32;
    use crate::src::opus_1_2_1::silk::VAD::SigProc_FIX_h::silk_ROR32;
    /* SILK_FIX_INLINES_H */
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
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
pub use crate::src::opus_1_2_1::silk::VAD::macros_h::silk_CLZ32;

pub use crate::structs_h::silk_LP_state;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_VAD_state;
pub use crate::structs_h::silk_encoder_state;
pub use crate::structs_h::silk_nsq_state;
pub use crate::structs_h::SideInfoIndices;

pub use crate::src::opus_1_2_1::silk::ana_filt_bank_1::silk_ana_filt_bank_1;
pub use crate::src::opus_1_2_1::silk::lin2log::silk_lin2log;
pub use crate::src::opus_1_2_1::silk::sigm_Q15::silk_sigm_Q15;
pub use crate::src::opus_1_2_1::silk::VAD::Inlines_h::silk_CLZ_FRAC;
pub use crate::src::opus_1_2_1::silk::VAD::Inlines_h::silk_SQRT_APPROX;
pub use crate::src::opus_1_2_1::silk::VAD::SigProc_FIX_h::silk_ROR32;
pub use crate::src::opus_1_2_1::silk::VAD::SigProc_FIX_h::silk_max_32;
pub use crate::src::opus_1_2_1::silk::VAD::SigProc_FIX_h::silk_max_int;
pub use crate::src::opus_1_2_1::silk::VAD::SigProc_FIX_h::silk_min_int;
/* *********************************/
/* Initialization of the Silk VAD */
/* *********************************/
#[no_mangle]

pub unsafe extern "C" fn silk_VAD_Init(
    mut psSilk_VAD: *mut crate::structs_h::silk_VAD_state,
) -> i32
/* I/O  Pointer to Silk VAD state                   */ {
    let mut b: i32 = 0;
    let mut ret: i32 = 0 as i32;
    /* reset state memory */
    crate::stdlib::memset(
        psSilk_VAD as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::structs_h::silk_VAD_state>() as libc::c_ulong,
    );
    /* init noise levels */
    /* Initialize array with approx pink noise levels (psd proportional to inverse of frequency) */
    b = 0 as i32;
    while b < 4 as i32 {
        (*psSilk_VAD).NoiseLevelBias[b as usize] =
            silk_max_32(50 as i32 / (b + 1 as i32), 1 as i32);
        b += 1
    }
    /* Initialize state */
    b = 0 as i32;
    while b < 4 as i32 {
        (*psSilk_VAD).NL[b as usize] =
            100 as i32 * (*psSilk_VAD).NoiseLevelBias[b as usize];
        (*psSilk_VAD).inv_NL[b as usize] = 0x7fffffff as i32 / (*psSilk_VAD).NL[b as usize];
        b += 1
    }
    (*psSilk_VAD).counter = 15 as i32;
    /* init smoothed energy-to-noise ratio*/
    b = 0 as i32;
    while b < 4 as i32 {
        (*psSilk_VAD).NrgRatioSmth_Q8[b as usize] = 100 as i32 * 256 as i32;
        b += 1
        /* 100 * 256 --> 20 dB SNR */
    }
    return ret;
}
/* Weighting factors for tilt measure */

static mut tiltWeights: [crate::opus_types_h::opus_int32; 4] = [
    30000 as i32,
    6000 as i32,
    -(12000 as i32),
    -(12000 as i32),
];
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
/* I/O  State                                       */
/* I/O  Left input signal, becomes mid signal       */
/* I/O  Right input signal, becomes side signal     */
/* I    Predictors                                  */
/* I    Samples rate (kHz)                          */
/* I    Number of samples                           */
/* Find least-squares prediction gain for one signal based on another and quantize it */
/* O    Returns predictor in Q13                    */
/* O    Ratio of residual and mid energies          */
/* I    Basis signal                                */
/* I    Target signal                               */
/* I/O  Smoothed mid, residual norms                */
/* I    Number of samples                           */
/* I    Smoothing coefficient                       */
/* Quantize mid/side predictors */
/* I/O  Predictors (out: quantized)                 */
/* O    Quantization indices                        */
/* Entropy code the mid/side quantization indices */
/* I/O  Compressor data structure                   */
/* I    Quantization indices                        */
/* Entropy code the mid-only flag */
/* I/O  Compressor data structure                   */
/* Decode mid/side predictors */
/* I/O  Compressor data structure                   */
/* O    Predictors                                  */
/* Decode mid-only flag */
/* I/O  Compressor data structure                   */
/* O    Flag that only mid channel has been coded   */
/* Encodes signs of excitation */
/* I/O  Compressor data structure               */
/* I    pulse signal                            */
/* I    length of input                         */
/* I    Signal type                             */
/* I    Quantization offset type                */
/* I    Sum of absolute pulses per block        */
/* Decodes signs of excitation */
/* I/O  Compressor data structure               */
/* I/O  pulse signal                            */
/* I    length of input                         */
/* I    Signal type                             */
/* I    Quantization offset type                */
/* I    Sum of absolute pulses per block        */
/* Check encoder control struct */
/* I    Control structure                           */
/* Control internal sampling rate */
/* I/O  Pointer to Silk encoder state               */
/* I    Control structure                           */
/* Control SNR of redidual quantizer */
/* I/O  Pointer to Silk encoder state               */
/* I    Target max bitrate (bps)                    */
/* **************/
/* Shell coder */
/* **************/
/* Encode quantization indices of excitation */
/* I/O  compressor data structure                   */
/* I    Signal type                                 */
/* I    quantOffsetType                             */
/* I    quantization indices                        */
/* I    Frame length                                */
/* Shell encoder, operates on one shell code frame of 16 pulses */
/* I/O  compressor data structure                   */
/* I    data: nonnegative pulse amplitudes          */
/* Shell decoder, operates on one shell code frame of 16 pulses */
/* O    data: nonnegative pulse amplitudes          */
/* I/O  Compressor data structure                   */
/* I    number of pulses per pulse-subframe         */
/* Gain scalar quantization with hysteresis, uniform on log scale */
/* O    gain indices                                */
/* I/O  gains (quantized out)                       */
/* I/O  last index in previous frame                */
/* I    first gain is delta coded if 1              */
/* I    number of subframes                         */
/* Gains scalar dequantization, uniform on log scale */
/* O    quantized gains                             */
/* I    gain indices                                */
/* I/O  last index in previous frame                */
/* I    first gain is delta coded if 1              */
/* I    number of subframes                          */
/* Compute unique identifier of gain indices vector */
/* O    returns unique identifier of gains          */
/* I    gain indices                                */
/* I    number of subframes                         */
/* Interpolate two vectors */
/* O    interpolated vector                         */
/* I    first vector                                */
/* I    second vector                               */
/* I    interp. factor, weight on 2nd vector        */
/* I    number of parameters                        */
/* LTP tap quantizer */
/* O    Quantized LTP gains             */
/* O    Codebook Index                  */
/* O    Periodicity Index               */
/* I/O  Cumulative max prediction gain  */
/* O    LTP prediction gain             */
/* I    Correlation matrix in Q18       */
/* I    Correlation vector in Q18       */
/* I    Number of samples per subframe  */
/* I    Number of subframes             */
/* I    Run-time architecture           */
/* Entropy constrained matrix-weighted VQ, for a single input data vector */
/* O    index of best codebook vector               */
/* O    best residual energy                        */
/* O    best total bitrate                          */
/* O    sum of absolute LTP coefficients            */
/* I    correlation matrix                          */
/* I    correlation vector                          */
/* I    codebook                                    */
/* I    codebook effective gain                     */
/* I    code length for each codebook vector        */
/* I    number of samples per subframe              */
/* I    maximum sum of absolute LTP coefficients    */
/* I    number of vectors in codebook               */
/* ***********************************/
/* Noise shaping quantization (NSQ) */
/* ***********************************/
/* I    Encoder State                   */
/* I/O  NSQ state                       */
/* I/O  Quantization Indices            */
/* I    Input                           */
/* O    Quantized pulse signal          */
/* I    Short term prediction coefs     */
/* I    Long term prediction coefs      */
/* I  Noise shaping coefs             */
/* I    Long term shaping coefs         */
/* I    Spectral tilt                   */
/* I    Low frequency shaping coefs     */
/* I    Quantization step sizes         */
/* I    Pitch lags                      */
/* I    Rate/distortion tradeoff        */
/* I    LTP state scaling               */
/* Noise shaping using delayed decision */
/* I    Encoder State                   */
/* I/O  NSQ state                       */
/* I/O  Quantization Indices            */
/* I    Input                           */
/* O    Quantized pulse signal          */
/* I    Short term prediction coefs     */
/* I    Long term prediction coefs      */
/* I  Noise shaping coefs             */
/* I    Long term shaping coefs         */
/* I    Spectral tilt                   */
/* I    Low frequency shaping coefs     */
/* I    Quantization step sizes         */
/* I    Pitch lags                      */
/* I    Rate/distortion tradeoff        */
/* I    LTP state scaling               */
/* ***********/
/* Silk VAD */
/* ***********/
/* Initialize the Silk VAD */
/* O    Return value, 0 if success                  */
/* I/O  Pointer to Silk VAD state                   */
/* Get speech activity level in Q8 */
/* **************************************/
/* Get the speech activity level in Q8 */
/* **************************************/
#[no_mangle]

pub unsafe extern "C" fn silk_VAD_GetSA_Q8_c(
    mut psEncC: *mut crate::structs_h::silk_encoder_state,
    mut pIn: *const crate::opus_types_h::opus_int16,
) -> i32
/* I    PCM input                                   */ {
    let mut SA_Q15: i32 = 0;
    let mut pSNR_dB_Q7: i32 = 0;
    let mut input_tilt: i32 = 0;
    let mut decimated_framelength1: i32 = 0;
    let mut decimated_framelength2: i32 = 0;
    let mut decimated_framelength: i32 = 0;
    let mut dec_subframe_length: i32 = 0;
    let mut dec_subframe_offset: i32 = 0;
    let mut SNR_Q7: i32 = 0;
    let mut i: i32 = 0;
    let mut b: i32 = 0;
    let mut s: i32 = 0;
    let mut sumSquared: crate::opus_types_h::opus_int32 = 0;
    let mut smooth_coef_Q16: crate::opus_types_h::opus_int32 = 0;
    let mut HPstateTmp: crate::opus_types_h::opus_int16 = 0;
    let mut X: *mut crate::opus_types_h::opus_int16 = 0 as *mut crate::opus_types_h::opus_int16;
    let mut Xnrg: [crate::opus_types_h::opus_int32; 4] = [0; 4];
    let mut NrgToNoiseRatio_Q8: [crate::opus_types_h::opus_int32; 4] = [0; 4];
    let mut speech_nrg: crate::opus_types_h::opus_int32 = 0;
    let mut x_tmp: crate::opus_types_h::opus_int32 = 0;
    let mut X_offset: [i32; 4] = [0; 4];
    let mut ret: i32 = 0 as i32;
    let mut psSilk_VAD: *mut crate::structs_h::silk_VAD_state = &mut (*psEncC).sVAD;
    /* Safety checks */
    /* **********************/
    /* Filter and Decimate */
    /* **********************/
    decimated_framelength1 = (*psEncC).frame_length >> 1 as i32;
    decimated_framelength2 = (*psEncC).frame_length >> 2 as i32;
    decimated_framelength = (*psEncC).frame_length >> 3 as i32;
    /* Decimate into 4 bands:
    0       L      3L       L              3L                             5L
            -      --       -              --                             --
            8       8       2               4                              4

    [0-1 kHz| temp. |1-2 kHz|    2-4 kHz    |            4-8 kHz           |

    They're arranged to allow the minimal ( frame_length / 4 ) extra
    scratch space during the downsampling process */
    X_offset[0 as i32 as usize] = 0 as i32;
    X_offset[1 as i32 as usize] = decimated_framelength + decimated_framelength2;
    X_offset[2 as i32 as usize] =
        X_offset[1 as i32 as usize] + decimated_framelength;
    X_offset[3 as i32 as usize] =
        X_offset[2 as i32 as usize] + decimated_framelength2;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong).wrapping_mul(
            (X_offset[3 as i32 as usize] + decimated_framelength1) as libc::c_ulong,
        ) as usize,
    );
    X = fresh0.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    /* 0-8 kHz to 0-4 kHz and 4-8 kHz */
    crate::src::opus_1_2_1::silk::ana_filt_bank_1::silk_ana_filt_bank_1(
        pIn,
        &mut *(*psSilk_VAD)
            .AnaState
            .as_mut_ptr()
            .offset(0 as i32 as isize),
        X,
        &mut *X.offset(*X_offset.as_mut_ptr().offset(3 as i32 as isize) as isize),
        (*psEncC).frame_length,
    );
    /* 0-4 kHz to 0-2 kHz and 2-4 kHz */
    crate::src::opus_1_2_1::silk::ana_filt_bank_1::silk_ana_filt_bank_1(
        X,
        &mut *(*psSilk_VAD)
            .AnaState1
            .as_mut_ptr()
            .offset(0 as i32 as isize),
        X,
        &mut *X.offset(*X_offset.as_mut_ptr().offset(2 as i32 as isize) as isize),
        decimated_framelength1,
    );
    /* 0-2 kHz to 0-1 kHz and 1-2 kHz */
    crate::src::opus_1_2_1::silk::ana_filt_bank_1::silk_ana_filt_bank_1(
        X,
        &mut *(*psSilk_VAD)
            .AnaState2
            .as_mut_ptr()
            .offset(0 as i32 as isize),
        X,
        &mut *X.offset(*X_offset.as_mut_ptr().offset(1 as i32 as isize) as isize),
        decimated_framelength2,
    );
    /* ********************************************/
    /* HP filter on lowest band (differentiator) */
    /* ********************************************/
    *X.offset((decimated_framelength - 1 as i32) as isize) =
        (*X.offset((decimated_framelength - 1 as i32) as isize) as i32
            >> 1 as i32) as crate::opus_types_h::opus_int16;
    HPstateTmp = *X.offset((decimated_framelength - 1 as i32) as isize);
    i = decimated_framelength - 1 as i32;
    while i > 0 as i32 {
        *X.offset((i - 1 as i32) as isize) =
            (*X.offset((i - 1 as i32) as isize) as i32 >> 1 as i32)
                as crate::opus_types_h::opus_int16;
        let ref mut fresh1 = *X.offset(i as isize);
        *fresh1 = (*fresh1 as i32
            - *X.offset((i - 1 as i32) as isize) as i32)
            as crate::opus_types_h::opus_int16;
        i -= 1
    }
    let ref mut fresh2 = *X.offset(0 as i32 as isize);
    *fresh2 = (*fresh2 as i32 - (*psSilk_VAD).HPstate as i32)
        as crate::opus_types_h::opus_int16;
    (*psSilk_VAD).HPstate = HPstateTmp;
    /* ************************************/
    /* Calculate the energy in each band */
    /* ************************************/
    b = 0 as i32;
    while b < 4 as i32 {
        /* Find the decimated framelength in the non-uniformly divided bands */
        decimated_framelength = (*psEncC).frame_length
            >> silk_min_int(4 as i32 - b, 4 as i32 - 1 as i32);
        /* Split length into subframe lengths */
        dec_subframe_length = decimated_framelength >> 2 as i32;
        dec_subframe_offset = 0 as i32;
        /* Compute energy per sub-frame */
        /* initialize with summed energy of last subframe */
        Xnrg[b as usize] = (*psSilk_VAD).XnrgSubfr[b as usize];
        s = 0 as i32;
        while s < (1 as i32) << 2 as i32 {
            sumSquared = 0 as i32;
            i = 0 as i32;
            while i < dec_subframe_length {
                /* The energy will be less than dec_subframe_length * ( silk_int16_MIN / 8 ) ^ 2.            */
                /* Therefore we can accumulate with no risk of overflow (unless dec_subframe_length > 128)  */
                x_tmp = *X.offset((X_offset[b as usize] + i + dec_subframe_offset) as isize)
                    as i32
                    >> 3 as i32;
                sumSquared = sumSquared
                    + x_tmp as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                        * x_tmp as crate::opus_types_h::opus_int16
                            as crate::opus_types_h::opus_int32;
                i += 1
            }
            /* Add/saturate summed energy of current subframe */
            if s < ((1 as i32) << 2 as i32) - 1 as i32 {
                Xnrg[b as usize] = if (Xnrg[b as usize] as crate::opus_types_h::opus_uint32)
                    .wrapping_add(sumSquared as crate::opus_types_h::opus_uint32)
                    & 0x80000000 as u32
                    != 0
                {
                    0x7fffffff as i32
                } else {
                    (Xnrg[b as usize]) + sumSquared
                }
            } else {
                /* Look-ahead subframe */
                Xnrg[b as usize] =
                    if (Xnrg[b as usize] as crate::opus_types_h::opus_uint32).wrapping_add(
                        (sumSquared >> 1 as i32) as crate::opus_types_h::opus_uint32,
                    ) & 0x80000000 as u32
                        != 0
                    {
                        0x7fffffff as i32
                    } else {
                        (Xnrg[b as usize]) + (sumSquared >> 1 as i32)
                    }
            }
            dec_subframe_offset += dec_subframe_length;
            s += 1
        }
        (*psSilk_VAD).XnrgSubfr[b as usize] = sumSquared;
        b += 1
    }
    /* *******************/
    /* Noise estimation */
    /* *******************/
    silk_VAD_GetNoiseLevels(
        &mut *Xnrg.as_mut_ptr().offset(0 as i32 as isize)
            as *mut crate::opus_types_h::opus_int32
            as *const crate::opus_types_h::opus_int32,
        psSilk_VAD,
    );
    /* **********************************************/
    /* Signal-plus-noise to noise ratio estimation */
    /* **********************************************/
    sumSquared = 0 as i32;
    input_tilt = 0 as i32;
    b = 0 as i32;
    while b < 4 as i32 {
        speech_nrg = Xnrg[b as usize] - (*psSilk_VAD).NL[b as usize];
        if speech_nrg > 0 as i32 {
            /* Divide, with sufficient resolution */
            if Xnrg[b as usize] as u32 & 0xff800000 as u32
                == 0 as i32 as u32
            {
                NrgToNoiseRatio_Q8[b as usize] =
                    ((Xnrg[b as usize] as crate::opus_types_h::opus_uint32) << 8 as i32)
                        as crate::opus_types_h::opus_int32
                        / ((*psSilk_VAD).NL[b as usize] + 1 as i32)
            } else {
                NrgToNoiseRatio_Q8[b as usize] = Xnrg[b as usize]
                    / (((*psSilk_VAD).NL[b as usize] >> 8 as i32) + 1 as i32)
            }
            /* Convert to log domain */
            SNR_Q7 =
                crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(NrgToNoiseRatio_Q8[b as usize])
                    - 8 as i32 * 128 as i32;
            /* Sum-of-squares */
            sumSquared = sumSquared
                + SNR_Q7 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                    * SNR_Q7 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32; /* Q14 */
            /* Tilt measure */
            if speech_nrg < (1 as i32) << 20 as i32 {
                /* Scale down SNR value for small subband speech energies */
                SNR_Q7 =
                    (((silk_SQRT_APPROX(speech_nrg) as crate::opus_types_h::opus_uint32)
                        << 6 as i32) as crate::opus_types_h::opus_int32
                        as i64
                        * SNR_Q7 as crate::opus_types_h::opus_int16 as i64
                        >> 16 as i32) as crate::opus_types_h::opus_int32
            }
            input_tilt = (input_tilt as i64
                + (tiltWeights[b as usize] as i64
                    * SNR_Q7 as crate::opus_types_h::opus_int16 as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32
        } else {
            NrgToNoiseRatio_Q8[b as usize] = 256 as i32
        }
        b += 1
    }
    /* Mean-of-squares */
    sumSquared = sumSquared / 4 as i32; /* Q14 */
    /* Root-mean-square approximation, scale to dBs, and write to output pointer */
    pSNR_dB_Q7 = (3 as i32 * silk_SQRT_APPROX(sumSquared))
        as crate::opus_types_h::opus_int16 as i32; /* Q7 */
    /* ********************************/
    /* Speech Probability Estimation */
    /* ********************************/
    SA_Q15 = crate::src::opus_1_2_1::silk::sigm_Q15::silk_sigm_Q15(
        (45000 as i32 as i64
            * pSNR_dB_Q7 as crate::opus_types_h::opus_int16 as i64
            >> 16 as i32) as crate::opus_types_h::opus_int32
            - 128 as i32,
    );
    /* *************************/
    /* Frequency Tilt Measure */
    /* *************************/
    (*psEncC).input_tilt_Q15 = (((crate::src::opus_1_2_1::silk::sigm_Q15::silk_sigm_Q15(input_tilt)
        - 16384 as i32)
        as crate::opus_types_h::opus_uint32)
        << 1 as i32) as crate::opus_types_h::opus_int32;
    /* *************************************************/
    /* Scale the sigmoid output based on power levels */
    /* *************************************************/
    speech_nrg = 0 as i32;
    b = 0 as i32;
    while b < 4 as i32 {
        /* Accumulate signal-without-noise energies, higher frequency bands have more weight */
        speech_nrg += (b + 1 as i32)
            * (Xnrg[b as usize] - (*psSilk_VAD).NL[b as usize] >> 4 as i32);
        b += 1
    }
    /* Power scaling */
    if speech_nrg <= 0 as i32 {
        SA_Q15 = SA_Q15 >> 1 as i32
    } else if speech_nrg < 32768 as i32 {
        if (*psEncC).frame_length == 10 as i32 * (*psEncC).fs_kHz {
            speech_nrg = (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32
                >> 16 as i32
                > 0x7fffffff as i32 >> 16 as i32
            {
                (if speech_nrg
                    > 0x80000000 as u32 as crate::opus_types_h::opus_int32
                        >> 16 as i32
                {
                    (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                        >> 16 as i32
                } else {
                    (if speech_nrg < 0x7fffffff as i32 >> 16 as i32 {
                        (0x7fffffff as i32) >> 16 as i32
                    } else {
                        speech_nrg
                    })
                })
            } else {
                (if speech_nrg > 0x7fffffff as i32 >> 16 as i32 {
                    (0x7fffffff as i32) >> 16 as i32
                } else {
                    (if speech_nrg
                        < 0x80000000 as u32 as crate::opus_types_h::opus_int32
                            >> 16 as i32
                    {
                        (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                            >> 16 as i32
                    } else {
                        speech_nrg
                    })
                })
            }) as crate::opus_types_h::opus_uint32)
                << 16 as i32) as crate::opus_types_h::opus_int32
        } else {
            speech_nrg = (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32
                >> 15 as i32
                > 0x7fffffff as i32 >> 15 as i32
            {
                (if speech_nrg
                    > 0x80000000 as u32 as crate::opus_types_h::opus_int32
                        >> 15 as i32
                {
                    (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                        >> 15 as i32
                } else {
                    (if speech_nrg < 0x7fffffff as i32 >> 15 as i32 {
                        (0x7fffffff as i32) >> 15 as i32
                    } else {
                        speech_nrg
                    })
                })
            } else {
                (if speech_nrg > 0x7fffffff as i32 >> 15 as i32 {
                    (0x7fffffff as i32) >> 15 as i32
                } else {
                    (if speech_nrg
                        < 0x80000000 as u32 as crate::opus_types_h::opus_int32
                            >> 15 as i32
                    {
                        (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                            >> 15 as i32
                    } else {
                        speech_nrg
                    })
                })
            }) as crate::opus_types_h::opus_uint32)
                << 15 as i32) as crate::opus_types_h::opus_int32
        }
        /* square-root */
        speech_nrg = silk_SQRT_APPROX(speech_nrg);
        SA_Q15 = ((32768 as i32 + speech_nrg) as i64
            * SA_Q15 as crate::opus_types_h::opus_int16 as i64
            >> 16 as i32) as crate::opus_types_h::opus_int32
    }
    /* Copy the resulting speech activity in Q8 */
    (*psEncC).speech_activity_Q8 = silk_min_int(SA_Q15 >> 7 as i32, 0xff as i32);
    /* **********************************/
    /* Energy Level and SNR estimation */
    /* **********************************/
    /* Smoothing coefficient */
    smooth_coef_Q16 = (4096 as i32 as i64
        * (SA_Q15 as i64
            * SA_Q15 as crate::opus_types_h::opus_int16 as i64
            >> 16 as i32) as crate::opus_types_h::opus_int32
            as crate::opus_types_h::opus_int16 as i64
        >> 16 as i32) as crate::opus_types_h::opus_int32;
    if (*psEncC).frame_length == 10 as i32 * (*psEncC).fs_kHz {
        smooth_coef_Q16 >>= 1 as i32
    }
    b = 0 as i32;
    while b < 4 as i32 {
        /* compute smoothed energy-to-noise ratio per band */
        (*psSilk_VAD).NrgRatioSmth_Q8[b as usize] =
            ((*psSilk_VAD).NrgRatioSmth_Q8[b as usize] as i64
                + ((NrgToNoiseRatio_Q8[b as usize] - (*psSilk_VAD).NrgRatioSmth_Q8[b as usize])
                    as i64
                    * smooth_coef_Q16 as crate::opus_types_h::opus_int16 as i64
                    >> 16 as i32)) as crate::opus_types_h::opus_int32;
        /* signal to noise ratio in dB per band */
        SNR_Q7 = 3 as i32
            * (crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(
                (*psSilk_VAD).NrgRatioSmth_Q8[b as usize],
            ) - 8 as i32 * 128 as i32);
        /* quality = sigmoid( 0.25 * ( SNR_dB - 16 ) ); */
        (*psEncC).input_quality_bands_Q15[b as usize] =
            crate::src::opus_1_2_1::silk::sigm_Q15::silk_sigm_Q15(
                SNR_Q7 - 16 as i32 * 128 as i32 >> 4 as i32,
            );
        b += 1
    }
    return ret;
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
/* Silk VAD noise level estimation */
/* *************************/
/* Noise level estimation */
/* *************************/
#[inline]

unsafe extern "C" fn silk_VAD_GetNoiseLevels(
    mut pX: *const crate::opus_types_h::opus_int32,
    mut psSilk_VAD: *mut crate::structs_h::silk_VAD_state,
)
/* I/O  Pointer to Silk VAD state                   */
{
    let mut k: i32 = 0;
    let mut nl: crate::opus_types_h::opus_int32 = 0;
    let mut nrg: crate::opus_types_h::opus_int32 = 0;
    let mut inv_nrg: crate::opus_types_h::opus_int32 = 0;
    let mut coef: i32 = 0;
    let mut min_coef: i32 = 0;
    /* Initially faster smoothing */
    if (*psSilk_VAD).counter < 1000 as i32 {
        /* 1000 = 20 sec */
        min_coef =
            0x7fff as i32 / (((*psSilk_VAD).counter >> 4 as i32) + 1 as i32)
    } else {
        min_coef = 0 as i32
    }
    k = 0 as i32;
    while k < 4 as i32 {
        /* Get old noise level estimate for current band */
        nl = (*psSilk_VAD).NL[k as usize];
        /* Add bias */
        nrg = if (*pX.offset(k as isize) as crate::opus_types_h::opus_uint32).wrapping_add(
            (*psSilk_VAD).NoiseLevelBias[k as usize] as crate::opus_types_h::opus_uint32,
        ) & 0x80000000 as u32
            != 0
        {
            0x7fffffff as i32
        } else {
            (*pX.offset(k as isize)) + (*psSilk_VAD).NoiseLevelBias[k as usize]
        };
        /* Invert energies */
        inv_nrg = 0x7fffffff as i32 / nrg;
        /* Less update when subband energy is high */
        if nrg
            > ((nl as crate::opus_types_h::opus_uint32) << 3 as i32)
                as crate::opus_types_h::opus_int32
        {
            coef = 1024 as i32 >> 3 as i32
        } else if nrg < nl {
            coef = 1024 as i32
        } else {
            coef = ((inv_nrg as i64 * nl as i64 >> 16 as i32)
                as crate::opus_types_h::opus_int32 as i64
                * ((1024 as i32) << 1 as i32) as crate::opus_types_h::opus_int16
                    as i64
                >> 16 as i32) as crate::opus_types_h::opus_int32
        }
        /* Initially faster smoothing */
        coef = silk_max_int(coef, min_coef);
        /* Smooth inverse energies */
        (*psSilk_VAD).inv_NL[k as usize] = ((*psSilk_VAD).inv_NL[k as usize] as i64
            + ((inv_nrg - (*psSilk_VAD).inv_NL[k as usize]) as i64
                * coef as crate::opus_types_h::opus_int16 as i64
                >> 16 as i32))
            as crate::opus_types_h::opus_int32;
        /* Compute noise level by inverting again */
        nl = 0x7fffffff as i32 / (*psSilk_VAD).inv_NL[k as usize];
        /* Limit noise levels (guarantee 7 bits of head room) */
        nl = if nl < 0xffffff as i32 {
            nl
        } else {
            0xffffff as i32
        };
        /* Store as part of state */
        (*psSilk_VAD).NL[k as usize] = nl;
        k += 1
    }
    /* Increment frame counter */
    (*psSilk_VAD).counter += 1;
}

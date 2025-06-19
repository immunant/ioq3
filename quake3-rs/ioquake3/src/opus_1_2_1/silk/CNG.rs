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

    use crate::src::opus_1_2_1::silk::CNG::macros_h::silk_CLZ32;
    use crate::src::opus_1_2_1::silk::CNG::SigProc_FIX_h::silk_ROR32;
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
pub use crate::src::opus_1_2_1::silk::CNG::macros_h::silk_CLZ32;

pub use crate::structs_h::silk_CNG_struct;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_PLC_struct;
pub use crate::structs_h::silk_decoder_control;
pub use crate::structs_h::silk_decoder_state;
pub use crate::structs_h::SideInfoIndices;

pub use crate::src::opus_1_2_1::silk::CNG::Inlines_h::silk_CLZ_FRAC;
pub use crate::src::opus_1_2_1::silk::CNG::Inlines_h::silk_SQRT_APPROX;
pub use crate::src::opus_1_2_1::silk::CNG::SigProc_FIX_h::silk_ROR32;
pub use crate::src::opus_1_2_1::silk::NLSF2A::silk_NLSF2A;
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
/* Generates excitation for CNG LPC synthesis */
#[inline]

unsafe extern "C" fn silk_CNG_exc(
    mut exc_Q14: *mut crate::opus_types_h::opus_int32,
    mut exc_buf_Q14: *mut crate::opus_types_h::opus_int32,
    mut length: i32,
    mut rand_seed: *mut crate::opus_types_h::opus_int32,
)
/* I/O  Seed to random index generator              */
{
    let mut seed: crate::opus_types_h::opus_int32 = 0;
    let mut i: i32 = 0;
    let mut idx: i32 = 0;
    let mut exc_mask: i32 = 0;
    exc_mask = 255 as i32;
    while exc_mask > length {
        exc_mask = exc_mask >> 1 as i32
    }
    seed = *rand_seed;
    i = 0 as i32;
    while i < length {
        seed = (907633515 as i32 as crate::opus_types_h::opus_uint32).wrapping_add(
            (seed as crate::opus_types_h::opus_uint32)
                .wrapping_mul(196314165 as i32 as crate::opus_types_h::opus_uint32),
        ) as crate::opus_types_h::opus_int32;
        idx = seed >> 24 as i32 & exc_mask;
        *exc_Q14.offset(i as isize) = *exc_buf_Q14.offset(idx as isize);
        i += 1
    }
    *rand_seed = seed;
}
#[no_mangle]

pub unsafe extern "C" fn silk_CNG_Reset(mut psDec: *mut crate::structs_h::silk_decoder_state)
/* I/O  Decoder state                               */
{
    let mut i: i32 = 0;
    let mut NLSF_step_Q15: i32 = 0;
    let mut NLSF_acc_Q15: i32 = 0;
    NLSF_step_Q15 = 0x7fff as i32 / ((*psDec).LPC_order + 1 as i32);
    NLSF_acc_Q15 = 0 as i32;
    i = 0 as i32;
    while i < (*psDec).LPC_order {
        NLSF_acc_Q15 += NLSF_step_Q15;
        (*psDec).sCNG.CNG_smth_NLSF_Q15[i as usize] =
            NLSF_acc_Q15 as crate::opus_types_h::opus_int16;
        i += 1
    }
    (*psDec).sCNG.CNG_smth_Gain_Q16 = 0 as i32;
    (*psDec).sCNG.rand_seed = 3176576 as i32;
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
/* O    Return value, 0 if success                  */
/* I/O  Encoder state                               */
/* I    PCM input                                   */
/* Low-pass filter with variable cutoff frequency based on  */
/* piece-wise linear interpolation between elliptic filters */
/* Start by setting transition_frame_no = 1;                */
/* I/O  LP filter state                             */
/* I/O  Low-pass filtered output signal             */
/* I    Frame length                                */
/* *****************/
/* NLSF Quantizer */
/* *****************/
/* Limit, stabilize, convert and quantize NLSFs */
/* I/O  Encoder state                               */
/* O    Prediction coefficients                     */
/* I/O  Normalized LSFs (quant out) (0 - (2^15-1))  */
/* I    Previous Normalized LSFs (0 - (2^15-1))     */
/* O    Returns RD value in Q25                     */
/* I    Codebook path vector [ LPC_ORDER + 1 ]      */
/* I/O  Quantized NLSF vector [ LPC_ORDER ]         */
/* I    Codebook object                             */
/* I    NLSF weight vector [ LPC_ORDER ]            */
/* I    Rate weight for the RD optimization         */
/* I    Max survivors after first stage             */
/* I    Signal type: 0/1/2                          */
/* Compute quantization errors for an LPC_order element input vector for a VQ codebook */
/* O    Quantization errors [K]                     */
/* I    Input vectors to be quantized [LPC_order]   */
/* I    Codebook vectors [K*LPC_order]              */
/* I    Codebook weights [K*LPC_order]              */
/* I    Number of codebook vectors                  */
/* I    Number of LPCs                              */
/* Delayed-decision quantizer for NLSF residuals */
/* O    Returns RD value in Q25                     */
/* O    Quantization indices [ order ]              */
/* I    Input [ order ]                             */
/* I    Weights [ order ]                           */
/* I    Backward predictor coefs [ order ]          */
/* I    Indices to entropy coding tables [ order ]  */
/* I    Rates []                                    */
/* I    Quantization step size                      */
/* I    Inverse quantization step size              */
/* I    R/D tradeoff                                */
/* I    Number of input values                      */
/* Unpack predictor values and indices for entropy coding tables */
/* O    Indices to entropy tables [ LPC_ORDER ]     */
/* O    LSF predictor [ LPC_ORDER ]                 */
/* I    Codebook object                             */
/* I    Index of vector in first LSF codebook       */
/* **********************/
/* NLSF vector decoder */
/* **********************/
/* O    Quantized NLSF vector [ LPC_ORDER ]         */
/* I    Codebook path vector [ LPC_ORDER + 1 ]      */
/* I    Codebook object                             */
/* ***************************************************/
/* Decoder Functions                                */
/* ***************************************************/
/* I/O  Decoder state pointer                       */
/* Set decoder sampling rate */
/* I/O  Decoder state pointer                       */
/* I    Sampling frequency (kHz)                    */
/* I    API Sampling frequency (Hz)                 */
/* ***************/
/* Decode frame */
/* ***************/
/* I/O  Pointer to Silk decoder state               */
/* I/O  Compressor data structure                   */
/* O    Pointer to output speech frame              */
/* O    Pointer to size of output frame             */
/* I    0: no loss, 1 loss, 2 decode fec            */
/* I    The type of conditional coding to use       */
/* I    Run-time architecture                       */
/* Decode indices from bitstream */
/* I/O  State                                       */
/* I/O  Compressor data structure                   */
/* I    Frame number                                */
/* I    Flag indicating LBRR data is being decoded  */
/* I    The type of conditional coding to use       */
/* Decode parameters from payload */
/* I/O  State                                       */
/* I/O  Decoder control                             */
/* I    The type of conditional coding to use       */
/* Core decoder. Performs inverse NSQ operation LTP + LPC */
/* I/O  Decoder state                               */
/* I    Decoder control                             */
/* O    Decoded speech                              */
/* I    Pulse signal                                */
/* I    Run-time architecture                       */
/* Decode quantization indices of excitation (Shell coding) */
/* I/O  Compressor data structure                   */
/* O    Excitation signal                           */
/* I    Sigtype                                     */
/* I    quantOffsetType                             */
/* I    Frame length                                */
/* *****************/
/* CNG */
/* *****************/
/* Reset CNG */
/* I/O  Decoder state                               */
/* Updates CNG estimate, and applies the CNG when packet was lost */
/* Updates CNG estimate, and applies the CNG when packet was lost   */
#[no_mangle]

pub unsafe extern "C" fn silk_CNG(
    mut psDec: *mut crate::structs_h::silk_decoder_state,
    mut psDecCtrl: *mut crate::structs_h::silk_decoder_control,
    mut frame: *mut crate::opus_types_h::opus_int16,
    mut length: i32,
)
/* I    Length of residual                          */
{
    let mut i: i32 = 0;
    let mut subfr: i32 = 0;
    let mut LPC_pred_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut max_Gain_Q16: crate::opus_types_h::opus_int32 = 0;
    let mut gain_Q16: crate::opus_types_h::opus_int32 = 0;
    let mut gain_Q10: crate::opus_types_h::opus_int32 = 0;
    let mut A_Q12: [crate::opus_types_h::opus_int16; 16] = [0; 16];
    let mut psCNG: *mut crate::structs_h::silk_CNG_struct = &mut (*psDec).sCNG;
    if (*psDec).fs_kHz != (*psCNG).fs_kHz {
        /* Reset state */
        silk_CNG_Reset(psDec);
        (*psCNG).fs_kHz = (*psDec).fs_kHz
    }
    if (*psDec).lossCnt == 0 as i32 && (*psDec).prevSignalType == 0 as i32 {
        /* Update CNG parameters */
        /* Smoothing of LSF's  */
        i = 0 as i32;
        while i < (*psDec).LPC_order {
            (*psCNG).CNG_smth_NLSF_Q15[i as usize] = ((*psCNG).CNG_smth_NLSF_Q15[i as usize]
                as i32
                + (((*psDec).prevNLSF_Q15[i as usize] as crate::opus_types_h::opus_int32
                    - (*psCNG).CNG_smth_NLSF_Q15[i as usize] as crate::opus_types_h::opus_int32)
                    as i64
                    * 16348 as i32 as crate::opus_types_h::opus_int16 as i64
                    >> 16 as i32) as crate::opus_types_h::opus_int32)
                as crate::opus_types_h::opus_int16;
            i += 1
        }
        /* Find the subframe with the highest gain */
        max_Gain_Q16 = 0 as i32;
        subfr = 0 as i32;
        i = 0 as i32;
        while i < (*psDec).nb_subfr {
            if (*psDecCtrl).Gains_Q16[i as usize] > max_Gain_Q16 {
                max_Gain_Q16 = (*psDecCtrl).Gains_Q16[i as usize];
                subfr = i
            }
            i += 1
        }
        /* Update CNG excitation buffer with excitation from this subframe */
        crate::stdlib::memmove(
            &mut *(*psCNG)
                .CNG_exc_buf_Q14
                .as_mut_ptr()
                .offset((*psDec).subfr_length as isize)
                as *mut crate::opus_types_h::opus_int32 as *mut libc::c_void,
            (*psCNG).CNG_exc_buf_Q14.as_mut_ptr() as *const libc::c_void,
            ((((*psDec).nb_subfr - 1 as i32) * (*psDec).subfr_length) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<crate::opus_types_h::opus_int32>() as libc::c_ulong
                ),
        );
        crate::stdlib::memcpy(
            (*psCNG).CNG_exc_buf_Q14.as_mut_ptr() as *mut libc::c_void,
            &mut *(*psDec)
                .exc_Q14
                .as_mut_ptr()
                .offset((subfr * (*psDec).subfr_length) as isize)
                as *mut crate::opus_types_h::opus_int32 as *const libc::c_void,
            ((*psDec).subfr_length as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::opus_types_h::opus_int32,
            >() as libc::c_ulong),
        );
        /* Smooth gains */
        i = 0 as i32;
        while i < (*psDec).nb_subfr {
            (*psCNG).CNG_smth_Gain_Q16 +=
                (((*psDecCtrl).Gains_Q16[i as usize] - (*psCNG).CNG_smth_Gain_Q16)
                    as i64
                    * 4634 as i32 as crate::opus_types_h::opus_int16 as i64
                    >> 16 as i32) as crate::opus_types_h::opus_int32;
            i += 1
        }
    }
    /* Add CNG when packet is lost or during DTX */
    if (*psDec).lossCnt != 0 {
        let mut CNG_sig_Q14: *mut crate::opus_types_h::opus_int32 =
            0 as *mut crate::opus_types_h::opus_int32;
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<crate::opus_types_h::opus_int32>() as libc::c_ulong)
                .wrapping_mul((length + 16 as i32) as libc::c_ulong) as usize,
        );
        CNG_sig_Q14 = fresh0.as_mut_ptr() as *mut crate::opus_types_h::opus_int32;
        /* Generate CNG excitation */
        gain_Q16 = ((*psDec).sPLC.randScale_Q14 as i64
            * (*psDec).sPLC.prevGain_Q16[1 as i32 as usize] as i64
            >> 16 as i32) as crate::opus_types_h::opus_int32;
        if gain_Q16 >= (1 as i32) << 21 as i32
            || (*psCNG).CNG_smth_Gain_Q16 > (1 as i32) << 23 as i32
        {
            gain_Q16 = (gain_Q16 >> 16 as i32) * (gain_Q16 >> 16 as i32);
            gain_Q16 = ((*psCNG).CNG_smth_Gain_Q16 >> 16 as i32)
                * ((*psCNG).CNG_smth_Gain_Q16 >> 16 as i32)
                - ((gain_Q16 as crate::opus_types_h::opus_uint32) << 5 as i32)
                    as crate::opus_types_h::opus_int32;
            gain_Q16 = ((silk_SQRT_APPROX(gain_Q16) as crate::opus_types_h::opus_uint32)
                << 16 as i32) as crate::opus_types_h::opus_int32
        } else {
            gain_Q16 = (gain_Q16 as i64 * gain_Q16 as i64
                >> 16 as i32) as crate::opus_types_h::opus_int32;
            gain_Q16 = ((*psCNG).CNG_smth_Gain_Q16 as i64
                * (*psCNG).CNG_smth_Gain_Q16 as i64
                >> 16 as i32) as crate::opus_types_h::opus_int32
                - ((gain_Q16 as crate::opus_types_h::opus_uint32) << 5 as i32)
                    as crate::opus_types_h::opus_int32;
            gain_Q16 = ((silk_SQRT_APPROX(gain_Q16) as crate::opus_types_h::opus_uint32)
                << 8 as i32) as crate::opus_types_h::opus_int32
        }
        gain_Q10 = gain_Q16 >> 6 as i32;
        silk_CNG_exc(
            CNG_sig_Q14.offset(16 as i32 as isize),
            (*psCNG).CNG_exc_buf_Q14.as_mut_ptr(),
            length,
            &mut (*psCNG).rand_seed,
        );
        /* Convert CNG NLSF to filter representation */
        crate::src::opus_1_2_1::silk::NLSF2A::silk_NLSF2A(
            A_Q12.as_mut_ptr(),
            (*psCNG).CNG_smth_NLSF_Q15.as_mut_ptr(),
            (*psDec).LPC_order,
            (*psDec).arch,
        );
        /* Generate CNG signal, by synthesis filtering */
        crate::stdlib::memcpy(
            CNG_sig_Q14 as *mut libc::c_void,
            (*psCNG).CNG_synth_state.as_mut_ptr() as *const libc::c_void,
            (16 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::opus_types_h::opus_int32,
            >() as libc::c_ulong),
        );
        i = 0 as i32;
        while i < length {
            /* Avoids introducing a bias because silk_SMLAWB() always rounds to -inf */
            LPC_pred_Q10 = (*psDec).LPC_order >> 1 as i32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*CNG_sig_Q14.offset((16 as i32 + i - 1 as i32) as isize)
                    as i64
                    * A_Q12[0 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*CNG_sig_Q14.offset((16 as i32 + i - 2 as i32) as isize)
                    as i64
                    * A_Q12[1 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*CNG_sig_Q14.offset((16 as i32 + i - 3 as i32) as isize)
                    as i64
                    * A_Q12[2 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*CNG_sig_Q14.offset((16 as i32 + i - 4 as i32) as isize)
                    as i64
                    * A_Q12[3 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*CNG_sig_Q14.offset((16 as i32 + i - 5 as i32) as isize)
                    as i64
                    * A_Q12[4 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*CNG_sig_Q14.offset((16 as i32 + i - 6 as i32) as isize)
                    as i64
                    * A_Q12[5 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*CNG_sig_Q14.offset((16 as i32 + i - 7 as i32) as isize)
                    as i64
                    * A_Q12[6 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*CNG_sig_Q14.offset((16 as i32 + i - 8 as i32) as isize)
                    as i64
                    * A_Q12[7 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*CNG_sig_Q14.offset((16 as i32 + i - 9 as i32) as isize)
                    as i64
                    * A_Q12[8 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            LPC_pred_Q10 = (LPC_pred_Q10 as i64
                + (*CNG_sig_Q14.offset((16 as i32 + i - 10 as i32) as isize)
                    as i64
                    * A_Q12[9 as i32 as usize] as i64
                    >> 16 as i32))
                as crate::opus_types_h::opus_int32;
            if (*psDec).LPC_order == 16 as i32 {
                LPC_pred_Q10 = (LPC_pred_Q10 as i64
                    + (*CNG_sig_Q14.offset((16 as i32 + i - 11 as i32) as isize)
                        as i64
                        * A_Q12[10 as i32 as usize] as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LPC_pred_Q10 = (LPC_pred_Q10 as i64
                    + (*CNG_sig_Q14.offset((16 as i32 + i - 12 as i32) as isize)
                        as i64
                        * A_Q12[11 as i32 as usize] as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LPC_pred_Q10 = (LPC_pred_Q10 as i64
                    + (*CNG_sig_Q14.offset((16 as i32 + i - 13 as i32) as isize)
                        as i64
                        * A_Q12[12 as i32 as usize] as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LPC_pred_Q10 = (LPC_pred_Q10 as i64
                    + (*CNG_sig_Q14.offset((16 as i32 + i - 14 as i32) as isize)
                        as i64
                        * A_Q12[13 as i32 as usize] as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LPC_pred_Q10 = (LPC_pred_Q10 as i64
                    + (*CNG_sig_Q14.offset((16 as i32 + i - 15 as i32) as isize)
                        as i64
                        * A_Q12[14 as i32 as usize] as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32;
                LPC_pred_Q10 = (LPC_pred_Q10 as i64
                    + (*CNG_sig_Q14.offset((16 as i32 + i - 16 as i32) as isize)
                        as i64
                        * A_Q12[15 as i32 as usize] as i64
                        >> 16 as i32))
                    as crate::opus_types_h::opus_int32
            }
            /* Update states */
            *CNG_sig_Q14.offset((16 as i32 + i) as isize) = if (*CNG_sig_Q14
                .offset((16 as i32 + i) as isize)
                as crate::opus_types_h::opus_uint32)
                .wrapping_add(
                    (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32
                        >> 4 as i32
                        > 0x7fffffff as i32 >> 4 as i32
                    {
                        (if LPC_pred_Q10
                            > 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                >> 4 as i32
                        {
                            (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                >> 4 as i32
                        } else {
                            (if LPC_pred_Q10 < 0x7fffffff as i32 >> 4 as i32 {
                                (0x7fffffff as i32) >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    } else {
                        (if LPC_pred_Q10 > 0x7fffffff as i32 >> 4 as i32 {
                            (0x7fffffff as i32) >> 4 as i32
                        } else {
                            (if LPC_pred_Q10
                                < 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                    >> 4 as i32
                            {
                                (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                    >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    }) as crate::opus_types_h::opus_uint32)
                        << 4 as i32) as crate::opus_types_h::opus_int32
                        as crate::opus_types_h::opus_uint32,
                )
                & 0x80000000 as u32
                == 0 as i32 as u32
            {
                if (*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                    & (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32
                        >> 4 as i32
                        > 0x7fffffff as i32 >> 4 as i32
                    {
                        (if LPC_pred_Q10
                            > 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                >> 4 as i32
                        {
                            (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                >> 4 as i32
                        } else {
                            (if LPC_pred_Q10 < 0x7fffffff as i32 >> 4 as i32 {
                                (0x7fffffff as i32) >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    } else {
                        (if LPC_pred_Q10 > 0x7fffffff as i32 >> 4 as i32 {
                            (0x7fffffff as i32) >> 4 as i32
                        } else {
                            (if LPC_pred_Q10
                                < 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                    >> 4 as i32
                            {
                                (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                    >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    }) as crate::opus_types_h::opus_uint32)
                        << 4 as i32)
                        as crate::opus_types_h::opus_int32) as u32
                    & 0x80000000 as u32
                    != 0 as i32 as u32
                {
                    0x80000000 as u32 as crate::opus_types_h::opus_int32
                } else {
                    (*CNG_sig_Q14.offset((16 as i32 + i) as isize))
                        + (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32
                            >> 4 as i32
                            > 0x7fffffff as i32 >> 4 as i32
                        {
                            (if LPC_pred_Q10
                                > 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                    >> 4 as i32
                            {
                                (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                    >> 4 as i32
                            } else {
                                (if LPC_pred_Q10 < 0x7fffffff as i32 >> 4 as i32 {
                                    (0x7fffffff as i32) >> 4 as i32
                                } else {
                                    LPC_pred_Q10
                                })
                            })
                        } else {
                            (if LPC_pred_Q10 > 0x7fffffff as i32 >> 4 as i32 {
                                (0x7fffffff as i32) >> 4 as i32
                            } else {
                                (if LPC_pred_Q10
                                    < 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                        >> 4 as i32
                                {
                                    (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                        >> 4 as i32
                                } else {
                                    LPC_pred_Q10
                                })
                            })
                        }) as crate::opus_types_h::opus_uint32)
                            << 4 as i32)
                            as crate::opus_types_h::opus_int32
                }
            } else if (*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                | (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32
                    >> 4 as i32
                    > 0x7fffffff as i32 >> 4 as i32
                {
                    (if LPC_pred_Q10
                        > 0x80000000 as u32 as crate::opus_types_h::opus_int32
                            >> 4 as i32
                    {
                        (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                            >> 4 as i32
                    } else {
                        (if LPC_pred_Q10 < 0x7fffffff as i32 >> 4 as i32 {
                            (0x7fffffff as i32) >> 4 as i32
                        } else {
                            LPC_pred_Q10
                        })
                    })
                } else {
                    (if LPC_pred_Q10 > 0x7fffffff as i32 >> 4 as i32 {
                        (0x7fffffff as i32) >> 4 as i32
                    } else {
                        (if LPC_pred_Q10
                            < 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                >> 4 as i32
                        {
                            (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                >> 4 as i32
                        } else {
                            LPC_pred_Q10
                        })
                    })
                }) as crate::opus_types_h::opus_uint32)
                    << 4 as i32) as crate::opus_types_h::opus_int32)
                as u32
                & 0x80000000 as u32
                == 0 as i32 as u32
            {
                0x7fffffff as i32
            } else {
                (*CNG_sig_Q14.offset((16 as i32 + i) as isize))
                    + (((if 0x80000000 as u32 as crate::opus_types_h::opus_int32
                        >> 4 as i32
                        > 0x7fffffff as i32 >> 4 as i32
                    {
                        (if LPC_pred_Q10
                            > 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                >> 4 as i32
                        {
                            (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                >> 4 as i32
                        } else {
                            (if LPC_pred_Q10 < 0x7fffffff as i32 >> 4 as i32 {
                                (0x7fffffff as i32) >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    } else {
                        (if LPC_pred_Q10 > 0x7fffffff as i32 >> 4 as i32 {
                            (0x7fffffff as i32) >> 4 as i32
                        } else {
                            (if LPC_pred_Q10
                                < 0x80000000 as u32 as crate::opus_types_h::opus_int32
                                    >> 4 as i32
                            {
                                (0x80000000 as u32 as crate::opus_types_h::opus_int32)
                                    >> 4 as i32
                            } else {
                                LPC_pred_Q10
                            })
                        })
                    }) as crate::opus_types_h::opus_uint32)
                        << 4 as i32)
                        as crate::opus_types_h::opus_int32
            };
            /* Scale with Gain and add to input signal */
            *frame.offset(i as isize) = if *frame.offset(i as isize)
                as crate::opus_types_h::opus_int32
                + (if (if 8 as i32 == 1 as i32 {
                    ((*CNG_sig_Q14.offset((16 as i32 + i) as isize) as i64
                        * gain_Q10 as i64
                        >> 16 as i32)
                        as crate::opus_types_h::opus_int32
                        >> 1 as i32)
                        + ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                            as i64
                            * gain_Q10 as i64
                            >> 16 as i32)
                            as crate::opus_types_h::opus_int32
                            & 1 as i32)
                } else {
                    (((*CNG_sig_Q14.offset((16 as i32 + i) as isize) as i64
                        * gain_Q10 as i64
                        >> 16 as i32)
                        as crate::opus_types_h::opus_int32
                        >> 8 as i32 - 1 as i32)
                        + 1 as i32)
                        >> 1 as i32
                }) > 0x7fff as i32
                {
                    0x7fff as i32
                } else {
                    (if (if 8 as i32 == 1 as i32 {
                        ((*CNG_sig_Q14.offset((16 as i32 + i) as isize) as i64
                            * gain_Q10 as i64
                            >> 16 as i32)
                            as crate::opus_types_h::opus_int32
                            >> 1 as i32)
                            + ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                as i64
                                * gain_Q10 as i64
                                >> 16 as i32)
                                as crate::opus_types_h::opus_int32
                                & 1 as i32)
                    } else {
                        (((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                            as i64
                            * gain_Q10 as i64
                            >> 16 as i32)
                            as crate::opus_types_h::opus_int32
                            >> 8 as i32 - 1 as i32)
                            + 1 as i32)
                            >> 1 as i32
                    }) < 0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
                    {
                        0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
                    } else {
                        (if 8 as i32 == 1 as i32 {
                            ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                as i64
                                * gain_Q10 as i64
                                >> 16 as i32)
                                as crate::opus_types_h::opus_int32
                                >> 1 as i32)
                                + ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                    as i64
                                    * gain_Q10 as i64
                                    >> 16 as i32)
                                    as crate::opus_types_h::opus_int32
                                    & 1 as i32)
                        } else {
                            (((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                as i64
                                * gain_Q10 as i64
                                >> 16 as i32)
                                as crate::opus_types_h::opus_int32
                                >> 8 as i32 - 1 as i32)
                                + 1 as i32)
                                >> 1 as i32
                        })
                    })
                })
                > 0x7fff as i32
            {
                0x7fff as i32
            } else if *frame.offset(i as isize) as crate::opus_types_h::opus_int32
                + (if (if 8 as i32 == 1 as i32 {
                    ((*CNG_sig_Q14.offset((16 as i32 + i) as isize) as i64
                        * gain_Q10 as i64
                        >> 16 as i32)
                        as crate::opus_types_h::opus_int32
                        >> 1 as i32)
                        + ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                            as i64
                            * gain_Q10 as i64
                            >> 16 as i32)
                            as crate::opus_types_h::opus_int32
                            & 1 as i32)
                } else {
                    (((*CNG_sig_Q14.offset((16 as i32 + i) as isize) as i64
                        * gain_Q10 as i64
                        >> 16 as i32)
                        as crate::opus_types_h::opus_int32
                        >> 8 as i32 - 1 as i32)
                        + 1 as i32)
                        >> 1 as i32
                }) > 0x7fff as i32
                {
                    0x7fff as i32
                } else {
                    (if (if 8 as i32 == 1 as i32 {
                        ((*CNG_sig_Q14.offset((16 as i32 + i) as isize) as i64
                            * gain_Q10 as i64
                            >> 16 as i32)
                            as crate::opus_types_h::opus_int32
                            >> 1 as i32)
                            + ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                as i64
                                * gain_Q10 as i64
                                >> 16 as i32)
                                as crate::opus_types_h::opus_int32
                                & 1 as i32)
                    } else {
                        (((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                            as i64
                            * gain_Q10 as i64
                            >> 16 as i32)
                            as crate::opus_types_h::opus_int32
                            >> 8 as i32 - 1 as i32)
                            + 1 as i32)
                            >> 1 as i32
                    }) < 0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
                    {
                        0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
                    } else {
                        (if 8 as i32 == 1 as i32 {
                            ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                as i64
                                * gain_Q10 as i64
                                >> 16 as i32)
                                as crate::opus_types_h::opus_int32
                                >> 1 as i32)
                                + ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                    as i64
                                    * gain_Q10 as i64
                                    >> 16 as i32)
                                    as crate::opus_types_h::opus_int32
                                    & 1 as i32)
                        } else {
                            (((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                as i64
                                * gain_Q10 as i64
                                >> 16 as i32)
                                as crate::opus_types_h::opus_int32
                                >> 8 as i32 - 1 as i32)
                                + 1 as i32)
                                >> 1 as i32
                        })
                    })
                })
                < 0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
            {
                0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
            } else {
                (*frame.offset(i as isize) as crate::opus_types_h::opus_int32)
                    + (if (if 8 as i32 == 1 as i32 {
                        ((*CNG_sig_Q14.offset((16 as i32 + i) as isize) as i64
                            * gain_Q10 as i64
                            >> 16 as i32)
                            as crate::opus_types_h::opus_int32
                            >> 1 as i32)
                            + ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                as i64
                                * gain_Q10 as i64
                                >> 16 as i32)
                                as crate::opus_types_h::opus_int32
                                & 1 as i32)
                    } else {
                        (((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                            as i64
                            * gain_Q10 as i64
                            >> 16 as i32)
                            as crate::opus_types_h::opus_int32
                            >> 8 as i32 - 1 as i32)
                            + 1 as i32)
                            >> 1 as i32
                    }) > 0x7fff as i32
                    {
                        0x7fff as i32
                    } else {
                        (if (if 8 as i32 == 1 as i32 {
                            ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                as i64
                                * gain_Q10 as i64
                                >> 16 as i32)
                                as crate::opus_types_h::opus_int32
                                >> 1 as i32)
                                + ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                    as i64
                                    * gain_Q10 as i64
                                    >> 16 as i32)
                                    as crate::opus_types_h::opus_int32
                                    & 1 as i32)
                        } else {
                            (((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                as i64
                                * gain_Q10 as i64
                                >> 16 as i32)
                                as crate::opus_types_h::opus_int32
                                >> 8 as i32 - 1 as i32)
                                + 1 as i32)
                                >> 1 as i32
                        }) < 0x8000 as i32 as crate::opus_types_h::opus_int16
                            as i32
                        {
                            0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
                        } else {
                            (if 8 as i32 == 1 as i32 {
                                ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                    as i64
                                    * gain_Q10 as i64
                                    >> 16 as i32)
                                    as crate::opus_types_h::opus_int32
                                    >> 1 as i32)
                                    + ((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                        as i64
                                        * gain_Q10 as i64
                                        >> 16 as i32)
                                        as crate::opus_types_h::opus_int32
                                        & 1 as i32)
                            } else {
                                (((*CNG_sig_Q14.offset((16 as i32 + i) as isize)
                                    as i64
                                    * gain_Q10 as i64
                                    >> 16 as i32)
                                    as crate::opus_types_h::opus_int32
                                    >> 8 as i32 - 1 as i32)
                                    + 1 as i32)
                                    >> 1 as i32
                            })
                        })
                    })
            } as crate::opus_types_h::opus_int16;
            i += 1
        }
        crate::stdlib::memcpy(
            (*psCNG).CNG_synth_state.as_mut_ptr() as *mut libc::c_void,
            &mut *CNG_sig_Q14.offset(length as isize) as *mut crate::opus_types_h::opus_int32
                as *const libc::c_void,
            (16 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::opus_types_h::opus_int32,
            >() as libc::c_ulong),
        );
    } else {
        crate::stdlib::memset(
            (*psCNG).CNG_synth_state.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            ((*psDec).LPC_order as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::opus_types_h::opus_int32,
            >() as libc::c_ulong),
        );
    };
}

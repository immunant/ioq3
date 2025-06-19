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
pub use crate::structs_h::silk_LP_state;
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
/*
    Elliptic/Cauer filters designed with 0.1 dB passband ripple,
    80 dB minimum stopband attenuation, and
    [0.95 : 0.15 : 0.35] normalized cut off frequencies.
*/
/* Helper function, interpolates the filter taps */
#[inline]

unsafe extern "C" fn silk_LP_interpolate_filter_taps(
    mut B_Q28: *mut opus_int32,
    mut A_Q28: *mut opus_int32,
    ind: i32,
    fac_Q16: opus_int32,
) {
    let mut nb: i32 = 0; /* ( fac_Q16 - ( 1 << 16 ) ) is in range of a 16-bit int */
    let mut na: i32 = 0;
    if ind < 5 as i32 - 1 as i32 {
        if fac_Q16 > 0 as i32 {
            if fac_Q16 < 32768 as i32 {
                /* fac_Q16 is in range of a 16-bit int */
                /* Piece-wise linear interpolation of B and A */
                nb = 0 as i32;
                while nb < 3 as i32 {
                    *B_Q28.offset(nb as isize) =
                        (crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_B_Q28[ind as usize][nb as usize]
                             as i64 +
                             ((crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_B_Q28[(ind +
                                                             1 as i32)
                                                            as
                                                            usize][nb as
                                                                       usize]
                                   -
                                   crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_B_Q28[ind as
                                                                usize][nb as
                                                                           usize])
                                  as i64 *
                                  fac_Q16 as opus_int16 as i64 >>
                                  16 as i32)) as opus_int32;
                    nb += 1
                }
                na = 0 as i32;
                while na < 2 as i32 {
                    *A_Q28.offset(na as isize) =
                        (crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_A_Q28[ind as usize][na as usize]
                             as i64 +
                             ((crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_A_Q28[(ind +
                                                             1 as i32)
                                                            as
                                                            usize][na as
                                                                       usize]
                                   -
                                   crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_A_Q28[ind as
                                                                usize][na as
                                                                           usize])
                                  as i64 *
                                  fac_Q16 as opus_int16 as i64 >>
                                  16 as i32)) as opus_int32;
                    na += 1
                }
            } else {
                /* Piece-wise linear interpolation of B and A */
                nb = 0 as i32;
                while nb < 3 as i32 {
                    *B_Q28.offset(nb as isize) =
                        (crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_B_Q28[(ind + 1 as i32) as
                                                      usize][nb as usize] as
                             i64 +
                             ((crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_B_Q28[(ind +
                                                             1 as i32)
                                                            as
                                                            usize][nb as
                                                                       usize]
                                   -
                                   crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_B_Q28[ind as
                                                                usize][nb as
                                                                           usize])
                                  as i64 *
                                  (fac_Q16 -
                                       ((1 as i32) <<
                                            16 as i32)) as opus_int16
                                      as i64 >>
                                  16 as i32)) as opus_int32;
                    nb += 1
                }
                na = 0 as i32;
                while na < 2 as i32 {
                    *A_Q28.offset(na as isize) =
                        (crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_A_Q28[(ind + 1 as i32) as
                                                      usize][na as usize] as
                             i64 +
                             ((crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_A_Q28[(ind +
                                                             1 as i32)
                                                            as
                                                            usize][na as
                                                                       usize]
                                   -
                                   crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_A_Q28[ind as
                                                                usize][na as
                                                                           usize])
                                  as i64 *
                                  (fac_Q16 -
                                       ((1 as i32) <<
                                            16 as i32)) as opus_int16
                                      as i64 >>
                                  16 as i32)) as opus_int32;
                    na += 1
                }
            }
        } else {
            crate::stdlib::memcpy(
                B_Q28 as *mut libc::c_void,
                crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_B_Q28[ind as usize]
                    .as_ptr() as *const libc::c_void,
                (3 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                    opus_int32,
                >() as libc::c_ulong),
            );
            crate::stdlib::memcpy(
                A_Q28 as *mut libc::c_void,
                crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_A_Q28[ind as usize]
                    .as_ptr() as *const libc::c_void,
                (2 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                    opus_int32,
                >() as libc::c_ulong),
            );
        }
    } else {
        crate::stdlib::memcpy(
            B_Q28 as *mut libc::c_void,
            crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_B_Q28
                [(5 as i32 - 1 as i32) as usize]
                .as_ptr() as *const libc::c_void,
            (3 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                opus_int32,
            >() as libc::c_ulong),
        );
        crate::stdlib::memcpy(
            A_Q28 as *mut libc::c_void,
            crate::src::opus_1_2_1::silk::tables_other::silk_Transition_LP_A_Q28
                [(5 as i32 - 1 as i32) as usize]
                .as_ptr() as *const libc::c_void,
            (2 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                opus_int32,
            >() as libc::c_ulong),
        );
    };
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
/* Low-pass filter with variable cutoff frequency based on  */
/* piece-wise linear interpolation between elliptic filters */
/* Start by setting psEncC->mode <> 0;                      */
/* Deactivate by setting psEncC->mode = 0;                  */
#[no_mangle]

pub unsafe extern "C" fn silk_LP_variable_cutoff(
    mut psLP: *mut silk_LP_state,
    mut frame: *mut opus_int16,
    frame_length: i32,
)
/* I    Frame length                                */
{
    let mut B_Q28: [opus_int32; 3] = [0; 3];
    let mut A_Q28: [opus_int32; 2] = [0; 2];
    let mut fac_Q16: opus_int32 = 0 as i32;
    let mut ind: i32 = 0 as i32;
    /* Run filter if needed */
    if (*psLP).mode != 0 as i32 {
        /* Calculate index and interpolation factor for interpolation */
        fac_Q16 = (((5120 as i32 / (5 as i32 * 4 as i32) - (*psLP).transition_frame_no)
            as opus_uint32)
            << 16 as i32 - 6 as i32) as opus_int32;
        ind = fac_Q16 >> 16 as i32;
        fac_Q16 -= ((ind as opus_uint32) << 16 as i32)
            as opus_int32;
        /* Interpolate filter coefficients */
        silk_LP_interpolate_filter_taps(B_Q28.as_mut_ptr(), A_Q28.as_mut_ptr(), ind, fac_Q16);
        /* Update transition frame number for next frame */
        (*psLP).transition_frame_no = if 0 as i32 > 5120 as i32 / (5 as i32 * 4 as i32) {
            if (*psLP).transition_frame_no + (*psLP).mode > 0 as i32 {
                0 as i32
            } else if (*psLP).transition_frame_no + (*psLP).mode
                < 5120 as i32 / (5 as i32 * 4 as i32)
            {
                (5120 as i32) / (5 as i32 * 4 as i32)
            } else {
                ((*psLP).transition_frame_no) + (*psLP).mode
            }
        } else if (*psLP).transition_frame_no + (*psLP).mode > 5120 as i32 / (5 as i32 * 4 as i32) {
            (5120 as i32) / (5 as i32 * 4 as i32)
        } else if (*psLP).transition_frame_no + (*psLP).mode < 0 as i32 {
            0 as i32
        } else {
            ((*psLP).transition_frame_no) + (*psLP).mode
        };
        /* ARMA low-pass filtering */
        crate::src::opus_1_2_1::silk::biquad_alt::silk_biquad_alt_stride1(
            frame,
            B_Q28.as_mut_ptr(),
            A_Q28.as_mut_ptr(),
            (*psLP).In_LP_State.as_mut_ptr(),
            frame,
            frame_length,
        );
    };
}

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
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;

pub use crate::structs_h::silk_LP_state;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_VAD_state;
pub use crate::structs_h::silk_encoder_state;
pub use crate::structs_h::silk_nsq_state;
pub use crate::structs_h::SideInfoIndices;

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
/* Limit, stabilize, convert and quantize NLSFs */
#[no_mangle]

pub unsafe extern "C" fn silk_process_NLSFs(
    mut psEncC: *mut silk_encoder_state,
    mut PredCoef_Q12: *mut [opus_int16; 16],
    mut pNLSF_Q15: *mut opus_int16,
    mut prev_NLSFq_Q15: *const opus_int16,
)
/* I    Previous Normalized LSFs (0 - (2^15-1))     */
{
    let mut i: i32 = 0;
    let mut doInterpolate: i32 = 0;
    let mut NLSF_mu_Q20: i32 = 0;
    let mut i_sqr_Q15: opus_int16 = 0;
    let mut pNLSF0_temp_Q15: [opus_int16; 16] = [0; 16];
    let mut pNLSFW_QW: [opus_int16; 16] = [0; 16];
    let mut pNLSFW0_temp_QW: [opus_int16; 16] = [0; 16];
    /* **********************/
    /* Calculate mu values */
    /* **********************/
    /* NLSF_mu  = 0.003 - 0.0015 * psEnc->speech_activity; */
    NLSF_mu_Q20 = ((0.003f64 * ((1 as i32 as i64) << 20 as i32) as f64 + 0.5f64) as opus_int32
        as i64
        + ((-0.001f64 * ((1 as i32 as i64) << 28 as i32) as f64 + 0.5f64) as opus_int32 as i64
            * (*psEncC).speech_activity_Q8 as opus_int16 as i64
            >> 16 as i32)) as opus_int32;
    if (*psEncC).nb_subfr == 2 as i32 {
        /* Multiply by 1.5 for 10 ms packets */
        NLSF_mu_Q20 = NLSF_mu_Q20 + (NLSF_mu_Q20 >> 1 as i32)
    }
    /* Calculate NLSF weights */
    crate::src::opus_1_2_1::silk::NLSF_VQ_weights_laroia::silk_NLSF_VQ_weights_laroia(
        pNLSFW_QW.as_mut_ptr(),
        pNLSF_Q15 as *const opus_int16,
        (*psEncC).predictLPCOrder,
    );
    /* Update NLSF weights for interpolated NLSFs */
    doInterpolate = ((*psEncC).useInterpolatedNLSFs == 1 as i32
        && ((*psEncC).indices.NLSFInterpCoef_Q2 as i32) < 4 as i32) as i32;
    if doInterpolate != 0 {
        /* Calculate the interpolated NLSF vector for the first half */
        crate::src::opus_1_2_1::silk::interpolate::silk_interpolate(
            pNLSF0_temp_Q15.as_mut_ptr(),
            prev_NLSFq_Q15,
            pNLSF_Q15 as *const opus_int16,
            (*psEncC).indices.NLSFInterpCoef_Q2 as i32,
            (*psEncC).predictLPCOrder,
        );
        /* Calculate first half NLSF weights for the interpolated NLSFs */
        crate::src::opus_1_2_1::silk::NLSF_VQ_weights_laroia::silk_NLSF_VQ_weights_laroia(
            pNLSFW0_temp_QW.as_mut_ptr(),
            pNLSF0_temp_Q15.as_mut_ptr(),
            (*psEncC).predictLPCOrder,
        );
        /* Update NLSF weights with contribution from first half */
        i_sqr_Q15 = ((((*psEncC).indices.NLSFInterpCoef_Q2 as opus_int16 as opus_int32
            * (*psEncC).indices.NLSFInterpCoef_Q2 as opus_int16 as opus_int32)
            as opus_uint32)
            << 11 as i32) as opus_int32 as opus_int16;
        i = 0 as i32;
        while i < (*psEncC).predictLPCOrder {
            pNLSFW_QW[i as usize] = ((pNLSFW_QW[i as usize] as i32 >> 1 as i32)
                + (pNLSFW0_temp_QW[i as usize] as opus_int32 * i_sqr_Q15 as opus_int32
                    >> 16 as i32)) as opus_int16;
            i += 1
        }
    }
    crate::src::opus_1_2_1::silk::NLSF_encode::silk_NLSF_encode(
        (*psEncC).indices.NLSFIndices.as_mut_ptr(),
        pNLSF_Q15,
        (*psEncC).psNLSF_CB as *const silk_NLSF_CB_struct,
        pNLSFW_QW.as_mut_ptr(),
        NLSF_mu_Q20,
        (*psEncC).NLSF_MSVQ_Survivors,
        (*psEncC).indices.signalType as i32,
    );
    /* Convert quantized NLSFs back to LPC coefficients */
    crate::src::opus_1_2_1::silk::NLSF2A::silk_NLSF2A(
        (*PredCoef_Q12.offset(1 as i32 as isize)).as_mut_ptr(),
        pNLSF_Q15 as *const opus_int16,
        (*psEncC).predictLPCOrder,
        (*psEncC).arch,
    );
    if doInterpolate != 0 {
        /* Calculate the interpolated, quantized LSF vector for the first half */
        crate::src::opus_1_2_1::silk::interpolate::silk_interpolate(
            pNLSF0_temp_Q15.as_mut_ptr(),
            prev_NLSFq_Q15,
            pNLSF_Q15 as *const opus_int16,
            (*psEncC).indices.NLSFInterpCoef_Q2 as i32,
            (*psEncC).predictLPCOrder,
        );
        /* Convert back to LPC coefficients */
        crate::src::opus_1_2_1::silk::NLSF2A::silk_NLSF2A(
            (*PredCoef_Q12.offset(0 as i32 as isize)).as_mut_ptr(),
            pNLSF0_temp_Q15.as_mut_ptr(),
            (*psEncC).predictLPCOrder,
            (*psEncC).arch,
        );
    } else {
        /* Copy LPC coefficients for first half from second half */
        crate::stdlib::memcpy(
            (*PredCoef_Q12.offset(0 as i32 as isize)).as_mut_ptr() as *mut libc::c_void,
            (*PredCoef_Q12.offset(1 as i32 as isize)).as_mut_ptr() as *const libc::c_void,
            ((*psEncC).predictLPCOrder as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<opus_int16>() as libc::c_ulong),
        );
    };
}

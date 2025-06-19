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
/* Delayed-decision quantizer for NLSF residuals */
#[no_mangle]

pub unsafe extern "C" fn silk_NLSF_del_dec_quant(
    mut indices: *mut i8,
    mut x_Q10: *const opus_int16,
    mut w_Q5: *const opus_int16,
    mut pred_coef_Q8: *const u8,
    mut ec_ix: *const opus_int16,
    mut ec_rates_Q5: *const u8,
    quant_step_size_Q16: i32,
    inv_quant_step_size_Q6: opus_int16,
    mu_Q20: opus_int32,
    order: opus_int16,
) -> opus_int32
/* I    Number of input values                      */ {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut nStates: i32 = 0;
    let mut ind_tmp: i32 = 0;
    let mut ind_min_max: i32 = 0;
    let mut ind_max_min: i32 = 0;
    let mut in_Q10: i32 = 0;
    let mut res_Q10: i32 = 0;
    let mut pred_Q10: i32 = 0;
    let mut diff_Q10: i32 = 0;
    let mut rate0_Q5: i32 = 0;
    let mut rate1_Q5: i32 = 0;
    let mut out0_Q10: opus_int16 = 0;
    let mut out1_Q10: opus_int16 = 0;
    let mut RD_tmp_Q25: opus_int32 = 0;
    let mut min_Q25: opus_int32 = 0;
    let mut min_max_Q25: opus_int32 = 0;
    let mut max_min_Q25: opus_int32 = 0;
    let mut ind_sort: [i32; 4] = [0; 4];
    let mut ind: [[i8; 16]; 4] = [[0; 16]; 4];
    let mut prev_out_Q10: [opus_int16; 8] = [0; 8];
    let mut RD_Q25: [opus_int32; 8] = [0; 8];
    let mut RD_min_Q25: [opus_int32; 4] = [0; 4];
    let mut RD_max_Q25: [opus_int32; 4] = [0; 4];
    let mut rates_Q5: *const u8 = 0 as *const u8;
    let mut out0_Q10_table: [i32; 20] = [0; 20];
    let mut out1_Q10_table: [i32; 20] = [0; 20];
    i = -(10 as i32);
    while i <= 10 as i32 - 1 as i32 {
        out0_Q10 = ((i as opus_uint32) << 10 as i32)
            as opus_int32
            as opus_int16;
        out1_Q10 = (out0_Q10 as i32 + 1024 as i32) as opus_int16;
        if i > 0 as i32 {
            out0_Q10 = (out0_Q10 as i32
                - (0.1f64 * ((1 as i32 as i64) << 10 as i32) as f64 + 0.5f64)
                    as opus_int32)
                as opus_int16;
            out1_Q10 = (out1_Q10 as i32
                - (0.1f64 * ((1 as i32 as i64) << 10 as i32) as f64 + 0.5f64)
                    as opus_int32)
                as opus_int16
        } else if i == 0 as i32 {
            out1_Q10 = (out1_Q10 as i32
                - (0.1f64 * ((1 as i32 as i64) << 10 as i32) as f64 + 0.5f64)
                    as opus_int32)
                as opus_int16
        } else if i == -(1 as i32) {
            out0_Q10 = (out0_Q10 as i32
                + (0.1f64 * ((1 as i32 as i64) << 10 as i32) as f64 + 0.5f64)
                    as opus_int32)
                as opus_int16
        } else {
            out0_Q10 = (out0_Q10 as i32
                + (0.1f64 * ((1 as i32 as i64) << 10 as i32) as f64 + 0.5f64)
                    as opus_int32)
                as opus_int16;
            out1_Q10 = (out1_Q10 as i32
                + (0.1f64 * ((1 as i32 as i64) << 10 as i32) as f64 + 0.5f64)
                    as opus_int32)
                as opus_int16
        }
        out0_Q10_table[(i + 10 as i32) as usize] = out0_Q10 as opus_int32
            * quant_step_size_Q16 as opus_int16
                as opus_int32
            >> 16 as i32;
        out1_Q10_table[(i + 10 as i32) as usize] = out1_Q10 as opus_int32
            * quant_step_size_Q16 as opus_int16
                as opus_int32
            >> 16 as i32;
        i += 1
    }
    /* must be power of two */
    nStates = 1 as i32;
    RD_Q25[0 as i32 as usize] = 0 as i32;
    prev_out_Q10[0 as i32 as usize] = 0 as i32 as opus_int16;
    i = order as i32 - 1 as i32;
    while i >= 0 as i32 {
        rates_Q5 = &*ec_rates_Q5.offset(*ec_ix.offset(i as isize) as isize) as *const u8;
        in_Q10 = *x_Q10.offset(i as isize) as i32;
        j = 0 as i32;
        while j < nStates {
            pred_Q10 = *pred_coef_Q8.offset(i as isize) as opus_int16
                as opus_int32
                * prev_out_Q10[j as usize] as opus_int32
                >> 8 as i32;
            res_Q10 = in_Q10 - pred_Q10;
            ind_tmp = inv_quant_step_size_Q6 as opus_int32
                * res_Q10 as opus_int16 as opus_int32
                >> 16 as i32;
            ind_tmp = if -(10 as i32) > 10 as i32 - 1 as i32 {
                if ind_tmp > -(10 as i32) {
                    -(10 as i32)
                } else if ind_tmp < 10 as i32 - 1 as i32 {
                    (10 as i32) - 1 as i32
                } else {
                    ind_tmp
                }
            } else if ind_tmp > 10 as i32 - 1 as i32 {
                (10 as i32) - 1 as i32
            } else if ind_tmp < -(10 as i32) {
                -(10 as i32)
            } else {
                ind_tmp
            };
            ind[j as usize][i as usize] = ind_tmp as i8;
            /* compute outputs for ind_tmp and ind_tmp + 1 */
            out0_Q10 =
                out0_Q10_table[(ind_tmp + 10 as i32) as usize] as opus_int16;
            out1_Q10 =
                out1_Q10_table[(ind_tmp + 10 as i32) as usize] as opus_int16;
            out0_Q10 = (out0_Q10 as i32 + pred_Q10) as opus_int16;
            out1_Q10 = (out1_Q10 as i32 + pred_Q10) as opus_int16;
            prev_out_Q10[j as usize] = out0_Q10;
            prev_out_Q10[(j + nStates) as usize] = out1_Q10;
            /* compute RD for ind_tmp and ind_tmp + 1 */
            if ind_tmp + 1 as i32 >= 4 as i32 {
                if ind_tmp + 1 as i32 == 4 as i32 {
                    rate0_Q5 = *rates_Q5.offset((ind_tmp + 4 as i32) as isize) as i32;
                    rate1_Q5 = 280 as i32
                } else {
                    rate0_Q5 = 280 as i32 - 43 as i32 * 4 as i32
                        + 43 as i32 as opus_int16
                            as opus_int32
                            * ind_tmp as opus_int16
                                as opus_int32;
                    rate1_Q5 = rate0_Q5 + 43 as i32
                }
            } else if ind_tmp <= -(4 as i32) {
                if ind_tmp == -(4 as i32) {
                    rate0_Q5 = 280 as i32;
                    rate1_Q5 = *rates_Q5.offset((ind_tmp + 1 as i32 + 4 as i32) as isize) as i32
                } else {
                    rate0_Q5 = 280 as i32 - 43 as i32 * 4 as i32
                        + -(43 as i32) as opus_int16
                            as opus_int32
                            * ind_tmp as opus_int16
                                as opus_int32;
                    rate1_Q5 = rate0_Q5 - 43 as i32
                }
            } else {
                rate0_Q5 = *rates_Q5.offset((ind_tmp + 4 as i32) as isize) as i32;
                rate1_Q5 = *rates_Q5.offset((ind_tmp + 1 as i32 + 4 as i32) as isize) as i32
            }
            RD_tmp_Q25 = RD_Q25[j as usize];
            diff_Q10 = in_Q10 - out0_Q10 as i32;
            RD_Q25[j as usize] = RD_tmp_Q25
                + diff_Q10 as opus_int16 as opus_int32
                    * diff_Q10 as opus_int16
                        as opus_int32
                    * *w_Q5.offset(i as isize) as i32
                + mu_Q20 as opus_int16 as opus_int32
                    * rate0_Q5 as opus_int16
                        as opus_int32;
            diff_Q10 = in_Q10 - out1_Q10 as i32;
            RD_Q25[(j + nStates) as usize] = RD_tmp_Q25
                + diff_Q10 as opus_int16 as opus_int32
                    * diff_Q10 as opus_int16
                        as opus_int32
                    * *w_Q5.offset(i as isize) as i32
                + mu_Q20 as opus_int16 as opus_int32
                    * rate1_Q5 as opus_int16
                        as opus_int32;
            j += 1
        }
        if nStates <= ((1 as i32) << 2 as i32) / 2 as i32 {
            /* double number of states and copy */
            j = 0 as i32;
            while j < nStates {
                ind[(j + nStates) as usize][i as usize] =
                    (ind[j as usize][i as usize] as i32 + 1 as i32) as i8;
                j += 1
            }
            nStates = ((nStates as opus_uint32) << 1 as i32)
                as opus_int32;
            j = nStates;
            while j < (1 as i32) << 2 as i32 {
                ind[j as usize][i as usize] = ind[(j - nStates) as usize][i as usize];
                j += 1
            }
        } else {
            /* sort lower and upper half of RD_Q25, pairwise */
            j = 0 as i32;
            while j < (1 as i32) << 2 as i32 {
                if RD_Q25[j as usize] > RD_Q25[(j + ((1 as i32) << 2 as i32)) as usize] {
                    RD_max_Q25[j as usize] = RD_Q25[j as usize];
                    RD_min_Q25[j as usize] = RD_Q25[(j + ((1 as i32) << 2 as i32)) as usize];
                    RD_Q25[j as usize] = RD_min_Q25[j as usize];
                    RD_Q25[(j + ((1 as i32) << 2 as i32)) as usize] = RD_max_Q25[j as usize];
                    /* swap prev_out values */
                    out0_Q10 = prev_out_Q10[j as usize];
                    prev_out_Q10[j as usize] =
                        prev_out_Q10[(j + ((1 as i32) << 2 as i32)) as usize];
                    prev_out_Q10[(j + ((1 as i32) << 2 as i32)) as usize] = out0_Q10;
                    ind_sort[j as usize] = j + ((1 as i32) << 2 as i32)
                } else {
                    RD_min_Q25[j as usize] = RD_Q25[j as usize];
                    RD_max_Q25[j as usize] = RD_Q25[(j + ((1 as i32) << 2 as i32)) as usize];
                    ind_sort[j as usize] = j
                }
                j += 1
            }
            loop
            /* compare the highest RD values of the winning half with the lowest one in the losing half, and copy if necessary */
            /* afterwards ind_sort[] will contain the indices of the NLSF_QUANT_DEL_DEC_STATES winning RD values */
            {
                min_max_Q25 = 0x7fffffff as i32;
                max_min_Q25 = 0 as i32;
                ind_min_max = 0 as i32;
                ind_max_min = 0 as i32;
                j = 0 as i32;
                while j < (1 as i32) << 2 as i32 {
                    if min_max_Q25 > RD_max_Q25[j as usize] {
                        min_max_Q25 = RD_max_Q25[j as usize];
                        ind_min_max = j
                    }
                    if max_min_Q25 < RD_min_Q25[j as usize] {
                        max_min_Q25 = RD_min_Q25[j as usize];
                        ind_max_min = j
                    }
                    j += 1
                }
                if min_max_Q25 >= max_min_Q25 {
                    break;
                }
                /* copy ind_min_max to ind_max_min */
                ind_sort[ind_max_min as usize] =
                    ind_sort[ind_min_max as usize] ^ (1 as i32) << 2 as i32;
                RD_Q25[ind_max_min as usize] =
                    RD_Q25[(ind_min_max + ((1 as i32) << 2 as i32)) as usize];
                prev_out_Q10[ind_max_min as usize] =
                    prev_out_Q10[(ind_min_max + ((1 as i32) << 2 as i32)) as usize];
                RD_min_Q25[ind_max_min as usize] = 0 as i32;
                RD_max_Q25[ind_min_max as usize] = 0x7fffffff as i32;
                crate::stdlib::memcpy(
                    ind[ind_max_min as usize].as_mut_ptr() as *mut libc::c_void,
                    ind[ind_min_max as usize].as_mut_ptr() as *const libc::c_void,
                    (16 as i32 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<i8>() as libc::c_ulong),
                );
            }
            /* increment index if it comes from the upper half */
            j = 0 as i32;
            while j < (1 as i32) << 2 as i32 {
                ind[j as usize][i as usize] =
                    (ind[j as usize][i as usize] as i32 + (ind_sort[j as usize] >> 2 as i32)) as i8;
                j += 1
            }
        }
        i -= 1
    }
    /* last sample: find winner, copy indices and return RD value */
    ind_tmp = 0 as i32;
    min_Q25 = 0x7fffffff as i32;
    j = 0 as i32;
    while j < 2 as i32 * ((1 as i32) << 2 as i32) {
        if min_Q25 > RD_Q25[j as usize] {
            min_Q25 = RD_Q25[j as usize];
            ind_tmp = j
        }
        j += 1
    }
    j = 0 as i32;
    while j < order as i32 {
        *indices.offset(j as isize) =
            ind[(ind_tmp & ((1 as i32) << 2 as i32) - 1 as i32) as usize][j as usize];
        j += 1
    }
    let ref mut fresh0 = *indices.offset(0 as i32 as isize);
    *fresh0 = (*fresh0 as i32 + (ind_tmp >> 2 as i32)) as i8;
    return min_Q25;
}

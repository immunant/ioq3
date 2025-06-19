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
#[no_mangle]

pub unsafe extern "C" fn silk_quant_LTP_gains(
    mut B_Q14: *mut opus_int16,
    mut cbk_index: *mut i8,
    mut periodicity_index: *mut i8,
    mut sum_log_gain_Q7: *mut opus_int32,
    mut pred_gain_dB_Q7: *mut i32,
    mut XX_Q17: *const opus_int32,
    mut xX_Q17: *const opus_int32,
    subfr_len: i32,
    nb_subfr: i32,
    mut _arch: i32,
)
/* I    Run-time architecture           */
{
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut cbk_size: i32 = 0;
    let mut temp_idx: [i8; 4] = [0; 4];
    let mut cl_ptr_Q5: *const u8 = 0 as *const u8;
    let mut cbk_ptr_Q7: *const i8 = 0 as *const i8;
    let mut cbk_gain_ptr_Q7: *const u8 = 0 as *const u8;
    let mut XX_Q17_ptr: *const opus_int32 = 0 as *const opus_int32;
    let mut xX_Q17_ptr: *const opus_int32 = 0 as *const opus_int32;
    let mut res_nrg_Q15_subfr: opus_int32 = 0;
    let mut res_nrg_Q15: opus_int32 = 0;
    let mut rate_dist_Q7_subfr: opus_int32 = 0;
    let mut rate_dist_Q7: opus_int32 = 0;
    let mut min_rate_dist_Q7: opus_int32 = 0;
    let mut sum_log_gain_tmp_Q7: opus_int32 = 0;
    let mut best_sum_log_gain_Q7: opus_int32 = 0;
    let mut max_gain_Q7: opus_int32 = 0;
    let mut gain_Q7: i32 = 0;
    /* **************************************************/
    /* iterate over different codebooks with different */
    /* rates/distortions, and choose best */
    /* **************************************************/
    min_rate_dist_Q7 = 0x7fffffff as i32;
    best_sum_log_gain_Q7 = 0 as i32;
    k = 0 as i32;
    while k < 3 as i32 {
        /* Safety margin for pitch gain control, to take into account factors
        such as state rescaling/rewhitening. */
        let mut gain_safety: opus_int32 =
            (0.4f64 * ((1 as i32 as i64) << 7 as i32) as f64 + 0.5f64) as opus_int32;
        cl_ptr_Q5 =
            crate::src::opus_1_2_1::silk::tables_LTP::silk_LTP_gain_BITS_Q5_ptrs[k as usize];
        cbk_ptr_Q7 = crate::src::opus_1_2_1::silk::tables_LTP::silk_LTP_vq_ptrs_Q7[k as usize];
        cbk_gain_ptr_Q7 =
            crate::src::opus_1_2_1::silk::tables_LTP::silk_LTP_vq_gain_ptrs_Q7[k as usize];
        cbk_size = crate::src::opus_1_2_1::silk::tables_LTP::silk_LTP_vq_sizes[k as usize] as i32;
        /* Set up pointers to first subframe */
        XX_Q17_ptr = XX_Q17;
        xX_Q17_ptr = xX_Q17;
        res_nrg_Q15 = 0 as i32;
        rate_dist_Q7 = 0 as i32;
        sum_log_gain_tmp_Q7 = *sum_log_gain_Q7;
        j = 0 as i32;
        while j < nb_subfr {
            max_gain_Q7 = crate::src::opus_1_2_1::silk::log2lin::silk_log2lin(
                (250.0f32 as f64 / 6.0f64 * ((1 as i32 as i64) << 7 as i32) as f64 + 0.5f64)
                    as opus_int32
                    - sum_log_gain_tmp_Q7
                    + ((7 as i32 as i64 * ((1 as i32 as i64) << 7 as i32)) as f64 + 0.5f64)
                        as opus_int32,
            ) - gain_safety;
            crate::src::opus_1_2_1::silk::VQ_WMat_EC::silk_VQ_WMat_EC_c(
                &mut *temp_idx.as_mut_ptr().offset(j as isize),
                &mut res_nrg_Q15_subfr,
                &mut rate_dist_Q7_subfr,
                &mut gain_Q7,
                XX_Q17_ptr,
                xX_Q17_ptr,
                cbk_ptr_Q7,
                cbk_gain_ptr_Q7,
                cl_ptr_Q5,
                subfr_len,
                max_gain_Q7,
                cbk_size,
            );
            /* I    Run-time architecture                                   */
            res_nrg_Q15 = if (res_nrg_Q15 as opus_uint32)
                .wrapping_add(res_nrg_Q15_subfr as opus_uint32)
                & 0x80000000 as u32
                != 0
            {
                0x7fffffff as i32
            } else {
                (res_nrg_Q15) + res_nrg_Q15_subfr
            };
            rate_dist_Q7 = if (rate_dist_Q7 as opus_uint32)
                .wrapping_add(rate_dist_Q7_subfr as opus_uint32)
                & 0x80000000 as u32
                != 0
            {
                0x7fffffff as i32
            } else {
                (rate_dist_Q7) + rate_dist_Q7_subfr
            };
            sum_log_gain_tmp_Q7 = if 0 as i32
                > sum_log_gain_tmp_Q7
                    + crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(gain_safety + gain_Q7)
                    - ((7 as i32 as i64 * ((1 as i32 as i64) << 7 as i32)) as f64 + 0.5f64)
                        as opus_int32
            {
                0 as i32
            } else {
                (sum_log_gain_tmp_Q7
                    + crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(gain_safety + gain_Q7))
                    - ((7 as i32 as i64 * ((1 as i32 as i64) << 7 as i32)) as f64 + 0.5f64)
                        as opus_int32
            };
            XX_Q17_ptr = XX_Q17_ptr.offset((5 as i32 * 5 as i32) as isize);
            xX_Q17_ptr = xX_Q17_ptr.offset(5 as i32 as isize);
            j += 1
        }
        if rate_dist_Q7 <= min_rate_dist_Q7 {
            min_rate_dist_Q7 = rate_dist_Q7;
            *periodicity_index = k as i8;
            crate::stdlib::memcpy(
                cbk_index as *mut libc::c_void,
                temp_idx.as_mut_ptr() as *const libc::c_void,
                (nb_subfr as usize)
                    .wrapping_mul(::std::mem::size_of::<i8>() as usize),
            );
            best_sum_log_gain_Q7 = sum_log_gain_tmp_Q7
        }
        k += 1
    }
    cbk_ptr_Q7 =
        crate::src::opus_1_2_1::silk::tables_LTP::silk_LTP_vq_ptrs_Q7[*periodicity_index as usize];
    j = 0 as i32;
    while j < nb_subfr {
        k = 0 as i32;
        while k < 5 as i32 {
            *B_Q14.offset((j * 5 as i32 + k) as isize) = ((*cbk_ptr_Q7
                .offset((*cbk_index.offset(j as isize) as i32 * 5 as i32 + k) as isize)
                as opus_uint32)
                << 7 as i32) as opus_int32
                as opus_int16;
            k += 1
        }
        j += 1
    }
    if nb_subfr == 2 as i32 {
        res_nrg_Q15 = res_nrg_Q15 >> 1 as i32
    } else {
        res_nrg_Q15 = res_nrg_Q15 >> 2 as i32
    }
    *sum_log_gain_Q7 = best_sum_log_gain_Q7;
    *pred_gain_dB_Q7 = -(3 as i32) as opus_int16 as opus_int32
        * (crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(res_nrg_Q15)
            - ((15 as i32) << 7 as i32)) as opus_int16 as opus_int32;
}

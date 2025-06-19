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
/* Control SNR of redidual quantizer */
#[no_mangle]

pub unsafe extern "C" fn silk_control_SNR(
    mut psEncC: *mut crate::structs_h::silk_encoder_state,
    mut TargetRate_bps: crate::opus_types_h::opus_int32,
) -> i32
/* I    Target max bitrate (bps)                    */ {
    let mut k: i32 = 0;
    let mut ret: i32 = 0 as i32;
    let mut frac_Q6: crate::opus_types_h::opus_int32 = 0;
    let mut rateTable: *const crate::opus_types_h::opus_int32 =
        0 as *const crate::opus_types_h::opus_int32;
    /* Set bitrate/coding quality */
    TargetRate_bps = if 5000 as i32 > 80000 as i32 {
        if TargetRate_bps > 5000 as i32 {
            5000 as i32
        } else if TargetRate_bps < 80000 as i32 {
            80000 as i32
        } else {
            TargetRate_bps
        }
    } else if TargetRate_bps > 80000 as i32 {
        80000 as i32
    } else if TargetRate_bps < 5000 as i32 {
        5000 as i32
    } else {
        TargetRate_bps
    };
    if TargetRate_bps != (*psEncC).TargetRate_bps {
        (*psEncC).TargetRate_bps = TargetRate_bps;
        /* If new TargetRate_bps, translate to SNR_dB value */
        if (*psEncC).fs_kHz == 8 as i32 {
            rateTable =
                crate::src::opus_1_2_1::silk::tables_other::silk_TargetRate_table_NB.as_ptr()
        } else if (*psEncC).fs_kHz == 12 as i32 {
            rateTable =
                crate::src::opus_1_2_1::silk::tables_other::silk_TargetRate_table_MB.as_ptr()
        } else {
            rateTable =
                crate::src::opus_1_2_1::silk::tables_other::silk_TargetRate_table_WB.as_ptr()
        }
        /* Reduce bitrate for 10 ms modes in these calculations */
        if (*psEncC).nb_subfr == 2 as i32 {
            TargetRate_bps -= 2200 as i32
        }
        /* Find bitrate interval in table and interpolate */
        k = 1 as i32;
        while k < 8 as i32 {
            if TargetRate_bps <= *rateTable.offset(k as isize) {
                frac_Q6 = (((TargetRate_bps - *rateTable.offset((k - 1 as i32) as isize))
                    as crate::opus_types_h::opus_uint32)
                    << 6 as i32)
                    as crate::opus_types_h::opus_int32
                    / (*rateTable.offset(k as isize)
                        - *rateTable.offset((k - 1 as i32) as isize));
                (*psEncC).SNR_dB_Q7 =
                    ((crate::src::opus_1_2_1::silk::tables_other::silk_SNR_table_Q1
                        [(k - 1 as i32) as usize]
                        as crate::opus_types_h::opus_uint32)
                        << 6 as i32) as crate::opus_types_h::opus_int32
                        + frac_Q6
                            * (crate::src::opus_1_2_1::silk::tables_other::silk_SNR_table_Q1
                                [k as usize] as i32
                                - crate::src::opus_1_2_1::silk::tables_other::silk_SNR_table_Q1
                                    [(k - 1 as i32) as usize]
                                    as i32);
                break;
            } else {
                k += 1
            }
        }
    }
    return ret;
}

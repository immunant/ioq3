use ::libc;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
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
/* Quantize mid/side predictors */
#[no_mangle]

pub unsafe extern "C" fn silk_stereo_quant_pred(
    mut pred_Q13: *mut crate::opus_types_h::opus_int32,
    mut ix: *mut [i8; 3],
)
/* O    Quantization indices                        */
{
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut low_Q13: crate::opus_types_h::opus_int32 = 0;
    let mut step_Q13: crate::opus_types_h::opus_int32 = 0;
    let mut lvl_Q13: crate::opus_types_h::opus_int32 = 0;
    let mut err_min_Q13: crate::opus_types_h::opus_int32 = 0;
    let mut err_Q13: crate::opus_types_h::opus_int32 = 0;
    let mut quant_pred_Q13: crate::opus_types_h::opus_int32 = 0 as i32;
    /* Quantize */
    n = 0 as i32;
    while n < 2 as i32 {
        /* Brute-force search over quantization levels */
        err_min_Q13 = 0x7fffffff as i32;
        i = 0 as i32;
        's_23: while i < 16 as i32 - 1 as i32 {
            low_Q13 = crate::src::opus_1_2_1::silk::tables_other::silk_stereo_pred_quant_Q13
                [i as usize] as crate::opus_types_h::opus_int32;
            step_Q13 = ((crate::src::opus_1_2_1::silk::tables_other::silk_stereo_pred_quant_Q13
                [(i + 1 as i32) as usize] as i32
                - low_Q13) as i64
                * (0.5f64 / 5 as i32 as f64
                    * ((1 as i32 as i64) << 16 as i32)
                        as f64
                    + 0.5f64) as crate::opus_types_h::opus_int32
                    as crate::opus_types_h::opus_int16 as i64
                >> 16 as i32) as crate::opus_types_h::opus_int32;
            j = 0 as i32;
            while j < 5 as i32 {
                lvl_Q13 = low_Q13
                    + step_Q13 as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32
                        * (2 as i32 * j + 1 as i32)
                            as crate::opus_types_h::opus_int16
                            as crate::opus_types_h::opus_int32;
                err_Q13 = if *pred_Q13.offset(n as isize) - lvl_Q13 > 0 as i32 {
                    (*pred_Q13.offset(n as isize)) - lvl_Q13
                } else {
                    -(*pred_Q13.offset(n as isize) - lvl_Q13)
                };
                if !(err_Q13 < err_min_Q13) {
                    break 's_23;
                }
                err_min_Q13 = err_Q13;
                quant_pred_Q13 = lvl_Q13;
                (*ix.offset(n as isize))[0 as i32 as usize] = i as i8;
                (*ix.offset(n as isize))[1 as i32 as usize] = j as i8;
                j += 1
            }
            i += 1
        }
        /* Error increasing, so we're past the optimum */
        (*ix.offset(n as isize))[2 as i32 as usize] =
            ((*ix.offset(n as isize))[0 as i32 as usize] as i32 / 3 as i32)
                as i8;
        let ref mut fresh0 = (*ix.offset(n as isize))[0 as i32 as usize];
        *fresh0 = (*fresh0 as i32
            - (*ix.offset(n as isize))[2 as i32 as usize] as i32 * 3 as i32)
            as i8;
        *pred_Q13.offset(n as isize) = quant_pred_Q13;
        n += 1
    }
    /* Subtract second from first predictor (helps when actually applying these) */
    let ref mut fresh1 = *pred_Q13.offset(0 as i32 as isize);
    *fresh1 -= *pred_Q13.offset(1 as i32 as isize);
}

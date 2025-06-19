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
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;

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
/*#define silk_enc_map(a)                ((a) > 0 ? 1 : 0)*/
/*#define silk_dec_map(a)                ((a) > 0 ? 1 : -1)*/
/* shifting avoids if-statement */
/* Encodes signs of excitation */
#[no_mangle]

pub unsafe extern "C" fn silk_encode_signs(
    mut psRangeEnc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut pulses: *const i8,
    mut length: i32,
    signalType: i32,
    quantOffsetType: i32,
    mut sum_pulses: *const i32,
)
/* I    Sum of absolute pulses per block            */
{
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p: i32 = 0;
    let mut icdf: [u8; 2] = [0; 2];
    let mut q_ptr: *const i8 = 0 as *const i8;
    let mut icdf_ptr: *const u8 = 0 as *const u8;
    icdf[1 as i32 as usize] = 0 as i32 as u8;
    q_ptr = pulses;
    i = 7 as i32 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
        * (quantOffsetType
            + ((signalType as crate::opus_types_h::opus_uint32) << 1 as i32)
                as crate::opus_types_h::opus_int32) as crate::opus_types_h::opus_int16
            as crate::opus_types_h::opus_int32;
    icdf_ptr = &*crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_sign_iCDF
        .as_ptr()
        .offset(i as isize) as *const u8;
    length = length + 16 as i32 / 2 as i32 >> 4 as i32;
    i = 0 as i32;
    while i < length {
        p = *sum_pulses.offset(i as isize);
        if p > 0 as i32 {
            icdf[0 as i32 as usize] = *icdf_ptr.offset(if (p & 0x1f as i32) < 6 as i32 {
                (p) & 0x1f as i32
            } else {
                6 as i32
            } as isize);
            j = 0 as i32;
            while j < 16 as i32 {
                if *q_ptr.offset(j as isize) as i32 != 0 as i32 {
                    crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
                        psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        (*q_ptr.offset(j as isize) as i32 >> 15 as i32) + 1 as i32,
                        icdf.as_mut_ptr(),
                        8 as i32 as u32,
                    );
                }
                j += 1
            }
        }
        q_ptr = q_ptr.offset(16 as i32 as isize);
        i += 1
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
/* Decodes signs of excitation */
#[no_mangle]

pub unsafe extern "C" fn silk_decode_signs(
    mut psRangeDec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut pulses: *mut crate::opus_types_h::opus_int16,
    mut length: i32,
    signalType: i32,
    quantOffsetType: i32,
    mut sum_pulses: *const i32,
)
/* I    Sum of absolute pulses per block            */
{
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p: i32 = 0;
    let mut icdf: [u8; 2] = [0; 2];
    let mut q_ptr: *mut crate::opus_types_h::opus_int16 = 0 as *mut crate::opus_types_h::opus_int16;
    let mut icdf_ptr: *const u8 = 0 as *const u8;
    icdf[1 as i32 as usize] = 0 as i32 as u8;
    q_ptr = pulses;
    i = 7 as i32 as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
        * (quantOffsetType
            + ((signalType as crate::opus_types_h::opus_uint32) << 1 as i32)
                as crate::opus_types_h::opus_int32) as crate::opus_types_h::opus_int16
            as crate::opus_types_h::opus_int32;
    icdf_ptr = &*crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_sign_iCDF
        .as_ptr()
        .offset(i as isize) as *const u8;
    length = length + 16 as i32 / 2 as i32 >> 4 as i32;
    i = 0 as i32;
    while i < length {
        p = *sum_pulses.offset(i as isize);
        if p > 0 as i32 {
            icdf[0 as i32 as usize] = *icdf_ptr.offset(if (p & 0x1f as i32) < 6 as i32 {
                (p) & 0x1f as i32
            } else {
                6 as i32
            } as isize);
            j = 0 as i32;
            while j < 16 as i32 {
                if *q_ptr.offset(j as isize) as i32 > 0 as i32 {
                    /* attach sign */
                    /* implementation with shift, subtraction, multiplication */
                    let ref mut fresh0 = *q_ptr.offset(j as isize);
                    *fresh0 = (*fresh0 as i32
                        * (((crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                            psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                            icdf.as_mut_ptr(),
                            8 as i32 as u32,
                        ) as crate::opus_types_h::opus_uint32)
                            << 1 as i32)
                            as crate::opus_types_h::opus_int32
                            - 1 as i32))
                        as crate::opus_types_h::opus_int16
                }
                j += 1
            }
        }
        q_ptr = q_ptr.offset(16 as i32 as isize);
        i += 1
    }
}

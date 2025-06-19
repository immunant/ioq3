use ::libc;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int16;
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
/* shell coder; pulse-subframe length is hardcoded */
#[inline]

unsafe extern "C" fn combine_pulses(mut out: *mut i32, mut in_0: *const i32, len: i32)
/* I    number of OUTPUT samples     */
{
    let mut k: i32 = 0;
    k = 0 as i32;
    while k < len {
        *out.offset(k as isize) = *in_0.offset((2 as i32 * k) as isize)
            + *in_0.offset((2 as i32 * k + 1 as i32) as isize);
        k += 1
    }
}
#[inline]

unsafe extern "C" fn encode_split(
    mut psRangeEnc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    p_child1: i32,
    p: i32,
    mut shell_table: *const u8,
)
/* I    table of shell cdfs                         */
{
    if p > 0 as i32 {
        crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx, p_child1,
                    &*shell_table.offset(*crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table_offsets.as_ptr().offset(p
                                                                                            as
                                                                                            isize)
                                             as isize),
                    8 as i32 as u32);
    };
}
#[inline]

unsafe extern "C" fn decode_split(
    mut p_child1: *mut crate::opus_types_h::opus_int16,
    mut p_child2: *mut crate::opus_types_h::opus_int16,
    mut psRangeDec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    p: i32,
    mut shell_table: *const u8,
)
/* I    table of shell cdfs                         */
{
    if p > 0 as i32 {
        *p_child1.offset(0 as i32 as isize) =
            crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        &*shell_table.offset(*crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table_offsets.as_ptr().offset(p
                                                                                                as
                                                                                                isize)
                                                 as isize),
                        8 as i32 as u32) as crate::opus_types_h::opus_int16;
        *p_child2.offset(0 as i32 as isize) =
            (p - *p_child1.offset(0 as i32 as isize) as i32) as crate::opus_types_h::opus_int16
    } else {
        *p_child1.offset(0 as i32 as isize) = 0 as i32 as crate::opus_types_h::opus_int16;
        *p_child2.offset(0 as i32 as isize) = 0 as i32 as crate::opus_types_h::opus_int16
    };
}
/* Shell encoder, operates on one shell code frame of 16 pulses */
#[no_mangle]

pub unsafe extern "C" fn silk_shell_encoder(
    mut psRangeEnc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut pulses0: *const i32,
)
/* I    data: nonnegative pulse amplitudes          */
{
    let mut pulses1: [i32; 8] = [0; 8];
    let mut pulses2: [i32; 4] = [0; 4];
    let mut pulses3: [i32; 2] = [0; 2];
    let mut pulses4: [i32; 1] = [0; 1];
    /* this function operates on one shell code frame of 16 pulses */
    /* tree representation per pulse-subframe */
    combine_pulses(pulses1.as_mut_ptr(), pulses0, 8 as i32);
    combine_pulses(pulses2.as_mut_ptr(), pulses1.as_mut_ptr(), 4 as i32);
    combine_pulses(pulses3.as_mut_ptr(), pulses2.as_mut_ptr(), 2 as i32);
    combine_pulses(pulses4.as_mut_ptr(), pulses3.as_mut_ptr(), 1 as i32);
    encode_split(
        psRangeEnc,
        pulses3[0 as i32 as usize],
        pulses4[0 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table3.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses2[0 as i32 as usize],
        pulses3[0 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table2.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses1[0 as i32 as usize],
        pulses2[0 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(0 as i32 as isize),
        pulses1[0 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(2 as i32 as isize),
        pulses1[1 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses1[2 as i32 as usize],
        pulses2[1 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(4 as i32 as isize),
        pulses1[2 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(6 as i32 as isize),
        pulses1[3 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses2[2 as i32 as usize],
        pulses3[1 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table2.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses1[4 as i32 as usize],
        pulses2[2 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(8 as i32 as isize),
        pulses1[4 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(10 as i32 as isize),
        pulses1[5 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses1[6 as i32 as usize],
        pulses2[3 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(12 as i32 as isize),
        pulses1[6 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(14 as i32 as isize),
        pulses1[7 as i32 as usize],
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
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
/* Shell decoder, operates on one shell code frame of 16 pulses */
#[no_mangle]

pub unsafe extern "C" fn silk_shell_decoder(
    mut pulses0: *mut crate::opus_types_h::opus_int16,
    mut psRangeDec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    pulses4: i32,
)
/* I    number of pulses per pulse-subframe         */
{
    let mut pulses3: [crate::opus_types_h::opus_int16; 2] = [0; 2];
    let mut pulses2: [crate::opus_types_h::opus_int16; 4] = [0; 4];
    let mut pulses1: [crate::opus_types_h::opus_int16; 8] = [0; 8];
    /* this function operates on one shell code frame of 16 pulses */
    decode_split(
        &mut *pulses3.as_mut_ptr().offset(0 as i32 as isize),
        &mut *pulses3.as_mut_ptr().offset(1 as i32 as isize),
        psRangeDec,
        pulses4,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table3.as_ptr(),
    );
    decode_split(
        &mut *pulses2.as_mut_ptr().offset(0 as i32 as isize),
        &mut *pulses2.as_mut_ptr().offset(1 as i32 as isize),
        psRangeDec,
        pulses3[0 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table2.as_ptr(),
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(0 as i32 as isize),
        &mut *pulses1.as_mut_ptr().offset(1 as i32 as isize),
        psRangeDec,
        pulses2[0 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table1.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(0 as i32 as isize),
        &mut *pulses0.offset(1 as i32 as isize),
        psRangeDec,
        pulses1[0 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(2 as i32 as isize),
        &mut *pulses0.offset(3 as i32 as isize),
        psRangeDec,
        pulses1[1 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(2 as i32 as isize),
        &mut *pulses1.as_mut_ptr().offset(3 as i32 as isize),
        psRangeDec,
        pulses2[1 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table1.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(4 as i32 as isize),
        &mut *pulses0.offset(5 as i32 as isize),
        psRangeDec,
        pulses1[2 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(6 as i32 as isize),
        &mut *pulses0.offset(7 as i32 as isize),
        psRangeDec,
        pulses1[3 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses2.as_mut_ptr().offset(2 as i32 as isize),
        &mut *pulses2.as_mut_ptr().offset(3 as i32 as isize),
        psRangeDec,
        pulses3[1 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table2.as_ptr(),
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(4 as i32 as isize),
        &mut *pulses1.as_mut_ptr().offset(5 as i32 as isize),
        psRangeDec,
        pulses2[2 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table1.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(8 as i32 as isize),
        &mut *pulses0.offset(9 as i32 as isize),
        psRangeDec,
        pulses1[4 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(10 as i32 as isize),
        &mut *pulses0.offset(11 as i32 as isize),
        psRangeDec,
        pulses1[5 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(6 as i32 as isize),
        &mut *pulses1.as_mut_ptr().offset(7 as i32 as isize),
        psRangeDec,
        pulses2[3 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table1.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(12 as i32 as isize),
        &mut *pulses0.offset(13 as i32 as isize),
        psRangeDec,
        pulses1[6 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(14 as i32 as isize),
        &mut *pulses0.offset(15 as i32 as isize),
        psRangeDec,
        pulses1[7 as i32 as usize] as i32,
        crate::src::opus_1_2_1::silk::tables_pulses_per_block::silk_shell_code_table0.as_ptr(),
    );
}

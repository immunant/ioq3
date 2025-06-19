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
/* Piece-wise linear mapping from bitrate in kbps to coding quality in dB SNR */
#[no_mangle]

pub static mut silk_TargetRate_table_NB: [opus_int32; 8] = [
    0 as i32,
    8000 as i32,
    9400 as i32,
    11500 as i32,
    13500 as i32,
    17500 as i32,
    25000 as i32,
    80000 as i32,
];
#[no_mangle]

pub static mut silk_TargetRate_table_MB: [opus_int32; 8] = [
    0 as i32,
    9000 as i32,
    12000 as i32,
    14500 as i32,
    18500 as i32,
    24500 as i32,
    35500 as i32,
    80000 as i32,
];
#[no_mangle]

pub static mut silk_TargetRate_table_WB: [opus_int32; 8] = [
    0 as i32,
    10500 as i32,
    14000 as i32,
    17000 as i32,
    21500 as i32,
    28500 as i32,
    42000 as i32,
    80000 as i32,
];
#[no_mangle]

pub static mut silk_SNR_table_Q1: [opus_int16; 8] = [
    18 as i32 as opus_int16,
    29 as i32 as opus_int16,
    38 as i32 as opus_int16,
    40 as i32 as opus_int16,
    46 as i32 as opus_int16,
    52 as i32 as opus_int16,
    62 as i32 as opus_int16,
    84 as i32 as opus_int16,
];
/* Tables for stereo predictor coding */
#[no_mangle]

pub static mut silk_stereo_pred_quant_Q13: [opus_int16; 16] = [
    -(13732 as i32) as opus_int16,
    -(10050 as i32) as opus_int16,
    -(8266 as i32) as opus_int16,
    -(7526 as i32) as opus_int16,
    -(6500 as i32) as opus_int16,
    -(5000 as i32) as opus_int16,
    -(2950 as i32) as opus_int16,
    -(820 as i32) as opus_int16,
    820 as i32 as opus_int16,
    2950 as i32 as opus_int16,
    5000 as i32 as opus_int16,
    6500 as i32 as opus_int16,
    7526 as i32 as opus_int16,
    8266 as i32 as opus_int16,
    10050 as i32 as opus_int16,
    13732 as i32 as opus_int16,
];
#[no_mangle]

pub static mut silk_stereo_pred_joint_iCDF: [u8; 25] = [
    249 as i32 as u8,
    247 as i32 as u8,
    246 as i32 as u8,
    245 as i32 as u8,
    244 as i32 as u8,
    234 as i32 as u8,
    210 as i32 as u8,
    202 as i32 as u8,
    201 as i32 as u8,
    200 as i32 as u8,
    197 as i32 as u8,
    174 as i32 as u8,
    82 as i32 as u8,
    59 as i32 as u8,
    56 as i32 as u8,
    55 as i32 as u8,
    54 as i32 as u8,
    46 as i32 as u8,
    22 as i32 as u8,
    12 as i32 as u8,
    11 as i32 as u8,
    10 as i32 as u8,
    9 as i32 as u8,
    7 as i32 as u8,
    0 as i32 as u8,
];
#[no_mangle]

pub static mut silk_stereo_only_code_mid_iCDF: [u8; 2] = [64 as i32 as u8, 0 as i32 as u8];
/* Tables for LBRR flags */

static mut silk_LBRR_flags_2_iCDF: [u8; 3] = [203 as i32 as u8, 150 as i32 as u8, 0 as i32 as u8];

static mut silk_LBRR_flags_3_iCDF: [u8; 7] = [
    215 as i32 as u8,
    195 as i32 as u8,
    166 as i32 as u8,
    125 as i32 as u8,
    110 as i32 as u8,
    82 as i32 as u8,
    0 as i32 as u8,
];
#[no_mangle]

pub static mut silk_LBRR_flags_iCDF_ptr: [*const u8; 2] = unsafe {
    [
        silk_LBRR_flags_2_iCDF.as_ptr(),
        silk_LBRR_flags_3_iCDF.as_ptr(),
    ]
};
/* Table for LSB coding */
#[no_mangle]

pub static mut silk_lsb_iCDF: [u8; 2] = [120 as i32 as u8, 0 as i32 as u8];
/* Tables for LTPScale */
#[no_mangle]

pub static mut silk_LTPscale_iCDF: [u8; 3] = [128 as i32 as u8, 64 as i32 as u8, 0 as i32 as u8];
/* Tables for signal type and offset coding */
#[no_mangle]

pub static mut silk_type_offset_VAD_iCDF: [u8; 4] = [
    232 as i32 as u8,
    158 as i32 as u8,
    10 as i32 as u8,
    0 as i32 as u8,
];
#[no_mangle]

pub static mut silk_type_offset_no_VAD_iCDF: [u8; 2] = [230 as i32 as u8, 0 as i32 as u8];
/* Tables for NLSF interpolation factor */
#[no_mangle]

pub static mut silk_NLSF_interpolation_factor_iCDF: [u8; 5] = [
    243 as i32 as u8,
    221 as i32 as u8,
    192 as i32 as u8,
    181 as i32 as u8,
    0 as i32 as u8,
];
/* Quantization offsets */
#[no_mangle]

pub static mut silk_Quantization_Offsets_Q10: [[opus_int16; 2]; 2] = [
    [100 as i32 as opus_int16, 240 as i32 as opus_int16],
    [32 as i32 as opus_int16, 100 as i32 as opus_int16],
];
/* Table for LTPScale */
#[no_mangle]

pub static mut silk_LTPScales_table_Q14: [opus_int16; 3] = [
    15565 as i32 as opus_int16,
    12288 as i32 as opus_int16,
    8192 as i32 as opus_int16,
];
/* Uniform entropy tables */
#[no_mangle]

pub static mut silk_uniform3_iCDF: [u8; 3] = [171 as i32 as u8, 85 as i32 as u8, 0 as i32 as u8];
#[no_mangle]

pub static mut silk_uniform4_iCDF: [u8; 4] = [
    192 as i32 as u8,
    128 as i32 as u8,
    64 as i32 as u8,
    0 as i32 as u8,
];
#[no_mangle]

pub static mut silk_uniform5_iCDF: [u8; 5] = [
    205 as i32 as u8,
    154 as i32 as u8,
    102 as i32 as u8,
    51 as i32 as u8,
    0 as i32 as u8,
];
#[no_mangle]

pub static mut silk_uniform6_iCDF: [u8; 6] = [
    213 as i32 as u8,
    171 as i32 as u8,
    128 as i32 as u8,
    85 as i32 as u8,
    43 as i32 as u8,
    0 as i32 as u8,
];
#[no_mangle]

pub static mut silk_uniform8_iCDF: [u8; 8] = [
    224 as i32 as u8,
    192 as i32 as u8,
    160 as i32 as u8,
    128 as i32 as u8,
    96 as i32 as u8,
    64 as i32 as u8,
    32 as i32 as u8,
    0 as i32 as u8,
];
#[no_mangle]

pub static mut silk_NLSF_EXT_iCDF: [u8; 7] = [
    100 as i32 as u8,
    40 as i32 as u8,
    16 as i32 as u8,
    7 as i32 as u8,
    3 as i32 as u8,
    1 as i32 as u8,
    0 as i32 as u8,
];
/*  Elliptic/Cauer filters designed with 0.1 dB passband ripple,
80 dB minimum stopband attenuation, and
[0.95 : 0.15 : 0.35] normalized cut off frequencies. */
/* Interpolation points for filter coefficients used in the bandwidth transition smoother */
#[no_mangle]

pub static mut silk_Transition_LP_B_Q28: [[opus_int32; 3]; 5] = [
    [250767114 as i32, 501534038 as i32, 250767114 as i32],
    [209867381 as i32, 419732057 as i32, 209867381 as i32],
    [170987846 as i32, 341967853 as i32, 170987846 as i32],
    [131531482 as i32, 263046905 as i32, 131531482 as i32],
    [89306658 as i32, 178584282 as i32, 89306658 as i32],
];
/* Interpolation points for filter coefficients used in the bandwidth transition smoother */
#[no_mangle]

pub static mut silk_Transition_LP_A_Q28: [[opus_int32; 2]; 5] = [
    [506393414 as i32, 239854379 as i32],
    [411067935 as i32, 169683996 as i32],
    [306733530 as i32, 116694253 as i32],
    [185807084 as i32, 77959395 as i32],
    [35497197 as i32, 57401098 as i32],
];

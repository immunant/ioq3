pub use crate::opus_types_h::opus_int16;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::int16_t;
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
/* Filter coefficients for IIR/FIR polyphase resampling     *
 * Total size: 179 Words (358 Bytes)                        */
/* Matlab code for the notch filter coefficients: */
/* B = [1, 0.147, 1];  A = [1, 0.107, 0.89]; G = 0.93; freqz(G * B, A, 2^14, 16e3); axis([0, 8000, -10, 1]) */
/* fprintf('\t%6d, %6d, %6d, %6d\n', round(B(2)*2^16), round(-A(2)*2^16), round((1-A(3))*2^16), round(G*2^15)) */
/* const opus_int16 silk_resampler_up2_hq_notch[ 4 ] = { 9634,  -7012,   7209,  30474 }; */
/* Tables with IIR and FIR coefficients for fractional downsamplers (123 Words) */
#[no_mangle]

pub static mut silk_Resampler_3_4_COEFS: [crate::opus_types_h::opus_int16; 29] = [
    -(20694 as i32) as crate::opus_types_h::opus_int16,
    -(13867 as i32) as crate::opus_types_h::opus_int16,
    -(49 as i32) as crate::opus_types_h::opus_int16,
    64 as i32 as crate::opus_types_h::opus_int16,
    17 as i32 as crate::opus_types_h::opus_int16,
    -(157 as i32) as crate::opus_types_h::opus_int16,
    353 as i32 as crate::opus_types_h::opus_int16,
    -(496 as i32) as crate::opus_types_h::opus_int16,
    163 as i32 as crate::opus_types_h::opus_int16,
    11047 as i32 as crate::opus_types_h::opus_int16,
    22205 as i32 as crate::opus_types_h::opus_int16,
    -(39 as i32) as crate::opus_types_h::opus_int16,
    6 as i32 as crate::opus_types_h::opus_int16,
    91 as i32 as crate::opus_types_h::opus_int16,
    -(170 as i32) as crate::opus_types_h::opus_int16,
    186 as i32 as crate::opus_types_h::opus_int16,
    23 as i32 as crate::opus_types_h::opus_int16,
    -(896 as i32) as crate::opus_types_h::opus_int16,
    6336 as i32 as crate::opus_types_h::opus_int16,
    19928 as i32 as crate::opus_types_h::opus_int16,
    -(19 as i32) as crate::opus_types_h::opus_int16,
    -(36 as i32) as crate::opus_types_h::opus_int16,
    102 as i32 as crate::opus_types_h::opus_int16,
    -(89 as i32) as crate::opus_types_h::opus_int16,
    -(24 as i32) as crate::opus_types_h::opus_int16,
    328 as i32 as crate::opus_types_h::opus_int16,
    -(951 as i32) as crate::opus_types_h::opus_int16,
    2568 as i32 as crate::opus_types_h::opus_int16,
    15909 as i32 as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_Resampler_2_3_COEFS: [crate::opus_types_h::opus_int16; 20] = [
    -(14457 as i32) as crate::opus_types_h::opus_int16,
    -(14019 as i32) as crate::opus_types_h::opus_int16,
    64 as i32 as crate::opus_types_h::opus_int16,
    128 as i32 as crate::opus_types_h::opus_int16,
    -(122 as i32) as crate::opus_types_h::opus_int16,
    36 as i32 as crate::opus_types_h::opus_int16,
    310 as i32 as crate::opus_types_h::opus_int16,
    -(768 as i32) as crate::opus_types_h::opus_int16,
    584 as i32 as crate::opus_types_h::opus_int16,
    9267 as i32 as crate::opus_types_h::opus_int16,
    17733 as i32 as crate::opus_types_h::opus_int16,
    12 as i32 as crate::opus_types_h::opus_int16,
    128 as i32 as crate::opus_types_h::opus_int16,
    18 as i32 as crate::opus_types_h::opus_int16,
    -(142 as i32) as crate::opus_types_h::opus_int16,
    288 as i32 as crate::opus_types_h::opus_int16,
    -(117 as i32) as crate::opus_types_h::opus_int16,
    -(865 as i32) as crate::opus_types_h::opus_int16,
    4123 as i32 as crate::opus_types_h::opus_int16,
    14459 as i32 as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_Resampler_1_2_COEFS: [crate::opus_types_h::opus_int16; 14] = [
    616 as i32 as crate::opus_types_h::opus_int16,
    -(14323 as i32) as crate::opus_types_h::opus_int16,
    -(10 as i32) as crate::opus_types_h::opus_int16,
    39 as i32 as crate::opus_types_h::opus_int16,
    58 as i32 as crate::opus_types_h::opus_int16,
    -(46 as i32) as crate::opus_types_h::opus_int16,
    -(84 as i32) as crate::opus_types_h::opus_int16,
    120 as i32 as crate::opus_types_h::opus_int16,
    184 as i32 as crate::opus_types_h::opus_int16,
    -(315 as i32) as crate::opus_types_h::opus_int16,
    -(541 as i32) as crate::opus_types_h::opus_int16,
    1284 as i32 as crate::opus_types_h::opus_int16,
    5380 as i32 as crate::opus_types_h::opus_int16,
    9024 as i32 as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_Resampler_1_3_COEFS: [crate::opus_types_h::opus_int16; 20] = [
    16102 as i32 as crate::opus_types_h::opus_int16,
    -(15162 as i32) as crate::opus_types_h::opus_int16,
    -(13 as i32) as crate::opus_types_h::opus_int16,
    0 as i32 as crate::opus_types_h::opus_int16,
    20 as i32 as crate::opus_types_h::opus_int16,
    26 as i32 as crate::opus_types_h::opus_int16,
    5 as i32 as crate::opus_types_h::opus_int16,
    -(31 as i32) as crate::opus_types_h::opus_int16,
    -(43 as i32) as crate::opus_types_h::opus_int16,
    -(4 as i32) as crate::opus_types_h::opus_int16,
    65 as i32 as crate::opus_types_h::opus_int16,
    90 as i32 as crate::opus_types_h::opus_int16,
    7 as i32 as crate::opus_types_h::opus_int16,
    -(157 as i32) as crate::opus_types_h::opus_int16,
    -(248 as i32) as crate::opus_types_h::opus_int16,
    -(44 as i32) as crate::opus_types_h::opus_int16,
    593 as i32 as crate::opus_types_h::opus_int16,
    1583 as i32 as crate::opus_types_h::opus_int16,
    2612 as i32 as crate::opus_types_h::opus_int16,
    3271 as i32 as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_Resampler_1_4_COEFS: [crate::opus_types_h::opus_int16; 20] = [
    22500 as i32 as crate::opus_types_h::opus_int16,
    -(15099 as i32) as crate::opus_types_h::opus_int16,
    3 as i32 as crate::opus_types_h::opus_int16,
    -(14 as i32) as crate::opus_types_h::opus_int16,
    -(20 as i32) as crate::opus_types_h::opus_int16,
    -(15 as i32) as crate::opus_types_h::opus_int16,
    2 as i32 as crate::opus_types_h::opus_int16,
    25 as i32 as crate::opus_types_h::opus_int16,
    37 as i32 as crate::opus_types_h::opus_int16,
    25 as i32 as crate::opus_types_h::opus_int16,
    -(16 as i32) as crate::opus_types_h::opus_int16,
    -(71 as i32) as crate::opus_types_h::opus_int16,
    -(107 as i32) as crate::opus_types_h::opus_int16,
    -(79 as i32) as crate::opus_types_h::opus_int16,
    50 as i32 as crate::opus_types_h::opus_int16,
    292 as i32 as crate::opus_types_h::opus_int16,
    623 as i32 as crate::opus_types_h::opus_int16,
    982 as i32 as crate::opus_types_h::opus_int16,
    1288 as i32 as crate::opus_types_h::opus_int16,
    1464 as i32 as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_Resampler_1_6_COEFS: [crate::opus_types_h::opus_int16; 20] = [
    27540 as i32 as crate::opus_types_h::opus_int16,
    -(15257 as i32) as crate::opus_types_h::opus_int16,
    17 as i32 as crate::opus_types_h::opus_int16,
    12 as i32 as crate::opus_types_h::opus_int16,
    8 as i32 as crate::opus_types_h::opus_int16,
    1 as i32 as crate::opus_types_h::opus_int16,
    -(10 as i32) as crate::opus_types_h::opus_int16,
    -(22 as i32) as crate::opus_types_h::opus_int16,
    -(30 as i32) as crate::opus_types_h::opus_int16,
    -(32 as i32) as crate::opus_types_h::opus_int16,
    -(22 as i32) as crate::opus_types_h::opus_int16,
    3 as i32 as crate::opus_types_h::opus_int16,
    44 as i32 as crate::opus_types_h::opus_int16,
    100 as i32 as crate::opus_types_h::opus_int16,
    168 as i32 as crate::opus_types_h::opus_int16,
    243 as i32 as crate::opus_types_h::opus_int16,
    317 as i32 as crate::opus_types_h::opus_int16,
    381 as i32 as crate::opus_types_h::opus_int16,
    429 as i32 as crate::opus_types_h::opus_int16,
    455 as i32 as crate::opus_types_h::opus_int16,
];
#[no_mangle]

pub static mut silk_Resampler_2_3_COEFS_LQ: [crate::opus_types_h::opus_int16; 6] = [
    -(2797 as i32) as crate::opus_types_h::opus_int16,
    -(6507 as i32) as crate::opus_types_h::opus_int16,
    4697 as i32 as crate::opus_types_h::opus_int16,
    10739 as i32 as crate::opus_types_h::opus_int16,
    1567 as i32 as crate::opus_types_h::opus_int16,
    8276 as i32 as crate::opus_types_h::opus_int16,
];
/* Table with interplation fractions of 1/24, 3/24, 5/24, ... , 23/24 : 23/24 (46 Words) */
#[no_mangle]

pub static mut silk_resampler_frac_FIR_12: [[crate::opus_types_h::opus_int16; 4]; 12] = [
    [
        189 as i32 as crate::opus_types_h::opus_int16,
        -(600 as i32) as crate::opus_types_h::opus_int16,
        617 as i32 as crate::opus_types_h::opus_int16,
        30567 as i32 as crate::opus_types_h::opus_int16,
    ],
    [
        117 as i32 as crate::opus_types_h::opus_int16,
        -(159 as i32) as crate::opus_types_h::opus_int16,
        -(1070 as i32) as crate::opus_types_h::opus_int16,
        29704 as i32 as crate::opus_types_h::opus_int16,
    ],
    [
        52 as i32 as crate::opus_types_h::opus_int16,
        221 as i32 as crate::opus_types_h::opus_int16,
        -(2392 as i32) as crate::opus_types_h::opus_int16,
        28276 as i32 as crate::opus_types_h::opus_int16,
    ],
    [
        -(4 as i32) as crate::opus_types_h::opus_int16,
        529 as i32 as crate::opus_types_h::opus_int16,
        -(3350 as i32) as crate::opus_types_h::opus_int16,
        26341 as i32 as crate::opus_types_h::opus_int16,
    ],
    [
        -(48 as i32) as crate::opus_types_h::opus_int16,
        758 as i32 as crate::opus_types_h::opus_int16,
        -(3956 as i32) as crate::opus_types_h::opus_int16,
        23973 as i32 as crate::opus_types_h::opus_int16,
    ],
    [
        -(80 as i32) as crate::opus_types_h::opus_int16,
        905 as i32 as crate::opus_types_h::opus_int16,
        -(4235 as i32) as crate::opus_types_h::opus_int16,
        21254 as i32 as crate::opus_types_h::opus_int16,
    ],
    [
        -(99 as i32) as crate::opus_types_h::opus_int16,
        972 as i32 as crate::opus_types_h::opus_int16,
        -(4222 as i32) as crate::opus_types_h::opus_int16,
        18278 as i32 as crate::opus_types_h::opus_int16,
    ],
    [
        -(107 as i32) as crate::opus_types_h::opus_int16,
        967 as i32 as crate::opus_types_h::opus_int16,
        -(3957 as i32) as crate::opus_types_h::opus_int16,
        15143 as i32 as crate::opus_types_h::opus_int16,
    ],
    [
        -(103 as i32) as crate::opus_types_h::opus_int16,
        896 as i32 as crate::opus_types_h::opus_int16,
        -(3487 as i32) as crate::opus_types_h::opus_int16,
        11950 as i32 as crate::opus_types_h::opus_int16,
    ],
    [
        -(91 as i32) as crate::opus_types_h::opus_int16,
        773 as i32 as crate::opus_types_h::opus_int16,
        -(2865 as i32) as crate::opus_types_h::opus_int16,
        8798 as i32 as crate::opus_types_h::opus_int16,
    ],
    [
        -(71 as i32) as crate::opus_types_h::opus_int16,
        611 as i32 as crate::opus_types_h::opus_int16,
        -(2143 as i32) as crate::opus_types_h::opus_int16,
        5784 as i32 as crate::opus_types_h::opus_int16,
    ],
    [
        -(46 as i32) as crate::opus_types_h::opus_int16,
        425 as i32 as crate::opus_types_h::opus_int16,
        -(1375 as i32) as crate::opus_types_h::opus_int16,
        2996 as i32 as crate::opus_types_h::opus_int16,
    ],
];

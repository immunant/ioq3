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

pub static mut silk_pitch_lag_iCDF: [u8; 32] = [
    253 as i32 as u8,
    250 as i32 as u8,
    244 as i32 as u8,
    233 as i32 as u8,
    212 as i32 as u8,
    182 as i32 as u8,
    150 as i32 as u8,
    131 as i32 as u8,
    120 as i32 as u8,
    110 as i32 as u8,
    98 as i32 as u8,
    85 as i32 as u8,
    72 as i32 as u8,
    60 as i32 as u8,
    49 as i32 as u8,
    40 as i32 as u8,
    32 as i32 as u8,
    25 as i32 as u8,
    19 as i32 as u8,
    15 as i32 as u8,
    13 as i32 as u8,
    11 as i32 as u8,
    9 as i32 as u8,
    8 as i32 as u8,
    7 as i32 as u8,
    6 as i32 as u8,
    5 as i32 as u8,
    4 as i32 as u8,
    3 as i32 as u8,
    2 as i32 as u8,
    1 as i32 as u8,
    0 as i32 as u8,
];
#[no_mangle]

pub static mut silk_pitch_delta_iCDF: [u8; 21] = [
    210 as i32 as u8,
    208 as i32 as u8,
    206 as i32 as u8,
    203 as i32 as u8,
    199 as i32 as u8,
    193 as i32 as u8,
    183 as i32 as u8,
    168 as i32 as u8,
    142 as i32 as u8,
    104 as i32 as u8,
    74 as i32 as u8,
    52 as i32 as u8,
    37 as i32 as u8,
    27 as i32 as u8,
    20 as i32 as u8,
    14 as i32 as u8,
    10 as i32 as u8,
    6 as i32 as u8,
    4 as i32 as u8,
    2 as i32 as u8,
    0 as i32 as u8,
];
#[no_mangle]

pub static mut silk_pitch_contour_iCDF: [u8; 34] = [
    223 as i32 as u8,
    201 as i32 as u8,
    183 as i32 as u8,
    167 as i32 as u8,
    152 as i32 as u8,
    138 as i32 as u8,
    124 as i32 as u8,
    111 as i32 as u8,
    98 as i32 as u8,
    88 as i32 as u8,
    79 as i32 as u8,
    70 as i32 as u8,
    62 as i32 as u8,
    56 as i32 as u8,
    50 as i32 as u8,
    44 as i32 as u8,
    39 as i32 as u8,
    35 as i32 as u8,
    31 as i32 as u8,
    27 as i32 as u8,
    24 as i32 as u8,
    21 as i32 as u8,
    18 as i32 as u8,
    16 as i32 as u8,
    14 as i32 as u8,
    12 as i32 as u8,
    10 as i32 as u8,
    8 as i32 as u8,
    6 as i32 as u8,
    4 as i32 as u8,
    3 as i32 as u8,
    2 as i32 as u8,
    1 as i32 as u8,
    0 as i32 as u8,
];
#[no_mangle]

pub static mut silk_pitch_contour_NB_iCDF: [u8; 11] = [
    188 as i32 as u8,
    176 as i32 as u8,
    155 as i32 as u8,
    138 as i32 as u8,
    119 as i32 as u8,
    97 as i32 as u8,
    67 as i32 as u8,
    43 as i32 as u8,
    26 as i32 as u8,
    10 as i32 as u8,
    0 as i32 as u8,
];
#[no_mangle]

pub static mut silk_pitch_contour_10_ms_iCDF: [u8; 12] = [
    165 as i32 as u8,
    119 as i32 as u8,
    80 as i32 as u8,
    61 as i32 as u8,
    47 as i32 as u8,
    35 as i32 as u8,
    27 as i32 as u8,
    20 as i32 as u8,
    14 as i32 as u8,
    9 as i32 as u8,
    4 as i32 as u8,
    0 as i32 as u8,
];
#[no_mangle]

pub static mut silk_pitch_contour_10_ms_NB_iCDF: [u8; 3] =
    [113 as i32 as u8, 63 as i32 as u8, 0 as i32 as u8];

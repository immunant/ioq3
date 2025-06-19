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

pub static mut silk_gain_iCDF: [[u8; 8]; 3] = [
    [
        224 as i32 as u8,
        112 as i32 as u8,
        44 as i32 as u8,
        15 as i32 as u8,
        3 as i32 as u8,
        2 as i32 as u8,
        1 as i32 as u8,
        0 as i32 as u8,
    ],
    [
        254 as i32 as u8,
        237 as i32 as u8,
        192 as i32 as u8,
        132 as i32 as u8,
        70 as i32 as u8,
        23 as i32 as u8,
        4 as i32 as u8,
        0 as i32 as u8,
    ],
    [
        255 as i32 as u8,
        252 as i32 as u8,
        226 as i32 as u8,
        155 as i32 as u8,
        61 as i32 as u8,
        11 as i32 as u8,
        2 as i32 as u8,
        0 as i32 as u8,
    ],
];
#[no_mangle]

pub static mut silk_delta_gain_iCDF: [u8; 41] = [
    250 as i32 as u8,
    245 as i32 as u8,
    234 as i32 as u8,
    203 as i32 as u8,
    71 as i32 as u8,
    50 as i32 as u8,
    42 as i32 as u8,
    38 as i32 as u8,
    35 as i32 as u8,
    33 as i32 as u8,
    31 as i32 as u8,
    29 as i32 as u8,
    28 as i32 as u8,
    27 as i32 as u8,
    26 as i32 as u8,
    25 as i32 as u8,
    24 as i32 as u8,
    23 as i32 as u8,
    22 as i32 as u8,
    21 as i32 as u8,
    20 as i32 as u8,
    19 as i32 as u8,
    18 as i32 as u8,
    17 as i32 as u8,
    16 as i32 as u8,
    15 as i32 as u8,
    14 as i32 as u8,
    13 as i32 as u8,
    12 as i32 as u8,
    11 as i32 as u8,
    10 as i32 as u8,
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

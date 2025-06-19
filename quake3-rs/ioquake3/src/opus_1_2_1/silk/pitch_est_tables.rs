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

pub static mut silk_CB_lags_stage2_10_ms: [[i8; 3]; 2] = [
    [0 as i32 as i8, 1 as i32 as i8, 0 as i32 as i8],
    [0 as i32 as i8, 0 as i32 as i8, 1 as i32 as i8],
];
#[no_mangle]

pub static mut silk_CB_lags_stage3_10_ms: [[i8; 12]; 2] = [
    [
        0 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        -(1 as i32) as i8,
        1 as i32 as i8,
        -(1 as i32) as i8,
        2 as i32 as i8,
        -(2 as i32) as i8,
        2 as i32 as i8,
        -(2 as i32) as i8,
        3 as i32 as i8,
        -(3 as i32) as i8,
    ],
    [
        0 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        -(1 as i32) as i8,
        2 as i32 as i8,
        -(1 as i32) as i8,
        2 as i32 as i8,
        -(2 as i32) as i8,
        3 as i32 as i8,
        -(2 as i32) as i8,
        3 as i32 as i8,
    ],
];
#[no_mangle]

pub static mut silk_Lag_range_stage3_10_ms: [[i8; 2]; 2] = [
    [-(3 as i32) as i8, 7 as i32 as i8],
    [-(2 as i32) as i8, 7 as i32 as i8],
];
#[no_mangle]

pub static mut silk_CB_lags_stage2: [[i8; 11]; 4] = [
    [
        0 as i32 as i8,
        2 as i32 as i8,
        -(1 as i32) as i8,
        -(1 as i32) as i8,
        -(1 as i32) as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
    ],
    [
        0 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
    ],
    [
        0 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
    ],
    [
        0 as i32 as i8,
        -(1 as i32) as i8,
        2 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        -(1 as i32) as i8,
        -(1 as i32) as i8,
    ],
];
#[no_mangle]

pub static mut silk_CB_lags_stage3: [[i8; 34]; 4] = [
    [
        0 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        -(1 as i32) as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        -(1 as i32) as i8,
        0 as i32 as i8,
        -(1 as i32) as i8,
        1 as i32 as i8,
        -(2 as i32) as i8,
        2 as i32 as i8,
        -(2 as i32) as i8,
        -(2 as i32) as i8,
        2 as i32 as i8,
        -(3 as i32) as i8,
        2 as i32 as i8,
        3 as i32 as i8,
        -(3 as i32) as i8,
        -(4 as i32) as i8,
        3 as i32 as i8,
        -(4 as i32) as i8,
        4 as i32 as i8,
        4 as i32 as i8,
        -(5 as i32) as i8,
        5 as i32 as i8,
        -(6 as i32) as i8,
        -(5 as i32) as i8,
        6 as i32 as i8,
        -(7 as i32) as i8,
        6 as i32 as i8,
        5 as i32 as i8,
        8 as i32 as i8,
        -(9 as i32) as i8,
    ],
    [
        0 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        -(1 as i32) as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        -(1 as i32) as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        -(1 as i32) as i8,
        -(1 as i32) as i8,
        1 as i32 as i8,
        -(1 as i32) as i8,
        2 as i32 as i8,
        1 as i32 as i8,
        -(1 as i32) as i8,
        2 as i32 as i8,
        -(2 as i32) as i8,
        -(2 as i32) as i8,
        2 as i32 as i8,
        -(2 as i32) as i8,
        2 as i32 as i8,
        2 as i32 as i8,
        3 as i32 as i8,
        -(3 as i32) as i8,
    ],
    [
        0 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        -(1 as i32) as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        2 as i32 as i8,
        1 as i32 as i8,
        -(1 as i32) as i8,
        2 as i32 as i8,
        -(1 as i32) as i8,
        -(1 as i32) as i8,
        2 as i32 as i8,
        -(1 as i32) as i8,
        2 as i32 as i8,
        2 as i32 as i8,
        -(1 as i32) as i8,
        3 as i32 as i8,
        -(2 as i32) as i8,
        -(2 as i32) as i8,
        -(2 as i32) as i8,
        3 as i32 as i8,
    ],
    [
        0 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        0 as i32 as i8,
        1 as i32 as i8,
        -(1 as i32) as i8,
        2 as i32 as i8,
        -(1 as i32) as i8,
        2 as i32 as i8,
        -(1 as i32) as i8,
        2 as i32 as i8,
        3 as i32 as i8,
        -(2 as i32) as i8,
        3 as i32 as i8,
        -(2 as i32) as i8,
        -(2 as i32) as i8,
        4 as i32 as i8,
        4 as i32 as i8,
        -(3 as i32) as i8,
        5 as i32 as i8,
        -(3 as i32) as i8,
        -(4 as i32) as i8,
        6 as i32 as i8,
        -(4 as i32) as i8,
        6 as i32 as i8,
        5 as i32 as i8,
        -(5 as i32) as i8,
        8 as i32 as i8,
        -(6 as i32) as i8,
        -(5 as i32) as i8,
        -(7 as i32) as i8,
        9 as i32 as i8,
    ],
];
#[no_mangle]

pub static mut silk_Lag_range_stage3: [[[i8; 2]; 4]; 3] = [
    [
        [-(5 as i32) as i8, 8 as i32 as i8],
        [-(1 as i32) as i8, 6 as i32 as i8],
        [-(1 as i32) as i8, 6 as i32 as i8],
        [-(4 as i32) as i8, 10 as i32 as i8],
    ],
    [
        [-(6 as i32) as i8, 10 as i32 as i8],
        [-(2 as i32) as i8, 6 as i32 as i8],
        [-(1 as i32) as i8, 6 as i32 as i8],
        [-(5 as i32) as i8, 10 as i32 as i8],
    ],
    [
        [-(9 as i32) as i8, 12 as i32 as i8],
        [-(3 as i32) as i8, 7 as i32 as i8],
        [-(2 as i32) as i8, 7 as i32 as i8],
        [-(7 as i32) as i8, 13 as i32 as i8],
    ],
];
#[no_mangle]

pub static mut silk_nb_cbk_searchs_stage3: [i8; 3] =
    [16 as i32 as i8, 24 as i32 as i8, 34 as i32 as i8];

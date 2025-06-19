use ::libc;
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

pub static mut silk_LTP_per_index_iCDF: [u8; 3] = [
    179 as i32 as u8,
    99 as i32 as u8,
    0 as i32 as u8,
];

static mut silk_LTP_gain_iCDF_0: [u8; 8] = [
    71 as i32 as u8,
    56 as i32 as u8,
    43 as i32 as u8,
    30 as i32 as u8,
    21 as i32 as u8,
    12 as i32 as u8,
    6 as i32 as u8,
    0 as i32 as u8,
];

static mut silk_LTP_gain_iCDF_1: [u8; 16] = [
    199 as i32 as u8,
    165 as i32 as u8,
    144 as i32 as u8,
    124 as i32 as u8,
    109 as i32 as u8,
    96 as i32 as u8,
    84 as i32 as u8,
    71 as i32 as u8,
    61 as i32 as u8,
    51 as i32 as u8,
    42 as i32 as u8,
    32 as i32 as u8,
    23 as i32 as u8,
    15 as i32 as u8,
    8 as i32 as u8,
    0 as i32 as u8,
];

static mut silk_LTP_gain_iCDF_2: [u8; 32] = [
    241 as i32 as u8,
    225 as i32 as u8,
    211 as i32 as u8,
    199 as i32 as u8,
    187 as i32 as u8,
    175 as i32 as u8,
    164 as i32 as u8,
    153 as i32 as u8,
    142 as i32 as u8,
    132 as i32 as u8,
    123 as i32 as u8,
    114 as i32 as u8,
    105 as i32 as u8,
    96 as i32 as u8,
    88 as i32 as u8,
    80 as i32 as u8,
    72 as i32 as u8,
    64 as i32 as u8,
    57 as i32 as u8,
    50 as i32 as u8,
    44 as i32 as u8,
    38 as i32 as u8,
    33 as i32 as u8,
    29 as i32 as u8,
    24 as i32 as u8,
    20 as i32 as u8,
    16 as i32 as u8,
    12 as i32 as u8,
    9 as i32 as u8,
    5 as i32 as u8,
    2 as i32 as u8,
    0 as i32 as u8,
];

static mut silk_LTP_gain_BITS_Q5_0: [u8; 8] = [
    15 as i32 as u8,
    131 as i32 as u8,
    138 as i32 as u8,
    138 as i32 as u8,
    155 as i32 as u8,
    155 as i32 as u8,
    173 as i32 as u8,
    173 as i32 as u8,
];

static mut silk_LTP_gain_BITS_Q5_1: [u8; 16] = [
    69 as i32 as u8,
    93 as i32 as u8,
    115 as i32 as u8,
    118 as i32 as u8,
    131 as i32 as u8,
    138 as i32 as u8,
    141 as i32 as u8,
    138 as i32 as u8,
    150 as i32 as u8,
    150 as i32 as u8,
    155 as i32 as u8,
    150 as i32 as u8,
    155 as i32 as u8,
    160 as i32 as u8,
    166 as i32 as u8,
    160 as i32 as u8,
];

static mut silk_LTP_gain_BITS_Q5_2: [u8; 32] = [
    131 as i32 as u8,
    128 as i32 as u8,
    134 as i32 as u8,
    141 as i32 as u8,
    141 as i32 as u8,
    141 as i32 as u8,
    145 as i32 as u8,
    145 as i32 as u8,
    145 as i32 as u8,
    150 as i32 as u8,
    155 as i32 as u8,
    155 as i32 as u8,
    155 as i32 as u8,
    155 as i32 as u8,
    160 as i32 as u8,
    160 as i32 as u8,
    160 as i32 as u8,
    160 as i32 as u8,
    166 as i32 as u8,
    166 as i32 as u8,
    173 as i32 as u8,
    173 as i32 as u8,
    182 as i32 as u8,
    192 as i32 as u8,
    182 as i32 as u8,
    192 as i32 as u8,
    192 as i32 as u8,
    192 as i32 as u8,
    205 as i32 as u8,
    192 as i32 as u8,
    205 as i32 as u8,
    224 as i32 as u8,
];
#[no_mangle]

pub static mut silk_LTP_gain_iCDF_ptrs: [*const u8; 3] = unsafe {
    [
        silk_LTP_gain_iCDF_0.as_ptr(),
        silk_LTP_gain_iCDF_1.as_ptr(),
        silk_LTP_gain_iCDF_2.as_ptr(),
    ]
};
#[no_mangle]

pub static mut silk_LTP_gain_BITS_Q5_ptrs: [*const u8; 3] = unsafe {
    [
        silk_LTP_gain_BITS_Q5_0.as_ptr(),
        silk_LTP_gain_BITS_Q5_1.as_ptr(),
        silk_LTP_gain_BITS_Q5_2.as_ptr(),
    ]
};

static mut silk_LTP_gain_vq_0: [[i8; 5]; 8] = [
    [
        4 as i32 as i8,
        6 as i32 as i8,
        24 as i32 as i8,
        7 as i32 as i8,
        5 as i32 as i8,
    ],
    [
        0 as i32 as i8,
        0 as i32 as i8,
        2 as i32 as i8,
        0 as i32 as i8,
        0 as i32 as i8,
    ],
    [
        12 as i32 as i8,
        28 as i32 as i8,
        41 as i32 as i8,
        13 as i32 as i8,
        -(4 as i32) as i8,
    ],
    [
        -(9 as i32) as i8,
        15 as i32 as i8,
        42 as i32 as i8,
        25 as i32 as i8,
        14 as i32 as i8,
    ],
    [
        1 as i32 as i8,
        -(2 as i32) as i8,
        62 as i32 as i8,
        41 as i32 as i8,
        -(9 as i32) as i8,
    ],
    [
        -(10 as i32) as i8,
        37 as i32 as i8,
        65 as i32 as i8,
        -(4 as i32) as i8,
        3 as i32 as i8,
    ],
    [
        -(6 as i32) as i8,
        4 as i32 as i8,
        66 as i32 as i8,
        7 as i32 as i8,
        -(8 as i32) as i8,
    ],
    [
        16 as i32 as i8,
        14 as i32 as i8,
        38 as i32 as i8,
        -(3 as i32) as i8,
        33 as i32 as i8,
    ],
];

static mut silk_LTP_gain_vq_1: [[i8; 5]; 16] = [
    [
        13 as i32 as i8,
        22 as i32 as i8,
        39 as i32 as i8,
        23 as i32 as i8,
        12 as i32 as i8,
    ],
    [
        -(1 as i32) as i8,
        36 as i32 as i8,
        64 as i32 as i8,
        27 as i32 as i8,
        -(6 as i32) as i8,
    ],
    [
        -(7 as i32) as i8,
        10 as i32 as i8,
        55 as i32 as i8,
        43 as i32 as i8,
        17 as i32 as i8,
    ],
    [
        1 as i32 as i8,
        1 as i32 as i8,
        8 as i32 as i8,
        1 as i32 as i8,
        1 as i32 as i8,
    ],
    [
        6 as i32 as i8,
        -(11 as i32) as i8,
        74 as i32 as i8,
        53 as i32 as i8,
        -(9 as i32) as i8,
    ],
    [
        -(12 as i32) as i8,
        55 as i32 as i8,
        76 as i32 as i8,
        -(12 as i32) as i8,
        8 as i32 as i8,
    ],
    [
        -(3 as i32) as i8,
        3 as i32 as i8,
        93 as i32 as i8,
        27 as i32 as i8,
        -(4 as i32) as i8,
    ],
    [
        26 as i32 as i8,
        39 as i32 as i8,
        59 as i32 as i8,
        3 as i32 as i8,
        -(8 as i32) as i8,
    ],
    [
        2 as i32 as i8,
        0 as i32 as i8,
        77 as i32 as i8,
        11 as i32 as i8,
        9 as i32 as i8,
    ],
    [
        -(8 as i32) as i8,
        22 as i32 as i8,
        44 as i32 as i8,
        -(6 as i32) as i8,
        7 as i32 as i8,
    ],
    [
        40 as i32 as i8,
        9 as i32 as i8,
        26 as i32 as i8,
        3 as i32 as i8,
        9 as i32 as i8,
    ],
    [
        -(7 as i32) as i8,
        20 as i32 as i8,
        101 as i32 as i8,
        -(7 as i32) as i8,
        4 as i32 as i8,
    ],
    [
        3 as i32 as i8,
        -(8 as i32) as i8,
        42 as i32 as i8,
        26 as i32 as i8,
        0 as i32 as i8,
    ],
    [
        -(15 as i32) as i8,
        33 as i32 as i8,
        68 as i32 as i8,
        2 as i32 as i8,
        23 as i32 as i8,
    ],
    [
        -(2 as i32) as i8,
        55 as i32 as i8,
        46 as i32 as i8,
        -(2 as i32) as i8,
        15 as i32 as i8,
    ],
    [
        3 as i32 as i8,
        -(1 as i32) as i8,
        21 as i32 as i8,
        16 as i32 as i8,
        41 as i32 as i8,
    ],
];

static mut silk_LTP_gain_vq_2: [[i8; 5]; 32] = [
    [
        -(6 as i32) as i8,
        27 as i32 as i8,
        61 as i32 as i8,
        39 as i32 as i8,
        5 as i32 as i8,
    ],
    [
        -(11 as i32) as i8,
        42 as i32 as i8,
        88 as i32 as i8,
        4 as i32 as i8,
        1 as i32 as i8,
    ],
    [
        -(2 as i32) as i8,
        60 as i32 as i8,
        65 as i32 as i8,
        6 as i32 as i8,
        -(4 as i32) as i8,
    ],
    [
        -(1 as i32) as i8,
        -(5 as i32) as i8,
        73 as i32 as i8,
        56 as i32 as i8,
        1 as i32 as i8,
    ],
    [
        -(9 as i32) as i8,
        19 as i32 as i8,
        94 as i32 as i8,
        29 as i32 as i8,
        -(9 as i32) as i8,
    ],
    [
        0 as i32 as i8,
        12 as i32 as i8,
        99 as i32 as i8,
        6 as i32 as i8,
        4 as i32 as i8,
    ],
    [
        8 as i32 as i8,
        -(19 as i32) as i8,
        102 as i32 as i8,
        46 as i32 as i8,
        -(13 as i32) as i8,
    ],
    [
        3 as i32 as i8,
        2 as i32 as i8,
        13 as i32 as i8,
        3 as i32 as i8,
        2 as i32 as i8,
    ],
    [
        9 as i32 as i8,
        -(21 as i32) as i8,
        84 as i32 as i8,
        72 as i32 as i8,
        -(18 as i32) as i8,
    ],
    [
        -(11 as i32) as i8,
        46 as i32 as i8,
        104 as i32 as i8,
        -(22 as i32) as i8,
        8 as i32 as i8,
    ],
    [
        18 as i32 as i8,
        38 as i32 as i8,
        48 as i32 as i8,
        23 as i32 as i8,
        0 as i32 as i8,
    ],
    [
        -(16 as i32) as i8,
        70 as i32 as i8,
        83 as i32 as i8,
        -(21 as i32) as i8,
        11 as i32 as i8,
    ],
    [
        5 as i32 as i8,
        -(11 as i32) as i8,
        117 as i32 as i8,
        22 as i32 as i8,
        -(8 as i32) as i8,
    ],
    [
        -(6 as i32) as i8,
        23 as i32 as i8,
        117 as i32 as i8,
        -(12 as i32) as i8,
        3 as i32 as i8,
    ],
    [
        3 as i32 as i8,
        -(8 as i32) as i8,
        95 as i32 as i8,
        28 as i32 as i8,
        4 as i32 as i8,
    ],
    [
        -(10 as i32) as i8,
        15 as i32 as i8,
        77 as i32 as i8,
        60 as i32 as i8,
        -(15 as i32) as i8,
    ],
    [
        -(1 as i32) as i8,
        4 as i32 as i8,
        124 as i32 as i8,
        2 as i32 as i8,
        -(4 as i32) as i8,
    ],
    [
        3 as i32 as i8,
        38 as i32 as i8,
        84 as i32 as i8,
        24 as i32 as i8,
        -(25 as i32) as i8,
    ],
    [
        2 as i32 as i8,
        13 as i32 as i8,
        42 as i32 as i8,
        13 as i32 as i8,
        31 as i32 as i8,
    ],
    [
        21 as i32 as i8,
        -(4 as i32) as i8,
        56 as i32 as i8,
        46 as i32 as i8,
        -(1 as i32) as i8,
    ],
    [
        -(1 as i32) as i8,
        35 as i32 as i8,
        79 as i32 as i8,
        -(13 as i32) as i8,
        19 as i32 as i8,
    ],
    [
        -(7 as i32) as i8,
        65 as i32 as i8,
        88 as i32 as i8,
        -(9 as i32) as i8,
        -(14 as i32) as i8,
    ],
    [
        20 as i32 as i8,
        4 as i32 as i8,
        81 as i32 as i8,
        49 as i32 as i8,
        -(29 as i32) as i8,
    ],
    [
        20 as i32 as i8,
        0 as i32 as i8,
        75 as i32 as i8,
        3 as i32 as i8,
        -(17 as i32) as i8,
    ],
    [
        5 as i32 as i8,
        -(9 as i32) as i8,
        44 as i32 as i8,
        92 as i32 as i8,
        -(8 as i32) as i8,
    ],
    [
        1 as i32 as i8,
        -(3 as i32) as i8,
        22 as i32 as i8,
        69 as i32 as i8,
        31 as i32 as i8,
    ],
    [
        -(6 as i32) as i8,
        95 as i32 as i8,
        41 as i32 as i8,
        -(12 as i32) as i8,
        5 as i32 as i8,
    ],
    [
        39 as i32 as i8,
        67 as i32 as i8,
        16 as i32 as i8,
        -(4 as i32) as i8,
        1 as i32 as i8,
    ],
    [
        0 as i32 as i8,
        -(6 as i32) as i8,
        120 as i32 as i8,
        55 as i32 as i8,
        -(36 as i32) as i8,
    ],
    [
        -(13 as i32) as i8,
        44 as i32 as i8,
        122 as i32 as i8,
        4 as i32 as i8,
        -(24 as i32) as i8,
    ],
    [
        81 as i32 as i8,
        5 as i32 as i8,
        11 as i32 as i8,
        3 as i32 as i8,
        7 as i32 as i8,
    ],
    [
        2 as i32 as i8,
        0 as i32 as i8,
        9 as i32 as i8,
        10 as i32 as i8,
        88 as i32 as i8,
    ],
];
// Initialized in run_static_initializers
#[no_mangle]

pub static mut silk_LTP_vq_ptrs_Q7: [*const i8; 3] = [0 as *const i8; 3];
/* Maximum frequency-dependent response of the pitch taps above,
computed as max(abs(freqz(taps))) */

static mut silk_LTP_gain_vq_0_gain: [u8; 8] = [
    46 as i32 as u8,
    2 as i32 as u8,
    90 as i32 as u8,
    87 as i32 as u8,
    93 as i32 as u8,
    91 as i32 as u8,
    82 as i32 as u8,
    98 as i32 as u8,
];

static mut silk_LTP_gain_vq_1_gain: [u8; 16] = [
    109 as i32 as u8,
    120 as i32 as u8,
    118 as i32 as u8,
    12 as i32 as u8,
    113 as i32 as u8,
    115 as i32 as u8,
    117 as i32 as u8,
    119 as i32 as u8,
    99 as i32 as u8,
    59 as i32 as u8,
    87 as i32 as u8,
    111 as i32 as u8,
    63 as i32 as u8,
    111 as i32 as u8,
    112 as i32 as u8,
    80 as i32 as u8,
];

static mut silk_LTP_gain_vq_2_gain: [u8; 32] = [
    126 as i32 as u8,
    124 as i32 as u8,
    125 as i32 as u8,
    124 as i32 as u8,
    129 as i32 as u8,
    121 as i32 as u8,
    126 as i32 as u8,
    23 as i32 as u8,
    132 as i32 as u8,
    127 as i32 as u8,
    127 as i32 as u8,
    127 as i32 as u8,
    126 as i32 as u8,
    127 as i32 as u8,
    122 as i32 as u8,
    133 as i32 as u8,
    130 as i32 as u8,
    134 as i32 as u8,
    101 as i32 as u8,
    118 as i32 as u8,
    119 as i32 as u8,
    145 as i32 as u8,
    126 as i32 as u8,
    86 as i32 as u8,
    124 as i32 as u8,
    120 as i32 as u8,
    123 as i32 as u8,
    119 as i32 as u8,
    170 as i32 as u8,
    173 as i32 as u8,
    107 as i32 as u8,
    109 as i32 as u8,
];
// Initialized in run_static_initializers
#[no_mangle]

pub static mut silk_LTP_vq_gain_ptrs_Q7: [*const u8; 3] = [0 as *const u8; 3];
#[no_mangle]

pub static mut silk_LTP_vq_sizes: [i8; 3] = [
    8 as i32 as i8,
    16 as i32 as i8,
    32 as i32 as i8,
];
unsafe extern "C" fn run_static_initializers() {
    silk_LTP_vq_ptrs_Q7 = [
        &*(*silk_LTP_gain_vq_0
            .as_ptr()
            .offset(0 as i32 as isize))
        .as_ptr()
        .offset(0 as i32 as isize) as *const i8 as *mut i8
            as *const i8,
        &*(*silk_LTP_gain_vq_1
            .as_ptr()
            .offset(0 as i32 as isize))
        .as_ptr()
        .offset(0 as i32 as isize) as *const i8 as *mut i8
            as *const i8,
        &*(*silk_LTP_gain_vq_2
            .as_ptr()
            .offset(0 as i32 as isize))
        .as_ptr()
        .offset(0 as i32 as isize) as *const i8 as *mut i8
            as *const i8,
    ];
    silk_LTP_vq_gain_ptrs_Q7 = [
        &*silk_LTP_gain_vq_0_gain
            .as_ptr()
            .offset(0 as i32 as isize) as *const u8,
        &*silk_LTP_gain_vq_1_gain
            .as_ptr()
            .offset(0 as i32 as isize) as *const u8,
        &*silk_LTP_gain_vq_2_gain
            .as_ptr()
            .offset(0 as i32 as isize) as *const u8,
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

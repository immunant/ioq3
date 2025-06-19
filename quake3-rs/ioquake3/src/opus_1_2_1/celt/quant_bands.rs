use ::libc;

pub mod entcode_h {
    /*OPT: ec_window must be at least 32 bits, but if you have fast arithmetic on a
    larger type, you can speed up the decoder by using it here.*/

    /*The number of bits to use for the range-coded part of unsigned integers.*/
    /*The resolution of fractional-precision bit usage measurements, i.e.,
    3 => 1/8th bits.*/
    /*The entropy encoder/decoder context.
    We use the same structure for both, so that common functions like ec_tell()
     can be used on either one.*/

    #[inline]

    pub unsafe extern "C" fn ec_range_bytes(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> crate::opus_types_h::opus_uint32 {
        return (*_this).offs;
    }
    #[inline]

    pub unsafe extern "C" fn ec_get_buffer(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> *mut u8 {
        return (*_this).buf;
    }
    /*Returns the number of bits "used" by the encoded or decoded symbols so far.
    This same number can be computed in either the encoder or the decoder, and is
     suitable for making coding decisions.
    Return: The number of bits.
            This will always be slightly larger than the exact value (e.g., all
             rounding error is in the positive direction).*/
    #[inline]

    pub unsafe extern "C" fn ec_tell(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> i32 {
        return (*_this).nbits_total
            - (::std::mem::size_of::<u32>() as usize as i32 * 8 as i32
                - (*_this).rng.leading_zeros() as i32);
    }
}

pub mod mathops_h {

    #[inline]

    pub unsafe extern "C" fn celt_log2(mut x: f32) -> f32 {
        let mut integer: i32 = 0;
        let mut frac: f32 = 0.;
        let mut in_0: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        in_0.f = x;
        integer = (in_0.i >> 23 as i32).wrapping_sub(127 as i32 as u32) as i32;
        in_0.i = (in_0.i as u32).wrapping_sub((integer << 23 as i32) as u32)
            as crate::opus_types_h::opus_uint32
            as crate::opus_types_h::opus_uint32;
        frac = in_0.f - 1.5f32;
        frac = -0.41445418f32
            + frac * (0.95909232f32 + frac * (-0.33951290f32 + frac * 0.16541097f32));
        return (1 as i32 + integer) as f32 + frac;
    }

    /* MATHOPS_H */
    /* FIXED_POINT */
}

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::celt_ener;
pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;
pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;

pub use crate::mathops_h::C2RustUnnamed_61;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_tell_frac;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::quant_bands::entcode_h::ec_get_buffer;
pub use crate::src::opus_1_2_1::celt::quant_bands::entcode_h::ec_range_bytes;
pub use crate::src::opus_1_2_1::celt::quant_bands::entcode_h::ec_tell;
pub use crate::src::opus_1_2_1::celt::quant_bands::mathops_h::celt_log2;

/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Written by Jean-Marc Valin */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* Mean energy in each band quantized in Q4 and converted back to float */
#[no_mangle]

pub static mut eMeans: [opus_val16; 25] = [
    6.437500f32,
    6.250000f32,
    5.750000f32,
    5.312500f32,
    5.062500f32,
    4.812500f32,
    4.500000f32,
    4.375000f32,
    4.875000f32,
    4.687500f32,
    4.562500f32,
    4.437500f32,
    4.875000f32,
    4.625000f32,
    4.312500f32,
    4.500000f32,
    4.375000f32,
    4.625000f32,
    4.750000f32,
    4.437500f32,
    3.750000f32,
    3.750000f32,
    3.750000f32,
    3.750000f32,
    3.750000f32,
];
/* prediction coefficients: 0.9, 0.8, 0.65, 0.5 */

static mut pred_coef: [opus_val16; 4] = [
    (29440 as i32 as f64 / 32768.0f64) as opus_val16,
    (26112 as i32 as f64 / 32768.0f64) as opus_val16,
    (21248 as i32 as f64 / 32768.0f64) as opus_val16,
    (16384 as i32 as f64 / 32768.0f64) as opus_val16,
];

static mut beta_coef: [opus_val16; 4] = [
    (30147 as i32 as f64 / 32768.0f64) as opus_val16,
    (22282 as i32 as f64 / 32768.0f64) as opus_val16,
    (12124 as i32 as f64 / 32768.0f64) as opus_val16,
    (6554 as i32 as f64 / 32768.0f64) as opus_val16,
];

static mut beta_intra: opus_val16 = (4915 as i32 as f64 / 32768.0f64) as opus_val16;
/*Parameters of the Laplace-like probability models used for the coarse energy.
There is one pair of parameters for each frame size, prediction type
 (inter/intra), and band number.
The first number of each pair is the probability of 0, and the second is the
 decay rate, both in Q8 precision.*/

static mut e_prob_model: [[[u8; 42]; 2]; 4] = [
    [
        [
            72 as i32 as u8,
            127 as i32 as u8,
            65 as i32 as u8,
            129 as i32 as u8,
            66 as i32 as u8,
            128 as i32 as u8,
            65 as i32 as u8,
            128 as i32 as u8,
            64 as i32 as u8,
            128 as i32 as u8,
            62 as i32 as u8,
            128 as i32 as u8,
            64 as i32 as u8,
            128 as i32 as u8,
            64 as i32 as u8,
            128 as i32 as u8,
            92 as i32 as u8,
            78 as i32 as u8,
            92 as i32 as u8,
            79 as i32 as u8,
            92 as i32 as u8,
            78 as i32 as u8,
            90 as i32 as u8,
            79 as i32 as u8,
            116 as i32 as u8,
            41 as i32 as u8,
            115 as i32 as u8,
            40 as i32 as u8,
            114 as i32 as u8,
            40 as i32 as u8,
            132 as i32 as u8,
            26 as i32 as u8,
            132 as i32 as u8,
            26 as i32 as u8,
            145 as i32 as u8,
            17 as i32 as u8,
            161 as i32 as u8,
            12 as i32 as u8,
            176 as i32 as u8,
            10 as i32 as u8,
            177 as i32 as u8,
            11 as i32 as u8,
        ],
        [
            24 as i32 as u8,
            179 as i32 as u8,
            48 as i32 as u8,
            138 as i32 as u8,
            54 as i32 as u8,
            135 as i32 as u8,
            54 as i32 as u8,
            132 as i32 as u8,
            53 as i32 as u8,
            134 as i32 as u8,
            56 as i32 as u8,
            133 as i32 as u8,
            55 as i32 as u8,
            132 as i32 as u8,
            55 as i32 as u8,
            132 as i32 as u8,
            61 as i32 as u8,
            114 as i32 as u8,
            70 as i32 as u8,
            96 as i32 as u8,
            74 as i32 as u8,
            88 as i32 as u8,
            75 as i32 as u8,
            88 as i32 as u8,
            87 as i32 as u8,
            74 as i32 as u8,
            89 as i32 as u8,
            66 as i32 as u8,
            91 as i32 as u8,
            67 as i32 as u8,
            100 as i32 as u8,
            59 as i32 as u8,
            108 as i32 as u8,
            50 as i32 as u8,
            120 as i32 as u8,
            40 as i32 as u8,
            122 as i32 as u8,
            37 as i32 as u8,
            97 as i32 as u8,
            43 as i32 as u8,
            78 as i32 as u8,
            50 as i32 as u8,
        ],
    ],
    [
        [
            83 as i32 as u8,
            78 as i32 as u8,
            84 as i32 as u8,
            81 as i32 as u8,
            88 as i32 as u8,
            75 as i32 as u8,
            86 as i32 as u8,
            74 as i32 as u8,
            87 as i32 as u8,
            71 as i32 as u8,
            90 as i32 as u8,
            73 as i32 as u8,
            93 as i32 as u8,
            74 as i32 as u8,
            93 as i32 as u8,
            74 as i32 as u8,
            109 as i32 as u8,
            40 as i32 as u8,
            114 as i32 as u8,
            36 as i32 as u8,
            117 as i32 as u8,
            34 as i32 as u8,
            117 as i32 as u8,
            34 as i32 as u8,
            143 as i32 as u8,
            17 as i32 as u8,
            145 as i32 as u8,
            18 as i32 as u8,
            146 as i32 as u8,
            19 as i32 as u8,
            162 as i32 as u8,
            12 as i32 as u8,
            165 as i32 as u8,
            10 as i32 as u8,
            178 as i32 as u8,
            7 as i32 as u8,
            189 as i32 as u8,
            6 as i32 as u8,
            190 as i32 as u8,
            8 as i32 as u8,
            177 as i32 as u8,
            9 as i32 as u8,
        ],
        [
            23 as i32 as u8,
            178 as i32 as u8,
            54 as i32 as u8,
            115 as i32 as u8,
            63 as i32 as u8,
            102 as i32 as u8,
            66 as i32 as u8,
            98 as i32 as u8,
            69 as i32 as u8,
            99 as i32 as u8,
            74 as i32 as u8,
            89 as i32 as u8,
            71 as i32 as u8,
            91 as i32 as u8,
            73 as i32 as u8,
            91 as i32 as u8,
            78 as i32 as u8,
            89 as i32 as u8,
            86 as i32 as u8,
            80 as i32 as u8,
            92 as i32 as u8,
            66 as i32 as u8,
            93 as i32 as u8,
            64 as i32 as u8,
            102 as i32 as u8,
            59 as i32 as u8,
            103 as i32 as u8,
            60 as i32 as u8,
            104 as i32 as u8,
            60 as i32 as u8,
            117 as i32 as u8,
            52 as i32 as u8,
            123 as i32 as u8,
            44 as i32 as u8,
            138 as i32 as u8,
            35 as i32 as u8,
            133 as i32 as u8,
            31 as i32 as u8,
            97 as i32 as u8,
            38 as i32 as u8,
            77 as i32 as u8,
            45 as i32 as u8,
        ],
    ],
    [
        [
            61 as i32 as u8,
            90 as i32 as u8,
            93 as i32 as u8,
            60 as i32 as u8,
            105 as i32 as u8,
            42 as i32 as u8,
            107 as i32 as u8,
            41 as i32 as u8,
            110 as i32 as u8,
            45 as i32 as u8,
            116 as i32 as u8,
            38 as i32 as u8,
            113 as i32 as u8,
            38 as i32 as u8,
            112 as i32 as u8,
            38 as i32 as u8,
            124 as i32 as u8,
            26 as i32 as u8,
            132 as i32 as u8,
            27 as i32 as u8,
            136 as i32 as u8,
            19 as i32 as u8,
            140 as i32 as u8,
            20 as i32 as u8,
            155 as i32 as u8,
            14 as i32 as u8,
            159 as i32 as u8,
            16 as i32 as u8,
            158 as i32 as u8,
            18 as i32 as u8,
            170 as i32 as u8,
            13 as i32 as u8,
            177 as i32 as u8,
            10 as i32 as u8,
            187 as i32 as u8,
            8 as i32 as u8,
            192 as i32 as u8,
            6 as i32 as u8,
            175 as i32 as u8,
            9 as i32 as u8,
            159 as i32 as u8,
            10 as i32 as u8,
        ],
        [
            21 as i32 as u8,
            178 as i32 as u8,
            59 as i32 as u8,
            110 as i32 as u8,
            71 as i32 as u8,
            86 as i32 as u8,
            75 as i32 as u8,
            85 as i32 as u8,
            84 as i32 as u8,
            83 as i32 as u8,
            91 as i32 as u8,
            66 as i32 as u8,
            88 as i32 as u8,
            73 as i32 as u8,
            87 as i32 as u8,
            72 as i32 as u8,
            92 as i32 as u8,
            75 as i32 as u8,
            98 as i32 as u8,
            72 as i32 as u8,
            105 as i32 as u8,
            58 as i32 as u8,
            107 as i32 as u8,
            54 as i32 as u8,
            115 as i32 as u8,
            52 as i32 as u8,
            114 as i32 as u8,
            55 as i32 as u8,
            112 as i32 as u8,
            56 as i32 as u8,
            129 as i32 as u8,
            51 as i32 as u8,
            132 as i32 as u8,
            40 as i32 as u8,
            150 as i32 as u8,
            33 as i32 as u8,
            140 as i32 as u8,
            29 as i32 as u8,
            98 as i32 as u8,
            35 as i32 as u8,
            77 as i32 as u8,
            42 as i32 as u8,
        ],
    ],
    [
        [
            42 as i32 as u8,
            121 as i32 as u8,
            96 as i32 as u8,
            66 as i32 as u8,
            108 as i32 as u8,
            43 as i32 as u8,
            111 as i32 as u8,
            40 as i32 as u8,
            117 as i32 as u8,
            44 as i32 as u8,
            123 as i32 as u8,
            32 as i32 as u8,
            120 as i32 as u8,
            36 as i32 as u8,
            119 as i32 as u8,
            33 as i32 as u8,
            127 as i32 as u8,
            33 as i32 as u8,
            134 as i32 as u8,
            34 as i32 as u8,
            139 as i32 as u8,
            21 as i32 as u8,
            147 as i32 as u8,
            23 as i32 as u8,
            152 as i32 as u8,
            20 as i32 as u8,
            158 as i32 as u8,
            25 as i32 as u8,
            154 as i32 as u8,
            26 as i32 as u8,
            166 as i32 as u8,
            21 as i32 as u8,
            173 as i32 as u8,
            16 as i32 as u8,
            184 as i32 as u8,
            13 as i32 as u8,
            184 as i32 as u8,
            10 as i32 as u8,
            150 as i32 as u8,
            13 as i32 as u8,
            139 as i32 as u8,
            15 as i32 as u8,
        ],
        [
            22 as i32 as u8,
            178 as i32 as u8,
            63 as i32 as u8,
            114 as i32 as u8,
            74 as i32 as u8,
            82 as i32 as u8,
            84 as i32 as u8,
            83 as i32 as u8,
            92 as i32 as u8,
            82 as i32 as u8,
            103 as i32 as u8,
            62 as i32 as u8,
            96 as i32 as u8,
            72 as i32 as u8,
            96 as i32 as u8,
            67 as i32 as u8,
            101 as i32 as u8,
            73 as i32 as u8,
            107 as i32 as u8,
            72 as i32 as u8,
            113 as i32 as u8,
            55 as i32 as u8,
            118 as i32 as u8,
            52 as i32 as u8,
            125 as i32 as u8,
            52 as i32 as u8,
            118 as i32 as u8,
            52 as i32 as u8,
            117 as i32 as u8,
            55 as i32 as u8,
            135 as i32 as u8,
            49 as i32 as u8,
            137 as i32 as u8,
            39 as i32 as u8,
            157 as i32 as u8,
            32 as i32 as u8,
            145 as i32 as u8,
            29 as i32 as u8,
            97 as i32 as u8,
            33 as i32 as u8,
            77 as i32 as u8,
            40 as i32 as u8,
        ],
    ],
];

static mut small_energy_icdf: [u8; 3] = [2 as i32 as u8, 1 as i32 as u8, 0 as i32 as u8];

unsafe extern "C" fn loss_distortion(
    mut eBands: *const opus_val16,
    mut oldEBands: *mut opus_val16,
    mut start: i32,
    mut end: i32,
    mut len: i32,
    mut C: i32,
) -> opus_val32 {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    let mut dist: opus_val32 = 0 as i32 as opus_val32;
    c = 0 as i32;
    loop {
        i = start;
        while i < end {
            let mut d: opus_val16 =
                *eBands.offset((i + c * len) as isize) - *oldEBands.offset((i + c * len) as isize);
            dist = dist + d * d;
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    return if (200 as i32 as f32) < dist {
        200 as i32 as f32
    } else {
        dist
    };
}

unsafe extern "C" fn quant_coarse_energy_impl(
    mut m: *const OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut eBands: *const opus_val16,
    mut oldEBands: *mut opus_val16,
    mut budget: opus_int32,
    mut tell: opus_int32,
    mut prob_model: *const u8,
    mut error: *mut opus_val16,
    mut enc: *mut ec_enc,
    mut C: i32,
    mut LM: i32,
    mut intra: i32,
    mut max_decay: opus_val16,
    mut lfe: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut badness: i32 = 0 as i32;
    let mut prev: [opus_val32; 2] = [0 as i32 as opus_val32, 0 as i32 as opus_val32];
    let mut coef: opus_val16 = 0.;
    let mut beta: opus_val16 = 0.;
    if tell + 3 as i32 <= budget {
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
            enc as *mut ec_ctx,
            intra,
            3 as i32 as u32,
        );
    }
    if intra != 0 {
        coef = 0 as i32 as opus_val16;
        beta = beta_intra
    } else {
        beta = beta_coef[LM as usize];
        coef = pred_coef[LM as usize]
    }
    /* Encode at a fixed coarse resolution */
    i = start;
    while i < end {
        c = 0 as i32;
        loop {
            let mut bits_left: i32 = 0;
            let mut qi: i32 = 0;
            let mut qi0: i32 = 0;
            let mut q: opus_val32 = 0.;
            let mut x: opus_val16 = 0.;
            let mut f: opus_val32 = 0.;
            let mut tmp: opus_val32 = 0.;
            let mut oldE: opus_val16 = 0.;
            let mut decay_bound: opus_val16 = 0.;
            x = *eBands.offset((i + c * (*m).nbEBands) as isize);
            oldE = if -9.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                -9.0f32
            } else {
                *oldEBands.offset((i + c * (*m).nbEBands) as isize)
            };
            f = x - coef * oldE - prev[c as usize];
            /* Rounding to nearest integer here is really important! */
            qi = crate::stdlib::floor((0.5f32 + f) as f64) as i32;
            decay_bound = (if -28.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                -28.0f32
            } else {
                *oldEBands.offset((i + c * (*m).nbEBands) as isize)
            }) - max_decay;
            /* Prevent the energy from going down too quickly (e.g. for bands
            that have just one bin) */
            if qi < 0 as i32 && x < decay_bound {
                qi += (decay_bound - x) as i32;
                if qi > 0 as i32 {
                    qi = 0 as i32
                }
            }
            qi0 = qi;
            /* If we don't have enough bits to encode all the energy, just assume
            something safe. */
            tell = ec_tell(enc);
            bits_left = budget - tell - 3 as i32 * C * (end - i);
            if i != start && bits_left < 30 as i32 {
                if bits_left < 24 as i32 {
                    qi = if (1 as i32) < qi { 1 as i32 } else { qi }
                }
                if bits_left < 16 as i32 {
                    qi = if -(1 as i32) > qi { -(1 as i32) } else { qi }
                }
            }
            if lfe != 0 && i >= 2 as i32 {
                qi = if qi < 0 as i32 { qi } else { 0 as i32 }
            }
            if budget - tell >= 15 as i32 {
                let mut pi: i32 = 0;
                pi = 2 as i32 * (if i < 20 as i32 { i } else { 20 as i32 });
                crate::src::opus_1_2_1::celt::laplace::ec_laplace_encode(
                    enc as *mut ec_ctx,
                    &mut qi,
                    ((*prob_model.offset(pi as isize) as i32) << 7 as i32) as u32,
                    (*prob_model.offset((pi + 1 as i32) as isize) as i32) << 6 as i32,
                );
            } else if budget - tell >= 2 as i32 {
                qi = if -(1 as i32) > (if qi < 1 as i32 { qi } else { 1 as i32 }) {
                    -(1 as i32)
                } else if qi < 1 as i32 {
                    qi
                } else {
                    1 as i32
                };
                crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
                    enc as *mut ec_ctx,
                    2 as i32 * qi ^ -((qi < 0 as i32) as i32),
                    small_energy_icdf.as_ptr(),
                    2 as i32 as u32,
                );
            } else if budget - tell >= 1 as i32 {
                qi = if (0 as i32) < qi { 0 as i32 } else { qi };
                crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                    enc as *mut ec_ctx,
                    -qi,
                    1 as i32 as u32,
                );
            } else {
                qi = -(1 as i32)
            }
            *error.offset((i + c * (*m).nbEBands) as isize) = f - qi as f32;
            badness += libc::abs(qi0 - qi);
            q = qi as opus_val32;
            tmp = coef * oldE + prev[c as usize] + q;
            *oldEBands.offset((i + c * (*m).nbEBands) as isize) = tmp;
            prev[c as usize] = prev[c as usize] + q - beta * q;
            c += 1;
            if !(c < C) {
                break;
            }
        }
        i += 1
    }
    return if lfe != 0 { 0 as i32 } else { badness };
}
#[no_mangle]

pub unsafe extern "C" fn quant_coarse_energy(
    mut m: *const OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut effEnd: i32,
    mut eBands: *const opus_val16,
    mut oldEBands: *mut opus_val16,
    mut budget: opus_uint32,
    mut error: *mut opus_val16,
    mut enc: *mut ec_enc,
    mut C: i32,
    mut LM: i32,
    mut nbAvailableBytes: i32,
    mut force_intra: i32,
    mut delayedIntra: *mut opus_val32,
    mut two_pass: i32,
    mut loss_rate: i32,
    mut lfe: i32,
) {
    let mut intra: i32 = 0;
    let mut max_decay: opus_val16 = 0.;
    let mut oldEBands_intra: *mut opus_val16 = 0 as *mut opus_val16;
    let mut error_intra: *mut opus_val16 = 0 as *mut opus_val16;
    let mut enc_start_state: ec_enc = ec_enc {
        buf: 0 as *mut u8,
        storage: 0,
        end_offs: 0,
        end_window: 0,
        nend_bits: 0,
        nbits_total: 0,
        offs: 0,
        rng: 0,
        val: 0,
        ext: 0,
        rem: 0,
        error: 0,
    };
    let mut tell: opus_uint32 = 0;
    let mut badness1: i32 = 0 as i32;
    let mut intra_bias: opus_int32 = 0;
    let mut new_distortion: opus_val32 = 0.;
    intra = (force_intra != 0
        || two_pass == 0
            && *delayedIntra > (2 as i32 * C * (end - start)) as f32
            && nbAvailableBytes > (end - start) * C) as i32;
    intra_bias =
        (budget as f32 * *delayedIntra * loss_rate as f32 / (C * 512 as i32) as f32) as opus_int32;
    new_distortion = loss_distortion(eBands, oldEBands, start, effEnd, (*m).nbEBands, C);
    tell = ec_tell(enc) as opus_uint32;
    if tell.wrapping_add(3 as i32 as u32) > budget {
        intra = 0 as i32;
        two_pass = intra
    }
    max_decay = 16.0f32;
    if end - start > 10 as i32 {
        max_decay = if max_decay < 0.125f32 * nbAvailableBytes as f32 {
            max_decay
        } else {
            (0.125f32) * nbAvailableBytes as f32
        }
    }
    if lfe != 0 {
        max_decay = 3.0f32
    }
    enc_start_state = *enc;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_val16>() as usize).wrapping_mul((C * (*m).nbEBands) as usize)
            as usize,
    );
    oldEBands_intra = fresh0.as_mut_ptr() as *mut opus_val16;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_val16>() as usize).wrapping_mul((C * (*m).nbEBands) as usize)
            as usize,
    );
    error_intra = fresh1.as_mut_ptr() as *mut opus_val16;
    crate::stdlib::memcpy(
        oldEBands_intra as *mut libc::c_void,
        oldEBands as *const libc::c_void,
        ((C * (*m).nbEBands) as usize)
            .wrapping_mul(::std::mem::size_of::<opus_val16>() as usize)
            .wrapping_add(
                (0 as i32 as isize * oldEBands_intra.offset_from(oldEBands) as isize) as usize,
            ),
    );
    if two_pass != 0 || intra != 0 {
        badness1 = quant_coarse_energy_impl(
            m,
            start,
            end,
            eBands,
            oldEBands_intra,
            budget as opus_int32,
            tell as opus_int32,
            e_prob_model[LM as usize][1 as i32 as usize].as_ptr(),
            error_intra,
            enc,
            C,
            LM,
            1 as i32,
            max_decay,
            lfe,
        )
    }
    if intra == 0 {
        let mut intra_buf: *mut u8 = 0 as *mut u8;
        let mut enc_intra_state: ec_enc = ec_enc {
            buf: 0 as *mut u8,
            storage: 0,
            end_offs: 0,
            end_window: 0,
            nend_bits: 0,
            nbits_total: 0,
            offs: 0,
            rng: 0,
            val: 0,
            ext: 0,
            rem: 0,
            error: 0,
        };
        let mut tell_intra: opus_int32 = 0;
        let mut nstart_bytes: opus_uint32 = 0;
        let mut nintra_bytes: opus_uint32 = 0;
        let mut save_bytes: opus_uint32 = 0;
        let mut badness2: i32 = 0;
        let mut intra_bits: *mut u8 = 0 as *mut u8;
        tell_intra = ec_tell_frac(enc as *mut ec_ctx) as opus_int32;
        enc_intra_state = *enc;
        nstart_bytes = ec_range_bytes(&mut enc_start_state);
        nintra_bytes = ec_range_bytes(&mut enc_intra_state);
        intra_buf = ec_get_buffer(&mut enc_intra_state).offset(nstart_bytes as isize);
        save_bytes = nintra_bytes.wrapping_sub(nstart_bytes);
        if save_bytes == 0 as i32 as u32 {
            save_bytes = 0 as i32 as opus_uint32
        }
        let mut fresh2 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<u8>() as usize).wrapping_mul(save_bytes as usize) as usize,
        );
        intra_bits = fresh2.as_mut_ptr() as *mut u8;
        /* Copy bits from intra bit-stream */
        crate::stdlib::memcpy(
            intra_bits as *mut libc::c_void,
            intra_buf as *const libc::c_void,
            (nintra_bytes.wrapping_sub(nstart_bytes) as usize)
                .wrapping_mul(::std::mem::size_of::<u8>() as usize)
                .wrapping_add(
                    (0 as i32 as isize * intra_bits.offset_from(intra_buf) as isize) as usize,
                ),
        );
        *enc = enc_start_state;
        badness2 = quant_coarse_energy_impl(
            m,
            start,
            end,
            eBands,
            oldEBands,
            budget as opus_int32,
            tell as opus_int32,
            e_prob_model[LM as usize][intra as usize].as_ptr(),
            error,
            enc,
            C,
            LM,
            0 as i32,
            max_decay,
            lfe,
        );
        if two_pass != 0
            && (badness1 < badness2
                || badness1 == badness2
                    && ec_tell_frac(enc as *mut ec_ctx) as opus_int32 + intra_bias > tell_intra)
        {
            *enc = enc_intra_state;
            /* Copy intra bits to bit-stream */
            crate::stdlib::memcpy(
                intra_buf as *mut libc::c_void,
                intra_bits as *const libc::c_void,
                (nintra_bytes.wrapping_sub(nstart_bytes) as usize)
                    .wrapping_mul(::std::mem::size_of::<u8>() as usize)
                    .wrapping_add(
                        (0 as i32 as isize * intra_buf.offset_from(intra_bits) as isize) as usize,
                    ),
            );
            crate::stdlib::memcpy(
                oldEBands as *mut libc::c_void,
                oldEBands_intra as *const libc::c_void,
                ((C * (*m).nbEBands) as usize)
                    .wrapping_mul(::std::mem::size_of::<opus_val16>() as usize)
                    .wrapping_add(
                        (0 as i32 as isize * oldEBands.offset_from(oldEBands_intra) as isize)
                            as usize,
                    ),
            );
            crate::stdlib::memcpy(
                error as *mut libc::c_void,
                error_intra as *const libc::c_void,
                ((C * (*m).nbEBands) as usize)
                    .wrapping_mul(::std::mem::size_of::<opus_val16>() as usize)
                    .wrapping_add(
                        (0 as i32 as isize * error.offset_from(error_intra) as isize) as usize,
                    ),
            );
            intra = 1 as i32
        }
    } else {
        crate::stdlib::memcpy(
            oldEBands as *mut libc::c_void,
            oldEBands_intra as *const libc::c_void,
            ((C * (*m).nbEBands) as usize)
                .wrapping_mul(::std::mem::size_of::<opus_val16>() as usize)
                .wrapping_add(
                    (0 as i32 as isize * oldEBands.offset_from(oldEBands_intra) as isize) as usize,
                ),
        );
        crate::stdlib::memcpy(
            error as *mut libc::c_void,
            error_intra as *const libc::c_void,
            ((C * (*m).nbEBands) as usize)
                .wrapping_mul(::std::mem::size_of::<opus_val16>() as usize)
                .wrapping_add(
                    (0 as i32 as isize * error.offset_from(error_intra) as isize) as usize,
                ),
        );
    }
    if intra != 0 {
        *delayedIntra = new_distortion
    } else {
        *delayedIntra =
            pred_coef[LM as usize] * pred_coef[LM as usize] * *delayedIntra + new_distortion
    };
}
#[no_mangle]

pub unsafe extern "C" fn quant_fine_energy(
    mut m: *const OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut oldEBands: *mut opus_val16,
    mut error: *mut opus_val16,
    mut fine_quant: *mut i32,
    mut enc: *mut ec_enc,
    mut C: i32,
) {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    /* Encode finer resolution */
    i = start;
    while i < end {
        let mut frac: opus_int16 = ((1 as i32) << *fine_quant.offset(i as isize)) as opus_int16;
        if !(*fine_quant.offset(i as isize) <= 0 as i32) {
            c = 0 as i32;
            loop {
                let mut q2: i32 = 0;
                let mut offset: opus_val16 = 0.;
                q2 = crate::stdlib::floor(
                    ((*error.offset((i + c * (*m).nbEBands) as isize) + 0.5f32)
                        * frac as i32 as f32) as f64,
                ) as i32;
                if q2 > frac as i32 - 1 as i32 {
                    q2 = frac as i32 - 1 as i32
                }
                if q2 < 0 as i32 {
                    q2 = 0 as i32
                }
                crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
                    enc as *mut ec_ctx,
                    q2 as opus_uint32,
                    *fine_quant.offset(i as isize) as u32,
                );
                offset = (q2 as f32 + 0.5f32)
                    * ((1 as i32) << 14 as i32 - *fine_quant.offset(i as isize)) as f32
                    * (1.0f32 / 16384 as i32 as f32)
                    - 0.5f32;
                let ref mut fresh3 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                *fresh3 += offset;
                let ref mut fresh4 = *error.offset((i + c * (*m).nbEBands) as isize);
                *fresh4 -= offset;
                c += 1;
                if !(c < C) {
                    break;
                }
                /*printf ("%f ", error[i] - offset);*/
            }
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn quant_energy_finalise(
    mut m: *const OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut oldEBands: *mut opus_val16,
    mut error: *mut opus_val16,
    mut fine_quant: *mut i32,
    mut fine_priority: *mut i32,
    mut bits_left: i32,
    mut enc: *mut ec_enc,
    mut C: i32,
) {
    let mut i: i32 = 0;
    let mut prio: i32 = 0;
    let mut c: i32 = 0;
    /* Use up the remaining bits */
    prio = 0 as i32;
    while prio < 2 as i32 {
        i = start;
        while i < end && bits_left >= C {
            if !(*fine_quant.offset(i as isize) >= 8 as i32
                || *fine_priority.offset(i as isize) != prio)
            {
                c = 0 as i32;
                loop {
                    let mut q2: i32 = 0;
                    let mut offset: opus_val16 = 0.;
                    q2 = if *error.offset((i + c * (*m).nbEBands) as isize) < 0 as i32 as f32 {
                        0 as i32
                    } else {
                        1 as i32
                    };
                    crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
                        enc as *mut ec_ctx,
                        q2 as opus_uint32,
                        1 as i32 as u32,
                    );
                    offset = (q2 as f32 - 0.5f32)
                        * ((1 as i32) << 14 as i32 - *fine_quant.offset(i as isize) - 1 as i32)
                            as f32
                        * (1.0f32 / 16384 as i32 as f32);
                    let ref mut fresh5 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                    *fresh5 += offset;
                    let ref mut fresh6 = *error.offset((i + c * (*m).nbEBands) as isize);
                    *fresh6 -= offset;
                    bits_left -= 1;
                    c += 1;
                    if !(c < C) {
                        break;
                    }
                }
            }
            i += 1
        }
        prio += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn unquant_coarse_energy(
    mut m: *const OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut oldEBands: *mut opus_val16,
    mut intra: i32,
    mut dec: *mut ec_dec,
    mut C: i32,
    mut LM: i32,
) {
    let mut prob_model: *const u8 = e_prob_model[LM as usize][intra as usize].as_ptr();
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut prev: [opus_val32; 2] = [0 as i32 as opus_val32, 0 as i32 as opus_val32];
    let mut coef: opus_val16 = 0.;
    let mut beta: opus_val16 = 0.;
    let mut budget: opus_int32 = 0;
    let mut tell: opus_int32 = 0;
    if intra != 0 {
        coef = 0 as i32 as opus_val16;
        beta = beta_intra
    } else {
        beta = beta_coef[LM as usize];
        coef = pred_coef[LM as usize]
    }
    budget = (*dec).storage.wrapping_mul(8 as i32 as u32) as opus_int32;
    /* Decode at a fixed coarse resolution */
    i = start;
    while i < end {
        c = 0 as i32;
        loop {
            let mut qi: i32 = 0;
            let mut q: opus_val32 = 0.;
            let mut tmp: opus_val32 = 0.;
            /* It would be better to express this invariant as a
            test on C at function entry, but that isn't enough
            to make the static analyzer happy. */
            tell = ec_tell(dec);
            if budget - tell >= 15 as i32 {
                let mut pi: i32 = 0;
                pi = 2 as i32 * (if i < 20 as i32 { i } else { 20 as i32 });
                qi = crate::src::opus_1_2_1::celt::laplace::ec_laplace_decode(
                    dec as *mut ec_ctx,
                    ((*prob_model.offset(pi as isize) as i32) << 7 as i32) as u32,
                    (*prob_model.offset((pi + 1 as i32) as isize) as i32) << 6 as i32,
                )
            } else if budget - tell >= 2 as i32 {
                qi = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                    dec as *mut ec_ctx,
                    small_energy_icdf.as_ptr(),
                    2 as i32 as u32,
                );
                qi = qi >> 1 as i32 ^ -(qi & 1 as i32)
            } else if budget - tell >= 1 as i32 {
                qi = -crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(
                    dec as *mut ec_ctx,
                    1 as i32 as u32,
                )
            } else {
                qi = -(1 as i32)
            }
            q = qi as opus_val32;
            *oldEBands.offset((i + c * (*m).nbEBands) as isize) =
                if -9.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                    -9.0f32
                } else {
                    *oldEBands.offset((i + c * (*m).nbEBands) as isize)
                };
            tmp = coef * *oldEBands.offset((i + c * (*m).nbEBands) as isize) + prev[c as usize] + q;
            *oldEBands.offset((i + c * (*m).nbEBands) as isize) = tmp;
            prev[c as usize] = prev[c as usize] + q - beta * q;
            c += 1;
            if !(c < C) {
                break;
            }
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn unquant_fine_energy(
    mut m: *const OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut oldEBands: *mut opus_val16,
    mut fine_quant: *mut i32,
    mut dec: *mut ec_dec,
    mut C: i32,
) {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    /* Decode finer resolution */
    i = start;
    while i < end {
        if !(*fine_quant.offset(i as isize) <= 0 as i32) {
            c = 0 as i32;
            loop {
                let mut q2: i32 = 0;
                let mut offset: opus_val16 = 0.;
                q2 = crate::src::opus_1_2_1::celt::entdec::ec_dec_bits(
                    dec as *mut ec_ctx,
                    *fine_quant.offset(i as isize) as u32,
                ) as i32;
                offset = (q2 as f32 + 0.5f32)
                    * ((1 as i32) << 14 as i32 - *fine_quant.offset(i as isize)) as f32
                    * (1.0f32 / 16384 as i32 as f32)
                    - 0.5f32;
                let ref mut fresh7 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                *fresh7 += offset;
                c += 1;
                if !(c < C) {
                    break;
                }
            }
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn unquant_energy_finalise(
    mut m: *const OpusCustomMode,
    mut start: i32,
    mut end: i32,
    mut oldEBands: *mut opus_val16,
    mut fine_quant: *mut i32,
    mut fine_priority: *mut i32,
    mut bits_left: i32,
    mut dec: *mut ec_dec,
    mut C: i32,
) {
    let mut i: i32 = 0;
    let mut prio: i32 = 0;
    let mut c: i32 = 0;
    /* Use up the remaining bits */
    prio = 0 as i32;
    while prio < 2 as i32 {
        i = start;
        while i < end && bits_left >= C {
            if !(*fine_quant.offset(i as isize) >= 8 as i32
                || *fine_priority.offset(i as isize) != prio)
            {
                c = 0 as i32;
                loop {
                    let mut q2: i32 = 0;
                    let mut offset: opus_val16 = 0.;
                    q2 = crate::src::opus_1_2_1::celt::entdec::ec_dec_bits(
                        dec as *mut ec_ctx,
                        1 as i32 as u32,
                    ) as i32;
                    offset = (q2 as f32 - 0.5f32)
                        * ((1 as i32) << 14 as i32 - *fine_quant.offset(i as isize) - 1 as i32)
                            as f32
                        * (1.0f32 / 16384 as i32 as f32);
                    let ref mut fresh8 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                    *fresh8 += offset;
                    bits_left -= 1;
                    c += 1;
                    if !(c < C) {
                        break;
                    }
                }
            }
            i += 1
        }
        prio += 1
    }
}
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Written by Jean-Marc Valin */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
#[no_mangle]

pub unsafe extern "C" fn amp2Log2(
    mut m: *const OpusCustomMode,
    mut effEnd: i32,
    mut end: i32,
    mut bandE: *mut celt_ener,
    mut bandLogE: *mut opus_val16,
    mut C: i32,
) {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    c = 0 as i32;
    loop {
        i = 0 as i32;
        while i < effEnd {
            *bandLogE.offset((i + c * (*m).nbEBands) as isize) =
                celt_log2(*bandE.offset((i + c * (*m).nbEBands) as isize)) - eMeans[i as usize];
            i += 1
        }
        i = effEnd;
        while i < end {
            *bandLogE.offset((c * (*m).nbEBands + i) as isize) = -14.0f32;
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
}

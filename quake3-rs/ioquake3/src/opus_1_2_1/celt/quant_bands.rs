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
    ) -> *mut libc::c_uchar {
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
    ) -> libc::c_int {
        return (*_this).nbits_total
            - (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
                - (*_this).rng.leading_zeros() as i32);
    }
}

pub mod mathops_h {

    #[inline]

    pub unsafe extern "C" fn celt_log2(mut x: libc::c_float) -> libc::c_float {
        let mut integer: libc::c_int = 0;
        let mut frac: libc::c_float = 0.;
        let mut in_0: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        in_0.f = x;
        integer = (in_0.i >> 23 as libc::c_int).wrapping_sub(127 as libc::c_int as libc::c_uint)
            as libc::c_int;
        in_0.i =
            (in_0.i as libc::c_uint).wrapping_sub((integer << 23 as libc::c_int) as libc::c_uint)
                as crate::opus_types_h::opus_uint32 as crate::opus_types_h::opus_uint32;
        frac = in_0.f - 1.5f32;
        frac = -0.41445418f32
            + frac * (0.95909232f32 + frac * (-0.33951290f32 + frac * 0.16541097f32));
        return (1 as libc::c_int + integer) as libc::c_float + frac;
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

pub static mut eMeans: [crate::arch_h::opus_val16; 25] = [
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

static mut pred_coef: [crate::arch_h::opus_val16; 4] = [
    (29440 as libc::c_int as libc::c_double / 32768.0f64) as crate::arch_h::opus_val16,
    (26112 as libc::c_int as libc::c_double / 32768.0f64) as crate::arch_h::opus_val16,
    (21248 as libc::c_int as libc::c_double / 32768.0f64) as crate::arch_h::opus_val16,
    (16384 as libc::c_int as libc::c_double / 32768.0f64) as crate::arch_h::opus_val16,
];

static mut beta_coef: [crate::arch_h::opus_val16; 4] = [
    (30147 as libc::c_int as libc::c_double / 32768.0f64) as crate::arch_h::opus_val16,
    (22282 as libc::c_int as libc::c_double / 32768.0f64) as crate::arch_h::opus_val16,
    (12124 as libc::c_int as libc::c_double / 32768.0f64) as crate::arch_h::opus_val16,
    (6554 as libc::c_int as libc::c_double / 32768.0f64) as crate::arch_h::opus_val16,
];

static mut beta_intra: crate::arch_h::opus_val16 =
    (4915 as libc::c_int as libc::c_double / 32768.0f64) as crate::arch_h::opus_val16;
/*Parameters of the Laplace-like probability models used for the coarse energy.
There is one pair of parameters for each frame size, prediction type
 (inter/intra), and band number.
The first number of each pair is the probability of 0, and the second is the
 decay rate, both in Q8 precision.*/

static mut e_prob_model: [[[libc::c_uchar; 42]; 2]; 4] = [
    [
        [
            72 as libc::c_int as libc::c_uchar,
            127 as libc::c_int as libc::c_uchar,
            65 as libc::c_int as libc::c_uchar,
            129 as libc::c_int as libc::c_uchar,
            66 as libc::c_int as libc::c_uchar,
            128 as libc::c_int as libc::c_uchar,
            65 as libc::c_int as libc::c_uchar,
            128 as libc::c_int as libc::c_uchar,
            64 as libc::c_int as libc::c_uchar,
            128 as libc::c_int as libc::c_uchar,
            62 as libc::c_int as libc::c_uchar,
            128 as libc::c_int as libc::c_uchar,
            64 as libc::c_int as libc::c_uchar,
            128 as libc::c_int as libc::c_uchar,
            64 as libc::c_int as libc::c_uchar,
            128 as libc::c_int as libc::c_uchar,
            92 as libc::c_int as libc::c_uchar,
            78 as libc::c_int as libc::c_uchar,
            92 as libc::c_int as libc::c_uchar,
            79 as libc::c_int as libc::c_uchar,
            92 as libc::c_int as libc::c_uchar,
            78 as libc::c_int as libc::c_uchar,
            90 as libc::c_int as libc::c_uchar,
            79 as libc::c_int as libc::c_uchar,
            116 as libc::c_int as libc::c_uchar,
            41 as libc::c_int as libc::c_uchar,
            115 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
            114 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            26 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            26 as libc::c_int as libc::c_uchar,
            145 as libc::c_int as libc::c_uchar,
            17 as libc::c_int as libc::c_uchar,
            161 as libc::c_int as libc::c_uchar,
            12 as libc::c_int as libc::c_uchar,
            176 as libc::c_int as libc::c_uchar,
            10 as libc::c_int as libc::c_uchar,
            177 as libc::c_int as libc::c_uchar,
            11 as libc::c_int as libc::c_uchar,
        ],
        [
            24 as libc::c_int as libc::c_uchar,
            179 as libc::c_int as libc::c_uchar,
            48 as libc::c_int as libc::c_uchar,
            138 as libc::c_int as libc::c_uchar,
            54 as libc::c_int as libc::c_uchar,
            135 as libc::c_int as libc::c_uchar,
            54 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            53 as libc::c_int as libc::c_uchar,
            134 as libc::c_int as libc::c_uchar,
            56 as libc::c_int as libc::c_uchar,
            133 as libc::c_int as libc::c_uchar,
            55 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            55 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            61 as libc::c_int as libc::c_uchar,
            114 as libc::c_int as libc::c_uchar,
            70 as libc::c_int as libc::c_uchar,
            96 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            88 as libc::c_int as libc::c_uchar,
            75 as libc::c_int as libc::c_uchar,
            88 as libc::c_int as libc::c_uchar,
            87 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            89 as libc::c_int as libc::c_uchar,
            66 as libc::c_int as libc::c_uchar,
            91 as libc::c_int as libc::c_uchar,
            67 as libc::c_int as libc::c_uchar,
            100 as libc::c_int as libc::c_uchar,
            59 as libc::c_int as libc::c_uchar,
            108 as libc::c_int as libc::c_uchar,
            50 as libc::c_int as libc::c_uchar,
            120 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
            122 as libc::c_int as libc::c_uchar,
            37 as libc::c_int as libc::c_uchar,
            97 as libc::c_int as libc::c_uchar,
            43 as libc::c_int as libc::c_uchar,
            78 as libc::c_int as libc::c_uchar,
            50 as libc::c_int as libc::c_uchar,
        ],
    ],
    [
        [
            83 as libc::c_int as libc::c_uchar,
            78 as libc::c_int as libc::c_uchar,
            84 as libc::c_int as libc::c_uchar,
            81 as libc::c_int as libc::c_uchar,
            88 as libc::c_int as libc::c_uchar,
            75 as libc::c_int as libc::c_uchar,
            86 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            87 as libc::c_int as libc::c_uchar,
            71 as libc::c_int as libc::c_uchar,
            90 as libc::c_int as libc::c_uchar,
            73 as libc::c_int as libc::c_uchar,
            93 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            93 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            109 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
            114 as libc::c_int as libc::c_uchar,
            36 as libc::c_int as libc::c_uchar,
            117 as libc::c_int as libc::c_uchar,
            34 as libc::c_int as libc::c_uchar,
            117 as libc::c_int as libc::c_uchar,
            34 as libc::c_int as libc::c_uchar,
            143 as libc::c_int as libc::c_uchar,
            17 as libc::c_int as libc::c_uchar,
            145 as libc::c_int as libc::c_uchar,
            18 as libc::c_int as libc::c_uchar,
            146 as libc::c_int as libc::c_uchar,
            19 as libc::c_int as libc::c_uchar,
            162 as libc::c_int as libc::c_uchar,
            12 as libc::c_int as libc::c_uchar,
            165 as libc::c_int as libc::c_uchar,
            10 as libc::c_int as libc::c_uchar,
            178 as libc::c_int as libc::c_uchar,
            7 as libc::c_int as libc::c_uchar,
            189 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
            190 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
            177 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
        ],
        [
            23 as libc::c_int as libc::c_uchar,
            178 as libc::c_int as libc::c_uchar,
            54 as libc::c_int as libc::c_uchar,
            115 as libc::c_int as libc::c_uchar,
            63 as libc::c_int as libc::c_uchar,
            102 as libc::c_int as libc::c_uchar,
            66 as libc::c_int as libc::c_uchar,
            98 as libc::c_int as libc::c_uchar,
            69 as libc::c_int as libc::c_uchar,
            99 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            89 as libc::c_int as libc::c_uchar,
            71 as libc::c_int as libc::c_uchar,
            91 as libc::c_int as libc::c_uchar,
            73 as libc::c_int as libc::c_uchar,
            91 as libc::c_int as libc::c_uchar,
            78 as libc::c_int as libc::c_uchar,
            89 as libc::c_int as libc::c_uchar,
            86 as libc::c_int as libc::c_uchar,
            80 as libc::c_int as libc::c_uchar,
            92 as libc::c_int as libc::c_uchar,
            66 as libc::c_int as libc::c_uchar,
            93 as libc::c_int as libc::c_uchar,
            64 as libc::c_int as libc::c_uchar,
            102 as libc::c_int as libc::c_uchar,
            59 as libc::c_int as libc::c_uchar,
            103 as libc::c_int as libc::c_uchar,
            60 as libc::c_int as libc::c_uchar,
            104 as libc::c_int as libc::c_uchar,
            60 as libc::c_int as libc::c_uchar,
            117 as libc::c_int as libc::c_uchar,
            52 as libc::c_int as libc::c_uchar,
            123 as libc::c_int as libc::c_uchar,
            44 as libc::c_int as libc::c_uchar,
            138 as libc::c_int as libc::c_uchar,
            35 as libc::c_int as libc::c_uchar,
            133 as libc::c_int as libc::c_uchar,
            31 as libc::c_int as libc::c_uchar,
            97 as libc::c_int as libc::c_uchar,
            38 as libc::c_int as libc::c_uchar,
            77 as libc::c_int as libc::c_uchar,
            45 as libc::c_int as libc::c_uchar,
        ],
    ],
    [
        [
            61 as libc::c_int as libc::c_uchar,
            90 as libc::c_int as libc::c_uchar,
            93 as libc::c_int as libc::c_uchar,
            60 as libc::c_int as libc::c_uchar,
            105 as libc::c_int as libc::c_uchar,
            42 as libc::c_int as libc::c_uchar,
            107 as libc::c_int as libc::c_uchar,
            41 as libc::c_int as libc::c_uchar,
            110 as libc::c_int as libc::c_uchar,
            45 as libc::c_int as libc::c_uchar,
            116 as libc::c_int as libc::c_uchar,
            38 as libc::c_int as libc::c_uchar,
            113 as libc::c_int as libc::c_uchar,
            38 as libc::c_int as libc::c_uchar,
            112 as libc::c_int as libc::c_uchar,
            38 as libc::c_int as libc::c_uchar,
            124 as libc::c_int as libc::c_uchar,
            26 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            27 as libc::c_int as libc::c_uchar,
            136 as libc::c_int as libc::c_uchar,
            19 as libc::c_int as libc::c_uchar,
            140 as libc::c_int as libc::c_uchar,
            20 as libc::c_int as libc::c_uchar,
            155 as libc::c_int as libc::c_uchar,
            14 as libc::c_int as libc::c_uchar,
            159 as libc::c_int as libc::c_uchar,
            16 as libc::c_int as libc::c_uchar,
            158 as libc::c_int as libc::c_uchar,
            18 as libc::c_int as libc::c_uchar,
            170 as libc::c_int as libc::c_uchar,
            13 as libc::c_int as libc::c_uchar,
            177 as libc::c_int as libc::c_uchar,
            10 as libc::c_int as libc::c_uchar,
            187 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
            192 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
            175 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
            159 as libc::c_int as libc::c_uchar,
            10 as libc::c_int as libc::c_uchar,
        ],
        [
            21 as libc::c_int as libc::c_uchar,
            178 as libc::c_int as libc::c_uchar,
            59 as libc::c_int as libc::c_uchar,
            110 as libc::c_int as libc::c_uchar,
            71 as libc::c_int as libc::c_uchar,
            86 as libc::c_int as libc::c_uchar,
            75 as libc::c_int as libc::c_uchar,
            85 as libc::c_int as libc::c_uchar,
            84 as libc::c_int as libc::c_uchar,
            83 as libc::c_int as libc::c_uchar,
            91 as libc::c_int as libc::c_uchar,
            66 as libc::c_int as libc::c_uchar,
            88 as libc::c_int as libc::c_uchar,
            73 as libc::c_int as libc::c_uchar,
            87 as libc::c_int as libc::c_uchar,
            72 as libc::c_int as libc::c_uchar,
            92 as libc::c_int as libc::c_uchar,
            75 as libc::c_int as libc::c_uchar,
            98 as libc::c_int as libc::c_uchar,
            72 as libc::c_int as libc::c_uchar,
            105 as libc::c_int as libc::c_uchar,
            58 as libc::c_int as libc::c_uchar,
            107 as libc::c_int as libc::c_uchar,
            54 as libc::c_int as libc::c_uchar,
            115 as libc::c_int as libc::c_uchar,
            52 as libc::c_int as libc::c_uchar,
            114 as libc::c_int as libc::c_uchar,
            55 as libc::c_int as libc::c_uchar,
            112 as libc::c_int as libc::c_uchar,
            56 as libc::c_int as libc::c_uchar,
            129 as libc::c_int as libc::c_uchar,
            51 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
            150 as libc::c_int as libc::c_uchar,
            33 as libc::c_int as libc::c_uchar,
            140 as libc::c_int as libc::c_uchar,
            29 as libc::c_int as libc::c_uchar,
            98 as libc::c_int as libc::c_uchar,
            35 as libc::c_int as libc::c_uchar,
            77 as libc::c_int as libc::c_uchar,
            42 as libc::c_int as libc::c_uchar,
        ],
    ],
    [
        [
            42 as libc::c_int as libc::c_uchar,
            121 as libc::c_int as libc::c_uchar,
            96 as libc::c_int as libc::c_uchar,
            66 as libc::c_int as libc::c_uchar,
            108 as libc::c_int as libc::c_uchar,
            43 as libc::c_int as libc::c_uchar,
            111 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
            117 as libc::c_int as libc::c_uchar,
            44 as libc::c_int as libc::c_uchar,
            123 as libc::c_int as libc::c_uchar,
            32 as libc::c_int as libc::c_uchar,
            120 as libc::c_int as libc::c_uchar,
            36 as libc::c_int as libc::c_uchar,
            119 as libc::c_int as libc::c_uchar,
            33 as libc::c_int as libc::c_uchar,
            127 as libc::c_int as libc::c_uchar,
            33 as libc::c_int as libc::c_uchar,
            134 as libc::c_int as libc::c_uchar,
            34 as libc::c_int as libc::c_uchar,
            139 as libc::c_int as libc::c_uchar,
            21 as libc::c_int as libc::c_uchar,
            147 as libc::c_int as libc::c_uchar,
            23 as libc::c_int as libc::c_uchar,
            152 as libc::c_int as libc::c_uchar,
            20 as libc::c_int as libc::c_uchar,
            158 as libc::c_int as libc::c_uchar,
            25 as libc::c_int as libc::c_uchar,
            154 as libc::c_int as libc::c_uchar,
            26 as libc::c_int as libc::c_uchar,
            166 as libc::c_int as libc::c_uchar,
            21 as libc::c_int as libc::c_uchar,
            173 as libc::c_int as libc::c_uchar,
            16 as libc::c_int as libc::c_uchar,
            184 as libc::c_int as libc::c_uchar,
            13 as libc::c_int as libc::c_uchar,
            184 as libc::c_int as libc::c_uchar,
            10 as libc::c_int as libc::c_uchar,
            150 as libc::c_int as libc::c_uchar,
            13 as libc::c_int as libc::c_uchar,
            139 as libc::c_int as libc::c_uchar,
            15 as libc::c_int as libc::c_uchar,
        ],
        [
            22 as libc::c_int as libc::c_uchar,
            178 as libc::c_int as libc::c_uchar,
            63 as libc::c_int as libc::c_uchar,
            114 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            82 as libc::c_int as libc::c_uchar,
            84 as libc::c_int as libc::c_uchar,
            83 as libc::c_int as libc::c_uchar,
            92 as libc::c_int as libc::c_uchar,
            82 as libc::c_int as libc::c_uchar,
            103 as libc::c_int as libc::c_uchar,
            62 as libc::c_int as libc::c_uchar,
            96 as libc::c_int as libc::c_uchar,
            72 as libc::c_int as libc::c_uchar,
            96 as libc::c_int as libc::c_uchar,
            67 as libc::c_int as libc::c_uchar,
            101 as libc::c_int as libc::c_uchar,
            73 as libc::c_int as libc::c_uchar,
            107 as libc::c_int as libc::c_uchar,
            72 as libc::c_int as libc::c_uchar,
            113 as libc::c_int as libc::c_uchar,
            55 as libc::c_int as libc::c_uchar,
            118 as libc::c_int as libc::c_uchar,
            52 as libc::c_int as libc::c_uchar,
            125 as libc::c_int as libc::c_uchar,
            52 as libc::c_int as libc::c_uchar,
            118 as libc::c_int as libc::c_uchar,
            52 as libc::c_int as libc::c_uchar,
            117 as libc::c_int as libc::c_uchar,
            55 as libc::c_int as libc::c_uchar,
            135 as libc::c_int as libc::c_uchar,
            49 as libc::c_int as libc::c_uchar,
            137 as libc::c_int as libc::c_uchar,
            39 as libc::c_int as libc::c_uchar,
            157 as libc::c_int as libc::c_uchar,
            32 as libc::c_int as libc::c_uchar,
            145 as libc::c_int as libc::c_uchar,
            29 as libc::c_int as libc::c_uchar,
            97 as libc::c_int as libc::c_uchar,
            33 as libc::c_int as libc::c_uchar,
            77 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
        ],
    ],
];

static mut small_energy_icdf: [libc::c_uchar; 3] = [
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];

unsafe extern "C" fn loss_distortion(
    mut eBands: *const crate::arch_h::opus_val16,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut len: libc::c_int,
    mut C: libc::c_int,
) -> crate::arch_h::opus_val32 {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut dist: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
    c = 0 as libc::c_int;
    loop {
        i = start;
        while i < end {
            let mut d: crate::arch_h::opus_val16 =
                *eBands.offset((i + c * len) as isize) - *oldEBands.offset((i + c * len) as isize);
            dist = dist + d * d;
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    return if (200 as libc::c_int as libc::c_float) < dist {
        200 as libc::c_int as libc::c_float
    } else {
        dist
    };
}

unsafe extern "C" fn quant_coarse_energy_impl(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut eBands: *const crate::arch_h::opus_val16,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut budget: crate::opus_types_h::opus_int32,
    mut tell: crate::opus_types_h::opus_int32,
    mut prob_model: *const libc::c_uchar,
    mut error: *mut crate::arch_h::opus_val16,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut C: libc::c_int,
    mut LM: libc::c_int,
    mut intra: libc::c_int,
    mut max_decay: crate::arch_h::opus_val16,
    mut lfe: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut badness: libc::c_int = 0 as libc::c_int;
    let mut prev: [crate::arch_h::opus_val32; 2] = [
        0 as libc::c_int as crate::arch_h::opus_val32,
        0 as libc::c_int as crate::arch_h::opus_val32,
    ];
    let mut coef: crate::arch_h::opus_val16 = 0.;
    let mut beta: crate::arch_h::opus_val16 = 0.;
    if tell + 3 as libc::c_int <= budget {
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            intra,
            3 as libc::c_int as libc::c_uint,
        );
    }
    if intra != 0 {
        coef = 0 as libc::c_int as crate::arch_h::opus_val16;
        beta = beta_intra
    } else {
        beta = beta_coef[LM as usize];
        coef = pred_coef[LM as usize]
    }
    /* Encode at a fixed coarse resolution */
    i = start;
    while i < end {
        c = 0 as libc::c_int;
        loop {
            let mut bits_left: libc::c_int = 0;
            let mut qi: libc::c_int = 0;
            let mut qi0: libc::c_int = 0;
            let mut q: crate::arch_h::opus_val32 = 0.;
            let mut x: crate::arch_h::opus_val16 = 0.;
            let mut f: crate::arch_h::opus_val32 = 0.;
            let mut tmp: crate::arch_h::opus_val32 = 0.;
            let mut oldE: crate::arch_h::opus_val16 = 0.;
            let mut decay_bound: crate::arch_h::opus_val16 = 0.;
            x = *eBands.offset((i + c * (*m).nbEBands) as isize);
            oldE = if -9.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                -9.0f32
            } else {
                *oldEBands.offset((i + c * (*m).nbEBands) as isize)
            };
            f = x - coef * oldE - prev[c as usize];
            /* Rounding to nearest integer here is really important! */
            qi = crate::stdlib::floor((0.5f32 + f) as libc::c_double) as libc::c_int;
            decay_bound = (if -28.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                -28.0f32
            } else {
                *oldEBands.offset((i + c * (*m).nbEBands) as isize)
            }) - max_decay;
            /* Prevent the energy from going down too quickly (e.g. for bands
            that have just one bin) */
            if qi < 0 as libc::c_int && x < decay_bound {
                qi += (decay_bound - x) as libc::c_int;
                if qi > 0 as libc::c_int {
                    qi = 0 as libc::c_int
                }
            }
            qi0 = qi;
            /* If we don't have enough bits to encode all the energy, just assume
            something safe. */
            tell = ec_tell(enc);
            bits_left = budget - tell - 3 as libc::c_int * C * (end - i);
            if i != start && bits_left < 30 as libc::c_int {
                if bits_left < 24 as libc::c_int {
                    qi = if (1 as libc::c_int) < qi {
                        1 as libc::c_int
                    } else {
                        qi
                    }
                }
                if bits_left < 16 as libc::c_int {
                    qi = if -(1 as libc::c_int) > qi {
                        -(1 as libc::c_int)
                    } else {
                        qi
                    }
                }
            }
            if lfe != 0 && i >= 2 as libc::c_int {
                qi = if qi < 0 as libc::c_int {
                    qi
                } else {
                    0 as libc::c_int
                }
            }
            if budget - tell >= 15 as libc::c_int {
                let mut pi: libc::c_int = 0;
                pi = 2 as libc::c_int
                    * (if i < 20 as libc::c_int {
                        i
                    } else {
                        20 as libc::c_int
                    });
                crate::src::opus_1_2_1::celt::laplace::ec_laplace_encode(
                    enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    &mut qi,
                    ((*prob_model.offset(pi as isize) as libc::c_int) << 7 as libc::c_int)
                        as libc::c_uint,
                    (*prob_model.offset((pi + 1 as libc::c_int) as isize) as libc::c_int)
                        << 6 as libc::c_int,
                );
            } else if budget - tell >= 2 as libc::c_int {
                qi = if -(1 as libc::c_int)
                    > (if qi < 1 as libc::c_int {
                        qi
                    } else {
                        1 as libc::c_int
                    }) {
                    -(1 as libc::c_int)
                } else if qi < 1 as libc::c_int {
                    qi
                } else {
                    1 as libc::c_int
                };
                crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
                    enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    2 as libc::c_int * qi ^ -((qi < 0 as libc::c_int) as libc::c_int),
                    small_energy_icdf.as_ptr(),
                    2 as libc::c_int as libc::c_uint,
                );
            } else if budget - tell >= 1 as libc::c_int {
                qi = if (0 as libc::c_int) < qi {
                    0 as libc::c_int
                } else {
                    qi
                };
                crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                    enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    -qi,
                    1 as libc::c_int as libc::c_uint,
                );
            } else {
                qi = -(1 as libc::c_int)
            }
            *error.offset((i + c * (*m).nbEBands) as isize) = f - qi as libc::c_float;
            badness += ::libc::abs(qi0 - qi);
            q = qi as crate::arch_h::opus_val32;
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
    return if lfe != 0 { 0 as libc::c_int } else { badness };
}
#[no_mangle]

pub unsafe extern "C" fn quant_coarse_energy(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut effEnd: libc::c_int,
    mut eBands: *const crate::arch_h::opus_val16,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut budget: crate::opus_types_h::opus_uint32,
    mut error: *mut crate::arch_h::opus_val16,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut C: libc::c_int,
    mut LM: libc::c_int,
    mut nbAvailableBytes: libc::c_int,
    mut force_intra: libc::c_int,
    mut delayedIntra: *mut crate::arch_h::opus_val32,
    mut two_pass: libc::c_int,
    mut loss_rate: libc::c_int,
    mut lfe: libc::c_int,
) {
    let mut intra: libc::c_int = 0;
    let mut max_decay: crate::arch_h::opus_val16 = 0.;
    let mut oldEBands_intra: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut error_intra: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut enc_start_state: crate::src::opus_1_2_1::celt::entcode::ec_enc =
        crate::src::opus_1_2_1::celt::entcode::ec_enc {
            buf: 0 as *mut libc::c_uchar,
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
    let mut tell: crate::opus_types_h::opus_uint32 = 0;
    let mut badness1: libc::c_int = 0 as libc::c_int;
    let mut intra_bias: crate::opus_types_h::opus_int32 = 0;
    let mut new_distortion: crate::arch_h::opus_val32 = 0.;
    intra = (force_intra != 0
        || two_pass == 0
            && *delayedIntra > (2 as libc::c_int * C * (end - start)) as libc::c_float
            && nbAvailableBytes > (end - start) * C) as libc::c_int;
    intra_bias = (budget as libc::c_float * *delayedIntra * loss_rate as libc::c_float
        / (C * 512 as libc::c_int) as libc::c_float)
        as crate::opus_types_h::opus_int32;
    new_distortion = loss_distortion(eBands, oldEBands, start, effEnd, (*m).nbEBands, C);
    tell = ec_tell(enc) as crate::opus_types_h::opus_uint32;
    if tell.wrapping_add(3 as libc::c_int as libc::c_uint) > budget {
        intra = 0 as libc::c_int;
        two_pass = intra
    }
    max_decay = 16.0f32;
    if end - start > 10 as libc::c_int {
        max_decay = if max_decay < 0.125f32 * nbAvailableBytes as libc::c_float {
            max_decay
        } else {
            (0.125f32) * nbAvailableBytes as libc::c_float
        }
    }
    if lfe != 0 {
        max_decay = 3.0f32
    }
    enc_start_state = *enc;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul((C * (*m).nbEBands) as libc::c_ulong) as usize,
    );
    oldEBands_intra = fresh0.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul((C * (*m).nbEBands) as libc::c_ulong) as usize,
    );
    error_intra = fresh1.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    crate::stdlib::memcpy(
        oldEBands_intra as *mut libc::c_void,
        oldEBands as *const libc::c_void,
        ((C * (*m).nbEBands) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long
                    * oldEBands_intra.wrapping_offset_from(oldEBands) as libc::c_long)
                    as libc::c_ulong,
            ),
    );
    if two_pass != 0 || intra != 0 {
        badness1 = quant_coarse_energy_impl(
            m,
            start,
            end,
            eBands,
            oldEBands_intra,
            budget as crate::opus_types_h::opus_int32,
            tell as crate::opus_types_h::opus_int32,
            e_prob_model[LM as usize][1 as libc::c_int as usize].as_ptr(),
            error_intra,
            enc,
            C,
            LM,
            1 as libc::c_int,
            max_decay,
            lfe,
        )
    }
    if intra == 0 {
        let mut intra_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut enc_intra_state: crate::src::opus_1_2_1::celt::entcode::ec_enc =
            crate::src::opus_1_2_1::celt::entcode::ec_enc {
                buf: 0 as *mut libc::c_uchar,
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
        let mut tell_intra: crate::opus_types_h::opus_int32 = 0;
        let mut nstart_bytes: crate::opus_types_h::opus_uint32 = 0;
        let mut nintra_bytes: crate::opus_types_h::opus_uint32 = 0;
        let mut save_bytes: crate::opus_types_h::opus_uint32 = 0;
        let mut badness2: libc::c_int = 0;
        let mut intra_bits: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        tell_intra = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        ) as crate::opus_types_h::opus_int32;
        enc_intra_state = *enc;
        nstart_bytes = ec_range_bytes(&mut enc_start_state);
        nintra_bytes = ec_range_bytes(&mut enc_intra_state);
        intra_buf = ec_get_buffer(&mut enc_intra_state).offset(nstart_bytes as isize);
        save_bytes = nintra_bytes.wrapping_sub(nstart_bytes);
        if save_bytes == 0 as libc::c_int as libc::c_uint {
            save_bytes = 0 as libc::c_int as crate::opus_types_h::opus_uint32
        }
        let mut fresh2 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                .wrapping_mul(save_bytes as libc::c_ulong) as usize,
        );
        intra_bits = fresh2.as_mut_ptr() as *mut libc::c_uchar;
        /* Copy bits from intra bit-stream */
        crate::stdlib::memcpy(
            intra_bits as *mut libc::c_void,
            intra_buf as *const libc::c_void,
            (nintra_bytes.wrapping_sub(nstart_bytes) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * intra_bits.wrapping_offset_from(intra_buf) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        *enc = enc_start_state;
        badness2 = quant_coarse_energy_impl(
            m,
            start,
            end,
            eBands,
            oldEBands,
            budget as crate::opus_types_h::opus_int32,
            tell as crate::opus_types_h::opus_int32,
            e_prob_model[LM as usize][intra as usize].as_ptr(),
            error,
            enc,
            C,
            LM,
            0 as libc::c_int,
            max_decay,
            lfe,
        );
        if two_pass != 0
            && (badness1 < badness2
                || badness1 == badness2
                    && crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(
                        enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    ) as crate::opus_types_h::opus_int32
                        + intra_bias
                        > tell_intra)
        {
            *enc = enc_intra_state;
            /* Copy intra bits to bit-stream */
            crate::stdlib::memcpy(
                intra_buf as *mut libc::c_void,
                intra_bits as *const libc::c_void,
                (nintra_bytes.wrapping_sub(nstart_bytes) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * intra_buf.wrapping_offset_from(intra_bits) as libc::c_long)
                            as libc::c_ulong,
                    ),
            );
            crate::stdlib::memcpy(
                oldEBands as *mut libc::c_void,
                oldEBands_intra as *const libc::c_void,
                ((C * (*m).nbEBands) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong
                    )
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * oldEBands.wrapping_offset_from(oldEBands_intra) as libc::c_long)
                            as libc::c_ulong,
                    ),
            );
            crate::stdlib::memcpy(
                error as *mut libc::c_void,
                error_intra as *const libc::c_void,
                ((C * (*m).nbEBands) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong
                    )
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * error.wrapping_offset_from(error_intra) as libc::c_long)
                            as libc::c_ulong,
                    ),
            );
            intra = 1 as libc::c_int
        }
    } else {
        crate::stdlib::memcpy(
            oldEBands as *mut libc::c_void,
            oldEBands_intra as *const libc::c_void,
            ((C * (*m).nbEBands) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * oldEBands.wrapping_offset_from(oldEBands_intra) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        crate::stdlib::memcpy(
            error as *mut libc::c_void,
            error_intra as *const libc::c_void,
            ((C * (*m).nbEBands) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * error.wrapping_offset_from(error_intra) as libc::c_long)
                        as libc::c_ulong,
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
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut error: *mut crate::arch_h::opus_val16,
    mut fine_quant: *mut libc::c_int,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut C: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    /* Encode finer resolution */
    i = start;
    while i < end {
        let mut frac: crate::opus_types_h::opus_int16 = ((1 as libc::c_int)
            << *fine_quant.offset(i as isize))
            as crate::opus_types_h::opus_int16;
        if !(*fine_quant.offset(i as isize) <= 0 as libc::c_int) {
            c = 0 as libc::c_int;
            loop {
                let mut q2: libc::c_int = 0;
                let mut offset: crate::arch_h::opus_val16 = 0.;
                q2 = crate::stdlib::floor(
                    ((*error.offset((i + c * (*m).nbEBands) as isize) + 0.5f32)
                        * frac as libc::c_int as libc::c_float)
                        as libc::c_double,
                ) as libc::c_int;
                if q2 > frac as libc::c_int - 1 as libc::c_int {
                    q2 = frac as libc::c_int - 1 as libc::c_int
                }
                if q2 < 0 as libc::c_int {
                    q2 = 0 as libc::c_int
                }
                crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
                    enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    q2 as crate::opus_types_h::opus_uint32,
                    *fine_quant.offset(i as isize) as libc::c_uint,
                );
                offset = (q2 as libc::c_float + 0.5f32)
                    * ((1 as libc::c_int) << 14 as libc::c_int - *fine_quant.offset(i as isize))
                        as libc::c_float
                    * (1.0f32 / 16384 as libc::c_int as libc::c_float)
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
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut error: *mut crate::arch_h::opus_val16,
    mut fine_quant: *mut libc::c_int,
    mut fine_priority: *mut libc::c_int,
    mut bits_left: libc::c_int,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut C: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut prio: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    /* Use up the remaining bits */
    prio = 0 as libc::c_int;
    while prio < 2 as libc::c_int {
        i = start;
        while i < end && bits_left >= C {
            if !(*fine_quant.offset(i as isize) >= 8 as libc::c_int
                || *fine_priority.offset(i as isize) != prio)
            {
                c = 0 as libc::c_int;
                loop {
                    let mut q2: libc::c_int = 0;
                    let mut offset: crate::arch_h::opus_val16 = 0.;
                    q2 = if *error.offset((i + c * (*m).nbEBands) as isize)
                        < 0 as libc::c_int as libc::c_float
                    {
                        0 as libc::c_int
                    } else {
                        1 as libc::c_int
                    };
                    crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
                        enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        q2 as crate::opus_types_h::opus_uint32,
                        1 as libc::c_int as libc::c_uint,
                    );
                    offset = (q2 as libc::c_float - 0.5f32)
                        * ((1 as libc::c_int)
                            << 14 as libc::c_int
                                - *fine_quant.offset(i as isize)
                                - 1 as libc::c_int) as libc::c_float
                        * (1.0f32 / 16384 as libc::c_int as libc::c_float);
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
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut intra: libc::c_int,
    mut dec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut C: libc::c_int,
    mut LM: libc::c_int,
) {
    let mut prob_model: *const libc::c_uchar = e_prob_model[LM as usize][intra as usize].as_ptr();
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut prev: [crate::arch_h::opus_val32; 2] = [
        0 as libc::c_int as crate::arch_h::opus_val32,
        0 as libc::c_int as crate::arch_h::opus_val32,
    ];
    let mut coef: crate::arch_h::opus_val16 = 0.;
    let mut beta: crate::arch_h::opus_val16 = 0.;
    let mut budget: crate::opus_types_h::opus_int32 = 0;
    let mut tell: crate::opus_types_h::opus_int32 = 0;
    if intra != 0 {
        coef = 0 as libc::c_int as crate::arch_h::opus_val16;
        beta = beta_intra
    } else {
        beta = beta_coef[LM as usize];
        coef = pred_coef[LM as usize]
    }
    budget = (*dec)
        .storage
        .wrapping_mul(8 as libc::c_int as libc::c_uint)
        as crate::opus_types_h::opus_int32;
    /* Decode at a fixed coarse resolution */
    i = start;
    while i < end {
        c = 0 as libc::c_int;
        loop {
            let mut qi: libc::c_int = 0;
            let mut q: crate::arch_h::opus_val32 = 0.;
            let mut tmp: crate::arch_h::opus_val32 = 0.;
            /* It would be better to express this invariant as a
            test on C at function entry, but that isn't enough
            to make the static analyzer happy. */
            tell = ec_tell(dec);
            if budget - tell >= 15 as libc::c_int {
                let mut pi: libc::c_int = 0;
                pi = 2 as libc::c_int
                    * (if i < 20 as libc::c_int {
                        i
                    } else {
                        20 as libc::c_int
                    });
                qi = crate::src::opus_1_2_1::celt::laplace::ec_laplace_decode(
                    dec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    ((*prob_model.offset(pi as isize) as libc::c_int) << 7 as libc::c_int)
                        as libc::c_uint,
                    (*prob_model.offset((pi + 1 as libc::c_int) as isize) as libc::c_int)
                        << 6 as libc::c_int,
                )
            } else if budget - tell >= 2 as libc::c_int {
                qi = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                    dec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    small_energy_icdf.as_ptr(),
                    2 as libc::c_int as libc::c_uint,
                );
                qi = qi >> 1 as libc::c_int ^ -(qi & 1 as libc::c_int)
            } else if budget - tell >= 1 as libc::c_int {
                qi = -crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(
                    dec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    1 as libc::c_int as libc::c_uint,
                )
            } else {
                qi = -(1 as libc::c_int)
            }
            q = qi as crate::arch_h::opus_val32;
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
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut fine_quant: *mut libc::c_int,
    mut dec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut C: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    /* Decode finer resolution */
    i = start;
    while i < end {
        if !(*fine_quant.offset(i as isize) <= 0 as libc::c_int) {
            c = 0 as libc::c_int;
            loop {
                let mut q2: libc::c_int = 0;
                let mut offset: crate::arch_h::opus_val16 = 0.;
                q2 = crate::src::opus_1_2_1::celt::entdec::ec_dec_bits(
                    dec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    *fine_quant.offset(i as isize) as libc::c_uint,
                ) as libc::c_int;
                offset = (q2 as libc::c_float + 0.5f32)
                    * ((1 as libc::c_int) << 14 as libc::c_int - *fine_quant.offset(i as isize))
                        as libc::c_float
                    * (1.0f32 / 16384 as libc::c_int as libc::c_float)
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
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut oldEBands: *mut crate::arch_h::opus_val16,
    mut fine_quant: *mut libc::c_int,
    mut fine_priority: *mut libc::c_int,
    mut bits_left: libc::c_int,
    mut dec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut C: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut prio: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    /* Use up the remaining bits */
    prio = 0 as libc::c_int;
    while prio < 2 as libc::c_int {
        i = start;
        while i < end && bits_left >= C {
            if !(*fine_quant.offset(i as isize) >= 8 as libc::c_int
                || *fine_priority.offset(i as isize) != prio)
            {
                c = 0 as libc::c_int;
                loop {
                    let mut q2: libc::c_int = 0;
                    let mut offset: crate::arch_h::opus_val16 = 0.;
                    q2 = crate::src::opus_1_2_1::celt::entdec::ec_dec_bits(
                        dec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        1 as libc::c_int as libc::c_uint,
                    ) as libc::c_int;
                    offset = (q2 as libc::c_float - 0.5f32)
                        * ((1 as libc::c_int)
                            << 14 as libc::c_int
                                - *fine_quant.offset(i as isize)
                                - 1 as libc::c_int) as libc::c_float
                        * (1.0f32 / 16384 as libc::c_int as libc::c_float);
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
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut effEnd: libc::c_int,
    mut end: libc::c_int,
    mut bandE: *mut crate::arch_h::celt_ener,
    mut bandLogE: *mut crate::arch_h::opus_val16,
    mut C: libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
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

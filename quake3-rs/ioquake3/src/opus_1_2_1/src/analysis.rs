// =============== BEGIN analysis_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TonalityAnalysisState {
    pub arch: i32,
    pub application: i32,
    pub Fs: crate::opus_types_h::opus_int32,
    pub angle: [f32; 240],
    pub d_angle: [f32; 240],
    pub d2_angle: [f32; 240],
    pub inmem: [crate::arch_h::opus_val32; 720],
    pub mem_fill: i32,
    pub prev_band_tonality: [f32; 18],
    pub prev_tonality: f32,
    pub prev_bandwidth: i32,
    pub E: [[f32; 18]; 8],
    pub logE: [[f32; 18]; 8],
    pub lowE: [f32; 18],
    pub highE: [f32; 18],
    pub meanE: [f32; 19],
    pub mem: [f32; 32],
    pub cmean: [f32; 8],
    pub std: [f32; 9],
    pub music_prob: f32,
    pub vad_prob: f32,
    pub Etracker: f32,
    pub lowECount: f32,
    pub E_count: i32,
    pub last_music: i32,
    pub count: i32,
    pub analysis_offset: i32,
    pub pspeech: [f32; 100],
    pub pmusic: [f32; 100],
    pub speech_confidence: f32,
    pub music_confidence: f32,
    pub speech_confidence_count: i32,
    pub music_confidence_count: i32,
    pub write_pos: i32,
    pub read_pos: i32,
    pub read_subframe: i32,
    pub hp_ener_accum: f32,
    pub downmix_state: [crate::arch_h::opus_val32; 3],
    pub info: [crate::celt_h::AnalysisInfo; 100],
}
use ::libc;

pub mod arch_h {

    /* Copyright (c) 2003-2008 Jean-Marc Valin
    Copyright (c) 2007-2008 CSIRO
    Copyright (c) 2007-2009 Xiph.Org Foundation
    Written by Jean-Marc Valin */
    /* *
       @file arch.h
       @brief Various architecture definitions for CELT
    */
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
    /* *< Minimum 16-bit value.   */
    /* *< Maximum 16-bit value.   */
    /* *< Minimum 32-bit value.   */
    /* *< Maximum 32-bit value.   */
    /* *< Minimum int value.   */
    /* *< Maximum int value.   */
    /* Set this if opus_int64 is a native type of the CPU. */
    /* Assume that all LP64 architectures have fast 64-bit types; also x86_64
    (which can be ILP32 for x32) and Win64 (which is LLP64). */
    /* FIXED_POINT */
    /* This code should reliably detect NaN/inf even when -ffast-math is used.
    Assumes IEEE 754 format. */
    #[inline]

    pub unsafe extern "C" fn celt_isnan(mut x: f32) -> i32 {
        let mut in_0: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        in_0.f = x;
        return (in_0.i >> 23 as i32 & 0xff as i32 as u32 == 0xff as i32 as u32
            && in_0.i & 0x7fffff as i32 as u32 != 0 as i32 as u32) as i32;
    }

    /* ARCH_H */
    /* !FIXED_POINT */
    /* This appears to be the same speed as C99's fabsf() but it's more portable. */
}

pub mod mathops_h {
    #[inline]

    pub unsafe extern "C" fn fast_atan2f(mut y: f32, mut x: f32) -> f32 {
        let mut x2: f32 = 0.;
        let mut y2: f32 = 0.;
        x2 = x * x;
        y2 = y * y;
        if x2 + y2 < 1e-18f32 {
            return 0 as i32 as f32;
        }
        if x2 < y2 {
            let mut den: f32 = (y2 + 0.67848403f32 * x2) * (y2 + 0.08595542f32 * x2);
            return -x * y * (y2 + 0.43157974f32 * x2) / den
                + (if y < 0 as i32 as f32 {
                    -(3.141592653f32 / 2 as i32 as f32)
                } else {
                    (3.141592653f32) / 2 as i32 as f32
                });
        } else {
            let mut den_0: f32 = (x2 + 0.67848403f32 * y2) * (x2 + 0.08595542f32 * y2);
            return x * y * (x2 + 0.43157974f32 * y2) / den_0
                + (if y < 0 as i32 as f32 {
                    -(3.141592653f32 / 2 as i32 as f32)
                } else {
                    (3.141592653f32) / 2 as i32 as f32
                })
                - (if x * y < 0 as i32 as f32 {
                    -(3.141592653f32 / 2 as i32 as f32)
                } else {
                    (3.141592653f32) / 2 as i32 as f32
                });
        };
    }
    /* MATHOPS_H */
    /* FIXED_POINT */
}

pub mod cpu_support_h {
    /* Copyright (c) 2010 Xiph.Org Foundation
     * Copyright (c) 2013 Parrot */
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
    #[inline]

    pub unsafe extern "C" fn opus_select_arch() -> i32 {
        return 0 as i32;
    }
}

pub mod float_cast_h {
    #[inline]

    pub unsafe extern "C" fn float2int(mut x: f32) -> crate::opus_types_h::opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }

    use ::std::arch::x86_64::_mm_cvt_ss2si;
    use ::std::arch::x86_64::_mm_set_ss;
    /* FLOAT_CAST_H */
    /* DISABLE_FLOAT_API */
}

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::arch_h::opus_val64;
pub use crate::celt_h::AnalysisInfo;
pub use crate::mathops_h::C2RustUnnamed_61;
pub use crate::opus_private_h::downmix_func;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::kiss_fft::opus_fft_c;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;
pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;
pub use crate::src::opus_1_2_1::src::analysis::arch_h::celt_isnan;

pub use crate::src::opus_1_2_1::src::analysis::cpu_support_h::opus_select_arch;
pub use crate::src::opus_1_2_1::src::analysis::float_cast_h::float2int;
pub use crate::src::opus_1_2_1::src::analysis::mathops_h::fast_atan2f;
pub use crate::src::opus_1_2_1::src::mlp::mlp_process;
pub use crate::src::opus_1_2_1::src::mlp::MLP;
pub use crate::src::opus_1_2_1::src::mlp_data::net;

/* Copyright (c) 2011 Xiph.Org Foundation
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
   A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR
   CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

static mut dct_table: [f32; 128] = [
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.250000f32,
    0.351851f32,
    0.338330f32,
    0.311806f32,
    0.273300f32,
    0.224292f32,
    0.166664f32,
    0.102631f32,
    0.034654f32,
    -0.034654f32,
    -0.102631f32,
    -0.166664f32,
    -0.224292f32,
    -0.273300f32,
    -0.311806f32,
    -0.338330f32,
    -0.351851f32,
    0.346760f32,
    0.293969f32,
    0.196424f32,
    0.068975f32,
    -0.068975f32,
    -0.196424f32,
    -0.293969f32,
    -0.346760f32,
    -0.346760f32,
    -0.293969f32,
    -0.196424f32,
    -0.068975f32,
    0.068975f32,
    0.196424f32,
    0.293969f32,
    0.346760f32,
    0.338330f32,
    0.224292f32,
    0.034654f32,
    -0.166664f32,
    -0.311806f32,
    -0.351851f32,
    -0.273300f32,
    -0.102631f32,
    0.102631f32,
    0.273300f32,
    0.351851f32,
    0.311806f32,
    0.166664f32,
    -0.034654f32,
    -0.224292f32,
    -0.338330f32,
    0.326641f32,
    0.135299f32,
    -0.135299f32,
    -0.326641f32,
    -0.326641f32,
    -0.135299f32,
    0.135299f32,
    0.326641f32,
    0.326641f32,
    0.135299f32,
    -0.135299f32,
    -0.326641f32,
    -0.326641f32,
    -0.135299f32,
    0.135299f32,
    0.326641f32,
    0.311806f32,
    0.034654f32,
    -0.273300f32,
    -0.338330f32,
    -0.102631f32,
    0.224292f32,
    0.351851f32,
    0.166664f32,
    -0.166664f32,
    -0.351851f32,
    -0.224292f32,
    0.102631f32,
    0.338330f32,
    0.273300f32,
    -0.034654f32,
    -0.311806f32,
    0.293969f32,
    -0.068975f32,
    -0.346760f32,
    -0.196424f32,
    0.196424f32,
    0.346760f32,
    0.068975f32,
    -0.293969f32,
    -0.293969f32,
    0.068975f32,
    0.346760f32,
    0.196424f32,
    -0.196424f32,
    -0.346760f32,
    -0.068975f32,
    0.293969f32,
    0.273300f32,
    -0.166664f32,
    -0.338330f32,
    0.034654f32,
    0.351851f32,
    0.102631f32,
    -0.311806f32,
    -0.224292f32,
    0.224292f32,
    0.311806f32,
    -0.102631f32,
    -0.351851f32,
    -0.034654f32,
    0.338330f32,
    0.166664f32,
    -0.273300f32,
];

static mut analysis_window: [f32; 240] = [
    0.000043f32,
    0.000171f32,
    0.000385f32,
    0.000685f32,
    0.001071f32,
    0.001541f32,
    0.002098f32,
    0.002739f32,
    0.003466f32,
    0.004278f32,
    0.005174f32,
    0.006156f32,
    0.007222f32,
    0.008373f32,
    0.009607f32,
    0.010926f32,
    0.012329f32,
    0.013815f32,
    0.015385f32,
    0.017037f32,
    0.018772f32,
    0.020590f32,
    0.022490f32,
    0.024472f32,
    0.026535f32,
    0.028679f32,
    0.030904f32,
    0.033210f32,
    0.035595f32,
    0.038060f32,
    0.040604f32,
    0.043227f32,
    0.045928f32,
    0.048707f32,
    0.051564f32,
    0.054497f32,
    0.057506f32,
    0.060591f32,
    0.063752f32,
    0.066987f32,
    0.070297f32,
    0.073680f32,
    0.077136f32,
    0.080665f32,
    0.084265f32,
    0.087937f32,
    0.091679f32,
    0.095492f32,
    0.099373f32,
    0.103323f32,
    0.107342f32,
    0.111427f32,
    0.115579f32,
    0.119797f32,
    0.124080f32,
    0.128428f32,
    0.132839f32,
    0.137313f32,
    0.141849f32,
    0.146447f32,
    0.151105f32,
    0.155823f32,
    0.160600f32,
    0.165435f32,
    0.170327f32,
    0.175276f32,
    0.180280f32,
    0.185340f32,
    0.190453f32,
    0.195619f32,
    0.200838f32,
    0.206107f32,
    0.211427f32,
    0.216797f32,
    0.222215f32,
    0.227680f32,
    0.233193f32,
    0.238751f32,
    0.244353f32,
    0.250000f32,
    0.255689f32,
    0.261421f32,
    0.267193f32,
    0.273005f32,
    0.278856f32,
    0.284744f32,
    0.290670f32,
    0.296632f32,
    0.302628f32,
    0.308658f32,
    0.314721f32,
    0.320816f32,
    0.326941f32,
    0.333097f32,
    0.339280f32,
    0.345492f32,
    0.351729f32,
    0.357992f32,
    0.364280f32,
    0.370590f32,
    0.376923f32,
    0.383277f32,
    0.389651f32,
    0.396044f32,
    0.402455f32,
    0.408882f32,
    0.415325f32,
    0.421783f32,
    0.428254f32,
    0.434737f32,
    0.441231f32,
    0.447736f32,
    0.454249f32,
    0.460770f32,
    0.467298f32,
    0.473832f32,
    0.480370f32,
    0.486912f32,
    0.493455f32,
    0.500000f32,
    0.506545f32,
    0.513088f32,
    0.519630f32,
    0.526168f32,
    0.532702f32,
    0.539230f32,
    0.545751f32,
    0.552264f32,
    0.558769f32,
    0.565263f32,
    0.571746f32,
    0.578217f32,
    0.584675f32,
    0.591118f32,
    0.597545f32,
    0.603956f32,
    0.610349f32,
    0.616723f32,
    0.623077f32,
    0.629410f32,
    0.635720f32,
    0.642008f32,
    0.648271f32,
    0.654508f32,
    0.660720f32,
    0.666903f32,
    0.673059f32,
    0.679184f32,
    0.685279f32,
    0.691342f32,
    0.697372f32,
    0.703368f32,
    0.709330f32,
    0.715256f32,
    0.721144f32,
    0.726995f32,
    0.732807f32,
    0.738579f32,
    0.744311f32,
    0.750000f32,
    0.755647f32,
    0.761249f32,
    0.766807f32,
    0.772320f32,
    0.777785f32,
    0.783203f32,
    0.788573f32,
    0.793893f32,
    0.799162f32,
    0.804381f32,
    0.809547f32,
    0.814660f32,
    0.819720f32,
    0.824724f32,
    0.829673f32,
    0.834565f32,
    0.839400f32,
    0.844177f32,
    0.848895f32,
    0.853553f32,
    0.858151f32,
    0.862687f32,
    0.867161f32,
    0.871572f32,
    0.875920f32,
    0.880203f32,
    0.884421f32,
    0.888573f32,
    0.892658f32,
    0.896677f32,
    0.900627f32,
    0.904508f32,
    0.908321f32,
    0.912063f32,
    0.915735f32,
    0.919335f32,
    0.922864f32,
    0.926320f32,
    0.929703f32,
    0.933013f32,
    0.936248f32,
    0.939409f32,
    0.942494f32,
    0.945503f32,
    0.948436f32,
    0.951293f32,
    0.954072f32,
    0.956773f32,
    0.959396f32,
    0.961940f32,
    0.964405f32,
    0.966790f32,
    0.969096f32,
    0.971321f32,
    0.973465f32,
    0.975528f32,
    0.977510f32,
    0.979410f32,
    0.981228f32,
    0.982963f32,
    0.984615f32,
    0.986185f32,
    0.987671f32,
    0.989074f32,
    0.990393f32,
    0.991627f32,
    0.992778f32,
    0.993844f32,
    0.994826f32,
    0.995722f32,
    0.996534f32,
    0.997261f32,
    0.997902f32,
    0.998459f32,
    0.998929f32,
    0.999315f32,
    0.999615f32,
    0.999829f32,
    0.999957f32,
    1.000000f32,
];

static mut tbands: [i32; 19] = [
    4 as i32, 8 as i32, 12 as i32, 16 as i32, 20 as i32, 24 as i32, 28 as i32, 32 as i32,
    40 as i32, 48 as i32, 56 as i32, 64 as i32, 80 as i32, 96 as i32, 112 as i32, 136 as i32,
    160 as i32, 192 as i32, 240 as i32,
];

unsafe extern "C" fn silk_resampler_down2_hp(
    mut S: *mut opus_val32,
    mut out: *mut opus_val32,
    mut in_0: *const opus_val32,
    mut inLen: i32,
) -> opus_val32
/* I    Number of input samples                                     */ {
    let mut k: i32 = 0;
    let mut len2: i32 = inLen / 2 as i32;
    let mut in32: opus_val32 = 0.;
    let mut out32: opus_val32 = 0.;
    let mut out32_hp: opus_val32 = 0.;
    let mut Y: opus_val32 = 0.;
    let mut X: opus_val32 = 0.;
    let mut hp_ener: opus_val64 = 0 as i32 as opus_val64;
    /* Internal variables and state are in Q10 format */
    k = 0 as i32;
    while k < len2 {
        /* Convert to Q10 */
        in32 = *in_0.offset((2 as i32 * k) as isize);
        /* All-pass section for even input sample */
        Y = in32 - *S.offset(0 as i32 as isize);
        X = 0.6074371f32 * Y;
        out32 = *S.offset(0 as i32 as isize) + X;
        *S.offset(0 as i32 as isize) = in32 + X;
        out32_hp = out32;
        /* Convert to Q10 */
        in32 = *in_0.offset((2 as i32 * k + 1 as i32) as isize);
        /* All-pass section for odd input sample, and add to output of previous section */
        Y = in32 - *S.offset(1 as i32 as isize);
        X = 0.15063f32 * Y;
        out32 = out32 + *S.offset(1 as i32 as isize);
        out32 = out32 + X;
        *S.offset(1 as i32 as isize) = in32 + X;
        Y = -in32 - *S.offset(2 as i32 as isize);
        X = 0.15063f32 * Y;
        out32_hp = out32_hp + *S.offset(2 as i32 as isize);
        out32_hp = out32_hp + X;
        *S.offset(2 as i32 as isize) = -in32 + X;
        hp_ener += out32_hp * out32_hp;
        /* Add, convert back to int16 and store to output */
        *out.offset(k as isize) = 0.5f32 * out32;
        k += 1
    }
    return hp_ener;
}

unsafe extern "C" fn downmix_and_resample(
    mut downmix: downmix_func,
    mut _x: *const libc::c_void,
    mut y: *mut opus_val32,
    mut S: *mut opus_val32,
    mut subframe: i32,
    mut offset: i32,
    mut c1: i32,
    mut c2: i32,
    mut C: i32,
    mut Fs: i32,
) -> opus_val32 {
    let mut tmp: *mut opus_val32 = 0 as *mut opus_val32;
    let mut scale: opus_val32 = 0.;
    let mut j: i32 = 0;
    let mut ret: opus_val32 = 0 as i32 as opus_val32;
    if subframe == 0 as i32 {
        return 0 as i32 as opus_val32;
    }
    if Fs == 48000 as i32 {
        subframe *= 2 as i32;
        offset *= 2 as i32
    } else if Fs == 16000 as i32 {
        subframe = subframe * 2 as i32 / 3 as i32;
        offset = offset * 2 as i32 / 3 as i32
    }
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_val32>() as libc::c_ulong)
            .wrapping_mul(subframe as libc::c_ulong) as usize,
    );
    tmp = fresh0.as_mut_ptr() as *mut opus_val32;
    downmix.expect("non-null function pointer")(_x, tmp, subframe, offset, c1, c2, C);
    scale = 1.0f32 / 32768 as i32 as f32;
    if c2 == -(2 as i32) {
        scale /= C as f32
    } else if c2 > -(1 as i32) {
        scale /= 2 as i32 as f32
    }
    j = 0 as i32;
    while j < subframe {
        let ref mut fresh1 = *tmp.offset(j as isize);
        *fresh1 *= scale;
        j += 1
    }
    if Fs == 48000 as i32 {
        ret = silk_resampler_down2_hp(S, y, tmp, subframe)
    } else if Fs == 24000 as i32 {
        crate::stdlib::memcpy(
            y as *mut libc::c_void,
            tmp as *const libc::c_void,
            (subframe as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<opus_val32>() as libc::c_ulong)
                .wrapping_add((0 as i32 as isize * y.offset_from(tmp) as isize) as libc::c_ulong),
        );
    } else if Fs == 16000 as i32 {
        let mut tmp3x: *mut opus_val32 = 0 as *mut opus_val32;
        let mut fresh2 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<opus_val32>() as libc::c_ulong)
                .wrapping_mul((3 as i32 * subframe) as libc::c_ulong) as usize,
        );
        tmp3x = fresh2.as_mut_ptr() as *mut opus_val32;
        /* Don't do this at home! This resampler is horrible and it's only (barely)
        usable for the purpose of the analysis because we don't care about all
        the aliasing between 8 kHz and 12 kHz. */
        j = 0 as i32;
        while j < subframe {
            *tmp3x.offset((3 as i32 * j) as isize) = *tmp.offset(j as isize);
            *tmp3x.offset((3 as i32 * j + 1 as i32) as isize) = *tmp.offset(j as isize);
            *tmp3x.offset((3 as i32 * j + 2 as i32) as isize) = *tmp.offset(j as isize);
            j += 1
        }
        silk_resampler_down2_hp(S, y, tmp3x, 3 as i32 * subframe);
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn tonality_analysis_init(
    mut tonal: *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
    mut Fs: opus_int32,
) {
    /* Initialize reusable fields. */
    (*tonal).arch = opus_select_arch();
    (*tonal).Fs = Fs;
    /* Clear remaining fields. */
    tonality_analysis_reset(tonal);
}
/* * Initialize a TonalityAnalysisState struct.
 *
 * This performs some possibly slow initialization steps which should
 * not be repeated every analysis step. No allocated memory is retained
 * by the state struct, so no cleanup call is required.
 */
/* * Reset a TonalityAnalysisState stuct.
 *
 * Call this when there's a discontinuity in the data.
 */
#[no_mangle]

pub unsafe extern "C" fn tonality_analysis_reset(
    mut tonal: *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
) {
    /* Clear non-reusable fields. */
    let mut start: *mut libc::c_char = &mut (*tonal).angle as *mut [f32; 240] as *mut libc::c_char;
    crate::stdlib::memset(
        start as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState>()
            as libc::c_ulong)
            .wrapping_sub(start.offset_from(tonal as *mut libc::c_char) as isize as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    (*tonal).music_confidence = 0.9f32;
    (*tonal).speech_confidence = 0.1f32;
}
#[no_mangle]

pub unsafe extern "C" fn tonality_get_info(
    mut tonal: *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
    mut info_out: *mut AnalysisInfo,
    mut len: i32,
) {
    let mut pos: i32 = 0;
    let mut curr_lookahead: i32 = 0;
    let mut psum: f32 = 0.;
    let mut tonality_max: f32 = 0.;
    let mut tonality_avg: f32 = 0.;
    let mut tonality_count: i32 = 0;
    let mut i: i32 = 0;
    pos = (*tonal).read_pos;
    curr_lookahead = (*tonal).write_pos - (*tonal).read_pos;
    if curr_lookahead < 0 as i32 {
        curr_lookahead += 100 as i32
    }
    /* On long frames, look at the second analysis window rather than the first. */
    if len > (*tonal).Fs / 50 as i32 && pos != (*tonal).write_pos {
        pos += 1;
        if pos == 100 as i32 {
            pos = 0 as i32
        }
    }
    if pos == (*tonal).write_pos {
        pos -= 1
    }
    if pos < 0 as i32 {
        pos = 100 as i32 - 1 as i32
    }
    crate::stdlib::memcpy(
        info_out as *mut libc::c_void,
        &mut *(*tonal).info.as_mut_ptr().offset(pos as isize) as *mut AnalysisInfo
            as *const libc::c_void,
        (1 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<AnalysisInfo>() as libc::c_ulong)
            .wrapping_add(
                (0 as i32 as isize
                    * info_out.offset_from(&mut *(*tonal).info.as_mut_ptr().offset(pos as isize))
                        as isize) as libc::c_ulong,
            ),
    );
    tonality_avg = (*info_out).tonality;
    tonality_max = tonality_avg;
    tonality_count = 1 as i32;
    /* If possible, look ahead for a tone to compensate for the delay in the tone detector. */
    i = 0 as i32;
    while i < 3 as i32 {
        pos += 1;
        if pos == 100 as i32 {
            pos = 0 as i32
        }
        if pos == (*tonal).write_pos {
            break;
        }
        tonality_max = if tonality_max > (*tonal).info[pos as usize].tonality {
            tonality_max
        } else {
            (*tonal).info[pos as usize].tonality
        };
        tonality_avg += (*tonal).info[pos as usize].tonality;
        tonality_count += 1;
        i += 1
    }
    (*info_out).tonality = if tonality_avg / tonality_count as f32 > tonality_max - 0.2f32 {
        (tonality_avg) / tonality_count as f32
    } else {
        (tonality_max) - 0.2f32
    };
    (*tonal).read_subframe += len / ((*tonal).Fs / 400 as i32);
    while (*tonal).read_subframe >= 8 as i32 {
        (*tonal).read_subframe -= 8 as i32;
        (*tonal).read_pos += 1
    }
    if (*tonal).read_pos >= 100 as i32 {
        (*tonal).read_pos -= 100 as i32
    }
    /* The -1 is to compensate for the delay in the features themselves. */
    curr_lookahead = if curr_lookahead - 1 as i32 > 0 as i32 {
        (curr_lookahead) - 1 as i32
    } else {
        0 as i32
    };
    psum = 0 as i32 as f32;
    /* Summing the probability of transition patterns that involve music at
    time (DETECT_SIZE-curr_lookahead-1) */
    i = 0 as i32;
    while i < 100 as i32 - curr_lookahead {
        psum += (*tonal).pmusic[i as usize];
        i += 1
    }
    while i < 100 as i32 {
        psum += (*tonal).pspeech[i as usize];
        i += 1
    }
    psum = psum * (*tonal).music_confidence + (1 as i32 as f32 - psum) * (*tonal).speech_confidence;
    /*printf("%f %f %f %f %f\n", psum, info_out->music_prob, info_out->vad_prob, info_out->activity_probability, info_out->tonality);*/
    (*info_out).music_prob = psum;
}

static mut std_feature_bias: [f32; 9] = [
    5.684947f32,
    3.475288f32,
    1.770634f32,
    1.599784f32,
    3.773215f32,
    2.163313f32,
    1.260756f32,
    1.116868f32,
    1.918795f32,
];

unsafe extern "C" fn tonality_analysis(
    mut tonal: *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
    mut celt_mode: *const OpusCustomMode,
    mut x: *const libc::c_void,
    mut len: i32,
    mut offset: i32,
    mut c1: i32,
    mut c2: i32,
    mut C: i32,
    mut lsb_depth: i32,
    mut downmix: downmix_func,
) {
    let mut i: i32 = 0;
    let mut b: i32 = 0;
    let mut kfft: *const kiss_fft_state =
        0 as *const kiss_fft_state;
    let mut in_0: *mut kiss_fft_cpx =
        0 as *mut kiss_fft_cpx;
    let mut out: *mut kiss_fft_cpx =
        0 as *mut kiss_fft_cpx;
    let mut N: i32 = 480 as i32;
    let mut N2: i32 = 240 as i32;
    let mut A: *mut f32 = (*tonal).angle.as_mut_ptr();
    let mut dA: *mut f32 = (*tonal).d_angle.as_mut_ptr();
    let mut d2A: *mut f32 = (*tonal).d2_angle.as_mut_ptr();
    let mut tonality: *mut f32 = 0 as *mut f32;
    let mut noisiness: *mut f32 = 0 as *mut f32;
    let mut band_tonality: [f32; 18] = [0.; 18];
    let mut logE: [f32; 18] = [0.; 18];
    let mut BFCC: [f32; 8] = [0.; 8];
    let mut features: [f32; 25] = [0.; 25];
    let mut frame_tonality: f32 = 0.;
    let mut max_frame_tonality: f32 = 0.;
    /*float tw_sum=0;*/
    let mut frame_noisiness: f32 = 0.;
    let pi4: f32 = (3.14159265358979323846f64
        * 3.14159265358979323846f64
        * 3.14159265358979323846f64
        * 3.14159265358979323846f64) as f32;
    let mut slope: f32 = 0 as i32 as f32;
    let mut frame_stationarity: f32 = 0.;
    let mut relativeE: f32 = 0.;
    let mut frame_probs: [f32; 2] = [0.; 2];
    let mut alpha: f32 = 0.;
    let mut alphaE: f32 = 0.;
    let mut alphaE2: f32 = 0.;
    let mut frame_loudness: f32 = 0.;
    let mut bandwidth_mask: f32 = 0.;
    let mut bandwidth: i32 = 0 as i32;
    let mut maxE: f32 = 0 as i32 as f32;
    let mut noise_floor: f32 = 0.;
    let mut remaining: i32 = 0;
    let mut info: *mut AnalysisInfo = 0 as *mut AnalysisInfo;
    let mut hp_ener: f32 = 0.;
    let mut tonality2: [f32; 240] = [0.; 240];
    let mut midE: [f32; 8] = [0.; 8];
    let mut spec_variability: f32 = 0 as i32 as f32;
    let mut band_log2: [f32; 19] = [0.; 19];
    let mut leakage_from: [f32; 19] = [0.; 19];
    let mut leakage_to: [f32; 19] = [0.; 19];
    alpha = 1.0f32
        / (if (10 as i32) < 1 as i32 + (*tonal).count {
            10 as i32
        } else {
            (1 as i32) + (*tonal).count
        }) as f32;
    alphaE = 1.0f32
        / (if (25 as i32) < 1 as i32 + (*tonal).count {
            25 as i32
        } else {
            (1 as i32) + (*tonal).count
        }) as f32;
    alphaE2 = 1.0f32
        / (if (500 as i32) < 1 as i32 + (*tonal).count {
            500 as i32
        } else {
            (1 as i32) + (*tonal).count
        }) as f32;
    if (*tonal).Fs == 48000 as i32 {
        /* len and offset are now at 24 kHz. */
        len /= 2 as i32;
        offset /= 2 as i32
    } else if (*tonal).Fs == 16000 as i32 {
        len = 3 as i32 * len / 2 as i32;
        offset = 3 as i32 * offset / 2 as i32
    }
    if (*tonal).count < 4 as i32 {
        if (*tonal).application == 2048 as i32 {
            (*tonal).music_prob = 0.1f32
        } else {
            (*tonal).music_prob = 0.625f32
        }
    }
    kfft = (*celt_mode).mdct.kfft[0 as i32 as usize];
    if (*tonal).count == 0 as i32 {
        (*tonal).mem_fill = 240 as i32
    }
    (*tonal).hp_ener_accum += downmix_and_resample(
        downmix,
        x,
        &mut *(*tonal)
            .inmem
            .as_mut_ptr()
            .offset((*tonal).mem_fill as isize),
        (*tonal).downmix_state.as_mut_ptr(),
        if len < 720 as i32 - (*tonal).mem_fill {
            len
        } else {
            (720 as i32) - (*tonal).mem_fill
        },
        offset,
        c1,
        c2,
        C,
        (*tonal).Fs,
    );
    if (*tonal).mem_fill + len < 720 as i32 {
        (*tonal).mem_fill += len;
        /* Don't have enough to update the analysis */
        return;
    }
    hp_ener = (*tonal).hp_ener_accum;
    let fresh3 = (*tonal).write_pos;
    (*tonal).write_pos = (*tonal).write_pos + 1;
    info = &mut *(*tonal).info.as_mut_ptr().offset(fresh3 as isize)
        as *mut AnalysisInfo;
    if (*tonal).write_pos >= 100 as i32 {
        (*tonal).write_pos -= 100 as i32
    }
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<kiss_fft_cpx>()
            as libc::c_ulong)
            .wrapping_mul(480 as i32 as libc::c_ulong) as usize,
    );
    in_0 = fresh4.as_mut_ptr() as *mut kiss_fft_cpx;
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<kiss_fft_cpx>()
            as libc::c_ulong)
            .wrapping_mul(480 as i32 as libc::c_ulong) as usize,
    );
    out = fresh5.as_mut_ptr() as *mut kiss_fft_cpx;
    let mut fresh6 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f32>() as libc::c_ulong).wrapping_mul(240 as i32 as libc::c_ulong)
            as usize,
    );
    tonality = fresh6.as_mut_ptr() as *mut f32;
    let mut fresh7 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f32>() as libc::c_ulong).wrapping_mul(240 as i32 as libc::c_ulong)
            as usize,
    );
    noisiness = fresh7.as_mut_ptr() as *mut f32;
    i = 0 as i32;
    while i < N2 {
        let mut w: f32 = analysis_window[i as usize];
        (*in_0.offset(i as isize)).r = w * (*tonal).inmem[i as usize];
        (*in_0.offset(i as isize)).i = w * (*tonal).inmem[(N2 + i) as usize];
        (*in_0.offset((N - i - 1 as i32) as isize)).r =
            w * (*tonal).inmem[(N - i - 1 as i32) as usize];
        (*in_0.offset((N - i - 1 as i32) as isize)).i =
            w * (*tonal).inmem[(N + N2 - i - 1 as i32) as usize];
        i += 1
    }
    crate::stdlib::memmove(
        (*tonal).inmem.as_mut_ptr() as *mut libc::c_void,
        (*tonal)
            .inmem
            .as_mut_ptr()
            .offset(720 as i32 as isize)
            .offset(-(240 as i32 as isize)) as *const libc::c_void,
        (240 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<opus_val32>() as libc::c_ulong)
            .wrapping_add(
                (0 as i32 as isize
                    * (*tonal).inmem.as_mut_ptr().offset_from(
                        (*tonal)
                            .inmem
                            .as_mut_ptr()
                            .offset(720 as i32 as isize)
                            .offset(-(240 as i32 as isize)),
                    ) as isize) as libc::c_ulong,
            ),
    );
    remaining = len - (720 as i32 - (*tonal).mem_fill);
    (*tonal).hp_ener_accum = downmix_and_resample(
        downmix,
        x,
        &mut *(*tonal).inmem.as_mut_ptr().offset(240 as i32 as isize),
        (*tonal).downmix_state.as_mut_ptr(),
        remaining,
        offset + 720 as i32 - (*tonal).mem_fill,
        c1,
        c2,
        C,
        (*tonal).Fs,
    );
    (*tonal).mem_fill = 240 as i32 + remaining;
    opus_fft_c(
        kfft as *const kiss_fft_state,
        in_0 as *const kiss_fft_cpx,
        out as *mut kiss_fft_cpx,
    );
    /* If there's any NaN on the input, the entire output will be NaN, so we only need to check one value. */
    if celt_isnan((*out.offset(0 as i32 as isize)).r) != 0 {
        (*info).valid = 0 as i32;
        return;
    }
    i = 1 as i32;
    while i < N2 {
        let mut X1r: f32 = 0.;
        let mut X2r: f32 = 0.;
        let mut X1i: f32 = 0.;
        let mut X2i: f32 = 0.;
        let mut angle: f32 = 0.;
        let mut d_angle: f32 = 0.;
        let mut d2_angle: f32 = 0.;
        let mut angle2: f32 = 0.;
        let mut d_angle2: f32 = 0.;
        let mut d2_angle2: f32 = 0.;
        let mut mod1: f32 = 0.;
        let mut mod2: f32 = 0.;
        let mut avg_mod: f32 = 0.;
        X1r = (*out.offset(i as isize)).r + (*out.offset((N - i) as isize)).r;
        X1i = (*out.offset(i as isize)).i - (*out.offset((N - i) as isize)).i;
        X2r = (*out.offset(i as isize)).i + (*out.offset((N - i) as isize)).i;
        X2i = (*out.offset((N - i) as isize)).r - (*out.offset(i as isize)).r;
        angle = (0.5f32 as f64 / 3.14159265358979323846f64) as f32 * fast_atan2f(X1i, X1r);
        d_angle = angle - *A.offset(i as isize);
        d2_angle = d_angle - *dA.offset(i as isize);
        angle2 = (0.5f32 as f64 / 3.14159265358979323846f64) as f32 * fast_atan2f(X2i, X2r);
        d_angle2 = angle2 - angle;
        d2_angle2 = d_angle2 - d_angle;
        mod1 = d2_angle - float2int(d2_angle) as f32;
        *noisiness.offset(i as isize) = crate::stdlib::fabs(mod1 as f64) as f32;
        mod1 *= mod1;
        mod1 *= mod1;
        mod2 = d2_angle2 - float2int(d2_angle2) as f32;
        *noisiness.offset(i as isize) += crate::stdlib::fabs(mod2 as f64) as f32;
        mod2 *= mod2;
        mod2 *= mod2;
        avg_mod = 0.25f32 * (*d2A.offset(i as isize) + mod1 + 2 as i32 as f32 * mod2);
        /* This introduces an extra delay of 2 frames in the detection. */
        *tonality.offset(i as isize) =
            1.0f32 / (1.0f32 + 40.0f32 * 16.0f32 * pi4 * avg_mod) - 0.015f32;
        /* No delay on this detection, but it's less reliable. */
        tonality2[i as usize] = 1.0f32 / (1.0f32 + 40.0f32 * 16.0f32 * pi4 * mod2) - 0.015f32;
        *A.offset(i as isize) = angle2;
        *dA.offset(i as isize) = d_angle2;
        *d2A.offset(i as isize) = mod2;
        i += 1
    }
    i = 2 as i32;
    while i < N2 - 1 as i32 {
        let mut tt: f32 = if tonality2[i as usize]
            < (if tonality2[(i - 1 as i32) as usize] > tonality2[(i + 1 as i32) as usize] {
                tonality2[(i - 1 as i32) as usize]
            } else {
                tonality2[(i + 1 as i32) as usize]
            }) {
            tonality2[i as usize]
        } else if tonality2[(i - 1 as i32) as usize] > tonality2[(i + 1 as i32) as usize] {
            tonality2[(i - 1 as i32) as usize]
        } else {
            tonality2[(i + 1 as i32) as usize]
        };
        *tonality.offset(i as isize) = 0.9f32
            * (if *tonality.offset(i as isize) > tt - 0.1f32 {
                *tonality.offset(i as isize)
            } else {
                (tt) - 0.1f32
            });
        i += 1
    }
    frame_tonality = 0 as i32 as f32;
    max_frame_tonality = 0 as i32 as f32;
    /*tw_sum = 0;*/
    (*info).activity = 0 as i32 as f32;
    frame_noisiness = 0 as i32 as f32;
    frame_stationarity = 0 as i32 as f32;
    if (*tonal).count == 0 {
        b = 0 as i32;
        while b < 18 as i32 {
            (*tonal).lowE[b as usize] = 1e10f64 as f32;
            (*tonal).highE[b as usize] = -1e10f64 as f32;
            b += 1
        }
    }
    relativeE = 0 as i32 as f32;
    frame_loudness = 0 as i32 as f32;
    /* The energy of the very first band is special because of DC. */
    let mut E: f32 = 0 as i32 as f32;
    let mut X1r_0: f32 = 0.;
    let mut X2r_0: f32 = 0.;
    X1r_0 = 2 as i32 as f32 * (*out.offset(0 as i32 as isize)).r;
    X2r_0 = 2 as i32 as f32 * (*out.offset(0 as i32 as isize)).i;
    E = X1r_0 * X1r_0 + X2r_0 * X2r_0;
    i = 1 as i32;
    while i < 4 as i32 {
        let mut binE: f32 = (*out.offset(i as isize)).r * (*out.offset(i as isize)).r
            + (*out.offset((N - i) as isize)).r * (*out.offset((N - i) as isize)).r
            + (*out.offset(i as isize)).i * (*out.offset(i as isize)).i
            + (*out.offset((N - i) as isize)).i * (*out.offset((N - i) as isize)).i;
        E += binE;
        i += 1
    }
    E = E;
    band_log2[0 as i32 as usize] =
        0.5f32 * 1.442695f32 * crate::stdlib::log((E + 1e-10f32) as f64) as f32;
    b = 0 as i32;
    while b < 18 as i32 {
        let mut E_0: f32 = 0 as i32 as f32;
        let mut tE: f32 = 0 as i32 as f32;
        let mut nE: f32 = 0 as i32 as f32;
        let mut L1: f32 = 0.;
        let mut L2: f32 = 0.;
        let mut stationarity: f32 = 0.;
        i = tbands[b as usize];
        while i < tbands[(b + 1 as i32) as usize] {
            let mut binE_0: f32 = (*out.offset(i as isize)).r * (*out.offset(i as isize)).r
                + (*out.offset((N - i) as isize)).r * (*out.offset((N - i) as isize)).r
                + (*out.offset(i as isize)).i * (*out.offset(i as isize)).i
                + (*out.offset((N - i) as isize)).i * (*out.offset((N - i) as isize)).i;
            binE_0 = binE_0;
            E_0 += binE_0;
            tE += binE_0
                * (if 0 as i32 as f32 > *tonality.offset(i as isize) {
                    0 as i32 as f32
                } else {
                    *tonality.offset(i as isize)
                });
            nE += binE_0 * 2.0f32 * (0.5f32 - *noisiness.offset(i as isize));
            i += 1
        }
        /* Check for extreme band energies that could cause NaNs later. */
        if !(E_0 < 1e9f32) || celt_isnan(E_0) != 0 {
            (*info).valid = 0 as i32;
            return;
        }
        (*tonal).E[(*tonal).E_count as usize][b as usize] = E_0;
        frame_noisiness += nE / (1e-15f32 + E_0);
        frame_loudness += crate::stdlib::sqrt((E_0 + 1e-10f32) as f64) as f32;
        logE[b as usize] = crate::stdlib::log((E_0 + 1e-10f32) as f64) as f32;
        band_log2[(b + 1 as i32) as usize] =
            0.5f32 * 1.442695f32 * crate::stdlib::log((E_0 + 1e-10f32) as f64) as f32;
        (*tonal).logE[(*tonal).E_count as usize][b as usize] = logE[b as usize];
        if (*tonal).count == 0 as i32 {
            (*tonal).lowE[b as usize] = logE[b as usize];
            (*tonal).highE[b as usize] = (*tonal).lowE[b as usize]
        }
        if (*tonal).highE[b as usize] as f64 > (*tonal).lowE[b as usize] as f64 + 7.5f64 {
            if (*tonal).highE[b as usize] - logE[b as usize]
                > logE[b as usize] - (*tonal).lowE[b as usize]
            {
                (*tonal).highE[b as usize] -= 0.01f32
            } else {
                (*tonal).lowE[b as usize] += 0.01f32
            }
        }
        if logE[b as usize] > (*tonal).highE[b as usize] {
            (*tonal).highE[b as usize] = logE[b as usize];
            (*tonal).lowE[b as usize] =
                if (*tonal).highE[b as usize] - 15 as i32 as f32 > (*tonal).lowE[b as usize] {
                    ((*tonal).highE[b as usize]) - 15 as i32 as f32
                } else {
                    (*tonal).lowE[b as usize]
                }
        } else if logE[b as usize] < (*tonal).lowE[b as usize] {
            (*tonal).lowE[b as usize] = logE[b as usize];
            (*tonal).highE[b as usize] =
                if ((*tonal).lowE[b as usize] + 15 as i32 as f32) < (*tonal).highE[b as usize] {
                    ((*tonal).lowE[b as usize]) + 15 as i32 as f32
                } else {
                    (*tonal).highE[b as usize]
                }
        }
        relativeE += (logE[b as usize] - (*tonal).lowE[b as usize])
            / (1e-15f32 + ((*tonal).highE[b as usize] - (*tonal).lowE[b as usize]));
        L2 = 0 as i32 as f32;
        L1 = L2;
        i = 0 as i32;
        while i < 8 as i32 {
            L1 += crate::stdlib::sqrt((*tonal).E[i as usize][b as usize] as f64) as f32;
            L2 += (*tonal).E[i as usize][b as usize];
            i += 1
        }
        stationarity = if 0.99f32
            < L1 / crate::stdlib::sqrt(1e-15f64 + (8 as i32 as f32 * L2) as f64) as f32
        {
            0.99f32
        } else {
            (L1) / crate::stdlib::sqrt(1e-15f64 + (8 as i32 as f32 * L2) as f64) as f32
        };
        stationarity *= stationarity;
        stationarity *= stationarity;
        frame_stationarity += stationarity;
        /*band_tonality[b] = tE/(1e-15+E)*/
        band_tonality[b as usize] =
            if tE / (1e-15f32 + E_0) > stationarity * (*tonal).prev_band_tonality[b as usize] {
                (tE) / (1e-15f32 + E_0)
            } else {
                (stationarity) * (*tonal).prev_band_tonality[b as usize]
            };
        frame_tonality += band_tonality[b as usize];
        if b >= 18 as i32 - 9 as i32 {
            frame_tonality -= band_tonality[(b - 18 as i32 + 9 as i32) as usize]
        }
        max_frame_tonality =
            if max_frame_tonality > (1.0f32 + 0.03f32 * (b - 18 as i32) as f32) * frame_tonality {
                max_frame_tonality
            } else {
                (1.0f32 + 0.03f32 * (b - 18 as i32) as f32) * frame_tonality
            };
        slope += band_tonality[b as usize] * (b - 8 as i32) as f32;
        /*printf("%f %f ", band_tonality[b], stationarity);*/
        (*tonal).prev_band_tonality[b as usize] = band_tonality[b as usize];
        b += 1
    }
    leakage_from[0 as i32 as usize] = band_log2[0 as i32 as usize];
    leakage_to[0 as i32 as usize] = band_log2[0 as i32 as usize] - 2.5f32;
    b = 1 as i32;
    while b < 18 as i32 + 1 as i32 {
        let mut leak_slope: f32 = 2.0f32
            * (tbands[b as usize] - tbands[(b - 1 as i32) as usize]) as f32
            / 4 as i32 as f32;
        leakage_from[b as usize] =
            if leakage_from[(b - 1 as i32) as usize] + leak_slope < band_log2[b as usize] {
                (leakage_from[(b - 1 as i32) as usize]) + leak_slope
            } else {
                band_log2[b as usize]
            };
        leakage_to[b as usize] =
            if leakage_to[(b - 1 as i32) as usize] - leak_slope > band_log2[b as usize] - 2.5f32 {
                (leakage_to[(b - 1 as i32) as usize]) - leak_slope
            } else {
                (band_log2[b as usize]) - 2.5f32
            };
        b += 1
    }
    b = 18 as i32 - 2 as i32;
    while b >= 0 as i32 {
        let mut leak_slope_0: f32 = 2.0f32
            * (tbands[(b + 1 as i32) as usize] - tbands[b as usize]) as f32
            / 4 as i32 as f32;
        leakage_from[b as usize] =
            if leakage_from[(b + 1 as i32) as usize] + leak_slope_0 < leakage_from[b as usize] {
                (leakage_from[(b + 1 as i32) as usize]) + leak_slope_0
            } else {
                leakage_from[b as usize]
            };
        leakage_to[b as usize] =
            if leakage_to[(b + 1 as i32) as usize] - leak_slope_0 > leakage_to[b as usize] {
                (leakage_to[(b + 1 as i32) as usize]) - leak_slope_0
            } else {
                leakage_to[b as usize]
            };
        b -= 1
    }
    b = 0 as i32;
    while b < 18 as i32 + 1 as i32 {
        /* leak_boost[] is made up of two terms. The first, based on leakage_to[],
        represents the boost needed to overcome the amount of analysis leakage
        cause in a weaker band b by louder neighbouring bands.
        The second, based on leakage_from[], applies to a loud band b for
        which the quantization noise causes synthesis leakage to the weaker
        neighbouring bands. */
        let mut boost: f32 =
            (if 0 as i32 as f32 > leakage_to[b as usize] - band_log2[b as usize] {
                0 as i32 as f32
            } else {
                (leakage_to[b as usize]) - band_log2[b as usize]
            }) + (if 0 as i32 as f32 > band_log2[b as usize] - (leakage_from[b as usize] + 2.5f32) {
                0 as i32 as f32
            } else {
                (band_log2[b as usize]) - (leakage_from[b as usize] + 2.5f32)
            });
        (*info).leak_boost[b as usize] =
            if (255 as i32) < crate::stdlib::floor(0.5f64 + (64.0f32 * boost) as f64) as i32 {
                255 as i32
            } else {
                crate::stdlib::floor(0.5f64 + (64.0f32 * boost) as f64) as i32
            } as u8;
        b += 1
    }
    while b < 19 as i32 {
        (*info).leak_boost[b as usize] = 0 as i32 as u8;
        b += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        let mut j: i32 = 0;
        let mut mindist: f32 = 1e15f32;
        j = 0 as i32;
        while j < 8 as i32 {
            let mut k: i32 = 0;
            let mut dist: f32 = 0 as i32 as f32;
            k = 0 as i32;
            while k < 18 as i32 {
                let mut tmp: f32 = 0.;
                tmp = (*tonal).logE[i as usize][k as usize] - (*tonal).logE[j as usize][k as usize];
                dist += tmp * tmp;
                k += 1
            }
            if j != i {
                mindist = if mindist < dist { mindist } else { dist }
            }
            j += 1
        }
        spec_variability += mindist;
        i += 1
    }
    spec_variability =
        crate::stdlib::sqrt((spec_variability / 8 as i32 as f32 / 18 as i32 as f32) as f64) as f32;
    bandwidth_mask = 0 as i32 as f32;
    bandwidth = 0 as i32;
    maxE = 0 as i32 as f32;
    noise_floor = 5.7e-4f32
        / ((1 as i32)
            << (if 0 as i32 > lsb_depth - 8 as i32 {
                0 as i32
            } else {
                (lsb_depth) - 8 as i32
            })) as f32;
    noise_floor *= noise_floor;
    b = 0 as i32;
    while b < 18 as i32 {
        let mut E_1: f32 = 0 as i32 as f32;
        let mut band_start: i32 = 0;
        let mut band_end: i32 = 0;
        /* Keep a margin of 300 Hz for aliasing */
        band_start = tbands[b as usize];
        band_end = tbands[(b + 1 as i32) as usize];
        i = band_start;
        while i < band_end {
            let mut binE_1: f32 = (*out.offset(i as isize)).r * (*out.offset(i as isize)).r
                + (*out.offset((N - i) as isize)).r * (*out.offset((N - i) as isize)).r
                + (*out.offset(i as isize)).i * (*out.offset(i as isize)).i
                + (*out.offset((N - i) as isize)).i * (*out.offset((N - i) as isize)).i;
            E_1 += binE_1;
            i += 1
        }
        E_1 = E_1;
        maxE = if maxE > E_1 { maxE } else { E_1 };
        (*tonal).meanE[b as usize] =
            if (1 as i32 as f32 - alphaE2) * (*tonal).meanE[b as usize] > E_1 {
                (1 as i32 as f32 - alphaE2) * (*tonal).meanE[b as usize]
            } else {
                E_1
            };
        E_1 = if E_1 > (*tonal).meanE[b as usize] {
            E_1
        } else {
            (*tonal).meanE[b as usize]
        };
        /* Use a simple follower with 13 dB/Bark slope for spreading function */
        bandwidth_mask = if 0.05f32 * bandwidth_mask > E_1 {
            (0.05f32) * bandwidth_mask
        } else {
            E_1
        };
        /* Consider the band "active" only if all these conditions are met:
           1) less than 10 dB below the simple follower
           2) less than 90 dB below the peak band (maximal masking possible considering
              both the ATH and the loudness-dependent slope of the spreading function)
           3) above the PCM quantization noise floor
           We use b+1 because the first CELT band isn't included in tbands[]
        */
        if E_1 as f64 > 0.1f64 * bandwidth_mask as f64
            && E_1 * 1e9f32 > maxE
            && E_1 > noise_floor * (band_end - band_start) as f32
        {
            bandwidth = b + 1 as i32
        }
        b += 1
    }
    /* Special case for the last two bands, for which we don't have spectrum but only
    the energy above 12 kHz. */
    if (*tonal).Fs == 48000 as i32 {
        let mut ratio: f32 = 0.;
        let mut E_2: f32 = hp_ener * (1.0f32 / (240 as i32 * 240 as i32) as f32);
        ratio = if (*tonal).prev_bandwidth == 20 as i32 {
            0.03f32
        } else {
            0.07f32
        };
        maxE = if maxE > E_2 { maxE } else { E_2 };
        (*tonal).meanE[b as usize] =
            if (1 as i32 as f32 - alphaE2) * (*tonal).meanE[b as usize] > E_2 {
                (1 as i32 as f32 - alphaE2) * (*tonal).meanE[b as usize]
            } else {
                E_2
            };
        E_2 = if E_2 > (*tonal).meanE[b as usize] {
            E_2
        } else {
            (*tonal).meanE[b as usize]
        };
        /* Use a simple follower with 13 dB/Bark slope for spreading function */
        bandwidth_mask = if 0.05f32 * bandwidth_mask > E_2 {
            (0.05f32) * bandwidth_mask
        } else {
            E_2
        };
        if E_2 > ratio * bandwidth_mask
            && E_2 * 1e9f32 > maxE
            && E_2 > noise_floor * 160 as i32 as f32
        {
            bandwidth = 20 as i32
        }
        /* This detector is unreliable, so if the bandwidth is close to SWB, assume it's FB. */
        if bandwidth >= 17 as i32 {
            bandwidth = 20 as i32
        }
    }
    if (*tonal).count <= 2 as i32 {
        bandwidth = 20 as i32
    }
    frame_loudness = 20 as i32 as f32 * crate::stdlib::log10(frame_loudness as f64) as f32;
    (*tonal).Etracker = if (*tonal).Etracker - 0.003f32 > frame_loudness {
        ((*tonal).Etracker) - 0.003f32
    } else {
        frame_loudness
    };
    (*tonal).lowECount *= 1 as i32 as f32 - alphaE;
    if frame_loudness < (*tonal).Etracker - 30 as i32 as f32 {
        (*tonal).lowECount += alphaE
    }
    i = 0 as i32;
    while i < 8 as i32 {
        let mut sum: f32 = 0 as i32 as f32;
        b = 0 as i32;
        while b < 16 as i32 {
            sum += dct_table[(i * 16 as i32 + b) as usize] * logE[b as usize];
            b += 1
        }
        BFCC[i as usize] = sum;
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        let mut sum_0: f32 = 0 as i32 as f32;
        b = 0 as i32;
        while b < 16 as i32 {
            sum_0 += dct_table[(i * 16 as i32 + b) as usize]
                * 0.5f32
                * ((*tonal).highE[b as usize] + (*tonal).lowE[b as usize]);
            b += 1
        }
        midE[i as usize] = sum_0;
        i += 1
    }
    frame_stationarity /= 18 as i32 as f32;
    relativeE /= 18 as i32 as f32;
    if (*tonal).count < 10 as i32 {
        relativeE = 0.5f32
    }
    frame_noisiness /= 18 as i32 as f32;
    (*info).activity = frame_noisiness + (1 as i32 as f32 - frame_noisiness) * relativeE;
    frame_tonality = max_frame_tonality / (18 as i32 - 9 as i32) as f32;
    frame_tonality = if frame_tonality > (*tonal).prev_tonality * 0.8f32 {
        frame_tonality
    } else {
        ((*tonal).prev_tonality) * 0.8f32
    };
    (*tonal).prev_tonality = frame_tonality;
    slope /= (8 as i32 * 8 as i32) as f32;
    (*info).tonality_slope = slope;
    (*tonal).E_count = ((*tonal).E_count + 1 as i32) % 8 as i32;
    (*tonal).count = if ((*tonal).count + 1 as i32) < 10000 as i32 {
        ((*tonal).count) + 1 as i32
    } else {
        10000 as i32
    };
    (*info).tonality = frame_tonality;
    i = 0 as i32;
    while i < 4 as i32 {
        features[i as usize] = -0.12299f32
            * (BFCC[i as usize] + (*tonal).mem[(i + 24 as i32) as usize])
            + 0.49195f32 * ((*tonal).mem[i as usize] + (*tonal).mem[(i + 16 as i32) as usize])
            + 0.69693f32 * (*tonal).mem[(i + 8 as i32) as usize]
            - 1.4349f32 * (*tonal).cmean[i as usize];
        i += 1
    }
    i = 0 as i32;
    while i < 4 as i32 {
        (*tonal).cmean[i as usize] =
            (1 as i32 as f32 - alpha) * (*tonal).cmean[i as usize] + alpha * BFCC[i as usize];
        i += 1
    }
    i = 0 as i32;
    while i < 4 as i32 {
        features[(4 as i32 + i) as usize] = 0.63246f32
            * (BFCC[i as usize] - (*tonal).mem[(i + 24 as i32) as usize])
            + 0.31623f32 * ((*tonal).mem[i as usize] - (*tonal).mem[(i + 16 as i32) as usize]);
        i += 1
    }
    i = 0 as i32;
    while i < 3 as i32 {
        features[(8 as i32 + i) as usize] = 0.53452f32
            * (BFCC[i as usize] + (*tonal).mem[(i + 24 as i32) as usize])
            - 0.26726f32 * ((*tonal).mem[i as usize] + (*tonal).mem[(i + 16 as i32) as usize])
            - 0.53452f32 * (*tonal).mem[(i + 8 as i32) as usize];
        i += 1
    }
    if (*tonal).count > 5 as i32 {
        i = 0 as i32;
        while i < 9 as i32 {
            (*tonal).std[i as usize] = (1 as i32 as f32 - alpha) * (*tonal).std[i as usize]
                + alpha * features[i as usize] * features[i as usize];
            i += 1
        }
    }
    i = 0 as i32;
    while i < 4 as i32 {
        features[i as usize] = BFCC[i as usize] - midE[i as usize];
        i += 1
    }
    i = 0 as i32;
    while i < 8 as i32 {
        (*tonal).mem[(i + 24 as i32) as usize] = (*tonal).mem[(i + 16 as i32) as usize];
        (*tonal).mem[(i + 16 as i32) as usize] = (*tonal).mem[(i + 8 as i32) as usize];
        (*tonal).mem[(i + 8 as i32) as usize] = (*tonal).mem[i as usize];
        (*tonal).mem[i as usize] = BFCC[i as usize];
        i += 1
    }
    i = 0 as i32;
    while i < 9 as i32 {
        features[(11 as i32 + i) as usize] = crate::stdlib::sqrt((*tonal).std[i as usize] as f64)
            as f32
            - std_feature_bias[i as usize];
        i += 1
    }
    features[18 as i32 as usize] = spec_variability - 0.78f32;
    features[20 as i32 as usize] = (*info).tonality - 0.154723f32;
    features[21 as i32 as usize] = (*info).activity - 0.724643f32;
    features[22 as i32 as usize] = frame_stationarity - 0.743717f32;
    features[23 as i32 as usize] = (*info).tonality_slope + 0.069216f32;
    features[24 as i32 as usize] = (*tonal).lowECount - 0.067930f32;
    mlp_process(
        &net as *const _
            as *const MLP,
        features.as_mut_ptr(),
        frame_probs.as_mut_ptr(),
    );
    frame_probs[0 as i32 as usize] = 0.5f32 * (frame_probs[0 as i32 as usize] + 1 as i32 as f32);
    /* Curve fitting between the MLP probability and the actual probability */
    /*frame_probs[0] = .01f + 1.21f*frame_probs[0]*frame_probs[0] - .23f*(float)pow(frame_probs[0], 10);*/
    /* Probability of active audio (as opposed to silence) */
    frame_probs[1 as i32 as usize] = 0.5f32 * frame_probs[1 as i32 as usize] + 0.5f32;
    frame_probs[1 as i32 as usize] *= frame_probs[1 as i32 as usize];
    /* Probability of speech or music vs noise */
    (*info).activity_probability = frame_probs[1 as i32 as usize];
    /*printf("%f %f\n", frame_probs[0], frame_probs[1]);*/
    /* Probability of state transition */
    let mut tau: f32 = 0.;
    let mut beta: f32 = 0.;
    let mut p0: f32 = 0.;
    let mut p1: f32 = 0.;
    let mut s0: f32 = 0.;
    let mut m0: f32 = 0.;
    let mut psum: f32 = 0.;
    let mut speech0: f32 = 0.;
    let mut music0: f32 = 0.;
    let mut p: f32 = 0.;
    let mut q: f32 = 0.;
    tau = 0.001f32 * (*tonal).music_prob + 0.01f32 * (1 as i32 as f32 - (*tonal).music_prob);
    p = if 0.05f32
        > (if 0.95f32 < frame_probs[1 as i32 as usize] {
            0.95f32
        } else {
            frame_probs[1 as i32 as usize]
        }) {
        0.05f32
    } else if 0.95f32 < frame_probs[1 as i32 as usize] {
        0.95f32
    } else {
        frame_probs[1 as i32 as usize]
    };
    q = if 0.05f32
        > (if 0.95f32 < (*tonal).vad_prob {
            0.95f32
        } else {
            (*tonal).vad_prob
        }) {
        0.05f32
    } else if 0.95f32 < (*tonal).vad_prob {
        0.95f32
    } else {
        (*tonal).vad_prob
    };
    beta = 0.02f32
        + 0.05f32 * crate::stdlib::fabs((p - q) as f64) as f32
            / (p * (1 as i32 as f32 - q) + q * (1 as i32 as f32 - p));
    p0 = (1 as i32 as f32 - (*tonal).vad_prob) * (1 as i32 as f32 - tau) + (*tonal).vad_prob * tau;
    p1 = (*tonal).vad_prob * (1 as i32 as f32 - tau) + (1 as i32 as f32 - (*tonal).vad_prob) * tau;
    p0 *= crate::stdlib::pow(
        (1 as i32 as f32 - frame_probs[1 as i32 as usize]) as f64,
        beta as f64,
    ) as f32;
    p1 *= crate::stdlib::pow(frame_probs[1 as i32 as usize] as f64, beta as f64) as f32;
    (*tonal).vad_prob = p1 / (p0 + p1);
    (*info).vad_prob = (*tonal).vad_prob;
    frame_probs[0 as i32 as usize] = (*tonal).vad_prob * frame_probs[0 as i32 as usize]
        + (1 as i32 as f32 - (*tonal).vad_prob) * 0.5f32;
    tau = 0.0001f32;
    p = if 0.05f32
        > (if 0.95f32 < frame_probs[0 as i32 as usize] {
            0.95f32
        } else {
            frame_probs[0 as i32 as usize]
        }) {
        0.05f32
    } else if 0.95f32 < frame_probs[0 as i32 as usize] {
        0.95f32
    } else {
        frame_probs[0 as i32 as usize]
    };
    q = if 0.05f32
        > (if 0.95f32 < (*tonal).music_prob {
            0.95f32
        } else {
            (*tonal).music_prob
        }) {
        0.05f32
    } else if 0.95f32 < (*tonal).music_prob {
        0.95f32
    } else {
        (*tonal).music_prob
    };
    beta = 0.02f32
        + 0.05f32 * crate::stdlib::fabs((p - q) as f64) as f32
            / (p * (1 as i32 as f32 - q) + q * (1 as i32 as f32 - p));
    p0 = (1 as i32 as f32 - (*tonal).music_prob) * (1 as i32 as f32 - tau)
        + (*tonal).music_prob * tau;
    p1 = (*tonal).music_prob * (1 as i32 as f32 - tau)
        + (1 as i32 as f32 - (*tonal).music_prob) * tau;
    p0 *= crate::stdlib::pow(
        (1 as i32 as f32 - frame_probs[0 as i32 as usize]) as f64,
        beta as f64,
    ) as f32;
    p1 *= crate::stdlib::pow(frame_probs[0 as i32 as usize] as f64, beta as f64) as f32;
    (*tonal).music_prob = p1 / (p0 + p1);
    (*info).music_prob = (*tonal).music_prob;
    psum = 1e-20f32;
    speech0 = crate::stdlib::pow(
        (1 as i32 as f32 - frame_probs[0 as i32 as usize]) as f64,
        beta as f64,
    ) as f32;
    music0 = crate::stdlib::pow(frame_probs[0 as i32 as usize] as f64, beta as f64) as f32;
    if (*tonal).count == 1 as i32 {
        if (*tonal).application == 2048 as i32 {
            (*tonal).pmusic[0 as i32 as usize] = 0.1f32
        } else {
            (*tonal).pmusic[0 as i32 as usize] = 0.625f32
        }
        (*tonal).pspeech[0 as i32 as usize] = 1 as i32 as f32 - (*tonal).pmusic[0 as i32 as usize]
    }
    s0 = (*tonal).pspeech[0 as i32 as usize] + (*tonal).pspeech[1 as i32 as usize];
    m0 = (*tonal).pmusic[0 as i32 as usize] + (*tonal).pmusic[1 as i32 as usize];
    (*tonal).pspeech[0 as i32 as usize] = s0 * (1 as i32 as f32 - tau) * speech0;
    (*tonal).pmusic[0 as i32 as usize] = m0 * (1 as i32 as f32 - tau) * music0;
    i = 1 as i32;
    while i < 100 as i32 - 1 as i32 {
        (*tonal).pspeech[i as usize] = (*tonal).pspeech[(i + 1 as i32) as usize] * speech0;
        (*tonal).pmusic[i as usize] = (*tonal).pmusic[(i + 1 as i32) as usize] * music0;
        i += 1
    }
    (*tonal).pspeech[(100 as i32 - 1 as i32) as usize] = m0 * tau * speech0;
    (*tonal).pmusic[(100 as i32 - 1 as i32) as usize] = s0 * tau * music0;
    i = 0 as i32;
    while i < 100 as i32 {
        psum += (*tonal).pspeech[i as usize] + (*tonal).pmusic[i as usize];
        i += 1
    }
    psum = 1.0f32 / psum;
    i = 0 as i32;
    while i < 100 as i32 {
        (*tonal).pspeech[i as usize] *= psum;
        (*tonal).pmusic[i as usize] *= psum;
        i += 1
    }
    psum = (*tonal).pmusic[0 as i32 as usize];
    i = 1 as i32;
    while i < 100 as i32 {
        psum += (*tonal).pspeech[i as usize];
        i += 1
    }
    if frame_probs[1 as i32 as usize] as f64 > 0.75f64 {
        if (*tonal).music_prob as f64 > 0.9f64 {
            let mut adapt: f32 = 0.;
            (*tonal).music_confidence_count += 1;
            adapt = 1.0f32 / (*tonal).music_confidence_count as f32;
            (*tonal).music_confidence_count = if (*tonal).music_confidence_count < 500 as i32 {
                (*tonal).music_confidence_count
            } else {
                500 as i32
            };
            (*tonal).music_confidence += adapt
                * (if -0.2f32 > frame_probs[0 as i32 as usize] - (*tonal).music_confidence {
                    -0.2f32
                } else {
                    (frame_probs[0 as i32 as usize]) - (*tonal).music_confidence
                })
        }
        if ((*tonal).music_prob as f64) < 0.1f64 {
            let mut adapt_0: f32 = 0.;
            (*tonal).speech_confidence_count += 1;
            adapt_0 = 1.0f32 / (*tonal).speech_confidence_count as f32;
            (*tonal).speech_confidence_count = if (*tonal).speech_confidence_count < 500 as i32 {
                (*tonal).speech_confidence_count
            } else {
                500 as i32
            };
            (*tonal).speech_confidence += adapt_0
                * (if 0.2f32 < frame_probs[0 as i32 as usize] - (*tonal).speech_confidence {
                    0.2f32
                } else {
                    (frame_probs[0 as i32 as usize]) - (*tonal).speech_confidence
                })
        }
    }
    (*tonal).last_music = ((*tonal).music_prob > 0.5f32) as i32;
    (*info).bandwidth = bandwidth;
    (*tonal).prev_bandwidth = bandwidth;
    /* Represents independence of the MLP probabilities, where
    beta=1 means fully independent. */
    /* Denormalized probability of speech (p0) and music (p1) after update */
    /* Probabilities for "all speech" and "all music" */
    /* Probability sum for renormalisation */
    /* Instantaneous probability of speech and music, with beta pre-applied. */
    /* More silence transitions for speech than for music. */
    /* p0 and p1 are the probabilities of speech and music at this frame
    using only information from previous frame and applying the
    state transition model */
    /* We apply the current probability with exponent beta to work around
    the fact that the probability estimates aren't independent. */
    /* Normalise the probabilities to get the Marokv probability of music. */
    /* Consider that silence has a 50-50 probability of being speech or music. */
    /* One transition every 3 minutes of active audio */
    /* Adapt beta based on how "unexpected" the new prob is */
    /* p0 and p1 are the probabilities of speech and music at this frame
    using only information from previous frame and applying the
    state transition model */
    /* We apply the current probability with exponent beta to work around
    the fact that the probability estimates aren't independent. */
    /* Normalise the probabilities to get the Marokv probability of music. */
    /*printf("%f %f %f %f\n", frame_probs[0], frame_probs[1], tonal->music_prob, tonal->vad_prob);*/
    /* This chunk of code deals with delayed decision. */
    /* Instantaneous probability of speech and music, with beta pre-applied. */
    /* Updated probability of having only speech (s0) or only music (m0),
    before considering the new observation. */
    /* Updates s0 and m0 with instantaneous probability. */
    /* Propagate the transition probabilities */
    /* Probability that the latest frame is speech, when all the previous ones were music. */
    /* Probability that the latest frame is music, when all the previous ones were speech. */
    /* Renormalise probabilities to 1 */
    /* Estimate our confidence in the speech/music decisions */
    /*printf("%d %d\n", info->bandwidth, info->opus_bandwidth);*/
    (*info).noisiness = frame_noisiness;
    (*info).valid = 1 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn run_analysis(
    mut analysis: *mut crate::src::opus_1_2_1::src::analysis::TonalityAnalysisState,
    mut celt_mode: *const OpusCustomMode,
    mut analysis_pcm: *const libc::c_void,
    mut analysis_frame_size: i32,
    mut frame_size: i32,
    mut c1: i32,
    mut c2: i32,
    mut C: i32,
    mut Fs: opus_int32,
    mut lsb_depth: i32,
    mut downmix: downmix_func,
    mut analysis_info: *mut AnalysisInfo,
) {
    let mut offset: i32 = 0;
    let mut pcm_len: i32 = 0;
    analysis_frame_size -= analysis_frame_size & 1 as i32;
    if !analysis_pcm.is_null() {
        /* Avoid overflow/wrap-around of the analysis buffer */
        analysis_frame_size = if ((100 as i32 - 5 as i32) * Fs / 50 as i32) < analysis_frame_size {
            ((100 as i32 - 5 as i32) * Fs) / 50 as i32
        } else {
            analysis_frame_size
        };
        pcm_len = analysis_frame_size - (*analysis).analysis_offset;
        offset = (*analysis).analysis_offset;
        while pcm_len > 0 as i32 {
            tonality_analysis(
                analysis,
                celt_mode,
                analysis_pcm,
                if (Fs / 50 as i32) < pcm_len {
                    (Fs) / 50 as i32
                } else {
                    pcm_len
                },
                offset,
                c1,
                c2,
                C,
                lsb_depth,
                downmix,
            );
            offset += Fs / 50 as i32;
            pcm_len -= Fs / 50 as i32
        }
        (*analysis).analysis_offset = analysis_frame_size;
        (*analysis).analysis_offset -= frame_size
    }
    (*analysis_info).valid = 0 as i32;
    tonality_get_info(analysis, analysis_info, frame_size);
}
/* DISABLE_FLOAT_API */

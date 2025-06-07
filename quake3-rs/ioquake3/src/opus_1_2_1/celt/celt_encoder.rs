use ::libc;

pub mod celt_h {

    pub static mut tapset_icdf: [libc::c_uchar; 3] = [
        2 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];

    pub static mut trim_icdf: [libc::c_uchar; 11] = [
        126 as libc::c_int as libc::c_uchar,
        124 as libc::c_int as libc::c_uchar,
        119 as libc::c_int as libc::c_uchar,
        109 as libc::c_int as libc::c_uchar,
        87 as libc::c_int as libc::c_uchar,
        41 as libc::c_int as libc::c_uchar,
        19 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];

    pub static mut spread_icdf: [libc::c_uchar; 4] = [
        25 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];

    /* CELT_H */
}

pub mod entcode_h {
    /*OPT: ec_window must be at least 32 bits, but if you have fast arithmetic on a
    larger type, you can speed up the decoder by using it here.*/

    /*The entropy encoder/decoder context.
    We use the same structure for both, so that common functions like ec_tell()
     can be used on either one.*/

    #[inline]

    pub unsafe extern "C" fn ec_get_error(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> libc::c_int {
        return (*_this).error;
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

    /* Copyright (c) 2002-2008 Jean-Marc Valin
    Copyright (c) 2007-2008 CSIRO
    Copyright (c) 2007-2009 Xiph.Org Foundation
    Written by Jean-Marc Valin */
    /* *
       @file mathops.h
       @brief Various math functions
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
    /* Multiplies two 16-bit fractional values. Bit-exactness of this macro is important */
    /* CELT doesn't need it for fixed-point, by analysis.c does. */
    /* For very small values, we don't care about the answer, so
    we can just return 0. */
    #[inline]

    pub unsafe extern "C" fn celt_maxabs16(
        mut x: *const crate::arch_h::opus_val16,
        mut len: libc::c_int,
    ) -> crate::arch_h::opus_val32 {
        let mut i: libc::c_int = 0;
        let mut maxval: crate::arch_h::opus_val16 = 0 as libc::c_int as crate::arch_h::opus_val16;
        let mut minval: crate::arch_h::opus_val16 = 0 as libc::c_int as crate::arch_h::opus_val16;
        i = 0 as libc::c_int;
        while i < len {
            maxval = if maxval > *x.offset(i as isize) {
                maxval
            } else {
                *x.offset(i as isize)
            };
            minval = if minval < *x.offset(i as isize) {
                minval
            } else {
                *x.offset(i as isize)
            };
            i += 1
        }
        return if maxval > -minval { maxval } else { -minval };
    }
    /* Note: This assumes radix-2 floating point with the exponent at bits 23..30 and an offset of 127
    denorm, +/- inf and NaN are *not* handled */
    /* * Base-2 log approximation (log2(x)). */
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

pub mod pitch_h {
    #[inline]

    pub unsafe extern "C" fn celt_inner_prod_c(
        mut x: *const crate::arch_h::opus_val16,
        mut y: *const crate::arch_h::opus_val16,
        mut N: libc::c_int,
    ) -> crate::arch_h::opus_val32 {
        let mut i: libc::c_int = 0;
        let mut xy: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
        i = 0 as libc::c_int;
        while i < N {
            xy = xy + *x.offset(i as isize) * *y.offset(i as isize);
            i += 1
        }
        return xy;
    }
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::stdarg_h::va_list;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::celt_ener;
pub use crate::arch_h::celt_norm;
pub use crate::arch_h::celt_sig;
pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::celt_h::AnalysisInfo;
pub use crate::celt_h::SILKInfo;
pub use crate::mathops_h::C2RustUnnamed_61;
pub use crate::src::opus_1_2_1::celt::celt::comb_filter;
pub use crate::src::opus_1_2_1::celt::celt::init_caps;
pub use crate::src::opus_1_2_1::celt::celt::resampling_factor;
pub use crate::src::opus_1_2_1::celt::celt::tf_select_table;
pub use crate::src::opus_1_2_1::celt::celt_encoder::celt_h::spread_icdf;
pub use crate::src::opus_1_2_1::celt::celt_encoder::celt_h::tapset_icdf;
pub use crate::src::opus_1_2_1::celt::celt_encoder::celt_h::trim_icdf;
pub use crate::src::opus_1_2_1::celt::celt_encoder::entcode_h::ec_get_error;
pub use crate::src::opus_1_2_1::celt::celt_encoder::entcode_h::ec_tell;
pub use crate::src::opus_1_2_1::celt::celt_encoder::mathops_h::celt_log2;
pub use crate::src::opus_1_2_1::celt::celt_encoder::mathops_h::celt_maxabs16;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_tell_frac;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::mdct::clt_mdct_forward_c;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;

pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;

pub use crate::src::opus_1_2_1::celt::celt_encoder::pitch_h::celt_inner_prod_c;

pub use crate::src::opus_1_2_1::celt::pitch::pitch_downsample;
pub use crate::src::opus_1_2_1::celt::pitch::pitch_search;
pub use crate::src::opus_1_2_1::celt::pitch::remove_doubling;

/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2010 Xiph.Org Foundation
Copyright (c) 2008 Gregory Maxwell
Written by Jean-Marc Valin and Gregory Maxwell */
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
/* * Encoder state
@brief Encoder state
*/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusCustomEncoder {
    pub mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    pub channels: libc::c_int,
    pub stream_channels: libc::c_int,
    pub force_intra: libc::c_int,
    pub clip: libc::c_int,
    pub disable_pf: libc::c_int,
    pub complexity: libc::c_int,
    pub upsample: libc::c_int,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub bitrate: crate::opus_types_h::opus_int32,
    pub vbr: libc::c_int,
    pub signalling: libc::c_int,
    pub constrained_vbr: libc::c_int,
    pub loss_rate: libc::c_int,
    pub lsb_depth: libc::c_int,
    pub lfe: libc::c_int,
    pub disable_inv: libc::c_int,
    pub arch: libc::c_int,
    pub rng: crate::opus_types_h::opus_uint32,
    pub spread_decision: libc::c_int,
    pub delayedIntra: crate::arch_h::opus_val32,
    pub tonal_average: libc::c_int,
    pub lastCodedBands: libc::c_int,
    pub hf_average: libc::c_int,
    pub tapset_decision: libc::c_int,
    pub prefilter_period: libc::c_int,
    pub prefilter_gain: crate::arch_h::opus_val16,
    pub prefilter_tapset: libc::c_int,
    pub consec_transient: libc::c_int,
    pub analysis: crate::celt_h::AnalysisInfo,
    pub silk_info: crate::celt_h::SILKInfo,
    pub preemph_memE: [crate::arch_h::opus_val32; 2],
    pub preemph_memD: [crate::arch_h::opus_val32; 2],
    pub vbr_reservoir: crate::opus_types_h::opus_int32,
    pub vbr_drift: crate::opus_types_h::opus_int32,
    pub vbr_offset: crate::opus_types_h::opus_int32,
    pub vbr_count: crate::opus_types_h::opus_int32,
    pub overlap_max: crate::arch_h::opus_val32,
    pub stereo_saving: crate::arch_h::opus_val16,
    pub intensity: libc::c_int,
    pub energy_mask: *mut crate::arch_h::opus_val16,
    pub spec_avg: crate::arch_h::opus_val16,
    pub in_mem: [crate::arch_h::celt_sig; 1],
}
/* Size = channels*mode->overlap */
/* celt_sig prefilter_mem[],  Size = channels*COMBFILTER_MAXPERIOD */
/* opus_val16 oldBandE[],     Size = channels*mode->nbEBands */
/* opus_val16 oldLogE[],      Size = channels*mode->nbEBands */
/* opus_val16 oldLogE2[],     Size = channels*mode->nbEBands */
/* opus_val16 energyError[],  Size = channels*mode->nbEBands */
#[no_mangle]

pub unsafe extern "C" fn celt_encoder_get_size(mut channels: libc::c_int) -> libc::c_int {
    let mut mode: *mut crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        crate::src::opus_1_2_1::celt::modes::opus_custom_mode_create(
            48000 as libc::c_int,
            960 as libc::c_int,
            0 as *mut libc::c_int,
        ) as *mut crate::src::opus_1_2_1::celt::modes::OpusCustomMode; /* opus_val16 oldBandE[channels*mode->nbEBands]; */
    /* opus_val16 oldLogE[channels*mode->nbEBands]; */
    /* opus_val16 oldLogE2[channels*mode->nbEBands]; */
    /* opus_val16 energyError[channels*mode->nbEBands]; */
    return opus_custom_encoder_get_size(mode, channels);
}
#[inline]

unsafe extern "C" fn opus_custom_encoder_get_size(
    mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut channels: libc::c_int,
) -> libc::c_int {
    let mut size: libc::c_int = (::std::mem::size_of::<OpusCustomEncoder>() as libc::c_ulong)
        .wrapping_add(
            ((channels * (*mode).overlap - 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong),
        )
        .wrapping_add(
            ((channels * 1024 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong),
        )
        .wrapping_add(
            ((4 as libc::c_int * channels * (*mode).nbEBands) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong),
        ) as libc::c_int;
    return size;
}
/* CUSTOM_MODES */

unsafe extern "C" fn opus_custom_encoder_init_arch(
    mut st: *mut OpusCustomEncoder,
    mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut channels: libc::c_int,
    mut arch: libc::c_int,
) -> libc::c_int {
    if channels < 0 as libc::c_int || channels > 2 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if st.is_null() || mode.is_null() {
        return -(7 as libc::c_int);
    }
    crate::stdlib::memset(
        st as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
        (opus_custom_encoder_get_size(mode, channels) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    (*st).mode = mode;
    (*st).channels = channels;
    (*st).stream_channels = (*st).channels;
    (*st).upsample = 1 as libc::c_int;
    (*st).start = 0 as libc::c_int;
    (*st).end = (*(*st).mode).effEBands;
    (*st).signalling = 1 as libc::c_int;
    (*st).arch = arch;
    (*st).constrained_vbr = 1 as libc::c_int;
    (*st).clip = 1 as libc::c_int;
    (*st).bitrate = -(1 as libc::c_int);
    (*st).vbr = 0 as libc::c_int;
    (*st).force_intra = 0 as libc::c_int;
    (*st).complexity = 5 as libc::c_int;
    (*st).lsb_depth = 24 as libc::c_int;
    opus_custom_encoder_ctl(st, 4028 as libc::c_int);
    return 0 as libc::c_int;
}
/* Encoder stuff */
#[no_mangle]

pub unsafe extern "C" fn celt_encoder_init(
    mut st: *mut OpusCustomEncoder,
    mut sampling_rate: crate::opus_types_h::opus_int32,
    mut channels: libc::c_int,
    mut arch: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = opus_custom_encoder_init_arch(
        st,
        crate::src::opus_1_2_1::celt::modes::opus_custom_mode_create(
            48000 as libc::c_int,
            960 as libc::c_int,
            0 as *mut libc::c_int,
        ) as *mut crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        channels,
        arch,
    );
    if ret != 0 as libc::c_int {
        return ret;
    }
    (*st).upsample = crate::src::opus_1_2_1::celt::celt::resampling_factor(sampling_rate);
    return 0 as libc::c_int;
}
/* CUSTOM_MODES */

unsafe extern "C" fn transient_analysis(
    mut in_0: *const crate::arch_h::opus_val32,
    mut len: libc::c_int,
    mut C: libc::c_int,
    mut tf_estimate: *mut crate::arch_h::opus_val16,
    mut tf_chan: *mut libc::c_int,
    mut allow_weak_transients: libc::c_int,
    mut weak_transient: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut mem0: crate::arch_h::opus_val32 = 0.;
    let mut mem1: crate::arch_h::opus_val32 = 0.;
    let mut is_transient: libc::c_int = 0 as libc::c_int;
    let mut mask_metric: crate::opus_types_h::opus_int32 = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut tf_max: crate::arch_h::opus_val16 = 0.;
    let mut len2: libc::c_int = 0;
    /* Forward masking: 6.7 dB/ms. */
    let mut forward_decay: crate::arch_h::opus_val16 = 0.0625f32;
    /* Table of 6*64/x, trained on real data to minimize the average error */
    static mut inv_table: [libc::c_uchar; 128] = [
        255 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
        156 as libc::c_int as libc::c_uchar,
        110 as libc::c_int as libc::c_uchar,
        86 as libc::c_int as libc::c_uchar,
        70 as libc::c_int as libc::c_uchar,
        59 as libc::c_int as libc::c_uchar,
        51 as libc::c_int as libc::c_uchar,
        45 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
        37 as libc::c_int as libc::c_uchar,
        33 as libc::c_int as libc::c_uchar,
        31 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        22 as libc::c_int as libc::c_uchar,
        21 as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
        19 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        17 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
    ];
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul(len as libc::c_ulong) as usize,
    );
    tmp = fresh0.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    *weak_transient = 0 as libc::c_int;
    /* For lower bitrates, let's be more conservative and have a forward masking
    decay of 3.3 dB/ms. This avoids having to code transients at very low
    bitrate (mostly for hybrid), which can result in unstable energy and/or
    partial collapse. */
    if allow_weak_transients != 0 {
        forward_decay = 0.03125f32
    }
    len2 = len / 2 as libc::c_int;
    c = 0 as libc::c_int;
    while c < C {
        let mut mean: crate::arch_h::opus_val32 = 0.;
        let mut unmask: crate::opus_types_h::opus_int32 = 0 as libc::c_int;
        let mut norm: crate::arch_h::opus_val32 = 0.;
        let mut maxE: crate::arch_h::opus_val16 = 0.;
        mem0 = 0 as libc::c_int as crate::arch_h::opus_val32;
        mem1 = 0 as libc::c_int as crate::arch_h::opus_val32;
        /* High-pass filter: (1 - 2*z^-1 + z^-2) / (1 - z^-1 + .5*z^-2) */
        i = 0 as libc::c_int;
        while i < len {
            let mut x: crate::arch_h::opus_val32 = 0.;
            let mut y: crate::arch_h::opus_val32 = 0.;
            x = *in_0.offset((i + c * len) as isize);
            y = mem0 + x;
            mem0 = mem1 + y - 2 as libc::c_int as libc::c_float * x;
            mem1 = x - 0.5f32 * y;
            *tmp.offset(i as isize) = y;
            i += 1
            /*printf("%f ", tmp[i]);*/
        }
        /*printf("\n");*/
        /* First few samples are bad because we don't propagate the memory */
        crate::stdlib::memset(
            tmp as *mut libc::c_void,
            0 as libc::c_int,
            (12 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong),
        );
        mean = 0 as libc::c_int as crate::arch_h::opus_val32;
        mem0 = 0 as libc::c_int as crate::arch_h::opus_val32;
        /* Grouping by two to reduce complexity */
        /* Forward pass to compute the post-echo threshold*/
        i = 0 as libc::c_int;
        while i < len2 {
            let mut x2: crate::arch_h::opus_val16 = *tmp.offset((2 as libc::c_int * i) as isize)
                * *tmp.offset((2 as libc::c_int * i) as isize)
                + *tmp.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize)
                    * *tmp.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
            mean += x2;
            *tmp.offset(i as isize) = mem0 + forward_decay * (x2 - mem0);
            mem0 = *tmp.offset(i as isize);
            i += 1
        }
        mem0 = 0 as libc::c_int as crate::arch_h::opus_val32;
        maxE = 0 as libc::c_int as crate::arch_h::opus_val16;
        /* Backward pass to compute the pre-echo threshold */
        i = len2 - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            /* Backward masking: 13.9 dB/ms. */
            *tmp.offset(i as isize) = mem0 + 0.125f32 * (*tmp.offset(i as isize) - mem0);
            mem0 = *tmp.offset(i as isize);
            maxE = if maxE > mem0 { maxE } else { mem0 };
            i -= 1
        }
        /*for (i=0;i<len2;i++)printf("%f ", tmp[i]/mean);printf("\n");*/
        /* Compute the ratio of the "frame energy" over the harmonic mean of the energy.
        This essentially corresponds to a bitrate-normalized temporal noise-to-mask
        ratio */
        /* As a compromise with the old transient detector, frame energy is the
        geometric mean of the energy and half the max */
        mean =
            crate::stdlib::sqrt((mean * maxE) as libc::c_double * 0.5f64 * len2 as libc::c_double)
                as libc::c_float;
        /* Inverse of the mean energy in Q15+6 */
        norm = len2 as libc::c_float / (1e-15f32 + mean);
        /* Compute harmonic mean discarding the unreliable boundaries
        The data is smooth, so we only take 1/4th of the samples */
        unmask = 0 as libc::c_int; /* Do not round to nearest */
        i = 12 as libc::c_int;
        while i < len2 - 5 as libc::c_int {
            let mut id: libc::c_int = 0;
            id = if 0 as libc::c_int as libc::c_double
                > (if (127 as libc::c_int as libc::c_double)
                    < crate::stdlib::floor(
                        (64 as libc::c_int as libc::c_float
                            * norm
                            * (*tmp.offset(i as isize) + 1e-15f32))
                            as libc::c_double,
                    )
                {
                    127 as libc::c_int as libc::c_double
                } else {
                    crate::stdlib::floor(
                        (64 as libc::c_int as libc::c_float
                            * norm
                            * (*tmp.offset(i as isize) + 1e-15f32))
                            as libc::c_double,
                    )
                }) {
                0 as libc::c_int as libc::c_double
            } else if (127 as libc::c_int as libc::c_double)
                < crate::stdlib::floor(
                    (64 as libc::c_int as libc::c_float
                        * norm
                        * (*tmp.offset(i as isize) + 1e-15f32))
                        as libc::c_double,
                )
            {
                127 as libc::c_int as libc::c_double
            } else {
                crate::stdlib::floor(
                    (64 as libc::c_int as libc::c_float
                        * norm
                        * (*tmp.offset(i as isize) + 1e-15f32))
                        as libc::c_double,
                )
            } as libc::c_int;
            unmask += inv_table[id as usize] as libc::c_int;
            i += 4 as libc::c_int
        }
        /*printf("%d\n", unmask);*/
        /* Normalize, compensate for the 1/4th of the sample and the factor of 6 in the inverse table */
        unmask = 64 as libc::c_int * unmask * 4 as libc::c_int
            / (6 as libc::c_int * (len2 - 17 as libc::c_int));
        if unmask > mask_metric {
            *tf_chan = c;
            mask_metric = unmask
        }
        c += 1
    }
    is_transient = (mask_metric > 200 as libc::c_int) as libc::c_int;
    /* For low bitrates, define "weak transients" that need to be
    handled differently to avoid partial collapse. */
    if allow_weak_transients != 0 && is_transient != 0 && mask_metric < 600 as libc::c_int {
        is_transient = 0 as libc::c_int;
        *weak_transient = 1 as libc::c_int
    }
    /* Arbitrary metric for VBR boost */
    tf_max = if 0 as libc::c_int as libc::c_float
        > crate::stdlib::sqrt((27 as libc::c_int * mask_metric) as libc::c_double) as libc::c_float
            - 42 as libc::c_int as libc::c_float
    {
        0 as libc::c_int as libc::c_float
    } else {
        (crate::stdlib::sqrt((27 as libc::c_int * mask_metric) as libc::c_double) as libc::c_float)
            - 42 as libc::c_int as libc::c_float
    };
    /* *tf_estimate = 1 + MIN16(1, sqrt(MAX16(0, tf_max-30))/20); */
    *tf_estimate = crate::stdlib::sqrt(
        if 0 as libc::c_int as libc::c_double
            > (0.0069f64 as crate::arch_h::opus_val32
                * (if (163 as libc::c_int as libc::c_float) < tf_max {
                    163 as libc::c_int as libc::c_float
                } else {
                    tf_max
                })) as libc::c_double
                - 0.139f64
        {
            0 as libc::c_int as libc::c_double
        } else {
            ((0.0069f64 as crate::arch_h::opus_val32
                * (if (163 as libc::c_int as libc::c_float) < tf_max {
                    163 as libc::c_int as libc::c_float
                } else {
                    tf_max
                })) as libc::c_double)
                - 0.139f64
        },
    ) as libc::c_float;
    /*printf("%d %f\n", tf_max, mask_metric);*/
    /*printf("%d %f %d\n", is_transient, (float)*tf_estimate, tf_max);*/
    return is_transient;
}
/* Looks for sudden increases of energy to decide whether we need to patch
the transient decision */

unsafe extern "C" fn patch_transient_decision(
    mut newE: *mut crate::arch_h::opus_val16,
    mut oldE: *mut crate::arch_h::opus_val16,
    mut nbEBands: libc::c_int,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut C: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut mean_diff: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
    let mut spread_old: [crate::arch_h::opus_val16; 26] = [0.; 26];
    /* Apply an aggressive (-6 dB/Bark) spreading function to the old frame to
    avoid false detection caused by irrelevant bands */
    if C == 1 as libc::c_int {
        spread_old[start as usize] = *oldE.offset(start as isize);
        i = start + 1 as libc::c_int;
        while i < end {
            spread_old[i as usize] = if spread_old[(i - 1 as libc::c_int) as usize] - 1.0f32
                > *oldE.offset(i as isize)
            {
                (spread_old[(i - 1 as libc::c_int) as usize]) - 1.0f32
            } else {
                *oldE.offset(i as isize)
            };
            i += 1
        }
    } else {
        spread_old[start as usize] =
            if *oldE.offset(start as isize) > *oldE.offset((start + nbEBands) as isize) {
                *oldE.offset(start as isize)
            } else {
                *oldE.offset((start + nbEBands) as isize)
            };
        i = start + 1 as libc::c_int;
        while i < end {
            spread_old[i as usize] = if spread_old[(i - 1 as libc::c_int) as usize] - 1.0f32
                > (if *oldE.offset(i as isize) > *oldE.offset((i + nbEBands) as isize) {
                    *oldE.offset(i as isize)
                } else {
                    *oldE.offset((i + nbEBands) as isize)
                }) {
                (spread_old[(i - 1 as libc::c_int) as usize]) - 1.0f32
            } else if *oldE.offset(i as isize) > *oldE.offset((i + nbEBands) as isize) {
                *oldE.offset(i as isize)
            } else {
                *oldE.offset((i + nbEBands) as isize)
            };
            i += 1
        }
    }
    i = end - 2 as libc::c_int;
    while i >= start {
        spread_old[i as usize] =
            if spread_old[i as usize] > spread_old[(i + 1 as libc::c_int) as usize] - 1.0f32 {
                spread_old[i as usize]
            } else {
                (spread_old[(i + 1 as libc::c_int) as usize]) - 1.0f32
            };
        i -= 1
    }
    /* Compute mean increase */
    c = 0 as libc::c_int;
    loop {
        i = if 2 as libc::c_int > start {
            2 as libc::c_int
        } else {
            start
        };
        while i < end - 1 as libc::c_int {
            let mut x1: crate::arch_h::opus_val16 = 0.;
            let mut x2: crate::arch_h::opus_val16 = 0.;
            x1 = if 0 as libc::c_int as libc::c_float > *newE.offset((i + c * nbEBands) as isize) {
                0 as libc::c_int as libc::c_float
            } else {
                *newE.offset((i + c * nbEBands) as isize)
            };
            x2 = if 0 as libc::c_int as libc::c_float > spread_old[i as usize] {
                0 as libc::c_int as libc::c_float
            } else {
                spread_old[i as usize]
            };
            mean_diff = mean_diff
                + (if 0 as libc::c_int as libc::c_float > x1 - x2 {
                    0 as libc::c_int as libc::c_float
                } else {
                    (x1) - x2
                });
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    mean_diff = mean_diff
        / (C * (end
            - 1 as libc::c_int
            - (if 2 as libc::c_int > start {
                2 as libc::c_int
            } else {
                start
            }))) as crate::arch_h::opus_val32;
    /*printf("%f %f %d\n", mean_diff, max_diff, count);*/
    return (mean_diff > 1.0f32) as libc::c_int;
}
/* * Apply window and compute the MDCT for all sub-frames and
all channels in a frame */

unsafe extern "C" fn compute_mdcts(
    mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut shortBlocks: libc::c_int,
    mut in_0: *mut crate::arch_h::celt_sig,
    mut out: *mut crate::arch_h::celt_sig,
    mut C: libc::c_int,
    mut CC: libc::c_int,
    mut LM: libc::c_int,
    mut upsample: libc::c_int,
    mut arch: libc::c_int,
) {
    let overlap: libc::c_int = (*mode).overlap;
    let mut N: libc::c_int = 0;
    let mut B: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if shortBlocks != 0 {
        B = shortBlocks;
        N = (*mode).shortMdctSize;
        shift = (*mode).maxLM
    } else {
        B = 1 as libc::c_int;
        N = (*mode).shortMdctSize << LM;
        shift = (*mode).maxLM - LM
    }
    c = 0 as libc::c_int;
    loop {
        b = 0 as libc::c_int;
        while b < B {
            /* Interleaving the sub-frames while doing the MDCTs */
            crate::src::opus_1_2_1::celt::mdct::clt_mdct_forward_c(
                &(*mode).mdct as *const _ as *const crate::src::opus_1_2_1::celt::mdct::mdct_lookup,
                in_0.offset((c * (B * N + overlap)) as isize)
                    .offset((b * N) as isize),
                &mut *out.offset((b + c * N * B) as isize),
                (*mode).window,
                overlap,
                shift,
                B,
                arch,
            );
            b += 1
        }
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    if CC == 2 as libc::c_int && C == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < B * N {
            *out.offset(i as isize) =
                0.5f32 * *out.offset(i as isize) + 0.5f32 * *out.offset((B * N + i) as isize);
            i += 1
        }
    }
    if upsample != 1 as libc::c_int {
        c = 0 as libc::c_int;
        loop {
            let mut bound: libc::c_int = B * N / upsample;
            i = 0 as libc::c_int;
            while i < bound {
                let ref mut fresh1 = *out.offset((c * B * N + i) as isize);
                *fresh1 *= upsample as libc::c_float;
                i += 1
            }
            crate::stdlib::memset(&mut *out.offset((c * B * N + bound) as isize) as
                       *mut crate::arch_h::celt_sig as *mut libc::c_void, 0 as libc::c_int,
                   ((B * N - bound) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>()
                                                        as libc::c_ulong));
            c += 1;
            if !(c < C) {
                break;
            }
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn celt_preemphasis(
    mut pcmp: *const crate::arch_h::opus_val16,
    mut inp: *mut crate::arch_h::celt_sig,
    mut N: libc::c_int,
    mut CC: libc::c_int,
    mut upsample: libc::c_int,
    mut coef: *const crate::arch_h::opus_val16,
    mut mem: *mut crate::arch_h::celt_sig,
    mut clip: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut coef0: crate::arch_h::opus_val16 = 0.;
    let mut m: crate::arch_h::celt_sig = 0.;
    let mut Nu: libc::c_int = 0;
    coef0 = *coef.offset(0 as libc::c_int as isize);
    m = *mem;
    /* Fast path for the normal 48kHz case and no clipping */
    if *coef.offset(1 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
        && upsample == 1 as libc::c_int
        && clip == 0
    {
        i = 0 as libc::c_int;
        while i < N {
            let mut x: crate::arch_h::opus_val16 = 0.;
            x = *pcmp.offset((CC * i) as isize) * 32768.0f32;
            /* Apply pre-emphasis */
            *inp.offset(i as isize) = x - m;
            m = coef0 * x;
            i += 1
        }
        *mem = m;
        return;
    }
    Nu = N / upsample;
    if upsample != 1 as libc::c_int {
        crate::stdlib::memset(
            inp as *mut libc::c_void,
            0 as libc::c_int,
            (N as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong),
        );
    }
    i = 0 as libc::c_int;
    while i < Nu {
        *inp.offset((i * upsample) as isize) = *pcmp.offset((CC * i) as isize) * 32768.0f32;
        i += 1
    }
    if clip != 0 {
        /* Clip input to avoid encoding non-portable files */
        i = 0 as libc::c_int;
        while i < Nu {
            *inp.offset((i * upsample) as isize) = if -65536.0f32
                > (if 65536.0f32 < *inp.offset((i * upsample) as isize) {
                    65536.0f32
                } else {
                    *inp.offset((i * upsample) as isize)
                }) {
                -65536.0f32
            } else if 65536.0f32 < *inp.offset((i * upsample) as isize) {
                65536.0f32
            } else {
                *inp.offset((i * upsample) as isize)
            };
            i += 1
        }
    }
    i = 0 as libc::c_int;
    while i < N {
        let mut x_0: crate::arch_h::opus_val16 = 0.;
        x_0 = *inp.offset(i as isize);
        /* Apply pre-emphasis */
        *inp.offset(i as isize) = x_0 - m;
        m = coef0 * x_0;
        i += 1
    }
    *mem = m;
}

unsafe extern "C" fn l1_metric(
    mut tmp: *const crate::arch_h::celt_norm,
    mut N: libc::c_int,
    mut LM: libc::c_int,
    mut bias: crate::arch_h::opus_val16,
) -> crate::arch_h::opus_val32 {
    let mut i: libc::c_int = 0;
    let mut L1: crate::arch_h::opus_val32 = 0.;
    L1 = 0 as libc::c_int as crate::arch_h::opus_val32;
    i = 0 as libc::c_int;
    while i < N {
        L1 += crate::stdlib::fabs(*tmp.offset(i as isize) as libc::c_double) as libc::c_float;
        i += 1
    }
    /* When in doubt, prefer good freq resolution */
    L1 = L1 + LM as libc::c_float * bias * L1;
    return L1;
}

unsafe extern "C" fn tf_analysis(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut len: libc::c_int,
    mut isTransient: libc::c_int,
    mut tf_res: *mut libc::c_int,
    mut lambda: libc::c_int,
    mut X: *mut crate::arch_h::celt_norm,
    mut N0: libc::c_int,
    mut LM: libc::c_int,
    mut tf_estimate: crate::arch_h::opus_val16,
    mut tf_chan: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut metric: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cost0: libc::c_int = 0;
    let mut cost1: libc::c_int = 0;
    let mut path0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut path1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut tmp_1: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut sel: libc::c_int = 0;
    let mut selcost: [libc::c_int; 2] = [0; 2];
    let mut tf_select: libc::c_int = 0 as libc::c_int;
    let mut bias: crate::arch_h::opus_val16 = 0.;
    bias = 0.04f32
        * (if -0.25f32 > 0.5f32 - tf_estimate {
            -0.25f32
        } else {
            (0.5f32) - tf_estimate
        });
    /*printf("%f ", bias);*/
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(len as libc::c_ulong)
            as usize,
    );
    metric = fresh2.as_mut_ptr() as *mut libc::c_int;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong).wrapping_mul(
            ((*(*m).eBands.offset(len as isize) as libc::c_int
                - *(*m).eBands.offset((len - 1 as libc::c_int) as isize) as libc::c_int)
                << LM) as libc::c_ulong,
        ) as usize,
    );
    tmp = fresh3.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong).wrapping_mul(
            ((*(*m).eBands.offset(len as isize) as libc::c_int
                - *(*m).eBands.offset((len - 1 as libc::c_int) as isize) as libc::c_int)
                << LM) as libc::c_ulong,
        ) as usize,
    );
    tmp_1 = fresh4.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(len as libc::c_ulong)
            as usize,
    );
    path0 = fresh5.as_mut_ptr() as *mut libc::c_int;
    let mut fresh6 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(len as libc::c_ulong)
            as usize,
    );
    path1 = fresh6.as_mut_ptr() as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        let mut k: libc::c_int = 0;
        let mut N: libc::c_int = 0;
        let mut narrow: libc::c_int = 0;
        let mut L1: crate::arch_h::opus_val32 = 0.;
        let mut best_L1: crate::arch_h::opus_val32 = 0.;
        let mut best_level: libc::c_int = 0 as libc::c_int;
        N = (*(*m).eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
            - *(*m).eBands.offset(i as isize) as libc::c_int)
            << LM;
        /*printf("%d ", metric[i]);*/
        narrow = (*(*m).eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
            - *(*m).eBands.offset(i as isize) as libc::c_int
            == 1 as libc::c_int) as libc::c_int;
        crate::stdlib::memcpy(
            tmp as *mut libc::c_void,
            &mut *X.offset(
                (tf_chan * N0 + ((*(*m).eBands.offset(i as isize) as libc::c_int) << LM)) as isize,
            ) as *mut crate::arch_h::celt_norm as *const libc::c_void,
            (N as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * tmp.wrapping_offset_from(&mut *X.offset(
                            (tf_chan * N0
                                + ((*(*m).eBands.offset(i as isize) as libc::c_int) << LM))
                                as isize,
                        )) as libc::c_long) as libc::c_ulong,
                ),
        );
        L1 = l1_metric(
            tmp,
            N,
            if isTransient != 0 {
                LM
            } else {
                0 as libc::c_int
            },
            bias,
        );
        best_L1 = L1;
        if isTransient != 0 && narrow == 0 {
            crate::stdlib::memcpy(tmp_1 as *mut libc::c_void, tmp as *const libc::c_void,
                   (N as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>()
                                                        as
                                                        libc::c_ulong).wrapping_add((0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_long
                                                                                         *
                                                                                         tmp_1.wrapping_offset_from(tmp)
                                                                                             as
                                                                                             libc::c_long)
                                                                                        as
                                                                                        libc::c_ulong));
            crate::src::opus_1_2_1::celt::bands::haar1(tmp_1, N >> LM, (1 as libc::c_int) << LM);
            L1 = l1_metric(tmp_1, N, LM + 1 as libc::c_int, bias);
            if L1 < best_L1 {
                best_L1 = L1;
                best_level = -(1 as libc::c_int)
            }
        }
        k = 0 as libc::c_int;
        while k < LM + !(isTransient != 0 || narrow != 0) as libc::c_int {
            let mut B: libc::c_int = 0;
            if isTransient != 0 {
                B = LM - k - 1 as libc::c_int
            } else {
                B = k + 1 as libc::c_int
            }
            crate::src::opus_1_2_1::celt::bands::haar1(tmp, N >> k, (1 as libc::c_int) << k);
            L1 = l1_metric(tmp, N, B, bias);
            if L1 < best_L1 {
                best_L1 = L1;
                best_level = k + 1 as libc::c_int
            }
            k += 1
        }
        if isTransient != 0 {
            *metric.offset(i as isize) = 2 as libc::c_int * best_level
        } else {
            *metric.offset(i as isize) = -(2 as libc::c_int) * best_level
        }
        if narrow != 0
            && (*metric.offset(i as isize) == 0 as libc::c_int
                || *metric.offset(i as isize) == -(2 as libc::c_int) * LM)
        {
            *metric.offset(i as isize) -= 1 as libc::c_int
        }
        i += 1
    }
    /* band is too narrow to be split down to LM=-1 */
    /* Just add the right channel if we're in stereo */
    /*if (C==2)
    for (j=0;j<N;j++)
       tmp[j] = ADD16(SHR16(tmp[j], 1),SHR16(X[N0+j+(m->eBands[i]<<LM)], 1));*/
    /* Check the -1 case for transients */
    /*printf ("%f ", L1);*/
    /*printf ("%d ", isTransient ? LM-best_level : best_level);*/
    /* metric is in Q1 to be able to select the mid-point (-0.5) for narrower bands */
    /* For bands that can't be split to -1, set the metric to the half-way point to avoid
    biasing the decision */
    /*printf("\n");*/
    /* Search for the optimal tf resolution, including tf_select */
    tf_select = 0 as libc::c_int;
    sel = 0 as libc::c_int;
    while sel < 2 as libc::c_int {
        cost0 = 0 as libc::c_int;
        cost1 = if isTransient != 0 {
            0 as libc::c_int
        } else {
            lambda
        };
        i = 1 as libc::c_int;
        while i < len {
            let mut curr0: libc::c_int = 0;
            let mut curr1: libc::c_int = 0;
            curr0 = if cost0 < cost1 + lambda {
                cost0
            } else {
                (cost1) + lambda
            };
            curr1 = if cost0 + lambda < cost1 {
                (cost0) + lambda
            } else {
                cost1
            };
            cost0 = curr0
                + ::libc::abs(
                    *metric.offset(i as isize)
                        - 2 as libc::c_int
                            * crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize][(4
                                as libc::c_int
                                * isTransient
                                + 2 as libc::c_int * sel
                                + 0 as libc::c_int)
                                as usize] as libc::c_int,
                );
            cost1 = curr1
                + ::libc::abs(
                    *metric.offset(i as isize)
                        - 2 as libc::c_int
                            * crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize][(4
                                as libc::c_int
                                * isTransient
                                + 2 as libc::c_int * sel
                                + 1 as libc::c_int)
                                as usize] as libc::c_int,
                );
            i += 1
        }
        cost0 = if cost0 < cost1 { cost0 } else { cost1 };
        selcost[sel as usize] = cost0;
        sel += 1
    }
    /* For now, we're conservative and only allow tf_select=1 for transients.
     * If tests confirm it's useful for non-transients, we could allow it. */
    if selcost[1 as libc::c_int as usize] < selcost[0 as libc::c_int as usize] && isTransient != 0 {
        tf_select = 1 as libc::c_int
    }
    cost0 = 0 as libc::c_int;
    cost1 = if isTransient != 0 {
        0 as libc::c_int
    } else {
        lambda
    };
    /* Viterbi forward pass */
    i = 1 as libc::c_int;
    while i < len {
        let mut curr0_0: libc::c_int = 0;
        let mut curr1_0: libc::c_int = 0;
        let mut from0: libc::c_int = 0;
        let mut from1: libc::c_int = 0;
        from0 = cost0;
        from1 = cost1 + lambda;
        if from0 < from1 {
            curr0_0 = from0;
            *path0.offset(i as isize) = 0 as libc::c_int
        } else {
            curr0_0 = from1;
            *path0.offset(i as isize) = 1 as libc::c_int
        }
        from0 = cost0 + lambda;
        from1 = cost1;
        if from0 < from1 {
            curr1_0 = from0;
            *path1.offset(i as isize) = 0 as libc::c_int
        } else {
            curr1_0 = from1;
            *path1.offset(i as isize) = 1 as libc::c_int
        }
        cost0 = curr0_0
            + ::libc::abs(
                *metric.offset(i as isize)
                    - 2 as libc::c_int
                        * crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize][(4
                            as libc::c_int
                            * isTransient
                            + 2 as libc::c_int * tf_select
                            + 0 as libc::c_int)
                            as usize] as libc::c_int,
            );
        cost1 = curr1_0
            + ::libc::abs(
                *metric.offset(i as isize)
                    - 2 as libc::c_int
                        * crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize][(4
                            as libc::c_int
                            * isTransient
                            + 2 as libc::c_int * tf_select
                            + 1 as libc::c_int)
                            as usize] as libc::c_int,
            );
        i += 1
    }
    *tf_res.offset((len - 1 as libc::c_int) as isize) = if cost0 < cost1 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    /* Viterbi backward pass to check the decisions */
    i = len - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        if *tf_res.offset((i + 1 as libc::c_int) as isize) == 1 as libc::c_int {
            *tf_res.offset(i as isize) = *path1.offset((i + 1 as libc::c_int) as isize)
        } else {
            *tf_res.offset(i as isize) = *path0.offset((i + 1 as libc::c_int) as isize)
        }
        i -= 1
    }
    /*printf("%d %f\n", *tf_sum, tf_estimate);*/
    return tf_select;
}

unsafe extern "C" fn tf_encode(
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut isTransient: libc::c_int,
    mut tf_res: *mut libc::c_int,
    mut LM: libc::c_int,
    mut tf_select: libc::c_int,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
) {
    let mut curr: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tf_select_rsv: libc::c_int = 0;
    let mut tf_changed: libc::c_int = 0;
    let mut logp: libc::c_int = 0;
    let mut budget: crate::opus_types_h::opus_uint32 = 0;
    let mut tell: crate::opus_types_h::opus_uint32 = 0;
    budget = (*enc)
        .storage
        .wrapping_mul(8 as libc::c_int as libc::c_uint);
    tell = ec_tell(enc) as crate::opus_types_h::opus_uint32;
    logp = if isTransient != 0 {
        2 as libc::c_int
    } else {
        4 as libc::c_int
    };
    /* Reserve space to code the tf_select decision. */
    tf_select_rsv = (LM > 0 as libc::c_int
        && tell
            .wrapping_add(logp as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            <= budget) as libc::c_int;
    budget = (budget as libc::c_uint).wrapping_sub(tf_select_rsv as libc::c_uint)
        as crate::opus_types_h::opus_uint32 as crate::opus_types_h::opus_uint32;
    tf_changed = 0 as libc::c_int;
    curr = tf_changed;
    i = start;
    while i < end {
        if tell.wrapping_add(logp as libc::c_uint) <= budget {
            crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                *tf_res.offset(i as isize) ^ curr,
                logp as libc::c_uint,
            );
            tell = ec_tell(enc) as crate::opus_types_h::opus_uint32;
            curr = *tf_res.offset(i as isize);
            tf_changed |= curr
        } else {
            *tf_res.offset(i as isize) = curr
        }
        logp = if isTransient != 0 {
            4 as libc::c_int
        } else {
            5 as libc::c_int
        };
        i += 1
    }
    /* Only code tf_select if it would actually make a difference. */
    if tf_select_rsv != 0
        && crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize]
            [(4 as libc::c_int * isTransient + 0 as libc::c_int + tf_changed) as usize]
            as libc::c_int
            != crate::src::opus_1_2_1::celt::celt::tf_select_table[LM as usize]
                [(4 as libc::c_int * isTransient + 2 as libc::c_int + tf_changed) as usize]
                as libc::c_int
    {
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            tf_select,
            1 as libc::c_int as libc::c_uint,
        );
    } else {
        tf_select = 0 as libc::c_int
    }
    i = start;
    while i < end {
        *tf_res.offset(i as isize) = crate::src::opus_1_2_1::celt::celt::tf_select_table
            [LM as usize][(4 as libc::c_int * isTransient
            + 2 as libc::c_int * tf_select
            + *tf_res.offset(i as isize)) as usize]
            as libc::c_int;
        i += 1
    }
    /*for(i=0;i<end;i++)printf("%d ", isTransient ? tf_res[i] : LM+tf_res[i]);printf("\n");*/
}

unsafe extern "C" fn alloc_trim_analysis(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut X: *const crate::arch_h::celt_norm,
    mut bandLogE: *const crate::arch_h::opus_val16,
    mut end: libc::c_int,
    mut LM: libc::c_int,
    mut C: libc::c_int,
    mut N0: libc::c_int,
    mut analysis: *mut crate::celt_h::AnalysisInfo,
    mut stereo_saving: *mut crate::arch_h::opus_val16,
    mut tf_estimate: crate::arch_h::opus_val16,
    mut intensity: libc::c_int,
    mut surround_trim: crate::arch_h::opus_val16,
    mut equiv_rate: crate::opus_types_h::opus_int32,
    mut _arch: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut diff: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
    let mut c: libc::c_int = 0;
    let mut trim_index: libc::c_int = 0;
    let mut trim: crate::arch_h::opus_val16 = 5.0f32;
    let mut logXC: crate::arch_h::opus_val16 = 0.;
    let mut logXC2: crate::arch_h::opus_val16 = 0.;
    /* At low bitrate, reducing the trim seems to help. At higher bitrates, it's less
    clear what's best, so we're keeping it as it was before, at least for now. */
    if equiv_rate < 64000 as libc::c_int {
        trim = 4.0f32
    } else if equiv_rate < 80000 as libc::c_int {
        let mut frac: crate::opus_types_h::opus_int32 =
            equiv_rate - 64000 as libc::c_int >> 10 as libc::c_int; /* Q10 */
        trim = 4.0f32 + 1.0f32 / 16.0f32 * frac as libc::c_float
    } /* Q10 */
    if C == 2 as libc::c_int {
        let mut sum: crate::arch_h::opus_val16 = 0 as libc::c_int as crate::arch_h::opus_val16;
        let mut minXC: crate::arch_h::opus_val16 = 0.;
        /* Compute inter-channel correlation for low frequencies */
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let mut partial: crate::arch_h::opus_val32 = 0.;
            partial = celt_inner_prod_c(
                &*X.offset(((*(*m).eBands.offset(i as isize) as libc::c_int) << LM) as isize),
                &*X.offset(
                    (N0 + ((*(*m).eBands.offset(i as isize) as libc::c_int) << LM)) as isize,
                ),
                (*(*m).eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *(*m).eBands.offset(i as isize) as libc::c_int)
                    << LM,
            );
            sum = sum + partial;
            i += 1
        }
        sum = 1.0f32 / 8 as libc::c_int as libc::c_float * sum;
        sum = if 1.0f32 < crate::stdlib::fabs(sum as libc::c_double) as libc::c_float {
            1.0f32
        } else {
            crate::stdlib::fabs(sum as libc::c_double) as libc::c_float
        };
        minXC = sum;
        i = 8 as libc::c_int;
        while i < intensity {
            let mut partial_0: crate::arch_h::opus_val32 = 0.;
            partial_0 = celt_inner_prod_c(
                &*X.offset(((*(*m).eBands.offset(i as isize) as libc::c_int) << LM) as isize),
                &*X.offset(
                    (N0 + ((*(*m).eBands.offset(i as isize) as libc::c_int) << LM)) as isize,
                ),
                (*(*m).eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *(*m).eBands.offset(i as isize) as libc::c_int)
                    << LM,
            );
            minXC = if minXC < crate::stdlib::fabs(partial_0 as libc::c_double) as libc::c_float {
                minXC
            } else {
                crate::stdlib::fabs(partial_0 as libc::c_double) as libc::c_float
            };
            i += 1
        }
        minXC = if 1.0f32 < crate::stdlib::fabs(minXC as libc::c_double) as libc::c_float {
            1.0f32
        } else {
            crate::stdlib::fabs(minXC as libc::c_double) as libc::c_float
        };
        /*printf ("%f\n", sum);*/
        /* mid-side savings estimations based on the LF average*/
        logXC = celt_log2(1.001f32 - sum * sum);
        /* mid-side savings estimations based on min correlation */
        logXC2 = if 0.5f32 * logXC > celt_log2(1.001f32 - minXC * minXC) {
            (0.5f32) * logXC
        } else {
            celt_log2(1.001f32 - minXC * minXC)
        };
        trim += if -4.0f32 > 0.75f32 * logXC {
            -4.0f32
        } else {
            (0.75f32) * logXC
        };
        *stereo_saving = if *stereo_saving + 0.25f32 < -(0.5f32 * logXC2) {
            (*stereo_saving) + 0.25f32
        } else {
            -(0.5f32 * logXC2)
        }
    }
    /* Estimate spectral tilt */
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < end - 1 as libc::c_int {
            diff += *bandLogE.offset((i + c * (*m).nbEBands) as isize)
                * (2 as libc::c_int + 2 as libc::c_int * i - end) as libc::c_float;
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    diff /= (C * (end - 1 as libc::c_int)) as libc::c_float;
    /*printf("%f\n", diff);*/
    trim -= if -2.0f32
        > (if 2.0f32 < (diff + 1.0f32) / 6 as libc::c_int as libc::c_float {
            2.0f32
        } else {
            (diff + 1.0f32) / 6 as libc::c_int as libc::c_float
        }) {
        -2.0f32
    } else if 2.0f32 < (diff + 1.0f32) / 6 as libc::c_int as libc::c_float {
        2.0f32
    } else {
        (diff + 1.0f32) / 6 as libc::c_int as libc::c_float
    };
    trim -= surround_trim;
    trim -= 2 as libc::c_int as libc::c_float * tf_estimate;
    if (*analysis).valid != 0 {
        trim -= if -2.0f32
            > (if 2.0f32 < 2.0f32 * ((*analysis).tonality_slope + 0.05f32) {
                2.0f32
            } else {
                (2.0f32) * ((*analysis).tonality_slope + 0.05f32)
            }) {
            -2.0f32
        } else if 2.0f32 < 2.0f32 * ((*analysis).tonality_slope + 0.05f32) {
            2.0f32
        } else {
            (2.0f32) * ((*analysis).tonality_slope + 0.05f32)
        }
    }
    trim_index = crate::stdlib::floor((0.5f32 + trim) as libc::c_double) as libc::c_int;
    trim_index = if 0 as libc::c_int
        > (if (10 as libc::c_int) < trim_index {
            10 as libc::c_int
        } else {
            trim_index
        }) {
        0 as libc::c_int
    } else if (10 as libc::c_int) < trim_index {
        10 as libc::c_int
    } else {
        trim_index
    };
    /*printf("%d\n", trim_index);*/
    return trim_index;
}

unsafe extern "C" fn stereo_analysis(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut X: *const crate::arch_h::celt_norm,
    mut LM: libc::c_int,
    mut N0: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut thetas: libc::c_int = 0;
    let mut sumLR: crate::arch_h::opus_val32 = 1e-15f32;
    let mut sumMS: crate::arch_h::opus_val32 = 1e-15f32;
    /* Use the L1 norm to model the entropy of the L/R signal vs the M/S signal */
    i = 0 as libc::c_int;
    while i < 13 as libc::c_int {
        let mut j: libc::c_int = 0;
        j = (*(*m).eBands.offset(i as isize) as libc::c_int) << LM;
        while j < (*(*m).eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int) << LM {
            let mut L: crate::arch_h::opus_val32 = 0.;
            let mut R: crate::arch_h::opus_val32 = 0.;
            let mut M: crate::arch_h::opus_val32 = 0.;
            let mut S: crate::arch_h::opus_val32 = 0.;
            /* We cast to 32-bit first because of the -32768 case */
            L = *X.offset(j as isize);
            R = *X.offset((N0 + j) as isize);
            M = L + R;
            S = L - R;
            sumLR = sumLR
                + (crate::stdlib::fabs(L as libc::c_double) as libc::c_float
                    + crate::stdlib::fabs(R as libc::c_double) as libc::c_float);
            sumMS = sumMS
                + (crate::stdlib::fabs(M as libc::c_double) as libc::c_float
                    + crate::stdlib::fabs(S as libc::c_double) as libc::c_float);
            j += 1
        }
        i += 1
    }
    sumMS = 0.707107f32 * sumMS;
    thetas = 13 as libc::c_int;
    /* We don't need thetas for lower bands with LM<=1 */
    if LM <= 1 as libc::c_int {
        thetas -= 8 as libc::c_int
    }
    return ((((*(*m).eBands.offset(13 as libc::c_int as isize) as libc::c_int)
        << LM + 1 as libc::c_int)
        + thetas) as libc::c_float
        * sumMS
        > ((*(*m).eBands.offset(13 as libc::c_int as isize) as libc::c_int)
            << LM + 1 as libc::c_int) as libc::c_float
            * sumLR) as libc::c_int;
}

unsafe extern "C" fn median_of_5(
    mut x: *const crate::arch_h::opus_val16,
) -> crate::arch_h::opus_val16 {
    let mut t0: crate::arch_h::opus_val16 = 0.;
    let mut t1: crate::arch_h::opus_val16 = 0.;
    let mut t2: crate::arch_h::opus_val16 = 0.;
    let mut t3: crate::arch_h::opus_val16 = 0.;
    let mut t4: crate::arch_h::opus_val16 = 0.;
    t2 = *x.offset(2 as libc::c_int as isize);
    if *x.offset(0 as libc::c_int as isize) > *x.offset(1 as libc::c_int as isize) {
        t0 = *x.offset(1 as libc::c_int as isize);
        t1 = *x.offset(0 as libc::c_int as isize)
    } else {
        t0 = *x.offset(0 as libc::c_int as isize);
        t1 = *x.offset(1 as libc::c_int as isize)
    }
    if *x.offset(3 as libc::c_int as isize) > *x.offset(4 as libc::c_int as isize) {
        t3 = *x.offset(4 as libc::c_int as isize);
        t4 = *x.offset(3 as libc::c_int as isize)
    } else {
        t3 = *x.offset(3 as libc::c_int as isize);
        t4 = *x.offset(4 as libc::c_int as isize)
    }
    if t0 > t3 {
        let mut tmp: crate::arch_h::opus_val16 = t0;
        t0 = t3;
        t3 = tmp;
        let mut tmp_0: crate::arch_h::opus_val16 = t1;
        t1 = t4;
        t4 = tmp_0
    }
    if t2 > t1 {
        if t1 < t3 {
            return if t2 < t3 { t2 } else { t3 };
        } else {
            return if t4 < t1 { t4 } else { t1 };
        }
    } else if t2 < t3 {
        return if t1 < t3 { t1 } else { t3 };
    } else {
        return if t2 < t4 { t2 } else { t4 };
    };
}

unsafe extern "C" fn median_of_3(
    mut x: *const crate::arch_h::opus_val16,
) -> crate::arch_h::opus_val16 {
    let mut t0: crate::arch_h::opus_val16 = 0.;
    let mut t1: crate::arch_h::opus_val16 = 0.;
    let mut t2: crate::arch_h::opus_val16 = 0.;
    if *x.offset(0 as libc::c_int as isize) > *x.offset(1 as libc::c_int as isize) {
        t0 = *x.offset(1 as libc::c_int as isize);
        t1 = *x.offset(0 as libc::c_int as isize)
    } else {
        t0 = *x.offset(0 as libc::c_int as isize);
        t1 = *x.offset(1 as libc::c_int as isize)
    }
    t2 = *x.offset(2 as libc::c_int as isize);
    if t1 < t2 {
        return t1;
    } else if t0 < t2 {
        return t2;
    } else {
        return t0;
    };
}

unsafe extern "C" fn dynalloc_analysis(
    mut bandLogE: *const crate::arch_h::opus_val16,
    mut bandLogE2: *const crate::arch_h::opus_val16,
    mut nbEBands: libc::c_int,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut C: libc::c_int,
    mut offsets: *mut libc::c_int,
    mut lsb_depth: libc::c_int,
    mut logN: *const crate::opus_types_h::opus_int16,
    mut isTransient: libc::c_int,
    mut vbr: libc::c_int,
    mut constrained_vbr: libc::c_int,
    mut eBands: *const crate::opus_types_h::opus_int16,
    mut LM: libc::c_int,
    mut effectiveBytes: libc::c_int,
    mut tot_boost_: *mut crate::opus_types_h::opus_int32,
    mut lfe: libc::c_int,
    mut surround_dynalloc: *mut crate::arch_h::opus_val16,
    mut analysis: *mut crate::celt_h::AnalysisInfo,
) -> crate::arch_h::opus_val16 {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut tot_boost: crate::opus_types_h::opus_int32 = 0 as libc::c_int;
    let mut maxDepth: crate::arch_h::opus_val16 = 0.;
    let mut follower: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut noise_floor: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut fresh7 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul((C * nbEBands) as libc::c_ulong) as usize,
    );
    follower = fresh7.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    let mut fresh8 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul((C * nbEBands) as libc::c_ulong) as usize,
    );
    noise_floor = fresh8.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    crate::stdlib::memset(
        offsets as *mut libc::c_void,
        0 as libc::c_int,
        (nbEBands as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    /* Dynamic allocation code */
    maxDepth = -31.9f32;
    i = 0 as libc::c_int;
    while i < end {
        /* Noise floor must take into account eMeans, the depth, the width of the bands
        and the preemphasis filter (approx. square of bark band ID) */
        *noise_floor.offset(i as isize) = 0.0625f32
            * *logN.offset(i as isize) as crate::arch_h::opus_val32
            + 0.5f32
            + (9 as libc::c_int - lsb_depth) as libc::c_float
            - crate::src::opus_1_2_1::celt::quant_bands::eMeans[i as usize]
            + 0.0062f64 as crate::arch_h::opus_val32
                * ((i + 5 as libc::c_int) * (i + 5 as libc::c_int)) as crate::arch_h::opus_val32;
        i += 1
    }
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < end {
            maxDepth = if maxDepth
                > *bandLogE.offset((c * nbEBands + i) as isize) - *noise_floor.offset(i as isize)
            {
                maxDepth
            } else {
                (*bandLogE.offset((c * nbEBands + i) as isize)) - *noise_floor.offset(i as isize)
            };
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    /* Make sure that dynamic allocation can't make us bust the budget */
    if effectiveBytes > 50 as libc::c_int && LM >= 1 as libc::c_int && lfe == 0 {
        let mut last: libc::c_int = 0 as libc::c_int;
        c = 0 as libc::c_int;
        loop {
            let mut offset: crate::arch_h::opus_val16 = 0.;
            let mut tmp: crate::arch_h::opus_val16 = 0.;
            let mut f: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
            f = &mut *follower.offset((c * nbEBands) as isize) as *mut crate::arch_h::opus_val16;
            *f.offset(0 as libc::c_int as isize) = *bandLogE2.offset((c * nbEBands) as isize);
            i = 1 as libc::c_int;
            while i < end {
                /* The last band to be at least 3 dB higher than the previous one
                is the last we'll consider. Otherwise, we run into problems on
                bandlimited signals. */
                if *bandLogE2.offset((c * nbEBands + i) as isize)
                    > *bandLogE2.offset((c * nbEBands + i - 1 as libc::c_int) as isize) + 0.5f32
                {
                    last = i
                }
                *f.offset(i as isize) = if *f.offset((i - 1 as libc::c_int) as isize) + 1.5f32
                    < *bandLogE2.offset((c * nbEBands + i) as isize)
                {
                    (*f.offset((i - 1 as libc::c_int) as isize)) + 1.5f32
                } else {
                    *bandLogE2.offset((c * nbEBands + i) as isize)
                };
                i += 1
            }
            i = last - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                *f.offset(i as isize) = if *f.offset(i as isize)
                    < (if *f.offset((i + 1 as libc::c_int) as isize) + 2.0f32
                        < *bandLogE2.offset((c * nbEBands + i) as isize)
                    {
                        (*f.offset((i + 1 as libc::c_int) as isize)) + 2.0f32
                    } else {
                        *bandLogE2.offset((c * nbEBands + i) as isize)
                    }) {
                    *f.offset(i as isize)
                } else if *f.offset((i + 1 as libc::c_int) as isize) + 2.0f32
                    < *bandLogE2.offset((c * nbEBands + i) as isize)
                {
                    (*f.offset((i + 1 as libc::c_int) as isize)) + 2.0f32
                } else {
                    *bandLogE2.offset((c * nbEBands + i) as isize)
                };
                i -= 1
            }
            /* Combine with a median filter to avoid dynalloc triggering unnecessarily.
            The "offset" value controls how conservative we are -- a higher offset
            reduces the impact of the median filter and makes dynalloc use more bits. */
            offset = 1.0f32;
            i = 2 as libc::c_int;
            while i < end - 2 as libc::c_int {
                *f.offset(i as isize) = if *f.offset(i as isize)
                    > median_of_5(
                        &*bandLogE2.offset((c * nbEBands + i - 2 as libc::c_int) as isize),
                    ) - offset
                {
                    *f.offset(i as isize)
                } else {
                    (median_of_5(
                        &*bandLogE2.offset((c * nbEBands + i - 2 as libc::c_int) as isize),
                    )) - offset
                };
                i += 1
            }
            tmp = median_of_3(&*bandLogE2.offset((c * nbEBands) as isize)) - offset;
            *f.offset(0 as libc::c_int as isize) = if *f.offset(0 as libc::c_int as isize) > tmp {
                *f.offset(0 as libc::c_int as isize)
            } else {
                tmp
            };
            *f.offset(1 as libc::c_int as isize) = if *f.offset(1 as libc::c_int as isize) > tmp {
                *f.offset(1 as libc::c_int as isize)
            } else {
                tmp
            };
            tmp = median_of_3(&*bandLogE2.offset((c * nbEBands + end - 3 as libc::c_int) as isize))
                - offset;
            *f.offset((end - 2 as libc::c_int) as isize) =
                if *f.offset((end - 2 as libc::c_int) as isize) > tmp {
                    *f.offset((end - 2 as libc::c_int) as isize)
                } else {
                    tmp
                };
            *f.offset((end - 1 as libc::c_int) as isize) =
                if *f.offset((end - 1 as libc::c_int) as isize) > tmp {
                    *f.offset((end - 1 as libc::c_int) as isize)
                } else {
                    tmp
                };
            i = 0 as libc::c_int;
            while i < end {
                *f.offset(i as isize) = if *f.offset(i as isize) > *noise_floor.offset(i as isize) {
                    *f.offset(i as isize)
                } else {
                    *noise_floor.offset(i as isize)
                };
                i += 1
            }
            c += 1;
            if !(c < C) {
                break;
            }
        }
        if C == 2 as libc::c_int {
            i = start;
            while i < end {
                /* Consider 24 dB "cross-talk" */
                *follower.offset((nbEBands + i) as isize) = if *follower
                    .offset((nbEBands + i) as isize)
                    > *follower.offset(i as isize) - 4.0f32
                {
                    *follower.offset((nbEBands + i) as isize)
                } else {
                    (*follower.offset(i as isize)) - 4.0f32
                };
                *follower.offset(i as isize) = if *follower.offset(i as isize)
                    > *follower.offset((nbEBands + i) as isize) - 4.0f32
                {
                    *follower.offset(i as isize)
                } else {
                    (*follower.offset((nbEBands + i) as isize)) - 4.0f32
                };
                *follower.offset(i as isize) = 0.5f32
                    * ((if 0 as libc::c_int as libc::c_float
                        > *bandLogE.offset(i as isize) - *follower.offset(i as isize)
                    {
                        0 as libc::c_int as libc::c_float
                    } else {
                        (*bandLogE.offset(i as isize)) - *follower.offset(i as isize)
                    }) + (if 0 as libc::c_int as libc::c_float
                        > *bandLogE.offset((nbEBands + i) as isize)
                            - *follower.offset((nbEBands + i) as isize)
                    {
                        0 as libc::c_int as libc::c_float
                    } else {
                        (*bandLogE.offset((nbEBands + i) as isize))
                            - *follower.offset((nbEBands + i) as isize)
                    }));
                i += 1
            }
        } else {
            i = start;
            while i < end {
                *follower.offset(i as isize) = if 0 as libc::c_int as libc::c_float
                    > *bandLogE.offset(i as isize) - *follower.offset(i as isize)
                {
                    0 as libc::c_int as libc::c_float
                } else {
                    (*bandLogE.offset(i as isize)) - *follower.offset(i as isize)
                };
                i += 1
            }
        }
        i = start;
        while i < end {
            *follower.offset(i as isize) =
                if *follower.offset(i as isize) > *surround_dynalloc.offset(i as isize) {
                    *follower.offset(i as isize)
                } else {
                    *surround_dynalloc.offset(i as isize)
                };
            i += 1
        }
        /* For non-transient CBR/CVBR frames, halve the dynalloc contribution */
        if (vbr == 0 || constrained_vbr != 0) && isTransient == 0 {
            i = start;
            while i < end {
                *follower.offset(i as isize) = 0.5f32 * *follower.offset(i as isize);
                i += 1
            }
        }
        i = start;
        while i < end {
            if i < 8 as libc::c_int {
                let ref mut fresh9 = *follower.offset(i as isize);
                *fresh9 *= 2 as libc::c_int as libc::c_float
            }
            if i >= 12 as libc::c_int {
                *follower.offset(i as isize) = 0.5f32 * *follower.offset(i as isize)
            }
            i += 1
        }
        if (*analysis).valid != 0 {
            i = start;
            while i
                < (if (19 as libc::c_int) < end {
                    19 as libc::c_int
                } else {
                    end
                })
            {
                *follower.offset(i as isize) = *follower.offset(i as isize)
                    + 1.0f32 / 64.0f32
                        * (*analysis).leak_boost[i as usize] as libc::c_int as libc::c_float;
                i += 1
            }
        }
        i = start;
        while i < end {
            let mut width: libc::c_int = 0;
            let mut boost: libc::c_int = 0;
            let mut boost_bits: libc::c_int = 0;
            *follower.offset(i as isize) =
                if *follower.offset(i as isize) < 4 as libc::c_int as libc::c_float {
                    *follower.offset(i as isize)
                } else {
                    4 as libc::c_int as libc::c_float
                };
            width = (C
                * (*eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *eBands.offset(i as isize) as libc::c_int))
                << LM;
            if width < 6 as libc::c_int {
                boost = *follower.offset(i as isize) as libc::c_int;
                boost_bits = boost * width << 3 as libc::c_int
            } else if width > 48 as libc::c_int {
                boost = (*follower.offset(i as isize) * 8 as libc::c_int as libc::c_float)
                    as libc::c_int;
                boost_bits = (boost * width << 3 as libc::c_int) / 8 as libc::c_int
            } else {
                boost = (*follower.offset(i as isize) * width as libc::c_float
                    / 6 as libc::c_int as libc::c_float) as libc::c_int;
                boost_bits = (boost * 6 as libc::c_int) << 3 as libc::c_int
            }
            /* For CBR and non-transient CVBR frames, limit dynalloc to 2/3 of the bits */
            if (vbr == 0 || constrained_vbr != 0 && isTransient == 0)
                && tot_boost + boost_bits >> 3 as libc::c_int >> 3 as libc::c_int
                    > 2 as libc::c_int * effectiveBytes / 3 as libc::c_int
            {
                let mut cap: crate::opus_types_h::opus_int32 =
                    ((2 as libc::c_int * effectiveBytes / 3 as libc::c_int) << 3 as libc::c_int)
                        << 3 as libc::c_int;
                *offsets.offset(i as isize) = cap - tot_boost;
                tot_boost = cap;
                break;
            } else {
                *offsets.offset(i as isize) = boost;
                tot_boost += boost_bits;
                i += 1
            }
        }
    }
    *tot_boost_ = tot_boost;
    return maxDepth;
}

unsafe extern "C" fn run_prefilter(
    mut st: *mut OpusCustomEncoder,
    mut in_0: *mut crate::arch_h::celt_sig,
    mut prefilter_mem: *mut crate::arch_h::celt_sig,
    mut CC: libc::c_int,
    mut N: libc::c_int,
    mut prefilter_tapset: libc::c_int,
    mut pitch: *mut libc::c_int,
    mut gain: *mut crate::arch_h::opus_val16,
    mut qgain: *mut libc::c_int,
    mut enabled: libc::c_int,
    mut nbAvailableBytes: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut _pre: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    let mut pre: [*mut crate::arch_h::celt_sig; 2] = [0 as *mut crate::arch_h::celt_sig; 2];
    let mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
    let mut pitch_index: libc::c_int = 0;
    let mut gain1: crate::arch_h::opus_val16 = 0.;
    let mut pf_threshold: crate::arch_h::opus_val16 = 0.;
    let mut pf_on: libc::c_int = 0;
    let mut qg: libc::c_int = 0;
    let mut overlap: libc::c_int = 0;
    mode = (*st).mode;
    overlap = (*mode).overlap;
    let mut fresh10 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong)
            .wrapping_mul((CC * (N + 1024 as libc::c_int)) as libc::c_ulong) as usize,
    );
    _pre = fresh10.as_mut_ptr() as *mut crate::arch_h::celt_sig;
    pre[0 as libc::c_int as usize] = _pre;
    pre[1 as libc::c_int as usize] = _pre.offset((N + 1024 as libc::c_int) as isize);
    c = 0 as libc::c_int;
    loop {
        crate::stdlib::memcpy(
            pre[c as usize] as *mut libc::c_void,
            prefilter_mem.offset((c * 1024 as libc::c_int) as isize) as *const libc::c_void,
            (1024 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * pre[c as usize].wrapping_offset_from(
                            prefilter_mem.offset((c * 1024 as libc::c_int) as isize),
                        ) as libc::c_long) as libc::c_ulong,
                ),
        );
        crate::stdlib::memcpy(
            pre[c as usize].offset(1024 as libc::c_int as isize) as *mut libc::c_void,
            in_0.offset((c * (N + overlap)) as isize)
                .offset(overlap as isize) as *const libc::c_void,
            (N as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * pre[c as usize]
                            .offset(1024 as libc::c_int as isize)
                            .wrapping_offset_from(
                                in_0.offset((c * (N + overlap)) as isize)
                                    .offset(overlap as isize),
                            ) as libc::c_long) as libc::c_ulong,
                ),
        );
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    if enabled != 0 {
        let mut pitch_buf: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
        let mut fresh11 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
                .wrapping_mul((1024 as libc::c_int + N >> 1 as libc::c_int) as libc::c_ulong)
                as usize,
        );
        pitch_buf = fresh11.as_mut_ptr() as *mut crate::arch_h::opus_val16;
        crate::src::opus_1_2_1::celt::pitch::pitch_downsample(
            pre.as_mut_ptr() as *mut *mut crate::arch_h::celt_sig,
            pitch_buf,
            1024 as libc::c_int + N,
            CC,
            (*st).arch,
        );
        /* Don't search for the fir last 1.5 octave of the range because
        there's too many false-positives due to short-term correlation */
        crate::src::opus_1_2_1::celt::pitch::pitch_search(
            pitch_buf.offset((1024 as libc::c_int >> 1 as libc::c_int) as isize),
            pitch_buf,
            N,
            1024 as libc::c_int - 3 as libc::c_int * 15 as libc::c_int,
            &mut pitch_index,
            (*st).arch,
        );
        pitch_index = 1024 as libc::c_int - pitch_index;
        gain1 = crate::src::opus_1_2_1::celt::pitch::remove_doubling(
            pitch_buf,
            1024 as libc::c_int,
            15 as libc::c_int,
            N,
            &mut pitch_index,
            (*st).prefilter_period,
            (*st).prefilter_gain,
            (*st).arch,
        );
        if pitch_index > 1024 as libc::c_int - 2 as libc::c_int {
            pitch_index = 1024 as libc::c_int - 2 as libc::c_int
        }
        gain1 = 0.7f32 * gain1;
        /*printf("%d %d %f %f\n", pitch_change, pitch_index, gain1, st->analysis.tonality);*/
        if (*st).loss_rate > 2 as libc::c_int {
            gain1 = 0.5f32 * gain1
        }
        if (*st).loss_rate > 4 as libc::c_int {
            gain1 = 0.5f32 * gain1
        }
        if (*st).loss_rate > 8 as libc::c_int {
            gain1 = 0 as libc::c_int as crate::arch_h::opus_val16
        }
    } else {
        gain1 = 0 as libc::c_int as crate::arch_h::opus_val16;
        pitch_index = 15 as libc::c_int
    }
    /* Gain threshold for enabling the prefilter/postfilter */
    pf_threshold = 0.2f32;
    /* Adjusting the threshold based on rate and continuity */
    if ::libc::abs(pitch_index - (*st).prefilter_period) * 10 as libc::c_int > pitch_index {
        pf_threshold += 0.2f32
    }
    if nbAvailableBytes < 25 as libc::c_int {
        pf_threshold += 0.1f32
    }
    if nbAvailableBytes < 35 as libc::c_int {
        pf_threshold += 0.1f32
    }
    if (*st).prefilter_gain > 0.4f32 {
        pf_threshold -= 0.1f32
    }
    if (*st).prefilter_gain > 0.55f32 {
        pf_threshold -= 0.1f32
    }
    /* Hard threshold at 0.2 */
    pf_threshold = if pf_threshold > 0.2f32 {
        pf_threshold
    } else {
        0.2f32
    };
    if gain1 < pf_threshold {
        gain1 = 0 as libc::c_int as crate::arch_h::opus_val16;
        pf_on = 0 as libc::c_int;
        qg = 0 as libc::c_int
    } else {
        /*This block is not gated by a total bits check only because
        of the nbAvailableBytes check above.*/
        if (crate::stdlib::fabs((gain1 - (*st).prefilter_gain) as libc::c_double) as libc::c_float)
            < 0.1f32
        {
            gain1 = (*st).prefilter_gain
        }
        qg = crate::stdlib::floor(
            (0.5f32
                + gain1 * 32 as libc::c_int as libc::c_float / 3 as libc::c_int as libc::c_float)
                as libc::c_double,
        ) as libc::c_int
            - 1 as libc::c_int;
        qg = if 0 as libc::c_int
            > (if (7 as libc::c_int) < qg {
                7 as libc::c_int
            } else {
                qg
            }) {
            0 as libc::c_int
        } else if (7 as libc::c_int) < qg {
            7 as libc::c_int
        } else {
            qg
        };
        gain1 = 0.09375f32 * (qg + 1 as libc::c_int) as libc::c_float;
        pf_on = 1 as libc::c_int
    }
    /*printf("%d %f\n", pitch_index, gain1);*/
    c = 0 as libc::c_int;
    loop {
        let mut offset: libc::c_int = (*mode).shortMdctSize - overlap;
        (*st).prefilter_period = if (*st).prefilter_period > 15 as libc::c_int {
            (*st).prefilter_period
        } else {
            15 as libc::c_int
        };
        crate::stdlib::memcpy(
            in_0.offset((c * (N + overlap)) as isize) as *mut libc::c_void,
            (*st).in_mem.as_mut_ptr().offset((c * overlap) as isize) as *const libc::c_void,
            (overlap as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * in_0
                            .offset((c * (N + overlap)) as isize)
                            .wrapping_offset_from(
                                (*st).in_mem.as_mut_ptr().offset((c * overlap) as isize),
                            ) as libc::c_long) as libc::c_ulong,
                ),
        );
        if offset != 0 {
            crate::src::opus_1_2_1::celt::celt::comb_filter(
                in_0.offset((c * (N + overlap)) as isize)
                    .offset(overlap as isize),
                pre[c as usize].offset(1024 as libc::c_int as isize),
                (*st).prefilter_period,
                (*st).prefilter_period,
                offset,
                -(*st).prefilter_gain,
                -(*st).prefilter_gain,
                (*st).prefilter_tapset,
                (*st).prefilter_tapset,
                0 as *const crate::arch_h::opus_val16,
                0 as libc::c_int,
                (*st).arch,
            );
        }
        crate::src::opus_1_2_1::celt::celt::comb_filter(
            in_0.offset((c * (N + overlap)) as isize)
                .offset(overlap as isize)
                .offset(offset as isize),
            pre[c as usize]
                .offset(1024 as libc::c_int as isize)
                .offset(offset as isize),
            (*st).prefilter_period,
            pitch_index,
            N - offset,
            -(*st).prefilter_gain,
            -gain1,
            (*st).prefilter_tapset,
            prefilter_tapset,
            (*mode).window,
            overlap,
            (*st).arch,
        );
        crate::stdlib::memcpy(
            (*st).in_mem.as_mut_ptr().offset((c * overlap) as isize) as *mut libc::c_void,
            in_0.offset((c * (N + overlap)) as isize).offset(N as isize) as *const libc::c_void,
            (overlap as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * (*st)
                            .in_mem
                            .as_mut_ptr()
                            .offset((c * overlap) as isize)
                            .wrapping_offset_from(
                                in_0.offset((c * (N + overlap)) as isize).offset(N as isize),
                            ) as libc::c_long) as libc::c_ulong,
                ),
        );
        if N > 1024 as libc::c_int {
            crate::stdlib::memcpy(
                prefilter_mem.offset((c * 1024 as libc::c_int) as isize) as *mut libc::c_void,
                pre[c as usize].offset(N as isize) as *const libc::c_void,
                (1024 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * prefilter_mem
                                .offset((c * 1024 as libc::c_int) as isize)
                                .wrapping_offset_from(pre[c as usize].offset(N as isize))
                                as libc::c_long) as libc::c_ulong,
                    ),
            );
        } else {
            crate::stdlib::memmove(
                prefilter_mem.offset((c * 1024 as libc::c_int) as isize) as *mut libc::c_void,
                prefilter_mem
                    .offset((c * 1024 as libc::c_int) as isize)
                    .offset(N as isize) as *const libc::c_void,
                ((1024 as libc::c_int - N) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * prefilter_mem
                                .offset((c * 1024 as libc::c_int) as isize)
                                .wrapping_offset_from(
                                    prefilter_mem
                                        .offset((c * 1024 as libc::c_int) as isize)
                                        .offset(N as isize),
                                ) as libc::c_long) as libc::c_ulong,
                    ),
            );
            crate::stdlib::memcpy(
                prefilter_mem
                    .offset((c * 1024 as libc::c_int) as isize)
                    .offset(1024 as libc::c_int as isize)
                    .offset(-(N as isize)) as *mut libc::c_void,
                pre[c as usize].offset(1024 as libc::c_int as isize) as *const libc::c_void,
                (N as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * prefilter_mem
                                .offset((c * 1024 as libc::c_int) as isize)
                                .offset(1024 as libc::c_int as isize)
                                .offset(-(N as isize))
                                .wrapping_offset_from(
                                    pre[c as usize].offset(1024 as libc::c_int as isize),
                                ) as libc::c_long) as libc::c_ulong,
                    ),
            );
        }
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    *gain = gain1;
    *pitch = pitch_index;
    *qgain = qg;
    return pf_on;
}

unsafe extern "C" fn compute_vbr(
    mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut analysis: *mut crate::celt_h::AnalysisInfo,
    mut base_target: crate::opus_types_h::opus_int32,
    mut LM: libc::c_int,
    mut bitrate: crate::opus_types_h::opus_int32,
    mut lastCodedBands: libc::c_int,
    mut C: libc::c_int,
    mut intensity: libc::c_int,
    mut constrained_vbr: libc::c_int,
    mut stereo_saving: crate::arch_h::opus_val16,
    mut tot_boost: libc::c_int,
    mut tf_estimate: crate::arch_h::opus_val16,
    mut pitch_change: libc::c_int,
    mut maxDepth: crate::arch_h::opus_val16,
    mut lfe: libc::c_int,
    mut has_surround_mask: libc::c_int,
    mut surround_masking: crate::arch_h::opus_val16,
    mut temporal_vbr: crate::arch_h::opus_val16,
) -> libc::c_int {
    /* The target rate in 8th bits per frame */
    let mut target: crate::opus_types_h::opus_int32 = 0;
    let mut coded_bins: libc::c_int = 0;
    let mut coded_bands: libc::c_int = 0;
    let mut tf_calibration: crate::arch_h::opus_val16 = 0.;
    let mut nbEBands: libc::c_int = 0;
    let mut eBands: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    nbEBands = (*mode).nbEBands;
    eBands = (*mode).eBands;
    coded_bands = if lastCodedBands != 0 {
        lastCodedBands
    } else {
        nbEBands
    };
    coded_bins = (*eBands.offset(coded_bands as isize) as libc::c_int) << LM;
    if C == 2 as libc::c_int {
        coded_bins += (*eBands.offset(
            (if intensity < coded_bands {
                intensity
            } else {
                coded_bands
            }) as isize,
        ) as libc::c_int)
            << LM
    }
    target = base_target;
    /*printf("%f %f %f %f %d %d ", st->analysis.activity, st->analysis.tonality, tf_estimate, st->stereo_saving, tot_boost, coded_bands);*/
    if (*analysis).valid != 0 && ((*analysis).activity as libc::c_double) < 0.4f64 {
        target -= ((coded_bins << 3 as libc::c_int) as libc::c_float
            * (0.4f32 - (*analysis).activity)) as crate::opus_types_h::opus_int32
    }
    /* Stereo savings */
    if C == 2 as libc::c_int {
        let mut coded_stereo_bands: libc::c_int = 0;
        let mut coded_stereo_dof: libc::c_int = 0;
        let mut max_frac: crate::arch_h::opus_val16 = 0.;
        coded_stereo_bands = if intensity < coded_bands {
            intensity
        } else {
            coded_bands
        };
        coded_stereo_dof = ((*eBands.offset(coded_stereo_bands as isize) as libc::c_int) << LM)
            - coded_stereo_bands;
        /* Maximum fraction of the bits we can save if the signal is mono. */
        max_frac = 0.8f32 * coded_stereo_dof as crate::arch_h::opus_val32
            / coded_bins as crate::arch_h::opus_val16;
        stereo_saving = if stereo_saving < 1.0f32 {
            stereo_saving
        } else {
            1.0f32
        };
        /*printf("%d %d %d ", coded_stereo_dof, coded_bins, tot_boost);*/
        target -= if (max_frac * target as libc::c_float)
            < (stereo_saving - 0.1f32)
                * (coded_stereo_dof << 3 as libc::c_int) as crate::arch_h::opus_val32
        {
            (max_frac) * target as libc::c_float
        } else {
            (stereo_saving - 0.1f32)
                * (coded_stereo_dof << 3 as libc::c_int) as crate::arch_h::opus_val32
        } as crate::opus_types_h::opus_int32
    }
    /* Boost the rate according to dynalloc (minus the dynalloc average for calibration). */
    target += tot_boost - ((19 as libc::c_int) << LM);
    /* Apply transient boost, compensating for average boost. */
    tf_calibration = 0.044f32;
    target += ((tf_estimate - tf_calibration) * target as libc::c_float)
        as crate::opus_types_h::opus_int32;
    /* Apply tonality boost */
    if (*analysis).valid != 0 && lfe == 0 {
        let mut tonal_target: crate::opus_types_h::opus_int32 = 0;
        let mut tonal: libc::c_float = 0.;
        /* Tonality boost (compensating for the average). */
        tonal = (if 0.0f32 > (*analysis).tonality - 0.15f32 {
            0.0f32
        } else {
            ((*analysis).tonality) - 0.15f32
        }) - 0.12f32;
        tonal_target = target
            + ((coded_bins << 3 as libc::c_int) as libc::c_float * 1.2f32 * tonal)
                as crate::opus_types_h::opus_int32;
        if pitch_change != 0 {
            tonal_target += ((coded_bins << 3 as libc::c_int) as libc::c_float * 0.8f32)
                as crate::opus_types_h::opus_int32
        }
        /*printf("%f %f ", analysis->tonality, tonal);*/
        target = tonal_target
    }
    if has_surround_mask != 0 && lfe == 0 {
        let mut surround_target: crate::opus_types_h::opus_int32 = target
            + (surround_masking * (coded_bins << 3 as libc::c_int) as crate::arch_h::opus_val32)
                as crate::opus_types_h::opus_int32;
        /*printf("%f %d %d %d %d %d %d ", surround_masking, coded_bins, st->end, st->intensity, surround_target, target, st->bitrate);*/
        target = if target / 4 as libc::c_int > surround_target {
            (target) / 4 as libc::c_int
        } else {
            surround_target
        }
    }
    let mut floor_depth: crate::opus_types_h::opus_int32 = 0;
    let mut bins: libc::c_int = 0;
    bins = (*eBands.offset((nbEBands - 2 as libc::c_int) as isize) as libc::c_int) << LM;
    /*printf("%f %d\n", maxDepth, floor_depth);*/
    floor_depth = ((C * bins << 3 as libc::c_int) as crate::arch_h::opus_val32 * maxDepth)
        as crate::opus_types_h::opus_int32;
    floor_depth = if floor_depth > target >> 2 as libc::c_int {
        floor_depth
    } else {
        (target) >> 2 as libc::c_int
    };
    target = if target < floor_depth {
        target
    } else {
        floor_depth
    };
    /*floor_depth = SHR32(MULT16_16((C*bins<<BITRES),celt_log2(SHL32(MAX16(1,sample_max),13))), DB_SHIFT);*/
    /* Make VBR less aggressive for constrained VBR because we can't keep a higher bitrate
    for long. Needs tuning. */
    if (has_surround_mask == 0 || lfe != 0) && constrained_vbr != 0 {
        target = base_target
            + (0.67f32 * (target - base_target) as libc::c_float) as crate::opus_types_h::opus_int32
    }
    if has_surround_mask == 0 && tf_estimate < 0.2f32 {
        let mut amount: crate::arch_h::opus_val16 = 0.;
        let mut tvbr_factor: crate::arch_h::opus_val16 = 0.;
        amount = 0.0000031f32
            * (if 0 as libc::c_int
                > (if (32000 as libc::c_int) < 96000 as libc::c_int - bitrate {
                    32000 as libc::c_int
                } else {
                    (96000 as libc::c_int) - bitrate
                })
            {
                0 as libc::c_int
            } else {
                (if (32000 as libc::c_int) < 96000 as libc::c_int - bitrate {
                    32000 as libc::c_int
                } else {
                    (96000 as libc::c_int) - bitrate
                })
            }) as libc::c_float;
        tvbr_factor = temporal_vbr * amount;
        target += (tvbr_factor * target as libc::c_float) as crate::opus_types_h::opus_int32
    }
    /* Don't allow more than doubling the rate */
    target = if 2 as libc::c_int * base_target < target {
        (2 as libc::c_int) * base_target
    } else {
        target
    };
    return target;
}
#[no_mangle]

pub unsafe extern "C" fn celt_encode_with_ec(
    mut st: *mut OpusCustomEncoder,
    mut pcm: *const crate::arch_h::opus_val16,
    mut frame_size: libc::c_int,
    mut compressed: *mut libc::c_uchar,
    mut nbCompressedBytes: libc::c_int,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut bits: crate::opus_types_h::opus_int32 = 0;
    let mut _enc: crate::src::opus_1_2_1::celt::entcode::ec_enc =
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
    let mut in_0: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    let mut freq: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    let mut X: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut bandE: *mut crate::arch_h::celt_ener = 0 as *mut crate::arch_h::celt_ener;
    let mut bandLogE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut bandLogE2: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut fine_quant: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut error: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut pulses: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut offsets: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut fine_priority: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tf_res: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut collapse_masks: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut prefilter_mem: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    let mut oldBandE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut oldLogE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut oldLogE2: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut energyError: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut shortBlocks: libc::c_int = 0 as libc::c_int;
    let mut isTransient: libc::c_int = 0 as libc::c_int;
    let CC: libc::c_int = (*st).channels;
    let C: libc::c_int = (*st).stream_channels;
    let mut LM: libc::c_int = 0;
    let mut M: libc::c_int = 0;
    let mut tf_select: libc::c_int = 0;
    let mut nbFilledBytes: libc::c_int = 0;
    let mut nbAvailableBytes: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut effEnd: libc::c_int = 0;
    let mut codedBands: libc::c_int = 0;
    let mut alloc_trim: libc::c_int = 0;
    let mut pitch_index: libc::c_int = 15 as libc::c_int;
    let mut gain1: crate::arch_h::opus_val16 = 0 as libc::c_int as crate::arch_h::opus_val16;
    let mut dual_stereo: libc::c_int = 0 as libc::c_int;
    let mut effectiveBytes: libc::c_int = 0;
    let mut dynalloc_logp: libc::c_int = 0;
    let mut vbr_rate: crate::opus_types_h::opus_int32 = 0;
    let mut total_bits: crate::opus_types_h::opus_int32 = 0;
    let mut total_boost: crate::opus_types_h::opus_int32 = 0;
    let mut balance: crate::opus_types_h::opus_int32 = 0;
    let mut tell: crate::opus_types_h::opus_int32 = 0;
    let mut tell0_frac: crate::opus_types_h::opus_int32 = 0;
    let mut prefilter_tapset: libc::c_int = 0 as libc::c_int;
    let mut pf_on: libc::c_int = 0;
    let mut anti_collapse_rsv: libc::c_int = 0;
    let mut anti_collapse_on: libc::c_int = 0 as libc::c_int;
    let mut silence: libc::c_int = 0 as libc::c_int;
    let mut tf_chan: libc::c_int = 0 as libc::c_int;
    let mut tf_estimate: crate::arch_h::opus_val16 = 0.;
    let mut pitch_change: libc::c_int = 0 as libc::c_int;
    let mut tot_boost: crate::opus_types_h::opus_int32 = 0;
    let mut sample_max: crate::arch_h::opus_val32 = 0.;
    let mut maxDepth: crate::arch_h::opus_val16 = 0.;
    let mut mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
    let mut nbEBands: libc::c_int = 0;
    let mut overlap: libc::c_int = 0;
    let mut eBands: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    let mut secondMdct: libc::c_int = 0;
    let mut signalBandwidth: libc::c_int = 0;
    let mut transient_got_disabled: libc::c_int = 0 as libc::c_int;
    let mut surround_masking: crate::arch_h::opus_val16 =
        0 as libc::c_int as crate::arch_h::opus_val16;
    let mut temporal_vbr: crate::arch_h::opus_val16 = 0 as libc::c_int as crate::arch_h::opus_val16;
    let mut surround_trim: crate::arch_h::opus_val16 =
        0 as libc::c_int as crate::arch_h::opus_val16;
    let mut equiv_rate: crate::opus_types_h::opus_int32 = 0;
    let mut hybrid: libc::c_int = 0;
    let mut weak_transient: libc::c_int = 0 as libc::c_int;
    let mut surround_dynalloc: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    mode = (*st).mode;
    nbEBands = (*mode).nbEBands;
    overlap = (*mode).overlap;
    eBands = (*mode).eBands;
    start = (*st).start;
    end = (*st).end;
    hybrid = (start != 0 as libc::c_int) as libc::c_int;
    tf_estimate = 0 as libc::c_int as crate::arch_h::opus_val16;
    if nbCompressedBytes < 2 as libc::c_int || pcm.is_null() {
        return -(1 as libc::c_int);
    }
    frame_size *= (*st).upsample;
    LM = 0 as libc::c_int;
    while LM <= (*mode).maxLM {
        if (*mode).shortMdctSize << LM == frame_size {
            break;
        }
        LM += 1
    }
    if LM > (*mode).maxLM {
        return -(1 as libc::c_int);
    }
    M = (1 as libc::c_int) << LM;
    N = M * (*mode).shortMdctSize;
    prefilter_mem = (*st).in_mem.as_mut_ptr().offset((CC * overlap) as isize);
    oldBandE = (*st)
        .in_mem
        .as_mut_ptr()
        .offset((CC * (overlap + 1024 as libc::c_int)) as isize)
        as *mut crate::arch_h::opus_val16;
    oldLogE = oldBandE.offset((CC * nbEBands) as isize);
    oldLogE2 = oldLogE.offset((CC * nbEBands) as isize);
    energyError = oldLogE2.offset((CC * nbEBands) as isize);
    if enc.is_null() {
        tell = 1 as libc::c_int;
        tell0_frac = tell;
        nbFilledBytes = 0 as libc::c_int
    } else {
        tell = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        ) as crate::opus_types_h::opus_int32;
        tell0_frac = tell;
        tell = ec_tell(enc);
        nbFilledBytes = tell + 4 as libc::c_int >> 3 as libc::c_int
    }
    /* Can't produce more than 1275 output bytes */
    nbCompressedBytes = if nbCompressedBytes < 1275 as libc::c_int {
        nbCompressedBytes
    } else {
        1275 as libc::c_int
    };
    nbAvailableBytes = nbCompressedBytes - nbFilledBytes;
    if (*st).vbr != 0 && (*st).bitrate != -(1 as libc::c_int) {
        let mut den: crate::opus_types_h::opus_int32 = (*mode).Fs >> 3 as libc::c_int;
        vbr_rate = ((*st).bitrate * frame_size + (den >> 1 as libc::c_int)) / den;
        effectiveBytes = vbr_rate >> 3 as libc::c_int + 3 as libc::c_int
    } else {
        let mut tmp: crate::opus_types_h::opus_int32 = 0;
        vbr_rate = 0 as libc::c_int;
        tmp = (*st).bitrate * frame_size;
        if tell > 1 as libc::c_int {
            tmp += tell
        }
        if (*st).bitrate != -(1 as libc::c_int) {
            nbCompressedBytes = if 2 as libc::c_int
                > (if nbCompressedBytes
                    < (tmp + 4 as libc::c_int * (*mode).Fs) / (8 as libc::c_int * (*mode).Fs)
                        - ((*st).signalling != 0) as libc::c_int
                {
                    nbCompressedBytes
                } else {
                    ((tmp + 4 as libc::c_int * (*mode).Fs) / (8 as libc::c_int * (*mode).Fs))
                        - ((*st).signalling != 0) as libc::c_int
                }) {
                2 as libc::c_int
            } else if nbCompressedBytes
                < (tmp + 4 as libc::c_int * (*mode).Fs) / (8 as libc::c_int * (*mode).Fs)
                    - ((*st).signalling != 0) as libc::c_int
            {
                nbCompressedBytes
            } else {
                ((tmp + 4 as libc::c_int * (*mode).Fs) / (8 as libc::c_int * (*mode).Fs))
                    - ((*st).signalling != 0) as libc::c_int
            }
        }
        effectiveBytes = nbCompressedBytes - nbFilledBytes
    }
    equiv_rate = (nbCompressedBytes * 8 as libc::c_int * 50 as libc::c_int
        >> 3 as libc::c_int - LM)
        - (40 as libc::c_int * C + 20 as libc::c_int)
            * ((400 as libc::c_int >> LM) - 50 as libc::c_int);
    if (*st).bitrate != -(1 as libc::c_int) {
        equiv_rate = if equiv_rate
            < (*st).bitrate
                - (40 as libc::c_int * C + 20 as libc::c_int)
                    * ((400 as libc::c_int >> LM) - 50 as libc::c_int)
        {
            equiv_rate
        } else {
            ((*st).bitrate)
                - (40 as libc::c_int * C + 20 as libc::c_int)
                    * ((400 as libc::c_int >> LM) - 50 as libc::c_int)
        }
    }
    if enc.is_null() {
        crate::src::opus_1_2_1::celt::entenc::ec_enc_init(
            &mut _enc as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            compressed,
            nbCompressedBytes as crate::opus_types_h::opus_uint32,
        );
        enc = &mut _enc
    }
    if vbr_rate > 0 as libc::c_int {
        /* Computes the max bit-rate allowed in VBR mode to avoid violating the
         target rate and buffering.
        We must do this up front so that bust-prevention logic triggers
         correctly if we don't have enough bits. */
        if (*st).constrained_vbr != 0 {
            let mut vbr_bound: crate::opus_types_h::opus_int32 = 0;
            let mut max_allowed: crate::opus_types_h::opus_int32 = 0;
            /* We could use any multiple of vbr_rate as bound (depending on the
             delay).
            This is clamped to ensure we use at least two bytes if the encoder
             was entirely empty, but to allow 0 in hybrid mode. */
            vbr_bound = vbr_rate;
            max_allowed = if (if (if tell == 1 as libc::c_int {
                2 as libc::c_int
            } else {
                0 as libc::c_int
            }) > vbr_rate + vbr_bound - (*st).vbr_reservoir
                >> 3 as libc::c_int + 3 as libc::c_int
            {
                (if tell == 1 as libc::c_int {
                    2 as libc::c_int
                } else {
                    0 as libc::c_int
                })
            } else {
                (vbr_rate + vbr_bound - (*st).vbr_reservoir) >> 3 as libc::c_int + 3 as libc::c_int
            }) < nbAvailableBytes
            {
                if (if tell == 1 as libc::c_int {
                    2 as libc::c_int
                } else {
                    0 as libc::c_int
                }) > vbr_rate + vbr_bound - (*st).vbr_reservoir
                    >> 3 as libc::c_int + 3 as libc::c_int
                {
                    if tell == 1 as libc::c_int {
                        2 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }
                } else {
                    (vbr_rate + vbr_bound - (*st).vbr_reservoir)
                        >> 3 as libc::c_int + 3 as libc::c_int
                }
            } else {
                nbAvailableBytes
            };
            if max_allowed < nbAvailableBytes {
                nbCompressedBytes = nbFilledBytes + max_allowed;
                nbAvailableBytes = max_allowed;
                crate::src::opus_1_2_1::celt::entenc::ec_enc_shrink(
                    enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    nbCompressedBytes as crate::opus_types_h::opus_uint32,
                );
            }
        }
    }
    total_bits = nbCompressedBytes * 8 as libc::c_int;
    effEnd = end;
    if effEnd > (*mode).effEBands {
        effEnd = (*mode).effEBands
    }
    let mut fresh12 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong)
            .wrapping_mul((CC * (N + overlap)) as libc::c_ulong) as usize,
    );
    in_0 = fresh12.as_mut_ptr() as *mut crate::arch_h::celt_sig;
    sample_max = if (*st).overlap_max > celt_maxabs16(pcm, C * (N - overlap) / (*st).upsample) {
        (*st).overlap_max
    } else {
        celt_maxabs16(pcm, C * (N - overlap) / (*st).upsample)
    };
    (*st).overlap_max = celt_maxabs16(
        pcm.offset((C * (N - overlap) / (*st).upsample) as isize),
        C * overlap / (*st).upsample,
    );
    sample_max = if sample_max > (*st).overlap_max {
        sample_max
    } else {
        (*st).overlap_max
    };
    silence = (sample_max
        <= 1 as libc::c_int as crate::arch_h::opus_val16
            / ((1 as libc::c_int) << (*st).lsb_depth) as libc::c_float)
        as libc::c_int;
    if tell == 1 as libc::c_int {
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            silence,
            15 as libc::c_int as libc::c_uint,
        );
    } else {
        silence = 0 as libc::c_int
    }
    if silence != 0 {
        /*In VBR mode there is no need to send more than the minimum. */
        if vbr_rate > 0 as libc::c_int {
            nbCompressedBytes = if nbCompressedBytes < nbFilledBytes + 2 as libc::c_int {
                nbCompressedBytes
            } else {
                (nbFilledBytes) + 2 as libc::c_int
            };
            effectiveBytes = nbCompressedBytes;
            total_bits = nbCompressedBytes * 8 as libc::c_int;
            nbAvailableBytes = 2 as libc::c_int;
            crate::src::opus_1_2_1::celt::entenc::ec_enc_shrink(
                enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                nbCompressedBytes as crate::opus_types_h::opus_uint32,
            );
        }
        /* Pretend we've filled all the remaining bits with zeros
        (that's what the initialiser did anyway) */
        tell = nbCompressedBytes * 8 as libc::c_int;
        (*enc).nbits_total += tell - ec_tell(enc)
    }
    c = 0 as libc::c_int;
    loop {
        let mut need_clip: libc::c_int = 0 as libc::c_int;
        need_clip = ((*st).clip != 0 && sample_max > 65536.0f32) as libc::c_int;
        celt_preemphasis(
            pcm.offset(c as isize),
            in_0.offset((c * (N + overlap)) as isize)
                .offset(overlap as isize),
            N,
            CC,
            (*st).upsample,
            (*mode).preemph.as_ptr(),
            (*st).preemph_memE.as_mut_ptr().offset(c as isize),
            need_clip,
        );
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    /* Find pitch period and gain */
    let mut enabled: libc::c_int = 0;
    let mut qg: libc::c_int = 0;
    enabled = (((*st).lfe != 0 && nbAvailableBytes > 3 as libc::c_int
        || nbAvailableBytes > 12 as libc::c_int * C)
        && hybrid == 0
        && silence == 0
        && (*st).disable_pf == 0
        && (*st).complexity >= 5 as libc::c_int) as libc::c_int;
    prefilter_tapset = (*st).tapset_decision;
    pf_on = run_prefilter(
        st,
        in_0,
        prefilter_mem,
        CC,
        N,
        prefilter_tapset,
        &mut pitch_index,
        &mut gain1,
        &mut qg,
        enabled,
        nbAvailableBytes,
    );
    if (gain1 > 0.4f32 || (*st).prefilter_gain > 0.4f32)
        && ((*st).analysis.valid == 0 || (*st).analysis.tonality as libc::c_double > 0.3f64)
        && (pitch_index as libc::c_double > 1.26f64 * (*st).prefilter_period as libc::c_double
            || (pitch_index as libc::c_double) < 0.79f64 * (*st).prefilter_period as libc::c_double)
    {
        pitch_change = 1 as libc::c_int
    }
    if pf_on == 0 as libc::c_int {
        if hybrid == 0 && tell + 16 as libc::c_int <= total_bits {
            crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                0 as libc::c_int,
                1 as libc::c_int as libc::c_uint,
            );
        }
    } else {
        /*This block is not gated by a total bits check only because
        of the nbAvailableBytes check above.*/
        let mut octave: libc::c_int = 0;
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            1 as libc::c_int,
            1 as libc::c_int as libc::c_uint,
        );
        pitch_index += 1 as libc::c_int;
        octave = ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
            * 8 as libc::c_int
            - (pitch_index as libc::c_uint).leading_zeros() as i32
            - 5 as libc::c_int;
        crate::src::opus_1_2_1::celt::entenc::ec_enc_uint(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            octave as crate::opus_types_h::opus_uint32,
            6 as libc::c_int as crate::opus_types_h::opus_uint32,
        );
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            (pitch_index - ((16 as libc::c_int) << octave)) as crate::opus_types_h::opus_uint32,
            (4 as libc::c_int + octave) as libc::c_uint,
        );
        pitch_index -= 1 as libc::c_int;
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            qg as crate::opus_types_h::opus_uint32,
            3 as libc::c_int as libc::c_uint,
        );
        crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            prefilter_tapset,
            tapset_icdf.as_ptr(),
            2 as libc::c_int as libc::c_uint,
        );
    }
    isTransient = 0 as libc::c_int;
    shortBlocks = 0 as libc::c_int;
    if (*st).complexity >= 1 as libc::c_int && (*st).lfe == 0 {
        /* Reduces the likelihood of energy instability on fricatives at low bitrate
        in hybrid mode. It seems like we still want to have real transients on vowels
        though (small SILK quantization offset value). */
        let mut allow_weak_transients: libc::c_int = (hybrid != 0
            && effectiveBytes < 15 as libc::c_int
            && (*st).silk_info.offset >= 100 as libc::c_int)
            as libc::c_int; /* *< Interleaved signal MDCTs */
        isTransient = transient_analysis(
            in_0,
            N + overlap,
            CC,
            &mut tf_estimate,
            &mut tf_chan,
            allow_weak_transients,
            &mut weak_transient,
        )
    }
    if LM > 0 as libc::c_int && ec_tell(enc) + 3 as libc::c_int <= total_bits {
        if isTransient != 0 {
            shortBlocks = M
        }
    } else {
        isTransient = 0 as libc::c_int;
        transient_got_disabled = 1 as libc::c_int
    }
    let mut fresh13 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong)
            .wrapping_mul((CC * N) as libc::c_ulong) as usize,
    );
    freq = fresh13.as_mut_ptr() as *mut crate::arch_h::celt_sig;
    let mut fresh14 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_ener>() as libc::c_ulong)
            .wrapping_mul((nbEBands * CC) as libc::c_ulong) as usize,
    );
    bandE = fresh14.as_mut_ptr() as *mut crate::arch_h::celt_ener;
    let mut fresh15 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul((nbEBands * CC) as libc::c_ulong) as usize,
    );
    bandLogE = fresh15.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    secondMdct = (shortBlocks != 0 && (*st).complexity >= 8 as libc::c_int) as libc::c_int;
    let mut fresh16 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul((C * nbEBands) as libc::c_ulong) as usize,
    );
    bandLogE2 = fresh16.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    if secondMdct != 0 {
        compute_mdcts(
            mode,
            0 as libc::c_int,
            in_0,
            freq,
            C,
            CC,
            LM,
            (*st).upsample,
            (*st).arch,
        );
        crate::src::opus_1_2_1::celt::bands::compute_band_energies(
            mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
            freq,
            bandE,
            effEnd,
            C,
            LM,
            (*st).arch,
        );
        crate::src::opus_1_2_1::celt::quant_bands::amp2Log2(
            mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
            effEnd,
            end,
            bandE,
            bandLogE2,
            C,
        );
        i = 0 as libc::c_int;
        while i < C * nbEBands {
            let ref mut fresh17 = *bandLogE2.offset(i as isize);
            *fresh17 += 0.5f32 * LM as libc::c_float;
            i += 1
        }
    }
    compute_mdcts(
        mode,
        shortBlocks,
        in_0,
        freq,
        C,
        CC,
        LM,
        (*st).upsample,
        (*st).arch,
    );
    if CC == 2 as libc::c_int && C == 1 as libc::c_int {
        tf_chan = 0 as libc::c_int
    }
    crate::src::opus_1_2_1::celt::bands::compute_band_energies(
        mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        freq,
        bandE,
        effEnd,
        C,
        LM,
        (*st).arch,
    );
    if (*st).lfe != 0 {
        i = 2 as libc::c_int;
        while i < end {
            *bandE.offset(i as isize) =
                if *bandE.offset(i as isize) < 1e-4f32 * *bandE.offset(0 as libc::c_int as isize) {
                    *bandE.offset(i as isize)
                } else {
                    (1e-4f32) * *bandE.offset(0 as libc::c_int as isize)
                };
            *bandE.offset(i as isize) = if *bandE.offset(i as isize) > 1e-15f32 {
                *bandE.offset(i as isize)
            } else {
                1e-15f32
            };
            i += 1
        }
    }
    crate::src::opus_1_2_1::celt::quant_bands::amp2Log2(
        mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        effEnd,
        end,
        bandE,
        bandLogE,
        C,
    );
    let mut fresh18 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul((C * nbEBands) as libc::c_ulong) as usize,
    );
    surround_dynalloc = fresh18.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    crate::stdlib::memset(
        surround_dynalloc as *mut libc::c_void,
        0 as libc::c_int,
        (end as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong),
    );
    /* This computes how much masking takes place between surround channels */
    if hybrid == 0 && !(*st).energy_mask.is_null() && (*st).lfe == 0 {
        let mut mask_end: libc::c_int = 0;
        let mut midband: libc::c_int = 0;
        let mut count_dynalloc: libc::c_int = 0;
        let mut mask_avg: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
        let mut diff: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
        let mut count: libc::c_int = 0 as libc::c_int;
        mask_end = if 2 as libc::c_int > (*st).lastCodedBands {
            2 as libc::c_int
        } else {
            (*st).lastCodedBands
        };
        c = 0 as libc::c_int;
        while c < C {
            i = 0 as libc::c_int;
            while i < mask_end {
                let mut mask: crate::arch_h::opus_val16 = 0.;
                mask = if (if *(*st).energy_mask.offset((nbEBands * c + i) as isize) < 0.25f32 {
                    *(*st).energy_mask.offset((nbEBands * c + i) as isize)
                } else {
                    0.25f32
                }) > -2.0f32
                {
                    if *(*st).energy_mask.offset((nbEBands * c + i) as isize) < 0.25f32 {
                        *(*st).energy_mask.offset((nbEBands * c + i) as isize)
                    } else {
                        0.25f32
                    }
                } else {
                    -2.0f32
                };
                if mask > 0 as libc::c_int as libc::c_float {
                    mask = 0.5f32 * mask
                }
                mask_avg += mask
                    * (*eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        - *eBands.offset(i as isize) as libc::c_int)
                        as crate::arch_h::opus_val32;
                count += *eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *eBands.offset(i as isize) as libc::c_int;
                diff += mask
                    * (1 as libc::c_int + 2 as libc::c_int * i - mask_end)
                        as crate::arch_h::opus_val32;
                i += 1
            }
            c += 1
        }
        mask_avg = mask_avg / count as crate::arch_h::opus_val16;
        mask_avg += 0.2f32;
        diff = diff * 6 as libc::c_int as libc::c_float
            / (C * (mask_end - 1 as libc::c_int) * (mask_end + 1 as libc::c_int) * mask_end)
                as libc::c_float;
        /* Again, being conservative */
        diff = 0.5f32 * diff;
        diff = if (if diff < 0.031f32 { diff } else { 0.031f32 }) > -0.031f32 {
            if diff < 0.031f32 {
                diff
            } else {
                0.031f32
            }
        } else {
            -0.031f32
        };
        /* Find the band that's in the middle of the coded spectrum */
        midband = 0 as libc::c_int;
        while (*eBands.offset((midband + 1 as libc::c_int) as isize) as libc::c_int)
            < *eBands.offset(mask_end as isize) as libc::c_int / 2 as libc::c_int
        {
            midband += 1
        }
        count_dynalloc = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < mask_end {
            let mut lin: crate::arch_h::opus_val32 = 0.;
            let mut unmask: crate::arch_h::opus_val16 = 0.;
            lin = mask_avg + diff * (i - midband) as libc::c_float;
            if C == 2 as libc::c_int {
                unmask = if *(*st).energy_mask.offset(i as isize)
                    > *(*st).energy_mask.offset((nbEBands + i) as isize)
                {
                    *(*st).energy_mask.offset(i as isize)
                } else {
                    *(*st).energy_mask.offset((nbEBands + i) as isize)
                }
            } else {
                unmask = *(*st).energy_mask.offset(i as isize)
            }
            unmask = if unmask < 0.0f32 { unmask } else { 0.0f32 };
            unmask -= lin;
            if unmask > 0.25f32 {
                *surround_dynalloc.offset(i as isize) = unmask - 0.25f32;
                count_dynalloc += 1
            }
            i += 1
        }
        if count_dynalloc >= 3 as libc::c_int {
            /* If we need dynalloc in many bands, it's probably because our
            initial masking rate was too low. */
            mask_avg += 0.25f32;
            if mask_avg > 0 as libc::c_int as libc::c_float {
                /* Something went really wrong in the original calculations,
                disabling masking. */
                mask_avg = 0 as libc::c_int as crate::arch_h::opus_val32;
                diff = 0 as libc::c_int as crate::arch_h::opus_val32;
                crate::stdlib::memset(
                    surround_dynalloc as *mut libc::c_void,
                    0 as libc::c_int,
                    (mask_end as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong
                        ),
                );
            } else {
                i = 0 as libc::c_int;
                while i < mask_end {
                    *surround_dynalloc.offset(i as isize) = if 0 as libc::c_int as libc::c_float
                        > *surround_dynalloc.offset(i as isize) - 0.25f32
                    {
                        0 as libc::c_int as libc::c_float
                    } else {
                        (*surround_dynalloc.offset(i as isize)) - 0.25f32
                    };
                    i += 1
                }
            }
        }
        mask_avg += 0.2f32;
        /* Convert to 1/64th units used for the trim */
        surround_trim = 64 as libc::c_int as libc::c_float * diff;
        /*printf("%d %d ", mask_avg, surround_trim);*/
        surround_masking = mask_avg
    }
    /* Temporal VBR (but not for LFE) */
    if (*st).lfe == 0 {
        let mut follow: crate::arch_h::opus_val16 = -10.0f32;
        let mut frame_avg: crate::arch_h::opus_val32 =
            0 as libc::c_int as crate::arch_h::opus_val32;
        let mut offset: crate::arch_h::opus_val16 = if shortBlocks != 0 {
            (0.5f32) * LM as libc::c_float
        } else {
            0 as libc::c_int as libc::c_float
        };
        i = start;
        while i < end {
            follow = if follow - 1.0f32 > *bandLogE.offset(i as isize) - offset {
                (follow) - 1.0f32
            } else {
                (*bandLogE.offset(i as isize)) - offset
            };
            if C == 2 as libc::c_int {
                follow = if follow > *bandLogE.offset((i + nbEBands) as isize) - offset {
                    follow
                } else {
                    (*bandLogE.offset((i + nbEBands) as isize)) - offset
                }
            }
            frame_avg += follow;
            i += 1
        }
        frame_avg /= (end - start) as libc::c_float;
        temporal_vbr = frame_avg - (*st).spec_avg;
        temporal_vbr = if 3.0f32
            < (if -1.5f32 > temporal_vbr {
                -1.5f32
            } else {
                temporal_vbr
            }) {
            3.0f32
        } else if -1.5f32 > temporal_vbr {
            -1.5f32
        } else {
            temporal_vbr
        };
        (*st).spec_avg += 0.02f32 * temporal_vbr
    }
    /*for (i=0;i<21;i++)
       printf("%f ", bandLogE[i]);
    printf("\n");*/
    if secondMdct == 0 {
        crate::stdlib::memcpy(
            bandLogE2 as *mut libc::c_void,
            bandLogE as *const libc::c_void,
            ((C * nbEBands) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * bandLogE2.wrapping_offset_from(bandLogE) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
    }
    /* Last chance to catch any transient we might have missed in the
    time-domain analysis */
    if LM > 0 as libc::c_int
        && ec_tell(enc) + 3 as libc::c_int <= total_bits
        && isTransient == 0
        && (*st).complexity >= 5 as libc::c_int
        && (*st).lfe == 0
        && hybrid == 0
    {
        if patch_transient_decision(bandLogE, oldBandE, nbEBands, start, end, C) != 0 {
            isTransient = 1 as libc::c_int;
            shortBlocks = M;
            compute_mdcts(
                mode,
                shortBlocks,
                in_0,
                freq,
                C,
                CC,
                LM,
                (*st).upsample,
                (*st).arch,
            );
            crate::src::opus_1_2_1::celt::bands::compute_band_energies(
                mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
                freq,
                bandE,
                effEnd,
                C,
                LM,
                (*st).arch,
            );
            crate::src::opus_1_2_1::celt::quant_bands::amp2Log2(
                mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
                effEnd,
                end,
                bandE,
                bandLogE,
                C,
            );
            /* Compensate for the scaling of short vs long mdcts */
            i = 0 as libc::c_int; /* *< Interleaved normalised MDCTs */
            while i < C * nbEBands {
                let ref mut fresh19 = *bandLogE2.offset(i as isize);
                *fresh19 += 0.5f32 * LM as libc::c_float;
                i += 1
            }
            tf_estimate = 0.2f32
        }
    }
    if LM > 0 as libc::c_int && ec_tell(enc) + 3 as libc::c_int <= total_bits {
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            isTransient,
            3 as libc::c_int as libc::c_uint,
        );
    }
    let mut fresh20 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_mul((C * N) as libc::c_ulong) as usize,
    );
    X = fresh20.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    /* Band normalisation */
    crate::src::opus_1_2_1::celt::bands::normalise_bands(
        mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        freq,
        X,
        bandE,
        effEnd,
        C,
        M,
    );
    let mut fresh21 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nbEBands as libc::c_ulong) as usize,
    );
    tf_res = fresh21.as_mut_ptr() as *mut libc::c_int;
    /* Disable variable tf resolution for hybrid and at very low bitrate */
    if effectiveBytes >= 15 as libc::c_int * C
        && hybrid == 0
        && (*st).complexity >= 2 as libc::c_int
        && (*st).lfe == 0
    {
        let mut lambda: libc::c_int = 0;
        lambda = if 5 as libc::c_int > 1280 as libc::c_int / effectiveBytes + 2 as libc::c_int {
            5 as libc::c_int
        } else {
            (1280 as libc::c_int / effectiveBytes) + 2 as libc::c_int
        };
        tf_select = tf_analysis(
            mode,
            effEnd,
            isTransient,
            tf_res,
            lambda,
            X,
            N,
            LM,
            tf_estimate,
            tf_chan,
        );
        i = effEnd;
        while i < end {
            *tf_res.offset(i as isize) = *tf_res.offset((effEnd - 1 as libc::c_int) as isize);
            i += 1
        }
    } else if hybrid != 0 && weak_transient != 0 {
        /* For weak transients, we rely on the fact that improving time resolution using
        TF on a long window is imperfect and will not result in an energy collapse at
        low bitrate. */
        i = 0 as libc::c_int;
        while i < end {
            *tf_res.offset(i as isize) = 1 as libc::c_int;
            i += 1
        }
        tf_select = 0 as libc::c_int
    } else if hybrid != 0 && effectiveBytes < 15 as libc::c_int {
        /* For low bitrate hybrid, we force temporal resolution to 5 ms rather than 2.5 ms. */
        i = 0 as libc::c_int;
        while i < end {
            *tf_res.offset(i as isize) = 0 as libc::c_int;
            i += 1
        }
        tf_select = isTransient
    } else {
        i = 0 as libc::c_int;
        while i < end {
            *tf_res.offset(i as isize) = isTransient;
            i += 1
        }
        tf_select = 0 as libc::c_int
    }
    let mut fresh22 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul((C * nbEBands) as libc::c_ulong) as usize,
    );
    error = fresh22.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    c = 0 as libc::c_int;
    loop {
        i = start;
        while i < end {
            /* When the energy is stable, slightly bias energy quantization towards
            the previous error to make the gain more stable (a constant offset is
            better than fluctuations). */
            if (crate::stdlib::fabs(
                (*bandLogE.offset((i + c * nbEBands) as isize)
                    - *oldBandE.offset((i + c * nbEBands) as isize))
                    as libc::c_double,
            ) as libc::c_float)
                < 2.0f32
            {
                let ref mut fresh23 = *bandLogE.offset((i + c * nbEBands) as isize);
                *fresh23 -= *energyError.offset((i + c * nbEBands) as isize) * 0.25f32
            }
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    crate::src::opus_1_2_1::celt::quant_bands::quant_coarse_energy(
        mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        start,
        end,
        effEnd,
        bandLogE,
        oldBandE,
        total_bits as crate::opus_types_h::opus_uint32,
        error,
        enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        C,
        LM,
        nbAvailableBytes,
        (*st).force_intra,
        &mut (*st).delayedIntra,
        ((*st).complexity >= 4 as libc::c_int) as libc::c_int,
        (*st).loss_rate,
        (*st).lfe,
    );
    tf_encode(start, end, isTransient, tf_res, LM, tf_select, enc);
    if ec_tell(enc) + 4 as libc::c_int <= total_bits {
        if (*st).lfe != 0 {
            (*st).tapset_decision = 0 as libc::c_int;
            (*st).spread_decision = 2 as libc::c_int
        } else if hybrid != 0 {
            if (*st).complexity == 0 as libc::c_int {
                (*st).spread_decision = 0 as libc::c_int
            } else if isTransient != 0 {
                (*st).spread_decision = 2 as libc::c_int
            } else {
                (*st).spread_decision = 3 as libc::c_int
            }
        } else if shortBlocks != 0
            || (*st).complexity < 3 as libc::c_int
            || nbAvailableBytes < 10 as libc::c_int * C
        {
            if (*st).complexity == 0 as libc::c_int {
                (*st).spread_decision = 0 as libc::c_int
            } else {
                (*st).spread_decision = 2 as libc::c_int
            }
        } else {
            /* Disable new spreading+tapset estimator until we can show it works
            better than the old one. So far it seems like spreading_decision()
            works best. */
            (*st).spread_decision = crate::src::opus_1_2_1::celt::bands::spreading_decision(
                mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
                X,
                &mut (*st).tonal_average,
                (*st).spread_decision,
                &mut (*st).hf_average,
                &mut (*st).tapset_decision,
                (pf_on != 0 && shortBlocks == 0) as libc::c_int,
                effEnd,
                C,
                M,
            )
            /*printf("%d %d\n", st->tapset_decision, st->spread_decision);*/
            /*printf("%f %d %f %d\n\n", st->analysis.tonality, st->spread_decision, st->analysis.tonality_slope, st->tapset_decision);*/
        }
        crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            (*st).spread_decision,
            spread_icdf.as_ptr(),
            5 as libc::c_int as libc::c_uint,
        );
    }
    let mut fresh24 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nbEBands as libc::c_ulong) as usize,
    );
    offsets = fresh24.as_mut_ptr() as *mut libc::c_int;
    maxDepth = dynalloc_analysis(
        bandLogE,
        bandLogE2,
        nbEBands,
        start,
        end,
        C,
        offsets,
        (*st).lsb_depth,
        (*mode).logN,
        isTransient,
        (*st).vbr,
        (*st).constrained_vbr,
        eBands,
        LM,
        effectiveBytes,
        &mut tot_boost,
        (*st).lfe,
        surround_dynalloc,
        &mut (*st).analysis,
    );
    /* For LFE, everything interesting is in the first band */
    if (*st).lfe != 0 {
        *offsets.offset(0 as libc::c_int as isize) =
            if (8 as libc::c_int) < effectiveBytes / 3 as libc::c_int {
                8 as libc::c_int
            } else {
                (effectiveBytes) / 3 as libc::c_int
            }
    }
    let mut fresh25 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nbEBands as libc::c_ulong) as usize,
    );
    cap = fresh25.as_mut_ptr() as *mut libc::c_int;
    crate::src::opus_1_2_1::celt::celt::init_caps(
        mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        cap,
        LM,
        C,
    );
    dynalloc_logp = 6 as libc::c_int;
    total_bits <<= 3 as libc::c_int;
    total_boost = 0 as libc::c_int;
    tell = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(
        enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) as crate::opus_types_h::opus_int32;
    i = start;
    while i < end {
        let mut width: libc::c_int = 0;
        let mut quanta: libc::c_int = 0;
        let mut dynalloc_loop_logp: libc::c_int = 0;
        let mut boost: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        width = (C
            * (*eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                - *eBands.offset(i as isize) as libc::c_int))
            << LM;
        /* quanta is 6 bits, but no more than 1 bit/sample
        and no less than 1/8 bit/sample */
        quanta = if (width << 3 as libc::c_int)
            < (if (6 as libc::c_int) << 3 as libc::c_int > width {
                (6 as libc::c_int) << 3 as libc::c_int
            } else {
                width
            }) {
            (width) << 3 as libc::c_int
        } else if (6 as libc::c_int) << 3 as libc::c_int > width {
            (6 as libc::c_int) << 3 as libc::c_int
        } else {
            width
        };
        dynalloc_loop_logp = dynalloc_logp;
        boost = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while (tell + (dynalloc_loop_logp << 3 as libc::c_int)) < total_bits - total_boost
            && boost < *cap.offset(i as isize)
        {
            let mut flag: libc::c_int = 0;
            flag = (j < *offsets.offset(i as isize)) as libc::c_int;
            crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                flag,
                dynalloc_loop_logp as libc::c_uint,
            );
            tell = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(
                enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            ) as crate::opus_types_h::opus_int32;
            if flag == 0 {
                break;
            }
            boost += quanta;
            total_boost += quanta;
            dynalloc_loop_logp = 1 as libc::c_int;
            j += 1
        }
        /* Making dynalloc more likely */
        if j != 0 {
            dynalloc_logp = if 2 as libc::c_int > dynalloc_logp - 1 as libc::c_int {
                2 as libc::c_int
            } else {
                (dynalloc_logp) - 1 as libc::c_int
            }
        }
        *offsets.offset(i as isize) = boost;
        i += 1
    }
    if C == 2 as libc::c_int {
        static mut intensity_thresholds: [crate::arch_h::opus_val16; 21] = [
            1 as libc::c_int as crate::arch_h::opus_val16,
            2 as libc::c_int as crate::arch_h::opus_val16,
            3 as libc::c_int as crate::arch_h::opus_val16,
            4 as libc::c_int as crate::arch_h::opus_val16,
            5 as libc::c_int as crate::arch_h::opus_val16,
            6 as libc::c_int as crate::arch_h::opus_val16,
            7 as libc::c_int as crate::arch_h::opus_val16,
            8 as libc::c_int as crate::arch_h::opus_val16,
            16 as libc::c_int as crate::arch_h::opus_val16,
            24 as libc::c_int as crate::arch_h::opus_val16,
            36 as libc::c_int as crate::arch_h::opus_val16,
            44 as libc::c_int as crate::arch_h::opus_val16,
            50 as libc::c_int as crate::arch_h::opus_val16,
            56 as libc::c_int as crate::arch_h::opus_val16,
            62 as libc::c_int as crate::arch_h::opus_val16,
            67 as libc::c_int as crate::arch_h::opus_val16,
            72 as libc::c_int as crate::arch_h::opus_val16,
            79 as libc::c_int as crate::arch_h::opus_val16,
            88 as libc::c_int as crate::arch_h::opus_val16,
            106 as libc::c_int as crate::arch_h::opus_val16,
            134 as libc::c_int as crate::arch_h::opus_val16,
        ];
        static mut intensity_histeresis: [crate::arch_h::opus_val16; 21] = [
            1 as libc::c_int as crate::arch_h::opus_val16,
            1 as libc::c_int as crate::arch_h::opus_val16,
            1 as libc::c_int as crate::arch_h::opus_val16,
            1 as libc::c_int as crate::arch_h::opus_val16,
            1 as libc::c_int as crate::arch_h::opus_val16,
            1 as libc::c_int as crate::arch_h::opus_val16,
            1 as libc::c_int as crate::arch_h::opus_val16,
            2 as libc::c_int as crate::arch_h::opus_val16,
            2 as libc::c_int as crate::arch_h::opus_val16,
            2 as libc::c_int as crate::arch_h::opus_val16,
            2 as libc::c_int as crate::arch_h::opus_val16,
            2 as libc::c_int as crate::arch_h::opus_val16,
            2 as libc::c_int as crate::arch_h::opus_val16,
            2 as libc::c_int as crate::arch_h::opus_val16,
            3 as libc::c_int as crate::arch_h::opus_val16,
            3 as libc::c_int as crate::arch_h::opus_val16,
            4 as libc::c_int as crate::arch_h::opus_val16,
            5 as libc::c_int as crate::arch_h::opus_val16,
            6 as libc::c_int as crate::arch_h::opus_val16,
            8 as libc::c_int as crate::arch_h::opus_val16,
            8 as libc::c_int as crate::arch_h::opus_val16,
        ];
        /* Always use MS for 2.5 ms frames until we can do a better analysis */
        if LM != 0 as libc::c_int {
            dual_stereo = stereo_analysis(mode, X, LM, N)
        }
        (*st).intensity = crate::src::opus_1_2_1::celt::bands::hysteresis_decision(
            (equiv_rate / 1000 as libc::c_int) as crate::arch_h::opus_val16,
            intensity_thresholds.as_ptr(),
            intensity_histeresis.as_ptr(),
            21 as libc::c_int,
            (*st).intensity,
        );
        (*st).intensity = if end
            < (if start > (*st).intensity {
                start
            } else {
                (*st).intensity
            }) {
            end
        } else if start > (*st).intensity {
            start
        } else {
            (*st).intensity
        }
    }
    alloc_trim = 5 as libc::c_int;
    if tell + ((6 as libc::c_int) << 3 as libc::c_int) <= total_bits - total_boost {
        if start > 0 as libc::c_int || (*st).lfe != 0 {
            (*st).stereo_saving = 0 as libc::c_int as crate::arch_h::opus_val16;
            alloc_trim = 5 as libc::c_int
        } else {
            alloc_trim = alloc_trim_analysis(
                mode,
                X,
                bandLogE,
                end,
                LM,
                C,
                N,
                &mut (*st).analysis,
                &mut (*st).stereo_saving,
                tf_estimate,
                (*st).intensity,
                surround_trim,
                equiv_rate,
                (*st).arch,
            )
        }
        crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            alloc_trim,
            trim_icdf.as_ptr(),
            7 as libc::c_int as libc::c_uint,
        );
        tell = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        ) as crate::opus_types_h::opus_int32
    }
    /* Variable bitrate */
    if vbr_rate > 0 as libc::c_int {
        let mut alpha: crate::arch_h::opus_val16 = 0.;
        let mut delta: crate::opus_types_h::opus_int32 = 0;
        /* The target rate in 8th bits per frame */
        let mut target: crate::opus_types_h::opus_int32 = 0;
        let mut base_target: crate::opus_types_h::opus_int32 = 0;
        let mut min_allowed: crate::opus_types_h::opus_int32 = 0;
        let mut lm_diff: libc::c_int = (*mode).maxLM - LM;
        /* Don't attempt to use more than 510 kb/s, even for frames smaller than 20 ms.
        The CELT allocator will just not be able to use more than that anyway. */
        nbCompressedBytes = if nbCompressedBytes < 1275 as libc::c_int >> 3 as libc::c_int - LM {
            nbCompressedBytes
        } else {
            (1275 as libc::c_int) >> 3 as libc::c_int - LM
        };
        if hybrid == 0 {
            base_target =
                vbr_rate - ((40 as libc::c_int * C + 20 as libc::c_int) << 3 as libc::c_int)
        } else {
            base_target = if 0 as libc::c_int
                > vbr_rate - ((9 as libc::c_int * C + 4 as libc::c_int) << 3 as libc::c_int)
            {
                0 as libc::c_int
            } else {
                (vbr_rate) - ((9 as libc::c_int * C + 4 as libc::c_int) << 3 as libc::c_int)
            }
        }
        if (*st).constrained_vbr != 0 {
            base_target += (*st).vbr_offset >> lm_diff
        }
        if hybrid == 0 {
            target = compute_vbr(
                mode,
                &mut (*st).analysis,
                base_target,
                LM,
                equiv_rate,
                (*st).lastCodedBands,
                C,
                (*st).intensity,
                (*st).constrained_vbr,
                (*st).stereo_saving,
                tot_boost,
                tf_estimate,
                pitch_change,
                maxDepth,
                (*st).lfe,
                ((*st).energy_mask != 0 as *mut libc::c_void as *mut crate::arch_h::opus_val16)
                    as libc::c_int,
                surround_masking,
                temporal_vbr,
            )
        } else {
            target = base_target;
            /* Tonal frames (offset<100) need more bits than noisy (offset>100) ones. */
            if (*st).silk_info.offset < 100 as libc::c_int {
                target += (12 as libc::c_int) << 3 as libc::c_int >> 3 as libc::c_int - LM
            }
            if (*st).silk_info.offset > 100 as libc::c_int {
                target -= (18 as libc::c_int) << 3 as libc::c_int >> 3 as libc::c_int - LM
            }
            /* Boosting bitrate on transients and vowels with significant temporal
            spikes. */
            target += ((tf_estimate - 0.25f32)
                * ((50 as libc::c_int) << 3 as libc::c_int) as libc::c_float)
                as crate::opus_types_h::opus_int32;
            /* If we have a strong transient, let's make sure it has enough bits to code
            the first two bands, so that it can use folding rather than noise. */
            if tf_estimate > 0.7f32 {
                target = if target > (50 as libc::c_int) << 3 as libc::c_int {
                    target
                } else {
                    (50 as libc::c_int) << 3 as libc::c_int
                }
            }
        }
        /* The current offset is removed from the target and the space used
        so far is added*/
        target = target + tell;
        /* In VBR mode the frame size must not be reduced so much that it would
         result in the encoder running out of bits.
        The margin of 2 bytes ensures that none of the bust-prevention logic
         in the decoder will have triggered so far. */
        min_allowed =
            (tell + total_boost + ((1 as libc::c_int) << 3 as libc::c_int + 3 as libc::c_int)
                - 1 as libc::c_int
                >> 3 as libc::c_int + 3 as libc::c_int)
                + 2 as libc::c_int;
        /* Take into account the 37 bits we need to have left in the packet to
        signal a redundant frame in hybrid mode. Creating a shorter packet would
        create an entropy coder desync. */
        if hybrid != 0 {
            min_allowed = if min_allowed
                > tell0_frac
                    + ((37 as libc::c_int) << 3 as libc::c_int)
                    + total_boost
                    + ((1 as libc::c_int) << 3 as libc::c_int + 3 as libc::c_int)
                    - 1 as libc::c_int
                    >> 3 as libc::c_int + 3 as libc::c_int
            {
                min_allowed
            } else {
                (tell0_frac
                    + ((37 as libc::c_int) << 3 as libc::c_int)
                    + total_boost
                    + ((1 as libc::c_int) << 3 as libc::c_int + 3 as libc::c_int)
                    - 1 as libc::c_int)
                    >> 3 as libc::c_int + 3 as libc::c_int
            }
        }
        nbAvailableBytes = target + ((1 as libc::c_int) << 3 as libc::c_int + 2 as libc::c_int)
            >> 3 as libc::c_int + 3 as libc::c_int;
        nbAvailableBytes = if min_allowed > nbAvailableBytes {
            min_allowed
        } else {
            nbAvailableBytes
        };
        nbAvailableBytes = if nbCompressedBytes < nbAvailableBytes {
            nbCompressedBytes
        } else {
            nbAvailableBytes
        };
        /* By how much did we "miss" the target on that frame */
        delta = target - vbr_rate;
        target = nbAvailableBytes << 3 as libc::c_int + 3 as libc::c_int;
        /*If the frame is silent we don't adjust our drift, otherwise
        the encoder will shoot to very high rates after hitting a
        span of silence, but we do allow the bitres to refill.
        This means that we'll undershoot our target in CVBR/VBR modes
        on files with lots of silence. */
        if silence != 0 {
            nbAvailableBytes = 2 as libc::c_int;
            target = (2 as libc::c_int * 8 as libc::c_int) << 3 as libc::c_int;
            delta = 0 as libc::c_int
        }
        if (*st).vbr_count < 970 as libc::c_int {
            (*st).vbr_count += 1;
            alpha = 1.0f32 / ((*st).vbr_count + 20 as libc::c_int) as libc::c_float
        } else {
            alpha = 0.001f32
        }
        /* How many bits have we used in excess of what we're allowed */
        if (*st).constrained_vbr != 0 {
            (*st).vbr_reservoir += target - vbr_rate
        }
        /*printf ("%d\n", st->vbr_reservoir);*/
        /* Compute the offset we need to apply in order to reach the target */
        if (*st).constrained_vbr != 0 {
            (*st).vbr_drift += (alpha
                * (delta * ((1 as libc::c_int) << lm_diff) - (*st).vbr_offset - (*st).vbr_drift)
                    as libc::c_float)
                as crate::opus_types_h::opus_int32;
            (*st).vbr_offset = -(*st).vbr_drift
        }
        /*printf ("%d\n", st->vbr_drift);*/
        if (*st).constrained_vbr != 0 && (*st).vbr_reservoir < 0 as libc::c_int {
            /* We're under the min value -- increase rate */
            let mut adjust: libc::c_int =
                -(*st).vbr_reservoir / ((8 as libc::c_int) << 3 as libc::c_int);
            /* Unless we're just coding silence */
            nbAvailableBytes += if silence != 0 {
                0 as libc::c_int
            } else {
                adjust
            };
            (*st).vbr_reservoir = 0 as libc::c_int
        }
        nbCompressedBytes = if nbCompressedBytes < nbAvailableBytes {
            nbCompressedBytes
        } else {
            nbAvailableBytes
        };
        /*printf("%d\n", nbCompressedBytes*50*8);*/
        /* This moves the raw bits to take into account the new compressed size */
        crate::src::opus_1_2_1::celt::entenc::ec_enc_shrink(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            nbCompressedBytes as crate::opus_types_h::opus_uint32,
        );
    }
    /* Bit allocation */
    let mut fresh26 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nbEBands as libc::c_ulong) as usize,
    );
    fine_quant = fresh26.as_mut_ptr() as *mut libc::c_int;
    let mut fresh27 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nbEBands as libc::c_ulong) as usize,
    );
    pulses = fresh27.as_mut_ptr() as *mut libc::c_int;
    let mut fresh28 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nbEBands as libc::c_ulong) as usize,
    );
    fine_priority = fresh28.as_mut_ptr() as *mut libc::c_int;
    /* bits =           packet size                    - where we are - safety*/
    bits = (((nbCompressedBytes * 8 as libc::c_int) << 3 as libc::c_int) as libc::c_uint)
        .wrapping_sub(crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        ))
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        as crate::opus_types_h::opus_int32;
    anti_collapse_rsv = if isTransient != 0
        && LM >= 2 as libc::c_int
        && bits >= (LM + 2 as libc::c_int) << 3 as libc::c_int
    {
        (1 as libc::c_int) << 3 as libc::c_int
    } else {
        0 as libc::c_int
    };
    bits -= anti_collapse_rsv;
    signalBandwidth = end - 1 as libc::c_int;
    if (*st).analysis.valid != 0 {
        let mut min_bandwidth: libc::c_int = 0;
        if equiv_rate < 32000 as libc::c_int * C {
            min_bandwidth = 13 as libc::c_int
        } else if equiv_rate < 48000 as libc::c_int * C {
            min_bandwidth = 16 as libc::c_int
        } else if equiv_rate < 60000 as libc::c_int * C {
            min_bandwidth = 18 as libc::c_int
        } else if equiv_rate < 80000 as libc::c_int * C {
            min_bandwidth = 19 as libc::c_int
        } else {
            min_bandwidth = 20 as libc::c_int
        }
        signalBandwidth = if (*st).analysis.bandwidth > min_bandwidth {
            (*st).analysis.bandwidth
        } else {
            min_bandwidth
        }
    }
    if (*st).lfe != 0 {
        signalBandwidth = 1 as libc::c_int
    }
    codedBands = crate::src::opus_1_2_1::celt::rate::compute_allocation(
        mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        start,
        end,
        offsets,
        cap,
        alloc_trim,
        &mut (*st).intensity,
        &mut dual_stereo,
        bits,
        &mut balance,
        pulses,
        fine_quant,
        fine_priority,
        C,
        LM,
        enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        1 as libc::c_int,
        (*st).lastCodedBands,
        signalBandwidth,
    );
    if (*st).lastCodedBands != 0 {
        (*st).lastCodedBands = if ((*st).lastCodedBands + 1 as libc::c_int)
            < (if (*st).lastCodedBands - 1 as libc::c_int > codedBands {
                ((*st).lastCodedBands) - 1 as libc::c_int
            } else {
                codedBands
            }) {
            ((*st).lastCodedBands) + 1 as libc::c_int
        } else if (*st).lastCodedBands - 1 as libc::c_int > codedBands {
            ((*st).lastCodedBands) - 1 as libc::c_int
        } else {
            codedBands
        }
    } else {
        (*st).lastCodedBands = codedBands
    }
    crate::src::opus_1_2_1::celt::quant_bands::quant_fine_energy(
        mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        start,
        end,
        oldBandE,
        error,
        fine_quant,
        enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        C,
    );
    /* Residual quantisation */
    let mut fresh29 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul((C * nbEBands) as libc::c_ulong) as usize,
    );
    collapse_masks = fresh29.as_mut_ptr() as *mut libc::c_uchar;
    crate::src::opus_1_2_1::celt::bands::quant_all_bands(
        1 as libc::c_int,
        mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        start,
        end,
        X,
        if C == 2 as libc::c_int {
            X.offset(N as isize)
        } else {
            0 as *mut crate::arch_h::celt_norm
        },
        collapse_masks,
        bandE,
        pulses,
        shortBlocks,
        (*st).spread_decision,
        dual_stereo,
        (*st).intensity,
        tf_res,
        nbCompressedBytes * ((8 as libc::c_int) << 3 as libc::c_int) - anti_collapse_rsv,
        balance,
        enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        LM,
        codedBands,
        &mut (*st).rng,
        (*st).complexity,
        (*st).arch,
        (*st).disable_inv,
    );
    if anti_collapse_rsv > 0 as libc::c_int {
        anti_collapse_on = ((*st).consec_transient < 2 as libc::c_int) as libc::c_int;
        crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
            enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            anti_collapse_on as crate::opus_types_h::opus_uint32,
            1 as libc::c_int as libc::c_uint,
        );
    }
    crate::src::opus_1_2_1::celt::quant_bands::quant_energy_finalise(
        mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        start,
        end,
        oldBandE,
        error,
        fine_quant,
        fine_priority,
        nbCompressedBytes * 8 as libc::c_int - ec_tell(enc),
        enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        C,
    );
    crate::stdlib::memset(
        energyError as *mut libc::c_void,
        0 as libc::c_int,
        ((nbEBands * CC) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong),
    );
    c = 0 as libc::c_int;
    loop {
        i = start;
        while i < end {
            *energyError.offset((i + c * nbEBands) as isize) = if -0.5f32
                > (if 0.5f32 < *error.offset((i + c * nbEBands) as isize) {
                    0.5f32
                } else {
                    *error.offset((i + c * nbEBands) as isize)
                }) {
                -0.5f32
            } else if 0.5f32 < *error.offset((i + c * nbEBands) as isize) {
                0.5f32
            } else {
                *error.offset((i + c * nbEBands) as isize)
            };
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    if silence != 0 {
        i = 0 as libc::c_int;
        while i < C * nbEBands {
            *oldBandE.offset(i as isize) = -28.0f32;
            i += 1
        }
    }
    (*st).prefilter_period = pitch_index;
    (*st).prefilter_gain = gain1;
    (*st).prefilter_tapset = prefilter_tapset;
    if CC == 2 as libc::c_int && C == 1 as libc::c_int {
        crate::stdlib::memcpy(
            &mut *oldBandE.offset(nbEBands as isize) as *mut crate::arch_h::opus_val16
                as *mut libc::c_void,
            oldBandE as *const libc::c_void,
            (nbEBands as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * (&mut *oldBandE.offset(nbEBands as isize)
                            as *mut crate::arch_h::opus_val16)
                            .wrapping_offset_from(oldBandE)
                            as libc::c_long) as libc::c_ulong,
                ),
        );
    }
    if isTransient == 0 {
        crate::stdlib::memcpy(
            oldLogE2 as *mut libc::c_void,
            oldLogE as *const libc::c_void,
            ((CC * nbEBands) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * oldLogE2.wrapping_offset_from(oldLogE) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        crate::stdlib::memcpy(
            oldLogE as *mut libc::c_void,
            oldBandE as *const libc::c_void,
            ((CC * nbEBands) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * oldLogE.wrapping_offset_from(oldBandE) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
    } else {
        i = 0 as libc::c_int;
        while i < CC * nbEBands {
            *oldLogE.offset(i as isize) =
                if *oldLogE.offset(i as isize) < *oldBandE.offset(i as isize) {
                    *oldLogE.offset(i as isize)
                } else {
                    *oldBandE.offset(i as isize)
                };
            i += 1
        }
    }
    /* In case start or end were to change */
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < start {
            *oldBandE.offset((c * nbEBands + i) as isize) =
                0 as libc::c_int as crate::arch_h::opus_val16;
            let ref mut fresh30 = *oldLogE2.offset((c * nbEBands + i) as isize);
            *fresh30 = -28.0f32;
            *oldLogE.offset((c * nbEBands + i) as isize) = *fresh30;
            i += 1
        }
        i = end;
        while i < nbEBands {
            *oldBandE.offset((c * nbEBands + i) as isize) =
                0 as libc::c_int as crate::arch_h::opus_val16;
            let ref mut fresh31 = *oldLogE2.offset((c * nbEBands + i) as isize);
            *fresh31 = -28.0f32;
            *oldLogE.offset((c * nbEBands + i) as isize) = *fresh31;
            i += 1
        }
        c += 1;
        if !(c < CC) {
            break;
        }
    }
    if isTransient != 0 || transient_got_disabled != 0 {
        (*st).consec_transient += 1
    } else {
        (*st).consec_transient = 0 as libc::c_int
    }
    (*st).rng = (*enc).rng;
    /* If there's any room left (can only happen for very high rates),
    it's already filled with zeros */
    crate::src::opus_1_2_1::celt::entenc::ec_enc_done(
        enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    );
    if ec_get_error(enc) != 0 {
        return -(3 as libc::c_int);
    } else {
        return nbCompressedBytes;
    };
}
/* CUSTOM_MODES */
#[no_mangle]

pub unsafe extern "C" fn opus_custom_encoder_ctl(
    mut st: *mut OpusCustomEncoder,
    mut request: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut current_block: u64;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    match request {
        4010 => {
            let mut value: libc::c_int = ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value < 0 as libc::c_int || value > 10 as libc::c_int {
                current_block = 796174441944384681;
            } else {
                (*st).complexity = value;
                current_block = 4488496028633655612;
            }
        }
        10010 => {
            let mut value_0: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_0 < 0 as libc::c_int || value_0 >= (*(*st).mode).nbEBands {
                current_block = 796174441944384681;
            } else {
                (*st).start = value_0;
                current_block = 4488496028633655612;
            }
        }
        10012 => {
            let mut value_1: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_1 < 1 as libc::c_int || value_1 > (*(*st).mode).nbEBands {
                current_block = 796174441944384681;
            } else {
                (*st).end = value_1;
                current_block = 4488496028633655612;
            }
        }
        10002 => {
            let mut value_2: libc::c_int = ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_2 < 0 as libc::c_int || value_2 > 2 as libc::c_int {
                current_block = 796174441944384681;
            } else {
                (*st).disable_pf = (value_2 <= 1 as libc::c_int) as libc::c_int;
                (*st).force_intra = (value_2 == 0 as libc::c_int) as libc::c_int;
                current_block = 4488496028633655612;
            }
        }
        4014 => {
            let mut value_3: libc::c_int = ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_3 < 0 as libc::c_int || value_3 > 100 as libc::c_int {
                current_block = 796174441944384681;
            } else {
                (*st).loss_rate = value_3;
                current_block = 4488496028633655612;
            }
        }
        4020 => {
            let mut value_4: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            (*st).constrained_vbr = value_4;
            current_block = 4488496028633655612;
        }
        4006 => {
            let mut value_5: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            (*st).vbr = value_5;
            current_block = 4488496028633655612;
        }
        4002 => {
            let mut value_6: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_6 <= 500 as libc::c_int && value_6 != -(1 as libc::c_int) {
                current_block = 796174441944384681;
            } else {
                value_6 = if value_6 < 260000 as libc::c_int * (*st).channels {
                    value_6
                } else {
                    (260000 as libc::c_int) * (*st).channels
                };
                (*st).bitrate = value_6;
                current_block = 4488496028633655612;
            }
        }
        10008 => {
            let mut value_7: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_7 < 1 as libc::c_int || value_7 > 2 as libc::c_int {
                current_block = 796174441944384681;
            } else {
                (*st).stream_channels = value_7;
                current_block = 4488496028633655612;
            }
        }
        4036 => {
            let mut value_8: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_8 < 8 as libc::c_int || value_8 > 24 as libc::c_int {
                current_block = 796174441944384681;
            } else {
                (*st).lsb_depth = value_8;
                current_block = 4488496028633655612;
            }
        }
        4037 => {
            let mut value_9: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            *value_9 = (*st).lsb_depth;
            current_block = 4488496028633655612;
        }
        4046 => {
            let mut value_10: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_10 < 0 as libc::c_int || value_10 > 1 as libc::c_int {
                current_block = 796174441944384681;
            } else {
                (*st).disable_inv = value_10;
                current_block = 4488496028633655612;
            }
        }
        4047 => {
            let mut value_11: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_11.is_null() {
                current_block = 796174441944384681;
            } else {
                *value_11 = (*st).disable_inv;
                current_block = 4488496028633655612;
            }
        }
        4028 => {
            let mut i: libc::c_int = 0;
            let mut oldBandE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
            let mut oldLogE: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
            let mut oldLogE2: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
            oldBandE =
                (*st).in_mem.as_mut_ptr().offset(
                    ((*st).channels * ((*(*st).mode).overlap + 1024 as libc::c_int)) as isize,
                ) as *mut crate::arch_h::opus_val16;
            oldLogE = oldBandE.offset(((*st).channels * (*(*st).mode).nbEBands) as isize);
            oldLogE2 = oldLogE.offset(((*st).channels * (*(*st).mode).nbEBands) as isize);
            crate::stdlib::memset(
                &mut (*st).rng as *mut crate::opus_types_h::opus_uint32 as *mut libc::c_char
                    as *mut libc::c_void,
                0 as libc::c_int,
                ((opus_custom_encoder_get_size((*st).mode, (*st).channels) as libc::c_long
                    - (&mut (*st).rng as *mut crate::opus_types_h::opus_uint32 as *mut libc::c_char)
                        .wrapping_offset_from(st as *mut libc::c_char)
                        as libc::c_long) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            i = 0 as libc::c_int;
            while i < (*st).channels * (*(*st).mode).nbEBands {
                let ref mut fresh32 = *oldLogE2.offset(i as isize);
                *fresh32 = -28.0f32;
                *oldLogE.offset(i as isize) = *fresh32;
                i += 1
            }
            (*st).vbr_offset = 0 as libc::c_int;
            (*st).delayedIntra = 1 as libc::c_int as crate::arch_h::opus_val32;
            (*st).spread_decision = 2 as libc::c_int;
            (*st).tonal_average = 256 as libc::c_int;
            (*st).hf_average = 0 as libc::c_int;
            (*st).tapset_decision = 0 as libc::c_int;
            current_block = 4488496028633655612;
        }
        10016 => {
            let mut value_12: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            (*st).signalling = value_12;
            current_block = 4488496028633655612;
        }
        10022 => {
            let mut info: *mut crate::celt_h::AnalysisInfo =
                ap.as_va_list().arg::<*mut crate::celt_h::AnalysisInfo>();
            if !info.is_null() {
                crate::stdlib::memcpy(
                    &mut (*st).analysis as *mut crate::celt_h::AnalysisInfo as *mut libc::c_void,
                    info as *const libc::c_void,
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<crate::celt_h::AnalysisInfo>() as libc::c_ulong
                        )
                        .wrapping_add(
                            (0 as libc::c_int as libc::c_long
                                * (&mut (*st).analysis as *mut crate::celt_h::AnalysisInfo)
                                    .wrapping_offset_from(info)
                                    as libc::c_long) as libc::c_ulong,
                        ),
                );
            }
            current_block = 4488496028633655612;
        }
        10028 => {
            let mut info_0: *mut crate::celt_h::SILKInfo =
                ap.as_va_list().arg::<*mut crate::celt_h::SILKInfo>();
            if !info_0.is_null() {
                crate::stdlib::memcpy(
                    &mut (*st).silk_info as *mut crate::celt_h::SILKInfo as *mut libc::c_void,
                    info_0 as *const libc::c_void,
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<crate::celt_h::SILKInfo>() as libc::c_ulong
                        )
                        .wrapping_add(
                            (0 as libc::c_int as libc::c_long
                                * (&mut (*st).silk_info as *mut crate::celt_h::SILKInfo)
                                    .wrapping_offset_from(info_0)
                                    as libc::c_long) as libc::c_ulong,
                        ),
                );
            }
            current_block = 4488496028633655612;
        }
        10015 => {
            let mut value_13: *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode = ap
                .as_va_list()
                .arg::<*mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode>();
            if value_13.is_null() {
                current_block = 796174441944384681;
            } else {
                *value_13 = (*st).mode;
                current_block = 4488496028633655612;
            }
        }
        4031 => {
            let mut value_14: *mut crate::opus_types_h::opus_uint32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_uint32>();
            if value_14.is_null() {
                current_block = 796174441944384681;
            } else {
                *value_14 = (*st).rng;
                current_block = 4488496028633655612;
            }
        }
        10024 => {
            let mut value_15: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            (*st).lfe = value_15;
            current_block = 4488496028633655612;
        }
        10026 => {
            let mut value_16: *mut crate::arch_h::opus_val16 =
                ap.as_va_list().arg::<*mut crate::arch_h::opus_val16>();
            (*st).energy_mask = value_16;
            current_block = 4488496028633655612;
        }
        _ => return -(5 as libc::c_int),
    }
    match current_block {
        4488496028633655612 => return 0 as libc::c_int,
        _ => return -(1 as libc::c_int),
    };
}

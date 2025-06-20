use ::libc;

pub mod opus_private_h {

    #[inline]

    pub unsafe extern "C" fn align(mut i: i32) -> i32 {
        let mut alignment: u32 = 8 as usize as u32;
        return (i as u32)
            .wrapping_add(alignment)
            .wrapping_sub(1 as i32 as u32)
            .wrapping_div(alignment)
            .wrapping_mul(alignment) as i32;
    }

    /* OPUS_PRIVATE_H */
}

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

    /* Note: This assumes radix-2 floating point with the exponent at bits 23..30 and an offset of 127
    denorm, +/- inf and NaN are *not* handled */
    /* * Base-2 log approximation (log2(x)). */
    #[inline]

    pub unsafe extern "C" fn celt_log2(mut x: f32) -> f32 {
        let mut integer: i32 = 0;
        let mut frac: f32 = 0.;
        let mut in_0: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        in_0.f = x;
        integer = (in_0.i >> 23 as i32).wrapping_sub(127 as i32 as u32) as i32;
        in_0.i = (in_0.i as u32).wrapping_sub((integer << 23 as i32) as u32)
            as crate::opus_types_h::opus_uint32;
        frac = in_0.f - 1.5f32;
        frac = -0.41445418f32
            + frac * (0.95909232f32 + frac * (-0.33951290f32 + frac * 0.16541097f32));
        return (1 as i32 + integer) as f32 + frac;
    }

    /* MATHOPS_H */
    /* FIXED_POINT */
}

pub mod os_support_h {
    /* Copyright (C) 2007 Jean-Marc Valin

       File: os_support.h
       This is the (tiny) OS abstraction layer. Aside from math.h, this is the
       only place where system headers are allowed.

       Redistribution and use in source and binary forms, with or without
       modification, are permitted provided that the following conditions are
       met:

       1. Redistributions of source code must retain the above copyright notice,
       this list of conditions and the following disclaimer.

       2. Redistributions in binary form must reproduce the above copyright
       notice, this list of conditions and the following disclaimer in the
       documentation and/or other materials provided with the distribution.

       THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
       IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
       OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
       DISCLAIMED. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT,
       INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
       (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
       SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
       HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
       STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
       ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
       POSSIBILITY OF SUCH DAMAGE.
    */
    /* * Opus wrapper for malloc(). To do your own dynamic allocation, all you need to do is replace this function and opus_free */
    /* * Same as celt_alloc(), except that the area is only needed inside a CELT call (might cause problem with wideband though) */
    /* Scratch space doesn't need to be cleared */
    /* * Opus wrapper for free(). To do your own dynamic allocation, all you need to do is replace this function and opus_alloc */
    #[inline]

    pub unsafe extern "C" fn opus_free(mut ptr: *mut libc::c_void) {
        libc::free(ptr);
    }
    #[inline]

    pub unsafe extern "C" fn opus_alloc(mut size: crate::stddef_h::size_t) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }

    /* OS_SUPPORT_H */
    /*#ifdef __GNUC__
    #pragma GCC poison printf sprintf
    #pragma GCC poison malloc free realloc calloc
    #endif*/
    /* * Set n elements of dst to zero */
    /* * Copy n elements from src to dst, allowing overlapping regions. The 0* term
    provides compile-time type checking */
    /* * Copy n elements from src to dst. The 0* term provides compile-time type checking  */
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

pub mod pitch_h {
    /*We make sure a C version is always available for cases where the overhead of
    vectorization and passing around an arch flag aren't worth it.*/
    #[inline]

    pub unsafe extern "C" fn celt_inner_prod_c(
        mut x: *const crate::arch_h::opus_val16,
        mut y: *const crate::arch_h::opus_val16,
        mut N: i32,
    ) -> crate::arch_h::opus_val32 {
        let mut i: i32 = 0;
        let mut xy: crate::arch_h::opus_val32 = 0 as i32 as crate::arch_h::opus_val32;
        i = 0 as i32;
        while i < N {
            xy = xy + *x.offset(i as isize) * *y.offset(i as isize);
            i += 1
        }
        return xy;
    }
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::celt_ener;
pub use crate::arch_h::celt_sig;
pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::mathops_h::C2RustUnnamed_61;
pub use crate::opus_private_h::downmix_func;
pub use crate::opus_private_h::foo;
pub use crate::opus_private_h::C2RustUnnamed_98;
pub use crate::opus_private_h::ChannelLayout;
pub use crate::opus_private_h::OpusRepacketizer;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::src::opus_encoder::downmix_float;
pub use crate::src::opus_1_2_1::src::opus_encoder::downmix_int;
pub use crate::src::opus_1_2_1::src::opus_encoder::frame_size_select;
pub use crate::src::opus_1_2_1::src::opus_encoder::opus_encode_native;

pub use crate::src::opus_1_2_1::src::opus_multistream::get_left_channel;
pub use crate::src::opus_1_2_1::src::opus_multistream::get_mono_channel;
pub use crate::src::opus_1_2_1::src::opus_multistream::get_right_channel;
pub use crate::src::opus_1_2_1::src::opus_multistream::validate_layout;
pub use crate::src::opus_1_2_1::src::opus_multistream_encoder::arch_h::celt_isnan;
pub use crate::src::opus_1_2_1::src::opus_multistream_encoder::opus_private_h::align;

pub use crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_out_range_impl;
pub use crate::stddef_h::size_t;

pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::mdct::clt_mdct_forward_c;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;
pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;
pub use crate::src::opus_1_2_1::src::opus_multistream_encoder::mathops_h::celt_log2;
pub use crate::stdarg_h::va_list;

pub use crate::src::opus_1_2_1::src::opus_multistream_encoder::cpu_support_h::opus_select_arch;
pub use crate::src::opus_1_2_1::src::opus_multistream_encoder::os_support_h::opus_alloc;
pub use crate::src::opus_1_2_1::src::opus_multistream_encoder::os_support_h::opus_free;
pub use crate::src::opus_1_2_1::src::opus_multistream_encoder::pitch_h::celt_inner_prod_c;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusMSEncoder {
    pub layout: ChannelLayout,
    pub arch: i32,
    pub lfe_stream: i32,
    pub application: i32,
    pub variable_duration: i32,
    pub mapping_type: MappingType,
    pub bitrate_bps: opus_int32,
}

pub type MappingType = u32;

pub const MAPPING_TYPE_SURROUND: MappingType = 1;

pub const MAPPING_TYPE_NONE: MappingType = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VorbisLayout {
    pub nb_streams: i32,
    pub nb_coupled_streams: i32,
    pub mapping: [u8; 8],
}

pub type opus_copy_channel_in_func = Option<
    unsafe extern "C" fn(
        _: *mut opus_val16,
        _: i32,
        _: *const libc::c_void,
        _: i32,
        _: i32,
        _: i32,
    ) -> (),
>;
/* Index is nb_channel-1*/

static mut vorbis_mappings: [VorbisLayout; 8] = [
    {
        let mut init = VorbisLayout {
            nb_streams: 1 as i32,
            nb_coupled_streams: 0 as i32,
            mapping: [0 as i32 as u8, 0, 0, 0, 0, 0, 0, 0],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 1 as i32,
            nb_coupled_streams: 1 as i32,
            mapping: [0 as i32 as u8, 1 as i32 as u8, 0, 0, 0, 0, 0, 0],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 2 as i32,
            nb_coupled_streams: 1 as i32,
            mapping: [
                0 as i32 as u8,
                2 as i32 as u8,
                1 as i32 as u8,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 2 as i32,
            nb_coupled_streams: 2 as i32,
            mapping: [
                0 as i32 as u8,
                1 as i32 as u8,
                2 as i32 as u8,
                3 as i32 as u8,
                0,
                0,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 3 as i32,
            nb_coupled_streams: 2 as i32,
            mapping: [
                0 as i32 as u8,
                4 as i32 as u8,
                1 as i32 as u8,
                2 as i32 as u8,
                3 as i32 as u8,
                0,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 4 as i32,
            nb_coupled_streams: 2 as i32,
            mapping: [
                0 as i32 as u8,
                4 as i32 as u8,
                1 as i32 as u8,
                2 as i32 as u8,
                3 as i32 as u8,
                5 as i32 as u8,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 4 as i32,
            nb_coupled_streams: 3 as i32,
            mapping: [
                0 as i32 as u8,
                4 as i32 as u8,
                1 as i32 as u8,
                2 as i32 as u8,
                3 as i32 as u8,
                5 as i32 as u8,
                6 as i32 as u8,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 5 as i32,
            nb_coupled_streams: 3 as i32,
            mapping: [
                0 as i32 as u8,
                6 as i32 as u8,
                1 as i32 as u8,
                2 as i32 as u8,
                3 as i32 as u8,
                4 as i32 as u8,
                5 as i32 as u8,
                7 as i32 as u8,
            ],
        };
        init
    },
];
/* Encoder states go here */
/* then opus_val32 window_mem[channels*120]; */
/* then opus_val32 preemph_mem[channels]; */

unsafe extern "C" fn ms_get_preemph_mem(mut st: *mut OpusMSEncoder) -> *mut opus_val32 {
    let mut s: i32 = 0;
    let mut ptr: *mut libc::c_char = std::ptr::null_mut();
    let mut coupled_size: i32 = 0;
    let mut mono_size: i32 = 0;
    coupled_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(2 as i32);
    mono_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(1 as i32);
    ptr = (st as *mut libc::c_char)
        .offset(align(::std::mem::size_of::<OpusMSEncoder>() as usize as i32) as isize);
    s = 0 as i32;
    while s < (*st).layout.nb_streams {
        if s < (*st).layout.nb_coupled_streams {
            ptr = ptr.offset(align(coupled_size) as isize)
        } else {
            ptr = ptr.offset(align(mono_size) as isize)
        }
        s += 1
    }
    /* void* cast avoids clang -Wcast-align warning */
    return ptr.offset(
        (((*st).layout.nb_channels * 120 as i32) as usize)
            .wrapping_mul(::std::mem::size_of::<opus_val32>() as usize) as isize,
    ) as *mut libc::c_void as *mut opus_val32;
}

unsafe extern "C" fn ms_get_window_mem(mut st: *mut OpusMSEncoder) -> *mut opus_val32 {
    let mut s: i32 = 0;
    let mut ptr: *mut libc::c_char = std::ptr::null_mut();
    let mut coupled_size: i32 = 0;
    let mut mono_size: i32 = 0;
    coupled_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(2 as i32);
    mono_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(1 as i32);
    ptr = (st as *mut libc::c_char)
        .offset(align(::std::mem::size_of::<OpusMSEncoder>() as usize as i32) as isize);
    s = 0 as i32;
    while s < (*st).layout.nb_streams {
        if s < (*st).layout.nb_coupled_streams {
            ptr = ptr.offset(align(coupled_size) as isize)
        } else {
            ptr = ptr.offset(align(mono_size) as isize)
        }
        s += 1
    }
    /* void* cast avoids clang -Wcast-align warning */
    return ptr as *mut libc::c_void as *mut opus_val32;
}

unsafe extern "C" fn validate_encoder_layout(mut layout: *const ChannelLayout) -> i32 {
    let mut s: i32 = 0;
    s = 0 as i32;
    while s < (*layout).nb_streams {
        if s < (*layout).nb_coupled_streams {
            if get_left_channel(layout as *const ChannelLayout, s, -(1 as i32)) == -(1 as i32) {
                return 0 as i32;
            }
            if get_right_channel(layout as *const ChannelLayout, s, -(1 as i32)) == -(1 as i32) {
                return 0 as i32;
            }
        } else if get_mono_channel(layout as *const ChannelLayout, s, -(1 as i32)) == -(1 as i32) {
            return 0 as i32;
        }
        s += 1
    }
    return 1 as i32;
}

unsafe extern "C" fn channel_pos(mut channels: i32, mut pos: *mut i32) {
    /* Position in the mix: 0 don't mix, 1: left, 2: center, 3:right */
    if channels == 4 as i32 {
        *pos.offset(0 as i32 as isize) = 1 as i32;
        *pos.offset(1 as i32 as isize) = 3 as i32;
        *pos.offset(2 as i32 as isize) = 1 as i32;
        *pos.offset(3 as i32 as isize) = 3 as i32
    } else if channels == 3 as i32 || channels == 5 as i32 || channels == 6 as i32 {
        *pos.offset(0 as i32 as isize) = 1 as i32;
        *pos.offset(1 as i32 as isize) = 2 as i32;
        *pos.offset(2 as i32 as isize) = 3 as i32;
        *pos.offset(3 as i32 as isize) = 1 as i32;
        *pos.offset(4 as i32 as isize) = 3 as i32;
        *pos.offset(5 as i32 as isize) = 0 as i32
    } else if channels == 7 as i32 {
        *pos.offset(0 as i32 as isize) = 1 as i32;
        *pos.offset(1 as i32 as isize) = 2 as i32;
        *pos.offset(2 as i32 as isize) = 3 as i32;
        *pos.offset(3 as i32 as isize) = 1 as i32;
        *pos.offset(4 as i32 as isize) = 3 as i32;
        *pos.offset(5 as i32 as isize) = 2 as i32;
        *pos.offset(6 as i32 as isize) = 0 as i32
    } else if channels == 8 as i32 {
        *pos.offset(0 as i32 as isize) = 1 as i32;
        *pos.offset(1 as i32 as isize) = 2 as i32;
        *pos.offset(2 as i32 as isize) = 3 as i32;
        *pos.offset(3 as i32 as isize) = 1 as i32;
        *pos.offset(4 as i32 as isize) = 3 as i32;
        *pos.offset(5 as i32 as isize) = 1 as i32;
        *pos.offset(6 as i32 as isize) = 3 as i32;
        *pos.offset(7 as i32 as isize) = 0 as i32
    };
}
/* Computes a rough approximation of log2(2^a + 2^b) */

unsafe extern "C" fn logSum(mut a: opus_val16, mut b: opus_val16) -> opus_val16 {
    let mut max: opus_val16 = 0.;
    let mut diff: opus_val32 = 0.;
    let mut frac: opus_val16 = 0.;
    static mut diff_table: [opus_val16; 17] = [
        0.5000000f32,
        0.2924813f32,
        0.1609640f32,
        0.0849625f32,
        0.0437314f32,
        0.0221971f32,
        0.0111839f32,
        0.0056136f32,
        0.0028123f32,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut low: i32 = 0;
    if a > b {
        max = a;
        diff = a - b
    } else {
        max = b;
        diff = b - a
    }
    if !(diff < 8.0f32) {
        /* inverted to catch NaNs */
        return max;
    }
    low = crate::stdlib::floor((2 as i32 as f32 * diff) as f64) as i32;
    frac = 2 as i32 as f32 * diff - low as f32;
    return max
        + diff_table[low as usize]
        + frac * (diff_table[(low + 1 as i32) as usize] - diff_table[low as usize]);
}
#[no_mangle]

pub unsafe extern "C" fn surround_analysis(
    mut celt_mode: *const OpusCustomMode,
    mut pcm: *const libc::c_void,
    mut bandLogE: *mut opus_val16,
    mut mem: *mut opus_val32,
    mut preemph_mem: *mut opus_val32,
    mut len: i32,
    mut overlap: i32,
    mut channels: i32,
    mut rate: i32,
    mut copy_channel_in: opus_copy_channel_in_func,
    mut arch: i32,
) {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    let mut LM: i32 = 0;
    let mut pos: [i32; 8] = [0 as i32, 0, 0, 0, 0, 0, 0, 0];
    let mut upsample: i32 = 0;
    let mut frame_size: i32 = 0;
    let mut freq_size: i32 = 0;
    let mut channel_offset: opus_val16 = 0.;
    let mut bandE: [opus_val32; 21] = [0.; 21];
    let mut maskLogE: [[opus_val16; 21]; 3] = [[0.; 21]; 3];
    let mut in_0: *mut opus_val32 = std::ptr::null_mut();
    let mut x: *mut opus_val16 = std::ptr::null_mut();
    let mut freq: *mut opus_val32 = std::ptr::null_mut();
    upsample = crate::src::opus_1_2_1::celt::celt::resampling_factor(rate);
    frame_size = len * upsample;
    freq_size = if (960 as i32) < frame_size {
        960 as i32
    } else {
        frame_size
    };
    /* LM = log2(frame_size / 120) */
    LM = 0 as i32;
    while LM < (*celt_mode).maxLM {
        if (*celt_mode).shortMdctSize << LM == frame_size {
            break;
        }
        LM += 1
    }
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_val32>() as usize).wrapping_mul((frame_size + overlap) as usize)
            as usize,
    );
    in_0 = fresh0.as_mut_ptr() as *mut opus_val32;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_val16>() as usize).wrapping_mul(len as usize) as usize,
    );
    x = fresh1.as_mut_ptr() as *mut opus_val16;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_val32>() as usize).wrapping_mul(freq_size as usize) as usize,
    );
    freq = fresh2.as_mut_ptr() as *mut opus_val32;
    channel_pos(channels, pos.as_mut_ptr());
    c = 0 as i32;
    while c < 3 as i32 {
        i = 0 as i32;
        while i < 21 as i32 {
            maskLogE[c as usize][i as usize] = -28.0f32;
            i += 1
        }
        c += 1
    }
    c = 0 as i32;
    while c < channels {
        let mut frame: i32 = 0;
        let mut nb_frames: i32 = frame_size / freq_size;
        crate::stdlib::memcpy(
            in_0 as *mut libc::c_void,
            mem.offset((c * overlap) as isize) as *const libc::c_void,
            (overlap as usize)
                .wrapping_mul(::std::mem::size_of::<opus_val32>() as usize)
                .wrapping_add(
                    (0 as i32 as isize
                        * in_0.offset_from(mem.offset((c * overlap) as isize)) as isize)
                        as usize,
                ),
        );
        Some(copy_channel_in.expect("non-null function pointer"))
            .expect("non-null function pointer")(x, 1 as i32, pcm, channels, c, len);
        crate::src::opus_1_2_1::celt::celt_encoder::celt_preemphasis(
            x,
            in_0.offset(overlap as isize),
            frame_size,
            1 as i32,
            upsample,
            (*celt_mode).preemph.as_ptr(),
            preemph_mem.offset(c as isize),
            0 as i32,
        );
        let mut sum: opus_val32 = 0.;
        sum = celt_inner_prod_c(in_0, in_0, frame_size + overlap);
        /* This should filter out both NaNs and ridiculous signals that could
        cause NaNs further down. */
        if !(sum < 1e18f32) || celt_isnan(sum) != 0 {
            crate::stdlib::memset(
                in_0 as *mut libc::c_void,
                0 as i32,
                ((frame_size + overlap) as usize)
                    .wrapping_mul(::std::mem::size_of::<opus_val32>() as usize),
            );
            *preemph_mem.offset(c as isize) = 0 as i32 as opus_val32
        }
        crate::stdlib::memset(
            bandE.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            (21 as i32 as usize).wrapping_mul(::std::mem::size_of::<opus_val32>() as usize),
        );
        frame = 0 as i32;
        while frame < nb_frames {
            let mut tmpE: [opus_val32; 21] = [0.; 21];
            clt_mdct_forward_c(
                &(*celt_mode).mdct as *const _ as *const mdct_lookup,
                in_0.offset((960 as i32 * frame) as isize),
                freq,
                (*celt_mode).window,
                overlap,
                (*celt_mode).maxLM - LM,
                1 as i32,
                arch,
            );
            if upsample != 1 as i32 {
                let mut bound: i32 = freq_size / upsample;
                i = 0 as i32;
                while i < bound {
                    let ref mut fresh3 = *freq.offset(i as isize);
                    *fresh3 *= upsample as f32;
                    i += 1
                }
                while i < freq_size {
                    *freq.offset(i as isize) = 0 as i32 as opus_val32;
                    i += 1
                }
            }
            crate::src::opus_1_2_1::celt::bands::compute_band_energies(
                celt_mode as *const OpusCustomMode,
                freq,
                tmpE.as_mut_ptr(),
                21 as i32,
                1 as i32,
                LM,
                arch,
            );
            /* If we have multiple frames, take the max energy. */
            i = 0 as i32;
            while i < 21 as i32 {
                bandE[i as usize] = if bandE[i as usize] > tmpE[i as usize] {
                    bandE[i as usize]
                } else {
                    tmpE[i as usize]
                };
                i += 1
            }
            frame += 1
        }
        crate::src::opus_1_2_1::celt::quant_bands::amp2Log2(
            celt_mode as *const OpusCustomMode,
            21 as i32,
            21 as i32,
            bandE.as_mut_ptr(),
            bandLogE.offset((21 as i32 * c) as isize),
            1 as i32,
        );
        /* Apply spreading function with -6 dB/band going up and -12 dB/band going down. */
        i = 1 as i32; /* Q8 */
        while i < 21 as i32 {
            *bandLogE.offset((21 as i32 * c + i) as isize) = if *bandLogE
                .offset((21 as i32 * c + i) as isize)
                > *bandLogE.offset((21 as i32 * c + i - 1 as i32) as isize) - 1.0f32
            {
                *bandLogE.offset((21 as i32 * c + i) as isize)
            } else {
                (*bandLogE.offset((21 as i32 * c + i - 1 as i32) as isize)) - 1.0f32
            }; /* Q8 */
            i += 1
        }
        i = 19 as i32;
        while i >= 0 as i32 {
            *bandLogE.offset((21 as i32 * c + i) as isize) = if *bandLogE
                .offset((21 as i32 * c + i) as isize)
                > *bandLogE.offset((21 as i32 * c + i + 1 as i32) as isize) - 2.0f32
            {
                *bandLogE.offset((21 as i32 * c + i) as isize)
            } else {
                (*bandLogE.offset((21 as i32 * c + i + 1 as i32) as isize)) - 2.0f32
            };
            i -= 1
        }
        if pos[c as usize] == 1 as i32 {
            i = 0 as i32;
            while i < 21 as i32 {
                maskLogE[0 as i32 as usize][i as usize] = logSum(
                    maskLogE[0 as i32 as usize][i as usize],
                    *bandLogE.offset((21 as i32 * c + i) as isize),
                );
                i += 1
            }
        } else if pos[c as usize] == 3 as i32 {
            i = 0 as i32;
            while i < 21 as i32 {
                maskLogE[2 as i32 as usize][i as usize] = logSum(
                    maskLogE[2 as i32 as usize][i as usize],
                    *bandLogE.offset((21 as i32 * c + i) as isize),
                );
                i += 1
            }
        } else if pos[c as usize] == 2 as i32 {
            i = 0 as i32;
            while i < 21 as i32 {
                maskLogE[0 as i32 as usize][i as usize] = logSum(
                    maskLogE[0 as i32 as usize][i as usize],
                    *bandLogE.offset((21 as i32 * c + i) as isize) - 0.5f32,
                );
                maskLogE[2 as i32 as usize][i as usize] = logSum(
                    maskLogE[2 as i32 as usize][i as usize],
                    *bandLogE.offset((21 as i32 * c + i) as isize) - 0.5f32,
                );
                i += 1
            }
        }
        crate::stdlib::memcpy(
            mem.offset((c * overlap) as isize) as *mut libc::c_void,
            in_0.offset(frame_size as isize) as *const libc::c_void,
            (overlap as usize)
                .wrapping_mul(::std::mem::size_of::<opus_val32>() as usize)
                .wrapping_add(
                    (0 as i32 as isize
                        * mem
                            .offset((c * overlap) as isize)
                            .offset_from(in_0.offset(frame_size as isize))
                            as isize) as usize,
                ),
        );
        c += 1
    }
    i = 0 as i32;
    while i < 21 as i32 {
        maskLogE[1 as i32 as usize][i as usize] =
            if maskLogE[0 as i32 as usize][i as usize] < maskLogE[2 as i32 as usize][i as usize] {
                maskLogE[0 as i32 as usize][i as usize]
            } else {
                maskLogE[2 as i32 as usize][i as usize]
            };
        i += 1
    }
    channel_offset = 0.5f32 * celt_log2(2.0f32 / (channels - 1 as i32) as f32);
    c = 0 as i32;
    while c < 3 as i32 {
        i = 0 as i32;
        while i < 21 as i32 {
            maskLogE[c as usize][i as usize] += channel_offset;
            i += 1
        }
        c += 1
    }
    c = 0 as i32;
    while c < channels {
        let mut mask: *mut opus_val16 = std::ptr::null_mut();
        if pos[c as usize] != 0 as i32 {
            mask = &mut *(*maskLogE
                .as_mut_ptr()
                .offset((*pos.as_mut_ptr().offset(c as isize) - 1 as i32) as isize))
            .as_mut_ptr()
            .offset(0 as i32 as isize) as *mut opus_val16;
            i = 0 as i32;
            while i < 21 as i32 {
                *bandLogE.offset((21 as i32 * c + i) as isize) =
                    *bandLogE.offset((21 as i32 * c + i) as isize) - *mask.offset(i as isize);
                i += 1
            }
        } else {
            i = 0 as i32;
            while i < 21 as i32 {
                *bandLogE.offset((21 as i32 * c + i) as isize) = 0 as i32 as opus_val16;
                i += 1
            }
        }
        c += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encoder_get_size(
    mut nb_streams: i32,
    mut nb_coupled_streams: i32,
) -> opus_int32 {
    let mut coupled_size: i32 = 0;
    let mut mono_size: i32 = 0;
    if nb_streams < 1 as i32 || nb_coupled_streams > nb_streams || nb_coupled_streams < 0 as i32 {
        return 0 as i32;
    }
    coupled_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(2 as i32);
    mono_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(1 as i32);
    return align(::std::mem::size_of::<OpusMSEncoder>() as usize as i32)
        + nb_coupled_streams * align(coupled_size)
        + (nb_streams - nb_coupled_streams) * align(mono_size);
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_surround_encoder_get_size(
    mut channels: i32,
    mut mapping_family: i32,
) -> opus_int32 {
    let mut nb_streams: i32 = 0;
    let mut nb_coupled_streams: i32 = 0;
    let mut size: opus_int32 = 0;
    if mapping_family == 0 as i32 {
        if channels == 1 as i32 {
            nb_streams = 1 as i32;
            nb_coupled_streams = 0 as i32
        } else if channels == 2 as i32 {
            nb_streams = 1 as i32;
            nb_coupled_streams = 1 as i32
        } else {
            return 0 as i32;
        }
    } else if mapping_family == 1 as i32 && channels <= 8 as i32 && channels >= 1 as i32 {
        nb_streams = vorbis_mappings[(channels - 1 as i32) as usize].nb_streams;
        nb_coupled_streams = vorbis_mappings[(channels - 1 as i32) as usize].nb_coupled_streams
    } else if mapping_family == 255 as i32 {
        nb_streams = channels;
        nb_coupled_streams = 0 as i32
    } else {
        return 0 as i32;
    }
    size = opus_multistream_encoder_get_size(nb_streams, nb_coupled_streams);
    if channels > 2 as i32 {
        size = (size as usize).wrapping_add(
            (channels as usize).wrapping_mul(
                (120 as i32 as usize)
                    .wrapping_mul(::std::mem::size_of::<opus_val32>() as usize)
                    .wrapping_add(::std::mem::size_of::<opus_val32>() as usize),
            ),
        ) as opus_int32
    }
    return size;
}

unsafe extern "C" fn opus_multistream_encoder_init_impl(
    mut st: *mut OpusMSEncoder,
    mut Fs: opus_int32,
    mut channels: i32,
    mut streams: i32,
    mut coupled_streams: i32,
    mut mapping: *const u8,
    mut application: i32,
    mut mapping_type: MappingType,
) -> i32 {
    let mut coupled_size: i32 = 0;
    let mut mono_size: i32 = 0;
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    let mut ptr: *mut libc::c_char = std::ptr::null_mut();
    if channels > 255 as i32
        || channels < 1 as i32
        || coupled_streams > streams
        || streams < 1 as i32
        || coupled_streams < 0 as i32
        || streams > 255 as i32 - coupled_streams
    {
        return -(1 as i32);
    }
    (*st).arch = opus_select_arch();
    (*st).layout.nb_channels = channels;
    (*st).layout.nb_streams = streams;
    (*st).layout.nb_coupled_streams = coupled_streams;
    if mapping_type as u32 != MAPPING_TYPE_SURROUND as i32 as u32 {
        (*st).lfe_stream = -(1 as i32)
    }
    (*st).bitrate_bps = -(1000 as i32);
    (*st).application = application;
    (*st).variable_duration = 5000 as i32;
    i = 0 as i32;
    while i < (*st).layout.nb_channels {
        (*st).layout.mapping[i as usize] = *mapping.offset(i as isize);
        i += 1
    }
    if validate_layout(&mut (*st).layout as *mut _ as *const ChannelLayout) == 0 {
        return -(1 as i32);
    }
    if mapping_type as u32 == MAPPING_TYPE_SURROUND as i32 as u32
        && validate_encoder_layout(&mut (*st).layout) == 0
    {
        return -(1 as i32);
    }
    ptr = (st as *mut libc::c_char)
        .offset(align(::std::mem::size_of::<OpusMSEncoder>() as usize as i32) as isize);
    coupled_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(2 as i32);
    mono_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(1 as i32);
    i = 0 as i32;
    while i < (*st).layout.nb_coupled_streams {
        ret = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_init(
            ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
            Fs,
            2 as i32,
            application,
        );
        if ret != 0 as i32 {
            return ret;
        }
        if i == (*st).lfe_stream {
            crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
                10024 as i32,
                1 as i32,
            );
        }
        ptr = ptr.offset(align(coupled_size) as isize);
        i += 1
    }
    while i < (*st).layout.nb_streams {
        ret = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_init(
            ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
            Fs,
            1 as i32,
            application,
        );
        if i == (*st).lfe_stream {
            crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
                10024 as i32,
                1 as i32,
            );
        }
        if ret != 0 as i32 {
            return ret;
        }
        ptr = ptr.offset(align(mono_size) as isize);
        i += 1
    }
    if mapping_type as u32 == MAPPING_TYPE_SURROUND as i32 as u32 {
        crate::stdlib::memset(
            ms_get_preemph_mem(st) as *mut libc::c_void,
            0 as i32,
            (channels as usize).wrapping_mul(::std::mem::size_of::<opus_val32>() as usize),
        );
        crate::stdlib::memset(
            ms_get_window_mem(st) as *mut libc::c_void,
            0 as i32,
            ((channels * 120 as i32) as usize)
                .wrapping_mul(::std::mem::size_of::<opus_val32>() as usize),
        );
    }
    (*st).mapping_type = mapping_type;
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encoder_init(
    mut st: *mut OpusMSEncoder,
    mut Fs: opus_int32,
    mut channels: i32,
    mut streams: i32,
    mut coupled_streams: i32,
    mut mapping: *const u8,
    mut application: i32,
) -> i32 {
    return opus_multistream_encoder_init_impl(
        st,
        Fs,
        channels,
        streams,
        coupled_streams,
        mapping,
        application,
        MAPPING_TYPE_NONE,
    );
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_surround_encoder_init(
    mut st: *mut OpusMSEncoder,
    mut Fs: opus_int32,
    mut channels: i32,
    mut mapping_family: i32,
    mut streams: *mut i32,
    mut coupled_streams: *mut i32,
    mut mapping: *mut u8,
    mut application: i32,
) -> i32 {
    let mut mapping_type: MappingType = MAPPING_TYPE_NONE;
    if channels > 255 as i32 || channels < 1 as i32 {
        return -(1 as i32);
    }
    (*st).lfe_stream = -(1 as i32);
    if mapping_family == 0 as i32 {
        if channels == 1 as i32 {
            *streams = 1 as i32;
            *coupled_streams = 0 as i32;
            *mapping.offset(0 as i32 as isize) = 0 as i32 as u8
        } else if channels == 2 as i32 {
            *streams = 1 as i32;
            *coupled_streams = 1 as i32;
            *mapping.offset(0 as i32 as isize) = 0 as i32 as u8;
            *mapping.offset(1 as i32 as isize) = 1 as i32 as u8
        } else {
            return -(5 as i32);
        }
    } else if mapping_family == 1 as i32 && channels <= 8 as i32 && channels >= 1 as i32 {
        let mut i: i32 = 0;
        *streams = vorbis_mappings[(channels - 1 as i32) as usize].nb_streams;
        *coupled_streams = vorbis_mappings[(channels - 1 as i32) as usize].nb_coupled_streams;
        i = 0 as i32;
        while i < channels {
            *mapping.offset(i as isize) =
                vorbis_mappings[(channels - 1 as i32) as usize].mapping[i as usize];
            i += 1
        }
        if channels >= 6 as i32 {
            (*st).lfe_stream = *streams - 1 as i32
        }
    } else if mapping_family == 255 as i32 {
        let mut i_0: i32 = 0;
        *streams = channels;
        *coupled_streams = 0 as i32;
        i_0 = 0 as i32;
        while i_0 < channels {
            *mapping.offset(i_0 as isize) = i_0 as u8;
            i_0 += 1
        }
    } else {
        return -(5 as i32);
    }
    if channels > 2 as i32 && mapping_family == 1 as i32 {
        mapping_type = MAPPING_TYPE_SURROUND
    } else {
        mapping_type = MAPPING_TYPE_NONE
    }
    return opus_multistream_encoder_init_impl(
        st,
        Fs,
        channels,
        *streams,
        *coupled_streams,
        mapping,
        application,
        mapping_type,
    );
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encoder_create(
    mut Fs: opus_int32,
    mut channels: i32,
    mut streams: i32,
    mut coupled_streams: i32,
    mut mapping: *const u8,
    mut application: i32,
    mut error: *mut i32,
) -> *mut OpusMSEncoder {
    let mut ret: i32 = 0;
    let mut st: *mut OpusMSEncoder = std::ptr::null_mut();
    if channels > 255 as i32
        || channels < 1 as i32
        || coupled_streams > streams
        || streams < 1 as i32
        || coupled_streams < 0 as i32
        || streams > 255 as i32 - coupled_streams
    {
        if !error.is_null() {
            *error = -(1 as i32)
        }
        return std::ptr::null_mut();
    }
    st = opus_alloc(opus_multistream_encoder_get_size(streams, coupled_streams) as size_t)
        as *mut OpusMSEncoder;
    if st.is_null() {
        if !error.is_null() {
            *error = -(7 as i32)
        }
        return std::ptr::null_mut();
    }
    ret = opus_multistream_encoder_init(
        st,
        Fs,
        channels,
        streams,
        coupled_streams,
        mapping,
        application,
    );
    if ret != 0 as i32 {
        opus_free(st as *mut libc::c_void);
        st = std::ptr::null_mut()
    }
    if !error.is_null() {
        *error = ret
    }
    return st;
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_surround_encoder_create(
    mut Fs: opus_int32,
    mut channels: i32,
    mut mapping_family: i32,
    mut streams: *mut i32,
    mut coupled_streams: *mut i32,
    mut mapping: *mut u8,
    mut application: i32,
    mut error: *mut i32,
) -> *mut OpusMSEncoder {
    let mut ret: i32 = 0;
    let mut size: opus_int32 = 0;
    let mut st: *mut OpusMSEncoder = std::ptr::null_mut();
    if channels > 255 as i32 || channels < 1 as i32 {
        if !error.is_null() {
            *error = -(1 as i32)
        }
        return std::ptr::null_mut();
    }
    size = opus_multistream_surround_encoder_get_size(channels, mapping_family);
    if size == 0 {
        if !error.is_null() {
            *error = -(5 as i32)
        }
        return std::ptr::null_mut();
    }
    st = opus_alloc(size as size_t) as *mut OpusMSEncoder;
    if st.is_null() {
        if !error.is_null() {
            *error = -(7 as i32)
        }
        return std::ptr::null_mut();
    }
    ret = opus_multistream_surround_encoder_init(
        st,
        Fs,
        channels,
        mapping_family,
        streams,
        coupled_streams,
        mapping,
        application,
    );
    if ret != 0 as i32 {
        opus_free(st as *mut libc::c_void);
        st = std::ptr::null_mut()
    }
    if !error.is_null() {
        *error = ret
    }
    return st;
}

unsafe extern "C" fn surround_rate_allocation(
    mut st: *mut OpusMSEncoder,
    mut rate: *mut opus_int32,
    mut frame_size: i32,
    mut Fs: opus_int32,
) {
    let mut i: i32 = 0;
    let mut channel_rate: opus_int32 = 0;
    let mut stream_offset: i32 = 0;
    let mut lfe_offset: i32 = 0;
    let mut coupled_ratio: i32 = 0;
    let mut lfe_ratio: i32 = 0;
    let mut nb_lfe: i32 = 0;
    let mut nb_uncoupled: i32 = 0;
    let mut nb_coupled: i32 = 0;
    let mut nb_normal: i32 = 0;
    let mut channel_offset: opus_int32 = 0;
    let mut bitrate: opus_int32 = 0;
    let mut total: i32 = 0;
    nb_lfe = ((*st).lfe_stream != -(1 as i32)) as i32;
    nb_coupled = (*st).layout.nb_coupled_streams;
    nb_uncoupled = (*st).layout.nb_streams - nb_coupled - nb_lfe;
    nb_normal = 2 as i32 * nb_coupled + nb_uncoupled;
    /* Give each non-LFE channel enough bits per channel for coding band energy. */
    channel_offset = 40 as i32
        * (if 50 as i32 > Fs / frame_size {
            50 as i32
        } else {
            (Fs) / frame_size
        });
    if (*st).bitrate_bps == -(1000 as i32) {
        bitrate = nb_normal * (channel_offset + Fs + 10000 as i32) + 8000 as i32 * nb_lfe
    } else if (*st).bitrate_bps == -(1 as i32) {
        bitrate = nb_normal * 300000 as i32 + nb_lfe * 128000 as i32
    } else {
        bitrate = (*st).bitrate_bps
    }
    /* Give LFE some basic stream_channel allocation but never exceed 1/20 of the
    total rate for the non-energy part to avoid problems at really low rate. */
    lfe_offset = (if (bitrate / 20 as i32) < 3000 as i32 {
        (bitrate) / 20 as i32
    } else {
        3000 as i32
    }) + 15 as i32
        * (if 50 as i32 > Fs / frame_size {
            50 as i32
        } else {
            (Fs) / frame_size
        });
    /* We give each stream (coupled or uncoupled) a starting bitrate.
    This models the main saving of coupled channels over uncoupled. */
    stream_offset =
        (bitrate - channel_offset * nb_normal - lfe_offset * nb_lfe) / nb_normal / 2 as i32;
    stream_offset = if 0 as i32
        > (if (20000 as i32) < stream_offset {
            20000 as i32
        } else {
            stream_offset
        }) {
        0 as i32
    } else if (20000 as i32) < stream_offset {
        20000 as i32
    } else {
        stream_offset
    };
    /* Coupled streams get twice the mono rate after the offset is allocated. */
    coupled_ratio = 512 as i32;
    /* Should depend on the bitrate, for now we assume LFE gets 1/8 the bits of mono */
    lfe_ratio = 32 as i32;
    total = (nb_uncoupled << 8 as i32) + coupled_ratio * nb_coupled + nb_lfe * lfe_ratio;
    channel_rate = (256 as i32 as i64
        * (bitrate
            - lfe_offset * nb_lfe
            - stream_offset * (nb_coupled + nb_uncoupled)
            - channel_offset * nb_normal) as i64
        / total as i64) as opus_int32;
    i = 0 as i32;
    while i < (*st).layout.nb_streams {
        if i < (*st).layout.nb_coupled_streams {
            *rate.offset(i as isize) = 2 as i32 * channel_offset
                + (if 0 as i32 > stream_offset + (channel_rate * coupled_ratio >> 8 as i32) {
                    0 as i32
                } else {
                    (stream_offset) + (channel_rate * coupled_ratio >> 8 as i32)
                })
        } else if i != (*st).lfe_stream {
            *rate.offset(i as isize) = channel_offset
                + (if 0 as i32 > stream_offset + channel_rate {
                    0 as i32
                } else {
                    (stream_offset) + channel_rate
                })
        } else {
            *rate.offset(i as isize) =
                if 0 as i32 > lfe_offset + (channel_rate * lfe_ratio >> 8 as i32) {
                    0 as i32
                } else {
                    (lfe_offset) + (channel_rate * lfe_ratio >> 8 as i32)
                }
        }
        i += 1
    }
}
/* ENABLE_EXPERIMENTAL_AMBISONICS */

unsafe extern "C" fn rate_allocation(
    mut st: *mut OpusMSEncoder,
    mut rate: *mut opus_int32,
    mut frame_size: i32,
) -> opus_int32 {
    let mut i: i32 = 0;
    let mut rate_sum: opus_int32 = 0 as i32;
    let mut Fs: opus_int32 = 0;
    let mut ptr: *mut libc::c_char = std::ptr::null_mut();
    ptr = (st as *mut libc::c_char)
        .offset(align(::std::mem::size_of::<OpusMSEncoder>() as usize as i32) as isize);
    crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
        ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
        4029 as i32,
        (&mut Fs as *mut opus_int32).offset(
            (&mut Fs as *mut opus_int32).offset_from(&mut Fs as *mut opus_int32) as isize as isize,
        ),
    );
    surround_rate_allocation(st, rate, frame_size, Fs);
    i = 0 as i32;
    while i < (*st).layout.nb_streams {
        *rate.offset(i as isize) = if *rate.offset(i as isize) > 500 as i32 {
            *rate.offset(i as isize)
        } else {
            500 as i32
        };
        rate_sum += *rate.offset(i as isize);
        i += 1
    }
    return rate_sum;
}
/* Max size in case the encoder decides to return six frames (6 x 20 ms = 120 ms) */

unsafe extern "C" fn opus_multistream_encode_native(
    mut st: *mut OpusMSEncoder,
    mut copy_channel_in: opus_copy_channel_in_func,
    mut pcm: *const libc::c_void,
    mut analysis_frame_size: i32,
    mut data: *mut u8,
    mut max_data_bytes: opus_int32,
    mut lsb_depth: i32,
    mut downmix: downmix_func,
    mut float_api: i32,
) -> i32 {
    let mut Fs: opus_int32 = 0;
    let mut coupled_size: i32 = 0;
    let mut mono_size: i32 = 0;
    let mut s: i32 = 0;
    let mut ptr: *mut libc::c_char = std::ptr::null_mut();
    let mut tot_size: i32 = 0;
    let mut buf: *mut opus_val16 = std::ptr::null_mut();
    let mut bandSMR: *mut opus_val16 = std::ptr::null_mut();
    let mut tmp_data: [u8; 7662] = [0; 7662];
    let mut rp: OpusRepacketizer = OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [std::ptr::null(); 48],
        len: [0; 48],
        framesize: 0,
    };
    let mut vbr: opus_int32 = 0;
    let mut celt_mode: *const OpusCustomMode = std::ptr::null();
    let mut bitrates: [opus_int32; 256] = [0; 256];
    let mut bandLogE: [opus_val16; 42] = [0.; 42];
    let mut mem: *mut opus_val32 = std::ptr::null_mut();
    let mut preemph_mem: *mut opus_val32 = std::ptr::null_mut();
    let mut frame_size: i32 = 0;
    let mut rate_sum: opus_int32 = 0;
    let mut smallest_packet: opus_int32 = 0;
    if (*st).mapping_type as u32 == MAPPING_TYPE_SURROUND as i32 as u32 {
        preemph_mem = ms_get_preemph_mem(st);
        mem = ms_get_window_mem(st)
    }
    ptr = (st as *mut libc::c_char)
        .offset(align(::std::mem::size_of::<OpusMSEncoder>() as usize as i32) as isize);
    crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
        ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
        4029 as i32,
        (&mut Fs as *mut opus_int32).offset(
            (&mut Fs as *mut opus_int32).offset_from(&mut Fs as *mut opus_int32) as isize as isize,
        ),
    );
    crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
        ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
        4007 as i32,
        (&mut vbr as *mut opus_int32).offset(
            (&mut vbr as *mut opus_int32).offset_from(&mut vbr as *mut opus_int32) as isize
                as isize,
        ),
    );
    crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
        ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
        10015 as i32,
        (&mut celt_mode as *mut *const OpusCustomMode).offset(
            (&mut celt_mode as *mut *const OpusCustomMode)
                .offset_from(&mut celt_mode as *mut *const OpusCustomMode) as isize
                as isize,
        ),
    );
    frame_size = frame_size_select(analysis_frame_size, (*st).variable_duration, Fs);
    if frame_size <= 0 as i32 {
        return -(1 as i32);
    }
    /* Smallest packet the encoder can produce. */
    smallest_packet = (*st).layout.nb_streams * 2 as i32 - 1 as i32;
    /* 100 ms needs an extra byte per stream for the ToC. */
    if Fs / frame_size == 10 as i32 {
        smallest_packet += (*st).layout.nb_streams
    }
    if max_data_bytes < smallest_packet {
        return -(2 as i32);
    }
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_val16>() as usize)
            .wrapping_mul((2 as i32 * frame_size) as usize) as usize,
    );
    buf = fresh4.as_mut_ptr() as *mut opus_val16;
    coupled_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(2 as i32);
    mono_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(1 as i32);
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<opus_val16>() as usize)
            .wrapping_mul((21 as i32 * (*st).layout.nb_channels) as usize) as usize,
    );
    bandSMR = fresh5.as_mut_ptr() as *mut opus_val16;
    if (*st).mapping_type as u32 == MAPPING_TYPE_SURROUND as i32 as u32 {
        surround_analysis(
            celt_mode,
            pcm,
            bandSMR,
            mem,
            preemph_mem,
            frame_size,
            120 as i32,
            (*st).layout.nb_channels,
            Fs,
            copy_channel_in,
            (*st).arch,
        );
    }
    /* Compute bitrate allocation between streams (this could be a lot better) */
    rate_sum = rate_allocation(st, bitrates.as_mut_ptr(), frame_size);
    if vbr == 0 {
        if (*st).bitrate_bps == -(1000 as i32) {
            max_data_bytes =
                if max_data_bytes < 3 as i32 * rate_sum / (3 as i32 * 8 as i32 * Fs / frame_size) {
                    max_data_bytes
                } else {
                    (3 as i32 * rate_sum) / (3 as i32 * 8 as i32 * Fs / frame_size)
                }
        } else if (*st).bitrate_bps != -(1 as i32) {
            max_data_bytes = if max_data_bytes
                < (if smallest_packet
                    > 3 as i32 * (*st).bitrate_bps / (3 as i32 * 8 as i32 * Fs / frame_size)
                {
                    smallest_packet
                } else {
                    (3 as i32 * (*st).bitrate_bps) / (3 as i32 * 8 as i32 * Fs / frame_size)
                }) {
                max_data_bytes
            } else if smallest_packet
                > 3 as i32 * (*st).bitrate_bps / (3 as i32 * 8 as i32 * Fs / frame_size)
            {
                smallest_packet
            } else {
                (3 as i32 * (*st).bitrate_bps) / (3 as i32 * 8 as i32 * Fs / frame_size)
            }
        }
    }
    ptr = (st as *mut libc::c_char)
        .offset(align(::std::mem::size_of::<OpusMSEncoder>() as usize as i32) as isize);
    s = 0 as i32;
    while s < (*st).layout.nb_streams {
        let mut enc: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
            std::ptr::null_mut();
        enc = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
        if s < (*st).layout.nb_coupled_streams {
            ptr = ptr.offset(align(coupled_size) as isize)
        } else {
            ptr = ptr.offset(align(mono_size) as isize)
        }
        crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
            enc,
            4002 as i32,
            bitrates[s as usize],
        );
        if (*st).mapping_type as u32 == MAPPING_TYPE_SURROUND as i32 as u32 {
            let mut equiv_rate: opus_int32 = 0;
            equiv_rate = (*st).bitrate_bps;
            if (frame_size * 50 as i32) < Fs {
                equiv_rate -= 60 as i32 * (Fs / frame_size - 50 as i32) * (*st).layout.nb_channels
            }
            if equiv_rate > 10000 as i32 * (*st).layout.nb_channels {
                crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc,
                    4008 as i32,
                    1105 as i32,
                );
            } else if equiv_rate > 7000 as i32 * (*st).layout.nb_channels {
                crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc,
                    4008 as i32,
                    1104 as i32,
                );
            } else if equiv_rate > 5000 as i32 * (*st).layout.nb_channels {
                crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc,
                    4008 as i32,
                    1103 as i32,
                );
            } else {
                crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc,
                    4008 as i32,
                    1101 as i32,
                );
            }
            if s < (*st).layout.nb_coupled_streams {
                /* To preserve the spatial image, force stereo CELT on coupled streams */
                crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc,
                    11002 as i32,
                    1002 as i32,
                );
                crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc,
                    4022 as i32,
                    2 as i32,
                );
            }
        }
        s += 1
    }
    ptr = (st as *mut libc::c_char)
        .offset(align(::std::mem::size_of::<OpusMSEncoder>() as usize as i32) as isize);
    /* Counting ToC */
    tot_size = 0 as i32;
    s = 0 as i32;
    while s < (*st).layout.nb_streams {
        let mut enc_0: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
            std::ptr::null_mut();
        let mut len: i32 = 0;
        let mut curr_max: i32 = 0;
        let mut c1: i32 = 0;
        let mut c2: i32 = 0;
        let mut ret: i32 = 0;

        crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_init(
            &mut rp as *mut _ as *mut OpusRepacketizer,
        ) as *mut OpusRepacketizer;
        enc_0 = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
        if s < (*st).layout.nb_coupled_streams {
            let mut i: i32 = 0;
            let mut left: i32 = 0;
            let mut right: i32 = 0;
            left = get_left_channel(
                &mut (*st).layout as *mut _ as *const ChannelLayout,
                s,
                -(1 as i32),
            );
            right = get_right_channel(
                &mut (*st).layout as *mut _ as *const ChannelLayout,
                s,
                -(1 as i32),
            );
            Some(copy_channel_in.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                buf,
                2 as i32,
                pcm,
                (*st).layout.nb_channels,
                left,
                frame_size,
            );
            Some(copy_channel_in.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                buf.offset(1 as i32 as isize),
                2 as i32,
                pcm,
                (*st).layout.nb_channels,
                right,
                frame_size,
            );
            ptr = ptr.offset(align(coupled_size) as isize);
            if (*st).mapping_type as u32 == MAPPING_TYPE_SURROUND as i32 as u32 {
                i = 0 as i32;
                while i < 21 as i32 {
                    bandLogE[i as usize] = *bandSMR.offset((21 as i32 * left + i) as isize);
                    bandLogE[(21 as i32 + i) as usize] =
                        *bandSMR.offset((21 as i32 * right + i) as isize);
                    i += 1
                }
            }
            c1 = left;
            c2 = right
        } else {
            let mut i_0: i32 = 0;
            let mut chan: i32 = get_mono_channel(
                &mut (*st).layout as *mut _ as *const ChannelLayout,
                s,
                -(1 as i32),
            );
            Some(copy_channel_in.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                buf,
                1 as i32,
                pcm,
                (*st).layout.nb_channels,
                chan,
                frame_size,
            );
            ptr = ptr.offset(align(mono_size) as isize);
            if (*st).mapping_type as u32 == MAPPING_TYPE_SURROUND as i32 as u32 {
                i_0 = 0 as i32;
                while i_0 < 21 as i32 {
                    bandLogE[i_0 as usize] = *bandSMR.offset((21 as i32 * chan + i_0) as isize);
                    i_0 += 1
                }
            }
            c1 = chan;
            c2 = -(1 as i32)
        }
        if (*st).mapping_type as u32 == MAPPING_TYPE_SURROUND as i32 as u32 {
            crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                enc_0,
                10026 as i32,
                bandLogE.as_mut_ptr().offset(
                    bandLogE.as_mut_ptr().offset_from(bandLogE.as_mut_ptr()) as isize as isize,
                ),
            );
        }
        /* number of bytes left (+Toc) */
        curr_max = max_data_bytes - tot_size;
        /* Reserve one byte for the last stream and two for the others */
        curr_max -= if 0 as i32 > 2 as i32 * ((*st).layout.nb_streams - s - 1 as i32) - 1 as i32 {
            0 as i32
        } else {
            (2 as i32 * ((*st).layout.nb_streams - s - 1 as i32)) - 1 as i32
        };
        /* For 100 ms, reserve an extra byte per stream for the ToC */
        if Fs / frame_size == 10 as i32 {
            curr_max -= (*st).layout.nb_streams - s - 1 as i32
        }
        curr_max = if curr_max < 6 as i32 * 1275 as i32 + 12 as i32 {
            curr_max
        } else {
            (6 as i32 * 1275 as i32) + 12 as i32
        };
        /* Repacketizer will add one or two bytes for self-delimited frames */
        if s != (*st).layout.nb_streams - 1 as i32 {
            curr_max -= if curr_max > 253 as i32 {
                2 as i32
            } else {
                1 as i32
            }
        }
        if vbr == 0 && s == (*st).layout.nb_streams - 1 as i32 {
            crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                enc_0,
                4002 as i32,
                curr_max * (8 as i32 * Fs / frame_size),
            );
        }
        len = opus_encode_native(
            enc_0,
            buf,
            frame_size,
            tmp_data.as_mut_ptr(),
            curr_max,
            lsb_depth,
            pcm,
            analysis_frame_size,
            c1,
            c2,
            (*st).layout.nb_channels,
            downmix,
            float_api,
        );
        if len < 0 as i32 {
            return len;
        }
        /* We need to use the repacketizer to add the self-delimiting lengths
        while taking into account the fact that the encoder can now return
        more than one frame at a time (e.g. 60 ms CELT-only) */
        ret = crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_cat(
            &mut rp as *mut _ as *mut OpusRepacketizer,
            tmp_data.as_mut_ptr(),
            len,
        );
        /* If the opus_repacketizer_cat() fails, then something's seriously wrong
        with the encoder. */
        if ret != 0 as i32 {
            return -(3 as i32);
        }
        len = opus_repacketizer_out_range_impl(
            &mut rp as *mut _ as *mut OpusRepacketizer,
            0 as i32,
            crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_get_nb_frames(
                &mut rp as *mut _ as *mut OpusRepacketizer,
            ),
            data,
            max_data_bytes - tot_size,
            (s != (*st).layout.nb_streams - 1 as i32) as i32,
            (vbr == 0 && s == (*st).layout.nb_streams - 1 as i32) as i32,
        );
        data = data.offset(len as isize);
        tot_size += len;
        s += 1
    }
    /*printf("\n");*/
    return tot_size;
}

unsafe extern "C" fn opus_copy_channel_in_float(
    mut dst: *mut opus_val16,
    mut dst_stride: i32,
    mut src: *const libc::c_void,
    mut src_stride: i32,
    mut src_channel: i32,
    mut frame_size: i32,
) {
    let mut float_src: *const f32 = std::ptr::null();
    let mut i: opus_int32 = 0;
    float_src = src as *const f32;
    i = 0 as i32;
    while i < frame_size {
        *dst.offset((i * dst_stride) as isize) =
            *float_src.offset((i * src_stride + src_channel) as isize);
        i += 1
    }
}

unsafe extern "C" fn opus_copy_channel_in_short(
    mut dst: *mut opus_val16,
    mut dst_stride: i32,
    mut src: *const libc::c_void,
    mut src_stride: i32,
    mut src_channel: i32,
    mut frame_size: i32,
) {
    let mut short_src: *const opus_int16 = std::ptr::null();
    let mut i: opus_int32 = 0;
    short_src = src as *const opus_int16;
    i = 0 as i32;
    while i < frame_size {
        *dst.offset((i * dst_stride) as isize) = 1 as i32 as f32 / 32768.0f32
            * *short_src.offset((i * src_stride + src_channel) as isize) as i32 as f32;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encode_float(
    mut st: *mut OpusMSEncoder,
    mut pcm: *const opus_val16,
    mut frame_size: i32,
    mut data: *mut u8,
    mut max_data_bytes: opus_int32,
) -> i32 {
    return opus_multistream_encode_native(
        st,
        Some(
            opus_copy_channel_in_float
                as unsafe extern "C" fn(
                    _: *mut opus_val16,
                    _: i32,
                    _: *const libc::c_void,
                    _: i32,
                    _: i32,
                    _: i32,
                ) -> (),
        ),
        pcm as *const libc::c_void,
        frame_size,
        data,
        max_data_bytes,
        24 as i32,
        Some(
            downmix_float
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *mut opus_val32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                ) -> (),
        ),
        1 as i32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encode(
    mut st: *mut OpusMSEncoder,
    mut pcm: *const opus_int16,
    mut frame_size: i32,
    mut data: *mut u8,
    mut max_data_bytes: opus_int32,
) -> i32 {
    return opus_multistream_encode_native(
        st,
        Some(
            opus_copy_channel_in_short
                as unsafe extern "C" fn(
                    _: *mut opus_val16,
                    _: i32,
                    _: *const libc::c_void,
                    _: i32,
                    _: i32,
                    _: i32,
                ) -> (),
        ),
        pcm as *const libc::c_void,
        frame_size,
        data,
        max_data_bytes,
        16 as i32,
        Some(
            downmix_int
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *mut opus_val32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                ) -> (),
        ),
        0 as i32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encoder_ctl(
    mut st: *mut OpusMSEncoder,
    mut request: i32,
    mut args: ...
) -> i32 {
    let mut current_block: u64;
    let mut ap: ::std::ffi::VaListImpl;
    let mut coupled_size: i32 = 0;
    let mut mono_size: i32 = 0;
    let mut ptr: *mut libc::c_char = std::ptr::null_mut();
    let mut ret: i32 = 0 as i32;
    ap = args.clone();
    coupled_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(2 as i32);
    mono_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(1 as i32);
    ptr = (st as *mut libc::c_char)
        .offset(align(::std::mem::size_of::<OpusMSEncoder>() as usize as i32) as isize);
    match request {
        4002 => {
            let mut value: opus_int32 = ap.as_va_list().arg::<opus_int32>();
            if value != -(1000 as i32) && value != -(1 as i32) {
                if value <= 0 as i32 {
                    current_block = 16375338222180917333;
                } else {
                    value = if 300000 as i32 * (*st).layout.nb_channels
                        < (if 500 as i32 * (*st).layout.nb_channels > value {
                            (500 as i32) * (*st).layout.nb_channels
                        } else {
                            value
                        }) {
                        (300000 as i32) * (*st).layout.nb_channels
                    } else if 500 as i32 * (*st).layout.nb_channels > value {
                        (500 as i32) * (*st).layout.nb_channels
                    } else {
                        value
                    };
                    current_block = 11650488183268122163;
                }
            } else {
                current_block = 11650488183268122163;
            }
            match current_block {
                16375338222180917333 => {}
                _ => {
                    (*st).bitrate_bps = value;
                    current_block = 1677945370889843322;
                }
            }
        }
        4003 => {
            let mut s: i32 = 0;
            let mut value_0: *mut opus_int32 = ap.as_va_list().arg::<*mut opus_int32>();
            if value_0.is_null() {
                current_block = 16375338222180917333;
            } else {
                *value_0 = 0 as i32;
                s = 0 as i32;
                while s < (*st).layout.nb_streams {
                    let mut rate: opus_int32 = 0;
                    let mut enc: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
                        std::ptr::null_mut();
                    enc = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                    if s < (*st).layout.nb_coupled_streams {
                        ptr = ptr.offset(align(coupled_size) as isize)
                    } else {
                        ptr = ptr.offset(align(mono_size) as isize)
                    }
                    crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                        enc,
                        request,
                        &mut rate as *mut opus_int32,
                    );
                    *value_0 += rate;
                    s += 1
                }
                current_block = 1677945370889843322;
            }
        }
        4037 | 4007 | 4001 | 4009 | 4011 | 4015 | 4017 | 11019 | 4021 | 4025 | 4027 | 4029
        | 4013 | 4023 | 4043 | 4047 => {
            let mut enc_0: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
                std::ptr::null_mut();
            /* For int32* GET params, just query the first stream */
            let mut value_1: *mut opus_int32 = ap.as_va_list().arg::<*mut opus_int32>();
            enc_0 = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
            ret = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                enc_0, request, value_1,
            );
            current_block = 1677945370889843322;
        }
        4031 => {
            let mut s_0: i32 = 0;
            let mut value_2: *mut opus_uint32 = ap.as_va_list().arg::<*mut opus_uint32>();
            let mut tmp: opus_uint32 = 0;
            if value_2.is_null() {
                current_block = 16375338222180917333;
            } else {
                *value_2 = 0 as i32 as opus_uint32;
                s_0 = 0 as i32;
                while s_0 < (*st).layout.nb_streams {
                    let mut enc_1: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
                        std::ptr::null_mut();
                    enc_1 = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                    if s_0 < (*st).layout.nb_coupled_streams {
                        ptr = ptr.offset(align(coupled_size) as isize)
                    } else {
                        ptr = ptr.offset(align(mono_size) as isize)
                    }
                    ret = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                        enc_1,
                        request,
                        &mut tmp as *mut opus_uint32,
                    );
                    if ret != 0 as i32 {
                        break;
                    }
                    *value_2 ^= tmp;
                    s_0 += 1
                }
                current_block = 1677945370889843322;
            }
        }
        4036 | 4010 | 4006 | 4020 | 4004 | 4008 | 4024 | 4000 | 4012 | 4014 | 4016 | 11002
        | 4022 | 4042 | 4046 => {
            let mut s_1: i32 = 0;
            /* This works for int32 params */
            let mut value_3: opus_int32 = ap.as_va_list().arg::<opus_int32>();
            s_1 = 0 as i32;
            while s_1 < (*st).layout.nb_streams {
                let mut enc_2: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
                    std::ptr::null_mut();
                enc_2 = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                if s_1 < (*st).layout.nb_coupled_streams {
                    ptr = ptr.offset(align(coupled_size) as isize)
                } else {
                    ptr = ptr.offset(align(mono_size) as isize)
                }
                ret = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc_2, request, value_3,
                );
                if ret != 0 as i32 {
                    break;
                }
                s_1 += 1
            }
            current_block = 1677945370889843322;
        }
        5120 => {
            let mut s_2: i32 = 0;
            let mut stream_id: opus_int32 = 0;
            let mut value_4: *mut *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
                std::ptr::null_mut();
            stream_id = ap.as_va_list().arg::<opus_int32>();
            if stream_id < 0 as i32 || stream_id >= (*st).layout.nb_streams {
                ret = -(1 as i32)
            }
            value_4 = ap
                .as_va_list()
                .arg::<*mut *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder>();
            if value_4.is_null() {
                current_block = 16375338222180917333;
            } else {
                s_2 = 0 as i32;
                while s_2 < stream_id {
                    if s_2 < (*st).layout.nb_coupled_streams {
                        ptr = ptr.offset(align(coupled_size) as isize)
                    } else {
                        ptr = ptr.offset(align(mono_size) as isize)
                    }
                    s_2 += 1
                }
                *value_4 = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                current_block = 1677945370889843322;
            }
        }
        4040 => {
            let mut value_5: opus_int32 = ap.as_va_list().arg::<opus_int32>();
            (*st).variable_duration = value_5;
            current_block = 1677945370889843322;
        }
        4041 => {
            let mut value_6: *mut opus_int32 = ap.as_va_list().arg::<*mut opus_int32>();
            if value_6.is_null() {
                current_block = 16375338222180917333;
            } else {
                *value_6 = (*st).variable_duration;
                current_block = 1677945370889843322;
            }
        }
        4028 => {
            let mut s_3: i32 = 0;
            if (*st).mapping_type as u32 == MAPPING_TYPE_SURROUND as i32 as u32 {
                crate::stdlib::memset(
                    ms_get_preemph_mem(st) as *mut libc::c_void,
                    0 as i32,
                    ((*st).layout.nb_channels as usize)
                        .wrapping_mul(::std::mem::size_of::<opus_val32>() as usize),
                );
                crate::stdlib::memset(
                    ms_get_window_mem(st) as *mut libc::c_void,
                    0 as i32,
                    (((*st).layout.nb_channels * 120 as i32) as usize)
                        .wrapping_mul(::std::mem::size_of::<opus_val32>() as usize),
                );
            }
            s_3 = 0 as i32;
            while s_3 < (*st).layout.nb_streams {
                let mut enc_3: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
                    std::ptr::null_mut();
                enc_3 = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                if s_3 < (*st).layout.nb_coupled_streams {
                    ptr = ptr.offset(align(coupled_size) as isize)
                } else {
                    ptr = ptr.offset(align(mono_size) as isize)
                }
                ret =
                    crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(enc_3, 4028 as i32);
                if ret != 0 as i32 {
                    break;
                }
                s_3 += 1
            }
            current_block = 1677945370889843322;
        }
        _ => {
            ret = -(5 as i32);
            current_block = 1677945370889843322;
        }
    }
    match current_block {
        16375338222180917333 => return -(1 as i32),
        _ => return ret,
    };
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encoder_destroy(mut st: *mut OpusMSEncoder) {
    opus_free(st as *mut libc::c_void);
}

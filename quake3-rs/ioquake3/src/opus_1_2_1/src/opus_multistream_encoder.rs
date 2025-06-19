use ::libc;

pub mod opus_private_h {

    #[inline]

    pub unsafe extern "C" fn align(mut i: libc::c_int) -> libc::c_int {
        let mut alignment: libc::c_uint = 8 as libc::c_ulong as libc::c_uint;
        return (i as libc::c_uint)
            .wrapping_add(alignment)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(alignment)
            .wrapping_mul(alignment) as libc::c_int;
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

    pub unsafe extern "C" fn celt_isnan(mut x: libc::c_float) -> libc::c_int {
        let mut in_0: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        in_0.f = x;
        return (in_0.i >> 23 as libc::c_int & 0xff as libc::c_int as libc::c_uint
            == 0xff as libc::c_int as libc::c_uint
            && in_0.i & 0x7fffff as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint)
            as libc::c_int;
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
        ::libc::free(ptr);
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

    pub unsafe extern "C" fn opus_select_arch() -> libc::c_int {
        return 0 as libc::c_int;
    }
}

pub mod pitch_h {
    /*We make sure a C version is always available for cases where the overhead of
    vectorization and passing around an arch flag aren't worth it.*/
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
    pub layout: crate::opus_private_h::ChannelLayout,
    pub arch: libc::c_int,
    pub lfe_stream: libc::c_int,
    pub application: libc::c_int,
    pub variable_duration: libc::c_int,
    pub mapping_type: MappingType,
    pub bitrate_bps: crate::opus_types_h::opus_int32,
}

pub type MappingType = libc::c_uint;

pub const MAPPING_TYPE_SURROUND: MappingType = 1;

pub const MAPPING_TYPE_NONE: MappingType = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VorbisLayout {
    pub nb_streams: libc::c_int,
    pub nb_coupled_streams: libc::c_int,
    pub mapping: [libc::c_uchar; 8],
}

pub type opus_copy_channel_in_func = Option<
    unsafe extern "C" fn(
        _: *mut crate::arch_h::opus_val16,
        _: libc::c_int,
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> (),
>;
/* Index is nb_channel-1*/

static mut vorbis_mappings: [VorbisLayout; 8] = [
    {
        let mut init = VorbisLayout {
            nb_streams: 1 as libc::c_int,
            nb_coupled_streams: 0 as libc::c_int,
            mapping: [0 as libc::c_int as libc::c_uchar, 0, 0, 0, 0, 0, 0, 0],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 1 as libc::c_int,
            nb_coupled_streams: 1 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                0,
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
            nb_streams: 2 as libc::c_int,
            nb_coupled_streams: 1 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                2 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
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
            nb_streams: 2 as libc::c_int,
            nb_coupled_streams: 2 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                2 as libc::c_int as libc::c_uchar,
                3 as libc::c_int as libc::c_uchar,
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
            nb_streams: 3 as libc::c_int,
            nb_coupled_streams: 2 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                4 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                2 as libc::c_int as libc::c_uchar,
                3 as libc::c_int as libc::c_uchar,
                0,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 4 as libc::c_int,
            nb_coupled_streams: 2 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                4 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                2 as libc::c_int as libc::c_uchar,
                3 as libc::c_int as libc::c_uchar,
                5 as libc::c_int as libc::c_uchar,
                0,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 4 as libc::c_int,
            nb_coupled_streams: 3 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                4 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                2 as libc::c_int as libc::c_uchar,
                3 as libc::c_int as libc::c_uchar,
                5 as libc::c_int as libc::c_uchar,
                6 as libc::c_int as libc::c_uchar,
                0,
            ],
        };
        init
    },
    {
        let mut init = VorbisLayout {
            nb_streams: 5 as libc::c_int,
            nb_coupled_streams: 3 as libc::c_int,
            mapping: [
                0 as libc::c_int as libc::c_uchar,
                6 as libc::c_int as libc::c_uchar,
                1 as libc::c_int as libc::c_uchar,
                2 as libc::c_int as libc::c_uchar,
                3 as libc::c_int as libc::c_uchar,
                4 as libc::c_int as libc::c_uchar,
                5 as libc::c_int as libc::c_uchar,
                7 as libc::c_int as libc::c_uchar,
            ],
        };
        init
    },
];
/* Encoder states go here */
/* then opus_val32 window_mem[channels*120]; */
/* then opus_val32 preemph_mem[channels]; */

unsafe extern "C" fn ms_get_preemph_mem(
    mut st: *mut OpusMSEncoder,
) -> *mut crate::arch_h::opus_val32 {
    let mut s: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    coupled_size =
        crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(2 as libc::c_int);
    mono_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(1 as libc::c_int);
    ptr = (st as *mut libc::c_char).offset(align(
        ::std::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    s = 0 as libc::c_int;
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
        (((*st).layout.nb_channels * 120 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong)
            as isize,
    ) as *mut libc::c_void as *mut crate::arch_h::opus_val32;
}

unsafe extern "C" fn ms_get_window_mem(
    mut st: *mut OpusMSEncoder,
) -> *mut crate::arch_h::opus_val32 {
    let mut s: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    coupled_size =
        crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(2 as libc::c_int);
    mono_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(1 as libc::c_int);
    ptr = (st as *mut libc::c_char).offset(align(
        ::std::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    s = 0 as libc::c_int;
    while s < (*st).layout.nb_streams {
        if s < (*st).layout.nb_coupled_streams {
            ptr = ptr.offset(align(coupled_size) as isize)
        } else {
            ptr = ptr.offset(align(mono_size) as isize)
        }
        s += 1
    }
    /* void* cast avoids clang -Wcast-align warning */
    return ptr as *mut libc::c_void as *mut crate::arch_h::opus_val32;
}

unsafe extern "C" fn validate_encoder_layout(
    mut layout: *const crate::opus_private_h::ChannelLayout,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    s = 0 as libc::c_int;
    while s < (*layout).nb_streams {
        if s < (*layout).nb_coupled_streams {
            if crate::src::opus_1_2_1::src::opus_multistream::get_left_channel(
                layout as *const crate::opus_private_h::ChannelLayout,
                s,
                -(1 as libc::c_int),
            ) == -(1 as libc::c_int)
            {
                return 0 as libc::c_int;
            }
            if crate::src::opus_1_2_1::src::opus_multistream::get_right_channel(
                layout as *const crate::opus_private_h::ChannelLayout,
                s,
                -(1 as libc::c_int),
            ) == -(1 as libc::c_int)
            {
                return 0 as libc::c_int;
            }
        } else if crate::src::opus_1_2_1::src::opus_multistream::get_mono_channel(
            layout as *const crate::opus_private_h::ChannelLayout,
            s,
            -(1 as libc::c_int),
        ) == -(1 as libc::c_int)
        {
            return 0 as libc::c_int;
        }
        s += 1
    }
    return 1 as libc::c_int;
}

unsafe extern "C" fn channel_pos(mut channels: libc::c_int, mut pos: *mut libc::c_int) {
    /* Position in the mix: 0 don't mix, 1: left, 2: center, 3:right */
    if channels == 4 as libc::c_int {
        *pos.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(1 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(2 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(3 as libc::c_int as isize) = 3 as libc::c_int
    } else if channels == 3 as libc::c_int
        || channels == 5 as libc::c_int
        || channels == 6 as libc::c_int
    {
        *pos.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(1 as libc::c_int as isize) = 2 as libc::c_int;
        *pos.offset(2 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(3 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(4 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(5 as libc::c_int as isize) = 0 as libc::c_int
    } else if channels == 7 as libc::c_int {
        *pos.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(1 as libc::c_int as isize) = 2 as libc::c_int;
        *pos.offset(2 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(3 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(4 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(5 as libc::c_int as isize) = 2 as libc::c_int;
        *pos.offset(6 as libc::c_int as isize) = 0 as libc::c_int
    } else if channels == 8 as libc::c_int {
        *pos.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(1 as libc::c_int as isize) = 2 as libc::c_int;
        *pos.offset(2 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(3 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(4 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(5 as libc::c_int as isize) = 1 as libc::c_int;
        *pos.offset(6 as libc::c_int as isize) = 3 as libc::c_int;
        *pos.offset(7 as libc::c_int as isize) = 0 as libc::c_int
    };
}
/* Computes a rough approximation of log2(2^a + 2^b) */

unsafe extern "C" fn logSum(
    mut a: crate::arch_h::opus_val16,
    mut b: crate::arch_h::opus_val16,
) -> crate::arch_h::opus_val16 {
    let mut max: crate::arch_h::opus_val16 = 0.;
    let mut diff: crate::arch_h::opus_val32 = 0.;
    let mut frac: crate::arch_h::opus_val16 = 0.;
    static mut diff_table: [crate::arch_h::opus_val16; 17] = [
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
    let mut low: libc::c_int = 0;
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
    low = crate::stdlib::floor((2 as libc::c_int as libc::c_float * diff) as libc::c_double)
        as libc::c_int;
    frac = 2 as libc::c_int as libc::c_float * diff - low as libc::c_float;
    return max
        + diff_table[low as usize]
        + frac * (diff_table[(low + 1 as libc::c_int) as usize] - diff_table[low as usize]);
}
#[no_mangle]

pub unsafe extern "C" fn surround_analysis(
    mut celt_mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut pcm: *const libc::c_void,
    mut bandLogE: *mut crate::arch_h::opus_val16,
    mut mem: *mut crate::arch_h::opus_val32,
    mut preemph_mem: *mut crate::arch_h::opus_val32,
    mut len: libc::c_int,
    mut overlap: libc::c_int,
    mut channels: libc::c_int,
    mut rate: libc::c_int,
    mut copy_channel_in: opus_copy_channel_in_func,
    mut arch: libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut LM: libc::c_int = 0;
    let mut pos: [libc::c_int; 8] = [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0];
    let mut upsample: libc::c_int = 0;
    let mut frame_size: libc::c_int = 0;
    let mut freq_size: libc::c_int = 0;
    let mut channel_offset: crate::arch_h::opus_val16 = 0.;
    let mut bandE: [crate::arch_h::opus_val32; 21] = [0.; 21];
    let mut maskLogE: [[crate::arch_h::opus_val16; 21]; 3] = [[0.; 21]; 3];
    let mut in_0: *mut crate::arch_h::opus_val32 = 0 as *mut crate::arch_h::opus_val32;
    let mut x: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut freq: *mut crate::arch_h::opus_val32 = 0 as *mut crate::arch_h::opus_val32;
    upsample = crate::src::opus_1_2_1::celt::celt::resampling_factor(rate);
    frame_size = len * upsample;
    freq_size = if (960 as libc::c_int) < frame_size {
        960 as libc::c_int
    } else {
        frame_size
    };
    /* LM = log2(frame_size / 120) */
    LM = 0 as libc::c_int;
    while LM < (*celt_mode).maxLM {
        if (*celt_mode).shortMdctSize << LM == frame_size {
            break;
        }
        LM += 1
    }
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong)
            .wrapping_mul((frame_size + overlap) as libc::c_ulong) as usize,
    );
    in_0 = fresh0.as_mut_ptr() as *mut crate::arch_h::opus_val32;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul(len as libc::c_ulong) as usize,
    );
    x = fresh1.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong)
            .wrapping_mul(freq_size as libc::c_ulong) as usize,
    );
    freq = fresh2.as_mut_ptr() as *mut crate::arch_h::opus_val32;
    channel_pos(channels, pos.as_mut_ptr());
    c = 0 as libc::c_int;
    while c < 3 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 21 as libc::c_int {
            maskLogE[c as usize][i as usize] = -28.0f32;
            i += 1
        }
        c += 1
    }
    c = 0 as libc::c_int;
    while c < channels {
        let mut frame: libc::c_int = 0;
        let mut nb_frames: libc::c_int = frame_size / freq_size;
        crate::stdlib::memcpy(
            in_0 as *mut libc::c_void,
            mem.offset((c * overlap) as isize) as *const libc::c_void,
            (overlap as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * in_0.offset_from(mem.offset((c * overlap) as isize)) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        Some(copy_channel_in.expect("non-null function pointer"))
            .expect("non-null function pointer")(x, 1 as libc::c_int, pcm, channels, c, len);
        crate::src::opus_1_2_1::celt::celt_encoder::celt_preemphasis(
            x,
            in_0.offset(overlap as isize),
            frame_size,
            1 as libc::c_int,
            upsample,
            (*celt_mode).preemph.as_ptr(),
            preemph_mem.offset(c as isize),
            0 as libc::c_int,
        );
        let mut sum: crate::arch_h::opus_val32 = 0.;
        sum = celt_inner_prod_c(in_0, in_0, frame_size + overlap);
        /* This should filter out both NaNs and ridiculous signals that could
        cause NaNs further down. */
        if !(sum < 1e18f32) || celt_isnan(sum) != 0 {
            crate::stdlib::memset(
                in_0 as *mut libc::c_void,
                0 as libc::c_int,
                ((frame_size + overlap) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong
                    ),
            );
            *preemph_mem.offset(c as isize) = 0 as libc::c_int as crate::arch_h::opus_val32
        }
        crate::stdlib::memset(
            bandE.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (21 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong),
        );
        frame = 0 as libc::c_int;
        while frame < nb_frames {
            let mut tmpE: [crate::arch_h::opus_val32; 21] = [0.; 21];
            crate::src::opus_1_2_1::celt::mdct::clt_mdct_forward_c(
                &(*celt_mode).mdct as *const _
                    as *const crate::src::opus_1_2_1::celt::mdct::mdct_lookup,
                in_0.offset((960 as libc::c_int * frame) as isize),
                freq,
                (*celt_mode).window,
                overlap,
                (*celt_mode).maxLM - LM,
                1 as libc::c_int,
                arch,
            );
            if upsample != 1 as libc::c_int {
                let mut bound: libc::c_int = freq_size / upsample;
                i = 0 as libc::c_int;
                while i < bound {
                    let ref mut fresh3 = *freq.offset(i as isize);
                    *fresh3 *= upsample as libc::c_float;
                    i += 1
                }
                while i < freq_size {
                    *freq.offset(i as isize) = 0 as libc::c_int as crate::arch_h::opus_val32;
                    i += 1
                }
            }
            crate::src::opus_1_2_1::celt::bands::compute_band_energies(
                celt_mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
                freq,
                tmpE.as_mut_ptr(),
                21 as libc::c_int,
                1 as libc::c_int,
                LM,
                arch,
            );
            /* If we have multiple frames, take the max energy. */
            i = 0 as libc::c_int;
            while i < 21 as libc::c_int {
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
            celt_mode as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
            21 as libc::c_int,
            21 as libc::c_int,
            bandE.as_mut_ptr(),
            bandLogE.offset((21 as libc::c_int * c) as isize),
            1 as libc::c_int,
        );
        /* Apply spreading function with -6 dB/band going up and -12 dB/band going down. */
        i = 1 as libc::c_int; /* Q8 */
        while i < 21 as libc::c_int {
            *bandLogE.offset((21 as libc::c_int * c + i) as isize) = if *bandLogE
                .offset((21 as libc::c_int * c + i) as isize)
                > *bandLogE.offset((21 as libc::c_int * c + i - 1 as libc::c_int) as isize) - 1.0f32
            {
                *bandLogE.offset((21 as libc::c_int * c + i) as isize)
            } else {
                (*bandLogE.offset((21 as libc::c_int * c + i - 1 as libc::c_int) as isize)) - 1.0f32
            }; /* Q8 */
            i += 1
        }
        i = 19 as libc::c_int;
        while i >= 0 as libc::c_int {
            *bandLogE.offset((21 as libc::c_int * c + i) as isize) = if *bandLogE
                .offset((21 as libc::c_int * c + i) as isize)
                > *bandLogE.offset((21 as libc::c_int * c + i + 1 as libc::c_int) as isize) - 2.0f32
            {
                *bandLogE.offset((21 as libc::c_int * c + i) as isize)
            } else {
                (*bandLogE.offset((21 as libc::c_int * c + i + 1 as libc::c_int) as isize)) - 2.0f32
            };
            i -= 1
        }
        if pos[c as usize] == 1 as libc::c_int {
            i = 0 as libc::c_int;
            while i < 21 as libc::c_int {
                maskLogE[0 as libc::c_int as usize][i as usize] = logSum(
                    maskLogE[0 as libc::c_int as usize][i as usize],
                    *bandLogE.offset((21 as libc::c_int * c + i) as isize),
                );
                i += 1
            }
        } else if pos[c as usize] == 3 as libc::c_int {
            i = 0 as libc::c_int;
            while i < 21 as libc::c_int {
                maskLogE[2 as libc::c_int as usize][i as usize] = logSum(
                    maskLogE[2 as libc::c_int as usize][i as usize],
                    *bandLogE.offset((21 as libc::c_int * c + i) as isize),
                );
                i += 1
            }
        } else if pos[c as usize] == 2 as libc::c_int {
            i = 0 as libc::c_int;
            while i < 21 as libc::c_int {
                maskLogE[0 as libc::c_int as usize][i as usize] = logSum(
                    maskLogE[0 as libc::c_int as usize][i as usize],
                    *bandLogE.offset((21 as libc::c_int * c + i) as isize) - 0.5f32,
                );
                maskLogE[2 as libc::c_int as usize][i as usize] = logSum(
                    maskLogE[2 as libc::c_int as usize][i as usize],
                    *bandLogE.offset((21 as libc::c_int * c + i) as isize) - 0.5f32,
                );
                i += 1
            }
        }
        crate::stdlib::memcpy(
            mem.offset((c * overlap) as isize) as *mut libc::c_void,
            in_0.offset(frame_size as isize) as *const libc::c_void,
            (overlap as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * mem
                            .offset((c * overlap) as isize)
                            .offset_from(in_0.offset(frame_size as isize))
                            as libc::c_long) as libc::c_ulong,
                ),
        );
        c += 1
    }
    i = 0 as libc::c_int;
    while i < 21 as libc::c_int {
        maskLogE[1 as libc::c_int as usize][i as usize] = if maskLogE[0 as libc::c_int as usize]
            [i as usize]
            < maskLogE[2 as libc::c_int as usize][i as usize]
        {
            maskLogE[0 as libc::c_int as usize][i as usize]
        } else {
            maskLogE[2 as libc::c_int as usize][i as usize]
        };
        i += 1
    }
    channel_offset = 0.5f32 * celt_log2(2.0f32 / (channels - 1 as libc::c_int) as libc::c_float);
    c = 0 as libc::c_int;
    while c < 3 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 21 as libc::c_int {
            maskLogE[c as usize][i as usize] += channel_offset;
            i += 1
        }
        c += 1
    }
    c = 0 as libc::c_int;
    while c < channels {
        let mut mask: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
        if pos[c as usize] != 0 as libc::c_int {
            mask = &mut *(*maskLogE
                .as_mut_ptr()
                .offset((*pos.as_mut_ptr().offset(c as isize) - 1 as libc::c_int) as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut crate::arch_h::opus_val16;
            i = 0 as libc::c_int;
            while i < 21 as libc::c_int {
                *bandLogE.offset((21 as libc::c_int * c + i) as isize) = *bandLogE
                    .offset((21 as libc::c_int * c + i) as isize)
                    - *mask.offset(i as isize);
                i += 1
            }
        } else {
            i = 0 as libc::c_int;
            while i < 21 as libc::c_int {
                *bandLogE.offset((21 as libc::c_int * c + i) as isize) =
                    0 as libc::c_int as crate::arch_h::opus_val16;
                i += 1
            }
        }
        c += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encoder_get_size(
    mut nb_streams: libc::c_int,
    mut nb_coupled_streams: libc::c_int,
) -> crate::opus_types_h::opus_int32 {
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    if nb_streams < 1 as libc::c_int
        || nb_coupled_streams > nb_streams
        || nb_coupled_streams < 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    coupled_size =
        crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(2 as libc::c_int);
    mono_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(1 as libc::c_int);
    return align(::std::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int)
        + nb_coupled_streams * align(coupled_size)
        + (nb_streams - nb_coupled_streams) * align(mono_size);
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_surround_encoder_get_size(
    mut channels: libc::c_int,
    mut mapping_family: libc::c_int,
) -> crate::opus_types_h::opus_int32 {
    let mut nb_streams: libc::c_int = 0;
    let mut nb_coupled_streams: libc::c_int = 0;
    let mut size: crate::opus_types_h::opus_int32 = 0;
    if mapping_family == 0 as libc::c_int {
        if channels == 1 as libc::c_int {
            nb_streams = 1 as libc::c_int;
            nb_coupled_streams = 0 as libc::c_int
        } else if channels == 2 as libc::c_int {
            nb_streams = 1 as libc::c_int;
            nb_coupled_streams = 1 as libc::c_int
        } else {
            return 0 as libc::c_int;
        }
    } else if mapping_family == 1 as libc::c_int
        && channels <= 8 as libc::c_int
        && channels >= 1 as libc::c_int
    {
        nb_streams = vorbis_mappings[(channels - 1 as libc::c_int) as usize].nb_streams;
        nb_coupled_streams =
            vorbis_mappings[(channels - 1 as libc::c_int) as usize].nb_coupled_streams
    } else if mapping_family == 255 as libc::c_int {
        nb_streams = channels;
        nb_coupled_streams = 0 as libc::c_int
    } else {
        return 0 as libc::c_int;
    }
    size = opus_multistream_encoder_get_size(nb_streams, nb_coupled_streams);
    if channels > 2 as libc::c_int {
        size = (size as libc::c_ulong).wrapping_add(
            (channels as libc::c_ulong).wrapping_mul(
                (120 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong
                    )
                    .wrapping_add(
                        ::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong
                    ),
            ),
        ) as crate::opus_types_h::opus_int32 as crate::opus_types_h::opus_int32
    }
    return size;
}

unsafe extern "C" fn opus_multistream_encoder_init_impl(
    mut st: *mut OpusMSEncoder,
    mut Fs: crate::opus_types_h::opus_int32,
    mut channels: libc::c_int,
    mut streams: libc::c_int,
    mut coupled_streams: libc::c_int,
    mut mapping: *const libc::c_uchar,
    mut application: libc::c_int,
    mut mapping_type: MappingType,
) -> libc::c_int {
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if channels > 255 as libc::c_int
        || channels < 1 as libc::c_int
        || coupled_streams > streams
        || streams < 1 as libc::c_int
        || coupled_streams < 0 as libc::c_int
        || streams > 255 as libc::c_int - coupled_streams
    {
        return -(1 as libc::c_int);
    }
    (*st).arch = opus_select_arch();
    (*st).layout.nb_channels = channels;
    (*st).layout.nb_streams = streams;
    (*st).layout.nb_coupled_streams = coupled_streams;
    if mapping_type as libc::c_uint != MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint {
        (*st).lfe_stream = -(1 as libc::c_int)
    }
    (*st).bitrate_bps = -(1000 as libc::c_int);
    (*st).application = application;
    (*st).variable_duration = 5000 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_channels {
        (*st).layout.mapping[i as usize] = *mapping.offset(i as isize);
        i += 1
    }
    if crate::src::opus_1_2_1::src::opus_multistream::validate_layout(
        &mut (*st).layout as *mut _ as *const crate::opus_private_h::ChannelLayout,
    ) == 0
    {
        return -(1 as libc::c_int);
    }
    if mapping_type as libc::c_uint == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint
        && validate_encoder_layout(&mut (*st).layout) == 0
    {
        return -(1 as libc::c_int);
    }
    ptr = (st as *mut libc::c_char).offset(align(
        ::std::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    coupled_size =
        crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(2 as libc::c_int);
    mono_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_coupled_streams {
        ret = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_init(
            ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
            Fs,
            2 as libc::c_int,
            application,
        );
        if ret != 0 as libc::c_int {
            return ret;
        }
        if i == (*st).lfe_stream {
            crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
                10024 as libc::c_int,
                1 as libc::c_int,
            );
        }
        ptr = ptr.offset(align(coupled_size) as isize);
        i += 1
    }
    while i < (*st).layout.nb_streams {
        ret = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_init(
            ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
            Fs,
            1 as libc::c_int,
            application,
        );
        if i == (*st).lfe_stream {
            crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
                10024 as libc::c_int,
                1 as libc::c_int,
            );
        }
        if ret != 0 as libc::c_int {
            return ret;
        }
        ptr = ptr.offset(align(mono_size) as isize);
        i += 1
    }
    if mapping_type as libc::c_uint == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint {
        crate::stdlib::memset(
            ms_get_preemph_mem(st) as *mut libc::c_void,
            0 as libc::c_int,
            (channels as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong),
        );
        crate::stdlib::memset(
            ms_get_window_mem(st) as *mut libc::c_void,
            0 as libc::c_int,
            ((channels * 120 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong),
        );
    }
    (*st).mapping_type = mapping_type;
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encoder_init(
    mut st: *mut OpusMSEncoder,
    mut Fs: crate::opus_types_h::opus_int32,
    mut channels: libc::c_int,
    mut streams: libc::c_int,
    mut coupled_streams: libc::c_int,
    mut mapping: *const libc::c_uchar,
    mut application: libc::c_int,
) -> libc::c_int {
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
    mut Fs: crate::opus_types_h::opus_int32,
    mut channels: libc::c_int,
    mut mapping_family: libc::c_int,
    mut streams: *mut libc::c_int,
    mut coupled_streams: *mut libc::c_int,
    mut mapping: *mut libc::c_uchar,
    mut application: libc::c_int,
) -> libc::c_int {
    let mut mapping_type: MappingType = MAPPING_TYPE_NONE;
    if channels > 255 as libc::c_int || channels < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*st).lfe_stream = -(1 as libc::c_int);
    if mapping_family == 0 as libc::c_int {
        if channels == 1 as libc::c_int {
            *streams = 1 as libc::c_int;
            *coupled_streams = 0 as libc::c_int;
            *mapping.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar
        } else if channels == 2 as libc::c_int {
            *streams = 1 as libc::c_int;
            *coupled_streams = 1 as libc::c_int;
            *mapping.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
            *mapping.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar
        } else {
            return -(5 as libc::c_int);
        }
    } else if mapping_family == 1 as libc::c_int
        && channels <= 8 as libc::c_int
        && channels >= 1 as libc::c_int
    {
        let mut i: libc::c_int = 0;
        *streams = vorbis_mappings[(channels - 1 as libc::c_int) as usize].nb_streams;
        *coupled_streams =
            vorbis_mappings[(channels - 1 as libc::c_int) as usize].nb_coupled_streams;
        i = 0 as libc::c_int;
        while i < channels {
            *mapping.offset(i as isize) =
                vorbis_mappings[(channels - 1 as libc::c_int) as usize].mapping[i as usize];
            i += 1
        }
        if channels >= 6 as libc::c_int {
            (*st).lfe_stream = *streams - 1 as libc::c_int
        }
    } else if mapping_family == 255 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        *streams = channels;
        *coupled_streams = 0 as libc::c_int;
        i_0 = 0 as libc::c_int;
        while i_0 < channels {
            *mapping.offset(i_0 as isize) = i_0 as libc::c_uchar;
            i_0 += 1
        }
    } else {
        return -(5 as libc::c_int);
    }
    if channels > 2 as libc::c_int && mapping_family == 1 as libc::c_int {
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
    mut Fs: crate::opus_types_h::opus_int32,
    mut channels: libc::c_int,
    mut streams: libc::c_int,
    mut coupled_streams: libc::c_int,
    mut mapping: *const libc::c_uchar,
    mut application: libc::c_int,
    mut error: *mut libc::c_int,
) -> *mut OpusMSEncoder {
    let mut ret: libc::c_int = 0;
    let mut st: *mut OpusMSEncoder = 0 as *mut OpusMSEncoder;
    if channels > 255 as libc::c_int
        || channels < 1 as libc::c_int
        || coupled_streams > streams
        || streams < 1 as libc::c_int
        || coupled_streams < 0 as libc::c_int
        || streams > 255 as libc::c_int - coupled_streams
    {
        if !error.is_null() {
            *error = -(1 as libc::c_int)
        }
        return 0 as *mut OpusMSEncoder;
    }
    st = opus_alloc(
        opus_multistream_encoder_get_size(streams, coupled_streams) as crate::stddef_h::size_t
    ) as *mut OpusMSEncoder;
    if st.is_null() {
        if !error.is_null() {
            *error = -(7 as libc::c_int)
        }
        return 0 as *mut OpusMSEncoder;
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
    if ret != 0 as libc::c_int {
        opus_free(st as *mut libc::c_void);
        st = 0 as *mut OpusMSEncoder
    }
    if !error.is_null() {
        *error = ret
    }
    return st;
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_surround_encoder_create(
    mut Fs: crate::opus_types_h::opus_int32,
    mut channels: libc::c_int,
    mut mapping_family: libc::c_int,
    mut streams: *mut libc::c_int,
    mut coupled_streams: *mut libc::c_int,
    mut mapping: *mut libc::c_uchar,
    mut application: libc::c_int,
    mut error: *mut libc::c_int,
) -> *mut OpusMSEncoder {
    let mut ret: libc::c_int = 0;
    let mut size: crate::opus_types_h::opus_int32 = 0;
    let mut st: *mut OpusMSEncoder = 0 as *mut OpusMSEncoder;
    if channels > 255 as libc::c_int || channels < 1 as libc::c_int {
        if !error.is_null() {
            *error = -(1 as libc::c_int)
        }
        return 0 as *mut OpusMSEncoder;
    }
    size = opus_multistream_surround_encoder_get_size(channels, mapping_family);
    if size == 0 {
        if !error.is_null() {
            *error = -(5 as libc::c_int)
        }
        return 0 as *mut OpusMSEncoder;
    }
    st = opus_alloc(size as crate::stddef_h::size_t) as *mut OpusMSEncoder;
    if st.is_null() {
        if !error.is_null() {
            *error = -(7 as libc::c_int)
        }
        return 0 as *mut OpusMSEncoder;
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
    if ret != 0 as libc::c_int {
        opus_free(st as *mut libc::c_void);
        st = 0 as *mut OpusMSEncoder
    }
    if !error.is_null() {
        *error = ret
    }
    return st;
}

unsafe extern "C" fn surround_rate_allocation(
    mut st: *mut OpusMSEncoder,
    mut rate: *mut crate::opus_types_h::opus_int32,
    mut frame_size: libc::c_int,
    mut Fs: crate::opus_types_h::opus_int32,
) {
    let mut i: libc::c_int = 0;
    let mut channel_rate: crate::opus_types_h::opus_int32 = 0;
    let mut stream_offset: libc::c_int = 0;
    let mut lfe_offset: libc::c_int = 0;
    let mut coupled_ratio: libc::c_int = 0;
    let mut lfe_ratio: libc::c_int = 0;
    let mut nb_lfe: libc::c_int = 0;
    let mut nb_uncoupled: libc::c_int = 0;
    let mut nb_coupled: libc::c_int = 0;
    let mut nb_normal: libc::c_int = 0;
    let mut channel_offset: crate::opus_types_h::opus_int32 = 0;
    let mut bitrate: crate::opus_types_h::opus_int32 = 0;
    let mut total: libc::c_int = 0;
    nb_lfe = ((*st).lfe_stream != -(1 as libc::c_int)) as libc::c_int;
    nb_coupled = (*st).layout.nb_coupled_streams;
    nb_uncoupled = (*st).layout.nb_streams - nb_coupled - nb_lfe;
    nb_normal = 2 as libc::c_int * nb_coupled + nb_uncoupled;
    /* Give each non-LFE channel enough bits per channel for coding band energy. */
    channel_offset = 40 as libc::c_int
        * (if 50 as libc::c_int > Fs / frame_size {
            50 as libc::c_int
        } else {
            (Fs) / frame_size
        });
    if (*st).bitrate_bps == -(1000 as libc::c_int) {
        bitrate =
            nb_normal * (channel_offset + Fs + 10000 as libc::c_int) + 8000 as libc::c_int * nb_lfe
    } else if (*st).bitrate_bps == -(1 as libc::c_int) {
        bitrate = nb_normal * 300000 as libc::c_int + nb_lfe * 128000 as libc::c_int
    } else {
        bitrate = (*st).bitrate_bps
    }
    /* Give LFE some basic stream_channel allocation but never exceed 1/20 of the
    total rate for the non-energy part to avoid problems at really low rate. */
    lfe_offset = (if (bitrate / 20 as libc::c_int) < 3000 as libc::c_int {
        (bitrate) / 20 as libc::c_int
    } else {
        3000 as libc::c_int
    }) + 15 as libc::c_int
        * (if 50 as libc::c_int > Fs / frame_size {
            50 as libc::c_int
        } else {
            (Fs) / frame_size
        });
    /* We give each stream (coupled or uncoupled) a starting bitrate.
    This models the main saving of coupled channels over uncoupled. */
    stream_offset =
        (bitrate - channel_offset * nb_normal - lfe_offset * nb_lfe) / nb_normal / 2 as libc::c_int;
    stream_offset = if 0 as libc::c_int
        > (if (20000 as libc::c_int) < stream_offset {
            20000 as libc::c_int
        } else {
            stream_offset
        }) {
        0 as libc::c_int
    } else if (20000 as libc::c_int) < stream_offset {
        20000 as libc::c_int
    } else {
        stream_offset
    };
    /* Coupled streams get twice the mono rate after the offset is allocated. */
    coupled_ratio = 512 as libc::c_int;
    /* Should depend on the bitrate, for now we assume LFE gets 1/8 the bits of mono */
    lfe_ratio = 32 as libc::c_int;
    total = (nb_uncoupled << 8 as libc::c_int) + coupled_ratio * nb_coupled + nb_lfe * lfe_ratio;
    channel_rate = (256 as libc::c_int as libc::c_longlong
        * (bitrate
            - lfe_offset * nb_lfe
            - stream_offset * (nb_coupled + nb_uncoupled)
            - channel_offset * nb_normal) as libc::c_longlong
        / total as libc::c_longlong) as crate::opus_types_h::opus_int32;
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_streams {
        if i < (*st).layout.nb_coupled_streams {
            *rate.offset(i as isize) = 2 as libc::c_int * channel_offset
                + (if 0 as libc::c_int
                    > stream_offset + (channel_rate * coupled_ratio >> 8 as libc::c_int)
                {
                    0 as libc::c_int
                } else {
                    (stream_offset) + (channel_rate * coupled_ratio >> 8 as libc::c_int)
                })
        } else if i != (*st).lfe_stream {
            *rate.offset(i as isize) = channel_offset
                + (if 0 as libc::c_int > stream_offset + channel_rate {
                    0 as libc::c_int
                } else {
                    (stream_offset) + channel_rate
                })
        } else {
            *rate.offset(i as isize) =
                if 0 as libc::c_int > lfe_offset + (channel_rate * lfe_ratio >> 8 as libc::c_int) {
                    0 as libc::c_int
                } else {
                    (lfe_offset) + (channel_rate * lfe_ratio >> 8 as libc::c_int)
                }
        }
        i += 1
    }
}
/* ENABLE_EXPERIMENTAL_AMBISONICS */

unsafe extern "C" fn rate_allocation(
    mut st: *mut OpusMSEncoder,
    mut rate: *mut crate::opus_types_h::opus_int32,
    mut frame_size: libc::c_int,
) -> crate::opus_types_h::opus_int32 {
    let mut i: libc::c_int = 0;
    let mut rate_sum: crate::opus_types_h::opus_int32 = 0 as libc::c_int;
    let mut Fs: crate::opus_types_h::opus_int32 = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = (st as *mut libc::c_char).offset(align(
        ::std::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
        ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
        4029 as libc::c_int,
        (&mut Fs as *mut crate::opus_types_h::opus_int32).offset(
            (&mut Fs as *mut crate::opus_types_h::opus_int32)
                .offset_from(&mut Fs as *mut crate::opus_types_h::opus_int32)
                as libc::c_long as isize,
        ),
    );
    surround_rate_allocation(st, rate, frame_size, Fs);
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_streams {
        *rate.offset(i as isize) = if *rate.offset(i as isize) > 500 as libc::c_int {
            *rate.offset(i as isize)
        } else {
            500 as libc::c_int
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
    mut analysis_frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut max_data_bytes: crate::opus_types_h::opus_int32,
    mut lsb_depth: libc::c_int,
    mut downmix: crate::opus_private_h::downmix_func,
    mut float_api: libc::c_int,
) -> libc::c_int {
    let mut Fs: crate::opus_types_h::opus_int32 = 0;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tot_size: libc::c_int = 0;
    let mut buf: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut bandSMR: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut tmp_data: [libc::c_uchar; 7662] = [0; 7662];
    let mut rp: crate::opus_private_h::OpusRepacketizer = crate::opus_private_h::OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [0 as *const libc::c_uchar; 48],
        len: [0; 48],
        framesize: 0,
    };
    let mut vbr: crate::opus_types_h::opus_int32 = 0;
    let mut celt_mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
    let mut bitrates: [crate::opus_types_h::opus_int32; 256] = [0; 256];
    let mut bandLogE: [crate::arch_h::opus_val16; 42] = [0.; 42];
    let mut mem: *mut crate::arch_h::opus_val32 = 0 as *mut crate::arch_h::opus_val32;
    let mut preemph_mem: *mut crate::arch_h::opus_val32 = 0 as *mut crate::arch_h::opus_val32;
    let mut frame_size: libc::c_int = 0;
    let mut rate_sum: crate::opus_types_h::opus_int32 = 0;
    let mut smallest_packet: crate::opus_types_h::opus_int32 = 0;
    if (*st).mapping_type as libc::c_uint == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint {
        preemph_mem = ms_get_preemph_mem(st);
        mem = ms_get_window_mem(st)
    }
    ptr = (st as *mut libc::c_char).offset(align(
        ::std::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
        ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
        4029 as libc::c_int,
        (&mut Fs as *mut crate::opus_types_h::opus_int32).offset(
            (&mut Fs as *mut crate::opus_types_h::opus_int32)
                .offset_from(&mut Fs as *mut crate::opus_types_h::opus_int32)
                as libc::c_long as isize,
        ),
    );
    crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
        ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
        4007 as libc::c_int,
        (&mut vbr as *mut crate::opus_types_h::opus_int32).offset(
            (&mut vbr as *mut crate::opus_types_h::opus_int32)
                .offset_from(&mut vbr as *mut crate::opus_types_h::opus_int32)
                as libc::c_long as isize,
        ),
    );
    crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
        ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
        10015 as libc::c_int,
        (&mut celt_mode as *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode).offset(
            (&mut celt_mode as *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode)
                .offset_from(
                    &mut celt_mode
                        as *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
                ) as libc::c_long as isize,
        ),
    );
    frame_size = crate::src::opus_1_2_1::src::opus_encoder::frame_size_select(
        analysis_frame_size,
        (*st).variable_duration,
        Fs,
    );
    if frame_size <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    /* Smallest packet the encoder can produce. */
    smallest_packet = (*st).layout.nb_streams * 2 as libc::c_int - 1 as libc::c_int;
    /* 100 ms needs an extra byte per stream for the ToC. */
    if Fs / frame_size == 10 as libc::c_int {
        smallest_packet += (*st).layout.nb_streams
    }
    if max_data_bytes < smallest_packet {
        return -(2 as libc::c_int);
    }
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul((2 as libc::c_int * frame_size) as libc::c_ulong) as usize,
    );
    buf = fresh4.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    coupled_size =
        crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(2 as libc::c_int);
    mono_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(1 as libc::c_int);
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul((21 as libc::c_int * (*st).layout.nb_channels) as libc::c_ulong)
            as usize,
    );
    bandSMR = fresh5.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    if (*st).mapping_type as libc::c_uint == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint {
        surround_analysis(
            celt_mode,
            pcm,
            bandSMR,
            mem,
            preemph_mem,
            frame_size,
            120 as libc::c_int,
            (*st).layout.nb_channels,
            Fs,
            copy_channel_in,
            (*st).arch,
        );
    }
    /* Compute bitrate allocation between streams (this could be a lot better) */
    rate_sum = rate_allocation(st, bitrates.as_mut_ptr(), frame_size);
    if vbr == 0 {
        if (*st).bitrate_bps == -(1000 as libc::c_int) {
            max_data_bytes = if max_data_bytes
                < 3 as libc::c_int * rate_sum
                    / (3 as libc::c_int * 8 as libc::c_int * Fs / frame_size)
            {
                max_data_bytes
            } else {
                (3 as libc::c_int * rate_sum)
                    / (3 as libc::c_int * 8 as libc::c_int * Fs / frame_size)
            }
        } else if (*st).bitrate_bps != -(1 as libc::c_int) {
            max_data_bytes = if max_data_bytes
                < (if smallest_packet
                    > 3 as libc::c_int * (*st).bitrate_bps
                        / (3 as libc::c_int * 8 as libc::c_int * Fs / frame_size)
                {
                    smallest_packet
                } else {
                    (3 as libc::c_int * (*st).bitrate_bps)
                        / (3 as libc::c_int * 8 as libc::c_int * Fs / frame_size)
                }) {
                max_data_bytes
            } else if smallest_packet
                > 3 as libc::c_int * (*st).bitrate_bps
                    / (3 as libc::c_int * 8 as libc::c_int * Fs / frame_size)
            {
                smallest_packet
            } else {
                (3 as libc::c_int * (*st).bitrate_bps)
                    / (3 as libc::c_int * 8 as libc::c_int * Fs / frame_size)
            }
        }
    }
    ptr = (st as *mut libc::c_char).offset(align(
        ::std::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    s = 0 as libc::c_int;
    while s < (*st).layout.nb_streams {
        let mut enc: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
            0 as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
        enc = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
        if s < (*st).layout.nb_coupled_streams {
            ptr = ptr.offset(align(coupled_size) as isize)
        } else {
            ptr = ptr.offset(align(mono_size) as isize)
        }
        crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
            enc,
            4002 as libc::c_int,
            bitrates[s as usize],
        );
        if (*st).mapping_type as libc::c_uint
            == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint
        {
            let mut equiv_rate: crate::opus_types_h::opus_int32 = 0;
            equiv_rate = (*st).bitrate_bps;
            if (frame_size * 50 as libc::c_int) < Fs {
                equiv_rate -= 60 as libc::c_int
                    * (Fs / frame_size - 50 as libc::c_int)
                    * (*st).layout.nb_channels
            }
            if equiv_rate > 10000 as libc::c_int * (*st).layout.nb_channels {
                crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc,
                    4008 as libc::c_int,
                    1105 as libc::c_int,
                );
            } else if equiv_rate > 7000 as libc::c_int * (*st).layout.nb_channels {
                crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc,
                    4008 as libc::c_int,
                    1104 as libc::c_int,
                );
            } else if equiv_rate > 5000 as libc::c_int * (*st).layout.nb_channels {
                crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc,
                    4008 as libc::c_int,
                    1103 as libc::c_int,
                );
            } else {
                crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc,
                    4008 as libc::c_int,
                    1101 as libc::c_int,
                );
            }
            if s < (*st).layout.nb_coupled_streams {
                /* To preserve the spatial image, force stereo CELT on coupled streams */
                crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc,
                    11002 as libc::c_int,
                    1002 as libc::c_int,
                );
                crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc,
                    4022 as libc::c_int,
                    2 as libc::c_int,
                );
            }
        }
        s += 1
    }
    ptr = (st as *mut libc::c_char).offset(align(
        ::std::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    /* Counting ToC */
    tot_size = 0 as libc::c_int;
    s = 0 as libc::c_int;
    while s < (*st).layout.nb_streams {
        let mut enc_0: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
            0 as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
        let mut len: libc::c_int = 0;
        let mut curr_max: libc::c_int = 0;
        let mut c1: libc::c_int = 0;
        let mut c2: libc::c_int = 0;
        let mut ret: libc::c_int = 0;

        crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_init(
            &mut rp as *mut _ as *mut crate::opus_private_h::OpusRepacketizer,
        ) as *mut crate::opus_private_h::OpusRepacketizer;
        enc_0 = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
        if s < (*st).layout.nb_coupled_streams {
            let mut i: libc::c_int = 0;
            let mut left: libc::c_int = 0;
            let mut right: libc::c_int = 0;
            left = crate::src::opus_1_2_1::src::opus_multistream::get_left_channel(
                &mut (*st).layout as *mut _ as *const crate::opus_private_h::ChannelLayout,
                s,
                -(1 as libc::c_int),
            );
            right = crate::src::opus_1_2_1::src::opus_multistream::get_right_channel(
                &mut (*st).layout as *mut _ as *const crate::opus_private_h::ChannelLayout,
                s,
                -(1 as libc::c_int),
            );
            Some(copy_channel_in.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                buf,
                2 as libc::c_int,
                pcm,
                (*st).layout.nb_channels,
                left,
                frame_size,
            );
            Some(copy_channel_in.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                buf.offset(1 as libc::c_int as isize),
                2 as libc::c_int,
                pcm,
                (*st).layout.nb_channels,
                right,
                frame_size,
            );
            ptr = ptr.offset(align(coupled_size) as isize);
            if (*st).mapping_type as libc::c_uint
                == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint
            {
                i = 0 as libc::c_int;
                while i < 21 as libc::c_int {
                    bandLogE[i as usize] = *bandSMR.offset((21 as libc::c_int * left + i) as isize);
                    bandLogE[(21 as libc::c_int + i) as usize] =
                        *bandSMR.offset((21 as libc::c_int * right + i) as isize);
                    i += 1
                }
            }
            c1 = left;
            c2 = right
        } else {
            let mut i_0: libc::c_int = 0;
            let mut chan: libc::c_int =
                crate::src::opus_1_2_1::src::opus_multistream::get_mono_channel(
                    &mut (*st).layout as *mut _ as *const crate::opus_private_h::ChannelLayout,
                    s,
                    -(1 as libc::c_int),
                );
            Some(copy_channel_in.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                buf,
                1 as libc::c_int,
                pcm,
                (*st).layout.nb_channels,
                chan,
                frame_size,
            );
            ptr = ptr.offset(align(mono_size) as isize);
            if (*st).mapping_type as libc::c_uint
                == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint
            {
                i_0 = 0 as libc::c_int;
                while i_0 < 21 as libc::c_int {
                    bandLogE[i_0 as usize] =
                        *bandSMR.offset((21 as libc::c_int * chan + i_0) as isize);
                    i_0 += 1
                }
            }
            c1 = chan;
            c2 = -(1 as libc::c_int)
        }
        if (*st).mapping_type as libc::c_uint
            == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint
        {
            crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                enc_0,
                10026 as libc::c_int,
                bandLogE.as_mut_ptr().offset(
                    bandLogE.as_mut_ptr().offset_from(bandLogE.as_mut_ptr()) as libc::c_long
                        as isize,
                ),
            );
        }
        /* number of bytes left (+Toc) */
        curr_max = max_data_bytes - tot_size;
        /* Reserve one byte for the last stream and two for the others */
        curr_max -= if 0 as libc::c_int
            > 2 as libc::c_int * ((*st).layout.nb_streams - s - 1 as libc::c_int) - 1 as libc::c_int
        {
            0 as libc::c_int
        } else {
            (2 as libc::c_int * ((*st).layout.nb_streams - s - 1 as libc::c_int)) - 1 as libc::c_int
        };
        /* For 100 ms, reserve an extra byte per stream for the ToC */
        if Fs / frame_size == 10 as libc::c_int {
            curr_max -= (*st).layout.nb_streams - s - 1 as libc::c_int
        }
        curr_max = if curr_max < 6 as libc::c_int * 1275 as libc::c_int + 12 as libc::c_int {
            curr_max
        } else {
            (6 as libc::c_int * 1275 as libc::c_int) + 12 as libc::c_int
        };
        /* Repacketizer will add one or two bytes for self-delimited frames */
        if s != (*st).layout.nb_streams - 1 as libc::c_int {
            curr_max -= if curr_max > 253 as libc::c_int {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            }
        }
        if vbr == 0 && s == (*st).layout.nb_streams - 1 as libc::c_int {
            crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                enc_0,
                4002 as libc::c_int,
                curr_max * (8 as libc::c_int * Fs / frame_size),
            );
        }
        len = crate::src::opus_1_2_1::src::opus_encoder::opus_encode_native(
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
        if len < 0 as libc::c_int {
            return len;
        }
        /* We need to use the repacketizer to add the self-delimiting lengths
        while taking into account the fact that the encoder can now return
        more than one frame at a time (e.g. 60 ms CELT-only) */
        ret = crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_cat(
            &mut rp as *mut _ as *mut crate::opus_private_h::OpusRepacketizer,
            tmp_data.as_mut_ptr(),
            len,
        );
        /* If the opus_repacketizer_cat() fails, then something's seriously wrong
        with the encoder. */
        if ret != 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        len = crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_out_range_impl(
            &mut rp as *mut _ as *mut crate::opus_private_h::OpusRepacketizer,
            0 as libc::c_int,
            crate::src::opus_1_2_1::src::repacketizer::opus_repacketizer_get_nb_frames(
                &mut rp as *mut _ as *mut crate::opus_private_h::OpusRepacketizer,
            ),
            data,
            max_data_bytes - tot_size,
            (s != (*st).layout.nb_streams - 1 as libc::c_int) as libc::c_int,
            (vbr == 0 && s == (*st).layout.nb_streams - 1 as libc::c_int) as libc::c_int,
        );
        data = data.offset(len as isize);
        tot_size += len;
        s += 1
    }
    /*printf("\n");*/
    return tot_size;
}

unsafe extern "C" fn opus_copy_channel_in_float(
    mut dst: *mut crate::arch_h::opus_val16,
    mut dst_stride: libc::c_int,
    mut src: *const libc::c_void,
    mut src_stride: libc::c_int,
    mut src_channel: libc::c_int,
    mut frame_size: libc::c_int,
) {
    let mut float_src: *const libc::c_float = 0 as *const libc::c_float;
    let mut i: crate::opus_types_h::opus_int32 = 0;
    float_src = src as *const libc::c_float;
    i = 0 as libc::c_int;
    while i < frame_size {
        *dst.offset((i * dst_stride) as isize) =
            *float_src.offset((i * src_stride + src_channel) as isize);
        i += 1
    }
}

unsafe extern "C" fn opus_copy_channel_in_short(
    mut dst: *mut crate::arch_h::opus_val16,
    mut dst_stride: libc::c_int,
    mut src: *const libc::c_void,
    mut src_stride: libc::c_int,
    mut src_channel: libc::c_int,
    mut frame_size: libc::c_int,
) {
    let mut short_src: *const crate::opus_types_h::opus_int16 =
        0 as *const crate::opus_types_h::opus_int16;
    let mut i: crate::opus_types_h::opus_int32 = 0;
    short_src = src as *const crate::opus_types_h::opus_int16;
    i = 0 as libc::c_int;
    while i < frame_size {
        *dst.offset((i * dst_stride) as isize) = 1 as libc::c_int as libc::c_float / 32768.0f32
            * *short_src.offset((i * src_stride + src_channel) as isize) as libc::c_int
                as libc::c_float;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encode_float(
    mut st: *mut OpusMSEncoder,
    mut pcm: *const crate::arch_h::opus_val16,
    mut frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut max_data_bytes: crate::opus_types_h::opus_int32,
) -> libc::c_int {
    return opus_multistream_encode_native(
        st,
        Some(
            opus_copy_channel_in_float
                as unsafe extern "C" fn(
                    _: *mut crate::arch_h::opus_val16,
                    _: libc::c_int,
                    _: *const libc::c_void,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                ) -> (),
        ),
        pcm as *const libc::c_void,
        frame_size,
        data,
        max_data_bytes,
        24 as libc::c_int,
        Some(
            crate::src::opus_1_2_1::src::opus_encoder::downmix_float
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *mut crate::arch_h::opus_val32,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                ) -> (),
        ),
        1 as libc::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encode(
    mut st: *mut OpusMSEncoder,
    mut pcm: *const crate::opus_types_h::opus_int16,
    mut frame_size: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut max_data_bytes: crate::opus_types_h::opus_int32,
) -> libc::c_int {
    return opus_multistream_encode_native(
        st,
        Some(
            opus_copy_channel_in_short
                as unsafe extern "C" fn(
                    _: *mut crate::arch_h::opus_val16,
                    _: libc::c_int,
                    _: *const libc::c_void,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                ) -> (),
        ),
        pcm as *const libc::c_void,
        frame_size,
        data,
        max_data_bytes,
        16 as libc::c_int,
        Some(
            crate::src::opus_1_2_1::src::opus_encoder::downmix_int
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *mut crate::arch_h::opus_val32,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: libc::c_int,
                ) -> (),
        ),
        0 as libc::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encoder_ctl(
    mut st: *mut OpusMSEncoder,
    mut request: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut current_block: u64;
    let mut ap: ::std::ffi::VaListImpl;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0 as libc::c_int;
    ap = args.clone();
    coupled_size =
        crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(2 as libc::c_int);
    mono_size = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_get_size(1 as libc::c_int);
    ptr = (st as *mut libc::c_char).offset(align(
        ::std::mem::size_of::<OpusMSEncoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    match request {
        4002 => {
            let mut value: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value != -(1000 as libc::c_int) && value != -(1 as libc::c_int) {
                if value <= 0 as libc::c_int {
                    current_block = 16375338222180917333;
                } else {
                    value = if 300000 as libc::c_int * (*st).layout.nb_channels
                        < (if 500 as libc::c_int * (*st).layout.nb_channels > value {
                            (500 as libc::c_int) * (*st).layout.nb_channels
                        } else {
                            value
                        }) {
                        (300000 as libc::c_int) * (*st).layout.nb_channels
                    } else if 500 as libc::c_int * (*st).layout.nb_channels > value {
                        (500 as libc::c_int) * (*st).layout.nb_channels
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
            let mut s: libc::c_int = 0;
            let mut value_0: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_0.is_null() {
                current_block = 16375338222180917333;
            } else {
                *value_0 = 0 as libc::c_int;
                s = 0 as libc::c_int;
                while s < (*st).layout.nb_streams {
                    let mut rate: crate::opus_types_h::opus_int32 = 0;
                    let mut enc: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
                        0 as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                    enc = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                    if s < (*st).layout.nb_coupled_streams {
                        ptr = ptr.offset(align(coupled_size) as isize)
                    } else {
                        ptr = ptr.offset(align(mono_size) as isize)
                    }
                    crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                        enc,
                        request,
                        &mut rate as *mut crate::opus_types_h::opus_int32,
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
                0 as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
            /* For int32* GET params, just query the first stream */
            let mut value_1: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            enc_0 = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
            ret = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                enc_0, request, value_1,
            );
            current_block = 1677945370889843322;
        }
        4031 => {
            let mut s_0: libc::c_int = 0;
            let mut value_2: *mut crate::opus_types_h::opus_uint32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_uint32>();
            let mut tmp: crate::opus_types_h::opus_uint32 = 0;
            if value_2.is_null() {
                current_block = 16375338222180917333;
            } else {
                *value_2 = 0 as libc::c_int as crate::opus_types_h::opus_uint32;
                s_0 = 0 as libc::c_int;
                while s_0 < (*st).layout.nb_streams {
                    let mut enc_1: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
                        0 as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                    enc_1 = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                    if s_0 < (*st).layout.nb_coupled_streams {
                        ptr = ptr.offset(align(coupled_size) as isize)
                    } else {
                        ptr = ptr.offset(align(mono_size) as isize)
                    }
                    ret = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                        enc_1,
                        request,
                        &mut tmp as *mut crate::opus_types_h::opus_uint32,
                    );
                    if ret != 0 as libc::c_int {
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
            let mut s_1: libc::c_int = 0;
            /* This works for int32 params */
            let mut value_3: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            s_1 = 0 as libc::c_int;
            while s_1 < (*st).layout.nb_streams {
                let mut enc_2: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
                    0 as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                enc_2 = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                if s_1 < (*st).layout.nb_coupled_streams {
                    ptr = ptr.offset(align(coupled_size) as isize)
                } else {
                    ptr = ptr.offset(align(mono_size) as isize)
                }
                ret = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc_2, request, value_3,
                );
                if ret != 0 as libc::c_int {
                    break;
                }
                s_1 += 1
            }
            current_block = 1677945370889843322;
        }
        5120 => {
            let mut s_2: libc::c_int = 0;
            let mut stream_id: crate::opus_types_h::opus_int32 = 0;
            let mut value_4: *mut *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
                0 as *mut *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
            stream_id = ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if stream_id < 0 as libc::c_int || stream_id >= (*st).layout.nb_streams {
                ret = -(1 as libc::c_int)
            }
            value_4 = ap
                .as_va_list()
                .arg::<*mut *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder>();
            if value_4.is_null() {
                current_block = 16375338222180917333;
            } else {
                s_2 = 0 as libc::c_int;
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
            let mut value_5: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            (*st).variable_duration = value_5;
            current_block = 1677945370889843322;
        }
        4041 => {
            let mut value_6: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_6.is_null() {
                current_block = 16375338222180917333;
            } else {
                *value_6 = (*st).variable_duration;
                current_block = 1677945370889843322;
            }
        }
        4028 => {
            let mut s_3: libc::c_int = 0;
            if (*st).mapping_type as libc::c_uint
                == MAPPING_TYPE_SURROUND as libc::c_int as libc::c_uint
            {
                crate::stdlib::memset(
                    ms_get_preemph_mem(st) as *mut libc::c_void,
                    0 as libc::c_int,
                    ((*st).layout.nb_channels as libc::c_ulong).wrapping_mul(
                        ::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong,
                    ),
                );
                crate::stdlib::memset(
                    ms_get_window_mem(st) as *mut libc::c_void,
                    0 as libc::c_int,
                    (((*st).layout.nb_channels * 120 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong
                        ),
                );
            }
            s_3 = 0 as libc::c_int;
            while s_3 < (*st).layout.nb_streams {
                let mut enc_3: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder =
                    0 as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                enc_3 = ptr as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder;
                if s_3 < (*st).layout.nb_coupled_streams {
                    ptr = ptr.offset(align(coupled_size) as isize)
                } else {
                    ptr = ptr.offset(align(mono_size) as isize)
                }
                ret = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
                    enc_3,
                    4028 as libc::c_int,
                );
                if ret != 0 as libc::c_int {
                    break;
                }
                s_3 += 1
            }
            current_block = 1677945370889843322;
        }
        _ => {
            ret = -(5 as libc::c_int);
            current_block = 1677945370889843322;
        }
    }
    match current_block {
        16375338222180917333 => return -(1 as libc::c_int),
        _ => return ret,
    };
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_encoder_destroy(mut st: *mut OpusMSEncoder) {
    opus_free(st as *mut libc::c_void);
}

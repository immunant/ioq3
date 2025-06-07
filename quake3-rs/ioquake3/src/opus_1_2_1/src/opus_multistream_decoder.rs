use ::libc;

pub mod opus_private_h {

    /* * Configures the encoder's expected percentage of voice
     * opposed to music or other signals.
     *
     * @note This interface is currently more aspiration than actuality. It's
     * ultimately expected to bias an automatic signal classifier, but it currently
     * just shifts the static bitrate to mode mapping around a little bit.
     *
     * @param[in] x <tt>int</tt>:   Voice percentage in the range 0-100, inclusive.
     * @hideinitializer */
    /* * Gets the encoder's configured voice ratio value, @see OPUS_SET_VOICE_RATIO
     *
     * @param[out] x <tt>int*</tt>:  Voice percentage in the range 0-100, inclusive.
     * @hideinitializer */
    /* Make sure everything is properly aligned. */
    #[inline]

    pub unsafe extern "C" fn align(mut i: libc::c_int) -> libc::c_int {
        let mut alignment: libc::c_uint = 8 as libc::c_ulong as libc::c_uint;
        /* Optimizing compilers should optimize div and multiply into and
        for all sensible alignment values. */
        return (i as libc::c_uint)
            .wrapping_add(alignment)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(alignment)
            .wrapping_mul(alignment) as libc::c_int;
    }

    /* OPUS_PRIVATE_H */
}

pub mod float_cast_h {
    /* Copyright (C) 2001 Erik de Castro Lopo <erikd AT mega-nerd DOT com> */
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
    /* Version 1.1 */
    /*============================================================================
     **      On Intel Pentium processors (especially PIII and probably P4), converting
     **      from float to int is very slow. To meet the C specs, the code produced by
     **      most C compilers targeting Pentium needs to change the FPU rounding mode
     **      before the float to int conversion is performed.
     **
     **      Changing the FPU rounding mode causes the FPU pipeline to be flushed. It
     **      is this flushing of the pipeline which is so slow.
     **
     **      Fortunately the ISO C99 specifications define the functions lrint, lrintf,
     **      llrint and llrintf which fix this problem as a side effect.
     **
     **      On Unix-like systems, the configure process should have detected the
     **      presence of these functions. If they weren't found we have to replace them
     **      here with a standard C cast.
     */
    /*
     **      The C99 prototypes for lrint and lrintf are as follows:
     **
     **              long int lrintf (float x) ;
     **              long int lrint  (double x) ;
     */
    /*      The presence of the required functions are detected during the configure
     **      process and the values HAVE_LRINT and HAVE_LRINTF are set accordingly in
     **      the config.h file.
     */
    /* With GCC, when SSE is available, the fastest conversion is cvtss2si. */
    #[inline]

    pub unsafe extern "C" fn float2int(mut x: libc::c_float) -> crate::opus_types_h::opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }
    #[inline]

    pub unsafe extern "C" fn FLOAT2INT16(mut x: libc::c_float) -> crate::opus_types_h::opus_int16 {
        x = x * 32768.0f32;
        x = if x > -(32768 as libc::c_int) as libc::c_float {
            x
        } else {
            -(32768 as libc::c_int) as libc::c_float
        };
        x = if x < 32767 as libc::c_int as libc::c_float {
            x
        } else {
            32767 as libc::c_int as libc::c_float
        };
        return float2int(x) as crate::opus_types_h::opus_int16;
    }

    use ::std::arch::x86_64::_mm_cvt_ss2si;
    use ::std::arch::x86_64::_mm_set_ss;
    /* FLOAT_CAST_H */
    /* DISABLE_FLOAT_API */
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

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::opus_private_h::foo;
pub use crate::opus_private_h::C2RustUnnamed_98;
pub use crate::opus_private_h::ChannelLayout;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::src::opus::opus_packet_parse_impl;
pub use crate::src::opus_1_2_1::src::opus_decoder::opus_decode_native;

pub use crate::src::opus_1_2_1::src::opus_multistream::get_left_channel;
pub use crate::src::opus_1_2_1::src::opus_multistream::get_mono_channel;
pub use crate::src::opus_1_2_1::src::opus_multistream::get_right_channel;
pub use crate::src::opus_1_2_1::src::opus_multistream::validate_layout;
pub use crate::src::opus_1_2_1::src::opus_multistream_decoder::opus_private_h::align;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;

pub use crate::src::opus_1_2_1::src::opus_multistream_decoder::float_cast_h::float2int;
pub use crate::src::opus_1_2_1::src::opus_multistream_decoder::float_cast_h::FLOAT2INT16;
pub use crate::src::opus_1_2_1::src::opus_multistream_decoder::os_support_h::opus_alloc;
pub use crate::src::opus_1_2_1::src::opus_multistream_decoder::os_support_h::opus_free;

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
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusMSDecoder {
    pub layout: crate::opus_private_h::ChannelLayout,
}

pub type opus_copy_channel_out_func = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_int,
        _: *const crate::arch_h::opus_val16,
        _: libc::c_int,
        _: libc::c_int,
    ) -> (),
>;
/* Decoder states go here */
/* DECODER */
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_decoder_get_size(
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
        crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_get_size(2 as libc::c_int);
    mono_size = crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_get_size(1 as libc::c_int);
    return align(::std::mem::size_of::<OpusMSDecoder>() as libc::c_ulong as libc::c_int)
        + nb_coupled_streams * align(coupled_size)
        + (nb_streams - nb_coupled_streams) * align(mono_size);
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_decoder_init(
    mut st: *mut OpusMSDecoder,
    mut Fs: crate::opus_types_h::opus_int32,
    mut channels: libc::c_int,
    mut streams: libc::c_int,
    mut coupled_streams: libc::c_int,
    mut mapping: *const libc::c_uchar,
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
    (*st).layout.nb_channels = channels;
    (*st).layout.nb_streams = streams;
    (*st).layout.nb_coupled_streams = coupled_streams;
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
    ptr = (st as *mut libc::c_char).offset(align(
        ::std::mem::size_of::<OpusMSDecoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    coupled_size =
        crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_get_size(2 as libc::c_int);
    mono_size = crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_get_size(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*st).layout.nb_coupled_streams {
        ret = crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_init(
            ptr as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder,
            Fs,
            2 as libc::c_int,
        );
        if ret != 0 as libc::c_int {
            return ret;
        }
        ptr = ptr.offset(align(coupled_size) as isize);
        i += 1
    }
    while i < (*st).layout.nb_streams {
        ret = crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_init(
            ptr as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder,
            Fs,
            1 as libc::c_int,
        );
        if ret != 0 as libc::c_int {
            return ret;
        }
        ptr = ptr.offset(align(mono_size) as isize);
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_decoder_create(
    mut Fs: crate::opus_types_h::opus_int32,
    mut channels: libc::c_int,
    mut streams: libc::c_int,
    mut coupled_streams: libc::c_int,
    mut mapping: *const libc::c_uchar,
    mut error: *mut libc::c_int,
) -> *mut OpusMSDecoder {
    let mut ret: libc::c_int = 0;
    let mut st: *mut OpusMSDecoder = 0 as *mut OpusMSDecoder;
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
        return 0 as *mut OpusMSDecoder;
    }
    st = opus_alloc(
        opus_multistream_decoder_get_size(streams, coupled_streams) as crate::stddef_h::size_t
    ) as *mut OpusMSDecoder;
    if st.is_null() {
        if !error.is_null() {
            *error = -(7 as libc::c_int)
        }
        return 0 as *mut OpusMSDecoder;
    }
    ret = opus_multistream_decoder_init(st, Fs, channels, streams, coupled_streams, mapping);
    if !error.is_null() {
        *error = ret
    }
    if ret != 0 as libc::c_int {
        opus_free(st as *mut libc::c_void);
        st = 0 as *mut OpusMSDecoder
    }
    return st;
}

unsafe extern "C" fn opus_multistream_packet_validate(
    mut data: *const libc::c_uchar,
    mut len: crate::opus_types_h::opus_int32,
    mut nb_streams: libc::c_int,
    mut Fs: crate::opus_types_h::opus_int32,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut toc: libc::c_uchar = 0;
    let mut size: [crate::opus_types_h::opus_int16; 48] = [0; 48];
    let mut samples: libc::c_int = 0 as libc::c_int;
    let mut packet_offset: crate::opus_types_h::opus_int32 = 0;
    s = 0 as libc::c_int;
    while s < nb_streams {
        let mut tmp_samples: libc::c_int = 0;
        if len <= 0 as libc::c_int {
            return -(4 as libc::c_int);
        }
        count = crate::src::opus_1_2_1::src::opus::opus_packet_parse_impl(
            data,
            len,
            (s != nb_streams - 1 as libc::c_int) as libc::c_int,
            &mut toc,
            0 as *mut *const libc::c_uchar,
            size.as_mut_ptr(),
            0 as *mut libc::c_int,
            &mut packet_offset,
        );
        if count < 0 as libc::c_int {
            return count;
        }
        tmp_samples = crate::src::opus_1_2_1::src::opus_decoder::opus_packet_get_nb_samples(
            data,
            packet_offset,
            Fs,
        );
        if s != 0 as libc::c_int && samples != tmp_samples {
            return -(4 as libc::c_int);
        }
        samples = tmp_samples;
        data = data.offset(packet_offset as isize);
        len -= packet_offset;
        s += 1
    }
    return samples;
}

unsafe extern "C" fn opus_multistream_decode_native(
    mut st: *mut OpusMSDecoder,
    mut data: *const libc::c_uchar,
    mut len: crate::opus_types_h::opus_int32,
    mut pcm: *mut libc::c_void,
    mut copy_channel_out: opus_copy_channel_out_func,
    mut frame_size: libc::c_int,
    mut decode_fec: libc::c_int,
    mut soft_clip: libc::c_int,
) -> libc::c_int {
    let mut Fs: crate::opus_types_h::opus_int32 = 0;
    let mut coupled_size: libc::c_int = 0;
    let mut mono_size: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut do_plc: libc::c_int = 0 as libc::c_int;
    let mut buf: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    /* Limit frame_size to avoid excessive stack allocations. */
    opus_multistream_decoder_ctl(
        st,
        4029 as libc::c_int,
        (&mut Fs as *mut crate::opus_types_h::opus_int32).offset(
            (&mut Fs as *mut crate::opus_types_h::opus_int32)
                .wrapping_offset_from(&mut Fs as *mut crate::opus_types_h::opus_int32)
                as libc::c_long as isize,
        ),
    );
    frame_size = if frame_size < Fs / 25 as libc::c_int * 3 as libc::c_int {
        frame_size
    } else {
        (Fs / 25 as libc::c_int) * 3 as libc::c_int
    };
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul((2 as libc::c_int * frame_size) as libc::c_ulong) as usize,
    );
    buf = fresh0.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    ptr = (st as *mut libc::c_char).offset(align(
        ::std::mem::size_of::<OpusMSDecoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    coupled_size =
        crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_get_size(2 as libc::c_int);
    mono_size = crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_get_size(1 as libc::c_int);
    if len == 0 as libc::c_int {
        do_plc = 1 as libc::c_int
    }
    if len < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if do_plc == 0 && len < 2 as libc::c_int * (*st).layout.nb_streams - 1 as libc::c_int {
        return -(4 as libc::c_int);
    }
    if do_plc == 0 {
        let mut ret: libc::c_int =
            opus_multistream_packet_validate(data, len, (*st).layout.nb_streams, Fs);
        if ret < 0 as libc::c_int {
            return ret;
        } else {
            if ret > frame_size {
                return -(2 as libc::c_int);
            }
        }
    }
    s = 0 as libc::c_int;
    while s < (*st).layout.nb_streams {
        let mut dec: *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder =
            0 as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
        let mut packet_offset: crate::opus_types_h::opus_int32 = 0;
        let mut ret_0: libc::c_int = 0;
        dec = ptr as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
        ptr = ptr.offset(if s < (*st).layout.nb_coupled_streams {
            align(coupled_size)
        } else {
            align(mono_size)
        } as isize);
        if do_plc == 0 && len <= 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        packet_offset = 0 as libc::c_int;
        ret_0 = crate::src::opus_1_2_1::src::opus_decoder::opus_decode_native(
            dec,
            data,
            len,
            buf,
            frame_size,
            decode_fec,
            (s != (*st).layout.nb_streams - 1 as libc::c_int) as libc::c_int,
            &mut packet_offset,
            soft_clip,
        );
        data = data.offset(packet_offset as isize);
        len -= packet_offset;
        if ret_0 <= 0 as libc::c_int {
            return ret_0;
        }
        frame_size = ret_0;
        if s < (*st).layout.nb_coupled_streams {
            let mut chan: libc::c_int = 0;
            let mut prev: libc::c_int = 0;
            prev = -(1 as libc::c_int);
            loop
            /* Copy "left" audio to the channel(s) where it belongs */
            {
                chan = crate::src::opus_1_2_1::src::opus_multistream::get_left_channel(
                    &mut (*st).layout as *mut _ as *const crate::opus_private_h::ChannelLayout,
                    s,
                    prev,
                );
                if !(chan != -(1 as libc::c_int)) {
                    break;
                }
                Some(copy_channel_out.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    pcm,
                    (*st).layout.nb_channels,
                    chan,
                    buf,
                    2 as libc::c_int,
                    frame_size,
                );
                prev = chan
            }
            prev = -(1 as libc::c_int);
            loop
            /* Copy "right" audio to the channel(s) where it belongs */
            {
                chan = crate::src::opus_1_2_1::src::opus_multistream::get_right_channel(
                    &mut (*st).layout as *mut _ as *const crate::opus_private_h::ChannelLayout,
                    s,
                    prev,
                );
                if !(chan != -(1 as libc::c_int)) {
                    break;
                }
                Some(copy_channel_out.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    pcm,
                    (*st).layout.nb_channels,
                    chan,
                    buf.offset(1 as libc::c_int as isize),
                    2 as libc::c_int,
                    frame_size,
                );
                prev = chan
            }
        } else {
            let mut chan_0: libc::c_int = 0;
            let mut prev_0: libc::c_int = 0;
            prev_0 = -(1 as libc::c_int);
            loop
            /* Copy audio to the channel(s) where it belongs */
            {
                chan_0 = crate::src::opus_1_2_1::src::opus_multistream::get_mono_channel(
                    &mut (*st).layout as *mut _ as *const crate::opus_private_h::ChannelLayout,
                    s,
                    prev_0,
                );
                if !(chan_0 != -(1 as libc::c_int)) {
                    break;
                }
                Some(copy_channel_out.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    pcm,
                    (*st).layout.nb_channels,
                    chan_0,
                    buf,
                    1 as libc::c_int,
                    frame_size,
                );
                prev_0 = chan_0
            }
        }
        s += 1
    }
    /* Handle muted channels */
    c = 0 as libc::c_int;
    while c < (*st).layout.nb_channels {
        if (*st).layout.mapping[c as usize] as libc::c_int == 255 as libc::c_int {
            Some(copy_channel_out.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                pcm,
                (*st).layout.nb_channels,
                c,
                0 as *const crate::arch_h::opus_val16,
                0 as libc::c_int,
                frame_size,
            );
        }
        c += 1
    }
    return frame_size;
}

unsafe extern "C" fn opus_copy_channel_out_float(
    mut dst: *mut libc::c_void,
    mut dst_stride: libc::c_int,
    mut dst_channel: libc::c_int,
    mut src: *const crate::arch_h::opus_val16,
    mut src_stride: libc::c_int,
    mut frame_size: libc::c_int,
) {
    let mut float_dst: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: crate::opus_types_h::opus_int32 = 0;
    float_dst = dst as *mut libc::c_float;
    if !src.is_null() {
        i = 0 as libc::c_int;
        while i < frame_size {
            *float_dst.offset((i * dst_stride + dst_channel) as isize) =
                *src.offset((i * src_stride) as isize);
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < frame_size {
            *float_dst.offset((i * dst_stride + dst_channel) as isize) =
                0 as libc::c_int as libc::c_float;
            i += 1
        }
    };
}

unsafe extern "C" fn opus_copy_channel_out_short(
    mut dst: *mut libc::c_void,
    mut dst_stride: libc::c_int,
    mut dst_channel: libc::c_int,
    mut src: *const crate::arch_h::opus_val16,
    mut src_stride: libc::c_int,
    mut frame_size: libc::c_int,
) {
    let mut short_dst: *mut crate::opus_types_h::opus_int16 =
        0 as *mut crate::opus_types_h::opus_int16;
    let mut i: crate::opus_types_h::opus_int32 = 0;
    short_dst = dst as *mut crate::opus_types_h::opus_int16;
    if !src.is_null() {
        i = 0 as libc::c_int;
        while i < frame_size {
            *short_dst.offset((i * dst_stride + dst_channel) as isize) =
                FLOAT2INT16(*src.offset((i * src_stride) as isize));
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < frame_size {
            *short_dst.offset((i * dst_stride + dst_channel) as isize) =
                0 as libc::c_int as crate::opus_types_h::opus_int16;
            i += 1
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_decode(
    mut st: *mut OpusMSDecoder,
    mut data: *const libc::c_uchar,
    mut len: crate::opus_types_h::opus_int32,
    mut pcm: *mut crate::opus_types_h::opus_int16,
    mut frame_size: libc::c_int,
    mut decode_fec: libc::c_int,
) -> libc::c_int {
    return opus_multistream_decode_native(
        st,
        data,
        len,
        pcm as *mut libc::c_void,
        Some(
            opus_copy_channel_out_short
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: *const crate::arch_h::opus_val16,
                    _: libc::c_int,
                    _: libc::c_int,
                ) -> (),
        ),
        frame_size,
        decode_fec,
        1 as libc::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_decode_float(
    mut st: *mut OpusMSDecoder,
    mut data: *const libc::c_uchar,
    mut len: crate::opus_types_h::opus_int32,
    mut pcm: *mut libc::c_float,
    mut frame_size: libc::c_int,
    mut decode_fec: libc::c_int,
) -> libc::c_int {
    return opus_multistream_decode_native(
        st,
        data,
        len,
        pcm as *mut libc::c_void,
        Some(
            opus_copy_channel_out_float
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: *const crate::arch_h::opus_val16,
                    _: libc::c_int,
                    _: libc::c_int,
                ) -> (),
        ),
        frame_size,
        decode_fec,
        0 as libc::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_decoder_ctl(
    mut st: *mut OpusMSDecoder,
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
        crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_get_size(2 as libc::c_int);
    mono_size = crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_get_size(1 as libc::c_int);
    ptr = (st as *mut libc::c_char).offset(align(
        ::std::mem::size_of::<OpusMSDecoder>() as libc::c_ulong as libc::c_int
    ) as isize);
    match request {
        4009 | 4029 | 4045 | 4039 | 4047 => {
            let mut dec: *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder =
                0 as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
            /* For int32* GET params, just query the first stream */
            let mut value: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            dec = ptr as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
            ret = crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_ctl(dec, request, value);
            current_block = 7330218953828964527;
        }
        4031 => {
            let mut s: libc::c_int = 0;
            let mut value_0: *mut crate::opus_types_h::opus_uint32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_uint32>();
            let mut tmp: crate::opus_types_h::opus_uint32 = 0;
            if value_0.is_null() {
                current_block = 14298507163138330979;
            } else {
                *value_0 = 0 as libc::c_int as crate::opus_types_h::opus_uint32;
                s = 0 as libc::c_int;
                while s < (*st).layout.nb_streams {
                    let mut dec_0: *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder =
                        0 as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
                    dec_0 = ptr as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
                    if s < (*st).layout.nb_coupled_streams {
                        ptr = ptr.offset(align(coupled_size) as isize)
                    } else {
                        ptr = ptr.offset(align(mono_size) as isize)
                    }
                    ret = crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_ctl(
                        dec_0,
                        request,
                        &mut tmp as *mut crate::opus_types_h::opus_uint32,
                    );
                    if ret != 0 as libc::c_int {
                        break;
                    }
                    *value_0 ^= tmp;
                    s += 1
                }
                current_block = 7330218953828964527;
            }
        }
        4028 => {
            let mut s_0: libc::c_int = 0;
            s_0 = 0 as libc::c_int;
            while s_0 < (*st).layout.nb_streams {
                let mut dec_1: *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder =
                    0 as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
                dec_1 = ptr as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
                if s_0 < (*st).layout.nb_coupled_streams {
                    ptr = ptr.offset(align(coupled_size) as isize)
                } else {
                    ptr = ptr.offset(align(mono_size) as isize)
                }
                ret = crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_ctl(
                    dec_1,
                    4028 as libc::c_int,
                );
                if ret != 0 as libc::c_int {
                    break;
                }
                s_0 += 1
            }
            current_block = 7330218953828964527;
        }
        5122 => {
            let mut s_1: libc::c_int = 0;
            let mut stream_id: crate::opus_types_h::opus_int32 = 0;
            let mut value_1: *mut *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder =
                0 as *mut *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
            stream_id = ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if stream_id < 0 as libc::c_int || stream_id >= (*st).layout.nb_streams {
                ret = -(1 as libc::c_int)
            }
            value_1 = ap
                .as_va_list()
                .arg::<*mut *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder>();
            if value_1.is_null() {
                current_block = 14298507163138330979;
            } else {
                s_1 = 0 as libc::c_int;
                while s_1 < stream_id {
                    if s_1 < (*st).layout.nb_coupled_streams {
                        ptr = ptr.offset(align(coupled_size) as isize)
                    } else {
                        ptr = ptr.offset(align(mono_size) as isize)
                    }
                    s_1 += 1
                }
                *value_1 = ptr as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
                current_block = 7330218953828964527;
            }
        }
        4034 | 4046 => {
            let mut s_2: libc::c_int = 0;
            /* This works for int32 params */
            let mut value_2: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            s_2 = 0 as libc::c_int;
            while s_2 < (*st).layout.nb_streams {
                let mut dec_2: *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder =
                    0 as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
                dec_2 = ptr as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder;
                if s_2 < (*st).layout.nb_coupled_streams {
                    ptr = ptr.offset(align(coupled_size) as isize)
                } else {
                    ptr = ptr.offset(align(mono_size) as isize)
                }
                ret = crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_ctl(
                    dec_2, request, value_2,
                );
                if ret != 0 as libc::c_int {
                    break;
                }
                s_2 += 1
            }
            current_block = 7330218953828964527;
        }
        _ => {
            ret = -(5 as libc::c_int);
            current_block = 7330218953828964527;
        }
    }
    match current_block {
        14298507163138330979 => return -(1 as libc::c_int),
        _ => return ret,
    };
}
/* * Initialize a previously allocated multistream encoder state.
 * The memory pointed to by \a st must be at least the size returned by
 * opus_multistream_encoder_get_size().
 * This is intended for applications which use their own allocator instead of
 * malloc.
 * To reset a previously initialized state, use the #OPUS_RESET_STATE CTL.
 * @see opus_multistream_encoder_create
 * @see opus_multistream_encoder_get_size
 * @param st <tt>OpusMSEncoder*</tt>: Multistream encoder state to initialize.
 * @param Fs <tt>opus_int32</tt>: Sampling rate of the input signal (in Hz).
 *                                This must be one of 8000, 12000, 16000,
 *                                24000, or 48000.
 * @param channels <tt>int</tt>: Number of channels in the input signal.
 *                               This must be at most 255.
 *                               It may be greater than the number of
 *                               coded channels (<code>streams +
 *                               coupled_streams</code>).
 * @param streams <tt>int</tt>: The total number of streams to encode from the
 *                              input.
 *                              This must be no more than the number of channels.
 * @param coupled_streams <tt>int</tt>: Number of coupled (2 channel) streams
 *                                      to encode.
 *                                      This must be no larger than the total
 *                                      number of streams.
 *                                      Additionally, The total number of
 *                                      encoded channels (<code>streams +
 *                                      coupled_streams</code>) must be no
 *                                      more than the number of input channels.
 * @param[in] mapping <code>const unsigned char[channels]</code>: Mapping from
 *                    encoded channels to input channels, as described in
 *                    @ref opus_multistream. As an extra constraint, the
 *                    multistream encoder does not allow encoding coupled
 *                    streams for which one channel is unused since this
 *                    is never a good idea.
 * @param application <tt>int</tt>: The target encoder application.
 *                                  This must be one of the following:
 * <dl>
 * <dt>#OPUS_APPLICATION_VOIP</dt>
 * <dd>Process signal for improved speech intelligibility.</dd>
 * <dt>#OPUS_APPLICATION_AUDIO</dt>
 * <dd>Favor faithfulness to the original input.</dd>
 * <dt>#OPUS_APPLICATION_RESTRICTED_LOWDELAY</dt>
 * <dd>Configure the minimum possible coding delay by disabling certain modes
 * of operation.</dd>
 * </dl>
 * @returns #OPUS_OK on success, or an error code (see @ref opus_errorcodes)
 *          on failure.
 */
/* * Encodes a multistream Opus frame.
 * @param st <tt>OpusMSEncoder*</tt>: Multistream encoder state.
 * @param[in] pcm <tt>const opus_int16*</tt>: The input signal as interleaved
 *                                            samples.
 *                                            This must contain
 *                                            <code>frame_size*channels</code>
 *                                            samples.
 * @param frame_size <tt>int</tt>: Number of samples per channel in the input
 *                                 signal.
 *                                 This must be an Opus frame size for the
 *                                 encoder's sampling rate.
 *                                 For example, at 48 kHz the permitted values
 *                                 are 120, 240, 480, 960, 1920, and 2880.
 *                                 Passing in a duration of less than 10 ms
 *                                 (480 samples at 48 kHz) will prevent the
 *                                 encoder from using the LPC or hybrid modes.
 * @param[out] data <tt>unsigned char*</tt>: Output payload.
 *                                           This must contain storage for at
 *                                           least \a max_data_bytes.
 * @param [in] max_data_bytes <tt>opus_int32</tt>: Size of the allocated
 *                                                 memory for the output
 *                                                 payload. This may be
 *                                                 used to impose an upper limit on
 *                                                 the instant bitrate, but should
 *                                                 not be used as the only bitrate
 *                                                 control. Use #OPUS_SET_BITRATE to
 *                                                 control the bitrate.
 * @returns The length of the encoded packet (in bytes) on success or a
 *          negative error code (see @ref opus_errorcodes) on failure.
 */
/* * Encodes a multistream Opus frame from floating point input.
 * @param st <tt>OpusMSEncoder*</tt>: Multistream encoder state.
 * @param[in] pcm <tt>const float*</tt>: The input signal as interleaved
 *                                       samples with a normal range of
 *                                       +/-1.0.
 *                                       Samples with a range beyond +/-1.0
 *                                       are supported but will be clipped by
 *                                       decoders using the integer API and
 *                                       should only be used if it is known
 *                                       that the far end supports extended
 *                                       dynamic range.
 *                                       This must contain
 *                                       <code>frame_size*channels</code>
 *                                       samples.
 * @param frame_size <tt>int</tt>: Number of samples per channel in the input
 *                                 signal.
 *                                 This must be an Opus frame size for the
 *                                 encoder's sampling rate.
 *                                 For example, at 48 kHz the permitted values
 *                                 are 120, 240, 480, 960, 1920, and 2880.
 *                                 Passing in a duration of less than 10 ms
 *                                 (480 samples at 48 kHz) will prevent the
 *                                 encoder from using the LPC or hybrid modes.
 * @param[out] data <tt>unsigned char*</tt>: Output payload.
 *                                           This must contain storage for at
 *                                           least \a max_data_bytes.
 * @param [in] max_data_bytes <tt>opus_int32</tt>: Size of the allocated
 *                                                 memory for the output
 *                                                 payload. This may be
 *                                                 used to impose an upper limit on
 *                                                 the instant bitrate, but should
 *                                                 not be used as the only bitrate
 *                                                 control. Use #OPUS_SET_BITRATE to
 *                                                 control the bitrate.
 * @returns The length of the encoded packet (in bytes) on success or a
 *          negative error code (see @ref opus_errorcodes) on failure.
 */
/* * Frees an <code>OpusMSEncoder</code> allocated by
 * opus_multistream_encoder_create().
 * @param st <tt>OpusMSEncoder*</tt>: Multistream encoder state to be freed.
 */
/* * Perform a CTL function on a multistream Opus encoder.
 *
 * Generally the request and subsequent arguments are generated by a
 * convenience macro.
 * @param st <tt>OpusMSEncoder*</tt>: Multistream encoder state.
 * @param request This and all remaining parameters should be replaced by one
 *                of the convenience macros in @ref opus_genericctls,
 *                @ref opus_encoderctls, or @ref opus_multistream_ctls.
 * @see opus_genericctls
 * @see opus_encoderctls
 * @see opus_multistream_ctls
 */
/* *@}*/
/* *\name Multistream decoder functions */
/* *@{*/
/* * Gets the size of an <code>OpusMSDecoder</code> structure.
 * @param streams <tt>int</tt>: The total number of streams coded in the
 *                              input.
 *                              This must be no more than 255.
 * @param coupled_streams <tt>int</tt>: Number streams to decode as coupled
 *                                      (2 channel) streams.
 *                                      This must be no larger than the total
 *                                      number of streams.
 *                                      Additionally, The total number of
 *                                      coded channels (<code>streams +
 *                                      coupled_streams</code>) must be no
 *                                      more than 255.
 * @returns The size in bytes on success, or a negative error code
 *          (see @ref opus_errorcodes) on error.
 */
/* * Allocates and initializes a multistream decoder state.
 * Call opus_multistream_decoder_destroy() to release
 * this object when finished.
 * @param Fs <tt>opus_int32</tt>: Sampling rate to decode at (in Hz).
 *                                This must be one of 8000, 12000, 16000,
 *                                24000, or 48000.
 * @param channels <tt>int</tt>: Number of channels to output.
 *                               This must be at most 255.
 *                               It may be different from the number of coded
 *                               channels (<code>streams +
 *                               coupled_streams</code>).
 * @param streams <tt>int</tt>: The total number of streams coded in the
 *                              input.
 *                              This must be no more than 255.
 * @param coupled_streams <tt>int</tt>: Number of streams to decode as coupled
 *                                      (2 channel) streams.
 *                                      This must be no larger than the total
 *                                      number of streams.
 *                                      Additionally, The total number of
 *                                      coded channels (<code>streams +
 *                                      coupled_streams</code>) must be no
 *                                      more than 255.
 * @param[in] mapping <code>const unsigned char[channels]</code>: Mapping from
 *                    coded channels to output channels, as described in
 *                    @ref opus_multistream.
 * @param[out] error <tt>int *</tt>: Returns #OPUS_OK on success, or an error
 *                                   code (see @ref opus_errorcodes) on
 *                                   failure.
 */
/* * Intialize a previously allocated decoder state object.
 * The memory pointed to by \a st must be at least the size returned by
 * opus_multistream_encoder_get_size().
 * This is intended for applications which use their own allocator instead of
 * malloc.
 * To reset a previously initialized state, use the #OPUS_RESET_STATE CTL.
 * @see opus_multistream_decoder_create
 * @see opus_multistream_deocder_get_size
 * @param st <tt>OpusMSEncoder*</tt>: Multistream encoder state to initialize.
 * @param Fs <tt>opus_int32</tt>: Sampling rate to decode at (in Hz).
 *                                This must be one of 8000, 12000, 16000,
 *                                24000, or 48000.
 * @param channels <tt>int</tt>: Number of channels to output.
 *                               This must be at most 255.
 *                               It may be different from the number of coded
 *                               channels (<code>streams +
 *                               coupled_streams</code>).
 * @param streams <tt>int</tt>: The total number of streams coded in the
 *                              input.
 *                              This must be no more than 255.
 * @param coupled_streams <tt>int</tt>: Number of streams to decode as coupled
 *                                      (2 channel) streams.
 *                                      This must be no larger than the total
 *                                      number of streams.
 *                                      Additionally, The total number of
 *                                      coded channels (<code>streams +
 *                                      coupled_streams</code>) must be no
 *                                      more than 255.
 * @param[in] mapping <code>const unsigned char[channels]</code>: Mapping from
 *                    coded channels to output channels, as described in
 *                    @ref opus_multistream.
 * @returns #OPUS_OK on success, or an error code (see @ref opus_errorcodes)
 *          on failure.
 */
/* * Decode a multistream Opus packet.
 * @param st <tt>OpusMSDecoder*</tt>: Multistream decoder state.
 * @param[in] data <tt>const unsigned char*</tt>: Input payload.
 *                                                Use a <code>NULL</code>
 *                                                pointer to indicate packet
 *                                                loss.
 * @param len <tt>opus_int32</tt>: Number of bytes in payload.
 * @param[out] pcm <tt>opus_int16*</tt>: Output signal, with interleaved
 *                                       samples.
 *                                       This must contain room for
 *                                       <code>frame_size*channels</code>
 *                                       samples.
 * @param frame_size <tt>int</tt>: The number of samples per channel of
 *                                 available space in \a pcm.
 *                                 If this is less than the maximum packet duration
 *                                 (120 ms; 5760 for 48kHz), this function will not be capable
 *                                 of decoding some packets. In the case of PLC (data==NULL)
 *                                 or FEC (decode_fec=1), then frame_size needs to be exactly
 *                                 the duration of audio that is missing, otherwise the
 *                                 decoder will not be in the optimal state to decode the
 *                                 next incoming packet. For the PLC and FEC cases, frame_size
 *                                 <b>must</b> be a multiple of 2.5 ms.
 * @param decode_fec <tt>int</tt>: Flag (0 or 1) to request that any in-band
 *                                 forward error correction data be decoded.
 *                                 If no such data is available, the frame is
 *                                 decoded as if it were lost.
 * @returns Number of samples decoded on success or a negative error code
 *          (see @ref opus_errorcodes) on failure.
 */
/* * Decode a multistream Opus packet with floating point output.
 * @param st <tt>OpusMSDecoder*</tt>: Multistream decoder state.
 * @param[in] data <tt>const unsigned char*</tt>: Input payload.
 *                                                Use a <code>NULL</code>
 *                                                pointer to indicate packet
 *                                                loss.
 * @param len <tt>opus_int32</tt>: Number of bytes in payload.
 * @param[out] pcm <tt>opus_int16*</tt>: Output signal, with interleaved
 *                                       samples.
 *                                       This must contain room for
 *                                       <code>frame_size*channels</code>
 *                                       samples.
 * @param frame_size <tt>int</tt>: The number of samples per channel of
 *                                 available space in \a pcm.
 *                                 If this is less than the maximum packet duration
 *                                 (120 ms; 5760 for 48kHz), this function will not be capable
 *                                 of decoding some packets. In the case of PLC (data==NULL)
 *                                 or FEC (decode_fec=1), then frame_size needs to be exactly
 *                                 the duration of audio that is missing, otherwise the
 *                                 decoder will not be in the optimal state to decode the
 *                                 next incoming packet. For the PLC and FEC cases, frame_size
 *                                 <b>must</b> be a multiple of 2.5 ms.
 * @param decode_fec <tt>int</tt>: Flag (0 or 1) to request that any in-band
 *                                 forward error correction data be decoded.
 *                                 If no such data is available, the frame is
 *                                 decoded as if it were lost.
 * @returns Number of samples decoded on success or a negative error code
 *          (see @ref opus_errorcodes) on failure.
 */
/* * Perform a CTL function on a multistream Opus decoder.
 *
 * Generally the request and subsequent arguments are generated by a
 * convenience macro.
 * @param st <tt>OpusMSDecoder*</tt>: Multistream decoder state.
 * @param request This and all remaining parameters should be replaced by one
 *                of the convenience macros in @ref opus_genericctls,
 *                @ref opus_decoderctls, or @ref opus_multistream_ctls.
 * @see opus_genericctls
 * @see opus_decoderctls
 * @see opus_multistream_ctls
 */
/* * Frees an <code>OpusMSDecoder</code> allocated by
 * opus_multistream_decoder_create().
 * @param st <tt>OpusMSDecoder</tt>: Multistream decoder state to be freed.
 */
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_decoder_destroy(mut st: *mut OpusMSDecoder) {
    opus_free(st as *mut libc::c_void);
}

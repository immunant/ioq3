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
            - (::std::mem::size_of::<u32>() as libc::c_ulong as i32 * 8 as i32
                - (*_this).rng.leading_zeros() as i32);
    }
}

pub mod opus_private_h {

    /* Make sure everything is properly aligned. */
    #[inline]

    pub unsafe extern "C" fn align(mut i: i32) -> i32 {
        let mut alignment: u32 = 8 as libc::c_ulong as u32;
        /* Optimizing compilers should optimize div and multiply into and
        for all sensible alignment values. */
        return (i as u32)
            .wrapping_add(alignment)
            .wrapping_sub(1 as i32 as u32)
            .wrapping_div(alignment)
            .wrapping_mul(alignment) as i32;
    }

    /* OPUS_PRIVATE_H */
}

pub mod mathops_h {

    /* * Base-2 exponential approximation (2^x). */
    #[inline]

    pub unsafe extern "C" fn celt_exp2(mut x: f32) -> f32 {
        let mut integer: i32 = 0;
        let mut frac: f32 = 0.;
        let mut res: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        integer = crate::stdlib::floor(x as f64) as i32;
        if integer < -(50 as i32) {
            return 0 as i32 as f32;
        }
        frac = x - integer as f32;
        /* K0 = 1, K1 = log(2), K2 = 3-4*log(2), K3 = 3*log(2) - 2 */
        res.f =
            0.99992522f32 + frac * (0.69583354f32 + frac * (0.22606716f32 + 0.078024523f32 * frac));
        res.i = res.i.wrapping_add((integer << 23 as i32) as u32) & 0x7fffffff as i32 as u32;
        return res.f;
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

pub mod stack_alloc_h {
    /* VAR_ARRAYS */
    #[inline]

    pub unsafe extern "C" fn _opus_false() -> i32 {
        return 0 as i32;
    }
    /* STACK_ALLOC_H */
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

    pub unsafe extern "C" fn float2int(mut x: f32) -> crate::opus_types_h::opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }
    #[inline]

    pub unsafe extern "C" fn FLOAT2INT16(mut x: f32) -> crate::opus_types_h::opus_int16 {
        x = x * 32768.0f32;
        x = if x > -(32768 as i32) as f32 {
            x
        } else {
            -(32768 as i32) as f32
        };
        x = if x < 32767 as i32 as f32 {
            x
        } else {
            32767 as i32 as f32
        };
        return float2int(x) as crate::opus_types_h::opus_int16;
    }

    use ::std::arch::x86_64::_mm_cvt_ss2si;
    use ::std::arch::x86_64::_mm_set_ss;
    /* FLOAT_CAST_H */
    /* DISABLE_FLOAT_API */
}

pub mod os_support_h {
    #[inline]

    pub unsafe extern "C" fn opus_alloc(mut size: crate::stddef_h::size_t) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
    /* * Opus wrapper for free(). To do your own dynamic allocation, all you need to do is replace this function and opus_alloc */
    #[inline]

    pub unsafe extern "C" fn opus_free(mut ptr: *mut libc::c_void) {
        ::libc::free(ptr);
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
pub use crate::stdarg_h::va_list;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;

pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;
pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;

pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::control_h::silk_DecControlStruct;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::src::opus_decoder::entcode_h::ec_tell;
pub use crate::stddef_h::size_t;

pub use crate::mathops_h::C2RustUnnamed_61;
pub use crate::opus_private_h::foo;
pub use crate::opus_private_h::C2RustUnnamed_98;
pub use crate::src::opus_1_2_1::src::opus::opus_packet_parse_impl;
pub use crate::src::opus_1_2_1::src::opus_decoder::mathops_h::celt_exp2;
pub use crate::src::opus_1_2_1::src::opus_decoder::opus_private_h::align;

pub use crate::src::opus_1_2_1::src::opus_decoder::cpu_support_h::opus_select_arch;
pub use crate::src::opus_1_2_1::src::opus_decoder::stack_alloc_h::_opus_false;

pub use crate::src::opus_1_2_1::src::opus_decoder::float_cast_h::float2int;
pub use crate::src::opus_1_2_1::src::opus_decoder::float_cast_h::FLOAT2INT16;
pub use crate::src::opus_1_2_1::src::opus_decoder::os_support_h::opus_alloc;
pub use crate::src::opus_1_2_1::src::opus_decoder::os_support_h::opus_free;

/* Copyright (c) 2010 Xiph.Org Foundation, Skype Limited
Written by Jean-Marc Valin and Koen Vos */
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
pub struct OpusDecoder {
    pub celt_dec_offset: i32,
    pub silk_dec_offset: i32,
    pub channels: i32,
    pub Fs: crate::opus_types_h::opus_int32,
    pub DecControl: crate::control_h::silk_DecControlStruct,
    pub decode_gain: i32,
    pub arch: i32,
    pub stream_channels: i32,
    pub bandwidth: i32,
    pub mode: i32,
    pub prev_mode: i32,
    pub frame_size: i32,
    pub prev_redundancy: i32,
    pub last_packet_duration: i32,
    pub softclip_mem: [crate::arch_h::opus_val16; 2],
    pub rangeFinal: crate::opus_types_h::opus_uint32,
}
/* * Gets the size of an <code>OpusDecoder</code> structure.
 * @param [in] channels <tt>int</tt>: Number of channels.
 *                                    This must be 1 or 2.
 * @returns The size in bytes.
 */
#[no_mangle]

pub unsafe extern "C" fn opus_decoder_get_size(mut channels: i32) -> i32 {
    let mut silkDecSizeBytes: i32 = 0;
    let mut celtDecSizeBytes: i32 = 0;
    let mut ret: i32 = 0;
    if channels < 1 as i32 || channels > 2 as i32 {
        return 0 as i32;
    }
    ret = crate::src::opus_1_2_1::silk::dec_API::silk_Get_Decoder_Size(&mut silkDecSizeBytes);
    if ret != 0 {
        return 0 as i32;
    }
    silkDecSizeBytes = align(silkDecSizeBytes);
    celtDecSizeBytes = crate::src::opus_1_2_1::celt::celt_decoder::celt_decoder_get_size(channels);
    return align(::std::mem::size_of::<OpusDecoder>() as libc::c_ulong as i32)
        + silkDecSizeBytes
        + celtDecSizeBytes;
}
/* * Initializes a previously allocated decoder state.
 * The state must be at least the size returned by opus_decoder_get_size().
 * This is intended for applications which use their own allocator instead of malloc. @see opus_decoder_create,opus_decoder_get_size
 * To reset a previously initialized state, use the #OPUS_RESET_STATE CTL.
 * @param [in] st <tt>OpusDecoder*</tt>: Decoder state.
 * @param [in] Fs <tt>opus_int32</tt>: Sampling rate to decode to (Hz).
 *                                     This must be one of 8000, 12000, 16000,
 *                                     24000, or 48000.
 * @param [in] channels <tt>int</tt>: Number of channels (1 or 2) to decode
 * @retval #OPUS_OK Success or @ref opus_errorcodes
 */
#[no_mangle]

pub unsafe extern "C" fn opus_decoder_init(
    mut st: *mut OpusDecoder,
    mut Fs: crate::opus_types_h::opus_int32,
    mut channels: i32,
) -> i32 {
    let mut silk_dec: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut celt_dec: *mut crate::src::opus_1_2_1::celt::celt_decoder::OpusCustomDecoder =
        0 as *mut crate::src::opus_1_2_1::celt::celt_decoder::OpusCustomDecoder;
    let mut ret: i32 = 0;
    let mut silkDecSizeBytes: i32 = 0;
    if Fs != 48000 as i32
        && Fs != 24000 as i32
        && Fs != 16000 as i32
        && Fs != 12000 as i32
        && Fs != 8000 as i32
        || channels != 1 as i32 && channels != 2 as i32
    {
        return -(1 as i32);
    }
    crate::stdlib::memset(
        st as *mut libc::c_char as *mut libc::c_void,
        0 as i32,
        (opus_decoder_get_size(channels) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    /* Initialize SILK encoder */
    ret = crate::src::opus_1_2_1::silk::dec_API::silk_Get_Decoder_Size(&mut silkDecSizeBytes);
    if ret != 0 {
        return -(3 as i32);
    }
    silkDecSizeBytes = align(silkDecSizeBytes);
    (*st).silk_dec_offset = align(::std::mem::size_of::<OpusDecoder>() as libc::c_ulong as i32);
    (*st).celt_dec_offset = (*st).silk_dec_offset + silkDecSizeBytes;
    silk_dec =
        (st as *mut libc::c_char).offset((*st).silk_dec_offset as isize) as *mut libc::c_void;
    celt_dec = (st as *mut libc::c_char).offset((*st).celt_dec_offset as isize)
        as *mut crate::src::opus_1_2_1::celt::celt_decoder::OpusCustomDecoder;
    (*st).channels = channels;
    (*st).stream_channels = (*st).channels;
    (*st).Fs = Fs;
    (*st).DecControl.API_sampleRate = (*st).Fs;
    (*st).DecControl.nChannelsAPI = (*st).channels;
    /* Reset decoder */
    ret = crate::src::opus_1_2_1::silk::dec_API::silk_InitDecoder(silk_dec);
    if ret != 0 {
        return -(3 as i32);
    }
    /* Initialize CELT decoder */
    ret = crate::src::opus_1_2_1::celt::celt_decoder::celt_decoder_init(celt_dec, Fs, channels);
    if ret != 0 as i32 {
        return -(3 as i32);
    }
    crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
        celt_dec,
        10016 as i32,
        0 as i32,
    );
    (*st).prev_mode = 0 as i32;
    (*st).frame_size = Fs / 400 as i32;
    (*st).arch = opus_select_arch();
    return 0 as i32;
}
/* * Allocates and initializes a decoder state.
 * @param [in] Fs <tt>opus_int32</tt>: Sample rate to decode at (Hz).
 *                                     This must be one of 8000, 12000, 16000,
 *                                     24000, or 48000.
 * @param [in] channels <tt>int</tt>: Number of channels (1 or 2) to decode
 * @param [out] error <tt>int*</tt>: #OPUS_OK Success or @ref opus_errorcodes
 *
 * Internally Opus stores data at 48000 Hz, so that should be the default
 * value for Fs. However, the decoder can efficiently decode to buffers
 * at 8, 12, 16, and 24 kHz so if for some reason the caller cannot use
 * data at the full sample rate, or knows the compressed data doesn't
 * use the full frequency range, it can request decoding at a reduced
 * rate. Likewise, the decoder is capable of filling in either mono or
 * interleaved stereo pcm buffers, at the caller's request.
 */
#[no_mangle]

pub unsafe extern "C" fn opus_decoder_create(
    mut Fs: crate::opus_types_h::opus_int32,
    mut channels: i32,
    mut error: *mut i32,
) -> *mut OpusDecoder {
    let mut ret: i32 = 0;
    let mut st: *mut OpusDecoder = 0 as *mut OpusDecoder;
    if Fs != 48000 as i32
        && Fs != 24000 as i32
        && Fs != 16000 as i32
        && Fs != 12000 as i32
        && Fs != 8000 as i32
        || channels != 1 as i32 && channels != 2 as i32
    {
        if !error.is_null() {
            *error = -(1 as i32)
        }
        return 0 as *mut OpusDecoder;
    }
    st = opus_alloc(opus_decoder_get_size(channels) as crate::stddef_h::size_t) as *mut OpusDecoder;
    if st.is_null() {
        if !error.is_null() {
            *error = -(7 as i32)
        }
        return 0 as *mut OpusDecoder;
    }
    ret = opus_decoder_init(st, Fs, channels);
    if !error.is_null() {
        *error = ret
    }
    if ret != 0 as i32 {
        opus_free(st as *mut libc::c_void);
        st = 0 as *mut OpusDecoder
    }
    return st;
}

unsafe extern "C" fn smooth_fade(
    mut in1: *const crate::arch_h::opus_val16,
    mut in2: *const crate::arch_h::opus_val16,
    mut out: *mut crate::arch_h::opus_val16,
    mut overlap: i32,
    mut channels: i32,
    mut window: *const crate::arch_h::opus_val16,
    mut Fs: crate::opus_types_h::opus_int32,
) {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut inc: i32 = 48000 as i32 / Fs;
    c = 0 as i32;
    while c < channels {
        i = 0 as i32;
        while i < overlap {
            let mut w: crate::arch_h::opus_val16 =
                *window.offset((i * inc) as isize) * *window.offset((i * inc) as isize);
            *out.offset((i * channels + c) as isize) = w * *in2.offset((i * channels + c) as isize)
                + (1.0f32 - w) * *in1.offset((i * channels + c) as isize);
            i += 1
        }
        c += 1
    }
}

unsafe extern "C" fn opus_packet_get_mode(mut data: *const u8) -> i32 {
    let mut mode: i32 = 0;
    if *data.offset(0 as i32 as isize) as i32 & 0x80 as i32 != 0 {
        mode = 1002 as i32
    } else if *data.offset(0 as i32 as isize) as i32 & 0x60 as i32 == 0x60 as i32 {
        mode = 1001 as i32
    } else {
        mode = 1000 as i32
    }
    return mode;
}

unsafe extern "C" fn opus_decode_frame(
    mut st: *mut OpusDecoder,
    mut data: *const u8,
    mut len: crate::opus_types_h::opus_int32,
    mut pcm: *mut crate::arch_h::opus_val16,
    mut frame_size: i32,
    mut decode_fec: i32,
) -> i32 {
    let mut silk_dec: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut celt_dec: *mut crate::src::opus_1_2_1::celt::celt_decoder::OpusCustomDecoder =
        0 as *mut crate::src::opus_1_2_1::celt::celt_decoder::OpusCustomDecoder;
    let mut i: i32 = 0;
    let mut silk_ret: i32 = 0 as i32;
    let mut celt_ret: i32 = 0 as i32;
    let mut dec: crate::src::opus_1_2_1::celt::entcode::ec_dec =
        crate::src::opus_1_2_1::celt::entcode::ec_dec {
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
    let mut silk_frame_size: crate::opus_types_h::opus_int32 = 0;
    let mut pcm_silk_size: i32 = 0;
    let mut pcm_silk: *mut crate::opus_types_h::opus_int16 =
        0 as *mut crate::opus_types_h::opus_int16;
    let mut pcm_transition_silk_size: i32 = 0;
    let mut pcm_transition_silk: *mut crate::arch_h::opus_val16 =
        0 as *mut crate::arch_h::opus_val16;
    let mut pcm_transition_celt_size: i32 = 0;
    let mut pcm_transition_celt: *mut crate::arch_h::opus_val16 =
        0 as *mut crate::arch_h::opus_val16;
    let mut pcm_transition: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut redundant_audio_size: i32 = 0;
    let mut redundant_audio: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut audiosize: i32 = 0;
    let mut mode: i32 = 0;
    let mut transition: i32 = 0 as i32;
    let mut start_band: i32 = 0;
    let mut redundancy: i32 = 0 as i32;
    let mut redundancy_bytes: i32 = 0 as i32;
    let mut celt_to_silk: i32 = 0 as i32;
    let mut c: i32 = 0;
    let mut F2_5: i32 = 0;
    let mut F5: i32 = 0;
    let mut F10: i32 = 0;
    let mut F20: i32 = 0;
    let mut window: *const crate::arch_h::opus_val16 = 0 as *const crate::arch_h::opus_val16;
    let mut redundant_rng: crate::opus_types_h::opus_uint32 =
        0 as i32 as crate::opus_types_h::opus_uint32;
    let mut celt_accum: i32 = 0;
    silk_dec =
        (st as *mut libc::c_char).offset((*st).silk_dec_offset as isize) as *mut libc::c_void;
    celt_dec = (st as *mut libc::c_char).offset((*st).celt_dec_offset as isize)
        as *mut crate::src::opus_1_2_1::celt::celt_decoder::OpusCustomDecoder;
    F20 = (*st).Fs / 50 as i32;
    F10 = F20 >> 1 as i32;
    F5 = F10 >> 1 as i32;
    F2_5 = F5 >> 1 as i32;
    if frame_size < F2_5 {
        return -(2 as i32);
    }
    /* Limit frame_size to avoid excessive stack allocations. */
    frame_size = if frame_size < (*st).Fs / 25 as i32 * 3 as i32 {
        frame_size
    } else {
        ((*st).Fs / 25 as i32) * 3 as i32
    };
    /* Payloads of 1 (2 including ToC) or 0 trigger the PLC/DTX */
    if len <= 1 as i32 {
        data = 0 as *const u8;
        /* In that case, don't conceal more than what the ToC says */
        frame_size = if frame_size < (*st).frame_size {
            frame_size
        } else {
            (*st).frame_size
        }
    }
    if !data.is_null() {
        audiosize = (*st).frame_size;
        mode = (*st).mode;
        crate::src::opus_1_2_1::celt::entdec::ec_dec_init(
            &mut dec as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            data as *mut u8,
            len as crate::opus_types_h::opus_uint32,
        );
    } else {
        audiosize = frame_size;
        mode = (*st).prev_mode;
        if mode == 0 as i32 {
            /* If we haven't got any packet yet, all we can do is return zeros */
            i = 0 as i32;
            while i < audiosize * (*st).channels {
                *pcm.offset(i as isize) = 0 as i32 as crate::arch_h::opus_val16;
                i += 1
            }
            return audiosize;
        }
        /* Avoids trying to run the PLC on sizes other than 2.5 (CELT), 5 (CELT),
        10, or 20 (e.g. 12.5 or 30 ms). */
        if audiosize > F20 {
            loop {
                let mut ret: i32 = opus_decode_frame(
                    st,
                    0 as *const u8,
                    0 as i32,
                    pcm,
                    if audiosize < F20 { audiosize } else { F20 },
                    0 as i32,
                );
                if ret < 0 as i32 {
                    return ret;
                }
                pcm = pcm.offset((ret * (*st).channels) as isize);
                audiosize -= ret;
                if !(audiosize > 0 as i32) {
                    break;
                }
            }
            return frame_size;
        } else {
            if audiosize < F20 {
                if audiosize > F10 {
                    audiosize = F10
                } else if mode != 1000 as i32 && audiosize > F5 && audiosize < F10 {
                    audiosize = F5
                }
            }
        }
    }
    /* In fixed-point, we can tell CELT to do the accumulation on top of the
    SILK PCM buffer. This saves some stack space. */
    celt_accum = 0 as i32;
    pcm_transition_silk_size = 0 as i32;
    pcm_transition_celt_size = 0 as i32;
    if !data.is_null()
        && (*st).prev_mode > 0 as i32
        && (mode == 1002 as i32 && (*st).prev_mode != 1002 as i32 && (*st).prev_redundancy == 0
            || mode != 1002 as i32 && (*st).prev_mode == 1002 as i32)
    {
        transition = 1 as i32;
        /* Decide where to allocate the stack memory for pcm_transition */
        if mode == 1002 as i32 {
            pcm_transition_celt_size = F5 * (*st).channels
        } else {
            pcm_transition_silk_size = F5 * (*st).channels
        }
    }
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul(pcm_transition_celt_size as libc::c_ulong) as usize,
    );
    pcm_transition_celt = fresh0.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    if transition != 0 && mode == 1002 as i32 {
        pcm_transition = pcm_transition_celt;
        opus_decode_frame(
            st,
            0 as *const u8,
            0 as i32,
            pcm_transition,
            if F5 < audiosize { F5 } else { audiosize },
            0 as i32,
        );
    }
    if audiosize > frame_size {
        /*fprintf(stderr, "PCM buffer too small: %d vs %d (mode = %d)\n", audiosize, frame_size, mode);*/
        return -(1 as i32);
    } else {
        frame_size = audiosize
    }
    /* Don't allocate any memory when in CELT-only mode */
    pcm_silk_size = if mode != 1002 as i32 && celt_accum == 0 {
        (if F10 > frame_size { F10 } else { frame_size }) * (*st).channels
    } else {
        0 as i32
    };
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong)
            .wrapping_mul(pcm_silk_size as libc::c_ulong) as usize,
    );
    pcm_silk = fresh1.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    /* SILK processing */
    if mode != 1002 as i32 {
        let mut lost_flag: i32 = 0;
        let mut decoded_samples: i32 = 0;
        let mut pcm_ptr: *mut crate::opus_types_h::opus_int16 =
            0 as *mut crate::opus_types_h::opus_int16;
        pcm_ptr = pcm_silk;
        if (*st).prev_mode == 1002 as i32 {
            crate::src::opus_1_2_1::silk::dec_API::silk_InitDecoder(silk_dec);
        }
        /* The SILK PLC cannot produce frames of less than 10 ms */
        (*st).DecControl.payloadSize_ms = if 10 as i32 > 1000 as i32 * audiosize / (*st).Fs {
            10 as i32
        } else {
            (1000 as i32 * audiosize) / (*st).Fs
        };
        if !data.is_null() {
            (*st).DecControl.nChannelsInternal = (*st).stream_channels;
            if mode == 1000 as i32 {
                if (*st).bandwidth == 1101 as i32 {
                    (*st).DecControl.internalSampleRate = 8000 as i32
                } else if (*st).bandwidth == 1102 as i32 {
                    (*st).DecControl.internalSampleRate = 12000 as i32
                } else if (*st).bandwidth == 1103 as i32 {
                    (*st).DecControl.internalSampleRate = 16000 as i32
                } else {
                    (*st).DecControl.internalSampleRate = 16000 as i32
                }
            } else {
                /* Hybrid mode */
                (*st).DecControl.internalSampleRate = 16000 as i32
            }
        }
        lost_flag = if data.is_null() {
            1 as i32
        } else {
            (2 as i32) * decode_fec
        };
        decoded_samples = 0 as i32;
        loop {
            /* Call SILK decoder */
            let mut first_frame: i32 = (decoded_samples == 0 as i32) as i32;
            silk_ret = crate::src::opus_1_2_1::silk::dec_API::silk_Decode(
                silk_dec,
                &mut (*st).DecControl as *mut _ as *mut crate::control_h::silk_DecControlStruct,
                lost_flag,
                first_frame,
                &mut dec as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                pcm_ptr,
                &mut silk_frame_size,
                (*st).arch,
            );
            if silk_ret != 0 {
                if lost_flag != 0 {
                    /* PLC failure should not be fatal */
                    silk_frame_size = frame_size;
                    i = 0 as i32;
                    while i < frame_size * (*st).channels {
                        *pcm_ptr.offset(i as isize) = 0 as i32 as crate::opus_types_h::opus_int16;
                        i += 1
                    }
                } else {
                    return -(3 as i32);
                }
            }
            pcm_ptr = pcm_ptr.offset((silk_frame_size * (*st).channels) as isize);
            decoded_samples += silk_frame_size;
            if !(decoded_samples < frame_size) {
                break;
            }
        }
    }
    start_band = 0 as i32;
    if decode_fec == 0
        && mode != 1002 as i32
        && !data.is_null()
        && ec_tell(&mut dec) + 17 as i32 + 20 as i32 * ((*st).mode == 1001 as i32) as i32
            <= 8 as i32 * len
    {
        /* Check if we have a redundant 0-8 kHz band */
        if mode == 1001 as i32 {
            redundancy = crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(
                &mut dec as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                12 as i32 as u32,
            )
        } else {
            redundancy = 1 as i32
        }
        if redundancy != 0 {
            celt_to_silk = crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(
                &mut dec as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                1 as i32 as u32,
            );
            /* redundancy_bytes will be at least two, in the non-hybrid
            case due to the ec_tell() check above */
            redundancy_bytes = if mode == 1001 as i32 {
                (crate::src::opus_1_2_1::celt::entdec::ec_dec_uint(
                    &mut dec as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    256 as i32 as crate::opus_types_h::opus_uint32,
                ) as crate::opus_types_h::opus_int32)
                    + 2 as i32
            } else {
                (len) - (ec_tell(&mut dec) + 7 as i32 >> 3 as i32)
            };
            len -= redundancy_bytes;
            /* This is a sanity check. It should never happen for a valid
            packet, so the exact behaviour is not normative. */
            if (len * 8 as i32) < ec_tell(&mut dec) {
                len = 0 as i32;
                redundancy_bytes = 0 as i32;
                redundancy = 0 as i32
            }
            /* Shrink decoder because of raw bits */
            dec.storage = (dec.storage as u32).wrapping_sub(redundancy_bytes as u32)
                as crate::opus_types_h::opus_uint32
                as crate::opus_types_h::opus_uint32
        }
    }
    if mode != 1002 as i32 {
        start_band = 17 as i32
    }
    let mut endband: i32 = 21 as i32;
    match (*st).bandwidth {
        1101 => endband = 13 as i32,
        1102 | 1103 => endband = 17 as i32,
        1104 => endband = 19 as i32,
        1105 => endband = 21 as i32,
        _ => {}
    }
    crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
        celt_dec,
        10012 as i32,
        endband,
    );
    crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
        celt_dec,
        10008 as i32,
        (*st).stream_channels,
    );
    if redundancy != 0 {
        transition = 0 as i32;
        pcm_transition_silk_size = 0 as i32
    }
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul(pcm_transition_silk_size as libc::c_ulong) as usize,
    );
    pcm_transition_silk = fresh2.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    if transition != 0 && mode != 1002 as i32 {
        pcm_transition = pcm_transition_silk;
        opus_decode_frame(
            st,
            0 as *const u8,
            0 as i32,
            pcm_transition,
            if F5 < audiosize { F5 } else { audiosize },
            0 as i32,
        );
    }
    /* Only allocation memory for redundancy if/when needed */
    redundant_audio_size = if redundancy != 0 {
        (F5) * (*st).channels
    } else {
        0 as i32
    };
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul(redundant_audio_size as libc::c_ulong) as usize,
    );
    redundant_audio = fresh3.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    /* 5 ms redundant frame for CELT->SILK*/
    if redundancy != 0 && celt_to_silk != 0 {
        crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
            celt_dec,
            10010 as i32,
            0 as i32,
        );
        crate::src::opus_1_2_1::celt::celt_decoder::celt_decode_with_ec(
            celt_dec,
            data.offset(len as isize),
            redundancy_bytes,
            redundant_audio,
            F5,
            0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_dec
                as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            0 as i32,
        );
        crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
            celt_dec,
            4031 as i32,
            (&mut redundant_rng as *mut crate::opus_types_h::opus_uint32).offset(
                (&mut redundant_rng as *mut crate::opus_types_h::opus_uint32)
                    .offset_from(&mut redundant_rng as *mut crate::opus_types_h::opus_uint32)
                    as libc::c_long as isize,
            ),
        );
    }
    /* MUST be after PLC */
    crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
        celt_dec,
        10010 as i32,
        start_band,
    );
    if mode != 1000 as i32 {
        let mut celt_frame_size: i32 = if F20 < frame_size { F20 } else { frame_size };
        /* Make sure to discard any previous CELT state */
        if mode != (*st).prev_mode && (*st).prev_mode > 0 as i32 && (*st).prev_redundancy == 0 {
            crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
                celt_dec,
                4028 as i32,
            );
        }
        /* Decode CELT */
        celt_ret = crate::src::opus_1_2_1::celt::celt_decoder::celt_decode_with_ec(
            celt_dec,
            if decode_fec != 0 {
                0 as *const u8
            } else {
                data
            },
            len,
            pcm,
            celt_frame_size,
            &mut dec as *mut _ as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            celt_accum,
        )
    } else {
        let mut silence: [u8; 2] = [0xff as i32 as u8, 0xff as i32 as u8];
        if celt_accum == 0 {
            i = 0 as i32;
            while i < frame_size * (*st).channels {
                *pcm.offset(i as isize) = 0 as i32 as crate::arch_h::opus_val16;
                i += 1
            }
        }
        /* For hybrid -> SILK transitions, we let the CELT MDCT
        do a fade-out by decoding a silence frame */
        if (*st).prev_mode == 1001 as i32
            && !(redundancy != 0 && celt_to_silk != 0 && (*st).prev_redundancy != 0)
        {
            crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
                celt_dec,
                10010 as i32,
                0 as i32,
            );
            crate::src::opus_1_2_1::celt::celt_decoder::celt_decode_with_ec(
                celt_dec,
                silence.as_mut_ptr(),
                2 as i32,
                pcm,
                F2_5,
                0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_dec
                    as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                celt_accum,
            );
        }
    }
    if mode != 1002 as i32 && celt_accum == 0 {
        i = 0 as i32;
        while i < frame_size * (*st).channels {
            *pcm.offset(i as isize) = *pcm.offset(i as isize)
                + 1.0f32 / 32768.0f32 * *pcm_silk.offset(i as isize) as i32 as f32;
            i += 1
        }
    }
    let mut celt_mode: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
    crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
        celt_dec,
        10015 as i32,
        (&mut celt_mode as *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode).offset(
            (&mut celt_mode as *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode)
                .offset_from(
                    &mut celt_mode
                        as *mut *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
                ) as libc::c_long as isize,
        ),
    );
    window = (*celt_mode).window;
    /* 5 ms redundant frame for SILK->CELT */
    if redundancy != 0 && celt_to_silk == 0 {
        crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(celt_dec, 4028 as i32);
        crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
            celt_dec,
            10010 as i32,
            0 as i32,
        );
        crate::src::opus_1_2_1::celt::celt_decoder::celt_decode_with_ec(
            celt_dec,
            data.offset(len as isize),
            redundancy_bytes,
            redundant_audio,
            F5,
            0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_dec
                as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            0 as i32,
        );
        crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
            celt_dec,
            4031 as i32,
            (&mut redundant_rng as *mut crate::opus_types_h::opus_uint32).offset(
                (&mut redundant_rng as *mut crate::opus_types_h::opus_uint32)
                    .offset_from(&mut redundant_rng as *mut crate::opus_types_h::opus_uint32)
                    as libc::c_long as isize,
            ),
        );
        smooth_fade(
            pcm.offset(((*st).channels * (frame_size - F2_5)) as isize),
            redundant_audio.offset(((*st).channels * F2_5) as isize),
            pcm.offset(((*st).channels * (frame_size - F2_5)) as isize),
            F2_5,
            (*st).channels,
            window,
            (*st).Fs,
        );
    }
    if redundancy != 0 && celt_to_silk != 0 {
        c = 0 as i32;
        while c < (*st).channels {
            i = 0 as i32;
            while i < F2_5 {
                *pcm.offset(((*st).channels * i + c) as isize) =
                    *redundant_audio.offset(((*st).channels * i + c) as isize);
                i += 1
            }
            c += 1
        }
        smooth_fade(
            redundant_audio.offset(((*st).channels * F2_5) as isize),
            pcm.offset(((*st).channels * F2_5) as isize),
            pcm.offset(((*st).channels * F2_5) as isize),
            F2_5,
            (*st).channels,
            window,
            (*st).Fs,
        );
    }
    if transition != 0 {
        if audiosize >= F5 {
            i = 0 as i32;
            while i < (*st).channels * F2_5 {
                *pcm.offset(i as isize) = *pcm_transition.offset(i as isize);
                i += 1
            }
            smooth_fade(
                pcm_transition.offset(((*st).channels * F2_5) as isize),
                pcm.offset(((*st).channels * F2_5) as isize),
                pcm.offset(((*st).channels * F2_5) as isize),
                F2_5,
                (*st).channels,
                window,
                (*st).Fs,
            );
        } else {
            /* Not enough time to do a clean transition, but we do it anyway
            This will not preserve amplitude perfectly and may introduce
            a bit of temporal aliasing, but it shouldn't be too bad and
            that's pretty much the best we can do. In any case, generating this
            transition it pretty silly in the first place */
            smooth_fade(
                pcm_transition,
                pcm,
                pcm,
                F2_5,
                (*st).channels,
                window,
                (*st).Fs,
            );
        }
    }
    if (*st).decode_gain != 0 {
        let mut gain: crate::arch_h::opus_val32 = 0.;
        gain = celt_exp2(6.48814081e-4f32 * (*st).decode_gain as f32);
        i = 0 as i32;
        while i < frame_size * (*st).channels {
            let mut x: crate::arch_h::opus_val32 = 0.;
            x = *pcm.offset(i as isize) * gain;
            *pcm.offset(i as isize) = x;
            i += 1
        }
    }
    if len <= 1 as i32 {
        (*st).rangeFinal = 0 as i32 as crate::opus_types_h::opus_uint32
    } else {
        (*st).rangeFinal = dec.rng ^ redundant_rng
    }
    (*st).prev_mode = mode;
    (*st).prev_redundancy = (redundancy != 0 && celt_to_silk == 0) as i32;
    if celt_ret >= 0 as i32 {
        _opus_false();
    }
    return if celt_ret < 0 as i32 {
        celt_ret
    } else {
        audiosize
    };
}
#[no_mangle]

pub unsafe extern "C" fn opus_decode_native(
    mut st: *mut OpusDecoder,
    mut data: *const u8,
    mut len: crate::opus_types_h::opus_int32,
    mut pcm: *mut crate::arch_h::opus_val16,
    mut frame_size: i32,
    mut decode_fec: i32,
    mut self_delimited: i32,
    mut packet_offset: *mut crate::opus_types_h::opus_int32,
    mut soft_clip: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut nb_samples: i32 = 0;
    let mut count: i32 = 0;
    let mut offset: i32 = 0;
    let mut toc: u8 = 0;
    let mut packet_frame_size: i32 = 0;
    let mut packet_bandwidth: i32 = 0;
    let mut packet_mode: i32 = 0;
    let mut packet_stream_channels: i32 = 0;
    /* 48 x 2.5 ms = 120 ms */
    let mut size: [crate::opus_types_h::opus_int16; 48] = [0; 48];
    if decode_fec < 0 as i32 || decode_fec > 1 as i32 {
        return -(1 as i32);
    }
    /* For FEC/PLC, frame_size has to be to have a multiple of 2.5 ms */
    if (decode_fec != 0 || len == 0 as i32 || data.is_null())
        && frame_size % ((*st).Fs / 400 as i32) != 0 as i32
    {
        return -(1 as i32);
    }
    if len == 0 as i32 || data.is_null() {
        let mut pcm_count: i32 = 0 as i32;
        loop {
            let mut ret: i32 = 0;
            ret = opus_decode_frame(
                st,
                0 as *const u8,
                0 as i32,
                pcm.offset((pcm_count * (*st).channels) as isize),
                frame_size - pcm_count,
                0 as i32,
            );
            if ret < 0 as i32 {
                return ret;
            }
            pcm_count += ret;
            if !(pcm_count < frame_size) {
                break;
            }
        }
        _opus_false();
        (*st).last_packet_duration = pcm_count;
        return pcm_count;
    } else {
        if len < 0 as i32 {
            return -(1 as i32);
        }
    }
    packet_mode = opus_packet_get_mode(data);
    packet_bandwidth = opus_packet_get_bandwidth(data);
    packet_frame_size =
        crate::src::opus_1_2_1::src::opus::opus_packet_get_samples_per_frame(data, (*st).Fs);
    packet_stream_channels = opus_packet_get_nb_channels(data);
    count = crate::src::opus_1_2_1::src::opus::opus_packet_parse_impl(
        data,
        len,
        self_delimited,
        &mut toc,
        0 as *mut *const u8,
        size.as_mut_ptr(),
        &mut offset,
        packet_offset,
    );
    if count < 0 as i32 {
        return count;
    }
    data = data.offset(offset as isize);
    if decode_fec != 0 {
        let mut duration_copy: i32 = 0;
        let mut ret_0: i32 = 0;
        /* If no FEC can be present, run the PLC (recursive call) */
        if frame_size < packet_frame_size || packet_mode == 1002 as i32 || (*st).mode == 1002 as i32
        {
            return opus_decode_native(
                st,
                0 as *const u8,
                0 as i32,
                pcm,
                frame_size,
                0 as i32,
                0 as i32,
                0 as *mut crate::opus_types_h::opus_int32,
                soft_clip,
            );
        }
        /* Otherwise, run the PLC on everything except the size for which we might have FEC */
        duration_copy = (*st).last_packet_duration;
        if frame_size - packet_frame_size != 0 as i32 {
            ret_0 = opus_decode_native(
                st,
                0 as *const u8,
                0 as i32,
                pcm,
                frame_size - packet_frame_size,
                0 as i32,
                0 as i32,
                0 as *mut crate::opus_types_h::opus_int32,
                soft_clip,
            );
            if ret_0 < 0 as i32 {
                (*st).last_packet_duration = duration_copy;
                return ret_0;
            }
        }
        /* Complete with FEC */
        (*st).mode = packet_mode;
        (*st).bandwidth = packet_bandwidth;
        (*st).frame_size = packet_frame_size;
        (*st).stream_channels = packet_stream_channels;
        ret_0 = opus_decode_frame(
            st,
            data,
            size[0 as i32 as usize] as crate::opus_types_h::opus_int32,
            pcm.offset(((*st).channels * (frame_size - packet_frame_size)) as isize),
            packet_frame_size,
            1 as i32,
        );
        if ret_0 < 0 as i32 {
            return ret_0;
        } else {
            _opus_false();
            (*st).last_packet_duration = frame_size;
            return frame_size;
        }
    }
    if count * packet_frame_size > frame_size {
        return -(2 as i32);
    }
    /* Update the state as the last step to avoid updating it on an invalid packet */
    (*st).mode = packet_mode;
    (*st).bandwidth = packet_bandwidth;
    (*st).frame_size = packet_frame_size;
    (*st).stream_channels = packet_stream_channels;
    nb_samples = 0 as i32;
    i = 0 as i32;
    while i < count {
        let mut ret_1: i32 = 0;
        ret_1 = opus_decode_frame(
            st,
            data,
            size[i as usize] as crate::opus_types_h::opus_int32,
            pcm.offset((nb_samples * (*st).channels) as isize),
            frame_size - nb_samples,
            0 as i32,
        );
        if ret_1 < 0 as i32 {
            return ret_1;
        }
        data = data.offset(size[i as usize] as i32 as isize);
        nb_samples += ret_1;
        i += 1
    }
    (*st).last_packet_duration = nb_samples;
    _opus_false();
    if soft_clip != 0 {
        crate::src::opus_1_2_1::src::opus::opus_pcm_soft_clip(
            pcm,
            nb_samples,
            (*st).channels,
            (*st).softclip_mem.as_mut_ptr(),
        );
    } else {
        (*st).softclip_mem[1 as i32 as usize] = 0 as i32 as crate::arch_h::opus_val16;
        (*st).softclip_mem[0 as i32 as usize] = (*st).softclip_mem[1 as i32 as usize]
    }
    return nb_samples;
}
/* * Decode an Opus packet.
 * @param [in] st <tt>OpusDecoder*</tt>: Decoder state
 * @param [in] data <tt>char*</tt>: Input payload. Use a NULL pointer to indicate packet loss
 * @param [in] len <tt>opus_int32</tt>: Number of bytes in payload*
 * @param [out] pcm <tt>opus_int16*</tt>: Output signal (interleaved if 2 channels). length
 *  is frame_size*channels*sizeof(opus_int16)
 * @param [in] frame_size Number of samples per channel of available space in \a pcm.
 *  If this is less than the maximum packet duration (120ms; 5760 for 48kHz), this function will
 *  not be capable of decoding some packets. In the case of PLC (data==NULL) or FEC (decode_fec=1),
 *  then frame_size needs to be exactly the duration of audio that is missing, otherwise the
 *  decoder will not be in the optimal state to decode the next incoming packet. For the PLC and
 *  FEC cases, frame_size <b>must</b> be a multiple of 2.5 ms.
 * @param [in] decode_fec <tt>int</tt>: Flag (0 or 1) to request that any in-band forward error correction data be
 *  decoded. If no such data is available, the frame is decoded as if it were lost.
 * @returns Number of decoded samples or @ref opus_errorcodes
 */
#[no_mangle]

pub unsafe extern "C" fn opus_decode(
    mut st: *mut OpusDecoder,
    mut data: *const u8,
    mut len: crate::opus_types_h::opus_int32,
    mut pcm: *mut crate::opus_types_h::opus_int16,
    mut frame_size: i32,
    mut decode_fec: i32,
) -> i32 {
    let mut out: *mut f32 = 0 as *mut f32;
    let mut ret: i32 = 0;
    let mut i: i32 = 0;
    let mut nb_samples: i32 = 0;
    if frame_size <= 0 as i32 {
        return -(1 as i32);
    }
    if !data.is_null() && len > 0 as i32 && decode_fec == 0 {
        nb_samples = opus_decoder_get_nb_samples(st, data, len);
        if nb_samples > 0 as i32 {
            frame_size = if frame_size < nb_samples {
                frame_size
            } else {
                nb_samples
            }
        } else {
            return -(4 as i32);
        }
    }
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f32>() as libc::c_ulong)
            .wrapping_mul((frame_size * (*st).channels) as libc::c_ulong) as usize,
    );
    out = fresh4.as_mut_ptr() as *mut f32;
    ret = opus_decode_native(
        st,
        data,
        len,
        out,
        frame_size,
        decode_fec,
        0 as i32,
        0 as *mut crate::opus_types_h::opus_int32,
        1 as i32,
    );
    if ret > 0 as i32 {
        i = 0 as i32;
        while i < ret * (*st).channels {
            *pcm.offset(i as isize) = FLOAT2INT16(*out.offset(i as isize));
            i += 1
        }
    }
    return ret;
}
/* * Decode an Opus packet with floating point output.
 * @param [in] st <tt>OpusDecoder*</tt>: Decoder state
 * @param [in] data <tt>char*</tt>: Input payload. Use a NULL pointer to indicate packet loss
 * @param [in] len <tt>opus_int32</tt>: Number of bytes in payload
 * @param [out] pcm <tt>float*</tt>: Output signal (interleaved if 2 channels). length
 *  is frame_size*channels*sizeof(float)
 * @param [in] frame_size Number of samples per channel of available space in \a pcm.
 *  If this is less than the maximum packet duration (120ms; 5760 for 48kHz), this function will
 *  not be capable of decoding some packets. In the case of PLC (data==NULL) or FEC (decode_fec=1),
 *  then frame_size needs to be exactly the duration of audio that is missing, otherwise the
 *  decoder will not be in the optimal state to decode the next incoming packet. For the PLC and
 *  FEC cases, frame_size <b>must</b> be a multiple of 2.5 ms.
 * @param [in] decode_fec <tt>int</tt>: Flag (0 or 1) to request that any in-band forward error correction data be
 *  decoded. If no such data is available the frame is decoded as if it were lost.
 * @returns Number of decoded samples or @ref opus_errorcodes
 */
#[no_mangle]

pub unsafe extern "C" fn opus_decode_float(
    mut st: *mut OpusDecoder,
    mut data: *const u8,
    mut len: crate::opus_types_h::opus_int32,
    mut pcm: *mut crate::arch_h::opus_val16,
    mut frame_size: i32,
    mut decode_fec: i32,
) -> i32 {
    if frame_size <= 0 as i32 {
        return -(1 as i32);
    }
    return opus_decode_native(
        st,
        data,
        len,
        pcm,
        frame_size,
        decode_fec,
        0 as i32,
        0 as *mut crate::opus_types_h::opus_int32,
        0 as i32,
    );
}
/* * Perform a CTL function on an Opus decoder.
 *
 * Generally the request and subsequent arguments are generated
 * by a convenience macro.
 * @param st <tt>OpusDecoder*</tt>: Decoder state.
 * @param request This and all remaining parameters should be replaced by one
 *                of the convenience macros in @ref opus_genericctls or
 *                @ref opus_decoderctls.
 * @see opus_genericctls
 * @see opus_decoderctls
 */
#[no_mangle]

pub unsafe extern "C" fn opus_decoder_ctl(
    mut st: *mut OpusDecoder,
    mut request: i32,
    mut args: ...
) -> i32 {
    let mut current_block: u64;
    let mut ret: i32 = 0 as i32;
    let mut ap: ::std::ffi::VaListImpl;
    let mut silk_dec: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut celt_dec: *mut crate::src::opus_1_2_1::celt::celt_decoder::OpusCustomDecoder =
        0 as *mut crate::src::opus_1_2_1::celt::celt_decoder::OpusCustomDecoder;
    silk_dec =
        (st as *mut libc::c_char).offset((*st).silk_dec_offset as isize) as *mut libc::c_void;
    celt_dec = (st as *mut libc::c_char).offset((*st).celt_dec_offset as isize)
        as *mut crate::src::opus_1_2_1::celt::celt_decoder::OpusCustomDecoder;
    ap = args.clone();
    match request {
        4009 => {
            let mut value: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value.is_null() {
                current_block = 7252614138838059896;
            } else {
                *value = (*st).bandwidth;
                current_block = 2116367355679836638;
            }
        }
        4031 => {
            let mut value_0: *mut crate::opus_types_h::opus_uint32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_uint32>();
            if value_0.is_null() {
                current_block = 7252614138838059896;
            } else {
                *value_0 = (*st).rangeFinal;
                current_block = 2116367355679836638;
            }
        }
        4028 => {
            crate::stdlib::memset(
                &mut (*st).stream_channels as *mut i32 as *mut libc::c_char as *mut libc::c_void,
                0 as i32,
                (::std::mem::size_of::<OpusDecoder>() as libc::c_ulong)
                    .wrapping_sub(
                        (&mut (*st).stream_channels as *mut i32 as *mut libc::c_char)
                            .offset_from(st as *mut libc::c_char)
                            as libc::c_long as libc::c_ulong,
                    )
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
                celt_dec,
                4028 as i32,
            );
            crate::src::opus_1_2_1::silk::dec_API::silk_InitDecoder(silk_dec);
            (*st).stream_channels = (*st).channels;
            (*st).frame_size = (*st).Fs / 400 as i32;
            current_block = 2116367355679836638;
        }
        4029 => {
            let mut value_1: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_1.is_null() {
                current_block = 7252614138838059896;
            } else {
                *value_1 = (*st).Fs;
                current_block = 2116367355679836638;
            }
        }
        4033 => {
            let mut value_2: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_2.is_null() {
                current_block = 7252614138838059896;
            } else {
                if (*st).prev_mode == 1002 as i32 {
                    crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
                        celt_dec,
                        4033 as i32,
                        value_2.offset(value_2.offset_from(value_2) as libc::c_long as isize),
                    );
                } else {
                    *value_2 = (*st).DecControl.prevPitchLag
                }
                current_block = 2116367355679836638;
            }
        }
        4045 => {
            let mut value_3: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_3.is_null() {
                current_block = 7252614138838059896;
            } else {
                *value_3 = (*st).decode_gain;
                current_block = 2116367355679836638;
            }
        }
        4034 => {
            let mut value_4: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_4 < -(32768 as i32) || value_4 > 32767 as i32 {
                current_block = 7252614138838059896;
            } else {
                (*st).decode_gain = value_4;
                current_block = 2116367355679836638;
            }
        }
        4039 => {
            let mut value_5: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_5.is_null() {
                current_block = 7252614138838059896;
            } else {
                *value_5 = (*st).last_packet_duration;
                current_block = 2116367355679836638;
            }
        }
        4046 => {
            let mut value_6: crate::opus_types_h::opus_int32 =
                ap.as_va_list().arg::<crate::opus_types_h::opus_int32>();
            if value_6 < 0 as i32 || value_6 > 1 as i32 {
                current_block = 7252614138838059896;
            } else {
                crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
                    celt_dec,
                    4046 as i32,
                    value_6,
                );
                current_block = 2116367355679836638;
            }
        }
        4047 => {
            let mut value_7: *mut crate::opus_types_h::opus_int32 =
                ap.as_va_list()
                    .arg::<*mut crate::opus_types_h::opus_int32>();
            if value_7.is_null() {
                current_block = 7252614138838059896;
            } else {
                crate::src::opus_1_2_1::celt::celt_decoder::opus_custom_decoder_ctl(
                    celt_dec,
                    4047 as i32,
                    value_7.offset(value_7.offset_from(value_7) as libc::c_long as isize),
                );
                current_block = 2116367355679836638;
            }
        }
        _ => {
            /*fprintf(stderr, "unknown opus_decoder_ctl() request: %d", request);*/
            ret = -(5 as i32);
            current_block = 2116367355679836638;
        }
    }
    match current_block {
        7252614138838059896 => return -(1 as i32),
        _ => return ret,
    };
}
/* * Frees an <code>OpusDecoder</code> allocated by opus_decoder_create().
 * @param[in] st <tt>OpusDecoder*</tt>: State to be freed.
 */
#[no_mangle]

pub unsafe extern "C" fn opus_decoder_destroy(mut st: *mut OpusDecoder) {
    opus_free(st as *mut libc::c_void);
}
/* * Gets the bandwidth of an Opus packet.
 * @param [in] data <tt>char*</tt>: Opus packet
 * @retval OPUS_BANDWIDTH_NARROWBAND Narrowband (4kHz bandpass)
 * @retval OPUS_BANDWIDTH_MEDIUMBAND Mediumband (6kHz bandpass)
 * @retval OPUS_BANDWIDTH_WIDEBAND Wideband (8kHz bandpass)
 * @retval OPUS_BANDWIDTH_SUPERWIDEBAND Superwideband (12kHz bandpass)
 * @retval OPUS_BANDWIDTH_FULLBAND Fullband (20kHz bandpass)
 * @retval OPUS_INVALID_PACKET The compressed data passed is corrupted or of an unsupported type
 */
#[no_mangle]

pub unsafe extern "C" fn opus_packet_get_bandwidth(mut data: *const u8) -> i32 {
    let mut bandwidth: i32 = 0;
    if *data.offset(0 as i32 as isize) as i32 & 0x80 as i32 != 0 {
        bandwidth = 1102 as i32 + (*data.offset(0 as i32 as isize) as i32 >> 5 as i32 & 0x3 as i32);
        if bandwidth == 1102 as i32 {
            bandwidth = 1101 as i32
        }
    } else if *data.offset(0 as i32 as isize) as i32 & 0x60 as i32 == 0x60 as i32 {
        bandwidth = if *data.offset(0 as i32 as isize) as i32 & 0x10 as i32 != 0 {
            1105 as i32
        } else {
            1104 as i32
        }
    } else {
        bandwidth = 1101 as i32 + (*data.offset(0 as i32 as isize) as i32 >> 5 as i32 & 0x3 as i32)
    }
    return bandwidth;
}
/* * Gets the number of channels from an Opus packet.
 * @param [in] data <tt>char*</tt>: Opus packet
 * @returns Number of channels
 * @retval OPUS_INVALID_PACKET The compressed data passed is corrupted or of an unsupported type
 */
#[no_mangle]

pub unsafe extern "C" fn opus_packet_get_nb_channels(mut data: *const u8) -> i32 {
    return if *data.offset(0 as i32 as isize) as i32 & 0x4 as i32 != 0 {
        2 as i32
    } else {
        1 as i32
    };
}
/* * Gets the number of frames in an Opus packet.
 * @param [in] packet <tt>char*</tt>: Opus packet
 * @param [in] len <tt>opus_int32</tt>: Length of packet
 * @returns Number of frames
 * @retval OPUS_BAD_ARG Insufficient data was passed to the function
 * @retval OPUS_INVALID_PACKET The compressed data passed is corrupted or of an unsupported type
 */
#[no_mangle]

pub unsafe extern "C" fn opus_packet_get_nb_frames(
    mut packet: *const u8,
    mut len: crate::opus_types_h::opus_int32,
) -> i32 {
    let mut count: i32 = 0;
    if len < 1 as i32 {
        return -(1 as i32);
    }
    count = *packet.offset(0 as i32 as isize) as i32 & 0x3 as i32;
    if count == 0 as i32 {
        return 1 as i32;
    } else if count != 3 as i32 {
        return 2 as i32;
    } else if len < 2 as i32 {
        return -(4 as i32);
    } else {
        return *packet.offset(1 as i32 as isize) as i32 & 0x3f as i32;
    };
}
/* * Gets the number of samples of an Opus packet.
 * @param [in] packet <tt>char*</tt>: Opus packet
 * @param [in] len <tt>opus_int32</tt>: Length of packet
 * @param [in] Fs <tt>opus_int32</tt>: Sampling rate in Hz.
 *                                     This must be a multiple of 400, or
 *                                     inaccurate results will be returned.
 * @returns Number of samples
 * @retval OPUS_BAD_ARG Insufficient data was passed to the function
 * @retval OPUS_INVALID_PACKET The compressed data passed is corrupted or of an unsupported type
 */
#[no_mangle]

pub unsafe extern "C" fn opus_packet_get_nb_samples(
    mut packet: *const u8,
    mut len: crate::opus_types_h::opus_int32,
    mut Fs: crate::opus_types_h::opus_int32,
) -> i32 {
    let mut samples: i32 = 0;
    let mut count: i32 = opus_packet_get_nb_frames(packet, len);
    if count < 0 as i32 {
        return count;
    }
    samples =
        count * crate::src::opus_1_2_1::src::opus::opus_packet_get_samples_per_frame(packet, Fs);
    /* Can't have more than 120 ms */
    if samples * 25 as i32 > Fs * 3 as i32 {
        return -(4 as i32);
    } else {
        return samples;
    };
}
/* * Gets the number of samples of an Opus packet.
 * @param [in] dec <tt>OpusDecoder*</tt>: Decoder state
 * @param [in] packet <tt>char*</tt>: Opus packet
 * @param [in] len <tt>opus_int32</tt>: Length of packet
 * @returns Number of samples
 * @retval OPUS_BAD_ARG Insufficient data was passed to the function
 * @retval OPUS_INVALID_PACKET The compressed data passed is corrupted or of an unsupported type
 */
#[no_mangle]

pub unsafe extern "C" fn opus_decoder_get_nb_samples(
    mut dec: *const OpusDecoder,
    mut packet: *const u8,
    mut len: crate::opus_types_h::opus_int32,
) -> i32 {
    return opus_packet_get_nb_samples(packet, len, (*dec).Fs);
}

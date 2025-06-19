use ::libc;

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
    #[inline]

    pub unsafe extern "C" fn opus_alloc(mut size: crate::stddef_h::size_t) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
    /* * Same as celt_alloc(), except that the area is only needed inside a CELT call (might cause problem with wideband though) */
    /* Scratch space doesn't need to be cleared */
    /* * Opus wrapper for free(). To do your own dynamic allocation, all you need to do is replace this function and opus_alloc */
    #[inline]

    pub unsafe extern "C" fn opus_free(mut ptr: *mut libc::c_void) {
        libc::free(ptr);
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

pub use crate::opus_private_h::OpusRepacketizer;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::src::opus_1_2_1::src::opus::encode_size;

pub use crate::src::opus_1_2_1::src::opus::opus_packet_parse_impl;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;

pub use crate::src::opus_1_2_1::src::repacketizer::os_support_h::opus_alloc;
pub use crate::src::opus_1_2_1::src::repacketizer::os_support_h::opus_free;

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
#[no_mangle]

pub unsafe extern "C" fn opus_repacketizer_get_size() -> i32 {
    return ::std::mem::size_of::<OpusRepacketizer>() as usize as i32;
}
#[no_mangle]

pub unsafe extern "C" fn opus_repacketizer_init(
    mut rp: *mut OpusRepacketizer,
) -> *mut OpusRepacketizer {
    (*rp).nb_frames = 0 as i32;
    return rp;
}
#[no_mangle]

pub unsafe extern "C" fn opus_repacketizer_create() -> *mut OpusRepacketizer {
    let mut rp: *mut OpusRepacketizer = 0 as *mut OpusRepacketizer;
    rp = opus_alloc(opus_repacketizer_get_size() as size_t) as *mut OpusRepacketizer;
    if rp.is_null() {
        return 0 as *mut OpusRepacketizer;
    }
    return opus_repacketizer_init(rp);
}
#[no_mangle]

pub unsafe extern "C" fn opus_repacketizer_destroy(mut rp: *mut OpusRepacketizer) {
    opus_free(rp as *mut libc::c_void);
}

unsafe extern "C" fn opus_repacketizer_cat_impl(
    mut rp: *mut OpusRepacketizer,
    mut data: *const u8,
    mut len: opus_int32,
    mut self_delimited: i32,
) -> i32 {
    let mut tmp_toc: u8 = 0;
    let mut curr_nb_frames: i32 = 0;
    let mut ret: i32 = 0;
    /* Set of check ToC */
    if len < 1 as i32 {
        return -(4 as i32);
    }
    if (*rp).nb_frames == 0 as i32 {
        (*rp).toc = *data.offset(0 as i32 as isize);
        (*rp).framesize =
            crate::src::opus_1_2_1::src::opus::opus_packet_get_samples_per_frame(data, 8000 as i32)
    } else if (*rp).toc as i32 & 0xfc as i32 != *data.offset(0 as i32 as isize) as i32 & 0xfc as i32
    {
        /*fprintf(stderr, "toc mismatch: 0x%x vs 0x%x\n", rp->toc, data[0]);*/
        return -(4 as i32);
    }
    curr_nb_frames =
        crate::src::opus_1_2_1::src::opus_decoder::opus_packet_get_nb_frames(data, len);
    if curr_nb_frames < 1 as i32 {
        return -(4 as i32);
    }
    /* Check the 120 ms maximum packet size */
    if (curr_nb_frames + (*rp).nb_frames) * (*rp).framesize > 960 as i32 {
        return -(4 as i32);
    }
    ret = opus_packet_parse_impl(
        data,
        len,
        self_delimited,
        &mut tmp_toc,
        &mut *(*rp).frames.as_mut_ptr().offset((*rp).nb_frames as isize),
        &mut *(*rp).len.as_mut_ptr().offset((*rp).nb_frames as isize),
        0 as *mut i32,
        0 as *mut opus_int32,
    );
    if ret < 1 as i32 {
        return ret;
    }
    (*rp).nb_frames += curr_nb_frames;
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn opus_repacketizer_cat(
    mut rp: *mut OpusRepacketizer,
    mut data: *const u8,
    mut len: opus_int32,
) -> i32 {
    return opus_repacketizer_cat_impl(rp, data, len, 0 as i32);
}
#[no_mangle]

pub unsafe extern "C" fn opus_repacketizer_get_nb_frames(mut rp: *mut OpusRepacketizer) -> i32 {
    return (*rp).nb_frames;
}
#[no_mangle]

pub unsafe extern "C" fn opus_repacketizer_out_range_impl(
    mut rp: *mut OpusRepacketizer,
    mut begin: i32,
    mut end: i32,
    mut data: *mut u8,
    mut maxlen: opus_int32,
    mut self_delimited: i32,
    mut pad: i32,
) -> opus_int32 {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut tot_size: opus_int32 = 0;
    let mut len: *mut opus_int16 = 0 as *mut opus_int16;
    let mut frames: *mut *const u8 = 0 as *mut *const u8;
    let mut ptr: *mut u8 = 0 as *mut u8;
    if begin < 0 as i32 || begin >= end || end > (*rp).nb_frames {
        /*fprintf(stderr, "%d %d %d\n", begin, end, rp->nb_frames);*/
        return -(1 as i32);
    }
    count = end - begin;
    len = (*rp).len.as_mut_ptr().offset(begin as isize);
    frames = (*rp).frames.as_mut_ptr().offset(begin as isize);
    if self_delimited != 0 {
        tot_size = 1 as i32 + (*len.offset((count - 1 as i32) as isize) as i32 >= 252 as i32) as i32
    } else {
        tot_size = 0 as i32
    }
    ptr = data;
    if count == 1 as i32 {
        /* Code 0 */
        tot_size += *len.offset(0 as i32 as isize) as i32 + 1 as i32;
        if tot_size > maxlen {
            return -(2 as i32);
        }
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 = ((*rp).toc as i32 & 0xfc as i32) as u8
    } else if count == 2 as i32 {
        if *len.offset(1 as i32 as isize) as i32 == *len.offset(0 as i32 as isize) as i32 {
            /* Code 1 */
            tot_size += 2 as i32 * *len.offset(0 as i32 as isize) as i32 + 1 as i32;
            if tot_size > maxlen {
                return -(2 as i32);
            }
            let fresh1 = ptr;
            ptr = ptr.offset(1);
            *fresh1 = ((*rp).toc as i32 & 0xfc as i32 | 0x1 as i32) as u8
        } else {
            /* Code 2 */
            tot_size += *len.offset(0 as i32 as isize) as i32
                + *len.offset(1 as i32 as isize) as i32
                + 2 as i32
                + (*len.offset(0 as i32 as isize) as i32 >= 252 as i32) as i32;
            if tot_size > maxlen {
                return -(2 as i32);
            }
            let fresh2 = ptr;
            ptr = ptr.offset(1);
            *fresh2 = ((*rp).toc as i32 & 0xfc as i32 | 0x2 as i32) as u8;
            ptr = ptr.offset(encode_size(*len.offset(0 as i32 as isize) as i32, ptr) as isize)
        }
    }
    if count > 2 as i32 || pad != 0 && tot_size < maxlen {
        /* Code 3 */
        let mut vbr: i32 = 0;
        let mut pad_amount: i32 = 0 as i32;
        /* Restart the process for the padding case */
        ptr = data;
        if self_delimited != 0 {
            tot_size =
                1 as i32 + (*len.offset((count - 1 as i32) as isize) as i32 >= 252 as i32) as i32
        } else {
            tot_size = 0 as i32
        }
        vbr = 0 as i32;
        i = 1 as i32;
        while i < count {
            if *len.offset(i as isize) as i32 != *len.offset(0 as i32 as isize) as i32 {
                vbr = 1 as i32;
                break;
            } else {
                i += 1
            }
        }
        if vbr != 0 {
            tot_size += 2 as i32;
            i = 0 as i32;
            while i < count - 1 as i32 {
                tot_size += 1 as i32
                    + (*len.offset(i as isize) as i32 >= 252 as i32) as i32
                    + *len.offset(i as isize) as i32;
                i += 1
            }
            tot_size += *len.offset((count - 1 as i32) as isize) as i32;
            if tot_size > maxlen {
                return -(2 as i32);
            }
            let fresh3 = ptr;
            ptr = ptr.offset(1);
            *fresh3 = ((*rp).toc as i32 & 0xfc as i32 | 0x3 as i32) as u8;
            let fresh4 = ptr;
            ptr = ptr.offset(1);
            *fresh4 = (count | 0x80 as i32) as u8
        } else {
            tot_size += count * *len.offset(0 as i32 as isize) as i32 + 2 as i32;
            if tot_size > maxlen {
                return -(2 as i32);
            }
            let fresh5 = ptr;
            ptr = ptr.offset(1);
            *fresh5 = ((*rp).toc as i32 & 0xfc as i32 | 0x3 as i32) as u8;
            let fresh6 = ptr;
            ptr = ptr.offset(1);
            *fresh6 = count as u8
        }
        pad_amount = if pad != 0 {
            (maxlen) - tot_size
        } else {
            0 as i32
        };
        if pad_amount != 0 as i32 {
            let mut nb_255s: i32 = 0;
            let ref mut fresh7 = *data.offset(1 as i32 as isize);
            *fresh7 = (*fresh7 as i32 | 0x40 as i32) as u8;
            nb_255s = (pad_amount - 1 as i32) / 255 as i32;
            i = 0 as i32;
            while i < nb_255s {
                let fresh8 = ptr;
                ptr = ptr.offset(1);
                *fresh8 = 255 as i32 as u8;
                i += 1
            }
            let fresh9 = ptr;
            ptr = ptr.offset(1);
            *fresh9 = (pad_amount - 255 as i32 * nb_255s - 1 as i32) as u8;
            tot_size += pad_amount
        }
        if vbr != 0 {
            i = 0 as i32;
            while i < count - 1 as i32 {
                ptr = ptr.offset(encode_size(*len.offset(i as isize) as i32, ptr) as isize);
                i += 1
            }
        }
    }
    if self_delimited != 0 {
        let mut sdlen: i32 = encode_size(*len.offset((count - 1 as i32) as isize) as i32, ptr);
        ptr = ptr.offset(sdlen as isize)
    }
    /* Copy the actual data */
    i = 0 as i32;
    while i < count {
        /* Using OPUS_MOVE() instead of OPUS_COPY() in case we're doing in-place
        padding from opus_packet_pad or opus_packet_unpad(). */
        crate::stdlib::memmove(
            ptr as *mut libc::c_void,
            *frames.offset(i as isize) as *const libc::c_void,
            (*len.offset(i as isize) as usize)
                .wrapping_mul(::std::mem::size_of::<u8>() as usize)
                .wrapping_add(
                    (0 as i32 as isize * ptr.offset_from(*frames.offset(i as isize)) as isize)
                        as usize,
                ),
        );
        ptr = ptr.offset(*len.offset(i as isize) as i32 as isize);
        i += 1
    }
    if pad != 0 {
        /* Fill padding with zeros. */
        while ptr < data.offset(maxlen as isize) {
            let fresh10 = ptr;
            ptr = ptr.offset(1);
            *fresh10 = 0 as i32 as u8
        }
    }
    return tot_size;
}
#[no_mangle]

pub unsafe extern "C" fn opus_repacketizer_out_range(
    mut rp: *mut OpusRepacketizer,
    mut begin: i32,
    mut end: i32,
    mut data: *mut u8,
    mut maxlen: opus_int32,
) -> opus_int32 {
    return opus_repacketizer_out_range_impl(rp, begin, end, data, maxlen, 0 as i32, 0 as i32);
}
#[no_mangle]

pub unsafe extern "C" fn opus_repacketizer_out(
    mut rp: *mut OpusRepacketizer,
    mut data: *mut u8,
    mut maxlen: opus_int32,
) -> opus_int32 {
    return opus_repacketizer_out_range_impl(
        rp,
        0 as i32,
        (*rp).nb_frames,
        data,
        maxlen,
        0 as i32,
        0 as i32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn opus_packet_pad(
    mut data: *mut u8,
    mut len: opus_int32,
    mut new_len: opus_int32,
) -> i32 {
    let mut rp: OpusRepacketizer = OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [0 as *const u8; 48],
        len: [0; 48],
        framesize: 0,
    };
    let mut ret: opus_int32 = 0;
    if len < 1 as i32 {
        return -(1 as i32);
    }
    if len == new_len {
        return 0 as i32;
    } else {
        if len > new_len {
            return -(1 as i32);
        }
    }
    opus_repacketizer_init(&mut rp);
    /* Moving payload to the end of the packet so we can do in-place padding */
    crate::stdlib::memmove(
        data.offset(new_len as isize).offset(-(len as isize)) as *mut libc::c_void,
        data as *const libc::c_void,
        (len as usize)
            .wrapping_mul(::std::mem::size_of::<u8>() as usize)
            .wrapping_add(
                (0 as i32 as isize
                    * data
                        .offset(new_len as isize)
                        .offset(-(len as isize))
                        .offset_from(data) as isize) as usize,
            ),
    );
    ret = opus_repacketizer_cat(
        &mut rp,
        data.offset(new_len as isize).offset(-(len as isize)),
        len,
    );
    if ret != 0 as i32 {
        return ret;
    }
    ret = opus_repacketizer_out_range_impl(
        &mut rp,
        0 as i32,
        rp.nb_frames,
        data,
        new_len,
        0 as i32,
        1 as i32,
    );
    if ret > 0 as i32 {
        return 0 as i32;
    } else {
        return ret;
    };
}
/* * Encodes an Opus frame.
 * @param [in] st <tt>OpusEncoder*</tt>: Encoder state
 * @param [in] pcm <tt>opus_int16*</tt>: Input signal (interleaved if 2 channels). length is frame_size*channels*sizeof(opus_int16)
 * @param [in] frame_size <tt>int</tt>: Number of samples per channel in the
 *                                      input signal.
 *                                      This must be an Opus frame size for
 *                                      the encoder's sampling rate.
 *                                      For example, at 48 kHz the permitted
 *                                      values are 120, 240, 480, 960, 1920,
 *                                      and 2880.
 *                                      Passing in a duration of less than
 *                                      10 ms (480 samples at 48 kHz) will
 *                                      prevent the encoder from using the LPC
 *                                      or hybrid modes.
 * @param [out] data <tt>unsigned char*</tt>: Output payload.
 *                                            This must contain storage for at
 *                                            least \a max_data_bytes.
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
/* * Encodes an Opus frame from floating point input.
 * @param [in] st <tt>OpusEncoder*</tt>: Encoder state
 * @param [in] pcm <tt>float*</tt>: Input in float format (interleaved if 2 channels), with a normal range of +/-1.0.
 *          Samples with a range beyond +/-1.0 are supported but will
 *          be clipped by decoders using the integer API and should
 *          only be used if it is known that the far end supports
 *          extended dynamic range.
 *          length is frame_size*channels*sizeof(float)
 * @param [in] frame_size <tt>int</tt>: Number of samples per channel in the
 *                                      input signal.
 *                                      This must be an Opus frame size for
 *                                      the encoder's sampling rate.
 *                                      For example, at 48 kHz the permitted
 *                                      values are 120, 240, 480, 960, 1920,
 *                                      and 2880.
 *                                      Passing in a duration of less than
 *                                      10 ms (480 samples at 48 kHz) will
 *                                      prevent the encoder from using the LPC
 *                                      or hybrid modes.
 * @param [out] data <tt>unsigned char*</tt>: Output payload.
 *                                            This must contain storage for at
 *                                            least \a max_data_bytes.
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
/* * Frees an <code>OpusEncoder</code> allocated by opus_encoder_create().
 * @param[in] st <tt>OpusEncoder*</tt>: State to be freed.
 */
/* * Perform a CTL function on an Opus encoder.
 *
 * Generally the request and subsequent arguments are generated
 * by a convenience macro.
 * @param st <tt>OpusEncoder*</tt>: Encoder state.
 * @param request This and all remaining parameters should be replaced by one
 *                of the convenience macros in @ref opus_genericctls or
 *                @ref opus_encoderctls.
 * @see opus_genericctls
 * @see opus_encoderctls
 */
/* *@}*/
/* * @defgroup opus_decoder Opus Decoder
 * @{
 *
 * @brief This page describes the process and functions used to decode Opus.
 *
 * The decoding process also starts with creating a decoder
 * state. This can be done with:
 * @code
 * int          error;
 * OpusDecoder *dec;
 * dec = opus_decoder_create(Fs, channels, &error);
 * @endcode
 * where
 * @li Fs is the sampling rate and must be 8000, 12000, 16000, 24000, or 48000
 * @li channels is the number of channels (1 or 2)
 * @li error will hold the error code in case of failure (or #OPUS_OK on success)
 * @li the return value is a newly created decoder state to be used for decoding
 *
 * While opus_decoder_create() allocates memory for the state, it's also possible
 * to initialize pre-allocated memory:
 * @code
 * int          size;
 * int          error;
 * OpusDecoder *dec;
 * size = opus_decoder_get_size(channels);
 * dec = malloc(size);
 * error = opus_decoder_init(dec, Fs, channels);
 * @endcode
 * where opus_decoder_get_size() returns the required size for the decoder state. Note that
 * future versions of this code may change the size, so no assuptions should be made about it.
 *
 * The decoder state is always continuous in memory and only a shallow copy is sufficient
 * to copy it (e.g. memcpy())
 *
 * To decode a frame, opus_decode() or opus_decode_float() must be called with a packet of compressed audio data:
 * @code
 * frame_size = opus_decode(dec, packet, len, decoded, max_size, 0);
 * @endcode
 * where
 *
 * @li packet is the byte array containing the compressed data
 * @li len is the exact number of bytes contained in the packet
 * @li decoded is the decoded audio data in opus_int16 (or float for opus_decode_float())
 * @li max_size is the max duration of the frame in samples (per channel) that can fit into the decoded_frame array
 *
 * opus_decode() and opus_decode_float() return the number of samples (per channel) decoded from the packet.
 * If that value is negative, then an error has occurred. This can occur if the packet is corrupted or if the audio
 * buffer is too small to hold the decoded audio.
 *
 * Opus is a stateful codec with overlapping blocks and as a result Opus
 * packets are not coded independently of each other. Packets must be
 * passed into the decoder serially and in the correct order for a correct
 * decode. Lost packets can be replaced with loss concealment by calling
 * the decoder with a null pointer and zero length for the missing packet.
 *
 * A single codec state may only be accessed from a single thread at
 * a time and any required locking must be performed by the caller. Separate
 * streams must be decoded with separate decoder states and can be decoded
 * in parallel unless the library was compiled with NONTHREADSAFE_PSEUDOSTACK
 * defined.
 *
 */
/* * Opus decoder state.
 * This contains the complete state of an Opus decoder.
 * It is position independent and can be freely copied.
 * @see opus_decoder_create,opus_decoder_init
 */
/* * Gets the size of an <code>OpusDecoder</code> structure.
 * @param [in] channels <tt>int</tt>: Number of channels.
 *                                    This must be 1 or 2.
 * @returns The size in bytes.
 */
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
/* * Frees an <code>OpusDecoder</code> allocated by opus_decoder_create().
 * @param[in] st <tt>OpusDecoder*</tt>: State to be freed.
 */
/* * Parse an opus packet into one or more frames.
 * Opus_decode will perform this operation internally so most applications do
 * not need to use this function.
 * This function does not copy the frames, the returned pointers are pointers into
 * the input packet.
 * @param [in] data <tt>char*</tt>: Opus packet to be parsed
 * @param [in] len <tt>opus_int32</tt>: size of data
 * @param [out] out_toc <tt>char*</tt>: TOC pointer
 * @param [out] frames <tt>char*[48]</tt> encapsulated frames
 * @param [out] size <tt>opus_int16[48]</tt> sizes of the encapsulated frames
 * @param [out] payload_offset <tt>int*</tt>: returns the position of the payload within the packet (in bytes)
 * @returns number of frames
 */
/* * Gets the bandwidth of an Opus packet.
 * @param [in] data <tt>char*</tt>: Opus packet
 * @retval OPUS_BANDWIDTH_NARROWBAND Narrowband (4kHz bandpass)
 * @retval OPUS_BANDWIDTH_MEDIUMBAND Mediumband (6kHz bandpass)
 * @retval OPUS_BANDWIDTH_WIDEBAND Wideband (8kHz bandpass)
 * @retval OPUS_BANDWIDTH_SUPERWIDEBAND Superwideband (12kHz bandpass)
 * @retval OPUS_BANDWIDTH_FULLBAND Fullband (20kHz bandpass)
 * @retval OPUS_INVALID_PACKET The compressed data passed is corrupted or of an unsupported type
 */
/* * Gets the number of samples per frame from an Opus packet.
 * @param [in] data <tt>char*</tt>: Opus packet.
 *                                  This must contain at least one byte of
 *                                  data.
 * @param [in] Fs <tt>opus_int32</tt>: Sampling rate in Hz.
 *                                     This must be a multiple of 400, or
 *                                     inaccurate results will be returned.
 * @returns Number of samples per frame.
 */
/* * Gets the number of channels from an Opus packet.
 * @param [in] data <tt>char*</tt>: Opus packet
 * @returns Number of channels
 * @retval OPUS_INVALID_PACKET The compressed data passed is corrupted or of an unsupported type
 */
/* * Gets the number of frames in an Opus packet.
 * @param [in] packet <tt>char*</tt>: Opus packet
 * @param [in] len <tt>opus_int32</tt>: Length of packet
 * @returns Number of frames
 * @retval OPUS_BAD_ARG Insufficient data was passed to the function
 * @retval OPUS_INVALID_PACKET The compressed data passed is corrupted or of an unsupported type
 */
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
/* * Gets the number of samples of an Opus packet.
 * @param [in] dec <tt>OpusDecoder*</tt>: Decoder state
 * @param [in] packet <tt>char*</tt>: Opus packet
 * @param [in] len <tt>opus_int32</tt>: Length of packet
 * @returns Number of samples
 * @retval OPUS_BAD_ARG Insufficient data was passed to the function
 * @retval OPUS_INVALID_PACKET The compressed data passed is corrupted or of an unsupported type
 */
/* * Applies soft-clipping to bring a float signal within the [-1,1] range. If
 * the signal is already in that range, nothing is done. If there are values
 * outside of [-1,1], then the signal is clipped as smoothly as possible to
 * both fit in the range and avoid creating excessive distortion in the
 * process.
 * @param [in,out] pcm <tt>float*</tt>: Input PCM and modified PCM
 * @param [in] frame_size <tt>int</tt> Number of samples per channel to process
 * @param [in] channels <tt>int</tt>: Number of channels
 * @param [in,out] softclip_mem <tt>float*</tt>: State memory for the soft clipping process (one float per channel, initialized to zero)
 */
/* *@}*/
/* * @defgroup opus_repacketizer Repacketizer
 * @{
 *
 * The repacketizer can be used to merge multiple Opus packets into a single
 * packet or alternatively to split Opus packets that have previously been
 * merged. Splitting valid Opus packets is always guaranteed to succeed,
 * whereas merging valid packets only succeeds if all frames have the same
 * mode, bandwidth, and frame size, and when the total duration of the merged
 * packet is no more than 120 ms. The 120 ms limit comes from the
 * specification and limits decoder memory requirements at a point where
 * framing overhead becomes negligible.
 *
 * The repacketizer currently only operates on elementary Opus
 * streams. It will not manipualte multistream packets successfully, except in
 * the degenerate case where they consist of data from a single stream.
 *
 * The repacketizing process starts with creating a repacketizer state, either
 * by calling opus_repacketizer_create() or by allocating the memory yourself,
 * e.g.,
 * @code
 * OpusRepacketizer *rp;
 * rp = (OpusRepacketizer*)malloc(opus_repacketizer_get_size());
 * if (rp != NULL)
 *     opus_repacketizer_init(rp);
 * @endcode
 *
 * Then the application should submit packets with opus_repacketizer_cat(),
 * extract new packets with opus_repacketizer_out() or
 * opus_repacketizer_out_range(), and then reset the state for the next set of
 * input packets via opus_repacketizer_init().
 *
 * For example, to split a sequence of packets into individual frames:
 * @code
 * unsigned char *data;
 * int len;
 * while (get_next_packet(&data, &len))
 * {
 *   unsigned char out[1276];
 *   opus_int32 out_len;
 *   int nb_frames;
 *   int err;
 *   int i;
 *   err = opus_repacketizer_cat(rp, data, len);
 *   if (err != OPUS_OK)
 *   {
 *     release_packet(data);
 *     return err;
 *   }
 *   nb_frames = opus_repacketizer_get_nb_frames(rp);
 *   for (i = 0; i < nb_frames; i++)
 *   {
 *     out_len = opus_repacketizer_out_range(rp, i, i+1, out, sizeof(out));
 *     if (out_len < 0)
 *     {
 *        release_packet(data);
 *        return (int)out_len;
 *     }
 *     output_next_packet(out, out_len);
 *   }
 *   opus_repacketizer_init(rp);
 *   release_packet(data);
 * }
 * @endcode
 *
 * Alternatively, to combine a sequence of frames into packets that each
 * contain up to <code>TARGET_DURATION_MS</code> milliseconds of data:
 * @code
 * // The maximum number of packets with duration TARGET_DURATION_MS occurs
 * // when the frame size is 2.5 ms, for a total of (TARGET_DURATION_MS*2/5)
 * // packets.
 * unsigned char *data[(TARGET_DURATION_MS*2/5)+1];
 * opus_int32 len[(TARGET_DURATION_MS*2/5)+1];
 * int nb_packets;
 * unsigned char out[1277*(TARGET_DURATION_MS*2/2)];
 * opus_int32 out_len;
 * int prev_toc;
 * nb_packets = 0;
 * while (get_next_packet(data+nb_packets, len+nb_packets))
 * {
 *   int nb_frames;
 *   int err;
 *   nb_frames = opus_packet_get_nb_frames(data[nb_packets], len[nb_packets]);
 *   if (nb_frames < 1)
 *   {
 *     release_packets(data, nb_packets+1);
 *     return nb_frames;
 *   }
 *   nb_frames += opus_repacketizer_get_nb_frames(rp);
 *   // If adding the next packet would exceed our target, or it has an
 *   // incompatible TOC sequence, output the packets we already have before
 *   // submitting it.
 *   // N.B., The nb_packets > 0 check ensures we've submitted at least one
 *   // packet since the last call to opus_repacketizer_init(). Otherwise a
 *   // single packet longer than TARGET_DURATION_MS would cause us to try to
 *   // output an (invalid) empty packet. It also ensures that prev_toc has
 *   // been set to a valid value. Additionally, len[nb_packets] > 0 is
 *   // guaranteed by the call to opus_packet_get_nb_frames() above, so the
 *   // reference to data[nb_packets][0] should be valid.
 *   if (nb_packets > 0 && (
 *       ((prev_toc & 0xFC) != (data[nb_packets][0] & 0xFC)) ||
 *       opus_packet_get_samples_per_frame(data[nb_packets], 48000)*nb_frames >
 *       TARGET_DURATION_MS*48))
 *   {
 *     out_len = opus_repacketizer_out(rp, out, sizeof(out));
 *     if (out_len < 0)
 *     {
 *        release_packets(data, nb_packets+1);
 *        return (int)out_len;
 *     }
 *     output_next_packet(out, out_len);
 *     opus_repacketizer_init(rp);
 *     release_packets(data, nb_packets);
 *     data[0] = data[nb_packets];
 *     len[0] = len[nb_packets];
 *     nb_packets = 0;
 *   }
 *   err = opus_repacketizer_cat(rp, data[nb_packets], len[nb_packets]);
 *   if (err != OPUS_OK)
 *   {
 *     release_packets(data, nb_packets+1);
 *     return err;
 *   }
 *   prev_toc = data[nb_packets][0];
 *   nb_packets++;
 * }
 * // Output the final, partial packet.
 * if (nb_packets > 0)
 * {
 *   out_len = opus_repacketizer_out(rp, out, sizeof(out));
 *   release_packets(data, nb_packets);
 *   if (out_len < 0)
 *     return (int)out_len;
 *   output_next_packet(out, out_len);
 * }
 * @endcode
 *
 * An alternate way of merging packets is to simply call opus_repacketizer_cat()
 * unconditionally until it fails. At that point, the merged packet can be
 * obtained with opus_repacketizer_out() and the input packet for which
 * opus_repacketizer_cat() needs to be re-added to a newly reinitialized
 * repacketizer state.
 */
/* * Gets the size of an <code>OpusRepacketizer</code> structure.
 * @returns The size in bytes.
 */
/* * (Re)initializes a previously allocated repacketizer state.
 * The state must be at least the size returned by opus_repacketizer_get_size().
 * This can be used for applications which use their own allocator instead of
 * malloc().
 * It must also be called to reset the queue of packets waiting to be
 * repacketized, which is necessary if the maximum packet duration of 120 ms
 * is reached or if you wish to submit packets with a different Opus
 * configuration (coding mode, audio bandwidth, frame size, or channel count).
 * Failure to do so will prevent a new packet from being added with
 * opus_repacketizer_cat().
 * @see opus_repacketizer_create
 * @see opus_repacketizer_get_size
 * @see opus_repacketizer_cat
 * @param rp <tt>OpusRepacketizer*</tt>: The repacketizer state to
 *                                       (re)initialize.
 * @returns A pointer to the same repacketizer state that was passed in.
 */
/* * Allocates memory and initializes the new repacketizer with
 * opus_repacketizer_init().
 */
/* * Frees an <code>OpusRepacketizer</code> allocated by
 * opus_repacketizer_create().
 * @param[in] rp <tt>OpusRepacketizer*</tt>: State to be freed.
 */
/* * Add a packet to the current repacketizer state.
 * This packet must match the configuration of any packets already submitted
 * for repacketization since the last call to opus_repacketizer_init().
 * This means that it must have the same coding mode, audio bandwidth, frame
 * size, and channel count.
 * This can be checked in advance by examining the top 6 bits of the first
 * byte of the packet, and ensuring they match the top 6 bits of the first
 * byte of any previously submitted packet.
 * The total duration of audio in the repacketizer state also must not exceed
 * 120 ms, the maximum duration of a single packet, after adding this packet.
 *
 * The contents of the current repacketizer state can be extracted into new
 * packets using opus_repacketizer_out() or opus_repacketizer_out_range().
 *
 * In order to add a packet with a different configuration or to add more
 * audio beyond 120 ms, you must clear the repacketizer state by calling
 * opus_repacketizer_init().
 * If a packet is too large to add to the current repacketizer state, no part
 * of it is added, even if it contains multiple frames, some of which might
 * fit.
 * If you wish to be able to add parts of such packets, you should first use
 * another repacketizer to split the packet into pieces and add them
 * individually.
 * @see opus_repacketizer_out_range
 * @see opus_repacketizer_out
 * @see opus_repacketizer_init
 * @param rp <tt>OpusRepacketizer*</tt>: The repacketizer state to which to
 *                                       add the packet.
 * @param[in] data <tt>const unsigned char*</tt>: The packet data.
 *                                                The application must ensure
 *                                                this pointer remains valid
 *                                                until the next call to
 *                                                opus_repacketizer_init() or
 *                                                opus_repacketizer_destroy().
 * @param len <tt>opus_int32</tt>: The number of bytes in the packet data.
 * @returns An error code indicating whether or not the operation succeeded.
 * @retval #OPUS_OK The packet's contents have been added to the repacketizer
 *                  state.
 * @retval #OPUS_INVALID_PACKET The packet did not have a valid TOC sequence,
 *                              the packet's TOC sequence was not compatible
 *                              with previously submitted packets (because
 *                              the coding mode, audio bandwidth, frame size,
 *                              or channel count did not match), or adding
 *                              this packet would increase the total amount of
 *                              audio stored in the repacketizer state to more
 *                              than 120 ms.
 */
/* * Construct a new packet from data previously submitted to the repacketizer
 * state via opus_repacketizer_cat().
 * @param rp <tt>OpusRepacketizer*</tt>: The repacketizer state from which to
 *                                       construct the new packet.
 * @param begin <tt>int</tt>: The index of the first frame in the current
 *                            repacketizer state to include in the output.
 * @param end <tt>int</tt>: One past the index of the last frame in the
 *                          current repacketizer state to include in the
 *                          output.
 * @param[out] data <tt>const unsigned char*</tt>: The buffer in which to
 *                                                 store the output packet.
 * @param maxlen <tt>opus_int32</tt>: The maximum number of bytes to store in
 *                                    the output buffer. In order to guarantee
 *                                    success, this should be at least
 *                                    <code>1276</code> for a single frame,
 *                                    or for multiple frames,
 *                                    <code>1277*(end-begin)</code>.
 *                                    However, <code>1*(end-begin)</code> plus
 *                                    the size of all packet data submitted to
 *                                    the repacketizer since the last call to
 *                                    opus_repacketizer_init() or
 *                                    opus_repacketizer_create() is also
 *                                    sufficient, and possibly much smaller.
 * @returns The total size of the output packet on success, or an error code
 *          on failure.
 * @retval #OPUS_BAD_ARG <code>[begin,end)</code> was an invalid range of
 *                       frames (begin < 0, begin >= end, or end >
 *                       opus_repacketizer_get_nb_frames()).
 * @retval #OPUS_BUFFER_TOO_SMALL \a maxlen was insufficient to contain the
 *                                complete output packet.
 */
/* * Return the total number of frames contained in packet data submitted to
 * the repacketizer state so far via opus_repacketizer_cat() since the last
 * call to opus_repacketizer_init() or opus_repacketizer_create().
 * This defines the valid range of packets that can be extracted with
 * opus_repacketizer_out_range() or opus_repacketizer_out().
 * @param rp <tt>OpusRepacketizer*</tt>: The repacketizer state containing the
 *                                       frames.
 * @returns The total number of frames contained in the packet data submitted
 *          to the repacketizer state.
 */
/* * Construct a new packet from data previously submitted to the repacketizer
 * state via opus_repacketizer_cat().
 * This is a convenience routine that returns all the data submitted so far
 * in a single packet.
 * It is equivalent to calling
 * @code
 * opus_repacketizer_out_range(rp, 0, opus_repacketizer_get_nb_frames(rp),
 *                             data, maxlen)
 * @endcode
 * @param rp <tt>OpusRepacketizer*</tt>: The repacketizer state from which to
 *                                       construct the new packet.
 * @param[out] data <tt>const unsigned char*</tt>: The buffer in which to
 *                                                 store the output packet.
 * @param maxlen <tt>opus_int32</tt>: The maximum number of bytes to store in
 *                                    the output buffer. In order to guarantee
 *                                    success, this should be at least
 *                                    <code>1277*opus_repacketizer_get_nb_frames(rp)</code>.
 *                                    However,
 *                                    <code>1*opus_repacketizer_get_nb_frames(rp)</code>
 *                                    plus the size of all packet data
 *                                    submitted to the repacketizer since the
 *                                    last call to opus_repacketizer_init() or
 *                                    opus_repacketizer_create() is also
 *                                    sufficient, and possibly much smaller.
 * @returns The total size of the output packet on success, or an error code
 *          on failure.
 * @retval #OPUS_BUFFER_TOO_SMALL \a maxlen was insufficient to contain the
 *                                complete output packet.
 */
/* * Pads a given Opus packet to a larger size (possibly changing the TOC sequence).
 * @param[in,out] data <tt>const unsigned char*</tt>: The buffer containing the
 *                                                   packet to pad.
 * @param len <tt>opus_int32</tt>: The size of the packet.
 *                                 This must be at least 1.
 * @param new_len <tt>opus_int32</tt>: The desired size of the packet after padding.
 *                                 This must be at least as large as len.
 * @returns an error code
 * @retval #OPUS_OK \a on success.
 * @retval #OPUS_BAD_ARG \a len was less than 1 or new_len was less than len.
 * @retval #OPUS_INVALID_PACKET \a data did not contain a valid Opus packet.
 */
/* * Remove all padding from a given Opus packet and rewrite the TOC sequence to
 * minimize space usage.
 * @param[in,out] data <tt>const unsigned char*</tt>: The buffer containing the
 *                                                   packet to strip.
 * @param len <tt>opus_int32</tt>: The size of the packet.
 *                                 This must be at least 1.
 * @returns The new size of the output packet on success, or an error code
 *          on failure.
 * @retval #OPUS_BAD_ARG \a len was less than 1.
 * @retval #OPUS_INVALID_PACKET \a data did not contain a valid Opus packet.
 */
#[no_mangle]

pub unsafe extern "C" fn opus_packet_unpad(mut data: *mut u8, mut len: opus_int32) -> opus_int32 {
    let mut rp: OpusRepacketizer = OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [0 as *const u8; 48],
        len: [0; 48],
        framesize: 0,
    };
    let mut ret: opus_int32 = 0;
    if len < 1 as i32 {
        return -(1 as i32);
    }
    opus_repacketizer_init(&mut rp);
    ret = opus_repacketizer_cat(&mut rp, data, len);
    if ret < 0 as i32 {
        return ret;
    }
    ret = opus_repacketizer_out_range_impl(
        &mut rp,
        0 as i32,
        rp.nb_frames,
        data,
        len,
        0 as i32,
        0 as i32,
    );
    return ret;
}
/* * Pads a given Opus multi-stream packet to a larger size (possibly changing the TOC sequence).
 * @param[in,out] data <tt>const unsigned char*</tt>: The buffer containing the
 *                                                   packet to pad.
 * @param len <tt>opus_int32</tt>: The size of the packet.
 *                                 This must be at least 1.
 * @param new_len <tt>opus_int32</tt>: The desired size of the packet after padding.
 *                                 This must be at least 1.
 * @param nb_streams <tt>opus_int32</tt>: The number of streams (not channels) in the packet.
 *                                 This must be at least as large as len.
 * @returns an error code
 * @retval #OPUS_OK \a on success.
 * @retval #OPUS_BAD_ARG \a len was less than 1.
 * @retval #OPUS_INVALID_PACKET \a data did not contain a valid Opus packet.
 */
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_packet_pad(
    mut data: *mut u8,
    mut len: opus_int32,
    mut new_len: opus_int32,
    mut nb_streams: i32,
) -> i32 {
    let mut s: i32 = 0;
    let mut count: i32 = 0;
    let mut toc: u8 = 0;
    let mut size: [opus_int16; 48] = [0; 48];
    let mut packet_offset: opus_int32 = 0;
    let mut amount: opus_int32 = 0;
    if len < 1 as i32 {
        return -(1 as i32);
    }
    if len == new_len {
        return 0 as i32;
    } else {
        if len > new_len {
            return -(1 as i32);
        }
    }
    amount = new_len - len;
    /* Seek to last stream */
    s = 0 as i32;
    while s < nb_streams - 1 as i32 {
        if len <= 0 as i32 {
            return -(4 as i32);
        }
        count = opus_packet_parse_impl(
            data,
            len,
            1 as i32,
            &mut toc,
            0 as *mut *const u8,
            size.as_mut_ptr(),
            0 as *mut i32,
            &mut packet_offset,
        );
        if count < 0 as i32 {
            return count;
        }
        data = data.offset(packet_offset as isize);
        len -= packet_offset;
        s += 1
    }
    return opus_packet_pad(data, len, len + amount);
}
/* * Remove all padding from a given Opus multi-stream packet and rewrite the TOC sequence to
 * minimize space usage.
 * @param[in,out] data <tt>const unsigned char*</tt>: The buffer containing the
 *                                                   packet to strip.
 * @param len <tt>opus_int32</tt>: The size of the packet.
 *                                 This must be at least 1.
 * @param nb_streams <tt>opus_int32</tt>: The number of streams (not channels) in the packet.
 *                                 This must be at least 1.
 * @returns The new size of the output packet on success, or an error code
 *          on failure.
 * @retval #OPUS_BAD_ARG \a len was less than 1 or new_len was less than len.
 * @retval #OPUS_INVALID_PACKET \a data did not contain a valid Opus packet.
 */
#[no_mangle]

pub unsafe extern "C" fn opus_multistream_packet_unpad(
    mut data: *mut u8,
    mut len: opus_int32,
    mut nb_streams: i32,
) -> opus_int32 {
    let mut s: i32 = 0;
    let mut toc: u8 = 0;
    let mut size: [opus_int16; 48] = [0; 48];
    let mut packet_offset: opus_int32 = 0;
    let mut rp: OpusRepacketizer = OpusRepacketizer {
        toc: 0,
        nb_frames: 0,
        frames: [0 as *const u8; 48],
        len: [0; 48],
        framesize: 0,
    };
    let mut dst: *mut u8 = 0 as *mut u8;
    let mut dst_len: opus_int32 = 0;
    if len < 1 as i32 {
        return -(1 as i32);
    }
    dst = data;
    dst_len = 0 as i32;
    /* Unpad all frames */
    s = 0 as i32;
    while s < nb_streams {
        let mut ret: opus_int32 = 0;
        let mut self_delimited: i32 = (s != nb_streams - 1 as i32) as i32;
        if len <= 0 as i32 {
            return -(4 as i32);
        }
        opus_repacketizer_init(&mut rp);
        ret = opus_packet_parse_impl(
            data,
            len,
            self_delimited,
            &mut toc,
            0 as *mut *const u8,
            size.as_mut_ptr(),
            0 as *mut i32,
            &mut packet_offset,
        );
        if ret < 0 as i32 {
            return ret;
        }
        ret = opus_repacketizer_cat_impl(&mut rp, data, packet_offset, self_delimited);
        if ret < 0 as i32 {
            return ret;
        }
        ret = opus_repacketizer_out_range_impl(
            &mut rp,
            0 as i32,
            rp.nb_frames,
            dst,
            len,
            self_delimited,
            0 as i32,
        );
        if ret < 0 as i32 {
            return ret;
        } else {
            dst_len += ret
        }
        dst = dst.offset(ret as isize);
        data = data.offset(packet_offset as isize);
        len -= packet_offset;
        s += 1
    }
    return dst_len;
}

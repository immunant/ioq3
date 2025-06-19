use ::libc;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;

pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
/* Copyright (c) 2011 Xiph.Org Foundation, Skype Limited
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
#[no_mangle]

pub unsafe extern "C" fn opus_pcm_soft_clip(
    mut _x: *mut libc::c_float,
    mut N: libc::c_int,
    mut C: libc::c_int,
    mut declip_mem: *mut libc::c_float,
) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut x: *mut libc::c_float = 0 as *mut libc::c_float;
    if C < 1 as libc::c_int || N < 1 as libc::c_int || _x.is_null() || declip_mem.is_null() {
        return;
    }
    /* First thing: saturate everything to +/- 2 which is the highest level our
    non-linearity can handle. At the point where the signal reaches +/-2,
    the derivative will be zero anyway, so this doesn't introduce any
    discontinuity in the derivative. */
    i = 0 as libc::c_int;
    while i < N * C {
        *_x.offset(i as isize) = if -2.0f32
            > (if 2.0f32 < *_x.offset(i as isize) {
                2.0f32
            } else {
                *_x.offset(i as isize)
            }) {
            -2.0f32
        } else if 2.0f32 < *_x.offset(i as isize) {
            2.0f32
        } else {
            *_x.offset(i as isize)
        };
        i += 1
    }
    c = 0 as libc::c_int;
    while c < C {
        let mut a: libc::c_float = 0.;
        let mut x0: libc::c_float = 0.;
        let mut curr: libc::c_int = 0;
        x = _x.offset(c as isize);
        a = *declip_mem.offset(c as isize);
        /* Continue applying the non-linearity from the previous frame to avoid
        any discontinuity. */
        i = 0 as libc::c_int;
        while i < N {
            if *x.offset((i * C) as isize) * a >= 0 as libc::c_int as libc::c_float {
                break;
            }
            *x.offset((i * C) as isize) = *x.offset((i * C) as isize)
                + a * *x.offset((i * C) as isize) * *x.offset((i * C) as isize);
            i += 1
        }
        curr = 0 as libc::c_int;
        x0 = *x.offset(0 as libc::c_int as isize);
        loop {
            let mut start: libc::c_int = 0;
            let mut end: libc::c_int = 0;
            let mut maxval: libc::c_float = 0.;
            let mut special: libc::c_int = 0 as libc::c_int;
            let mut peak_pos: libc::c_int = 0;
            i = curr;
            while i < N {
                if *x.offset((i * C) as isize) > 1 as libc::c_int as libc::c_float
                    || *x.offset((i * C) as isize) < -(1 as libc::c_int) as libc::c_float
                {
                    break;
                }
                i += 1
            }
            if i == N {
                a = 0 as libc::c_int as libc::c_float;
                break;
            } else {
                peak_pos = i;
                end = i;
                start = end;
                maxval = crate::stdlib::fabs(*x.offset((i * C) as isize) as libc::c_double)
                    as libc::c_float;
                /* Look for first zero crossing before clipping */
                while start > 0 as libc::c_int
                    && *x.offset((i * C) as isize)
                        * *x.offset(((start - 1 as libc::c_int) * C) as isize)
                        >= 0 as libc::c_int as libc::c_float
                {
                    start -= 1
                }
                /* Look for first zero crossing after clipping */
                while end < N
                    && *x.offset((i * C) as isize) * *x.offset((end * C) as isize)
                        >= 0 as libc::c_int as libc::c_float
                {
                    /* Look for other peaks until the next zero-crossing. */
                    if crate::stdlib::fabs(*x.offset((end * C) as isize) as libc::c_double)
                        as libc::c_float
                        > maxval
                    {
                        maxval = crate::stdlib::fabs(*x.offset((end * C) as isize) as libc::c_double)
                            as libc::c_float;
                        peak_pos = end
                    }
                    end += 1
                }
                /* Detect the special case where we clip before the first zero crossing */
                special = (start == 0 as libc::c_int
                    && *x.offset((i * C) as isize) * *x.offset(0 as libc::c_int as isize)
                        >= 0 as libc::c_int as libc::c_float)
                    as libc::c_int;
                /* Compute a such that maxval + a*maxval^2 = 1 */
                a = (maxval - 1 as libc::c_int as libc::c_float) / (maxval * maxval);
                /* Slightly boost "a" by 2^-22. This is just enough to ensure -ffast-math
                does not cause output values larger than +/-1, but small enough not
                to matter even for 24-bit output.  */
                a += a * 2.4e-7f32;
                if *x.offset((i * C) as isize) > 0 as libc::c_int as libc::c_float {
                    a = -a
                }
                /* Apply soft clipping */
                i = start;
                while i < end {
                    *x.offset((i * C) as isize) = *x.offset((i * C) as isize)
                        + a * *x.offset((i * C) as isize) * *x.offset((i * C) as isize);
                    i += 1
                }
                if special != 0 && peak_pos >= 2 as libc::c_int {
                    /* Add a linear ramp from the first sample to the signal peak.
                    This avoids a discontinuity at the beginning of the frame. */
                    let mut delta: libc::c_float = 0.;
                    let mut offset: libc::c_float = x0 - *x.offset(0 as libc::c_int as isize);
                    delta = offset / peak_pos as libc::c_float;
                    i = curr;
                    while i < peak_pos {
                        offset -= delta;
                        *x.offset((i * C) as isize) += offset;
                        *x.offset((i * C) as isize) = if -1.0f32
                            > (if 1.0f32 < *x.offset((i * C) as isize) {
                                1.0f32
                            } else {
                                *x.offset((i * C) as isize)
                            }) {
                            -1.0f32
                        } else if 1.0f32 < *x.offset((i * C) as isize) {
                            1.0f32
                        } else {
                            *x.offset((i * C) as isize)
                        };
                        i += 1
                    }
                }
                curr = end;
                if curr == N {
                    break;
                }
            }
        }
        *declip_mem.offset(c as isize) = a;
        c += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn encode_size(
    mut size: libc::c_int,
    mut data: *mut libc::c_uchar,
) -> libc::c_int {
    if size < 252 as libc::c_int {
        *data.offset(0 as libc::c_int as isize) = size as libc::c_uchar;
        return 1 as libc::c_int;
    } else {
        *data.offset(0 as libc::c_int as isize) =
            (252 as libc::c_int + (size & 0x3 as libc::c_int)) as libc::c_uchar;
        *data.offset(1 as libc::c_int as isize) = (size
            - *data.offset(0 as libc::c_int as isize) as libc::c_int
            >> 2 as libc::c_int) as libc::c_uchar;
        return 2 as libc::c_int;
    };
}

unsafe extern "C" fn parse_size(
    mut data: *const libc::c_uchar,
    mut len: crate::opus_types_h::opus_int32,
    mut size: *mut crate::opus_types_h::opus_int16,
) -> libc::c_int {
    if len < 1 as libc::c_int {
        *size = -(1 as libc::c_int) as crate::opus_types_h::opus_int16;
        return -(1 as libc::c_int);
    } else if (*data.offset(0 as libc::c_int as isize) as libc::c_int) < 252 as libc::c_int {
        *size = *data.offset(0 as libc::c_int as isize) as crate::opus_types_h::opus_int16;
        return 1 as libc::c_int;
    } else if len < 2 as libc::c_int {
        *size = -(1 as libc::c_int) as crate::opus_types_h::opus_int16;
        return -(1 as libc::c_int);
    } else {
        *size = (4 as libc::c_int * *data.offset(1 as libc::c_int as isize) as libc::c_int
            + *data.offset(0 as libc::c_int as isize) as libc::c_int)
            as crate::opus_types_h::opus_int16;
        return 2 as libc::c_int;
    };
}
#[no_mangle]

pub unsafe extern "C" fn opus_packet_get_samples_per_frame(
    mut data: *const libc::c_uchar,
    mut Fs: crate::opus_types_h::opus_int32,
) -> libc::c_int {
    let mut audiosize: libc::c_int = 0;
    if *data.offset(0 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int != 0 {
        audiosize = *data.offset(0 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int
            & 0x3 as libc::c_int;
        audiosize = (Fs << audiosize) / 400 as libc::c_int
    } else if *data.offset(0 as libc::c_int as isize) as libc::c_int & 0x60 as libc::c_int
        == 0x60 as libc::c_int
    {
        audiosize =
            if *data.offset(0 as libc::c_int as isize) as libc::c_int & 0x8 as libc::c_int != 0 {
                (Fs) / 50 as libc::c_int
            } else {
                (Fs) / 100 as libc::c_int
            }
    } else {
        audiosize = *data.offset(0 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int
            & 0x3 as libc::c_int;
        if audiosize == 3 as libc::c_int {
            audiosize = Fs * 60 as libc::c_int / 1000 as libc::c_int
        } else {
            audiosize = (Fs << audiosize) / 100 as libc::c_int
        }
    }
    return audiosize;
}
/* Copyright (c) 2012 Xiph.Org Foundation
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
/* offsetof */
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
/* Optimizing compilers should optimize div and multiply into and
for all sensible alignment values. */
#[no_mangle]

pub unsafe extern "C" fn opus_packet_parse_impl(
    mut data: *const libc::c_uchar,
    mut len: crate::opus_types_h::opus_int32,
    mut self_delimited: libc::c_int,
    mut out_toc: *mut libc::c_uchar,
    mut frames: *mut *const libc::c_uchar,
    mut size: *mut crate::opus_types_h::opus_int16,
    mut payload_offset: *mut libc::c_int,
    mut packet_offset: *mut crate::opus_types_h::opus_int32,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut cbr: libc::c_int = 0;
    let mut ch: libc::c_uchar = 0;
    let mut toc: libc::c_uchar = 0;
    let mut framesize: libc::c_int = 0;
    let mut last_size: crate::opus_types_h::opus_int32 = 0;
    let mut pad: crate::opus_types_h::opus_int32 = 0 as libc::c_int;
    let mut data0: *const libc::c_uchar = data;
    if size.is_null() || len < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if len == 0 as libc::c_int {
        return -(4 as libc::c_int);
    }
    framesize = opus_packet_get_samples_per_frame(data, 48000 as libc::c_int);
    cbr = 0 as libc::c_int;
    let fresh0 = data;
    data = data.offset(1);
    toc = *fresh0;
    len -= 1;
    last_size = len;
    match toc as libc::c_int & 0x3 as libc::c_int {
        0 => {
            /* One frame */
            count = 1 as libc::c_int
        }
        1 => {
            /* Two CBR frames */
            count = 2 as libc::c_int;
            cbr = 1 as libc::c_int;
            if self_delimited == 0 {
                if len & 0x1 as libc::c_int != 0 {
                    return -(4 as libc::c_int);
                }
                last_size = len / 2 as libc::c_int;
                /* If last_size doesn't fit in size[0], we'll catch it later */
                *size.offset(0 as libc::c_int as isize) =
                    last_size as crate::opus_types_h::opus_int16
            }
        }
        2 => {
            /* Two VBR frames */
            count = 2 as libc::c_int;
            bytes = parse_size(data, len, size);
            len -= bytes;
            if (*size.offset(0 as libc::c_int as isize) as libc::c_int) < 0 as libc::c_int
                || *size.offset(0 as libc::c_int as isize) as libc::c_int > len
            {
                return -(4 as libc::c_int);
            }
            data = data.offset(bytes as isize);
            last_size = len - *size.offset(0 as libc::c_int as isize) as libc::c_int
        }
        _ => {
            /* Multiple CBR/VBR frames (from 0 to 120 ms) */
            /*case 3:*/
            if len < 1 as libc::c_int {
                return -(4 as libc::c_int);
            }
            /* Number of frames encoded in bits 0 to 5 */
            let fresh1 = data;
            data = data.offset(1);
            ch = *fresh1;
            count = ch as libc::c_int & 0x3f as libc::c_int;
            if count <= 0 as libc::c_int || framesize * count > 5760 as libc::c_int {
                return -(4 as libc::c_int);
            }
            len -= 1;
            /* Padding flag is bit 6 */
            if ch as libc::c_int & 0x40 as libc::c_int != 0 {
                let mut p: libc::c_int = 0;
                loop {
                    let mut tmp: libc::c_int = 0;
                    if len <= 0 as libc::c_int {
                        return -(4 as libc::c_int);
                    }
                    let fresh2 = data;
                    data = data.offset(1);
                    p = *fresh2 as libc::c_int;
                    len -= 1;
                    tmp = if p == 255 as libc::c_int {
                        254 as libc::c_int
                    } else {
                        p
                    };
                    len -= tmp;
                    pad += tmp;
                    if !(p == 255 as libc::c_int) {
                        break;
                    }
                }
            }
            if len < 0 as libc::c_int {
                return -(4 as libc::c_int);
            }
            /* VBR flag is bit 7 */
            cbr = (ch as libc::c_int & 0x80 as libc::c_int == 0) as libc::c_int;
            if cbr == 0 {
                /* VBR case */
                last_size = len;
                i = 0 as libc::c_int;
                while i < count - 1 as libc::c_int {
                    bytes = parse_size(data, len, size.offset(i as isize));
                    len -= bytes;
                    if (*size.offset(i as isize) as libc::c_int) < 0 as libc::c_int
                        || *size.offset(i as isize) as libc::c_int > len
                    {
                        return -(4 as libc::c_int);
                    }
                    data = data.offset(bytes as isize);
                    last_size -= bytes + *size.offset(i as isize) as libc::c_int;
                    i += 1
                }
                if last_size < 0 as libc::c_int {
                    return -(4 as libc::c_int);
                }
            } else if self_delimited == 0 {
                /* CBR case */
                last_size = len / count;
                if last_size * count != len {
                    return -(4 as libc::c_int);
                }
                i = 0 as libc::c_int;
                while i < count - 1 as libc::c_int {
                    *size.offset(i as isize) = last_size as crate::opus_types_h::opus_int16;
                    i += 1
                }
            }
        }
    }
    /* Self-delimited framing has an extra size for the last frame. */
    if self_delimited != 0 {
        bytes = parse_size(
            data,
            len,
            size.offset(count as isize)
                .offset(-(1 as libc::c_int as isize)),
        );
        len -= bytes;
        if (*size.offset((count - 1 as libc::c_int) as isize) as libc::c_int) < 0 as libc::c_int
            || *size.offset((count - 1 as libc::c_int) as isize) as libc::c_int > len
        {
            return -(4 as libc::c_int);
        }
        data = data.offset(bytes as isize);
        /* For CBR packets, apply the size to all the frames. */
        if cbr != 0 {
            if *size.offset((count - 1 as libc::c_int) as isize) as libc::c_int * count > len {
                return -(4 as libc::c_int);
            }
            i = 0 as libc::c_int;
            while i < count - 1 as libc::c_int {
                *size.offset(i as isize) = *size.offset((count - 1 as libc::c_int) as isize);
                i += 1
            }
        } else if bytes + *size.offset((count - 1 as libc::c_int) as isize) as libc::c_int
            > last_size
        {
            return -(4 as libc::c_int);
        }
    } else {
        /* Because it's not encoded explicitly, it's possible the size of the
        last packet (or all the packets, for the CBR case) is larger than
        1275. Reject them here.*/
        if last_size > 1275 as libc::c_int {
            return -(4 as libc::c_int);
        }
        *size.offset((count - 1 as libc::c_int) as isize) =
            last_size as crate::opus_types_h::opus_int16
    }
    if !payload_offset.is_null() {
        *payload_offset = data.offset_from(data0) as libc::c_long as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < count {
        if !frames.is_null() {
            let ref mut fresh3 = *frames.offset(i as isize);
            *fresh3 = data
        }
        data = data.offset(*size.offset(i as isize) as libc::c_int as isize);
        i += 1
    }
    if !packet_offset.is_null() {
        *packet_offset = pad
            + data.offset_from(data0) as libc::c_long as crate::opus_types_h::opus_int32
    }
    if !out_toc.is_null() {
        *out_toc = toc
    }
    return count;
}
#[no_mangle]

pub unsafe extern "C" fn opus_packet_parse(
    mut data: *const libc::c_uchar,
    mut len: crate::opus_types_h::opus_int32,
    mut out_toc: *mut libc::c_uchar,
    mut frames: *mut *const libc::c_uchar,
    mut size: *mut crate::opus_types_h::opus_int16,
    mut payload_offset: *mut libc::c_int,
) -> libc::c_int {
    return opus_packet_parse_impl(
        data,
        len,
        0 as libc::c_int,
        out_toc,
        frames,
        size,
        payload_offset,
        0 as *mut crate::opus_types_h::opus_int32,
    );
}

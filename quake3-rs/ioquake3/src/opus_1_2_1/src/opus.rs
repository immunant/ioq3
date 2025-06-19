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
    mut _x: *mut f32,
    mut N: i32,
    mut C: i32,
    mut declip_mem: *mut f32,
) {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
    let mut x: *mut f32 = 0 as *mut f32;
    if C < 1 as i32 || N < 1 as i32 || _x.is_null() || declip_mem.is_null() {
        return;
    }
    /* First thing: saturate everything to +/- 2 which is the highest level our
    non-linearity can handle. At the point where the signal reaches +/-2,
    the derivative will be zero anyway, so this doesn't introduce any
    discontinuity in the derivative. */
    i = 0 as i32;
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
    c = 0 as i32;
    while c < C {
        let mut a: f32 = 0.;
        let mut x0: f32 = 0.;
        let mut curr: i32 = 0;
        x = _x.offset(c as isize);
        a = *declip_mem.offset(c as isize);
        /* Continue applying the non-linearity from the previous frame to avoid
        any discontinuity. */
        i = 0 as i32;
        while i < N {
            if *x.offset((i * C) as isize) * a >= 0 as i32 as f32 {
                break;
            }
            *x.offset((i * C) as isize) = *x.offset((i * C) as isize)
                + a * *x.offset((i * C) as isize) * *x.offset((i * C) as isize);
            i += 1
        }
        curr = 0 as i32;
        x0 = *x.offset(0 as i32 as isize);
        loop {
            let mut start: i32 = 0;
            let mut end: i32 = 0;
            let mut maxval: f32 = 0.;
            let mut special: i32 = 0 as i32;
            let mut peak_pos: i32 = 0;
            i = curr;
            while i < N {
                if *x.offset((i * C) as isize) > 1 as i32 as f32
                    || *x.offset((i * C) as isize) < -(1 as i32) as f32
                {
                    break;
                }
                i += 1
            }
            if i == N {
                a = 0 as i32 as f32;
                break;
            } else {
                peak_pos = i;
                end = i;
                start = end;
                maxval = crate::stdlib::fabs(*x.offset((i * C) as isize) as f64) as f32;
                /* Look for first zero crossing before clipping */
                while start > 0 as i32
                    && *x.offset((i * C) as isize) * *x.offset(((start - 1 as i32) * C) as isize)
                        >= 0 as i32 as f32
                {
                    start -= 1
                }
                /* Look for first zero crossing after clipping */
                while end < N
                    && *x.offset((i * C) as isize) * *x.offset((end * C) as isize)
                        >= 0 as i32 as f32
                {
                    /* Look for other peaks until the next zero-crossing. */
                    if crate::stdlib::fabs(*x.offset((end * C) as isize) as f64) as f32 > maxval {
                        maxval = crate::stdlib::fabs(*x.offset((end * C) as isize) as f64) as f32;
                        peak_pos = end
                    }
                    end += 1
                }
                /* Detect the special case where we clip before the first zero crossing */
                special = (start == 0 as i32
                    && *x.offset((i * C) as isize) * *x.offset(0 as i32 as isize)
                        >= 0 as i32 as f32) as i32;
                /* Compute a such that maxval + a*maxval^2 = 1 */
                a = (maxval - 1 as i32 as f32) / (maxval * maxval);
                /* Slightly boost "a" by 2^-22. This is just enough to ensure -ffast-math
                does not cause output values larger than +/-1, but small enough not
                to matter even for 24-bit output.  */
                a += a * 2.4e-7f32;
                if *x.offset((i * C) as isize) > 0 as i32 as f32 {
                    a = -a
                }
                /* Apply soft clipping */
                i = start;
                while i < end {
                    *x.offset((i * C) as isize) = *x.offset((i * C) as isize)
                        + a * *x.offset((i * C) as isize) * *x.offset((i * C) as isize);
                    i += 1
                }
                if special != 0 && peak_pos >= 2 as i32 {
                    /* Add a linear ramp from the first sample to the signal peak.
                    This avoids a discontinuity at the beginning of the frame. */
                    let mut delta: f32 = 0.;
                    let mut offset: f32 = x0 - *x.offset(0 as i32 as isize);
                    delta = offset / peak_pos as f32;
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

pub unsafe extern "C" fn encode_size(mut size: i32, mut data: *mut u8) -> i32 {
    if size < 252 as i32 {
        *data.offset(0 as i32 as isize) = size as u8;
        return 1 as i32;
    } else {
        *data.offset(0 as i32 as isize) = (252 as i32 + (size & 0x3 as i32)) as u8;
        *data.offset(1 as i32 as isize) =
            (size - *data.offset(0 as i32 as isize) as i32 >> 2 as i32) as u8;
        return 2 as i32;
    };
}

unsafe extern "C" fn parse_size(
    mut data: *const u8,
    mut len: opus_int32,
    mut size: *mut opus_int16,
) -> i32 {
    if len < 1 as i32 {
        *size = -(1 as i32) as opus_int16;
        return -(1 as i32);
    } else if (*data.offset(0 as i32 as isize) as i32) < 252 as i32 {
        *size = *data.offset(0 as i32 as isize) as opus_int16;
        return 1 as i32;
    } else if len < 2 as i32 {
        *size = -(1 as i32) as opus_int16;
        return -(1 as i32);
    } else {
        *size = (4 as i32 * *data.offset(1 as i32 as isize) as i32
            + *data.offset(0 as i32 as isize) as i32) as opus_int16;
        return 2 as i32;
    };
}
#[no_mangle]

pub unsafe extern "C" fn opus_packet_get_samples_per_frame(
    mut data: *const u8,
    mut Fs: opus_int32,
) -> i32 {
    let mut audiosize: i32 = 0;
    if *data.offset(0 as i32 as isize) as i32 & 0x80 as i32 != 0 {
        audiosize = *data.offset(0 as i32 as isize) as i32 >> 3 as i32 & 0x3 as i32;
        audiosize = (Fs << audiosize) / 400 as i32
    } else if *data.offset(0 as i32 as isize) as i32 & 0x60 as i32 == 0x60 as i32 {
        audiosize = if *data.offset(0 as i32 as isize) as i32 & 0x8 as i32 != 0 {
            (Fs) / 50 as i32
        } else {
            (Fs) / 100 as i32
        }
    } else {
        audiosize = *data.offset(0 as i32 as isize) as i32 >> 3 as i32 & 0x3 as i32;
        if audiosize == 3 as i32 {
            audiosize = Fs * 60 as i32 / 1000 as i32
        } else {
            audiosize = (Fs << audiosize) / 100 as i32
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
    mut data: *const u8,
    mut len: opus_int32,
    mut self_delimited: i32,
    mut out_toc: *mut u8,
    mut frames: *mut *const u8,
    mut size: *mut opus_int16,
    mut payload_offset: *mut i32,
    mut packet_offset: *mut opus_int32,
) -> i32 {
    let mut i: i32 = 0;
    let mut bytes: i32 = 0;
    let mut count: i32 = 0;
    let mut cbr: i32 = 0;
    let mut ch: u8 = 0;
    let mut toc: u8 = 0;
    let mut framesize: i32 = 0;
    let mut last_size: opus_int32 = 0;
    let mut pad: opus_int32 = 0 as i32;
    let mut data0: *const u8 = data;
    if size.is_null() || len < 0 as i32 {
        return -(1 as i32);
    }
    if len == 0 as i32 {
        return -(4 as i32);
    }
    framesize = opus_packet_get_samples_per_frame(data, 48000 as i32);
    cbr = 0 as i32;
    let fresh0 = data;
    data = data.offset(1);
    toc = *fresh0;
    len -= 1;
    last_size = len;
    match toc as i32 & 0x3 as i32 {
        0 => {
            /* One frame */
            count = 1 as i32
        }
        1 => {
            /* Two CBR frames */
            count = 2 as i32;
            cbr = 1 as i32;
            if self_delimited == 0 {
                if len & 0x1 as i32 != 0 {
                    return -(4 as i32);
                }
                last_size = len / 2 as i32;
                /* If last_size doesn't fit in size[0], we'll catch it later */
                *size.offset(0 as i32 as isize) = last_size as opus_int16
            }
        }
        2 => {
            /* Two VBR frames */
            count = 2 as i32;
            bytes = parse_size(data, len, size);
            len -= bytes;
            if (*size.offset(0 as i32 as isize) as i32) < 0 as i32
                || *size.offset(0 as i32 as isize) as i32 > len
            {
                return -(4 as i32);
            }
            data = data.offset(bytes as isize);
            last_size = len - *size.offset(0 as i32 as isize) as i32
        }
        _ => {
            /* Multiple CBR/VBR frames (from 0 to 120 ms) */
            /*case 3:*/
            if len < 1 as i32 {
                return -(4 as i32);
            }
            /* Number of frames encoded in bits 0 to 5 */
            let fresh1 = data;
            data = data.offset(1);
            ch = *fresh1;
            count = ch as i32 & 0x3f as i32;
            if count <= 0 as i32 || framesize * count > 5760 as i32 {
                return -(4 as i32);
            }
            len -= 1;
            /* Padding flag is bit 6 */
            if ch as i32 & 0x40 as i32 != 0 {
                let mut p: i32 = 0;
                loop {
                    let mut tmp: i32 = 0;
                    if len <= 0 as i32 {
                        return -(4 as i32);
                    }
                    let fresh2 = data;
                    data = data.offset(1);
                    p = *fresh2 as i32;
                    len -= 1;
                    tmp = if p == 255 as i32 { 254 as i32 } else { p };
                    len -= tmp;
                    pad += tmp;
                    if !(p == 255 as i32) {
                        break;
                    }
                }
            }
            if len < 0 as i32 {
                return -(4 as i32);
            }
            /* VBR flag is bit 7 */
            cbr = (ch as i32 & 0x80 as i32 == 0) as i32;
            if cbr == 0 {
                /* VBR case */
                last_size = len;
                i = 0 as i32;
                while i < count - 1 as i32 {
                    bytes = parse_size(data, len, size.offset(i as isize));
                    len -= bytes;
                    if (*size.offset(i as isize) as i32) < 0 as i32
                        || *size.offset(i as isize) as i32 > len
                    {
                        return -(4 as i32);
                    }
                    data = data.offset(bytes as isize);
                    last_size -= bytes + *size.offset(i as isize) as i32;
                    i += 1
                }
                if last_size < 0 as i32 {
                    return -(4 as i32);
                }
            } else if self_delimited == 0 {
                /* CBR case */
                last_size = len / count;
                if last_size * count != len {
                    return -(4 as i32);
                }
                i = 0 as i32;
                while i < count - 1 as i32 {
                    *size.offset(i as isize) = last_size as opus_int16;
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
            size.offset(count as isize).offset(-(1 as i32 as isize)),
        );
        len -= bytes;
        if (*size.offset((count - 1 as i32) as isize) as i32) < 0 as i32
            || *size.offset((count - 1 as i32) as isize) as i32 > len
        {
            return -(4 as i32);
        }
        data = data.offset(bytes as isize);
        /* For CBR packets, apply the size to all the frames. */
        if cbr != 0 {
            if *size.offset((count - 1 as i32) as isize) as i32 * count > len {
                return -(4 as i32);
            }
            i = 0 as i32;
            while i < count - 1 as i32 {
                *size.offset(i as isize) = *size.offset((count - 1 as i32) as isize);
                i += 1
            }
        } else if bytes + *size.offset((count - 1 as i32) as isize) as i32 > last_size {
            return -(4 as i32);
        }
    } else {
        /* Because it's not encoded explicitly, it's possible the size of the
        last packet (or all the packets, for the CBR case) is larger than
        1275. Reject them here.*/
        if last_size > 1275 as i32 {
            return -(4 as i32);
        }
        *size.offset((count - 1 as i32) as isize) = last_size as opus_int16
    }
    if !payload_offset.is_null() {
        *payload_offset = data.offset_from(data0) as isize as i32
    }
    i = 0 as i32;
    while i < count {
        if !frames.is_null() {
            let ref mut fresh3 = *frames.offset(i as isize);
            *fresh3 = data
        }
        data = data.offset(*size.offset(i as isize) as i32 as isize);
        i += 1
    }
    if !packet_offset.is_null() {
        *packet_offset = pad + data.offset_from(data0) as isize as opus_int32
    }
    if !out_toc.is_null() {
        *out_toc = toc
    }
    return count;
}
#[no_mangle]

pub unsafe extern "C" fn opus_packet_parse(
    mut data: *const u8,
    mut len: opus_int32,
    mut out_toc: *mut u8,
    mut frames: *mut *const u8,
    mut size: *mut opus_int16,
    mut payload_offset: *mut i32,
) -> i32 {
    return opus_packet_parse_impl(
        data,
        len,
        0 as i32,
        out_toc,
        frames,
        size,
        payload_offset,
        0 as *mut opus_int32,
    );
}

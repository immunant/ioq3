use ::libc;

pub use crate::opus_private_h::ChannelLayout;
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

pub unsafe extern "C" fn validate_layout(
    mut layout: *const crate::opus_private_h::ChannelLayout,
) -> i32 {
    let mut i: i32 = 0;
    let mut max_channel: i32 = 0;
    max_channel = (*layout).nb_streams + (*layout).nb_coupled_streams;
    if max_channel > 255 as i32 {
        return 0 as i32;
    }
    i = 0 as i32;
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as i32 >= max_channel
            && (*layout).mapping[i as usize] as i32 != 255 as i32
        {
            return 0 as i32;
        }
        i += 1
    }
    return 1 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn get_left_channel(
    mut layout: *const crate::opus_private_h::ChannelLayout,
    mut stream_id: i32,
    mut prev: i32,
) -> i32 {
    let mut i: i32 = 0;
    i = if prev < 0 as i32 {
        0 as i32
    } else {
        (prev) + 1 as i32
    };
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as i32 == stream_id * 2 as i32 {
            return i;
        }
        i += 1
    }
    return -(1 as i32);
}
#[no_mangle]

pub unsafe extern "C" fn get_right_channel(
    mut layout: *const crate::opus_private_h::ChannelLayout,
    mut stream_id: i32,
    mut prev: i32,
) -> i32 {
    let mut i: i32 = 0;
    i = if prev < 0 as i32 {
        0 as i32
    } else {
        (prev) + 1 as i32
    };
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as i32
            == stream_id * 2 as i32 + 1 as i32
        {
            return i;
        }
        i += 1
    }
    return -(1 as i32);
}
#[no_mangle]

pub unsafe extern "C" fn get_mono_channel(
    mut layout: *const crate::opus_private_h::ChannelLayout,
    mut stream_id: i32,
    mut prev: i32,
) -> i32 {
    let mut i: i32 = 0;
    i = if prev < 0 as i32 {
        0 as i32
    } else {
        (prev) + 1 as i32
    };
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as i32 == stream_id + (*layout).nb_coupled_streams
        {
            return i;
        }
        i += 1
    }
    return -(1 as i32);
}

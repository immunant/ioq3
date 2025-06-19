use ::libc;

pub use crate::snd_local_h::adpcm_state;
pub use crate::snd_local_h::adpcm_state_t;
pub use crate::snd_local_h::dma_t;
pub use crate::snd_local_h::sfx_s;
pub use crate::snd_local_h::sfx_t;
pub use crate::snd_local_h::sndBuffer;
pub use crate::snd_local_h::sndBuffer_s;
pub use crate::snd_local_h::wavinfo_t;
pub use crate::src::client::snd_dma::dma;
pub use crate::src::client::snd_mem::SND_malloc;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
/* **********************************************************
Copyright 1992 by Stichting Mathematisch Centrum, Amsterdam, The
Netherlands.

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the names of Stichting Mathematisch
Centrum or CWI not be used in advertising or publicity pertaining to
distribution of the software without specific, written prior permission.

STICHTING MATHEMATISCH CENTRUM DISCLAIMS ALL WARRANTIES WITH REGARD TO
THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS, IN NO EVENT SHALL STICHTING MATHEMATISCH CENTRUM BE LIABLE
FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT
OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

******************************************************************/
/*
** Intel/DVI ADPCM coder/decoder.
**
** The algorithm for this coder was taken from the IMA Compatibility Project
** proceedings, Vol 2, Number 2; May 1992.
**
** Version 1.2, 18-Dec-92.
*/
/* Intel ADPCM step variation table */

static mut indexTable: [i32; 16] = [
    -(1 as i32),
    -(1 as i32),
    -(1 as i32),
    -(1 as i32),
    2 as i32,
    4 as i32,
    6 as i32,
    8 as i32,
    -(1 as i32),
    -(1 as i32),
    -(1 as i32),
    -(1 as i32),
    2 as i32,
    4 as i32,
    6 as i32,
    8 as i32,
];

static mut stepsizeTable: [i32; 89] = [
    7 as i32,
    8 as i32,
    9 as i32,
    10 as i32,
    11 as i32,
    12 as i32,
    13 as i32,
    14 as i32,
    16 as i32,
    17 as i32,
    19 as i32,
    21 as i32,
    23 as i32,
    25 as i32,
    28 as i32,
    31 as i32,
    34 as i32,
    37 as i32,
    41 as i32,
    45 as i32,
    50 as i32,
    55 as i32,
    60 as i32,
    66 as i32,
    73 as i32,
    80 as i32,
    88 as i32,
    97 as i32,
    107 as i32,
    118 as i32,
    130 as i32,
    143 as i32,
    157 as i32,
    173 as i32,
    190 as i32,
    209 as i32,
    230 as i32,
    253 as i32,
    279 as i32,
    307 as i32,
    337 as i32,
    371 as i32,
    408 as i32,
    449 as i32,
    494 as i32,
    544 as i32,
    598 as i32,
    658 as i32,
    724 as i32,
    796 as i32,
    876 as i32,
    963 as i32,
    1060 as i32,
    1166 as i32,
    1282 as i32,
    1411 as i32,
    1552 as i32,
    1707 as i32,
    1878 as i32,
    2066 as i32,
    2272 as i32,
    2499 as i32,
    2749 as i32,
    3024 as i32,
    3327 as i32,
    3660 as i32,
    4026 as i32,
    4428 as i32,
    4871 as i32,
    5358 as i32,
    5894 as i32,
    6484 as i32,
    7132 as i32,
    7845 as i32,
    8630 as i32,
    9493 as i32,
    10442 as i32,
    11487 as i32,
    12635 as i32,
    13899 as i32,
    15289 as i32,
    16818 as i32,
    18500 as i32,
    20350 as i32,
    22385 as i32,
    24623 as i32,
    27086 as i32,
    29794 as i32,
    32767 as i32,
];
#[no_mangle]

pub unsafe extern "C" fn S_AdpcmEncode(
    mut indata: *mut i16,
    mut outdata: *mut libc::c_char,
    mut len: i32,
    mut state: *mut adpcm_state,
) {
    let mut inp: *mut i16 = 0 as *mut i16; /* Input buffer pointer */
    let mut outp: *mut i8 = 0 as *mut i8; /* output buffer pointer */
    let mut val: i32 = 0; /* Current input sample value */
    let mut sign: i32 = 0; /* Current adpcm sign bit */
    let mut delta: i32 = 0; /* Current adpcm output value */
    let mut diff: i32 = 0; /* Difference between val and sample */
    let mut step: i32 = 0; /* Stepsize */
    let mut valpred: i32 = 0; /* Predicted output value */
    let mut vpdiff: i32 = 0; /* Current change to valpred */
    let mut index: i32 = 0; /* Current step change index */
    let mut outputbuffer: i32 = 0; /* place to keep previous 4-bit value */
    let mut bufferstep: i32 = 0; /* toggle between outputbuffer/output */
    outp = outdata as *mut i8; // quiet a compiler warning
    inp = indata;
    valpred = (*state).sample as i32;
    index = (*state).index as i32;
    step = stepsizeTable[index as usize];
    outputbuffer = 0 as i32;
    bufferstep = 1 as i32;
    while len > 0 as i32 {
        let fresh0 = inp;
        inp = inp.offset(1);
        val = *fresh0 as i32;
        /* Step 1 - compute difference with previous value */
        diff = val - valpred;
        sign = if diff < 0 as i32 { 8 as i32 } else { 0 as i32 };
        if sign != 0 {
            diff = -diff
        }
        /* Step 2 - Divide and clamp */
        /* Note:
        	** This code *approximately* computes:
        	**    delta = diff*4/step;
        	**    vpdiff = (delta+0.5)*step/4;
        	** but in shift step bits are dropped. The net result of this is
        	** that even if you have fast mul/div hardware you cannot put it to
        	** good use since the fixup would be too expensive.
        	*/
        delta = 0 as i32;
        vpdiff = step >> 3 as i32;
        if diff >= step {
            delta = 4 as i32;
            diff -= step;
            vpdiff += step
        }
        step >>= 1 as i32;
        if diff >= step {
            delta |= 2 as i32;
            diff -= step;
            vpdiff += step
        }
        step >>= 1 as i32;
        if diff >= step {
            delta |= 1 as i32;
            vpdiff += step
        }
        /* Step 3 - Update previous value */
        if sign != 0 {
            valpred -= vpdiff
        } else {
            valpred += vpdiff
        }
        /* Step 4 - Clamp previous value to 16 bits */
        if valpred > 32767 as i32 {
            valpred = 32767 as i32
        } else if valpred < -(32768 as i32) {
            valpred = -(32768 as i32)
        }
        /* Step 5 - Assemble value, update index and step values */
        delta |= sign;
        index += indexTable[delta as usize];
        if index < 0 as i32 {
            index = 0 as i32
        }
        if index > 88 as i32 {
            index = 88 as i32
        }
        step = stepsizeTable[index as usize];
        /* Step 6 - Output value */
        if bufferstep != 0 {
            outputbuffer = delta << 4 as i32 & 0xf0 as i32
        } else {
            let fresh1 = outp;
            outp = outp.offset(1);
            *fresh1 = (delta & 0xf as i32 | outputbuffer) as i8
        }
        bufferstep = (bufferstep == 0) as i32;
        len -= 1
    }
    /* Output last step, if needed */
    if bufferstep == 0 {
        let fresh2 = outp;
        outp = outp.offset(1);
        *fresh2 = outputbuffer as i8
    }
    (*state).sample = valpred as i16;
    (*state).index = index as libc::c_char;
}
/* static */
#[no_mangle]

pub unsafe extern "C" fn S_AdpcmDecode(
    mut indata: *const libc::c_char,
    mut outdata: *mut i16,
    mut len: i32,
    mut state: *mut adpcm_state,
) {
    let mut inp: *mut i8 = 0 as *mut i8; /* Input buffer pointer */
    let mut outp: i32 = 0; /* output buffer pointer */
    let mut sign: i32 = 0; /* Current adpcm sign bit */
    let mut delta: i32 = 0; /* Current adpcm output value */
    let mut step: i32 = 0; /* Stepsize */
    let mut valpred: i32 = 0; /* Predicted value */
    let mut vpdiff: i32 = 0; /* Current change to valpred */
    let mut index: i32 = 0; /* Current step change index */
    let mut inputbuffer: i32 = 0; /* place to keep next 4-bit value */
    let mut bufferstep: i32 = 0; /* toggle between inputbuffer/input */
    outp = 0 as i32; // quiet a compiler warning
    inp = indata as *mut i8;
    valpred = (*state).sample as i32;
    index = (*state).index as i32;
    step = stepsizeTable[index as usize];
    bufferstep = 0 as i32;
    inputbuffer = 0 as i32;
    while len > 0 as i32 {
        /* Step 1 - get the delta value */
        if bufferstep != 0 {
            delta = inputbuffer & 0xf as i32
        } else {
            let fresh3 = inp;
            inp = inp.offset(1);
            inputbuffer = *fresh3 as i32;
            delta = inputbuffer >> 4 as i32 & 0xf as i32
        }
        bufferstep = (bufferstep == 0) as i32;
        /* Step 2 - Find new index value (for later) */
        index += indexTable[delta as usize];
        if index < 0 as i32 {
            index = 0 as i32
        }
        if index > 88 as i32 {
            index = 88 as i32
        }
        /* Step 3 - Separate sign and magnitude */
        sign = delta & 8 as i32;
        delta = delta & 7 as i32;
        /* Step 4 - Compute difference and new predicted value */
        /*
        	** Computes 'vpdiff = (delta+0.5)*step/4', but see comment
        	** in adpcm_coder.
        	*/
        vpdiff = step >> 3 as i32;
        if delta & 4 as i32 != 0 {
            vpdiff += step
        }
        if delta & 2 as i32 != 0 {
            vpdiff += step >> 1 as i32
        }
        if delta & 1 as i32 != 0 {
            vpdiff += step >> 2 as i32
        }
        if sign != 0 {
            valpred -= vpdiff
        } else {
            valpred += vpdiff
        }
        /* Step 5 - clamp output value */
        if valpred > 32767 as i32 {
            valpred = 32767 as i32
        } else if valpred < -(32768 as i32) {
            valpred = -(32768 as i32)
        }
        /* Step 6 - Update step value */
        step = stepsizeTable[index as usize];
        /* Step 7 - Output value */
        *outdata.offset(outp as isize) = valpred as i16;
        outp += 1;
        len -= 1
    }
    (*state).sample = valpred as i16;
    (*state).index = index as libc::c_char;
}
/*
====================
S_AdpcmMemoryNeeded

Returns the amount of memory (in bytes) needed to store the samples in out internal adpcm format
====================
*/
#[no_mangle]

pub unsafe extern "C" fn S_AdpcmMemoryNeeded(mut info: *const wavinfo_t) -> i32 {
    let mut scale: f32 = 0.;
    let mut scaledSampleCount: i32 = 0;
    let mut sampleMemory: i32 = 0;
    let mut blockCount: i32 = 0;
    let mut headerMemory: i32 = 0;
    // determine scale to convert from input sampling rate to desired sampling rate
    scale = (*info).rate as f32 / dma.speed as f32;
    // calc number of samples at playback sampling rate
    scaledSampleCount = ((*info).samples as f32 / scale) as i32;
    // calc memory need to store those samples using ADPCM at 4 bits per sample
    sampleMemory = scaledSampleCount / 2 as i32;
    // calc number of sample blocks needed of PAINTBUFFER_SIZE
    blockCount = scaledSampleCount / 4096 as i32;
    if scaledSampleCount % 4096 as i32 != 0 {
        blockCount += 1
    }
    // calc memory needed to store the block headers
    headerMemory =
        (blockCount as usize).wrapping_mul(::std::mem::size_of::<adpcm_state_t>() as usize) as i32;
    return sampleMemory + headerMemory;
}
/*
====================
S_AdpcmGetSamples
====================
*/
#[no_mangle]

pub unsafe extern "C" fn S_AdpcmGetSamples(mut chunk: *mut sndBuffer, mut to: *mut i16) {
    let mut state: adpcm_state_t = adpcm_state_t {
        sample: 0,
        index: 0,
    };
    let mut out: *mut byte = 0 as *mut byte;
    // get the starting state from the block header
    state.index = (*chunk).adpcm.index;
    state.sample = (*chunk).adpcm.sample;
    out = (*chunk).sndChunk.as_mut_ptr() as *mut byte;
    // get samples
    S_AdpcmDecode(
        out as *mut libc::c_char as *const libc::c_char,
        to,
        1024 as i32 * 2 as i32 * 2 as i32,
        &mut state,
    );
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
// snd_local.h -- private sound definations
// this is in samples
// samples
// floats
// floats
// the final values will be clamped to +/- 0x00ffff00 and shifted down
/* Previous output value */
/* Index into stepsize table */
// couldn't be loaded, so use buzz
// not in Memory
// not in Memory
// mono samples in buffer
// samples with all channels in buffer (samples divided by channels)
// don't mix less than this #
//arbitrary
// START_SAMPLE_IMMEDIATE = set immediately on next mix
// to allow overriding a specific sound
// to allow overriding a specific sound
// 0-255 volume after spatialization
// 0-255 volume after spatialization
// 0-255 volume before spatialization
// only use if fixed_origin is set
// use origin instead of fetching entnum's origin
// sfx structure
// chunk starts this many bytes from file start
// Interface between Q3 sound "api" and the sound backend
/*
====================================================================

  SYSTEM SPECIFIC FUNCTIONS

====================================================================
*/
// initializes cycling through a DMA buffer and returns information on it
// gets the current DMA position
// shutdown the DMA xfer.
//====================================================================
// spatializes a channel
// adpcm functions
/*
====================
S_AdpcmEncodeSound
====================
*/
#[no_mangle]

pub unsafe extern "C" fn S_AdpcmEncodeSound(mut sfx: *mut sfx_t, mut samples: *mut i16) {
    let mut state: adpcm_state_t = adpcm_state_t {
        sample: 0,
        index: 0,
    };
    let mut inOffset: i32 = 0;
    let mut count: i32 = 0;
    let mut n: i32 = 0;
    let mut newchunk: *mut sndBuffer = 0 as *mut sndBuffer;
    let mut chunk: *mut sndBuffer = 0 as *mut sndBuffer;
    let mut out: *mut byte = 0 as *mut byte;
    inOffset = 0 as i32;
    count = (*sfx).soundLength;
    state.index = 0 as i32 as libc::c_char;
    state.sample = *samples.offset(0 as i32 as isize);
    chunk = 0 as *mut sndBuffer;
    while count != 0 {
        n = count;
        if n > 1024 as i32 * 2 as i32 * 2 as i32 {
            n = 1024 as i32 * 2 as i32 * 2 as i32
        }
        newchunk = SND_malloc() as *mut sndBuffer_s;
        if (*sfx).soundData.is_null() {
            (*sfx).soundData = newchunk
        } else if !chunk.is_null() {
            (*chunk).next = newchunk
        }
        chunk = newchunk;
        // output the header
        (*chunk).adpcm.index = state.index;
        (*chunk).adpcm.sample = state.sample;
        out = (*chunk).sndChunk.as_mut_ptr() as *mut byte;
        // encode the samples
        S_AdpcmEncode(
            samples.offset(inOffset as isize),
            out as *mut libc::c_char,
            n,
            &mut state,
        );
        inOffset += n;
        count -= n
    }
}

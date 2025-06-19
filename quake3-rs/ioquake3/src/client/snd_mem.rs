use ::libc;

pub use crate::snd_local_h::adpcm_state;
pub use crate::snd_local_h::adpcm_state_t;
pub use crate::snd_local_h::dma_t;
pub use crate::snd_local_h::sfx_s;
pub use crate::snd_local_h::sfx_t;
pub use crate::snd_local_h::sndBuffer;
pub use crate::snd_local_h::sndBuffer_s;
pub use crate::src::client::snd_adpcm::S_AdpcmEncodeSound;
pub use crate::src::client::snd_codec::snd_info_s;
pub use crate::src::client::snd_codec::snd_info_t;
pub use crate::src::client::snd_codec::S_CodecLoad;
pub use crate::src::client::snd_dma::dma;
pub use crate::src::client::snd_dma::S_FreeOldestSound;

pub use crate::src::qcommon::common::Com_Printf;

pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;

/*
===============================================================================

memory management

===============================================================================
*/

static mut buffer: *mut crate::snd_local_h::sndBuffer =
    0 as *const crate::snd_local_h::sndBuffer as *mut crate::snd_local_h::sndBuffer;

static mut freelist: *mut crate::snd_local_h::sndBuffer =
    0 as *const crate::snd_local_h::sndBuffer as *mut crate::snd_local_h::sndBuffer;

static mut inUse: i32 = 0 as i32;

static mut totalInUse: i32 = 0 as i32;
#[no_mangle]

pub static mut sfxScratchBuffer: *mut i16 = 0 as *const i16 as *mut i16;
#[no_mangle]

pub static mut sfxScratchPointer: *mut crate::snd_local_h::sfx_t =
    0 as *const crate::snd_local_h::sfx_t as *mut crate::snd_local_h::sfx_t;
#[no_mangle]

pub static mut sfxScratchIndex: i32 = 0 as i32;
#[no_mangle]

pub unsafe extern "C" fn SND_free(mut v: *mut crate::snd_local_h::sndBuffer) {
    let ref mut fresh0 = *(v as *mut *mut crate::snd_local_h::sndBuffer);
    *fresh0 = freelist;
    freelist = v;
    inUse = (inUse as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<crate::snd_local_h::sndBuffer>() as libc::c_ulong)
        as i32;
}
#[no_mangle]

pub unsafe extern "C" fn SND_malloc() -> *mut crate::snd_local_h::sndBuffer {
    let mut v: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    while freelist.is_null() {
        crate::src::client::snd_dma::S_FreeOldestSound();
    }
    inUse = (inUse as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<crate::snd_local_h::sndBuffer>() as libc::c_ulong)
        as i32;
    totalInUse = (totalInUse as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<crate::snd_local_h::sndBuffer>() as libc::c_ulong)
        as i32;
    v = freelist;
    freelist = *(freelist as *mut *mut crate::snd_local_h::sndBuffer);
    (*v).next = 0 as *mut crate::snd_local_h::sndBuffer_s;
    return v;
}
#[no_mangle]

pub unsafe extern "C" fn SND_setup() {
    let mut p: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut q: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut cv: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    let mut scs: i32 = 0;
    cv = crate::src::qcommon::cvar::Cvar_Get(
        b"com_soundMegs\x00" as *const u8 as *const libc::c_char,
        b"8\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x1 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    scs = (*cv).integer * 1536 as i32;
    buffer = crate::stdlib::malloc(
        (scs as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::snd_local_h::sndBuffer>() as libc::c_ulong),
    ) as *mut crate::snd_local_h::sndBuffer;
    // allocate the stack based hunk allocator
    sfxScratchBuffer = crate::stdlib::malloc(
        (1024 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<i16>() as libc::c_ulong)
            .wrapping_mul(4 as i32 as libc::c_ulong),
    ) as *mut i16; //Hunk_Alloc(SND_CHUNK_SIZE * sizeof(short) * 4);
    sfxScratchPointer = 0 as *mut crate::snd_local_h::sfx_t;
    inUse = (scs as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<crate::snd_local_h::sndBuffer>() as libc::c_ulong)
        as i32;
    p = buffer;
    q = p.offset(scs as isize);
    loop {
        q = q.offset(-1);
        if !(q > p) {
            break;
        }
        let ref mut fresh1 = *(q as *mut *mut crate::snd_local_h::sndBuffer);
        *fresh1 = q.offset(-(1 as i32 as isize))
    }
    let ref mut fresh2 = *(q as *mut *mut crate::snd_local_h::sndBuffer);
    *fresh2 = 0 as *mut crate::snd_local_h::sndBuffer;
    freelist = p.offset(scs as isize).offset(-(1 as i32 as isize));
    crate::src::qcommon::common::Com_Printf(
        b"Sound memory manager started\n\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn SND_shutdown() {
    ::libc::free(sfxScratchBuffer as *mut libc::c_void);
    ::libc::free(buffer as *mut libc::c_void);
}
/*
================
ResampleSfx

resample / decimate to the current source rate
================
*/

unsafe extern "C" fn ResampleSfx(
    mut sfx: *mut crate::snd_local_h::sfx_t,
    mut channels: i32,
    mut inrate: i32,
    mut inwidth: i32,
    mut samples: i32,
    mut data: *mut crate::src::qcommon::q_shared::byte,
    mut _compressed: crate::src::qcommon::q_shared::qboolean,
) -> i32 {
    let mut outcount: i32 = 0; // this is usually 0.5, 1, or 2
    let mut srcsample: i32 = 0;
    let mut stepscale: f32 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut sample: i32 = 0;
    let mut samplefrac: i32 = 0;
    let mut fracstep: i32 = 0;
    let mut part: i32 = 0;
    let mut chunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    stepscale = inrate as f32 / crate::src::client::snd_dma::dma.speed as f32;
    outcount = (samples as f32 / stepscale) as i32;
    srcsample = 0 as i32;
    samplefrac = 0 as i32;
    fracstep = (stepscale * 256 as i32 as f32 * channels as f32) as i32;
    chunk = (*sfx).soundData;
    i = 0 as i32;
    while i < outcount {
        srcsample += samplefrac >> 8 as i32;
        samplefrac &= 255 as i32;
        samplefrac += fracstep;
        j = 0 as i32;
        while j < channels {
            if inwidth == 2 as i32 {
                sample = *(data as *mut i16).offset((srcsample + j) as isize) as i32
            } else {
                sample = (((*data.offset((srcsample + j) as isize) as i32 - 128 as i32) as u32)
                    << 8 as i32) as i32
            }
            part = i * channels + j & 1024 as i32 - 1 as i32;
            if part == 0 as i32 {
                let mut newchunk: *mut crate::snd_local_h::sndBuffer =
                    0 as *mut crate::snd_local_h::sndBuffer;
                newchunk = SND_malloc();
                if chunk.is_null() {
                    (*sfx).soundData = newchunk
                } else {
                    (*chunk).next = newchunk
                }
                chunk = newchunk
            }
            (*chunk).sndChunk[part as usize] = sample as i16;
            j += 1
        }
        i += 1
    }
    return outcount;
}
/*
================
ResampleSfx

resample / decimate to the current source rate
================
*/

unsafe extern "C" fn ResampleSfxRaw(
    mut sfx: *mut i16,
    mut channels: i32,
    mut inrate: i32,
    mut inwidth: i32,
    mut samples: i32,
    mut data: *mut crate::src::qcommon::q_shared::byte,
) -> i32 {
    let mut outcount: i32 = 0; // this is usually 0.5, 1, or 2
    let mut srcsample: i32 = 0;
    let mut stepscale: f32 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut sample: i32 = 0;
    let mut samplefrac: i32 = 0;
    let mut fracstep: i32 = 0;
    stepscale = inrate as f32 / crate::src::client::snd_dma::dma.speed as f32;
    outcount = (samples as f32 / stepscale) as i32;
    srcsample = 0 as i32;
    samplefrac = 0 as i32;
    fracstep = (stepscale * 256 as i32 as f32 * channels as f32) as i32;
    i = 0 as i32;
    while i < outcount {
        srcsample += samplefrac >> 8 as i32;
        samplefrac &= 255 as i32;
        samplefrac += fracstep;
        j = 0 as i32;
        while j < channels {
            if inwidth == 2 as i32 {
                sample = *(data as *mut i16).offset((srcsample + j) as isize) as i32
            } else {
                sample = (*data.offset((srcsample + j) as isize) as i32 - 128 as i32) << 8 as i32
            }
            *sfx.offset((i * channels + j) as isize) = sample as i16;
            j += 1
        }
        i += 1
    }
    return outcount;
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
//=============================================================================
/*
==============
S_LoadSound

The filename may be different than sfx->name in the case
of a forced fallback of a player specific sound
==============
*/
#[no_mangle]

pub unsafe extern "C" fn S_LoadSound(
    mut sfx: *mut crate::snd_local_h::sfx_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut data: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut samples: *mut i16 = 0 as *mut i16;
    let mut info: crate::src::client::snd_codec::snd_info_t =
        crate::src::client::snd_codec::snd_info_t {
            rate: 0,
            width: 0,
            channels: 0,
            samples: 0,
            size: 0,
            dataofs: 0,
        };
    //	int		size;
    // load it in
    data = crate::src::client::snd_codec::S_CodecLoad(
        (*sfx).soundName.as_mut_ptr(),
        &mut info as *mut _ as *mut crate::src::client::snd_codec::snd_info_s,
    ) as *mut crate::src::qcommon::q_shared::byte;
    if data.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if info.width == 1 as i32 {
        crate::src::qcommon::common::Com_DPrintf(
            b"^3WARNING: %s is a 8 bit audio file\n\x00" as *const u8 as *const libc::c_char,
            (*sfx).soundName.as_mut_ptr(),
        );
    }
    if info.rate != 22050 as i32 {
        crate::src::qcommon::common::Com_DPrintf(
            b"^3WARNING: %s is not a 22kHz audio file\n\x00" as *const u8 as *const libc::c_char,
            (*sfx).soundName.as_mut_ptr(),
        );
    }
    samples = crate::src::qcommon::common::Hunk_AllocateTempMemory(
        ((info.channels * info.samples) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<i16>() as libc::c_ulong)
            .wrapping_mul(2 as i32 as libc::c_ulong) as i32,
    ) as *mut i16;
    (*sfx).lastTimeUsed = crate::src::qcommon::common::Com_Milliseconds() + 1 as i32;
    // each of these compression schemes works just fine
    // but the 16bit quality is much nicer and with a local
    // install assured we can rely upon the sound memory
    // manager to do the right thing for us and page
    // sound in as needed
    if info.channels == 1 as i32
        && (*sfx).soundCompressed as u32 == crate::src::qcommon::q_shared::qtrue as i32 as u32
    {
        (*sfx).soundCompressionMethod = 1 as i32;
        (*sfx).soundData = 0 as *mut crate::snd_local_h::sndBuffer;
        (*sfx).soundLength = ResampleSfxRaw(
            samples,
            info.channels,
            info.rate,
            info.width,
            info.samples,
            data.offset(info.dataofs as isize),
        );
        crate::src::client::snd_adpcm::S_AdpcmEncodeSound(
            sfx as *mut crate::snd_local_h::sfx_s,
            samples,
        );
    } else {
        (*sfx).soundCompressionMethod = 0 as i32;
        (*sfx).soundData = 0 as *mut crate::snd_local_h::sndBuffer;
        (*sfx).soundLength = ResampleSfx(
            sfx,
            info.channels,
            info.rate,
            info.width,
            info.samples,
            data.offset(info.dataofs as isize),
            crate::src::qcommon::q_shared::qfalse,
        )
    }
    (*sfx).soundChannels = info.channels;
    crate::src::qcommon::common::Hunk_FreeTempMemory(samples as *mut libc::c_void);
    crate::src::qcommon::common::Hunk_FreeTempMemory(data as *mut libc::c_void);
    return crate::src::qcommon::q_shared::qtrue;
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
// if origin is NULL, the sound will be dynamically sourced from the entity
// cinematics and voice-over-network will send raw samples
// 1.0 volume will be direct output of source samples
// stop all sounds and the background track
// all continuous looping sounds must be added before calling S_Update
// recompute the relative volumes for all running sounds
// relative to the given entityNum / orientation
// let the sound system know where an entity currently is
// RegisterSound will allways return a valid sample, even if it
// has to create a placeholder.  This prevents continuous filesystem
// checks for missing files
#[no_mangle]

pub unsafe extern "C" fn S_DisplayFreeMemory() {
    crate::src::qcommon::common::Com_Printf(
        b"%d bytes free sound buffer memory, %d total used\n\x00" as *const u8
            as *const libc::c_char,
        inUse,
        totalInUse,
    );
}

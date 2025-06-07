use ::libc;

pub use crate::snd_local_h::adpcm_state;
pub use crate::snd_local_h::adpcm_state_t;
pub use crate::snd_local_h::channel_t;
pub use crate::snd_local_h::dma_t;
pub use crate::snd_local_h::portable_samplepair_t;
pub use crate::snd_local_h::sfx_s;
pub use crate::snd_local_h::sfx_t;
pub use crate::snd_local_h::sndBuffer;
pub use crate::snd_local_h::sndBuffer_s;

pub use crate::src::client::snd_adpcm::S_AdpcmGetSamples;
pub use crate::src::client::snd_dma::dma;
pub use crate::src::client::snd_dma::loop_channels;
pub use crate::src::client::snd_dma::numLoopChannels;
pub use crate::src::client::snd_dma::s_channels;
pub use crate::src::client::snd_dma::s_paintedtime;
pub use crate::src::client::snd_dma::s_rawend;
pub use crate::src::client::snd_dma::s_rawsamples;
pub use crate::src::client::snd_dma::s_testsound;
pub use crate::src::client::snd_main::s_muted;
pub use crate::src::client::snd_main::s_volume;
pub use crate::src::client::snd_mem::sfxScratchBuffer;
pub use crate::src::client::snd_mem::sfxScratchIndex;
pub use crate::src::client::snd_mem::sfxScratchPointer;
pub use crate::src::client::snd_wavelet::decodeWavelet;
pub use crate::src::client::snd_wavelet::mulawToShort;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;

/* Previous output value */
/* Index into stepsize table */
// couldn't be loaded, so use buzz
// not in Memory
// not in Memory
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
// snd_mix.c -- portable code to mix sounds for snd_dma.c

static mut paintbuffer: [crate::snd_local_h::portable_samplepair_t; 4096] =
    [crate::snd_local_h::portable_samplepair_t { left: 0, right: 0 }; 4096];

static mut snd_vol: libc::c_int = 0;
#[no_mangle]

pub static mut snd_p: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]

pub static mut snd_linear_count: libc::c_int = 0;
#[no_mangle]

pub static mut snd_out: *mut libc::c_short = 0 as *const libc::c_short as *mut libc::c_short;
// if configured not to use asm
#[no_mangle]

pub unsafe extern "C" fn S_WriteLinearBlastStereo16() {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < snd_linear_count {
        val = *snd_p.offset(i as isize) >> 8 as libc::c_int;
        if val > 0x7fff as libc::c_int {
            *snd_out.offset(i as isize) = 0x7fff as libc::c_int as libc::c_short
        } else if val < -(32768 as libc::c_int) {
            *snd_out.offset(i as isize) = -(32768 as libc::c_int) as libc::c_short
        } else {
            *snd_out.offset(i as isize) = val as libc::c_short
        }
        val = *snd_p.offset((i + 1 as libc::c_int) as isize) >> 8 as libc::c_int;
        if val > 0x7fff as libc::c_int {
            *snd_out.offset((i + 1 as libc::c_int) as isize) =
                0x7fff as libc::c_int as libc::c_short
        } else if val < -(32768 as libc::c_int) {
            *snd_out.offset((i + 1 as libc::c_int) as isize) =
                -(32768 as libc::c_int) as libc::c_short
        } else {
            *snd_out.offset((i + 1 as libc::c_int) as isize) = val as libc::c_short
        }
        i += 2 as libc::c_int
    }
}
#[no_mangle]

pub unsafe extern "C" fn S_TransferStereo16(
    mut pbuf: *mut libc::c_ulong,
    mut endtime: libc::c_int,
) {
    let mut lpos: libc::c_int = 0;
    let mut ls_paintedtime: libc::c_int = 0;
    snd_p = paintbuffer.as_mut_ptr() as *mut libc::c_int;
    ls_paintedtime = crate::src::client::snd_dma::s_paintedtime;
    while ls_paintedtime < endtime {
        // handle recirculating buffer issues
        lpos = ls_paintedtime % crate::src::client::snd_dma::dma.fullsamples;
        // snd_linear_count * (dma.samplebits/8)
        snd_out = (pbuf as *mut libc::c_short).offset((lpos << 1 as libc::c_int) as isize); // lpos * dma.channels
        snd_linear_count = crate::src::client::snd_dma::dma.fullsamples - lpos; // snd_linear_count *= dma.channels
        if ls_paintedtime + snd_linear_count > endtime {
            snd_linear_count = endtime - ls_paintedtime
        }
        snd_linear_count <<= 1 as libc::c_int;
        S_WriteLinearBlastStereo16();
        snd_p = snd_p.offset(snd_linear_count as isize);
        ls_paintedtime += snd_linear_count >> 1 as libc::c_int;
        if crate::src::client::cl_avi::CL_VideoRecording() as u64 != 0 {
            crate::src::client::cl_avi::CL_WriteAVIAudioFrame(
                snd_out as *mut crate::src::qcommon::q_shared::byte,
                snd_linear_count << 1 as libc::c_int,
            );
        }
    }
}
// write a linear blast of samples
// snd_linear_count / dma.channels
/*
===================
S_TransferPaintBuffer

===================
*/
#[no_mangle]

pub unsafe extern "C" fn S_TransferPaintBuffer(mut endtime: libc::c_int) {
    let mut out_idx: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut step: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut pbuf: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    pbuf = crate::src::client::snd_dma::dma.buffer as *mut libc::c_ulong;
    if (*crate::src::client::snd_dma::s_testsound).integer != 0 {
        // write a fixed sine wave
        count = endtime - crate::src::client::snd_dma::s_paintedtime;
        i = 0 as libc::c_int;
        while i < count {
            paintbuffer[i as usize].right = (crate::stdlib::sin(
                (crate::src::client::snd_dma::s_paintedtime + i) as libc::c_double * 0.1f64,
            ) * 20000 as libc::c_int as libc::c_double
                * 256 as libc::c_int as libc::c_double)
                as libc::c_int;
            paintbuffer[i as usize].left = paintbuffer[i as usize].right;
            i += 1
        }
    }
    if crate::src::client::snd_dma::dma.samplebits == 16 as libc::c_int
        && crate::src::client::snd_dma::dma.channels == 2 as libc::c_int
    {
        // optimized case
        S_TransferStereo16(pbuf, endtime);
    } else {
        // general case
        p = paintbuffer.as_mut_ptr() as *mut libc::c_int;
        count = (endtime - crate::src::client::snd_dma::s_paintedtime)
            * crate::src::client::snd_dma::dma.channels;
        out_idx = crate::src::client::snd_dma::s_paintedtime
            * crate::src::client::snd_dma::dma.channels
            % crate::src::client::snd_dma::dma.samples;
        step = 3 as libc::c_int
            - (if crate::src::client::snd_dma::dma.channels < 2 as libc::c_int {
                crate::src::client::snd_dma::dma.channels
            } else {
                2 as libc::c_int
            });
        if crate::src::client::snd_dma::dma.isfloat != 0
            && crate::src::client::snd_dma::dma.samplebits == 32 as libc::c_int
        {
            let mut out: *mut libc::c_float = pbuf as *mut libc::c_float;
            i = 0 as libc::c_int;
            while i < count {
                if i % crate::src::client::snd_dma::dma.channels >= 2 as libc::c_int {
                    val = 0 as libc::c_int
                } else {
                    val = *p >> 8 as libc::c_int;
                    p = p.offset(step as isize)
                }
                if val > 0x7fff as libc::c_int {
                    val = 0x7fff as libc::c_int
                } else if val < -(32767 as libc::c_int) {
                    /* clamp to one less than max to make division max out at -1.0f. */
                    val = -(32767 as libc::c_int)
                }
                *out.offset(out_idx as isize) = val as libc::c_float / 32767.0f32;
                out_idx = (out_idx + 1 as libc::c_int) % crate::src::client::snd_dma::dma.samples;
                i += 1
            }
        } else if crate::src::client::snd_dma::dma.samplebits == 16 as libc::c_int {
            let mut out_0: *mut libc::c_short = pbuf as *mut libc::c_short;
            i = 0 as libc::c_int;
            while i < count {
                if i % crate::src::client::snd_dma::dma.channels >= 2 as libc::c_int {
                    val = 0 as libc::c_int
                } else {
                    val = *p >> 8 as libc::c_int;
                    p = p.offset(step as isize)
                }
                if val > 0x7fff as libc::c_int {
                    val = 0x7fff as libc::c_int
                } else if val < -(32768 as libc::c_int) {
                    val = -(32768 as libc::c_int)
                }
                *out_0.offset(out_idx as isize) = val as libc::c_short;
                out_idx = (out_idx + 1 as libc::c_int) % crate::src::client::snd_dma::dma.samples;
                i += 1
            }
        } else if crate::src::client::snd_dma::dma.samplebits == 8 as libc::c_int {
            let mut out_1: *mut libc::c_uchar = pbuf as *mut libc::c_uchar;
            i = 0 as libc::c_int;
            while i < count {
                if i % crate::src::client::snd_dma::dma.channels >= 2 as libc::c_int {
                    val = 0 as libc::c_int
                } else {
                    val = *p >> 8 as libc::c_int;
                    p = p.offset(step as isize)
                }
                if val > 0x7fff as libc::c_int {
                    val = 0x7fff as libc::c_int
                } else if val < -(32768 as libc::c_int) {
                    val = -(32768 as libc::c_int)
                }
                *out_1.offset(out_idx as isize) =
                    ((val >> 8 as libc::c_int) + 128 as libc::c_int) as libc::c_uchar;
                out_idx = (out_idx + 1 as libc::c_int) % crate::src::client::snd_dma::dma.samples;
                i += 1
            }
        }
    };
}
/*
===============================================================================

CHANNEL MIXING

===============================================================================
*/

unsafe extern "C" fn S_PaintChannelFrom16_scalar(
    mut ch: *mut crate::snd_local_h::channel_t,
    mut sc: *const crate::snd_local_h::sfx_t,
    mut count: libc::c_int,
    mut sampleOffset: libc::c_int,
    mut bufferOffset: libc::c_int,
) {
    let mut data: libc::c_int = 0;
    let mut aoff: libc::c_int = 0;
    let mut boff: libc::c_int = 0;
    let mut leftvol: libc::c_int = 0;
    let mut rightvol: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut samp: *mut crate::snd_local_h::portable_samplepair_t =
        0 as *mut crate::snd_local_h::portable_samplepair_t;
    let mut chunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut samples: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut ooff: libc::c_float = 0.;
    let mut fdata: [libc::c_float; 2] = [0.; 2];
    let mut fdiv: libc::c_float = 0.;
    let mut fleftvol: libc::c_float = 0.;
    let mut frightvol: libc::c_float = 0.;
    if (*sc).soundChannels <= 0 as libc::c_int {
        return;
    }
    samp = &mut *paintbuffer.as_mut_ptr().offset(bufferOffset as isize)
        as *mut crate::snd_local_h::portable_samplepair_t;
    if (*ch).doppler as u64 != 0 {
        sampleOffset = (sampleOffset as libc::c_float * (*ch).oldDopplerScale) as libc::c_int
    }
    if (*sc).soundChannels == 2 as libc::c_int {
        sampleOffset *= (*sc).soundChannels;
        if sampleOffset & 1 as libc::c_int != 0 {
            sampleOffset &= !(1 as libc::c_int)
        }
    }
    chunk = (*sc).soundData;
    while sampleOffset >= 1024 as libc::c_int {
        chunk = (*chunk).next;
        sampleOffset -= 1024 as libc::c_int;
        if chunk.is_null() {
            chunk = (*sc).soundData
        }
    }
    if (*ch).doppler as u64 == 0 || (*ch).dopplerScale == 1.0f32 {
        leftvol = (*ch).leftvol * snd_vol;
        rightvol = (*ch).rightvol * snd_vol;
        samples = (*chunk).sndChunk.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < count {
            let fresh0 = sampleOffset;
            sampleOffset = sampleOffset + 1;
            data = *samples.offset(fresh0 as isize) as libc::c_int;
            (*samp.offset(i as isize)).left += data * leftvol >> 8 as libc::c_int;
            if (*sc).soundChannels == 2 as libc::c_int {
                let fresh1 = sampleOffset;
                sampleOffset = sampleOffset + 1;
                data = *samples.offset(fresh1 as isize) as libc::c_int
            }
            (*samp.offset(i as isize)).right += data * rightvol >> 8 as libc::c_int;
            if sampleOffset == 1024 as libc::c_int {
                chunk = (*chunk).next;
                samples = (*chunk).sndChunk.as_mut_ptr();
                sampleOffset = 0 as libc::c_int
            }
            i += 1
        }
    } else {
        fleftvol = ((*ch).leftvol * snd_vol) as libc::c_float;
        frightvol = ((*ch).rightvol * snd_vol) as libc::c_float;
        ooff = sampleOffset as libc::c_float;
        samples = (*chunk).sndChunk.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < count {
            aoff = ooff as libc::c_int;
            ooff = ooff + (*ch).dopplerScale * (*sc).soundChannels as libc::c_float;
            boff = ooff as libc::c_int;
            fdata[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
            fdata[0 as libc::c_int as usize] = fdata[1 as libc::c_int as usize];
            j = aoff;
            while j < boff {
                if j == 1024 as libc::c_int {
                    chunk = (*chunk).next;
                    if chunk.is_null() {
                        chunk = (*sc).soundData
                    }
                    samples = (*chunk).sndChunk.as_mut_ptr();
                    ooff -= 1024 as libc::c_int as libc::c_float
                }
                if (*sc).soundChannels == 2 as libc::c_int {
                    fdata[0 as libc::c_int as usize] +=
                        *samples.offset((j & 1024 as libc::c_int - 1 as libc::c_int) as isize)
                            as libc::c_int as libc::c_float;
                    fdata[1 as libc::c_int as usize] += *samples.offset(
                        (j + 1 as libc::c_int & 1024 as libc::c_int - 1 as libc::c_int) as isize,
                    ) as libc::c_int
                        as libc::c_float
                } else {
                    fdata[0 as libc::c_int as usize] +=
                        *samples.offset((j & 1024 as libc::c_int - 1 as libc::c_int) as isize)
                            as libc::c_int as libc::c_float;
                    fdata[1 as libc::c_int as usize] +=
                        *samples.offset((j & 1024 as libc::c_int - 1 as libc::c_int) as isize)
                            as libc::c_int as libc::c_float
                }
                j += (*sc).soundChannels
            }
            fdiv = (256 as libc::c_int * (boff - aoff) / (*sc).soundChannels) as libc::c_float;
            let ref mut fresh2 = (*samp.offset(i as isize)).left;
            *fresh2 = (*fresh2 as libc::c_float
                + fdata[0 as libc::c_int as usize] * fleftvol / fdiv)
                as libc::c_int;
            let ref mut fresh3 = (*samp.offset(i as isize)).right;
            *fresh3 = (*fresh3 as libc::c_float
                + fdata[1 as libc::c_int as usize] * frightvol / fdiv)
                as libc::c_int;
            i += 1
        }
    };
}

unsafe extern "C" fn S_PaintChannelFrom16(
    mut ch: *mut crate::snd_local_h::channel_t,
    mut sc: *const crate::snd_local_h::sfx_t,
    mut count: libc::c_int,
    mut sampleOffset: libc::c_int,
    mut bufferOffset: libc::c_int,
) {
    S_PaintChannelFrom16_scalar(ch, sc, count, sampleOffset, bufferOffset);
}
#[no_mangle]

pub unsafe extern "C" fn S_PaintChannelFromWavelet(
    mut ch: *mut crate::snd_local_h::channel_t,
    mut sc: *mut crate::snd_local_h::sfx_t,
    mut count: libc::c_int,
    mut sampleOffset: libc::c_int,
    mut bufferOffset: libc::c_int,
) {
    let mut data: libc::c_int = 0;
    let mut leftvol: libc::c_int = 0;
    let mut rightvol: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut samp: *mut crate::snd_local_h::portable_samplepair_t =
        0 as *mut crate::snd_local_h::portable_samplepair_t;
    let mut chunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut samples: *mut libc::c_short = 0 as *mut libc::c_short;
    leftvol = (*ch).leftvol * snd_vol;
    rightvol = (*ch).rightvol * snd_vol;
    i = 0 as libc::c_int;
    samp = &mut *paintbuffer.as_mut_ptr().offset(bufferOffset as isize)
        as *mut crate::snd_local_h::portable_samplepair_t;
    chunk = (*sc).soundData;
    while sampleOffset >= 1024 as libc::c_int / 2 as libc::c_int * 4 as libc::c_int {
        chunk = (*chunk).next;
        sampleOffset -= 1024 as libc::c_int / 2 as libc::c_int * 4 as libc::c_int;
        i += 1
    }
    if i != crate::src::client::snd_mem::sfxScratchIndex
        || crate::src::client::snd_mem::sfxScratchPointer != sc
    {
        crate::src::client::snd_adpcm::S_AdpcmGetSamples(
            chunk as *mut crate::snd_local_h::sndBuffer_s,
            crate::src::client::snd_mem::sfxScratchBuffer,
        );
        crate::src::client::snd_mem::sfxScratchIndex = i;
        crate::src::client::snd_mem::sfxScratchPointer = sc
    }
    samples = crate::src::client::snd_mem::sfxScratchBuffer;
    i = 0 as libc::c_int;
    while i < count {
        let fresh4 = sampleOffset;
        sampleOffset = sampleOffset + 1;
        data = *samples.offset(fresh4 as isize) as libc::c_int;
        (*samp.offset(i as isize)).left += data * leftvol >> 8 as libc::c_int;
        (*samp.offset(i as isize)).right += data * rightvol >> 8 as libc::c_int;
        if sampleOffset == 1024 as libc::c_int * 2 as libc::c_int {
            chunk = (*chunk).next;
            crate::src::client::snd_wavelet::decodeWavelet(
                chunk as *mut crate::snd_local_h::sndBuffer_s,
                crate::src::client::snd_mem::sfxScratchBuffer,
            );
            crate::src::client::snd_mem::sfxScratchIndex += 1;
            sampleOffset = 0 as libc::c_int
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn S_PaintChannelFromADPCM(
    mut ch: *mut crate::snd_local_h::channel_t,
    mut sc: *mut crate::snd_local_h::sfx_t,
    mut count: libc::c_int,
    mut sampleOffset: libc::c_int,
    mut bufferOffset: libc::c_int,
) {
    let mut data: libc::c_int = 0;
    let mut leftvol: libc::c_int = 0;
    let mut rightvol: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut samp: *mut crate::snd_local_h::portable_samplepair_t =
        0 as *mut crate::snd_local_h::portable_samplepair_t;
    let mut chunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut samples: *mut libc::c_short = 0 as *mut libc::c_short;
    leftvol = (*ch).leftvol * snd_vol;
    rightvol = (*ch).rightvol * snd_vol;
    i = 0 as libc::c_int;
    samp = &mut *paintbuffer.as_mut_ptr().offset(bufferOffset as isize)
        as *mut crate::snd_local_h::portable_samplepair_t;
    chunk = (*sc).soundData;
    if (*ch).doppler as u64 != 0 {
        sampleOffset = (sampleOffset as libc::c_float * (*ch).oldDopplerScale) as libc::c_int
    }
    while sampleOffset >= 1024 as libc::c_int * 4 as libc::c_int {
        chunk = (*chunk).next;
        sampleOffset -= 1024 as libc::c_int * 4 as libc::c_int;
        i += 1
    }
    if i != crate::src::client::snd_mem::sfxScratchIndex
        || crate::src::client::snd_mem::sfxScratchPointer != sc
    {
        crate::src::client::snd_adpcm::S_AdpcmGetSamples(
            chunk as *mut crate::snd_local_h::sndBuffer_s,
            crate::src::client::snd_mem::sfxScratchBuffer,
        );
        crate::src::client::snd_mem::sfxScratchIndex = i;
        crate::src::client::snd_mem::sfxScratchPointer = sc
    }
    samples = crate::src::client::snd_mem::sfxScratchBuffer;
    i = 0 as libc::c_int;
    while i < count {
        let fresh5 = sampleOffset;
        sampleOffset = sampleOffset + 1;
        data = *samples.offset(fresh5 as isize) as libc::c_int;
        (*samp.offset(i as isize)).left += data * leftvol >> 8 as libc::c_int;
        (*samp.offset(i as isize)).right += data * rightvol >> 8 as libc::c_int;
        if sampleOffset == 1024 as libc::c_int * 4 as libc::c_int {
            chunk = (*chunk).next;
            crate::src::client::snd_adpcm::S_AdpcmGetSamples(
                chunk as *mut crate::snd_local_h::sndBuffer_s,
                crate::src::client::snd_mem::sfxScratchBuffer,
            );
            sampleOffset = 0 as libc::c_int;
            crate::src::client::snd_mem::sfxScratchIndex += 1
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn S_PaintChannelFromMuLaw(
    mut ch: *mut crate::snd_local_h::channel_t,
    mut sc: *mut crate::snd_local_h::sfx_t,
    mut count: libc::c_int,
    mut sampleOffset: libc::c_int,
    mut bufferOffset: libc::c_int,
) {
    let mut data: libc::c_int = 0;
    let mut leftvol: libc::c_int = 0;
    let mut rightvol: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut samp: *mut crate::snd_local_h::portable_samplepair_t =
        0 as *mut crate::snd_local_h::portable_samplepair_t;
    let mut chunk: *mut crate::snd_local_h::sndBuffer = 0 as *mut crate::snd_local_h::sndBuffer;
    let mut samples: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut ooff: libc::c_float = 0.;
    leftvol = (*ch).leftvol * snd_vol;
    rightvol = (*ch).rightvol * snd_vol;
    samp = &mut *paintbuffer.as_mut_ptr().offset(bufferOffset as isize)
        as *mut crate::snd_local_h::portable_samplepair_t;
    chunk = (*sc).soundData;
    while sampleOffset >= 1024 as libc::c_int * 2 as libc::c_int {
        chunk = (*chunk).next;
        sampleOffset -= 1024 as libc::c_int * 2 as libc::c_int;
        if chunk.is_null() {
            chunk = (*sc).soundData
        }
    }
    if (*ch).doppler as u64 == 0 {
        samples = ((*chunk).sndChunk.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte)
            .offset(sampleOffset as isize);
        i = 0 as libc::c_int;
        while i < count {
            data = crate::src::client::snd_wavelet::mulawToShort[*samples as usize] as libc::c_int;
            (*samp.offset(i as isize)).left += data * leftvol >> 8 as libc::c_int;
            (*samp.offset(i as isize)).right += data * rightvol >> 8 as libc::c_int;
            samples = samples.offset(1);
            if !chunk.is_null()
                && samples
                    == ((*chunk).sndChunk.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte)
                        .offset((1024 as libc::c_int * 2 as libc::c_int) as isize)
            {
                chunk = (*chunk).next;
                samples = (*chunk).sndChunk.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte
            }
            i += 1
        }
    } else {
        ooff = sampleOffset as libc::c_float;
        samples = (*chunk).sndChunk.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
        i = 0 as libc::c_int;
        while i < count {
            data = crate::src::client::snd_wavelet::mulawToShort
                [*samples.offset(ooff as libc::c_int as isize) as usize]
                as libc::c_int;
            ooff = ooff + (*ch).dopplerScale;
            (*samp.offset(i as isize)).left += data * leftvol >> 8 as libc::c_int;
            (*samp.offset(i as isize)).right += data * rightvol >> 8 as libc::c_int;
            if ooff >= (1024 as libc::c_int * 2 as libc::c_int) as libc::c_float {
                chunk = (*chunk).next;
                if chunk.is_null() {
                    chunk = (*sc).soundData
                }
                samples =
                    (*chunk).sndChunk.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
                ooff = 0.0f64 as libc::c_float
            }
            i += 1
        }
    };
}
/*
===================
S_PaintChannels
===================
*/
#[no_mangle]

pub unsafe extern "C" fn S_PaintChannels(mut endtime: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut stream: libc::c_int = 0;
    let mut ch: *mut crate::snd_local_h::channel_t = 0 as *mut crate::snd_local_h::channel_t;
    let mut sc: *mut crate::snd_local_h::sfx_t = 0 as *mut crate::snd_local_h::sfx_t;
    let mut ltime: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut sampleOffset: libc::c_int = 0;
    if (*crate::src::client::snd_main::s_muted).integer != 0 {
        snd_vol = 0 as libc::c_int
    } else {
        snd_vol = ((*crate::src::client::snd_main::s_volume).value
            * 255 as libc::c_int as libc::c_float) as libc::c_int
    }
    //Com_Printf ("%i to %i\n", s_paintedtime, endtime);
    while crate::src::client::snd_dma::s_paintedtime < endtime {
        // if paintbuffer is smaller than DMA buffer
        // we may need to fill it multiple times
        end = endtime;
        if endtime - crate::src::client::snd_dma::s_paintedtime > 4096 as libc::c_int {
            end = crate::src::client::snd_dma::s_paintedtime + 4096 as libc::c_int
        }
        // clear the paint buffer and mix any raw samples...
        crate::stdlib::memset(
            paintbuffer.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[crate::snd_local_h::portable_samplepair_t; 4096]>()
                as libc::c_ulong,
        );
        stream = 0 as libc::c_int;
        while stream < 64 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int {
            if crate::src::client::snd_dma::s_rawend[stream as usize]
                >= crate::src::client::snd_dma::s_paintedtime
            {
                // copy from the streaming sound source
                let mut rawsamples: *const crate::snd_local_h::portable_samplepair_t =
                    crate::src::client::snd_dma::s_rawsamples[stream as usize].as_mut_ptr();
                let stop: libc::c_int =
                    if end < crate::src::client::snd_dma::s_rawend[stream as usize] {
                        end
                    } else {
                        crate::src::client::snd_dma::s_rawend[stream as usize]
                    };
                i = crate::src::client::snd_dma::s_paintedtime;
                while i < stop {
                    let s: libc::c_int = i & 16384 as libc::c_int - 1 as libc::c_int;
                    paintbuffer[(i - crate::src::client::snd_dma::s_paintedtime) as usize].left +=
                        (*rawsamples.offset(s as isize)).left;
                    paintbuffer[(i - crate::src::client::snd_dma::s_paintedtime) as usize].right +=
                        (*rawsamples.offset(s as isize)).right;
                    i += 1
                }
            }
            stream += 1
        }
        // paint in the channels.
        ch = crate::src::client::snd_dma::s_channels.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 96 as libc::c_int {
            if !((*ch).thesfx.is_null()
                || ((*ch).leftvol as libc::c_double) < 0.25f64
                    && ((*ch).rightvol as libc::c_double) < 0.25f64)
            {
                ltime = crate::src::client::snd_dma::s_paintedtime;
                sc = (*ch).thesfx;
                if !((*sc).soundData.is_null() || (*sc).soundLength == 0 as libc::c_int) {
                    sampleOffset = ltime - (*ch).startSample;
                    count = end - ltime;
                    if sampleOffset + count > (*sc).soundLength {
                        count = (*sc).soundLength - sampleOffset
                    }
                    if count > 0 as libc::c_int {
                        if (*sc).soundCompressionMethod == 1 as libc::c_int {
                            S_PaintChannelFromADPCM(
                                ch,
                                sc,
                                count,
                                sampleOffset,
                                ltime - crate::src::client::snd_dma::s_paintedtime,
                            );
                        } else if (*sc).soundCompressionMethod == 2 as libc::c_int {
                            S_PaintChannelFromWavelet(
                                ch,
                                sc,
                                count,
                                sampleOffset,
                                ltime - crate::src::client::snd_dma::s_paintedtime,
                            );
                        } else if (*sc).soundCompressionMethod == 3 as libc::c_int {
                            S_PaintChannelFromMuLaw(
                                ch,
                                sc,
                                count,
                                sampleOffset,
                                ltime - crate::src::client::snd_dma::s_paintedtime,
                            );
                        } else {
                            S_PaintChannelFrom16(
                                ch,
                                sc,
                                count,
                                sampleOffset,
                                ltime - crate::src::client::snd_dma::s_paintedtime,
                            );
                        }
                    }
                }
            }
            i += 1;
            ch = ch.offset(1)
        }
        // paint in the looped channels.
        ch = crate::src::client::snd_dma::loop_channels.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < crate::src::client::snd_dma::numLoopChannels {
            if !((*ch).thesfx.is_null() || (*ch).leftvol == 0 && (*ch).rightvol == 0) {
                ltime = crate::src::client::snd_dma::s_paintedtime;
                sc = (*ch).thesfx;
                if !((*sc).soundData.is_null() || (*sc).soundLength == 0 as libc::c_int) {
                    loop
                    // we might have to make two passes if it
                    // is a looping sound effect and the end of
                    // the sample is hit
                    {
                        sampleOffset = ltime % (*sc).soundLength;
                        count = end - ltime;
                        if sampleOffset + count > (*sc).soundLength {
                            count = (*sc).soundLength - sampleOffset
                        }
                        if count > 0 as libc::c_int {
                            if (*sc).soundCompressionMethod == 1 as libc::c_int {
                                S_PaintChannelFromADPCM(
                                    ch,
                                    sc,
                                    count,
                                    sampleOffset,
                                    ltime - crate::src::client::snd_dma::s_paintedtime,
                                );
                            } else if (*sc).soundCompressionMethod == 2 as libc::c_int {
                                S_PaintChannelFromWavelet(
                                    ch,
                                    sc,
                                    count,
                                    sampleOffset,
                                    ltime - crate::src::client::snd_dma::s_paintedtime,
                                );
                            } else if (*sc).soundCompressionMethod == 3 as libc::c_int {
                                S_PaintChannelFromMuLaw(
                                    ch,
                                    sc,
                                    count,
                                    sampleOffset,
                                    ltime - crate::src::client::snd_dma::s_paintedtime,
                                );
                            } else {
                                S_PaintChannelFrom16(
                                    ch,
                                    sc,
                                    count,
                                    sampleOffset,
                                    ltime - crate::src::client::snd_dma::s_paintedtime,
                                );
                            }
                            ltime += count
                        }
                        if !(ltime < end) {
                            break;
                        }
                    }
                }
            }
            i += 1;
            ch = ch.offset(1)
        }
        // transfer out according to DMA format
        S_TransferPaintBuffer(end);
        crate::src::client::snd_dma::s_paintedtime = end
    }
}

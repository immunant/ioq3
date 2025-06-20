use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int16_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::snd_local_h::dma_t;
pub use crate::src::client::snd_dma::dma;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::Q_stricmp;

pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::SDL_AudioCallback;
pub use crate::stdlib::SDL_AudioDeviceID;
pub use crate::stdlib::SDL_AudioFormat;
pub use crate::stdlib::SDL_AudioSpec;
pub use crate::stdlib::SDL_ClearQueuedAudio;
pub use crate::stdlib::SDL_CloseAudioDevice;
pub use crate::stdlib::SDL_DequeueAudio;
pub use crate::stdlib::SDL_GetCurrentAudioDriver;

pub use crate::stdlib::SDL_GetQueuedAudioSize;
pub use crate::stdlib::SDL_LockAudioDevice;
pub use crate::stdlib::SDL_OpenAudioDevice;
pub use crate::stdlib::SDL_PauseAudioDevice;
pub use crate::stdlib::SDL_UnlockAudioDevice;
pub use crate::stdlib::SDL_memset;
pub use crate::stdlib::Sint16;
pub use crate::stdlib::Uint16;
pub use crate::stdlib::Uint32;
pub use crate::stdlib::Uint8;
pub use crate::stdlib::SDL_FALSE;
pub use crate::stdlib::SDL_TRUE;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_155 {
    pub enumFormat: Uint16,
    pub stringFormat: *mut libc::c_char,
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
#[no_mangle]

pub static mut snd_inited: qboolean = qfalse;
#[no_mangle]

pub static mut s_sdlBits: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut s_sdlSpeed: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut s_sdlChannels: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut s_sdlDevSamps: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut s_sdlMixSamps: *mut cvar_t = std::ptr::null_mut();
/* The audio callback. All the magic happens here. */

static mut dmapos: i32 = 0 as i32;

static mut dmasize: i32 = 0 as i32;

static mut sdlPlaybackDevice: SDL_AudioDeviceID = 0;

static mut sdlCaptureDevice: SDL_AudioDeviceID = 0;

static mut s_sdlCapture: *mut cvar_t = std::ptr::null_mut();

static mut sdlMasterGain: f32 = 1.0f32;
/*
===============
SNDDMA_AudioCallback
===============
*/

unsafe extern "C" fn SNDDMA_AudioCallback(
    mut _userdata: *mut libc::c_void,
    mut stream: *mut Uint8,
    mut len: i32,
) {
    let mut pos: i32 = dmapos * (dma.samplebits / 8 as i32);
    if pos >= dmasize {
        pos = 0 as i32;
        dmapos = pos
    }
    if snd_inited as u64 == 0 {
        /* shouldn't happen, but just in case... */
        crate::stdlib::memset(stream as *mut libc::c_void, '\u{0}' as i32, len as usize); /* bytes to buffer's end. */
        return;
    } else {
        let mut tobufend: i32 = dmasize - pos;
        let mut len1: i32 = len;
        let mut len2: i32 = 0 as i32;
        if len1 > tobufend {
            len1 = tobufend;
            len2 = len - len1
        }
        crate::stdlib::memcpy(
            stream as *mut libc::c_void,
            dma.buffer.offset(pos as isize) as *const libc::c_void,
            len1 as usize,
        );
        if len2 <= 0 as i32 {
            dmapos += len1 / (dma.samplebits / 8 as i32)
        } else {
            /* wraparound? */
            crate::stdlib::memcpy(
                stream.offset(len1 as isize) as *mut libc::c_void,
                dma.buffer as *const libc::c_void,
                len2 as usize,
            );
            dmapos = len2 / (dma.samplebits / 8 as i32)
        }
    }
    if dmapos >= dmasize {
        dmapos = 0 as i32
    }
    if sdlMasterGain != 1.0f32 {
        let mut i: i32 = 0;
        if dma.isfloat != 0 && dma.samplebits == 32 as i32 {
            let mut ptr: *mut f32 = stream as *mut f32;
            len = (len as usize).wrapping_div(::std::mem::size_of::<f32>() as usize) as i32;
            i = 0 as i32;
            while i < len {
                *ptr *= sdlMasterGain;
                i += 1;
                ptr = ptr.offset(1)
            }
        } else if dma.samplebits == 16 as i32 {
            let mut ptr_0: *mut Sint16 = stream as *mut Sint16;
            len = (len as usize).wrapping_div(::std::mem::size_of::<Sint16>() as usize) as i32;
            i = 0 as i32;
            while i < len {
                *ptr_0 = (*ptr_0 as f32 * sdlMasterGain) as Sint16;
                i += 1;
                ptr_0 = ptr_0.offset(1)
            }
        } else if dma.samplebits == 8 as i32 {
            let mut ptr_1: *mut Uint8 = stream;
            len = (len as usize).wrapping_div(::std::mem::size_of::<Uint8>() as usize) as i32;
            i = 0 as i32;
            while i < len {
                *ptr_1 = (*ptr_1 as f32 * sdlMasterGain) as Uint8;
                i += 1;
                ptr_1 = ptr_1.offset(1)
            }
        }
    };
}

static mut formatToStringTable: [C2RustUnnamed_155; 8] = [
    {
        let mut init = C2RustUnnamed_155 {
            enumFormat: 0x8 as i32 as Uint16,
            stringFormat: b"AUDIO_U8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_155 {
            enumFormat: 0x8008 as i32 as Uint16,
            stringFormat: b"AUDIO_S8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_155 {
            enumFormat: 0x10 as i32 as Uint16,
            stringFormat: b"AUDIO_U16LSB\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_155 {
            enumFormat: 0x8010 as i32 as Uint16,
            stringFormat: b"AUDIO_S16LSB\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_155 {
            enumFormat: 0x1010 as i32 as Uint16,
            stringFormat: b"AUDIO_U16MSB\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_155 {
            enumFormat: 0x9010 as i32 as Uint16,
            stringFormat: b"AUDIO_S16MSB\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_155 {
            enumFormat: 0x8120 as i32 as Uint16,
            stringFormat: b"AUDIO_F32LSB\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_155 {
            enumFormat: 0x9120 as i32 as Uint16,
            stringFormat: b"AUDIO_F32MSB\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
];
// Initialized in run_static_initializers

static mut formatToStringTableSize: i32 = 0;
/*
===============
SNDDMA_PrintAudiospec
===============
*/

unsafe extern "C" fn SNDDMA_PrintAudiospec(
    mut str: *const libc::c_char,
    mut spec: *const SDL_AudioSpec,
) {
    let mut i: i32 = 0;
    let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
    Com_Printf(b"%s:\n\x00" as *const u8 as *const libc::c_char, str);
    i = 0 as i32;
    while i < formatToStringTableSize {
        if (*spec).format as i32 == formatToStringTable[i as usize].enumFormat as i32 {
            fmt = formatToStringTable[i as usize].stringFormat
        }
        i += 1
    }
    if !fmt.is_null() {
        Com_Printf(
            b"  Format:   %s\n\x00" as *const u8 as *const libc::c_char,
            fmt,
        );
    } else {
        Com_Printf(b"  Format:   ^1UNKNOWN\n\x00" as *const u8 as *const libc::c_char);
    }
    Com_Printf(
        b"  Freq:     %d\n\x00" as *const u8 as *const libc::c_char,
        (*spec).freq,
    );
    Com_Printf(
        b"  Samples:  %d\n\x00" as *const u8 as *const libc::c_char,
        (*spec).samples as i32,
    );
    Com_Printf(
        b"  Channels: %d\n\x00" as *const u8 as *const libc::c_char,
        (*spec).channels as i32,
    );
}
/*
===============
SNDDMA_Init
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_Init() -> qboolean {
    let mut desired: SDL_AudioSpec = SDL_AudioSpec {
        freq: 0,
        format: 0,
        channels: 0,
        silence: 0,
        samples: 0,
        padding: 0,
        size: 0,
        callback: None,
        userdata: 0 as *mut libc::c_void,
    };
    let mut obtained: SDL_AudioSpec = SDL_AudioSpec {
        freq: 0,
        format: 0,
        channels: 0,
        silence: 0,
        samples: 0,
        padding: 0,
        size: 0,
        callback: None,
        userdata: 0 as *mut libc::c_void,
    };
    let mut tmp: i32 = 0;
    if snd_inited as u64 != 0 {
        return qtrue;
    }
    if s_sdlBits.is_null() {
        s_sdlBits = crate::src::qcommon::cvar::Cvar_Get(
            b"s_sdlBits\x00" as *const u8 as *const libc::c_char,
            b"16\x00" as *const u8 as *const libc::c_char,
            0x1 as i32,
        ) as *mut cvar_s;
        s_sdlSpeed = crate::src::qcommon::cvar::Cvar_Get(
            b"s_sdlSpeed\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            0x1 as i32,
        ) as *mut cvar_s;
        s_sdlChannels = crate::src::qcommon::cvar::Cvar_Get(
            b"s_sdlChannels\x00" as *const u8 as *const libc::c_char,
            b"2\x00" as *const u8 as *const libc::c_char,
            0x1 as i32,
        ) as *mut cvar_s;
        s_sdlDevSamps = crate::src::qcommon::cvar::Cvar_Get(
            b"s_sdlDevSamps\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            0x1 as i32,
        ) as *mut cvar_s;
        s_sdlMixSamps = crate::src::qcommon::cvar::Cvar_Get(
            b"s_sdlMixSamps\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            0x1 as i32,
        ) as *mut cvar_s
    }
    Com_Printf(b"SDL_Init( SDL_INIT_AUDIO )... \x00" as *const u8 as *const libc::c_char);
    if crate::stdlib::SDL_Init(0x10 as u32) != 0 as i32 {
        Com_Printf(
            b"FAILED (%s)\n\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::SDL_GetError(),
        );
        return qfalse;
    }
    Com_Printf(b"OK\n\x00" as *const u8 as *const libc::c_char);
    Com_Printf(
        b"SDL audio driver is \"%s\".\n\x00" as *const u8 as *const libc::c_char,
        SDL_GetCurrentAudioDriver(),
    );
    crate::stdlib::memset(
        &mut desired as *mut SDL_AudioSpec as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<SDL_AudioSpec>() as usize,
    );
    crate::stdlib::memset(
        &mut obtained as *mut SDL_AudioSpec as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<SDL_AudioSpec>() as usize,
    );
    tmp = (*s_sdlBits).value as i32;
    if tmp != 16 as i32 && tmp != 8 as i32 {
        tmp = 16 as i32
    }
    desired.freq = (*s_sdlSpeed).value as i32;
    if desired.freq == 0 {
        desired.freq = 22050 as i32
    }
    desired.format = if tmp == 16 as i32 {
        0x8010 as i32
    } else {
        0x8 as i32
    } as SDL_AudioFormat;
    // I dunno if this is the best idea, but I'll give it a try...
    //  should probably check a cvar for this...
    if (*s_sdlDevSamps).value != 0. {
        desired.samples = (*s_sdlDevSamps).value as Uint16
    } else if desired.freq <= 11025 as i32 {
        desired.samples = 256 as i32 as Uint16
    } else if desired.freq <= 22050 as i32 {
        desired.samples = 512 as i32 as Uint16
    } else if desired.freq <= 44100 as i32 {
        desired.samples = 1024 as i32 as Uint16
    } else {
        desired.samples = 2048 as i32 as Uint16
    }
    desired.channels = (*s_sdlChannels).value as i32 as Uint8;
    desired.callback = Some(
        SNDDMA_AudioCallback
            as unsafe extern "C" fn(_: *mut libc::c_void, _: *mut Uint8, _: i32) -> (),
    );
    sdlPlaybackDevice = SDL_OpenAudioDevice(
        0 as *const libc::c_char,
        SDL_FALSE as i32,
        &mut desired,
        &mut obtained,
        0x1 as i32 | 0x2 as i32 | 0x4 as i32 | 0x8 as i32,
    );
    if sdlPlaybackDevice == 0 as i32 as u32 {
        Com_Printf(
            b"SDL_OpenAudioDevice() failed: %s\n\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::SDL_GetError(),
        );
        crate::stdlib::SDL_QuitSubSystem(0x10 as u32);
        return qfalse;
    }
    SNDDMA_PrintAudiospec(
        b"SDL_AudioSpec\x00" as *const u8 as *const libc::c_char,
        &mut obtained,
    );
    // just pick a sane default.
    // (*shrug*)
    // dma.samples needs to be big, or id's mixer will just refuse to
    //  work at all; we need to keep it significantly bigger than the
    //  amount of SDL callback samples, and just copy a little each time
    //  the callback runs.
    // 32768 is what the OSS driver filled in here on my system. I don't
    //  know if it's a good value overall, but at least we know it's
    //  reasonable...this is why I let the user override.
    tmp = (*s_sdlMixSamps).value as i32;
    if tmp == 0 {
        tmp = obtained.samples as i32 * obtained.channels as i32 * 10 as i32
    }
    // samples must be divisible by number of channels
    tmp -= tmp % obtained.channels as i32;
    dmapos = 0 as i32;
    dma.samplebits = obtained.format as i32 & 0xff as i32;
    dma.isfloat = obtained.format as i32 & (1 as i32) << 8 as i32;
    dma.channels = obtained.channels as i32;
    dma.samples = tmp;
    dma.fullsamples = dma.samples / dma.channels;
    dma.submission_chunk = 1 as i32;
    dma.speed = obtained.freq;
    dmasize = dma.samples * (dma.samplebits / 8 as i32);
    dma.buffer = crate::stdlib::calloc(1 as i32 as usize, dmasize as usize) as *mut byte;
    // !!! FIXME: some of these SDL_OpenAudioDevice() values should be cvars.
    s_sdlCapture = crate::src::qcommon::cvar::Cvar_Get(
        b"s_sdlCapture\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    ) as *mut cvar_s;
    // !!! FIXME: pulseaudio capture records audio the entire time the program is running. https://bugzilla.libsdl.org/show_bug.cgi?id=4087
    if Q_stricmp(
        SDL_GetCurrentAudioDriver(),
        b"pulseaudio\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        Com_Printf(b"SDL audio capture support disabled for pulseaudio (https://bugzilla.libsdl.org/show_bug.cgi?id=4087)\n\x00"
                       as *const u8 as *const libc::c_char);
    } else if (*s_sdlCapture).integer == 0 {
        Com_Printf(
            b"SDL audio capture support disabled by user (\'+set s_sdlCapture 1\' to enable)\n\x00"
                as *const u8 as *const libc::c_char,
        );
    } else if (*crate::src::client::cl_main::cl_useMumble).integer != 0 {
        Com_Printf(
            b"SDL audio capture support disabled for Mumble support\n\x00" as *const u8
                as *const libc::c_char,
        );
    } else {
        /* !!! FIXME: list available devices and let cvar specify one, like OpenAL does */
        let mut spec: SDL_AudioSpec = SDL_AudioSpec {
            freq: 0,
            format: 0,
            channels: 0,
            silence: 0,
            samples: 0,
            padding: 0,
            size: 0,
            callback: None,
            userdata: 0 as *mut libc::c_void,
        }; // start callback.
        SDL_memset(
            &mut spec as *mut SDL_AudioSpec as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<SDL_AudioSpec>() as usize,
        );
        spec.freq = 48000 as i32;
        spec.format = 0x8010 as i32 as SDL_AudioFormat;
        spec.channels = 1 as i32 as Uint8;
        spec.samples = (20 as i32 * 48 as i32 * 3 as i32 * 4 as i32) as Uint16;
        sdlCaptureDevice = SDL_OpenAudioDevice(
            0 as *const libc::c_char,
            SDL_TRUE as i32,
            &mut spec,
            0 as *mut SDL_AudioSpec,
            0 as i32,
        );
        Com_Printf(
            b"SDL capture device %s.\n\x00" as *const u8 as *const libc::c_char,
            if sdlCaptureDevice == 0 as i32 as u32 {
                b"failed to open\x00" as *const u8 as *const libc::c_char
            } else {
                b"opened\x00" as *const u8 as *const libc::c_char
            },
        );
    }
    sdlMasterGain = 1.0f32;
    Com_Printf(b"Starting SDL audio callback...\n\x00" as *const u8 as *const libc::c_char);
    SDL_PauseAudioDevice(sdlPlaybackDevice, 0 as i32);
    // don't unpause the capture device; we'll do that in StartCapture.
    Com_Printf(b"SDL audio initialized.\n\x00" as *const u8 as *const libc::c_char);
    snd_inited = qtrue;
    return qtrue;
}
/*
===============
SNDDMA_GetDMAPos
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_GetDMAPos() -> i32 {
    return dmapos;
}
/*
===============
SNDDMA_Shutdown
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_Shutdown() {
    if sdlPlaybackDevice != 0 as i32 as u32 {
        Com_Printf(
            b"Closing SDL audio playback device...\n\x00" as *const u8 as *const libc::c_char,
        );
        SDL_CloseAudioDevice(sdlPlaybackDevice);
        Com_Printf(b"SDL audio playback device closed.\n\x00" as *const u8 as *const libc::c_char);
        sdlPlaybackDevice = 0 as i32 as SDL_AudioDeviceID
    }
    if sdlCaptureDevice != 0 {
        Com_Printf(
            b"Closing SDL audio capture device...\n\x00" as *const u8 as *const libc::c_char,
        );
        SDL_CloseAudioDevice(sdlCaptureDevice);
        Com_Printf(b"SDL audio capture device closed.\n\x00" as *const u8 as *const libc::c_char);
        sdlCaptureDevice = 0 as i32 as SDL_AudioDeviceID
    }
    crate::stdlib::SDL_QuitSubSystem(0x10 as u32);
    libc::free(dma.buffer as *mut libc::c_void);
    dma.buffer = 0 as *mut byte;
    dmasize = 0 as i32;
    dmapos = dmasize;
    snd_inited = qfalse;
    Com_Printf(b"SDL audio shut down.\n\x00" as *const u8 as *const libc::c_char);
}
/*
===============
SNDDMA_Submit

Send sound to device if buffer isn't really the dma buffer
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_Submit() {
    SDL_UnlockAudioDevice(sdlPlaybackDevice);
}
/*
===============
SNDDMA_BeginPainting
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_BeginPainting() {
    SDL_LockAudioDevice(sdlPlaybackDevice);
}
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_StartCapture() {
    if sdlCaptureDevice != 0 {
        SDL_ClearQueuedAudio(sdlCaptureDevice);
        SDL_PauseAudioDevice(sdlCaptureDevice, 0 as i32);
    };
}
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_AvailableCaptureSamples() -> i32 {
    // divided by 2 to convert from bytes to (mono16) samples.
    return if sdlCaptureDevice != 0 {
        SDL_GetQueuedAudioSize(sdlCaptureDevice).wrapping_div(2 as i32 as u32)
    } else {
        0 as i32 as u32
    } as i32;
}
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_Capture(mut samples: i32, mut data: *mut byte) {
    // multiplied by 2 to convert from (mono16) samples to bytes.
    if sdlCaptureDevice != 0 {
        SDL_DequeueAudio(
            sdlCaptureDevice,
            data as *mut libc::c_void,
            (samples * 2 as i32) as Uint32,
        );
    } else {
        SDL_memset(
            data as *mut libc::c_void,
            '\u{0}' as i32,
            (samples * 2 as i32) as size_t,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_StopCapture() {
    if sdlCaptureDevice != 0 {
        SDL_PauseAudioDevice(sdlCaptureDevice, 1 as i32);
    };
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
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_MasterGain(mut val: f32) {
    sdlMasterGain = val;
}
unsafe extern "C" fn run_static_initializers() {
    formatToStringTableSize = (::std::mem::size_of::<[C2RustUnnamed_155; 8]>() as usize)
        .wrapping_div(::std::mem::size_of::<C2RustUnnamed_155>() as usize)
        as i32
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

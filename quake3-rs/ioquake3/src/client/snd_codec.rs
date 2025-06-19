// =============== BEGIN snd_codec_h ================
pub type snd_info_t = crate::src::client::snd_codec::snd_info_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_info_s {
    pub rate: i32,
    pub width: i32,
    pub channels: i32,
    pub samples: i32,
    pub size: i32,
    pub dataofs: i32,
}

pub type snd_codec_t = crate::src::client::snd_codec::snd_codec_s;

pub type snd_stream_t = crate::src::client::snd_codec::snd_stream_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_stream_s {
    pub codec: *mut crate::src::client::snd_codec::snd_codec_t,
    pub file: crate::src::qcommon::q_shared::fileHandle_t,
    pub info: crate::src::client::snd_codec::snd_info_t,
    pub length: i32,
    pub pos: i32,
    pub ptr: *mut libc::c_void,
}

pub type CODEC_LOAD = Option<
    unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut crate::src::client::snd_codec::snd_info_t,
    ) -> *mut libc::c_void,
>;

pub type CODEC_OPEN = Option<
    unsafe extern "C" fn(
        _: *const libc::c_char,
    ) -> *mut crate::src::client::snd_codec::snd_stream_t,
>;

pub type CODEC_READ = Option<
    unsafe extern "C" fn(
        _: *mut crate::src::client::snd_codec::snd_stream_t,
        _: i32,
        _: *mut libc::c_void,
    ) -> i32,
>;

pub type CODEC_CLOSE =
    Option<unsafe extern "C" fn(_: *mut crate::src::client::snd_codec::snd_stream_t) -> ()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_codec_s {
    pub ext: *mut libc::c_char,
    pub load: crate::src::client::snd_codec::CODEC_LOAD,
    pub open: crate::src::client::snd_codec::CODEC_OPEN,
    pub read: crate::src::client::snd_codec::CODEC_READ,
    pub close: crate::src::client::snd_codec::CODEC_CLOSE,
    pub next: *mut crate::src::client::snd_codec::snd_codec_t,
}
use ::libc;

pub use crate::src::client::snd_codec_ogg::ogg_codec;
pub use crate::src::client::snd_codec_opus::opus_codec;
pub use crate::src::client::snd_codec_wav::wav_codec;

pub use crate::src::qcommon::common::Com_Printf;

pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::COM_GetExtension;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.
Copyright (C) 2005 Stuart Dalton (badcdev@gmail.com)

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

static mut codecs: *mut crate::src::client::snd_codec::snd_codec_t = 0
    as *const crate::src::client::snd_codec::snd_codec_t
    as *mut crate::src::client::snd_codec::snd_codec_t;
/*
=================
S_CodecGetSound

Opens/loads a sound, tries codec based on the sound's file extension
then tries all supported codecs.
=================
*/

unsafe extern "C" fn S_CodecGetSound(
    mut filename: *const libc::c_char,
    mut info: *mut crate::src::client::snd_codec::snd_info_t,
) -> *mut libc::c_void {
    let mut codec: *mut crate::src::client::snd_codec::snd_codec_t =
        0 as *mut crate::src::client::snd_codec::snd_codec_t;
    let mut orgCodec: *mut crate::src::client::snd_codec::snd_codec_t =
        0 as *mut crate::src::client::snd_codec::snd_codec_t;
    let mut orgNameFailed: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut localName: [libc::c_char; 64] = [0; 64];
    let mut ext: *const libc::c_char = 0 as *const libc::c_char;
    let mut altName: [libc::c_char; 64] = [0; 64];
    let mut rtn: *mut libc::c_void = 0 as *mut libc::c_void;
    crate::src::qcommon::q_shared::Q_strncpyz(localName.as_mut_ptr(), filename, 64 as i32);
    ext = crate::src::qcommon::q_shared::COM_GetExtension(localName.as_mut_ptr());
    if *ext != 0 {
        // Look for the correct loader and use it
        codec = codecs;
        while !codec.is_null() {
            if crate::src::qcommon::q_shared::Q_stricmp(ext, (*codec).ext) == 0 {
                // Load
                if !info.is_null() {
                    rtn = (*codec).load.expect("non-null function pointer")(
                        localName.as_mut_ptr(),
                        info,
                    )
                } else {
                    rtn = (*codec).open.expect("non-null function pointer")(localName.as_mut_ptr())
                        as *mut libc::c_void
                }
                break;
            } else {
                codec = (*codec).next
            }
        }
        // A loader was found
        if !codec.is_null() {
            if rtn.is_null() {
                // Loader failed, most likely because the file isn't there;
                // try again without the extension
                orgNameFailed = crate::src::qcommon::q_shared::qtrue;
                orgCodec = codec;
                crate::src::qcommon::q_shared::COM_StripExtension(
                    filename,
                    localName.as_mut_ptr(),
                    64 as i32,
                );
            } else {
                // Something loaded
                return rtn;
            }
        }
    }
    // Try and find a suitable match using all
    // the sound codecs supported
    codec = codecs;
    while !codec.is_null() {
        if !(codec == orgCodec) {
            crate::src::qcommon::q_shared::Com_sprintf(
                altName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                b"%s.%s\x00" as *const u8 as *const libc::c_char,
                localName.as_mut_ptr(),
                (*codec).ext,
            );
            // Load
            if !info.is_null() {
                rtn = (*codec).load.expect("non-null function pointer")(altName.as_mut_ptr(), info)
            } else {
                rtn = (*codec).open.expect("non-null function pointer")(altName.as_mut_ptr())
                    as *mut libc::c_void
            }
            if !rtn.is_null() {
                if orgNameFailed as u64 != 0 {
                    crate::src::qcommon::common::Com_DPrintf(
                        b"^3WARNING: %s not present, using %s instead\n\x00" as *const u8
                            as *const libc::c_char,
                        filename,
                        altName.as_mut_ptr(),
                    );
                }
                return rtn;
            }
        }
        codec = (*codec).next
    }
    crate::src::qcommon::common::Com_Printf(
        b"^3WARNING: Failed to %s sound %s!\n\x00" as *const u8 as *const libc::c_char,
        if !info.is_null() {
            b"load\x00" as *const u8 as *const libc::c_char
        } else {
            b"open\x00" as *const u8 as *const libc::c_char
        },
        filename,
    );
    return 0 as *mut libc::c_void;
}
// Codec management
/*
=================
S_CodecInit
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_CodecInit() {
    codecs = 0 as *mut crate::src::client::snd_codec::snd_codec_t;
    S_CodecRegister(&mut crate::src::client::snd_codec_opus::opus_codec);
    S_CodecRegister(&mut crate::src::client::snd_codec_ogg::ogg_codec);
    // Register wav codec last so that it is always tried first when a file extension was not found
    S_CodecRegister(&mut crate::src::client::snd_codec_wav::wav_codec);
}
/*
=================
S_CodecShutdown
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_CodecShutdown() {
    codecs = 0 as *mut crate::src::client::snd_codec::snd_codec_t;
}
/*
=================
S_CodecRegister
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_CodecRegister(
    mut codec: *mut crate::src::client::snd_codec::snd_codec_t,
) {
    (*codec).next = codecs;
    codecs = codec;
}
/*
=================
S_CodecLoad
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_CodecLoad(
    mut filename: *const libc::c_char,
    mut info: *mut crate::src::client::snd_codec::snd_info_t,
) -> *mut libc::c_void {
    return S_CodecGetSound(filename, info);
}
/*
=================
S_CodecOpenStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_CodecOpenStream(
    mut filename: *const libc::c_char,
) -> *mut crate::src::client::snd_codec::snd_stream_t {
    return S_CodecGetSound(
        filename,
        0 as *mut crate::src::client::snd_codec::snd_info_t,
    ) as *mut crate::src::client::snd_codec::snd_stream_t;
}
#[no_mangle]

pub unsafe extern "C" fn S_CodecCloseStream(
    mut stream: *mut crate::src::client::snd_codec::snd_stream_t,
) {
    (*(*stream).codec).close.expect("non-null function pointer")(stream);
}
#[no_mangle]

pub unsafe extern "C" fn S_CodecReadStream(
    mut stream: *mut crate::src::client::snd_codec::snd_stream_t,
    mut bytes: i32,
    mut buffer: *mut libc::c_void,
) -> i32 {
    return (*(*stream).codec).read.expect("non-null function pointer")(stream, bytes, buffer);
}
// Util functions (used by codecs)
//=======================================================================
// Util functions (used by codecs)
/*
=================
S_CodecUtilOpen
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_CodecUtilOpen(
    mut filename: *const libc::c_char,
    mut codec: *mut crate::src::client::snd_codec::snd_codec_t,
) -> *mut crate::src::client::snd_codec::snd_stream_t {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    let mut hnd: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut length: i32 = 0;
    // Try to open the file
    length = crate::src::qcommon::files::FS_FOpenFileRead(
        filename,
        &mut hnd,
        crate::src::qcommon::q_shared::qtrue,
    ) as i32;
    if hnd == 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"Can\'t read sound file %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // Allocate a stream
    stream = crate::src::qcommon::common::Z_Malloc(::std::mem::size_of::<
        crate::src::client::snd_codec::snd_stream_t,
    >() as libc::c_ulong as i32) as *mut crate::src::client::snd_codec::snd_stream_t;
    if stream.is_null() {
        crate::src::qcommon::files::FS_FCloseFile(hnd);
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // Copy over, return
    (*stream).codec = codec;
    (*stream).file = hnd;
    (*stream).length = length;
    return stream;
}
/*
=================
S_CodecUtilClose
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_CodecUtilClose(
    mut stream: *mut *mut crate::src::client::snd_codec::snd_stream_t,
) {
    crate::src::qcommon::files::FS_FCloseFile((**stream).file);
    crate::src::qcommon::common::Z_Free(*stream as *mut libc::c_void);
    *stream = 0 as *mut crate::src::client::snd_codec::snd_stream_t;
}

use ::libc;

pub use crate::stddef_h::wchar_t;
pub use crate::stdlib::C2RustUnnamed_18;
pub use crate::stdlib::__mbstate_t;

pub use crate::al_h::ALboolean;
pub use crate::al_h::ALchar;
pub use crate::al_h::ALdouble;
pub use crate::al_h::ALenum;
pub use crate::al_h::ALfloat;
pub use crate::al_h::ALint;
pub use crate::al_h::ALsizei;
pub use crate::al_h::ALuint;
pub use crate::al_h::ALvoid;
pub use crate::al_h::LPALBUFFERDATA;
pub use crate::al_h::LPALDELETEBUFFERS;
pub use crate::al_h::LPALDELETESOURCES;
pub use crate::al_h::LPALDISABLE;
pub use crate::al_h::LPALDISTANCEMODEL;
pub use crate::al_h::LPALDOPPLERFACTOR;
pub use crate::al_h::LPALENABLE;
pub use crate::al_h::LPALGENBUFFERS;
pub use crate::al_h::LPALGENSOURCES;
pub use crate::al_h::LPALGETBOOLEAN;
pub use crate::al_h::LPALGETBOOLEANV;
pub use crate::al_h::LPALGETBUFFERF;
pub use crate::al_h::LPALGETBUFFERI;
pub use crate::al_h::LPALGETDOUBLE;
pub use crate::al_h::LPALGETDOUBLEV;
pub use crate::al_h::LPALGETENUMVALUE;
pub use crate::al_h::LPALGETERROR;
pub use crate::al_h::LPALGETFLOAT;
pub use crate::al_h::LPALGETFLOATV;
pub use crate::al_h::LPALGETINTEGER;
pub use crate::al_h::LPALGETINTEGERV;
pub use crate::al_h::LPALGETLISTENER3F;
pub use crate::al_h::LPALGETLISTENERF;
pub use crate::al_h::LPALGETLISTENERFV;
pub use crate::al_h::LPALGETLISTENERI;
pub use crate::al_h::LPALGETPROCADDRESS;
pub use crate::al_h::LPALGETSOURCE3F;
pub use crate::al_h::LPALGETSOURCEF;
pub use crate::al_h::LPALGETSOURCEFV;
pub use crate::al_h::LPALGETSOURCEI;
pub use crate::al_h::LPALGETSTRING;
pub use crate::al_h::LPALISBUFFER;
pub use crate::al_h::LPALISENABLED;
pub use crate::al_h::LPALISEXTENSIONPRESENT;
pub use crate::al_h::LPALISSOURCE;
pub use crate::al_h::LPALLISTENER3F;
pub use crate::al_h::LPALLISTENERF;
pub use crate::al_h::LPALLISTENERFV;
pub use crate::al_h::LPALLISTENERI;
pub use crate::al_h::LPALSOURCE3F;
pub use crate::al_h::LPALSOURCEF;
pub use crate::al_h::LPALSOURCEFV;
pub use crate::al_h::LPALSOURCEI;
pub use crate::al_h::LPALSOURCEPAUSE;
pub use crate::al_h::LPALSOURCEPAUSEV;
pub use crate::al_h::LPALSOURCEPLAY;
pub use crate::al_h::LPALSOURCEPLAYV;
pub use crate::al_h::LPALSOURCEQUEUEBUFFERS;
pub use crate::al_h::LPALSOURCEREWIND;
pub use crate::al_h::LPALSOURCEREWINDV;
pub use crate::al_h::LPALSOURCESTOP;
pub use crate::al_h::LPALSOURCESTOPV;
pub use crate::al_h::LPALSOURCEUNQUEUEBUFFERS;
pub use crate::al_h::LPALSPEEDOFSOUND;
pub use crate::alc_h::ALCboolean;
pub use crate::alc_h::ALCchar;
pub use crate::alc_h::ALCcontext;
pub use crate::alc_h::ALCcontext_struct;
pub use crate::alc_h::ALCdevice;
pub use crate::alc_h::ALCdevice_struct;
pub use crate::alc_h::ALCenum;
pub use crate::alc_h::ALCint;
pub use crate::alc_h::ALCsizei;
pub use crate::alc_h::ALCuint;
pub use crate::alc_h::ALCvoid;
pub use crate::alc_h::LPALCCAPTURECLOSEDEVICE;
pub use crate::alc_h::LPALCCAPTUREOPENDEVICE;
pub use crate::alc_h::LPALCCAPTURESAMPLES;
pub use crate::alc_h::LPALCCAPTURESTART;
pub use crate::alc_h::LPALCCAPTURESTOP;
pub use crate::alc_h::LPALCCLOSEDEVICE;
pub use crate::alc_h::LPALCCREATECONTEXT;
pub use crate::alc_h::LPALCDESTROYCONTEXT;
pub use crate::alc_h::LPALCGETCONTEXTSDEVICE;
pub use crate::alc_h::LPALCGETCURRENTCONTEXT;
pub use crate::alc_h::LPALCGETENUMVALUE;
pub use crate::alc_h::LPALCGETERROR;
pub use crate::alc_h::LPALCGETINTEGERV;
pub use crate::alc_h::LPALCGETPROCADDRESS;
pub use crate::alc_h::LPALCGETSTRING;
pub use crate::alc_h::LPALCISEXTENSIONPRESENT;
pub use crate::alc_h::LPALCMAKECONTEXTCURRENT;
pub use crate::alc_h::LPALCOPENDEVICE;
pub use crate::alc_h::LPALCPROCESSCONTEXT;
pub use crate::alc_h::LPALCSUSPENDCONTEXT;

pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;

pub use crate::stdlib::mbstate_t;

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
// Dynamically loads OpenAL
#[no_mangle]

pub static mut qalEnable: crate::al_h::LPALENABLE = None;
#[no_mangle]

pub static mut qalDisable: crate::al_h::LPALDISABLE = None;
#[no_mangle]

pub static mut qalIsEnabled: crate::al_h::LPALISENABLED = None;
#[no_mangle]

pub static mut qalGetString: crate::al_h::LPALGETSTRING = None;
#[no_mangle]

pub static mut qalGetBooleanv: crate::al_h::LPALGETBOOLEANV = None;
#[no_mangle]

pub static mut qalGetIntegerv: crate::al_h::LPALGETINTEGERV = None;
#[no_mangle]

pub static mut qalGetFloatv: crate::al_h::LPALGETFLOATV = None;
#[no_mangle]

pub static mut qalGetDoublev: crate::al_h::LPALGETDOUBLEV = None;
#[no_mangle]

pub static mut qalGetBoolean: crate::al_h::LPALGETBOOLEAN = None;
#[no_mangle]

pub static mut qalGetInteger: crate::al_h::LPALGETINTEGER = None;
#[no_mangle]

pub static mut qalGetFloat: crate::al_h::LPALGETFLOAT = None;
#[no_mangle]

pub static mut qalGetDouble: crate::al_h::LPALGETDOUBLE = None;
#[no_mangle]

pub static mut qalGetError: crate::al_h::LPALGETERROR = None;
#[no_mangle]

pub static mut qalIsExtensionPresent: crate::al_h::LPALISEXTENSIONPRESENT = None;
#[no_mangle]

pub static mut qalGetProcAddress: crate::al_h::LPALGETPROCADDRESS = None;
#[no_mangle]

pub static mut qalGetEnumValue: crate::al_h::LPALGETENUMVALUE = None;
#[no_mangle]

pub static mut qalListenerf: crate::al_h::LPALLISTENERF = None;
#[no_mangle]

pub static mut qalListener3f: crate::al_h::LPALLISTENER3F = None;
#[no_mangle]

pub static mut qalListenerfv: crate::al_h::LPALLISTENERFV = None;
#[no_mangle]

pub static mut qalListeneri: crate::al_h::LPALLISTENERI = None;
#[no_mangle]

pub static mut qalGetListenerf: crate::al_h::LPALGETLISTENERF = None;
#[no_mangle]

pub static mut qalGetListener3f: crate::al_h::LPALGETLISTENER3F = None;
#[no_mangle]

pub static mut qalGetListenerfv: crate::al_h::LPALGETLISTENERFV = None;
#[no_mangle]

pub static mut qalGetListeneri: crate::al_h::LPALGETLISTENERI = None;
#[no_mangle]

pub static mut qalGenSources: crate::al_h::LPALGENSOURCES = None;
#[no_mangle]

pub static mut qalDeleteSources: crate::al_h::LPALDELETESOURCES = None;
#[no_mangle]

pub static mut qalIsSource: crate::al_h::LPALISSOURCE = None;
#[no_mangle]

pub static mut qalSourcef: crate::al_h::LPALSOURCEF = None;
#[no_mangle]

pub static mut qalSource3f: crate::al_h::LPALSOURCE3F = None;
#[no_mangle]

pub static mut qalSourcefv: crate::al_h::LPALSOURCEFV = None;
#[no_mangle]

pub static mut qalSourcei: crate::al_h::LPALSOURCEI = None;
#[no_mangle]

pub static mut qalGetSourcef: crate::al_h::LPALGETSOURCEF = None;
#[no_mangle]

pub static mut qalGetSource3f: crate::al_h::LPALGETSOURCE3F = None;
#[no_mangle]

pub static mut qalGetSourcefv: crate::al_h::LPALGETSOURCEFV = None;
#[no_mangle]

pub static mut qalGetSourcei: crate::al_h::LPALGETSOURCEI = None;
#[no_mangle]

pub static mut qalSourcePlayv: crate::al_h::LPALSOURCEPLAYV = None;
#[no_mangle]

pub static mut qalSourceStopv: crate::al_h::LPALSOURCESTOPV = None;
#[no_mangle]

pub static mut qalSourceRewindv: crate::al_h::LPALSOURCEREWINDV = None;
#[no_mangle]

pub static mut qalSourcePausev: crate::al_h::LPALSOURCEPAUSEV = None;
#[no_mangle]

pub static mut qalSourcePlay: crate::al_h::LPALSOURCEPLAY = None;
#[no_mangle]

pub static mut qalSourceStop: crate::al_h::LPALSOURCESTOP = None;
#[no_mangle]

pub static mut qalSourceRewind: crate::al_h::LPALSOURCEREWIND = None;
#[no_mangle]

pub static mut qalSourcePause: crate::al_h::LPALSOURCEPAUSE = None;
#[no_mangle]

pub static mut qalSourceQueueBuffers: crate::al_h::LPALSOURCEQUEUEBUFFERS = None;
#[no_mangle]

pub static mut qalSourceUnqueueBuffers: crate::al_h::LPALSOURCEUNQUEUEBUFFERS = None;
#[no_mangle]

pub static mut qalGenBuffers: crate::al_h::LPALGENBUFFERS = None;
#[no_mangle]

pub static mut qalDeleteBuffers: crate::al_h::LPALDELETEBUFFERS = None;
#[no_mangle]

pub static mut qalIsBuffer: crate::al_h::LPALISBUFFER = None;
#[no_mangle]

pub static mut qalBufferData: crate::al_h::LPALBUFFERDATA = None;
#[no_mangle]

pub static mut qalGetBufferf: crate::al_h::LPALGETBUFFERF = None;
#[no_mangle]

pub static mut qalGetBufferi: crate::al_h::LPALGETBUFFERI = None;
#[no_mangle]

pub static mut qalDopplerFactor: crate::al_h::LPALDOPPLERFACTOR = None;
#[no_mangle]

pub static mut qalSpeedOfSound: crate::al_h::LPALSPEEDOFSOUND = None;
#[no_mangle]

pub static mut qalDistanceModel: crate::al_h::LPALDISTANCEMODEL = None;
#[no_mangle]

pub static mut qalcCreateContext: crate::alc_h::LPALCCREATECONTEXT = None;
#[no_mangle]

pub static mut qalcMakeContextCurrent: crate::alc_h::LPALCMAKECONTEXTCURRENT = None;
#[no_mangle]

pub static mut qalcProcessContext: crate::alc_h::LPALCPROCESSCONTEXT = None;
#[no_mangle]

pub static mut qalcSuspendContext: crate::alc_h::LPALCSUSPENDCONTEXT = None;
#[no_mangle]

pub static mut qalcDestroyContext: crate::alc_h::LPALCDESTROYCONTEXT = None;
#[no_mangle]

pub static mut qalcGetCurrentContext: crate::alc_h::LPALCGETCURRENTCONTEXT = None;
#[no_mangle]

pub static mut qalcGetContextsDevice: crate::alc_h::LPALCGETCONTEXTSDEVICE = None;
#[no_mangle]

pub static mut qalcOpenDevice: crate::alc_h::LPALCOPENDEVICE = None;
#[no_mangle]

pub static mut qalcCloseDevice: crate::alc_h::LPALCCLOSEDEVICE = None;
#[no_mangle]

pub static mut qalcGetError: crate::alc_h::LPALCGETERROR = None;
#[no_mangle]

pub static mut qalcIsExtensionPresent: crate::alc_h::LPALCISEXTENSIONPRESENT = None;
#[no_mangle]

pub static mut qalcGetProcAddress: crate::alc_h::LPALCGETPROCADDRESS = None;
#[no_mangle]

pub static mut qalcGetEnumValue: crate::alc_h::LPALCGETENUMVALUE = None;
#[no_mangle]

pub static mut qalcGetString: crate::alc_h::LPALCGETSTRING = None;
#[no_mangle]

pub static mut qalcGetIntegerv: crate::alc_h::LPALCGETINTEGERV = None;
#[no_mangle]

pub static mut qalcCaptureOpenDevice: crate::alc_h::LPALCCAPTUREOPENDEVICE = None;
#[no_mangle]

pub static mut qalcCaptureCloseDevice: crate::alc_h::LPALCCAPTURECLOSEDEVICE = None;
#[no_mangle]

pub static mut qalcCaptureStart: crate::alc_h::LPALCCAPTURESTART = None;
#[no_mangle]

pub static mut qalcCaptureStop: crate::alc_h::LPALCCAPTURESTOP = None;
#[no_mangle]

pub static mut qalcCaptureSamples: crate::alc_h::LPALCCAPTURESAMPLES = None;

static mut OpenALLib: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;

static mut alinit_fail: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
/*
=================
GPA
=================
*/

unsafe extern "C" fn GPA(mut str: *mut libc::c_char) -> *mut libc::c_void {
    let mut rv: *mut libc::c_void = 0 as *mut libc::c_void;
    rv = crate::stdlib::SDL_LoadFunction(OpenALLib, str);
    if rv.is_null() {
        crate::src::qcommon::common::Com_Printf(
            b" Can\'t load symbol %s\n\x00" as *const u8 as *const libc::c_char,
            str,
        );
        alinit_fail = crate::src::qcommon::q_shared::qtrue;
        return 0 as *mut libc::c_void;
    } else {
        crate::src::qcommon::common::Com_DPrintf(
            b" Loaded symbol %s (%p)\n\x00" as *const u8 as *const libc::c_char,
            str,
            rv,
        );
        return rv;
    };
}
/*
=================
QAL_Init
=================
*/
#[no_mangle]

pub unsafe extern "C" fn QAL_Init(
    mut libname: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    if !OpenALLib.is_null() {
        return crate::src::qcommon::q_shared::qtrue;
    }
    OpenALLib =
        crate::src::sys::sys_main::Sys_LoadDll(libname, crate::src::qcommon::q_shared::qtrue);
    if OpenALLib.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    alinit_fail = crate::src::qcommon::q_shared::qfalse;
    qalEnable =
        ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALENABLE>(GPA(b"alEnable\x00"
            as *const u8
            as *const libc::c_char
            as *mut libc::c_char));
    qalDisable =
        ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALDISABLE>(GPA(b"alDisable\x00"
            as *const u8
            as *const libc::c_char
            as *mut libc::c_char));
    qalIsEnabled = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALISENABLED>(GPA(
        b"alIsEnabled\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetString = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETSTRING>(GPA(
        b"alGetString\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetBooleanv = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETBOOLEANV>(GPA(
        b"alGetBooleanv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetIntegerv = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETINTEGERV>(GPA(
        b"alGetIntegerv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetFloatv = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETFLOATV>(GPA(
        b"alGetFloatv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetDoublev = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETDOUBLEV>(GPA(
        b"alGetDoublev\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetBoolean = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETBOOLEAN>(GPA(
        b"alGetBoolean\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetInteger = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETINTEGER>(GPA(
        b"alGetInteger\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetFloat = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETFLOAT>(GPA(
        b"alGetFloat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetDouble = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETDOUBLE>(GPA(
        b"alGetDouble\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetError = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETERROR>(GPA(
        b"alGetError\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalIsExtensionPresent =
        ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALISEXTENSIONPRESENT>(GPA(
            b"alIsExtensionPresent\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalGetProcAddress = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETPROCADDRESS>(
        GPA(b"alGetProcAddress\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalGetEnumValue = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETENUMVALUE>(
        GPA(b"alGetEnumValue\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalListenerf = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALLISTENERF>(GPA(
        b"alListenerf\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalListener3f = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALLISTENER3F>(GPA(
        b"alListener3f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalListenerfv = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALLISTENERFV>(GPA(
        b"alListenerfv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalListeneri = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALLISTENERI>(GPA(
        b"alListeneri\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetListenerf = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETLISTENERF>(
        GPA(b"alGetListenerf\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalGetListener3f = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETLISTENER3F>(
        GPA(b"alGetListener3f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalGetListenerfv = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETLISTENERFV>(
        GPA(b"alGetListenerfv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalGetListeneri = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETLISTENERI>(
        GPA(b"alGetListeneri\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalGenSources = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGENSOURCES>(GPA(
        b"alGenSources\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalDeleteSources = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALDELETESOURCES>(
        GPA(b"alDeleteSources\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalIsSource = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALISSOURCE>(GPA(
        b"alIsSource\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalSourcef =
        ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCEF>(GPA(b"alSourcef\x00"
            as *const u8
            as *const libc::c_char
            as *mut libc::c_char));
    qalSource3f = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCE3F>(GPA(
        b"alSource3f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalSourcefv = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCEFV>(GPA(
        b"alSourcefv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalSourcei =
        ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCEI>(GPA(b"alSourcei\x00"
            as *const u8
            as *const libc::c_char
            as *mut libc::c_char));
    qalGetSourcef = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETSOURCEF>(GPA(
        b"alGetSourcef\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetSource3f = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETSOURCE3F>(GPA(
        b"alGetSource3f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetSourcefv = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETSOURCEFV>(GPA(
        b"alGetSourcefv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetSourcei = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETSOURCEI>(GPA(
        b"alGetSourcei\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalSourcePlayv = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCEPLAYV>(GPA(
        b"alSourcePlayv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalSourceStopv = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCESTOPV>(GPA(
        b"alSourceStopv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalSourceRewindv = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCEREWINDV>(
        GPA(b"alSourceRewindv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalSourcePausev = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCEPAUSEV>(
        GPA(b"alSourcePausev\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalSourcePlay = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCEPLAY>(GPA(
        b"alSourcePlay\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalSourceStop = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCESTOP>(GPA(
        b"alSourceStop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalSourceRewind = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCEREWIND>(
        GPA(b"alSourceRewind\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalSourcePause = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCEPAUSE>(GPA(
        b"alSourcePause\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalSourceQueueBuffers =
        ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCEQUEUEBUFFERS>(GPA(
            b"alSourceQueueBuffers\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalSourceUnqueueBuffers =
        ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSOURCEUNQUEUEBUFFERS>(GPA(
            b"alSourceUnqueueBuffers\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalGenBuffers = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGENBUFFERS>(GPA(
        b"alGenBuffers\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalDeleteBuffers = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALDELETEBUFFERS>(
        GPA(b"alDeleteBuffers\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalIsBuffer = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALISBUFFER>(GPA(
        b"alIsBuffer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalBufferData = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALBUFFERDATA>(GPA(
        b"alBufferData\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetBufferf = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETBUFFERF>(GPA(
        b"alGetBufferf\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalGetBufferi = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALGETBUFFERI>(GPA(
        b"alGetBufferi\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalDopplerFactor = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALDOPPLERFACTOR>(
        GPA(b"alDopplerFactor\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalSpeedOfSound = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALSPEEDOFSOUND>(
        GPA(b"alSpeedOfSound\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalDistanceModel = ::std::mem::transmute::<*mut libc::c_void, crate::al_h::LPALDISTANCEMODEL>(
        GPA(b"alDistanceModel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalcCreateContext = ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCCREATECONTEXT>(
        GPA(b"alcCreateContext\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalcMakeContextCurrent =
        ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCMAKECONTEXTCURRENT>(GPA(
            b"alcMakeContextCurrent\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalcProcessContext =
        ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCPROCESSCONTEXT>(GPA(
            b"alcProcessContext\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalcSuspendContext =
        ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCSUSPENDCONTEXT>(GPA(
            b"alcSuspendContext\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalcDestroyContext =
        ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCDESTROYCONTEXT>(GPA(
            b"alcDestroyContext\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalcGetCurrentContext =
        ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCGETCURRENTCONTEXT>(GPA(
            b"alcGetCurrentContext\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalcGetContextsDevice =
        ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCGETCONTEXTSDEVICE>(GPA(
            b"alcGetContextsDevice\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalcOpenDevice = ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCOPENDEVICE>(
        GPA(b"alcOpenDevice\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalcCloseDevice = ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCCLOSEDEVICE>(
        GPA(b"alcCloseDevice\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalcGetError = ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCGETERROR>(GPA(
        b"alcGetError\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalcIsExtensionPresent =
        ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCISEXTENSIONPRESENT>(GPA(
            b"alcIsExtensionPresent\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalcGetProcAddress =
        ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCGETPROCADDRESS>(GPA(
            b"alcGetProcAddress\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalcGetEnumValue = ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCGETENUMVALUE>(
        GPA(b"alcGetEnumValue\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalcGetString = ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCGETSTRING>(GPA(
        b"alcGetString\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qalcGetIntegerv = ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCGETINTEGERV>(
        GPA(b"alcGetIntegerv\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalcCaptureOpenDevice =
        ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCCAPTUREOPENDEVICE>(GPA(
            b"alcCaptureOpenDevice\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalcCaptureCloseDevice =
        ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCCAPTURECLOSEDEVICE>(GPA(
            b"alcCaptureCloseDevice\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    qalcCaptureStart = ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCCAPTURESTART>(
        GPA(b"alcCaptureStart\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalcCaptureStop = ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCCAPTURESTOP>(
        GPA(b"alcCaptureStop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    qalcCaptureSamples =
        ::std::mem::transmute::<*mut libc::c_void, crate::alc_h::LPALCCAPTURESAMPLES>(GPA(
            b"alcCaptureSamples\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ));
    if alinit_fail as u64 != 0 {
        QAL_Shutdown();
        crate::src::qcommon::common::Com_Printf(
            b" One or more symbols not found\n\x00" as *const u8 as *const libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=================
QAL_Shutdown
=================
*/
#[no_mangle]

pub unsafe extern "C" fn QAL_Shutdown() {
    if !OpenALLib.is_null() {
        crate::stdlib::SDL_UnloadObject(OpenALLib);
        OpenALLib = 0 as *mut libc::c_void
    }
    qalEnable = None;
    qalDisable = None;
    qalIsEnabled = None;
    qalGetString = None;
    qalGetBooleanv = None;
    qalGetIntegerv = None;
    qalGetFloatv = None;
    qalGetDoublev = None;
    qalGetBoolean = None;
    qalGetInteger = None;
    qalGetFloat = None;
    qalGetDouble = None;
    qalGetError = None;
    qalIsExtensionPresent = None;
    qalGetProcAddress = None;
    qalGetEnumValue = None;
    qalListenerf = None;
    qalListener3f = None;
    qalListenerfv = None;
    qalListeneri = None;
    qalGetListenerf = None;
    qalGetListener3f = None;
    qalGetListenerfv = None;
    qalGetListeneri = None;
    qalGenSources = None;
    qalDeleteSources = None;
    qalIsSource = None;
    qalSourcef = None;
    qalSource3f = None;
    qalSourcefv = None;
    qalSourcei = None;
    qalGetSourcef = None;
    qalGetSource3f = None;
    qalGetSourcefv = None;
    qalGetSourcei = None;
    qalSourcePlayv = None;
    qalSourceStopv = None;
    qalSourceRewindv = None;
    qalSourcePausev = None;
    qalSourcePlay = None;
    qalSourceStop = None;
    qalSourceRewind = None;
    qalSourcePause = None;
    qalSourceQueueBuffers = None;
    qalSourceUnqueueBuffers = None;
    qalGenBuffers = None;
    qalDeleteBuffers = None;
    qalIsBuffer = None;
    qalBufferData = None;
    qalGetBufferf = None;
    qalGetBufferi = None;
    qalDopplerFactor = None;
    qalSpeedOfSound = None;
    qalDistanceModel = None;
    qalcCreateContext = None;
    qalcMakeContextCurrent = None;
    qalcProcessContext = None;
    qalcSuspendContext = None;
    qalcDestroyContext = None;
    qalcGetCurrentContext = None;
    qalcGetContextsDevice = None;
    qalcOpenDevice = None;
    qalcCloseDevice = None;
    qalcGetError = None;
    qalcIsExtensionPresent = None;
    qalcGetProcAddress = None;
    qalcGetEnumValue = None;
    qalcGetString = None;
    qalcGetIntegerv = None;
    qalcCaptureOpenDevice = None;
    qalcCaptureCloseDevice = None;
    qalcCaptureStart = None;
    qalcCaptureStop = None;
    qalcCaptureSamples = None;
}

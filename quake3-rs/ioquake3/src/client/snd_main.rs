use ::libc;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::qcommon_h::xcommand_t;
pub use crate::snd_local_h::soundInterface_t;

pub use crate::src::client::snd_dma::S_Base_Init;
pub use crate::src::client::snd_openal::S_AL_Init;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::cmd::Cmd_RemoveCommand;
pub use crate::src::qcommon::common::com_minimized;
pub use crate::src::qcommon::common::com_unfocused;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;

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
#[no_mangle]

pub static mut s_volume: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut s_muted: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut s_musicVolume: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut s_doppler: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut s_backend: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut s_muteWhenMinimized: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut s_muteWhenUnfocused: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut si: crate::snd_local_h::soundInterface_t = crate::snd_local_h::soundInterface_t {
    Shutdown: None,
    StartSound: None,
    StartLocalSound: None,
    StartBackgroundTrack: None,
    StopBackgroundTrack: None,
    RawSamples: None,
    StopAllSounds: None,
    ClearLoopingSounds: None,
    AddLoopingSound: None,
    AddRealLoopingSound: None,
    StopLoopingSound: None,
    Respatialize: None,
    UpdateEntityPosition: None,
    Update: None,
    DisableSounds: None,
    BeginRegistration: None,
    RegisterSound: None,
    ClearSoundBuffer: None,
    SoundInfo: None,
    SoundList: None,
    StartCapture: None,
    AvailableCaptureSamples: None,
    Capture: None,
    StopCapture: None,
    MasterGain: None,
};
/*
=================
S_ValidateInterface
=================
*/

unsafe extern "C" fn S_ValidSoundInterface(
    mut si_0: *mut crate::snd_local_h::soundInterface_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*si_0).Shutdown.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).StartSound.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).StartLocalSound.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).StartBackgroundTrack.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).StopBackgroundTrack.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).RawSamples.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).StopAllSounds.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).ClearLoopingSounds.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).AddLoopingSound.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).AddRealLoopingSound.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).StopLoopingSound.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).Respatialize.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).UpdateEntityPosition.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).Update.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).DisableSounds.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).BeginRegistration.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).RegisterSound.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).ClearSoundBuffer.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).SoundInfo.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).SoundList.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).StartCapture.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).AvailableCaptureSamples.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).Capture.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).StopCapture.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*si_0).MasterGain.is_none() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=================
S_StartSound
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_StartSound(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut entnum: libc::c_int,
    mut entchannel: libc::c_int,
    mut sfx: crate::src::qcommon::q_shared::sfxHandle_t,
) {
    if si.StartSound.is_some() {
        si.StartSound.expect("non-null function pointer")(origin, entnum, entchannel, sfx);
    };
}
/*
=================
S_StartLocalSound
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_StartLocalSound(
    mut sfx: crate::src::qcommon::q_shared::sfxHandle_t,
    mut channelNum: libc::c_int,
) {
    if si.StartLocalSound.is_some() {
        si.StartLocalSound.expect("non-null function pointer")(sfx, channelNum);
    };
}
/*
=================
S_StartBackgroundTrack
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_StartBackgroundTrack(
    mut intro: *const libc::c_char,
    mut loop_0: *const libc::c_char,
) {
    if si.StartBackgroundTrack.is_some() {
        si.StartBackgroundTrack.expect("non-null function pointer")(intro, loop_0);
    };
}
/*
=================
S_StopBackgroundTrack
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_StopBackgroundTrack() {
    if si.StopBackgroundTrack.is_some() {
        si.StopBackgroundTrack.expect("non-null function pointer")();
    };
}
/*
=================
S_RawSamples
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_RawSamples(
    mut stream: libc::c_int,
    mut samples: libc::c_int,
    mut rate: libc::c_int,
    mut width: libc::c_int,
    mut channels: libc::c_int,
    mut data: *const crate::src::qcommon::q_shared::byte,
    mut volume: libc::c_float,
    mut entityNum: libc::c_int,
) {
    if si.RawSamples.is_some() {
        si.RawSamples.expect("non-null function pointer")(
            stream, samples, rate, width, channels, data, volume, entityNum,
        );
    };
}
/*
=================
S_StopAllSounds
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_StopAllSounds() {
    if si.StopAllSounds.is_some() {
        si.StopAllSounds.expect("non-null function pointer")();
    };
}
/*
=================
S_ClearLoopingSounds
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_ClearLoopingSounds(
    mut killall: crate::src::qcommon::q_shared::qboolean,
) {
    if si.ClearLoopingSounds.is_some() {
        si.ClearLoopingSounds.expect("non-null function pointer")(killall);
    };
}
/*
=================
S_AddLoopingSound
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_AddLoopingSound(
    mut entityNum: libc::c_int,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut velocity: *const crate::src::qcommon::q_shared::vec_t,
    mut sfx: crate::src::qcommon::q_shared::sfxHandle_t,
) {
    if si.AddLoopingSound.is_some() {
        si.AddLoopingSound.expect("non-null function pointer")(entityNum, origin, velocity, sfx);
    };
}
/*
=================
S_AddRealLoopingSound
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_AddRealLoopingSound(
    mut entityNum: libc::c_int,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut velocity: *const crate::src::qcommon::q_shared::vec_t,
    mut sfx: crate::src::qcommon::q_shared::sfxHandle_t,
) {
    if si.AddRealLoopingSound.is_some() {
        si.AddRealLoopingSound.expect("non-null function pointer")(
            entityNum, origin, velocity, sfx,
        );
    };
}
/*
=================
S_StopLoopingSound
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_StopLoopingSound(mut entityNum: libc::c_int) {
    if si.StopLoopingSound.is_some() {
        si.StopLoopingSound.expect("non-null function pointer")(entityNum);
    };
}
/*
=================
S_Respatialize
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Respatialize(
    mut entityNum: libc::c_int,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut axis: *mut crate::src::qcommon::q_shared::vec3_t,
    mut inwater: libc::c_int,
) {
    if si.Respatialize.is_some() {
        si.Respatialize.expect("non-null function pointer")(entityNum, origin, axis, inwater);
    };
}
/*
=================
S_UpdateEntityPosition
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_UpdateEntityPosition(
    mut entityNum: libc::c_int,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
) {
    if si.UpdateEntityPosition.is_some() {
        si.UpdateEntityPosition.expect("non-null function pointer")(entityNum, origin);
    };
}
/*
=================
S_Update
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Update() {
    if (*s_muted).integer != 0 {
        if !((*s_muteWhenMinimized).integer != 0
            && (*crate::src::qcommon::common::com_minimized).integer != 0)
            && !((*s_muteWhenUnfocused).integer != 0
                && (*crate::src::qcommon::common::com_unfocused).integer != 0)
        {
            (*s_muted).integer = crate::src::qcommon::q_shared::qfalse as libc::c_int;
            (*s_muted).modified = crate::src::qcommon::q_shared::qtrue
        }
    } else if (*s_muteWhenMinimized).integer != 0
        && (*crate::src::qcommon::common::com_minimized).integer != 0
        || (*s_muteWhenUnfocused).integer != 0
            && (*crate::src::qcommon::common::com_unfocused).integer != 0
    {
        (*s_muted).integer = crate::src::qcommon::q_shared::qtrue as libc::c_int;
        (*s_muted).modified = crate::src::qcommon::q_shared::qtrue
    }
    if si.Update.is_some() {
        si.Update.expect("non-null function pointer")();
    };
}
/*
=================
S_DisableSounds
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_DisableSounds() {
    if si.DisableSounds.is_some() {
        si.DisableSounds.expect("non-null function pointer")();
    };
}
/*
=================
S_BeginRegistration
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_BeginRegistration() {
    if si.BeginRegistration.is_some() {
        si.BeginRegistration.expect("non-null function pointer")();
    };
}
/*
=================
S_RegisterSound
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_RegisterSound(
    mut sample: *const libc::c_char,
    mut compressed: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    if si.RegisterSound.is_some() {
        return si.RegisterSound.expect("non-null function pointer")(sample, compressed);
    } else {
        return 0 as libc::c_int;
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
// qcommon.h -- definitions common between client and server, but not game.or ref modules
//Ignore __attribute__ on non-gcc platforms
//#define	PRE_RELEASE_DEMO
//============================================================================
//
// msg.c
//
// if false, do a Com_Error
// set to true if the buffer size failed (with allowoverflow set)
// set to true if the buffer size failed (with allowoverflow set)
// for bitwise reads and writes
// TTimo
// copy a msg_t in case we need to store it as is for a bit
// (as I needed this to keep an msg_t from a static var for later use)
// sets data buffer as MSG_Init does prior to do the copy
//============================================================================
/*
==============================================================

NET

==============================================================
*/
// if this flag is set, always attempt ipv6 connections instead of ipv4 if a v6 address is found.
// disables ipv6 multicast support if set.
// number of old messages that must be kept on client and
// server for delta comrpession and ping estimation
// max number of usercmd_t in a packet
// max string commands buffered for restransmit
// an address lookup failed
// maximum length of an IPv6 address string including trailing '\0'
// Needed for IPv6 link-local addresses
// max length of a message, which may
// be fragmented into multiple packets
// ACK window of 48 download chunks. Cannot set this higher, or clients
// will overflow the reliable commands buffer
// 896 byte block chunks
/*
Netchan handles packet fragmentation and out of order / duplicate suppression
*/
// between last packet and previous
// qport value to write when transmitting
// sequencing variables
// incoming fragment assembly buffer
// outgoing fragment buffer
// we need to space out the sending of large fragmented messages
/*
==============================================================

PROTOCOL

==============================================================
*/
// 1.31 - 67
// maintain a list of compatible protocols for demo playing
// NOTE: that stuff only works with two digits protocols
// override on command line, config files etc.
// broadcast scan this many ports after
// PORT_SERVER so a single machine can
// run multiple servers
// the svc_strings[] array in cl_parse.c should mirror this
//
// server to client
//
// [short] [string] only in gamestate messages
// only in gamestate messages
// [string] to be executed by client game module
// [short] size [size bytes]
// new commands, supported only by ioquake3 protocol but not legacy
// not wrapped in USE_VOIP, so this value is reserved.
//
//
// client to server
//
// [[usercmd_t]
// [[usercmd_t]
// [string] message
// new commands, supported only by ioquake3 protocol but not legacy
// not wrapped in USE_VOIP, so this value is reserved.
//
/*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
// module should be bare: "cgame", not "cgame.dll" or "vm/cgame.qvm"
/*
==============================================================

CMD

Command text buffering and command execution

==============================================================
*/
/*

Any number of commands can be added in a frame, from several different sources.
Most commands come from either keybindings or console line input, but entire text
files can be execed.

*/
// allocates an initial text buffer that will grow as needed
// Adds command text at the end of the buffer, does NOT add a final \n
// this can be used in place of either Cbuf_AddText or Cbuf_InsertText
// Pulls off \n terminated lines of text from the command buffer and sends
// them through Cmd_ExecuteString.  Stops when the buffer is empty.
// Normally called once per frame, but may be explicitly invoked.
// Do not call inside a command function, or current args will be destroyed.
//===========================================================================
/*

Command execution takes a null terminated string, breaks it into tokens,
then searches for a command or variable that matches the first token.

*/
// called by the init functions of other parts of the program to
// register commands and functions to call for them.
// The cmd_name is referenced later, so it should not be in temp memory
// if function is NULL, the command will be forwarded to the server
// as a clc_clientCommand instead of executed locally
// don't allow VMs to remove system commands
// callback with each valid string
// The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
// Takes a null terminated string.  Does not need to be /n terminated.
// breaks the string up into arg tokens.
// Parses a single line of text into arguments and tries to execute it
// as if it was typed at the console
/*
==============================================================

CVAR

==============================================================
*/
/*

cvar_t variables are used to hold scalar or string variables that can be changed
or displayed at the console or prog code as well as accessed directly
in C code.

The user can access cvars from the console in three ways:
r_draworder			prints the current value
r_draworder 0		sets the current value to 0
set r_draworder 0	as above, but creates the cvar if not present

Cvars are restricted from having the same names as commands to keep this
interface from being ambiguous.

The are also occasionally used to communicated information between different
modules of the program.

*/
// creates the variable if it doesn't exist, or returns the existing one
// if it exists, the value will not be changed, but flags will be ORed in
// that allows variables to be unarchived without needing bitflags
// if value is "", the value will not override a previously set value.
// basically a slightly modified Cvar_Get for the interpreted modules
// updates an interpreted modules' version of a cvar
// will create the variable with no flags if it doesn't exist
// same as Cvar_Set, but allows more control over setting of cvar
// sometimes we set variables from an untrusted source: fail if flags & CVAR_PROTECTED
// don't set the cvar immediately
// expands value to a string and calls Cvar_Set/Cvar_SetSafe
// returns 0 if not defined or non numeric
// returns an empty string if not defined
// returns CVAR_NONEXISTENT if cvar doesn't exist or the flags of that particular CVAR.
// callback with each valid string
// reset all testing vars to a safe value
// called by Cmd_ExecuteString when Cmd_Argv(0) doesn't match a known
// command.  Returns true if the command was a variable reference that
// was handled. (print or change)
// writes lines containing "set variable value" for all variables
// with the archive flag set to true.
// returns an info string containing all the cvars that have the given bit set
// in their flags ( CVAR_USERINFO, CVAR_SERVERINFO, CVAR_SYSTEMINFO, etc )
// whenever a cvar is modifed, its flags will be OR'd into this, so
// a single check can determine if any CVAR_USERINFO, CVAR_SERVERINFO,
// etc, variables have been modified since the last check.  The bit
// can then be cleared to allow another change detection.
/*
==============================================================

FILESYSTEM

No stdio calls should be used by any part of the game, because
we need to deal with all sorts of directory and seperator char
issues.
==============================================================
*/
// referenced flags
// these are in loop specific order so don't change the order
// number of id paks that will never be autodownloaded from baseq3/missionpack
// shutdown and restart the filesystem so changes to fs_gamedir can take effect
// directory should not have either a leading or trailing /
// if extension is "/", only subdirectories will be returned
// the returned files will not include any directories or /
// will properly create any needed paths and deal with seperater character issues
// if uniqueFILE is true, then a new FILE will be fopened even if the file
// is found in an already open pak file.  If uniqueFILE is false, you must call
// FS_FCloseFile instead of fclose, otherwise the pak FILE would be improperly closed
// It is generally safe to always set uniqueFILE to true, because the majority of
// file IO goes through FS_ReadFile, which Does The Right Thing already.
// returns 1 if a file is in the PAK file, otherwise -1
// properly handles partial reads and reads from other dlls
// note: you can't just fclose from another DLL, due to MS libc issues
// returns the length of the file
// a null buffer will just return the file length without loading
// as a quick check for existence. -1 length == not present
// A 0 byte will always be appended at the end, so string ops are safe.
// the buffer should be considered read-only, because it may be cached
// for other uses.
// forces flush on files we're writing to.
// frees the memory returned by FS_ReadFile
// writes a complete file, creating any subdirectories needed
// doesn't work for files that are opened from a pack file
// where are we?
// like fprintf
// opens a file for reading, writing, or appending depending on the value of mode
// seek on a file
// Returns a space separated string containing the checksums of all loaded pk3 files.
// Servers with sv_pure set will get this string and pass it to clients.
// Returns a space separated string containing the checksums of all loaded
// AND referenced pk3 files. Servers with sv_pure set will get this string
// back from clients for pure validation
// clears referenced booleans on loaded pk3s
// If the string is empty, all data sources will be allowed.
// If not empty, only pk3 files that match one of the space
// separated checksums will be checked for files, with the
// sole exception of .cfg files.
/*
==============================================================

Edit fields and command line history/completion

==============================================================
*/
/*
==============================================================

MISC

==============================================================
*/
// centralizing the declarations for cl_cdkey
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=470
// returned by Sys_GetProcessorFeatures
// centralized and cleaned, that's the max string you can send to a Com_Printf / Com_DPrintf (above gets truncated)
// SE_NONE must be zero
// evTime is still valid
// evValue is a key code, evValue2 is the down flag
// evValue is an ascii char
// evValue and evValue2 are relative signed x / y moves
// evValue is an axis number and evValue2 is the current state (-127 to 127)
// evPtr is a char*
// bytes of data pointed to by evPtr, for journaling
// this must be manually freed if not NULL
// will be journaled properly
// checks for and removes command line "+set var arg" constructs
// if match is NULL, all set commands will be executed, otherwise
// only a set with the exact name.  Only used during startup.
// for building release pak files
// both client and server must agree to pause
// com_speeds times
// renderer backend time
/*

--- low memory ----
server vm
server clipmap
---mark---
renderer initialization (shaders, etc)
UI vm
cgame vm
renderer map
renderer models

---free---

temp file loading
--- high memory ---

*/
// NOT 0 filled memory
// returns 0 filled memory
// NOT 0 filled memory only for small allocations
// commandLine should not include the executable name (argv[0])
/*
==============================================================

CLIENT / SERVER SYSTEMS

==============================================================
*/
//
// client interface
//
// the keyboard binding interface must be setup before execing
// config files, but the rest of client startup will happen later
// char events are for field typing, not game control
// do a screen update before starting to load a map
// when the server is going to load a new map, the entire hunk
// will be cleared, so the client must shutdown cgame, ui, and
// the renderer
// adds the current command line as a clc_clientCommand to the client message.
// things like godmode, noclip, etc, are commands directed to the server,
// so when they are typed in at the console, they will need to be forwarded.
// bring up the "need a cd to play" dialog
// dump all memory on an error
// shutdown client
// initialize renderer interface
// start all the client stuff using the hunk
// Restart sound subsystem
// for keyname autocompletion
// for writing the config files
/*
=================
S_ClearSoundBuffer
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_ClearSoundBuffer() {
    if si.ClearSoundBuffer.is_some() {
        si.ClearSoundBuffer.expect("non-null function pointer")();
    };
}
/*
=================
S_SoundInfo
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_SoundInfo() {
    if si.SoundInfo.is_some() {
        si.SoundInfo.expect("non-null function pointer")();
    };
}
/*
=================
S_SoundList
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_SoundList() {
    if si.SoundList.is_some() {
        si.SoundList.expect("non-null function pointer")();
    };
}
/*
=================
S_StartCapture
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_StartCapture() {
    if si.StartCapture.is_some() {
        si.StartCapture.expect("non-null function pointer")();
    };
}
/*
=================
S_AvailableCaptureSamples
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_AvailableCaptureSamples() -> libc::c_int {
    if si.AvailableCaptureSamples.is_some() {
        return si
            .AvailableCaptureSamples
            .expect("non-null function pointer")();
    }
    return 0 as libc::c_int;
}
/*
=================
S_Capture
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Capture(
    mut samples: libc::c_int,
    mut data: *mut crate::src::qcommon::q_shared::byte,
) {
    if si.Capture.is_some() {
        si.Capture.expect("non-null function pointer")(samples, data);
    };
}
/*
=================
S_StopCapture
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_StopCapture() {
    if si.StopCapture.is_some() {
        si.StopCapture.expect("non-null function pointer")();
    };
}
// if origin is NULL, the sound will be dynamically sourced from the entity
// if origin is NULL, the sound will be dynamically sourced from the entity
// if origin is NULL, the sound will be dynamically sourced from the entity
// cinematics and voice-over-network will send raw samples
// 1.0 volume will be direct output of source samples
// cinematics and voice-over-network will send raw samples
// 1.0 volume will be direct output of source samples
// cinematics and voice-over-network will send raw samples
// 1.0 volume will be direct output of source samples
// stop all sounds and the background track
// stop all sounds and the background track
// stop all sounds and the background track
// all continuous looping sounds must be added before calling S_Update
// all continuous looping sounds must be added before calling S_Update
// all continuous looping sounds must be added before calling S_Update
// recompute the relative volumes for all running sounds
// relative to the given entityNum / orientation
// recompute the relative volumes for all running sounds
// relative to the given entityNum / orientation
// recompute the relative volumes for all running sounds
// relative to the given entityNum / orientation
// let the sound system know where an entity currently is
// let the sound system know where an entity currently is
// let the sound system know where an entity currently is
// RegisterSound will allways return a valid sample, even if it
// has to create a placeholder.  This prevents continuous filesystem
// checks for missing files
// RegisterSound will allways return a valid sample, even if it
// has to create a placeholder.  This prevents continuous filesystem
// checks for missing files
// RegisterSound will allways return a valid sample, even if it
// has to create a placeholder.  This prevents continuous filesystem
// checks for missing files
/*
=================
S_MasterGain
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_MasterGain(mut gain: libc::c_float) {
    if si.MasterGain.is_some() {
        si.MasterGain.expect("non-null function pointer")(gain);
    };
}
//=============================================================================
/*
=================
S_Play_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Play_f() {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut h: crate::src::qcommon::q_shared::sfxHandle_t = 0;
    if si.RegisterSound.is_none() || si.StartLocalSound.is_none() {
        return;
    }
    c = crate::src::qcommon::cmd::Cmd_Argc();
    if c < 2 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"Usage: play <sound filename> [sound filename] [sound filename] ...\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    i = 1 as libc::c_int;
    while i < c {
        h = si.RegisterSound.expect("non-null function pointer")(
            crate::src::qcommon::cmd::Cmd_Argv(i),
            crate::src::qcommon::q_shared::qfalse,
        );
        if h != 0 {
            si.StartLocalSound.expect("non-null function pointer")(
                h,
                crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND as libc::c_int,
            );
        }
        i += 1
    }
}
/*
=================
S_Music_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Music_f() {
    let mut c: libc::c_int = 0;
    if si.StartBackgroundTrack.is_none() {
        return;
    }
    c = crate::src::qcommon::cmd::Cmd_Argc();
    if c == 2 as libc::c_int {
        si.StartBackgroundTrack.expect("non-null function pointer")(
            crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
            0 as *const libc::c_char,
        );
    } else if c == 3 as libc::c_int {
        si.StartBackgroundTrack.expect("non-null function pointer")(
            crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
            crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int),
        );
    } else {
        crate::src::qcommon::common::Com_Printf(
            b"Usage: music <musicfile> [loopfile]\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    };
}
/*
=================
S_Music_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_StopMusic_f() {
    if si.StopBackgroundTrack.is_none() {
        return;
    }
    si.StopBackgroundTrack.expect("non-null function pointer")();
}
//=============================================================================
/*
=================
S_Init
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Init() {
    let mut cv: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    let mut started: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::common::Com_Printf(
        b"------ Initializing Sound ------\n\x00" as *const u8 as *const libc::c_char,
    );
    s_volume = crate::src::qcommon::cvar::Cvar_Get(
        b"s_volume\x00" as *const u8 as *const libc::c_char,
        b"0.8\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    s_musicVolume = crate::src::qcommon::cvar::Cvar_Get(
        b"s_musicvolume\x00" as *const u8 as *const libc::c_char,
        b"0.25\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    s_muted = crate::src::qcommon::cvar::Cvar_Get(
        b"s_muted\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    s_doppler = crate::src::qcommon::cvar::Cvar_Get(
        b"s_doppler\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    s_backend = crate::src::qcommon::cvar::Cvar_Get(
        b"s_backend\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    s_muteWhenMinimized = crate::src::qcommon::cvar::Cvar_Get(
        b"s_muteWhenMinimized\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    s_muteWhenUnfocused = crate::src::qcommon::cvar::Cvar_Get(
        b"s_muteWhenUnfocused\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cv = crate::src::qcommon::cvar::Cvar_Get(
        b"s_initsound\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    if (*cv).integer == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"Sound disabled.\n\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        crate::src::client::snd_codec::S_CodecInit();
        crate::src::qcommon::cmd::Cmd_AddCommand(
            b"play\x00" as *const u8 as *const libc::c_char,
            Some(S_Play_f as unsafe extern "C" fn() -> ()),
        );
        crate::src::qcommon::cmd::Cmd_AddCommand(
            b"music\x00" as *const u8 as *const libc::c_char,
            Some(S_Music_f as unsafe extern "C" fn() -> ()),
        );
        crate::src::qcommon::cmd::Cmd_AddCommand(
            b"stopmusic\x00" as *const u8 as *const libc::c_char,
            Some(S_StopMusic_f as unsafe extern "C" fn() -> ()),
        );
        crate::src::qcommon::cmd::Cmd_AddCommand(
            b"s_list\x00" as *const u8 as *const libc::c_char,
            Some(S_SoundList as unsafe extern "C" fn() -> ()),
        );
        crate::src::qcommon::cmd::Cmd_AddCommand(
            b"s_stop\x00" as *const u8 as *const libc::c_char,
            Some(S_StopAllSounds as unsafe extern "C" fn() -> ()),
        );
        crate::src::qcommon::cmd::Cmd_AddCommand(
            b"s_info\x00" as *const u8 as *const libc::c_char,
            Some(S_SoundInfo as unsafe extern "C" fn() -> ()),
        );
        cv = crate::src::qcommon::cvar::Cvar_Get(
            b"s_useOpenAL\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
            0x1 as libc::c_int | 0x20 as libc::c_int,
        ) as *mut crate::src::qcommon::q_shared::cvar_s;
        if (*cv).integer != 0 {
            //OpenAL
            started = crate::src::client::snd_openal::S_AL_Init(
                &mut si as *mut _ as *mut crate::snd_local_h::soundInterface_t,
            );
            crate::src::qcommon::cvar::Cvar_Set(
                b"s_backend\x00" as *const u8 as *const libc::c_char,
                b"OpenAL\x00" as *const u8 as *const libc::c_char,
            );
        }
        if started as u64 == 0 {
            started = crate::src::client::snd_dma::S_Base_Init(
                &mut si as *mut _ as *mut crate::snd_local_h::soundInterface_t,
            );
            crate::src::qcommon::cvar::Cvar_Set(
                b"s_backend\x00" as *const u8 as *const libc::c_char,
                b"base\x00" as *const u8 as *const libc::c_char,
            );
        }
        if started as u64 != 0 {
            if S_ValidSoundInterface(&mut si) as u64 == 0 {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                    b"Sound interface invalid\x00" as *const u8 as *const libc::c_char,
                );
            }
            S_SoundInfo();
            crate::src::qcommon::common::Com_Printf(
                b"Sound initialization successful.\n\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            crate::src::qcommon::common::Com_Printf(
                b"Sound initialization failed.\n\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    crate::src::qcommon::common::Com_Printf(
        b"--------------------------------\n\x00" as *const u8 as *const libc::c_char,
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
/*
=================
S_Shutdown
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_Shutdown() {
    if si.Shutdown.is_some() {
        si.Shutdown.expect("non-null function pointer")();
    }
    crate::stdlib::memset(
        &mut si as *mut crate::snd_local_h::soundInterface_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::snd_local_h::soundInterface_t>() as libc::c_ulong,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"play\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"music\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"stopmusic\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"s_list\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"s_stop\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"s_info\x00" as *const u8 as *const libc::c_char);
    crate::src::client::snd_codec::S_CodecShutdown();
}

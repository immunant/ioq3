use ::libc;

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
        return ::libc::strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    }
}

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
    #[inline]

    pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__mode_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__sigset_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::mode_t;
pub use crate::stdlib::time_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
pub use ::libc::tm;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::src::asm::ftola::qvmftolsse;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtime_s;
pub use crate::src::qcommon::q_shared::qtime_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::COM_CompareExtension;
pub use crate::src::qcommon::q_shared::COM_DefaultExtension;
pub use crate::src::qcommon::q_shared::Com_HexStrToInt;
pub use crate::src::qcommon::q_shared::Com_SkipCharset;
pub use crate::src::qcommon::q_shared::Com_TruncateLongString;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_stricmpn;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::stdlib::__jmp_buf;
pub use crate::stdlib::__jmp_buf_tag;
pub use crate::stdlib::_setjmp;
pub use crate::stdlib::jmp_buf;
pub use crate::stdlib::longjmp;

pub use crate::qcommon_h::completionFunc_t;
pub use crate::qcommon_h::cpuFeatures_t;
pub use crate::qcommon_h::field_t;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::sysEventType_t;
pub use crate::qcommon_h::sysEvent_t;
pub use crate::qcommon_h::xcommand_t;
pub use crate::qcommon_h::CF_3DNOW;
pub use crate::qcommon_h::CF_3DNOW_EXT;
pub use crate::qcommon_h::CF_ALTIVEC;
pub use crate::qcommon_h::CF_MMX;
pub use crate::qcommon_h::CF_MMX_EXT;
pub use crate::qcommon_h::CF_RDTSC;
pub use crate::qcommon_h::CF_SSE;
pub use crate::qcommon_h::CF_SSE2;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::qcommon_h::NS_CLIENT;
pub use crate::qcommon_h::NS_SERVER;
pub use crate::qcommon_h::SE_CHAR;
pub use crate::qcommon_h::SE_CONSOLE;
pub use crate::qcommon_h::SE_JOYSTICK_AXIS;
pub use crate::qcommon_h::SE_KEY;
pub use crate::qcommon_h::SE_MOUSE;
pub use crate::qcommon_h::SE_NONE;
pub use crate::qcommon_h::TAG_BOTLIB;
pub use crate::qcommon_h::TAG_FREE;
pub use crate::qcommon_h::TAG_GENERAL;
pub use crate::qcommon_h::TAG_RENDERER;
pub use crate::qcommon_h::TAG_SMALL;
pub use crate::qcommon_h::TAG_STATIC;
pub use crate::src::client::cl_console::CL_ConsolePrint;
pub use crate::src::client::cl_input::CL_JoystickEvent;
pub use crate::src::client::cl_input::CL_MouseEvent;
pub use crate::src::client::cl_keys::CL_CharEvent;
pub use crate::src::client::cl_keys::CL_InitKeyCommands;
pub use crate::src::client::cl_keys::CL_KeyEvent;
pub use crate::src::client::cl_keys::Key_KeynameCompletion;
pub use crate::src::client::cl_keys::Key_WriteBindings;
pub use crate::src::client::cl_main::CL_CDDialog;
pub use crate::src::client::cl_main::CL_Disconnect;
pub use crate::src::client::cl_main::CL_FlushMemory;
pub use crate::src::client::cl_main::CL_Frame;
pub use crate::src::client::cl_main::CL_Init;
pub use crate::src::client::cl_main::CL_PacketEvent;
pub use crate::src::client::cl_main::CL_Shutdown;
pub use crate::src::client::cl_main::CL_StartHunkUsers;
pub use crate::src::client::cl_ui::UI_usesUniqueCDKey;
pub use crate::src::qcommon::cmd::Cbuf_AddText;
pub use crate::src::qcommon::cmd::Cbuf_Execute;
pub use crate::src::qcommon::cmd::Cbuf_ExecuteText;
pub use crate::src::qcommon::cmd::Cbuf_Init;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_Args;
pub use crate::src::qcommon::cmd::Cmd_ArgsFrom;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::cmd::Cmd_CommandCompletion;
pub use crate::src::qcommon::cmd::Cmd_CompleteArgument;
pub use crate::src::qcommon::cmd::Cmd_CompleteCfgName;
pub use crate::src::qcommon::cmd::Cmd_Init;
pub use crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc;
pub use crate::src::qcommon::cmd::Cmd_TokenizeString;
pub use crate::src::qcommon::cmd::Cmd_TokenizeStringIgnoreQuotes;
pub use crate::src::qcommon::common::stdlib_float_h::atof;
pub use crate::src::qcommon::cvar::cvar_modifiedFlags;
pub use crate::src::qcommon::cvar::Cvar_CheckRange;
pub use crate::src::qcommon::cvar::Cvar_CommandCompletion;
pub use crate::src::qcommon::cvar::Cvar_Flags;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_Init;
pub use crate::src::qcommon::cvar::Cvar_Restart;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_Set2;
pub use crate::src::qcommon::cvar::Cvar_SetDescription;
pub use crate::src::qcommon::cvar::Cvar_SetValue;
pub use crate::src::qcommon::cvar::Cvar_VariableString;
pub use crate::src::qcommon::cvar::Cvar_WriteVariables;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_FCreateOpenPipeFile;
pub use crate::src::qcommon::files::FS_FOpenFileRead;
pub use crate::src::qcommon::files::FS_FOpenFileWrite;
pub use crate::src::qcommon::files::FS_FilenameCompletion;
pub use crate::src::qcommon::files::FS_ForceFlush;
pub use crate::src::qcommon::files::FS_GetCurrentGameDir;
pub use crate::src::qcommon::files::FS_HomeRemove;
pub use crate::src::qcommon::files::FS_InitFilesystem;
pub use crate::src::qcommon::files::FS_Initialized;
pub use crate::src::qcommon::files::FS_LoadStack;
pub use crate::src::qcommon::files::FS_Printf;
pub use crate::src::qcommon::files::FS_PureServerSetLoadedPaks;
pub use crate::src::qcommon::files::FS_Read;
pub use crate::src::qcommon::files::FS_Restart;
pub use crate::src::qcommon::files::FS_SV_FOpenFileRead;
pub use crate::src::qcommon::files::FS_SV_FOpenFileWrite;
pub use crate::src::qcommon::files::FS_Shutdown;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::msg::MSG_Init;
pub use crate::src::qcommon::msg::MSG_ReportChangeVectors_f;
pub use crate::src::qcommon::net_chan::NET_FlushPacketQueue;
pub use crate::src::qcommon::net_chan::NET_GetLoopPacket;
pub use crate::src::qcommon::net_chan::Netchan_Init;
pub use crate::src::qcommon::net_ip::NET_Restart_f;
pub use crate::src::qcommon::net_ip::NET_Sleep;
pub use crate::src::qcommon::vm::VM_Clear;
pub use crate::src::qcommon::vm::VM_Forced_Unload_Done;
pub use crate::src::qcommon::vm::VM_Forced_Unload_Start;
pub use crate::src::qcommon::vm::VM_Init;
pub use crate::src::sdl::sdl_input::IN_Frame;
pub use crate::src::server::sv_init::SV_Init;
pub use crate::src::server::sv_init::SV_Shutdown;
pub use crate::src::server::sv_main::SV_Frame;
pub use crate::src::server::sv_main::SV_FrameMsec;
pub use crate::src::server::sv_main::SV_PacketEvent;
pub use crate::src::server::sv_main::SV_SendQueuedPackets;
pub use crate::src::sys::sys_main::Sys_ConsoleInput;
pub use crate::src::sys::sys_main::Sys_Error;
pub use crate::src::sys::sys_main::Sys_GetProcessorFeatures;
pub use crate::src::sys::sys_main::Sys_Init;
pub use crate::src::sys::sys_main::Sys_InitPIDFile;
pub use crate::src::sys::sys_main::Sys_Print;
pub use crate::src::sys::sys_main::Sys_Quit;
pub use crate::src::sys::sys_unix::Sys_Milliseconds;
pub use crate::src::sys::sys_unix::Sys_RandomBytes;
pub use crate::src::sys::sys_unix::Sys_SetEnv;

pub use crate::src::qcommon::common::ctype_h::tolower;
pub use crate::src::qcommon::common::ctype_h::toupper;
pub use crate::stdlib::__ctype_tolower_loc;
pub use crate::stdlib::__ctype_toupper_loc;

extern "C" {
    #[no_mangle]
    pub fn CIN_CloseAllVideos();
    #[no_mangle]
    pub fn CL_ShutdownCGame();
    #[no_mangle]
    pub fn CL_ShutdownUI();
    #[no_mangle]
    pub fn SV_ShutdownGameProgs();
    /*
    =================
    Com_ReadCDKey
    =================
    */
    #[no_mangle]
    pub fn CL_CDKeyValidate(
        key: *const libc::c_char,
        checksum: *const libc::c_char,
    ) -> crate::src::qcommon::q_shared::qboolean;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hunkUsed_t {
    pub mark: libc::c_int,
    pub permanent: libc::c_int,
    pub temp: libc::c_int,
    pub tempHighwater: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memblock_s {
    pub size: libc::c_int,
    pub tag: libc::c_int,
    pub next: *mut memblock_s,
    pub prev: *mut memblock_s,
    pub id: libc::c_int,
}

pub type memblock_t = memblock_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memzone_t {
    pub size: libc::c_int,
    pub used: libc::c_int,
    pub blocklist: memblock_t,
    pub rover: *mut memblock_t,
}
// including the header and possibly tiny fragments
// a tag of 0 is a free block
// should be ZONEID
// static mem blocks to reduce a lot of small zone overhead

pub type memstatic_t = memstatic_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memstatic_s {
    pub b: memblock_t,
    pub mem: [crate::src::qcommon::q_shared::byte; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hunkHeader_t {
    pub magic: libc::c_int,
    pub size: libc::c_int,
}

pub type hunkblock_t = hunkblock_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hunkblock_s {
    pub size: libc::c_int,
    pub printed: crate::src::qcommon::q_shared::byte,
    pub next: *mut hunkblock_s,
    pub label: *mut libc::c_char,
    pub file: *mut libc::c_char,
    pub line: libc::c_int,
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
// common.c -- misc functions used in client and server
// umask
#[no_mangle]

pub static mut demo_protocols: [libc::c_int; 3] =
    [67 as libc::c_int, 66 as libc::c_int, 0 as libc::c_int];
#[no_mangle]

pub static mut com_argc: libc::c_int = 0;
#[no_mangle]

pub static mut com_argv: [*mut libc::c_char; 51] =
    [0 as *const libc::c_char as *mut libc::c_char; 51];
#[no_mangle]

pub static mut abortframe: crate::stdlib::jmp_buf = [crate::stdlib::__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: crate::stdlib::__sigset_t { __val: [0; 16] },
}; 1];
// an ERR_DROP occurred, exit the entire frame
#[no_mangle]

pub static mut debuglogfile: *mut crate::stdlib::FILE =
    0 as *const crate::stdlib::FILE as *mut crate::stdlib::FILE;

static mut pipefile: crate::src::qcommon::q_shared::fileHandle_t = 0;

static mut logfile: crate::src::qcommon::q_shared::fileHandle_t = 0;
#[no_mangle]

pub static mut com_journalFile: crate::src::qcommon::q_shared::fileHandle_t = 0;
// events are written here
#[no_mangle]

pub static mut com_journalDataFile: crate::src::qcommon::q_shared::fileHandle_t = 0;
// config files are written here
#[no_mangle]

pub static mut com_speeds: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_developer: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_dedicated: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_timescale: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_fixedtime: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_journal: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_maxfps: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_altivec: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_timedemo: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_sv_running: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_cl_running: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_logfile: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
// 1 = buffer log, 2 = flush after each print
#[no_mangle]

pub static mut com_pipefile: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_showtrace: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_version: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_blood: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_buildScript: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
// for automated data building scripts
#[no_mangle]

pub static mut com_introPlayed: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_paused: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut sv_paused: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_packetdelay: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut sv_packetdelay: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_cameraMode: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_ansiColor: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_unfocused: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_maxfpsUnfocused: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_minimized: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_maxfpsMinimized: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_abnormalExit: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_standalone: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_gamename: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_protocol: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_legacyprotocol: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_basegame: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_homepath: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut com_busyWait: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut con_autochat: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut Q_VMftol: Option<unsafe extern "C" fn() -> libc::c_int> = None;
// com_speeds times
#[no_mangle]

pub static mut time_game: libc::c_int = 0;
#[no_mangle]

pub static mut time_frontend: libc::c_int = 0;
// renderer frontend time
#[no_mangle]

pub static mut time_backend: libc::c_int = 0;
// renderer backend time
#[no_mangle]

pub static mut com_frameTime: libc::c_int = 0;
#[no_mangle]

pub static mut com_frameNumber: libc::c_int = 0;
#[no_mangle]

pub static mut com_errorEntered: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub static mut com_fullyInitialized: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub static mut com_gameRestarting: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub static mut com_gameClientRestarting: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub static mut com_errorMessage: [libc::c_char; 4096] = [0; 4096];
//============================================================================

static mut rd_buffer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;

static mut rd_buffersize: libc::c_int = 0;

static mut rd_flush: Option<unsafe extern "C" fn(_: *mut libc::c_char) -> ()> = None;
#[no_mangle]

pub unsafe extern "C" fn Com_BeginRedirect(
    mut buffer: *mut libc::c_char,
    mut buffersize: libc::c_int,
    mut flush: Option<unsafe extern "C" fn(_: *mut libc::c_char) -> ()>,
) {
    if buffer.is_null() || buffersize == 0 || flush.is_none() {
        return;
    }
    rd_buffer = buffer;
    rd_buffersize = buffersize;
    rd_flush = flush;
    *rd_buffer = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]

pub unsafe extern "C" fn Com_EndRedirect() {
    if rd_flush.is_some() {
        rd_flush.expect("non-null function pointer")(rd_buffer);
    }
    rd_buffer = 0 as *mut libc::c_char;
    rd_buffersize = 0 as libc::c_int;
    rd_flush = None;
}
/*
=============
Com_Printf

Both client and server can use this, and it will output
to the appropriate place.

A raw string should NEVER be passed as fmt, because of "%f" type crashers.
=============
*/
#[no_mangle]

pub unsafe extern "C" fn Com_Printf(mut fmt: *const libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut msg: [libc::c_char; 4096] = [0; 4096];
    static mut opening_qconsole: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        msg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        fmt,
        argptr.as_va_list(),
    );
    if !rd_buffer.is_null() {
        if crate::stdlib::strlen(msg.as_mut_ptr()).wrapping_add(crate::stdlib::strlen(rd_buffer))
            > (rd_buffersize - 1 as libc::c_int) as libc::c_ulong
        {
            rd_flush.expect("non-null function pointer")(rd_buffer);
            *rd_buffer = 0 as libc::c_int as libc::c_char
        }
        crate::src::qcommon::q_shared::Q_strcat(rd_buffer, rd_buffersize, msg.as_mut_ptr());
        // TTimo nooo .. that would defeat the purpose
        //rd_flush(rd_buffer);
        //*rd_buffer = 0;
        return;
    }
    crate::src::client::cl_console::CL_ConsolePrint(msg.as_mut_ptr());
    // echo to dedicated console and early console
    crate::src::sys::sys_main::Sys_Print(msg.as_mut_ptr());
    // logfile
    if !com_logfile.is_null() && (*com_logfile).integer != 0 {
        // TTimo: only open the qconsole.log if the filesystem is in an initialized state
        //   also, avoid recursing in the qconsole.log opening (i.e. if fs_debug is on)
        if logfile == 0
            && crate::src::qcommon::files::FS_Initialized() as libc::c_uint != 0
            && opening_qconsole as u64 == 0
        {
            let mut newtime: *mut ::libc::tm = 0 as *mut ::libc::tm;
            let mut aclock: crate::stdlib::time_t = 0;
            opening_qconsole = crate::src::qcommon::q_shared::qtrue;
            ::libc::time(&mut aclock);
            newtime = ::libc::localtime(&mut aclock) as *mut ::libc::tm;
            logfile = crate::src::qcommon::files::FS_FOpenFileWrite(
                b"qconsole.log\x00" as *const u8 as *const libc::c_char,
            );
            if logfile != 0 {
                Com_Printf(
                    b"logfile opened on %s\n\x00" as *const u8 as *const libc::c_char,
                    crate::stdlib::asctime(newtime),
                );
                if (*com_logfile).integer > 1 as libc::c_int {
                    // force it to not buffer so we get valid
                    // data even if we are crashing
                    crate::src::qcommon::files::FS_ForceFlush(logfile);
                }
            } else {
                Com_Printf(
                    b"Opening qconsole.log failed!\n\x00" as *const u8 as *const libc::c_char,
                );
                crate::src::qcommon::cvar::Cvar_SetValue(
                    b"logfile\x00" as *const u8 as *const libc::c_char,
                    0 as libc::c_int as libc::c_float,
                );
            }
            opening_qconsole = crate::src::qcommon::q_shared::qfalse
        }
        if logfile != 0 && crate::src::qcommon::files::FS_Initialized() as libc::c_uint != 0 {
            crate::src::qcommon::files::FS_Write(
                msg.as_mut_ptr() as *const libc::c_void,
                crate::stdlib::strlen(msg.as_mut_ptr()) as libc::c_int,
                logfile,
            );
        }
    };
}
/*
================
Com_DPrintf

A Com_Printf that only shows up if the "developer" cvar is set
================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_DPrintf(mut fmt: *const libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut msg: [libc::c_char; 4096] = [0; 4096];
    if com_developer.is_null() || (*com_developer).integer == 0 {
        return;
        // don't confuse non-developers with techie stuff...
    }
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        msg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        fmt,
        argptr.as_va_list(),
    );
    Com_Printf(
        b"%s\x00" as *const u8 as *const libc::c_char,
        msg.as_mut_ptr(),
    );
}
/*
=============
Com_Error

Both client and server can use this, and it will
do the appropriate thing.
=============
*/
#[no_mangle]

pub unsafe extern "C" fn Com_Error(
    mut code: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut lastErrorTime: libc::c_int = 0;
    static mut errorCount: libc::c_int = 0;
    let mut currentTime: libc::c_int = 0;
    let mut restartClient: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if com_errorEntered as u64 != 0 {
        crate::src::sys::sys_main::Sys_Error(
            b"recursive error after: %s\x00" as *const u8 as *const libc::c_char,
            com_errorMessage.as_mut_ptr(),
        );
    }
    com_errorEntered = crate::src::qcommon::q_shared::qtrue;
    crate::src::qcommon::cvar::Cvar_Set(
        b"com_errorCode\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            code,
        ),
    );
    // when we are running automated scripts, make sure we
    // know if anything failed
    if !com_buildScript.is_null() && (*com_buildScript).integer != 0 {
        code = crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int
    }
    // if we are getting a solid stream of ERR_DROP, do an ERR_FATAL
    currentTime = crate::src::sys::sys_unix::Sys_Milliseconds();
    if currentTime - lastErrorTime < 100 as libc::c_int {
        errorCount += 1;
        if errorCount > 3 as libc::c_int {
            code = crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int
        }
    } else {
        errorCount = 0 as libc::c_int
    }
    lastErrorTime = currentTime;
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        com_errorMessage.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        fmt,
        argptr.as_va_list(),
    );
    if code != crate::src::qcommon::q_shared::ERR_DISCONNECT as libc::c_int
        && code != crate::src::qcommon::q_shared::ERR_NEED_CD as libc::c_int
    {
        crate::src::qcommon::cvar::Cvar_Set(
            b"com_errorMessage\x00" as *const u8 as *const libc::c_char,
            com_errorMessage.as_mut_ptr(),
        );
    }
    restartClient = (com_gameClientRestarting as libc::c_uint != 0
        && !(!com_cl_running.is_null() && (*com_cl_running).integer != 0))
        as libc::c_int as crate::src::qcommon::q_shared::qboolean;
    com_gameRestarting = crate::src::qcommon::q_shared::qfalse;
    com_gameClientRestarting = crate::src::qcommon::q_shared::qfalse;
    if code == crate::src::qcommon::q_shared::ERR_DISCONNECT as libc::c_int
        || code == crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT as libc::c_int
    {
        crate::src::qcommon::vm::VM_Forced_Unload_Start();
        crate::src::server::sv_init::SV_Shutdown(
            b"Server disconnected\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if restartClient as u64 != 0 {
            crate::src::client::cl_main::CL_Init();
        }
        crate::src::client::cl_main::CL_Disconnect(crate::src::qcommon::q_shared::qtrue);
        crate::src::client::cl_main::CL_FlushMemory();
        crate::src::qcommon::vm::VM_Forced_Unload_Done();
        // make sure we can get at our local stuff
        crate::src::qcommon::files::FS_PureServerSetLoadedPaks(
            b"\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
        com_errorEntered = crate::src::qcommon::q_shared::qfalse;
        crate::stdlib::longjmp(abortframe.as_mut_ptr(), -(1 as libc::c_int));
    } else {
        if code == crate::src::qcommon::q_shared::ERR_DROP as libc::c_int {
            Com_Printf(
                b"********************\nERROR: %s\n********************\n\x00" as *const u8
                    as *const libc::c_char,
                com_errorMessage.as_mut_ptr(),
            );
            crate::src::qcommon::vm::VM_Forced_Unload_Start();
            crate::src::server::sv_init::SV_Shutdown(crate::src::qcommon::q_shared::va(
                b"Server crashed: %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                com_errorMessage.as_mut_ptr(),
            ));
            if restartClient as u64 != 0 {
                crate::src::client::cl_main::CL_Init();
            }
            crate::src::client::cl_main::CL_Disconnect(crate::src::qcommon::q_shared::qtrue);
            crate::src::client::cl_main::CL_FlushMemory();
            crate::src::qcommon::vm::VM_Forced_Unload_Done();
            crate::src::qcommon::files::FS_PureServerSetLoadedPaks(
                b"\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char,
            );
            com_errorEntered = crate::src::qcommon::q_shared::qfalse;
            crate::stdlib::longjmp(abortframe.as_mut_ptr(), -(1 as libc::c_int));
        } else {
            if code == crate::src::qcommon::q_shared::ERR_NEED_CD as libc::c_int {
                crate::src::qcommon::vm::VM_Forced_Unload_Start();
                crate::src::server::sv_init::SV_Shutdown(
                    b"Server didn\'t have CD\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                if restartClient as u64 != 0 {
                    crate::src::client::cl_main::CL_Init();
                }
                if !com_cl_running.is_null() && (*com_cl_running).integer != 0 {
                    crate::src::client::cl_main::CL_Disconnect(
                        crate::src::qcommon::q_shared::qtrue,
                    );
                    crate::src::client::cl_main::CL_FlushMemory();
                    crate::src::qcommon::vm::VM_Forced_Unload_Done();
                    crate::src::client::cl_main::CL_CDDialog();
                } else {
                    Com_Printf(b"Server didn\'t have CD\n\x00" as *const u8 as *const libc::c_char);
                    crate::src::qcommon::vm::VM_Forced_Unload_Done();
                }
                crate::src::qcommon::files::FS_PureServerSetLoadedPaks(
                    b"\x00" as *const u8 as *const libc::c_char,
                    b"\x00" as *const u8 as *const libc::c_char,
                );
                com_errorEntered = crate::src::qcommon::q_shared::qfalse;
                crate::stdlib::longjmp(abortframe.as_mut_ptr(), -(1 as libc::c_int));
            } else {
                crate::src::qcommon::vm::VM_Forced_Unload_Start();
                crate::src::client::cl_main::CL_Shutdown(
                    crate::src::qcommon::q_shared::va(
                        b"Client fatal crashed: %s\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        com_errorMessage.as_mut_ptr(),
                    ),
                    crate::src::qcommon::q_shared::qtrue,
                    crate::src::qcommon::q_shared::qtrue,
                );
                crate::src::server::sv_init::SV_Shutdown(crate::src::qcommon::q_shared::va(
                    b"Server fatal crashed: %s\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    com_errorMessage.as_mut_ptr(),
                ));
                crate::src::qcommon::vm::VM_Forced_Unload_Done();
            }
        }
    }
    Com_Shutdown();
    crate::src::sys::sys_main::Sys_Error(
        b"%s\x00" as *const u8 as *const libc::c_char,
        com_errorMessage.as_mut_ptr(),
    );
}
/*
=============
Com_Quit_f

Both client and server can use this, and it will
do the appropriate things.
=============
*/
#[no_mangle]

pub unsafe extern "C" fn Com_Quit_f() -> ! {
    // don't try to shutdown if we are in a recursive error
    let mut p: *mut libc::c_char = crate::src::qcommon::cmd::Cmd_Args();
    if com_errorEntered as u64 == 0 {
        // Some VMs might execute "quit" command directly,
        // which would trigger an unload of active VM error.
        // Sys_Quit will kill this process anyways, so
        // a corrupt call stack makes no difference
        crate::src::qcommon::vm::VM_Forced_Unload_Start();
        crate::src::server::sv_init::SV_Shutdown(if *p.offset(0 as libc::c_int as isize)
            as libc::c_int
            != 0
        {
            p as *const libc::c_char
        } else {
            b"Server quit\x00" as *const u8 as *const libc::c_char
        } as *mut libc::c_char);
        crate::src::client::cl_main::CL_Shutdown(
            if *p.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                p as *const libc::c_char
            } else {
                b"Client quit\x00" as *const u8 as *const libc::c_char
            } as *mut libc::c_char,
            crate::src::qcommon::q_shared::qtrue,
            crate::src::qcommon::q_shared::qtrue,
        );
        crate::src::qcommon::vm::VM_Forced_Unload_Done();
        Com_Shutdown();
        crate::src::qcommon::files::FS_Shutdown(crate::src::qcommon::q_shared::qtrue);
    }
    crate::src::sys::sys_main::Sys_Quit();
}
#[no_mangle]

pub static mut com_numConsoleLines: libc::c_int = 0;
#[no_mangle]

pub static mut com_consoleLines: [*mut libc::c_char; 32] =
    [0 as *const libc::c_char as *mut libc::c_char; 32];
/*
==================
Com_ParseCommandLine

Break it up into multiple console lines
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_ParseCommandLine(mut commandLine: *mut libc::c_char) {
    let mut inq: libc::c_int = 0 as libc::c_int;
    com_consoleLines[0 as libc::c_int as usize] = commandLine;
    com_numConsoleLines = 1 as libc::c_int;
    while *commandLine != 0 {
        if *commandLine as libc::c_int == '\"' as i32 {
            inq = (inq == 0) as libc::c_int
        }
        // look for a + separating character
        // if commandLine came from a file, we might have real line seperators
        if *commandLine as libc::c_int == '+' as i32 && inq == 0
            || *commandLine as libc::c_int == '\n' as i32
            || *commandLine as libc::c_int == '\r' as i32
        {
            if com_numConsoleLines == 32 as libc::c_int {
                return;
            }
            com_consoleLines[com_numConsoleLines as usize] =
                commandLine.offset(1 as libc::c_int as isize);
            com_numConsoleLines += 1;
            *commandLine = 0 as libc::c_int as libc::c_char
        }
        commandLine = commandLine.offset(1)
    }
}
/*
===================
Com_SafeMode

Check for "safe" on the command line, which will
skip loading of q3config.cfg
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_SafeMode() -> crate::src::qcommon::q_shared::qboolean {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < com_numConsoleLines {
        crate::src::qcommon::cmd::Cmd_TokenizeString(com_consoleLines[i as usize]);
        if crate::src::qcommon::q_shared::Q_stricmp(
            crate::src::qcommon::cmd::Cmd_Argv(0 as libc::c_int),
            b"safe\x00" as *const u8 as *const libc::c_char,
        ) == 0
            || crate::src::qcommon::q_shared::Q_stricmp(
                crate::src::qcommon::cmd::Cmd_Argv(0 as libc::c_int),
                b"cvar_restart\x00" as *const u8 as *const libc::c_char,
            ) == 0
        {
            *com_consoleLines[i as usize].offset(0 as libc::c_int as isize) =
                0 as libc::c_int as libc::c_char;
            return crate::src::qcommon::q_shared::qtrue;
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
===============
Com_StartupVariable

Searches for command line parameters that are set commands.
If match is not NULL, only that cvar will be looked for.
That is necessary because cddir and basedir need to be set
before the filesystem is started, but all other sets should
be after execing the config and default.
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Com_StartupVariable(mut match_0: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < com_numConsoleLines {
        crate::src::qcommon::cmd::Cmd_TokenizeString(com_consoleLines[i as usize]);
        if !(::libc::strcmp(
            crate::src::qcommon::cmd::Cmd_Argv(0 as libc::c_int),
            b"set\x00" as *const u8 as *const libc::c_char,
        ) != 0)
        {
            s = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int);
            if match_0.is_null() || ::libc::strcmp(s, match_0) == 0 {
                if crate::src::qcommon::cvar::Cvar_Flags(s) as libc::c_uint
                    == 0x80000000 as libc::c_uint
                {
                    crate::src::qcommon::cvar::Cvar_Get(
                        s,
                        crate::src::qcommon::cmd::Cmd_ArgsFrom(2 as libc::c_int),
                        0x80 as libc::c_int,
                    ) as *mut crate::src::qcommon::q_shared::cvar_s;
                } else {
                    crate::src::qcommon::cvar::Cvar_Set2(
                        s,
                        crate::src::qcommon::cmd::Cmd_ArgsFrom(2 as libc::c_int),
                        crate::src::qcommon::q_shared::qfalse,
                    ) as *mut crate::src::qcommon::q_shared::cvar_s;
                }
            }
        }
        i += 1
    }
}
/*
=================
Com_AddStartupCommands

Adds command line parameters as script statements
Commands are separated by + signs

Returns qtrue if any late commands were added, which
will keep the demoloop from immediately starting
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_AddStartupCommands() -> crate::src::qcommon::q_shared::qboolean {
    let mut i: libc::c_int = 0;
    let mut added: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    added = crate::src::qcommon::q_shared::qfalse;
    // quote every token, so args with semicolons can work
    i = 0 as libc::c_int;
    while i < com_numConsoleLines {
        if !(com_consoleLines[i as usize].is_null()
            || *com_consoleLines[i as usize].offset(0 as libc::c_int as isize) == 0)
        {
            // set commands already added with Com_StartupVariable
            if !(crate::src::qcommon::q_shared::Q_stricmpn(
                com_consoleLines[i as usize],
                b"set \x00" as *const u8 as *const libc::c_char,
                4 as libc::c_int,
            ) == 0)
            {
                added = crate::src::qcommon::q_shared::qtrue;
                crate::src::qcommon::cmd::Cbuf_AddText(com_consoleLines[i as usize]);
                crate::src::qcommon::cmd::Cbuf_AddText(
                    b"\n\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        i += 1
    }
    return added;
}
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn Info_Print(mut s: *const libc::c_char) {
    let mut key: [libc::c_char; 8192] = [0; 8192];
    let mut value: [libc::c_char; 8192] = [0; 8192];
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    if *s as libc::c_int == '\\' as i32 {
        s = s.offset(1)
    }
    while *s != 0 {
        o = key.as_mut_ptr();
        while *s as libc::c_int != 0 && *s as libc::c_int != '\\' as i32 {
            let fresh0 = s;
            s = s.offset(1);
            let fresh1 = o;
            o = o.offset(1);
            *fresh1 = *fresh0
        }
        l = o.wrapping_offset_from(key.as_mut_ptr()) as libc::c_long as libc::c_int;
        if l < 20 as libc::c_int {
            crate::stdlib::memset(
                o as *mut libc::c_void,
                ' ' as i32,
                (20 as libc::c_int - l) as libc::c_ulong,
            );
            key[20 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char
        } else {
            *o = 0 as libc::c_int as libc::c_char
        }
        Com_Printf(
            b"%s \x00" as *const u8 as *const libc::c_char,
            key.as_mut_ptr(),
        );
        if *s == 0 {
            Com_Printf(b"MISSING VALUE\n\x00" as *const u8 as *const libc::c_char);
            return;
        }
        o = value.as_mut_ptr();
        s = s.offset(1);
        while *s as libc::c_int != 0 && *s as libc::c_int != '\\' as i32 {
            let fresh2 = s;
            s = s.offset(1);
            let fresh3 = o;
            o = o.offset(1);
            *fresh3 = *fresh2
        }
        *o = 0 as libc::c_int as libc::c_char;
        if *s != 0 {
            s = s.offset(1)
        }
        Com_Printf(
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            value.as_mut_ptr(),
        );
    }
}
/*
============
Com_StringContains
============
*/
#[no_mangle]

pub unsafe extern "C" fn Com_StringContains(
    mut str1: *mut libc::c_char,
    mut str2: *mut libc::c_char,
    mut casesensitive: libc::c_int,
) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    len = crate::stdlib::strlen(str1).wrapping_sub(crate::stdlib::strlen(str2)) as libc::c_int;
    i = 0 as libc::c_int;
    while i <= len {
        j = 0 as libc::c_int;
        while *str2.offset(j as isize) != 0 {
            if casesensitive != 0 {
                if *str1.offset(j as isize) as libc::c_int
                    != *str2.offset(j as isize) as libc::c_int
                {
                    break;
                }
            } else if ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *str1.offset(j as isize) as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = toupper(*str1.offset(j as isize) as libc::c_int)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_toupper_loc())
                        .offset(*str1.offset(j as isize) as libc::c_int as isize)
                }
                __res
            }) != ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *str2.offset(j as isize) as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = toupper(*str2.offset(j as isize) as libc::c_int)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_toupper_loc())
                        .offset(*str2.offset(j as isize) as libc::c_int as isize)
                }
                __res
            }) {
                break;
            }
            j += 1
        }
        if *str2.offset(j as isize) == 0 {
            return str1;
        }
        i += 1;
        str1 = str1.offset(1)
    }
    return 0 as *mut libc::c_char;
}
/*
============
Com_Filter
============
*/
#[no_mangle]

pub unsafe extern "C" fn Com_Filter(
    mut filter: *mut libc::c_char,
    mut name: *mut libc::c_char,
    mut casesensitive: libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    while *filter != 0 {
        if *filter as libc::c_int == '*' as i32 {
            filter = filter.offset(1);
            i = 0 as libc::c_int;
            while *filter != 0 {
                if *filter as libc::c_int == '*' as i32 || *filter as libc::c_int == '?' as i32 {
                    break;
                }
                buf[i as usize] = *filter;
                filter = filter.offset(1);
                i += 1
            }
            buf[i as usize] = '\u{0}' as i32 as libc::c_char;
            if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
                ptr = Com_StringContains(name, buf.as_mut_ptr(), casesensitive);
                if ptr.is_null() {
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
                name = ptr.offset(crate::stdlib::strlen(buf.as_mut_ptr()) as isize)
            }
        } else if *filter as libc::c_int == '?' as i32 {
            filter = filter.offset(1);
            name = name.offset(1)
        } else if *filter as libc::c_int == '[' as i32
            && *filter.offset(1 as libc::c_int as isize) as libc::c_int == '[' as i32
        {
            filter = filter.offset(1)
        } else if *filter as libc::c_int == '[' as i32 {
            filter = filter.offset(1);
            found = crate::src::qcommon::q_shared::qfalse as libc::c_int;
            while *filter as libc::c_int != 0 && found == 0 {
                if *filter as libc::c_int == ']' as i32
                    && *filter.offset(1 as libc::c_int as isize) as libc::c_int != ']' as i32
                {
                    break;
                }
                if *filter.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
                    && *filter.offset(2 as libc::c_int as isize) as libc::c_int != 0
                    && (*filter.offset(2 as libc::c_int as isize) as libc::c_int != ']' as i32
                        || *filter.offset(3 as libc::c_int as isize) as libc::c_int == ']' as i32)
                {
                    if casesensitive != 0 {
                        if *name as libc::c_int >= *filter as libc::c_int
                            && *name as libc::c_int
                                <= *filter.offset(2 as libc::c_int as isize) as libc::c_int
                        {
                            found = crate::src::qcommon::q_shared::qtrue as libc::c_int
                        }
                    } else if ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *name as libc::c_int;
                                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                    __c
                                } else {
                                    *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                                }
                            } else {
                                __res = toupper(*name as libc::c_int)
                            }
                        } else {
                            __res = *(*crate::stdlib::__ctype_toupper_loc())
                                .offset(*name as libc::c_int as isize)
                        }
                        __res
                    }) >= ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *filter as libc::c_int;
                                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                    __c
                                } else {
                                    *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                                }
                            } else {
                                __res = toupper(*filter as libc::c_int)
                            }
                        } else {
                            __res = *(*crate::stdlib::__ctype_toupper_loc())
                                .offset(*filter as libc::c_int as isize)
                        }
                        __res
                    }) && ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *name as libc::c_int;
                                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                    __c
                                } else {
                                    *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                                }
                            } else {
                                __res = toupper(*name as libc::c_int)
                            }
                        } else {
                            __res = *(*crate::stdlib::__ctype_toupper_loc())
                                .offset(*name as libc::c_int as isize)
                        }
                        __res
                    }) <= ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int =
                                    *filter.offset(2 as libc::c_int as isize) as libc::c_int;
                                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                    __c
                                } else {
                                    *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                                }
                            } else {
                                __res = toupper(
                                    *filter.offset(2 as libc::c_int as isize) as libc::c_int
                                )
                            }
                        } else {
                            __res = *(*crate::stdlib::__ctype_toupper_loc())
                                .offset(*filter.offset(2 as libc::c_int as isize) as libc::c_int
                                    as isize)
                        }
                        __res
                    }) {
                        found = crate::src::qcommon::q_shared::qtrue as libc::c_int
                    }
                    filter = filter.offset(3 as libc::c_int as isize)
                } else {
                    if casesensitive != 0 {
                        if *filter as libc::c_int == *name as libc::c_int {
                            found = crate::src::qcommon::q_shared::qtrue as libc::c_int
                        }
                    } else if ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *filter as libc::c_int;
                                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                    __c
                                } else {
                                    *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                                }
                            } else {
                                __res = toupper(*filter as libc::c_int)
                            }
                        } else {
                            __res = *(*crate::stdlib::__ctype_toupper_loc())
                                .offset(*filter as libc::c_int as isize)
                        }
                        __res
                    }) == ({
                        let mut __res: libc::c_int = 0;
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *name as libc::c_int;
                                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                    __c
                                } else {
                                    *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                                }
                            } else {
                                __res = toupper(*name as libc::c_int)
                            }
                        } else {
                            __res = *(*crate::stdlib::__ctype_toupper_loc())
                                .offset(*name as libc::c_int as isize)
                        }
                        __res
                    }) {
                        found = crate::src::qcommon::q_shared::qtrue as libc::c_int
                    }
                    filter = filter.offset(1)
                }
            }
            if found == 0 {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            while *filter != 0 {
                if *filter as libc::c_int == ']' as i32
                    && *filter.offset(1 as libc::c_int as isize) as libc::c_int != ']' as i32
                {
                    break;
                }
                filter = filter.offset(1)
            }
            filter = filter.offset(1);
            name = name.offset(1)
        } else {
            if casesensitive != 0 {
                if *filter as libc::c_int != *name as libc::c_int {
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
            } else if ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *filter as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = toupper(*filter as libc::c_int)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_toupper_loc())
                        .offset(*filter as libc::c_int as isize)
                }
                __res
            }) != ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *name as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = toupper(*name as libc::c_int)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_toupper_loc())
                        .offset(*name as libc::c_int as isize)
                }
                __res
            }) {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            filter = filter.offset(1);
            name = name.offset(1)
        }
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
/*
============
Com_FilterPath
============
*/
#[no_mangle]

pub unsafe extern "C" fn Com_FilterPath(
    mut filter: *mut libc::c_char,
    mut name: *mut libc::c_char,
    mut casesensitive: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut new_filter: [libc::c_char; 64] = [0; 64];
    let mut new_name: [libc::c_char; 64] = [0; 64];
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int - 1 as libc::c_int && *filter.offset(i as isize) as libc::c_int != 0
    {
        if *filter.offset(i as isize) as libc::c_int == '\\' as i32
            || *filter.offset(i as isize) as libc::c_int == ':' as i32
        {
            new_filter[i as usize] = '/' as i32 as libc::c_char
        } else {
            new_filter[i as usize] = *filter.offset(i as isize)
        }
        i += 1
    }
    new_filter[i as usize] = '\u{0}' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int - 1 as libc::c_int && *name.offset(i as isize) as libc::c_int != 0 {
        if *name.offset(i as isize) as libc::c_int == '\\' as i32
            || *name.offset(i as isize) as libc::c_int == ':' as i32
        {
            new_name[i as usize] = '/' as i32 as libc::c_char
        } else {
            new_name[i as usize] = *name.offset(i as isize)
        }
        i += 1
    }
    new_name[i as usize] = '\u{0}' as i32 as libc::c_char;
    return Com_Filter(
        new_filter.as_mut_ptr(),
        new_name.as_mut_ptr(),
        casesensitive,
    );
}
/*
================
Com_RealTime
================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_RealTime(
    mut qtime: *mut crate::src::qcommon::q_shared::qtime_t,
) -> libc::c_int {
    let mut t: crate::stdlib::time_t = 0;
    let mut tms: *mut ::libc::tm = 0 as *mut ::libc::tm;
    t = ::libc::time(0 as *mut crate::stdlib::time_t);
    if qtime.is_null() {
        return t as libc::c_int;
    }
    tms = ::libc::localtime(&mut t) as *mut ::libc::tm;
    if !tms.is_null() {
        (*qtime).tm_sec = (*tms).tm_sec;
        (*qtime).tm_min = (*tms).tm_min;
        (*qtime).tm_hour = (*tms).tm_hour;
        (*qtime).tm_mday = (*tms).tm_mday;
        (*qtime).tm_mon = (*tms).tm_mon;
        (*qtime).tm_year = (*tms).tm_year;
        (*qtime).tm_wday = (*tms).tm_wday;
        (*qtime).tm_yday = (*tms).tm_yday;
        (*qtime).tm_isdst = (*tms).tm_isdst
    }
    return t as libc::c_int;
}
// main zone for all "dynamic" memory allocation

static mut mainzone: *mut memzone_t = 0 as *const memzone_t as *mut memzone_t;
// we also have a small zone for small allocations that would only
// fragment the main zone (think of cvar and cmd strings)

static mut smallzone: *mut memzone_t = 0 as *const memzone_t as *mut memzone_t;
/*
========================
Z_ClearZone
========================
*/

unsafe extern "C" fn Z_ClearZone(mut zone: *mut memzone_t, mut size: libc::c_int) {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    // set the entire zone to one free block
    block = (zone as *mut crate::src::qcommon::q_shared::byte)
        .offset(::std::mem::size_of::<memzone_t>() as libc::c_ulong as isize)
        as *mut memblock_t; // in use block
    (*zone).blocklist.prev = block; // free block
    (*zone).blocklist.next = (*zone).blocklist.prev;
    (*zone).blocklist.tag = 1 as libc::c_int;
    (*zone).blocklist.id = 0 as libc::c_int;
    (*zone).blocklist.size = 0 as libc::c_int;
    (*zone).rover = block;
    (*zone).size = size;
    (*zone).used = 0 as libc::c_int;
    (*block).next = &mut (*zone).blocklist;
    (*block).prev = (*block).next;
    (*block).tag = 0 as libc::c_int;
    (*block).id = 0x1d4a11 as libc::c_int;
    (*block).size = (size as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<memzone_t>() as libc::c_ulong)
        as libc::c_int;
}
/*
========================
Z_AvailableZoneMemory
========================
*/

unsafe extern "C" fn Z_AvailableZoneMemory(mut zone: *mut memzone_t) -> libc::c_int {
    return (*zone).size - (*zone).used;
}
/*
========================
Z_AvailableMemory
========================
*/
#[no_mangle]

pub unsafe extern "C" fn Z_AvailableMemory() -> libc::c_int {
    return Z_AvailableZoneMemory(mainzone);
}
// NOT 0 filled memory only for small allocations
/*
========================
Z_Free
========================
*/
#[no_mangle]

pub unsafe extern "C" fn Z_Free(mut ptr: *mut libc::c_void) {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    let mut other: *mut memblock_t = 0 as *mut memblock_t;
    let mut zone: *mut memzone_t = 0 as *mut memzone_t;
    if ptr.is_null() {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Z_Free: NULL pointer\x00" as *const u8 as *const libc::c_char,
        );
    }
    block = (ptr as *mut crate::src::qcommon::q_shared::byte)
        .offset(-(::std::mem::size_of::<memblock_t>() as libc::c_ulong as isize))
        as *mut memblock_t;
    if (*block).id != 0x1d4a11 as libc::c_int {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Z_Free: freed a pointer without ZONEID\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*block).tag == 0 as libc::c_int {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Z_Free: freed a freed pointer\x00" as *const u8 as *const libc::c_char,
        );
    }
    // if static memory
    if (*block).tag == crate::qcommon_h::TAG_STATIC as libc::c_int {
        return;
    }
    // check the memory trash tester
    if *((block as *mut crate::src::qcommon::q_shared::byte)
        .offset((*block).size as isize)
        .offset(-(4 as libc::c_int as isize)) as *mut libc::c_int)
        != 0x1d4a11 as libc::c_int
    {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Z_Free: memory block wrote past end\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*block).tag == crate::qcommon_h::TAG_SMALL as libc::c_int {
        zone = smallzone
    } else {
        zone = mainzone
    }
    (*zone).used -= (*block).size;
    // set the block to something that should cause problems
    // if it is referenced...
    crate::stdlib::memset(
        ptr,
        0xaa as libc::c_int,
        ((*block).size as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<memblock_t>() as libc::c_ulong),
    ); // mark as free
    (*block).tag = 0 as libc::c_int;
    other = (*block).prev;
    if (*other).tag == 0 {
        // merge with previous free block
        (*other).size += (*block).size;
        (*other).next = (*block).next;
        (*(*other).next).prev = other;
        if block == (*zone).rover {
            (*zone).rover = other
        }
        block = other
    }
    (*zone).rover = block;
    other = (*block).next;
    if (*other).tag == 0 {
        // merge the next free block onto the end
        (*block).size += (*other).size;
        (*block).next = (*other).next;
        (*(*block).next).prev = block
    };
}
/*
================
Z_FreeTags
================
*/
#[no_mangle]

pub unsafe extern "C" fn Z_FreeTags(mut tag: libc::c_int) {
    let mut zone: *mut memzone_t = 0 as *mut memzone_t;
    if tag == crate::qcommon_h::TAG_SMALL as libc::c_int {
        zone = smallzone
    } else {
        zone = mainzone
    }
    // use the rover as our pointer, because
    // Z_Free automatically adjusts it
    (*zone).rover = (*zone).blocklist.next;
    loop {
        if (*(*zone).rover).tag == tag {
            Z_Free((*zone).rover.offset(1 as libc::c_int as isize) as *mut libc::c_void);
        } else {
            (*zone).rover = (*(*zone).rover).next
        }
        if !((*zone).rover != &mut (*zone).blocklist as *mut memblock_t) {
            break;
        }
    }
}
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
/*
================
Z_TagMalloc
================
*/
#[no_mangle]

pub unsafe extern "C" fn Z_TagMalloc(
    mut size: libc::c_int,
    mut tag: libc::c_int,
) -> *mut libc::c_void {
    let mut extra: libc::c_int = 0;
    let mut start: *mut memblock_t = 0 as *mut memblock_t;
    let mut rover: *mut memblock_t = 0 as *mut memblock_t;
    let mut new: *mut memblock_t = 0 as *mut memblock_t;
    let mut base: *mut memblock_t = 0 as *mut memblock_t;
    let mut zone: *mut memzone_t = 0 as *mut memzone_t;
    if tag == 0 {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Z_TagMalloc: tried to use a 0 tag\x00" as *const u8 as *const libc::c_char,
        );
    }
    if tag == crate::qcommon_h::TAG_SMALL as libc::c_int {
        zone = smallzone
    } else {
        zone = mainzone
    }
    //
    // scan through the block list looking for the first free block
    // of sufficient size
    //
    size = (size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<memblock_t>() as libc::c_ulong)
        as libc::c_int as libc::c_int; // account for size of block header
    size += 4 as libc::c_int; // space for memory trash tester
    size = ((size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<crate::stdlib::intptr_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & !(::std::mem::size_of::<crate::stdlib::intptr_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int; // align to 32/64 bit boundary
    rover = (*zone).rover;
    base = rover;
    start = (*base).prev;
    loop {
        if rover == start {
            // scaned all the way around the list
            Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                b"Z_Malloc: failed on allocation of %i bytes from the %s zone\x00" as *const u8
                    as *const libc::c_char,
                size,
                if zone == smallzone {
                    b"small\x00" as *const u8 as *const libc::c_char
                } else {
                    b"main\x00" as *const u8 as *const libc::c_char
                },
            );
        }
        if (*rover).tag != 0 {
            rover = (*rover).next;
            base = rover
        } else {
            rover = (*rover).next
        }
        if !((*base).tag != 0 || (*base).size < size) {
            break;
        }
    }
    //
    // found a block big enough
    //
    extra = (*base).size - size;
    if extra > 64 as libc::c_int {
        // there will be a free fragment after the allocated block
        new = (base as *mut crate::src::qcommon::q_shared::byte).offset(size as isize)
            as *mut memblock_t; // free block
        (*new).size = extra; // no longer a free block
        (*new).tag = 0 as libc::c_int; // next allocation will start looking here
        (*new).prev = base; //
        (*new).id = 0x1d4a11 as libc::c_int;
        (*new).next = (*base).next;
        (*(*new).next).prev = new;
        (*base).next = new;
        (*base).size = size
    }
    (*base).tag = tag;
    (*zone).rover = (*base).next;
    (*zone).used += (*base).size;
    (*base).id = 0x1d4a11 as libc::c_int;
    // marker for memory trash testing
    *((base as *mut crate::src::qcommon::q_shared::byte)
        .offset((*base).size as isize)
        .offset(-(4 as libc::c_int as isize)) as *mut libc::c_int) = 0x1d4a11 as libc::c_int;
    return (base as *mut crate::src::qcommon::q_shared::byte)
        .offset(::std::mem::size_of::<memblock_t>() as libc::c_ulong as isize)
        as *mut libc::c_void;
}
// NOT 0 filled memory
/*
========================
Z_Malloc
========================
*/
#[no_mangle]

pub unsafe extern "C" fn Z_Malloc(mut size: libc::c_int) -> *mut libc::c_void {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    //Z_CheckHeap ();	// DEBUG
    buf = Z_TagMalloc(size, crate::qcommon_h::TAG_GENERAL as libc::c_int);
    crate::stdlib::memset(buf, 0 as libc::c_int, size as libc::c_ulong);
    return buf;
}
// returns 0 filled memory
#[no_mangle]

pub unsafe extern "C" fn S_Malloc(mut size: libc::c_int) -> *mut libc::c_void {
    return Z_TagMalloc(size, crate::qcommon_h::TAG_SMALL as libc::c_int);
}
/*
========================
Z_CheckHeap
========================
*/

unsafe extern "C" fn Z_CheckHeap() {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    block = (*mainzone).blocklist.next;
    while !((*block).next == &mut (*mainzone).blocklist as *mut memblock_t) {
        if (block as *mut crate::src::qcommon::q_shared::byte).offset((*block).size as isize)
            != (*block).next as *mut crate::src::qcommon::q_shared::byte
        {
            Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                b"Z_CheckHeap: block size does not touch the next block\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*(*block).next).prev != block {
            Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                b"Z_CheckHeap: next block doesn\'t have proper back link\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*block).tag == 0 && (*(*block).next).tag == 0 {
            Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                b"Z_CheckHeap: two consecutive free blocks\x00" as *const u8 as *const libc::c_char,
            );
        }
        block = (*block).next
    }
}
/*
========================
Z_LogZoneHeap
========================
*/
#[no_mangle]

pub unsafe extern "C" fn Z_LogZoneHeap(mut zone: *mut memzone_t, mut name: *mut libc::c_char) {
    let mut block: *mut memblock_t = 0 as *mut memblock_t; // + 32 bit alignment
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut size: libc::c_int = 0;
    let mut allocSize: libc::c_int = 0;
    let mut numBlocks: libc::c_int = 0;
    if logfile == 0 || crate::src::qcommon::files::FS_Initialized() as u64 == 0 {
        return;
    }
    numBlocks = 0 as libc::c_int;
    size = numBlocks;
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"\r\n================\r\n%s log\r\n================\r\n\x00" as *const u8
            as *const libc::c_char,
        name,
    );
    crate::src::qcommon::files::FS_Write(
        buf.as_mut_ptr() as *const libc::c_void,
        crate::stdlib::strlen(buf.as_mut_ptr()) as libc::c_int,
        logfile,
    );
    block = (*zone).blocklist.next;
    while (*block).next != &mut (*zone).blocklist as *mut memblock_t {
        if (*block).tag != 0 {
            size += (*block).size;
            numBlocks += 1
        }
        block = (*block).next
    }
    allocSize = (numBlocks as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<memblock_t>() as libc::c_ulong)
        as libc::c_int;
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"%d %s memory in %d blocks\r\n\x00" as *const u8 as *const libc::c_char,
        size,
        name,
        numBlocks,
    );
    crate::src::qcommon::files::FS_Write(
        buf.as_mut_ptr() as *const libc::c_void,
        crate::stdlib::strlen(buf.as_mut_ptr()) as libc::c_int,
        logfile,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"%d %s memory overhead\r\n\x00" as *const u8 as *const libc::c_char,
        size - allocSize,
        name,
    );
    crate::src::qcommon::files::FS_Write(
        buf.as_mut_ptr() as *const libc::c_void,
        crate::stdlib::strlen(buf.as_mut_ptr()) as libc::c_int,
        logfile,
    );
}
/*
========================
Z_LogHeap
========================
*/
#[no_mangle]

pub unsafe extern "C" fn Z_LogHeap() {
    Z_LogZoneHeap(
        mainzone,
        b"MAIN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Z_LogZoneHeap(
        smallzone,
        b"SMALL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
// Initialized in run_static_initializers
#[no_mangle]

pub static mut emptystring: memstatic_t = memstatic_t {
    b: memblock_t {
        size: 0,
        tag: 0,
        next: 0 as *const memblock_s as *mut memblock_s,
        prev: 0 as *const memblock_s as *mut memblock_s,
        id: 0,
    },
    mem: [0; 2],
};
// Initialized in run_static_initializers
#[no_mangle]

pub static mut numberstring: [memstatic_t; 10] = [memstatic_t {
    b: memblock_t {
        size: 0,
        tag: 0,
        next: 0 as *const memblock_s as *mut memblock_s,
        prev: 0 as *const memblock_s as *mut memblock_s,
        id: 0,
    },
    mem: [0; 2],
}; 10];
/*
========================
CopyString

 NOTE:	never write over the memory CopyString returns because
        memory from a memstatic_t might be returned
========================
*/
#[no_mangle]

pub unsafe extern "C" fn CopyString(mut in_0: *const libc::c_char) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    if *in_0.offset(0 as libc::c_int as isize) == 0 {
        return (&mut emptystring as *mut memstatic_t as *mut libc::c_char)
            .offset(::std::mem::size_of::<memblock_t>() as libc::c_ulong as isize);
    } else {
        if *in_0.offset(1 as libc::c_int as isize) == 0 {
            if *in_0.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                && *in_0.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32
            {
                return (&mut *numberstring.as_mut_ptr().offset(
                    (*in_0.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32) as isize,
                ) as *mut memstatic_t as *mut libc::c_char)
                    .offset(::std::mem::size_of::<memblock_t>() as libc::c_ulong as isize);
            }
        }
    }
    out = S_Malloc(
        crate::stdlib::strlen(in_0).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_char;
    ::libc::strcpy(out, in_0);
    return out;
}

static mut hunkblocks: *mut hunkblock_t = 0 as *const hunkblock_t as *mut hunkblock_t;

static mut hunk_low: hunkUsed_t = hunkUsed_t {
    mark: 0,
    permanent: 0,
    temp: 0,
    tempHighwater: 0,
};

static mut hunk_high: hunkUsed_t = hunkUsed_t {
    mark: 0,
    permanent: 0,
    temp: 0,
    tempHighwater: 0,
};

static mut hunk_permanent: *mut hunkUsed_t = 0 as *const hunkUsed_t as *mut hunkUsed_t;

static mut hunk_temp: *mut hunkUsed_t = 0 as *const hunkUsed_t as *mut hunkUsed_t;

static mut s_hunkData: *mut crate::src::qcommon::q_shared::byte =
    0 as *const crate::src::qcommon::q_shared::byte as *mut crate::src::qcommon::q_shared::byte;

static mut s_hunkTotal: libc::c_int = 0;

static mut s_zoneTotal: libc::c_int = 0;

static mut s_smallZoneTotal: libc::c_int = 0;
/*
=================
Com_Meminfo_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_Meminfo_f() {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    let mut zoneBytes: libc::c_int = 0;
    let mut zoneBlocks: libc::c_int = 0;
    let mut smallZoneBytes: libc::c_int = 0;
    let mut botlibBytes: libc::c_int = 0;
    let mut rendererBytes: libc::c_int = 0;
    let mut unused: libc::c_int = 0;
    zoneBytes = 0 as libc::c_int;
    botlibBytes = 0 as libc::c_int;
    rendererBytes = 0 as libc::c_int;
    zoneBlocks = 0 as libc::c_int;
    block = (*mainzone).blocklist.next;
    loop {
        if crate::src::qcommon::cmd::Cmd_Argc() != 1 as libc::c_int {
            Com_Printf(
                b"block:%p    size:%7i    tag:%3i\n\x00" as *const u8 as *const libc::c_char,
                block as *mut libc::c_void,
                (*block).size,
                (*block).tag,
            );
        }
        if (*block).tag != 0 {
            zoneBytes += (*block).size;
            zoneBlocks += 1;
            if (*block).tag == crate::qcommon_h::TAG_BOTLIB as libc::c_int {
                botlibBytes += (*block).size
            } else if (*block).tag == crate::qcommon_h::TAG_RENDERER as libc::c_int {
                rendererBytes += (*block).size
            }
        }
        if (*block).next == &mut (*mainzone).blocklist as *mut memblock_t {
            break;
        }
        if (block as *mut crate::src::qcommon::q_shared::byte).offset((*block).size as isize)
            != (*block).next as *mut crate::src::qcommon::q_shared::byte
        {
            Com_Printf(
                b"ERROR: block size does not touch the next block\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*(*block).next).prev != block {
            Com_Printf(
                b"ERROR: next block doesn\'t have proper back link\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*block).tag == 0 && (*(*block).next).tag == 0 {
            Com_Printf(
                b"ERROR: two consecutive free blocks\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        block = (*block).next
    }
    smallZoneBytes = 0 as libc::c_int;
    block = (*smallzone).blocklist.next;
    loop {
        if (*block).tag != 0 {
            smallZoneBytes += (*block).size
        }
        if (*block).next == &mut (*smallzone).blocklist as *mut memblock_t {
            break;
        }
        block = (*block).next
    }
    Com_Printf(
        b"%8i bytes total hunk\n\x00" as *const u8 as *const libc::c_char,
        s_hunkTotal,
    );
    Com_Printf(
        b"%8i bytes total zone\n\x00" as *const u8 as *const libc::c_char,
        s_zoneTotal,
    );
    Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Com_Printf(
        b"%8i low mark\n\x00" as *const u8 as *const libc::c_char,
        hunk_low.mark,
    );
    Com_Printf(
        b"%8i low permanent\n\x00" as *const u8 as *const libc::c_char,
        hunk_low.permanent,
    );
    if hunk_low.temp != hunk_low.permanent {
        Com_Printf(
            b"%8i low temp\n\x00" as *const u8 as *const libc::c_char,
            hunk_low.temp,
        );
    }
    Com_Printf(
        b"%8i low tempHighwater\n\x00" as *const u8 as *const libc::c_char,
        hunk_low.tempHighwater,
    );
    Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Com_Printf(
        b"%8i high mark\n\x00" as *const u8 as *const libc::c_char,
        hunk_high.mark,
    );
    Com_Printf(
        b"%8i high permanent\n\x00" as *const u8 as *const libc::c_char,
        hunk_high.permanent,
    );
    if hunk_high.temp != hunk_high.permanent {
        Com_Printf(
            b"%8i high temp\n\x00" as *const u8 as *const libc::c_char,
            hunk_high.temp,
        );
    }
    Com_Printf(
        b"%8i high tempHighwater\n\x00" as *const u8 as *const libc::c_char,
        hunk_high.tempHighwater,
    );
    Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Com_Printf(
        b"%8i total hunk in use\n\x00" as *const u8 as *const libc::c_char,
        hunk_low.permanent + hunk_high.permanent,
    );
    unused = 0 as libc::c_int;
    if hunk_low.tempHighwater > hunk_low.permanent {
        unused += hunk_low.tempHighwater - hunk_low.permanent
    }
    if hunk_high.tempHighwater > hunk_high.permanent {
        unused += hunk_high.tempHighwater - hunk_high.permanent
    }
    Com_Printf(
        b"%8i unused highwater\n\x00" as *const u8 as *const libc::c_char,
        unused,
    );
    Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Com_Printf(
        b"%8i bytes in %i zone blocks\n\x00" as *const u8 as *const libc::c_char,
        zoneBytes,
        zoneBlocks,
    );
    Com_Printf(
        b"        %8i bytes in dynamic botlib\n\x00" as *const u8 as *const libc::c_char,
        botlibBytes,
    );
    Com_Printf(
        b"        %8i bytes in dynamic renderer\n\x00" as *const u8 as *const libc::c_char,
        rendererBytes,
    );
    Com_Printf(
        b"        %8i bytes in dynamic other\n\x00" as *const u8 as *const libc::c_char,
        zoneBytes - (botlibBytes + rendererBytes),
    );
    Com_Printf(
        b"        %8i bytes in small Zone memory\n\x00" as *const u8 as *const libc::c_char,
        smallZoneBytes,
    );
}
/*
===============
Com_TouchMemory

Touch all known used data to make sure it is paged in
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Com_TouchMemory() {
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sum: libc::c_uint = 0;
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    Z_CheckHeap();
    start = crate::src::sys::sys_unix::Sys_Milliseconds();
    sum = 0 as libc::c_int as libc::c_uint;
    j = hunk_low.permanent >> 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < j {
        // only need to touch each page
        sum =
            sum.wrapping_add(*(s_hunkData as *mut libc::c_int).offset(i as isize) as libc::c_uint);
        i += 64 as libc::c_int
    }
    i = s_hunkTotal - hunk_high.permanent >> 2 as libc::c_int;
    j = hunk_high.permanent >> 2 as libc::c_int;
    while i < j {
        // only need to touch each page
        sum =
            sum.wrapping_add(*(s_hunkData as *mut libc::c_int).offset(i as isize) as libc::c_uint);
        i += 64 as libc::c_int
    }
    block = (*mainzone).blocklist.next;
    loop {
        if (*block).tag != 0 {
            j = (*block).size >> 2 as libc::c_int;
            i = 0 as libc::c_int;
            while i < j {
                // only need to touch each page
                sum = sum
                    .wrapping_add(*(block as *mut libc::c_int).offset(i as isize) as libc::c_uint);
                i += 64 as libc::c_int
            }
        }
        if (*block).next == &mut (*mainzone).blocklist as *mut memblock_t {
            break;
        }
        block = (*block).next
    }
    end = crate::src::sys::sys_unix::Sys_Milliseconds();
    Com_Printf(
        b"Com_TouchMemory: %i msec\n\x00" as *const u8 as *const libc::c_char,
        end - start,
    );
}
/*
=================
Com_InitZoneMemory
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_InitSmallZoneMemory() {
    s_smallZoneTotal = 512 as libc::c_int * 1024 as libc::c_int;
    smallzone = crate::stdlib::calloc(
        s_smallZoneTotal as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut memzone_t;
    if smallzone.is_null() {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Small zone data failed to allocate %1.1f megs\x00" as *const u8
                as *const libc::c_char,
            (s_smallZoneTotal as libc::c_float
                / (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_float)
                as libc::c_double,
        );
    }
    Z_ClearZone(smallzone, s_smallZoneTotal);
}
#[no_mangle]

pub unsafe extern "C" fn Com_InitZoneMemory() {
    let mut cv: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    // Please note: com_zoneMegs can only be set on the command line, and
    // not in q3config.cfg or Com_StartupVariable, as they haven't been
    // executed by this point. It's a chicken and egg problem. We need the
    // memory manager configured to handle those places where you would
    // configure the memory manager.
    // allocate the random block zone
    cv = crate::src::qcommon::cvar::Cvar_Get(
        b"com_zoneMegs\x00" as *const u8 as *const libc::c_char,
        b"24\x00" as *const u8 as *const libc::c_char,
        0x20 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    if (*cv).integer < 24 as libc::c_int {
        s_zoneTotal = 1024 as libc::c_int * 1024 as libc::c_int * 24 as libc::c_int
    } else {
        s_zoneTotal = (*cv).integer * 1024 as libc::c_int * 1024 as libc::c_int
    }
    mainzone = crate::stdlib::calloc(
        s_zoneTotal as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut memzone_t;
    if mainzone.is_null() {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Zone data failed to allocate %i megs\x00" as *const u8 as *const libc::c_char,
            s_zoneTotal / (1024 as libc::c_int * 1024 as libc::c_int),
        );
    }
    Z_ClearZone(mainzone, s_zoneTotal);
}
/*
=================
Hunk_Log
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Hunk_Log() {
    let mut block: *mut hunkblock_t = 0 as *mut hunkblock_t;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut size: libc::c_int = 0;
    let mut numBlocks: libc::c_int = 0;
    if logfile == 0 || crate::src::qcommon::files::FS_Initialized() as u64 == 0 {
        return;
    }
    size = 0 as libc::c_int;
    numBlocks = 0 as libc::c_int;
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"\r\n================\r\nHunk log\r\n================\r\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::src::qcommon::files::FS_Write(
        buf.as_mut_ptr() as *const libc::c_void,
        crate::stdlib::strlen(buf.as_mut_ptr()) as libc::c_int,
        logfile,
    );
    block = hunkblocks;
    while !block.is_null() {
        size += (*block).size;
        numBlocks += 1;
        block = (*block).next
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"%d Hunk memory\r\n\x00" as *const u8 as *const libc::c_char,
        size,
    );
    crate::src::qcommon::files::FS_Write(
        buf.as_mut_ptr() as *const libc::c_void,
        crate::stdlib::strlen(buf.as_mut_ptr()) as libc::c_int,
        logfile,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"%d hunk blocks\r\n\x00" as *const u8 as *const libc::c_char,
        numBlocks,
    );
    crate::src::qcommon::files::FS_Write(
        buf.as_mut_ptr() as *const libc::c_void,
        crate::stdlib::strlen(buf.as_mut_ptr()) as libc::c_int,
        logfile,
    );
}
/*
=================
Hunk_SmallLog
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Hunk_SmallLog() {
    let mut block: *mut hunkblock_t = 0 as *mut hunkblock_t;
    let mut block2: *mut hunkblock_t = 0 as *mut hunkblock_t;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut size: libc::c_int = 0;
    let mut locsize: libc::c_int = 0;
    let mut numBlocks: libc::c_int = 0;
    if logfile == 0 || crate::src::qcommon::files::FS_Initialized() as u64 == 0 {
        return;
    }
    block = hunkblocks;
    while !block.is_null() {
        (*block).printed = crate::src::qcommon::q_shared::qfalse as libc::c_int
            as crate::src::qcommon::q_shared::byte;
        block = (*block).next
    }
    size = 0 as libc::c_int;
    numBlocks = 0 as libc::c_int;
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"\r\n================\r\nHunk Small log\r\n================\r\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::src::qcommon::files::FS_Write(
        buf.as_mut_ptr() as *const libc::c_void,
        crate::stdlib::strlen(buf.as_mut_ptr()) as libc::c_int,
        logfile,
    );
    block = hunkblocks;
    while !block.is_null() {
        if !((*block).printed != 0) {
            locsize = (*block).size;
            block2 = (*block).next;
            while !block2.is_null() {
                if !((*block).line != (*block2).line) {
                    if !(crate::src::qcommon::q_shared::Q_stricmp((*block).file, (*block2).file)
                        != 0)
                    {
                        size += (*block2).size;
                        locsize += (*block2).size;
                        (*block2).printed = crate::src::qcommon::q_shared::qtrue as libc::c_int
                            as crate::src::qcommon::q_shared::byte
                    }
                }
                block2 = (*block2).next
            }
            size += (*block).size;
            numBlocks += 1
        }
        block = (*block).next
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"%d Hunk memory\r\n\x00" as *const u8 as *const libc::c_char,
        size,
    );
    crate::src::qcommon::files::FS_Write(
        buf.as_mut_ptr() as *const libc::c_void,
        crate::stdlib::strlen(buf.as_mut_ptr()) as libc::c_int,
        logfile,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"%d hunk blocks\r\n\x00" as *const u8 as *const libc::c_char,
        numBlocks,
    );
    crate::src::qcommon::files::FS_Write(
        buf.as_mut_ptr() as *const libc::c_void,
        crate::stdlib::strlen(buf.as_mut_ptr()) as libc::c_int,
        logfile,
    );
}
/*
=================
Com_InitHunkZoneMemory
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_InitHunkMemory() {
    let mut cv: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    let mut nMinAlloc: libc::c_int = 0;
    let mut pMsg: *mut libc::c_char = 0 as *mut libc::c_char;
    // make sure the file system has allocated and "not" freed any temp blocks
    // this allows the config and product id files ( journal files too ) to be loaded
    // by the file system without redunant routines in the file system utilizing different
    // memory systems
    if crate::src::qcommon::files::FS_LoadStack() != 0 as libc::c_int {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Hunk initialization failed. File system load stack not zero\x00" as *const u8
                as *const libc::c_char,
        );
    }
    // allocate the stack based hunk allocator
    cv = crate::src::qcommon::cvar::Cvar_Get(
        b"com_hunkMegs\x00" as *const u8 as *const libc::c_char,
        b"128\x00" as *const u8 as *const libc::c_char,
        0x20 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::qcommon::cvar::Cvar_SetDescription(
        cv as *mut crate::src::qcommon::q_shared::cvar_s,
        b"The size of the hunk memory segment\x00" as *const u8 as *const libc::c_char,
    );
    // if we are not dedicated min allocation is 56, otherwise min is 1
    if !com_dedicated.is_null() && (*com_dedicated).integer != 0 {
        nMinAlloc = 1 as libc::c_int;
        pMsg = b"Minimum com_hunkMegs for a dedicated server is %i, allocating %i megs.\n\x00"
            as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        nMinAlloc = 56 as libc::c_int;
        pMsg = b"Minimum com_hunkMegs is %i, allocating %i megs.\n\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char
    }
    if (*cv).integer < nMinAlloc {
        s_hunkTotal = 1024 as libc::c_int * 1024 as libc::c_int * nMinAlloc;
        Com_Printf(
            pMsg,
            nMinAlloc,
            s_hunkTotal / (1024 as libc::c_int * 1024 as libc::c_int),
        );
    } else {
        s_hunkTotal = (*cv).integer * 1024 as libc::c_int * 1024 as libc::c_int
    }
    s_hunkData = crate::stdlib::calloc(
        (s_hunkTotal + 31 as libc::c_int) as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut crate::src::qcommon::q_shared::byte;
    if s_hunkData.is_null() {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Hunk data failed to allocate %i megs\x00" as *const u8 as *const libc::c_char,
            s_hunkTotal / (1024 as libc::c_int * 1024 as libc::c_int),
        );
    }
    // cacheline align
    s_hunkData = (s_hunkData as crate::stdlib::intptr_t + 31 as libc::c_int as libc::c_long
        & !(31 as libc::c_int) as libc::c_long)
        as *mut crate::src::qcommon::q_shared::byte;
    Hunk_Clear();
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"meminfo\x00" as *const u8 as *const libc::c_char,
        Some(Com_Meminfo_f as unsafe extern "C" fn() -> ()),
    );
}
/*
====================
Hunk_MemoryRemaining
====================
*/
#[no_mangle]

pub unsafe extern "C" fn Hunk_MemoryRemaining() -> libc::c_int {
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    low = if hunk_low.permanent > hunk_low.temp {
        hunk_low.permanent
    } else {
        hunk_low.temp
    };
    high = if hunk_high.permanent > hunk_high.temp {
        hunk_high.permanent
    } else {
        hunk_high.temp
    };
    return s_hunkTotal - (low + high);
}
/*
===================
Hunk_SetMark

The server calls this after the level and game VM have been loaded
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Hunk_SetMark() {
    hunk_low.mark = hunk_low.permanent;
    hunk_high.mark = hunk_high.permanent;
}
/*
=================
Hunk_ClearToMark

The client calls this before starting a vid_restart or snd_restart
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Hunk_ClearToMark() {
    hunk_low.temp = hunk_low.mark;
    hunk_low.permanent = hunk_low.temp;
    hunk_high.temp = hunk_high.mark;
    hunk_high.permanent = hunk_high.temp;
}
/*
=================
Hunk_CheckMark
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Hunk_CheckMark() -> crate::src::qcommon::q_shared::qboolean {
    if hunk_low.mark != 0 || hunk_high.mark != 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
=================
Hunk_Clear

The server calls this before shutting down or loading a new map
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Hunk_Clear() {
    CL_ShutdownCGame();
    CL_ShutdownUI();
    SV_ShutdownGameProgs();
    CIN_CloseAllVideos();
    hunk_low.mark = 0 as libc::c_int;
    hunk_low.permanent = 0 as libc::c_int;
    hunk_low.temp = 0 as libc::c_int;
    hunk_low.tempHighwater = 0 as libc::c_int;
    hunk_high.mark = 0 as libc::c_int;
    hunk_high.permanent = 0 as libc::c_int;
    hunk_high.temp = 0 as libc::c_int;
    hunk_high.tempHighwater = 0 as libc::c_int;
    hunk_permanent = &mut hunk_low;
    hunk_temp = &mut hunk_high;
    Com_Printf(b"Hunk_Clear: reset the hunk ok\n\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::vm::VM_Clear();
}

unsafe extern "C" fn Hunk_SwapBanks() {
    let mut swap: *mut hunkUsed_t = 0 as *mut hunkUsed_t;
    // can't swap banks if there is any temp already allocated
    if (*hunk_temp).temp != (*hunk_temp).permanent {
        return;
    }
    // if we have a larger highwater mark on this side, start making
    // our permanent allocations here and use the other side for temp
    if (*hunk_temp).tempHighwater - (*hunk_temp).permanent
        > (*hunk_permanent).tempHighwater - (*hunk_permanent).permanent
    {
        swap = hunk_temp;
        hunk_temp = hunk_permanent;
        hunk_permanent = swap
    };
}
/*
=================
Hunk_Alloc

Allocate permanent (until the hunk is cleared) memory
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Hunk_Alloc(
    mut size: libc::c_int,
    mut preference: crate::src::qcommon::q_shared::ha_pref,
) -> *mut libc::c_void {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    if s_hunkData.is_null() {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Hunk_Alloc: Hunk memory system not initialized\x00" as *const u8
                as *const libc::c_char,
        );
    }
    // can't do preference if there is any temp allocated
    if preference as libc::c_uint
        == crate::src::qcommon::q_shared::h_dontcare as libc::c_int as libc::c_uint
        || (*hunk_temp).temp != (*hunk_temp).permanent
    {
        Hunk_SwapBanks();
    } else if preference as libc::c_uint
        == crate::src::qcommon::q_shared::h_low as libc::c_int as libc::c_uint
        && hunk_permanent != &mut hunk_low as *mut hunkUsed_t
    {
        Hunk_SwapBanks();
    } else if preference as libc::c_uint
        == crate::src::qcommon::q_shared::h_high as libc::c_int as libc::c_uint
        && hunk_permanent != &mut hunk_high as *mut hunkUsed_t
    {
        Hunk_SwapBanks();
    }
    // round to cacheline
    size = size + 31 as libc::c_int & !(31 as libc::c_int);
    if hunk_low.temp + hunk_high.temp + size > s_hunkTotal {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Hunk_Alloc failed on %i\x00" as *const u8 as *const libc::c_char,
            size,
        );
    }
    if hunk_permanent == &mut hunk_low as *mut hunkUsed_t {
        buf = s_hunkData.offset((*hunk_permanent).permanent as isize) as *mut libc::c_void;
        (*hunk_permanent).permanent += size
    } else {
        (*hunk_permanent).permanent += size;
        buf = s_hunkData
            .offset(s_hunkTotal as isize)
            .offset(-((*hunk_permanent).permanent as isize)) as *mut libc::c_void
    }
    (*hunk_permanent).temp = (*hunk_permanent).permanent;
    crate::stdlib::memset(buf, 0 as libc::c_int, size as libc::c_ulong);
    return buf;
}
/*
=================
Hunk_AllocateTempMemory

This is used by the file loading system.
Multiple files can be loaded in temporary memory.
When the files-in-use count reaches zero, all temp memory will be deleted
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Hunk_AllocateTempMemory(mut size: libc::c_int) -> *mut libc::c_void {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut hdr: *mut hunkHeader_t = 0 as *mut hunkHeader_t;
    // return a Z_Malloc'd block if the hunk has not been initialized
    // this allows the config and product id files ( journal files too ) to be loaded
    // by the file system without redunant routines in the file system utilizing different
    // memory systems
    if s_hunkData.is_null() {
        return Z_Malloc(size);
    }
    Hunk_SwapBanks();
    size = ((size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<crate::stdlib::intptr_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & !(::std::mem::size_of::<crate::stdlib::intptr_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
    .wrapping_add(::std::mem::size_of::<hunkHeader_t>() as libc::c_ulong) as libc::c_int;
    if (*hunk_temp).temp + (*hunk_permanent).permanent + size > s_hunkTotal {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Hunk_AllocateTempMemory: failed on %i\x00" as *const u8 as *const libc::c_char,
            size,
        );
    }
    if hunk_temp == &mut hunk_low as *mut hunkUsed_t {
        buf = s_hunkData.offset((*hunk_temp).temp as isize) as *mut libc::c_void;
        (*hunk_temp).temp += size
    } else {
        (*hunk_temp).temp += size;
        buf = s_hunkData
            .offset(s_hunkTotal as isize)
            .offset(-((*hunk_temp).temp as isize)) as *mut libc::c_void
    }
    if (*hunk_temp).temp > (*hunk_temp).tempHighwater {
        (*hunk_temp).tempHighwater = (*hunk_temp).temp
    }
    hdr = buf as *mut hunkHeader_t;
    buf = hdr.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    (*hdr).magic = 0x89537892 as libc::c_uint as libc::c_int;
    (*hdr).size = size;
    // don't bother clearing, because we are going to load a file over it
    return buf;
}
/*
==================
Hunk_FreeTempMemory
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Hunk_FreeTempMemory(mut buf: *mut libc::c_void) {
    let mut hdr: *mut hunkHeader_t = 0 as *mut hunkHeader_t;
    // free with Z_Free if the hunk has not been initialized
    // this allows the config and product id files ( journal files too ) to be loaded
    // by the file system without redunant routines in the file system utilizing different
    // memory systems
    if s_hunkData.is_null() {
        Z_Free(buf);
        return;
    }
    hdr = (buf as *mut hunkHeader_t).offset(-(1 as libc::c_int as isize));
    if (*hdr).magic as libc::c_uint != 0x89537892 as libc::c_uint {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Hunk_FreeTempMemory: bad magic\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*hdr).magic = 0x89537893 as libc::c_uint as libc::c_int;
    // this only works if the files are freed in stack order,
    // otherwise the memory will stay around until Hunk_ClearTempMemory
    if hunk_temp == &mut hunk_low as *mut hunkUsed_t {
        if hdr
            == s_hunkData
                .offset((*hunk_temp).temp as isize)
                .offset(-((*hdr).size as isize)) as *mut libc::c_void
                as *mut hunkHeader_t
        {
            (*hunk_temp).temp -= (*hdr).size
        } else {
            Com_Printf(
                b"Hunk_FreeTempMemory: not the final block\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if hdr
        == s_hunkData
            .offset(s_hunkTotal as isize)
            .offset(-((*hunk_temp).temp as isize)) as *mut libc::c_void
            as *mut hunkHeader_t
    {
        (*hunk_temp).temp -= (*hdr).size
    } else {
        Com_Printf(
            b"Hunk_FreeTempMemory: not the final block\n\x00" as *const u8 as *const libc::c_char,
        );
    };
}
/*
=================
Hunk_ClearTempMemory

The temp space is no longer needed.  If we have left more
touched but unused memory on this side, have future
permanent allocs use this side.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Hunk_ClearTempMemory() {
    if !s_hunkData.is_null() {
        (*hunk_temp).temp = (*hunk_temp).permanent
    };
}

static mut com_pushedEventsHead: libc::c_int = 0 as libc::c_int;

static mut com_pushedEventsTail: libc::c_int = 0 as libc::c_int;

static mut com_pushedEvents: [crate::qcommon_h::sysEvent_t; 1024] = [crate::qcommon_h::sysEvent_t {
    evTime: 0,
    evType: crate::qcommon_h::SE_NONE,
    evValue: 0,
    evValue2: 0,
    evPtrLength: 0,
    evPtr: 0 as *const libc::c_void as *mut libc::c_void,
}; 1024];
/*
=================
Com_InitJournaling
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_InitJournaling() {
    Com_StartupVariable(b"journal\x00" as *const u8 as *const libc::c_char);
    com_journal = crate::src::qcommon::cvar::Cvar_Get(
        b"journal\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x10 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    if (*com_journal).integer == 0 {
        return;
    }
    if (*com_journal).integer == 1 as libc::c_int {
        Com_Printf(b"Journaling events\n\x00" as *const u8 as *const libc::c_char);
        com_journalFile = crate::src::qcommon::files::FS_FOpenFileWrite(
            b"journal.dat\x00" as *const u8 as *const libc::c_char,
        );
        com_journalDataFile = crate::src::qcommon::files::FS_FOpenFileWrite(
            b"journaldata.dat\x00" as *const u8 as *const libc::c_char,
        )
    } else if (*com_journal).integer == 2 as libc::c_int {
        Com_Printf(b"Replaying journaled events\n\x00" as *const u8 as *const libc::c_char);
        crate::src::qcommon::files::FS_FOpenFileRead(
            b"journal.dat\x00" as *const u8 as *const libc::c_char,
            &mut com_journalFile,
            crate::src::qcommon::q_shared::qtrue,
        );
        crate::src::qcommon::files::FS_FOpenFileRead(
            b"journaldata.dat\x00" as *const u8 as *const libc::c_char,
            &mut com_journalDataFile,
            crate::src::qcommon::q_shared::qtrue,
        );
    }
    if com_journalFile == 0 || com_journalDataFile == 0 {
        crate::src::qcommon::cvar::Cvar_Set(
            b"com_journal\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        com_journalFile = 0 as libc::c_int;
        com_journalDataFile = 0 as libc::c_int;
        Com_Printf(b"Couldn\'t open journal files\n\x00" as *const u8 as *const libc::c_char);
    };
}

static mut eventQueue: [crate::qcommon_h::sysEvent_t; 256] = [crate::qcommon_h::sysEvent_t {
    evTime: 0,
    evType: crate::qcommon_h::SE_NONE,
    evValue: 0,
    evValue2: 0,
    evPtrLength: 0,
    evPtr: 0 as *const libc::c_void as *mut libc::c_void,
}; 256];

static mut eventHead: libc::c_int = 0 as libc::c_int;

static mut eventTail: libc::c_int = 0 as libc::c_int;
/*
================
Com_QueueEvent

A time of 0 will get the current time
Ptr should either be null, or point to a block of data that can
be freed by the game later.
================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_QueueEvent(
    mut time_0: libc::c_int,
    mut type_0: crate::qcommon_h::sysEventType_t,
    mut value: libc::c_int,
    mut value2: libc::c_int,
    mut ptrLength: libc::c_int,
    mut ptr: *mut libc::c_void,
) {
    let mut ev: *mut crate::qcommon_h::sysEvent_t = 0 as *mut crate::qcommon_h::sysEvent_t;
    // combine mouse movement with previous mouse event
    if type_0 as libc::c_uint == crate::qcommon_h::SE_MOUSE as libc::c_int as libc::c_uint
        && eventHead != eventTail
    {
        ev = &mut *eventQueue.as_mut_ptr().offset(
            (eventHead + 256 as libc::c_int - 1 as libc::c_int
                & 256 as libc::c_int - 1 as libc::c_int) as isize,
        ) as *mut crate::qcommon_h::sysEvent_t;
        if (*ev).evType as libc::c_uint == crate::qcommon_h::SE_MOUSE as libc::c_int as libc::c_uint
        {
            (*ev).evValue += value;
            (*ev).evValue2 += value2;
            return;
        }
    }
    ev = &mut *eventQueue
        .as_mut_ptr()
        .offset((eventHead & 256 as libc::c_int - 1 as libc::c_int) as isize)
        as *mut crate::qcommon_h::sysEvent_t;
    if eventHead - eventTail >= 256 as libc::c_int {
        Com_Printf(b"Com_QueueEvent: overflow\n\x00" as *const u8 as *const libc::c_char);
        // we are discarding an event, but don't leak memory
        if !(*ev).evPtr.is_null() {
            Z_Free((*ev).evPtr);
        }
        eventTail += 1
    }
    eventHead += 1;
    if time_0 == 0 as libc::c_int {
        time_0 = crate::src::sys::sys_unix::Sys_Milliseconds()
    }
    (*ev).evTime = time_0;
    (*ev).evType = type_0;
    (*ev).evValue = value;
    (*ev).evValue2 = value2;
    (*ev).evPtrLength = ptrLength;
    (*ev).evPtr = ptr;
}
/*
================
Com_GetSystemEvent

================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_GetSystemEvent() -> crate::qcommon_h::sysEvent_t {
    let mut ev: crate::qcommon_h::sysEvent_t = crate::qcommon_h::sysEvent_t {
        evTime: 0,
        evType: crate::qcommon_h::SE_NONE,
        evValue: 0,
        evValue2: 0,
        evPtrLength: 0,
        evPtr: 0 as *const libc::c_void as *mut libc::c_void,
    };
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    // return if we have data
    if eventHead > eventTail {
        eventTail += 1;
        return eventQueue
            [(eventTail - 1 as libc::c_int & 256 as libc::c_int - 1 as libc::c_int) as usize];
    }
    // check for console commands
    s = crate::src::sys::sys_main::Sys_ConsoleInput();
    if !s.is_null() {
        let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: libc::c_int = 0;
        len =
            crate::stdlib::strlen(s).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        b = Z_Malloc(len) as *mut libc::c_char;
        ::libc::strcpy(b, s);
        Com_QueueEvent(
            0 as libc::c_int,
            crate::qcommon_h::SE_CONSOLE,
            0 as libc::c_int,
            0 as libc::c_int,
            len,
            b as *mut libc::c_void,
        );
    }
    // return if we have data
    if eventHead > eventTail {
        eventTail += 1;
        return eventQueue
            [(eventTail - 1 as libc::c_int & 256 as libc::c_int - 1 as libc::c_int) as usize];
    }
    // create an empty event to return
    crate::stdlib::memset(
        &mut ev as *mut crate::qcommon_h::sysEvent_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::qcommon_h::sysEvent_t>() as libc::c_ulong,
    );
    ev.evTime = crate::src::sys::sys_unix::Sys_Milliseconds();
    return ev;
}
/*
=================
Com_GetRealEvent
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_GetRealEvent() -> crate::qcommon_h::sysEvent_t {
    let mut r: libc::c_int = 0;
    let mut ev: crate::qcommon_h::sysEvent_t = crate::qcommon_h::sysEvent_t {
        evTime: 0,
        evType: crate::qcommon_h::SE_NONE,
        evValue: 0,
        evValue2: 0,
        evPtrLength: 0,
        evPtr: 0 as *const libc::c_void as *mut libc::c_void,
    };
    // either get an event from the system or the journal file
    if (*com_journal).integer == 2 as libc::c_int {
        r = crate::src::qcommon::files::FS_Read(
            &mut ev as *mut crate::qcommon_h::sysEvent_t as *mut libc::c_void,
            ::std::mem::size_of::<crate::qcommon_h::sysEvent_t>() as libc::c_ulong as libc::c_int,
            com_journalFile,
        );
        if r as libc::c_ulong
            != ::std::mem::size_of::<crate::qcommon_h::sysEvent_t>() as libc::c_ulong
        {
            Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                b"Error reading from journal file\x00" as *const u8 as *const libc::c_char,
            );
        }
        if ev.evPtrLength != 0 {
            ev.evPtr = Z_Malloc(ev.evPtrLength);
            r = crate::src::qcommon::files::FS_Read(ev.evPtr, ev.evPtrLength, com_journalFile);
            if r != ev.evPtrLength {
                Com_Error(
                    crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                    b"Error reading from journal file\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
    } else {
        ev = Com_GetSystemEvent();
        // write the journal value out if needed
        if (*com_journal).integer == 1 as libc::c_int {
            r = crate::src::qcommon::files::FS_Write(
                &mut ev as *mut crate::qcommon_h::sysEvent_t as *const libc::c_void,
                ::std::mem::size_of::<crate::qcommon_h::sysEvent_t>() as libc::c_ulong
                    as libc::c_int,
                com_journalFile,
            );
            if r as libc::c_ulong
                != ::std::mem::size_of::<crate::qcommon_h::sysEvent_t>() as libc::c_ulong
            {
                Com_Error(
                    crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                    b"Error writing to journal file\x00" as *const u8 as *const libc::c_char,
                );
            }
            if ev.evPtrLength != 0 {
                r = crate::src::qcommon::files::FS_Write(ev.evPtr, ev.evPtrLength, com_journalFile);
                if r != ev.evPtrLength {
                    Com_Error(
                        crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                        b"Error writing to journal file\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
    return ev;
}
/*
=================
Com_InitPushEvent
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_InitPushEvent() {
    // clear the static buffer array
    // this requires SE_NONE to be accepted as a valid but NOP event
    crate::stdlib::memset(
        com_pushedEvents.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[crate::qcommon_h::sysEvent_t; 1024]>() as libc::c_ulong,
    );
    // reset counters while we are at it
    // beware: GetEvent might still return an SE_NONE from the buffer
    com_pushedEventsHead = 0 as libc::c_int;
    com_pushedEventsTail = 0 as libc::c_int;
}
/*
=================
Com_PushEvent
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_PushEvent(mut event: *mut crate::qcommon_h::sysEvent_t) {
    let mut ev: *mut crate::qcommon_h::sysEvent_t = 0 as *mut crate::qcommon_h::sysEvent_t;
    static mut printedWarning: libc::c_int = 0 as libc::c_int;
    ev = &mut *com_pushedEvents
        .as_mut_ptr()
        .offset((com_pushedEventsHead & 1024 as libc::c_int - 1 as libc::c_int) as isize)
        as *mut crate::qcommon_h::sysEvent_t;
    if com_pushedEventsHead - com_pushedEventsTail >= 1024 as libc::c_int {
        // don't print the warning constantly, or it can give time for more...
        if printedWarning == 0 {
            printedWarning = crate::src::qcommon::q_shared::qtrue as libc::c_int;
            Com_Printf(
                b"WARNING: Com_PushEvent overflow\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        if !(*ev).evPtr.is_null() {
            Z_Free((*ev).evPtr);
        }
        com_pushedEventsTail += 1
    } else {
        printedWarning = crate::src::qcommon::q_shared::qfalse as libc::c_int
    }
    *ev = *event;
    com_pushedEventsHead += 1;
}
/*
=================
Com_GetEvent
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_GetEvent() -> crate::qcommon_h::sysEvent_t {
    if com_pushedEventsHead > com_pushedEventsTail {
        com_pushedEventsTail += 1;
        return com_pushedEvents[(com_pushedEventsTail - 1 as libc::c_int
            & 1024 as libc::c_int - 1 as libc::c_int) as usize];
    }
    return Com_GetRealEvent();
}
/*
=================
Com_RunAndTimeServerPacket
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_RunAndTimeServerPacket(
    mut evFrom: *mut crate::qcommon_h::netadr_t,
    mut buf: *mut crate::qcommon_h::msg_t,
) {
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut msec: libc::c_int = 0;
    t1 = 0 as libc::c_int;
    if (*com_speeds).integer != 0 {
        t1 = crate::src::sys::sys_unix::Sys_Milliseconds()
    }
    crate::src::server::sv_main::SV_PacketEvent(
        *evFrom as crate::qcommon_h::netadr_t,
        buf as *mut crate::qcommon_h::msg_t,
    );
    if (*com_speeds).integer != 0 {
        t2 = crate::src::sys::sys_unix::Sys_Milliseconds();
        msec = t2 - t1;
        if (*com_speeds).integer == 3 as libc::c_int {
            Com_Printf(
                b"SV_PacketEvent time: %i\n\x00" as *const u8 as *const libc::c_char,
                msec,
            );
        }
    };
}
/*
=================
Com_EventLoop

Returns last event time
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_EventLoop() -> libc::c_int {
    let mut ev: crate::qcommon_h::sysEvent_t = crate::qcommon_h::sysEvent_t {
        evTime: 0,
        evType: crate::qcommon_h::SE_NONE,
        evValue: 0,
        evValue2: 0,
        evPtrLength: 0,
        evPtr: 0 as *const libc::c_void as *mut libc::c_void,
    };
    let mut evFrom: crate::qcommon_h::netadr_t = crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    };
    let mut bufData: [crate::src::qcommon::q_shared::byte; 16384] = [0; 16384];
    let mut buf: crate::qcommon_h::msg_t = crate::qcommon_h::msg_t {
        allowoverflow: crate::src::qcommon::q_shared::qfalse,
        overflowed: crate::src::qcommon::q_shared::qfalse,
        oob: crate::src::qcommon::q_shared::qfalse,
        data: 0 as *mut crate::src::qcommon::q_shared::byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    crate::src::qcommon::msg::MSG_Init(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        bufData.as_mut_ptr(),
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16384]>() as libc::c_ulong
            as libc::c_int,
    );
    loop {
        ev = Com_GetEvent();
        // if no more events are available
        if ev.evType as libc::c_uint == crate::qcommon_h::SE_NONE as libc::c_int as libc::c_uint {
            // manually send packet events for the loopback channel
            while crate::src::qcommon::net_chan::NET_GetLoopPacket(
                crate::qcommon_h::NS_CLIENT,
                &mut evFrom as *mut _ as *mut crate::qcommon_h::netadr_t,
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
            ) as u64
                != 0
            {
                crate::src::client::cl_main::CL_PacketEvent(
                    evFrom as crate::qcommon_h::netadr_t,
                    &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                );
            }
            while crate::src::qcommon::net_chan::NET_GetLoopPacket(
                crate::qcommon_h::NS_SERVER,
                &mut evFrom as *mut _ as *mut crate::qcommon_h::netadr_t,
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
            ) as u64
                != 0
            {
                // if the server just shut down, flush the events
                if (*com_sv_running).integer != 0 {
                    Com_RunAndTimeServerPacket(&mut evFrom, &mut buf);
                }
            }
            return ev.evTime;
        }
        match ev.evType as libc::c_uint {
            1 => {
                crate::src::client::cl_keys::CL_KeyEvent(
                    ev.evValue,
                    ev.evValue2 as crate::src::qcommon::q_shared::qboolean,
                    ev.evTime as libc::c_uint,
                );
            }
            2 => {
                crate::src::client::cl_keys::CL_CharEvent(ev.evValue);
            }
            3 => {
                crate::src::client::cl_input::CL_MouseEvent(ev.evValue, ev.evValue2, ev.evTime);
            }
            4 => {
                crate::src::client::cl_input::CL_JoystickEvent(ev.evValue, ev.evValue2, ev.evTime);
            }
            5 => {
                crate::src::qcommon::cmd::Cbuf_AddText(ev.evPtr as *mut libc::c_char);
                crate::src::qcommon::cmd::Cbuf_AddText(
                    b"\n\x00" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                Com_Error(
                    crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                    b"Com_EventLoop: bad event type %i\x00" as *const u8 as *const libc::c_char,
                    ev.evType as libc::c_uint,
                );
            }
        }
        // free any block data
        if !ev.evPtr.is_null() {
            Z_Free(ev.evPtr);
        }
    }
    // never reached
}
/*
================
Com_Milliseconds

Can be used for profiling, but will be journaled accurately
================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_Milliseconds() -> libc::c_int {
    let mut ev: crate::qcommon_h::sysEvent_t = crate::qcommon_h::sysEvent_t {
        evTime: 0,
        evType: crate::qcommon_h::SE_NONE,
        evValue: 0,
        evValue2: 0,
        evPtrLength: 0,
        evPtr: 0 as *const libc::c_void as *mut libc::c_void,
    };
    loop
    // get events and push them until we get a null event with the current time
    {
        ev = Com_GetRealEvent();
        if ev.evType as libc::c_uint != crate::qcommon_h::SE_NONE as libc::c_int as libc::c_uint {
            Com_PushEvent(&mut ev);
        }
        if !(ev.evType as libc::c_uint != crate::qcommon_h::SE_NONE as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    return ev.evTime;
}
//============================================================================
/*
=============
Com_Error_f

Just throw a fatal error to
test error shutdown procedures
=============
*/

unsafe extern "C" fn Com_Error_f() -> ! {
    if crate::src::qcommon::cmd::Cmd_Argc() > 1 as libc::c_int {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Testing drop error\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Testing fatal error\x00" as *const u8 as *const libc::c_char,
        );
    };
}
/*
=============
Com_Freeze_f

Just freeze in place for a given number of seconds to test
error recovery
=============
*/

unsafe extern "C" fn Com_Freeze_f() {
    let mut s: libc::c_float = 0.;
    let mut start: libc::c_int = 0;
    let mut now: libc::c_int = 0;
    if crate::src::qcommon::cmd::Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(b"freeze <seconds>\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    s = atof(crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int)) as libc::c_float;
    start = Com_Milliseconds();
    loop {
        now = Com_Milliseconds();
        if (now - start) as libc::c_double * 0.001f64 > s as libc::c_double {
            break;
        }
    }
}
/*
=================
Com_Crash_f

A way to force a bus error for development reasons
=================
*/

unsafe extern "C" fn Com_Crash_f() {
    ::std::ptr::write_volatile(0 as *mut libc::c_int, 0x12345678 as libc::c_int);
}
/*
==================
Com_Setenv_f

For controlling environment variables
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_Setenv_f() {
    let mut argc: libc::c_int = crate::src::qcommon::cmd::Cmd_Argc();
    let mut arg1: *mut libc::c_char = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int);
    if argc > 2 as libc::c_int {
        let mut arg2: *mut libc::c_char = crate::src::qcommon::cmd::Cmd_ArgsFrom(2 as libc::c_int);
        crate::src::sys::sys_unix::Sys_SetEnv(arg1, arg2);
    } else if argc == 2 as libc::c_int {
        let mut env: *mut libc::c_char = ::libc::getenv(arg1);
        if !env.is_null() {
            Com_Printf(
                b"%s=%s\n\x00" as *const u8 as *const libc::c_char,
                arg1,
                env,
            );
        } else {
            Com_Printf(
                b"%s undefined\n\x00" as *const u8 as *const libc::c_char,
                arg1,
            );
        }
    };
}
/*
==================
Com_ExecuteCfg

For controlling environment variables
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_ExecuteCfg() {
    crate::src::qcommon::cmd::Cbuf_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_NOW as libc::c_int,
        b"exec default.cfg\n\x00" as *const u8 as *const libc::c_char,
    ); // Always execute after exec to prevent text buffer overflowing
    crate::src::qcommon::cmd::Cbuf_Execute();
    if Com_SafeMode() as u64 == 0 {
        // skip the q3config.cfg and autoexec.cfg if "safe" is on the command line
        crate::src::qcommon::cmd::Cbuf_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_NOW as libc::c_int,
            b"exec q3config.cfg\n\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::cmd::Cbuf_Execute();
        crate::src::qcommon::cmd::Cbuf_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_NOW as libc::c_int,
            b"exec autoexec.cfg\n\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::cmd::Cbuf_Execute();
    };
}
/*
==================
Com_GameRestart

Change to a new mod properly with cleaning up cvars before switching.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_GameRestart(
    mut checksumFeed: libc::c_int,
    mut disconnect: crate::src::qcommon::q_shared::qboolean,
) {
    // make sure no recursion can be triggered
    if com_gameRestarting as u64 == 0 && com_fullyInitialized as libc::c_uint != 0 {
        com_gameRestarting = crate::src::qcommon::q_shared::qtrue;
        com_gameClientRestarting =
            (*com_cl_running).integer as crate::src::qcommon::q_shared::qboolean;
        // Kill server if we have one
        if (*com_sv_running).integer != 0 {
            crate::src::server::sv_init::SV_Shutdown(
                b"Game directory changed\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        if com_gameClientRestarting as u64 != 0 {
            if disconnect as u64 != 0 {
                crate::src::client::cl_main::CL_Disconnect(crate::src::qcommon::q_shared::qfalse);
            }
            crate::src::client::cl_main::CL_Shutdown(
                b"Game directory changed\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                disconnect,
                crate::src::qcommon::q_shared::qfalse,
            );
        }
        crate::src::qcommon::files::FS_Restart(checksumFeed);
        // Clean out any user and VM created cvars
        crate::src::qcommon::cvar::Cvar_Restart(crate::src::qcommon::q_shared::qtrue);
        Com_ExecuteCfg();
        if disconnect as u64 != 0 {
            // We don't want to change any network settings if gamedir
            // change was triggered by a connect to server because the
            // new network settings might make the connection fail.
            crate::src::qcommon::net_ip::NET_Restart_f();
        }
        if com_gameClientRestarting as u64 != 0 {
            crate::src::client::cl_main::CL_Init();
            crate::src::client::cl_main::CL_StartHunkUsers(crate::src::qcommon::q_shared::qfalse);
        }
        com_gameRestarting = crate::src::qcommon::q_shared::qfalse;
        com_gameClientRestarting = crate::src::qcommon::q_shared::qfalse
    };
}
/*
==================
Com_GameRestart_f

Expose possibility to change current running mod to the user
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_GameRestart_f() {
    crate::src::qcommon::cvar::Cvar_Set(
        b"fs_game\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
    );
    Com_GameRestart(0 as libc::c_int, crate::src::qcommon::q_shared::qtrue);
}
// TTimo: centralizing the cl_cdkey stuff after I discovered a buffer overflow problem with the dedicated server version
//   not sure it's necessary to have different defaults for regular and dedicated, but I don't want to risk it
//   https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=470
#[no_mangle]

pub static mut cl_cdkey: [libc::c_char; 34] = [
    32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
    32, 32, 32, 32, 32, 32, 32, 32, 0, 0,
];
#[no_mangle]

pub unsafe extern "C" fn Com_ReadCDKey(mut filename: *const libc::c_char) {
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut buffer: [libc::c_char; 33] = [0; 33];
    let mut fbuffer: [libc::c_char; 4096] = [0; 4096];
    crate::src::qcommon::q_shared::Com_sprintf(
        fbuffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"%s/q3key\x00" as *const u8 as *const libc::c_char,
        filename,
    );
    crate::src::qcommon::files::FS_SV_FOpenFileRead(fbuffer.as_mut_ptr(), &mut f);
    if f == 0 {
        crate::src::qcommon::q_shared::Q_strncpyz(
            cl_cdkey.as_mut_ptr(),
            b"                \x00" as *const u8 as *const libc::c_char,
            17 as libc::c_int,
        );
        return;
    }
    crate::stdlib::memset(
        buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
    );
    crate::src::qcommon::files::FS_Read(
        buffer.as_mut_ptr() as *mut libc::c_void,
        16 as libc::c_int,
        f,
    );
    crate::src::qcommon::files::FS_FCloseFile(f);
    if CL_CDKeyValidate(buffer.as_mut_ptr(), 0 as *const libc::c_char) as u64 != 0 {
        crate::src::qcommon::q_shared::Q_strncpyz(
            cl_cdkey.as_mut_ptr(),
            buffer.as_mut_ptr(),
            17 as libc::c_int,
        );
    } else {
        crate::src::qcommon::q_shared::Q_strncpyz(
            cl_cdkey.as_mut_ptr(),
            b"                \x00" as *const u8 as *const libc::c_char,
            17 as libc::c_int,
        );
    };
}
/*
=================
Com_AppendCDKey
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_AppendCDKey(mut filename: *const libc::c_char) {
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut buffer: [libc::c_char; 33] = [0; 33];
    let mut fbuffer: [libc::c_char; 4096] = [0; 4096];
    crate::src::qcommon::q_shared::Com_sprintf(
        fbuffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"%s/q3key\x00" as *const u8 as *const libc::c_char,
        filename,
    );
    crate::src::qcommon::files::FS_SV_FOpenFileRead(fbuffer.as_mut_ptr(), &mut f);
    if f == 0 {
        crate::src::qcommon::q_shared::Q_strncpyz(
            &mut *cl_cdkey.as_mut_ptr().offset(16 as libc::c_int as isize),
            b"                \x00" as *const u8 as *const libc::c_char,
            17 as libc::c_int,
        );
        return;
    }
    crate::stdlib::memset(
        buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
    );
    crate::src::qcommon::files::FS_Read(
        buffer.as_mut_ptr() as *mut libc::c_void,
        16 as libc::c_int,
        f,
    );
    crate::src::qcommon::files::FS_FCloseFile(f);
    if CL_CDKeyValidate(buffer.as_mut_ptr(), 0 as *const libc::c_char) as u64 != 0 {
        ::libc::strcat(
            &mut *cl_cdkey.as_mut_ptr().offset(16 as libc::c_int as isize),
            buffer.as_mut_ptr(),
        );
    } else {
        crate::src::qcommon::q_shared::Q_strncpyz(
            &mut *cl_cdkey.as_mut_ptr().offset(16 as libc::c_int as isize),
            b"                \x00" as *const u8 as *const libc::c_char,
            17 as libc::c_int,
        );
    };
}
/*
=================
Com_WriteCDKey
=================
*/

unsafe extern "C" fn Com_WriteCDKey(
    mut filename: *const libc::c_char,
    mut ikey: *const libc::c_char,
) {
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut fbuffer: [libc::c_char; 4096] = [0; 4096];
    let mut key: [libc::c_char; 17] = [0; 17];
    let mut savedumask: crate::stdlib::mode_t = 0;
    crate::src::qcommon::q_shared::Com_sprintf(
        fbuffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"%s/q3key\x00" as *const u8 as *const libc::c_char,
        filename,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(key.as_mut_ptr(), ikey, 17 as libc::c_int);
    if CL_CDKeyValidate(key.as_mut_ptr(), 0 as *const libc::c_char) as u64 == 0 {
        return;
    }
    savedumask = ::libc::umask(0o77 as libc::c_int as crate::stdlib::__mode_t);
    f = crate::src::qcommon::files::FS_SV_FOpenFileWrite(fbuffer.as_mut_ptr());
    if f == 0 {
        Com_Printf(
            b"Couldn\'t write CD key to %s.\n\x00" as *const u8 as *const libc::c_char,
            fbuffer.as_mut_ptr(),
        );
    } else {
        crate::src::qcommon::files::FS_Write(
            key.as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int,
            f,
        );
        crate::src::qcommon::files::FS_Printf(
            f,
            b"\n// generated by quake, do not modify\r\n\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::files::FS_Printf(
            f,
            b"// Do not give this file to ANYONE.\r\n\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::files::FS_Printf(
            f,
            b"// id Software and Activision will NOT ask you to send this file to them.\r\n\x00"
                as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::files::FS_FCloseFile(f);
    }
    ::libc::umask(savedumask);
}
// STANDALONE

unsafe extern "C" fn Com_DetectAltivec() {
    // Only detect if user hasn't forcibly disabled it.
    if (*com_altivec).integer != 0 {
        static mut altivec: crate::src::qcommon::q_shared::qboolean =
            crate::src::qcommon::q_shared::qfalse;
        static mut detected: crate::src::qcommon::q_shared::qboolean =
            crate::src::qcommon::q_shared::qfalse;
        if detected as u64 == 0 {
            altivec = (crate::src::sys::sys_main::Sys_GetProcessorFeatures() as libc::c_uint
                & crate::qcommon_h::CF_ALTIVEC as libc::c_int as libc::c_uint)
                as crate::src::qcommon::q_shared::qboolean;
            detected = crate::src::qcommon::q_shared::qtrue
        }
        if altivec as u64 == 0 {
            crate::src::qcommon::cvar::Cvar_Set(
                b"com_altivec\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
            // we don't have it! Disable support!
        }
    };
}
/*
=================
Com_DetectSSE
Find out whether we have SSE support for Q_ftol function
=================
*/

unsafe extern "C" fn Com_DetectSSE() {
    Q_VMftol = Some(crate::src::asm::ftola::qvmftolsse as unsafe extern "C" fn() -> libc::c_int);
    Com_Printf(b"SSE instruction set enabled\n\x00" as *const u8 as *const libc::c_char);
}
/*
=================
Com_InitRand
Seed the random number generator, if possible with an OS supplied random seed.
=================
*/

unsafe extern "C" fn Com_InitRand() {
    let mut seed: libc::c_uint = 0;
    if crate::src::sys::sys_unix::Sys_RandomBytes(
        &mut seed as *mut libc::c_uint as *mut crate::src::qcommon::q_shared::byte,
        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int,
    ) as u64
        != 0
    {
        ::libc::srand(seed);
    } else {
        ::libc::srand(::libc::time(0 as *mut crate::stdlib::time_t) as libc::c_uint);
    };
}
// commandLine should not include the executable name (argv[0])
/*
=================
Com_Init
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_Init(mut commandLine: *mut libc::c_char) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qport: libc::c_int = 0;
    Com_Printf(
        b"%s %s %s\n\x00" as *const u8 as *const libc::c_char,
        b"ioq3 1.36_GIT_d0fe4462-2020-01-10\x00" as *const u8 as *const libc::c_char,
        b"linux-x86_64\x00" as *const u8 as *const libc::c_char,
        b"Jan 10 2020\x00" as *const u8 as *const libc::c_char,
    );
    if crate::stdlib::_setjmp(abortframe.as_mut_ptr()) != 0 {
        crate::src::sys::sys_main::Sys_Error(
            b"Error during initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    // Clear queues
    crate::stdlib::memset(
        &mut *eventQueue.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut crate::qcommon_h::sysEvent_t as *mut libc::c_void,
        0 as libc::c_int,
        (256 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::qcommon_h::sysEvent_t>() as libc::c_ulong),
    );
    // initialize the weak pseudo-random number generator for use later.
    Com_InitRand();
    // do this before anything else decides to push events
    Com_InitPushEvent();
    Com_InitSmallZoneMemory();
    crate::src::qcommon::cvar::Cvar_Init();
    // prepare enough of the subsystems to handle
    // cvar and command buffer management
    Com_ParseCommandLine(commandLine);
    //	Swap_Init ();
    crate::src::qcommon::cmd::Cbuf_Init();
    Com_DetectSSE();
    // override anything from the config files with command line args
    Com_StartupVariable(0 as *const libc::c_char);
    Com_InitZoneMemory();
    crate::src::qcommon::cmd::Cmd_Init();
    // get the developer cvar set as early as possible
    com_developer = crate::src::qcommon::cvar::Cvar_Get(
        b"developer\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    // done early so bind command exists
    crate::src::client::cl_keys::CL_InitKeyCommands();
    com_standalone = crate::src::qcommon::cvar::Cvar_Get(
        b"com_standalone\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_basegame = crate::src::qcommon::cvar::Cvar_Get(
        b"com_basegame\x00" as *const u8 as *const libc::c_char,
        b"baseq3\x00" as *const u8 as *const libc::c_char,
        0x10 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_homepath = crate::src::qcommon::cvar::Cvar_Get(
        b"com_homepath\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x10 as libc::c_int | 0x2000 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::qcommon::files::FS_InitFilesystem();
    Com_InitJournaling();
    // Add some commands here already so users can use them from config files
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"setenv\x00" as *const u8 as *const libc::c_char,
        Some(Com_Setenv_f as unsafe extern "C" fn() -> ()),
    );
    if !com_developer.is_null() && (*com_developer).integer != 0 {
        crate::src::qcommon::cmd::Cmd_AddCommand(
            b"error\x00" as *const u8 as *const libc::c_char,
            ::std::mem::transmute::<
                Option<unsafe extern "C" fn() -> !>,
                crate::qcommon_h::xcommand_t,
            >(Some(Com_Error_f as unsafe extern "C" fn() -> !)),
        );
        crate::src::qcommon::cmd::Cmd_AddCommand(
            b"crash\x00" as *const u8 as *const libc::c_char,
            Some(Com_Crash_f as unsafe extern "C" fn() -> ()),
        );
        crate::src::qcommon::cmd::Cmd_AddCommand(
            b"freeze\x00" as *const u8 as *const libc::c_char,
            Some(Com_Freeze_f as unsafe extern "C" fn() -> ()),
        );
    }
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"quit\x00" as *const u8 as *const libc::c_char,
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> !>, crate::qcommon_h::xcommand_t>(
            Some(Com_Quit_f as unsafe extern "C" fn() -> !),
        ),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"changeVectors\x00" as *const u8 as *const libc::c_char,
        Some(crate::src::qcommon::msg::MSG_ReportChangeVectors_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"writeconfig\x00" as *const u8 as *const libc::c_char,
        Some(Com_WriteConfig_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"writeconfig\x00" as *const u8 as *const libc::c_char,
        Some(
            crate::src::qcommon::cmd::Cmd_CompleteCfgName
                as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> (),
        ),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"game_restart\x00" as *const u8 as *const libc::c_char,
        Some(Com_GameRestart_f as unsafe extern "C" fn() -> ()),
    );
    Com_ExecuteCfg();
    // override anything from the config files with command line args
    Com_StartupVariable(0 as *const libc::c_char);
    // get dedicated here for proper hunk megs initialization
    com_dedicated = crate::src::qcommon::cvar::Cvar_Get(
        b"dedicated\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x20 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::qcommon::cvar::Cvar_CheckRange(
        com_dedicated as *mut crate::src::qcommon::q_shared::cvar_s,
        0 as libc::c_int as libc::c_float,
        2 as libc::c_int as libc::c_float,
        crate::src::qcommon::q_shared::qtrue,
    );
    // allocate the stack based hunk allocator
    Com_InitHunkMemory();
    // if any archived cvars are modified after this, we will trigger a writing
    // of the config file
    crate::src::qcommon::cvar::cvar_modifiedFlags &= !(0x1 as libc::c_int);
    //
    // init commands and vars
    //
    com_altivec = crate::src::qcommon::cvar::Cvar_Get(
        b"com_altivec\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_maxfps = crate::src::qcommon::cvar::Cvar_Get(
        b"com_maxfps\x00" as *const u8 as *const libc::c_char,
        b"85\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_blood = crate::src::qcommon::cvar::Cvar_Get(
        b"com_blood\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_logfile = crate::src::qcommon::cvar::Cvar_Get(
        b"logfile\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_timescale = crate::src::qcommon::cvar::Cvar_Get(
        b"timescale\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int | 0x8 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_fixedtime = crate::src::qcommon::cvar::Cvar_Get(
        b"fixedtime\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_showtrace = crate::src::qcommon::cvar::Cvar_Get(
        b"com_showtrace\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_speeds = crate::src::qcommon::cvar::Cvar_Get(
        b"com_speeds\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_timedemo = crate::src::qcommon::cvar::Cvar_Get(
        b"timedemo\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_cameraMode = crate::src::qcommon::cvar::Cvar_Get(
        b"com_cameraMode\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_paused = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_paused\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    sv_paused = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_paused\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_packetdelay = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_packetdelay\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    sv_packetdelay = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_packetdelay\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_sv_running = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_running\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_cl_running = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_running\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_buildScript = crate::src::qcommon::cvar::Cvar_Get(
        b"com_buildScript\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_ansiColor = crate::src::qcommon::cvar::Cvar_Get(
        b"com_ansiColor\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_unfocused = crate::src::qcommon::cvar::Cvar_Get(
        b"com_unfocused\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_maxfpsUnfocused = crate::src::qcommon::cvar::Cvar_Get(
        b"com_maxfpsUnfocused\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_minimized = crate::src::qcommon::cvar::Cvar_Get(
        b"com_minimized\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_maxfpsMinimized = crate::src::qcommon::cvar::Cvar_Get(
        b"com_maxfpsMinimized\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_abnormalExit = crate::src::qcommon::cvar::Cvar_Get(
        b"com_abnormalExit\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_busyWait = crate::src::qcommon::cvar::Cvar_Get(
        b"com_busyWait\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"com_errorMessage\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int | 0x400 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_introPlayed = crate::src::qcommon::cvar::Cvar_Get(
        b"com_introplayed\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    s = crate::src::qcommon::q_shared::va(
        b"%s %s %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ioq3 1.36_GIT_d0fe4462-2020-01-10\x00" as *const u8 as *const libc::c_char,
        b"linux-x86_64\x00" as *const u8 as *const libc::c_char,
        b"Jan 10 2020\x00" as *const u8 as *const libc::c_char,
    );
    com_version = crate::src::qcommon::cvar::Cvar_Get(
        b"version\x00" as *const u8 as *const libc::c_char,
        s,
        0x40 as libc::c_int | 0x4 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_gamename = crate::src::qcommon::cvar::Cvar_Get(
        b"com_gamename\x00" as *const u8 as *const libc::c_char,
        b"Quake3Arena\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int | 0x10 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_protocol = crate::src::qcommon::cvar::Cvar_Get(
        b"com_protocol\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            71 as libc::c_int,
        ),
        0x4 as libc::c_int | 0x10 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    com_legacyprotocol = crate::src::qcommon::cvar::Cvar_Get(
        b"com_legacyprotocol\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            68 as libc::c_int,
        ),
        0x10 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    // Keep for compatibility with old mods / mods that haven't updated yet.
    if (*com_legacyprotocol).integer > 0 as libc::c_int {
        crate::src::qcommon::cvar::Cvar_Get(
            b"protocol\x00" as *const u8 as *const libc::c_char,
            (*com_legacyprotocol).string,
            0x40 as libc::c_int,
        ) as *mut crate::src::qcommon::q_shared::cvar_s;
    } else {
        crate::src::qcommon::cvar::Cvar_Get(
            b"protocol\x00" as *const u8 as *const libc::c_char,
            (*com_protocol).string,
            0x40 as libc::c_int,
        ) as *mut crate::src::qcommon::q_shared::cvar_s;
    }
    con_autochat = crate::src::qcommon::cvar::Cvar_Get(
        b"con_autochat\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::sys::sys_main::Sys_Init();
    crate::src::sys::sys_main::Sys_InitPIDFile(crate::src::qcommon::files::FS_GetCurrentGameDir());
    // Pick a random port value
    Com_RandomBytes(
        &mut qport as *mut libc::c_int as *mut crate::src::qcommon::q_shared::byte,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    );
    crate::src::qcommon::net_chan::Netchan_Init(qport & 0xffff as libc::c_int);
    crate::src::qcommon::vm::VM_Init();
    crate::src::server::sv_init::SV_Init();
    (*com_dedicated).modified = crate::src::qcommon::q_shared::qfalse;
    crate::src::client::cl_main::CL_Init();
    // set com_frameTime so that if a map is started on the
    // command line it will still be able to count on com_frameTime
    // being random enough for a serverid
    com_frameTime = Com_Milliseconds();
    // add + commands from command line
    if Com_AddStartupCommands() as u64 == 0 {
        // if the user didn't give any commands, run default action
        if (*com_dedicated).integer == 0 {
            crate::src::qcommon::cmd::Cbuf_AddText(
                b"cinematic idlogo.RoQ\n\x00" as *const u8 as *const libc::c_char,
            );
            if (*com_introPlayed).integer == 0 {
                crate::src::qcommon::cvar::Cvar_Set(
                    (*com_introPlayed).name,
                    b"1\x00" as *const u8 as *const libc::c_char,
                );
                crate::src::qcommon::cvar::Cvar_Set(
                    b"nextmap\x00" as *const u8 as *const libc::c_char,
                    b"cinematic intro.RoQ\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    // start in full screen ui mode
    crate::src::qcommon::cvar::Cvar_Set(
        b"r_uiFullScreen\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::client::cl_main::CL_StartHunkUsers(crate::src::qcommon::q_shared::qfalse);
    // make sure single player is off by default
    crate::src::qcommon::cvar::Cvar_Set(
        b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    com_fullyInitialized = crate::src::qcommon::q_shared::qtrue;
    // always set the cvar, but only print the info if it makes sense.
    Com_DetectAltivec();
    com_pipefile = crate::src::qcommon::cvar::Cvar_Get(
        b"com_pipefile\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x20 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    if *(*com_pipefile).string.offset(0 as libc::c_int as isize) != 0 {
        pipefile = crate::src::qcommon::files::FS_FCreateOpenPipeFile((*com_pipefile).string)
    }
    Com_Printf(b"--- Common Initialization Complete ---\n\x00" as *const u8 as *const libc::c_char);
}
/*
===============
Com_ReadFromPipe

Read whatever is in com_pipefile, if anything, and execute it
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Com_ReadFromPipe() {
    static mut buf: [libc::c_char; 1024] = [0; 1024];
    static mut accu: libc::c_int = 0 as libc::c_int;
    let mut read: libc::c_int = 0;
    if pipefile == 0 {
        return;
    }
    loop {
        read = crate::src::qcommon::files::FS_Read(
            buf.as_mut_ptr().offset(accu as isize) as *mut libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(accu as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            pipefile,
        );
        if !(read > 0 as libc::c_int) {
            break;
        }
        let mut brk: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i: libc::c_int = 0;
        i = accu;
        while i < accu + read {
            if buf[i as usize] as libc::c_int == '\u{0}' as i32 {
                buf[i as usize] = '\n' as i32 as libc::c_char
            }
            if buf[i as usize] as libc::c_int == '\n' as i32
                || buf[i as usize] as libc::c_int == '\r' as i32
            {
                brk = &mut *buf.as_mut_ptr().offset((i + 1 as libc::c_int) as isize)
                    as *mut libc::c_char
            }
            i += 1
        }
        buf[(accu + read) as usize] = '\u{0}' as i32 as libc::c_char;
        accu += read;
        if !brk.is_null() {
            let mut tmp: libc::c_char = *brk;
            *brk = '\u{0}' as i32 as libc::c_char;
            crate::src::qcommon::cmd::Cbuf_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
                buf.as_mut_ptr(),
            );
            *brk = tmp;
            accu = (accu as libc::c_long
                - brk.wrapping_offset_from(buf.as_mut_ptr()) as libc::c_long)
                as libc::c_int;
            crate::stdlib::memmove(
                buf.as_mut_ptr() as *mut libc::c_void,
                brk as *const libc::c_void,
                (accu + 1 as libc::c_int) as libc::c_ulong,
            );
        } else if accu as libc::c_ulong
            >= (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            // full
            crate::src::qcommon::cmd::Cbuf_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
                buf.as_mut_ptr(),
            );
            accu = 0 as libc::c_int
        }
    }
}
//==================================================================
#[no_mangle]

pub unsafe extern "C" fn Com_WriteConfigToFile(mut filename: *const libc::c_char) {
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    f = crate::src::qcommon::files::FS_FOpenFileWrite(filename);
    if f == 0 {
        Com_Printf(
            b"Couldn\'t write %s.\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        return;
    }
    crate::src::qcommon::files::FS_Printf(
        f,
        b"// generated by quake, do not modify\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::client::cl_keys::Key_WriteBindings(f);
    crate::src::qcommon::cvar::Cvar_WriteVariables(f);
    crate::src::qcommon::files::FS_FCloseFile(f);
}
/*
===============
Com_WriteConfiguration

Writes key bindings and archived cvars to config file if modified
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Com_WriteConfiguration() {
    // if we are quiting without fully initializing, make sure
    // we don't write out anything
    if com_fullyInitialized as u64 == 0 {
        return;
    }
    if crate::src::qcommon::cvar::cvar_modifiedFlags & 0x1 as libc::c_int == 0 {
        return;
    }
    crate::src::qcommon::cvar::cvar_modifiedFlags &= !(0x1 as libc::c_int);
    Com_WriteConfigToFile(b"q3config.cfg\x00" as *const u8 as *const libc::c_char);
    // not needed for dedicated or standalone
    if (*com_standalone).integer == 0 {
        let mut gamedir: *const libc::c_char = 0 as *const libc::c_char;
        gamedir = crate::src::qcommon::cvar::Cvar_VariableString(
            b"fs_game\x00" as *const u8 as *const libc::c_char,
        );
        if crate::src::client::cl_ui::UI_usesUniqueCDKey() as libc::c_uint != 0
            && *gamedir.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int
        {
            Com_WriteCDKey(
                gamedir,
                &mut *cl_cdkey.as_mut_ptr().offset(16 as libc::c_int as isize),
            );
        } else {
            Com_WriteCDKey(
                b"baseq3\x00" as *const u8 as *const libc::c_char,
                cl_cdkey.as_mut_ptr(),
            );
        }
    };
}
/*
===============
Com_WriteConfig_f

Write the config file to a specific name
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Com_WriteConfig_f() {
    let mut filename: [libc::c_char; 64] = [0; 64];
    if crate::src::qcommon::cmd::Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(b"Usage: writeconfig <filename>\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        filename.as_mut_ptr(),
        crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::qcommon::q_shared::COM_DefaultExtension(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b".cfg\x00" as *const u8 as *const libc::c_char,
    );
    if crate::src::qcommon::q_shared::COM_CompareExtension(
        filename.as_mut_ptr(),
        b".cfg\x00" as *const u8 as *const libc::c_char,
    ) as u64
        == 0
    {
        Com_Printf(
            b"Com_WriteConfig_f: Only the \".cfg\" extension is supported by this command!\n\x00"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    Com_Printf(
        b"Writing %s.\n\x00" as *const u8 as *const libc::c_char,
        filename.as_mut_ptr(),
    );
    Com_WriteConfigToFile(filename.as_mut_ptr());
}
/*
================
Com_ModifyMsec
================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_ModifyMsec(mut msec: libc::c_int) -> libc::c_int {
    let mut clampTime: libc::c_int = 0;
    //
    // modify time for debugging values
    //
    if (*com_fixedtime).integer != 0 {
        msec = (*com_fixedtime).integer
    } else if (*com_timescale).value != 0. {
        msec = (msec as libc::c_float * (*com_timescale).value) as libc::c_int
    } else if (*com_cameraMode).integer != 0 {
        msec = (msec as libc::c_float * (*com_timescale).value) as libc::c_int
    }
    // don't let it scale below 1 msec
    if msec < 1 as libc::c_int && (*com_timescale).value != 0. {
        msec = 1 as libc::c_int
    }
    if (*com_dedicated).integer != 0 {
        // dedicated servers don't want to clamp for a much longer
        // period, because it would mess up all the client's views
        // of time.
        if (*com_sv_running).integer != 0 && msec > 500 as libc::c_int {
            Com_Printf(
                b"Hitch warning: %i msec frame time\n\x00" as *const u8 as *const libc::c_char,
                msec,
            );
        }
        clampTime = 5000 as libc::c_int
    } else if (*com_sv_running).integer == 0 {
        // clients of remote servers do not want to clamp time, because
        // it would skew their view of the server's time temporarily
        clampTime = 5000 as libc::c_int
    } else {
        // for local single player gaming
        // we may want to clamp the time to prevent players from
        // flying off edges when something hitches.
        clampTime = 200 as libc::c_int
    }
    if msec > clampTime {
        msec = clampTime
    }
    return msec;
}
/*
=================
Com_TimeVal
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_TimeVal(mut minMsec: libc::c_int) -> libc::c_int {
    let mut timeVal: libc::c_int = 0;
    timeVal = crate::src::sys::sys_unix::Sys_Milliseconds() - com_frameTime;
    if timeVal >= minMsec {
        timeVal = 0 as libc::c_int
    } else {
        timeVal = minMsec - timeVal
    }
    return timeVal;
}
/*
=================
Com_Frame
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_Frame() {
    let mut msec: libc::c_int = 0;
    let mut minMsec: libc::c_int = 0;
    let mut timeVal: libc::c_int = 0;
    let mut timeValSV: libc::c_int = 0;
    static mut lastTime: libc::c_int = 0 as libc::c_int;
    static mut bias: libc::c_int = 0 as libc::c_int;
    let mut timeBeforeFirstEvents: libc::c_int = 0;
    let mut timeBeforeServer: libc::c_int = 0;
    let mut timeBeforeEvents: libc::c_int = 0;
    let mut timeBeforeClient: libc::c_int = 0;
    let mut timeAfter: libc::c_int = 0;
    if crate::stdlib::_setjmp(abortframe.as_mut_ptr()) != 0 {
        return;
        // an ERR_DROP was thrown
    }
    timeBeforeFirstEvents = 0 as libc::c_int;
    timeBeforeServer = 0 as libc::c_int;
    timeBeforeEvents = 0 as libc::c_int;
    timeBeforeClient = 0 as libc::c_int;
    timeAfter = 0 as libc::c_int;
    // write config file if anything changed
    Com_WriteConfiguration();
    //
    // main event loop
    //
    if (*com_speeds).integer != 0 {
        timeBeforeFirstEvents = crate::src::sys::sys_unix::Sys_Milliseconds()
    }
    // Figure out how much time we have
    if (*com_timedemo).integer == 0 {
        if (*com_dedicated).integer != 0 {
            minMsec = crate::src::server::sv_main::SV_FrameMsec()
        } else {
            if (*com_minimized).integer != 0 && (*com_maxfpsMinimized).integer > 0 as libc::c_int {
                minMsec = 1000 as libc::c_int / (*com_maxfpsMinimized).integer
            } else if (*com_unfocused).integer != 0
                && (*com_maxfpsUnfocused).integer > 0 as libc::c_int
            {
                minMsec = 1000 as libc::c_int / (*com_maxfpsUnfocused).integer
            } else if (*com_maxfps).integer > 0 as libc::c_int {
                minMsec = 1000 as libc::c_int / (*com_maxfps).integer
            } else {
                minMsec = 1 as libc::c_int
            }
            timeVal = com_frameTime - lastTime;
            bias += timeVal - minMsec;
            if bias > minMsec {
                bias = minMsec
            }
            // Adjust minMsec if previous frame took too long to render so
            // that framerate is stable at the requested value.
            minMsec -= bias
        }
    } else {
        minMsec = 1 as libc::c_int
    }
    loop {
        if (*com_sv_running).integer != 0 {
            timeValSV = crate::src::server::sv_main::SV_SendQueuedPackets();
            timeVal = Com_TimeVal(minMsec);
            if timeValSV < timeVal {
                timeVal = timeValSV
            }
        } else {
            timeVal = Com_TimeVal(minMsec)
        }
        if (*com_busyWait).integer != 0 || timeVal < 1 as libc::c_int {
            crate::src::qcommon::net_ip::NET_Sleep(0 as libc::c_int);
        } else {
            crate::src::qcommon::net_ip::NET_Sleep(timeVal - 1 as libc::c_int);
        }
        if !(Com_TimeVal(minMsec) != 0) {
            break;
        }
    }
    crate::src::sdl::sdl_input::IN_Frame();
    lastTime = com_frameTime;
    com_frameTime = Com_EventLoop();
    msec = com_frameTime - lastTime;
    crate::src::qcommon::cmd::Cbuf_Execute();
    if (*com_altivec).modified as u64 != 0 {
        Com_DetectAltivec();
        (*com_altivec).modified = crate::src::qcommon::q_shared::qfalse
    }
    // mess with msec if needed
    msec = Com_ModifyMsec(msec);
    //
    // server side
    //
    if (*com_speeds).integer != 0 {
        timeBeforeServer = crate::src::sys::sys_unix::Sys_Milliseconds()
    }
    crate::src::server::sv_main::SV_Frame(msec);
    // if "dedicated" has been modified, start up
    // or shut down the client system.
    // Do this after the server may have started,
    // but before the client tries to auto-connect
    if (*com_dedicated).modified as u64 != 0 {
        // get the latched value

        crate::src::qcommon::cvar::Cvar_Get(
            b"dedicated\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        ) as *mut crate::src::qcommon::q_shared::cvar_s;
        (*com_dedicated).modified = crate::src::qcommon::q_shared::qfalse;
        if (*com_dedicated).integer == 0 {
            crate::src::server::sv_init::SV_Shutdown(
                b"dedicated set to 0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            crate::src::client::cl_main::CL_FlushMemory();
        }
    }
    //
    // client system
    //
    //
    // run event loop a second time to get server to client packets
    // without a frame of latency
    //
    if (*com_speeds).integer != 0 {
        timeBeforeEvents = crate::src::sys::sys_unix::Sys_Milliseconds()
    }
    Com_EventLoop();
    crate::src::qcommon::cmd::Cbuf_Execute();
    //
    // client side
    //
    if (*com_speeds).integer != 0 {
        timeBeforeClient = crate::src::sys::sys_unix::Sys_Milliseconds()
    }
    crate::src::client::cl_main::CL_Frame(msec);
    if (*com_speeds).integer != 0 {
        timeAfter = crate::src::sys::sys_unix::Sys_Milliseconds()
    }
    crate::src::qcommon::net_chan::NET_FlushPacketQueue();
    //
    // report timing information
    //
    if (*com_speeds).integer != 0 {
        let mut all: libc::c_int = 0;
        let mut sv: libc::c_int = 0;
        let mut ev: libc::c_int = 0;
        let mut cl: libc::c_int = 0;
        all = timeAfter - timeBeforeServer;
        sv = timeBeforeEvents - timeBeforeServer;
        ev = timeBeforeServer - timeBeforeFirstEvents + timeBeforeClient - timeBeforeEvents;
        cl = timeAfter - timeBeforeClient;
        sv -= time_game;
        cl -= time_frontend + time_backend;
        Com_Printf(
            b"frame:%i all:%3i sv:%3i ev:%3i cl:%3i gm:%3i rf:%3i bk:%3i\n\x00" as *const u8
                as *const libc::c_char,
            com_frameNumber,
            all,
            sv,
            ev,
            cl,
            time_game,
            time_frontend,
            time_backend,
        );
    }
    //
    // trace optimization tracking
    //
    if (*com_showtrace).integer != 0 {
        extern "C" {
            #[no_mangle]
            pub static mut c_traces: libc::c_int;
        }
        extern "C" {
            #[no_mangle]
            pub static mut c_brush_traces: libc::c_int;
        }
        extern "C" {
            #[no_mangle]
            pub static mut c_patch_traces: libc::c_int;
        }
        extern "C" {
            #[no_mangle]
            pub static mut c_pointcontents: libc::c_int;
        }
        Com_Printf(
            b"%4i traces  (%ib %ip) %4i points\n\x00" as *const u8 as *const libc::c_char,
            c_traces,
            c_brush_traces,
            c_patch_traces,
            c_pointcontents,
        );
        c_traces = 0 as libc::c_int;
        c_brush_traces = 0 as libc::c_int;
        c_patch_traces = 0 as libc::c_int;
        c_pointcontents = 0 as libc::c_int
    }
    Com_ReadFromPipe();
    com_frameNumber += 1;
}
/*
=================
Com_Shutdown
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_Shutdown() {
    if logfile != 0 {
        crate::src::qcommon::files::FS_FCloseFile(logfile);
        logfile = 0 as libc::c_int
    }
    if com_journalFile != 0 {
        crate::src::qcommon::files::FS_FCloseFile(com_journalFile);
        com_journalFile = 0 as libc::c_int
    }
    if pipefile != 0 {
        crate::src::qcommon::files::FS_FCloseFile(pipefile);
        crate::src::qcommon::files::FS_HomeRemove((*com_pipefile).string);
    };
}
/*
===========================================
command line completion
===========================================
*/
/*
==================
Field_Clear
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Field_Clear(mut edit: *mut crate::qcommon_h::field_t) {
    crate::stdlib::memset(
        (*edit).buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        256 as libc::c_int as libc::c_ulong,
    );
    (*edit).cursor = 0 as libc::c_int;
    (*edit).scroll = 0 as libc::c_int;
}

static mut completionString: *const libc::c_char = 0 as *const libc::c_char;

static mut shortestMatch: [libc::c_char; 1024] = [0; 1024];

static mut matchCount: libc::c_int = 0;
// field we are working on, passed to Field_AutoComplete(&g_consoleCommand for instance)

static mut completionField: *mut crate::qcommon_h::field_t =
    0 as *const crate::qcommon_h::field_t as *mut crate::qcommon_h::field_t;
/*
===============
FindMatches

===============
*/

unsafe extern "C" fn FindMatches(mut s: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    if crate::src::qcommon::q_shared::Q_stricmpn(
        s,
        completionString,
        crate::stdlib::strlen(completionString) as libc::c_int,
    ) != 0
    {
        return;
    }
    matchCount += 1;
    if matchCount == 1 as libc::c_int {
        crate::src::qcommon::q_shared::Q_strncpyz(
            shortestMatch.as_mut_ptr(),
            s,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        return;
    }
    // cut shortestMatch to the amount common with s
    i = 0 as libc::c_int;
    while shortestMatch[i as usize] != 0 {
        if i as libc::c_ulong >= crate::stdlib::strlen(s) {
            shortestMatch[i as usize] = 0 as libc::c_int as libc::c_char;
            break;
        } else {
            if ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = shortestMatch[i as usize] as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = tolower(shortestMatch[i as usize] as libc::c_int)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_tolower_loc())
                        .offset(shortestMatch[i as usize] as libc::c_int as isize)
                }
                __res
            }) != ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *s.offset(i as isize) as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = tolower(*s.offset(i as isize) as libc::c_int)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_tolower_loc())
                        .offset(*s.offset(i as isize) as libc::c_int as isize)
                }
                __res
            }) {
                shortestMatch[i as usize] = 0 as libc::c_int as libc::c_char
            }
            i += 1
        }
    }
}
/*
===============
PrintMatches

===============
*/

unsafe extern "C" fn PrintMatches(mut s: *const libc::c_char) {
    if crate::src::qcommon::q_shared::Q_stricmpn(
        s,
        shortestMatch.as_mut_ptr(),
        crate::stdlib::strlen(shortestMatch.as_mut_ptr()) as libc::c_int,
    ) == 0
    {
        Com_Printf(b"    %s\n\x00" as *const u8 as *const libc::c_char, s);
    };
}
/*
===============
PrintCvarMatches

===============
*/

unsafe extern "C" fn PrintCvarMatches(mut s: *const libc::c_char) {
    let mut value: [libc::c_char; 64] = [0; 64];
    if crate::src::qcommon::q_shared::Q_stricmpn(
        s,
        shortestMatch.as_mut_ptr(),
        crate::stdlib::strlen(shortestMatch.as_mut_ptr()) as libc::c_int,
    ) == 0
    {
        crate::src::qcommon::q_shared::Com_TruncateLongString(
            value.as_mut_ptr(),
            crate::src::qcommon::cvar::Cvar_VariableString(s),
        );
        Com_Printf(
            b"    %s = \"%s\"\n\x00" as *const u8 as *const libc::c_char,
            s,
            value.as_mut_ptr(),
        );
    };
}
/*
===============
Field_FindFirstSeparator
===============
*/

unsafe extern "C" fn Field_FindFirstSeparator(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < crate::stdlib::strlen(s) {
        if *s.offset(i as isize) as libc::c_int == ';' as i32 {
            return &mut *s.offset(i as isize) as *mut libc::c_char;
        }
        i += 1
    }
    return 0 as *mut libc::c_char;
}
/*
===============
Field_Complete
===============
*/

unsafe extern "C" fn Field_Complete() -> crate::src::qcommon::q_shared::qboolean {
    let mut completionOffset: libc::c_int = 0;
    if matchCount == 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qtrue;
    }
    completionOffset = crate::stdlib::strlen((*completionField).buffer.as_mut_ptr())
        .wrapping_sub(crate::stdlib::strlen(completionString))
        as libc::c_int;
    crate::src::qcommon::q_shared::Q_strncpyz(
        &mut *(*completionField)
            .buffer
            .as_mut_ptr()
            .offset(completionOffset as isize),
        shortestMatch.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(completionOffset as libc::c_ulong) as libc::c_int,
    );
    (*completionField).cursor =
        crate::stdlib::strlen((*completionField).buffer.as_mut_ptr()) as libc::c_int;
    if matchCount == 1 as libc::c_int {
        crate::src::qcommon::q_shared::Q_strcat(
            (*completionField).buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            b" \x00" as *const u8 as *const libc::c_char,
        );
        (*completionField).cursor += 1;
        return crate::src::qcommon::q_shared::qtrue;
    }
    Com_Printf(
        b"]%s\n\x00" as *const u8 as *const libc::c_char,
        (*completionField).buffer.as_mut_ptr(),
    );
    return crate::src::qcommon::q_shared::qfalse;
}
/*
===============
Field_CompleteKeyname
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Field_CompleteKeyname() {
    matchCount = 0 as libc::c_int;
    shortestMatch[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    crate::src::client::cl_keys::Key_KeynameCompletion(Some(
        FindMatches as unsafe extern "C" fn(_: *const libc::c_char) -> (),
    ));
    if Field_Complete() as u64 == 0 {
        crate::src::client::cl_keys::Key_KeynameCompletion(Some(
            PrintMatches as unsafe extern "C" fn(_: *const libc::c_char) -> (),
        ));
    };
}
/*
===============
Field_CompleteFilename
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Field_CompleteFilename(
    mut dir: *const libc::c_char,
    mut ext: *const libc::c_char,
    mut stripExt: crate::src::qcommon::q_shared::qboolean,
    mut allowNonPureFilesOnDisk: crate::src::qcommon::q_shared::qboolean,
) {
    matchCount = 0 as libc::c_int;
    shortestMatch[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    crate::src::qcommon::files::FS_FilenameCompletion(
        dir,
        ext,
        stripExt,
        Some(FindMatches as unsafe extern "C" fn(_: *const libc::c_char) -> ()),
        allowNonPureFilesOnDisk,
    );
    if Field_Complete() as u64 == 0 {
        crate::src::qcommon::files::FS_FilenameCompletion(
            dir,
            ext,
            stripExt,
            Some(PrintMatches as unsafe extern "C" fn(_: *const libc::c_char) -> ()),
            allowNonPureFilesOnDisk,
        );
    };
}
/*
===============
Field_CompleteCommand
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Field_CompleteCommand(
    mut cmd: *mut libc::c_char,
    mut doCommands: crate::src::qcommon::q_shared::qboolean,
    mut doCvars: crate::src::qcommon::q_shared::qboolean,
) {
    let mut completionArgument: libc::c_int = 0 as libc::c_int;
    // Skip leading whitespace and quotes
    cmd = crate::src::qcommon::q_shared::Com_SkipCharset(
        cmd,
        b" \"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_TokenizeStringIgnoreQuotes(cmd);
    completionArgument = crate::src::qcommon::cmd::Cmd_Argc();
    // If there is trailing whitespace on the cmd
    if *cmd
        .offset(crate::stdlib::strlen(cmd) as isize)
        .offset(-(1 as libc::c_int as isize)) as libc::c_int
        == ' ' as i32
    {
        completionString = b"\x00" as *const u8 as *const libc::c_char;
        completionArgument += 1
    } else {
        completionString = crate::src::qcommon::cmd::Cmd_Argv(completionArgument - 1 as libc::c_int)
    }
    // add a '\' to the start of the buffer if it might be sent as chat otherwise
    if (*con_autochat).integer != 0
        && (*completionField).buffer[0 as libc::c_int as usize] as libc::c_int != 0
        && (*completionField).buffer[0 as libc::c_int as usize] as libc::c_int != '\\' as i32
    {
        if (*completionField).buffer[0 as libc::c_int as usize] as libc::c_int != '/' as i32 {
            // Buffer is full, refuse to complete
            if crate::stdlib::strlen((*completionField).buffer.as_mut_ptr())
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                >= ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
            {
                return;
            }
            crate::stdlib::memmove(
                &mut *(*completionField)
                    .buffer
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut libc::c_char
                    as *mut libc::c_void,
                &mut *(*completionField)
                    .buffer
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut libc::c_char
                    as *const libc::c_void,
                crate::stdlib::strlen((*completionField).buffer.as_mut_ptr())
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            (*completionField).cursor += 1
        }
        (*completionField).buffer[0 as libc::c_int as usize] = '\\' as i32 as libc::c_char
    }
    if completionArgument > 1 as libc::c_int {
        let mut baseCmd: *const libc::c_char = crate::src::qcommon::cmd::Cmd_Argv(0 as libc::c_int);
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        // This should always be true
        if *baseCmd.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
            || *baseCmd.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            baseCmd = baseCmd.offset(1)
        } // Compound command
        p = Field_FindFirstSeparator(cmd);
        if !p.is_null() {
            Field_CompleteCommand(
                p.offset(1 as libc::c_int as isize),
                crate::src::qcommon::q_shared::qtrue,
                crate::src::qcommon::q_shared::qtrue,
            );
        } else {
            crate::src::qcommon::cmd::Cmd_CompleteArgument(baseCmd, cmd, completionArgument);
        }
    } else {
        if *completionString.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
            || *completionString.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            completionString = completionString.offset(1)
        }
        matchCount = 0 as libc::c_int;
        shortestMatch[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        if crate::stdlib::strlen(completionString) == 0 as libc::c_int as libc::c_ulong {
            return;
        }
        if doCommands as u64 != 0 {
            crate::src::qcommon::cmd::Cmd_CommandCompletion(Some(
                FindMatches as unsafe extern "C" fn(_: *const libc::c_char) -> (),
            ));
        }
        if doCvars as u64 != 0 {
            crate::src::qcommon::cvar::Cvar_CommandCompletion(Some(
                FindMatches as unsafe extern "C" fn(_: *const libc::c_char) -> (),
            ));
        }
        if Field_Complete() as u64 == 0 {
            // run through again, printing matches
            if doCommands as u64 != 0 {
                crate::src::qcommon::cmd::Cmd_CommandCompletion(Some(
                    PrintMatches as unsafe extern "C" fn(_: *const libc::c_char) -> (),
                ));
            }
            if doCvars as u64 != 0 {
                crate::src::qcommon::cvar::Cvar_CommandCompletion(Some(
                    PrintCvarMatches as unsafe extern "C" fn(_: *const libc::c_char) -> (),
                ));
            }
        }
    };
}
/*
===============
Field_AutoComplete

Perform Tab expansion
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Field_AutoComplete(mut field: *mut crate::qcommon_h::field_t) {
    completionField = field;
    Field_CompleteCommand(
        (*completionField).buffer.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue,
        crate::src::qcommon::q_shared::qtrue,
    );
}
/*
==================
Com_RandomBytes

fills string array with len random bytes, preferably from the OS randomizer
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_RandomBytes(
    mut string: *mut crate::src::qcommon::q_shared::byte,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if crate::src::sys::sys_unix::Sys_RandomBytes(string, len) as u64 != 0 {
        return;
    }
    Com_Printf(
        b"Com_RandomBytes: using weak randomization\n\x00" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < len {
        *string.offset(i as isize) = (::libc::rand() % 256 as libc::c_int) as libc::c_uchar;
        i += 1
    }
}
/*
==================
Com_IsVoipTarget

Returns non-zero if given clientNum is enabled in voipTargets, zero otherwise.
If clientNum is negative return if any bit is set.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_IsVoipTarget(
    mut voipTargets: *mut crate::stdlib::uint8_t,
    mut voipTargetsSize: libc::c_int,
    mut clientNum: libc::c_int,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut index: libc::c_int = 0;
    if clientNum < 0 as libc::c_int {
        index = 0 as libc::c_int;
        while index < voipTargetsSize {
            if *voipTargets.offset(index as isize) != 0 {
                return crate::src::qcommon::q_shared::qtrue;
            }
            index += 1
        }
        return crate::src::qcommon::q_shared::qfalse;
    }
    index = clientNum >> 3 as libc::c_int;
    if index < voipTargetsSize {
        return (*voipTargets.offset(index as isize) as libc::c_int
            & (1 as libc::c_int) << (clientNum & 0x7 as libc::c_int))
            as crate::src::qcommon::q_shared::qboolean;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
===============
Field_CompletePlayerName
===============
*/

unsafe extern "C" fn Field_CompletePlayerNameFinal(
    mut whitespace: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut completionOffset: libc::c_int = 0;
    if matchCount == 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qtrue;
    }
    completionOffset = crate::stdlib::strlen((*completionField).buffer.as_mut_ptr())
        .wrapping_sub(crate::stdlib::strlen(completionString))
        as libc::c_int;
    crate::src::qcommon::q_shared::Q_strncpyz(
        &mut *(*completionField)
            .buffer
            .as_mut_ptr()
            .offset(completionOffset as isize),
        shortestMatch.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(completionOffset as libc::c_ulong) as libc::c_int,
    );
    (*completionField).cursor =
        crate::stdlib::strlen((*completionField).buffer.as_mut_ptr()) as libc::c_int;
    if matchCount == 1 as libc::c_int && whitespace as libc::c_uint != 0 {
        crate::src::qcommon::q_shared::Q_strcat(
            (*completionField).buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            b" \x00" as *const u8 as *const libc::c_char,
        );
        (*completionField).cursor += 1;
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}

unsafe extern "C" fn Name_PlayerNameCompletion(
    mut names: *mut *const libc::c_char,
    mut nameCount: libc::c_int,
    mut callback: Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nameCount {
        callback.expect("non-null function pointer")(*names.offset(i as isize));
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn Com_FieldStringToPlayerName(
    mut name: *mut libc::c_char,
    mut length: libc::c_int,
    mut rawname: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut hex: [libc::c_char; 5] = [0; 5];
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    if name.is_null() || rawname.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if length <= 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qtrue;
    }
    i = 0 as libc::c_int;
    while *rawname as libc::c_int != 0 && i + 1 as libc::c_int <= length {
        if *rawname as libc::c_int == '\\' as i32 {
            crate::src::qcommon::q_shared::Q_strncpyz(
                hex.as_mut_ptr(),
                rawname.offset(1 as libc::c_int as isize),
                ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as libc::c_int,
            );
            ch = crate::src::qcommon::q_shared::Com_HexStrToInt(hex.as_mut_ptr());
            if ch > -(1 as libc::c_int) {
                *name.offset(i as isize) = ch as libc::c_char;
                rawname = rawname.offset(4 as libc::c_int as isize)
            //hex string length, 0xXX
            } else {
                *name.offset(i as isize) = *rawname
            }
        } else {
            *name.offset(i as isize) = *rawname
        }
        rawname = rawname.offset(1);
        i += 1
    }
    *name.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
    return crate::src::qcommon::q_shared::qtrue;
}
// checks for and removes command line "+set var arg" constructs
// if match is NULL, all set commands will be executed, otherwise
// only a set with the exact name.  Only used during startup.
#[no_mangle]

pub unsafe extern "C" fn Com_PlayerNameToFieldString(
    mut str: *mut libc::c_char,
    mut length: libc::c_int,
    mut name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    if str.is_null() || name.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if length <= 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qtrue;
    }
    *str = '\u{0}' as i32 as libc::c_char;
    p = name;
    i = 0 as libc::c_int;
    while *p as libc::c_int != '\u{0}' as i32 {
        if i + 1 as libc::c_int >= length {
            break;
        }
        if *p as libc::c_int <= ' ' as i32 {
            if i + 5 as libc::c_int + 1 as libc::c_int >= length {
                break;
            }
            x1 = *p as libc::c_int >> 4 as libc::c_int;
            x2 = *p as libc::c_int & 15 as libc::c_int;
            *str.offset((i + 0 as libc::c_int) as isize) = '\\' as i32 as libc::c_char;
            *str.offset((i + 1 as libc::c_int) as isize) = '0' as i32 as libc::c_char;
            *str.offset((i + 2 as libc::c_int) as isize) = 'x' as i32 as libc::c_char;
            *str.offset((i + 3 as libc::c_int) as isize) = if x1 > 9 as libc::c_int {
                (x1 - 10 as libc::c_int) + 'a' as i32
            } else {
                (x1) + '0' as i32
            } as libc::c_char;
            *str.offset((i + 4 as libc::c_int) as isize) = if x2 > 9 as libc::c_int {
                (x2 - 10 as libc::c_int) + 'a' as i32
            } else {
                (x2) + '0' as i32
            } as libc::c_char;
            i += 4 as libc::c_int
        } else {
            *str.offset(i as isize) = *p
        }
        i += 1;
        p = p.offset(1)
    }
    *str.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn Field_CompletePlayerName(
    mut names: *mut *const libc::c_char,
    mut nameCount: libc::c_int,
) {
    let mut whitespace: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    matchCount = 0 as libc::c_int;
    shortestMatch[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if nameCount <= 0 as libc::c_int {
        return;
    }
    Name_PlayerNameCompletion(
        names,
        nameCount,
        Some(FindMatches as unsafe extern "C" fn(_: *const libc::c_char) -> ()),
    );
    if *completionString.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
        Com_PlayerNameToFieldString(
            shortestMatch.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            *names.offset(0 as libc::c_int as isize),
        );
    }
    //allow to tab player names
    //if full player name switch to next player name
    if *completionString.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
        && crate::src::qcommon::q_shared::Q_stricmp(shortestMatch.as_mut_ptr(), completionString)
            == 0 as libc::c_int
        && nameCount > 1 as libc::c_int
    {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < nameCount {
            if crate::src::qcommon::q_shared::Q_stricmp(*names.offset(i as isize), completionString)
                == 0 as libc::c_int
            {
                i += 1;
                if i >= nameCount {
                    i = 0 as libc::c_int
                }
                Com_PlayerNameToFieldString(
                    shortestMatch.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                    *names.offset(i as isize),
                );
                break;
            } else {
                i += 1
            }
        }
    }
    if matchCount > 1 as libc::c_int {
        Com_Printf(
            b"]%s\n\x00" as *const u8 as *const libc::c_char,
            (*completionField).buffer.as_mut_ptr(),
        );
        Name_PlayerNameCompletion(
            names,
            nameCount,
            Some(PrintMatches as unsafe extern "C" fn(_: *const libc::c_char) -> ()),
        );
    }
    whitespace = if nameCount == 1 as libc::c_int {
        crate::src::qcommon::q_shared::qtrue as libc::c_int
    } else {
        crate::src::qcommon::q_shared::qfalse as libc::c_int
    } as crate::src::qcommon::q_shared::qboolean;
    (Field_CompletePlayerNameFinal(whitespace) as u64) == 0;
}
#[no_mangle]

pub unsafe extern "C" fn Com_strCompare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut pa: *mut *const libc::c_char = a as *mut *const libc::c_char;
    let mut pb: *mut *const libc::c_char = b as *mut *const libc::c_char;
    return ::libc::strcmp(*pa, *pb);
}
unsafe extern "C" fn run_static_initializers() {
    emptystring = {
        let mut init = memstatic_s {
            b: {
                let mut init = memblock_s {
                    size: ((::std::mem::size_of::<memblock_t>() as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong)
                        & !(3 as libc::c_int) as libc::c_ulong)
                        as libc::c_int,
                    tag: crate::qcommon_h::TAG_STATIC as libc::c_int,
                    next: 0 as *mut memblock_s,
                    prev: 0 as *mut memblock_s,
                    id: 0x1d4a11 as libc::c_int,
                };
                init
            },
            mem: [
                '\u{0}' as i32 as crate::src::qcommon::q_shared::byte,
                '\u{0}' as i32 as crate::src::qcommon::q_shared::byte,
            ],
        };
        init
    };
    numberstring = [
        {
            let mut init = memstatic_s {
                b: {
                    let mut init = memblock_s {
                        size: ((::std::mem::size_of::<memstatic_t>() as libc::c_ulong)
                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                            & !(3 as libc::c_int) as libc::c_ulong)
                            as libc::c_int,
                        tag: crate::qcommon_h::TAG_STATIC as libc::c_int,
                        next: 0 as *mut memblock_s,
                        prev: 0 as *mut memblock_s,
                        id: 0x1d4a11 as libc::c_int,
                    };
                    init
                },
                mem: [
                    '0' as i32 as crate::src::qcommon::q_shared::byte,
                    '\u{0}' as i32 as crate::src::qcommon::q_shared::byte,
                ],
            };
            init
        },
        {
            let mut init = memstatic_s {
                b: {
                    let mut init = memblock_s {
                        size: ((::std::mem::size_of::<memstatic_t>() as libc::c_ulong)
                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                            & !(3 as libc::c_int) as libc::c_ulong)
                            as libc::c_int,
                        tag: crate::qcommon_h::TAG_STATIC as libc::c_int,
                        next: 0 as *mut memblock_s,
                        prev: 0 as *mut memblock_s,
                        id: 0x1d4a11 as libc::c_int,
                    };
                    init
                },
                mem: [
                    '1' as i32 as crate::src::qcommon::q_shared::byte,
                    '\u{0}' as i32 as crate::src::qcommon::q_shared::byte,
                ],
            };
            init
        },
        {
            let mut init = memstatic_s {
                b: {
                    let mut init = memblock_s {
                        size: ((::std::mem::size_of::<memstatic_t>() as libc::c_ulong)
                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                            & !(3 as libc::c_int) as libc::c_ulong)
                            as libc::c_int,
                        tag: crate::qcommon_h::TAG_STATIC as libc::c_int,
                        next: 0 as *mut memblock_s,
                        prev: 0 as *mut memblock_s,
                        id: 0x1d4a11 as libc::c_int,
                    };
                    init
                },
                mem: [
                    '2' as i32 as crate::src::qcommon::q_shared::byte,
                    '\u{0}' as i32 as crate::src::qcommon::q_shared::byte,
                ],
            };
            init
        },
        {
            let mut init = memstatic_s {
                b: {
                    let mut init = memblock_s {
                        size: ((::std::mem::size_of::<memstatic_t>() as libc::c_ulong)
                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                            & !(3 as libc::c_int) as libc::c_ulong)
                            as libc::c_int,
                        tag: crate::qcommon_h::TAG_STATIC as libc::c_int,
                        next: 0 as *mut memblock_s,
                        prev: 0 as *mut memblock_s,
                        id: 0x1d4a11 as libc::c_int,
                    };
                    init
                },
                mem: [
                    '3' as i32 as crate::src::qcommon::q_shared::byte,
                    '\u{0}' as i32 as crate::src::qcommon::q_shared::byte,
                ],
            };
            init
        },
        {
            let mut init = memstatic_s {
                b: {
                    let mut init = memblock_s {
                        size: ((::std::mem::size_of::<memstatic_t>() as libc::c_ulong)
                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                            & !(3 as libc::c_int) as libc::c_ulong)
                            as libc::c_int,
                        tag: crate::qcommon_h::TAG_STATIC as libc::c_int,
                        next: 0 as *mut memblock_s,
                        prev: 0 as *mut memblock_s,
                        id: 0x1d4a11 as libc::c_int,
                    };
                    init
                },
                mem: [
                    '4' as i32 as crate::src::qcommon::q_shared::byte,
                    '\u{0}' as i32 as crate::src::qcommon::q_shared::byte,
                ],
            };
            init
        },
        {
            let mut init = memstatic_s {
                b: {
                    let mut init = memblock_s {
                        size: ((::std::mem::size_of::<memstatic_t>() as libc::c_ulong)
                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                            & !(3 as libc::c_int) as libc::c_ulong)
                            as libc::c_int,
                        tag: crate::qcommon_h::TAG_STATIC as libc::c_int,
                        next: 0 as *mut memblock_s,
                        prev: 0 as *mut memblock_s,
                        id: 0x1d4a11 as libc::c_int,
                    };
                    init
                },
                mem: [
                    '5' as i32 as crate::src::qcommon::q_shared::byte,
                    '\u{0}' as i32 as crate::src::qcommon::q_shared::byte,
                ],
            };
            init
        },
        {
            let mut init = memstatic_s {
                b: {
                    let mut init = memblock_s {
                        size: ((::std::mem::size_of::<memstatic_t>() as libc::c_ulong)
                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                            & !(3 as libc::c_int) as libc::c_ulong)
                            as libc::c_int,
                        tag: crate::qcommon_h::TAG_STATIC as libc::c_int,
                        next: 0 as *mut memblock_s,
                        prev: 0 as *mut memblock_s,
                        id: 0x1d4a11 as libc::c_int,
                    };
                    init
                },
                mem: [
                    '6' as i32 as crate::src::qcommon::q_shared::byte,
                    '\u{0}' as i32 as crate::src::qcommon::q_shared::byte,
                ],
            };
            init
        },
        {
            let mut init = memstatic_s {
                b: {
                    let mut init = memblock_s {
                        size: ((::std::mem::size_of::<memstatic_t>() as libc::c_ulong)
                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                            & !(3 as libc::c_int) as libc::c_ulong)
                            as libc::c_int,
                        tag: crate::qcommon_h::TAG_STATIC as libc::c_int,
                        next: 0 as *mut memblock_s,
                        prev: 0 as *mut memblock_s,
                        id: 0x1d4a11 as libc::c_int,
                    };
                    init
                },
                mem: [
                    '7' as i32 as crate::src::qcommon::q_shared::byte,
                    '\u{0}' as i32 as crate::src::qcommon::q_shared::byte,
                ],
            };
            init
        },
        {
            let mut init = memstatic_s {
                b: {
                    let mut init = memblock_s {
                        size: ((::std::mem::size_of::<memstatic_t>() as libc::c_ulong)
                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                            & !(3 as libc::c_int) as libc::c_ulong)
                            as libc::c_int,
                        tag: crate::qcommon_h::TAG_STATIC as libc::c_int,
                        next: 0 as *mut memblock_s,
                        prev: 0 as *mut memblock_s,
                        id: 0x1d4a11 as libc::c_int,
                    };
                    init
                },
                mem: [
                    '8' as i32 as crate::src::qcommon::q_shared::byte,
                    '\u{0}' as i32 as crate::src::qcommon::q_shared::byte,
                ],
            };
            init
        },
        {
            let mut init = memstatic_s {
                b: {
                    let mut init = memblock_s {
                        size: ((::std::mem::size_of::<memstatic_t>() as libc::c_ulong)
                            .wrapping_add(3 as libc::c_int as libc::c_ulong)
                            & !(3 as libc::c_int) as libc::c_ulong)
                            as libc::c_int,
                        tag: crate::qcommon_h::TAG_STATIC as libc::c_int,
                        next: 0 as *mut memblock_s,
                        prev: 0 as *mut memblock_s,
                        id: 0x1d4a11 as libc::c_int,
                    };
                    init
                },
                mem: [
                    '9' as i32 as crate::src::qcommon::q_shared::byte,
                    '\u{0}' as i32 as crate::src::qcommon::q_shared::byte,
                ],
            };
            init
        },
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

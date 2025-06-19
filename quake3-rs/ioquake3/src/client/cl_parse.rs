use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
        return ::libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as i32,
        ) as i32;
    }
}

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::curl_h::CURL;
pub use crate::multi_h::CURLM;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::svc_EOF;
pub use crate::qcommon_h::svc_bad;
pub use crate::qcommon_h::svc_baseline;
pub use crate::qcommon_h::svc_configstring;
pub use crate::qcommon_h::svc_download;
pub use crate::qcommon_h::svc_gamestate;
pub use crate::qcommon_h::svc_nop;
pub use crate::qcommon_h::svc_ops_e;
pub use crate::qcommon_h::svc_serverCommand;
pub use crate::qcommon_h::svc_snapshot;
pub use crate::qcommon_h::svc_voipOpus;
pub use crate::qcommon_h::svc_voipSpeex;
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
pub use crate::src::qcommon::common::cl_paused;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::cvar::Cvar_Flags;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_SetCheatState;
pub use crate::src::qcommon::cvar::Cvar_SetSafe;
pub use crate::src::qcommon::cvar::Cvar_SetValue;
pub use crate::src::qcommon::cvar::Cvar_VariableString;
pub use crate::src::qcommon::cvar::Cvar_VariableStringBuffer;
pub use crate::src::qcommon::cvar::Cvar_VariableValue;
pub use crate::src::qcommon::files::FS_ConditionalRestart;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_InvalidGameDir;
pub use crate::src::qcommon::files::FS_PureServerSetLoadedPaks;
pub use crate::src::qcommon::files::FS_PureServerSetReferencedPaks;
pub use crate::src::qcommon::files::FS_SV_FOpenFileWrite;
pub use crate::src::qcommon::files::FS_SV_Rename;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::msg::MSG_Bitstream;
pub use crate::src::qcommon::msg::MSG_ReadBigString;
pub use crate::src::qcommon::msg::MSG_ReadBits;
pub use crate::src::qcommon::msg::MSG_ReadByte;
pub use crate::src::qcommon::msg::MSG_ReadData;
pub use crate::src::qcommon::msg::MSG_ReadDeltaEntity;
pub use crate::src::qcommon::msg::MSG_ReadDeltaPlayerstate;
pub use crate::src::qcommon::msg::MSG_ReadLong;
pub use crate::src::qcommon::msg::MSG_ReadShort;
pub use crate::src::qcommon::msg::MSG_ReadString;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Info_NextPair;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::CA_ACTIVE;
pub use crate::src::qcommon::q_shared::CA_AUTHORIZING;
pub use crate::src::qcommon::q_shared::CA_CHALLENGING;
pub use crate::src::qcommon::q_shared::CA_CINEMATIC;
pub use crate::src::qcommon::q_shared::CA_CONNECTED;
pub use crate::src::qcommon::q_shared::CA_CONNECTING;
pub use crate::src::qcommon::q_shared::CA_DISCONNECTED;
pub use crate::src::qcommon::q_shared::CA_LOADING;
pub use crate::src::qcommon::q_shared::CA_PRIMED;
pub use crate::src::qcommon::q_shared::CA_UNINITIALIZED;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint8_t;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;

pub use crate::client_h::clSnapshot_t;
pub use crate::client_h::clientActive_t;
pub use crate::client_h::clientConnection_t;
pub use crate::client_h::clientStatic_t;
pub use crate::client_h::outPacket_t;
pub use crate::client_h::serverInfo_t;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::src::client::cl_console::Con_Close;
pub use crate::src::client::cl_input::CL_WritePacket;
pub use crate::src::client::cl_main::cl;
pub use crate::src::client::cl_main::cl_autoRecordDemo;
pub use crate::src::client::cl_main::cl_oldGame;
pub use crate::src::client::cl_main::cl_oldGameSet;
pub use crate::src::client::cl_main::cl_shownet;
pub use crate::src::client::cl_main::cl_voip;
pub use crate::src::client::cl_main::clc;
pub use crate::src::client::cl_main::cls;
pub use crate::src::client::cl_main::CL_AddReliableCommand;
pub use crate::src::client::cl_main::CL_ClearState;
pub use crate::src::client::cl_main::CL_InitDownloads;
pub use crate::src::client::cl_main::CL_NextDownload;
pub use crate::src::client::cl_main::CL_StopRecord_f;
pub use crate::src::client::cl_parse::stdlib_h::atoi;

pub use ::libc::strtol;
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
// cl_parse.c  -- parse a message received from the server
#[no_mangle]

pub static mut svc_strings: [*mut libc::c_char; 256] = [
    b"svc_bad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_nop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_gamestate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_configstring\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_baseline\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_serverCommand\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_download\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_snapshot\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_EOF\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_voipSpeex\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_voipOpus\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]

pub unsafe extern "C" fn SHOWNET(mut msg: *mut crate::qcommon_h::msg_t, mut s: *mut libc::c_char) {
    if (*crate::src::client::cl_main::cl_shownet).integer >= 2 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"%3i:%s\n\x00" as *const u8 as *const libc::c_char,
            (*msg).readcount - 1 as i32,
            s,
        );
    };
}
/*
=========================================================================

MESSAGE PARSING

=========================================================================
*/
/*
==================
CL_DeltaEntity

Parses deltas from the given base and adds the resulting entity
to the current frame
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_DeltaEntity(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut frame: *mut crate::client_h::clSnapshot_t,
    mut newnum: i32,
    mut old: *mut crate::src::qcommon::q_shared::entityState_t,
    mut unchanged: crate::src::qcommon::q_shared::qboolean,
) {
    let mut state: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    // save the parsed entity state into the big circular buffer so
    // it can be used as the source for a later delta
    state = &mut *crate::src::client::cl_main::cl
        .parseEntities
        .as_mut_ptr()
        .offset(
            (crate::src::client::cl_main::cl.parseEntitiesNum
                & 32 as i32 * 256 as i32 - 1 as i32) as isize,
        ) as *mut crate::src::qcommon::q_shared::entityState_t;
    if unchanged as u64 != 0 {
        *state = *old
    } else {
        crate::src::qcommon::msg::MSG_ReadDeltaEntity(
            msg as *mut crate::qcommon_h::msg_t,
            old as *mut crate::src::qcommon::q_shared::entityState_s,
            state as *mut crate::src::qcommon::q_shared::entityState_s,
            newnum,
        );
    }
    if (*state).number == ((1 as i32) << 10 as i32) - 1 as i32 {
        return;
        // entity was delta removed
    }
    crate::src::client::cl_main::cl.parseEntitiesNum += 1;
    (*frame).numEntities += 1;
}
/*
==================
CL_ParsePacketEntities

==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ParsePacketEntities(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut oldframe: *mut crate::client_h::clSnapshot_t,
    mut newframe: *mut crate::client_h::clSnapshot_t,
) {
    let mut newnum: i32 = 0;
    let mut oldstate: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut oldindex: i32 = 0;
    let mut oldnum: i32 = 0;
    (*newframe).parseEntitiesNum = crate::src::client::cl_main::cl.parseEntitiesNum;
    (*newframe).numEntities = 0 as i32;
    // delta from the entities present in oldframe
    oldindex = 0 as i32;
    oldstate = 0 as *mut crate::src::qcommon::q_shared::entityState_t;
    if oldframe.is_null() {
        oldnum = 99999 as i32
    } else if oldindex >= (*oldframe).numEntities {
        oldnum = 99999 as i32
    } else {
        oldstate = &mut *crate::src::client::cl_main::cl
            .parseEntities
            .as_mut_ptr()
            .offset(
                ((*oldframe).parseEntitiesNum + oldindex
                    & 32 as i32 * 256 as i32 - 1 as i32)
                    as isize,
            ) as *mut crate::src::qcommon::q_shared::entityState_t;
        oldnum = (*oldstate).number
    }
    loop {
        // read the entity index number
        newnum = crate::src::qcommon::msg::MSG_ReadBits(
            msg as *mut crate::qcommon_h::msg_t,
            10 as i32,
        );
        if newnum == ((1 as i32) << 10 as i32) - 1 as i32 {
            break;
        }
        if (*msg).readcount > (*msg).cursize {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"CL_ParsePacketEntities: end of message\x00" as *const u8 as *const libc::c_char,
            );
        }
        while oldnum < newnum {
            // one or more entities from the old packet are unchanged
            if (*crate::src::client::cl_main::cl_shownet).integer == 3 as i32 {
                crate::src::qcommon::common::Com_Printf(
                    b"%3i:  unchanged: %i\n\x00" as *const u8 as *const libc::c_char,
                    (*msg).readcount,
                    oldnum,
                );
            }
            CL_DeltaEntity(
                msg,
                newframe,
                oldnum,
                oldstate,
                crate::src::qcommon::q_shared::qtrue,
            );
            oldindex += 1;
            if oldindex >= (*oldframe).numEntities {
                oldnum = 99999 as i32
            } else {
                oldstate = &mut *crate::src::client::cl_main::cl
                    .parseEntities
                    .as_mut_ptr()
                    .offset(
                        ((*oldframe).parseEntitiesNum + oldindex
                            & 32 as i32 * 256 as i32 - 1 as i32)
                            as isize,
                    )
                    as *mut crate::src::qcommon::q_shared::entityState_t;
                oldnum = (*oldstate).number
            }
        }
        if oldnum == newnum {
            // delta from previous state
            if (*crate::src::client::cl_main::cl_shownet).integer == 3 as i32 {
                crate::src::qcommon::common::Com_Printf(
                    b"%3i:  delta: %i\n\x00" as *const u8 as *const libc::c_char,
                    (*msg).readcount,
                    newnum,
                );
            }
            CL_DeltaEntity(
                msg,
                newframe,
                newnum,
                oldstate,
                crate::src::qcommon::q_shared::qfalse,
            );
            oldindex += 1;
            if oldindex >= (*oldframe).numEntities {
                oldnum = 99999 as i32
            } else {
                oldstate = &mut *crate::src::client::cl_main::cl
                    .parseEntities
                    .as_mut_ptr()
                    .offset(
                        ((*oldframe).parseEntitiesNum + oldindex
                            & 32 as i32 * 256 as i32 - 1 as i32)
                            as isize,
                    )
                    as *mut crate::src::qcommon::q_shared::entityState_t;
                oldnum = (*oldstate).number
            }
        } else {
            if !(oldnum > newnum) {
                continue;
            }
            // delta from baseline
            if (*crate::src::client::cl_main::cl_shownet).integer == 3 as i32 {
                crate::src::qcommon::common::Com_Printf(
                    b"%3i:  baseline: %i\n\x00" as *const u8 as *const libc::c_char,
                    (*msg).readcount,
                    newnum,
                );
            }
            CL_DeltaEntity(
                msg,
                newframe,
                newnum,
                &mut *crate::src::client::cl_main::cl
                    .entityBaselines
                    .as_mut_ptr()
                    .offset(newnum as isize),
                crate::src::qcommon::q_shared::qfalse,
            );
        }
    }
    // any remaining entities in the old frame are copied over
    while oldnum != 99999 as i32 {
        // one or more entities from the old packet are unchanged
        if (*crate::src::client::cl_main::cl_shownet).integer == 3 as i32 {
            crate::src::qcommon::common::Com_Printf(
                b"%3i:  unchanged: %i\n\x00" as *const u8 as *const libc::c_char,
                (*msg).readcount,
                oldnum,
            );
        }
        CL_DeltaEntity(
            msg,
            newframe,
            oldnum,
            oldstate,
            crate::src::qcommon::q_shared::qtrue,
        );
        oldindex += 1;
        if oldindex >= (*oldframe).numEntities {
            oldnum = 99999 as i32
        } else {
            oldstate = &mut *crate::src::client::cl_main::cl
                .parseEntities
                .as_mut_ptr()
                .offset(
                    ((*oldframe).parseEntitiesNum + oldindex
                        & 32 as i32 * 256 as i32 - 1 as i32)
                        as isize,
                ) as *mut crate::src::qcommon::q_shared::entityState_t;
            oldnum = (*oldstate).number
        }
    }
}
/*
================
CL_ParseSnapshot

If the snapshot is parsed properly, it will be copied to
cl.snap and saved in cl.snapshots[].  If the snapshot is invalid
for any reason, no changes to the state will be made at all.
================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ParseSnapshot(mut msg: *mut crate::qcommon_h::msg_t) {
    let mut len: i32 = 0;
    let mut old: *mut crate::client_h::clSnapshot_t = 0 as *mut crate::client_h::clSnapshot_t;
    let mut newSnap: crate::client_h::clSnapshot_t = crate::client_h::clSnapshot_t {
        valid: crate::src::qcommon::q_shared::qfalse,
        snapFlags: 0,
        serverTime: 0,
        messageNum: 0,
        deltaNum: 0,
        ping: 0,
        areamask: [0; 32],
        cmdNum: 0,
        ps: crate::src::qcommon::q_shared::playerState_t {
            commandTime: 0,
            pm_type: 0,
            bobCycle: 0,
            pm_flags: 0,
            pm_time: 0,
            origin: [0.; 3],
            velocity: [0.; 3],
            weaponTime: 0,
            gravity: 0,
            speed: 0,
            delta_angles: [0; 3],
            groundEntityNum: 0,
            legsTimer: 0,
            legsAnim: 0,
            torsoTimer: 0,
            torsoAnim: 0,
            movementDir: 0,
            grapplePoint: [0.; 3],
            eFlags: 0,
            eventSequence: 0,
            events: [0; 2],
            eventParms: [0; 2],
            externalEvent: 0,
            externalEventParm: 0,
            externalEventTime: 0,
            clientNum: 0,
            weapon: 0,
            weaponstate: 0,
            viewangles: [0.; 3],
            viewheight: 0,
            damageEvent: 0,
            damageYaw: 0,
            damagePitch: 0,
            damageCount: 0,
            stats: [0; 16],
            persistant: [0; 16],
            powerups: [0; 16],
            ammo: [0; 16],
            generic1: 0,
            loopSound: 0,
            jumppad_ent: 0,
            ping: 0,
            pmove_framecount: 0,
            jumppad_frame: 0,
            entityEventSequence: 0,
        },
        numEntities: 0,
        parseEntitiesNum: 0,
        serverCommandNum: 0,
    };
    let mut deltaNum: i32 = 0;
    let mut oldMessageNum: i32 = 0;
    let mut i: i32 = 0;
    let mut packetNum: i32 = 0;
    // get the reliable sequence acknowledge number
    // NOTE: now sent with all server to client messages
    //clc.reliableAcknowledge = MSG_ReadLong( msg );
    // read in the new snapshot to a temporary buffer
    // we will only copy to cl.snap if it is valid
    crate::stdlib::memset(
        &mut newSnap as *mut crate::client_h::clSnapshot_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::client_h::clSnapshot_t>() as libc::c_ulong,
    );
    // we will have read any new server commands in this
    // message before we got to svc_snapshot
    newSnap.serverCommandNum = crate::src::client::cl_main::clc.serverCommandSequence;
    newSnap.serverTime =
        crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t);
    // if we were just unpaused, we can only *now* really let the
    // change come into effect or the client hangs.
    (*crate::src::qcommon::common::cl_paused).modified = crate::src::qcommon::q_shared::qfalse;
    newSnap.messageNum = crate::src::client::cl_main::clc.serverMessageSequence;
    deltaNum = crate::src::qcommon::msg::MSG_ReadByte(msg as *mut crate::qcommon_h::msg_t);
    if deltaNum == 0 {
        newSnap.deltaNum = -(1 as i32)
    } else {
        newSnap.deltaNum = newSnap.messageNum - deltaNum
    }
    newSnap.snapFlags = crate::src::qcommon::msg::MSG_ReadByte(msg as *mut crate::qcommon_h::msg_t);
    // If the frame is delta compressed from data that we
    // no longer have available, we must suck up the rest of
    // the frame, but not use it, then ask for a non-compressed
    // message
    if newSnap.deltaNum <= 0 as i32 {
        newSnap.valid = crate::src::qcommon::q_shared::qtrue;
        old = 0 as *mut crate::client_h::clSnapshot_t;
        crate::src::client::cl_main::clc.demowaiting = crate::src::qcommon::q_shared::qfalse
    // uncompressed frame
    // we can start recording now
    } else {
        old = &mut *crate::src::client::cl_main::cl
            .snapshots
            .as_mut_ptr()
            .offset((newSnap.deltaNum & 32 as i32 - 1 as i32) as isize)
            as *mut crate::client_h::clSnapshot_t;
        if (*old).valid as u64 == 0 {
            // should never happen
            crate::src::qcommon::common::Com_Printf(
                b"Delta from invalid frame (not supposed to happen!).\n\x00" as *const u8
                    as *const libc::c_char,
            );
        } else if (*old).messageNum != newSnap.deltaNum {
            // The frame that the server did the delta from
            // is too old, so we can't reconstruct it properly.
            crate::src::qcommon::common::Com_Printf(
                b"Delta frame too old.\n\x00" as *const u8 as *const libc::c_char,
            );
        } else if crate::src::client::cl_main::cl.parseEntitiesNum - (*old).parseEntitiesNum
            > 32 as i32 * 256 as i32 - 256 as i32
        {
            crate::src::qcommon::common::Com_Printf(
                b"Delta parseEntitiesNum too old.\n\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            newSnap.valid = crate::src::qcommon::q_shared::qtrue
            // valid delta parse
        }
    }
    // read areamask
    len = crate::src::qcommon::msg::MSG_ReadByte(msg as *mut crate::qcommon_h::msg_t);
    if len as libc::c_ulong
        > ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 32]>() as libc::c_ulong
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"CL_ParseSnapshot: Invalid size %d for areamask\x00" as *const u8
                as *const libc::c_char,
            len,
        );
    }
    crate::src::qcommon::msg::MSG_ReadData(
        msg as *mut crate::qcommon_h::msg_t,
        &mut newSnap.areamask as *mut [crate::src::qcommon::q_shared::byte; 32]
            as *mut libc::c_void,
        len,
    );
    // read playerinfo
    SHOWNET(
        msg,
        b"playerstate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !old.is_null() {
        crate::src::qcommon::msg::MSG_ReadDeltaPlayerstate(
            msg as *mut crate::qcommon_h::msg_t,
            &mut (*old).ps as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
            &mut newSnap.ps as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
        );
    } else {
        crate::src::qcommon::msg::MSG_ReadDeltaPlayerstate(
            msg as *mut crate::qcommon_h::msg_t,
            0 as *mut crate::src::qcommon::q_shared::playerState_s
                as *mut crate::src::qcommon::q_shared::playerState_s,
            &mut newSnap.ps as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
        );
    }
    // read packet entities
    SHOWNET(
        msg,
        b"packet entities\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    CL_ParsePacketEntities(msg, old, &mut newSnap);
    // if not valid, dump the entire thing now that it has
    // been properly read
    if newSnap.valid as u64 == 0 {
        return;
    }
    // clear the valid flags of any snapshots between the last
    // received and this one, so if there was a dropped packet
    // it won't look like something valid to delta from next
    // time we wrap around in the buffer
    oldMessageNum = crate::src::client::cl_main::cl.snap.messageNum + 1 as i32;
    if newSnap.messageNum - oldMessageNum >= 32 as i32 {
        oldMessageNum = newSnap.messageNum - (32 as i32 - 1 as i32)
    }
    while oldMessageNum < newSnap.messageNum {
        crate::src::client::cl_main::cl.snapshots
            [(oldMessageNum & 32 as i32 - 1 as i32) as usize]
            .valid = crate::src::qcommon::q_shared::qfalse;
        oldMessageNum += 1
    }
    // copy to the current good spot
    crate::src::client::cl_main::cl.snap = newSnap;
    crate::src::client::cl_main::cl.snap.ping = 999 as i32;
    // calculate ping time
    i = 0 as i32;
    while i < 32 as i32 {
        packetNum =
            crate::src::client::cl_main::clc.netchan.outgoingSequence - 1 as i32 - i
                & 32 as i32 - 1 as i32;
        if crate::src::client::cl_main::cl.snap.ps.commandTime
            >= crate::src::client::cl_main::cl.outPackets[packetNum as usize].p_serverTime
        {
            crate::src::client::cl_main::cl.snap.ping = crate::src::client::cl_main::cls.realtime
                - crate::src::client::cl_main::cl.outPackets[packetNum as usize].p_realtime;
            break;
        } else {
            i += 1
        }
    }
    // save the frame off in the backup array for later delta comparisons
    crate::src::client::cl_main::cl.snapshots[(crate::src::client::cl_main::cl.snap.messageNum
        & 32 as i32 - 1 as i32)
        as usize] = crate::src::client::cl_main::cl.snap;
    if (*crate::src::client::cl_main::cl_shownet).integer == 3 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"   snapshot:%i  delta:%i  ping:%i\n\x00" as *const u8 as *const libc::c_char,
            crate::src::client::cl_main::cl.snap.messageNum,
            crate::src::client::cl_main::cl.snap.deltaNum,
            crate::src::client::cl_main::cl.snap.ping,
        );
    }
    crate::src::client::cl_main::cl.newSnapshots = crate::src::qcommon::q_shared::qtrue;
}
//=====================================================================
#[no_mangle]

pub static mut cl_connectedToPureServer: i32 = 0;
#[no_mangle]

pub static mut cl_connectedToCheatServer: i32 = 0;
/*
==================
CL_SystemInfoChanged

The systeminfo configstring has been changed, so parse
new information out of it.  This will happen at every
gamestate, and possibly during gameplay.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_SystemInfoChanged() {
    let mut systemInfo: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: [libc::c_char; 8192] = [0; 8192];
    let mut value: [libc::c_char; 8192] = [0; 8192];
    let mut gameSet: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    systemInfo = crate::src::client::cl_main::cl
        .gameState
        .stringData
        .as_mut_ptr()
        .offset(
            crate::src::client::cl_main::cl.gameState.stringOffsets[1 as i32 as usize]
                as isize,
        );
    // NOTE TTimo:
    // when the serverId changes, any further messages we send to the server will use this new serverId
    // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=475
    // in some cases, outdated cp commands might get sent with this news serverId
    crate::src::client::cl_main::cl.serverId =
        atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            systemInfo,
            b"sv_serverid\x00" as *const u8 as *const libc::c_char,
        ));
    if crate::src::client::cl_main::clc.compat as u64 != 0 {
        crate::src::client::cl_main::clc.voipEnabled = crate::src::qcommon::q_shared::qfalse
    } else {
        s = crate::src::qcommon::q_shared::Info_ValueForKey(
            systemInfo,
            b"sv_voipProtocol\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::client::cl_main::clc.voipEnabled = (crate::src::qcommon::q_shared::Q_stricmp(
            s,
            b"opus\x00" as *const u8 as *const libc::c_char,
        ) == 0) as i32
            as crate::src::qcommon::q_shared::qboolean
    }
    // don't set any vars when playing a demo
    if crate::src::client::cl_main::clc.demoplaying as u64 != 0 {
        return;
    }
    s = crate::src::qcommon::q_shared::Info_ValueForKey(
        systemInfo,
        b"sv_cheats\x00" as *const u8 as *const libc::c_char,
    );
    cl_connectedToCheatServer = atoi(s);
    if cl_connectedToCheatServer == 0 {
        crate::src::qcommon::cvar::Cvar_SetCheatState();
    }
    // check pure server string
    s = crate::src::qcommon::q_shared::Info_ValueForKey(
        systemInfo,
        b"sv_paks\x00" as *const u8 as *const libc::c_char,
    );
    t = crate::src::qcommon::q_shared::Info_ValueForKey(
        systemInfo,
        b"sv_pakNames\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::files::FS_PureServerSetLoadedPaks(s, t);
    s = crate::src::qcommon::q_shared::Info_ValueForKey(
        systemInfo,
        b"sv_referencedPaks\x00" as *const u8 as *const libc::c_char,
    );
    t = crate::src::qcommon::q_shared::Info_ValueForKey(
        systemInfo,
        b"sv_referencedPakNames\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::files::FS_PureServerSetReferencedPaks(s, t);
    gameSet = crate::src::qcommon::q_shared::qfalse;
    // scan through all the variables in the systeminfo and locally set cvars to match
    s = systemInfo;
    while !s.is_null() {
        let mut cvar_flags: i32 = 0;
        crate::src::qcommon::q_shared::Info_NextPair(&mut s, key.as_mut_ptr(), value.as_mut_ptr());
        if key[0 as i32 as usize] == 0 {
            break;
        }
        // ehw!
        if crate::src::qcommon::q_shared::Q_stricmp(
            key.as_mut_ptr(),
            b"fs_game\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if crate::src::qcommon::files::FS_InvalidGameDir(value.as_mut_ptr()) as u64 != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"^3WARNING: Server sent invalid fs_game value %s\n\x00" as *const u8
                        as *const libc::c_char,
                    value.as_mut_ptr(),
                );
                continue;
            } else {
                gameSet = crate::src::qcommon::q_shared::qtrue
            }
        }
        cvar_flags = crate::src::qcommon::cvar::Cvar_Flags(key.as_mut_ptr());
        if cvar_flags as u32 == 0x80000000 as u32 {
            crate::src::qcommon::cvar::Cvar_Get(
                key.as_mut_ptr(),
                value.as_mut_ptr(),
                0x800 as i32 | 0x40 as i32,
            ) as *mut crate::src::qcommon::q_shared::cvar_s;
        } else {
            // If this cvar may not be modified by a server discard the value.
            if cvar_flags & (0x8 as i32 | 0x800 as i32 | 0x80 as i32) == 0 {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    key.as_mut_ptr(),
                    b"g_synchronousClients\x00" as *const u8 as *const libc::c_char,
                ) != 0
                    && crate::src::qcommon::q_shared::Q_stricmp(
                        key.as_mut_ptr(),
                        b"pmove_fixed\x00" as *const u8 as *const libc::c_char,
                    ) != 0
                    && crate::src::qcommon::q_shared::Q_stricmp(
                        key.as_mut_ptr(),
                        b"pmove_msec\x00" as *const u8 as *const libc::c_char,
                    ) != 0
                {
                    crate::src::qcommon::common::Com_Printf(
                        b"^3WARNING: server is not allowed to set %s=%s\n\x00" as *const u8
                            as *const libc::c_char,
                        key.as_mut_ptr(),
                        value.as_mut_ptr(),
                    );
                    continue;
                }
            }
            crate::src::qcommon::cvar::Cvar_SetSafe(key.as_mut_ptr(), value.as_mut_ptr());
        }
    }
    // if game folder should not be set and it is set at the client side
    if gameSet as u64 == 0
        && *crate::src::qcommon::cvar::Cvar_VariableString(
            b"fs_game\x00" as *const u8 as *const libc::c_char,
        ) as i32
            != 0
    {
        crate::src::qcommon::cvar::Cvar_Set(
            b"fs_game\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
    }
    cl_connectedToPureServer = crate::src::qcommon::cvar::Cvar_VariableValue(
        b"sv_pure\x00" as *const u8 as *const libc::c_char,
    ) as i32;
}
/*
==================
CL_ParseServerInfo
==================
*/

unsafe extern "C" fn CL_ParseServerInfo() {
    let mut serverInfo: *const libc::c_char = 0 as *const libc::c_char;
    serverInfo = crate::src::client::cl_main::cl
        .gameState
        .stringData
        .as_mut_ptr()
        .offset(
            crate::src::client::cl_main::cl.gameState.stringOffsets[0 as i32 as usize]
                as isize,
        );
    crate::src::client::cl_main::clc.sv_allowDownload =
        atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            serverInfo,
            b"sv_allowDownload\x00" as *const u8 as *const libc::c_char,
        ));
    crate::src::qcommon::q_shared::Q_strncpyz(
        crate::src::client::cl_main::clc.sv_dlURL.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            serverInfo,
            b"sv_dlURL\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
    );
}
/*
==================
CL_ParseGamestate
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ParseGamestate(mut msg: *mut crate::qcommon_h::msg_t) {
    let mut i: i32 = 0;
    let mut es: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut newnum: i32 = 0;
    let mut nullstate: crate::src::qcommon::q_shared::entityState_t =
        crate::src::qcommon::q_shared::entityState_t {
            number: 0,
            eType: 0,
            eFlags: 0,
            pos: crate::src::qcommon::q_shared::trajectory_t {
                trType: crate::src::qcommon::q_shared::TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            apos: crate::src::qcommon::q_shared::trajectory_t {
                trType: crate::src::qcommon::q_shared::TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            time: 0,
            time2: 0,
            origin: [0.; 3],
            origin2: [0.; 3],
            angles: [0.; 3],
            angles2: [0.; 3],
            otherEntityNum: 0,
            otherEntityNum2: 0,
            groundEntityNum: 0,
            constantLight: 0,
            loopSound: 0,
            modelindex: 0,
            modelindex2: 0,
            clientNum: 0,
            frame: 0,
            solid: 0,
            event: 0,
            eventParm: 0,
            powerups: 0,
            weapon: 0,
            legsAnim: 0,
            torsoAnim: 0,
            generic1: 0,
        };
    let mut cmd: i32 = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldGame: [libc::c_char; 64] = [0; 64];
    crate::src::client::cl_console::Con_Close();
    crate::src::client::cl_main::clc.connectPacketCount = 0 as i32;
    // wipe local client state
    crate::src::client::cl_main::CL_ClearState();
    // a gamestate always marks a server command sequence
    crate::src::client::cl_main::clc.serverCommandSequence =
        crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t);
    // parse all the configstrings and baselines
    crate::src::client::cl_main::cl.gameState.dataCount = 1 as i32; // leave a 0 at the beginning for uninitialized configstrings
    loop {
        cmd = crate::src::qcommon::msg::MSG_ReadByte(msg as *mut crate::qcommon_h::msg_t);
        if cmd == crate::qcommon_h::svc_EOF as i32 {
            break;
        }
        if cmd == crate::qcommon_h::svc_configstring as i32 {
            let mut len: i32 = 0;
            i = crate::src::qcommon::msg::MSG_ReadShort(msg as *mut crate::qcommon_h::msg_t);
            if i < 0 as i32 || i >= 1024 as i32 {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_DROP as i32,
                    b"configstring > MAX_CONFIGSTRINGS\x00" as *const u8 as *const libc::c_char,
                );
            }
            s = crate::src::qcommon::msg::MSG_ReadBigString(msg as *mut crate::qcommon_h::msg_t);
            len = crate::stdlib::strlen(s) as i32;
            if len + 1 as i32 + crate::src::client::cl_main::cl.gameState.dataCount
                > 16000 as i32
            {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_DROP as i32,
                    b"MAX_GAMESTATE_CHARS exceeded\x00" as *const u8 as *const libc::c_char,
                );
            }
            // append it to the gameState string buffer
            crate::src::client::cl_main::cl.gameState.stringOffsets[i as usize] =
                crate::src::client::cl_main::cl.gameState.dataCount;
            crate::stdlib::memcpy(
                crate::src::client::cl_main::cl
                    .gameState
                    .stringData
                    .as_mut_ptr()
                    .offset(crate::src::client::cl_main::cl.gameState.dataCount as isize)
                    as *mut libc::c_void,
                s as *const libc::c_void,
                (len + 1 as i32) as libc::c_ulong,
            );
            crate::src::client::cl_main::cl.gameState.dataCount += len + 1 as i32
        } else if cmd == crate::qcommon_h::svc_baseline as i32 {
            newnum = crate::src::qcommon::msg::MSG_ReadBits(
                msg as *mut crate::qcommon_h::msg_t,
                10 as i32,
            );
            if newnum < 0 as i32 || newnum >= (1 as i32) << 10 as i32 {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_DROP as i32,
                    b"Baseline number out of range: %i\x00" as *const u8 as *const libc::c_char,
                    newnum,
                );
            }
            crate::stdlib::memset(
                &mut nullstate as *mut crate::src::qcommon::q_shared::entityState_t
                    as *mut libc::c_void,
                0 as i32,
                ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>()
                    as libc::c_ulong,
            );
            es = &mut *crate::src::client::cl_main::cl
                .entityBaselines
                .as_mut_ptr()
                .offset(newnum as isize)
                as *mut crate::src::qcommon::q_shared::entityState_t;
            crate::src::qcommon::msg::MSG_ReadDeltaEntity(
                msg as *mut crate::qcommon_h::msg_t,
                &mut nullstate as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
                es as *mut crate::src::qcommon::q_shared::entityState_s,
                newnum,
            );
        } else {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"CL_ParseGamestate: bad command byte\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    crate::src::client::cl_main::clc.clientNum =
        crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t);
    // read the checksum feed
    crate::src::client::cl_main::clc.checksumFeed =
        crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t);
    // save old gamedir
    crate::src::qcommon::cvar::Cvar_VariableStringBuffer(
        b"fs_game\x00" as *const u8 as *const libc::c_char,
        oldGame.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    // parse useful values out of CS_SERVERINFO
    CL_ParseServerInfo();
    // parse serverId and other cvars
    CL_SystemInfoChanged();
    // stop recording now so the demo won't have an unnecessary level load at the end.
    if (*crate::src::client::cl_main::cl_autoRecordDemo).integer != 0
        && crate::src::client::cl_main::clc.demorecording as u32 != 0
    {
        crate::src::client::cl_main::CL_StopRecord_f();
    }
    // reinitialize the filesystem if the game directory has changed
    if crate::src::client::cl_main::cl_oldGameSet as u64 == 0
        && crate::src::qcommon::cvar::Cvar_Flags(b"fs_game\x00" as *const u8 as *const libc::c_char)
            & 0x40000000 as i32
            != 0
    {
        crate::src::client::cl_main::cl_oldGameSet = crate::src::qcommon::q_shared::qtrue;
        crate::src::qcommon::q_shared::Q_strncpyz(
            crate::src::client::cl_main::cl_oldGame.as_mut_ptr(),
            oldGame.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        );
    }
    crate::src::qcommon::files::FS_ConditionalRestart(
        crate::src::client::cl_main::clc.checksumFeed,
        crate::src::qcommon::q_shared::qfalse,
    );
    // This used to call CL_StartHunkUsers, but now we enter the download state before loading the
    // cgame
    crate::src::client::cl_main::CL_InitDownloads();
    // make sure the game starts
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_paused\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
}
//=====================================================================
/*
=====================
CL_ParseDownload

A download message has been received from the server
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ParseDownload(mut msg: *mut crate::qcommon_h::msg_t) {
    let mut size: i32 = 0;
    let mut data: [u8; 16384] = [0; 16384];
    let mut block: crate::stdlib::uint16_t = 0;
    if *crate::src::client::cl_main::clc
        .downloadTempName
        .as_mut_ptr()
        == 0
    {
        crate::src::qcommon::common::Com_Printf(
            b"Server sending download, but no download was requested\n\x00" as *const u8
                as *const libc::c_char,
        );
        crate::src::client::cl_main::CL_AddReliableCommand(
            b"stopdl\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::qfalse,
        );
        return;
    }
    // read the data
    block = crate::src::qcommon::msg::MSG_ReadShort(msg as *mut crate::qcommon_h::msg_t)
        as crate::stdlib::uint16_t;
    if block == 0 && crate::src::client::cl_main::clc.downloadBlock == 0 {
        // block zero is special, contains file size
        crate::src::client::cl_main::clc.downloadSize =
            crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t);
        crate::src::qcommon::cvar::Cvar_SetValue(
            b"cl_downloadSize\x00" as *const u8 as *const libc::c_char,
            crate::src::client::cl_main::clc.downloadSize as f32,
        );
        if crate::src::client::cl_main::clc.downloadSize < 0 as i32 {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"%s\x00" as *const u8 as *const libc::c_char,
                crate::src::qcommon::msg::MSG_ReadString(msg as *mut crate::qcommon_h::msg_t),
            );
        }
    }
    size = crate::src::qcommon::msg::MSG_ReadShort(msg as *mut crate::qcommon_h::msg_t);
    if size < 0 as i32
        || size as libc::c_ulong > ::std::mem::size_of::<[u8; 16384]>() as libc::c_ulong
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"CL_ParseDownload: Invalid size %d for download chunk\x00" as *const u8
                as *const libc::c_char,
            size,
        );
    }
    crate::src::qcommon::msg::MSG_ReadData(
        msg as *mut crate::qcommon_h::msg_t,
        data.as_mut_ptr() as *mut libc::c_void,
        size,
    );
    if crate::src::client::cl_main::clc.downloadBlock & 0xffff as i32
        != block as i32
    {
        crate::src::qcommon::common::Com_DPrintf(
            b"CL_ParseDownload: Expected block %d, got %d\n\x00" as *const u8
                as *const libc::c_char,
            crate::src::client::cl_main::clc.downloadBlock & 0xffff as i32,
            block as i32,
        );
        return;
    }
    // open the file if not opened yet
    if crate::src::client::cl_main::clc.download == 0 {
        crate::src::client::cl_main::clc.download =
            crate::src::qcommon::files::FS_SV_FOpenFileWrite(
                crate::src::client::cl_main::clc
                    .downloadTempName
                    .as_mut_ptr(),
            );
        if crate::src::client::cl_main::clc.download == 0 {
            crate::src::qcommon::common::Com_Printf(
                b"Could not create %s\n\x00" as *const u8 as *const libc::c_char,
                crate::src::client::cl_main::clc
                    .downloadTempName
                    .as_mut_ptr(),
            );
            crate::src::client::cl_main::CL_AddReliableCommand(
                b"stopdl\x00" as *const u8 as *const libc::c_char,
                crate::src::qcommon::q_shared::qfalse,
            );
            crate::src::client::cl_main::CL_NextDownload();
            return;
        }
    }
    if size != 0 {
        crate::src::qcommon::files::FS_Write(
            data.as_mut_ptr() as *const libc::c_void,
            size,
            crate::src::client::cl_main::clc.download,
        );
    }
    crate::src::client::cl_main::CL_AddReliableCommand(
        crate::src::qcommon::q_shared::va(
            b"nextdl %d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::client::cl_main::clc.downloadBlock,
        ),
        crate::src::qcommon::q_shared::qfalse,
    );
    crate::src::client::cl_main::clc.downloadBlock += 1;
    crate::src::client::cl_main::clc.downloadCount += size;
    // So UI gets access to it
    crate::src::qcommon::cvar::Cvar_SetValue(
        b"cl_downloadCount\x00" as *const u8 as *const libc::c_char,
        crate::src::client::cl_main::clc.downloadCount as f32,
    );
    if size == 0 {
        // A zero length block means EOF
        if crate::src::client::cl_main::clc.download != 0 {
            crate::src::qcommon::files::FS_FCloseFile(crate::src::client::cl_main::clc.download);
            crate::src::client::cl_main::clc.download = 0 as i32;
            // rename the file
            crate::src::qcommon::files::FS_SV_Rename(
                crate::src::client::cl_main::clc
                    .downloadTempName
                    .as_mut_ptr(),
                crate::src::client::cl_main::clc.downloadName.as_mut_ptr(),
                crate::src::qcommon::q_shared::qfalse,
            );
        }
        // send intentions now
        // We need this because without it, we would hold the last nextdl and then start
        // loading right away.  If we take a while to load, the server is happily trying
        // to send us that last block over and over.
        // Write it twice to help make sure we acknowledge the download
        crate::src::client::cl_input::CL_WritePacket();
        crate::src::client::cl_input::CL_WritePacket();
        // get another file if needed
        crate::src::client::cl_main::CL_NextDownload(); // too quiet to play.
    }; // VoIP is disabled.
}

unsafe extern "C" fn CL_ShouldIgnoreVoipSender(
    mut sender: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*crate::src::client::cl_main::cl_voip).integer == 0 {
        return crate::src::qcommon::q_shared::qtrue;
    } else {
        if sender == crate::src::client::cl_main::clc.clientNum
            && crate::src::client::cl_main::clc.demoplaying as u64 == 0
        {
            return crate::src::qcommon::q_shared::qtrue;
        } else {
            if crate::src::client::cl_main::clc.voipMuteAll as u64 != 0 {
                // ignore own voice (unless playing back a demo).
                return crate::src::qcommon::q_shared::qtrue;
            } else {
                if crate::src::client::cl_main::clc.voipIgnore[sender as usize] as u64 != 0 {
                    // all channels are muted with extreme prejudice.
                    return crate::src::qcommon::q_shared::qtrue;
                } else {
                    if crate::src::client::cl_main::clc.voipGain[sender as usize] == 0.0f32 {
                        return crate::src::qcommon::q_shared::qtrue;
                    }
                }
            }
        }
    } // just ignoring this guy.
    return crate::src::qcommon::q_shared::qfalse;
}
/*
=====================
CL_PlayVoip

Play raw data
=====================
*/

unsafe extern "C" fn CL_PlayVoip(
    mut sender: i32,
    mut samplecnt: i32,
    mut data: *const crate::src::qcommon::q_shared::byte,
    mut flags: i32,
) {
    if flags & 0x2 as i32 != 0 {
        crate::src::client::snd_main::S_RawSamples(
            sender + 1 as i32,
            samplecnt,
            48000 as i32,
            2 as i32,
            1 as i32,
            data,
            crate::src::client::cl_main::clc.voipGain[sender as usize],
            -(1 as i32),
        );
    }
    if flags & 0x1 as i32 != 0 {
        crate::src::client::snd_main::S_RawSamples(
            sender + 64 as i32 + 1 as i32,
            samplecnt,
            48000 as i32,
            2 as i32,
            1 as i32,
            data,
            1.0f32,
            sender,
        );
    };
}
/*
=====================
CL_ParseVoip

A VoIP message has been received from the server
=====================
*/

unsafe extern "C" fn CL_ParseVoip(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut ignoreData: crate::src::qcommon::q_shared::qboolean,
) {
    static mut decoded: [i16; 11520] = [0; 11520]; // !!! FIXME: don't hard code
    let sender: i32 =
        crate::src::qcommon::msg::MSG_ReadShort(msg as *mut crate::qcommon_h::msg_t); // short/invalid packet, bail.
    let generation: i32 =
        crate::src::qcommon::msg::MSG_ReadByte(msg as *mut crate::qcommon_h::msg_t); // short/invalid packet, bail.
    let sequence: i32 =
        crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t); // short/invalid packet, bail.
    let frames: i32 =
        crate::src::qcommon::msg::MSG_ReadByte(msg as *mut crate::qcommon_h::msg_t); // short/invalid packet, bail.
    let packetsize: i32 =
        crate::src::qcommon::msg::MSG_ReadShort(msg as *mut crate::qcommon_h::msg_t); // short/invalid packet, bail.
    let flags: i32 = crate::src::qcommon::msg::MSG_ReadBits(
        msg as *mut crate::qcommon_h::msg_t,
        2 as i32,
    );
    let mut encoded: [u8; 4000] = [0; 4000];
    let mut numSamples: i32 = 0;
    let mut seqdiff: i32 = 0;
    let mut written: i32 = 0 as i32;
    let mut i: i32 = 0;
    crate::src::qcommon::common::Com_DPrintf(
        b"VoIP: %d-byte packet from client %d\n\x00" as *const u8 as *const libc::c_char,
        packetsize,
        sender,
    );
    if sender < 0 as i32 {
        return;
    } else {
        if generation < 0 as i32 {
            return;
        } else {
            if sequence < 0 as i32 {
                return;
            } else {
                if frames < 0 as i32 {
                    return;
                } else {
                    if packetsize < 0 as i32 {
                        return;
                    }
                }
            }
        }
    }
    if packetsize as libc::c_ulong > ::std::mem::size_of::<[u8; 4000]>() as libc::c_ulong
    {
        // overlarge packet?
        let mut bytesleft: i32 = packetsize;
        while bytesleft != 0 {
            let mut br: i32 = bytesleft;
            if br as libc::c_ulong > ::std::mem::size_of::<[u8; 4000]>() as libc::c_ulong
            {
                br = ::std::mem::size_of::<[u8; 4000]>() as libc::c_ulong as i32
            }
            crate::src::qcommon::msg::MSG_ReadData(
                msg as *mut crate::qcommon_h::msg_t,
                encoded.as_mut_ptr() as *mut libc::c_void,
                br,
            );
            bytesleft -= br
        }
        return;
        // overlarge packet, bail.
    }
    crate::src::qcommon::msg::MSG_ReadData(
        msg as *mut crate::qcommon_h::msg_t,
        encoded.as_mut_ptr() as *mut libc::c_void,
        packetsize,
    );
    if ignoreData as u64 != 0 {
        return;
    // just ignore legacy speex voip data
    } else {
        if crate::src::client::cl_main::clc.voipCodecInitialized as u64 == 0 {
            return;
        // can't handle VoIP without libopus!
        } else {
            if sender >= 64 as i32 {
                return;
            // bogus sender.
            } else {
                if CL_ShouldIgnoreVoipSender(sender) as u64 != 0 {
                    return;
                    // Channel is muted, bail.
                }
            }
        }
    }
    // !!! FIXME: make sure data is narrowband? Does decoder handle this?
    crate::src::qcommon::common::Com_DPrintf(
        b"VoIP: packet accepted!\n\x00" as *const u8 as *const libc::c_char,
    );
    seqdiff = sequence - crate::src::client::cl_main::clc.voipIncomingSequence[sender as usize];
    // This is a new "generation" ... a new recording started, reset the bits.
    if generation
        != crate::src::client::cl_main::clc.voipIncomingGeneration[sender as usize] as i32
    {
        crate::src::qcommon::common::Com_DPrintf(
            b"VoIP: new generation %d!\n\x00" as *const u8 as *const libc::c_char,
            generation,
        );
        crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_ctl(
            crate::src::client::cl_main::clc.opusDecoder[sender as usize],
            4028 as i32,
        );
        crate::src::client::cl_main::clc.voipIncomingGeneration[sender as usize] =
            generation as crate::src::qcommon::q_shared::byte;
        seqdiff = 0 as i32
    } else if seqdiff < 0 as i32 {
        // we're ahead of the sequence?!
        // This shouldn't happen unless the packet is corrupted or something.
        crate::src::qcommon::common::Com_DPrintf(
            b"VoIP: misordered sequence! %d < %d!\n\x00" as *const u8 as *const libc::c_char,
            sequence,
            crate::src::client::cl_main::clc.voipIncomingSequence[sender as usize],
        );
        // reset the decoder just in case.
        crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_ctl(
            crate::src::client::cl_main::clc.opusDecoder[sender as usize],
            4028 as i32,
        );
        seqdiff = 0 as i32
    } else if (seqdiff
        * (20 as i32 * 48 as i32 * 3 as i32)
        * 2 as i32) as libc::c_ulong
        >= ::std::mem::size_of::<[i16; 11520]>() as libc::c_ulong
    {
        // dropped more than we can handle?
        // just start over.
        crate::src::qcommon::common::Com_DPrintf(
            b"VoIP: Dropped way too many (%d) frames from client #%d\n\x00" as *const u8
                as *const libc::c_char,
            seqdiff,
            sender,
        );
        crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_ctl(
            crate::src::client::cl_main::clc.opusDecoder[sender as usize],
            4028 as i32,
        );
        seqdiff = 0 as i32
    }
    if seqdiff != 0 as i32 {
        crate::src::qcommon::common::Com_DPrintf(
            b"VoIP: Dropped %d frames from client #%d\n\x00" as *const u8 as *const libc::c_char,
            seqdiff,
            sender,
        );
        // tell opus that we're missing frames...
        i = 0 as i32;
        while i < seqdiff {
            numSamples = crate::src::opus_1_2_1::src::opus_decoder::opus_decode(
                crate::src::client::cl_main::clc.opusDecoder[sender as usize],
                0 as *const u8,
                0 as i32,
                decoded.as_mut_ptr().offset(written as isize),
                20 as i32 * 48 as i32 * 3 as i32,
                0 as i32,
            );
            if numSamples <= 0 as i32 {
                crate::src::qcommon::common::Com_DPrintf(
                    b"VoIP: Error decoding frame %d from client #%d\n\x00" as *const u8
                        as *const libc::c_char,
                    i,
                    sender,
                );
            } else {
                written += numSamples
            }
            i += 1
        }
    }
    numSamples = crate::src::opus_1_2_1::src::opus_decoder::opus_decode(
        crate::src::client::cl_main::clc.opusDecoder[sender as usize],
        encoded.as_mut_ptr(),
        packetsize,
        decoded.as_mut_ptr().offset(written as isize),
        (::std::mem::size_of::<[i16; 11520]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<i16>() as libc::c_ulong)
            .wrapping_sub(written as libc::c_ulong) as i32,
        0 as i32,
    );
    if numSamples <= 0 as i32 {
        crate::src::qcommon::common::Com_DPrintf(
            b"VoIP: Error decoding voip data from client #%d\n\x00" as *const u8
                as *const libc::c_char,
            sender,
        );
        numSamples = 0 as i32
    }
    written += numSamples;
    crate::src::qcommon::common::Com_DPrintf(
        b"VoIP: playback %d bytes, %d samples, %d frames\n\x00" as *const u8 as *const libc::c_char,
        written * 2 as i32,
        written,
        frames,
    );
    if written > 0 as i32 {
        CL_PlayVoip(
            sender,
            written,
            decoded.as_mut_ptr() as *const crate::src::qcommon::q_shared::byte,
            flags,
        );
    }
    crate::src::client::cl_main::clc.voipIncomingSequence[sender as usize] = sequence + frames;
}
/*
=====================
CL_ParseCommandString

Command strings are just saved off until cgame asks for them
when it transitions a snapshot
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ParseCommandString(mut msg: *mut crate::qcommon_h::msg_t) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut seq: i32 = 0;
    let mut index: i32 = 0;
    seq = crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t);
    s = crate::src::qcommon::msg::MSG_ReadString(msg as *mut crate::qcommon_h::msg_t);
    // see if we have already executed stored it off
    if crate::src::client::cl_main::clc.serverCommandSequence >= seq {
        return;
    }
    crate::src::client::cl_main::clc.serverCommandSequence = seq;
    index = seq & 64 as i32 - 1 as i32;
    crate::src::qcommon::q_shared::Q_strncpyz(
        crate::src::client::cl_main::clc.serverCommands[index as usize].as_mut_ptr(),
        s,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
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
// client.h -- primary header for client
/* USE_CURL */
// file full of random crap that gets used to create cl_guid
// time between connection packet retransmits
// snapshots are a view of the server at a given time
// cleared if delta parsing was invalid
// rate delayed and dropped commands
// server time the message is valid for (in msec)
// copied from netchan->incoming_sequence
// messageNum the delta is from
// time from when cmdNum-1 was sent to time packet was reeceived
// portalarea visibility bits
// the next cmdNum the server is expecting
// complete information about the current player at this time
// all of the entities that need to be presented
// at the time of this snapshot
// execute all commands up to this before
// making the snapshot current
/*
=============================================================================

the clientActive_t structure is wiped completely at every
new gamestate_t, potentially several times during an established connection

=============================================================================
*/
// cl.cmdNumber when packet was sent
// usercmd->serverTime when packet was sent
// cls.realtime when packet was sent
// the parseEntities array must be large enough to hold PACKET_BACKUP frames of
// entities, so that when a delta compressed message arives from the server
// it can be un-deltad from the original
// it requres several frames in a timeout condition
// to disconnect, preventing debugging breaks from
// causing immediate disconnects on continue
// latest received from server
// may be paused during play
// to prevent time from flowing bakcwards
// to check tournament restarts
// cl.serverTime = cls.realtime + cl.serverTimeDelta
// this value changes as net lag varies
// set if any cgame frame has been forced to extrapolate
// cleared when CL_AdjustTimeDelta looks at it
// set on parse of any valid packet
// configstrings
// extracted from CS_SERVERINFO
// index (not anded off) into cl_parse_entities[]
// added to by mouse events
// set by joystick events
// cgame communicates a few values to the client system
// current weapon to add to usercmd_t
// cmds[cmdNumber] is the predicted command, [cmdNumber-1] is the last
// properly generated command
// each mesage will send several old cmds
// incremented each frame, because multiple
// frames may need to be packed into a single packet
// information about each packet we have sent out
// the client maintains its own idea of view angles, which are
// sent to the server each frame.  It is cleared to 0 upon entering each level.
// the server sends a delta each frame which is added to the locally
// tracked view angles to account for standing on rotating objects,
// and teleport direction changes
// included in each client message so the server
// can tell if it is for a prior map_restart
// big stuff at end of structure so most offsets are 15 bits or less
// for delta compression when not in previous frame
/*
=============================================================================

the clientConnection_t structure is wiped when disconnecting from a server,
either to go to a full screen console, play a demo, or connect to a different server

A connection can be to either a server through the network layer or a
demo through a file.

=============================================================================
*/
// connection status
// for retransmits during connection
// for timeouts
// name of server from original connect (used by reconnect)
// for connection retransmits
// for display on connection dialog
// for display on connection dialog
// from the server to use for connecting
// from the server for checksum calculations
// these are our reliable messages that go to the server
// the last one the server has executed
// server message (unreliable) and command (reliable) sequence
// numbers are NOT cleared at level changes, but continue to
// increase as long as the connection is valid
// message sequence is used by both the network layer and the
// delta compression layer
// reliable messages received from server
// last server command grabbed or executed with CL_GetServerCommand
// file transfer from server
/* USE_CURL */
// block we are waiting for
// how many bytes we got
// how many bytes we got
// list of paks we need to download
// if true, we need to do another FS_Restart because we downloaded a pak
// demo information
// don't record until a non-delta message is received
// counter of rendered frames
// cls.realtime before first frame
// each frame will be at this time + frameNum * 50
// time the last frame was rendered
// minimum frame duration
// maximum frame duration
// log of frame durations
// incoming data...
// !!! FIXME: convert from parallel arrays to array of a struct.
// outgoing data...
// if voipTargets[i / 8] & (1 << (i % 8)),
// then we are sending to clientnum i.
// big stuff at end of structure so most offsets are 15 bits or less
/*
==================================================================

the clientStatic_t structure is never wiped, and is used even when
no client connection is active at all
(except when CL_Shutdown is called)

==================================================================
*/
// bring up the cd needed dialog next frame
// when the server clears the hunk, all of these must be restarted
// msec since last frame
// ignores pause
// ignoring pause, so console always works
// additional global servers
// source currently pinging or updating
// update server info
// rendering info
//=============================================================================
// interface to cgame dll or vm
// interface to ui dll or vm
// interface to refresh .dll
//
// cvars
//
// cl_voipSendTarget is a string: "all" to broadcast to everyone, "none" to
//  send to no one, or a comma-separated list of client numbers:
//  "0,7,2,23" ... an empty string is treated like "all".
// 20ms at 48k
// 3 frame is 60ms of audio, the max opus will encode at once
//=================================================
//
// cl_main
//
//
// cl_input
//
// key nums holding it down
// msec timestamp
// msec down this frame if both a down and up happened
// current state
// set when down, not cleared when up
//
// cl_parse.c
//
/*
=====================
CL_ParseServerMessage
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ParseServerMessage(mut msg: *mut crate::qcommon_h::msg_t) {
    let mut cmd: i32 = 0;
    if (*crate::src::client::cl_main::cl_shownet).integer == 1 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"%i \x00" as *const u8 as *const libc::c_char,
            (*msg).cursize,
        );
    } else if (*crate::src::client::cl_main::cl_shownet).integer >= 2 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"------------------\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::src::qcommon::msg::MSG_Bitstream(msg as *mut crate::qcommon_h::msg_t);
    // get the reliable sequence acknowledge number
    crate::src::client::cl_main::clc.reliableAcknowledge =
        crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t);
    //
    if crate::src::client::cl_main::clc.reliableAcknowledge
        < crate::src::client::cl_main::clc.reliableSequence - 64 as i32
    {
        crate::src::client::cl_main::clc.reliableAcknowledge =
            crate::src::client::cl_main::clc.reliableSequence
    }
    loop
    //
    // parse the message
    //
    {
        if (*msg).readcount > (*msg).cursize {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"CL_ParseServerMessage: read past end of server message\x00" as *const u8
                    as *const libc::c_char,
            );
        } else {
            cmd = crate::src::qcommon::msg::MSG_ReadByte(msg as *mut crate::qcommon_h::msg_t);
            if cmd == crate::qcommon_h::svc_EOF as i32 {
                SHOWNET(
                    msg,
                    b"END OF MESSAGE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                break;
            } else {
                if (*crate::src::client::cl_main::cl_shownet).integer >= 2 as i32 {
                    if cmd < 0 as i32 || svc_strings[cmd as usize].is_null() {
                        crate::src::qcommon::common::Com_Printf(
                            b"%3i:BAD CMD %i\n\x00" as *const u8 as *const libc::c_char,
                            (*msg).readcount - 1 as i32,
                            cmd,
                        );
                    } else {
                        SHOWNET(msg, svc_strings[cmd as usize]);
                    }
                }
                // other commands
                match cmd {
                    1 => {}
                    5 => {
                        CL_ParseCommandString(msg);
                    }
                    2 => {
                        CL_ParseGamestate(msg);
                    }
                    7 => {
                        CL_ParseSnapshot(msg);
                    }
                    6 => {
                        CL_ParseDownload(msg);
                    }
                    9 => {
                        CL_ParseVoip(msg, crate::src::qcommon::q_shared::qtrue);
                    }
                    10 => {
                        CL_ParseVoip(
                            msg,
                            (crate::src::client::cl_main::clc.voipEnabled as u64 == 0)
                                as i32
                                as crate::src::qcommon::q_shared::qboolean,
                        );
                    }
                    _ => {
                        crate::src::qcommon::common::Com_Error(
                            crate::src::qcommon::q_shared::ERR_DROP as i32,
                            b"CL_ParseServerMessage: Illegible server message\x00" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
            }
        }
    }
}

use ::libc;

pub mod qcommon_h {

    #[inline]

    pub unsafe extern "C" fn _vmf(mut x: crate::stdlib::intptr_t) -> f32 {
        let mut fi: crate::src::qcommon::q_shared::floatint_t =
            crate::src::qcommon::q_shared::floatint_t { f: 0. };
        fi.i = x as i32;
        return fi.f;
    }

    // _QCOMMON_H_
    // flags for sv_allowDownload and cl_allowDownload
}

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
        return libc::strtol(
            __nptr,
            std::ptr::null_mut() as *mut *mut libc::c_char,
            10 as i32,
        ) as i32;
    }
}

pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::uint8_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::cg_public_h::snapshot_t;
pub use crate::cg_public_h::CG_ACOS;
pub use crate::cg_public_h::CG_ADDCOMMAND;
pub use crate::cg_public_h::CG_ARGC;
pub use crate::cg_public_h::CG_ARGS;
pub use crate::cg_public_h::CG_ARGV;
pub use crate::cg_public_h::CG_ATAN2;
pub use crate::cg_public_h::CG_CEIL;
pub use crate::cg_public_h::CG_CIN_DRAWCINEMATIC;
pub use crate::cg_public_h::CG_CIN_PLAYCINEMATIC;
pub use crate::cg_public_h::CG_CIN_RUNCINEMATIC;
pub use crate::cg_public_h::CG_CIN_SETEXTENTS;
pub use crate::cg_public_h::CG_CIN_STOPCINEMATIC;
pub use crate::cg_public_h::CG_CM_BOXTRACE;
pub use crate::cg_public_h::CG_CM_CAPSULETRACE;
pub use crate::cg_public_h::CG_CM_INLINEMODEL;
pub use crate::cg_public_h::CG_CM_LOADMAP;
pub use crate::cg_public_h::CG_CM_LOADMODEL;
pub use crate::cg_public_h::CG_CM_MARKFRAGMENTS;
pub use crate::cg_public_h::CG_CM_NUMINLINEMODELS;
pub use crate::cg_public_h::CG_CM_POINTCONTENTS;
pub use crate::cg_public_h::CG_CM_TEMPBOXMODEL;
pub use crate::cg_public_h::CG_CM_TEMPCAPSULEMODEL;
pub use crate::cg_public_h::CG_CM_TRANSFORMEDBOXTRACE;
pub use crate::cg_public_h::CG_CM_TRANSFORMEDCAPSULETRACE;
pub use crate::cg_public_h::CG_CM_TRANSFORMEDPOINTCONTENTS;
pub use crate::cg_public_h::CG_CONSOLE_COMMAND;
pub use crate::cg_public_h::CG_COS;
pub use crate::cg_public_h::CG_CROSSHAIR_PLAYER;
pub use crate::cg_public_h::CG_CVAR_REGISTER;
pub use crate::cg_public_h::CG_CVAR_SET;
pub use crate::cg_public_h::CG_CVAR_UPDATE;
pub use crate::cg_public_h::CG_CVAR_VARIABLESTRINGBUFFER;
pub use crate::cg_public_h::CG_DRAW_ACTIVE_FRAME;
pub use crate::cg_public_h::CG_ERROR;
pub use crate::cg_public_h::CG_EVENT_HANDLING;
pub use crate::cg_public_h::CG_FLOOR;
pub use crate::cg_public_h::CG_FS_FCLOSEFILE;
pub use crate::cg_public_h::CG_FS_FOPENFILE;
pub use crate::cg_public_h::CG_FS_READ;
pub use crate::cg_public_h::CG_FS_SEEK;
pub use crate::cg_public_h::CG_FS_WRITE;
pub use crate::cg_public_h::CG_GETCURRENTCMDNUMBER;
pub use crate::cg_public_h::CG_GETCURRENTSNAPSHOTNUMBER;
pub use crate::cg_public_h::CG_GETGAMESTATE;
pub use crate::cg_public_h::CG_GETGLCONFIG;
pub use crate::cg_public_h::CG_GETSERVERCOMMAND;
pub use crate::cg_public_h::CG_GETSNAPSHOT;
pub use crate::cg_public_h::CG_GETUSERCMD;
pub use crate::cg_public_h::CG_GET_ENTITY_TOKEN;
pub use crate::cg_public_h::CG_INIT;
pub use crate::cg_public_h::CG_KEY_EVENT;
pub use crate::cg_public_h::CG_KEY_GETCATCHER;
pub use crate::cg_public_h::CG_KEY_GETKEY;
pub use crate::cg_public_h::CG_KEY_ISDOWN;
pub use crate::cg_public_h::CG_KEY_SETCATCHER;
pub use crate::cg_public_h::CG_LAST_ATTACKER;
pub use crate::cg_public_h::CG_MEMCPY;
pub use crate::cg_public_h::CG_MEMORY_REMAINING;
pub use crate::cg_public_h::CG_MEMSET;
pub use crate::cg_public_h::CG_MILLISECONDS;
pub use crate::cg_public_h::CG_MOUSE_EVENT;
pub use crate::cg_public_h::CG_PC_ADD_GLOBAL_DEFINE;
pub use crate::cg_public_h::CG_PC_FREE_SOURCE;
pub use crate::cg_public_h::CG_PC_LOAD_SOURCE;
pub use crate::cg_public_h::CG_PC_READ_TOKEN;
pub use crate::cg_public_h::CG_PC_SOURCE_FILE_AND_LINE;
pub use crate::cg_public_h::CG_PRINT;
pub use crate::cg_public_h::CG_REAL_TIME;
pub use crate::cg_public_h::CG_REMOVECOMMAND;
pub use crate::cg_public_h::CG_R_ADDADDITIVELIGHTTOSCENE;
pub use crate::cg_public_h::CG_R_ADDLIGHTTOSCENE;
pub use crate::cg_public_h::CG_R_ADDPOLYSTOSCENE;
pub use crate::cg_public_h::CG_R_ADDPOLYTOSCENE;
pub use crate::cg_public_h::CG_R_ADDREFENTITYTOSCENE;
pub use crate::cg_public_h::CG_R_CLEARSCENE;
pub use crate::cg_public_h::CG_R_DRAWSTRETCHPIC;
pub use crate::cg_public_h::CG_R_INPVS;
pub use crate::cg_public_h::CG_R_LERPTAG;
pub use crate::cg_public_h::CG_R_LIGHTFORPOINT;
pub use crate::cg_public_h::CG_R_LOADWORLDMAP;
pub use crate::cg_public_h::CG_R_MODELBOUNDS;
pub use crate::cg_public_h::CG_R_REGISTERFONT;
pub use crate::cg_public_h::CG_R_REGISTERMODEL;
pub use crate::cg_public_h::CG_R_REGISTERSHADER;
pub use crate::cg_public_h::CG_R_REGISTERSHADERNOMIP;
pub use crate::cg_public_h::CG_R_REGISTERSKIN;
pub use crate::cg_public_h::CG_R_REMAP_SHADER;
pub use crate::cg_public_h::CG_R_RENDERSCENE;
pub use crate::cg_public_h::CG_R_SETCOLOR;
pub use crate::cg_public_h::CG_SENDCLIENTCOMMAND;
pub use crate::cg_public_h::CG_SENDCONSOLECOMMAND;
pub use crate::cg_public_h::CG_SETUSERCMDVALUE;
pub use crate::cg_public_h::CG_SHUTDOWN;
pub use crate::cg_public_h::CG_SIN;
pub use crate::cg_public_h::CG_SNAPVECTOR;
pub use crate::cg_public_h::CG_SQRT;
pub use crate::cg_public_h::CG_STRNCPY;
pub use crate::cg_public_h::CG_S_ADDLOOPINGSOUND;
pub use crate::cg_public_h::CG_S_ADDREALLOOPINGSOUND;
pub use crate::cg_public_h::CG_S_CLEARLOOPINGSOUNDS;
pub use crate::cg_public_h::CG_S_REGISTERSOUND;
pub use crate::cg_public_h::CG_S_RESPATIALIZE;
pub use crate::cg_public_h::CG_S_STARTBACKGROUNDTRACK;
pub use crate::cg_public_h::CG_S_STARTLOCALSOUND;
pub use crate::cg_public_h::CG_S_STARTSOUND;
pub use crate::cg_public_h::CG_S_STOPBACKGROUNDTRACK;
pub use crate::cg_public_h::CG_S_STOPLOOPINGSOUND;
pub use crate::cg_public_h::CG_S_UPDATEENTITYPOSITION;
pub use crate::cg_public_h::CG_TESTPRINTFLOAT;
pub use crate::cg_public_h::CG_TESTPRINTINT;
pub use crate::cg_public_h::CG_UPDATESCREEN;
pub use crate::curl_h::CURL;
pub use crate::multi_h::CURLM;
pub use crate::opus_types_h::opus_int32;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::vmInterpret_t;
pub use crate::qcommon_h::vm_t;
pub use crate::qcommon_h::xcommand_t;
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
pub use crate::qcommon_h::VMI_BYTECODE;
pub use crate::qcommon_h::VMI_COMPILED;
pub use crate::qcommon_h::VMI_NATIVE;
pub use crate::src::asm::snapvector::qsnapvectorsse;
pub use crate::src::client::cl_cgame::qcommon_h::_vmf;
pub use crate::src::qcommon::cmd::Cbuf_AddText;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_ArgsBuffer;
pub use crate::src::qcommon::cmd::Cmd_ArgsFrom;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::cmd::Cmd_ArgvBuffer;
pub use crate::src::qcommon::cmd::Cmd_RemoveCommandSafe;
pub use crate::src::qcommon::cmd::Cmd_TokenizeString;
pub use crate::src::qcommon::common::com_sv_running;
pub use crate::src::qcommon::common::com_timescale;
pub use crate::src::qcommon::common::sv_paused;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Com_RealTime;
pub use crate::src::qcommon::common::Com_TouchMemory;
pub use crate::src::qcommon::common::Hunk_MemoryRemaining;
pub use crate::src::qcommon::cvar::Cvar_Register;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_SetCheatState;
pub use crate::src::qcommon::cvar::Cvar_SetSafe;
pub use crate::src::qcommon::cvar::Cvar_Update;
pub use crate::src::qcommon::cvar::Cvar_VariableStringBuffer;
pub use crate::src::qcommon::cvar::Cvar_VariableValue;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_FOpenFileByMode;
pub use crate::src::qcommon::files::FS_Read;
pub use crate::src::qcommon::files::FS_Seek;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::q_math::Q_acos;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::floatint_t;
pub use crate::src::qcommon::q_shared::fontInfo_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::glyphInfo_t;
pub use crate::src::qcommon::q_shared::markFragment_t;
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::pc_token_s;
pub use crate::src::qcommon::q_shared::pc_token_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtime_s;
pub use crate::src::qcommon::q_shared::qtime_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
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
pub use crate::src::qcommon::q_shared::FMV_EOF;
pub use crate::src::qcommon::q_shared::FMV_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_BLT;
pub use crate::src::qcommon::q_shared::FMV_ID_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_WAIT;
pub use crate::src::qcommon::q_shared::FMV_LOOPED;
pub use crate::src::qcommon::q_shared::FMV_PLAY;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::qcommon::vm::VM_ArgPtr;
pub use crate::src::qcommon::vm::VM_Call;
pub use crate::src::qcommon::vm::VM_Create;
pub use crate::src::qcommon::vm::VM_Debug;
pub use crate::src::qcommon::vm::VM_Free;
pub use crate::src::sys::sys_unix::Sys_LowPhysicalMemory;
pub use crate::src::sys::sys_unix::Sys_Milliseconds;
pub use crate::tr_public_h::refexport_t;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::polyVert_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
pub use crate::tr_types_h::stereoFrame_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::RT_BEAM;
pub use crate::tr_types_h::RT_LIGHTNING;
pub use crate::tr_types_h::RT_MAX_REF_ENTITY_TYPE;
pub use crate::tr_types_h::RT_MODEL;
pub use crate::tr_types_h::RT_POLY;
pub use crate::tr_types_h::RT_PORTALSURFACE;
pub use crate::tr_types_h::RT_RAIL_CORE;
pub use crate::tr_types_h::RT_RAIL_RINGS;
pub use crate::tr_types_h::RT_SPRITE;
pub use crate::tr_types_h::STEREO_CENTER;
pub use crate::tr_types_h::STEREO_LEFT;
pub use crate::tr_types_h::STEREO_RIGHT;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
pub use crate::vm_local_h::vm_s;

pub use crate::be_aas_h::aas_altroutegoal_s;
pub use crate::be_aas_h::aas_areainfo_s;
pub use crate::be_aas_h::aas_clientmove_s;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_predictroute_s;
pub use crate::botlib_h::aas_export_s;
pub use crate::botlib_h::aas_export_t;
pub use crate::botlib_h::ai_export_s;
pub use crate::botlib_h::ai_export_t;
pub use crate::botlib_h::bot_entitystate_s;
pub use crate::botlib_h::bot_entitystate_t;
pub use crate::botlib_h::bot_input_s;
pub use crate::botlib_h::bot_input_t;
pub use crate::botlib_h::botlib_export_s;
pub use crate::botlib_h::botlib_export_t;
pub use crate::botlib_h::ea_export_s;
pub use crate::botlib_h::ea_export_t;
pub use crate::client_h::clSnapshot_t;
pub use crate::client_h::clientActive_t;
pub use crate::client_h::clientConnection_t;
pub use crate::client_h::clientStatic_t;
pub use crate::client_h::outPacket_t;
pub use crate::client_h::serverInfo_t;
pub use crate::src::botlib::be_ai_chat::bot_consolemessage_s;
pub use crate::src::botlib::be_ai_chat::bot_match_s;
pub use crate::src::botlib::be_ai_goal::bot_goal_s;
pub use crate::src::botlib::be_ai_move::bot_initmove_s;
pub use crate::src::botlib::be_ai_move::bot_moveresult_s;
pub use crate::src::botlib::be_ai_weap::weaponinfo_s;
pub use crate::src::client::cl_cgame::stdlib_h::atoi;
pub use crate::src::client::cl_cin::CIN_DrawCinematic;
pub use crate::src::client::cl_cin::CIN_PlayCinematic;
pub use crate::src::client::cl_cin::CIN_RunCinematic;
pub use crate::src::client::cl_cin::CIN_SetExtents;
pub use crate::src::client::cl_cin::CIN_StopCinematic;
pub use crate::src::client::cl_console::Con_ClearNotify;
pub use crate::src::client::cl_console::Con_Close;
pub use crate::src::client::cl_keys::Key_GetCatcher;

pub use crate::src::client::cl_keys::Key_SetCatcher;
pub use crate::src::client::cl_main::cgvm;
pub use crate::src::client::cl_main::cl;
pub use crate::src::client::cl_main::cl_activeAction;
pub use crate::src::client::cl_main::cl_freezeDemo;
pub use crate::src::client::cl_main::cl_showTimeDelta;
pub use crate::src::client::cl_main::cl_timeNudge;
pub use crate::src::client::cl_main::cl_timedemo;
pub use crate::src::client::cl_main::cl_useMumble;
pub use crate::src::client::cl_main::clc;
pub use crate::src::client::cl_main::cls;
pub use crate::src::client::cl_main::re;
pub use crate::src::client::cl_main::CL_AddReliableCommand;
pub use crate::src::client::cl_main::CL_CheckPaused;
pub use crate::src::client::cl_main::CL_ReadDemoMessage;
pub use crate::src::client::cl_main::CL_Voip_f;
pub use crate::src::client::cl_parse::cl_connectedToCheatServer;
pub use crate::src::client::cl_parse::cl_connectedToPureServer;
pub use crate::src::client::cl_parse::CL_SystemInfoChanged;
pub use crate::src::client::cl_scrn::SCR_UpdateScreen;

pub use ::libc::abs;

pub use ::libc::strtol;
extern "C" {
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
    // cl_cgame.c  -- client system interaction with client game
    #[no_mangle]
    pub static mut botlib_export: *mut botlib_export_t;
}
/*
====================
CL_GetGameState
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GetGameState(mut gs: *mut gameState_t) {
    *gs = cl.gameState;
}
/*
====================
CL_GetGlconfig
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GetGlconfig(mut glconfig: *mut glconfig_t) {
    *glconfig = cls.glconfig;
}
/*
====================
CL_GetUserCmd
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GetUserCmd(mut cmdNumber: i32, mut ucmd: *mut usercmd_t) -> qboolean {
    // cmds[cmdNumber] is the last properly generated command
    // can't return anything that we haven't created yet
    if cmdNumber > cl.cmdNumber {
        Com_Error(
            ERR_DROP as i32,
            b"CL_GetUserCmd: %i >= %i\x00" as *const u8 as *const libc::c_char,
            cmdNumber,
            cl.cmdNumber,
        );
    }
    // the usercmd has been overwritten in the wrapping
    // buffer because it is too far out of date
    if cmdNumber <= cl.cmdNumber - 64 as i32 {
        return qfalse;
    }
    *ucmd = cl.cmds[(cmdNumber & 64 as i32 - 1 as i32) as usize];
    return qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CL_GetCurrentCmdNumber() -> i32 {
    return cl.cmdNumber;
}
/*
====================
CL_GetParseEntityState
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GetParseEntityState(
    mut parseEntityNumber: i32,
    mut state: *mut entityState_t,
) -> qboolean {
    // can't return anything that hasn't been parsed yet
    if parseEntityNumber >= cl.parseEntitiesNum {
        Com_Error(
            ERR_DROP as i32,
            b"CL_GetParseEntityState: %i >= %i\x00" as *const u8 as *const libc::c_char,
            parseEntityNumber,
            cl.parseEntitiesNum,
        );
    }
    // can't return anything that has been overwritten in the circular buffer
    if parseEntityNumber <= cl.parseEntitiesNum - 32 as i32 * 256 as i32 {
        return qfalse;
    }
    *state = cl.parseEntities[(parseEntityNumber & 32 as i32 * 256 as i32 - 1 as i32) as usize];
    return qtrue;
}
/*
====================
CL_GetCurrentSnapshotNumber
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GetCurrentSnapshotNumber(
    mut snapshotNumber: *mut i32,
    mut serverTime: *mut i32,
) {
    *snapshotNumber = cl.snap.messageNum;
    *serverTime = cl.snap.serverTime;
}
/*
====================
CL_GetSnapshot
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GetSnapshot(
    mut snapshotNumber: i32,
    mut snapshot: *mut snapshot_t,
) -> qboolean {
    let mut clSnap: *mut clSnapshot_t = std::ptr::null_mut();
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    if snapshotNumber > cl.snap.messageNum {
        Com_Error(
            ERR_DROP as i32,
            b"CL_GetSnapshot: snapshotNumber > cl.snapshot.messageNum\x00" as *const u8
                as *const libc::c_char,
        );
    }
    // if the frame has fallen out of the circular buffer, we can't return it
    if cl.snap.messageNum - snapshotNumber >= 32 as i32 {
        return qfalse;
    }
    // if the frame is not valid, we can't return it
    clSnap = &mut *cl
        .snapshots
        .as_mut_ptr()
        .offset((snapshotNumber & 32 as i32 - 1 as i32) as isize) as *mut clSnapshot_t;
    if (*clSnap).valid as u64 == 0 {
        return qfalse;
    }
    // if the entities in the frame have fallen out of their
    // circular buffer, we can't return it
    if cl.parseEntitiesNum - (*clSnap).parseEntitiesNum >= 32 as i32 * 256 as i32 {
        return qfalse;
    }
    // write the snapshot
    (*snapshot).snapFlags = (*clSnap).snapFlags;
    (*snapshot).serverCommandSequence = (*clSnap).serverCommandNum;
    (*snapshot).ping = (*clSnap).ping;
    (*snapshot).serverTime = (*clSnap).serverTime;
    crate::stdlib::memcpy(
        (*snapshot).areamask.as_mut_ptr() as *mut libc::c_void,
        (*clSnap).areamask.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[byte; 32]>() as usize,
    );
    (*snapshot).ps = (*clSnap).ps;
    count = (*clSnap).numEntities;
    if count > 256 as i32 {
        Com_DPrintf(
            b"CL_GetSnapshot: truncated %i entities to %i\n\x00" as *const u8
                as *const libc::c_char,
            count,
            256 as i32,
        );
        count = 256 as i32
    }
    (*snapshot).numEntities = count;
    i = 0 as i32;
    while i < count {
        (*snapshot).entities[i as usize] = cl.parseEntities
            [((*clSnap).parseEntitiesNum + i & 32 as i32 * 256 as i32 - 1 as i32) as usize];
        i += 1
    }
    // FIXME: configstring changes and server commands!!!
    return qtrue;
}
/*
=====================
CL_SetUserCmdValue
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_SetUserCmdValue(mut userCmdValue: i32, mut sensitivityScale: f32) {
    cl.cgameUserCmdValue = userCmdValue;
    cl.cgameSensitivity = sensitivityScale;
}
/*
=====================
CL_AddCgameCommand
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_AddCgameCommand(mut cmdName: *const libc::c_char) {
    Cmd_AddCommand(cmdName, None);
}
/*
=====================
CL_ConfigstringModified
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ConfigstringModified() {
    let mut old: *mut libc::c_char = std::ptr::null_mut();
    let mut s: *mut libc::c_char = std::ptr::null_mut();
    let mut i: i32 = 0;
    let mut index: i32 = 0;
    let mut dup: *mut libc::c_char = std::ptr::null_mut();
    let mut oldGs: gameState_t = gameState_t {
        stringOffsets: [0; 1024],
        stringData: [0; 16000],
        dataCount: 0,
    };
    let mut len: i32 = 0;
    index = atoi(Cmd_Argv(1 as i32));
    if index < 0 as i32 || index >= 1024 as i32 {
        Com_Error(
            ERR_DROP as i32,
            b"CL_ConfigstringModified: bad index %i\x00" as *const u8 as *const libc::c_char,
            index,
        );
    }
    // get everything after "cs <num>"
    s = Cmd_ArgsFrom(2 as i32);
    old = cl
        .gameState
        .stringData
        .as_mut_ptr()
        .offset(cl.gameState.stringOffsets[index as usize] as isize);
    if libc::strcmp(old, s) == 0 {
        return;
        // unchanged
    }
    // build the new gameState_t
    oldGs = cl.gameState;
    crate::stdlib::memset(
        &mut cl.gameState as *mut gameState_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<gameState_t>() as usize,
    );
    // leave the first 0 for uninitialized strings
    cl.gameState.dataCount = 1 as i32;
    i = 0 as i32;
    while i < 1024 as i32 {
        if i == index {
            dup = s
        } else {
            dup = oldGs
                .stringData
                .as_mut_ptr()
                .offset(oldGs.stringOffsets[i as usize] as isize)
        }
        if !(*dup.offset(0 as i32 as isize) == 0) {
            len = crate::stdlib::strlen(dup) as i32;
            if len + 1 as i32 + cl.gameState.dataCount > 16000 as i32 {
                Com_Error(
                    ERR_DROP as i32,
                    b"MAX_GAMESTATE_CHARS exceeded\x00" as *const u8 as *const libc::c_char,
                );
            }
            // append it to the gameState string buffer
            cl.gameState.stringOffsets[i as usize] = cl.gameState.dataCount;
            crate::stdlib::memcpy(
                cl.gameState
                    .stringData
                    .as_mut_ptr()
                    .offset(cl.gameState.dataCount as isize) as *mut libc::c_void,
                dup as *const libc::c_void,
                (len + 1 as i32) as usize,
            );
            cl.gameState.dataCount += len + 1 as i32
        }
        i += 1
        // leave with the default empty string
    }
    if index == 1 as i32 {
        // parse serverId and other cvars
        CL_SystemInfoChanged();
    };
}
/*
===================
CL_GetServerCommand

Set up argc/argv for the given command
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GetServerCommand(mut serverCommandNumber: i32) -> qboolean {
    let mut s: *mut libc::c_char = std::ptr::null_mut();
    let mut cmd: *mut libc::c_char = std::ptr::null_mut();
    static mut bigConfigString: [libc::c_char; 8192] = [0; 8192];
    let mut argc: i32 = 0;
    // if we have irretrievably lost a reliable command, drop the connection
    if serverCommandNumber <= clc.serverCommandSequence - 64 as i32 {
        // when a demo record was started after the client got a whole bunch of
        // reliable commands then the client never got those first reliable commands
        if clc.demoplaying as u64 != 0 {
            return qfalse;
        }
        Com_Error(
            ERR_DROP as i32,
            b"CL_GetServerCommand: a reliable command was cycled out\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if serverCommandNumber > clc.serverCommandSequence {
        Com_Error(
            ERR_DROP as i32,
            b"CL_GetServerCommand: requested a command not received\x00" as *const u8
                as *const libc::c_char,
        );
    }
    s = clc.serverCommands[(serverCommandNumber & 64 as i32 - 1 as i32) as usize].as_mut_ptr();
    clc.lastExecutedServerCommand = serverCommandNumber;
    Com_DPrintf(
        b"serverCommand: %i : %s\n\x00" as *const u8 as *const libc::c_char,
        serverCommandNumber,
        s,
    );
    loop {
        Cmd_TokenizeString(s);
        cmd = Cmd_Argv(0 as i32);
        argc = Cmd_Argc();
        if libc::strcmp(cmd, b"disconnect\x00" as *const u8 as *const libc::c_char) == 0 {
            // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=552
            // allow server to indicate why they were disconnected
            if argc >= 2 as i32 {
                Com_Error(
                    ERR_SERVERDISCONNECT as i32,
                    b"Server disconnected - %s\x00" as *const u8 as *const libc::c_char,
                    Cmd_Argv(1 as i32),
                );
            } else {
                Com_Error(
                    ERR_SERVERDISCONNECT as i32,
                    b"Server disconnected\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        if libc::strcmp(cmd, b"bcs0\x00" as *const u8 as *const libc::c_char) == 0 {
            Com_sprintf(
                bigConfigString.as_mut_ptr(),
                8192 as i32,
                b"cs %s \"%s\x00" as *const u8 as *const libc::c_char,
                Cmd_Argv(1 as i32),
                Cmd_Argv(2 as i32),
            );
            return qfalse;
        }
        if libc::strcmp(cmd, b"bcs1\x00" as *const u8 as *const libc::c_char) == 0 {
            s = Cmd_Argv(2 as i32);
            if crate::stdlib::strlen(bigConfigString.as_mut_ptr())
                .wrapping_add(crate::stdlib::strlen(s))
                >= 8192 as i32 as usize
            {
                Com_Error(
                    ERR_DROP as i32,
                    b"bcs exceeded BIG_INFO_STRING\x00" as *const u8 as *const libc::c_char,
                );
            }
            libc::strcat(bigConfigString.as_mut_ptr(), s);
            return qfalse;
        }
        if !(libc::strcmp(cmd, b"bcs2\x00" as *const u8 as *const libc::c_char) == 0) {
            break;
        }
        s = Cmd_Argv(2 as i32);
        if crate::stdlib::strlen(bigConfigString.as_mut_ptr())
            .wrapping_add(crate::stdlib::strlen(s))
            .wrapping_add(1 as i32 as usize)
            >= 8192 as i32 as usize
        {
            Com_Error(
                ERR_DROP as i32,
                b"bcs exceeded BIG_INFO_STRING\x00" as *const u8 as *const libc::c_char,
            );
        }
        libc::strcat(bigConfigString.as_mut_ptr(), s);
        libc::strcat(
            bigConfigString.as_mut_ptr(),
            b"\"\x00" as *const u8 as *const libc::c_char,
        );
        s = bigConfigString.as_mut_ptr()
    }
    if libc::strcmp(cmd, b"cs\x00" as *const u8 as *const libc::c_char) == 0 {
        CL_ConfigstringModified();
        // reparse the string, because CL_ConfigstringModified may have done another Cmd_TokenizeString()
        Cmd_TokenizeString(s);
        return qtrue;
    }
    if libc::strcmp(cmd, b"map_restart\x00" as *const u8 as *const libc::c_char) == 0 {
        // clear notify lines and outgoing commands before passing
        // the restart to the cgame
        Con_ClearNotify();
        // reparse the string, because Con_ClearNotify() may have done another Cmd_TokenizeString()
        Cmd_TokenizeString(s);
        crate::stdlib::memset(
            cl.cmds.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<[usercmd_t; 64]>() as usize,
        );
        return qtrue;
    }
    // the clientLevelShot command is used during development
    // to generate 128*128 screenshots from the intermission
    // point of levels for the menu system to use
    // we pass it along to the cgame to make appropriate adjustments,
    // but we also clear the console and notify lines here
    if libc::strcmp(
        cmd,
        b"clientLevelShot\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        // don't do it if we aren't running the server locally,
        // otherwise malicious remote servers could overwrite
        // the existing thumbnails
        if (*com_sv_running).integer == 0 {
            return qfalse;
        }
        // close the console
        Con_Close();
        // take a special screenshot next frame
        Cbuf_AddText(
            b"wait ; wait ; wait ; wait ; screenshot levelshot\n\x00" as *const u8
                as *const libc::c_char,
        );
        return qtrue;
    }
    // we may want to put a "connect to other server" command here
    // cgame can now act on the command
    return qtrue;
}
/*
====================
CL_CM_LoadMap

Just adds default parameters that cgame doesn't need to know about
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CM_LoadMap(mut mapname: *const libc::c_char) {
    let mut checksum: i32 = 0;
    crate::src::qcommon::cm_load::CM_LoadMap(mapname, qtrue, &mut checksum);
}
/*
====================
CL_ShutdonwCGame

====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ShutdownCGame() {
    Key_SetCatcher(Key_GetCatcher() & !(0x8 as i32));
    cls.cgameStarted = qfalse;
    if cgvm.is_null() {
        return;
    }
    VM_Call(cgvm, CG_SHUTDOWN as i32);
    VM_Free(cgvm);
    cgvm = std::ptr::null_mut();
}

unsafe extern "C" fn FloatAsInt(mut f: f32) -> i32 {
    let mut fi: floatint_t = floatint_t { f: 0. };
    fi.f = f;
    return fi.i;
}
/*
====================
CL_CgameSystemCalls

The cgame module is making a system call
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CgameSystemCalls(mut args: *mut intptr_t) -> intptr_t {
    match *args.offset(0 as i32 as isize) {
        0 => {
            Com_Printf(
                b"%s\x00" as *const u8 as *const libc::c_char,
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        1 => {
            Com_Error(
                ERR_DROP as i32,
                b"%s\x00" as *const u8 as *const libc::c_char,
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
            );
        }
        2 => return Sys_Milliseconds() as intptr_t,
        3 => {
            Cvar_Register(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vmCvar_t as *mut vmCvar_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const libc::c_char,
                *args.offset(4 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        4 => {
            Cvar_Update(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vmCvar_t as *mut vmCvar_t
            );
            return 0 as i32 as intptr_t;
        }
        5 => {
            Cvar_SetSafe(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        6 => {
            Cvar_VariableStringBuffer(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        7 => return Cmd_Argc() as intptr_t,
        8 => {
            Cmd_ArgvBuffer(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        9 => {
            Cmd_ArgsBuffer(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut libc::c_char,
                *args.offset(2 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        10 => {
            return FS_FOpenFileByMode(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut fileHandle_t,
                *args.offset(3 as i32 as isize) as fsMode_t,
            ) as intptr_t
        }
        11 => {
            FS_Read(
                VM_ArgPtr(*args.offset(1 as i32 as isize)),
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as fileHandle_t,
            );
            return 0 as i32 as intptr_t;
        }
        12 => {
            FS_Write(
                VM_ArgPtr(*args.offset(1 as i32 as isize)),
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as fileHandle_t,
            );
            return 0 as i32 as intptr_t;
        }
        13 => {
            FS_FCloseFile(*args.offset(1 as i32 as isize) as fileHandle_t);
            return 0 as i32 as intptr_t;
        }
        89 => {
            return FS_Seek(
                *args.offset(1 as i32 as isize) as fileHandle_t,
                *args.offset(2 as i32 as isize),
                *args.offset(3 as i32 as isize) as i32,
            ) as intptr_t
        }
        14 => {
            Cbuf_AddText(VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char);
            return 0 as i32 as intptr_t;
        }
        15 => {
            CL_AddCgameCommand(VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char);
            return 0 as i32 as intptr_t;
        }
        72 => {
            Cmd_RemoveCommandSafe(VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char);
            return 0 as i32 as intptr_t;
        }
        16 => {
            CL_AddReliableCommand(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                qfalse,
            );
            return 0 as i32 as intptr_t;
        }
        17 => {
            // this is used during lengthy level loading, so pump message loop
            //		Com_EventLoop();	// FIXME: if a server restarts here, BAD THINGS HAPPEN!
            // We can't call Com_EventLoop here, a restart will crash and this _does_ happen
            // if there is a map change while we are downloading at pk3.
            // ZOID
            SCR_UpdateScreen();
            return 0 as i32 as intptr_t;
        }
        18 => {
            CL_CM_LoadMap(VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char);
            return 0 as i32 as intptr_t;
        }
        19 => return crate::src::qcommon::cm_load::CM_NumInlineModels() as intptr_t,
        20 => {
            return crate::src::qcommon::cm_load::CM_InlineModel(
                *args.offset(1 as i32 as isize) as i32
            ) as intptr_t
        }
        22 => {
            return crate::src::qcommon::cm_load::CM_TempBoxModel(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
                qfalse as i32,
            ) as intptr_t
        }
        82 => {
            return crate::src::qcommon::cm_load::CM_TempBoxModel(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
                qtrue as i32,
            ) as intptr_t
        }
        23 => {
            return crate::src::qcommon::cm_test::CM_PointContents(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const vec_t,
                *args.offset(2 as i32 as isize) as clipHandle_t,
            ) as intptr_t
        }
        24 => {
            return crate::src::qcommon::cm_test::CM_TransformedPointContents(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const vec_t,
                *args.offset(2 as i32 as isize) as clipHandle_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *const vec_t,
            ) as intptr_t
        }
        25 => {
            crate::src::qcommon::cm_trace::CM_BoxTrace(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut trace_t as *mut trace_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *mut vec_t,
                *args.offset(6 as i32 as isize) as clipHandle_t,
                *args.offset(7 as i32 as isize) as i32,
                qfalse as i32,
            );
            return 0 as i32 as intptr_t;
        }
        83 => {
            crate::src::qcommon::cm_trace::CM_BoxTrace(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut trace_t as *mut trace_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *mut vec_t,
                *args.offset(6 as i32 as isize) as clipHandle_t,
                *args.offset(7 as i32 as isize) as i32,
                qtrue as i32,
            );
            return 0 as i32 as intptr_t;
        }
        26 => {
            crate::src::qcommon::cm_trace::CM_TransformedBoxTrace(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut trace_t as *mut trace_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *mut vec_t,
                *args.offset(6 as i32 as isize) as clipHandle_t,
                *args.offset(7 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(8 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(9 as i32 as isize)) as *const vec_t,
                qfalse as i32,
            );
            return 0 as i32 as intptr_t;
        }
        84 => {
            crate::src::qcommon::cm_trace::CM_TransformedBoxTrace(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut trace_t as *mut trace_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *mut vec_t,
                *args.offset(6 as i32 as isize) as clipHandle_t,
                *args.offset(7 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(8 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(9 as i32 as isize)) as *const vec_t,
                qtrue as i32,
            );
            return 0 as i32 as intptr_t;
        }
        27 => {
            return re.MarkFragments.expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec3_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const vec_t,
                *args.offset(4 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(5 as i32 as isize)) as *mut vec_t,
                *args.offset(6 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(7 as i32 as isize)) as *mut markFragment_t,
            ) as intptr_t
        }
        28 => {
            crate::src::client::snd_main::S_StartSound(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t,
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as i32,
                *args.offset(4 as i32 as isize) as sfxHandle_t,
            );
            return 0 as i32 as intptr_t;
        }
        29 => {
            crate::src::client::snd_main::S_StartLocalSound(
                *args.offset(1 as i32 as isize) as sfxHandle_t,
                *args.offset(2 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        30 => {
            crate::src::client::snd_main::S_ClearLoopingSounds(
                *args.offset(1 as i32 as isize) as qboolean
            );
            return 0 as i32 as intptr_t;
        }
        31 => {
            crate::src::client::snd_main::S_AddLoopingSound(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const vec_t,
                *args.offset(4 as i32 as isize) as sfxHandle_t,
            );
            return 0 as i32 as intptr_t;
        }
        80 => {
            crate::src::client::snd_main::S_AddRealLoopingSound(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const vec_t,
                *args.offset(4 as i32 as isize) as sfxHandle_t,
            );
            return 0 as i32 as intptr_t;
        }
        81 => {
            crate::src::client::snd_main::S_StopLoopingSound(*args.offset(1 as i32 as isize) as i32);
            return 0 as i32 as intptr_t;
        }
        32 => {
            crate::src::client::snd_main::S_UpdateEntityPosition(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
            );
            return 0 as i32 as intptr_t;
        }
        33 => {
            crate::src::client::snd_main::S_Respatialize(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec3_t,
                *args.offset(4 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        34 => {
            return crate::src::client::snd_main::S_RegisterSound(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                *args.offset(2 as i32 as isize) as qboolean,
            ) as intptr_t
        }
        35 => {
            crate::src::client::snd_main::S_StartBackgroundTrack(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        36 => {
            re.LoadWorld.expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            ) as *const libc::c_char);
            return 0 as i32 as intptr_t;
        }
        37 => {
            return re.RegisterModel.expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *const libc::c_char) as intptr_t
        }
        38 => {
            return re.RegisterSkin.expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *const libc::c_char) as intptr_t
        }
        39 => {
            return re.RegisterShader.expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *const libc::c_char) as intptr_t
        }
        57 => {
            return re.RegisterShaderNoMip.expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *const libc::c_char) as intptr_t
        }
        59 => {
            re.RegisterFont.expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut fontInfo_t,
            );
            return 0 as i32 as intptr_t;
        }
        40 => {
            re.ClearScene.expect("non-null function pointer")();
            return 0 as i32 as intptr_t;
        }
        41 => {
            re.AddRefEntityToScene.expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *const refEntity_t);
            return 0 as i32 as intptr_t;
        }
        42 => {
            re.AddPolyToScene.expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as qhandle_t,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const polyVert_t,
                1 as i32,
            );
            return 0 as i32 as intptr_t;
        }
        87 => {
            re.AddPolyToScene.expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as qhandle_t,
                *args.offset(2 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const polyVert_t,
                *args.offset(4 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        73 => {
            return re.LightForPoint.expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut vec_t,
            ) as intptr_t
        }
        43 => {
            re.AddLightToScene.expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const vec_t,
                _vmf(*args.offset(2 as i32 as isize)),
                _vmf(*args.offset(3 as i32 as isize)),
                _vmf(*args.offset(4 as i32 as isize)),
                _vmf(*args.offset(5 as i32 as isize)),
            );
            return 0 as i32 as intptr_t;
        }
        85 => {
            re.AddAdditiveLightToScene
                .expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const vec_t,
                _vmf(*args.offset(2 as i32 as isize)),
                _vmf(*args.offset(3 as i32 as isize)),
                _vmf(*args.offset(4 as i32 as isize)),
                _vmf(*args.offset(5 as i32 as isize)),
            );
            return 0 as i32 as intptr_t;
        }
        44 => {
            re.RenderScene.expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            ) as *const refdef_t);
            return 0 as i32 as intptr_t;
        }
        45 => {
            re.SetColor.expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            ) as *const f32);
            return 0 as i32 as intptr_t;
        }
        46 => {
            re.DrawStretchPic.expect("non-null function pointer")(
                _vmf(*args.offset(1 as i32 as isize)),
                _vmf(*args.offset(2 as i32 as isize)),
                _vmf(*args.offset(3 as i32 as isize)),
                _vmf(*args.offset(4 as i32 as isize)),
                _vmf(*args.offset(5 as i32 as isize)),
                _vmf(*args.offset(6 as i32 as isize)),
                _vmf(*args.offset(7 as i32 as isize)),
                _vmf(*args.offset(8 as i32 as isize)),
                *args.offset(9 as i32 as isize) as qhandle_t,
            );
            return 0 as i32 as intptr_t;
        }
        47 => {
            re.ModelBounds.expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as qhandle_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut vec_t,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut vec_t,
            );
            return 0 as i32 as intptr_t;
        }
        48 => {
            return re.LerpTag.expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut orientation_t,
                *args.offset(2 as i32 as isize) as qhandle_t,
                *args.offset(3 as i32 as isize) as i32,
                *args.offset(4 as i32 as isize) as i32,
                _vmf(*args.offset(5 as i32 as isize)),
                VM_ArgPtr(*args.offset(6 as i32 as isize)) as *const libc::c_char,
            ) as intptr_t
        }
        49 => {
            CL_GetGlconfig(VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut glconfig_t);
            return 0 as i32 as intptr_t;
        }
        50 => {
            CL_GetGameState(VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut gameState_t);
            return 0 as i32 as intptr_t;
        }
        51 => {
            CL_GetCurrentSnapshotNumber(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut i32,
            );
            return 0 as i32 as intptr_t;
        }
        52 => {
            return CL_GetSnapshot(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut snapshot_t,
            ) as intptr_t
        }
        53 => return CL_GetServerCommand(*args.offset(1 as i32 as isize) as i32) as intptr_t,
        54 => return CL_GetCurrentCmdNumber() as intptr_t,
        55 => {
            return CL_GetUserCmd(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut usercmd_t,
            ) as intptr_t
        }
        56 => {
            CL_SetUserCmdValue(
                *args.offset(1 as i32 as isize) as i32,
                _vmf(*args.offset(2 as i32 as isize)),
            );
            return 0 as i32 as intptr_t;
        }
        58 => return Hunk_MemoryRemaining() as intptr_t,
        60 => {
            return crate::src::client::cl_keys::Key_IsDown(*args.offset(1 as i32 as isize) as i32)
                as intptr_t
        }
        61 => return Key_GetCatcher() as intptr_t,
        62 => {
            // Don't allow the cgame module to close the console
            Key_SetCatcher(
                (*args.offset(1 as i32 as isize) | (Key_GetCatcher() & 0x1 as i32) as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        63 => {
            return crate::src::client::cl_keys::Key_GetKey(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            ) as *const libc::c_char) as intptr_t
        }
        100 => {
            crate::stdlib::memset(
                VM_ArgPtr(*args.offset(1 as i32 as isize)),
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as usize,
            );
            return 0 as i32 as intptr_t;
        }
        101 => {
            crate::stdlib::memcpy(
                VM_ArgPtr(*args.offset(1 as i32 as isize)),
                VM_ArgPtr(*args.offset(2 as i32 as isize)),
                *args.offset(3 as i32 as isize) as usize,
            );
            return 0 as i32 as intptr_t;
        }
        102 => {
            crate::stdlib::strncpy(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
                *args.offset(3 as i32 as isize) as usize,
            );
            return *args.offset(1 as i32 as isize);
        }
        103 => {
            return FloatAsInt(
                crate::stdlib::sin(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as intptr_t
        }
        104 => {
            return FloatAsInt(
                crate::stdlib::cos(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as intptr_t
        }
        105 => {
            return FloatAsInt(crate::stdlib::atan2(
                _vmf(*args.offset(1 as i32 as isize)) as f64,
                _vmf(*args.offset(2 as i32 as isize)) as f64,
            ) as f32) as intptr_t
        }
        106 => {
            return FloatAsInt(
                crate::stdlib::sqrt(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as intptr_t
        }
        107 => {
            return FloatAsInt(
                crate::stdlib::floor(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as intptr_t
        }
        108 => {
            return FloatAsInt(
                crate::stdlib::ceil(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as intptr_t
        }
        111 => return FloatAsInt(Q_acos(_vmf(*args.offset(1 as i32 as isize)))) as intptr_t,
        64 => {
            return (*botlib_export)
                .PC_AddGlobalDefine
                .expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *mut libc::c_char) as intptr_t
        }
        65 => {
            return (*botlib_export)
                .PC_LoadSourceHandle
                .expect("non-null function pointer")(VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *const libc::c_char) as intptr_t
        }
        66 => {
            return (*botlib_export)
                .PC_FreeSourceHandle
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            ) as intptr_t
        }
        67 => {
            return (*botlib_export)
                .PC_ReadTokenHandle
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut pc_token_t,
            ) as intptr_t
        }
        68 => {
            return (*botlib_export)
                .PC_SourceFileAndLine
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *mut libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut i32,
            ) as intptr_t
        }
        69 => {
            crate::src::client::snd_main::S_StopBackgroundTrack();
            return 0 as i32 as intptr_t;
        }
        70 => {
            return Com_RealTime(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut qtime_t as *mut qtime_s
            ) as intptr_t
        }
        71 => {
            qsnapvectorsse(VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut vec_t);
            return 0 as i32 as intptr_t;
        }
        74 => {
            return CIN_PlayCinematic(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as i32,
                *args.offset(4 as i32 as isize) as i32,
                *args.offset(5 as i32 as isize) as i32,
                *args.offset(6 as i32 as isize) as i32,
            ) as intptr_t
        }
        75 => return CIN_StopCinematic(*args.offset(1 as i32 as isize) as i32) as intptr_t,
        76 => return CIN_RunCinematic(*args.offset(1 as i32 as isize) as i32) as intptr_t,
        77 => {
            CIN_DrawCinematic(*args.offset(1 as i32 as isize) as i32);
            return 0 as i32 as intptr_t;
        }
        78 => {
            CIN_SetExtents(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as i32,
                *args.offset(4 as i32 as isize) as i32,
                *args.offset(5 as i32 as isize) as i32,
            );
            return 0 as i32 as intptr_t;
        }
        79 => {
            re.RemapShader.expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const libc::c_char,
                VM_ArgPtr(*args.offset(3 as i32 as isize)) as *const libc::c_char,
            );
            return 0 as i32 as intptr_t;
        }
        86 => {
            /*
                case CG_LOADCAMERA:
                    return loadCamera(VMA(1));

                case CG_STARTCAMERA:
                    startCamera(args[1]);
                    return 0;

                case CG_GETCAMERAINFO:
                    return getCameraInfo(args[1], VMA(2), VMA(3));
            */
            return re.GetEntityToken.expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *mut libc::c_char,
                *args.offset(2 as i32 as isize) as i32,
            ) as intptr_t;
        }
        88 => {
            return re.inPVS.expect("non-null function pointer")(
                VM_ArgPtr(*args.offset(1 as i32 as isize)) as *const vec_t,
                VM_ArgPtr(*args.offset(2 as i32 as isize)) as *const vec_t,
            ) as intptr_t
        }
        _ => {
            Com_Error(
                ERR_DROP as i32,
                b"Bad cgame system trap: %ld\x00" as *const u8 as *const libc::c_char,
                *args.offset(0 as i32 as isize),
            );
        }
    };
}
/*
====================
CL_InitCGame

Should only be called by CL_StartHunkUsers
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_InitCGame() {
    let mut info: *const libc::c_char = std::ptr::null();
    let mut mapname: *const libc::c_char = std::ptr::null();
    let mut t1: i32 = 0;
    let mut t2: i32 = 0;
    let mut interpret: vmInterpret_t = VMI_NATIVE;
    t1 = Sys_Milliseconds();
    // put away the console
    Con_Close();
    // find the current mapname
    info = cl
        .gameState
        .stringData
        .as_mut_ptr()
        .offset(cl.gameState.stringOffsets[0 as i32 as usize] as isize);
    mapname = Info_ValueForKey(info, b"mapname\x00" as *const u8 as *const libc::c_char);
    Com_sprintf(
        cl.mapname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as usize as i32,
        b"maps/%s.bsp\x00" as *const u8 as *const libc::c_char,
        mapname,
    );
    // load the dll or bytecode
    interpret =
        Cvar_VariableValue(b"vm_cgame\x00" as *const u8 as *const libc::c_char) as vmInterpret_t;
    if cl_connectedToPureServer != 0 {
        // if sv_pure is set we only allow qvms to be loaded
        if interpret as u32 != VMI_COMPILED as i32 as u32
            && interpret as u32 != VMI_BYTECODE as i32 as u32
        {
            interpret = VMI_COMPILED
        }
    }
    cgvm = VM_Create(
        b"cgame\x00" as *const u8 as *const libc::c_char,
        Some(CL_CgameSystemCalls as unsafe extern "C" fn(_: *mut intptr_t) -> intptr_t),
        interpret,
    );
    if cgvm.is_null() {
        Com_Error(
            ERR_DROP as i32,
            b"VM_Create on cgame failed\x00" as *const u8 as *const libc::c_char,
        );
    }
    clc.state = CA_LOADING;
    // init for this gamestate
    // use the lastExecutedServerCommand instead of the serverCommandSequence
    // otherwise server commands sent just before a gamestate are dropped
    VM_Call(
        cgvm,
        CG_INIT as i32,
        clc.serverMessageSequence,
        clc.lastExecutedServerCommand,
        clc.clientNum,
    );
    // reset any CVAR_CHEAT cvars registered by cgame
    if clc.demoplaying as u64 == 0 && cl_connectedToCheatServer == 0 {
        Cvar_SetCheatState();
    }
    // we will send a usercmd this frame, which
    // will cause the server to send us the first snapshot
    clc.state = CA_PRIMED;
    t2 = Sys_Milliseconds();
    Com_Printf(
        b"CL_InitCGame: %5.2f seconds\n\x00" as *const u8 as *const libc::c_char,
        (t2 - t1) as f64 / 1000.0f64,
    );
    // have the renderer touch all its images, so they are present
    // on the card even if the driver does deferred loading
    re.EndRegistration.expect("non-null function pointer")();
    // make sure everything is paged in
    if Sys_LowPhysicalMemory() as u64 == 0 {
        Com_TouchMemory();
    }
    // clear anything that got printed
    Con_ClearNotify();
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
/*
====================
CL_GameCommand

See if the current console command is claimed by the cgame
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GameCommand() -> qboolean {
    if cgvm.is_null() {
        return qfalse;
    }
    return VM_Call(cgvm, CG_CONSOLE_COMMAND as i32) as qboolean;
}
/*
=====================
CL_CGameRendering
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CGameRendering(mut stereo: stereoFrame_t) {
    VM_Call(
        cgvm,
        CG_DRAW_ACTIVE_FRAME as i32,
        cl.serverTime,
        stereo as u32,
        clc.demoplaying as u32,
    );
    VM_Debug(0 as i32);
}
#[no_mangle]

pub unsafe extern "C" fn CL_AdjustTimeDelta() {
    let mut newDelta: i32 = 0;
    let mut deltaDelta: i32 = 0;
    cl.newSnapshots = qfalse;
    // the delta never drifts when replaying a demo
    if clc.demoplaying as u64 != 0 {
        return;
    } // FIXME: is this a problem for cgame?
    newDelta = cl.snap.serverTime - cls.realtime;
    deltaDelta = abs(newDelta - cl.serverTimeDelta);
    if deltaDelta > 500 as i32 {
        cl.serverTimeDelta = newDelta;
        cl.oldServerTime = cl.snap.serverTime;
        cl.serverTime = cl.snap.serverTime;
        if (*cl_showTimeDelta).integer != 0 {
            Com_Printf(b"<RESET> \x00" as *const u8 as *const libc::c_char);
        }
    } else if deltaDelta > 100 as i32 {
        // fast adjust, cut the difference in half
        if (*cl_showTimeDelta).integer != 0 {
            Com_Printf(b"<FAST> \x00" as *const u8 as *const libc::c_char);
        }
        cl.serverTimeDelta = cl.serverTimeDelta + newDelta >> 1 as i32
    } else if (*com_timescale).value == 0 as i32 as f32 || (*com_timescale).value == 1 as i32 as f32
    {
        if cl.extrapolatedSnapshot as u64 != 0 {
            cl.extrapolatedSnapshot = qfalse;
            cl.serverTimeDelta -= 2 as i32
        } else {
            // slow drift adjust, only move 1 or 2 msec
            // if any of the frames between this and the previous snapshot
            // had to be extrapolated, nudge our sense of time back a little
            // the granularity of +1 / -2 is too high for timescale modified frametimes
            // otherwise, move our sense of time forward to minimize total latency
            cl.serverTimeDelta += 1
        }
    }
    if (*cl_showTimeDelta).integer != 0 {
        Com_Printf(
            b"%i \x00" as *const u8 as *const libc::c_char,
            cl.serverTimeDelta,
        );
    };
}
/*
==================
CL_FirstSnapshot
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_FirstSnapshot() {
    // ignore snapshots that don't have entities
    if cl.snap.snapFlags & 2 as i32 != 0 {
        return;
    }
    clc.state = CA_ACTIVE;
    // set the timedelta so we are exactly on this first frame
    cl.serverTimeDelta = cl.snap.serverTime - cls.realtime;
    cl.oldServerTime = cl.snap.serverTime;
    clc.timeDemoBaseTime = cl.snap.serverTime;
    // if this is the first frame of active play,
    // execute the contents of activeAction now
    // this is to allow scripting a timedemo to start right
    // after loading
    if *(*cl_activeAction).string.offset(0 as i32 as isize) != 0 {
        Cbuf_AddText((*cl_activeAction).string);
        Cvar_Set(
            b"activeAction\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*cl_useMumble).integer != 0 && crate::src::client::libmumblelink::mumble_islinked() == 0 {
        let mut ret: i32 = crate::src::client::libmumblelink::mumble_link(
            b"ioquake3\x00" as *const u8 as *const libc::c_char,
        );
        Com_Printf(
            b"Mumble: Linking to Mumble application %s\n\x00" as *const u8 as *const libc::c_char,
            if ret == 0 as i32 {
                b"ok\x00" as *const u8 as *const libc::c_char
            } else {
                b"failed\x00" as *const u8 as *const libc::c_char
            },
        );
    }
    if clc.voipCodecInitialized as u64 == 0 {
        let mut i: i32 = 0;
        let mut error: i32 = 0;
        clc.opusEncoder = crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_create(
            48000 as i32,
            1 as i32,
            2048 as i32,
            &mut error,
        );
        if error != 0 {
            Com_DPrintf(
                b"VoIP: Error opus_encoder_create %d\n\x00" as *const u8 as *const libc::c_char,
                error,
            );
            return;
        }
        i = 0 as i32;
        while i < 64 as i32 {
            clc.opusDecoder[i as usize] =
                crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_create(
                    48000 as i32,
                    1 as i32,
                    &mut error,
                );
            if error != 0 {
                Com_DPrintf(
                    b"VoIP: Error opus_decoder_create(%d) %d\n\x00" as *const u8
                        as *const libc::c_char,
                    i,
                    error,
                );
                return;
            }
            clc.voipIgnore[i as usize] = qfalse;
            clc.voipGain[i as usize] = 1.0f32;
            i += 1
        }
        clc.voipCodecInitialized = qtrue;
        clc.voipMuteAll = qfalse;
        Cmd_AddCommand(
            b"voip\x00" as *const u8 as *const libc::c_char,
            Some(CL_Voip_f as unsafe extern "C" fn() -> ()),
        );
        Cvar_Set(
            b"cl_voipSendTarget\x00" as *const u8 as *const libc::c_char,
            b"spatial\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::memset(
            clc.voipTargets.as_mut_ptr() as *mut libc::c_void,
            !(0 as i32),
            ::std::mem::size_of::<[uint8_t; 8]>() as usize,
        );
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
//====================================================================
//
// console
//
//
// cl_scrn.c
//
// returns in virtual 640x480 coordinates
// draws a string with embedded color control characters with fade
// ignores embedded color control characters
//
// cl_cin.c
//
//
// cl_cgame.c
//
/*
==================
CL_SetCGameTime
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_SetCGameTime() {
    // getting a valid frame message ends the connection process
    if clc.state as u32 != CA_ACTIVE as i32 as u32 {
        if clc.state as u32 != CA_PRIMED as i32 as u32 {
            return;
        }
        if clc.demoplaying as u64 != 0 {
            // we shouldn't get the first snapshot on the same frame
            // as the gamestate, because it causes a bad time skip
            if clc.firstDemoFrameSkipped as u64 == 0 {
                clc.firstDemoFrameSkipped = qtrue;
                return;
            }
            CL_ReadDemoMessage();
        }
        if cl.newSnapshots as u64 != 0 {
            cl.newSnapshots = qfalse;
            CL_FirstSnapshot();
        }
        if clc.state as u32 != CA_ACTIVE as i32 as u32 {
            return;
        }
    }
    // if we have gotten to this point, cl.snap is guaranteed to be valid
    if cl.snap.valid as u64 == 0 {
        Com_Error(
            ERR_DROP as i32,
            b"CL_SetCGameTime: !cl.snap.valid\x00" as *const u8 as *const libc::c_char,
        );
    }
    // allow pause in single player
    if (*sv_paused).integer != 0 && CL_CheckPaused() as u32 != 0 && (*com_sv_running).integer != 0 {
        // paused
        return;
    }
    if cl.snap.serverTime < cl.oldFrameServerTime {
        Com_Error(
            ERR_DROP as i32,
            b"cl.snap.serverTime < cl.oldFrameServerTime\x00" as *const u8 as *const libc::c_char,
        );
    }
    cl.oldFrameServerTime = cl.snap.serverTime;
    // get our current view of time
    if !(clc.demoplaying as u32 != 0 && (*cl_freezeDemo).integer != 0) {
        // cl_timeNudge is a user adjustable cvar that allows more
        // or less latency to be added in the interest of better
        // smoothness or better responsiveness.
        let mut tn: i32 = 0;
        tn = (*cl_timeNudge).integer;
        if tn < -(30 as i32) {
            tn = -(30 as i32)
        } else if tn > 30 as i32 {
            tn = 30 as i32
        }
        cl.serverTime = cls.realtime + cl.serverTimeDelta - tn;
        // guarantee that time will never flow backwards, even if
        // serverTimeDelta made an adjustment or cl_timeNudge was changed
        if cl.serverTime < cl.oldServerTime {
            cl.serverTime = cl.oldServerTime
        }
        cl.oldServerTime = cl.serverTime;
        // note if we are almost past the latest frame (without timeNudge),
        // so we will try and adjust back a bit when the next snapshot arrives
        if cls.realtime + cl.serverTimeDelta >= cl.snap.serverTime - 5 as i32 {
            cl.extrapolatedSnapshot = qtrue
        }
    }
    // if we have gotten new snapshots, drift serverTimeDelta
    // don't do this every frame, or a period of packet loss would
    // make a huge adjustment
    if cl.newSnapshots as u64 != 0 {
        CL_AdjustTimeDelta();
    }
    if clc.demoplaying as u64 == 0 {
        return;
    }
    // if we are playing a demo back, we can just keep reading
    // messages from the demo file until the cgame definitely
    // has valid snapshots to interpolate between
    // a timedemo will always use a deterministic set of time samples
    // no matter what speed machine it is run on,
    // while a normal demo may have different time samples
    // each time it is played back
    if (*cl_timedemo).integer != 0 {
        let mut now: i32 = Sys_Milliseconds();
        let mut frameDuration: i32 = 0;
        if clc.timeDemoStart == 0 {
            clc.timeDemoLastFrame = now;
            clc.timeDemoStart = clc.timeDemoLastFrame;
            clc.timeDemoMinDuration = 2147483647 as i32;
            clc.timeDemoMaxDuration = 0 as i32
        }
        frameDuration = now - clc.timeDemoLastFrame;
        clc.timeDemoLastFrame = now;
        // Ignore the first measurement as it'll always be 0
        if clc.timeDemoFrames > 0 as i32 {
            if frameDuration > clc.timeDemoMaxDuration {
                clc.timeDemoMaxDuration = frameDuration
            }
            if frameDuration < clc.timeDemoMinDuration {
                clc.timeDemoMinDuration = frameDuration
            }
            // 255 ms = about 4fps
            if frameDuration > 127 as i32 * 2 as i32 + 1 as i32 {
                frameDuration = 127 as i32 * 2 as i32 + 1 as i32
            }
            clc.timeDemoDurations[((clc.timeDemoFrames - 1 as i32) % 4096 as i32) as usize] =
                frameDuration as u8
        }
        clc.timeDemoFrames += 1;
        cl.serverTime = clc.timeDemoBaseTime + clc.timeDemoFrames * 50 as i32
    }
    while cl.serverTime >= cl.snap.serverTime {
        // feed another messag, which should change
        // the contents of cl.snap
        CL_ReadDemoMessage();
        if clc.state as u32 != CA_ACTIVE as i32 as u32 {
            return;
            // end of demo
        }
    }
}

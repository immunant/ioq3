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

pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::uint8_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::client_h::clSnapshot_t;
pub use crate::client_h::clientActive_t;
pub use crate::client_h::clientConnection_t;
pub use crate::client_h::clientStatic_t;
pub use crate::client_h::outPacket_t;
pub use crate::client_h::serverInfo_t;
pub use crate::curl_h::CURL;
pub use crate::multi_h::CURLM;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::vmInterpret_t;
pub use crate::qcommon_h::vm_t;
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
pub use crate::src::client::cl_cin::CIN_DrawCinematic;
pub use crate::src::client::cl_cin::CIN_PlayCinematic;
pub use crate::src::client::cl_cin::CIN_RunCinematic;
pub use crate::src::client::cl_cin::CIN_SetExtents;
pub use crate::src::client::cl_cin::CIN_StopCinematic;
pub use crate::src::client::cl_keys::Key_GetCatcher;
pub use crate::src::client::cl_keys::Key_KeynumToString;
pub use crate::src::client::cl_keys::Key_SetCatcher;
pub use crate::src::client::cl_main::cl;
pub use crate::src::client::cl_main::clc;
pub use crate::src::client::cl_main::cls;
pub use crate::src::client::cl_main::re;
pub use crate::src::client::cl_main::CL_CDKeyValidate;
pub use crate::src::client::cl_main::CL_ClearPing;
pub use crate::src::client::cl_main::CL_GetPing;
pub use crate::src::client::cl_main::CL_GetPingInfo;
pub use crate::src::client::cl_main::CL_GetPingQueueCount;
pub use crate::src::client::cl_main::CL_ServerStatus;
pub use crate::src::client::cl_main::CL_UpdateVisiblePings_f;
pub use crate::src::client::cl_parse::cl_connectedToPureServer;
pub use crate::src::client::cl_scrn::SCR_UpdateScreen;
pub use crate::src::client::cl_ui::qcommon_h::_vmf;
pub use crate::src::qcommon::cmd::Cbuf_ExecuteText;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_ArgvBuffer;
pub use crate::src::qcommon::common::cl_cdkey;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Com_RealTime;
pub use crate::src::qcommon::common::Hunk_MemoryRemaining;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::cvar::cvar_modifiedFlags;
pub use crate::src::qcommon::cvar::Cvar_InfoStringBuffer;
pub use crate::src::qcommon::cvar::Cvar_Register;
pub use crate::src::qcommon::cvar::Cvar_Reset;
pub use crate::src::qcommon::cvar::Cvar_SetSafe;
pub use crate::src::qcommon::cvar::Cvar_SetValueSafe;
pub use crate::src::qcommon::cvar::Cvar_Update;
pub use crate::src::qcommon::cvar::Cvar_VariableString;
pub use crate::src::qcommon::cvar::Cvar_VariableStringBuffer;
pub use crate::src::qcommon::cvar::Cvar_VariableValue;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_FOpenFileByMode;
pub use crate::src::qcommon::files::FS_GetFileList;
pub use crate::src::qcommon::files::FS_Read;
pub use crate::src::qcommon::files::FS_SV_FOpenFileRead;
pub use crate::src::qcommon::files::FS_SV_FOpenFileWrite;
pub use crate::src::qcommon::files::FS_Seek;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::net_chan::NET_StringToAdr;
pub use crate::src::qcommon::net_ip::NET_AdrToStringwPort;
pub use crate::src::qcommon::net_ip::NET_CompareAdr;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
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
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
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
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
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
pub use crate::src::qcommon::vm::VM_Free;
pub use crate::src::sys::sys_main::Sys_GetClipboardData;
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
pub use crate::ui_public_h::uiClientState_t;
pub use crate::ui_public_h::UI_ARGC;
pub use crate::ui_public_h::UI_ARGV;
pub use crate::ui_public_h::UI_ATAN2;
pub use crate::ui_public_h::UI_CEIL;
pub use crate::ui_public_h::UI_CIN_DRAWCINEMATIC;
pub use crate::ui_public_h::UI_CIN_PLAYCINEMATIC;
pub use crate::ui_public_h::UI_CIN_RUNCINEMATIC;
pub use crate::ui_public_h::UI_CIN_SETEXTENTS;
pub use crate::ui_public_h::UI_CIN_STOPCINEMATIC;
pub use crate::ui_public_h::UI_CMD_EXECUTETEXT;
pub use crate::ui_public_h::UI_CM_LERPTAG;
pub use crate::ui_public_h::UI_CM_LOADMODEL;
pub use crate::ui_public_h::UI_CONSOLE_COMMAND;
pub use crate::ui_public_h::UI_COS;
pub use crate::ui_public_h::UI_CVAR_CREATE;
pub use crate::ui_public_h::UI_CVAR_INFOSTRINGBUFFER;
pub use crate::ui_public_h::UI_CVAR_REGISTER;
pub use crate::ui_public_h::UI_CVAR_RESET;
pub use crate::ui_public_h::UI_CVAR_SET;
pub use crate::ui_public_h::UI_CVAR_SETVALUE;
pub use crate::ui_public_h::UI_CVAR_UPDATE;
pub use crate::ui_public_h::UI_CVAR_VARIABLESTRINGBUFFER;
pub use crate::ui_public_h::UI_CVAR_VARIABLEVALUE;
pub use crate::ui_public_h::UI_DRAW_CONNECT_SCREEN;
pub use crate::ui_public_h::UI_ERROR;
pub use crate::ui_public_h::UI_FLOOR;
pub use crate::ui_public_h::UI_FS_FCLOSEFILE;
pub use crate::ui_public_h::UI_FS_FOPENFILE;
pub use crate::ui_public_h::UI_FS_GETFILELIST;
pub use crate::ui_public_h::UI_FS_READ;
pub use crate::ui_public_h::UI_FS_SEEK;
pub use crate::ui_public_h::UI_FS_WRITE;
pub use crate::ui_public_h::UI_GETAPIVERSION;
pub use crate::ui_public_h::UI_GETCLIENTSTATE;
pub use crate::ui_public_h::UI_GETCLIPBOARDDATA;
pub use crate::ui_public_h::UI_GETCONFIGSTRING;
pub use crate::ui_public_h::UI_GETGLCONFIG;
pub use crate::ui_public_h::UI_GET_CDKEY;
pub use crate::ui_public_h::UI_HASUNIQUECDKEY;
pub use crate::ui_public_h::UI_INIT;
pub use crate::ui_public_h::UI_IS_FULLSCREEN;
pub use crate::ui_public_h::UI_KEY_CLEARSTATES;
pub use crate::ui_public_h::UI_KEY_EVENT;
pub use crate::ui_public_h::UI_KEY_GETBINDINGBUF;
pub use crate::ui_public_h::UI_KEY_GETCATCHER;
pub use crate::ui_public_h::UI_KEY_GETOVERSTRIKEMODE;
pub use crate::ui_public_h::UI_KEY_ISDOWN;
pub use crate::ui_public_h::UI_KEY_KEYNUMTOSTRINGBUF;
pub use crate::ui_public_h::UI_KEY_SETBINDING;
pub use crate::ui_public_h::UI_KEY_SETCATCHER;
pub use crate::ui_public_h::UI_KEY_SETOVERSTRIKEMODE;
pub use crate::ui_public_h::UI_LAN_ADDSERVER;
pub use crate::ui_public_h::UI_LAN_CLEARPING;
pub use crate::ui_public_h::UI_LAN_COMPARESERVERS;
pub use crate::ui_public_h::UI_LAN_GETPING;
pub use crate::ui_public_h::UI_LAN_GETPINGINFO;
pub use crate::ui_public_h::UI_LAN_GETPINGQUEUECOUNT;
pub use crate::ui_public_h::UI_LAN_GETSERVERADDRESSSTRING;
pub use crate::ui_public_h::UI_LAN_GETSERVERCOUNT;
pub use crate::ui_public_h::UI_LAN_GETSERVERINFO;
pub use crate::ui_public_h::UI_LAN_GETSERVERPING;
pub use crate::ui_public_h::UI_LAN_LOADCACHEDSERVERS;
pub use crate::ui_public_h::UI_LAN_MARKSERVERVISIBLE;
pub use crate::ui_public_h::UI_LAN_REMOVESERVER;
pub use crate::ui_public_h::UI_LAN_RESETPINGS;
pub use crate::ui_public_h::UI_LAN_SAVECACHEDSERVERS;
pub use crate::ui_public_h::UI_LAN_SERVERISVISIBLE;
pub use crate::ui_public_h::UI_LAN_SERVERSTATUS;
pub use crate::ui_public_h::UI_LAN_UPDATEVISIBLEPINGS;
pub use crate::ui_public_h::UI_MEMCPY;
pub use crate::ui_public_h::UI_MEMORY_REMAINING;
pub use crate::ui_public_h::UI_MEMSET;
pub use crate::ui_public_h::UI_MILLISECONDS;
pub use crate::ui_public_h::UI_MOUSE_EVENT;
pub use crate::ui_public_h::UI_PC_ADD_GLOBAL_DEFINE;
pub use crate::ui_public_h::UI_PC_FREE_SOURCE;
pub use crate::ui_public_h::UI_PC_LOAD_SOURCE;
pub use crate::ui_public_h::UI_PC_READ_TOKEN;
pub use crate::ui_public_h::UI_PC_SOURCE_FILE_AND_LINE;
pub use crate::ui_public_h::UI_PRINT;
pub use crate::ui_public_h::UI_REAL_TIME;
pub use crate::ui_public_h::UI_REFRESH;
pub use crate::ui_public_h::UI_R_ADDLIGHTTOSCENE;
pub use crate::ui_public_h::UI_R_ADDPOLYTOSCENE;
pub use crate::ui_public_h::UI_R_ADDREFENTITYTOSCENE;
pub use crate::ui_public_h::UI_R_CLEARSCENE;
pub use crate::ui_public_h::UI_R_DRAWSTRETCHPIC;
pub use crate::ui_public_h::UI_R_MODELBOUNDS;
pub use crate::ui_public_h::UI_R_REGISTERFONT;
pub use crate::ui_public_h::UI_R_REGISTERMODEL;
pub use crate::ui_public_h::UI_R_REGISTERSHADERNOMIP;
pub use crate::ui_public_h::UI_R_REGISTERSKIN;
pub use crate::ui_public_h::UI_R_REMAP_SHADER;
pub use crate::ui_public_h::UI_R_RENDERSCENE;
pub use crate::ui_public_h::UI_R_SETCOLOR;
pub use crate::ui_public_h::UI_SET_ACTIVE_MENU;
pub use crate::ui_public_h::UI_SET_CDKEY;
pub use crate::ui_public_h::UI_SET_PBCLSTATUS;
pub use crate::ui_public_h::UI_SHUTDOWN;
pub use crate::ui_public_h::UI_SIN;
pub use crate::ui_public_h::UI_SQRT;
pub use crate::ui_public_h::UI_STRNCPY;
pub use crate::ui_public_h::UI_S_REGISTERSOUND;
pub use crate::ui_public_h::UI_S_STARTBACKGROUNDTRACK;
pub use crate::ui_public_h::UI_S_STARTLOCALSOUND;
pub use crate::ui_public_h::UI_S_STOPBACKGROUNDTRACK;
pub use crate::ui_public_h::UI_UPDATESCREEN;
pub use crate::ui_public_h::UI_VERIFY_CDKEY;
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
pub use crate::src::botlib::be_ai_chat::bot_consolemessage_s;
pub use crate::src::botlib::be_ai_chat::bot_match_s;
pub use crate::src::botlib::be_ai_goal::bot_goal_s;
pub use crate::src::botlib::be_ai_move::bot_initmove_s;
pub use crate::src::botlib::be_ai_move::bot_moveresult_s;
pub use crate::src::botlib::be_ai_weap::weaponinfo_s;

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
    #[no_mangle]
    pub static mut botlib_export: *mut crate::botlib_h::botlib_export_t;
}
#[no_mangle]

pub static mut uivm: *mut crate::qcommon_h::vm_t =
    0 as *const crate::qcommon_h::vm_t as *mut crate::qcommon_h::vm_t;
/*
====================
GetClientState
====================
*/

unsafe extern "C" fn GetClientState(mut state: *mut crate::ui_public_h::uiClientState_t) {
    (*state).connectPacketCount = crate::src::client::cl_main::clc.connectPacketCount;
    (*state).connState = crate::src::client::cl_main::clc.state;
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*state).servername.as_mut_ptr(),
        crate::src::client::cl_main::clc.servername.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*state).updateInfoString.as_mut_ptr(),
        crate::src::client::cl_main::cls
            .updateInfoString
            .as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*state).messageString.as_mut_ptr(),
        crate::src::client::cl_main::clc.serverMessage.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    (*state).clientNum = crate::src::client::cl_main::cl.snap.ps.clientNum;
}
/*
====================
LAN_LoadCachedServers
====================
*/
#[no_mangle]

pub unsafe extern "C" fn LAN_LoadCachedServers() {
    let mut size: i32 = 0;
    let mut fileIn: crate::src::qcommon::q_shared::fileHandle_t = 0;
    crate::src::client::cl_main::cls.numfavoriteservers = 0 as i32;
    crate::src::client::cl_main::cls.numglobalservers =
        crate::src::client::cl_main::cls.numfavoriteservers;
    crate::src::client::cl_main::cls.numGlobalServerAddresses = 0 as i32;
    if crate::src::qcommon::files::FS_SV_FOpenFileRead(
        b"servercache.dat\x00" as *const u8 as *const libc::c_char,
        &mut fileIn,
    ) != 0
    {
        crate::src::qcommon::files::FS_Read(
            &mut crate::src::client::cl_main::cls.numglobalservers as *mut i32 as *mut libc::c_void,
            ::std::mem::size_of::<i32>() as libc::c_ulong as i32,
            fileIn,
        );
        crate::src::qcommon::files::FS_Read(
            &mut crate::src::client::cl_main::cls.numfavoriteservers as *mut i32
                as *mut libc::c_void,
            ::std::mem::size_of::<i32>() as libc::c_ulong as i32,
            fileIn,
        );
        crate::src::qcommon::files::FS_Read(
            &mut size as *mut i32 as *mut libc::c_void,
            ::std::mem::size_of::<i32>() as libc::c_ulong as i32,
            fileIn,
        );
        if size as libc::c_ulong
            == (::std::mem::size_of::<[crate::client_h::serverInfo_t; 4096]>() as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<[crate::client_h::serverInfo_t; 128]>() as libc::c_ulong,
                )
        {
            crate::src::qcommon::files::FS_Read(
                &mut crate::src::client::cl_main::cls.globalServers
                    as *mut [crate::client_h::serverInfo_t; 4096]
                    as *mut libc::c_void,
                ::std::mem::size_of::<[crate::client_h::serverInfo_t; 4096]>() as libc::c_ulong
                    as i32,
                fileIn,
            );
            crate::src::qcommon::files::FS_Read(
                &mut crate::src::client::cl_main::cls.favoriteServers
                    as *mut [crate::client_h::serverInfo_t; 128]
                    as *mut libc::c_void,
                ::std::mem::size_of::<[crate::client_h::serverInfo_t; 128]>() as libc::c_ulong
                    as i32,
                fileIn,
            );
        } else {
            crate::src::client::cl_main::cls.numfavoriteservers = 0 as i32;
            crate::src::client::cl_main::cls.numglobalservers =
                crate::src::client::cl_main::cls.numfavoriteservers;
            crate::src::client::cl_main::cls.numGlobalServerAddresses = 0 as i32
        }
        crate::src::qcommon::files::FS_FCloseFile(fileIn);
    };
}
/*
====================
LAN_SaveServersToCache
====================
*/
#[no_mangle]

pub unsafe extern "C" fn LAN_SaveServersToCache() {
    let mut size: i32 = 0;
    let mut fileOut: crate::src::qcommon::q_shared::fileHandle_t =
        crate::src::qcommon::files::FS_SV_FOpenFileWrite(
            b"servercache.dat\x00" as *const u8 as *const libc::c_char,
        );
    crate::src::qcommon::files::FS_Write(
        &mut crate::src::client::cl_main::cls.numglobalservers as *mut i32 as *const libc::c_void,
        ::std::mem::size_of::<i32>() as libc::c_ulong as i32,
        fileOut,
    );
    crate::src::qcommon::files::FS_Write(
        &mut crate::src::client::cl_main::cls.numfavoriteservers as *mut i32 as *const libc::c_void,
        ::std::mem::size_of::<i32>() as libc::c_ulong as i32,
        fileOut,
    );
    size = (::std::mem::size_of::<[crate::client_h::serverInfo_t; 4096]>() as libc::c_ulong)
        .wrapping_add(
            ::std::mem::size_of::<[crate::client_h::serverInfo_t; 128]>() as libc::c_ulong,
        ) as i32;
    crate::src::qcommon::files::FS_Write(
        &mut size as *mut i32 as *const libc::c_void,
        ::std::mem::size_of::<i32>() as libc::c_ulong as i32,
        fileOut,
    );
    crate::src::qcommon::files::FS_Write(
        &mut crate::src::client::cl_main::cls.globalServers
            as *mut [crate::client_h::serverInfo_t; 4096] as *const libc::c_void,
        ::std::mem::size_of::<[crate::client_h::serverInfo_t; 4096]>() as libc::c_ulong as i32,
        fileOut,
    );
    crate::src::qcommon::files::FS_Write(
        &mut crate::src::client::cl_main::cls.favoriteServers
            as *mut [crate::client_h::serverInfo_t; 128] as *const libc::c_void,
        ::std::mem::size_of::<[crate::client_h::serverInfo_t; 128]>() as libc::c_ulong as i32,
        fileOut,
    );
    crate::src::qcommon::files::FS_FCloseFile(fileOut);
}
/*
====================
LAN_ResetPings
====================
*/

unsafe extern "C" fn LAN_ResetPings(mut source: i32) {
    let mut count: i32 = 0;
    let mut i: i32 = 0;
    let mut servers: *mut crate::client_h::serverInfo_t = 0 as *mut crate::client_h::serverInfo_t;
    count = 0 as i32;
    match source {
        0 => {
            servers = &mut *crate::src::client::cl_main::cls
                .localServers
                .as_mut_ptr()
                .offset(0 as i32 as isize)
                as *mut crate::client_h::serverInfo_t;
            count = 128 as i32
        }
        1 | 2 => {
            servers = &mut *crate::src::client::cl_main::cls
                .globalServers
                .as_mut_ptr()
                .offset(0 as i32 as isize)
                as *mut crate::client_h::serverInfo_t;
            count = 4096 as i32
        }
        3 => {
            servers = &mut *crate::src::client::cl_main::cls
                .favoriteServers
                .as_mut_ptr()
                .offset(0 as i32 as isize)
                as *mut crate::client_h::serverInfo_t;
            count = 128 as i32
        }
        _ => {}
    }
    if !servers.is_null() {
        i = 0 as i32;
        while i < count {
            (*servers.offset(i as isize)).ping = -(1 as i32);
            i += 1
        }
    };
}
/*
====================
LAN_AddServer
====================
*/

unsafe extern "C" fn LAN_AddServer(
    mut source: i32,
    mut name: *const libc::c_char,
    mut address: *const libc::c_char,
) -> i32 {
    let mut max: i32 = 0;
    let mut count: *mut i32 = 0 as *mut i32;
    let mut i: i32 = 0;
    let mut adr: crate::qcommon_h::netadr_t = crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    };
    let mut servers: *mut crate::client_h::serverInfo_t = 0 as *mut crate::client_h::serverInfo_t;
    max = 128 as i32;
    count = 0 as *mut i32;
    match source {
        0 => {
            count = &mut crate::src::client::cl_main::cls.numlocalservers;
            servers = &mut *crate::src::client::cl_main::cls
                .localServers
                .as_mut_ptr()
                .offset(0 as i32 as isize)
                as *mut crate::client_h::serverInfo_t
        }
        1 | 2 => {
            max = 4096 as i32;
            count = &mut crate::src::client::cl_main::cls.numglobalservers;
            servers = &mut *crate::src::client::cl_main::cls
                .globalServers
                .as_mut_ptr()
                .offset(0 as i32 as isize)
                as *mut crate::client_h::serverInfo_t
        }
        3 => {
            count = &mut crate::src::client::cl_main::cls.numfavoriteservers;
            servers = &mut *crate::src::client::cl_main::cls
                .favoriteServers
                .as_mut_ptr()
                .offset(0 as i32 as isize)
                as *mut crate::client_h::serverInfo_t
        }
        _ => {}
    }
    if !servers.is_null() && *count < max {
        crate::src::qcommon::net_chan::NET_StringToAdr(
            address,
            &mut adr as *mut _ as *mut crate::qcommon_h::netadr_t,
            crate::qcommon_h::NA_UNSPEC,
        );
        i = 0 as i32;
        while i < *count {
            if crate::src::qcommon::net_ip::NET_CompareAdr(
                (*servers.offset(i as isize)).adr as crate::qcommon_h::netadr_t,
                adr as crate::qcommon_h::netadr_t,
            ) as u64
                != 0
            {
                break;
            }
            i += 1
        }
        if i >= *count {
            (*servers.offset(*count as isize)).adr = adr;
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*servers.offset(*count as isize)).hostName.as_mut_ptr(),
                name,
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
            );
            (*servers.offset(*count as isize)).visible = crate::src::qcommon::q_shared::qtrue;
            *count += 1;
            return 1 as i32;
        }
        return 0 as i32;
    }
    return -(1 as i32);
}
/*
====================
LAN_RemoveServer
====================
*/

unsafe extern "C" fn LAN_RemoveServer(mut source: i32, mut addr: *const libc::c_char) {
    let mut count: *mut i32 = 0 as *mut i32;
    let mut i: i32 = 0;
    let mut servers: *mut crate::client_h::serverInfo_t = 0 as *mut crate::client_h::serverInfo_t;
    count = 0 as *mut i32;
    match source {
        0 => {
            count = &mut crate::src::client::cl_main::cls.numlocalservers;
            servers = &mut *crate::src::client::cl_main::cls
                .localServers
                .as_mut_ptr()
                .offset(0 as i32 as isize)
                as *mut crate::client_h::serverInfo_t
        }
        1 | 2 => {
            count = &mut crate::src::client::cl_main::cls.numglobalservers;
            servers = &mut *crate::src::client::cl_main::cls
                .globalServers
                .as_mut_ptr()
                .offset(0 as i32 as isize)
                as *mut crate::client_h::serverInfo_t
        }
        3 => {
            count = &mut crate::src::client::cl_main::cls.numfavoriteservers;
            servers = &mut *crate::src::client::cl_main::cls
                .favoriteServers
                .as_mut_ptr()
                .offset(0 as i32 as isize)
                as *mut crate::client_h::serverInfo_t
        }
        _ => {}
    }
    if !servers.is_null() {
        let mut comp: crate::qcommon_h::netadr_t = crate::qcommon_h::netadr_t {
            type_0: crate::qcommon_h::NA_BAD,
            ip: [0; 4],
            ip6: [0; 16],
            port: 0,
            scope_id: 0,
        };
        crate::src::qcommon::net_chan::NET_StringToAdr(
            addr,
            &mut comp as *mut _ as *mut crate::qcommon_h::netadr_t,
            crate::qcommon_h::NA_UNSPEC,
        );
        i = 0 as i32;
        while i < *count {
            if crate::src::qcommon::net_ip::NET_CompareAdr(
                comp as crate::qcommon_h::netadr_t,
                (*servers.offset(i as isize)).adr as crate::qcommon_h::netadr_t,
            ) as u64
                != 0
            {
                let mut j: i32 = i;
                while j < *count - 1 as i32 {
                    crate::stdlib::memcpy(
                        &mut *servers.offset(j as isize) as *mut crate::client_h::serverInfo_t
                            as *mut libc::c_void,
                        &mut *servers.offset((j + 1 as i32) as isize)
                            as *mut crate::client_h::serverInfo_t
                            as *const libc::c_void,
                        ::std::mem::size_of::<crate::client_h::serverInfo_t>() as libc::c_ulong,
                    );
                    j += 1
                }
                *count -= 1;
                break;
            } else {
                i += 1
            }
        }
    };
}
/*
====================
LAN_GetServerCount
====================
*/

unsafe extern "C" fn LAN_GetServerCount(mut source: i32) -> i32 {
    match source {
        0 => return crate::src::client::cl_main::cls.numlocalservers,
        1 | 2 => return crate::src::client::cl_main::cls.numglobalservers,
        3 => return crate::src::client::cl_main::cls.numfavoriteservers,
        _ => {}
    }
    return 0 as i32;
}
/*
====================
LAN_GetLocalServerAddressString
====================
*/

unsafe extern "C" fn LAN_GetServerAddressString(
    mut source: i32,
    mut n: i32,
    mut buf: *mut libc::c_char,
    mut buflen: i32,
) {
    match source {
        0 => {
            if n >= 0 as i32 && n < 128 as i32 {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    buf,
                    crate::src::qcommon::net_ip::NET_AdrToStringwPort(
                        crate::src::client::cl_main::cls.localServers[n as usize].adr
                            as crate::qcommon_h::netadr_t,
                    ),
                    buflen,
                );
                return;
            }
        }
        1 | 2 => {
            if n >= 0 as i32 && n < 4096 as i32 {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    buf,
                    crate::src::qcommon::net_ip::NET_AdrToStringwPort(
                        crate::src::client::cl_main::cls.globalServers[n as usize].adr
                            as crate::qcommon_h::netadr_t,
                    ),
                    buflen,
                );
                return;
            }
        }
        3 => {
            if n >= 0 as i32 && n < 128 as i32 {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    buf,
                    crate::src::qcommon::net_ip::NET_AdrToStringwPort(
                        crate::src::client::cl_main::cls.favoriteServers[n as usize].adr
                            as crate::qcommon_h::netadr_t,
                    ),
                    buflen,
                );
                return;
            }
        }
        _ => {}
    }
    *buf.offset(0 as i32 as isize) = '\u{0}' as i32 as libc::c_char;
}
/*
====================
LAN_GetServerInfo
====================
*/

unsafe extern "C" fn LAN_GetServerInfo(
    mut source: i32,
    mut n: i32,
    mut buf: *mut libc::c_char,
    mut buflen: i32,
) {
    let mut info: [libc::c_char; 1024] = [0; 1024];
    let mut server: *mut crate::client_h::serverInfo_t = 0 as *mut crate::client_h::serverInfo_t;
    info[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
    match source {
        0 => {
            if n >= 0 as i32 && n < 128 as i32 {
                server = &mut *crate::src::client::cl_main::cls
                    .localServers
                    .as_mut_ptr()
                    .offset(n as isize)
                    as *mut crate::client_h::serverInfo_t
            }
        }
        1 | 2 => {
            if n >= 0 as i32 && n < 4096 as i32 {
                server = &mut *crate::src::client::cl_main::cls
                    .globalServers
                    .as_mut_ptr()
                    .offset(n as isize)
                    as *mut crate::client_h::serverInfo_t
            }
        }
        3 => {
            if n >= 0 as i32 && n < 128 as i32 {
                server = &mut *crate::src::client::cl_main::cls
                    .favoriteServers
                    .as_mut_ptr()
                    .offset(n as isize)
                    as *mut crate::client_h::serverInfo_t
            }
        }
        _ => {}
    }
    if !server.is_null() && !buf.is_null() {
        *buf.offset(0 as i32 as isize) = '\u{0}' as i32 as libc::c_char;
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"hostname\x00" as *const u8 as *const libc::c_char,
            (*server).hostName.as_mut_ptr(),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"mapname\x00" as *const u8 as *const libc::c_char,
            (*server).mapName.as_mut_ptr(),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"clients\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*server).clients,
            ),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*server).maxClients,
            ),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"ping\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*server).ping,
            ),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"minping\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*server).minPing,
            ),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"maxping\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*server).maxPing,
            ),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"game\x00" as *const u8 as *const libc::c_char,
            (*server).game.as_mut_ptr(),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"gametype\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*server).gameType,
            ),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"nettype\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*server).netType,
            ),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"addr\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::net_ip::NET_AdrToStringwPort(
                (*server).adr as crate::qcommon_h::netadr_t,
            ),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"punkbuster\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*server).punkbuster,
            ),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"g_needpass\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*server).g_needpass,
            ),
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            info.as_mut_ptr(),
            b"g_humanplayers\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*server).g_humanplayers,
            ),
        );
        crate::src::qcommon::q_shared::Q_strncpyz(buf, info.as_mut_ptr(), buflen);
    } else if !buf.is_null() {
        *buf.offset(0 as i32 as isize) = '\u{0}' as i32 as libc::c_char
    };
}
/*
====================
LAN_GetServerPing
====================
*/

unsafe extern "C" fn LAN_GetServerPing(mut source: i32, mut n: i32) -> i32 {
    let mut server: *mut crate::client_h::serverInfo_t = 0 as *mut crate::client_h::serverInfo_t;
    match source {
        0 => {
            if n >= 0 as i32 && n < 128 as i32 {
                server = &mut *crate::src::client::cl_main::cls
                    .localServers
                    .as_mut_ptr()
                    .offset(n as isize)
                    as *mut crate::client_h::serverInfo_t
            }
        }
        1 | 2 => {
            if n >= 0 as i32 && n < 4096 as i32 {
                server = &mut *crate::src::client::cl_main::cls
                    .globalServers
                    .as_mut_ptr()
                    .offset(n as isize)
                    as *mut crate::client_h::serverInfo_t
            }
        }
        3 => {
            if n >= 0 as i32 && n < 128 as i32 {
                server = &mut *crate::src::client::cl_main::cls
                    .favoriteServers
                    .as_mut_ptr()
                    .offset(n as isize)
                    as *mut crate::client_h::serverInfo_t
            }
        }
        _ => {}
    }
    if !server.is_null() {
        return (*server).ping;
    }
    return -(1 as i32);
}
/*
====================
LAN_GetServerPtr
====================
*/

unsafe extern "C" fn LAN_GetServerPtr(
    mut source: i32,
    mut n: i32,
) -> *mut crate::client_h::serverInfo_t {
    match source {
        0 => {
            if n >= 0 as i32 && n < 128 as i32 {
                return &mut *crate::src::client::cl_main::cls
                    .localServers
                    .as_mut_ptr()
                    .offset(n as isize)
                    as *mut crate::client_h::serverInfo_t;
            }
        }
        1 | 2 => {
            if n >= 0 as i32 && n < 4096 as i32 {
                return &mut *crate::src::client::cl_main::cls
                    .globalServers
                    .as_mut_ptr()
                    .offset(n as isize)
                    as *mut crate::client_h::serverInfo_t;
            }
        }
        3 => {
            if n >= 0 as i32 && n < 128 as i32 {
                return &mut *crate::src::client::cl_main::cls
                    .favoriteServers
                    .as_mut_ptr()
                    .offset(n as isize)
                    as *mut crate::client_h::serverInfo_t;
            }
        }
        _ => {}
    }
    return 0 as *mut crate::client_h::serverInfo_t;
}
/*
====================
LAN_CompareServers
====================
*/

unsafe extern "C" fn LAN_CompareServers(
    mut source: i32,
    mut sortKey: i32,
    mut sortDir: i32,
    mut s1: i32,
    mut s2: i32,
) -> i32 {
    let mut res: i32 = 0;
    let mut server1: *mut crate::client_h::serverInfo_t = 0 as *mut crate::client_h::serverInfo_t;
    let mut server2: *mut crate::client_h::serverInfo_t = 0 as *mut crate::client_h::serverInfo_t;
    let mut clients1: i32 = 0;
    let mut clients2: i32 = 0;
    server1 = LAN_GetServerPtr(source, s1);
    server2 = LAN_GetServerPtr(source, s2);
    if server1.is_null() || server2.is_null() {
        return 0 as i32;
    }
    res = 0 as i32;
    match sortKey {
        0 => {
            res = crate::src::qcommon::q_shared::Q_stricmp(
                (*server1).hostName.as_mut_ptr(),
                (*server2).hostName.as_mut_ptr(),
            )
        }
        1 => {
            res = crate::src::qcommon::q_shared::Q_stricmp(
                (*server1).mapName.as_mut_ptr(),
                (*server2).mapName.as_mut_ptr(),
            )
        }
        2 => {
            // sub sort by max clients
            if (*server1).clients == (*server2).clients {
                clients1 = (*server1).maxClients;
                clients2 = (*server2).maxClients
            } else {
                clients1 = (*server1).clients;
                clients2 = (*server2).clients
            }
            if clients1 < clients2 {
                res = -(1 as i32)
            } else if clients1 > clients2 {
                res = 1 as i32
            } else {
                res = 0 as i32
            }
        }
        3 => {
            if (*server1).gameType < (*server2).gameType {
                res = -(1 as i32)
            } else if (*server1).gameType > (*server2).gameType {
                res = 1 as i32
            } else {
                res = 0 as i32
            }
        }
        4 => {
            if (*server1).ping < (*server2).ping {
                res = -(1 as i32)
            } else if (*server1).ping > (*server2).ping {
                res = 1 as i32
            } else {
                res = 0 as i32
            }
        }
        _ => {}
    }
    if sortDir != 0 {
        if res < 0 as i32 {
            return 1 as i32;
        }
        if res > 0 as i32 {
            return -(1 as i32);
        }
        return 0 as i32;
    }
    return res;
}
/*
====================
LAN_GetPingQueueCount
====================
*/

unsafe extern "C" fn LAN_GetPingQueueCount() -> i32 {
    return crate::src::client::cl_main::CL_GetPingQueueCount();
}
/*
====================
LAN_ClearPing
====================
*/

unsafe extern "C" fn LAN_ClearPing(mut n: i32) {
    crate::src::client::cl_main::CL_ClearPing(n);
}
/*
====================
LAN_GetPing
====================
*/

unsafe extern "C" fn LAN_GetPing(
    mut n: i32,
    mut buf: *mut libc::c_char,
    mut buflen: i32,
    mut pingtime: *mut i32,
) {
    crate::src::client::cl_main::CL_GetPing(n, buf, buflen, pingtime);
}
/*
====================
LAN_GetPingInfo
====================
*/

unsafe extern "C" fn LAN_GetPingInfo(mut n: i32, mut buf: *mut libc::c_char, mut buflen: i32) {
    crate::src::client::cl_main::CL_GetPingInfo(n, buf, buflen);
}
/*
====================
LAN_MarkServerVisible
====================
*/

unsafe extern "C" fn LAN_MarkServerVisible(
    mut source: i32,
    mut n: i32,
    mut visible: crate::src::qcommon::q_shared::qboolean,
) {
    if n == -(1 as i32) {
        let mut count: i32 = 128 as i32;
        let mut server: *mut crate::client_h::serverInfo_t =
            0 as *mut crate::client_h::serverInfo_t;
        match source {
            0 => {
                server = &mut *crate::src::client::cl_main::cls
                    .localServers
                    .as_mut_ptr()
                    .offset(0 as i32 as isize)
                    as *mut crate::client_h::serverInfo_t
            }
            1 | 2 => {
                server = &mut *crate::src::client::cl_main::cls
                    .globalServers
                    .as_mut_ptr()
                    .offset(0 as i32 as isize)
                    as *mut crate::client_h::serverInfo_t;
                count = 4096 as i32
            }
            3 => {
                server = &mut *crate::src::client::cl_main::cls
                    .favoriteServers
                    .as_mut_ptr()
                    .offset(0 as i32 as isize)
                    as *mut crate::client_h::serverInfo_t
            }
            _ => {}
        }
        if !server.is_null() {
            n = 0 as i32;
            while n < count {
                (*server.offset(n as isize)).visible = visible;
                n += 1
            }
        }
    } else {
        match source {
            0 => {
                if n >= 0 as i32 && n < 128 as i32 {
                    crate::src::client::cl_main::cls.localServers[n as usize].visible = visible
                }
            }
            1 | 2 => {
                if n >= 0 as i32 && n < 4096 as i32 {
                    crate::src::client::cl_main::cls.globalServers[n as usize].visible = visible
                }
            }
            3 => {
                if n >= 0 as i32 && n < 128 as i32 {
                    crate::src::client::cl_main::cls.favoriteServers[n as usize].visible = visible
                }
            }
            _ => {}
        }
    };
}
/*
=======================
LAN_ServerIsVisible
=======================
*/

unsafe extern "C" fn LAN_ServerIsVisible(mut source: i32, mut n: i32) -> i32 {
    match source {
        0 => {
            if n >= 0 as i32 && n < 128 as i32 {
                return crate::src::client::cl_main::cls.localServers[n as usize].visible as i32;
            }
        }
        1 | 2 => {
            if n >= 0 as i32 && n < 4096 as i32 {
                return crate::src::client::cl_main::cls.globalServers[n as usize].visible as i32;
            }
        }
        3 => {
            if n >= 0 as i32 && n < 128 as i32 {
                return crate::src::client::cl_main::cls.favoriteServers[n as usize].visible as i32;
            }
        }
        _ => {}
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
=======================
LAN_UpdateVisiblePings
=======================
*/
#[no_mangle]

pub unsafe extern "C" fn LAN_UpdateVisiblePings(
    mut source: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    return crate::src::client::cl_main::CL_UpdateVisiblePings_f(source);
}
/*
====================
LAN_GetServerStatus
====================
*/
#[no_mangle]

pub unsafe extern "C" fn LAN_GetServerStatus(
    mut serverAddress: *mut libc::c_char,
    mut serverStatus: *mut libc::c_char,
    mut maxLen: i32,
) -> i32 {
    return crate::src::client::cl_main::CL_ServerStatus(serverAddress, serverStatus, maxLen);
}
/*
====================
CL_GetGlConfig
====================
*/

unsafe extern "C" fn CL_GetGlconfig(mut config: *mut crate::tr_types_h::glconfig_t) {
    *config = crate::src::client::cl_main::cls.glconfig;
}
/*
====================
CL_GetClipboardData
====================
*/

unsafe extern "C" fn CL_GetClipboardData(mut buf: *mut libc::c_char, mut buflen: i32) {
    let mut cbd: *mut libc::c_char = 0 as *mut libc::c_char;
    cbd = crate::src::sys::sys_main::Sys_GetClipboardData();
    if cbd.is_null() {
        *buf = 0 as i32 as libc::c_char;
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(buf, cbd, buflen);
    crate::src::qcommon::common::Z_Free(cbd as *mut libc::c_void);
}
/*
====================
Key_KeynumToStringBuf
====================
*/

unsafe extern "C" fn Key_KeynumToStringBuf(
    mut keynum: i32,
    mut buf: *mut libc::c_char,
    mut buflen: i32,
) {
    crate::src::qcommon::q_shared::Q_strncpyz(
        buf,
        crate::src::client::cl_keys::Key_KeynumToString(keynum),
        buflen,
    );
}
/*
====================
Key_GetBindingBuf
====================
*/

unsafe extern "C" fn Key_GetBindingBuf(
    mut keynum: i32,
    mut buf: *mut libc::c_char,
    mut buflen: i32,
) {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    value = crate::src::client::cl_keys::Key_GetBinding(keynum);
    if !value.is_null() {
        crate::src::qcommon::q_shared::Q_strncpyz(buf, value, buflen);
    } else {
        *buf = 0 as i32 as libc::c_char
    };
}
/*
====================
CLUI_GetCDKey
====================
*/

unsafe extern "C" fn CLUI_GetCDKey(mut buf: *mut libc::c_char, mut _buflen: i32) {
    let mut gamedir: *const libc::c_char = 0 as *const libc::c_char;
    gamedir = crate::src::qcommon::cvar::Cvar_VariableString(
        b"fs_game\x00" as *const u8 as *const libc::c_char,
    );
    if UI_usesUniqueCDKey() as u32 != 0 && *gamedir.offset(0 as i32 as isize) as i32 != 0 as i32 {
        crate::stdlib::memcpy(
            buf as *mut libc::c_void,
            &mut *crate::src::qcommon::common::cl_cdkey
                .as_mut_ptr()
                .offset(16 as i32 as isize) as *mut libc::c_char as *const libc::c_void,
            16 as i32 as libc::c_ulong,
        );
        *buf.offset(16 as i32 as isize) = 0 as i32 as libc::c_char
    } else {
        crate::stdlib::memcpy(
            buf as *mut libc::c_void,
            crate::src::qcommon::common::cl_cdkey.as_mut_ptr() as *const libc::c_void,
            16 as i32 as libc::c_ulong,
        );
        *buf.offset(16 as i32 as isize) = 0 as i32 as libc::c_char
    };
}
/*
====================
CLUI_SetCDKey
====================
*/

unsafe extern "C" fn CLUI_SetCDKey(mut buf: *mut libc::c_char) {
    let mut gamedir: *const libc::c_char = 0 as *const libc::c_char;
    gamedir = crate::src::qcommon::cvar::Cvar_VariableString(
        b"fs_game\x00" as *const u8 as *const libc::c_char,
    );
    if UI_usesUniqueCDKey() as u32 != 0 && *gamedir.offset(0 as i32 as isize) as i32 != 0 as i32 {
        crate::stdlib::memcpy(
            &mut *crate::src::qcommon::common::cl_cdkey
                .as_mut_ptr()
                .offset(16 as i32 as isize) as *mut libc::c_char as *mut libc::c_void,
            buf as *const libc::c_void,
            16 as i32 as libc::c_ulong,
        );
        crate::src::qcommon::common::cl_cdkey[32 as i32 as usize] = 0 as i32 as libc::c_char;
        // set the flag so the fle will be written at the next opportunity
        crate::src::qcommon::cvar::cvar_modifiedFlags |= 0x1 as i32
    } else {
        crate::stdlib::memcpy(
            crate::src::qcommon::common::cl_cdkey.as_mut_ptr() as *mut libc::c_void,
            buf as *const libc::c_void,
            16 as i32 as libc::c_ulong,
        );
        // set the flag so the fle will be written at the next opportunity
        crate::src::qcommon::cvar::cvar_modifiedFlags |= 0x1 as i32
    };
}
/*
====================
GetConfigString
====================
*/

unsafe extern "C" fn GetConfigString(
    mut index: i32,
    mut buf: *mut libc::c_char,
    mut size: i32,
) -> i32 {
    let mut offset: i32 = 0;
    if index < 0 as i32 || index >= 1024 as i32 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    offset = crate::src::client::cl_main::cl.gameState.stringOffsets[index as usize];
    if offset == 0 {
        if size != 0 {
            *buf.offset(0 as i32 as isize) = 0 as i32 as libc::c_char
        }
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        buf,
        crate::src::client::cl_main::cl
            .gameState
            .stringData
            .as_mut_ptr()
            .offset(offset as isize),
        size,
    );
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
====================
FloatAsInt
====================
*/

unsafe extern "C" fn FloatAsInt(mut f: f32) -> i32 {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.f = f;
    return fi.i;
}
/*
====================
CL_UISystemCalls

The ui module is making a system call
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_UISystemCalls(
    mut args: *mut crate::stdlib::intptr_t,
) -> crate::stdlib::intptr_t {
    match *args.offset(0 as i32 as isize) {
        0 => {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"%s\x00" as *const u8 as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
            );
        }
        1 => {
            crate::src::qcommon::common::Com_Printf(
                b"%s\x00" as *const u8 as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        2 => return crate::src::sys::sys_unix::Sys_Milliseconds() as crate::stdlib::intptr_t,
        50 => {
            crate::src::qcommon::cvar::Cvar_Register(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *mut crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(3 as i32 as isize))
                    as *const libc::c_char,
                *args.offset(4 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        51 => {
            crate::src::qcommon::cvar::Cvar_Update(crate::src::qcommon::vm::VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *mut crate::src::qcommon::q_shared::vmCvar_t
                as *mut crate::src::qcommon::q_shared::vmCvar_t);
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        3 => {
            crate::src::qcommon::cvar::Cvar_SetSafe(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *const libc::c_char,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        4 => {
            return FloatAsInt(crate::src::qcommon::cvar::Cvar_VariableValue(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
            )) as crate::stdlib::intptr_t
        }
        5 => {
            crate::src::qcommon::cvar::Cvar_VariableStringBuffer(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        6 => {
            crate::src::qcommon::cvar::Cvar_SetValueSafe(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
                _vmf(*args.offset(2 as i32 as isize)),
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        7 => {
            crate::src::qcommon::cvar::Cvar_Reset(crate::src::qcommon::vm::VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            ) as *const libc::c_char);
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        8 => {
            crate::src::qcommon::cvar::Cvar_Register(
                0 as *mut crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *const libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        9 => {
            crate::src::qcommon::cvar::Cvar_InfoStringBuffer(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        10 => return crate::src::qcommon::cmd::Cmd_Argc() as crate::stdlib::intptr_t,
        11 => {
            crate::src::qcommon::cmd::Cmd_ArgvBuffer(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        12 => {
            if *args.offset(1 as i32 as isize)
                == crate::src::qcommon::q_shared::EXEC_NOW as i32 as libc::c_long
                && (crate::stdlib::strncmp(
                    crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                        as *const libc::c_char,
                    b"snd_restart\x00" as *const u8 as *const libc::c_char,
                    11 as i32 as libc::c_ulong,
                ) == 0
                    || crate::stdlib::strncmp(
                        crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                            as *const libc::c_char,
                        b"vid_restart\x00" as *const u8 as *const libc::c_char,
                        11 as i32 as libc::c_ulong,
                    ) == 0
                    || crate::stdlib::strncmp(
                        crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                            as *const libc::c_char,
                        b"quit\x00" as *const u8 as *const libc::c_char,
                        5 as i32 as libc::c_ulong,
                    ) == 0)
            {
                crate::src::qcommon::common::Com_Printf(
                    b"^3turning EXEC_NOW \'%.11s\' into EXEC_INSERT\n\x00" as *const u8
                        as *const libc::c_char,
                    crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                        as *const libc::c_char,
                );
                *args.offset(1 as i32 as isize) =
                    crate::src::qcommon::q_shared::EXEC_INSERT as i32 as crate::stdlib::intptr_t
            }
            crate::src::qcommon::cmd::Cbuf_ExecuteText(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *const libc::c_char,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        13 => {
            return crate::src::qcommon::files::FS_FOpenFileByMode(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut crate::src::qcommon::q_shared::fileHandle_t,
                *args.offset(3 as i32 as isize) as crate::src::qcommon::q_shared::fsMode_t,
            ) as crate::stdlib::intptr_t
        }
        14 => {
            crate::src::qcommon::files::FS_Read(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize)),
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as crate::src::qcommon::q_shared::fileHandle_t,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        15 => {
            crate::src::qcommon::files::FS_Write(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize)),
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as crate::src::qcommon::q_shared::fileHandle_t,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        16 => {
            crate::src::qcommon::files::FS_FCloseFile(
                *args.offset(1 as i32 as isize) as crate::src::qcommon::q_shared::fileHandle_t
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        17 => {
            return crate::src::qcommon::files::FS_GetFileList(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(3 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(4 as i32 as isize) as i32,
            ) as crate::stdlib::intptr_t
        }
        86 => {
            return crate::src::qcommon::files::FS_Seek(
                *args.offset(1 as i32 as isize) as crate::src::qcommon::q_shared::fileHandle_t,
                *args.offset(2 as i32 as isize),
                *args.offset(3 as i32 as isize) as i32,
            ) as crate::stdlib::intptr_t
        }
        18 => {
            return crate::src::client::cl_main::re
                .RegisterModel
                .expect("non-null function pointer")(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        19 => {
            return crate::src::client::cl_main::re
                .RegisterSkin
                .expect("non-null function pointer")(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        20 => {
            return crate::src::client::cl_main::re
                .RegisterShaderNoMip
                .expect("non-null function pointer")(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        21 => {
            crate::src::client::cl_main::re
                .ClearScene
                .expect("non-null function pointer")();
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        22 => {
            crate::src::client::cl_main::re
                .AddRefEntityToScene
                .expect("non-null function pointer")(
                crate::src::qcommon::vm::VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *const crate::tr_types_h::refEntity_t
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        23 => {
            crate::src::client::cl_main::re
                .AddPolyToScene
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as crate::src::qcommon::q_shared::qhandle_t,
                *args.offset(2 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(3 as i32 as isize))
                    as *const crate::tr_types_h::polyVert_t,
                1 as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        24 => {
            crate::src::client::cl_main::re
                .AddLightToScene
                .expect("non-null function pointer")(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
                _vmf(*args.offset(2 as i32 as isize)),
                _vmf(*args.offset(3 as i32 as isize)),
                _vmf(*args.offset(4 as i32 as isize)),
                _vmf(*args.offset(5 as i32 as isize)),
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        25 => {
            crate::src::client::cl_main::re
                .RenderScene
                .expect("non-null function pointer")(
                crate::src::qcommon::vm::VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *const crate::tr_types_h::refdef_t
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        26 => {
            crate::src::client::cl_main::re
                .SetColor
                .expect("non-null function pointer")(
                crate::src::qcommon::vm::VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            ) as *const f32
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        27 => {
            crate::src::client::cl_main::re
                .DrawStretchPic
                .expect("non-null function pointer")(
                _vmf(*args.offset(1 as i32 as isize)),
                _vmf(*args.offset(2 as i32 as isize)),
                _vmf(*args.offset(3 as i32 as isize)),
                _vmf(*args.offset(4 as i32 as isize)),
                _vmf(*args.offset(5 as i32 as isize)),
                _vmf(*args.offset(6 as i32 as isize)),
                _vmf(*args.offset(7 as i32 as isize)),
                _vmf(*args.offset(8 as i32 as isize)),
                *args.offset(9 as i32 as isize) as crate::src::qcommon::q_shared::qhandle_t,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        56 => {
            crate::src::client::cl_main::re
                .ModelBounds
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as crate::src::qcommon::q_shared::qhandle_t,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(3 as i32 as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        28 => {
            crate::src::client::cl_scrn::SCR_UpdateScreen();
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        29 => {
            crate::src::client::cl_main::re
                .LerpTag
                .expect("non-null function pointer")(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *mut crate::src::qcommon::q_shared::orientation_t,
                *args.offset(2 as i32 as isize) as crate::src::qcommon::q_shared::qhandle_t,
                *args.offset(3 as i32 as isize) as i32,
                *args.offset(4 as i32 as isize) as i32,
                _vmf(*args.offset(5 as i32 as isize)),
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(6 as i32 as isize))
                    as *const libc::c_char,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        31 => {
            return crate::src::client::snd_main::S_RegisterSound(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
                *args.offset(2 as i32 as isize) as crate::src::qcommon::q_shared::qboolean,
            ) as crate::stdlib::intptr_t
        }
        32 => {
            crate::src::client::snd_main::S_StartLocalSound(
                *args.offset(1 as i32 as isize) as crate::src::qcommon::q_shared::sfxHandle_t,
                *args.offset(2 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        33 => {
            Key_KeynumToStringBuf(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        34 => {
            Key_GetBindingBuf(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        35 => {
            crate::src::client::cl_keys::Key_SetBinding(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *const libc::c_char,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        36 => {
            return crate::src::client::cl_keys::Key_IsDown(*args.offset(1 as i32 as isize) as i32)
                as crate::stdlib::intptr_t
        }
        37 => {
            return crate::src::client::cl_keys::Key_GetOverstrikeMode() as crate::stdlib::intptr_t
        }
        38 => {
            crate::src::client::cl_keys::Key_SetOverstrikeMode(
                *args.offset(1 as i32 as isize) as crate::src::qcommon::q_shared::qboolean
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        39 => {
            crate::src::client::cl_keys::Key_ClearStates();
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        40 => return crate::src::client::cl_keys::Key_GetCatcher() as crate::stdlib::intptr_t,
        41 => {
            // Don't allow the ui module to close the console
            crate::src::client::cl_keys::Key_SetCatcher(
                (*args.offset(1 as i32 as isize)
                    | (crate::src::client::cl_keys::Key_GetCatcher() & 0x1 as i32) as libc::c_long)
                    as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        42 => {
            CL_GetClipboardData(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(2 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        44 => {
            GetClientState(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *mut crate::ui_public_h::uiClientState_t,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        43 => {
            CL_GetGlconfig(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *mut crate::tr_types_h::glconfig_t,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        45 => {
            return GetConfigString(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            ) as crate::stdlib::intptr_t
        }
        71 => {
            LAN_LoadCachedServers();
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        72 => {
            LAN_SaveServersToCache();
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        73 => {
            return LAN_AddServer(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(3 as i32 as isize))
                    as *const libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        74 => {
            LAN_RemoveServer(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *const libc::c_char,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        46 => return LAN_GetPingQueueCount() as crate::stdlib::intptr_t,
        47 => {
            LAN_ClearPing(*args.offset(1 as i32 as isize) as i32);
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        48 => {
            LAN_GetPing(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(4 as i32 as isize)) as *mut i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        49 => {
            LAN_GetPingInfo(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        65 => {
            return LAN_GetServerCount(*args.offset(1 as i32 as isize) as i32)
                as crate::stdlib::intptr_t
        }
        66 => {
            LAN_GetServerAddressString(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(3 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(4 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        67 => {
            LAN_GetServerInfo(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(3 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(4 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        83 => {
            return LAN_GetServerPing(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            ) as crate::stdlib::intptr_t
        }
        68 => {
            LAN_MarkServerVisible(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as crate::src::qcommon::q_shared::qboolean,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        84 => {
            return LAN_ServerIsVisible(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
            ) as crate::stdlib::intptr_t
        }
        69 => {
            return LAN_UpdateVisiblePings(*args.offset(1 as i32 as isize) as i32)
                as crate::stdlib::intptr_t
        }
        70 => {
            LAN_ResetPings(*args.offset(1 as i32 as isize) as i32);
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        82 => {
            return LAN_GetServerStatus(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *mut libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(3 as i32 as isize) as i32,
            ) as crate::stdlib::intptr_t
        }
        85 => {
            return LAN_CompareServers(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as i32,
                *args.offset(4 as i32 as isize) as i32,
                *args.offset(5 as i32 as isize) as i32,
            ) as crate::stdlib::intptr_t
        }
        52 => {
            return crate::src::qcommon::common::Hunk_MemoryRemaining() as crate::stdlib::intptr_t
        }
        53 => {
            CLUI_GetCDKey(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *mut libc::c_char,
                *args.offset(2 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        54 => {
            CLUI_SetCDKey(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *mut libc::c_char,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        87 => return 0 as i32 as crate::stdlib::intptr_t,
        55 => {
            crate::src::client::cl_main::re
                .RegisterFont
                .expect("non-null function pointer")(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
                *args.offset(2 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(3 as i32 as isize))
                    as *mut crate::src::qcommon::q_shared::fontInfo_t,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        100 => {
            crate::stdlib::memset(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize)),
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as libc::c_ulong,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        101 => {
            crate::stdlib::memcpy(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize)),
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize)),
                *args.offset(3 as i32 as isize) as libc::c_ulong,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        102 => {
            crate::stdlib::strncpy(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *mut libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *const libc::c_char,
                *args.offset(3 as i32 as isize) as libc::c_ulong,
            );
            return *args.offset(1 as i32 as isize);
        }
        103 => {
            return FloatAsInt(
                crate::stdlib::sin(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as crate::stdlib::intptr_t
        }
        104 => {
            return FloatAsInt(
                crate::stdlib::cos(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as crate::stdlib::intptr_t
        }
        105 => {
            return FloatAsInt(crate::stdlib::atan2(
                _vmf(*args.offset(1 as i32 as isize)) as f64,
                _vmf(*args.offset(2 as i32 as isize)) as f64,
            ) as f32) as crate::stdlib::intptr_t
        }
        106 => {
            return FloatAsInt(
                crate::stdlib::sqrt(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as crate::stdlib::intptr_t
        }
        107 => {
            return FloatAsInt(
                crate::stdlib::floor(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as crate::stdlib::intptr_t
        }
        108 => {
            return FloatAsInt(
                crate::stdlib::ceil(_vmf(*args.offset(1 as i32 as isize)) as f64) as f32,
            ) as crate::stdlib::intptr_t
        }
        57 => {
            return (*botlib_export)
                .PC_AddGlobalDefine
                .expect("non-null function pointer")(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *mut libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        58 => {
            return (*botlib_export)
                .PC_LoadSourceHandle
                .expect("non-null function pointer")(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        59 => {
            return (*botlib_export)
                .PC_FreeSourceHandle
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32
            ) as crate::stdlib::intptr_t
        }
        60 => {
            return (*botlib_export)
                .PC_ReadTokenHandle
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut crate::src::qcommon::q_shared::pc_token_t,
            ) as crate::stdlib::intptr_t
        }
        61 => {
            return (*botlib_export)
                .PC_SourceFileAndLine
                .expect("non-null function pointer")(
                *args.offset(1 as i32 as isize) as i32,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *mut libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(3 as i32 as isize)) as *mut i32,
            ) as crate::stdlib::intptr_t
        }
        62 => {
            crate::src::client::snd_main::S_StopBackgroundTrack();
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        63 => {
            crate::src::client::snd_main::S_StartBackgroundTrack(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *const libc::c_char,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        64 => {
            return crate::src::qcommon::common::Com_RealTime(crate::src::qcommon::vm::VM_ArgPtr(
                *args.offset(1 as i32 as isize),
            )
                as *mut crate::src::qcommon::q_shared::qtime_t
                as *mut crate::src::qcommon::q_shared::qtime_s)
                as crate::stdlib::intptr_t
        }
        75 => {
            crate::src::qcommon::common::Com_DPrintf(
                b"UI_CIN_PlayCinematic\n\x00" as *const u8 as *const libc::c_char,
            );
            return crate::src::client::cl_cin::CIN_PlayCinematic(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as i32,
                *args.offset(4 as i32 as isize) as i32,
                *args.offset(5 as i32 as isize) as i32,
                *args.offset(6 as i32 as isize) as i32,
            ) as crate::stdlib::intptr_t;
        }
        76 => {
            return crate::src::client::cl_cin::CIN_StopCinematic(
                *args.offset(1 as i32 as isize) as i32
            ) as crate::stdlib::intptr_t
        }
        77 => {
            return crate::src::client::cl_cin::CIN_RunCinematic(
                *args.offset(1 as i32 as isize) as i32
            ) as crate::stdlib::intptr_t
        }
        78 => {
            crate::src::client::cl_cin::CIN_DrawCinematic(*args.offset(1 as i32 as isize) as i32);
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        79 => {
            crate::src::client::cl_cin::CIN_SetExtents(
                *args.offset(1 as i32 as isize) as i32,
                *args.offset(2 as i32 as isize) as i32,
                *args.offset(3 as i32 as isize) as i32,
                *args.offset(4 as i32 as isize) as i32,
                *args.offset(5 as i32 as isize) as i32,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        80 => {
            crate::src::client::cl_main::re
                .RemapShader
                .expect("non-null function pointer")(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(3 as i32 as isize))
                    as *const libc::c_char,
            );
            return 0 as i32 as crate::stdlib::intptr_t;
        }
        81 => {
            return crate::src::client::cl_main::CL_CDKeyValidate(
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(1 as i32 as isize))
                    as *const libc::c_char,
                crate::src::qcommon::vm::VM_ArgPtr(*args.offset(2 as i32 as isize))
                    as *const libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        _ => {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"Bad UI system trap: %ld\x00" as *const u8 as *const libc::c_char,
                *args.offset(0 as i32 as isize),
            );
        }
    };
}
/*
====================
CL_ShutdownUI
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ShutdownUI() {
    crate::src::client::cl_keys::Key_SetCatcher(
        crate::src::client::cl_keys::Key_GetCatcher() & !(0x2 as i32),
    );
    crate::src::client::cl_main::cls.uiStarted = crate::src::qcommon::q_shared::qfalse;
    if uivm.is_null() {
        return;
    }
    crate::src::qcommon::vm::VM_Call(uivm, crate::ui_public_h::UI_SHUTDOWN as i32);
    crate::src::qcommon::vm::VM_Free(uivm);
    uivm = 0 as *mut crate::qcommon_h::vm_t;
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
//
// cl_ui.c
//
#[no_mangle]

pub unsafe extern "C" fn CL_InitUI() {
    let mut v: i32 = 0;
    let mut interpret: crate::qcommon_h::vmInterpret_t = crate::qcommon_h::VMI_NATIVE;
    // load the dll or bytecode
    interpret = crate::src::qcommon::cvar::Cvar_VariableValue(
        b"vm_ui\x00" as *const u8 as *const libc::c_char,
    ) as crate::qcommon_h::vmInterpret_t;
    if crate::src::client::cl_parse::cl_connectedToPureServer != 0 {
        // if sv_pure is set we only allow qvms to be loaded
        if interpret as u32 != crate::qcommon_h::VMI_COMPILED as i32 as u32
            && interpret as u32 != crate::qcommon_h::VMI_BYTECODE as i32 as u32
        {
            interpret = crate::qcommon_h::VMI_COMPILED
        }
    }
    uivm = crate::src::qcommon::vm::VM_Create(
        b"ui\x00" as *const u8 as *const libc::c_char,
        Some(
            CL_UISystemCalls
                as unsafe extern "C" fn(_: *mut crate::stdlib::intptr_t) -> crate::stdlib::intptr_t,
        ),
        interpret,
    );
    if uivm.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"VM_Create on UI failed\x00" as *const u8 as *const libc::c_char,
        );
    }
    // sanity check
    v = crate::src::qcommon::vm::VM_Call(uivm, crate::ui_public_h::UI_GETAPIVERSION as i32) as i32;
    if v == 4 as i32 {
        //		Com_Printf(S_COLOR_YELLOW "WARNING: loading old Quake III Arena User Interface version %d\n", v );
        // init for this gamestate
        crate::src::qcommon::vm::VM_Call(
            uivm,
            crate::ui_public_h::UI_INIT as i32,
            (crate::src::client::cl_main::clc.state as u32
                >= crate::src::qcommon::q_shared::CA_AUTHORIZING as i32 as u32
                && (crate::src::client::cl_main::clc.state as u32)
                    < crate::src::qcommon::q_shared::CA_ACTIVE as i32 as u32) as i32,
        );
    } else if v != 6 as i32 {
        // Free uivm now, so UI_SHUTDOWN doesn't get called later.
        crate::src::qcommon::vm::VM_Free(uivm);
        uivm = 0 as *mut crate::qcommon_h::vm_t;
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"User Interface is version %d, expected %d\x00" as *const u8 as *const libc::c_char,
            v,
            6 as i32,
        );
    } else {
        // init for this gamestate
        crate::src::qcommon::vm::VM_Call(
            uivm,
            crate::ui_public_h::UI_INIT as i32,
            (crate::src::client::cl_main::clc.state as u32
                >= crate::src::qcommon::q_shared::CA_AUTHORIZING as i32 as u32
                && (crate::src::client::cl_main::clc.state as u32)
                    < crate::src::qcommon::q_shared::CA_ACTIVE as i32 as u32) as i32,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn UI_usesUniqueCDKey() -> crate::src::qcommon::q_shared::qboolean {
    if !uivm.is_null() {
        return (crate::src::qcommon::vm::VM_Call(
            uivm,
            crate::ui_public_h::UI_HASUNIQUECDKEY as i32,
        ) == crate::src::qcommon::q_shared::qtrue as i32 as libc::c_long) as i32
            as crate::src::qcommon::q_shared::qboolean;
    } else {
        return crate::src::qcommon::q_shared::qfalse;
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
// call before filesystem access
// FIXME: move logging to common?
// AVI files have the start of pixel lines 4 byte-aligned
//
// server interface
//
//
// UI interface
//
/*
====================
UI_GameCommand

See if the current console command is claimed by the ui
====================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GameCommand() -> crate::src::qcommon::q_shared::qboolean {
    if uivm.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::vm::VM_Call(
        uivm,
        crate::ui_public_h::UI_CONSOLE_COMMAND as i32,
        crate::src::client::cl_main::cls.realtime,
    ) as crate::src::qcommon::q_shared::qboolean;
}

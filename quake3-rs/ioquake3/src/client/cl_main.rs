use ::libc;

pub mod stdlib_h {

    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return ::libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
    }
}

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
        return ::libc::strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    }
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::wchar_t;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__uint8_t;

pub use crate::stdlib::C2RustUnnamed_18;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__mbstate_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::src::client::cl_main::stdlib_h::atoi;
pub use crate::stdlib::_ISalnum;
pub use crate::stdlib::_ISalpha;
pub use crate::stdlib::_ISblank;
pub use crate::stdlib::_IScntrl;
pub use crate::stdlib::_ISdigit;
pub use crate::stdlib::_ISgraph;
pub use crate::stdlib::_ISlower;
pub use crate::stdlib::_ISprint;
pub use crate::stdlib::_ISpunct;
pub use crate::stdlib::_ISspace;
pub use crate::stdlib::_ISupper;
pub use crate::stdlib::_ISxdigit;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::__ctype_b_loc;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::qsort;
pub use crate::stdlib::uint8_t;
pub use ::libc::rand;
pub use ::libc::strtod;
pub use ::libc::strtol;

pub use crate::qcommon_h::completionFunc_t;
pub use crate::qcommon_h::field_t;
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
pub use crate::qcommon_h::TAG_BOTLIB;
pub use crate::qcommon_h::TAG_FREE;
pub use crate::qcommon_h::TAG_GENERAL;
pub use crate::qcommon_h::TAG_RENDERER;
pub use crate::qcommon_h::TAG_SMALL;
pub use crate::qcommon_h::TAG_STATIC;
pub use crate::src::asm::ftola::qftolsse;
pub use crate::src::client::cl_scrn::SCR_DebugGraph;
pub use crate::src::client::snd_main::S_ClearSoundBuffer;
pub use crate::src::qcommon::cmd::Cbuf_AddText;
pub use crate::src::qcommon::cmd::Cbuf_Execute;
pub use crate::src::qcommon::cmd::Cbuf_ExecuteText;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_Args;
pub use crate::src::qcommon::cmd::Cmd_ArgsFrom;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::cmd::Cmd_Cmd;
pub use crate::src::qcommon::cmd::Cmd_RemoveCommand;
pub use crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc;
pub use crate::src::qcommon::cmd::Cmd_TokenizeString;
pub use crate::src::qcommon::common::cl_cdkey;
pub use crate::src::qcommon::common::cl_paused;
pub use crate::src::qcommon::common::com_cl_running;
pub use crate::src::qcommon::common::com_dedicated;
pub use crate::src::qcommon::common::com_errorEntered;
pub use crate::src::qcommon::common::com_fullyInitialized;
pub use crate::src::qcommon::common::com_gamename;
pub use crate::src::qcommon::common::com_legacyprotocol;
pub use crate::src::qcommon::common::com_protocol;
pub use crate::src::qcommon::common::com_standalone;
pub use crate::src::qcommon::common::com_sv_running;
pub use crate::src::qcommon::common::com_timescale;
pub use crate::src::qcommon::common::com_version;
pub use crate::src::qcommon::common::demo_protocols;
pub use crate::src::qcommon::common::sv_paused;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_EventLoop;
pub use crate::src::qcommon::common::Com_FieldStringToPlayerName;
pub use crate::src::qcommon::common::Com_Milliseconds;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Com_RandomBytes;
pub use crate::src::qcommon::common::Com_RealTime;
pub use crate::src::qcommon::common::Com_strCompare;
pub use crate::src::qcommon::common::Field_CompleteCommand;
pub use crate::src::qcommon::common::Field_CompleteFilename;
pub use crate::src::qcommon::common::Field_CompletePlayerName;
pub use crate::src::qcommon::common::Hunk_Alloc;
pub use crate::src::qcommon::common::Hunk_AllocateTempMemory;
pub use crate::src::qcommon::common::Hunk_Clear;
pub use crate::src::qcommon::common::Hunk_ClearToMark;
pub use crate::src::qcommon::common::Hunk_FreeTempMemory;
pub use crate::src::qcommon::common::Info_Print;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::common::Z_TagMalloc;
pub use crate::src::qcommon::cvar::cvar_modifiedFlags;
pub use crate::src::qcommon::cvar::Cvar_CheckRange;
pub use crate::src::qcommon::cvar::Cvar_ForceReset;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_InfoString;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_Set2;
pub use crate::src::qcommon::cvar::Cvar_SetDescription;
pub use crate::src::qcommon::cvar::Cvar_SetValue;
pub use crate::src::qcommon::cvar::Cvar_VariableIntegerValue;
pub use crate::src::qcommon::cvar::Cvar_VariableString;
pub use crate::src::qcommon::cvar::Cvar_VariableStringBuffer;
pub use crate::src::qcommon::cvar::Cvar_VariableValue;
pub use crate::src::qcommon::files::FS_BuildOSPath;
pub use crate::src::qcommon::files::FS_ClearPakReferences;
pub use crate::src::qcommon::files::FS_ComparePaks;
pub use crate::src::qcommon::files::FS_CompareZipChecksum;
pub use crate::src::qcommon::files::FS_ConditionalRestart;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_FOpenFileRead;
pub use crate::src::qcommon::files::FS_FOpenFileWrite;
pub use crate::src::qcommon::files::FS_FileExists;
pub use crate::src::qcommon::files::FS_FileIsInPAK;
pub use crate::src::qcommon::files::FS_FreeFile;
pub use crate::src::qcommon::files::FS_FreeFileList;
pub use crate::src::qcommon::files::FS_ListFiles;
pub use crate::src::qcommon::files::FS_LoadedPakNames;
pub use crate::src::qcommon::files::FS_Printf;
pub use crate::src::qcommon::files::FS_PureServerSetLoadedPaks;
pub use crate::src::qcommon::files::FS_PureServerSetReferencedPaks;
pub use crate::src::qcommon::files::FS_Read;
pub use crate::src::qcommon::files::FS_ReadFile;
pub use crate::src::qcommon::files::FS_ReferencedPakNames;
pub use crate::src::qcommon::files::FS_ReferencedPakPureChecksums;
pub use crate::src::qcommon::files::FS_Restart;
pub use crate::src::qcommon::files::FS_SV_FOpenFileRead;
pub use crate::src::qcommon::files::FS_SV_FOpenFileWrite;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::files::FS_WriteFile;
pub use crate::src::qcommon::md5::Com_MD5File;
pub use crate::src::qcommon::msg::MSG_BeginReadingOOB;
pub use crate::src::qcommon::msg::MSG_Bitstream;
pub use crate::src::qcommon::msg::MSG_Init;
pub use crate::src::qcommon::msg::MSG_ReadLong;
pub use crate::src::qcommon::msg::MSG_ReadString;
pub use crate::src::qcommon::msg::MSG_ReadStringLine;
pub use crate::src::qcommon::msg::MSG_WriteBigString;
pub use crate::src::qcommon::msg::MSG_WriteByte;
pub use crate::src::qcommon::msg::MSG_WriteDeltaEntity;
pub use crate::src::qcommon::msg::MSG_WriteLong;
pub use crate::src::qcommon::msg::MSG_WriteShort;
pub use crate::src::qcommon::net_chan::NET_OutOfBandData;
pub use crate::src::qcommon::net_chan::NET_OutOfBandPrint;
pub use crate::src::qcommon::net_chan::NET_SendPacket;
pub use crate::src::qcommon::net_chan::NET_StringToAdr;
pub use crate::src::qcommon::net_chan::Netchan_Setup;
pub use crate::src::qcommon::net_ip::NET_AdrToString;
pub use crate::src::qcommon::net_ip::NET_AdrToStringwPort;
pub use crate::src::qcommon::net_ip::NET_CompareAdr;
pub use crate::src::qcommon::net_ip::NET_IsLocalAddress;
pub use crate::src::qcommon::net_ip::Sys_IsLANAddress;
pub use crate::src::qcommon::net_ip::Sys_ShowIP;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fontInfo_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::glyphInfo_t;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::markFragment_t;
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtime_s;
pub use crate::src::qcommon::q_shared::qtime_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::COM_SkipPath;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::qcommon::q_shared::Com_SkipTokens;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_isanumber;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_stricmpn;
pub use crate::src::qcommon::q_shared::Q_strncmp;
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
pub use crate::src::qcommon::q_shared::PRINT_ALL;
pub use crate::src::qcommon::q_shared::PRINT_DEVELOPER;
pub use crate::src::qcommon::q_shared::PRINT_ERROR;
pub use crate::src::qcommon::q_shared::PRINT_WARNING;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::qcommon::vm::VM_Call;
pub use crate::src::sdl::sdl_input::IN_Init;
pub use crate::src::sdl::sdl_input::IN_Restart;
pub use crate::src::sdl::sdl_input::IN_Shutdown;
pub use crate::src::server::sv_init::SV_Shutdown;
pub use crate::src::server::sv_main::SV_Frame;
pub use crate::src::sys::sys_unix::Sys_LowPhysicalMemory;
pub use crate::src::sys::sys_unix::Sys_Milliseconds;
pub use crate::src::sys::sys_unix::Sys_SetEnv;
pub use crate::stdlib::mbstate_t;
pub use crate::vm_local_h::vm_s;

pub use crate::client_h::clSnapshot_t;
pub use crate::client_h::clientActive_t;
pub use crate::client_h::clientConnection_t;
pub use crate::client_h::clientStatic_t;
pub use crate::client_h::outPacket_t;
pub use crate::client_h::ping_t;
pub use crate::client_h::serverInfo_t;
pub use crate::curl_h::CURL;
pub use crate::multi_h::CURLM;
pub use crate::src::client::cl_avi::CL_CloseAVI;
pub use crate::src::client::cl_avi::CL_OpenAVIForWriting;
pub use crate::src::client::cl_avi::CL_TakeVideoFrame;
pub use crate::src::client::cl_avi::CL_VideoRecording;
pub use crate::src::client::cl_avi::CL_WriteAVIVideoFrame;
pub use crate::src::client::cl_cgame::CL_InitCGame;
pub use crate::src::client::cl_cgame::CL_SetCGameTime;
pub use crate::src::client::cl_cgame::CL_ShutdownCGame;
pub use crate::src::client::cl_cin::CIN_PlayCinematic;
pub use crate::src::client::cl_cin::CIN_RunCinematic;
pub use crate::src::client::cl_cin::CIN_UploadCinematic;
pub use crate::src::client::cl_cin::CL_PlayCinematic_f;
pub use crate::src::client::cl_cin::SCR_RunCinematic;
pub use crate::src::client::cl_cin::SCR_StopCinematic;
pub use crate::src::client::cl_console::g_console_field_width;
pub use crate::src::client::cl_console::Con_Close;
pub use crate::src::client::cl_console::Con_Init;
pub use crate::src::client::cl_console::Con_RunConsole;
pub use crate::src::client::cl_console::Con_Shutdown;
pub use crate::src::client::cl_input::cl_anglespeedkey;
pub use crate::src::client::cl_input::cl_pitchspeed;
pub use crate::src::client::cl_input::cl_run;
pub use crate::src::client::cl_input::cl_yawspeed;
pub use crate::src::client::cl_input::CL_InitInput;
pub use crate::src::client::cl_input::CL_SendCmd;
pub use crate::src::client::cl_input::CL_ShutdownInput;
pub use crate::src::client::cl_input::CL_WritePacket;
pub use crate::src::client::cl_keys::Key_GetCatcher;
pub use crate::src::client::cl_keys::Key_SetCatcher;
pub use crate::src::client::cl_net_chan::CL_Netchan_Process;
pub use crate::src::client::cl_parse::cl_connectedToPureServer;
pub use crate::src::client::cl_parse::CL_ParseServerMessage;
pub use crate::src::client::cl_scrn::cl_timegraph;
pub use crate::src::client::cl_scrn::SCR_Init;
pub use crate::src::client::cl_scrn::SCR_UpdateScreen;
pub use crate::src::client::cl_ui::uivm;
pub use crate::src::client::cl_ui::CL_InitUI;
pub use crate::src::client::cl_ui::CL_ShutdownUI;

pub use crate::tr_public_h::refexport_t;
pub use crate::tr_public_h::refimport_t;
pub use crate::tr_public_h::GetRefAPI_t;
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

pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::cg_public_h::CG_CONSOLE_COMMAND;
pub use crate::cg_public_h::CG_CROSSHAIR_PLAYER;
pub use crate::cg_public_h::CG_DRAW_ACTIVE_FRAME;
pub use crate::cg_public_h::CG_EVENT_HANDLING;
pub use crate::cg_public_h::CG_INIT;
pub use crate::cg_public_h::CG_KEY_EVENT;
pub use crate::cg_public_h::CG_LAST_ATTACKER;
pub use crate::cg_public_h::CG_MOUSE_EVENT;
pub use crate::cg_public_h::CG_SHUTDOWN;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::src::client::cl_main::stdlib_float_h::atof;

pub use crate::ui_public_h::UIMENU_BAD_CD_KEY;
pub use crate::ui_public_h::UIMENU_INGAME;
pub use crate::ui_public_h::UIMENU_MAIN;
pub use crate::ui_public_h::UIMENU_NEED_CD;
pub use crate::ui_public_h::UIMENU_NONE;
pub use crate::ui_public_h::UIMENU_POSTGAME;
pub use crate::ui_public_h::UIMENU_TEAM;
pub use crate::ui_public_h::UI_CONSOLE_COMMAND;
pub use crate::ui_public_h::UI_DRAW_CONNECT_SCREEN;
pub use crate::ui_public_h::UI_GETAPIVERSION;
pub use crate::ui_public_h::UI_HASUNIQUECDKEY;
pub use crate::ui_public_h::UI_INIT;
pub use crate::ui_public_h::UI_IS_FULLSCREEN;
pub use crate::ui_public_h::UI_KEY_EVENT;
pub use crate::ui_public_h::UI_MOUSE_EVENT;
pub use crate::ui_public_h::UI_REFRESH;
pub use crate::ui_public_h::UI_SET_ACTIVE_MENU;
pub use crate::ui_public_h::UI_SHUTDOWN;

pub type serverStatus_t = serverStatus_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serverStatus_s {
    pub string: [libc::c_char; 8192],
    pub address: crate::qcommon_h::netadr_t,
    pub time: libc::c_int,
    pub startTime: libc::c_int,
    pub pending: crate::src::qcommon::q_shared::qboolean,
    pub print: crate::src::qcommon::q_shared::qboolean,
    pub retrieved: crate::src::qcommon::q_shared::qboolean,
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
// cl_main.c  -- client main loop
#[no_mangle]

pub static mut cl_useMumble: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_mumbleScale: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_voipUseVAD: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_voipVADThreshold: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_voipSend: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_voipSendTarget: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_voipGainDuringCapture: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_voipCaptureMult: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_voipShowMeter: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_voipProtocol: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_voip: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_renderer: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_nodelta: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_debugMove: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_noprint: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_motd: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut rcon_client_password: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut rconAddress: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_timeout: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_maxpackets: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_packetdup: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_timeNudge: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_showTimeDelta: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_freezeDemo: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_shownet: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_showSend: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_timedemo: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_timedemoLog: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_autoRecordDemo: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_aviFrameRate: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_aviMotionJpeg: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_forceavidemo: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_freelook: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_sensitivity: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_mouseAccel: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_mouseAccelOffset: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_mouseAccelStyle: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_showMouseRate: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut m_pitch: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut m_yaw: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut m_forward: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut m_side: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut m_filter: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut j_pitch: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut j_yaw: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut j_forward: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut j_side: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut j_up: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut j_pitch_axis: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut j_yaw_axis: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut j_forward_axis: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut j_side_axis: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut j_up_axis: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_activeAction: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_motdString: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_allowDownload: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_conXOffset: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_inGameVideo: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_serverStatusResendTime: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_lanForcePackets: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_guidServerUniq: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_consoleKeys: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_rate: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl: crate::client_h::clientActive_t = crate::client_h::clientActive_t {
    timeoutcount: 0,
    snap: crate::client_h::clSnapshot_t {
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
    },
    serverTime: 0,
    oldServerTime: 0,
    oldFrameServerTime: 0,
    serverTimeDelta: 0,
    extrapolatedSnapshot: crate::src::qcommon::q_shared::qfalse,
    newSnapshots: crate::src::qcommon::q_shared::qfalse,
    gameState: crate::src::qcommon::q_shared::gameState_t {
        stringOffsets: [0; 1024],
        stringData: [0; 16000],
        dataCount: 0,
    },
    mapname: [0; 64],
    parseEntitiesNum: 0,
    mouseDx: [0; 2],
    mouseDy: [0; 2],
    mouseIndex: 0,
    joystickAxis: [0; 16],
    cgameUserCmdValue: 0,
    cgameSensitivity: 0.,
    cmds: [crate::src::qcommon::q_shared::usercmd_t {
        serverTime: 0,
        angles: [0; 3],
        buttons: 0,
        weapon: 0,
        forwardmove: 0,
        rightmove: 0,
        upmove: 0,
    }; 64],
    cmdNumber: 0,
    outPackets: [crate::client_h::outPacket_t {
        p_cmdNumber: 0,
        p_serverTime: 0,
        p_realtime: 0,
    }; 32],
    viewangles: [0.; 3],
    serverId: 0,
    snapshots: [crate::client_h::clSnapshot_t {
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
    }; 32],
    entityBaselines: [crate::src::qcommon::q_shared::entityState_t {
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
    }; 1024],
    parseEntities: [crate::src::qcommon::q_shared::entityState_t {
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
    }; 8192],
};
#[no_mangle]

pub static mut clc: crate::client_h::clientConnection_t = crate::client_h::clientConnection_t {
    state: crate::src::qcommon::q_shared::CA_UNINITIALIZED,
    clientNum: 0,
    lastPacketSentTime: 0,
    lastPacketTime: 0,
    servername: [0; 4096],
    serverAddress: crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    },
    connectTime: 0,
    connectPacketCount: 0,
    serverMessage: [0; 1024],
    challenge: 0,
    checksumFeed: 0,
    reliableSequence: 0,
    reliableAcknowledge: 0,
    reliableCommands: [[0; 1024]; 64],
    serverMessageSequence: 0,
    serverCommandSequence: 0,
    lastExecutedServerCommand: 0,
    serverCommands: [[0; 1024]; 64],
    download: 0,
    downloadTempName: [0; 4096],
    downloadName: [0; 4096],
    cURLEnabled: crate::src::qcommon::q_shared::qfalse,
    cURLUsed: crate::src::qcommon::q_shared::qfalse,
    cURLDisconnected: crate::src::qcommon::q_shared::qfalse,
    downloadURL: [0; 4096],
    downloadCURL: 0 as *const libc::c_void as *mut libc::c_void,
    downloadCURLM: 0 as *const libc::c_void as *mut libc::c_void,
    sv_allowDownload: 0,
    sv_dlURL: [0; 256],
    downloadNumber: 0,
    downloadBlock: 0,
    downloadCount: 0,
    downloadSize: 0,
    downloadList: [0; 1024],
    downloadRestart: crate::src::qcommon::q_shared::qfalse,
    demoName: [0; 64],
    spDemoRecording: crate::src::qcommon::q_shared::qfalse,
    demorecording: crate::src::qcommon::q_shared::qfalse,
    demoplaying: crate::src::qcommon::q_shared::qfalse,
    demowaiting: crate::src::qcommon::q_shared::qfalse,
    firstDemoFrameSkipped: crate::src::qcommon::q_shared::qfalse,
    demofile: 0,
    timeDemoFrames: 0,
    timeDemoStart: 0,
    timeDemoBaseTime: 0,
    timeDemoLastFrame: 0,
    timeDemoMinDuration: 0,
    timeDemoMaxDuration: 0,
    timeDemoDurations: [0; 4096],
    aviVideoFrameRemainder: 0.,
    aviSoundFrameRemainder: 0.,
    voipEnabled: crate::src::qcommon::q_shared::qfalse,
    voipCodecInitialized: crate::src::qcommon::q_shared::qfalse,
    opusDecoder: [0 as *const crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder
        as *mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder; 64],
    voipIncomingGeneration: [0; 64],
    voipIncomingSequence: [0; 64],
    voipGain: [0.; 64],
    voipIgnore: [crate::src::qcommon::q_shared::qfalse; 64],
    voipMuteAll: crate::src::qcommon::q_shared::qfalse,
    voipTargets: [0; 8],
    voipFlags: 0,
    opusEncoder: 0 as *const crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder
        as *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
    voipOutgoingDataSize: 0,
    voipOutgoingDataFrames: 0,
    voipOutgoingSequence: 0,
    voipOutgoingGeneration: 0,
    voipOutgoingData: [0; 1024],
    voipPower: 0.,
    compat: crate::src::qcommon::q_shared::qfalse,
    netchan: crate::qcommon_h::netchan_t {
        sock: crate::qcommon_h::NS_CLIENT,
        dropped: 0,
        remoteAddress: crate::qcommon_h::netadr_t {
            type_0: crate::qcommon_h::NA_BAD,
            ip: [0; 4],
            ip6: [0; 16],
            port: 0,
            scope_id: 0,
        },
        qport: 0,
        incomingSequence: 0,
        outgoingSequence: 0,
        fragmentSequence: 0,
        fragmentLength: 0,
        fragmentBuffer: [0; 16384],
        unsentFragments: crate::src::qcommon::q_shared::qfalse,
        unsentFragmentStart: 0,
        unsentLength: 0,
        unsentBuffer: [0; 16384],
        challenge: 0,
        lastSentTime: 0,
        lastSentSize: 0,
        compat: crate::src::qcommon::q_shared::qfalse,
    },
};
#[no_mangle]

pub static mut cls: crate::client_h::clientStatic_t = crate::client_h::clientStatic_t {
    cddialog: crate::src::qcommon::q_shared::qfalse,
    rendererStarted: crate::src::qcommon::q_shared::qfalse,
    soundStarted: crate::src::qcommon::q_shared::qfalse,
    soundRegistered: crate::src::qcommon::q_shared::qfalse,
    uiStarted: crate::src::qcommon::q_shared::qfalse,
    cgameStarted: crate::src::qcommon::q_shared::qfalse,
    framecount: 0,
    frametime: 0,
    realtime: 0,
    realFrametime: 0,
    numlocalservers: 0,
    localServers: [crate::client_h::serverInfo_t {
        adr: crate::qcommon_h::netadr_t {
            type_0: crate::qcommon_h::NA_BAD,
            ip: [0; 4],
            ip6: [0; 16],
            port: 0,
            scope_id: 0,
        },
        hostName: [0; 32],
        mapName: [0; 32],
        game: [0; 32],
        netType: 0,
        gameType: 0,
        clients: 0,
        maxClients: 0,
        minPing: 0,
        maxPing: 0,
        ping: 0,
        visible: crate::src::qcommon::q_shared::qfalse,
        punkbuster: 0,
        g_humanplayers: 0,
        g_needpass: 0,
    }; 128],
    numglobalservers: 0,
    globalServers: [crate::client_h::serverInfo_t {
        adr: crate::qcommon_h::netadr_t {
            type_0: crate::qcommon_h::NA_BAD,
            ip: [0; 4],
            ip6: [0; 16],
            port: 0,
            scope_id: 0,
        },
        hostName: [0; 32],
        mapName: [0; 32],
        game: [0; 32],
        netType: 0,
        gameType: 0,
        clients: 0,
        maxClients: 0,
        minPing: 0,
        maxPing: 0,
        ping: 0,
        visible: crate::src::qcommon::q_shared::qfalse,
        punkbuster: 0,
        g_humanplayers: 0,
        g_needpass: 0,
    }; 4096],
    numGlobalServerAddresses: 0,
    globalServerAddresses: [crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    }; 4096],
    numfavoriteservers: 0,
    favoriteServers: [crate::client_h::serverInfo_t {
        adr: crate::qcommon_h::netadr_t {
            type_0: crate::qcommon_h::NA_BAD,
            ip: [0; 4],
            ip6: [0; 16],
            port: 0,
            scope_id: 0,
        },
        hostName: [0; 32],
        mapName: [0; 32],
        game: [0; 32],
        netType: 0,
        gameType: 0,
        clients: 0,
        maxClients: 0,
        minPing: 0,
        maxPing: 0,
        ping: 0,
        visible: crate::src::qcommon::q_shared::qfalse,
        punkbuster: 0,
        g_humanplayers: 0,
        g_needpass: 0,
    }; 128],
    pingUpdateSource: 0,
    updateServer: crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    },
    updateChallenge: [0; 1024],
    updateInfoString: [0; 1024],
    authorizeServer: crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    },
    rconAddress: crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    },
    glconfig: crate::tr_types_h::glconfig_t {
        renderer_string: [0; 1024],
        vendor_string: [0; 1024],
        version_string: [0; 1024],
        extensions_string: [0; 8192],
        maxTextureSize: 0,
        numTextureUnits: 0,
        colorBits: 0,
        depthBits: 0,
        stencilBits: 0,
        driverType: crate::tr_types_h::GLDRV_ICD,
        hardwareType: crate::tr_types_h::GLHW_GENERIC,
        deviceSupportsGamma: crate::src::qcommon::q_shared::qfalse,
        textureCompression: crate::tr_types_h::TC_NONE,
        textureEnvAddAvailable: crate::src::qcommon::q_shared::qfalse,
        vidWidth: 0,
        vidHeight: 0,
        windowAspect: 0.,
        displayFrequency: 0,
        isFullscreen: crate::src::qcommon::q_shared::qfalse,
        stereoEnabled: crate::src::qcommon::q_shared::qfalse,
        smpActive: crate::src::qcommon::q_shared::qfalse,
    },
    charSetShader: 0,
    whiteShader: 0,
    consoleShader: 0,
};
#[no_mangle]

pub static mut cgvm: *mut crate::qcommon_h::vm_t =
    0 as *const crate::qcommon_h::vm_t as *mut crate::qcommon_h::vm_t;
#[no_mangle]

pub static mut cl_reconnectArgs: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]

pub static mut cl_oldGame: [libc::c_char; 64] = [0; 64];
#[no_mangle]

pub static mut cl_oldGameSet: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
// Structure containing functions exported from refresh DLL
#[no_mangle]

pub static mut re: crate::tr_public_h::refexport_t = crate::tr_public_h::refexport_t {
    Shutdown: None,
    BeginRegistration: None,
    RegisterModel: None,
    RegisterSkin: None,
    RegisterShader: None,
    RegisterShaderNoMip: None,
    LoadWorld: None,
    SetWorldVisData: None,
    EndRegistration: None,
    ClearScene: None,
    AddRefEntityToScene: None,
    AddPolyToScene: None,
    LightForPoint: None,
    AddLightToScene: None,
    AddAdditiveLightToScene: None,
    RenderScene: None,
    SetColor: None,
    DrawStretchPic: None,
    DrawStretchRaw: None,
    UploadCinematic: None,
    BeginFrame: None,
    EndFrame: None,
    MarkFragments: None,
    LerpTag: None,
    ModelBounds: None,
    RegisterFont: None,
    RemapShader: None,
    GetEntityToken: None,
    inPVS: None,
    TakeVideoFrame: None,
};

static mut rendererLib: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]

pub static mut cl_pinglist: [crate::client_h::ping_t; 32] = [crate::client_h::ping_t {
    adr: crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    },
    start: 0,
    time: 0,
    info: [0; 1024],
}; 32];
#[no_mangle]

pub static mut cl_serverStatusList: [serverStatus_t; 16] = [serverStatus_t {
    string: [0; 8192],
    address: crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    },
    time: 0,
    startTime: 0,
    pending: crate::src::qcommon::q_shared::qfalse,
    print: crate::src::qcommon::q_shared::qfalse,
    retrieved: crate::src::qcommon::q_shared::qfalse,
}; 16];

static mut noGameRestart: libc::c_int = crate::src::qcommon::q_shared::qfalse as libc::c_int;
/*
===============
CL_CDDialog

Called by Com_Error when a cd is needed
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CDDialog() {
    cls.cddialog = crate::src::qcommon::q_shared::qtrue;
    // start it next frame
}

unsafe extern "C" fn CL_UpdateMumble() {
    let mut pos: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut scale: libc::c_float = (*cl_mumbleScale).value;
    let mut tmp: libc::c_float = 0.;
    if (*cl_useMumble).integer == 0 {
        return;
    }
    // !!! FIXME: not sure if this is even close to correct.
    crate::src::qcommon::q_math::AngleVectors(
        cl.snap.ps.viewangles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        up.as_mut_ptr(),
    );
    pos[0 as libc::c_int as usize] = cl.snap.ps.origin[0 as libc::c_int as usize] * scale;
    pos[1 as libc::c_int as usize] = cl.snap.ps.origin[2 as libc::c_int as usize] * scale;
    pos[2 as libc::c_int as usize] = cl.snap.ps.origin[1 as libc::c_int as usize] * scale;
    tmp = forward[1 as libc::c_int as usize];
    forward[1 as libc::c_int as usize] = forward[2 as libc::c_int as usize];
    forward[2 as libc::c_int as usize] = tmp;
    tmp = up[1 as libc::c_int as usize];
    up[1 as libc::c_int as usize] = up[2 as libc::c_int as usize];
    up[2 as libc::c_int as usize] = tmp;
    if (*cl_useMumble).integer > 1 as libc::c_int {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%f %f %f, %f %f %f, %f %f %f\n\x00" as *const u8 as *const libc::c_char,
            pos[0 as libc::c_int as usize] as libc::c_double,
            pos[1 as libc::c_int as usize] as libc::c_double,
            pos[2 as libc::c_int as usize] as libc::c_double,
            forward[0 as libc::c_int as usize] as libc::c_double,
            forward[1 as libc::c_int as usize] as libc::c_double,
            forward[2 as libc::c_int as usize] as libc::c_double,
            up[0 as libc::c_int as usize] as libc::c_double,
            up[1 as libc::c_int as usize] as libc::c_double,
            up[2 as libc::c_int as usize] as libc::c_double,
        );
    }
    crate::src::client::libmumblelink::mumble_update_coordinates(
        pos.as_mut_ptr(),
        forward.as_mut_ptr(),
        up.as_mut_ptr(),
    );
}

unsafe extern "C" fn CL_UpdateVoipIgnore(
    mut idstr: *const libc::c_char,
    mut ignore: crate::src::qcommon::q_shared::qboolean,
) {
    if *idstr as libc::c_int >= '0' as i32 && *idstr as libc::c_int <= '9' as i32 {
        let id: libc::c_int = atoi(idstr);
        if id >= 0 as libc::c_int && id < 64 as libc::c_int {
            clc.voipIgnore[id as usize] = ignore;
            CL_AddReliableCommand(
                crate::src::qcommon::q_shared::va(
                    b"voip %s %d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    if ignore as libc::c_uint != 0 {
                        b"ignore\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"unignore\x00" as *const u8 as *const libc::c_char
                    },
                    id,
                ),
                crate::src::qcommon::q_shared::qfalse,
            );
            crate::src::qcommon::common::Com_Printf(
                b"VoIP: %s ignoring player #%d\n\x00" as *const u8 as *const libc::c_char,
                if ignore as libc::c_uint != 0 {
                    b"Now\x00" as *const u8 as *const libc::c_char
                } else {
                    b"No longer\x00" as *const u8 as *const libc::c_char
                },
                id,
            );
            return;
        }
    }
    crate::src::qcommon::common::Com_Printf(
        b"VoIP: invalid player ID#\n\x00" as *const u8 as *const libc::c_char,
    );
}

unsafe extern "C" fn CL_UpdateVoipGain(mut idstr: *const libc::c_char, mut gain: libc::c_float) {
    if *idstr as libc::c_int >= '0' as i32 && *idstr as libc::c_int <= '9' as i32 {
        let id: libc::c_int = atoi(idstr);
        if gain < 0.0f32 {
            gain = 0.0f32
        }
        if id >= 0 as libc::c_int && id < 64 as libc::c_int {
            clc.voipGain[id as usize] = gain;
            crate::src::qcommon::common::Com_Printf(
                b"VoIP: player #%d gain now set to %f\n\x00" as *const u8 as *const libc::c_char,
                id,
                gain as libc::c_double,
            );
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn CL_Voip_f() {
    let mut cmd: *const libc::c_char = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int);
    let mut reason: *const libc::c_char = 0 as *const libc::c_char;
    if clc.state as libc::c_uint
        != crate::src::qcommon::q_shared::CA_ACTIVE as libc::c_int as libc::c_uint
    {
        reason = b"Not connected to a server\x00" as *const u8 as *const libc::c_char
    } else if clc.voipCodecInitialized as u64 == 0 {
        reason = b"Voip codec not initialized\x00" as *const u8 as *const libc::c_char
    } else if clc.voipEnabled as u64 == 0 {
        reason = b"Server doesn\'t support VoIP\x00" as *const u8 as *const libc::c_char
    } else if clc.demoplaying as u64 == 0
        && (crate::src::qcommon::cvar::Cvar_VariableValue(
            b"g_gametype\x00" as *const u8 as *const libc::c_char,
        ) == crate::bg_public_h::GT_SINGLE_PLAYER as libc::c_int as libc::c_float
            || crate::src::qcommon::cvar::Cvar_VariableValue(
                b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char,
            ) != 0.)
    {
        reason = b"running in single-player mode\x00" as *const u8 as *const libc::c_char
    }
    if !reason.is_null() {
        crate::src::qcommon::common::Com_Printf(
            b"VoIP: command ignored: %s\n\x00" as *const u8 as *const libc::c_char,
            reason,
        );
        return;
    }
    if ::libc::strcmp(cmd, b"ignore\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        CL_UpdateVoipIgnore(
            crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int),
            crate::src::qcommon::q_shared::qtrue,
        );
    } else if ::libc::strcmp(cmd, b"unignore\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        CL_UpdateVoipIgnore(
            crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int),
            crate::src::qcommon::q_shared::qfalse,
        );
    } else if ::libc::strcmp(cmd, b"gain\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if crate::src::qcommon::cmd::Cmd_Argc() > 3 as libc::c_int {
            CL_UpdateVoipGain(
                crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int),
                atof(crate::src::qcommon::cmd::Cmd_Argv(3 as libc::c_int)) as libc::c_float,
            );
        } else if crate::src::qcommon::q_shared::Q_isanumber(crate::src::qcommon::cmd::Cmd_Argv(
            2 as libc::c_int,
        )) as u64
            != 0
        {
            let mut id: libc::c_int = atoi(crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int));
            if id >= 0 as libc::c_int && id < 64 as libc::c_int {
                crate::src::qcommon::common::Com_Printf(
                    b"VoIP: current gain for player #%d is %f\n\x00" as *const u8
                        as *const libc::c_char,
                    id,
                    clc.voipGain[id as usize] as libc::c_double,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b"VoIP: invalid player ID#\n\x00" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            crate::src::qcommon::common::Com_Printf(
                b"usage: voip gain <playerID#> [value]\n\x00" as *const u8 as *const libc::c_char,
            );
        }
    } else if ::libc::strcmp(cmd, b"muteall\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        crate::src::qcommon::common::Com_Printf(
            b"VoIP: muting incoming voice\n\x00" as *const u8 as *const libc::c_char,
        );
        CL_AddReliableCommand(
            b"voip muteall\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::qfalse,
        );
        clc.voipMuteAll = crate::src::qcommon::q_shared::qtrue
    } else if ::libc::strcmp(cmd, b"unmuteall\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        crate::src::qcommon::common::Com_Printf(
            b"VoIP: unmuting incoming voice\n\x00" as *const u8 as *const libc::c_char,
        );
        CL_AddReliableCommand(
            b"voip unmuteall\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::qfalse,
        );
        clc.voipMuteAll = crate::src::qcommon::q_shared::qfalse
    } else {
        crate::src::qcommon::common::Com_Printf(b"usage: voip [un]ignore <playerID#>\n       voip [un]muteall\n       voip gain <playerID#> [value]\n\x00"
                       as *const u8 as *const libc::c_char);
    };
}

unsafe extern "C" fn CL_VoipNewGeneration() {
    // don't have a zero generation so new clients won't match, and don't
    //  wrap to negative so MSG_ReadLong() doesn't "fail."
    clc.voipOutgoingGeneration = clc.voipOutgoingGeneration.wrapping_add(1);
    if clc.voipOutgoingGeneration as libc::c_int <= 0 as libc::c_int {
        clc.voipOutgoingGeneration = 1 as libc::c_int as crate::src::qcommon::q_shared::byte
    }
    clc.voipPower = 0.0f32;
    clc.voipOutgoingSequence = 0 as libc::c_int;
    crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_ctl(
        clc.opusEncoder,
        4028 as libc::c_int,
    );
}
/*
===============
CL_VoipParseTargets

sets clc.voipTargets according to cl_voipSendTarget
Generally we don't want who's listening to change during a transmission,
so this is only called when the key is first pressed
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_VoipParseTargets() {
    let mut target: *const libc::c_char = (*cl_voipSendTarget).string;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_int = 0;
    crate::stdlib::memset(
        clc.voipTargets.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[crate::stdlib::uint8_t; 8]>() as libc::c_ulong,
    );
    clc.voipFlags =
        (clc.voipFlags as libc::c_int & !(0x1 as libc::c_int)) as crate::stdlib::uint8_t;
    while !target.is_null() {
        while *target as libc::c_int == ',' as i32 || *target as libc::c_int == ' ' as i32 {
            target = target.offset(1)
        }
        if *target == 0 {
            break;
        }
        if *(*crate::stdlib::__ctype_b_loc()).offset(*target as libc::c_int as isize) as libc::c_int
            & crate::stdlib::_ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            val = ::libc::strtol(target, &mut end, 10 as libc::c_int) as libc::c_int;
            target = end
        } else {
            if crate::src::qcommon::q_shared::Q_stricmpn(
                target,
                b"all\x00" as *const u8 as *const libc::c_char,
                3 as libc::c_int,
            ) == 0
            {
                crate::stdlib::memset(
                    clc.voipTargets.as_mut_ptr() as *mut libc::c_void,
                    !(0 as libc::c_int),
                    ::std::mem::size_of::<[crate::stdlib::uint8_t; 8]>() as libc::c_ulong,
                );
                return;
            }
            if crate::src::qcommon::q_shared::Q_stricmpn(
                target,
                b"spatial\x00" as *const u8 as *const libc::c_char,
                7 as libc::c_int,
            ) == 0
            {
                clc.voipFlags =
                    (clc.voipFlags as libc::c_int | 0x1 as libc::c_int) as crate::stdlib::uint8_t;
                target = target.offset(7 as libc::c_int as isize);
                continue;
            } else {
                if crate::src::qcommon::q_shared::Q_stricmpn(
                    target,
                    b"attacker\x00" as *const u8 as *const libc::c_char,
                    8 as libc::c_int,
                ) == 0
                {
                    val = crate::src::qcommon::vm::VM_Call(
                        cgvm,
                        crate::cg_public_h::CG_LAST_ATTACKER as libc::c_int,
                    ) as libc::c_int;
                    target = target.offset(8 as libc::c_int as isize)
                } else if crate::src::qcommon::q_shared::Q_stricmpn(
                    target,
                    b"crosshair\x00" as *const u8 as *const libc::c_char,
                    9 as libc::c_int,
                ) == 0
                {
                    val = crate::src::qcommon::vm::VM_Call(
                        cgvm,
                        crate::cg_public_h::CG_CROSSHAIR_PLAYER as libc::c_int,
                    ) as libc::c_int;
                    target = target.offset(9 as libc::c_int as isize)
                } else {
                    while *target as libc::c_int != 0
                        && *target as libc::c_int != ',' as i32
                        && *target as libc::c_int != ' ' as i32
                    {
                        target = target.offset(1)
                    }
                    continue;
                }
                if val < 0 as libc::c_int {
                    continue;
                }
            }
        }
        if val < 0 as libc::c_int || val >= 64 as libc::c_int {
            crate::src::qcommon::common::Com_Printf(
                b"^3WARNING: VoIP target %d is not a valid client number\n\x00" as *const u8
                    as *const libc::c_char,
                val,
            );
        } else {
            clc.voipTargets[(val / 8 as libc::c_int) as usize] =
                (clc.voipTargets[(val / 8 as libc::c_int) as usize] as libc::c_int
                    | (1 as libc::c_int) << val % 8 as libc::c_int)
                    as crate::stdlib::uint8_t
        }
    }
}
/*
===============
CL_CaptureVoip

Record more audio from the hardware if required and encode it into Opus
 data for later transmission.
===============
*/

unsafe extern "C" fn CL_CaptureVoip() {
    let audioMult: libc::c_float = (*cl_voipCaptureMult).value;
    let useVad: crate::src::qcommon::q_shared::qboolean =
        ((*cl_voipUseVAD).integer != 0 as libc::c_int) as libc::c_int
            as crate::src::qcommon::q_shared::qboolean;
    let mut initialFrame: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut finalFrame: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    // if we're using Mumble, don't try to handle VoIP transmission ourselves.
    if (*cl_useMumble).integer != 0 {
        return;
    }
    // If your data rate is too low, you'll get Connection Interrupted warnings
    //  when VoIP packets arrive, even if you have a broadband connection.
    //  This might work on rates lower than 25000, but for safety's sake, we'll
    //  just demand it. Who doesn't have at least a DSL line now, anyhow? If
    //  you don't, you don't need VoIP.  :)
    if (*cl_voip).modified as libc::c_uint != 0 || (*cl_rate).modified as libc::c_uint != 0 {
        if (*cl_voip).integer != 0 && (*cl_rate).integer < 25000 as libc::c_int {
            crate::src::qcommon::common::Com_Printf(
                b"^3Your network rate is too slow for VoIP.\n\x00" as *const u8
                    as *const libc::c_char,
            ); // just in case this gets called at a bad time.
            crate::src::qcommon::common::Com_Printf(
                b"Set \'Data Rate\' to \'LAN/Cable/xDSL\' in \'Setup/System/Network\'.\n\x00"
                    as *const u8 as *const libc::c_char,
            ); // packet is pending transmission, don't record more yet.
            crate::src::qcommon::common::Com_Printf(
                b"Until then, VoIP is disabled.\n\x00" as *const u8 as *const libc::c_char,
            ); // lots of things reset this.
            crate::src::qcommon::cvar::Cvar_Set(
                b"cl_voip\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            ); // basically silenced incoming audio.
        } // not connected to a server.
        crate::src::qcommon::cvar::Cvar_Set(
            b"cl_voipProtocol\x00" as *const u8 as *const libc::c_char,
            if (*cl_voip).integer != 0 {
                b"opus\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
        ); // server doesn't support VoIP.
        (*cl_voip).modified = crate::src::qcommon::q_shared::qfalse; // playing back a demo.
        (*cl_rate).modified = crate::src::qcommon::q_shared::qfalse
    } // client has VoIP support disabled.
    if clc.voipCodecInitialized as u64 == 0 {
        return;
    }
    if clc.voipOutgoingDataSize > 0 as libc::c_int {
        return;
    }
    if (*cl_voipUseVAD).modified as u64 != 0 {
        crate::src::qcommon::cvar::Cvar_Set(
            b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
            if useVad as libc::c_uint != 0 {
                b"1\x00" as *const u8 as *const libc::c_char
            } else {
                b"0\x00" as *const u8 as *const libc::c_char
            },
        );
        (*cl_voipUseVAD).modified = crate::src::qcommon::q_shared::qfalse
    }
    if useVad as libc::c_uint != 0 && (*cl_voipSend).integer == 0 {
        crate::src::qcommon::cvar::Cvar_Set(
            b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*cl_voipSend).modified as u64 != 0 {
        let mut dontCapture: crate::src::qcommon::q_shared::qboolean =
            crate::src::qcommon::q_shared::qfalse;
        if clc.state as libc::c_uint
            != crate::src::qcommon::q_shared::CA_ACTIVE as libc::c_int as libc::c_uint
        {
            dontCapture = crate::src::qcommon::q_shared::qtrue
        } else if clc.voipEnabled as u64 == 0 {
            dontCapture = crate::src::qcommon::q_shared::qtrue
        } else if clc.demoplaying as u64 != 0 {
            dontCapture = crate::src::qcommon::q_shared::qtrue
        } else if (*cl_voip).integer == 0 as libc::c_int {
            dontCapture = crate::src::qcommon::q_shared::qtrue
        } else if audioMult == 0.0f32 {
            dontCapture = crate::src::qcommon::q_shared::qtrue
        }
        (*cl_voipSend).modified = crate::src::qcommon::q_shared::qfalse;
        if dontCapture as u64 != 0 {
            crate::src::qcommon::cvar::Cvar_Set(
                b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        if (*cl_voipSend).integer != 0 {
            initialFrame = crate::src::qcommon::q_shared::qtrue
        } else {
            finalFrame = crate::src::qcommon::q_shared::qtrue
        }
    }
    // try to get more audio data from the sound card...
    if initialFrame as u64 != 0 {
        crate::src::client::snd_main::S_MasterGain(crate::src::qcommon::q_shared::Com_Clamp(
            0.0f32,
            1.0f32,
            (*cl_voipGainDuringCapture).value,
        ));
        crate::src::client::snd_main::S_StartCapture();
        CL_VoipNewGeneration();
        CL_VoipParseTargets();
    }
    if (*cl_voipSend).integer != 0 || finalFrame as libc::c_uint != 0 {
        // user wants to capture audio?
        let mut samples: libc::c_int = crate::src::client::snd_main::S_AvailableCaptureSamples();
        let packetSamples: libc::c_int = if finalFrame as libc::c_uint != 0 {
            (20 as libc::c_int) * 48 as libc::c_int
        } else {
            (20 as libc::c_int * 48 as libc::c_int) * 3 as libc::c_int
        };
        // enough data buffered in audio hardware to process yet?
        if samples >= packetSamples {
            // audio capture is always MONO16.
            static mut sampbuffer: [crate::stdlib::int16_t; 2880] = [0; 2880];
            let mut voipPower: libc::c_float = 0.0f32;
            let mut voipFrames: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            let mut bytes: libc::c_int = 0;
            if samples > 20 as libc::c_int * 48 as libc::c_int * 3 as libc::c_int {
                samples = 20 as libc::c_int * 48 as libc::c_int * 3 as libc::c_int
            }
            // !!! FIXME: maybe separate recording from encoding, so voipPower
            // !!! FIXME:  updates faster than 4Hz?
            samples -= samples % (20 as libc::c_int * 48 as libc::c_int); // grab from audio card.
            if samples != 120 as libc::c_int
                && samples != 240 as libc::c_int
                && samples != 480 as libc::c_int
                && samples != 960 as libc::c_int
                && samples != 1920 as libc::c_int
                && samples != 2880 as libc::c_int
            {
                crate::src::qcommon::common::Com_Printf(
                    b"Voip: bad number of samples %d\n\x00" as *const u8 as *const libc::c_char,
                    samples,
                );
                return;
            }
            voipFrames = samples / (20 as libc::c_int * 48 as libc::c_int);
            crate::src::client::snd_main::S_Capture(
                samples,
                sampbuffer.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
            );
            // check the "power" of this packet...
            i = 0 as libc::c_int;
            while i < samples {
                let flsamp: libc::c_float = sampbuffer[i as usize] as libc::c_float;
                let s: libc::c_float =
                    crate::stdlib::fabs(flsamp as libc::c_double) as libc::c_float;
                voipPower += s * s;
                sampbuffer[i as usize] = (flsamp * audioMult) as crate::stdlib::int16_t;
                i += 1
            }
            // encode raw audio samples into Opus data...
            bytes = crate::src::opus_1_2_1::src::opus_encoder::opus_encode(
                clc.opusEncoder,
                sampbuffer.as_mut_ptr(),
                samples,
                clc.voipOutgoingData.as_mut_ptr() as *mut libc::c_uchar,
                ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 1024]>()
                    as libc::c_ulong as crate::opus_types_h::opus_int32,
            );
            if bytes <= 0 as libc::c_int {
                crate::src::qcommon::common::Com_DPrintf(
                    b"VoIP: Error encoding %d samples\n\x00" as *const u8 as *const libc::c_char,
                    samples,
                );
                bytes = 0 as libc::c_int
            }
            clc.voipPower =
                voipPower / (32768.0f32 * 32768.0f32 * samples as libc::c_float) * 100.0f32;
            if useVad as libc::c_uint != 0 && clc.voipPower < (*cl_voipVADThreshold).value {
                CL_VoipNewGeneration();
            // no "talk" for at least 1/4 second.
            } else {
                clc.voipOutgoingDataSize = bytes;
                clc.voipOutgoingDataFrames = voipFrames;
                crate::src::qcommon::common::Com_DPrintf(
                    b"VoIP: Send %d frames, %d bytes, %f power\n\x00" as *const u8
                        as *const libc::c_char,
                    voipFrames,
                    bytes,
                    clc.voipPower as libc::c_double,
                );
            }
        }
    }
    // User requested we stop recording, and we've now processed the last of
    //  any previously-buffered data. Pause the capture device, etc.
    if finalFrame as u64 != 0 {
        crate::src::client::snd_main::S_StopCapture();
        crate::src::client::snd_main::S_MasterGain(1.0f32);
        clc.voipPower = 0.0f32
        // force this value so it doesn't linger.
    };
}
/*
=======================================================================

CLIENT RELIABLE COMMAND COMMUNICATION

=======================================================================
*/
/*
======================
CL_AddReliableCommand

The given command will be transmitted to the server, and is guaranteed to
not have future usercmd_t executed before it is executed
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_AddReliableCommand(
    mut cmd: *const libc::c_char,
    mut isDisconnectCmd: crate::src::qcommon::q_shared::qboolean,
) {
    let mut unacknowledged: libc::c_int = clc.reliableSequence - clc.reliableAcknowledge;
    // if we would be losing an old command that hasn't been acknowledged,
    // we must drop the connection
    // also leave one slot open for the disconnect command in this case.
    if isDisconnectCmd as libc::c_uint != 0 && unacknowledged > 64 as libc::c_int
        || isDisconnectCmd as u64 == 0 && unacknowledged >= 64 as libc::c_int
    {
        if crate::src::qcommon::common::com_errorEntered as u64 != 0 {
            return;
        } else {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"Client command overflow\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    clc.reliableSequence += 1;
    crate::src::qcommon::q_shared::Q_strncpyz(
        clc.reliableCommands
            [(clc.reliableSequence & 64 as libc::c_int - 1 as libc::c_int) as usize]
            .as_mut_ptr(),
        cmd,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
}
//
// cl_main.c
//
/*
=======================================================================

CLIENT SIDE DEMO RECORDING

=======================================================================
*/
/*
====================
CL_WriteDemoMessage

Dumps the current net message, prefixed by the length
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_WriteDemoMessage(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut headerBytes: libc::c_int,
) {
    let mut len: libc::c_int = 0;
    let mut swlen: libc::c_int = 0;
    // write the packet sequence
    len = clc.serverMessageSequence;
    swlen = len;
    crate::src::qcommon::files::FS_Write(
        &mut swlen as *mut libc::c_int as *const libc::c_void,
        4 as libc::c_int,
        clc.demofile,
    );
    // skip the packet sequencing information
    len = (*msg).cursize - headerBytes;
    swlen = len;
    crate::src::qcommon::files::FS_Write(
        &mut swlen as *mut libc::c_int as *const libc::c_void,
        4 as libc::c_int,
        clc.demofile,
    );
    crate::src::qcommon::files::FS_Write(
        (*msg).data.offset(headerBytes as isize) as *const libc::c_void,
        len,
        clc.demofile,
    );
}
/*
====================
CL_StopRecording_f

stop recording a demo
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_StopRecord_f() {
    let mut len: libc::c_int = 0;
    if clc.demorecording as u64 == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"Not recording a demo.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    // finish up
    len = -(1 as libc::c_int);
    crate::src::qcommon::files::FS_Write(
        &mut len as *mut libc::c_int as *const libc::c_void,
        4 as libc::c_int,
        clc.demofile,
    );
    crate::src::qcommon::files::FS_Write(
        &mut len as *mut libc::c_int as *const libc::c_void,
        4 as libc::c_int,
        clc.demofile,
    );
    crate::src::qcommon::files::FS_FCloseFile(clc.demofile);
    clc.demofile = 0 as libc::c_int;
    clc.demorecording = crate::src::qcommon::q_shared::qfalse;
    clc.spDemoRecording = crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::common::Com_Printf(
        b"Stopped demo.\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
==================
CL_DemoFilename
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_DemoFilename(
    mut number: libc::c_int,
    mut fileName: *mut libc::c_char,
    mut fileNameSize: libc::c_int,
) {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    if number < 0 as libc::c_int || number > 9999 as libc::c_int {
        number = 9999 as libc::c_int
    }
    a = number / 1000 as libc::c_int;
    number -= a * 1000 as libc::c_int;
    b = number / 100 as libc::c_int;
    number -= b * 100 as libc::c_int;
    c = number / 10 as libc::c_int;
    number -= c * 10 as libc::c_int;
    d = number;
    crate::src::qcommon::q_shared::Com_sprintf(
        fileName,
        fileNameSize,
        b"demo%i%i%i%i\x00" as *const u8 as *const libc::c_char,
        a,
        b,
        c,
        d,
    );
}
/*
====================
CL_Record_f

record <demoname>

Begins recording a demo from the current position
====================
*/

static mut demoName: [libc::c_char; 64] = [0; 64];
// compiler bug workaround
#[no_mangle]

pub unsafe extern "C" fn CL_Record_f() {
    let mut name: [libc::c_char; 4096] = [0; 4096];
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
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ent: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
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
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if crate::src::qcommon::cmd::Cmd_Argc() > 2 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"record <demoname>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if clc.demorecording as u64 != 0 {
        if clc.spDemoRecording as u64 == 0 {
            crate::src::qcommon::common::Com_Printf(
                b"Already recording.\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        return;
    }
    if clc.state as libc::c_uint
        != crate::src::qcommon::q_shared::CA_ACTIVE as libc::c_int as libc::c_uint
    {
        crate::src::qcommon::common::Com_Printf(
            b"You must be in a level to record.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    // sync 0 doesn't prevent recording, so not forcing it off .. everyone does g_sync 1 ; record ; g_sync 0 ..
    if crate::src::qcommon::net_ip::NET_IsLocalAddress(
        clc.serverAddress as crate::qcommon_h::netadr_t,
    ) as libc::c_uint
        != 0
        && crate::src::qcommon::cvar::Cvar_VariableValue(
            b"g_synchronousClients\x00" as *const u8 as *const libc::c_char,
        ) == 0.
    {
        crate::src::qcommon::common::Com_Printf(b"^3WARNING: You should set \'g_synchronousClients 1\' for smoother demo recording\n\x00"
                       as *const u8 as *const libc::c_char);
    }
    if crate::src::qcommon::cmd::Cmd_Argc() == 2 as libc::c_int {
        s = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int);
        crate::src::qcommon::q_shared::Q_strncpyz(
            demoName.as_mut_ptr(),
            s,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        if clc.compat as u64 != 0 {
            crate::src::qcommon::q_shared::Com_sprintf(
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
                b"demos/%s.%s%d\x00" as *const u8 as *const libc::c_char,
                demoName.as_mut_ptr(),
                b"dm_\x00" as *const u8 as *const libc::c_char,
                (*crate::src::qcommon::common::com_legacyprotocol).integer,
            );
        } else {
            crate::src::qcommon::q_shared::Com_sprintf(
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
                b"demos/%s.%s%d\x00" as *const u8 as *const libc::c_char,
                demoName.as_mut_ptr(),
                b"dm_\x00" as *const u8 as *const libc::c_char,
                (*crate::src::qcommon::common::com_protocol).integer,
            );
        }
    } else {
        let mut number: libc::c_int = 0;
        // scan for a free demo name
        number = 0 as libc::c_int;
        while number <= 9999 as libc::c_int {
            CL_DemoFilename(
                number,
                demoName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            if clc.compat as u64 != 0 {
                crate::src::qcommon::q_shared::Com_sprintf(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
                    b"demos/%s.%s%d\x00" as *const u8 as *const libc::c_char,
                    demoName.as_mut_ptr(),
                    b"dm_\x00" as *const u8 as *const libc::c_char,
                    (*crate::src::qcommon::common::com_legacyprotocol).integer,
                );
            } else {
                crate::src::qcommon::q_shared::Com_sprintf(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
                    b"demos/%s.%s%d\x00" as *const u8 as *const libc::c_char,
                    demoName.as_mut_ptr(),
                    b"dm_\x00" as *const u8 as *const libc::c_char,
                    (*crate::src::qcommon::common::com_protocol).integer,
                );
            }
            if crate::src::qcommon::files::FS_FileExists(name.as_mut_ptr()) as u64 == 0 {
                break;
            }
            number += 1
            // file doesn't exist
        }
    }
    // open the demo file
    crate::src::qcommon::common::Com_Printf(
        b"recording to %s.\n\x00" as *const u8 as *const libc::c_char,
        name.as_mut_ptr(),
    );
    clc.demofile = crate::src::qcommon::files::FS_FOpenFileWrite(name.as_mut_ptr());
    if clc.demofile == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"ERROR: couldn\'t open.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    clc.demorecording = crate::src::qcommon::q_shared::qtrue;
    if crate::src::qcommon::cvar::Cvar_VariableValue(
        b"ui_recordSPDemo\x00" as *const u8 as *const libc::c_char,
    ) != 0.
    {
        clc.spDemoRecording = crate::src::qcommon::q_shared::qtrue
    } else {
        clc.spDemoRecording = crate::src::qcommon::q_shared::qfalse
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        clc.demoName.as_mut_ptr(),
        demoName.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    // don't start saving messages until a non-delta compressed message is received
    clc.demowaiting = crate::src::qcommon::q_shared::qtrue;
    // write out the gamestate message
    crate::src::qcommon::msg::MSG_Init(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        bufData.as_mut_ptr(),
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16384]>() as libc::c_ulong
            as libc::c_int,
    );
    crate::src::qcommon::msg::MSG_Bitstream(&mut buf as *mut _ as *mut crate::qcommon_h::msg_t);
    // NOTE, MRE: all server->client messages now acknowledge
    crate::src::qcommon::msg::MSG_WriteLong(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        clc.reliableSequence,
    );
    crate::src::qcommon::msg::MSG_WriteByte(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        crate::qcommon_h::svc_gamestate as libc::c_int,
    );
    crate::src::qcommon::msg::MSG_WriteLong(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        clc.serverCommandSequence,
    );
    // configstrings
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if !(cl.gameState.stringOffsets[i as usize] == 0) {
            s = cl
                .gameState
                .stringData
                .as_mut_ptr()
                .offset(cl.gameState.stringOffsets[i as usize] as isize);
            crate::src::qcommon::msg::MSG_WriteByte(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                crate::qcommon_h::svc_configstring as libc::c_int,
            );
            crate::src::qcommon::msg::MSG_WriteShort(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                i,
            );
            crate::src::qcommon::msg::MSG_WriteBigString(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                s,
            );
        }
        i += 1
    }
    // baselines
    crate::stdlib::memset(
        &mut nullstate as *mut crate::src::qcommon::q_shared::entityState_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 10 as libc::c_int {
        ent = &mut *cl.entityBaselines.as_mut_ptr().offset(i as isize)
            as *mut crate::src::qcommon::q_shared::entityState_t;
        if !((*ent).number == 0) {
            crate::src::qcommon::msg::MSG_WriteByte(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                crate::qcommon_h::svc_baseline as libc::c_int,
            );
            crate::src::qcommon::msg::MSG_WriteDeltaEntity(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                &mut nullstate as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
                ent as *mut crate::src::qcommon::q_shared::entityState_s,
                crate::src::qcommon::q_shared::qtrue,
            );
        }
        i += 1
    }
    crate::src::qcommon::msg::MSG_WriteByte(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        crate::qcommon_h::svc_EOF as libc::c_int,
    );
    // finished writing the gamestate stuff
    // write the client num
    crate::src::qcommon::msg::MSG_WriteLong(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        clc.clientNum,
    );
    // write the checksum feed
    crate::src::qcommon::msg::MSG_WriteLong(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        clc.checksumFeed,
    );
    // finished writing the client packet
    crate::src::qcommon::msg::MSG_WriteByte(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        crate::qcommon_h::svc_EOF as libc::c_int,
    );
    // write it to the demo file
    len = clc.serverMessageSequence - 1 as libc::c_int;
    crate::src::qcommon::files::FS_Write(
        &mut len as *mut libc::c_int as *const libc::c_void,
        4 as libc::c_int,
        clc.demofile,
    );
    len = buf.cursize;
    crate::src::qcommon::files::FS_Write(
        &mut len as *mut libc::c_int as *const libc::c_void,
        4 as libc::c_int,
        clc.demofile,
    );
    crate::src::qcommon::files::FS_Write(
        buf.data as *const libc::c_void,
        buf.cursize,
        clc.demofile,
    );
    // the rest of the demo file will be copied from net messages
}
/*
=======================================================================

CLIENT SIDE DEMO PLAYBACK

=======================================================================
*/
/*
=================
CL_DemoFrameDurationSDev
=================
*/

unsafe extern "C" fn CL_DemoFrameDurationSDev() -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut numFrames: libc::c_int = 0;
    let mut mean: libc::c_float = 0.0f32;
    let mut variance: libc::c_float = 0.0f32;
    if clc.timeDemoFrames - 1 as libc::c_int > 4096 as libc::c_int {
        numFrames = 4096 as libc::c_int
    } else {
        numFrames = clc.timeDemoFrames - 1 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < numFrames {
        mean += clc.timeDemoDurations[i as usize] as libc::c_int as libc::c_float;
        i += 1
    }
    mean /= numFrames as libc::c_float;
    i = 0 as libc::c_int;
    while i < numFrames {
        let mut x: libc::c_float = clc.timeDemoDurations[i as usize] as libc::c_float;
        variance += (x - mean) * (x - mean);
        i += 1
    }
    variance /= numFrames as libc::c_float;
    return crate::stdlib::sqrt(variance as libc::c_double) as libc::c_float;
}
/*
=================
CL_DemoCompleted
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_DemoCompleted() {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    if !cl_timedemo.is_null() && (*cl_timedemo).integer != 0 {
        let mut time: libc::c_int = 0;
        time = crate::src::sys::sys_unix::Sys_Milliseconds() - clc.timeDemoStart;
        if time > 0 as libc::c_int {
            // Millisecond times are frame durations:
            // minimum/average/maximum/std deviation
            crate::src::qcommon::q_shared::Com_sprintf(
                buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b"%i frames %3.1f seconds %3.1f fps %d.0/%.1f/%d.0/%.1f ms\n\x00" as *const u8
                    as *const libc::c_char,
                clc.timeDemoFrames,
                time as libc::c_double / 1000.0f64,
                clc.timeDemoFrames as libc::c_double * 1000.0f64 / time as libc::c_double,
                clc.timeDemoMinDuration,
                (time as libc::c_float / clc.timeDemoFrames as libc::c_float) as libc::c_double,
                clc.timeDemoMaxDuration,
                CL_DemoFrameDurationSDev() as libc::c_double,
            );
            crate::src::qcommon::common::Com_Printf(
                b"%s\x00" as *const u8 as *const libc::c_char,
                buffer.as_mut_ptr(),
            );
            // Write a log of all the frame durations
            if !cl_timedemoLog.is_null()
                && crate::stdlib::strlen((*cl_timedemoLog).string)
                    > 0 as libc::c_int as libc::c_ulong
            {
                let mut i: libc::c_int = 0;
                let mut numFrames: libc::c_int = 0;
                let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
                if clc.timeDemoFrames - 1 as libc::c_int > 4096 as libc::c_int {
                    numFrames = 4096 as libc::c_int
                } else {
                    numFrames = clc.timeDemoFrames - 1 as libc::c_int
                }
                f = crate::src::qcommon::files::FS_FOpenFileWrite((*cl_timedemoLog).string);
                if f != 0 {
                    crate::src::qcommon::files::FS_Printf(
                        f,
                        b"# %s\x00" as *const u8 as *const libc::c_char,
                        buffer.as_mut_ptr(),
                    );
                    i = 0 as libc::c_int;
                    while i < numFrames {
                        crate::src::qcommon::files::FS_Printf(
                            f,
                            b"%d\n\x00" as *const u8 as *const libc::c_char,
                            clc.timeDemoDurations[i as usize] as libc::c_int,
                        );
                        i += 1
                    }
                    crate::src::qcommon::files::FS_FCloseFile(f);
                    crate::src::qcommon::common::Com_Printf(
                        b"%s written\n\x00" as *const u8 as *const libc::c_char,
                        (*cl_timedemoLog).string,
                    );
                } else {
                    crate::src::qcommon::common::Com_Printf(
                        b"Couldn\'t open %s for writing\n\x00" as *const u8 as *const libc::c_char,
                        (*cl_timedemoLog).string,
                    );
                }
            }
        }
    }
    CL_Disconnect(crate::src::qcommon::q_shared::qtrue);
    CL_NextDemo();
}
/*
=================
CL_ReadDemoMessage
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ReadDemoMessage() {
    let mut r: libc::c_int = 0;
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
    let mut bufData: [crate::src::qcommon::q_shared::byte; 16384] = [0; 16384];
    let mut s: libc::c_int = 0;
    if clc.demofile == 0 {
        CL_DemoCompleted();
        return;
    }
    // get the sequence number
    r = crate::src::qcommon::files::FS_Read(
        &mut s as *mut libc::c_int as *mut libc::c_void,
        4 as libc::c_int,
        clc.demofile,
    );
    if r != 4 as libc::c_int {
        CL_DemoCompleted();
        return;
    }
    clc.serverMessageSequence = s;
    // init the message
    crate::src::qcommon::msg::MSG_Init(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        bufData.as_mut_ptr(),
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16384]>() as libc::c_ulong
            as libc::c_int,
    );
    // get the length
    r = crate::src::qcommon::files::FS_Read(
        &mut buf.cursize as *mut libc::c_int as *mut libc::c_void,
        4 as libc::c_int,
        clc.demofile,
    );
    if r != 4 as libc::c_int {
        CL_DemoCompleted();
        return;
    }
    buf.cursize = buf.cursize;
    if buf.cursize == -(1 as libc::c_int) {
        CL_DemoCompleted();
        return;
    }
    if buf.cursize > buf.maxsize {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CL_ReadDemoMessage: demoMsglen > MAX_MSGLEN\x00" as *const u8 as *const libc::c_char,
        );
    }
    r = crate::src::qcommon::files::FS_Read(
        buf.data as *mut libc::c_void,
        buf.cursize,
        clc.demofile,
    );
    if r != buf.cursize {
        crate::src::qcommon::common::Com_Printf(
            b"Demo file was truncated.\n\x00" as *const u8 as *const libc::c_char,
        );
        CL_DemoCompleted();
        return;
    }
    clc.lastPacketTime = cls.realtime;
    buf.readcount = 0 as libc::c_int;
    crate::src::client::cl_parse::CL_ParseServerMessage(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
    );
}
/*
====================
CL_WalkDemoExt
====================
*/

unsafe extern "C" fn CL_WalkDemoExt(
    mut arg: *mut libc::c_char,
    mut name: *mut libc::c_char,
    mut demofile: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    *demofile = 0 as libc::c_int;
    if (*crate::src::qcommon::common::com_legacyprotocol).integer > 0 as libc::c_int {
        crate::src::qcommon::q_shared::Com_sprintf(
            name,
            4096 as libc::c_int,
            b"demos/%s.%s%d\x00" as *const u8 as *const libc::c_char,
            arg,
            b"dm_\x00" as *const u8 as *const libc::c_char,
            (*crate::src::qcommon::common::com_legacyprotocol).integer,
        );
        crate::src::qcommon::files::FS_FOpenFileRead(
            name,
            demofile,
            crate::src::qcommon::q_shared::qtrue,
        );
        if *demofile != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"Demo file: %s\n\x00" as *const u8 as *const libc::c_char,
                name,
            );
            return (*crate::src::qcommon::common::com_legacyprotocol).integer;
        }
    }
    if (*crate::src::qcommon::common::com_protocol).integer
        != (*crate::src::qcommon::common::com_legacyprotocol).integer
    {
        crate::src::qcommon::q_shared::Com_sprintf(
            name,
            4096 as libc::c_int,
            b"demos/%s.%s%d\x00" as *const u8 as *const libc::c_char,
            arg,
            b"dm_\x00" as *const u8 as *const libc::c_char,
            (*crate::src::qcommon::common::com_protocol).integer,
        );
        crate::src::qcommon::files::FS_FOpenFileRead(
            name,
            demofile,
            crate::src::qcommon::q_shared::qtrue,
        );
        if *demofile != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"Demo file: %s\n\x00" as *const u8 as *const libc::c_char,
                name,
            );
            return (*crate::src::qcommon::common::com_protocol).integer;
        }
    }
    crate::src::qcommon::common::Com_Printf(
        b"Not found: %s\n\x00" as *const u8 as *const libc::c_char,
        name,
    );
    while *crate::src::qcommon::common::demo_protocols
        .as_mut_ptr()
        .offset(i as isize)
        != 0
    {
        if *crate::src::qcommon::common::demo_protocols
            .as_mut_ptr()
            .offset(i as isize)
            == (*crate::src::qcommon::common::com_legacyprotocol).integer
        {
            continue;
        }
        if *crate::src::qcommon::common::demo_protocols
            .as_mut_ptr()
            .offset(i as isize)
            == (*crate::src::qcommon::common::com_protocol).integer
        {
            continue;
        }
        crate::src::qcommon::q_shared::Com_sprintf(
            name,
            4096 as libc::c_int,
            b"demos/%s.%s%d\x00" as *const u8 as *const libc::c_char,
            arg,
            b"dm_\x00" as *const u8 as *const libc::c_char,
            *crate::src::qcommon::common::demo_protocols
                .as_mut_ptr()
                .offset(i as isize),
        );
        crate::src::qcommon::files::FS_FOpenFileRead(
            name,
            demofile,
            crate::src::qcommon::q_shared::qtrue,
        );
        if *demofile != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"Demo file: %s\n\x00" as *const u8 as *const libc::c_char,
                name,
            );
            return *crate::src::qcommon::common::demo_protocols
                .as_mut_ptr()
                .offset(i as isize);
        } else {
            crate::src::qcommon::common::Com_Printf(
                b"Not found: %s\n\x00" as *const u8 as *const libc::c_char,
                name,
            );
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
/*
====================
CL_CompleteDemoName
====================
*/

unsafe extern "C" fn CL_CompleteDemoName(mut _args: *mut libc::c_char, mut argNum: libc::c_int) {
    if argNum == 2 as libc::c_int {
        let mut demoExt: [libc::c_char; 16] = [0; 16];
        crate::src::qcommon::q_shared::Com_sprintf(
            demoExt.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
            b".%s%d\x00" as *const u8 as *const libc::c_char,
            b"dm_\x00" as *const u8 as *const libc::c_char,
            (*crate::src::qcommon::common::com_protocol).integer,
        );
        crate::src::qcommon::common::Field_CompleteFilename(
            b"demos\x00" as *const u8 as *const libc::c_char,
            demoExt.as_mut_ptr(),
            crate::src::qcommon::q_shared::qtrue,
            crate::src::qcommon::q_shared::qtrue,
        );
    };
}
/*
====================
CL_PlayDemo_f

demo <demoname>

====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_PlayDemo_f() {
    let mut name: [libc::c_char; 4096] = [0; 4096];
    let mut arg: [libc::c_char; 4096] = [0; 4096];
    let mut ext_test: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut protocol: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut retry: [libc::c_char; 4096] = [0; 4096];
    if crate::src::qcommon::cmd::Cmd_Argc() != 2 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"demo <demoname>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    // make sure a local server is killed
    // 2 means don't force disconnect of local client
    crate::src::qcommon::cvar::Cvar_Set(
        b"sv_killserver\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
    );
    // open the demo file
    crate::src::qcommon::q_shared::Q_strncpyz(
        arg.as_mut_ptr(),
        crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
    );
    CL_Disconnect(crate::src::qcommon::q_shared::qtrue);
    // check for an extension .DEMOEXT_?? (?? is protocol)
    ext_test = ::libc::strrchr(arg.as_mut_ptr(), '.' as i32);
    if !ext_test.is_null()
        && crate::src::qcommon::q_shared::Q_stricmpn(
            ext_test.offset(1 as libc::c_int as isize),
            b"dm_\x00" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) == 0
    {
        protocol = atoi(
            ext_test.offset(
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as isize,
            ),
        );
        i = 0 as libc::c_int;
        while *crate::src::qcommon::common::demo_protocols
            .as_mut_ptr()
            .offset(i as isize)
            != 0
        {
            if *crate::src::qcommon::common::demo_protocols
                .as_mut_ptr()
                .offset(i as isize)
                == protocol
            {
                break;
            }
            i += 1
        }
        if *crate::src::qcommon::common::demo_protocols
            .as_mut_ptr()
            .offset(i as isize)
            != 0
            || protocol == (*crate::src::qcommon::common::com_protocol).integer
            || protocol == (*crate::src::qcommon::common::com_legacyprotocol).integer
        {
            crate::src::qcommon::q_shared::Com_sprintf(
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
                b"demos/%s\x00" as *const u8 as *const libc::c_char,
                arg.as_mut_ptr(),
            );
            crate::src::qcommon::files::FS_FOpenFileRead(
                name.as_mut_ptr(),
                &mut clc.demofile,
                crate::src::qcommon::q_shared::qtrue,
            );
        } else {
            let mut len: libc::c_int = 0;
            crate::src::qcommon::common::Com_Printf(
                b"Protocol %d not supported for demos\n\x00" as *const u8 as *const libc::c_char,
                protocol,
            );
            len = ext_test.offset_from(arg.as_mut_ptr()) as libc::c_long as libc::c_int;
            if len as libc::c_ulong
                >= (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            {
                len = (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int
            }
            crate::src::qcommon::q_shared::Q_strncpyz(
                retry.as_mut_ptr(),
                arg.as_mut_ptr(),
                len + 1 as libc::c_int,
            );
            retry[len as usize] = '\u{0}' as i32 as libc::c_char;
            protocol = CL_WalkDemoExt(retry.as_mut_ptr(), name.as_mut_ptr(), &mut clc.demofile)
        }
    } else {
        protocol = CL_WalkDemoExt(arg.as_mut_ptr(), name.as_mut_ptr(), &mut clc.demofile)
    }
    if clc.demofile == 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"couldn\'t open %s\x00" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
        );
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        clc.demoName.as_mut_ptr(),
        arg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::client::cl_console::Con_Close();
    clc.state = crate::src::qcommon::q_shared::CA_CONNECTED;
    clc.demoplaying = crate::src::qcommon::q_shared::qtrue;
    crate::src::qcommon::q_shared::Q_strncpyz(
        clc.servername.as_mut_ptr(),
        arg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
    );
    if protocol <= (*crate::src::qcommon::common::com_legacyprotocol).integer {
        clc.compat = crate::src::qcommon::q_shared::qtrue
    } else {
        clc.compat = crate::src::qcommon::q_shared::qfalse
    }
    // read demo messages until connected
    while clc.state as libc::c_uint
        >= crate::src::qcommon::q_shared::CA_CONNECTED as libc::c_int as libc::c_uint
        && (clc.state as libc::c_uint)
            < crate::src::qcommon::q_shared::CA_PRIMED as libc::c_int as libc::c_uint
    {
        CL_ReadDemoMessage();
    }
    // don't get the first snapshot this frame, to prevent the long
    // time from the gamestate load from messing causing a time skip
    clc.firstDemoFrameSkipped = crate::src::qcommon::q_shared::qfalse;
}
/*
====================
CL_StartDemoLoop

Closing the main menu will restart the demo loop
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_StartDemoLoop() {
    // start the demo loop again
    crate::src::qcommon::cmd::Cbuf_AddText(b"d1\n\x00" as *const u8 as *const libc::c_char);
    crate::src::client::cl_keys::Key_SetCatcher(0 as libc::c_int);
}
/*
==================
CL_NextDemo

Called when a demo or cinematic finishes
If the "nextdemo" cvar is set, that command will be issued
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_NextDemo() {
    let mut v: [libc::c_char; 1024] = [0; 1024];
    crate::src::qcommon::q_shared::Q_strncpyz(
        v.as_mut_ptr(),
        crate::src::qcommon::cvar::Cvar_VariableString(
            b"nextdemo\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    v[(1024 as libc::c_int - 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    crate::src::qcommon::common::Com_DPrintf(
        b"CL_NextDemo: %s\n\x00" as *const u8 as *const libc::c_char,
        v.as_mut_ptr(),
    );
    if v[0 as libc::c_int as usize] == 0 {
        return;
    }
    crate::src::qcommon::cvar::Cvar_Set(
        b"nextdemo\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cbuf_AddText(v.as_mut_ptr());
    crate::src::qcommon::cmd::Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cbuf_Execute();
}
//======================================================================
/*
=====================
CL_ShutdownAll
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ShutdownAll(mut shutdownRef: crate::src::qcommon::q_shared::qboolean) {
    if crate::src::client::cl_avi::CL_VideoRecording() as u64 != 0 {
        crate::src::client::cl_avi::CL_CloseAVI();
    }
    if clc.demorecording as u64 != 0 {
        CL_StopRecord_f();
    }
    crate::src::client::cl_curl::CL_cURL_Shutdown();
    // clear sounds
    crate::src::client::snd_main::S_DisableSounds();
    // shutdown CGame
    crate::src::client::cl_cgame::CL_ShutdownCGame();
    // shutdown UI
    crate::src::client::cl_ui::CL_ShutdownUI();
    // shutdown the renderer
    if shutdownRef as u64 != 0 {
        CL_ShutdownRef(); // don't destroy window or context
    } else if re.Shutdown.is_some() {
        re.Shutdown.expect("non-null function pointer")(crate::src::qcommon::q_shared::qfalse);
    }
    cls.uiStarted = crate::src::qcommon::q_shared::qfalse;
    cls.cgameStarted = crate::src::qcommon::q_shared::qfalse;
    cls.rendererStarted = crate::src::qcommon::q_shared::qfalse;
    cls.soundRegistered = crate::src::qcommon::q_shared::qfalse;
}
/*
=================
CL_ClearMemory

Called by Com_GameRestart
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ClearMemory(mut shutdownRef: crate::src::qcommon::q_shared::qboolean) {
    // shutdown all the client stuff
    CL_ShutdownAll(shutdownRef);
    // if not running a server clear the whole hunk
    if crate::src::qcommon::common::com_sv_running.is_null()
        || (*crate::src::qcommon::common::com_sv_running).integer == 0
    {
        // clear the whole hunk
        crate::src::qcommon::common::Hunk_Clear();
        // clear collision map data
        crate::src::qcommon::cm_load::CM_ClearMap();
    } else {
        // clear all the client data on the hunk
        crate::src::qcommon::common::Hunk_ClearToMark();
    };
}
/*
=================
CL_FlushMemory

Called by CL_MapLoading, CL_Connect_f, CL_PlayDemo_f, and CL_ParseGamestate the only
ways a client gets into a game
Also called by Com_Error
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_FlushMemory() {
    CL_ClearMemory(crate::src::qcommon::q_shared::qfalse);
    CL_StartHunkUsers(crate::src::qcommon::q_shared::qfalse);
}
/*
=====================
CL_MapLoading

A local server is starting to load a map, so update the
screen to let the user know about it, then dump all client
memory on the hunk from cgame, ui, and renderer
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_MapLoading() {
    if (*crate::src::qcommon::common::com_dedicated).integer != 0 {
        clc.state = crate::src::qcommon::q_shared::CA_DISCONNECTED;
        crate::src::client::cl_keys::Key_SetCatcher(0x1 as libc::c_int);
        return;
    }
    if (*crate::src::qcommon::common::com_cl_running).integer == 0 {
        return;
    }
    crate::src::client::cl_console::Con_Close();
    crate::src::client::cl_keys::Key_SetCatcher(0 as libc::c_int);
    // if we are already connected to the local host, stay connected
    if clc.state as libc::c_uint
        >= crate::src::qcommon::q_shared::CA_CONNECTED as libc::c_int as libc::c_uint
        && crate::src::qcommon::q_shared::Q_stricmp(
            clc.servername.as_mut_ptr(),
            b"localhost\x00" as *const u8 as *const libc::c_char,
        ) == 0
    {
        clc.state = crate::src::qcommon::q_shared::CA_CONNECTED; // so the connect screen is drawn
        crate::stdlib::memset(
            cls.updateInfoString.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        crate::stdlib::memset(
            clc.serverMessage.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        crate::stdlib::memset(
            &mut cl.gameState as *mut crate::src::qcommon::q_shared::gameState_t
                as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::gameState_t>() as libc::c_ulong,
        );
        clc.lastPacketSentTime = -(9999 as libc::c_int);
        crate::src::client::cl_scrn::SCR_UpdateScreen();
    } else {
        // clear nextmap so the cinematic shutdown doesn't execute it
        crate::src::qcommon::cvar::Cvar_Set(
            b"nextmap\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        ); // so the connect screen is drawn
        CL_Disconnect(crate::src::qcommon::q_shared::qtrue);
        crate::src::qcommon::q_shared::Q_strncpyz(
            clc.servername.as_mut_ptr(),
            b"localhost\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        );
        clc.state = crate::src::qcommon::q_shared::CA_CHALLENGING;
        crate::src::client::cl_keys::Key_SetCatcher(0 as libc::c_int);
        crate::src::client::cl_scrn::SCR_UpdateScreen();
        clc.connectTime = -(3000 as libc::c_int);
        crate::src::qcommon::net_chan::NET_StringToAdr(
            clc.servername.as_mut_ptr(),
            &mut clc.serverAddress as *mut _ as *mut crate::qcommon_h::netadr_t,
            crate::qcommon_h::NA_UNSPEC,
        );
        // we don't need a challenge on the localhost
        CL_CheckForResend();
    };
}
/*
=====================
CL_ClearState

Called before parsing a gamestate
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ClearState() {
    //	S_StopAllSounds();
    crate::stdlib::memset(
        &mut cl as *mut crate::client_h::clientActive_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::client_h::clientActive_t>() as libc::c_ulong,
    );
}
/*
====================
CL_UpdateGUID

update cl_guid using QKEY_FILE and optional prefix
====================
*/

unsafe extern "C" fn CL_UpdateGUID(mut prefix: *const libc::c_char, mut prefix_len: libc::c_int) {
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut len: libc::c_int = 0;
    len = crate::src::qcommon::files::FS_SV_FOpenFileRead(
        b"qkey\x00" as *const u8 as *const libc::c_char,
        &mut f,
    ) as libc::c_int;
    crate::src::qcommon::files::FS_FCloseFile(f);
    if len != 2048 as libc::c_int {
        crate::src::qcommon::cvar::Cvar_Set(
            b"cl_guid\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        crate::src::qcommon::cvar::Cvar_Set(
            b"cl_guid\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::md5::Com_MD5File(
                b"qkey\x00" as *const u8 as *const libc::c_char,
                2048 as libc::c_int,
                prefix,
                prefix_len,
            ),
        );
    };
}

unsafe extern "C" fn CL_OldGame() {
    if cl_oldGameSet as u64 != 0 {
        // change back to previous fs_game
        cl_oldGameSet = crate::src::qcommon::q_shared::qfalse;

        crate::src::qcommon::cvar::Cvar_Set2(
            b"fs_game\x00" as *const u8 as *const libc::c_char,
            cl_oldGame.as_mut_ptr(),
            crate::src::qcommon::q_shared::qtrue,
        ) as *mut crate::src::qcommon::q_shared::cvar_s;
        crate::src::qcommon::files::FS_ConditionalRestart(
            clc.checksumFeed,
            crate::src::qcommon::q_shared::qfalse,
        );
    };
}
/*
=====================
CL_Disconnect

Called when a connection, demo, or cinematic is being terminated.
Goes from a connected state to either a menu state or a console state
Sends a disconnect message to the server
This is also called on Com_Error and Com_Quit, so it shouldn't cause any errors
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Disconnect(mut showMainMenu: crate::src::qcommon::q_shared::qboolean) {
    if crate::src::qcommon::common::com_cl_running.is_null()
        || (*crate::src::qcommon::common::com_cl_running).integer == 0
    {
        return;
    }
    // shutting down the client so enter full screen ui mode
    crate::src::qcommon::cvar::Cvar_Set(
        b"r_uiFullScreen\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
    ); // disable this for a moment.
    if clc.demorecording as u64 != 0 {
        CL_StopRecord_f(); // dump any pending VoIP transmission.
    } // clean up any state...
    if clc.download != 0 {
        crate::src::qcommon::files::FS_FCloseFile(clc.download);
        clc.download = 0 as libc::c_int
    }
    let ref mut fresh0 = *clc.downloadName.as_mut_ptr();
    *fresh0 = 0 as libc::c_int as libc::c_char;
    *clc.downloadTempName.as_mut_ptr() = *fresh0;
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_downloadName\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    if (*cl_useMumble).integer != 0 && crate::src::client::libmumblelink::mumble_islinked() != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"Mumble: Unlinking from Mumble application\n\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::client::libmumblelink::mumble_unlink();
    }
    if (*cl_voipSend).integer != 0 {
        let mut tmp: libc::c_int = (*cl_voipUseVAD).integer;
        (*cl_voipUseVAD).integer = 0 as libc::c_int;
        clc.voipOutgoingDataSize = 0 as libc::c_int;
        crate::src::qcommon::cvar::Cvar_Set(
            b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        CL_CaptureVoip();
        (*cl_voipUseVAD).integer = tmp
    }
    if clc.voipCodecInitialized as u64 != 0 {
        let mut i: libc::c_int = 0;
        crate::src::opus_1_2_1::src::opus_encoder::opus_encoder_destroy(clc.opusEncoder);
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            crate::src::opus_1_2_1::src::opus_decoder::opus_decoder_destroy(
                clc.opusDecoder[i as usize],
            );
            i += 1
        }
        clc.voipCodecInitialized = crate::src::qcommon::q_shared::qfalse
    }
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"voip\x00" as *const u8 as *const libc::c_char);
    if clc.demofile != 0 {
        crate::src::qcommon::files::FS_FCloseFile(clc.demofile);
        clc.demofile = 0 as libc::c_int
    }
    if !crate::src::client::cl_ui::uivm.is_null() && showMainMenu as libc::c_uint != 0 {
        crate::src::qcommon::vm::VM_Call(
            crate::src::client::cl_ui::uivm,
            crate::ui_public_h::UI_SET_ACTIVE_MENU as libc::c_int,
            crate::ui_public_h::UIMENU_NONE as libc::c_int,
        );
    }
    crate::src::client::cl_cin::SCR_StopCinematic();
    crate::src::client::snd_main::S_ClearSoundBuffer();
    // send a disconnect message to the server
    // send it a few times in case one is dropped
    if clc.state as libc::c_uint
        >= crate::src::qcommon::q_shared::CA_CONNECTED as libc::c_int as libc::c_uint
    {
        CL_AddReliableCommand(
            b"disconnect\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::qtrue,
        );
        crate::src::client::cl_input::CL_WritePacket();
        crate::src::client::cl_input::CL_WritePacket();
        crate::src::client::cl_input::CL_WritePacket();
    }
    // Remove pure paks
    crate::src::qcommon::files::FS_PureServerSetLoadedPaks(
        b"\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::files::FS_PureServerSetReferencedPaks(
        b"\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    CL_ClearState();
    // wipe the client connection
    crate::stdlib::memset(
        &mut clc as *mut crate::client_h::clientConnection_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::client_h::clientConnection_t>() as libc::c_ulong,
    );
    clc.state = crate::src::qcommon::q_shared::CA_DISCONNECTED;
    // allow cheats locally
    crate::src::qcommon::cvar::Cvar_Set(
        b"sv_cheats\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
    );
    // not connected to a pure server anymore
    crate::src::client::cl_parse::cl_connectedToPureServer =
        crate::src::qcommon::q_shared::qfalse as libc::c_int;
    // not connected to voip server anymore.
    clc.voipEnabled = crate::src::qcommon::q_shared::qfalse;
    // Stop recording any video
    if crate::src::client::cl_avi::CL_VideoRecording() as u64 != 0 {
        // Finish rendering current frame
        crate::src::client::cl_scrn::SCR_UpdateScreen();
        crate::src::client::cl_avi::CL_CloseAVI();
    }
    CL_UpdateGUID(0 as *const libc::c_char, 0 as libc::c_int);
    if noGameRestart == 0 {
        CL_OldGame();
    } else {
        noGameRestart = crate::src::qcommon::q_shared::qfalse as libc::c_int
    };
}
/*
===================
CL_ForwardCommandToServer

adds the current command line as a clientCommand
things like godmode, noclip, etc, are commands directed to the server,
so when they are typed in at the console, they will need to be forwarded.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ForwardCommandToServer(mut string: *const libc::c_char) {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    cmd = crate::src::qcommon::cmd::Cmd_Argv(0 as libc::c_int);
    // ignore key up commands
    if *cmd.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        return;
    }
    if clc.demoplaying as libc::c_uint != 0
        || (clc.state as libc::c_uint)
            < crate::src::qcommon::q_shared::CA_CONNECTED as libc::c_int as libc::c_uint
        || *cmd.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
    {
        crate::src::qcommon::common::Com_Printf(
            b"Unknown command \"%s^7\"\n\x00" as *const u8 as *const libc::c_char,
            cmd,
        );
        return;
    }
    if crate::src::qcommon::cmd::Cmd_Argc() > 1 as libc::c_int {
        CL_AddReliableCommand(string, crate::src::qcommon::q_shared::qfalse);
    } else {
        CL_AddReliableCommand(cmd, crate::src::qcommon::q_shared::qfalse);
    };
}
/*
===================
CL_RequestMotd

===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_RequestMotd() {
    let mut info: [libc::c_char; 1024] = [0; 1024];
    if (*cl_motd).integer == 0 {
        return;
    }
    crate::src::qcommon::common::Com_Printf(
        b"Resolving %s\n\x00" as *const u8 as *const libc::c_char,
        b"update.quake3arena.com\x00" as *const u8 as *const libc::c_char,
    );
    if crate::src::qcommon::net_chan::NET_StringToAdr(
        b"update.quake3arena.com\x00" as *const u8 as *const libc::c_char,
        &mut cls.updateServer as *mut _ as *mut crate::qcommon_h::netadr_t,
        crate::qcommon_h::NA_IP,
    ) == 0
    {
        crate::src::qcommon::common::Com_Printf(
            b"Couldn\'t resolve address\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    cls.updateServer.port =
        crate::src::qcommon::q_shared::ShortSwap(27951 as libc::c_int as libc::c_short)
            as libc::c_ushort;
    crate::src::qcommon::common::Com_Printf(
        b"%s resolved to %i.%i.%i.%i:%i\n\x00" as *const u8 as *const libc::c_char,
        b"update.quake3arena.com\x00" as *const u8 as *const libc::c_char,
        cls.updateServer.ip[0 as libc::c_int as usize] as libc::c_int,
        cls.updateServer.ip[1 as libc::c_int as usize] as libc::c_int,
        cls.updateServer.ip[2 as libc::c_int as usize] as libc::c_int,
        cls.updateServer.ip[3 as libc::c_int as usize] as libc::c_int,
        crate::src::qcommon::q_shared::ShortSwap(cls.updateServer.port as libc::c_short)
            as libc::c_int,
    );
    info[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    crate::src::qcommon::q_shared::Com_sprintf(
        cls.updateChallenge.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        b"%i\x00" as *const u8 as *const libc::c_char,
        ((::libc::rand() as libc::c_uint) << 16 as libc::c_int
            ^ ::libc::rand() as libc::c_uint
            ^ crate::src::qcommon::common::Com_Milliseconds() as libc::c_uint)
            as libc::c_int,
    );
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        info.as_mut_ptr(),
        b"challenge\x00" as *const u8 as *const libc::c_char,
        cls.updateChallenge.as_mut_ptr(),
    );
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        info.as_mut_ptr(),
        b"renderer\x00" as *const u8 as *const libc::c_char,
        cls.glconfig.renderer_string.as_mut_ptr(),
    );
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        info.as_mut_ptr(),
        b"version\x00" as *const u8 as *const libc::c_char,
        (*crate::src::qcommon::common::com_version).string,
    );
    crate::src::qcommon::net_chan::NET_OutOfBandPrint(
        crate::qcommon_h::NS_CLIENT,
        cls.updateServer as crate::qcommon_h::netadr_t,
        b"getmotd \"%s\"\n\x00" as *const u8 as *const libc::c_char,
        info.as_mut_ptr(),
    );
}
/*
===================
CL_RequestAuthorization

Authorization server protocol
-----------------------------

All commands are text in Q3 out of band packets (leading 0xff 0xff 0xff 0xff).

Whenever the client tries to get a challenge from the server it wants to
connect to, it also blindly fires off a packet to the authorize server:

getKeyAuthorize <challenge> <cdkey>

cdkey may be "demo"


#OLD The authorize server returns a:
#OLD
#OLD keyAthorize <challenge> <accept | deny>
#OLD
#OLD A client will be accepted if the cdkey is valid and it has not been used by any other IP
#OLD address in the last 15 minutes.


The server sends a:

getIpAuthorize <challenge> <ip>

The authorize server returns a:

ipAuthorize <challenge> <accept | deny | demo | unknown >

A client will be accepted if a valid cdkey was sent by that ip (only) in the last 15 minutes.
If no response is received from the authorize server after two tries, the client will be let
in anyway.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_RequestAuthorization() {
    let mut nums: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut fs: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    if cls.authorizeServer.port == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"Resolving %s\n\x00" as *const u8 as *const libc::c_char,
            b"authorize.quake3arena.com\x00" as *const u8 as *const libc::c_char,
        );
        if crate::src::qcommon::net_chan::NET_StringToAdr(
            b"authorize.quake3arena.com\x00" as *const u8 as *const libc::c_char,
            &mut cls.authorizeServer as *mut _ as *mut crate::qcommon_h::netadr_t,
            crate::qcommon_h::NA_IP,
        ) == 0
        {
            crate::src::qcommon::common::Com_Printf(
                b"Couldn\'t resolve address\n\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        cls.authorizeServer.port =
            crate::src::qcommon::q_shared::ShortSwap(27952 as libc::c_int as libc::c_short)
                as libc::c_ushort;
        crate::src::qcommon::common::Com_Printf(
            b"%s resolved to %i.%i.%i.%i:%i\n\x00" as *const u8 as *const libc::c_char,
            b"authorize.quake3arena.com\x00" as *const u8 as *const libc::c_char,
            cls.authorizeServer.ip[0 as libc::c_int as usize] as libc::c_int,
            cls.authorizeServer.ip[1 as libc::c_int as usize] as libc::c_int,
            cls.authorizeServer.ip[2 as libc::c_int as usize] as libc::c_int,
            cls.authorizeServer.ip[3 as libc::c_int as usize] as libc::c_int,
            crate::src::qcommon::q_shared::ShortSwap(cls.authorizeServer.port as libc::c_short)
                as libc::c_int,
        );
    }
    if cls.authorizeServer.type_0 as libc::c_uint
        == crate::qcommon_h::NA_BAD as libc::c_int as libc::c_uint
    {
        return;
    }
    // only grab the alphanumeric values from the cdkey, to avoid any dashes or spaces
    j = 0 as libc::c_int;
    l = crate::stdlib::strlen(crate::src::qcommon::common::cl_cdkey.as_mut_ptr()) as libc::c_int;
    if l > 32 as libc::c_int {
        l = 32 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < l {
        if crate::src::qcommon::common::cl_cdkey[i as usize] as libc::c_int >= '0' as i32
            && crate::src::qcommon::common::cl_cdkey[i as usize] as libc::c_int <= '9' as i32
            || crate::src::qcommon::common::cl_cdkey[i as usize] as libc::c_int >= 'a' as i32
                && crate::src::qcommon::common::cl_cdkey[i as usize] as libc::c_int <= 'z' as i32
            || crate::src::qcommon::common::cl_cdkey[i as usize] as libc::c_int >= 'A' as i32
                && crate::src::qcommon::common::cl_cdkey[i as usize] as libc::c_int <= 'Z' as i32
        {
            nums[j as usize] = crate::src::qcommon::common::cl_cdkey[i as usize];
            j += 1
        }
        i += 1
    }
    nums[j as usize] = 0 as libc::c_int as libc::c_char;
    fs = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_anonymous\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x10 as libc::c_int | 0x8 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::qcommon::net_chan::NET_OutOfBandPrint(
        crate::qcommon_h::NS_CLIENT,
        cls.authorizeServer as crate::qcommon_h::netadr_t,
        b"getKeyAuthorize %i %s\x00" as *const u8 as *const libc::c_char,
        (*fs).integer,
        nums.as_mut_ptr(),
    );
}
/*
======================================================================

CONSOLE COMMANDS

======================================================================
*/
/*
==================
CL_ForwardToServer_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ForwardToServer_f() {
    if clc.state as libc::c_uint
        != crate::src::qcommon::q_shared::CA_ACTIVE as libc::c_int as libc::c_uint
        || clc.demoplaying as libc::c_uint != 0
    {
        crate::src::qcommon::common::Com_Printf(
            b"Not connected to a server.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    // don't forward the first argument
    if crate::src::qcommon::cmd::Cmd_Argc() > 1 as libc::c_int {
        CL_AddReliableCommand(
            crate::src::qcommon::cmd::Cmd_Args(),
            crate::src::qcommon::q_shared::qfalse,
        );
    };
}
/*
==================
CL_Disconnect_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Disconnect_f() {
    crate::src::client::cl_cin::SCR_StopCinematic();
    crate::src::qcommon::cvar::Cvar_Set(
        b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    if clc.state as libc::c_uint
        != crate::src::qcommon::q_shared::CA_DISCONNECTED as libc::c_int as libc::c_uint
        && clc.state as libc::c_uint
            != crate::src::qcommon::q_shared::CA_CINEMATIC as libc::c_int as libc::c_uint
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DISCONNECT as libc::c_int,
            b"Disconnected from server\x00" as *const u8 as *const libc::c_char,
        );
    };
}
/*
================
CL_Reconnect_f

================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Reconnect_f() {
    if crate::stdlib::strlen(cl_reconnectArgs.as_mut_ptr()) == 0 {
        return;
    }
    crate::src::qcommon::cvar::Cvar_Set(
        b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cbuf_AddText(crate::src::qcommon::q_shared::va(
        b"connect %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        cl_reconnectArgs.as_mut_ptr(),
    ));
}
/*
================
CL_Connect_f

================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Connect_f() {
    let mut server: [libc::c_char; 4096] = [0; 4096];
    let mut serverString: *const libc::c_char = 0 as *const libc::c_char;
    let mut argc: libc::c_int = crate::src::qcommon::cmd::Cmd_Argc();
    let mut family: crate::qcommon_h::netadrtype_t = crate::qcommon_h::NA_UNSPEC;
    if argc != 2 as libc::c_int && argc != 3 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"usage: connect [-4|-6] server\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if argc == 2 as libc::c_int {
        crate::src::qcommon::q_shared::Q_strncpyz(
            server.as_mut_ptr(),
            crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        );
    } else {
        if ::libc::strcmp(
            crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
            b"-4\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            family = crate::qcommon_h::NA_IP
        } else if ::libc::strcmp(
            crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
            b"-6\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            family = crate::qcommon_h::NA_IP6
        } else {
            crate::src::qcommon::common::Com_Printf(
                b"warning: only -4 or -6 as address type understood.\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            server.as_mut_ptr(),
            crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        );
    }
    // save arguments for reconnect
    crate::src::qcommon::q_shared::Q_strncpyz(
        cl_reconnectArgs.as_mut_ptr(),
        crate::src::qcommon::cmd::Cmd_Args(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::qcommon::cvar::Cvar_Set(
        b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    // fire a message off to the motd server
    CL_RequestMotd();
    // clear any previous "server full" type messages
    clc.serverMessage[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if (*crate::src::qcommon::common::com_sv_running).integer != 0
        && ::libc::strcmp(
            server.as_mut_ptr(),
            b"localhost\x00" as *const u8 as *const libc::c_char,
        ) == 0
    {
        // if running a local server, kill it
        crate::src::server::sv_init::SV_Shutdown(
            b"Server quit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    // make sure a local server is killed
    crate::src::qcommon::cvar::Cvar_Set(
        b"sv_killserver\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::server::sv_main::SV_Frame(0 as libc::c_int);
    noGameRestart = crate::src::qcommon::q_shared::qtrue as libc::c_int;
    CL_Disconnect(crate::src::qcommon::q_shared::qtrue);
    crate::src::client::cl_console::Con_Close();
    crate::src::qcommon::q_shared::Q_strncpyz(
        clc.servername.as_mut_ptr(),
        server.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
    );
    if crate::src::qcommon::net_chan::NET_StringToAdr(
        clc.servername.as_mut_ptr(),
        &mut clc.serverAddress as *mut _ as *mut crate::qcommon_h::netadr_t,
        family,
    ) == 0
    {
        crate::src::qcommon::common::Com_Printf(
            b"Bad server address\n\x00" as *const u8 as *const libc::c_char,
        );
        clc.state = crate::src::qcommon::q_shared::CA_DISCONNECTED;
        return;
    }
    if clc.serverAddress.port as libc::c_int == 0 as libc::c_int {
        clc.serverAddress.port =
            crate::src::qcommon::q_shared::ShortSwap(27960 as libc::c_int as libc::c_short)
                as libc::c_ushort
    }
    serverString = crate::src::qcommon::net_ip::NET_AdrToStringwPort(
        clc.serverAddress as crate::qcommon_h::netadr_t,
    );
    crate::src::qcommon::common::Com_Printf(
        b"%s resolved to %s\n\x00" as *const u8 as *const libc::c_char,
        clc.servername.as_mut_ptr(),
        serverString,
    );
    if (*cl_guidServerUniq).integer != 0 {
        CL_UpdateGUID(
            serverString,
            crate::stdlib::strlen(serverString) as libc::c_int,
        );
    } else {
        CL_UpdateGUID(0 as *const libc::c_char, 0 as libc::c_int);
    }
    // if we aren't playing on a lan, we need to authenticate
    // with the cd key
    if crate::src::qcommon::net_ip::NET_IsLocalAddress(
        clc.serverAddress as crate::qcommon_h::netadr_t,
    ) as u64
        != 0
    {
        clc.state = crate::src::qcommon::q_shared::CA_CHALLENGING
    } else {
        clc.state = crate::src::qcommon::q_shared::CA_CONNECTING;
        // Set a client challenge number that ideally is mirrored back by the server.
        clc.challenge = ((::libc::rand() as libc::c_uint) << 16 as libc::c_int
            ^ ::libc::rand() as libc::c_uint
            ^ crate::src::qcommon::common::Com_Milliseconds() as libc::c_uint)
            as libc::c_int
    } // CL_CheckForResend() will fire immediately
    crate::src::client::cl_keys::Key_SetCatcher(0 as libc::c_int);
    clc.connectTime = -(99999 as libc::c_int);
    clc.connectPacketCount = 0 as libc::c_int;
    // server connection string
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_currentServerAddress\x00" as *const u8 as *const libc::c_char,
        server.as_mut_ptr(),
    );
}
/*
==================
CL_CompleteRcon
==================
*/

unsafe extern "C" fn CL_CompleteRcon(mut args: *mut libc::c_char, mut argNum: libc::c_int) {
    if argNum == 2 as libc::c_int {
        // Skip "rcon "
        let mut p: *mut libc::c_char = crate::src::qcommon::q_shared::Com_SkipTokens(
            args,
            1 as libc::c_int,
            b" \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if p > args {
            crate::src::qcommon::common::Field_CompleteCommand(
                p,
                crate::src::qcommon::q_shared::qtrue,
                crate::src::qcommon::q_shared::qtrue,
            );
        }
    };
}
/*
==================
CL_CompletePlayerName
==================
*/

unsafe extern "C" fn CL_CompletePlayerName(mut _args: *mut libc::c_char, mut argNum: libc::c_int) {
    if argNum == 2 as libc::c_int {
        let mut names: [[libc::c_char; 32]; 64] = [[0; 32]; 64];
        let mut namesPtr: [*const libc::c_char; 64] = [0 as *const libc::c_char; 64];
        let mut i: libc::c_int = 0;
        let mut clientCount: libc::c_int = 0;
        let mut nameCount: libc::c_int = 0;
        let mut info: *const libc::c_char = 0 as *const libc::c_char;
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        //configstring
        info = cl
            .gameState
            .stringData
            .as_mut_ptr()
            .offset(cl.gameState.stringOffsets[0 as libc::c_int as usize] as isize);
        clientCount = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
        ));
        nameCount = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < clientCount {
            if !(i == clc.clientNum) {
                info = cl.gameState.stringData.as_mut_ptr().offset(
                    cl.gameState.stringOffsets
                        [(32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + i) as usize]
                        as isize,
                );
                name = crate::src::qcommon::q_shared::Info_ValueForKey(
                    info,
                    b"n\x00" as *const u8 as *const libc::c_char,
                );
                if !(*name.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32) {
                    crate::src::qcommon::q_shared::Q_strncpyz(
                        names[nameCount as usize].as_mut_ptr(),
                        name,
                        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
                    );
                    crate::src::qcommon::q_shared::Q_CleanStr(
                        names[nameCount as usize].as_mut_ptr(),
                    );
                    namesPtr[nameCount as usize] = names[nameCount as usize].as_mut_ptr();
                    nameCount += 1
                }
            }
            i += 1
        }
        crate::stdlib::qsort(
            namesPtr.as_mut_ptr() as *mut libc::c_void,
            nameCount as crate::stddef_h::size_t,
            ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
            Some(
                crate::src::qcommon::common::Com_strCompare
                    as unsafe extern "C" fn(
                        _: *const libc::c_void,
                        _: *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        crate::src::qcommon::common::Field_CompletePlayerName(namesPtr.as_mut_ptr(), nameCount);
    };
}
/*
=====================
CL_Rcon_f

  Send the rest of the command line over as
  an unconnected command.
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Rcon_f() {
    let mut message: [libc::c_char; 1024] = [0; 1024];
    let mut to: crate::qcommon_h::netadr_t = crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    };
    if *(*rcon_client_password)
        .string
        .offset(0 as libc::c_int as isize)
        == 0
    {
        crate::src::qcommon::common::Com_Printf(
            b"You must set \'rconpassword\' before\nissuing an rcon command.\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    message[0 as libc::c_int as usize] = -(1 as libc::c_int) as libc::c_char;
    message[1 as libc::c_int as usize] = -(1 as libc::c_int) as libc::c_char;
    message[2 as libc::c_int as usize] = -(1 as libc::c_int) as libc::c_char;
    message[3 as libc::c_int as usize] = -(1 as libc::c_int) as libc::c_char;
    message[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    crate::src::qcommon::q_shared::Q_strcat(
        message.as_mut_ptr(),
        1024 as libc::c_int,
        b"rcon \x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::q_shared::Q_strcat(
        message.as_mut_ptr(),
        1024 as libc::c_int,
        (*rcon_client_password).string,
    );
    crate::src::qcommon::q_shared::Q_strcat(
        message.as_mut_ptr(),
        1024 as libc::c_int,
        b" \x00" as *const u8 as *const libc::c_char,
    );
    // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=543
    crate::src::qcommon::q_shared::Q_strcat(
        message.as_mut_ptr(),
        1024 as libc::c_int,
        crate::src::qcommon::cmd::Cmd_Cmd().offset(5 as libc::c_int as isize),
    );
    if clc.state as libc::c_uint
        >= crate::src::qcommon::q_shared::CA_CONNECTED as libc::c_int as libc::c_uint
    {
        to = clc.netchan.remoteAddress
    } else {
        if crate::stdlib::strlen((*rconAddress).string) == 0 {
            crate::src::qcommon::common::Com_Printf(b"You must either be connected,\nor set the \'rconAddress\' cvar\nto issue rcon commands\n\x00"
                           as *const u8 as *const libc::c_char);
            return;
        }
        crate::src::qcommon::net_chan::NET_StringToAdr(
            (*rconAddress).string,
            &mut to as *mut _ as *mut crate::qcommon_h::netadr_t,
            crate::qcommon_h::NA_UNSPEC,
        );
        if to.port as libc::c_int == 0 as libc::c_int {
            to.port =
                crate::src::qcommon::q_shared::ShortSwap(27960 as libc::c_int as libc::c_short)
                    as libc::c_ushort
        }
    }
    crate::src::qcommon::net_chan::NET_SendPacket(
        crate::qcommon_h::NS_CLIENT,
        crate::stdlib::strlen(message.as_mut_ptr()).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_int,
        message.as_mut_ptr() as *const libc::c_void,
        to as crate::qcommon_h::netadr_t,
    );
    cls.rconAddress = to;
}
/*
=================
CL_SendPureChecksums
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_SendPureChecksums() {
    let mut cMsg: [libc::c_char; 1024] = [0; 1024];
    // if we are pure we need to send back a command with our referenced pk3 checksums
    crate::src::qcommon::q_shared::Com_sprintf(
        cMsg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        b"cp %d %s\x00" as *const u8 as *const libc::c_char,
        cl.serverId,
        crate::src::qcommon::files::FS_ReferencedPakPureChecksums(),
    );
    CL_AddReliableCommand(cMsg.as_mut_ptr(), crate::src::qcommon::q_shared::qfalse);
}
/*
=================
CL_ResetPureClientAtServer
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ResetPureClientAtServer() {
    CL_AddReliableCommand(
        b"vdr\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::qfalse,
    );
}
/*
=================
CL_Vid_Restart_f

Restart the video subsystem

we also have to reload the UI and CGame because the renderer
doesn't know what graphics to reload
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Vid_Restart_f() {
    // Settings may have changed so stop recording now
    if crate::src::client::cl_avi::CL_VideoRecording() as u64 != 0 {
        crate::src::client::cl_avi::CL_CloseAVI();
    }
    if clc.demorecording as u64 != 0 {
        CL_StopRecord_f();
    }
    // don't let them loop during the restart
    crate::src::client::snd_main::S_StopAllSounds();
    if crate::src::qcommon::files::FS_ConditionalRestart(
        clc.checksumFeed,
        crate::src::qcommon::q_shared::qtrue,
    ) as u64
        == 0
    {
        // if not running a server clear the whole hunk
        if (*crate::src::qcommon::common::com_sv_running).integer != 0 {
            // clear all the client data on the hunk
            crate::src::qcommon::common::Hunk_ClearToMark();
        } else {
            // clear the whole hunk
            crate::src::qcommon::common::Hunk_Clear();
        }
        // shutdown the UI
        crate::src::client::cl_ui::CL_ShutdownUI();
        // shutdown the CGame
        crate::src::client::cl_cgame::CL_ShutdownCGame();
        // shutdown the renderer and clear the renderer interface
        CL_ShutdownRef();
        // client is no longer pure until new checksums are sent
        CL_ResetPureClientAtServer();
        // clear pak references
        crate::src::qcommon::files::FS_ClearPakReferences(0x2 as libc::c_int | 0x4 as libc::c_int);
        // reinitialize the filesystem if the game directory or checksum has changed
        cls.rendererStarted = crate::src::qcommon::q_shared::qfalse;
        cls.uiStarted = crate::src::qcommon::q_shared::qfalse;
        cls.cgameStarted = crate::src::qcommon::q_shared::qfalse;
        cls.soundRegistered = crate::src::qcommon::q_shared::qfalse;
        // unpause so the cgame definitely gets a snapshot and renders a frame
        crate::src::qcommon::cvar::Cvar_Set(
            b"cl_paused\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        // initialize the renderer interface
        CL_InitRef();
        // startup all the client stuff
        CL_StartHunkUsers(crate::src::qcommon::q_shared::qfalse);
        // start the cgame if connected
        if clc.state as libc::c_uint
            > crate::src::qcommon::q_shared::CA_CONNECTED as libc::c_int as libc::c_uint
            && clc.state as libc::c_uint
                != crate::src::qcommon::q_shared::CA_CINEMATIC as libc::c_int as libc::c_uint
        {
            cls.cgameStarted = crate::src::qcommon::q_shared::qtrue;
            crate::src::client::cl_cgame::CL_InitCGame();
            // send pure checksums
            CL_SendPureChecksums();
        }
    };
}
/*
=================
CL_Snd_Restart

Restart the sound subsystem
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Snd_Shutdown() {
    crate::src::client::snd_main::S_Shutdown();
    cls.soundStarted = crate::src::qcommon::q_shared::qfalse;
}
/*
=================
CL_Snd_Restart_f

Restart the sound subsystem
The cgame and game must also be forced to restart because
handles will be invalid
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Snd_Restart_f() {
    CL_Snd_Shutdown();
    // sound will be reinitialized by vid_restart
    CL_Vid_Restart_f();
}
/*
==================
CL_PK3List_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_OpenedPK3List_f() {
    crate::src::qcommon::common::Com_Printf(
        b"Opened PK3 Names: %s\n\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::files::FS_LoadedPakNames(),
    );
}
/*
==================
CL_PureList_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ReferencedPK3List_f() {
    crate::src::qcommon::common::Com_Printf(
        b"Referenced PK3 Names: %s\n\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::files::FS_ReferencedPakNames(),
    );
}
/*
==================
CL_Configstrings_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Configstrings_f() {
    let mut i: libc::c_int = 0;
    let mut ofs: libc::c_int = 0;
    if clc.state as libc::c_uint
        != crate::src::qcommon::q_shared::CA_ACTIVE as libc::c_int as libc::c_uint
    {
        crate::src::qcommon::common::Com_Printf(
            b"Not connected to a server.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        ofs = cl.gameState.stringOffsets[i as usize];
        if !(ofs == 0) {
            crate::src::qcommon::common::Com_Printf(
                b"%4i: %s\n\x00" as *const u8 as *const libc::c_char,
                i,
                cl.gameState.stringData.as_mut_ptr().offset(ofs as isize),
            );
        }
        i += 1
    }
}
/*
==============
CL_Clientinfo_f
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Clientinfo_f() {
    crate::src::qcommon::common::Com_Printf(
        b"--------- Client Information ---------\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::common::Com_Printf(
        b"state: %i\n\x00" as *const u8 as *const libc::c_char,
        clc.state as libc::c_uint,
    );
    crate::src::qcommon::common::Com_Printf(
        b"Server: %s\n\x00" as *const u8 as *const libc::c_char,
        clc.servername.as_mut_ptr(),
    );
    crate::src::qcommon::common::Com_Printf(
        b"User info settings:\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::common::Info_Print(crate::src::qcommon::cvar::Cvar_InfoString(
        0x2 as libc::c_int,
    ));
    crate::src::qcommon::common::Com_Printf(
        b"--------------------------------------\n\x00" as *const u8 as *const libc::c_char,
    );
}
//====================================================================
/*
=================
CL_DownloadsComplete

Called when all downloading has been completed
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_DownloadsComplete() {
    // if we downloaded with cURL
    if clc.cURLUsed as u64 != 0 {
        clc.cURLUsed = crate::src::qcommon::q_shared::qfalse;
        crate::src::client::cl_curl::CL_cURL_Shutdown();
        if clc.cURLDisconnected as u64 != 0 {
            if clc.downloadRestart as u64 != 0 {
                crate::src::qcommon::files::FS_Restart(clc.checksumFeed);
                clc.downloadRestart = crate::src::qcommon::q_shared::qfalse
            }
            clc.cURLDisconnected = crate::src::qcommon::q_shared::qfalse;
            CL_Reconnect_f();
            return;
        }
    }
    // if we downloaded files we need to restart the file system
    if clc.downloadRestart as u64 != 0 {
        clc.downloadRestart = crate::src::qcommon::q_shared::qfalse; // We possibly downloaded a pak, restart the file system to load it
        crate::src::qcommon::files::FS_Restart(clc.checksumFeed);
        // inform the server so we get new gamestate info
        CL_AddReliableCommand(
            b"donedl\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::qfalse,
        );
        // by sending the donedl command we request a new gamestate
        // so we don't want to load stuff yet
        return;
    }
    // let the client game init and load data
    clc.state = crate::src::qcommon::q_shared::CA_LOADING;
    // Pump the loop, this may change gamestate!
    crate::src::qcommon::common::Com_EventLoop();
    // if the gamestate was changed by calling Com_EventLoop
    // then we loaded everything already and we don't want to do it again.
    if clc.state as libc::c_uint
        != crate::src::qcommon::q_shared::CA_LOADING as libc::c_int as libc::c_uint
    {
        return;
    }
    // starting to load a map so we get out of full screen ui mode
    crate::src::qcommon::cvar::Cvar_Set(
        b"r_uiFullScreen\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    // flush client memory and start loading stuff
    // this will also (re)load the UI
    // if this is a local client then only the client part of the hunk
    // will be cleared, note that this is done after the hunk mark has been set
    CL_FlushMemory();
    // initialize the CGame
    cls.cgameStarted = crate::src::qcommon::q_shared::qtrue;
    crate::src::client::cl_cgame::CL_InitCGame();
    // set pure checksums
    CL_SendPureChecksums();
    crate::src::client::cl_input::CL_WritePacket();
    crate::src::client::cl_input::CL_WritePacket();
    crate::src::client::cl_input::CL_WritePacket();
}
/*
=================
CL_BeginDownload

Requests a file to download from the server.  Stores it in the current
game directory.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_BeginDownload(
    mut localName: *const libc::c_char,
    mut remoteName: *const libc::c_char,
) {
    crate::src::qcommon::common::Com_DPrintf(b"***** CL_BeginDownload *****\nLocalname: %s\nRemotename: %s\n****************************\n\x00"
                    as *const u8 as *const libc::c_char, localName,
                remoteName);
    crate::src::qcommon::q_shared::Q_strncpyz(
        clc.downloadName.as_mut_ptr(),
        localName,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        clc.downloadTempName.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"%s.tmp\x00" as *const u8 as *const libc::c_char,
        localName,
    );
    // Set so UI gets access to it
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_downloadName\x00" as *const u8 as *const libc::c_char,
        remoteName,
    ); // Starting new file
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_downloadSize\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_downloadCount\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cvar::Cvar_SetValue(
        b"cl_downloadTime\x00" as *const u8 as *const libc::c_char,
        cls.realtime as libc::c_float,
    );
    clc.downloadBlock = 0 as libc::c_int;
    clc.downloadCount = 0 as libc::c_int;
    CL_AddReliableCommand(
        crate::src::qcommon::q_shared::va(
            b"download %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            remoteName,
        ),
        crate::src::qcommon::q_shared::qfalse,
    );
}
/*
=================
CL_NextDownload

A download completed or failed
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_NextDownload() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut remoteName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut localName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut useCURL: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    // A download has finished, check whether this matches a referenced checksum
    if *clc.downloadName.as_mut_ptr() != 0 {
        let mut zippath: *mut libc::c_char = crate::src::qcommon::files::FS_BuildOSPath(
            crate::src::qcommon::cvar::Cvar_VariableString(
                b"fs_homepath\x00" as *const u8 as *const libc::c_char,
            ),
            clc.downloadName.as_mut_ptr(),
            b"\x00" as *const u8 as *const libc::c_char,
        );
        *zippath.offset(
            crate::stdlib::strlen(zippath).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '\u{0}' as i32 as libc::c_char;
        if crate::src::qcommon::files::FS_CompareZipChecksum(zippath) as u64 == 0 {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"Incorrect checksum for file: %s\x00" as *const u8 as *const libc::c_char,
                clc.downloadName.as_mut_ptr(),
            );
        }
    }
    let ref mut fresh1 = *clc.downloadName.as_mut_ptr();
    *fresh1 = 0 as libc::c_int as libc::c_char;
    *clc.downloadTempName.as_mut_ptr() = *fresh1;
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_downloadName\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    // We are looking to start a download here
    if *clc.downloadList.as_mut_ptr() != 0 {
        s = clc.downloadList.as_mut_ptr();
        // format is:
        //  @remotename@localname@remotename@localname, etc.
        if *s as libc::c_int == '@' as i32 {
            s = s.offset(1)
        } // point at the nul byte
        remoteName = s;
        s = ::libc::strchr(s, '@' as i32);
        if s.is_null() {
            CL_DownloadsComplete();
            return;
        }
        let fresh2 = s;
        s = s.offset(1);
        *fresh2 = 0 as libc::c_int as libc::c_char;
        localName = s;
        s = ::libc::strchr(s, '@' as i32);
        if !s.is_null() {
            let fresh3 = s;
            s = s.offset(1);
            *fresh3 = 0 as libc::c_int as libc::c_char
        } else {
            s = localName.offset(crate::stdlib::strlen(localName) as isize)
        }
        if (*cl_allowDownload).integer & 2 as libc::c_int == 0 {
            if clc.sv_allowDownload & 2 as libc::c_int != 0 {
                crate::src::qcommon::common::Com_Printf(b"WARNING: server does not allow download redirection (sv_allowDownload is %d)\n\x00"
                               as *const u8 as *const libc::c_char,
                           clc.sv_allowDownload);
            } else if *clc.sv_dlURL.as_mut_ptr() == 0 {
                crate::src::qcommon::common::Com_Printf(b"WARNING: server allows download redirection, but does not have sv_dlURL set\n\x00"
                               as *const u8 as *const libc::c_char);
            } else if crate::src::client::cl_curl::CL_cURL_Init() as u64 == 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"WARNING: could not load cURL library\n\x00" as *const u8
                        as *const libc::c_char,
                );
            } else {
                crate::src::client::cl_curl::CL_cURL_BeginDownload(
                    localName,
                    crate::src::qcommon::q_shared::va(
                        b"%s/%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        clc.sv_dlURL.as_mut_ptr(),
                        remoteName,
                    ),
                );
                useCURL = crate::src::qcommon::q_shared::qtrue
            }
        } else if clc.sv_allowDownload & 2 as libc::c_int == 0 {
            crate::src::qcommon::common::Com_Printf(b"WARNING: server allows download redirection, but it disabled by client configuration (cl_allowDownload is %d)\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*cl_allowDownload).integer);
        }
        /* USE_CURL */
        if useCURL as u64 == 0 {
            if (*cl_allowDownload).integer & 4 as libc::c_int != 0 {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                    b"UDP Downloads are disabled on your client. (cl_allowDownload is %d)\x00"
                        as *const u8 as *const libc::c_char,
                    (*cl_allowDownload).integer,
                );
            } else {
                CL_BeginDownload(localName, remoteName);
            }
        }
        clc.downloadRestart = crate::src::qcommon::q_shared::qtrue;
        // move over the rest
        crate::stdlib::memmove(
            clc.downloadList.as_mut_ptr() as *mut libc::c_void,
            s as *const libc::c_void,
            crate::stdlib::strlen(s).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        return;
    }
    CL_DownloadsComplete();
}
/*
=================
CL_InitDownloads

After receiving a valid game state, we valid the cgame and local zip files here
and determine if we need to download them
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_InitDownloads() {
    let mut missingfiles: [libc::c_char; 1024] = [0; 1024];
    if (*cl_allowDownload).integer & 1 as libc::c_int == 0 {
        // autodownload is disabled on the client
        // but it's possible that some referenced files on the server are missing
        if crate::src::qcommon::files::FS_ComparePaks(
            missingfiles.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            crate::src::qcommon::q_shared::qfalse,
        ) as u64
            != 0
        {
            // NOTE TTimo I would rather have that printed as a modal message box
            //   but at this point while joining the game we don't know wether we will successfully join or not
            crate::src::qcommon::common::Com_Printf(b"\nWARNING: You are missing some files referenced by the server:\n%sYou might not be able to join the game\nGo to the setting menu to turn on autodownload, or get the file elsewhere\n\n\x00"
                           as *const u8 as *const libc::c_char,
                       missingfiles.as_mut_ptr());
        }
    } else if crate::src::qcommon::files::FS_ComparePaks(
        clc.downloadList.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        crate::src::qcommon::q_shared::qtrue,
    ) as u64
        != 0
    {
        crate::src::qcommon::common::Com_Printf(
            b"Need paks: %s\n\x00" as *const u8 as *const libc::c_char,
            clc.downloadList.as_mut_ptr(),
        );
        if *clc.downloadList.as_mut_ptr() != 0 {
            // if autodownloading is not enabled on the server
            clc.state = crate::src::qcommon::q_shared::CA_CONNECTED;
            let ref mut fresh4 = *clc.downloadName.as_mut_ptr();
            *fresh4 = 0 as libc::c_int as libc::c_char;
            *clc.downloadTempName.as_mut_ptr() = *fresh4;
            crate::src::qcommon::cvar::Cvar_Set(
                b"cl_downloadName\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char,
            );
            CL_NextDownload();
            return;
        }
    }
    CL_DownloadsComplete();
}
/*
=================
CL_CheckForResend

Resend a connect message if the last one has timed out
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CheckForResend() {
    let mut port: libc::c_int = 0;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    let mut data: [libc::c_char; 1034] = [0; 1034];
    // don't send anything if playing back a demo
    if clc.demoplaying as u64 != 0 {
        return;
    }
    // resend if we haven't gotten a reply yet
    if clc.state as libc::c_uint
        != crate::src::qcommon::q_shared::CA_CONNECTING as libc::c_int as libc::c_uint
        && clc.state as libc::c_uint
            != crate::src::qcommon::q_shared::CA_CHALLENGING as libc::c_int as libc::c_uint
    {
        return;
    } // for retransmit requests
    if cls.realtime - clc.connectTime < 3000 as libc::c_int {
        return;
    }
    clc.connectTime = cls.realtime;
    clc.connectPacketCount += 1;
    match clc.state as libc::c_uint {
        3 => {
            // requesting a challenge .. IPv6 users always get in as authorize server supports no ipv6.
            if (*crate::src::qcommon::common::com_standalone).integer == 0
                && clc.serverAddress.type_0 as libc::c_uint
                    == crate::qcommon_h::NA_IP as libc::c_int as libc::c_uint
                && crate::src::qcommon::net_ip::Sys_IsLANAddress(
                    clc.serverAddress as crate::qcommon_h::netadr_t,
                ) as u64
                    == 0
            {
                CL_RequestAuthorization();
            }
            // The challenge request shall be followed by a client challenge so no malicious server can hijack this connection.
            // Add the gamename so the server knows we're running the correct game or can reject the client
            // with a meaningful message
            crate::src::qcommon::q_shared::Com_sprintf(
                data.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1034]>() as libc::c_ulong as libc::c_int,
                b"getchallenge %d %s\x00" as *const u8 as *const libc::c_char,
                clc.challenge,
                (*crate::src::qcommon::common::com_gamename).string,
            );
            crate::src::qcommon::net_chan::NET_OutOfBandPrint(
                crate::qcommon_h::NS_CLIENT,
                clc.serverAddress as crate::qcommon_h::netadr_t,
                b"%s\x00" as *const u8 as *const libc::c_char,
                data.as_mut_ptr(),
            );
        }
        4 => {
            // sending back the challenge
            port = crate::src::qcommon::cvar::Cvar_VariableValue(
                b"net_qport\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_int;
            crate::src::qcommon::q_shared::Q_strncpyz(
                info.as_mut_ptr(),
                crate::src::qcommon::cvar::Cvar_InfoString(0x2 as libc::c_int),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            );
            if (*crate::src::qcommon::common::com_legacyprotocol).integer
                == (*crate::src::qcommon::common::com_protocol).integer
            {
                clc.compat = crate::src::qcommon::q_shared::qtrue
            }
            if clc.compat as u64 != 0 {
                crate::src::qcommon::q_shared::Info_SetValueForKey(
                    info.as_mut_ptr(),
                    b"protocol\x00" as *const u8 as *const libc::c_char,
                    crate::src::qcommon::q_shared::va(
                        b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        (*crate::src::qcommon::common::com_legacyprotocol).integer,
                    ),
                );
            } else {
                crate::src::qcommon::q_shared::Info_SetValueForKey(
                    info.as_mut_ptr(),
                    b"protocol\x00" as *const u8 as *const libc::c_char,
                    crate::src::qcommon::q_shared::va(
                        b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        (*crate::src::qcommon::common::com_protocol).integer,
                    ),
                );
            }
            crate::src::qcommon::q_shared::Info_SetValueForKey(
                info.as_mut_ptr(),
                b"qport\x00" as *const u8 as *const libc::c_char,
                crate::src::qcommon::q_shared::va(
                    b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    port,
                ),
            );
            crate::src::qcommon::q_shared::Info_SetValueForKey(
                info.as_mut_ptr(),
                b"challenge\x00" as *const u8 as *const libc::c_char,
                crate::src::qcommon::q_shared::va(
                    b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    clc.challenge,
                ),
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                data.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1034]>() as libc::c_ulong as libc::c_int,
                b"connect \"%s\"\x00" as *const u8 as *const libc::c_char,
                info.as_mut_ptr(),
            );
            crate::src::qcommon::net_chan::NET_OutOfBandData(
                crate::qcommon_h::NS_CLIENT,
                clc.serverAddress as crate::qcommon_h::netadr_t,
                data.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
                crate::stdlib::strlen(data.as_mut_ptr()) as libc::c_int,
            );
            // the most current userinfo has been sent, so watch for any
            // newer changes to userinfo variables
            crate::src::qcommon::cvar::cvar_modifiedFlags &= !(0x2 as libc::c_int)
        }
        _ => {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                b"CL_CheckForResend: bad clc.state\x00" as *const u8 as *const libc::c_char,
            );
        }
    };
}
/*
===================
CL_MotdPacket

===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_MotdPacket(mut from: crate::qcommon_h::netadr_t) {
    let mut challenge: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut info: *mut libc::c_char = 0 as *mut libc::c_char;
    // if not from our server, ignore it
    if crate::src::qcommon::net_ip::NET_CompareAdr(
        from as crate::qcommon_h::netadr_t,
        cls.updateServer as crate::qcommon_h::netadr_t,
    ) as u64
        == 0
    {
        return;
    }
    info = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int);
    // check challenge
    challenge = crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"challenge\x00" as *const u8 as *const libc::c_char,
    );
    if ::libc::strcmp(challenge, cls.updateChallenge.as_mut_ptr()) != 0 {
        return;
    }
    challenge = crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"motd\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        cls.updateInfoString.as_mut_ptr(),
        info,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_motdString\x00" as *const u8 as *const libc::c_char,
        challenge,
    );
}
/*
===================
CL_InitServerInfo
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_InitServerInfo(
    mut server: *mut crate::client_h::serverInfo_t,
    mut address: *mut crate::qcommon_h::netadr_t,
) {
    (*server).adr = *address;
    (*server).clients = 0 as libc::c_int;
    (*server).hostName[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    (*server).mapName[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    (*server).maxClients = 0 as libc::c_int;
    (*server).maxPing = 0 as libc::c_int;
    (*server).minPing = 0 as libc::c_int;
    (*server).ping = -(1 as libc::c_int);
    (*server).game[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    (*server).gameType = 0 as libc::c_int;
    (*server).netType = 0 as libc::c_int;
    (*server).punkbuster = 0 as libc::c_int;
    (*server).g_humanplayers = 0 as libc::c_int;
    (*server).g_needpass = 0 as libc::c_int;
}
/*
===================
CL_ServersResponsePacket
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ServersResponsePacket(
    mut from: *const crate::qcommon_h::netadr_t,
    mut msg: *mut crate::qcommon_h::msg_t,
    mut extended: crate::src::qcommon::q_shared::qboolean,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut addresses: [crate::qcommon_h::netadr_t; 256] = [crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    }; 256];
    let mut numservers: libc::c_int = 0;
    let mut buffptr: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut buffend: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    crate::src::qcommon::common::Com_Printf(
        b"CL_ServersResponsePacket from %s\n\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::net_ip::NET_AdrToStringwPort(*from as crate::qcommon_h::netadr_t),
    );
    if cls.numglobalservers == -(1 as libc::c_int) {
        // state to detect lack of servers or lack of response
        cls.numglobalservers = 0 as libc::c_int;
        cls.numGlobalServerAddresses = 0 as libc::c_int
    }
    // parse through server response string
    numservers = 0 as libc::c_int;
    buffptr = (*msg).data;
    buffend = buffptr.offset((*msg).cursize as isize);
    // advance to initial token
    while !(*buffptr as libc::c_int == '\\' as i32
        || extended as libc::c_uint != 0 && *buffptr as libc::c_int == '/' as i32)
    {
        buffptr = buffptr.offset(1);
        if !(buffptr < buffend) {
            break;
        }
    }
    while buffptr.offset(1 as libc::c_int as isize) < buffend {
        // IPv4 address
        if *buffptr as libc::c_int == '\\' as i32 {
            buffptr = buffptr.offset(1);
            if (buffend.offset_from(buffptr) as libc::c_long as libc::c_ulong)
                < (::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 4]>()
                    as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                break;
            }
            i = 0 as libc::c_int;
            while (i as libc::c_ulong)
                < ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 4]>() as libc::c_ulong
            {
                let fresh5 = buffptr;
                buffptr = buffptr.offset(1);
                addresses[numservers as usize].ip[i as usize] = *fresh5;
                i += 1
            }
            addresses[numservers as usize].type_0 = crate::qcommon_h::NA_IP
        } else {
            // IPv6 address, if it's an extended response
            if !(extended as libc::c_uint != 0 && *buffptr as libc::c_int == '/' as i32) {
                break;
            }
            buffptr = buffptr.offset(1);
            if (buffend.offset_from(buffptr) as libc::c_long as libc::c_ulong)
                < (::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16]>()
                    as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                break;
            }
            i = 0 as libc::c_int;
            while (i as libc::c_ulong)
                < ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16]>()
                    as libc::c_ulong
            {
                let fresh6 = buffptr;
                buffptr = buffptr.offset(1);
                addresses[numservers as usize].ip6[i as usize] = *fresh6;
                i += 1
            }
            addresses[numservers as usize].type_0 = crate::qcommon_h::NA_IP6;
            addresses[numservers as usize].scope_id = (*from).scope_id
        }
        // parse out port
        let fresh7 = buffptr;
        buffptr = buffptr.offset(1);
        addresses[numservers as usize].port =
            ((*fresh7 as libc::c_int) << 8 as libc::c_int) as libc::c_ushort;
        let fresh8 = buffptr;
        buffptr = buffptr.offset(1);
        addresses[numservers as usize].port = (addresses[numservers as usize].port as libc::c_int
            + *fresh8 as libc::c_int)
            as libc::c_ushort;
        addresses[numservers as usize].port = crate::src::qcommon::q_shared::ShortSwap(
            addresses[numservers as usize].port as libc::c_short,
        ) as libc::c_ushort;
        // syntax check
        if *buffptr as libc::c_int != '\\' as i32 && *buffptr as libc::c_int != '/' as i32 {
            break;
        }
        numservers += 1;
        if numservers >= 256 as libc::c_int {
            break;
        }
    }
    count = cls.numglobalservers;
    i = 0 as libc::c_int;
    while i < numservers && count < 4096 as libc::c_int {
        // build net address
        let mut server: *mut crate::client_h::serverInfo_t =
            &mut *cls.globalServers.as_mut_ptr().offset(count as isize)
                as *mut crate::client_h::serverInfo_t;
        // Tequila: It's possible to have sent many master server requests. Then
        // we may receive many times the same addresses from the master server.
        // We just avoid to add a server if it is still in the global servers list.
        j = 0 as libc::c_int;
        while j < count {
            if crate::src::qcommon::net_ip::NET_CompareAdr(
                cls.globalServers[j as usize].adr as crate::qcommon_h::netadr_t,
                addresses[i as usize] as crate::qcommon_h::netadr_t,
            ) as u64
                != 0
            {
                break;
            }
            j += 1
        }
        if !(j < count) {
            CL_InitServerInfo(server, &mut *addresses.as_mut_ptr().offset(i as isize));
            // advance to next slot
            count += 1
        }
        i += 1
    }
    // if getting the global list
    if count >= 4096 as libc::c_int && cls.numGlobalServerAddresses < 4096 as libc::c_int {
        // if we couldn't store the servers in the main list anymore
        while i < numservers && cls.numGlobalServerAddresses < 4096 as libc::c_int {
            // just store the addresses in an additional list
            let fresh9 = cls.numGlobalServerAddresses;
            cls.numGlobalServerAddresses = cls.numGlobalServerAddresses + 1;
            cls.globalServerAddresses[fresh9 as usize] = addresses[i as usize];
            i += 1
        }
    }
    cls.numglobalservers = count;
    total = count + cls.numGlobalServerAddresses;
    crate::src::qcommon::common::Com_Printf(
        b"%d servers parsed (total %d)\n\x00" as *const u8 as *const libc::c_char,
        numservers,
        total,
    );
}
/*
=================
CL_ConnectionlessPacket

Responses to broadcasts, etc
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ConnectionlessPacket(
    mut from: crate::qcommon_h::netadr_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char; // skip the -1
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut challenge: libc::c_int = 0 as libc::c_int;
    crate::src::qcommon::msg::MSG_BeginReadingOOB(msg as *mut crate::qcommon_h::msg_t);
    crate::src::qcommon::msg::MSG_ReadLong(msg as *mut crate::qcommon_h::msg_t);
    s = crate::src::qcommon::msg::MSG_ReadStringLine(msg as *mut crate::qcommon_h::msg_t);
    crate::src::qcommon::cmd::Cmd_TokenizeString(s);
    c = crate::src::qcommon::cmd::Cmd_Argv(0 as libc::c_int);
    crate::src::qcommon::common::Com_DPrintf(
        b"CL packet %s: %s\n\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::net_ip::NET_AdrToStringwPort(from as crate::qcommon_h::netadr_t),
        c,
    );
    // challenge from the server we are connecting to
    if crate::src::qcommon::q_shared::Q_stricmp(
        c,
        b"challengeResponse\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut strver: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ver: libc::c_int = 0;
        if clc.state as libc::c_uint
            != crate::src::qcommon::q_shared::CA_CONNECTING as libc::c_int as libc::c_uint
        {
            crate::src::qcommon::common::Com_DPrintf(
                b"Unwanted challenge response received. Ignored.\n\x00" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        c = crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int);
        if *c != 0 {
            challenge = atoi(c)
        }
        strver = crate::src::qcommon::cmd::Cmd_Argv(3 as libc::c_int);
        if *strver != 0 {
            ver = atoi(strver);
            if ver != (*crate::src::qcommon::common::com_protocol).integer {
                if (*crate::src::qcommon::common::com_legacyprotocol).integer > 0 as libc::c_int {
                    // Server is ioq3 but has a different protocol than we do.
                    // Fall back to idq3 protocol.
                    clc.compat = crate::src::qcommon::q_shared::qtrue;
                    crate::src::qcommon::common::Com_Printf(b"^3Warning: Server reports protocol version %d, we have %d. Trying legacy protocol %d.\n\x00"
                                   as *const u8 as *const libc::c_char, ver,
                               (*crate::src::qcommon::common::com_protocol).integer,
                               (*crate::src::qcommon::common::com_legacyprotocol).integer);
                } else {
                    crate::src::qcommon::common::Com_Printf(b"^3Warning: Server reports protocol version %d, we have %d. Trying anyways.\n\x00"
                                   as *const u8 as *const libc::c_char, ver,
                               (*crate::src::qcommon::common::com_protocol).integer);
                }
            }
        } else {
            clc.compat = crate::src::qcommon::q_shared::qtrue
        }
        if clc.compat as u64 != 0 {
            if crate::src::qcommon::net_ip::NET_CompareAdr(
                from as crate::qcommon_h::netadr_t,
                clc.serverAddress as crate::qcommon_h::netadr_t,
            ) as u64
                == 0
            {
                // This challenge response is not coming from the expected address.
                // Check whether we have a matching client challenge to prevent
                // connection hi-jacking.
                if *c == 0 || challenge != clc.challenge {
                    crate::src::qcommon::common::Com_DPrintf(
                        b"Challenge response received from unexpected source. Ignored.\n\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    return;
                }
            }
        } else if *c == 0 || challenge != clc.challenge {
            crate::src::qcommon::common::Com_Printf(
                b"Bad challenge for challengeResponse. Ignored.\n\x00" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        // start sending challenge response instead of challenge request packets
        clc.challenge = atoi(crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int));
        clc.state = crate::src::qcommon::q_shared::CA_CHALLENGING;
        clc.connectPacketCount = 0 as libc::c_int;
        clc.connectTime = -(99999 as libc::c_int);
        // take this address as the new server address.  This allows
        // a server proxy to hand off connections to multiple servers
        clc.serverAddress = from;
        crate::src::qcommon::common::Com_DPrintf(
            b"challengeResponse: %d\n\x00" as *const u8 as *const libc::c_char,
            clc.challenge,
        );
        return;
    }
    // server connection
    if crate::src::qcommon::q_shared::Q_stricmp(
        c,
        b"connectResponse\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if clc.state as libc::c_uint
            >= crate::src::qcommon::q_shared::CA_CONNECTED as libc::c_int as libc::c_uint
        {
            crate::src::qcommon::common::Com_Printf(
                b"Dup connect received. Ignored.\n\x00" as *const u8 as *const libc::c_char,
            ); // send first packet immediately
            return;
        }
        if clc.state as libc::c_uint
            != crate::src::qcommon::q_shared::CA_CHALLENGING as libc::c_int as libc::c_uint
        {
            crate::src::qcommon::common::Com_Printf(
                b"connectResponse packet while not connecting. Ignored.\n\x00" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        if crate::src::qcommon::net_ip::NET_CompareAdr(
            from as crate::qcommon_h::netadr_t,
            clc.serverAddress as crate::qcommon_h::netadr_t,
        ) as u64
            == 0
        {
            crate::src::qcommon::common::Com_Printf(
                b"connectResponse from wrong address. Ignored.\n\x00" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        if clc.compat as u64 == 0 {
            c = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int);
            if *c != 0 {
                challenge = atoi(c)
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b"Bad connectResponse received. Ignored.\n\x00" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
            if challenge != clc.challenge {
                crate::src::qcommon::common::Com_Printf(
                    b"ConnectResponse with bad challenge received. Ignored.\n\x00" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
        }
        crate::src::qcommon::net_chan::Netchan_Setup(
            crate::qcommon_h::NS_CLIENT,
            &mut clc.netchan as *mut _ as *mut crate::qcommon_h::netchan_t,
            from as crate::qcommon_h::netadr_t,
            crate::src::qcommon::cvar::Cvar_VariableValue(
                b"net_qport\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_int,
            clc.challenge,
            clc.compat,
        );
        clc.state = crate::src::qcommon::q_shared::CA_CONNECTED;
        clc.lastPacketSentTime = -(9999 as libc::c_int);
        return;
    }
    // server responding to an info broadcast
    if crate::src::qcommon::q_shared::Q_stricmp(
        c,
        b"infoResponse\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        CL_ServerInfoPacket(from, msg);
        return;
    }
    // server responding to a get playerlist
    if crate::src::qcommon::q_shared::Q_stricmp(
        c,
        b"statusResponse\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        CL_ServerStatusResponse(from, msg);
        return;
    }
    // echo request from server
    if crate::src::qcommon::q_shared::Q_stricmp(c, b"echo\x00" as *const u8 as *const libc::c_char)
        == 0
    {
        // NOTE: we may have to add exceptions for auth and update servers
        if crate::src::qcommon::net_ip::NET_CompareAdr(
            from as crate::qcommon_h::netadr_t,
            clc.serverAddress as crate::qcommon_h::netadr_t,
        ) as libc::c_uint
            != 0
            || crate::src::qcommon::net_ip::NET_CompareAdr(
                from as crate::qcommon_h::netadr_t,
                cls.rconAddress as crate::qcommon_h::netadr_t,
            ) as libc::c_uint
                != 0
        {
            crate::src::qcommon::net_chan::NET_OutOfBandPrint(
                crate::qcommon_h::NS_CLIENT,
                from as crate::qcommon_h::netadr_t,
                b"%s\x00" as *const u8 as *const libc::c_char,
                crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
            );
        }
        return;
    }
    // cd check
    if crate::src::qcommon::q_shared::Q_stricmp(
        c,
        b"keyAuthorize\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        // we don't use these now, so dump them on the floor
        return;
    }
    // global MOTD from id
    if crate::src::qcommon::q_shared::Q_stricmp(c, b"motd\x00" as *const u8 as *const libc::c_char)
        == 0
    {
        CL_MotdPacket(from);
        return;
    }
    // echo request from server
    if crate::src::qcommon::q_shared::Q_stricmp(c, b"print\x00" as *const u8 as *const libc::c_char)
        == 0
    {
        // NOTE: we may have to add exceptions for auth and update servers
        if crate::src::qcommon::net_ip::NET_CompareAdr(
            from as crate::qcommon_h::netadr_t,
            clc.serverAddress as crate::qcommon_h::netadr_t,
        ) as libc::c_uint
            != 0
            || crate::src::qcommon::net_ip::NET_CompareAdr(
                from as crate::qcommon_h::netadr_t,
                cls.rconAddress as crate::qcommon_h::netadr_t,
            ) as libc::c_uint
                != 0
        {
            s = crate::src::qcommon::msg::MSG_ReadString(msg as *mut crate::qcommon_h::msg_t);
            crate::src::qcommon::q_shared::Q_strncpyz(
                clc.serverMessage.as_mut_ptr(),
                s,
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::qcommon::common::Com_Printf(
                b"%s\x00" as *const u8 as *const libc::c_char,
                s,
            );
        }
        return;
    }
    // list of servers sent back by a master server (classic)
    if crate::src::qcommon::q_shared::Q_strncmp(
        c,
        b"getserversResponse\x00" as *const u8 as *const libc::c_char,
        18 as libc::c_int,
    ) == 0
    {
        CL_ServersResponsePacket(&mut from, msg, crate::src::qcommon::q_shared::qfalse);
        return;
    }
    // list of servers sent back by a master server (extended)
    if crate::src::qcommon::q_shared::Q_strncmp(
        c,
        b"getserversExtResponse\x00" as *const u8 as *const libc::c_char,
        21 as libc::c_int,
    ) == 0
    {
        CL_ServersResponsePacket(&mut from, msg, crate::src::qcommon::q_shared::qtrue);
        return;
    }
    crate::src::qcommon::common::Com_DPrintf(
        b"Unknown connectionless packet command.\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
CL_PacketEvent

A packet has arrived from the main event loop
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_PacketEvent(
    mut from: crate::qcommon_h::netadr_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) {
    let mut headerBytes: libc::c_int = 0;
    clc.lastPacketTime = cls.realtime;
    if (*msg).cursize >= 4 as libc::c_int
        && *((*msg).data as *mut libc::c_int) == -(1 as libc::c_int)
    {
        CL_ConnectionlessPacket(from, msg);
        return;
    }
    if (clc.state as libc::c_uint)
        < crate::src::qcommon::q_shared::CA_CONNECTED as libc::c_int as libc::c_uint
    {
        return;
        // can't be a valid sequenced packet
    }
    if (*msg).cursize < 4 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"%s: Runt packet\n\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::net_ip::NET_AdrToStringwPort(from as crate::qcommon_h::netadr_t),
        );
        return;
    }
    //
    // packet from server
    //
    if crate::src::qcommon::net_ip::NET_CompareAdr(
        from as crate::qcommon_h::netadr_t,
        clc.netchan.remoteAddress as crate::qcommon_h::netadr_t,
    ) as u64
        == 0
    {
        crate::src::qcommon::common::Com_DPrintf(
            b"%s:sequenced packet without connection\n\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::net_ip::NET_AdrToStringwPort(from as crate::qcommon_h::netadr_t),
        );
        // FIXME: send a client disconnect?
        return;
    }
    if crate::src::client::cl_net_chan::CL_Netchan_Process(
        &mut clc.netchan as *mut _ as *mut crate::qcommon_h::netchan_t,
        msg as *mut crate::qcommon_h::msg_t,
    ) as u64
        == 0
    {
        return;
        // out of order, duplicated, etc
    }
    // the header is different lengths for reliable and unreliable messages
    headerBytes = (*msg).readcount;
    // track the last message received so it can be returned in
    // client messages, allowing the server to detect a dropped
    // gamestate
    clc.serverMessageSequence = *((*msg).data as *mut libc::c_int);
    clc.lastPacketTime = cls.realtime;
    crate::src::client::cl_parse::CL_ParseServerMessage(msg as *mut crate::qcommon_h::msg_t);
    //
    // we don't know if it is ok to save a demo message until
    // after we have parsed the frame
    //
    if clc.demorecording as libc::c_uint != 0 && clc.demowaiting as u64 == 0 {
        CL_WriteDemoMessage(msg, headerBytes);
    };
}
/*
==================
CL_CheckTimeout

==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CheckTimeout() {
    //
    // check timeout
    //
    if (CL_CheckPaused() as u64 == 0 || (*crate::src::qcommon::common::sv_paused).integer == 0)
        && clc.state as libc::c_uint
            >= crate::src::qcommon::q_shared::CA_CONNECTED as libc::c_int as libc::c_uint
        && clc.state as libc::c_uint
            != crate::src::qcommon::q_shared::CA_CINEMATIC as libc::c_int as libc::c_uint
        && (cls.realtime - clc.lastPacketTime) as libc::c_float
            > (*cl_timeout).value * 1000 as libc::c_int as libc::c_float
    {
        cl.timeoutcount += 1;
        if cl.timeoutcount > 5 as libc::c_int {
            // timeoutcount saves debugger
            crate::src::qcommon::common::Com_Printf(
                b"\nServer connection timed out.\n\x00" as *const u8 as *const libc::c_char,
            );
            CL_Disconnect(crate::src::qcommon::q_shared::qtrue);
            return;
        }
    } else {
        cl.timeoutcount = 0 as libc::c_int
    };
}
/*
==================
CL_CheckPaused
Check whether client has been paused.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CheckPaused() -> crate::src::qcommon::q_shared::qboolean {
    // if cl_paused->modified is set, the cvar has only been changed in
    // this frame. Keep paused in this frame to ensure the server doesn't
    // lag behind.
    if (*crate::src::qcommon::common::cl_paused).integer != 0
        || (*crate::src::qcommon::common::cl_paused).modified as libc::c_uint != 0
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
//============================================================================
/*
==================
CL_CheckUserinfo

==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CheckUserinfo() {
    // don't add reliable commands when not yet connected
    if (clc.state as libc::c_uint)
        < crate::src::qcommon::q_shared::CA_CONNECTED as libc::c_int as libc::c_uint
    {
        return;
    }
    // don't overflow the reliable command buffer when paused
    if CL_CheckPaused() as u64 != 0 {
        return;
    }
    // send a reliable userinfo update if needed
    if crate::src::qcommon::cvar::cvar_modifiedFlags & 0x2 as libc::c_int != 0 {
        crate::src::qcommon::cvar::cvar_modifiedFlags &= !(0x2 as libc::c_int);
        CL_AddReliableCommand(
            crate::src::qcommon::q_shared::va(
                b"userinfo \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                crate::src::qcommon::cvar::Cvar_InfoString(0x2 as libc::c_int),
            ),
            crate::src::qcommon::q_shared::qfalse,
        );
    };
}
/*
==================
CL_Frame

==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Frame(mut msec: libc::c_int) {
    if (*crate::src::qcommon::common::com_cl_running).integer == 0 {
        return;
    }
    if !clc.downloadCURLM.is_null() {
        crate::src::client::cl_curl::CL_cURL_PerformDownload();
        // we can't process frames normally when in disconnected
        // download mode since the ui vm expects clc.state to be
        // CA_CONNECTED
        if clc.cURLDisconnected as u64 != 0 {
            cls.realFrametime = msec;
            cls.frametime = msec;
            cls.realtime += cls.frametime;
            crate::src::client::cl_scrn::SCR_UpdateScreen();
            crate::src::client::snd_main::S_Update();
            crate::src::client::cl_console::Con_RunConsole();
            cls.framecount += 1;
            return;
        }
    }
    if cls.cddialog as u64 != 0 {
        // bring up the cd error dialog if needed
        cls.cddialog = crate::src::qcommon::q_shared::qfalse;
        crate::src::qcommon::vm::VM_Call(
            crate::src::client::cl_ui::uivm,
            crate::ui_public_h::UI_SET_ACTIVE_MENU as libc::c_int,
            crate::ui_public_h::UIMENU_NEED_CD as libc::c_int,
        );
    } else if clc.state as libc::c_uint
        == crate::src::qcommon::q_shared::CA_DISCONNECTED as libc::c_int as libc::c_uint
        && crate::src::client::cl_keys::Key_GetCatcher() & 0x2 as libc::c_int == 0
        && (*crate::src::qcommon::common::com_sv_running).integer == 0
        && !crate::src::client::cl_ui::uivm.is_null()
    {
        // if disconnected, bring up the menu
        crate::src::client::snd_main::S_StopAllSounds();
        crate::src::qcommon::vm::VM_Call(
            crate::src::client::cl_ui::uivm,
            crate::ui_public_h::UI_SET_ACTIVE_MENU as libc::c_int,
            crate::ui_public_h::UIMENU_MAIN as libc::c_int,
        );
    }
    // if recording an avi, lock to a fixed fps
    if crate::src::client::cl_avi::CL_VideoRecording() as libc::c_uint != 0
        && (*cl_aviFrameRate).integer != 0
        && msec != 0
    {
        // save the current screen
        if clc.state as libc::c_uint
            == crate::src::qcommon::q_shared::CA_ACTIVE as libc::c_int as libc::c_uint
            || (*cl_forceavidemo).integer != 0
        {
            let mut fps: libc::c_float = if (*cl_aviFrameRate).value
                * (*crate::src::qcommon::common::com_timescale).value
                < 1000.0f32
            {
                ((*cl_aviFrameRate).value) * (*crate::src::qcommon::common::com_timescale).value
            } else {
                1000.0f32
            };
            let mut frameDuration: libc::c_float = (if 1000.0f32 / fps > 1.0f32 {
                (1000.0f32) / fps
            } else {
                1.0f32
            }) + clc.aviVideoFrameRemainder;
            crate::src::client::cl_avi::CL_TakeVideoFrame();
            msec = frameDuration as libc::c_int;
            clc.aviVideoFrameRemainder = frameDuration - msec as libc::c_float
        }
    }
    if (*cl_autoRecordDemo).integer != 0 {
        if clc.state as libc::c_uint
            == crate::src::qcommon::q_shared::CA_ACTIVE as libc::c_int as libc::c_uint
            && clc.demorecording as u64 == 0
            && clc.demoplaying as u64 == 0
        {
            // If not recording a demo, and we should be, start one
            let mut now: crate::src::qcommon::q_shared::qtime_t =
                crate::src::qcommon::q_shared::qtime_t {
                    tm_sec: 0,
                    tm_min: 0,
                    tm_hour: 0,
                    tm_mday: 0,
                    tm_mon: 0,
                    tm_year: 0,
                    tm_wday: 0,
                    tm_yday: 0,
                    tm_isdst: 0,
                };
            let mut nowString: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut mapName: [libc::c_char; 64] = [0; 64];
            let mut serverName: [libc::c_char; 4096] = [0; 4096];
            crate::src::qcommon::common::Com_RealTime(
                &mut now as *mut _ as *mut crate::src::qcommon::q_shared::qtime_s,
            );
            nowString = crate::src::qcommon::q_shared::va(
                b"%04d%02d%02d%02d%02d%02d\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                1900 as libc::c_int + now.tm_year,
                1 as libc::c_int + now.tm_mon,
                now.tm_mday,
                now.tm_hour,
                now.tm_min,
                now.tm_sec,
            );
            crate::src::qcommon::q_shared::Q_strncpyz(
                serverName.as_mut_ptr(),
                clc.servername.as_mut_ptr(),
                4096 as libc::c_int,
            );
            // Replace the ":" in the address as it is not a valid
            // file name character
            p = ::libc::strstr(
                serverName.as_mut_ptr(),
                b":\x00" as *const u8 as *const libc::c_char,
            );
            if !p.is_null() {
                *p = '.' as i32 as libc::c_char
            }
            crate::src::qcommon::q_shared::Q_strncpyz(
                mapName.as_mut_ptr(),
                crate::src::qcommon::q_shared::COM_SkipPath(cl.mapname.as_mut_ptr()),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::qcommon::q_shared::COM_StripExtension(
                mapName.as_mut_ptr(),
                mapName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::qcommon::cmd::Cbuf_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_NOW as libc::c_int,
                crate::src::qcommon::q_shared::va(
                    b"record %s-%s-%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    nowString,
                    serverName.as_mut_ptr(),
                    mapName.as_mut_ptr(),
                ),
            );
        } else if clc.state as libc::c_uint
            != crate::src::qcommon::q_shared::CA_ACTIVE as libc::c_int as libc::c_uint
            && clc.demorecording as libc::c_uint != 0
        {
            // Recording, but not CA_ACTIVE, so stop recording
            CL_StopRecord_f();
        }
    }
    // save the msec before checking pause
    cls.realFrametime = msec;
    // decide the simulation time
    cls.frametime = msec;
    cls.realtime += cls.frametime;
    if (*crate::src::client::cl_scrn::cl_timegraph).integer != 0 {
        crate::src::client::cl_scrn::SCR_DebugGraph(
            (cls.realFrametime as libc::c_double * 0.25f64) as libc::c_float,
        );
    }
    // see if we need to update any userinfo
    CL_CheckUserinfo();
    // if we haven't gotten a packet in a long time,
    // drop the connection
    CL_CheckTimeout();
    // send intentions now
    crate::src::client::cl_input::CL_SendCmd();
    // resend a connection request if necessary
    CL_CheckForResend();
    // decide on the serverTime to render
    crate::src::client::cl_cgame::CL_SetCGameTime();
    // update the screen
    crate::src::client::cl_scrn::SCR_UpdateScreen();
    // update audio
    crate::src::client::snd_main::S_Update();
    CL_CaptureVoip();
    CL_UpdateMumble();
    // advance local effects for next frame
    crate::src::client::cl_cin::SCR_RunCinematic();
    crate::src::client::cl_console::Con_RunConsole();
    cls.framecount += 1;
}
//============================================================================
/*
================
CL_RefPrintf

DLL glue
================
*/

unsafe extern "C" fn CL_RefPrintf(
    mut print_level: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut msg: [libc::c_char; 4096] = [0; 4096];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        msg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        fmt,
        argptr.as_va_list(),
    );
    if print_level == crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"%s\x00" as *const u8 as *const libc::c_char,
            msg.as_mut_ptr(),
        );
    } else if print_level == crate::src::qcommon::q_shared::PRINT_WARNING as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"^3%s\x00" as *const u8 as *const libc::c_char,
            msg.as_mut_ptr(),
        );
    // yellow
    } else if print_level == crate::src::qcommon::q_shared::PRINT_DEVELOPER as libc::c_int {
        crate::src::qcommon::common::Com_DPrintf(
            b"^1%s\x00" as *const u8 as *const libc::c_char,
            msg.as_mut_ptr(),
        );
        // red
    };
}
/*
============
CL_ShutdownRef
============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ShutdownRef() {
    if re.Shutdown.is_some() {
        re.Shutdown.expect("non-null function pointer")(crate::src::qcommon::q_shared::qtrue);
    }
    crate::stdlib::memset(
        &mut re as *mut crate::tr_public_h::refexport_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::tr_public_h::refexport_t>() as libc::c_ulong,
    );
    if !rendererLib.is_null() {
        crate::stdlib::SDL_UnloadObject(rendererLib);
        rendererLib = 0 as *mut libc::c_void
    };
}
/*
============
CL_InitRenderer
============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_InitRenderer() {
    // this sets up the renderer and calls R_Init
    re.BeginRegistration.expect("non-null function pointer")(&mut cls.glconfig);
    // load character sets
    cls.charSetShader = re.RegisterShader.expect("non-null function pointer")(
        b"gfx/2d/bigchars\x00" as *const u8 as *const libc::c_char,
    );
    cls.whiteShader = re.RegisterShader.expect("non-null function pointer")(
        b"white\x00" as *const u8 as *const libc::c_char,
    );
    cls.consoleShader = re.RegisterShader.expect("non-null function pointer")(
        b"console\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::client::cl_console::g_console_field_width =
        cls.glconfig.vidWidth / 8 as libc::c_int - 2 as libc::c_int;
    crate::src::client::cl_keys::g_consoleField.widthInChars =
        crate::src::client::cl_console::g_console_field_width;
}
/*
============================
CL_StartHunkUsers

After the server has cleared the hunk, these will need to be restarted
This is the only place that any of these functions are called from
============================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_StartHunkUsers(
    mut rendererOnly: crate::src::qcommon::q_shared::qboolean,
) {
    if crate::src::qcommon::common::com_cl_running.is_null() {
        return;
    }
    if (*crate::src::qcommon::common::com_cl_running).integer == 0 {
        return;
    }
    if cls.rendererStarted as u64 == 0 {
        cls.rendererStarted = crate::src::qcommon::q_shared::qtrue;
        CL_InitRenderer();
    }
    if rendererOnly as u64 != 0 {
        return;
    }
    if cls.soundStarted as u64 == 0 {
        cls.soundStarted = crate::src::qcommon::q_shared::qtrue;
        crate::src::client::snd_main::S_Init();
    }
    if cls.soundRegistered as u64 == 0 {
        cls.soundRegistered = crate::src::qcommon::q_shared::qtrue;
        crate::src::client::snd_main::S_BeginRegistration();
    }
    if (*crate::src::qcommon::common::com_dedicated).integer != 0 {
        return;
    }
    if cls.uiStarted as u64 == 0 {
        cls.uiStarted = crate::src::qcommon::q_shared::qtrue;
        crate::src::client::cl_ui::CL_InitUI();
    };
}
/*
============
CL_RefMalloc
============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_RefMalloc(mut size: libc::c_int) -> *mut libc::c_void {
    return crate::src::qcommon::common::Z_TagMalloc(
        size,
        crate::qcommon_h::TAG_RENDERER as libc::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CL_ScaledMilliseconds() -> libc::c_int {
    return (crate::src::sys::sys_unix::Sys_Milliseconds() as libc::c_float
        * (*crate::src::qcommon::common::com_timescale).value) as libc::c_int;
}
/*
============
CL_InitRef
============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_InitRef() {
    let mut ri: crate::tr_public_h::refimport_t = crate::tr_public_h::refimport_t {
        Printf: None,
        Error: None,
        Milliseconds: None,
        Hunk_Alloc: None,
        Hunk_AllocateTempMemory: None,
        Hunk_FreeTempMemory: None,
        Malloc: None,
        Free: None,
        Cvar_Get: None,
        Cvar_Set: None,
        Cvar_SetValue: None,
        Cvar_CheckRange: None,
        Cvar_SetDescription: None,
        Cvar_VariableIntegerValue: None,
        Cmd_AddCommand: None,
        Cmd_RemoveCommand: None,
        Cmd_Argc: None,
        Cmd_Argv: None,
        Cmd_ExecuteText: None,
        CM_ClusterPVS: None,
        CM_DrawDebugSurface: None,
        FS_FileIsInPAK: None,
        FS_ReadFile: None,
        FS_FreeFile: None,
        FS_ListFiles: None,
        FS_FreeFileList: None,
        FS_WriteFile: None,
        FS_FileExists: None,
        CIN_UploadCinematic: None,
        CIN_PlayCinematic: None,
        CIN_RunCinematic: None,
        CL_WriteAVIVideoFrame: None,
        IN_Init: None,
        IN_Shutdown: None,
        IN_Restart: None,
        ftol: None,
        Sys_SetEnv: None,
        Sys_GLimpSafeInit: None,
        Sys_GLimpInit: None,
        Sys_LowPhysicalMemory: None,
    };
    let mut ret: *mut crate::tr_public_h::refexport_t = 0 as *mut crate::tr_public_h::refexport_t;
    let mut GetRefAPI: crate::tr_public_h::GetRefAPI_t = None;
    let mut dllName: [libc::c_char; 4096] = [0; 4096];
    crate::src::qcommon::common::Com_Printf(
        b"----- Initializing Renderer ----\n\x00" as *const u8 as *const libc::c_char,
    );
    cl_renderer = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_renderer\x00" as *const u8 as *const libc::c_char,
        b"opengl2\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x20 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::qcommon::q_shared::Com_sprintf(
        dllName.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        b"renderer_%s_x86_64.so\x00" as *const u8 as *const libc::c_char,
        (*cl_renderer).string,
    );
    rendererLib = crate::src::sys::sys_main::Sys_LoadDll(
        dllName.as_mut_ptr(),
        crate::src::qcommon::q_shared::qfalse,
    );
    if rendererLib.is_null()
        && ::libc::strcmp((*cl_renderer).string, (*cl_renderer).resetString) != 0
    {
        crate::src::qcommon::common::Com_Printf(
            b"failed:\n\"%s\"\n\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::SDL_GetError(),
        );
        crate::src::qcommon::cvar::Cvar_ForceReset(
            b"cl_renderer\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::q_shared::Com_sprintf(
            dllName.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
            b"renderer_opengl2_x86_64.so\x00" as *const u8 as *const libc::c_char,
        );
        rendererLib = crate::src::sys::sys_main::Sys_LoadDll(
            dllName.as_mut_ptr(),
            crate::src::qcommon::q_shared::qfalse,
        )
    }
    if rendererLib.is_null() {
        crate::src::qcommon::common::Com_Printf(
            b"failed:\n\"%s\"\n\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::SDL_GetError(),
        );
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Failed to load renderer\x00" as *const u8 as *const libc::c_char,
        );
    }
    GetRefAPI = ::std::mem::transmute::<*mut libc::c_void, crate::tr_public_h::GetRefAPI_t>(
        crate::stdlib::SDL_LoadFunction(
            rendererLib,
            b"GetRefAPI\x00" as *const u8 as *const libc::c_char,
        ),
    );
    if GetRefAPI.is_none() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Can\'t load symbol GetRefAPI: \'%s\'\x00" as *const u8 as *const libc::c_char,
            crate::stdlib::SDL_GetError(),
        );
    }
    ri.Cmd_AddCommand = Some(
        crate::src::qcommon::cmd::Cmd_AddCommand
            as unsafe extern "C" fn(_: *const libc::c_char, _: crate::qcommon_h::xcommand_t) -> (),
    );
    ri.Cmd_RemoveCommand = Some(
        crate::src::qcommon::cmd::Cmd_RemoveCommand
            as unsafe extern "C" fn(_: *const libc::c_char) -> (),
    );
    ri.Cmd_Argc = Some(crate::src::qcommon::cmd::Cmd_Argc as unsafe extern "C" fn() -> libc::c_int);
    ri.Cmd_Argv = Some(
        crate::src::qcommon::cmd::Cmd_Argv
            as unsafe extern "C" fn(_: libc::c_int) -> *mut libc::c_char,
    );
    ri.Cmd_ExecuteText = Some(
        crate::src::qcommon::cmd::Cbuf_ExecuteText
            as unsafe extern "C" fn(_: libc::c_int, _: *const libc::c_char) -> (),
    );
    ri.Printf = Some(
        CL_RefPrintf as unsafe extern "C" fn(_: libc::c_int, _: *const libc::c_char, _: ...) -> (),
    );
    ri.Error = Some(
        crate::src::qcommon::common::Com_Error
            as unsafe extern "C" fn(_: libc::c_int, _: *const libc::c_char, _: ...) -> !,
    );
    ri.Milliseconds = Some(CL_ScaledMilliseconds as unsafe extern "C" fn() -> libc::c_int);
    ri.Malloc = Some(CL_RefMalloc as unsafe extern "C" fn(_: libc::c_int) -> *mut libc::c_void);
    ri.Free = Some(
        crate::src::qcommon::common::Z_Free as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
    );
    ri.Hunk_Alloc = Some(
        crate::src::qcommon::common::Hunk_Alloc
            as unsafe extern "C" fn(
                _: libc::c_int,
                _: crate::src::qcommon::q_shared::ha_pref,
            ) -> *mut libc::c_void,
    );
    ri.Hunk_AllocateTempMemory = Some(
        crate::src::qcommon::common::Hunk_AllocateTempMemory
            as unsafe extern "C" fn(_: libc::c_int) -> *mut libc::c_void,
    );
    ri.Hunk_FreeTempMemory = Some(
        crate::src::qcommon::common::Hunk_FreeTempMemory
            as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
    );
    ri.CM_ClusterPVS = Some(
        crate::src::qcommon::cm_test::CM_ClusterPVS
            as unsafe extern "C" fn(_: libc::c_int) -> *mut crate::src::qcommon::q_shared::byte,
    );
    ri.CM_DrawDebugSurface = Some(
        crate::src::qcommon::cm_patch::CM_DrawDebugSurface
            as unsafe extern "C" fn(
                _: Option<
                    unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_int,
                        _: *mut libc::c_float,
                    ) -> (),
                >,
            ) -> (),
    );
    ri.FS_ReadFile = Some(
        crate::src::qcommon::files::FS_ReadFile
            as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *mut *mut libc::c_void,
            ) -> libc::c_long,
    );
    ri.FS_FreeFile = Some(
        crate::src::qcommon::files::FS_FreeFile as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
    );
    ri.FS_WriteFile = Some(
        crate::src::qcommon::files::FS_WriteFile
            as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const libc::c_void,
                _: libc::c_int,
            ) -> (),
    );
    ri.FS_FreeFileList = Some(
        crate::src::qcommon::files::FS_FreeFileList
            as unsafe extern "C" fn(_: *mut *mut libc::c_char) -> (),
    );
    ri.FS_ListFiles = Some(
        crate::src::qcommon::files::FS_ListFiles
            as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const libc::c_char,
                _: *mut libc::c_int,
            ) -> *mut *mut libc::c_char,
    );
    ri.FS_FileIsInPAK = Some(
        crate::src::qcommon::files::FS_FileIsInPAK
            as unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_int) -> libc::c_int,
    );
    ri.FS_FileExists = Some(
        crate::src::qcommon::files::FS_FileExists
            as unsafe extern "C" fn(
                _: *const libc::c_char,
            ) -> crate::src::qcommon::q_shared::qboolean,
    );
    ri.Cvar_Get = Some(
        crate::src::qcommon::cvar::Cvar_Get
            as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const libc::c_char,
                _: libc::c_int,
            ) -> *mut crate::src::qcommon::q_shared::cvar_t,
    );
    ri.Cvar_Set = Some(
        crate::src::qcommon::cvar::Cvar_Set
            as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> (),
    );
    ri.Cvar_SetValue = Some(
        crate::src::qcommon::cvar::Cvar_SetValue
            as unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_float) -> (),
    );
    ri.Cvar_CheckRange = Some(
        crate::src::qcommon::cvar::Cvar_CheckRange
            as unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::cvar_t,
                _: libc::c_float,
                _: libc::c_float,
                _: crate::src::qcommon::q_shared::qboolean,
            ) -> (),
    );
    ri.Cvar_SetDescription = Some(
        crate::src::qcommon::cvar::Cvar_SetDescription
            as unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::cvar_t,
                _: *const libc::c_char,
            ) -> (),
    );
    ri.Cvar_VariableIntegerValue = Some(
        crate::src::qcommon::cvar::Cvar_VariableIntegerValue
            as unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int,
    );
    // cinematic stuff
    ri.CIN_UploadCinematic = Some(
        crate::src::client::cl_cin::CIN_UploadCinematic
            as unsafe extern "C" fn(_: libc::c_int) -> (),
    );
    ri.CIN_PlayCinematic = Some(
        crate::src::client::cl_cin::CIN_PlayCinematic
            as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
            ) -> libc::c_int,
    );
    ri.CIN_RunCinematic = Some(
        crate::src::client::cl_cin::CIN_RunCinematic
            as unsafe extern "C" fn(_: libc::c_int) -> crate::src::qcommon::q_shared::e_status,
    );
    ri.CL_WriteAVIVideoFrame = Some(
        crate::src::client::cl_avi::CL_WriteAVIVideoFrame
            as unsafe extern "C" fn(
                _: *const crate::src::qcommon::q_shared::byte,
                _: libc::c_int,
            ) -> (),
    );
    ri.IN_Init = Some(
        crate::src::sdl::sdl_input::IN_Init as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
    );
    ri.IN_Shutdown = Some(crate::src::sdl::sdl_input::IN_Shutdown as unsafe extern "C" fn() -> ());
    ri.IN_Restart = Some(crate::src::sdl::sdl_input::IN_Restart as unsafe extern "C" fn() -> ());
    ri.ftol = Some(
        crate::src::asm::ftola::qftolsse as unsafe extern "C" fn(_: libc::c_float) -> libc::c_long,
    );
    ri.Sys_SetEnv = Some(
        crate::src::sys::sys_unix::Sys_SetEnv
            as unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> (),
    );
    ri.Sys_GLimpSafeInit =
        Some(crate::src::sys::sys_unix::Sys_GLimpSafeInit as unsafe extern "C" fn() -> ());
    ri.Sys_GLimpInit =
        Some(crate::src::sys::sys_unix::Sys_GLimpInit as unsafe extern "C" fn() -> ());
    ri.Sys_LowPhysicalMemory = Some(
        crate::src::sys::sys_unix::Sys_LowPhysicalMemory
            as unsafe extern "C" fn() -> crate::src::qcommon::q_shared::qboolean,
    );
    ret = GetRefAPI.expect("non-null function pointer")(8 as libc::c_int, &mut ri);
    crate::src::qcommon::common::Com_Printf(
        b"-------------------------------\n\x00" as *const u8 as *const libc::c_char,
    );
    if ret.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Couldn\'t initialize refresh\x00" as *const u8 as *const libc::c_char,
        );
    }
    re = *ret;
    // unpause so the cgame definitely gets a snapshot and renders a frame
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_paused\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
}
//===========================================================================================
#[no_mangle]

pub unsafe extern "C" fn CL_SetModel_f() {
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: [libc::c_char; 256] = [0; 256];
    arg = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int);
    if *arg.offset(0 as libc::c_int as isize) != 0 {
        crate::src::qcommon::cvar::Cvar_Set(b"model\x00" as *const u8 as *const libc::c_char, arg);
        crate::src::qcommon::cvar::Cvar_Set(
            b"headmodel\x00" as *const u8 as *const libc::c_char,
            arg,
        );
    } else {
        crate::src::qcommon::cvar::Cvar_VariableStringBuffer(
            b"model\x00" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        crate::src::qcommon::common::Com_Printf(
            b"model is set to %s\n\x00" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
        );
    };
}
//===========================================================================================
/*
===============
CL_Video_f

video
video [filename]
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Video_f() {
    let mut filename: [libc::c_char; 4096] = [0; 4096];
    let mut i: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    if clc.demoplaying as u64 == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"The video command can only be used when playing back demos\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if crate::src::qcommon::cmd::Cmd_Argc() == 2 as libc::c_int {
        // explicit filename
        crate::src::qcommon::q_shared::Com_sprintf(
            filename.as_mut_ptr(),
            4096 as libc::c_int,
            b"videos/%s.avi\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
        );
    } else {
        // scan for a free filename
        i = 0 as libc::c_int;
        while i <= 9999 as libc::c_int {
            let mut a: libc::c_int = 0;
            let mut b: libc::c_int = 0;
            let mut c: libc::c_int = 0;
            let mut d: libc::c_int = 0;
            last = i;
            a = last / 1000 as libc::c_int;
            last -= a * 1000 as libc::c_int;
            b = last / 100 as libc::c_int;
            last -= b * 100 as libc::c_int;
            c = last / 10 as libc::c_int;
            last -= c * 10 as libc::c_int;
            d = last;
            crate::src::qcommon::q_shared::Com_sprintf(
                filename.as_mut_ptr(),
                4096 as libc::c_int,
                b"videos/video%d%d%d%d.avi\x00" as *const u8 as *const libc::c_char,
                a,
                b,
                c,
                d,
            );
            if crate::src::qcommon::files::FS_FileExists(filename.as_mut_ptr()) as u64 == 0 {
                break;
            }
            i += 1
            // file doesn't exist
        }
        if i > 9999 as libc::c_int {
            crate::src::qcommon::common::Com_Printf(
                b"^1ERROR: no free file names to create video\n\x00" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
    }
    crate::src::client::cl_avi::CL_OpenAVIForWriting(filename.as_mut_ptr());
}
/*
===============
CL_StopVideo_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_StopVideo_f() {
    crate::src::client::cl_avi::CL_CloseAVI();
}
/*
===============
CL_GenerateQKey

test to see if a valid QKEY_FILE exists.  If one does not, try to generate
it by filling it with 2048 bytes of random data.
===============
*/

unsafe extern "C" fn CL_GenerateQKey() {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut buff: [libc::c_uchar; 2048] = [0; 2048];
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    len = crate::src::qcommon::files::FS_SV_FOpenFileRead(
        b"qkey\x00" as *const u8 as *const libc::c_char,
        &mut f,
    ) as libc::c_int;
    crate::src::qcommon::files::FS_FCloseFile(f);
    if len == 2048 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"QKEY found.\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    } else {
        if len > 0 as libc::c_int {
            crate::src::qcommon::common::Com_Printf(
                b"QKEY file size != %d, regenerating\n\x00" as *const u8 as *const libc::c_char,
                2048 as libc::c_int,
            );
        }
        crate::src::qcommon::common::Com_Printf(
            b"QKEY building random string\n\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::common::Com_RandomBytes(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; 2048]>() as libc::c_ulong as libc::c_int,
        );
        f = crate::src::qcommon::files::FS_SV_FOpenFileWrite(
            b"qkey\x00" as *const u8 as *const libc::c_char,
        );
        if f == 0 {
            crate::src::qcommon::common::Com_Printf(
                b"QKEY could not open %s for write\n\x00" as *const u8 as *const libc::c_char,
                b"qkey\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        crate::src::qcommon::files::FS_Write(
            buff.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 2048]>() as libc::c_ulong as libc::c_int,
            f,
        );
        crate::src::qcommon::files::FS_FCloseFile(f);
        crate::src::qcommon::common::Com_Printf(
            b"QKEY generated\n\x00" as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn CL_Sayto_f() {
    let mut rawname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut cleanName: [libc::c_char; 32] = [0; 32];
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut clientNum: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if crate::src::qcommon::cmd::Cmd_Argc() < 3 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"sayto <player name> <text>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    rawname = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int);
    crate::src::qcommon::common::Com_FieldStringToPlayerName(
        name.as_mut_ptr(),
        32 as libc::c_int,
        rawname,
    );
    info = cl
        .gameState
        .stringData
        .as_mut_ptr()
        .offset(cl.gameState.stringOffsets[0 as libc::c_int as usize] as isize);
    count = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
    ));
    clientNum = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < count {
        info = cl.gameState.stringData.as_mut_ptr().offset(
            cl.gameState.stringOffsets
                [(32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + i) as usize]
                as isize,
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            cleanName.as_mut_ptr(),
            crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"n\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        );
        crate::src::qcommon::q_shared::Q_CleanStr(cleanName.as_mut_ptr());
        if crate::src::qcommon::q_shared::Q_stricmp(cleanName.as_mut_ptr(), name.as_mut_ptr()) == 0
        {
            clientNum = i;
            break;
        } else {
            i += 1
        }
    }
    if clientNum <= -(1 as libc::c_int) {
        crate::src::qcommon::common::Com_Printf(
            b"No such player name: %s.\n\x00" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
        );
        return;
    }
    p = crate::src::qcommon::cmd::Cmd_ArgsFrom(2 as libc::c_int);
    if *p as libc::c_int == '\"' as i32 {
        p = p.offset(1);
        *p.offset(
            crate::stdlib::strlen(p).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 0 as libc::c_int as libc::c_char
    }
    CL_AddReliableCommand(
        crate::src::qcommon::q_shared::va(
            b"tell %i \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            clientNum,
            p,
        ),
        crate::src::qcommon::q_shared::qfalse,
    );
}
/*
====================
CL_Init
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Init() {
    crate::src::qcommon::common::Com_Printf(
        b"----- Client Initialization -----\n\x00" as *const u8 as *const libc::c_char,
    ); // no longer CA_UNINITIALIZED
    crate::src::client::cl_console::Con_Init();
    if crate::src::qcommon::common::com_fullyInitialized as u64 == 0 {
        CL_ClearState();
        clc.state = crate::src::qcommon::q_shared::CA_DISCONNECTED;
        cl_oldGameSet = crate::src::qcommon::q_shared::qfalse
    }
    cls.realtime = 0 as libc::c_int;
    crate::src::client::cl_input::CL_InitInput();
    //
    // register our variables
    //
    cl_noprint = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_noprint\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_motd = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_motd\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_timeout = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_timeout\x00" as *const u8 as *const libc::c_char,
        b"200\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_timeNudge = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_timeNudge\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_shownet = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_shownet\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_showSend = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_showSend\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_showTimeDelta = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_showTimeDelta\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_freezeDemo = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_freezeDemo\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    rcon_client_password = crate::src::qcommon::cvar::Cvar_Get(
        b"rconPassword\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_activeAction = crate::src::qcommon::cvar::Cvar_Get(
        b"activeAction\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_timedemo = crate::src::qcommon::cvar::Cvar_Get(
        b"timedemo\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_timedemoLog = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_timedemoLog\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_autoRecordDemo = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_autoRecordDemo\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_aviFrameRate = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_aviFrameRate\x00" as *const u8 as *const libc::c_char,
        b"25\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_aviMotionJpeg = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_aviMotionJpeg\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_forceavidemo = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_forceavidemo\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    rconAddress = crate::src::qcommon::cvar::Cvar_Get(
        b"rconAddress\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::client::cl_input::cl_yawspeed = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_yawspeed\x00" as *const u8 as *const libc::c_char,
        b"140\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::client::cl_input::cl_pitchspeed = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_pitchspeed\x00" as *const u8 as *const libc::c_char,
        b"140\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    )
        as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::client::cl_input::cl_anglespeedkey = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_anglespeedkey\x00" as *const u8 as *const libc::c_char,
        b"1.5\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    )
        as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_maxpackets = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_maxpackets\x00" as *const u8 as *const libc::c_char,
        b"30\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_packetdup = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_packetdup\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::client::cl_input::cl_run = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_run\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_sensitivity = crate::src::qcommon::cvar::Cvar_Get(
        b"sensitivity\x00" as *const u8 as *const libc::c_char,
        b"5\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_mouseAccel = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_mouseAccel\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_freelook = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_freelook\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    // 0: legacy mouse acceleration
    // 1: new implementation
    cl_mouseAccelStyle = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_mouseAccelStyle\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    // offset for the power function (for style 1, ignored otherwise)
    // this should be set to the max rate value
    cl_mouseAccelOffset = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_mouseAccelOffset\x00" as *const u8 as *const libc::c_char,
        b"5\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::qcommon::cvar::Cvar_CheckRange(
        cl_mouseAccelOffset as *mut crate::src::qcommon::q_shared::cvar_s,
        0.001f32,
        50000.0f32,
        crate::src::qcommon::q_shared::qfalse,
    );
    cl_showMouseRate = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_showmouserate\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_allowDownload = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_allowDownload\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::client::cl_curl::cl_cURLLib = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_cURLLib\x00" as *const u8 as *const libc::c_char,
        b"libcurl.so.4\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x2000 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_conXOffset = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_conXOffset\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_inGameVideo = crate::src::qcommon::cvar::Cvar_Get(
        b"r_inGameVideo\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_serverStatusResendTime = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_serverStatusResendTime\x00" as *const u8 as *const libc::c_char,
        b"750\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    // init autoswitch so the ui will have it correctly even
    // if the cgame hasn't been started

    crate::src::qcommon::cvar::Cvar_Get(
        b"cg_autoswitch\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    m_pitch = crate::src::qcommon::cvar::Cvar_Get(
        b"m_pitch\x00" as *const u8 as *const libc::c_char,
        b"0.022\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    m_yaw = crate::src::qcommon::cvar::Cvar_Get(
        b"m_yaw\x00" as *const u8 as *const libc::c_char,
        b"0.022\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    m_forward = crate::src::qcommon::cvar::Cvar_Get(
        b"m_forward\x00" as *const u8 as *const libc::c_char,
        b"0.25\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    m_side = crate::src::qcommon::cvar::Cvar_Get(
        b"m_side\x00" as *const u8 as *const libc::c_char,
        b"0.25\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    m_filter = crate::src::qcommon::cvar::Cvar_Get(
        b"m_filter\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    j_pitch = crate::src::qcommon::cvar::Cvar_Get(
        b"j_pitch\x00" as *const u8 as *const libc::c_char,
        b"0.022\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    j_yaw = crate::src::qcommon::cvar::Cvar_Get(
        b"j_yaw\x00" as *const u8 as *const libc::c_char,
        b"-0.022\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    j_forward = crate::src::qcommon::cvar::Cvar_Get(
        b"j_forward\x00" as *const u8 as *const libc::c_char,
        b"-0.25\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    j_side = crate::src::qcommon::cvar::Cvar_Get(
        b"j_side\x00" as *const u8 as *const libc::c_char,
        b"0.25\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    j_up = crate::src::qcommon::cvar::Cvar_Get(
        b"j_up\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    j_pitch_axis = crate::src::qcommon::cvar::Cvar_Get(
        b"j_pitch_axis\x00" as *const u8 as *const libc::c_char,
        b"3\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    j_yaw_axis = crate::src::qcommon::cvar::Cvar_Get(
        b"j_yaw_axis\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    j_forward_axis = crate::src::qcommon::cvar::Cvar_Get(
        b"j_forward_axis\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    j_side_axis = crate::src::qcommon::cvar::Cvar_Get(
        b"j_side_axis\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    j_up_axis = crate::src::qcommon::cvar::Cvar_Get(
        b"j_up_axis\x00" as *const u8 as *const libc::c_char,
        b"4\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::qcommon::cvar::Cvar_CheckRange(
        j_pitch_axis as *mut crate::src::qcommon::q_shared::cvar_s,
        0 as libc::c_int as libc::c_float,
        (16 as libc::c_int - 1 as libc::c_int) as libc::c_float,
        crate::src::qcommon::q_shared::qtrue,
    );
    crate::src::qcommon::cvar::Cvar_CheckRange(
        j_yaw_axis as *mut crate::src::qcommon::q_shared::cvar_s,
        0 as libc::c_int as libc::c_float,
        (16 as libc::c_int - 1 as libc::c_int) as libc::c_float,
        crate::src::qcommon::q_shared::qtrue,
    );
    crate::src::qcommon::cvar::Cvar_CheckRange(
        j_forward_axis as *mut crate::src::qcommon::q_shared::cvar_s,
        0 as libc::c_int as libc::c_float,
        (16 as libc::c_int - 1 as libc::c_int) as libc::c_float,
        crate::src::qcommon::q_shared::qtrue,
    );
    crate::src::qcommon::cvar::Cvar_CheckRange(
        j_side_axis as *mut crate::src::qcommon::q_shared::cvar_s,
        0 as libc::c_int as libc::c_float,
        (16 as libc::c_int - 1 as libc::c_int) as libc::c_float,
        crate::src::qcommon::q_shared::qtrue,
    );
    crate::src::qcommon::cvar::Cvar_CheckRange(
        j_up_axis as *mut crate::src::qcommon::q_shared::cvar_s,
        0 as libc::c_int as libc::c_float,
        (16 as libc::c_int - 1 as libc::c_int) as libc::c_float,
        crate::src::qcommon::q_shared::qtrue,
    );
    cl_motdString = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_motdString\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"cl_maxPing\x00" as *const u8 as *const libc::c_char,
        b"800\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_lanForcePackets = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_lanForcePackets\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_guidServerUniq = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_guidServerUniq\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    // ~ and `, as keys and characters
    cl_consoleKeys = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_consoleKeys\x00" as *const u8 as *const libc::c_char,
        b"~ ` 0x7e 0x60\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    // userinfo

    crate::src::qcommon::cvar::Cvar_Get(
        b"name\x00" as *const u8 as *const libc::c_char,
        b"UnnamedPlayer\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_rate = crate::src::qcommon::cvar::Cvar_Get(
        b"rate\x00" as *const u8 as *const libc::c_char,
        b"25000\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"snaps\x00" as *const u8 as *const libc::c_char,
        b"20\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"model\x00" as *const u8 as *const libc::c_char,
        b"sarge\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"headmodel\x00" as *const u8 as *const libc::c_char,
        b"sarge\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"team_model\x00" as *const u8 as *const libc::c_char,
        b"james\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"team_headmodel\x00" as *const u8 as *const libc::c_char,
        b"*james\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"g_redTeam\x00" as *const u8 as *const libc::c_char,
        b"Stroggs\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"g_blueTeam\x00" as *const u8 as *const libc::c_char,
        b"Pagans\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"color1\x00" as *const u8 as *const libc::c_char,
        b"4\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"color2\x00" as *const u8 as *const libc::c_char,
        b"5\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"handicap\x00" as *const u8 as *const libc::c_char,
        b"100\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"teamtask\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"sex\x00" as *const u8 as *const libc::c_char,
        b"male\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"cl_anonymous\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"password\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;

    crate::src::qcommon::cvar::Cvar_Get(
        b"cg_predictItems\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_useMumble = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_useMumble\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x20 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_mumbleScale = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_mumbleScale\x00" as *const u8 as *const libc::c_char,
        b"0.0254\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_voipSend = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_voipSendTarget = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_voipSendTarget\x00" as *const u8 as *const libc::c_char,
        b"spatial\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_voipGainDuringCapture = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_voipGainDuringCapture\x00" as *const u8 as *const libc::c_char,
        b"0.2\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_voipCaptureMult = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_voipCaptureMult\x00" as *const u8 as *const libc::c_char,
        b"2.0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_voipUseVAD = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_voipUseVAD\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_voipVADThreshold = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_voipVADThreshold\x00" as *const u8 as *const libc::c_char,
        b"0.25\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_voipShowMeter = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_voipShowMeter\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    cl_voip = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_voip\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::qcommon::cvar::Cvar_CheckRange(
        cl_voip as *mut crate::src::qcommon::q_shared::cvar_s,
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        crate::src::qcommon::q_shared::qtrue,
    );
    cl_voipProtocol = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_voipProtocol\x00" as *const u8 as *const libc::c_char,
        if (*cl_voip).integer != 0 {
            b"opus\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        0x2 as libc::c_int | 0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    // cgame might not be initialized before menu is used

    crate::src::qcommon::cvar::Cvar_Get(
        b"cg_viewsize\x00" as *const u8 as *const libc::c_char,
        b"100\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    // Make sure cg_stereoSeparation is zero as that variable is deprecated and should not be used anymore.

    crate::src::qcommon::cvar::Cvar_Get(
        b"cg_stereoSeparation\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    //
    // register our commands
    //
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"cmd\x00" as *const u8 as *const libc::c_char,
        Some(CL_ForwardToServer_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"configstrings\x00" as *const u8 as *const libc::c_char,
        Some(CL_Configstrings_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"clientinfo\x00" as *const u8 as *const libc::c_char,
        Some(CL_Clientinfo_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"snd_restart\x00" as *const u8 as *const libc::c_char,
        Some(CL_Snd_Restart_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"vid_restart\x00" as *const u8 as *const libc::c_char,
        Some(CL_Vid_Restart_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"disconnect\x00" as *const u8 as *const libc::c_char,
        Some(CL_Disconnect_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"record\x00" as *const u8 as *const libc::c_char,
        Some(CL_Record_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"demo\x00" as *const u8 as *const libc::c_char,
        Some(CL_PlayDemo_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"demo\x00" as *const u8 as *const libc::c_char,
        Some(
            CL_CompleteDemoName as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> (),
        ),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"cinematic\x00" as *const u8 as *const libc::c_char,
        Some(crate::src::client::cl_cin::CL_PlayCinematic_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"stoprecord\x00" as *const u8 as *const libc::c_char,
        Some(CL_StopRecord_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"connect\x00" as *const u8 as *const libc::c_char,
        Some(CL_Connect_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"reconnect\x00" as *const u8 as *const libc::c_char,
        Some(CL_Reconnect_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"localservers\x00" as *const u8 as *const libc::c_char,
        Some(CL_LocalServers_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"globalservers\x00" as *const u8 as *const libc::c_char,
        Some(CL_GlobalServers_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"rcon\x00" as *const u8 as *const libc::c_char,
        Some(CL_Rcon_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"rcon\x00" as *const u8 as *const libc::c_char,
        Some(CL_CompleteRcon as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"ping\x00" as *const u8 as *const libc::c_char,
        Some(CL_Ping_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"serverstatus\x00" as *const u8 as *const libc::c_char,
        Some(CL_ServerStatus_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"showip\x00" as *const u8 as *const libc::c_char,
        Some(CL_ShowIP_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"fs_openedList\x00" as *const u8 as *const libc::c_char,
        Some(CL_OpenedPK3List_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"fs_referencedList\x00" as *const u8 as *const libc::c_char,
        Some(CL_ReferencedPK3List_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"model\x00" as *const u8 as *const libc::c_char,
        Some(CL_SetModel_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"video\x00" as *const u8 as *const libc::c_char,
        Some(CL_Video_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"stopvideo\x00" as *const u8 as *const libc::c_char,
        Some(CL_StopVideo_f as unsafe extern "C" fn() -> ()),
    );
    if (*crate::src::qcommon::common::com_dedicated).integer == 0 {
        crate::src::qcommon::cmd::Cmd_AddCommand(
            b"sayto\x00" as *const u8 as *const libc::c_char,
            Some(CL_Sayto_f as unsafe extern "C" fn() -> ()),
        );
        crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
            b"sayto\x00" as *const u8 as *const libc::c_char,
            Some(
                CL_CompletePlayerName
                    as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> (),
            ),
        );
    }
    CL_InitRef();
    crate::src::client::cl_scrn::SCR_Init();
    //	Cbuf_Execute ();
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_running\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
    );
    CL_GenerateQKey();

    crate::src::qcommon::cvar::Cvar_Get(
        b"cl_guid\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x40 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    CL_UpdateGUID(0 as *const libc::c_char, 0 as libc::c_int);
    crate::src::qcommon::common::Com_Printf(
        b"----- Client Initialization Complete -----\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
===============
CL_Shutdown

===============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Shutdown(
    mut finalmsg: *mut libc::c_char,
    mut disconnect: crate::src::qcommon::q_shared::qboolean,
    mut quit: crate::src::qcommon::q_shared::qboolean,
) {
    static mut recursive: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    // check whether the client is running at all.
    if !(!crate::src::qcommon::common::com_cl_running.is_null()
        && (*crate::src::qcommon::common::com_cl_running).integer != 0)
    {
        return;
    }
    crate::src::qcommon::common::Com_Printf(
        b"----- Client Shutdown (%s) -----\n\x00" as *const u8 as *const libc::c_char,
        finalmsg,
    );
    if recursive as u64 != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: Recursive shutdown\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    recursive = crate::src::qcommon::q_shared::qtrue;
    noGameRestart = quit as libc::c_int;
    if disconnect as u64 != 0 {
        CL_Disconnect(crate::src::qcommon::q_shared::qtrue);
    }
    CL_ClearMemory(crate::src::qcommon::q_shared::qtrue);
    CL_Snd_Shutdown();
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"cmd\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"configstrings\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"clientinfo\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"snd_restart\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"vid_restart\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"disconnect\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"record\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"demo\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"cinematic\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"stoprecord\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"connect\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"reconnect\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"localservers\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"globalservers\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"rcon\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"ping\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"serverstatus\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"showip\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"fs_openedList\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"fs_referencedList\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"model\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"video\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"stopvideo\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::client::cl_input::CL_ShutdownInput();
    crate::src::client::cl_console::Con_Shutdown();
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_running\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    recursive = crate::src::qcommon::q_shared::qfalse;
    crate::stdlib::memset(
        &mut cls as *mut crate::client_h::clientStatic_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::client_h::clientStatic_t>() as libc::c_ulong,
    );
    crate::src::client::cl_keys::Key_SetCatcher(0 as libc::c_int);
    crate::src::qcommon::common::Com_Printf(
        b"-----------------------\n\x00" as *const u8 as *const libc::c_char,
    );
}

unsafe extern "C" fn CL_SetServerInfo(
    mut server: *mut crate::client_h::serverInfo_t,
    mut info: *const libc::c_char,
    mut ping: libc::c_int,
) {
    if !server.is_null() {
        if !info.is_null() {
            (*server).clients = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"clients\x00" as *const u8 as *const libc::c_char,
            ));
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*server).hostName.as_mut_ptr(),
                crate::src::qcommon::q_shared::Info_ValueForKey(
                    info,
                    b"hostname\x00" as *const u8 as *const libc::c_char,
                ),
                32 as libc::c_int,
            );
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*server).mapName.as_mut_ptr(),
                crate::src::qcommon::q_shared::Info_ValueForKey(
                    info,
                    b"mapname\x00" as *const u8 as *const libc::c_char,
                ),
                32 as libc::c_int,
            );
            (*server).maxClients = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
            ));
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*server).game.as_mut_ptr(),
                crate::src::qcommon::q_shared::Info_ValueForKey(
                    info,
                    b"game\x00" as *const u8 as *const libc::c_char,
                ),
                32 as libc::c_int,
            );
            (*server).gameType = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"gametype\x00" as *const u8 as *const libc::c_char,
            ));
            (*server).netType = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"nettype\x00" as *const u8 as *const libc::c_char,
            ));
            (*server).minPing = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"minping\x00" as *const u8 as *const libc::c_char,
            ));
            (*server).maxPing = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"maxping\x00" as *const u8 as *const libc::c_char,
            ));
            (*server).punkbuster = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"punkbuster\x00" as *const u8 as *const libc::c_char,
            ));
            (*server).g_humanplayers = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"g_humanplayers\x00" as *const u8 as *const libc::c_char,
            ));
            (*server).g_needpass = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"g_needpass\x00" as *const u8 as *const libc::c_char,
            ))
        }
        (*server).ping = ping
    };
}

unsafe extern "C" fn CL_SetServerInfoByAddress(
    mut from: crate::qcommon_h::netadr_t,
    mut info: *const libc::c_char,
    mut ping: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        if crate::src::qcommon::net_ip::NET_CompareAdr(
            from as crate::qcommon_h::netadr_t,
            cls.localServers[i as usize].adr as crate::qcommon_h::netadr_t,
        ) as u64
            != 0
        {
            CL_SetServerInfo(
                &mut *cls.localServers.as_mut_ptr().offset(i as isize),
                info,
                ping,
            );
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4096 as libc::c_int {
        if crate::src::qcommon::net_ip::NET_CompareAdr(
            from as crate::qcommon_h::netadr_t,
            cls.globalServers[i as usize].adr as crate::qcommon_h::netadr_t,
        ) as u64
            != 0
        {
            CL_SetServerInfo(
                &mut *cls.globalServers.as_mut_ptr().offset(i as isize),
                info,
                ping,
            );
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        if crate::src::qcommon::net_ip::NET_CompareAdr(
            from as crate::qcommon_h::netadr_t,
            cls.favoriteServers[i as usize].adr as crate::qcommon_h::netadr_t,
        ) as u64
            != 0
        {
            CL_SetServerInfo(
                &mut *cls.favoriteServers.as_mut_ptr().offset(i as isize),
                info,
                ping,
            );
        }
        i += 1
    }
}
/*
===================
CL_ServerInfoPacket
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ServerInfoPacket(
    mut from: crate::qcommon_h::netadr_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) {
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    let mut infoString: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prot: libc::c_int = 0;
    let mut gamename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gameMismatch: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    infoString = crate::src::qcommon::msg::MSG_ReadString(msg as *mut crate::qcommon_h::msg_t);
    // if this isn't the correct gamename, ignore it
    gamename = crate::src::qcommon::q_shared::Info_ValueForKey(
        infoString,
        b"gamename\x00" as *const u8 as *const libc::c_char,
    );
    // gamename is optional for legacy protocol
    if (*crate::src::qcommon::common::com_legacyprotocol).integer != 0 && *gamename == 0 {
        gameMismatch = crate::src::qcommon::q_shared::qfalse
    } else {
        gameMismatch = (*gamename == 0
            || ::libc::strcmp(
                gamename,
                (*crate::src::qcommon::common::com_gamename).string,
            ) != 0 as libc::c_int) as libc::c_int
            as crate::src::qcommon::q_shared::qboolean
    }
    if gameMismatch as u64 != 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"Game mismatch in info packet: %s\n\x00" as *const u8 as *const libc::c_char,
            infoString,
        );
        return;
    }
    // if this isn't the correct protocol version, ignore it
    prot = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        infoString,
        b"protocol\x00" as *const u8 as *const libc::c_char,
    ));
    if prot != (*crate::src::qcommon::common::com_protocol).integer
        && prot != (*crate::src::qcommon::common::com_legacyprotocol).integer
    {
        crate::src::qcommon::common::Com_DPrintf(
            b"Different protocol info packet: %s\n\x00" as *const u8 as *const libc::c_char,
            infoString,
        );
        return;
    }
    // iterate servers waiting for ping response
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if cl_pinglist[i as usize].adr.port as libc::c_int != 0
            && cl_pinglist[i as usize].time == 0
            && crate::src::qcommon::net_ip::NET_CompareAdr(
                from as crate::qcommon_h::netadr_t,
                cl_pinglist[i as usize].adr as crate::qcommon_h::netadr_t,
            ) as libc::c_uint
                != 0
        {
            // calc ping time
            cl_pinglist[i as usize].time =
                crate::src::sys::sys_unix::Sys_Milliseconds() - cl_pinglist[i as usize].start;
            crate::src::qcommon::common::Com_DPrintf(
                b"ping time %dms from %s\n\x00" as *const u8 as *const libc::c_char,
                cl_pinglist[i as usize].time,
                crate::src::qcommon::net_ip::NET_AdrToString(from as crate::qcommon_h::netadr_t),
            );
            // save of info
            crate::src::qcommon::q_shared::Q_strncpyz(
                cl_pinglist[i as usize].info.as_mut_ptr(),
                infoString,
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            );
            // tack on the net type
            // NOTE: make sure these types are in sync with the netnames strings in the UI
            match from.type_0 as libc::c_uint {
                3 | 4 => type_0 = 1 as libc::c_int,
                5 => type_0 = 2 as libc::c_int,
                _ => type_0 = 0 as libc::c_int,
            }
            crate::src::qcommon::q_shared::Info_SetValueForKey(
                cl_pinglist[i as usize].info.as_mut_ptr(),
                b"nettype\x00" as *const u8 as *const libc::c_char,
                crate::src::qcommon::q_shared::va(
                    b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    type_0,
                ),
            );
            CL_SetServerInfoByAddress(from, infoString, cl_pinglist[i as usize].time);
            return;
        }
        i += 1
    }
    // if not just sent a local broadcast or pinging local servers
    if cls.pingUpdateSource != 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        // empty slot
        if cls.localServers[i as usize].adr.port as libc::c_int == 0 as libc::c_int {
            break;
        }
        // avoid duplicate
        if crate::src::qcommon::net_ip::NET_CompareAdr(
            from as crate::qcommon_h::netadr_t,
            cls.localServers[i as usize].adr as crate::qcommon_h::netadr_t,
        ) as u64
            != 0
        {
            return;
        }
        i += 1
    }
    if i == 128 as libc::c_int {
        crate::src::qcommon::common::Com_DPrintf(
            b"MAX_OTHER_SERVERS hit, dropping infoResponse\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    // add this to the list
    cls.numlocalservers = i + 1 as libc::c_int;
    CL_InitServerInfo(
        &mut *cls.localServers.as_mut_ptr().offset(i as isize),
        &mut from,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        info.as_mut_ptr(),
        crate::src::qcommon::msg::MSG_ReadString(msg as *mut crate::qcommon_h::msg_t),
        1024 as libc::c_int,
    );
    if crate::stdlib::strlen(info.as_mut_ptr()) != 0 {
        if info[crate::stdlib::strlen(info.as_mut_ptr())
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] as libc::c_int
            != '\n' as i32
        {
            crate::src::qcommon::q_shared::Q_strcat(
                info.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b"\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        crate::src::qcommon::common::Com_Printf(
            b"%s: %s\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::net_ip::NET_AdrToStringwPort(from as crate::qcommon_h::netadr_t),
            info.as_mut_ptr(),
        );
    };
}
/*
===================
CL_GetServerStatus
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GetServerStatus(
    mut from: crate::qcommon_h::netadr_t,
) -> *mut serverStatus_t {
    let mut i: libc::c_int = 0;
    let mut oldest: libc::c_int = 0;
    let mut oldestTime: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if crate::src::qcommon::net_ip::NET_CompareAdr(
            from as crate::qcommon_h::netadr_t,
            cl_serverStatusList[i as usize].address as crate::qcommon_h::netadr_t,
        ) as u64
            != 0
        {
            return &mut *cl_serverStatusList.as_mut_ptr().offset(i as isize)
                as *mut serverStatus_t;
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if cl_serverStatusList[i as usize].retrieved as u64 != 0 {
            return &mut *cl_serverStatusList.as_mut_ptr().offset(i as isize)
                as *mut serverStatus_t;
        }
        i += 1
    }
    oldest = -(1 as libc::c_int);
    oldestTime = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if oldest == -(1 as libc::c_int) || cl_serverStatusList[i as usize].startTime < oldestTime {
            oldest = i;
            oldestTime = cl_serverStatusList[i as usize].startTime
        }
        i += 1
    }
    return &mut *cl_serverStatusList.as_mut_ptr().offset(oldest as isize) as *mut serverStatus_t;
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
/*
===================
CL_ServerStatus
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ServerStatus(
    mut serverAddress: *mut libc::c_char,
    mut serverStatusString: *mut libc::c_char,
    mut maxLen: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut to: crate::qcommon_h::netadr_t = crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    };
    let mut serverStatus: *mut serverStatus_t = 0 as *mut serverStatus_t;
    // if no server address then reset all server status requests
    if serverAddress.is_null() {
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            cl_serverStatusList[i as usize].address.port = 0 as libc::c_int as libc::c_ushort;
            cl_serverStatusList[i as usize].retrieved = crate::src::qcommon::q_shared::qtrue;
            i += 1
        }
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    // get the address
    if crate::src::qcommon::net_chan::NET_StringToAdr(
        serverAddress,
        &mut to as *mut _ as *mut crate::qcommon_h::netadr_t,
        crate::qcommon_h::NA_UNSPEC,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    serverStatus = CL_GetServerStatus(to);
    // if no server status string then reset the server status request for this address
    if serverStatusString.is_null() {
        (*serverStatus).retrieved = crate::src::qcommon::q_shared::qtrue;
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    // if this server status request has the same address
    if crate::src::qcommon::net_ip::NET_CompareAdr(
        to as crate::qcommon_h::netadr_t,
        (*serverStatus).address as crate::qcommon_h::netadr_t,
    ) as u64
        != 0
    {
        // if we received a response for this server status request
        if (*serverStatus).pending as u64 == 0 {
            crate::src::qcommon::q_shared::Q_strncpyz(
                serverStatusString,
                (*serverStatus).string.as_mut_ptr(),
                maxLen,
            );
            (*serverStatus).retrieved = crate::src::qcommon::q_shared::qtrue;
            (*serverStatus).startTime = 0 as libc::c_int;
            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
        } else {
            // resend the request regularly
            if (*serverStatus).startTime
                < crate::src::qcommon::common::Com_Milliseconds()
                    - (*cl_serverStatusResendTime).integer
            {
                (*serverStatus).print = crate::src::qcommon::q_shared::qfalse;
                (*serverStatus).pending = crate::src::qcommon::q_shared::qtrue;
                (*serverStatus).retrieved = crate::src::qcommon::q_shared::qfalse;
                (*serverStatus).time = 0 as libc::c_int;
                (*serverStatus).startTime = crate::src::qcommon::common::Com_Milliseconds();
                crate::src::qcommon::net_chan::NET_OutOfBandPrint(
                    crate::qcommon_h::NS_CLIENT,
                    to as crate::qcommon_h::netadr_t,
                    b"getstatus\x00" as *const u8 as *const libc::c_char,
                );
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
        }
    } else if (*serverStatus).retrieved as u64 != 0 {
        (*serverStatus).address = to;
        (*serverStatus).print = crate::src::qcommon::q_shared::qfalse;
        (*serverStatus).pending = crate::src::qcommon::q_shared::qtrue;
        (*serverStatus).retrieved = crate::src::qcommon::q_shared::qfalse;
        (*serverStatus).startTime = crate::src::qcommon::common::Com_Milliseconds();
        (*serverStatus).time = 0 as libc::c_int;
        crate::src::qcommon::net_chan::NET_OutOfBandPrint(
            crate::qcommon_h::NS_CLIENT,
            to as crate::qcommon_h::netadr_t,
            b"getstatus\x00" as *const u8 as *const libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
// if retrieved
/*
===================
CL_ServerStatusResponse
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ServerStatusResponse(
    mut from: crate::qcommon_h::netadr_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut ping: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut serverStatus: *mut serverStatus_t = 0 as *mut serverStatus_t;
    serverStatus = 0 as *mut serverStatus_t;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if crate::src::qcommon::net_ip::NET_CompareAdr(
            from as crate::qcommon_h::netadr_t,
            cl_serverStatusList[i as usize].address as crate::qcommon_h::netadr_t,
        ) as u64
            != 0
        {
            serverStatus =
                &mut *cl_serverStatusList.as_mut_ptr().offset(i as isize) as *mut serverStatus_t;
            break;
        } else {
            i += 1
        }
    }
    // if we didn't request this server status
    if serverStatus.is_null() {
        return;
    }
    s = crate::src::qcommon::msg::MSG_ReadStringLine(msg as *mut crate::qcommon_h::msg_t);
    len = 0 as libc::c_int;
    crate::src::qcommon::q_shared::Com_sprintf(
        &mut *(*serverStatus).string.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(len as libc::c_ulong) as libc::c_int,
        b"%s\x00" as *const u8 as *const libc::c_char,
        s,
    );
    if (*serverStatus).print as u64 != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"Server settings:\n\x00" as *const u8 as *const libc::c_char,
        );
        // print cvars
        while *s != 0 {
            i = 0 as libc::c_int;
            while i < 2 as libc::c_int && *s as libc::c_int != 0 {
                if *s as libc::c_int == '\\' as i32 {
                    s = s.offset(1)
                }
                l = 0 as libc::c_int;
                while *s != 0 {
                    let fresh10 = l;
                    l = l + 1;
                    info[fresh10 as usize] = *s;
                    if l >= 1024 as libc::c_int - 1 as libc::c_int {
                        break;
                    }
                    s = s.offset(1);
                    if *s as libc::c_int == '\\' as i32 {
                        break;
                    }
                }
                info[l as usize] = '\u{0}' as i32 as libc::c_char;
                if i != 0 {
                    crate::src::qcommon::common::Com_Printf(
                        b"%s\n\x00" as *const u8 as *const libc::c_char,
                        info.as_mut_ptr(),
                    );
                } else {
                    crate::src::qcommon::common::Com_Printf(
                        b"%-24s\x00" as *const u8 as *const libc::c_char,
                        info.as_mut_ptr(),
                    );
                }
                i += 1
            }
        }
    }
    len = crate::stdlib::strlen((*serverStatus).string.as_mut_ptr()) as libc::c_int;
    crate::src::qcommon::q_shared::Com_sprintf(
        &mut *(*serverStatus).string.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(len as libc::c_ulong) as libc::c_int,
        b"\\\x00" as *const u8 as *const libc::c_char,
    );
    if (*serverStatus).print as u64 != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"\nPlayers:\n\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::common::Com_Printf(
            b"num: score: ping: name:\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    s = crate::src::qcommon::msg::MSG_ReadStringLine(msg as *mut crate::qcommon_h::msg_t);
    while *s != 0 {
        len = crate::stdlib::strlen((*serverStatus).string.as_mut_ptr()) as libc::c_int;
        crate::src::qcommon::q_shared::Com_sprintf(
            &mut *(*serverStatus).string.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
                .wrapping_sub(len as libc::c_ulong) as libc::c_int,
            b"\\%s\x00" as *const u8 as *const libc::c_char,
            s,
        );
        if (*serverStatus).print as u64 != 0 {
            ping = 0 as libc::c_int;
            score = ping;
            ::libc::sscanf(
                s,
                b"%d %d\x00" as *const u8 as *const libc::c_char,
                &mut score as *mut libc::c_int,
                &mut ping as *mut libc::c_int,
            );
            s = ::libc::strchr(s, ' ' as i32);
            if !s.is_null() {
                s = ::libc::strchr(s.offset(1 as libc::c_int as isize), ' ' as i32)
            }
            if !s.is_null() {
                s = s.offset(1)
            } else {
                s = b"unknown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            crate::src::qcommon::common::Com_Printf(
                b"%-2d   %-3d    %-3d   %s\n\x00" as *const u8 as *const libc::c_char,
                i,
                score,
                ping,
                s,
            );
        }
        s = crate::src::qcommon::msg::MSG_ReadStringLine(msg as *mut crate::qcommon_h::msg_t);
        i += 1
    }
    len = crate::stdlib::strlen((*serverStatus).string.as_mut_ptr()) as libc::c_int;
    crate::src::qcommon::q_shared::Com_sprintf(
        &mut *(*serverStatus).string.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(len as libc::c_ulong) as libc::c_int,
        b"\\\x00" as *const u8 as *const libc::c_char,
    );
    (*serverStatus).time = crate::src::qcommon::common::Com_Milliseconds();
    (*serverStatus).address = from;
    (*serverStatus).pending = crate::src::qcommon::q_shared::qfalse;
    if (*serverStatus).print as u64 != 0 {
        (*serverStatus).retrieved = crate::src::qcommon::q_shared::qtrue
    };
}
/*
==================
CL_LocalServers_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_LocalServers_f() {
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut to: crate::qcommon_h::netadr_t = crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    };
    crate::src::qcommon::common::Com_Printf(
        b"Scanning for servers on the local network...\n\x00" as *const u8 as *const libc::c_char,
    );
    // reset the list, waiting for response
    cls.numlocalservers = 0 as libc::c_int;
    cls.pingUpdateSource = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        let mut b: crate::src::qcommon::q_shared::qboolean = cls.localServers[i as usize].visible;
        crate::stdlib::memset(
            &mut *cls.localServers.as_mut_ptr().offset(i as isize)
                as *mut crate::client_h::serverInfo_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::client_h::serverInfo_t>() as libc::c_ulong,
        );
        cls.localServers[i as usize].visible = b;
        i += 1
    }
    crate::stdlib::memset(
        &mut to as *mut crate::qcommon_h::netadr_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::qcommon_h::netadr_t>() as libc::c_ulong,
    );
    // The 'xxx' in the message is a challenge that will be echoed back
    // by the server.  We don't care about that here, but master servers
    // can use that to prevent spoofed server responses from invalid ip
    message =
        b"\xff\xff\xff\xffgetinfo xxx\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    // send each message twice in case one is dropped
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        // send a broadcast packet on each server port
        // we support multiple server ports so a single machine
        // can nicely run multiple servers
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            to.port = crate::src::qcommon::q_shared::ShortSwap(
                (27960 as libc::c_int + j) as libc::c_short,
            ) as libc::c_ushort;
            to.type_0 = crate::qcommon_h::NA_BROADCAST;
            crate::src::qcommon::net_chan::NET_SendPacket(
                crate::qcommon_h::NS_CLIENT,
                crate::stdlib::strlen(message) as libc::c_int,
                message as *const libc::c_void,
                to as crate::qcommon_h::netadr_t,
            );
            to.type_0 = crate::qcommon_h::NA_MULTICAST6;
            crate::src::qcommon::net_chan::NET_SendPacket(
                crate::qcommon_h::NS_CLIENT,
                crate::stdlib::strlen(message) as libc::c_int,
                message as *const libc::c_void,
                to as crate::qcommon_h::netadr_t,
            );
            j += 1
        }
        i += 1
    }
}
/*
==================
CL_GlobalServers_f

Originally master 0 was Internet and master 1 was MPlayer.
ioquake3 2008; added support for requesting five separate master servers using 0-4.
ioquake3 2017; made master 0 fetch all master servers and 1-5 request a single master server.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GlobalServers_f() {
    let mut to: crate::qcommon_h::netadr_t = crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    };
    let mut count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut masterNum: libc::c_int = 0;
    let mut command: [libc::c_char; 1024] = [0; 1024];
    let mut masteraddress: *mut libc::c_char = 0 as *mut libc::c_char;
    count = crate::src::qcommon::cmd::Cmd_Argc();
    if count < 3 as libc::c_int
        || {
            masterNum = atoi(crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int));
            (masterNum) < 0 as libc::c_int
        }
        || masterNum > 5 as libc::c_int
    {
        crate::src::qcommon::common::Com_Printf(
            b"usage: globalservers <master# 0-%d> <protocol> [keywords]\n\x00" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        );
        return;
    }
    // request from all master servers
    if masterNum == 0 as libc::c_int {
        let mut numAddress: libc::c_int = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= 5 as libc::c_int {
            ::libc::sprintf(
                command.as_mut_ptr(),
                b"sv_master%d\x00" as *const u8 as *const libc::c_char,
                i,
            );
            masteraddress = crate::src::qcommon::cvar::Cvar_VariableString(command.as_mut_ptr());
            if !(*masteraddress == 0) {
                numAddress += 1;
                crate::src::qcommon::q_shared::Com_sprintf(
                    command.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                    b"globalservers %d %s %s\n\x00" as *const u8 as *const libc::c_char,
                    i,
                    crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int),
                    crate::src::qcommon::cmd::Cmd_ArgsFrom(3 as libc::c_int),
                );
                crate::src::qcommon::cmd::Cbuf_AddText(command.as_mut_ptr());
            }
            i += 1
        }
        if numAddress == 0 {
            crate::src::qcommon::common::Com_Printf(
                b"CL_GlobalServers_f: Error: No master server addresses.\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        return;
    }
    ::libc::sprintf(
        command.as_mut_ptr(),
        b"sv_master%d\x00" as *const u8 as *const libc::c_char,
        masterNum,
    );
    masteraddress = crate::src::qcommon::cvar::Cvar_VariableString(command.as_mut_ptr());
    if *masteraddress == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"CL_GlobalServers_f: Error: No master server address given.\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    // reset the list, waiting for response
    // -1 is used to distinguish a "no response"
    i = crate::src::qcommon::net_chan::NET_StringToAdr(
        masteraddress,
        &mut to as *mut _ as *mut crate::qcommon_h::netadr_t,
        crate::qcommon_h::NA_UNSPEC,
    );
    if i == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"CL_GlobalServers_f: Error: could not resolve address of master %s\n\x00" as *const u8
                as *const libc::c_char,
            masteraddress,
        );
        return;
    } else {
        if i == 2 as libc::c_int {
            to.port =
                crate::src::qcommon::q_shared::ShortSwap(27950 as libc::c_int as libc::c_short)
                    as libc::c_ushort
        }
    }
    crate::src::qcommon::common::Com_Printf(
        b"Requesting servers from %s (%s)...\n\x00" as *const u8 as *const libc::c_char,
        masteraddress,
        crate::src::qcommon::net_ip::NET_AdrToStringwPort(to as crate::qcommon_h::netadr_t),
    );
    cls.numglobalservers = -(1 as libc::c_int);
    cls.pingUpdateSource = 2 as libc::c_int;
    // Use the extended query for IPv6 masters
    if to.type_0 as libc::c_uint == crate::qcommon_h::NA_IP6 as libc::c_int as libc::c_uint
        || to.type_0 as libc::c_uint
            == crate::qcommon_h::NA_MULTICAST6 as libc::c_int as libc::c_uint
    {
        let mut v4enabled: libc::c_int = crate::src::qcommon::cvar::Cvar_VariableIntegerValue(
            b"net_enabled\x00" as *const u8 as *const libc::c_char,
        ) & 0x1 as libc::c_int;
        if v4enabled != 0 {
            crate::src::qcommon::q_shared::Com_sprintf(
                command.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b"getserversExt %s %s\x00" as *const u8 as *const libc::c_char,
                (*crate::src::qcommon::common::com_gamename).string,
                crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int),
            );
        } else {
            crate::src::qcommon::q_shared::Com_sprintf(
                command.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b"getserversExt %s %s ipv6\x00" as *const u8 as *const libc::c_char,
                (*crate::src::qcommon::common::com_gamename).string,
                crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int),
            );
        }
    } else if crate::src::qcommon::q_shared::Q_stricmp(
        (*crate::src::qcommon::common::com_gamename).string,
        b"Quake3Arena\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        crate::src::qcommon::q_shared::Com_sprintf(
            command.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"getservers %s\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int),
        );
    } else {
        crate::src::qcommon::q_shared::Com_sprintf(
            command.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"getservers %s %s\x00" as *const u8 as *const libc::c_char,
            (*crate::src::qcommon::common::com_gamename).string,
            crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int),
        );
    }
    i = 3 as libc::c_int;
    while i < count {
        crate::src::qcommon::q_shared::Q_strcat(
            command.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b" \x00" as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::q_shared::Q_strcat(
            command.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            crate::src::qcommon::cmd::Cmd_Argv(i),
        );
        i += 1
    }
    crate::src::qcommon::net_chan::NET_OutOfBandPrint(
        crate::qcommon_h::NS_SERVER,
        to as crate::qcommon_h::netadr_t,
        b"%s\x00" as *const u8 as *const libc::c_char,
        command.as_mut_ptr(),
    );
}
/*
==================
CL_GetPing
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GetPing(
    mut n: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buflen: libc::c_int,
    mut pingtime: *mut libc::c_int,
) {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut time: libc::c_int = 0;
    let mut maxPing: libc::c_int = 0;
    if n < 0 as libc::c_int || n >= 32 as libc::c_int || cl_pinglist[n as usize].adr.port == 0 {
        // empty or invalid slot
        *buf.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        *pingtime = 0 as libc::c_int;
        return;
    }
    str = crate::src::qcommon::net_ip::NET_AdrToStringwPort(
        cl_pinglist[n as usize].adr as crate::qcommon_h::netadr_t,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(buf, str, buflen);
    time = cl_pinglist[n as usize].time;
    if time == 0 {
        // check for timeout
        time = crate::src::sys::sys_unix::Sys_Milliseconds() - cl_pinglist[n as usize].start;
        maxPing = crate::src::qcommon::cvar::Cvar_VariableIntegerValue(
            b"cl_maxPing\x00" as *const u8 as *const libc::c_char,
        );
        if maxPing < 100 as libc::c_int {
            maxPing = 100 as libc::c_int
        }
        if time < maxPing {
            // not timed out yet
            time = 0 as libc::c_int
        }
    }
    CL_SetServerInfoByAddress(
        cl_pinglist[n as usize].adr,
        cl_pinglist[n as usize].info.as_mut_ptr(),
        cl_pinglist[n as usize].time,
    );
    *pingtime = time;
}
/*
==================
CL_GetPingInfo
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GetPingInfo(
    mut n: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buflen: libc::c_int,
) {
    if n < 0 as libc::c_int || n >= 32 as libc::c_int || cl_pinglist[n as usize].adr.port == 0 {
        // empty or invalid slot
        if buflen != 0 {
            *buf.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char
        }
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        buf,
        cl_pinglist[n as usize].info.as_mut_ptr(),
        buflen,
    );
}
/*
==================
CL_ClearPing
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ClearPing(mut n: libc::c_int) {
    if n < 0 as libc::c_int || n >= 32 as libc::c_int {
        return;
    }
    cl_pinglist[n as usize].adr.port = 0 as libc::c_int as libc::c_ushort;
}
/*
==================
CL_GetPingQueueCount
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GetPingQueueCount() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut pingptr: *mut crate::client_h::ping_t = 0 as *mut crate::client_h::ping_t;
    count = 0 as libc::c_int;
    pingptr = cl_pinglist.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (*pingptr).adr.port != 0 {
            count += 1
        }
        i += 1;
        pingptr = pingptr.offset(1)
    }
    return count;
}
/*
==================
CL_GetFreePing
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_GetFreePing() -> *mut crate::client_h::ping_t {
    let mut pingptr: *mut crate::client_h::ping_t = 0 as *mut crate::client_h::ping_t;
    let mut best: *mut crate::client_h::ping_t = 0 as *mut crate::client_h::ping_t;
    let mut oldest: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    pingptr = cl_pinglist.as_mut_ptr();
    let mut current_block_3: u64;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        // find free ping slot
        if (*pingptr).adr.port != 0 {
            if (*pingptr).time == 0 {
                if crate::src::sys::sys_unix::Sys_Milliseconds() - (*pingptr).start
                    < 500 as libc::c_int
                {
                    // still waiting for response
                    current_block_3 = 735147466149431745;
                } else {
                    current_block_3 = 1841672684692190573;
                }
            } else if (*pingptr).time < 500 as libc::c_int {
                current_block_3 = 735147466149431745;
            } else {
                current_block_3 = 1841672684692190573;
            }
            match current_block_3 {
                1841672684692190573 => {}
                _ =>
                // results have not been queried
                {
                    i += 1;
                    pingptr = pingptr.offset(1);
                    continue;
                }
            }
        }
        // clear it
        (*pingptr).adr.port = 0 as libc::c_int as libc::c_ushort;
        return pingptr;
    }
    // use oldest entry
    pingptr = cl_pinglist.as_mut_ptr();
    best = cl_pinglist.as_mut_ptr();
    oldest = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        // scan for oldest
        time = crate::src::sys::sys_unix::Sys_Milliseconds() - (*pingptr).start;
        if time > oldest {
            oldest = time;
            best = pingptr
        }
        i += 1;
        pingptr = pingptr.offset(1)
    }
    return best;
}
/*
==================
CL_Ping_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_Ping_f() {
    let mut to: crate::qcommon_h::netadr_t = crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    };
    let mut pingptr: *mut crate::client_h::ping_t = 0 as *mut crate::client_h::ping_t;
    let mut server: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argc: libc::c_int = 0;
    let mut family: crate::qcommon_h::netadrtype_t = crate::qcommon_h::NA_UNSPEC;
    argc = crate::src::qcommon::cmd::Cmd_Argc();
    if argc != 2 as libc::c_int && argc != 3 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"usage: ping [-4|-6] server\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if argc == 2 as libc::c_int {
        server = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int)
    } else {
        if ::libc::strcmp(
            crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
            b"-4\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            family = crate::qcommon_h::NA_IP
        } else if ::libc::strcmp(
            crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
            b"-6\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            family = crate::qcommon_h::NA_IP6
        } else {
            crate::src::qcommon::common::Com_Printf(
                b"warning: only -4 or -6 as address type understood.\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        server = crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int)
    }
    crate::stdlib::memset(
        &mut to as *mut crate::qcommon_h::netadr_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::qcommon_h::netadr_t>() as libc::c_ulong,
    );
    if crate::src::qcommon::net_chan::NET_StringToAdr(
        server,
        &mut to as *mut _ as *mut crate::qcommon_h::netadr_t,
        family,
    ) == 0
    {
        return;
    }
    pingptr = CL_GetFreePing();
    crate::stdlib::memcpy(
        &mut (*pingptr).adr as *mut crate::qcommon_h::netadr_t as *mut libc::c_void,
        &mut to as *mut crate::qcommon_h::netadr_t as *const libc::c_void,
        ::std::mem::size_of::<crate::qcommon_h::netadr_t>() as libc::c_ulong,
    );
    (*pingptr).start = crate::src::sys::sys_unix::Sys_Milliseconds();
    (*pingptr).time = 0 as libc::c_int;
    CL_SetServerInfoByAddress((*pingptr).adr, 0 as *const libc::c_char, 0 as libc::c_int);
    crate::src::qcommon::net_chan::NET_OutOfBandPrint(
        crate::qcommon_h::NS_CLIENT,
        to as crate::qcommon_h::netadr_t,
        b"getinfo xxx\x00" as *const u8 as *const libc::c_char,
    );
}
/*
==================
CL_UpdateVisiblePings_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_UpdateVisiblePings_f(
    mut source: libc::c_int,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut slots: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut buff: [libc::c_char; 1024] = [0; 1024];
    let mut pingTime: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut status: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    if source < 0 as libc::c_int || source > 3 as libc::c_int {
        return crate::src::qcommon::q_shared::qfalse;
    }
    cls.pingUpdateSource = source;
    slots = CL_GetPingQueueCount();
    if slots < 32 as libc::c_int {
        let mut server: *mut crate::client_h::serverInfo_t =
            0 as *mut crate::client_h::serverInfo_t;
        match source {
            0 => {
                server = &mut *cls
                    .localServers
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::client_h::serverInfo_t;
                max = cls.numlocalservers
            }
            2 => {
                server = &mut *cls
                    .globalServers
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::client_h::serverInfo_t;
                max = cls.numglobalservers
            }
            3 => {
                server = &mut *cls
                    .favoriteServers
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::client_h::serverInfo_t;
                max = cls.numfavoriteservers
            }
            _ => return crate::src::qcommon::q_shared::qfalse,
        }
        i = 0 as libc::c_int;
        while i < max {
            if (*server.offset(i as isize)).visible as u64 != 0 {
                if (*server.offset(i as isize)).ping == -(1 as libc::c_int) {
                    let mut j: libc::c_int = 0;
                    if slots >= 32 as libc::c_int {
                        break;
                    }
                    j = 0 as libc::c_int;
                    while j < 32 as libc::c_int {
                        if !(cl_pinglist[j as usize].adr.port == 0) {
                            if crate::src::qcommon::net_ip::NET_CompareAdr(
                                cl_pinglist[j as usize].adr as crate::qcommon_h::netadr_t,
                                (*server.offset(i as isize)).adr as crate::qcommon_h::netadr_t,
                            ) as u64
                                != 0
                            {
                                break;
                            }
                        }
                        j += 1
                    }
                    if j >= 32 as libc::c_int {
                        status = crate::src::qcommon::q_shared::qtrue;
                        j = 0 as libc::c_int;
                        while j < 32 as libc::c_int {
                            if cl_pinglist[j as usize].adr.port == 0 {
                                break;
                            }
                            j += 1
                        }
                        crate::stdlib::memcpy(
                            &mut (*cl_pinglist.as_mut_ptr().offset(j as isize)).adr
                                as *mut crate::qcommon_h::netadr_t
                                as *mut libc::c_void,
                            &mut (*server.offset(i as isize)).adr as *mut crate::qcommon_h::netadr_t
                                as *const libc::c_void,
                            ::std::mem::size_of::<crate::qcommon_h::netadr_t>() as libc::c_ulong,
                        );
                        cl_pinglist[j as usize].start =
                            crate::src::sys::sys_unix::Sys_Milliseconds();
                        cl_pinglist[j as usize].time = 0 as libc::c_int;
                        crate::src::qcommon::net_chan::NET_OutOfBandPrint(
                            crate::qcommon_h::NS_CLIENT,
                            cl_pinglist[j as usize].adr as crate::qcommon_h::netadr_t,
                            b"getinfo xxx\x00" as *const u8 as *const libc::c_char,
                        );
                        slots += 1
                    }
                } else if (*server.offset(i as isize)).ping == 0 as libc::c_int {
                    // if the server has a ping higher than cl_maxPing or
                    // the ping packet got lost
                    // if we are updating global servers
                    if source == 2 as libc::c_int {
                        //
                        if cls.numGlobalServerAddresses > 0 as libc::c_int {
                            // overwrite this server with one from the additional global servers
                            cls.numGlobalServerAddresses -= 1;
                            CL_InitServerInfo(
                                &mut *server.offset(i as isize),
                                &mut *cls
                                    .globalServerAddresses
                                    .as_mut_ptr()
                                    .offset(cls.numGlobalServerAddresses as isize),
                            );
                            // NOTE: the server[i].visible flag stays untouched
                        }
                    }
                }
            }
            i += 1
        }
    }
    if slots != 0 {
        status = crate::src::qcommon::q_shared::qtrue
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if !(cl_pinglist[i as usize].adr.port == 0) {
            CL_GetPing(i, buff.as_mut_ptr(), 1024 as libc::c_int, &mut pingTime);
            if pingTime != 0 as libc::c_int {
                CL_ClearPing(i);
                status = crate::src::qcommon::q_shared::qtrue
            }
        }
        i += 1
    }
    return status;
}
/*
==================
CL_ServerStatus_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ServerStatus_f() {
    let mut to: crate::qcommon_h::netadr_t = crate::qcommon_h::netadr_t {
        type_0: crate::qcommon_h::NA_BAD,
        ip: [0; 4],
        ip6: [0; 16],
        port: 0,
        scope_id: 0,
    };
    let mut toptr: *mut crate::qcommon_h::netadr_t = 0 as *mut crate::qcommon_h::netadr_t;
    let mut server: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut serverStatus: *mut serverStatus_t = 0 as *mut serverStatus_t;
    let mut argc: libc::c_int = 0;
    let mut family: crate::qcommon_h::netadrtype_t = crate::qcommon_h::NA_UNSPEC;
    argc = crate::src::qcommon::cmd::Cmd_Argc();
    if argc != 2 as libc::c_int && argc != 3 as libc::c_int {
        if clc.state as libc::c_uint
            != crate::src::qcommon::q_shared::CA_ACTIVE as libc::c_int as libc::c_uint
            || clc.demoplaying as libc::c_uint != 0
        {
            crate::src::qcommon::common::Com_Printf(
                b"Not connected to a server.\n\x00" as *const u8 as *const libc::c_char,
            );
            crate::src::qcommon::common::Com_Printf(
                b"usage: serverstatus [-4|-6] server\n\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        toptr = &mut clc.serverAddress
    }
    if toptr.is_null() {
        crate::stdlib::memset(
            &mut to as *mut crate::qcommon_h::netadr_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::qcommon_h::netadr_t>() as libc::c_ulong,
        );
        if argc == 2 as libc::c_int {
            server = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int)
        } else {
            if ::libc::strcmp(
                crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
                b"-4\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                family = crate::qcommon_h::NA_IP
            } else if ::libc::strcmp(
                crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
                b"-6\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                family = crate::qcommon_h::NA_IP6
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b"warning: only -4 or -6 as address type understood.\n\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            server = crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int)
        }
        toptr = &mut to;
        if crate::src::qcommon::net_chan::NET_StringToAdr(
            server,
            toptr as *mut crate::qcommon_h::netadr_t,
            family,
        ) == 0
        {
            return;
        }
    }
    crate::src::qcommon::net_chan::NET_OutOfBandPrint(
        crate::qcommon_h::NS_CLIENT,
        *toptr as crate::qcommon_h::netadr_t,
        b"getstatus\x00" as *const u8 as *const libc::c_char,
    );
    serverStatus = CL_GetServerStatus(*toptr);
    (*serverStatus).address = *toptr;
    (*serverStatus).print = crate::src::qcommon::q_shared::qtrue;
    (*serverStatus).pending = crate::src::qcommon::q_shared::qtrue;
}
/*
==================
CL_ShowIP_f
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ShowIP_f() {
    crate::src::qcommon::net_ip::Sys_ShowIP();
}
/*
=================
CL_CDKeyValidate
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CDKeyValidate(
    mut key: *const libc::c_char,
    mut checksum: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut ch: libc::c_char = 0;
    let mut sum: crate::src::qcommon::q_shared::byte = 0;
    let mut chs: [libc::c_char; 3] = [0; 3];
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    len = crate::stdlib::strlen(key) as libc::c_int;
    if len != 16 as libc::c_int {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if !checksum.is_null() && crate::stdlib::strlen(checksum) != 2 as libc::c_int as libc::c_ulong {
        return crate::src::qcommon::q_shared::qfalse;
    }
    sum = 0 as libc::c_int as crate::src::qcommon::q_shared::byte;
    // for loop gets rid of conditional assignment warning
    i = 0 as libc::c_int;
    while i < len {
        let fresh11 = key;
        key = key.offset(1);
        ch = *fresh11;
        if ch as libc::c_int >= 'a' as i32 && ch as libc::c_int <= 'z' as i32 {
            ch = (ch as libc::c_int - 32 as libc::c_int) as libc::c_char
        }
        match ch as libc::c_int {
            50 | 51 | 55 | 65 | 66 | 67 | 68 | 71 | 72 | 74 | 76 | 80 | 82 | 83 | 84 | 87 => {
                sum =
                    (sum as libc::c_int + ch as libc::c_int) as crate::src::qcommon::q_shared::byte;
                i += 1
            }
            _ => return crate::src::qcommon::q_shared::qfalse,
        }
    }
    ::libc::sprintf(
        chs.as_mut_ptr(),
        b"%02x\x00" as *const u8 as *const libc::c_char,
        sum as libc::c_int,
    );
    if !checksum.is_null()
        && crate::src::qcommon::q_shared::Q_stricmp(chs.as_mut_ptr(), checksum) == 0
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    if checksum.is_null() {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}

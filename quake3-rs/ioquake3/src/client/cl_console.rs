use ::libc;

pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::uint8_t;

pub use crate::client_h::clSnapshot_t;
pub use crate::client_h::clientActive_t;
pub use crate::client_h::clientConnection_t;
pub use crate::client_h::clientStatic_t;
pub use crate::client_h::outPacket_t;
pub use crate::client_h::serverInfo_t;
pub use crate::qcommon_h::completionFunc_t;
pub use crate::qcommon_h::field_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
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
pub use crate::src::client::cl_keys::CL_KeyEvent;
pub use crate::src::client::cl_keys::CL_LoadConsoleHistory;
pub use crate::src::client::cl_keys::Key_GetCatcher;
pub use crate::src::client::cl_keys::Key_SetCatcher;
pub use crate::src::client::cl_main::cgvm;
pub use crate::src::client::cl_main::cl;
pub use crate::src::client::cl_main::cl_conXOffset;
pub use crate::src::client::cl_main::cl_noprint;
pub use crate::src::client::cl_main::clc;
pub use crate::src::client::cl_main::cls;
pub use crate::src::client::cl_main::re;
pub use crate::src::client::cl_scrn::SCR_AdjustFrom640;
pub use crate::src::client::cl_scrn::SCR_DrawBigString;
pub use crate::src::client::cl_scrn::SCR_DrawPic;
pub use crate::src::client::cl_scrn::SCR_DrawSmallChar;
pub use crate::src::client::cl_scrn::SCR_FillRect;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::cmd::Cmd_RemoveCommand;
pub use crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc;
pub use crate::src::qcommon::common::com_cl_running;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Field_Clear;
pub use crate::src::qcommon::common::Field_CompleteFilename;
pub use crate::src::qcommon::common::Hunk_AllocateTempMemory;
pub use crate::src::qcommon::common::Hunk_FreeTempMemory;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_FOpenFileWrite;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::q_math::g_color_table;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fontInfo_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::glyphInfo_t;
pub use crate::src::qcommon::q_shared::markFragment_t;
pub use crate::src::qcommon::q_shared::orientation_t;
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
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::COM_CompareExtension;
pub use crate::src::qcommon::q_shared::COM_DefaultExtension;
pub use crate::src::qcommon::q_shared::Q_IsColorString;
pub use crate::src::qcommon::q_shared::Q_strcat;
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
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::qcommon::vm::VM_Call;
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

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::bg_public_h::PM_DEAD;
pub use crate::bg_public_h::PM_FREEZE;
pub use crate::bg_public_h::PM_INTERMISSION;
pub use crate::bg_public_h::PM_NOCLIP;
pub use crate::bg_public_h::PM_NORMAL;
pub use crate::bg_public_h::PM_SPECTATOR;
pub use crate::bg_public_h::PM_SPINTERMISSION;
pub use crate::cg_public_h::CG_CONSOLE_COMMAND;
pub use crate::cg_public_h::CG_CROSSHAIR_PLAYER;
pub use crate::cg_public_h::CG_DRAW_ACTIVE_FRAME;
pub use crate::cg_public_h::CG_EVENT_HANDLING;
pub use crate::cg_public_h::CG_INIT;
pub use crate::cg_public_h::CG_KEY_EVENT;
pub use crate::cg_public_h::CG_LAST_ATTACKER;
pub use crate::cg_public_h::CG_MOUSE_EVENT;
pub use crate::cg_public_h::CG_SHUTDOWN;
pub use crate::curl_h::CURL;
pub use crate::keycodes_h::K_ALT;
pub use crate::keycodes_h::K_AUX1;
pub use crate::keycodes_h::K_AUX10;
pub use crate::keycodes_h::K_AUX11;
pub use crate::keycodes_h::K_AUX12;
pub use crate::keycodes_h::K_AUX13;
pub use crate::keycodes_h::K_AUX14;
pub use crate::keycodes_h::K_AUX15;
pub use crate::keycodes_h::K_AUX16;
pub use crate::keycodes_h::K_AUX2;
pub use crate::keycodes_h::K_AUX3;
pub use crate::keycodes_h::K_AUX4;
pub use crate::keycodes_h::K_AUX5;
pub use crate::keycodes_h::K_AUX6;
pub use crate::keycodes_h::K_AUX7;
pub use crate::keycodes_h::K_AUX8;
pub use crate::keycodes_h::K_AUX9;
pub use crate::keycodes_h::K_BACKSPACE;
pub use crate::keycodes_h::K_BREAK;
pub use crate::keycodes_h::K_CAPSLOCK;
pub use crate::keycodes_h::K_COMMAND;
pub use crate::keycodes_h::K_COMPOSE;
pub use crate::keycodes_h::K_CONSOLE;
pub use crate::keycodes_h::K_CTRL;
pub use crate::keycodes_h::K_DEL;
pub use crate::keycodes_h::K_DOWNARROW;
pub use crate::keycodes_h::K_END;
pub use crate::keycodes_h::K_ENTER;
pub use crate::keycodes_h::K_ESCAPE;
pub use crate::keycodes_h::K_EURO;
pub use crate::keycodes_h::K_F1;
pub use crate::keycodes_h::K_F10;
pub use crate::keycodes_h::K_F11;
pub use crate::keycodes_h::K_F12;
pub use crate::keycodes_h::K_F13;
pub use crate::keycodes_h::K_F14;
pub use crate::keycodes_h::K_F15;
pub use crate::keycodes_h::K_F2;
pub use crate::keycodes_h::K_F3;
pub use crate::keycodes_h::K_F4;
pub use crate::keycodes_h::K_F5;
pub use crate::keycodes_h::K_F6;
pub use crate::keycodes_h::K_F7;
pub use crate::keycodes_h::K_F8;
pub use crate::keycodes_h::K_F9;
pub use crate::keycodes_h::K_HELP;
pub use crate::keycodes_h::K_HOME;
pub use crate::keycodes_h::K_INS;
pub use crate::keycodes_h::K_JOY1;
pub use crate::keycodes_h::K_JOY10;
pub use crate::keycodes_h::K_JOY11;
pub use crate::keycodes_h::K_JOY12;
pub use crate::keycodes_h::K_JOY13;
pub use crate::keycodes_h::K_JOY14;
pub use crate::keycodes_h::K_JOY15;
pub use crate::keycodes_h::K_JOY16;
pub use crate::keycodes_h::K_JOY17;
pub use crate::keycodes_h::K_JOY18;
pub use crate::keycodes_h::K_JOY19;
pub use crate::keycodes_h::K_JOY2;
pub use crate::keycodes_h::K_JOY20;
pub use crate::keycodes_h::K_JOY21;
pub use crate::keycodes_h::K_JOY22;
pub use crate::keycodes_h::K_JOY23;
pub use crate::keycodes_h::K_JOY24;
pub use crate::keycodes_h::K_JOY25;
pub use crate::keycodes_h::K_JOY26;
pub use crate::keycodes_h::K_JOY27;
pub use crate::keycodes_h::K_JOY28;
pub use crate::keycodes_h::K_JOY29;
pub use crate::keycodes_h::K_JOY3;
pub use crate::keycodes_h::K_JOY30;
pub use crate::keycodes_h::K_JOY31;
pub use crate::keycodes_h::K_JOY32;
pub use crate::keycodes_h::K_JOY4;
pub use crate::keycodes_h::K_JOY5;
pub use crate::keycodes_h::K_JOY6;
pub use crate::keycodes_h::K_JOY7;
pub use crate::keycodes_h::K_JOY8;
pub use crate::keycodes_h::K_JOY9;
pub use crate::keycodes_h::K_KP_5;
pub use crate::keycodes_h::K_KP_DEL;
pub use crate::keycodes_h::K_KP_DOWNARROW;
pub use crate::keycodes_h::K_KP_END;
pub use crate::keycodes_h::K_KP_ENTER;
pub use crate::keycodes_h::K_KP_EQUALS;
pub use crate::keycodes_h::K_KP_HOME;
pub use crate::keycodes_h::K_KP_INS;
pub use crate::keycodes_h::K_KP_LEFTARROW;
pub use crate::keycodes_h::K_KP_MINUS;
pub use crate::keycodes_h::K_KP_NUMLOCK;
pub use crate::keycodes_h::K_KP_PGDN;
pub use crate::keycodes_h::K_KP_PGUP;
pub use crate::keycodes_h::K_KP_PLUS;
pub use crate::keycodes_h::K_KP_RIGHTARROW;
pub use crate::keycodes_h::K_KP_SLASH;
pub use crate::keycodes_h::K_KP_STAR;
pub use crate::keycodes_h::K_KP_UPARROW;
pub use crate::keycodes_h::K_LEFTARROW;
pub use crate::keycodes_h::K_MENU;
pub use crate::keycodes_h::K_MODE;
pub use crate::keycodes_h::K_MOUSE1;
pub use crate::keycodes_h::K_MOUSE2;
pub use crate::keycodes_h::K_MOUSE3;
pub use crate::keycodes_h::K_MOUSE4;
pub use crate::keycodes_h::K_MOUSE5;
pub use crate::keycodes_h::K_MWHEELDOWN;
pub use crate::keycodes_h::K_MWHEELUP;
pub use crate::keycodes_h::K_PAD0_A;
pub use crate::keycodes_h::K_PAD0_B;
pub use crate::keycodes_h::K_PAD0_BACK;
pub use crate::keycodes_h::K_PAD0_DPAD_DOWN;
pub use crate::keycodes_h::K_PAD0_DPAD_LEFT;
pub use crate::keycodes_h::K_PAD0_DPAD_RIGHT;
pub use crate::keycodes_h::K_PAD0_DPAD_UP;
pub use crate::keycodes_h::K_PAD0_GUIDE;
pub use crate::keycodes_h::K_PAD0_LEFTSHOULDER;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_LEFTTRIGGER;
pub use crate::keycodes_h::K_PAD0_RIGHTSHOULDER;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_RIGHTTRIGGER;
pub use crate::keycodes_h::K_PAD0_START;
pub use crate::keycodes_h::K_PAD0_X;
pub use crate::keycodes_h::K_PAD0_Y;
pub use crate::keycodes_h::K_PAUSE;
pub use crate::keycodes_h::K_PGDN;
pub use crate::keycodes_h::K_PGUP;
pub use crate::keycodes_h::K_POWER;
pub use crate::keycodes_h::K_PRINT;
pub use crate::keycodes_h::K_RIGHTARROW;
pub use crate::keycodes_h::K_SCROLLOCK;
pub use crate::keycodes_h::K_SHIFT;
pub use crate::keycodes_h::K_SPACE;
pub use crate::keycodes_h::K_SUPER;
pub use crate::keycodes_h::K_SYSREQ;
pub use crate::keycodes_h::K_TAB;
pub use crate::keycodes_h::K_UNDO;
pub use crate::keycodes_h::K_UPARROW;
pub use crate::keycodes_h::K_WORLD_0;
pub use crate::keycodes_h::K_WORLD_1;
pub use crate::keycodes_h::K_WORLD_10;
pub use crate::keycodes_h::K_WORLD_11;
pub use crate::keycodes_h::K_WORLD_12;
pub use crate::keycodes_h::K_WORLD_13;
pub use crate::keycodes_h::K_WORLD_14;
pub use crate::keycodes_h::K_WORLD_15;
pub use crate::keycodes_h::K_WORLD_16;
pub use crate::keycodes_h::K_WORLD_17;
pub use crate::keycodes_h::K_WORLD_18;
pub use crate::keycodes_h::K_WORLD_19;
pub use crate::keycodes_h::K_WORLD_2;
pub use crate::keycodes_h::K_WORLD_20;
pub use crate::keycodes_h::K_WORLD_21;
pub use crate::keycodes_h::K_WORLD_22;
pub use crate::keycodes_h::K_WORLD_23;
pub use crate::keycodes_h::K_WORLD_24;
pub use crate::keycodes_h::K_WORLD_25;
pub use crate::keycodes_h::K_WORLD_26;
pub use crate::keycodes_h::K_WORLD_27;
pub use crate::keycodes_h::K_WORLD_28;
pub use crate::keycodes_h::K_WORLD_29;
pub use crate::keycodes_h::K_WORLD_3;
pub use crate::keycodes_h::K_WORLD_30;
pub use crate::keycodes_h::K_WORLD_31;
pub use crate::keycodes_h::K_WORLD_32;
pub use crate::keycodes_h::K_WORLD_33;
pub use crate::keycodes_h::K_WORLD_34;
pub use crate::keycodes_h::K_WORLD_35;
pub use crate::keycodes_h::K_WORLD_36;
pub use crate::keycodes_h::K_WORLD_37;
pub use crate::keycodes_h::K_WORLD_38;
pub use crate::keycodes_h::K_WORLD_39;
pub use crate::keycodes_h::K_WORLD_4;
pub use crate::keycodes_h::K_WORLD_40;
pub use crate::keycodes_h::K_WORLD_41;
pub use crate::keycodes_h::K_WORLD_42;
pub use crate::keycodes_h::K_WORLD_43;
pub use crate::keycodes_h::K_WORLD_44;
pub use crate::keycodes_h::K_WORLD_45;
pub use crate::keycodes_h::K_WORLD_46;
pub use crate::keycodes_h::K_WORLD_47;
pub use crate::keycodes_h::K_WORLD_48;
pub use crate::keycodes_h::K_WORLD_49;
pub use crate::keycodes_h::K_WORLD_5;
pub use crate::keycodes_h::K_WORLD_50;
pub use crate::keycodes_h::K_WORLD_51;
pub use crate::keycodes_h::K_WORLD_52;
pub use crate::keycodes_h::K_WORLD_53;
pub use crate::keycodes_h::K_WORLD_54;
pub use crate::keycodes_h::K_WORLD_55;
pub use crate::keycodes_h::K_WORLD_56;
pub use crate::keycodes_h::K_WORLD_57;
pub use crate::keycodes_h::K_WORLD_58;
pub use crate::keycodes_h::K_WORLD_59;
pub use crate::keycodes_h::K_WORLD_6;
pub use crate::keycodes_h::K_WORLD_60;
pub use crate::keycodes_h::K_WORLD_61;
pub use crate::keycodes_h::K_WORLD_62;
pub use crate::keycodes_h::K_WORLD_63;
pub use crate::keycodes_h::K_WORLD_64;
pub use crate::keycodes_h::K_WORLD_65;
pub use crate::keycodes_h::K_WORLD_66;
pub use crate::keycodes_h::K_WORLD_67;
pub use crate::keycodes_h::K_WORLD_68;
pub use crate::keycodes_h::K_WORLD_69;
pub use crate::keycodes_h::K_WORLD_7;
pub use crate::keycodes_h::K_WORLD_70;
pub use crate::keycodes_h::K_WORLD_71;
pub use crate::keycodes_h::K_WORLD_72;
pub use crate::keycodes_h::K_WORLD_73;
pub use crate::keycodes_h::K_WORLD_74;
pub use crate::keycodes_h::K_WORLD_75;
pub use crate::keycodes_h::K_WORLD_76;
pub use crate::keycodes_h::K_WORLD_77;
pub use crate::keycodes_h::K_WORLD_78;
pub use crate::keycodes_h::K_WORLD_79;
pub use crate::keycodes_h::K_WORLD_8;
pub use crate::keycodes_h::K_WORLD_80;
pub use crate::keycodes_h::K_WORLD_81;
pub use crate::keycodes_h::K_WORLD_82;
pub use crate::keycodes_h::K_WORLD_83;
pub use crate::keycodes_h::K_WORLD_84;
pub use crate::keycodes_h::K_WORLD_85;
pub use crate::keycodes_h::K_WORLD_86;
pub use crate::keycodes_h::K_WORLD_87;
pub use crate::keycodes_h::K_WORLD_88;
pub use crate::keycodes_h::K_WORLD_89;
pub use crate::keycodes_h::K_WORLD_9;
pub use crate::keycodes_h::K_WORLD_90;
pub use crate::keycodes_h::K_WORLD_91;
pub use crate::keycodes_h::K_WORLD_92;
pub use crate::keycodes_h::K_WORLD_93;
pub use crate::keycodes_h::K_WORLD_94;
pub use crate::keycodes_h::K_WORLD_95;
pub use crate::keycodes_h::MAX_KEYS;
pub use crate::multi_h::CURLM;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct console_t {
    pub initialized: crate::src::qcommon::q_shared::qboolean,
    pub text: [i16; 32768],
    pub current: i32,
    pub x: i32,
    pub display: i32,
    pub linewidth: i32,
    pub totallines: i32,
    pub xadjust: f32,
    pub displayFrac: f32,
    pub finalFrac: f32,
    pub vislines: i32,
    pub times: [i32; 4],
    pub color: crate::src::qcommon::q_shared::vec4_t,
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
// console.c
#[no_mangle]

pub static mut g_console_field_width: i32 = 78 as i32;
#[no_mangle]

pub static mut con: console_t = console_t {
    initialized: crate::src::qcommon::q_shared::qfalse,
    text: [0; 32768],
    current: 0,
    x: 0,
    display: 0,
    linewidth: 0,
    totallines: 0,
    xadjust: 0.,
    displayFrac: 0.,
    finalFrac: 0.,
    vislines: 0,
    times: [0; 4],
    color: [0.; 4],
};
#[no_mangle]

pub static mut con_conspeed: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut con_autoclear: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut con_notifytime: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
/*
================
Con_ToggleConsole_f
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_ToggleConsole_f() {
    // Can't toggle the console when it's the only thing available
    if crate::src::client::cl_main::clc.state as u32
        == crate::src::qcommon::q_shared::CA_DISCONNECTED as i32 as u32
        && crate::src::client::cl_keys::Key_GetCatcher() == 0x1 as i32
    {
        return;
    }
    if (*con_autoclear).integer != 0 {
        crate::src::qcommon::common::Field_Clear(
            &mut crate::src::client::cl_keys::g_consoleField as *mut _
                as *mut crate::qcommon_h::field_t,
        );
    }
    crate::src::client::cl_keys::g_consoleField.widthInChars = g_console_field_width;
    Con_ClearNotify();
    crate::src::client::cl_keys::Key_SetCatcher(
        crate::src::client::cl_keys::Key_GetCatcher() ^ 0x1 as i32,
    );
}
/*
===================
Con_ToggleMenu_f
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_ToggleMenu_f() {
    crate::src::client::cl_keys::CL_KeyEvent(
        crate::keycodes_h::K_ESCAPE as i32,
        crate::src::qcommon::q_shared::qtrue,
        crate::src::sys::sys_unix::Sys_Milliseconds() as u32,
    );
    crate::src::client::cl_keys::CL_KeyEvent(
        crate::keycodes_h::K_ESCAPE as i32,
        crate::src::qcommon::q_shared::qfalse,
        crate::src::sys::sys_unix::Sys_Milliseconds() as u32,
    );
}
/*
================
Con_MessageMode_f
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_MessageMode_f() {
    crate::src::client::cl_keys::chat_playerNum = -(1 as i32);
    crate::src::client::cl_keys::chat_team = crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::common::Field_Clear(
        &mut crate::src::client::cl_keys::chatField as *mut _ as *mut crate::qcommon_h::field_t,
    );
    crate::src::client::cl_keys::chatField.widthInChars = 30 as i32;
    crate::src::client::cl_keys::Key_SetCatcher(
        crate::src::client::cl_keys::Key_GetCatcher() ^ 0x4 as i32,
    );
}
/*
================
Con_MessageMode2_f
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_MessageMode2_f() {
    crate::src::client::cl_keys::chat_playerNum = -(1 as i32);
    crate::src::client::cl_keys::chat_team = crate::src::qcommon::q_shared::qtrue;
    crate::src::qcommon::common::Field_Clear(
        &mut crate::src::client::cl_keys::chatField as *mut _ as *mut crate::qcommon_h::field_t,
    );
    crate::src::client::cl_keys::chatField.widthInChars = 25 as i32;
    crate::src::client::cl_keys::Key_SetCatcher(
        crate::src::client::cl_keys::Key_GetCatcher() ^ 0x4 as i32,
    );
}
/*
================
Con_MessageMode3_f
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_MessageMode3_f() {
    crate::src::client::cl_keys::chat_playerNum = crate::src::qcommon::vm::VM_Call(
        crate::src::client::cl_main::cgvm,
        crate::cg_public_h::CG_CROSSHAIR_PLAYER as i32,
    ) as i32;
    if crate::src::client::cl_keys::chat_playerNum < 0 as i32
        || crate::src::client::cl_keys::chat_playerNum >= 64 as i32
    {
        crate::src::client::cl_keys::chat_playerNum = -(1 as i32);
        return;
    }
    crate::src::client::cl_keys::chat_team = crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::common::Field_Clear(
        &mut crate::src::client::cl_keys::chatField as *mut _ as *mut crate::qcommon_h::field_t,
    );
    crate::src::client::cl_keys::chatField.widthInChars = 30 as i32;
    crate::src::client::cl_keys::Key_SetCatcher(
        crate::src::client::cl_keys::Key_GetCatcher() ^ 0x4 as i32,
    );
}
/*
================
Con_MessageMode4_f
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_MessageMode4_f() {
    crate::src::client::cl_keys::chat_playerNum = crate::src::qcommon::vm::VM_Call(
        crate::src::client::cl_main::cgvm,
        crate::cg_public_h::CG_LAST_ATTACKER as i32,
    ) as i32;
    if crate::src::client::cl_keys::chat_playerNum < 0 as i32
        || crate::src::client::cl_keys::chat_playerNum >= 64 as i32
    {
        crate::src::client::cl_keys::chat_playerNum = -(1 as i32);
        return;
    }
    crate::src::client::cl_keys::chat_team = crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::common::Field_Clear(
        &mut crate::src::client::cl_keys::chatField as *mut _ as *mut crate::qcommon_h::field_t,
    );
    crate::src::client::cl_keys::chatField.widthInChars = 30 as i32;
    crate::src::client::cl_keys::Key_SetCatcher(
        crate::src::client::cl_keys::Key_GetCatcher() ^ 0x4 as i32,
    );
}
/*
================
Con_Clear_f
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_Clear_f() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 32768 as i32 {
        con.text[i as usize] = (('7' as i32 - '0' as i32 & 0x7 as i32) << 8 as i32
            | ' ' as i32) as i16;
        i += 1
    }
    Con_Bottom();
    // go to end
}
/*
================
Con_Dump_f

Save the console contents out to a file
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_Dump_f() {
    let mut l: i32 = 0;
    let mut x: i32 = 0;
    let mut i: i32 = 0;
    let mut line: *mut i16 = 0 as *mut i16;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut bufferlen: i32 = 0;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: [libc::c_char; 64] = [0; 64];
    if crate::src::qcommon::cmd::Cmd_Argc() != 2 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"usage: condump <filename>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        filename.as_mut_ptr(),
        crate::src::qcommon::cmd::Cmd_Argv(1 as i32),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::COM_DefaultExtension(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        b".txt\x00" as *const u8 as *const libc::c_char,
    );
    if crate::src::qcommon::q_shared::COM_CompareExtension(
        filename.as_mut_ptr(),
        b".txt\x00" as *const u8 as *const libc::c_char,
    ) as u64
        == 0
    {
        crate::src::qcommon::common::Com_Printf(
            b"Con_Dump_f: Only the \".txt\" extension is supported by this command!\n\x00"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    f = crate::src::qcommon::files::FS_FOpenFileWrite(filename.as_mut_ptr());
    if f == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"ERROR: couldn\'t open %s.\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return;
    }
    crate::src::qcommon::common::Com_Printf(
        b"Dumped console text to %s.\n\x00" as *const u8 as *const libc::c_char,
        filename.as_mut_ptr(),
    );
    // skip empty lines
    l = con.current - con.totallines + 1 as i32;
    while l <= con.current {
        line = con
            .text
            .as_mut_ptr()
            .offset((l % con.totallines * con.linewidth) as isize);
        x = 0 as i32;
        while x < con.linewidth {
            if *line.offset(x as isize) as i32 & 0xff as i32 != ' ' as i32 {
                break;
            }
            x += 1
        }
        if x != con.linewidth {
            break;
        }
        l += 1
    }
    bufferlen = (con.linewidth as libc::c_ulong).wrapping_add(
        (2 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as i32;
    buffer = crate::src::qcommon::common::Hunk_AllocateTempMemory(bufferlen) as *mut libc::c_char;
    // write the remaining lines
    *buffer.offset((bufferlen - 1 as i32) as isize) = 0 as i32 as libc::c_char;
    while l <= con.current {
        line = con
            .text
            .as_mut_ptr()
            .offset((l % con.totallines * con.linewidth) as isize);
        i = 0 as i32;
        while i < con.linewidth {
            *buffer.offset(i as isize) =
                (*line.offset(i as isize) as i32 & 0xff as i32) as libc::c_char;
            i += 1
        }
        x = con.linewidth - 1 as i32;
        while x >= 0 as i32 {
            if !(*buffer.offset(x as isize) as i32 == ' ' as i32) {
                break;
            }
            *buffer.offset(x as isize) = 0 as i32 as libc::c_char;
            x -= 1
        }
        crate::src::qcommon::q_shared::Q_strcat(
            buffer,
            bufferlen,
            b"\n\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::files::FS_Write(
            buffer as *const libc::c_void,
            crate::stdlib::strlen(buffer) as i32,
            f,
        );
        l += 1
    }
    crate::src::qcommon::common::Hunk_FreeTempMemory(buffer as *mut libc::c_void);
    crate::src::qcommon::files::FS_FCloseFile(f);
}
/*
================
Con_ClearNotify
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_ClearNotify() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 4 as i32 {
        con.times[i as usize] = 0 as i32;
        i += 1
    }
}
/*
================
Con_CheckResize

If the line width has changed, reformat the buffer.
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_CheckResize() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut width: i32 = 0;
    let mut oldwidth: i32 = 0;
    let mut oldtotallines: i32 = 0;
    let mut numlines: i32 = 0;
    let mut numchars: i32 = 0;
    let mut tbuf: [i16; 32768] = [0; 32768];
    width = 640 as i32 / 8 as i32 - 2 as i32;
    if width == con.linewidth {
        return;
    }
    if width < 1 as i32 {
        // video hasn't been initialized yet
        width = 78 as i32;
        con.linewidth = width;
        con.totallines = 32768 as i32 / con.linewidth;
        i = 0 as i32;
        while i < 32768 as i32 {
            con.text[i as usize] = (('7' as i32 - '0' as i32 & 0x7 as i32)
                << 8 as i32
                | ' ' as i32) as i16;
            i += 1
        }
    } else {
        oldwidth = con.linewidth;
        con.linewidth = width;
        oldtotallines = con.totallines;
        con.totallines = 32768 as i32 / con.linewidth;
        numlines = oldtotallines;
        if con.totallines < numlines {
            numlines = con.totallines
        }
        numchars = oldwidth;
        if con.linewidth < numchars {
            numchars = con.linewidth
        }
        crate::stdlib::memcpy(
            tbuf.as_mut_ptr() as *mut libc::c_void,
            con.text.as_mut_ptr() as *const libc::c_void,
            (32768 as i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<i16>() as libc::c_ulong),
        );
        i = 0 as i32;
        while i < 32768 as i32 {
            con.text[i as usize] = (('7' as i32 - '0' as i32 & 0x7 as i32)
                << 8 as i32
                | ' ' as i32) as i16;
            i += 1
        }
        i = 0 as i32;
        while i < numlines {
            j = 0 as i32;
            while j < numchars {
                con.text[((con.totallines - 1 as i32 - i) * con.linewidth + j) as usize] =
                    tbuf[((con.current - i + oldtotallines) % oldtotallines * oldwidth + j)
                        as usize];
                j += 1
            }
            i += 1
        }
        Con_ClearNotify();
    }
    con.current = con.totallines - 1 as i32;
    con.display = con.current;
}
/*
==================
Cmd_CompleteTxtName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_CompleteTxtName(
    mut _args: *mut libc::c_char,
    mut argNum: i32,
) {
    if argNum == 2 as i32 {
        crate::src::qcommon::common::Field_CompleteFilename(
            b"\x00" as *const u8 as *const libc::c_char,
            b"txt\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::qfalse,
            crate::src::qcommon::q_shared::qtrue,
        );
    };
}
/*
================
Con_Init
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_Init() {
    let mut i: i32 = 0;
    con_notifytime = crate::src::qcommon::cvar::Cvar_Get(
        b"con_notifytime\x00" as *const u8 as *const libc::c_char,
        b"3\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    con_conspeed = crate::src::qcommon::cvar::Cvar_Get(
        b"scr_conspeed\x00" as *const u8 as *const libc::c_char,
        b"3\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    con_autoclear = crate::src::qcommon::cvar::Cvar_Get(
        b"con_autoclear\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::qcommon::common::Field_Clear(
        &mut crate::src::client::cl_keys::g_consoleField as *mut _
            as *mut crate::qcommon_h::field_t,
    );
    crate::src::client::cl_keys::g_consoleField.widthInChars = g_console_field_width;
    i = 0 as i32;
    while i < 32 as i32 {
        crate::src::qcommon::common::Field_Clear(
            &mut *crate::src::client::cl_keys::historyEditLines
                .as_mut_ptr()
                .offset(i as isize) as *mut _ as *mut crate::qcommon_h::field_t,
        );
        crate::src::client::cl_keys::historyEditLines[i as usize].widthInChars =
            g_console_field_width;
        i += 1
    }
    crate::src::client::cl_keys::CL_LoadConsoleHistory();
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"toggleconsole\x00" as *const u8 as *const libc::c_char,
        Some(Con_ToggleConsole_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"togglemenu\x00" as *const u8 as *const libc::c_char,
        Some(Con_ToggleMenu_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"messagemode\x00" as *const u8 as *const libc::c_char,
        Some(Con_MessageMode_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"messagemode2\x00" as *const u8 as *const libc::c_char,
        Some(Con_MessageMode2_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"messagemode3\x00" as *const u8 as *const libc::c_char,
        Some(Con_MessageMode3_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"messagemode4\x00" as *const u8 as *const libc::c_char,
        Some(Con_MessageMode4_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"clear\x00" as *const u8 as *const libc::c_char,
        Some(Con_Clear_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"condump\x00" as *const u8 as *const libc::c_char,
        Some(Con_Dump_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"condump\x00" as *const u8 as *const libc::c_char,
        Some(
            Cmd_CompleteTxtName as unsafe extern "C" fn(_: *mut libc::c_char, _: i32) -> (),
        ),
    );
}
/*
================
Con_Shutdown
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_Shutdown() {
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"toggleconsole\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"togglemenu\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"messagemode\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"messagemode2\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"messagemode3\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"messagemode4\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"clear\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"condump\x00" as *const u8 as *const libc::c_char);
}
/*
===============
Con_Linefeed
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Con_Linefeed(mut skipnotify: crate::src::qcommon::q_shared::qboolean) {
    let mut i: i32 = 0;
    // mark time for transparent overlay
    if con.current >= 0 as i32 {
        if skipnotify as u64 != 0 {
            con.times[(con.current % 4 as i32) as usize] = 0 as i32
        } else {
            con.times[(con.current % 4 as i32) as usize] =
                crate::src::client::cl_main::cls.realtime
        }
    }
    con.x = 0 as i32;
    if con.display == con.current {
        con.display += 1
    }
    con.current += 1;
    i = 0 as i32;
    while i < con.linewidth {
        con.text[(con.current % con.totallines * con.linewidth + i) as usize] =
            (('7' as i32 - '0' as i32 & 0x7 as i32) << 8 as i32 | ' ' as i32)
                as i16;
        i += 1
    }
}
/*
================
CL_ConsolePrint

Handles cursor positioning, line wrapping, etc
All console printing must go through this in order to be logged to disk
If no console is visible, the text will appear at the top of the game window
================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ConsolePrint(mut txt: *mut libc::c_char) {
    let mut y: i32 = 0; // NERVE - SMF
    let mut l: i32 = 0; // NERVE - SMF
    let mut c: u8 = 0;
    let mut color: u16 = 0;
    let mut skipnotify: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut prev: i32 = 0;
    // TTimo - prefix for text that shows up in console but not in notify
    // backported from RTCW
    if crate::src::qcommon::q_shared::Q_strncmp(
        txt,
        b"[skipnotify]\x00" as *const u8 as *const libc::c_char,
        12 as i32,
    ) == 0
    {
        skipnotify = crate::src::qcommon::q_shared::qtrue;
        txt = txt.offset(12 as i32 as isize)
    }
    // for some demos we don't want to ever show anything on the console
    if !crate::src::client::cl_main::cl_noprint.is_null()
        && (*crate::src::client::cl_main::cl_noprint).integer != 0
    {
        return;
    }
    if con.initialized as u64 == 0 {
        con.color[3 as i32 as usize] = 1.0f32;
        con.color[2 as i32 as usize] = con.color[3 as i32 as usize];
        con.color[1 as i32 as usize] = con.color[2 as i32 as usize];
        con.color[0 as i32 as usize] = con.color[1 as i32 as usize];
        con.linewidth = -(1 as i32);
        Con_CheckResize();
        con.initialized = crate::src::qcommon::q_shared::qtrue
    }
    color = ('7' as i32 - '0' as i32 & 0x7 as i32) as u16;
    loop {
        c = *(txt as *mut u8);
        if !(c as i32 != 0 as i32) {
            break;
        }
        if crate::src::qcommon::q_shared::Q_IsColorString(txt) as u64 != 0 {
            color = (*txt.offset(1 as i32 as isize) as i32 - '0' as i32
                & 0x7 as i32) as u16;
            txt = txt.offset(2 as i32 as isize)
        } else {
            // count word length
            l = 0 as i32;
            while l < con.linewidth {
                if *txt.offset(l as isize) as i32 <= ' ' as i32 {
                    break;
                }
                l += 1
            }
            // word wrap
            if l != con.linewidth && con.x + l >= con.linewidth {
                Con_Linefeed(skipnotify);
            }
            txt = txt.offset(1);
            match c as i32 {
                10 => {
                    Con_Linefeed(skipnotify);
                }
                13 => con.x = 0 as i32,
                _ => {
                    // display character and advance
                    y = con.current % con.totallines;
                    con.text[(y * con.linewidth + con.x) as usize] =
                        ((color as i32) << 8 as i32 | c as i32)
                            as i16;
                    con.x += 1;
                    if con.x >= con.linewidth {
                        Con_Linefeed(skipnotify);
                    }
                }
            }
        }
    }
    // mark time for transparent overlay
    if con.current >= 0 as i32 {
        // NERVE - SMF
        if skipnotify as u64 != 0 {
            prev = con.current % 4 as i32 - 1 as i32;
            if prev < 0 as i32 {
                prev = 4 as i32 - 1 as i32
            }
            con.times[prev as usize] = 0 as i32
        } else {
            // -NERVE - SMF
            con.times[(con.current % 4 as i32) as usize] =
                crate::src::client::cl_main::cls.realtime
        }
    };
}
/*
==============================================================================

DRAWING

==============================================================================
*/
/*
================
Con_DrawInput

Draw the editline after a ] prompt
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_DrawInput() {
    let mut y: i32 = 0;
    if crate::src::client::cl_main::clc.state as u32
        != crate::src::qcommon::q_shared::CA_DISCONNECTED as i32 as u32
        && crate::src::client::cl_keys::Key_GetCatcher() & 0x1 as i32 == 0
    {
        return;
    }
    y = con.vislines - 16 as i32 * 2 as i32;
    crate::src::client::cl_main::re
        .SetColor
        .expect("non-null function pointer")(con.color.as_mut_ptr());
    crate::src::client::cl_scrn::SCR_DrawSmallChar(
        (con.xadjust + (1 as i32 * 8 as i32) as f32) as i32,
        y,
        ']' as i32,
    );
    crate::src::client::cl_keys::Field_Draw(
        &mut crate::src::client::cl_keys::g_consoleField as *mut _
            as *mut crate::qcommon_h::field_t,
        (con.xadjust + (2 as i32 * 8 as i32) as f32) as i32,
        y,
        640 as i32 - 3 as i32 * 8 as i32,
        crate::src::qcommon::q_shared::qtrue,
        crate::src::qcommon::q_shared::qtrue,
    );
}
/*
================
Con_DrawNotify

Draws the last few lines of output transparently over the game top
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_DrawNotify() {
    let mut x: i32 = 0;
    let mut v: i32 = 0;
    let mut text: *mut i16 = 0 as *mut i16;
    let mut i: i32 = 0;
    let mut time: i32 = 0;
    let mut skip: i32 = 0;
    let mut currentColor: i32 = 0;
    currentColor = 7 as i32;
    crate::src::client::cl_main::re
        .SetColor
        .expect("non-null function pointer")(
        crate::src::qcommon::q_math::g_color_table[currentColor as usize].as_mut_ptr(),
    );
    v = 0 as i32;
    i = con.current - 4 as i32 + 1 as i32;
    while i <= con.current {
        if !(i < 0 as i32) {
            time = con.times[(i % 4 as i32) as usize];
            if !(time == 0 as i32) {
                time = crate::src::client::cl_main::cls.realtime - time;
                if !(time as f32
                    > (*con_notifytime).value * 1000 as i32 as f32)
                {
                    text = con
                        .text
                        .as_mut_ptr()
                        .offset((i % con.totallines * con.linewidth) as isize);
                    if !(crate::src::client::cl_main::cl.snap.ps.pm_type
                        != crate::bg_public_h::PM_INTERMISSION as i32
                        && crate::src::client::cl_keys::Key_GetCatcher()
                            & (0x2 as i32 | 0x8 as i32)
                            != 0)
                    {
                        x = 0 as i32;
                        while x < con.linewidth {
                            if !(*text.offset(x as isize) as i32 & 0xff as i32
                                == ' ' as i32)
                            {
                                if *text.offset(x as isize) as i32 >> 8 as i32
                                    & 0x7 as i32
                                    != currentColor
                                {
                                    currentColor = *text.offset(x as isize) as i32
                                        >> 8 as i32
                                        & 0x7 as i32;
                                    crate::src::client::cl_main::re
                                        .SetColor
                                        .expect("non-null function pointer")(
                                        crate::src::qcommon::q_math::g_color_table
                                            [currentColor as usize]
                                            .as_mut_ptr(),
                                    );
                                }
                                crate::src::client::cl_scrn::SCR_DrawSmallChar(
                                    ((*crate::src::client::cl_main::cl_conXOffset).integer
                                        as f32
                                        + con.xadjust
                                        + ((x + 1 as i32) * 8 as i32)
                                            as f32)
                                        as i32,
                                    v,
                                    *text.offset(x as isize) as i32 & 0xff as i32,
                                );
                            }
                            x += 1
                        }
                        v += 16 as i32
                    }
                }
            }
        }
        i += 1
    }
    crate::src::client::cl_main::re
        .SetColor
        .expect("non-null function pointer")(0 as *const f32);
    if crate::src::client::cl_keys::Key_GetCatcher() & (0x2 as i32 | 0x8 as i32)
        != 0
    {
        return;
    }
    // draw the chat line
    if crate::src::client::cl_keys::Key_GetCatcher() & 0x4 as i32 != 0 {
        if crate::src::client::cl_keys::chat_team as u64 != 0 {
            crate::src::client::cl_scrn::SCR_DrawBigString(
                8 as i32,
                v,
                b"say_team:\x00" as *const u8 as *const libc::c_char,
                1.0f32,
                crate::src::qcommon::q_shared::qfalse,
            );
            skip = 10 as i32
        } else {
            crate::src::client::cl_scrn::SCR_DrawBigString(
                8 as i32,
                v,
                b"say:\x00" as *const u8 as *const libc::c_char,
                1.0f32,
                crate::src::qcommon::q_shared::qfalse,
            );
            skip = 5 as i32
        }
        crate::src::client::cl_keys::Field_BigDraw(
            &mut crate::src::client::cl_keys::chatField as *mut _ as *mut crate::qcommon_h::field_t,
            skip * 16 as i32,
            v,
            640 as i32 - (skip + 1 as i32) * 16 as i32,
            crate::src::qcommon::q_shared::qtrue,
            crate::src::qcommon::q_shared::qtrue,
        );
    };
}
/*
================
Con_DrawSolidConsole

Draws the console with the solid background
================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_DrawSolidConsole(mut frac: f32) {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut rows: i32 = 0;
    let mut text: *mut i16 = 0 as *mut i16;
    let mut row: i32 = 0;
    let mut lines: i32 = 0;
    //	qhandle_t		conShader;
    let mut currentColor: i32 = 0;
    let mut color: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    lines = (crate::src::client::cl_main::cls.glconfig.vidHeight as f32 * frac)
        as i32;
    if lines <= 0 as i32 {
        return;
    }
    if lines > crate::src::client::cl_main::cls.glconfig.vidHeight {
        lines = crate::src::client::cl_main::cls.glconfig.vidHeight
    }
    // on wide screens, we will center the text
    con.xadjust = 0 as i32 as f32;
    crate::src::client::cl_scrn::SCR_AdjustFrom640(
        &mut con.xadjust,
        0 as *mut f32,
        0 as *mut f32,
        0 as *mut f32,
    );
    // draw the background
    y = (frac * 480 as i32 as f32) as i32;
    if y < 1 as i32 {
        y = 0 as i32
    } else {
        crate::src::client::cl_scrn::SCR_DrawPic(
            0 as i32 as f32,
            0 as i32 as f32,
            640 as i32 as f32,
            y as f32,
            crate::src::client::cl_main::cls.consoleShader,
        );
    }
    color[0 as i32 as usize] = 1 as i32 as crate::src::qcommon::q_shared::vec_t;
    color[1 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    color[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    color[3 as i32 as usize] = 1 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::client::cl_scrn::SCR_FillRect(
        0 as i32 as f32,
        y as f32,
        640 as i32 as f32,
        2 as i32 as f32,
        color.as_mut_ptr(),
    );
    // draw the version number
    crate::src::client::cl_main::re
        .SetColor
        .expect("non-null function pointer")(
        crate::src::qcommon::q_math::g_color_table
            [('1' as i32 - '0' as i32 & 0x7 as i32) as usize]
            .as_mut_ptr(),
    );
    i = crate::stdlib::strlen(
        b"ioq3 1.36_GIT_d0fe4462-2020-01-10\x00" as *const u8 as *const libc::c_char,
    ) as i32;
    x = 0 as i32;
    while x < i {
        crate::src::client::cl_scrn::SCR_DrawSmallChar(
            crate::src::client::cl_main::cls.glconfig.vidWidth
                - (i - x + 1 as i32) * 8 as i32,
            lines - 16 as i32,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"ioq3 1.36_GIT_d0fe4462-2020-01-10\x00",
            ))[x as usize] as i32,
        );
        x += 1
    }
    // draw the text
    con.vislines = lines; // rows of text to draw
    rows = (lines - 16 as i32) / 16 as i32;
    y = lines - 16 as i32 * 3 as i32;
    // draw from the bottom up
    if con.display != con.current {
        // draw arrows to show the buffer is backscrolled
        crate::src::client::cl_main::re
            .SetColor
            .expect("non-null function pointer")(
            crate::src::qcommon::q_math::g_color_table
                [('1' as i32 - '0' as i32 & 0x7 as i32) as usize]
                .as_mut_ptr(),
        );
        x = 0 as i32;
        while x < con.linewidth {
            crate::src::client::cl_scrn::SCR_DrawSmallChar(
                (con.xadjust + ((x + 1 as i32) * 8 as i32) as f32)
                    as i32,
                y,
                '^' as i32,
            );
            x += 4 as i32
        }
        y -= 16 as i32;
        rows -= 1
    }
    row = con.display;
    if con.x == 0 as i32 {
        row -= 1
    }
    currentColor = 7 as i32;
    crate::src::client::cl_main::re
        .SetColor
        .expect("non-null function pointer")(
        crate::src::qcommon::q_math::g_color_table[currentColor as usize].as_mut_ptr(),
    );
    i = 0 as i32;
    while i < rows {
        if row < 0 as i32 {
            break;
        }
        if !(con.current - row >= con.totallines) {
            text = con
                .text
                .as_mut_ptr()
                .offset((row % con.totallines * con.linewidth) as isize);
            x = 0 as i32;
            while x < con.linewidth {
                if !(*text.offset(x as isize) as i32 & 0xff as i32 == ' ' as i32) {
                    if *text.offset(x as isize) as i32 >> 8 as i32
                        & 0x7 as i32
                        != currentColor
                    {
                        currentColor = *text.offset(x as isize) as i32 >> 8 as i32
                            & 0x7 as i32;
                        crate::src::client::cl_main::re
                            .SetColor
                            .expect("non-null function pointer")(
                            crate::src::qcommon::q_math::g_color_table[currentColor as usize]
                                .as_mut_ptr(),
                        );
                    }
                    crate::src::client::cl_scrn::SCR_DrawSmallChar(
                        (con.xadjust + ((x + 1 as i32) * 8 as i32) as f32)
                            as i32,
                        y,
                        *text.offset(x as isize) as i32 & 0xff as i32,
                    );
                }
                x += 1
            }
        }
        // past scrollback wrap point
        i += 1;
        y -= 16 as i32;
        row -= 1
    }
    // draw the input prompt, user text, and cursor if desired
    Con_DrawInput();
    crate::src::client::cl_main::re
        .SetColor
        .expect("non-null function pointer")(0 as *const f32);
}
/*
==================
Con_DrawConsole
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_DrawConsole() {
    // check for console width changes from a vid mode change
    Con_CheckResize();
    // if disconnected, render console full screen
    if crate::src::client::cl_main::clc.state as u32
        == crate::src::qcommon::q_shared::CA_DISCONNECTED as i32 as u32
    {
        if crate::src::client::cl_keys::Key_GetCatcher() & (0x2 as i32 | 0x8 as i32)
            == 0
        {
            Con_DrawSolidConsole(1.0f64 as f32);
            return;
        }
    }
    if con.displayFrac != 0. {
        Con_DrawSolidConsole(con.displayFrac);
    } else if crate::src::client::cl_main::clc.state as u32
        == crate::src::qcommon::q_shared::CA_ACTIVE as i32 as u32
    {
        Con_DrawNotify();
    };
}
// draw notify lines
//================================================================
/*
==================
Con_RunConsole

Scroll it up or down
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Con_RunConsole() {
    // decide on the destination height of the console
    if crate::src::client::cl_keys::Key_GetCatcher() & 0x1 as i32 != 0 {
        // none visible
        con.finalFrac = 0.5f64 as f32
    } else {
        con.finalFrac = 0 as i32 as f32
    } // half screen
      // scroll towards the destination height
    if con.finalFrac < con.displayFrac {
        con.displayFrac = (con.displayFrac as f64
            - ((*con_conspeed).value
                * crate::src::client::cl_main::cls.realFrametime as f32)
                as f64
                * 0.001f64) as f32; // none visible
        if con.finalFrac > con.displayFrac {
            con.displayFrac = con.finalFrac
        }
    } else if con.finalFrac > con.displayFrac {
        con.displayFrac = (con.displayFrac as f64
            + ((*con_conspeed).value
                * crate::src::client::cl_main::cls.realFrametime as f32)
                as f64
                * 0.001f64) as f32;
        if con.finalFrac < con.displayFrac {
            con.displayFrac = con.finalFrac
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn Con_PageUp() {
    con.display -= 2 as i32;
    if con.current - con.display >= con.totallines {
        con.display = con.current - con.totallines + 1 as i32
    };
}
#[no_mangle]

pub unsafe extern "C" fn Con_PageDown() {
    con.display += 2 as i32;
    if con.display > con.current {
        con.display = con.current
    };
}
#[no_mangle]

pub unsafe extern "C" fn Con_Top() {
    con.display = con.totallines;
    if con.current - con.display >= con.totallines {
        con.display = con.current - con.totallines + 1 as i32
    };
}
#[no_mangle]

pub unsafe extern "C" fn Con_Bottom() {
    con.display = con.current;
}
#[no_mangle]

pub unsafe extern "C" fn Con_Close() {
    if (*crate::src::qcommon::common::com_cl_running).integer == 0 {
        return;
    }
    crate::src::qcommon::common::Field_Clear(
        &mut crate::src::client::cl_keys::g_consoleField as *mut _
            as *mut crate::qcommon_h::field_t,
    );
    Con_ClearNotify();
    crate::src::client::cl_keys::Key_SetCatcher(
        crate::src::client::cl_keys::Key_GetCatcher() & !(0x1 as i32),
    );
    con.finalFrac = 0 as i32 as f32;
    con.displayFrac = 0 as i32 as f32;
}

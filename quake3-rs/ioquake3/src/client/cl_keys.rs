use ::libc;

pub mod ctype_h {

    #[inline]

    pub unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
        return if __c >= -(128 as i32) && __c < 256 as i32 {
            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

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
pub use crate::src::client::cl_keys::ctype_h::tolower;
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
pub use crate::stdlib::__ctype_b_loc;
pub use crate::stdlib::__ctype_tolower_loc;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::uint8_t;

pub use crate::keys_h::qkey_t;
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
pub use crate::src::qcommon::cmd::Cbuf_AddText;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc;
pub use crate::src::qcommon::common::con_autochat;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::CopyString;
pub use crate::src::qcommon::common::Field_AutoComplete;
pub use crate::src::qcommon::common::Field_Clear;
pub use crate::src::qcommon::common::Field_CompleteCommand;
pub use crate::src::qcommon::common::Field_CompleteKeyname;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::cvar::cvar_modifiedFlags;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_SetValue;
pub use crate::src::qcommon::cvar::Cvar_VariableIntegerValue;
pub use crate::src::qcommon::cvar::Cvar_VariableValue;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_FOpenFileRead;
pub use crate::src::qcommon::files::FS_FOpenFileWrite;
pub use crate::src::qcommon::files::FS_Printf;
pub use crate::src::qcommon::files::FS_Read;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::Com_HexStrToInt;
pub use crate::src::qcommon::q_shared::Com_SkipTokens;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_strcat;
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
pub use crate::src::qcommon::vm::VM_Call;
pub use crate::src::sys::sys_main::Sys_GetClipboardData;
pub use crate::vm_local_h::vm_s;

pub use crate::cg_public_h::CGAME_EVENT_EDITHUD;
pub use crate::cg_public_h::CGAME_EVENT_NONE;
pub use crate::cg_public_h::CGAME_EVENT_SCOREBOARD;
pub use crate::cg_public_h::CGAME_EVENT_TEAMMENU;
pub use crate::cg_public_h::CG_CONSOLE_COMMAND;
pub use crate::cg_public_h::CG_CROSSHAIR_PLAYER;
pub use crate::cg_public_h::CG_DRAW_ACTIVE_FRAME;
pub use crate::cg_public_h::CG_EVENT_HANDLING;
pub use crate::cg_public_h::CG_INIT;
pub use crate::cg_public_h::CG_KEY_EVENT;
pub use crate::cg_public_h::CG_LAST_ATTACKER;
pub use crate::cg_public_h::CG_MOUSE_EVENT;
pub use crate::cg_public_h::CG_SHUTDOWN;
pub use crate::client_h::clientConnection_t;
pub use crate::client_h::clientStatic_t;
pub use crate::client_h::serverInfo_t;
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
pub use crate::src::client::cl_console::g_console_field_width;
pub use crate::src::client::cl_console::Con_Bottom;
pub use crate::src::client::cl_console::Con_PageDown;
pub use crate::src::client::cl_console::Con_PageUp;
pub use crate::src::client::cl_console::Con_ToggleConsole_f;
pub use crate::src::client::cl_console::Con_Top;
pub use crate::src::client::cl_main::cgvm;
pub use crate::src::client::cl_main::clc;
pub use crate::src::client::cl_main::cls;
pub use crate::src::client::cl_main::CL_AddReliableCommand;
pub use crate::src::client::cl_main::CL_Disconnect_f;
pub use crate::src::client::cl_scrn::SCR_DrawBigString;
pub use crate::src::client::cl_scrn::SCR_DrawSmallChar;
pub use crate::src::client::cl_scrn::SCR_DrawSmallStringExt;
pub use crate::src::client::cl_scrn::SCR_UpdateScreen;
pub use crate::src::client::cl_ui::uivm;

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

pub use crate::src::client::cl_keys::stdlib_h::atoi;

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

pub use ::libc::strtol;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct keyname_t {
    pub name: *mut libc::c_char,
    pub keynum: i32,
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

key up events are sent even if in console mode

*/
#[no_mangle]

pub static mut historyEditLines: [crate::qcommon_h::field_t; 32] = [crate::qcommon_h::field_t {
    cursor: 0,
    scroll: 0,
    widthInChars: 0,
    buffer: [0; 256],
}; 32];
#[no_mangle]

pub static mut nextHistoryLine: i32 = 0;
// the last line in the history buffer, not masked
#[no_mangle]

pub static mut historyLine: i32 = 0;
// the line being displayed from history buffer
// will be <= nextHistoryLine
#[no_mangle]

pub static mut g_consoleField: crate::qcommon_h::field_t = crate::qcommon_h::field_t {
    cursor: 0,
    scroll: 0,
    widthInChars: 0,
    buffer: [0; 256],
};
#[no_mangle]

pub static mut chatField: crate::qcommon_h::field_t = crate::qcommon_h::field_t {
    cursor: 0,
    scroll: 0,
    widthInChars: 0,
    buffer: [0; 256],
};
#[no_mangle]

pub static mut chat_team: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub static mut chat_playerNum: i32 = 0;
#[no_mangle]

pub static mut key_overstrikeMode: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub static mut anykeydown: i32 = 0;
#[no_mangle]

pub static mut keys: [crate::keys_h::qkey_t; 366] = [crate::keys_h::qkey_t {
    down: crate::src::qcommon::q_shared::qfalse,
    repeats: 0,
    binding: 0 as *const libc::c_char as *mut libc::c_char,
}; 366];
// names not in this list can either be lowercase ascii, or '0xnn' hex sequences
#[no_mangle]

pub static mut keynames: [keyname_t; 244] = [
    {
        let mut init = keyname_t {
            name: b"TAB\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_TAB as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"ENTER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_ENTER as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"ESCAPE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_ESCAPE as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"SPACE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_SPACE as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"BACKSPACE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_BACKSPACE as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"UPARROW\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_UPARROW as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"DOWNARROW\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_DOWNARROW as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"LEFTARROW\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_LEFTARROW as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"RIGHTARROW\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_RIGHTARROW as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"ALT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_ALT as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"CTRL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_CTRL as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"SHIFT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_SHIFT as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"COMMAND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_COMMAND as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"CAPSLOCK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_CAPSLOCK as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F1 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F2 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F3 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F4 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F5 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F6 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F7 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F8 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F9 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F10\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F10 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F11\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F11 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F12\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F12 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F13\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F13 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F14\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F14 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F15\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_F15 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"INS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_INS as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"DEL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_DEL as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PGDN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PGDN as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PGUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PGUP as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"HOME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_HOME as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"END\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_END as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MOUSE1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_MOUSE1 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MOUSE2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_MOUSE2 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MOUSE3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_MOUSE3 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MOUSE4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_MOUSE4 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MOUSE5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_MOUSE5 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MWHEELUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_MWHEELUP as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MWHEELDOWN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_MWHEELDOWN as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY1 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY2 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY3 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY4 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY5 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY6 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY7 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY8 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY9 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY10\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY10 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY11\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY11 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY12\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY12 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY13\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY13 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY14\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY14 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY15\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY15 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY16\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY16 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY17\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY17 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY18\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY18 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY19\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY19 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY20\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY20 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY21\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY21 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY22\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY22 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY23\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY23 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY24\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY24 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY25\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY25 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY26\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY26 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY27\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY27 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY28\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY28 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY29\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY29 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY30\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY30 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY31\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY31 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY32\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_JOY32 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX1 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX2 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX3 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX4 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX5 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX6 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX7 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX8 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX9 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX10\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX10 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX11\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX11 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX12\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX12 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX13\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX13 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX14\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX14 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX15\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX15 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX16\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_AUX16 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_HOME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_HOME as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_UPARROW\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_UPARROW as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_PGUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_PGUP as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_LEFTARROW\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_LEFTARROW as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_5 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_RIGHTARROW\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_RIGHTARROW as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_END\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_END as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_DOWNARROW\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_DOWNARROW as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_PGDN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_PGDN as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_ENTER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_ENTER as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_INS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_INS as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_DEL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_DEL as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_SLASH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_SLASH as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_MINUS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_MINUS as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_PLUS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_PLUS as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_NUMLOCK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_NUMLOCK as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_STAR\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_STAR as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_EQUALS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_KP_EQUALS as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAUSE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAUSE as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"SEMICOLON\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: ';' as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_0 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_1 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_2 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_3 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_4 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_5 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_6 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_7 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_8 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_9 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_10\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_10 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_11\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_11 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_12\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_12 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_13\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_13 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_14\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_14 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_15\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_15 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_16\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_16 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_17\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_17 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_18\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_18 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_19\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_19 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_20\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_20 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_21\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_21 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_22\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_22 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_23\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_23 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_24\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_24 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_25\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_25 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_26\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_26 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_27\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_27 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_28\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_28 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_29\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_29 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_30\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_30 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_31\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_31 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_32\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_32 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_33\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_33 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_34\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_34 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_35\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_35 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_36\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_36 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_37\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_37 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_38\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_38 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_39\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_39 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_40\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_40 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_41\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_41 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_42\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_42 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_43\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_43 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_44\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_44 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_45\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_45 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_46\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_46 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_47\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_47 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_48\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_48 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_49\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_49 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_50\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_50 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_51\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_51 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_52\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_52 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_53\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_53 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_54\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_54 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_55\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_55 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_56\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_56 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_57\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_57 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_58\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_58 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_59\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_59 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_60\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_60 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_61\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_61 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_62\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_62 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_63\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_63 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_64\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_64 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_65\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_65 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_66\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_66 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_67\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_67 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_68\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_68 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_69\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_69 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_70\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_70 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_71\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_71 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_72\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_72 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_73\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_73 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_74\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_74 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_75\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_75 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_76\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_76 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_77\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_77 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_78\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_78 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_79\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_79 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_80\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_80 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_81\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_81 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_82\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_82 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_83\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_83 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_84\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_84 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_85\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_85 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_86\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_86 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_87\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_87 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_88\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_88 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_89\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_89 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_90\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_90 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_91\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_91 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_92\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_92 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_93\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_93 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_94\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_94 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WORLD_95\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_WORLD_95 as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"WINDOWS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_SUPER as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"COMPOSE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_COMPOSE as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MODE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_MODE as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"HELP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_HELP as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PRINT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PRINT as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"SYSREQ\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_SYSREQ as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"SCROLLOCK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_SCROLLOCK as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"BREAK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_BREAK as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MENU\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_MENU as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"POWER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_POWER as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"EURO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_EURO as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"UNDO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_UNDO as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_A\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_A as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_B\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_B as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_X\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_X as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_Y\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_Y as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_BACK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_BACK as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_GUIDE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_GUIDE as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_START\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_START as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_LEFTSTICK_CLICK\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_LEFTSTICK_CLICK as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_RIGHTSTICK_CLICK\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_RIGHTSTICK_CLICK as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_LEFTSHOULDER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_LEFTSHOULDER as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_RIGHTSHOULDER\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_RIGHTSHOULDER as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_DPAD_UP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_DPAD_UP as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_DPAD_DOWN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_DPAD_DOWN as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_DPAD_LEFT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_DPAD_LEFT as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_DPAD_RIGHT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_DPAD_RIGHT as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_LEFTSTICK_LEFT\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_LEFTSTICK_LEFT as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_LEFTSTICK_RIGHT\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_LEFTSTICK_RIGHT as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_LEFTSTICK_UP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_LEFTSTICK_UP as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_LEFTSTICK_DOWN\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_LEFTSTICK_DOWN as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_RIGHTSTICK_LEFT\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_RIGHTSTICK_LEFT as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_RIGHTSTICK_RIGHT\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_RIGHTSTICK_RIGHT as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_RIGHTSTICK_UP\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_RIGHTSTICK_UP as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_RIGHTSTICK_DOWN\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_RIGHTSTICK_DOWN as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_LEFTTRIGGER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_LEFTTRIGGER as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAD0_RIGHTTRIGGER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: crate::keycodes_h::K_PAD0_RIGHTTRIGGER as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            keynum: 0 as i32,
        };
        init
    },
];
/*
=============================================================================

EDIT FIELDS

=============================================================================
*/
/*
===================
Field_Draw

Handles horizontal scrolling and cursor blinking
x, y, and width are in pixels
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Field_VariableSizeDraw(
    mut edit: *mut crate::qcommon_h::field_t,
    mut x: i32,
    mut y: i32,
    mut _width: i32,
    mut size: i32,
    mut showCursor: crate::src::qcommon::q_shared::qboolean,
    mut noColorEscape: crate::src::qcommon::q_shared::qboolean,
) {
    let mut len: i32 = 0; // - 1 so there is always a space for the cursor
    let mut drawLen: i32 = 0;
    let mut prestep: i32 = 0;
    let mut cursorChar: i32 = 0;
    let mut str: [libc::c_char; 1024] = [0; 1024];
    let mut i: i32 = 0;
    drawLen = (*edit).widthInChars - 1 as i32;
    len = crate::stdlib::strlen((*edit).buffer.as_mut_ptr()) as i32;
    // guarantee that cursor will be visible
    if len <= drawLen {
        prestep = 0 as i32
    } else {
        if (*edit).scroll + drawLen > len {
            (*edit).scroll = len - drawLen;
            if (*edit).scroll < 0 as i32 {
                (*edit).scroll = 0 as i32
            }
        }
        prestep = (*edit).scroll
    }
    if prestep + drawLen > len {
        drawLen = len - prestep
    }
    // extract <drawLen> characters from the field at <prestep>
    if drawLen >= 1024 as i32 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"drawLen >= MAX_STRING_CHARS\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::stdlib::memcpy(
        str.as_mut_ptr() as *mut libc::c_void,
        (*edit).buffer.as_mut_ptr().offset(prestep as isize) as *const libc::c_void,
        drawLen as libc::c_ulong,
    );
    str[drawLen as usize] = 0 as i32 as libc::c_char;
    // draw it
    if size == 8 as i32 {
        let mut color: [f32; 4] = [0.; 4];
        color[3 as i32 as usize] = 1.0f64 as f32;
        color[2 as i32 as usize] = color[3 as i32 as usize];
        color[1 as i32 as usize] = color[2 as i32 as usize];
        color[0 as i32 as usize] = color[1 as i32 as usize];
        crate::src::client::cl_scrn::SCR_DrawSmallStringExt(
            x,
            y,
            str.as_mut_ptr(),
            color.as_mut_ptr(),
            crate::src::qcommon::q_shared::qfalse,
            noColorEscape,
        );
    } else {
        // draw big string with drop shadow
        crate::src::client::cl_scrn::SCR_DrawBigString(
            x,
            y,
            str.as_mut_ptr(),
            1.0f64 as f32,
            noColorEscape,
        );
    }
    // draw the cursor
    if showCursor as u64 != 0 {
        if crate::src::client::cl_main::cls.realtime >> 8 as i32 & 1 as i32 != 0 {
            return;
            // off blink
        }
        if key_overstrikeMode as u64 != 0 {
            cursorChar = 11 as i32
        } else {
            cursorChar = 10 as i32
        }
        i = (drawLen as libc::c_ulong).wrapping_sub(crate::stdlib::strlen(str.as_mut_ptr())) as i32;
        if size == 8 as i32 {
            crate::src::client::cl_scrn::SCR_DrawSmallChar(
                x + ((*edit).cursor - prestep - i) * size,
                y,
                cursorChar,
            );
        } else {
            str[0 as i32 as usize] = cursorChar as libc::c_char;
            str[1 as i32 as usize] = 0 as i32 as libc::c_char;
            crate::src::client::cl_scrn::SCR_DrawBigString(
                x + ((*edit).cursor - prestep - i) * size,
                y,
                str.as_mut_ptr(),
                1.0f64 as f32,
                crate::src::qcommon::q_shared::qfalse,
            );
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn Field_Draw(
    mut edit: *mut crate::qcommon_h::field_t,
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut showCursor: crate::src::qcommon::q_shared::qboolean,
    mut noColorEscape: crate::src::qcommon::q_shared::qboolean,
) {
    Field_VariableSizeDraw(edit, x, y, width, 8 as i32, showCursor, noColorEscape);
}
#[no_mangle]

pub unsafe extern "C" fn Field_BigDraw(
    mut edit: *mut crate::qcommon_h::field_t,
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut showCursor: crate::src::qcommon::q_shared::qboolean,
    mut noColorEscape: crate::src::qcommon::q_shared::qboolean,
) {
    Field_VariableSizeDraw(edit, x, y, width, 16 as i32, showCursor, noColorEscape);
}
/*
================
Field_Paste
================
*/
#[no_mangle]

pub unsafe extern "C" fn Field_Paste(mut edit: *mut crate::qcommon_h::field_t) {
    let mut cbd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pasteLen: i32 = 0;
    let mut i: i32 = 0;
    cbd = crate::src::sys::sys_main::Sys_GetClipboardData();
    if cbd.is_null() {
        return;
    }
    // send as if typed, so insert / overstrike works properly
    pasteLen = crate::stdlib::strlen(cbd) as i32;
    i = 0 as i32;
    while i < pasteLen {
        Field_CharEvent(edit, *cbd.offset(i as isize) as i32);
        i += 1
    }
    crate::src::qcommon::common::Z_Free(cbd as *mut libc::c_void);
}
/*
=================
Field_KeyDownEvent

Performs the basic line editing functions for the console,
in-game talk, and menu fields

Key events are used for non-printable characters, others are gotten from char events.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Field_KeyDownEvent(
    mut edit: *mut crate::qcommon_h::field_t,
    mut key: i32,
) {
    let mut len: i32 = 0;
    // shift-insert is paste
    if (key == crate::keycodes_h::K_INS as i32 || key == crate::keycodes_h::K_KP_INS as i32)
        && keys[crate::keycodes_h::K_SHIFT as i32 as usize].down as u32 != 0
    {
        Field_Paste(edit);
        return;
    }
    key = {
        let mut __res: i32 = 0;
        if ::std::mem::size_of::<i32>() as libc::c_ulong > 1 as i32 as libc::c_ulong {
            if 0 != 0 {
                let mut __c: i32 = key;
                __res = if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                }
            } else {
                __res = tolower(key)
            }
        } else {
            __res = *(*crate::stdlib::__ctype_tolower_loc()).offset(key as isize)
        }
        __res
    };
    len = crate::stdlib::strlen((*edit).buffer.as_mut_ptr()) as i32;
    match key {
        140 => {
            if (*edit).cursor < len {
                crate::stdlib::memmove(
                    (*edit).buffer.as_mut_ptr().offset((*edit).cursor as isize)
                        as *mut libc::c_void,
                    (*edit)
                        .buffer
                        .as_mut_ptr()
                        .offset((*edit).cursor as isize)
                        .offset(1 as i32 as isize) as *const libc::c_void,
                    (len - (*edit).cursor) as libc::c_ulong,
                );
            }
        }
        135 => {
            if (*edit).cursor < len {
                (*edit).cursor += 1
            }
        }
        134 => {
            if (*edit).cursor > 0 as i32 {
                (*edit).cursor -= 1
            }
        }
        143 => (*edit).cursor = 0 as i32,
        144 => (*edit).cursor = len,
        139 => {
            key_overstrikeMode =
                (key_overstrikeMode as u64 == 0) as i32 as crate::src::qcommon::q_shared::qboolean
        }
        _ => {}
    }
    // Change scroll if cursor is no longer visible
    if (*edit).cursor < (*edit).scroll {
        (*edit).scroll = (*edit).cursor
    } else if (*edit).cursor >= (*edit).scroll + (*edit).widthInChars && (*edit).cursor <= len {
        (*edit).scroll = (*edit).cursor - (*edit).widthInChars + 1 as i32
    };
}
/*
==================
Field_CharEvent
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Field_CharEvent(mut edit: *mut crate::qcommon_h::field_t, mut ch: i32) {
    let mut len: i32 = 0;
    if ch == 'v' as i32 - 'a' as i32 + 1 as i32 {
        // ctrl-v is paste
        Field_Paste(edit);
        return;
    }
    if ch == 'c' as i32 - 'a' as i32 + 1 as i32 {
        // ctrl-c clears the field
        crate::src::qcommon::common::Field_Clear(edit as *mut crate::qcommon_h::field_t);
        return;
    }
    len = crate::stdlib::strlen((*edit).buffer.as_mut_ptr()) as i32;
    if ch == 'h' as i32 - 'a' as i32 + 1 as i32 {
        // ctrl-h is backspace
        if (*edit).cursor > 0 as i32 {
            crate::stdlib::memmove(
                (*edit)
                    .buffer
                    .as_mut_ptr()
                    .offset((*edit).cursor as isize)
                    .offset(-(1 as i32 as isize)) as *mut libc::c_void,
                (*edit).buffer.as_mut_ptr().offset((*edit).cursor as isize) as *const libc::c_void,
                (len + 1 as i32 - (*edit).cursor) as libc::c_ulong,
            );
            (*edit).cursor -= 1;
            if (*edit).cursor < (*edit).scroll {
                (*edit).scroll -= 1
            }
        }
        return;
    }
    if ch == 'a' as i32 - 'a' as i32 + 1 as i32 {
        // ctrl-a is home
        (*edit).cursor = 0 as i32;
        (*edit).scroll = 0 as i32;
        return;
    }
    if ch == 'e' as i32 - 'a' as i32 + 1 as i32 {
        // ctrl-e is end
        (*edit).cursor = len;
        (*edit).scroll = (*edit).cursor - (*edit).widthInChars;
        return;
    }
    //
    // ignore any other non printable chars
    //
    if ch < 32 as i32 {
        return;
    } // insert mode
    if key_overstrikeMode as u64 != 0 {
        // - 2 to leave room for the leading slash and trailing \0
        if (*edit).cursor == 256 as i32 - 2 as i32 {
            return;
        }
        (*edit).buffer[(*edit).cursor as usize] = ch as libc::c_char;
        (*edit).cursor += 1
    } else {
        // - 2 to leave room for the leading slash and trailing \0
        if len == 256 as i32 - 2 as i32 {
            return;
            // all full
        }
        crate::stdlib::memmove(
            (*edit)
                .buffer
                .as_mut_ptr()
                .offset((*edit).cursor as isize)
                .offset(1 as i32 as isize) as *mut libc::c_void,
            (*edit).buffer.as_mut_ptr().offset((*edit).cursor as isize) as *const libc::c_void,
            (len + 1 as i32 - (*edit).cursor) as libc::c_ulong,
        );
        (*edit).buffer[(*edit).cursor as usize] = ch as libc::c_char;
        (*edit).cursor += 1
    }
    if (*edit).cursor >= (*edit).widthInChars {
        (*edit).scroll += 1
    }
    if (*edit).cursor == len + 1 as i32 {
        (*edit).buffer[(*edit).cursor as usize] = 0 as i32 as libc::c_char
    };
}
/*
=============================================================================

CONSOLE LINE EDITING

==============================================================================
*/
/*
====================
Console_Key

Handles history and console scrollback
====================
*/
#[no_mangle]

pub unsafe extern "C" fn Console_Key(mut key: i32) {
    // ctrl-L clears screen
    if key == 'l' as i32 && keys[crate::keycodes_h::K_CTRL as i32 as usize].down as u32 != 0 {
        crate::src::qcommon::cmd::Cbuf_AddText(b"clear\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    // enter finishes the line
    if key == crate::keycodes_h::K_ENTER as i32 || key == crate::keycodes_h::K_KP_ENTER as i32 {
        // if not in the game explicitly prepend a slash if needed
        if crate::src::client::cl_main::clc.state as u32
            != crate::src::qcommon::q_shared::CA_ACTIVE as i32 as u32
            && (*crate::src::qcommon::common::con_autochat).integer != 0
            && g_consoleField.buffer[0 as i32 as usize] as i32 != 0
            && g_consoleField.buffer[0 as i32 as usize] as i32 != '\\' as i32
            && g_consoleField.buffer[0 as i32 as usize] as i32 != '/' as i32
        {
            let mut temp: [libc::c_char; 255] = [0; 255];
            crate::src::qcommon::q_shared::Q_strncpyz(
                temp.as_mut_ptr(),
                g_consoleField.buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong as i32,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                g_consoleField.buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
                b"\\%s\x00" as *const u8 as *const libc::c_char,
                temp.as_mut_ptr(),
            );
            g_consoleField.cursor += 1
        }
        crate::src::qcommon::common::Com_Printf(
            b"]%s\n\x00" as *const u8 as *const libc::c_char,
            g_consoleField.buffer.as_mut_ptr(),
        );
        // leading slash is an explicit command
        if g_consoleField.buffer[0 as i32 as usize] as i32 == '\\' as i32
            || g_consoleField.buffer[0 as i32 as usize] as i32 == '/' as i32
        {
            crate::src::qcommon::cmd::Cbuf_AddText(
                g_consoleField.buffer.as_mut_ptr().offset(1 as i32 as isize),
            ); // valid command
            crate::src::qcommon::cmd::Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
        } else if g_consoleField.buffer[0 as i32 as usize] == 0 {
            return;
        // other text will be chat messages
        // empty lines just scroll the console without adding to history
        } else {
            if (*crate::src::qcommon::common::con_autochat).integer != 0 {
                crate::src::qcommon::cmd::Cbuf_AddText(
                    b"cmd say \x00" as *const u8 as *const libc::c_char,
                );
            }
            crate::src::qcommon::cmd::Cbuf_AddText(g_consoleField.buffer.as_mut_ptr());
            crate::src::qcommon::cmd::Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        // copy line to history buffer
        historyEditLines[(nextHistoryLine % 32 as i32) as usize] = g_consoleField; // may take some time
        nextHistoryLine += 1;
        historyLine = nextHistoryLine;
        crate::src::qcommon::common::Field_Clear(
            &mut g_consoleField as *mut _ as *mut crate::qcommon_h::field_t,
        );
        g_consoleField.widthInChars = crate::src::client::cl_console::g_console_field_width;
        CL_SaveConsoleHistory();
        if crate::src::client::cl_main::clc.state as u32
            == crate::src::qcommon::q_shared::CA_DISCONNECTED as i32 as u32
        {
            crate::src::client::cl_scrn::SCR_UpdateScreen();
            // force an update, because the command
        }
        return;
    }
    // command completion
    if key == crate::keycodes_h::K_TAB as i32 {
        crate::src::qcommon::common::Field_AutoComplete(
            &mut g_consoleField as *mut _ as *mut crate::qcommon_h::field_t,
        );
        return;
    }
    // command history (ctrl-p ctrl-n for unix style)
    if key == crate::keycodes_h::K_MWHEELUP as i32
        && keys[crate::keycodes_h::K_SHIFT as i32 as usize].down as u32 != 0
        || key == crate::keycodes_h::K_UPARROW as i32
        || key == crate::keycodes_h::K_KP_UPARROW as i32
        || ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<i32>() as libc::c_ulong > 1 as i32 as libc::c_ulong {
                if 0 != 0 {
                    let mut __c: i32 = key;
                    __res = if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower(key)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_tolower_loc()).offset(key as isize)
            }
            __res
        }) == 'p' as i32
            && keys[crate::keycodes_h::K_CTRL as i32 as usize].down as u32 != 0
    {
        if nextHistoryLine - historyLine < 32 as i32 && historyLine > 0 as i32 {
            historyLine -= 1
        }
        g_consoleField = historyEditLines[(historyLine % 32 as i32) as usize];
        return;
    }
    if key == crate::keycodes_h::K_MWHEELDOWN as i32
        && keys[crate::keycodes_h::K_SHIFT as i32 as usize].down as u32 != 0
        || key == crate::keycodes_h::K_DOWNARROW as i32
        || key == crate::keycodes_h::K_KP_DOWNARROW as i32
        || ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<i32>() as libc::c_ulong > 1 as i32 as libc::c_ulong {
                if 0 != 0 {
                    let mut __c: i32 = key;
                    __res = if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower(key)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_tolower_loc()).offset(key as isize)
            }
            __res
        }) == 'n' as i32
            && keys[crate::keycodes_h::K_CTRL as i32 as usize].down as u32 != 0
    {
        historyLine += 1;
        if historyLine >= nextHistoryLine {
            historyLine = nextHistoryLine;
            crate::src::qcommon::common::Field_Clear(
                &mut g_consoleField as *mut _ as *mut crate::qcommon_h::field_t,
            );
            g_consoleField.widthInChars = crate::src::client::cl_console::g_console_field_width;
            return;
        }
        g_consoleField = historyEditLines[(historyLine % 32 as i32) as usize];
        return;
    }
    // console scrolling
    if key == crate::keycodes_h::K_PGUP as i32 {
        crate::src::client::cl_console::Con_PageUp();
        return;
    }
    if key == crate::keycodes_h::K_PGDN as i32 {
        crate::src::client::cl_console::Con_PageDown();
        return;
    }
    if key == crate::keycodes_h::K_MWHEELUP as i32 {
        //----(SA)	added some mousewheel functionality to the console
        crate::src::client::cl_console::Con_PageUp();
        if keys[crate::keycodes_h::K_CTRL as i32 as usize].down as u64 != 0 {
            // hold <ctrl> to accelerate scrolling
            crate::src::client::cl_console::Con_PageUp();
            crate::src::client::cl_console::Con_PageUp();
        }
        return;
    }
    if key == crate::keycodes_h::K_MWHEELDOWN as i32 {
        //----(SA)	added some mousewheel functionality to the console
        crate::src::client::cl_console::Con_PageDown();
        if keys[crate::keycodes_h::K_CTRL as i32 as usize].down as u64 != 0 {
            // hold <ctrl> to accelerate scrolling
            crate::src::client::cl_console::Con_PageDown();
            crate::src::client::cl_console::Con_PageDown();
        }
        return;
    }
    // ctrl-home = top of console
    if key == crate::keycodes_h::K_HOME as i32
        && keys[crate::keycodes_h::K_CTRL as i32 as usize].down as u32 != 0
    {
        crate::src::client::cl_console::Con_Top();
        return;
    }
    // ctrl-end = bottom of console
    if key == crate::keycodes_h::K_END as i32
        && keys[crate::keycodes_h::K_CTRL as i32 as usize].down as u32 != 0
    {
        crate::src::client::cl_console::Con_Bottom();
        return;
    }
    // pass to the normal editline routine
    Field_KeyDownEvent(&mut g_consoleField, key);
}
//============================================================================
/*
================
Message_Key

In game talk message
================
*/
#[no_mangle]

pub unsafe extern "C" fn Message_Key(mut key: i32) {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    if key == crate::keycodes_h::K_ESCAPE as i32 {
        Key_SetCatcher(Key_GetCatcher() & !(0x4 as i32));
        crate::src::qcommon::common::Field_Clear(
            &mut chatField as *mut _ as *mut crate::qcommon_h::field_t,
        );
        return;
    }
    if key == crate::keycodes_h::K_ENTER as i32 || key == crate::keycodes_h::K_KP_ENTER as i32 {
        if chatField.buffer[0 as i32 as usize] as i32 != 0
            && crate::src::client::cl_main::clc.state as u32
                == crate::src::qcommon::q_shared::CA_ACTIVE as i32 as u32
        {
            if chat_playerNum != -(1 as i32) {
                crate::src::qcommon::q_shared::Com_sprintf(
                    buffer.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
                    b"tell %i \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                    chat_playerNum,
                    chatField.buffer.as_mut_ptr(),
                );
            } else if chat_team as u64 != 0 {
                crate::src::qcommon::q_shared::Com_sprintf(
                    buffer.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
                    b"say_team \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                    chatField.buffer.as_mut_ptr(),
                );
            } else {
                crate::src::qcommon::q_shared::Com_sprintf(
                    buffer.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
                    b"say \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                    chatField.buffer.as_mut_ptr(),
                );
            }
            crate::src::client::cl_main::CL_AddReliableCommand(
                buffer.as_mut_ptr(),
                crate::src::qcommon::q_shared::qfalse,
            );
        }
        Key_SetCatcher(Key_GetCatcher() & !(0x4 as i32));
        crate::src::qcommon::common::Field_Clear(
            &mut chatField as *mut _ as *mut crate::qcommon_h::field_t,
        );
        return;
    }
    Field_KeyDownEvent(&mut chatField, key);
}
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn Key_GetOverstrikeMode() -> crate::src::qcommon::q_shared::qboolean {
    return key_overstrikeMode;
}
#[no_mangle]

pub unsafe extern "C" fn Key_SetOverstrikeMode(mut state: crate::src::qcommon::q_shared::qboolean) {
    key_overstrikeMode = state;
}
/*
===================
Key_IsDown
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Key_IsDown(mut keynum: i32) -> crate::src::qcommon::q_shared::qboolean {
    if keynum < 0 as i32 || keynum >= crate::keycodes_h::MAX_KEYS as i32 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return keys[keynum as usize].down;
}
/*
===================
Key_StringToKeynum

Returns a key number to be used to index keys[] by looking at
the given string.  Single ascii characters return themselves, while
the K_* names are matched up.

0x11 will be interpreted as raw hex, which will allow new controlers

to be configured even if they don't have defined names.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Key_StringToKeynum(mut str: *mut libc::c_char) -> i32 {
    let mut kn: *mut keyname_t = 0 as *mut keyname_t;
    let mut n: i32 = 0;
    if str.is_null() || *str.offset(0 as i32 as isize) == 0 {
        return -(1 as i32);
    }
    if *str.offset(1 as i32 as isize) == 0 {
        return {
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong > 1 as i32 as libc::c_ulong {
                if 0 != 0 {
                    let mut __c: i32 = *str.offset(0 as i32 as isize) as i32;
                    __res = if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower(*str.offset(0 as i32 as isize) as i32)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_tolower_loc())
                    .offset(*str.offset(0 as i32 as isize) as i32 as isize)
            }
            __res
        };
    }
    // check for hex code
    n = crate::src::qcommon::q_shared::Com_HexStrToInt(str);
    if n >= 0 as i32 && n < crate::keycodes_h::MAX_KEYS as i32 {
        return n;
    }
    // scan for a text match
    kn = keynames.as_mut_ptr();
    while !(*kn).name.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(str, (*kn).name) == 0 {
            return (*kn).keynum;
        }
        kn = kn.offset(1)
    }
    return -(1 as i32);
}
/*
===================
Key_KeynumToString

Returns a string (either a single ascii char, a K_* name, or a 0x11 hex string) for the
given keynum.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Key_KeynumToString(mut keynum: i32) -> *mut libc::c_char {
    let mut kn: *mut keyname_t = 0 as *mut keyname_t;
    static mut tinystr: [libc::c_char; 5] = [0; 5];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if keynum == -(1 as i32) {
        return b"<KEY NOT FOUND>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if keynum < 0 as i32 || keynum >= crate::keycodes_h::MAX_KEYS as i32 {
        return b"<OUT OF RANGE>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    // check for printable ascii (don't use quote)
    if keynum > 32 as i32 && keynum < 127 as i32 && keynum != '\"' as i32 && keynum != ';' as i32 {
        tinystr[0 as i32 as usize] = keynum as libc::c_char;
        tinystr[1 as i32 as usize] = 0 as i32 as libc::c_char;
        return tinystr.as_mut_ptr();
    }
    // check for a key string
    kn = keynames.as_mut_ptr();
    while !(*kn).name.is_null() {
        if keynum == (*kn).keynum {
            return (*kn).name;
        }
        kn = kn.offset(1)
    }
    // make a hex string
    i = keynum >> 4 as i32;
    j = keynum & 15 as i32;
    tinystr[0 as i32 as usize] = '0' as i32 as libc::c_char;
    tinystr[1 as i32 as usize] = 'x' as i32 as libc::c_char;
    tinystr[2 as i32 as usize] = if i > 9 as i32 {
        (i - 10 as i32) + 'a' as i32
    } else {
        (i) + '0' as i32
    } as libc::c_char;
    tinystr[3 as i32 as usize] = if j > 9 as i32 {
        (j - 10 as i32) + 'a' as i32
    } else {
        (j) + '0' as i32
    } as libc::c_char;
    tinystr[4 as i32 as usize] = 0 as i32 as libc::c_char;
    return tinystr.as_mut_ptr();
}
/*
===================
Key_SetBinding
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Key_SetBinding(mut keynum: i32, mut binding: *const libc::c_char) {
    if keynum < 0 as i32 || keynum >= crate::keycodes_h::MAX_KEYS as i32 {
        return;
    }
    // free old bindings
    if !keys[keynum as usize].binding.is_null() {
        crate::src::qcommon::common::Z_Free(keys[keynum as usize].binding as *mut libc::c_void);
    }
    // allocate memory for new binding
    keys[keynum as usize].binding = crate::src::qcommon::common::CopyString(binding);
    // consider this like modifying an archived cvar, so the
    // file write will be triggered at the next opportunity
    crate::src::qcommon::cvar::cvar_modifiedFlags |= 0x1 as i32;
}
/*
===================
Key_GetBinding
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Key_GetBinding(mut keynum: i32) -> *mut libc::c_char {
    if keynum < 0 as i32 || keynum >= crate::keycodes_h::MAX_KEYS as i32 {
        return b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return keys[keynum as usize].binding;
}
/*
===================
Key_GetKey
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Key_GetKey(mut binding: *const libc::c_char) -> i32 {
    let mut i: i32 = 0;
    if !binding.is_null() {
        i = 0 as i32;
        while i < crate::keycodes_h::MAX_KEYS as i32 {
            if !keys[i as usize].binding.is_null()
                && crate::src::qcommon::q_shared::Q_stricmp(binding, keys[i as usize].binding)
                    == 0 as i32
            {
                return i;
            }
            i += 1
        }
    }
    return -(1 as i32);
}
/*
===================
Key_Unbind_f
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Key_Unbind_f() {
    let mut b: i32 = 0;
    if crate::src::qcommon::cmd::Cmd_Argc() != 2 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"unbind <key> : remove commands from a key\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    b = Key_StringToKeynum(crate::src::qcommon::cmd::Cmd_Argv(1 as i32));
    if b == -(1 as i32) {
        crate::src::qcommon::common::Com_Printf(
            b"\"%s\" isn\'t a valid key\n\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::cmd::Cmd_Argv(1 as i32),
        );
        return;
    }
    Key_SetBinding(b, b"\x00" as *const u8 as *const libc::c_char);
}
/*
===================
Key_Unbindall_f
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Key_Unbindall_f() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < crate::keycodes_h::MAX_KEYS as i32 {
        if !keys[i as usize].binding.is_null() {
            Key_SetBinding(i, b"\x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    }
}
/*
===================
Key_Bind_f
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Key_Bind_f() {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut b: i32 = 0;
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    c = crate::src::qcommon::cmd::Cmd_Argc();
    if c < 2 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"bind <key> [command] : attach a command to a key\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    b = Key_StringToKeynum(crate::src::qcommon::cmd::Cmd_Argv(1 as i32));
    if b == -(1 as i32) {
        crate::src::qcommon::common::Com_Printf(
            b"\"%s\" isn\'t a valid key\n\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::cmd::Cmd_Argv(1 as i32),
        );
        return;
    }
    if c == 2 as i32 {
        if !keys[b as usize].binding.is_null()
            && *keys[b as usize].binding.offset(0 as i32 as isize) as i32 != 0
        {
            crate::src::qcommon::common::Com_Printf(
                b"\"%s\" = \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                Key_KeynumToString(b),
                keys[b as usize].binding,
            );
        } else {
            crate::src::qcommon::common::Com_Printf(
                b"\"%s\" is not bound\n\x00" as *const u8 as *const libc::c_char,
                Key_KeynumToString(b),
            );
        }
        return;
    }
    // copy the rest of the command line
    cmd[0 as i32 as usize] = 0 as i32 as libc::c_char; // start out with a null string
    i = 2 as i32;
    while i < c {
        ::libc::strcat(cmd.as_mut_ptr(), crate::src::qcommon::cmd::Cmd_Argv(i));
        if i != c - 1 as i32 {
            ::libc::strcat(
                cmd.as_mut_ptr(),
                b" \x00" as *const u8 as *const libc::c_char,
            );
        }
        i += 1
    }
    Key_SetBinding(b, cmd.as_mut_ptr());
}
// for keyname autocompletion
/*
============
Key_WriteBindings

Writes lines containing "bind key value"
============
*/
#[no_mangle]

pub unsafe extern "C" fn Key_WriteBindings(mut f: crate::src::qcommon::q_shared::fileHandle_t) {
    let mut i: i32 = 0;
    crate::src::qcommon::files::FS_Printf(
        f,
        b"unbindall\n\x00" as *const u8 as *const libc::c_char,
    );
    i = 0 as i32;
    while i < crate::keycodes_h::MAX_KEYS as i32 {
        if !keys[i as usize].binding.is_null()
            && *keys[i as usize].binding.offset(0 as i32 as isize) as i32 != 0
        {
            crate::src::qcommon::files::FS_Printf(
                f,
                b"bind %s \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                Key_KeynumToString(i),
                keys[i as usize].binding,
            );
        }
        i += 1
    }
}
/*
============
Key_Bindlist_f

============
*/
#[no_mangle]

pub unsafe extern "C" fn Key_Bindlist_f() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < crate::keycodes_h::MAX_KEYS as i32 {
        if !keys[i as usize].binding.is_null()
            && *keys[i as usize].binding.offset(0 as i32 as isize) as i32 != 0
        {
            crate::src::qcommon::common::Com_Printf(
                b"%s \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                Key_KeynumToString(i),
                keys[i as usize].binding,
            );
        }
        i += 1
    }
}
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
/*
============
Key_KeynameCompletion
============
*/
#[no_mangle]

pub unsafe extern "C" fn Key_KeynameCompletion(
    mut callback: Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>,
) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while !keynames[i as usize].name.is_null() {
        callback.expect("non-null function pointer")(keynames[i as usize].name);
        i += 1
    }
}
/*
====================
Key_CompleteUnbind
====================
*/

unsafe extern "C" fn Key_CompleteUnbind(mut args: *mut libc::c_char, mut argNum: i32) {
    if argNum == 2 as i32 {
        // Skip "unbind "
        let mut p: *mut libc::c_char = crate::src::qcommon::q_shared::Com_SkipTokens(
            args,
            1 as i32,
            b" \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if p > args {
            crate::src::qcommon::common::Field_CompleteKeyname();
        }
    };
}
/*
====================
Key_CompleteBind
====================
*/

unsafe extern "C" fn Key_CompleteBind(mut args: *mut libc::c_char, mut argNum: i32) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if argNum == 2 as i32 {
        // Skip "bind "
        p = crate::src::qcommon::q_shared::Com_SkipTokens(
            args,
            1 as i32,
            b" \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if p > args {
            crate::src::qcommon::common::Field_CompleteKeyname();
        }
    } else if argNum >= 3 as i32 {
        // Skip "bind <key> "
        p = crate::src::qcommon::q_shared::Com_SkipTokens(
            args,
            2 as i32,
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
===================
CL_InitKeyCommands
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_InitKeyCommands() {
    // register our functions
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"bind\x00" as *const u8 as *const libc::c_char,
        Some(Key_Bind_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"bind\x00" as *const u8 as *const libc::c_char,
        Some(Key_CompleteBind as unsafe extern "C" fn(_: *mut libc::c_char, _: i32) -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"unbind\x00" as *const u8 as *const libc::c_char,
        Some(Key_Unbind_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"unbind\x00" as *const u8 as *const libc::c_char,
        Some(Key_CompleteUnbind as unsafe extern "C" fn(_: *mut libc::c_char, _: i32) -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"unbindall\x00" as *const u8 as *const libc::c_char,
        Some(Key_Unbindall_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"bindlist\x00" as *const u8 as *const libc::c_char,
        Some(Key_Bindlist_f as unsafe extern "C" fn() -> ()),
    );
}
/*
===================
CL_BindUICommand

Returns qtrue if bind command should be executed while user interface is shown
===================
*/

unsafe extern "C" fn CL_BindUICommand(
    mut cmd: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    if Key_GetCatcher() & 0x1 as i32 != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        cmd,
        b"toggleconsole\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        cmd,
        b"togglemenu\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
===================
CL_ParseBinding

Execute the commands in the bind string
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ParseBinding(
    mut key: i32,
    mut down: crate::src::qcommon::q_shared::qboolean,
    mut time: u32,
) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut p: *mut libc::c_char = buf.as_mut_ptr();
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut allCommands: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut allowUpCmds: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if crate::src::client::cl_main::clc.state as u32
        == crate::src::qcommon::q_shared::CA_DISCONNECTED as i32 as u32
        && Key_GetCatcher() == 0 as i32
    {
        return;
    }
    if keys[key as usize].binding.is_null()
        || *keys[key as usize].binding.offset(0 as i32 as isize) == 0
    {
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        buf.as_mut_ptr(),
        keys[key as usize].binding,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
    );
    // run all bind commands if console, ui, etc aren't reading keys
    allCommands = (Key_GetCatcher() == 0 as i32) as i32 as crate::src::qcommon::q_shared::qboolean;
    // allow button up commands if in game even if key catcher is set
    allowUpCmds = (crate::src::client::cl_main::clc.state as u32
        != crate::src::qcommon::q_shared::CA_DISCONNECTED as i32 as u32) as i32
        as crate::src::qcommon::q_shared::qboolean;
    loop {
        while *(*crate::stdlib::__ctype_b_loc()).offset(*p as i32 as isize) as i32
            & crate::stdlib::_ISspace as i32 as u16 as i32
            != 0
        {
            p = p.offset(1)
        }
        end = ::libc::strchr(p, ';' as i32);
        if !end.is_null() {
            *end = '\u{0}' as i32 as libc::c_char
        }
        if *p as i32 == '+' as i32 {
            // button commands add keynum and time as parameters
            // so that multiple sources can be discriminated and
            // subframe corrected
            if allCommands as u32 != 0 || allowUpCmds as u32 != 0 && down as u64 == 0 {
                let mut cmd: [libc::c_char; 1024] = [0; 1024];
                crate::src::qcommon::q_shared::Com_sprintf(
                    cmd.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
                    b"%c%s %d %d\n\x00" as *const u8 as *const libc::c_char,
                    if down as u32 != 0 {
                        '+' as i32
                    } else {
                        '-' as i32
                    },
                    p.offset(1 as i32 as isize),
                    key,
                    time,
                );
                crate::src::qcommon::cmd::Cbuf_AddText(cmd.as_mut_ptr());
            }
        } else if down as u64 != 0 {
            // normal commands only execute on key press
            if allCommands as u32 != 0 || CL_BindUICommand(p) as u32 != 0 {
                crate::src::qcommon::cmd::Cbuf_AddText(p);
                crate::src::qcommon::cmd::Cbuf_AddText(
                    b"\n\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        if end.is_null() {
            break;
        }
        p = end.offset(1 as i32 as isize)
    }
}
/*
===================
CL_KeyDownEvent

Called by CL_KeyEvent to handle a keypress
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_KeyDownEvent(mut key: i32, mut time: u32) {
    keys[key as usize].down = crate::src::qcommon::q_shared::qtrue;
    keys[key as usize].repeats += 1;
    if keys[key as usize].repeats == 1 as i32 {
        anykeydown += 1
    }
    if keys[crate::keycodes_h::K_ALT as i32 as usize].down as u32 != 0
        && key == crate::keycodes_h::K_ENTER as i32
    {
        // don't repeat fullscreen toggle when keys are held down
        if keys[crate::keycodes_h::K_ENTER as i32 as usize].repeats > 1 as i32 {
            return;
        }
        crate::src::qcommon::cvar::Cvar_SetValue(
            b"r_fullscreen\x00" as *const u8 as *const libc::c_char,
            (crate::src::qcommon::cvar::Cvar_VariableIntegerValue(
                b"r_fullscreen\x00" as *const u8 as *const libc::c_char,
            ) == 0) as i32 as f32,
        );
        return;
    }
    // console key is hardcoded, so the user can never unbind it
    if key == crate::keycodes_h::K_CONSOLE as i32
        || keys[crate::keycodes_h::K_SHIFT as i32 as usize].down as u32 != 0
            && key == crate::keycodes_h::K_ESCAPE as i32
    {
        crate::src::client::cl_console::Con_ToggleConsole_f();
        Key_ClearStates();
        return;
    }
    // keys can still be used for bound actions
    if (key < 128 as i32 || key == crate::keycodes_h::K_MOUSE1 as i32)
        && (crate::src::client::cl_main::clc.demoplaying as u32 != 0
            || crate::src::client::cl_main::clc.state as u32
                == crate::src::qcommon::q_shared::CA_CINEMATIC as i32 as u32)
        && Key_GetCatcher() == 0 as i32
    {
        if crate::src::qcommon::cvar::Cvar_VariableValue(
            b"com_cameraMode\x00" as *const u8 as *const libc::c_char,
        ) == 0 as i32 as f32
        {
            crate::src::qcommon::cvar::Cvar_Set(
                b"nextdemo\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char,
            );
            key = crate::keycodes_h::K_ESCAPE as i32
        }
    }
    // escape is always handled special
    if key == crate::keycodes_h::K_ESCAPE as i32 {
        if Key_GetCatcher() & 0x4 as i32 != 0 {
            // clear message mode
            Message_Key(key);
            return;
        }
        // escape always gets out of CGAME stuff
        if Key_GetCatcher() & 0x8 as i32 != 0 {
            Key_SetCatcher(Key_GetCatcher() & !(0x8 as i32));
            crate::src::qcommon::vm::VM_Call(
                crate::src::client::cl_main::cgvm,
                crate::cg_public_h::CG_EVENT_HANDLING as i32,
                crate::cg_public_h::CGAME_EVENT_NONE as i32,
            );
            return;
        }
        if Key_GetCatcher() & 0x2 as i32 == 0 {
            if crate::src::client::cl_main::clc.state as u32
                == crate::src::qcommon::q_shared::CA_ACTIVE as i32 as u32
                && crate::src::client::cl_main::clc.demoplaying as u64 == 0
            {
                crate::src::qcommon::vm::VM_Call(
                    crate::src::client::cl_ui::uivm,
                    crate::ui_public_h::UI_SET_ACTIVE_MENU as i32,
                    crate::ui_public_h::UIMENU_INGAME as i32,
                );
            } else if crate::src::client::cl_main::clc.state as u32
                != crate::src::qcommon::q_shared::CA_DISCONNECTED as i32 as u32
            {
                crate::src::client::cl_main::CL_Disconnect_f();
                crate::src::client::snd_main::S_StopAllSounds();
                crate::src::qcommon::vm::VM_Call(
                    crate::src::client::cl_ui::uivm,
                    crate::ui_public_h::UI_SET_ACTIVE_MENU as i32,
                    crate::ui_public_h::UIMENU_MAIN as i32,
                );
            }
            return;
        }
        crate::src::qcommon::vm::VM_Call(
            crate::src::client::cl_ui::uivm,
            crate::ui_public_h::UI_KEY_EVENT as i32,
            key,
            crate::src::qcommon::q_shared::qtrue as i32,
        );
        return;
    }
    // send the bound action
    CL_ParseBinding(key, crate::src::qcommon::q_shared::qtrue, time);
    // distribute the key down event to the appropriate handler
    if Key_GetCatcher() & 0x1 as i32 != 0 {
        Console_Key(key);
    } else if Key_GetCatcher() & 0x2 as i32 != 0 {
        if !crate::src::client::cl_ui::uivm.is_null() {
            crate::src::qcommon::vm::VM_Call(
                crate::src::client::cl_ui::uivm,
                crate::ui_public_h::UI_KEY_EVENT as i32,
                key,
                crate::src::qcommon::q_shared::qtrue as i32,
            );
        }
    } else if Key_GetCatcher() & 0x8 as i32 != 0 {
        if !crate::src::client::cl_main::cgvm.is_null() {
            crate::src::qcommon::vm::VM_Call(
                crate::src::client::cl_main::cgvm,
                crate::cg_public_h::CG_KEY_EVENT as i32,
                key,
                crate::src::qcommon::q_shared::qtrue as i32,
            );
        }
    } else if Key_GetCatcher() & 0x4 as i32 != 0 {
        Message_Key(key);
    } else if crate::src::client::cl_main::clc.state as u32
        == crate::src::qcommon::q_shared::CA_DISCONNECTED as i32 as u32
    {
        Console_Key(key);
    };
}
/*
===================
CL_KeyUpEvent

Called by CL_KeyEvent to handle a keyrelease
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_KeyUpEvent(mut key: i32, mut time: u32) {
    keys[key as usize].repeats = 0 as i32;
    keys[key as usize].down = crate::src::qcommon::q_shared::qfalse;
    anykeydown -= 1;
    if anykeydown < 0 as i32 {
        anykeydown = 0 as i32
    }
    // don't process key-up events for the console key
    if key == crate::keycodes_h::K_CONSOLE as i32
        || key == crate::keycodes_h::K_ESCAPE as i32
            && keys[crate::keycodes_h::K_SHIFT as i32 as usize].down as u32 != 0
    {
        return;
    }
    //
    // key up events only perform actions if the game key binding is
    // a button command (leading + sign).  These will be processed even in
    // console mode and menu mode, to keep the character from continuing
    // an action started before a mode switch.
    //
    CL_ParseBinding(key, crate::src::qcommon::q_shared::qfalse, time);
    if Key_GetCatcher() & 0x2 as i32 != 0 && !crate::src::client::cl_ui::uivm.is_null() {
        crate::src::qcommon::vm::VM_Call(
            crate::src::client::cl_ui::uivm,
            crate::ui_public_h::UI_KEY_EVENT as i32,
            key,
            crate::src::qcommon::q_shared::qfalse as i32,
        );
    } else if Key_GetCatcher() & 0x8 as i32 != 0 && !crate::src::client::cl_main::cgvm.is_null() {
        crate::src::qcommon::vm::VM_Call(
            crate::src::client::cl_main::cgvm,
            crate::cg_public_h::CG_KEY_EVENT as i32,
            key,
            crate::src::qcommon::q_shared::qfalse as i32,
        );
    };
}
/*
===================
CL_KeyEvent

Called by the system for both key up and key down events
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_KeyEvent(
    mut key: i32,
    mut down: crate::src::qcommon::q_shared::qboolean,
    mut time: u32,
) {
    if down as u64 != 0 {
        CL_KeyDownEvent(key, time);
    } else {
        CL_KeyUpEvent(key, time);
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
/*
===================
CL_CharEvent

Normal keyboard characters, already shifted / capslocked / etc
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CharEvent(mut key: i32) {
    // delete is not a printable character and is
    // otherwise handled by Field_KeyDownEvent
    if key == 127 as i32 {
        return;
    }
    // distribute the key down event to the appropriate handler
    if Key_GetCatcher() & 0x1 as i32 != 0 {
        Field_CharEvent(&mut g_consoleField, key);
    } else if Key_GetCatcher() & 0x2 as i32 != 0 {
        crate::src::qcommon::vm::VM_Call(
            crate::src::client::cl_ui::uivm,
            crate::ui_public_h::UI_KEY_EVENT as i32,
            key | 1024 as i32,
            crate::src::qcommon::q_shared::qtrue as i32,
        );
    } else if Key_GetCatcher() & 0x4 as i32 != 0 {
        Field_CharEvent(&mut chatField, key);
    } else if crate::src::client::cl_main::clc.state as u32
        == crate::src::qcommon::q_shared::CA_DISCONNECTED as i32 as u32
    {
        Field_CharEvent(&mut g_consoleField, key);
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
// if > 1, it is autorepeating
// NOTE TTimo the declaration of field_t and Field_Clear is now in qcommon/qcommon.h
/*
===================
Key_ClearStates
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Key_ClearStates() {
    let mut i: i32 = 0;
    anykeydown = 0 as i32;
    i = 0 as i32;
    while i < crate::keycodes_h::MAX_KEYS as i32 {
        if keys[i as usize].down as u64 != 0 {
            CL_KeyEvent(i, crate::src::qcommon::q_shared::qfalse, 0 as i32 as u32);
        }
        keys[i as usize].down = crate::src::qcommon::q_shared::qfalse;
        keys[i as usize].repeats = 0 as i32;
        i += 1
    }
}

static mut keyCatchers: i32 = 0 as i32;
/*
====================
Key_GetCatcher
====================
*/
#[no_mangle]

pub unsafe extern "C" fn Key_GetCatcher() -> i32 {
    return keyCatchers;
}
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
/*
====================
Key_SetCatcher
====================
*/
#[no_mangle]

pub unsafe extern "C" fn Key_SetCatcher(mut catcher: i32) {
    // If the catcher state is changing, clear all key states
    if catcher != keyCatchers {
        Key_ClearStates();
    }
    keyCatchers = catcher;
}

static mut consoleSaveBuffer: [libc::c_char; 1024] = [0; 1024];

static mut consoleSaveBufferSize: i32 = 0 as i32;
/*
================
CL_LoadConsoleHistory

Load the console history from cl_consoleHistory
================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_LoadConsoleHistory() {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: i32 = 0;
    let mut numChars: i32 = 0;
    let mut numLines: i32 = 0 as i32;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    consoleSaveBufferSize = crate::src::qcommon::files::FS_FOpenFileRead(
        b"q3history\x00" as *const u8 as *const libc::c_char,
        &mut f,
        crate::src::qcommon::q_shared::qfalse,
    ) as i32;
    if f == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"Couldn\'t read %s.\n\x00" as *const u8 as *const libc::c_char,
            b"q3history\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if consoleSaveBufferSize < 1024 as i32
        && crate::src::qcommon::files::FS_Read(
            consoleSaveBuffer.as_mut_ptr() as *mut libc::c_void,
            consoleSaveBufferSize,
            f,
        ) == consoleSaveBufferSize
    {
        consoleSaveBuffer[consoleSaveBufferSize as usize] = '\u{0}' as i32 as libc::c_char;
        text_p = consoleSaveBuffer.as_mut_ptr();
        i = 32 as i32 - 1 as i32;
        while i >= 0 as i32 {
            token = crate::src::qcommon::q_shared::COM_Parse(&mut text_p);
            if *token == 0 {
                break;
            }
            historyEditLines[i as usize].cursor = atoi(token);
            token = crate::src::qcommon::q_shared::COM_Parse(&mut text_p);
            if *token == 0 {
                break;
            }
            historyEditLines[i as usize].scroll = atoi(token);
            token = crate::src::qcommon::q_shared::COM_Parse(&mut text_p);
            if *token == 0 {
                break;
            }
            numChars = atoi(token);
            text_p = text_p.offset(1);
            if numChars as libc::c_ulong
                > crate::stdlib::strlen(consoleSaveBuffer.as_mut_ptr()).wrapping_sub(
                    text_p.offset_from(consoleSaveBuffer.as_mut_ptr()) as libc::c_long
                        as libc::c_ulong,
                )
            {
                crate::src::qcommon::common::Com_DPrintf(
                    b"^3WARNING: probable corrupt history\n\x00" as *const u8
                        as *const libc::c_char,
                );
                break;
            } else {
                crate::stdlib::memcpy(
                    historyEditLines[i as usize].buffer.as_mut_ptr() as *mut libc::c_void,
                    text_p as *const libc::c_void,
                    numChars as libc::c_ulong,
                );
                historyEditLines[i as usize].buffer[numChars as usize] =
                    '\u{0}' as i32 as libc::c_char;
                text_p = text_p.offset(numChars as isize);
                numLines += 1;
                i -= 1
            }
        }
        crate::stdlib::memmove(
            &mut *historyEditLines.as_mut_ptr().offset(0 as i32 as isize)
                as *mut crate::qcommon_h::field_t as *mut libc::c_void,
            &mut *historyEditLines
                .as_mut_ptr()
                .offset((i + 1 as i32) as isize) as *mut crate::qcommon_h::field_t
                as *const libc::c_void,
            (numLines as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::qcommon_h::field_t>() as libc::c_ulong),
        );
        i = numLines;
        while i < 32 as i32 {
            crate::src::qcommon::common::Field_Clear(
                &mut *historyEditLines.as_mut_ptr().offset(i as isize) as *mut _
                    as *mut crate::qcommon_h::field_t,
            );
            i += 1
        }
        nextHistoryLine = numLines;
        historyLine = nextHistoryLine
    } else {
        crate::src::qcommon::common::Com_Printf(
            b"Couldn\'t read %s.\n\x00" as *const u8 as *const libc::c_char,
            b"q3history\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::src::qcommon::files::FS_FCloseFile(f);
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
/*
================
CL_SaveConsoleHistory

Save the console history into the cvar cl_consoleHistory
so that it persists across invocations of q3
================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_SaveConsoleHistory() {
    let mut i: i32 = 0;
    let mut lineLength: i32 = 0;
    let mut saveBufferLength: i32 = 0;
    let mut additionalLength: i32 = 0;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    consoleSaveBuffer[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
    i = (nextHistoryLine - 1 as i32) % 32 as i32;
    loop {
        if historyEditLines[i as usize].buffer[0 as i32 as usize] != 0 {
            lineLength =
                crate::stdlib::strlen(historyEditLines[i as usize].buffer.as_mut_ptr()) as i32;
            saveBufferLength = crate::stdlib::strlen(consoleSaveBuffer.as_mut_ptr()) as i32;
            //ICK
            additionalLength = (lineLength as libc::c_ulong).wrapping_add(crate::stdlib::strlen(
                b"999 999 999  \x00" as *const u8 as *const libc::c_char,
            )) as i32;
            if !(saveBufferLength + additionalLength < 1024 as i32) {
                break;
            }
            crate::src::qcommon::q_shared::Q_strcat(
                consoleSaveBuffer.as_mut_ptr(),
                1024 as i32,
                crate::src::qcommon::q_shared::va(
                    b"%d %d %d %s \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    historyEditLines[i as usize].cursor,
                    historyEditLines[i as usize].scroll,
                    lineLength,
                    historyEditLines[i as usize].buffer.as_mut_ptr(),
                ),
            );
        }
        i = (i - 1 as i32 + 32 as i32) % 32 as i32;
        if !(i != (nextHistoryLine - 1 as i32) % 32 as i32) {
            break;
        }
    }
    consoleSaveBufferSize = crate::stdlib::strlen(consoleSaveBuffer.as_mut_ptr()) as i32;
    f = crate::src::qcommon::files::FS_FOpenFileWrite(
        b"q3history\x00" as *const u8 as *const libc::c_char,
    );
    if f == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"Couldn\'t write %s.\n\x00" as *const u8 as *const libc::c_char,
            b"q3history\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if crate::src::qcommon::files::FS_Write(
        consoleSaveBuffer.as_mut_ptr() as *const libc::c_void,
        consoleSaveBufferSize,
        f,
    ) < consoleSaveBufferSize
    {
        crate::src::qcommon::common::Com_Printf(
            b"Couldn\'t write %s.\n\x00" as *const u8 as *const libc::c_char,
            b"q3history\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::src::qcommon::files::FS_FCloseFile(f);
}

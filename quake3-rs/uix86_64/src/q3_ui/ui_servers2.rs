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

pub use crate::stddef_h::size_t;

pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
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
pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_confirm::UI_ConfirmMenu_Style;
pub use crate::src::q3_ui::ui_confirm::UI_Message;
pub use crate::src::q3_ui::ui_main::ui_browserGameType;
pub use crate::src::q3_ui::ui_main::ui_browserMaster;
pub use crate::src::q3_ui::ui_main::ui_browserShowEmpty;
pub use crate::src::q3_ui::ui_main::ui_browserShowFull;
pub use crate::src::q3_ui::ui_main::ui_browserSortKey;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::menu_move_sound;
pub use crate::src::q3_ui::ui_qmenu::menu_text_color;
pub use crate::src::q3_ui::ui_qmenu::text_color_normal;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_DefaultKey;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor;
pub use crate::src::q3_ui::ui_qmenu::ScrollList_Key;
pub use crate::src::q3_ui::ui_servers2::stdlib_h::atoi;
pub use crate::src::q3_ui::ui_specifyserver::UI_SpecifyServerMenu;
pub use crate::src::q3_ui::ui_startserver::UI_StartServerMenu;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::Q_strupr;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Register;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_LAN_ClearPing;
pub use crate::src::ui::ui_syscalls::trap_LAN_GetPing;
pub use crate::src::ui::ui_syscalls::trap_LAN_GetPingInfo;
pub use crate::src::ui::ui_syscalls::trap_LAN_GetPingQueueCount;
pub use crate::src::ui::ui_syscalls::trap_LAN_GetServerAddressString;
pub use crate::src::ui::ui_syscalls::trap_LAN_GetServerCount;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_SetPbClStatus;
pub use crate::stdlib::__compar_fn_t;

pub use crate::stdlib::qsort;

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
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menuradiobutton_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::uiStatic_t;

pub use ::libc::strtol;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arenaservers_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub master: crate::ui_local_h::menulist_s,
    pub gametype: crate::ui_local_h::menulist_s,
    pub sortkey: crate::ui_local_h::menulist_s,
    pub showfull: crate::ui_local_h::menuradiobutton_s,
    pub showempty: crate::ui_local_h::menuradiobutton_s,
    pub list: crate::ui_local_h::menulist_s,
    pub mappic: crate::ui_local_h::menubitmap_s,
    pub arrows: crate::ui_local_h::menubitmap_s,
    pub up: crate::ui_local_h::menubitmap_s,
    pub down: crate::ui_local_h::menubitmap_s,
    pub status: crate::ui_local_h::menutext_s,
    pub statusbar: crate::ui_local_h::menutext_s,
    pub remove: crate::ui_local_h::menubitmap_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub refresh: crate::ui_local_h::menubitmap_s,
    pub specify: crate::ui_local_h::menubitmap_s,
    pub create: crate::ui_local_h::menubitmap_s,
    pub go: crate::ui_local_h::menubitmap_s,
    pub pinglist: [pinglist_t; 32],
    pub table: [table_t; 128],
    pub items: [*mut libc::c_char; 128],
    pub numqueriedservers: libc::c_int,
    pub numservers: *mut libc::c_int,
    pub serverlist: *mut servernode_t,
    pub currentping: libc::c_int,
    pub refreshservers: crate::src::qcommon::q_shared::qboolean,
    pub nextpingtime: libc::c_int,
    pub maxservers: libc::c_int,
    pub refreshtime: libc::c_int,
    pub favoriteaddresses: [[libc::c_char; 64]; 16],
    pub numfavoriteaddresses: libc::c_int,
    pub punkbuster: crate::ui_local_h::menulist_s,
    pub pblogo: crate::ui_local_h::menubitmap_s,
}

pub type servernode_t = servernode_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servernode_s {
    pub adrstr: [libc::c_char; 64],
    pub hostname: [libc::c_char; 25],
    pub mapname: [libc::c_char; 16],
    pub numclients: libc::c_int,
    pub maxclients: libc::c_int,
    pub pingtime: libc::c_int,
    pub gametype: libc::c_int,
    pub gamename: [libc::c_char; 12],
    pub nettype: libc::c_int,
    pub minPing: libc::c_int,
    pub maxPing: libc::c_int,
    pub bPB: crate::src::qcommon::q_shared::qboolean,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct table_t {
    pub buff: [libc::c_char; 68],
    pub servernode: *mut servernode_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinglist_t {
    pub adrstr: [libc::c_char; 64],
    pub start: libc::c_int,
}

static mut master_items: [*const libc::c_char; 9] = [
    b"Local\x00" as *const u8 as *const libc::c_char,
    b"Internet\x00" as *const u8 as *const libc::c_char,
    b"Master1\x00" as *const u8 as *const libc::c_char,
    b"Master2\x00" as *const u8 as *const libc::c_char,
    b"Master3\x00" as *const u8 as *const libc::c_char,
    b"Master4\x00" as *const u8 as *const libc::c_char,
    b"Master5\x00" as *const u8 as *const libc::c_char,
    b"Favorites\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

static mut servertype_items: [*const libc::c_char; 6] = [
    b"All\x00" as *const u8 as *const libc::c_char,
    b"Free For All\x00" as *const u8 as *const libc::c_char,
    b"Team Deathmatch\x00" as *const u8 as *const libc::c_char,
    b"Tournament\x00" as *const u8 as *const libc::c_char,
    b"Capture the Flag\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

static mut sortkey_items: [*const libc::c_char; 6] = [
    b"Server Name\x00" as *const u8 as *const libc::c_char,
    b"Map Name\x00" as *const u8 as *const libc::c_char,
    b"Open Player Spots\x00" as *const u8 as *const libc::c_char,
    b"Game Type\x00" as *const u8 as *const libc::c_char,
    b"Ping Time\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

static mut gamenames: [*mut libc::c_char; 14] = [
    b"DM \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"1v1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SP \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Team DM\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"CTF\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"One Flag CTF\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"OverLoad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Harvester\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Rocket Arena 3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Q3F\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Urban Terror\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"OSP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"???\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];

static mut netnames: [*mut libc::c_char; 4] = [
    b"??? \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"UDP \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"UDP6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];

static mut quake3worldMessage: [libc::c_char; 59] = [
    86, 105, 115, 105, 116, 32, 119, 119, 119, 46, 113, 117, 97, 107, 101, 51, 119, 111, 114, 108,
    100, 46, 99, 111, 109, 32, 45, 32, 78, 101, 119, 115, 44, 32, 67, 111, 109, 109, 117, 110, 105,
    116, 121, 44, 32, 69, 118, 101, 110, 116, 115, 44, 32, 70, 105, 108, 101, 115, 0,
];
#[no_mangle]

pub static mut punkbuster_items: [*const libc::c_char; 3] = [
    b"Disabled\x00" as *const u8 as *const libc::c_char,
    b"Enabled\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]

pub static mut punkbuster_msg: [*const libc::c_char; 5] = [
    b"PunkBuster will be\x00" as *const u8 as *const libc::c_char,
    b"disabled the next time\x00" as *const u8 as *const libc::c_char,
    b"Quake III Arena\x00" as *const u8 as *const libc::c_char,
    b"is started.\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

static mut g_arenaservers: arenaservers_t = arenaservers_t {
    menu: crate::ui_local_h::menuframework_s {
        cursor: 0,
        cursor_prev: 0,
        nitems: 0,
        items: [0 as *const libc::c_void as *mut libc::c_void; 64],
        draw: None,
        key: None,
        wrapAround: crate::src::qcommon::q_shared::qfalse,
        fullscreen: crate::src::qcommon::q_shared::qfalse,
        showlogo: crate::src::qcommon::q_shared::qfalse,
    },
    banner: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    master: crate::ui_local_h::menulist_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    gametype: crate::ui_local_h::menulist_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    sortkey: crate::ui_local_h::menulist_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    showfull: crate::ui_local_h::menuradiobutton_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        curvalue: 0,
    },
    showempty: crate::ui_local_h::menuradiobutton_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        curvalue: 0,
    },
    list: crate::ui_local_h::menulist_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    mappic: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    arrows: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    up: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    down: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    status: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    statusbar: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    remove: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    back: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    refresh: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    specify: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    create: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    go: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    pinglist: [pinglist_t {
        adrstr: [0; 64],
        start: 0,
    }; 32],
    table: [table_t {
        buff: [0; 68],
        servernode: 0 as *const servernode_t as *mut servernode_t,
    }; 128],
    items: [0 as *const libc::c_char as *mut libc::c_char; 128],
    numqueriedservers: 0,
    numservers: 0 as *const libc::c_int as *mut libc::c_int,
    serverlist: 0 as *const servernode_t as *mut servernode_t,
    currentping: 0,
    refreshservers: crate::src::qcommon::q_shared::qfalse,
    nextpingtime: 0,
    maxservers: 0,
    refreshtime: 0,
    favoriteaddresses: [[0; 64]; 16],
    numfavoriteaddresses: 0,
    punkbuster: crate::ui_local_h::menulist_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    pblogo: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
};

static mut g_globalserverlist: [[servernode_t; 128]; 6] = [[servernode_t {
    adrstr: [0; 64],
    hostname: [0; 25],
    mapname: [0; 16],
    numclients: 0,
    maxclients: 0,
    pingtime: 0,
    gametype: 0,
    gamename: [0; 12],
    nettype: 0,
    minPing: 0,
    maxPing: 0,
    bPB: crate::src::qcommon::q_shared::qfalse,
}; 128]; 6];

static mut g_numglobalservers: [libc::c_int; 6] = [0; 6];

static mut g_localserverlist: [servernode_t; 128] = [servernode_t {
    adrstr: [0; 64],
    hostname: [0; 25],
    mapname: [0; 16],
    numclients: 0,
    maxclients: 0,
    pingtime: 0,
    gametype: 0,
    gamename: [0; 12],
    nettype: 0,
    minPing: 0,
    maxPing: 0,
    bPB: crate::src::qcommon::q_shared::qfalse,
}; 128];

static mut g_numlocalservers: libc::c_int = 0;

static mut g_favoriteserverlist: [servernode_t; 16] = [servernode_t {
    adrstr: [0; 64],
    hostname: [0; 25],
    mapname: [0; 16],
    numclients: 0,
    maxclients: 0,
    pingtime: 0,
    gametype: 0,
    gamename: [0; 12],
    nettype: 0,
    minPing: 0,
    maxPing: 0,
    bPB: crate::src::qcommon::q_shared::qfalse,
}; 16];

static mut g_numfavoriteservers: libc::c_int = 0;

static mut g_servertype: libc::c_int = 0;

static mut g_gametype: libc::c_int = 0;

static mut g_sortkey: libc::c_int = 0;

static mut g_emptyservers: libc::c_int = 0;

static mut g_fullservers: libc::c_int = 0;
/*
=================
ArenaServers_MaxPing
=================
*/

unsafe extern "C" fn ArenaServers_MaxPing() -> libc::c_int {
    let mut maxPing: libc::c_int = 0;
    maxPing = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cl_maxPing\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if maxPing < 100 as libc::c_int {
        maxPing = 100 as libc::c_int
    }
    return maxPing;
}
/*
=================
ArenaServers_Compare
=================
*/

unsafe extern "C" fn ArenaServers_Compare(
    mut arg1: *const libc::c_void,
    mut arg2: *const libc::c_void,
) -> libc::c_int {
    let mut f1: libc::c_float = 0.;
    let mut f2: libc::c_float = 0.;
    let mut t1: *mut servernode_t = 0 as *mut servernode_t;
    let mut t2: *mut servernode_t = 0 as *mut servernode_t;
    t1 = arg1 as *mut servernode_t;
    t2 = arg2 as *mut servernode_t;
    match g_sortkey {
        0 => {
            return crate::src::qcommon::q_shared::Q_stricmp(
                (*t1).hostname.as_mut_ptr(),
                (*t2).hostname.as_mut_ptr(),
            )
        }
        1 => {
            return crate::src::qcommon::q_shared::Q_stricmp(
                (*t1).mapname.as_mut_ptr(),
                (*t2).mapname.as_mut_ptr(),
            )
        }
        2 => {
            f1 = ((*t1).maxclients - (*t1).numclients) as libc::c_float;
            if f1 < 0 as libc::c_int as libc::c_float {
                f1 = 0 as libc::c_int as libc::c_float
            }
            f2 = ((*t2).maxclients - (*t2).numclients) as libc::c_float;
            if f2 < 0 as libc::c_int as libc::c_float {
                f2 = 0 as libc::c_int as libc::c_float
            }
            if f1 < f2 {
                return 1 as libc::c_int;
            }
            if f1 == f2 {
                return 0 as libc::c_int;
            }
            return -(1 as libc::c_int);
        }
        3 => {
            if (*t1).gametype < (*t2).gametype {
                return -(1 as libc::c_int);
            }
            if (*t1).gametype == (*t2).gametype {
                return 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        4 => {
            if (*t1).pingtime < (*t2).pingtime {
                return -(1 as libc::c_int);
            }
            if (*t1).pingtime > (*t2).pingtime {
                return 1 as libc::c_int;
            }
            return crate::src::qcommon::q_shared::Q_stricmp(
                (*t1).hostname.as_mut_ptr(),
                (*t2).hostname.as_mut_ptr(),
            );
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
/*
=================
ArenaServers_SourceForLAN

Convert ui's g_servertype to AS_* used by trap calls.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ArenaServers_SourceForLAN() -> libc::c_int {
    match g_servertype {
        1 | 2 | 3 | 4 | 5 | 6 => return 2 as libc::c_int,
        7 => return 3 as libc::c_int,
        0 | _ => return 0 as libc::c_int,
    };
}
/*
=================
ArenaServers_Go
=================
*/

unsafe extern "C" fn ArenaServers_Go() {
    let mut servernode: *mut servernode_t = 0 as *mut servernode_t;
    servernode = g_arenaservers.table[g_arenaservers.list.curvalue as usize].servernode;
    if !servernode.is_null() {
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
            crate::src::qcommon::q_shared::va(
                b"connect %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*servernode).adrstr.as_mut_ptr(),
            ),
        );
    };
}
/*
=================
ArenaServers_UpdatePicture
=================
*/

unsafe extern "C" fn ArenaServers_UpdatePicture() {
    static mut picname: [libc::c_char; 64] = [0; 64];
    let mut servernodeptr: *mut servernode_t = 0 as *mut servernode_t;
    if g_arenaservers.list.numitems == 0 {
        g_arenaservers.mappic.generic.name = 0 as *const libc::c_char
    } else {
        servernodeptr = g_arenaservers.table[g_arenaservers.list.curvalue as usize].servernode;
        crate::src::qcommon::q_shared::Com_sprintf(
            picname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"levelshots/%s.tga\x00" as *const u8 as *const libc::c_char,
            (*servernodeptr).mapname.as_mut_ptr(),
        );
        g_arenaservers.mappic.generic.name = picname.as_mut_ptr()
    }
    // force shader update during draw
    g_arenaservers.mappic.shader = 0 as libc::c_int;
}
/*
=================
ArenaServers_UpdateMenu
=================
*/

unsafe extern "C" fn ArenaServers_UpdateMenu() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut buff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut servernodeptr: *mut servernode_t = 0 as *mut servernode_t;
    let mut tableptr: *mut table_t = 0 as *mut table_t;
    let mut pingColor: *mut libc::c_char = 0 as *mut libc::c_char;
    if g_arenaservers.numqueriedservers > 0 as libc::c_int {
        // servers found
        if g_arenaservers.refreshservers as libc::c_uint != 0
            && g_arenaservers.currentping <= g_arenaservers.numqueriedservers
        {
            // show progress
            crate::src::qcommon::q_shared::Com_sprintf(
                g_arenaservers.status.string,
                64 as libc::c_int,
                b"%d of %d Arena Servers.\x00" as *const u8 as *const libc::c_char,
                g_arenaservers.currentping,
                g_arenaservers.numqueriedservers,
            );
            g_arenaservers.statusbar.string =
                b"Press SPACE to stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
            crate::stdlib::qsort(
                g_arenaservers.serverlist as *mut libc::c_void,
                *g_arenaservers.numservers as crate::stddef_h::size_t,
                ::std::mem::size_of::<servernode_t>() as libc::c_ulong,
                Some(
                    ArenaServers_Compare
                        as unsafe extern "C" fn(
                            _: *const libc::c_void,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
        } else {
            // all servers pinged - enable controls
            g_arenaservers.gametype.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.sortkey.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.showempty.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.showfull.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.list.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.refresh.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.go.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.punkbuster.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            // update status bar
            if g_servertype >= 1 as libc::c_int && g_servertype <= 6 as libc::c_int {
                g_arenaservers.statusbar.string = quake3worldMessage.as_mut_ptr()
            } else {
                g_arenaservers.statusbar.string =
                    b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
        }
    } else {
        // no servers found
        if g_arenaservers.refreshservers as u64 != 0 {
            ::libc::strcpy(
                g_arenaservers.status.string,
                b"Scanning For Servers.\x00" as *const u8 as *const libc::c_char,
            );
            g_arenaservers.statusbar.string =
                b"Press SPACE to stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
            // disable controls during refresh
            g_arenaservers.gametype.generic.flags |= 0x2000 as libc::c_int as libc::c_uint;
            g_arenaservers.sortkey.generic.flags |= 0x2000 as libc::c_int as libc::c_uint;
            g_arenaservers.showempty.generic.flags |= 0x2000 as libc::c_int as libc::c_uint;
            g_arenaservers.showfull.generic.flags |= 0x2000 as libc::c_int as libc::c_uint;
            g_arenaservers.list.generic.flags |= 0x2000 as libc::c_int as libc::c_uint;
            g_arenaservers.refresh.generic.flags |= 0x2000 as libc::c_int as libc::c_uint;
            g_arenaservers.go.generic.flags |= 0x2000 as libc::c_int as libc::c_uint;
            g_arenaservers.punkbuster.generic.flags |= 0x2000 as libc::c_int as libc::c_uint
        } else {
            if g_arenaservers.numqueriedservers < 0 as libc::c_int {
                ::libc::strcpy(
                    g_arenaservers.status.string,
                    b"No Response From Master Server.\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                ::libc::strcpy(
                    g_arenaservers.status.string,
                    b"No Servers Found.\x00" as *const u8 as *const libc::c_char,
                );
            }
            // update status bar
            if g_servertype >= 1 as libc::c_int && g_servertype <= 6 as libc::c_int {
                g_arenaservers.statusbar.string = quake3worldMessage.as_mut_ptr()
            } else {
                g_arenaservers.statusbar.string =
                    b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            // end of refresh - set control state
            g_arenaservers.master.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.gametype.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.sortkey.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.showempty.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.showfull.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.list.generic.flags |= 0x2000 as libc::c_int as libc::c_uint;
            g_arenaservers.refresh.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint);
            g_arenaservers.go.generic.flags |= 0x2000 as libc::c_int as libc::c_uint;
            g_arenaservers.punkbuster.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint)
        }
        // zero out list box
        g_arenaservers.list.numitems = 0 as libc::c_int;
        g_arenaservers.list.curvalue = 0 as libc::c_int;
        g_arenaservers.list.top = 0 as libc::c_int;
        // update picture
        ArenaServers_UpdatePicture();
        return;
    }
    // build list box strings - apply culling filters
    servernodeptr = g_arenaservers.serverlist;
    count = *g_arenaservers.numservers;
    let mut current_block_80: u64;
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while i < count {
        tableptr = &mut *g_arenaservers.table.as_mut_ptr().offset(j as isize) as *mut table_t;
        (*tableptr).servernode = servernodeptr;
        buff = (*tableptr).buff.as_mut_ptr();
        // can only cull valid results
        if !(g_emptyservers == 0 && (*servernodeptr).numclients == 0) {
            if !(g_fullservers == 0 && (*servernodeptr).numclients == (*servernodeptr).maxclients) {
                match g_gametype {
                    1 => {
                        if (*servernodeptr).gametype != crate::bg_public_h::GT_FFA as libc::c_int {
                            current_block_80 = 13325891313334703151;
                        } else {
                            current_block_80 = 17441561948628420366;
                        }
                    }
                    2 => {
                        if (*servernodeptr).gametype != crate::bg_public_h::GT_TEAM as libc::c_int {
                            current_block_80 = 13325891313334703151;
                        } else {
                            current_block_80 = 17441561948628420366;
                        }
                    }
                    3 => {
                        if (*servernodeptr).gametype
                            != crate::bg_public_h::GT_TOURNAMENT as libc::c_int
                        {
                            current_block_80 = 13325891313334703151;
                        } else {
                            current_block_80 = 17441561948628420366;
                        }
                    }
                    4 => {
                        if (*servernodeptr).gametype != crate::bg_public_h::GT_CTF as libc::c_int {
                            current_block_80 = 13325891313334703151;
                        } else {
                            current_block_80 = 17441561948628420366;
                        }
                    }
                    0 | _ => {
                        current_block_80 = 17441561948628420366;
                    }
                }
                match current_block_80 {
                    13325891313334703151 => {}
                    _ => {
                        if (*servernodeptr).pingtime < (*servernodeptr).minPing {
                            pingColor =
                                b"^4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                        } else if (*servernodeptr).maxPing != 0
                            && (*servernodeptr).pingtime > (*servernodeptr).maxPing
                        {
                            pingColor =
                                b"^4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                        } else if (*servernodeptr).pingtime < 200 as libc::c_int {
                            pingColor =
                                b"^2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                        } else if (*servernodeptr).pingtime < 400 as libc::c_int {
                            pingColor =
                                b"^3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                        } else {
                            pingColor =
                                b"^1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                        }
                        crate::src::qcommon::q_shared::Com_sprintf(
                            buff,
                            68 as libc::c_int,
                            b"%-20.20s %-12.12s %2d/%2d %-8.8s %4s%s%3d ^3%s\x00" as *const u8
                                as *const libc::c_char,
                            (*servernodeptr).hostname.as_mut_ptr(),
                            (*servernodeptr).mapname.as_mut_ptr(),
                            (*servernodeptr).numclients,
                            (*servernodeptr).maxclients,
                            (*servernodeptr).gamename.as_mut_ptr(),
                            netnames[(*servernodeptr).nettype as usize],
                            pingColor,
                            (*servernodeptr).pingtime,
                            if (*servernodeptr).bPB as libc::c_uint != 0 {
                                b"Yes\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"No\x00" as *const u8 as *const libc::c_char
                            },
                        );
                        j += 1
                    }
                }
            }
        }
        i += 1;
        servernodeptr = servernodeptr.offset(1)
    }
    g_arenaservers.list.numitems = j;
    g_arenaservers.list.curvalue = 0 as libc::c_int;
    g_arenaservers.list.top = 0 as libc::c_int;
    // update picture
    ArenaServers_UpdatePicture();
}
/*
=================
ArenaServers_Remove
=================
*/

unsafe extern "C" fn ArenaServers_Remove() {
    let mut i: libc::c_int = 0;
    let mut servernodeptr: *mut servernode_t = 0 as *mut servernode_t;
    let mut tableptr: *mut table_t = 0 as *mut table_t;
    if g_arenaservers.list.numitems == 0 {
        return;
    }
    // remove selected item from display list
    // items are in scattered order due to sort and cull
    // perform delete on list box contents, resync all lists
    tableptr = &mut *g_arenaservers
        .table
        .as_mut_ptr()
        .offset(g_arenaservers.list.curvalue as isize) as *mut table_t;
    servernodeptr = (*tableptr).servernode;
    // find address in master list
    i = 0 as libc::c_int;
    while i < g_arenaservers.numfavoriteaddresses {
        if crate::src::qcommon::q_shared::Q_stricmp(
            g_arenaservers.favoriteaddresses[i as usize].as_mut_ptr(),
            (*servernodeptr).adrstr.as_mut_ptr(),
        ) == 0
        {
            // delete address from master list
            if i < g_arenaservers.numfavoriteaddresses - 1 as libc::c_int {
                // shift items up
                crate::stdlib::memcpy(
                    &mut *g_arenaservers
                        .favoriteaddresses
                        .as_mut_ptr()
                        .offset(i as isize) as *mut [libc::c_char; 64]
                        as *mut libc::c_void,
                    &mut *g_arenaservers
                        .favoriteaddresses
                        .as_mut_ptr()
                        .offset((i + 1 as libc::c_int) as isize)
                        as *mut [libc::c_char; 64] as *const libc::c_void,
                    ((g_arenaservers.numfavoriteaddresses - i - 1 as libc::c_int)
                        * 64 as libc::c_int) as libc::c_ulong,
                );
            }
            g_arenaservers.numfavoriteaddresses -= 1;
            crate::stdlib::memset(
                &mut *g_arenaservers
                    .favoriteaddresses
                    .as_mut_ptr()
                    .offset(g_arenaservers.numfavoriteaddresses as isize)
                    as *mut [libc::c_char; 64] as *mut libc::c_void,
                0 as libc::c_int,
                64 as libc::c_int as libc::c_ulong,
            );
            break;
        } else {
            i += 1
        }
    }
    // find address in server list
    i = 0 as libc::c_int;
    while i < g_numfavoriteservers {
        if &mut *g_favoriteserverlist.as_mut_ptr().offset(i as isize) as *mut servernode_t
            == servernodeptr
        {
            // delete address from server list
            if i < g_numfavoriteservers - 1 as libc::c_int {
                // shift items up
                crate::stdlib::memcpy(
                    &mut *g_favoriteserverlist.as_mut_ptr().offset(i as isize) as *mut servernode_t
                        as *mut libc::c_void,
                    &mut *g_favoriteserverlist
                        .as_mut_ptr()
                        .offset((i + 1 as libc::c_int) as isize)
                        as *mut servernode_t as *const libc::c_void,
                    ((g_numfavoriteservers - i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<servernode_t>() as libc::c_ulong),
                );
            }
            g_numfavoriteservers -= 1;
            crate::stdlib::memset(
                &mut *g_favoriteserverlist
                    .as_mut_ptr()
                    .offset(g_numfavoriteservers as isize) as *mut servernode_t
                    as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<servernode_t>() as libc::c_ulong,
            );
            break;
        } else {
            i += 1
        }
    }
    g_arenaservers.numqueriedservers = g_arenaservers.numfavoriteaddresses;
    g_arenaservers.currentping = g_arenaservers.numfavoriteaddresses;
}
/*
=================
ArenaServers_Insert
=================
*/

unsafe extern "C" fn ArenaServers_Insert(
    mut adrstr: *mut libc::c_char,
    mut info: *mut libc::c_char,
    mut pingtime: libc::c_int,
) {
    let mut servernodeptr: *mut servernode_t = 0 as *mut servernode_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if pingtime >= ArenaServers_MaxPing() && g_servertype != 7 as libc::c_int {
        // slow global or local servers do not get entered
        return;
    }
    if *g_arenaservers.numservers >= g_arenaservers.maxservers {
        // list full;
        servernodeptr = g_arenaservers
            .serverlist
            .offset(*g_arenaservers.numservers as isize)
            .offset(-(1 as libc::c_int as isize))
    } else {
        // next slot
        servernodeptr = g_arenaservers
            .serverlist
            .offset(*g_arenaservers.numservers as isize);
        *g_arenaservers.numservers += 1
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*servernodeptr).adrstr.as_mut_ptr(),
        adrstr,
        64 as libc::c_int,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*servernodeptr).hostname.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"hostname\x00" as *const u8 as *const libc::c_char,
        ),
        22 as libc::c_int,
    );
    crate::src::qcommon::q_shared::Q_CleanStr((*servernodeptr).hostname.as_mut_ptr());
    crate::src::qcommon::q_shared::Q_strupr((*servernodeptr).hostname.as_mut_ptr());
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*servernodeptr).mapname.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"mapname\x00" as *const u8 as *const libc::c_char,
        ),
        16 as libc::c_int,
    );
    crate::src::qcommon::q_shared::Q_CleanStr((*servernodeptr).mapname.as_mut_ptr());
    crate::src::qcommon::q_shared::Q_strupr((*servernodeptr).mapname.as_mut_ptr());
    (*servernodeptr).numclients = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"clients\x00" as *const u8 as *const libc::c_char,
    ));
    (*servernodeptr).maxclients = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
    ));
    (*servernodeptr).pingtime = pingtime;
    (*servernodeptr).minPing = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"minPing\x00" as *const u8 as *const libc::c_char,
    ));
    (*servernodeptr).maxPing = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"maxPing\x00" as *const u8 as *const libc::c_char,
    ));
    (*servernodeptr).bPB = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"punkbuster\x00" as *const u8 as *const libc::c_char,
    )) as crate::src::qcommon::q_shared::qboolean;
    /*
    s = Info_ValueForKey( info, "nettype" );
    for (i=0; ;i++)
    {
        if (!netnames[i])
        {
            servernodeptr->nettype = 0;
            break;
        }
        else if (!Q_stricmp( netnames[i], s ))
        {
            servernodeptr->nettype = i;
            break;
        }
    }
    */
    (*servernodeptr).nettype = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"nettype\x00" as *const u8 as *const libc::c_char,
    )); //-1;
    if (*servernodeptr).nettype < 0 as libc::c_int
        || (*servernodeptr).nettype as libc::c_ulong
            >= (::std::mem::size_of::<[*mut libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        (*servernodeptr).nettype = 0 as libc::c_int
    }
    s = crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"game\x00" as *const u8 as *const libc::c_char,
    );
    i = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"gametype\x00" as *const u8 as *const libc::c_char,
    ));
    if i < 0 as libc::c_int {
        i = 0 as libc::c_int
    } else if i > 11 as libc::c_int {
        i = 12 as libc::c_int
    }
    if *s != 0 {
        (*servernodeptr).gametype = i;
        crate::src::qcommon::q_shared::Q_strncpyz(
            (*servernodeptr).gamename.as_mut_ptr(),
            s,
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as libc::c_int,
        );
    } else {
        (*servernodeptr).gametype = i;
        crate::src::qcommon::q_shared::Q_strncpyz(
            (*servernodeptr).gamename.as_mut_ptr(),
            gamenames[i as usize],
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as libc::c_int,
        );
    };
}
/*
=================
ArenaServers_LoadFavorites

Load cvar address book entries into local lists.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ArenaServers_LoadFavorites() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut numtempitems: libc::c_int = 0;
    let mut adrstr: [libc::c_char; 64] = [0; 64];
    let mut templist: [servernode_t; 16] = [servernode_t {
        adrstr: [0; 64],
        hostname: [0; 25],
        mapname: [0; 16],
        numclients: 0,
        maxclients: 0,
        pingtime: 0,
        gametype: 0,
        gamename: [0; 12],
        nettype: 0,
        minPing: 0,
        maxPing: 0,
        bPB: crate::src::qcommon::q_shared::qfalse,
    }; 16];
    let mut found: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    found = crate::src::qcommon::q_shared::qfalse;
    // copy the old
    crate::stdlib::memcpy(
        templist.as_mut_ptr() as *mut libc::c_void,
        g_favoriteserverlist.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<servernode_t>() as libc::c_ulong)
            .wrapping_mul(16 as libc::c_int as libc::c_ulong),
    );
    numtempitems = g_numfavoriteservers;
    // clear the current for sync
    crate::stdlib::memset(
        g_favoriteserverlist.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<servernode_t>() as libc::c_ulong)
            .wrapping_mul(16 as libc::c_int as libc::c_ulong),
    );
    g_numfavoriteservers = 0 as libc::c_int;
    // resync existing results with new or deleted cvars
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
            crate::src::qcommon::q_shared::va(
                b"server%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                i + 1 as libc::c_int,
            ),
            adrstr.as_mut_ptr(),
            64 as libc::c_int,
        );
        if !(adrstr[0 as libc::c_int as usize] == 0) {
            // favorite server addresses must be maintained outside refresh list
            // this mimics local and global netadr's stored in client
            // these can be fetched to fill ping list
            ::libc::strcpy(
                g_arenaservers.favoriteaddresses[g_numfavoriteservers as usize].as_mut_ptr(),
                adrstr.as_mut_ptr(),
            );
            // find this server in the old list
            j = 0 as libc::c_int;
            while j < numtempitems {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    templist[j as usize].adrstr.as_mut_ptr(),
                    adrstr.as_mut_ptr(),
                ) == 0
                {
                    break;
                }
                j += 1
            }
            if j < numtempitems {
                // found server - add exisiting results
                crate::stdlib::memcpy(
                    &mut *g_favoriteserverlist
                        .as_mut_ptr()
                        .offset(g_numfavoriteservers as isize)
                        as *mut servernode_t as *mut libc::c_void,
                    &mut *templist.as_mut_ptr().offset(j as isize) as *mut servernode_t
                        as *const libc::c_void,
                    ::std::mem::size_of::<servernode_t>() as libc::c_ulong,
                );
                found = crate::src::qcommon::q_shared::qtrue
            } else {
                // add new server
                crate::src::qcommon::q_shared::Q_strncpyz(
                    g_favoriteserverlist[g_numfavoriteservers as usize]
                        .adrstr
                        .as_mut_ptr(),
                    adrstr.as_mut_ptr(),
                    64 as libc::c_int,
                );
                g_favoriteserverlist[g_numfavoriteservers as usize].pingtime =
                    ArenaServers_MaxPing()
            }
            g_numfavoriteservers += 1
        }
        i += 1
    }
    g_arenaservers.numfavoriteaddresses = g_numfavoriteservers;
    if found as u64 == 0 {
        // no results were found, reset server list
        // list will be automatically refreshed when selected
        g_numfavoriteservers = 0 as libc::c_int
    };
}
/*
=================
ArenaServers_StopRefresh
=================
*/

unsafe extern "C" fn ArenaServers_StopRefresh() {
    if g_arenaservers.refreshservers as u64 == 0 {
        // not currently refreshing
        return;
    }
    g_arenaservers.refreshservers = crate::src::qcommon::q_shared::qfalse;
    // final tally
    if g_arenaservers.numqueriedservers >= 0 as libc::c_int {
        g_arenaservers.currentping = *g_arenaservers.numservers;
        g_arenaservers.numqueriedservers = *g_arenaservers.numservers
    }
    // sort
    crate::stdlib::qsort(
        g_arenaservers.serverlist as *mut libc::c_void,
        *g_arenaservers.numservers as crate::stddef_h::size_t,
        ::std::mem::size_of::<servernode_t>() as libc::c_ulong,
        Some(
            ArenaServers_Compare
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    ArenaServers_UpdateMenu();
}
/*
=================
ArenaServers_DoRefresh
=================
*/

unsafe extern "C" fn ArenaServers_DoRefresh() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    let mut maxPing: libc::c_int = 0;
    let mut adrstr: [libc::c_char; 64] = [0; 64];
    let mut info: [libc::c_char; 1024] = [0; 1024];
    if crate::src::q3_ui::ui_atoms::uis.realtime < g_arenaservers.refreshtime {
        if g_servertype != 7 as libc::c_int {
            if g_servertype == 0 as libc::c_int {
                if crate::src::ui::ui_syscalls::trap_LAN_GetServerCount(0 as libc::c_int) == 0 {
                    return;
                }
            }
            if crate::src::ui::ui_syscalls::trap_LAN_GetServerCount(ArenaServers_SourceForLAN())
                < 0 as libc::c_int
            {
                // still waiting for response
                return;
            }
        }
    } else if g_servertype == 0 as libc::c_int {
        if crate::src::ui::ui_syscalls::trap_LAN_GetServerCount(0 as libc::c_int) == 0 {
            // no local servers found, check again
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
                b"localservers\n\x00" as *const u8 as *const libc::c_char,
            );
            g_arenaservers.refreshtime =
                crate::src::q3_ui::ui_atoms::uis.realtime + 5000 as libc::c_int;
            return;
        }
    }
    if crate::src::q3_ui::ui_atoms::uis.realtime < g_arenaservers.nextpingtime {
        // wait for time trigger
        return;
    }
    // trigger at 10Hz intervals
    g_arenaservers.nextpingtime = crate::src::q3_ui::ui_atoms::uis.realtime + 10 as libc::c_int;
    // process ping results
    maxPing = ArenaServers_MaxPing();
    let mut current_block_41: u64;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        crate::src::ui::ui_syscalls::trap_LAN_GetPing(
            i,
            adrstr.as_mut_ptr(),
            64 as libc::c_int,
            &mut time,
        );
        if !(adrstr[0 as libc::c_int as usize] == 0) {
            // find ping result in our local list
            j = 0 as libc::c_int;
            while j < 32 as libc::c_int {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    adrstr.as_mut_ptr(),
                    g_arenaservers.pinglist[j as usize].adrstr.as_mut_ptr(),
                ) == 0
                {
                    break;
                }
                j += 1
            }
            if j < 32 as libc::c_int {
                // found it
                if time == 0 {
                    time = crate::src::q3_ui::ui_atoms::uis.realtime
                        - g_arenaservers.pinglist[j as usize].start;
                    if time < maxPing {
                        current_block_41 = 9828876828309294594;
                    } else {
                        current_block_41 = 6417057564578538666;
                    }
                } else {
                    current_block_41 = 6417057564578538666;
                }
                match current_block_41 {
                    9828876828309294594 => {}
                    _ => {
                        if time > maxPing {
                            // stale it out
                            info[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
                            time = maxPing;
                            // set hostname for nonresponsive favorite server
                            if g_servertype == 7 as libc::c_int {
                                crate::src::qcommon::q_shared::Info_SetValueForKey(
                                    info.as_mut_ptr(),
                                    b"hostname\x00" as *const u8 as *const libc::c_char,
                                    adrstr.as_mut_ptr(),
                                );
                                crate::src::qcommon::q_shared::Info_SetValueForKey(
                                    info.as_mut_ptr(),
                                    b"game\x00" as *const u8 as *const libc::c_char,
                                    b"???\x00" as *const u8 as *const libc::c_char,
                                );
                            }
                        } else {
                            crate::src::ui::ui_syscalls::trap_LAN_GetPingInfo(
                                i,
                                info.as_mut_ptr(),
                                1024 as libc::c_int,
                            );
                        }
                        // insert ping results
                        ArenaServers_Insert(adrstr.as_mut_ptr(), info.as_mut_ptr(), time);
                        // clear this query from internal list
                        g_arenaservers.pinglist[j as usize].adrstr[0 as libc::c_int as usize] =
                            '\u{0}' as i32 as libc::c_char;
                        current_block_41 = 1924505913685386279;
                    }
                }
            } else {
                current_block_41 = 1924505913685386279;
            }
            match current_block_41 {
                9828876828309294594 => {}
                _ => {
                    // clear this query from external list
                    crate::src::ui::ui_syscalls::trap_LAN_ClearPing(i);
                }
            }
        }
        // still waiting
        i += 1
    }
    // get results of servers query
    // counts can increase as servers respond
    if g_servertype == 7 as libc::c_int {
        g_arenaservers.numqueriedservers = g_arenaservers.numfavoriteaddresses
    } else {
        g_arenaservers.numqueriedservers =
            crate::src::ui::ui_syscalls::trap_LAN_GetServerCount(ArenaServers_SourceForLAN())
    }
    //	if (g_arenaservers.numqueriedservers > g_arenaservers.maxservers)
    //		g_arenaservers.numqueriedservers = g_arenaservers.maxservers;
    // send ping requests in reasonable bursts
    // iterate ping through all found servers
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int && g_arenaservers.currentping < g_arenaservers.numqueriedservers {
        if crate::src::ui::ui_syscalls::trap_LAN_GetPingQueueCount() >= 32 as libc::c_int {
            // ping queue is full
            break;
        } else {
            // find empty slot
            j = 0 as libc::c_int;
            while j < 32 as libc::c_int {
                if g_arenaservers.pinglist[j as usize].adrstr[0 as libc::c_int as usize] == 0 {
                    break;
                }
                j += 1
            }
            if j >= 32 as libc::c_int {
                break;
            }
            // get an address to ping
            if g_servertype == 7 as libc::c_int {
                ::libc::strcpy(
                    adrstr.as_mut_ptr(),
                    g_arenaservers.favoriteaddresses[g_arenaservers.currentping as usize]
                        .as_mut_ptr(),
                );
            } else {
                crate::src::ui::ui_syscalls::trap_LAN_GetServerAddressString(
                    ArenaServers_SourceForLAN(),
                    g_arenaservers.currentping,
                    adrstr.as_mut_ptr(),
                    64 as libc::c_int,
                );
            }
            ::libc::strcpy(
                g_arenaservers.pinglist[j as usize].adrstr.as_mut_ptr(),
                adrstr.as_mut_ptr(),
            );
            g_arenaservers.pinglist[j as usize].start = crate::src::q3_ui::ui_atoms::uis.realtime;
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_NOW as libc::c_int,
                crate::src::qcommon::q_shared::va(
                    b"ping %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    adrstr.as_mut_ptr(),
                ),
            );
            // advance to next server
            g_arenaservers.currentping += 1;
            i += 1
        }
    }
    if crate::src::ui::ui_syscalls::trap_LAN_GetPingQueueCount() == 0 {
        // all pings completed
        ArenaServers_StopRefresh();
        return;
    }
    // update the user interface with ping status
    ArenaServers_UpdateMenu();
}
/*
=================
ArenaServers_StartRefresh
=================
*/

unsafe extern "C" fn ArenaServers_StartRefresh() {
    let mut i: libc::c_int = 0;
    let mut myargs: [libc::c_char; 32] = [0; 32];
    let mut protocol: [libc::c_char; 32] = [0; 32];
    crate::stdlib::memset(
        g_arenaservers.serverlist as *mut libc::c_void,
        0 as libc::c_int,
        (g_arenaservers.maxservers as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<table_t>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        g_arenaservers.pinglist[i as usize].adrstr[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char;
        crate::src::ui::ui_syscalls::trap_LAN_ClearPing(i);
        i += 1
    }
    g_arenaservers.refreshservers = crate::src::qcommon::q_shared::qtrue;
    g_arenaservers.currentping = 0 as libc::c_int;
    g_arenaservers.nextpingtime = 0 as libc::c_int;
    *g_arenaservers.numservers = 0 as libc::c_int;
    g_arenaservers.numqueriedservers = 0 as libc::c_int;
    // allow max 5 seconds for responses
    g_arenaservers.refreshtime = crate::src::q3_ui::ui_atoms::uis.realtime + 5000 as libc::c_int;
    // place menu in zeroed state
    ArenaServers_UpdateMenu();
    if g_servertype == 0 as libc::c_int {
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
            b"localservers\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if g_servertype >= 1 as libc::c_int && g_servertype <= 6 as libc::c_int {
        match g_arenaservers.gametype.curvalue {
            1 => {
                ::libc::strcpy(
                    myargs.as_mut_ptr(),
                    b" ffa\x00" as *const u8 as *const libc::c_char,
                );
            }
            2 => {
                ::libc::strcpy(
                    myargs.as_mut_ptr(),
                    b" team\x00" as *const u8 as *const libc::c_char,
                );
            }
            3 => {
                ::libc::strcpy(
                    myargs.as_mut_ptr(),
                    b" tourney\x00" as *const u8 as *const libc::c_char,
                );
            }
            4 => {
                ::libc::strcpy(
                    myargs.as_mut_ptr(),
                    b" ctf\x00" as *const u8 as *const libc::c_char,
                );
            }
            0 | _ => myargs[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char,
        }
        if g_emptyservers != 0 {
            ::libc::strcat(
                myargs.as_mut_ptr(),
                b" empty\x00" as *const u8 as *const libc::c_char,
            );
        }
        if g_fullservers != 0 {
            ::libc::strcat(
                myargs.as_mut_ptr(),
                b" full\x00" as *const u8 as *const libc::c_char,
            );
        }
        protocol[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
            b"debug_protocol\x00" as *const u8 as *const libc::c_char,
            protocol.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        );
        if crate::stdlib::strlen(protocol.as_mut_ptr()) != 0 {
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
                crate::src::qcommon::q_shared::va(
                    b"globalservers %d %s%s\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g_servertype - 1 as libc::c_int,
                    protocol.as_mut_ptr(),
                    myargs.as_mut_ptr(),
                ),
            );
        } else {
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
                crate::src::qcommon::q_shared::va(
                    b"globalservers %d %d%s\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g_servertype - 1 as libc::c_int,
                    crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
                        b"protocol\x00" as *const u8 as *const libc::c_char,
                    ) as libc::c_int,
                    myargs.as_mut_ptr(),
                ),
            );
        }
    };
}
/*
=================
ArenaServers_SaveChanges
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ArenaServers_SaveChanges() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < g_arenaservers.numfavoriteaddresses {
        crate::src::ui::ui_syscalls::trap_Cvar_Set(
            crate::src::qcommon::q_shared::va(
                b"server%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                i + 1 as libc::c_int,
            ),
            g_arenaservers.favoriteaddresses[i as usize].as_mut_ptr(),
        );
        i += 1
    }
    while i < 16 as libc::c_int {
        crate::src::ui::ui_syscalls::trap_Cvar_Set(
            crate::src::qcommon::q_shared::va(
                b"server%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                i + 1 as libc::c_int,
            ),
            b"\x00" as *const u8 as *const libc::c_char,
        );
        i += 1
    }
}
/*
=================
ArenaServers_Sort
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ArenaServers_Sort(mut type_0: libc::c_int) {
    if g_sortkey == type_0 {
        return;
    }
    g_sortkey = type_0;
    crate::stdlib::qsort(
        g_arenaservers.serverlist as *mut libc::c_void,
        *g_arenaservers.numservers as crate::stddef_h::size_t,
        ::std::mem::size_of::<servernode_t>() as libc::c_ulong,
        Some(
            ArenaServers_Compare
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
/*
=================
ArenaServers_SetType
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ArenaServers_SetType(mut type_0: libc::c_int) -> libc::c_int {
    ArenaServers_StopRefresh();
    if type_0 >= 2 as libc::c_int && type_0 <= 6 as libc::c_int {
        let mut masterstr: [libc::c_char; 2] = [0; 2];
        let mut cvarname: [libc::c_char; 11] = [0; 11];
        let mut direction: libc::c_int = 0;
        if type_0 == g_servertype || type_0 == (g_servertype + 1 as libc::c_int) % 8 as libc::c_int
        {
            direction = 1 as libc::c_int
        } else {
            direction = -(1 as libc::c_int)
        }
        while type_0 >= 2 as libc::c_int && type_0 <= 6 as libc::c_int {
            crate::src::qcommon::q_shared::Com_sprintf(
                cvarname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as libc::c_int,
                b"sv_master%d\x00" as *const u8 as *const libc::c_char,
                type_0 - 1 as libc::c_int,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
                cvarname.as_mut_ptr(),
                masterstr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as libc::c_int,
            );
            if *masterstr.as_mut_ptr() != 0 {
                break;
            }
            type_0 += direction
        }
    }
    g_servertype = type_0;
    match type_0 {
        1 | 2 | 3 | 4 | 5 | 6 => {
            g_arenaservers.remove.generic.flags |=
                0x4000 as libc::c_int as libc::c_uint | 0x1000 as libc::c_int as libc::c_uint;
            g_arenaservers.serverlist =
                g_globalserverlist[(type_0 - 1 as libc::c_int) as usize].as_mut_ptr();
            g_arenaservers.numservers = &mut *g_numglobalservers
                .as_mut_ptr()
                .offset((type_0 - 1 as libc::c_int) as isize)
                as *mut libc::c_int;
            g_arenaservers.maxservers = 128 as libc::c_int
        }
        7 => {
            g_arenaservers.remove.generic.flags &=
                !(0x4000 as libc::c_int as libc::c_uint | 0x1000 as libc::c_int as libc::c_uint);
            g_arenaservers.serverlist = g_favoriteserverlist.as_mut_ptr();
            g_arenaservers.numservers = &mut g_numfavoriteservers;
            g_arenaservers.maxservers = 16 as libc::c_int
        }
        0 | _ => {
            g_arenaservers.remove.generic.flags |=
                0x4000 as libc::c_int as libc::c_uint | 0x1000 as libc::c_int as libc::c_uint;
            g_arenaservers.serverlist = g_localserverlist.as_mut_ptr();
            g_arenaservers.numservers = &mut g_numlocalservers;
            g_arenaservers.maxservers = 128 as libc::c_int
        }
    }
    if *g_arenaservers.numservers == 0 {
        ArenaServers_StartRefresh();
    } else {
        // avoid slow operation, use existing results
        g_arenaservers.currentping = *g_arenaservers.numservers;
        g_arenaservers.numqueriedservers = *g_arenaservers.numservers;
        ArenaServers_UpdateMenu();
        ::libc::strcpy(
            g_arenaservers.status.string,
            b"hit refresh to update\x00" as *const u8 as *const libc::c_char,
        );
    }
    return type_0;
}
/*
=================
PunkBuster_Confirm
=================
*/

unsafe extern "C" fn Punkbuster_ConfirmEnable(mut result: crate::src::qcommon::q_shared::qboolean) {
    if result as u64 != 0 {
        crate::src::ui::ui_syscalls::trap_SetPbClStatus(1 as libc::c_int);
    }
    g_arenaservers.punkbuster.curvalue = crate::src::qcommon::q_shared::Com_Clamp(
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"cl_punkbuster\x00" as *const u8 as *const libc::c_char,
        ),
    ) as libc::c_int;
}

unsafe extern "C" fn Punkbuster_ConfirmDisable(
    mut result: crate::src::qcommon::q_shared::qboolean,
) {
    if result as u64 != 0 {
        crate::src::ui::ui_syscalls::trap_SetPbClStatus(0 as libc::c_int);
        crate::src::q3_ui::ui_confirm::UI_Message(punkbuster_msg.as_mut_ptr());
    }
    g_arenaservers.punkbuster.curvalue = crate::src::qcommon::q_shared::Com_Clamp(
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"cl_punkbuster\x00" as *const u8 as *const libc::c_char,
        ),
    ) as libc::c_int;
}
/*
=================
ArenaServers_Event
=================
*/

unsafe extern "C" fn ArenaServers_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    let mut id: libc::c_int = 0;
    id = (*(ptr as *mut crate::ui_local_h::menucommon_s)).id;
    if event != 3 as libc::c_int && id != 15 as libc::c_int {
        return;
    }
    match id {
        10 => {
            g_arenaservers.master.curvalue = ArenaServers_SetType(g_arenaservers.master.curvalue);
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_browserMaster\x00" as *const u8 as *const libc::c_char,
                g_arenaservers.master.curvalue as libc::c_float,
            );
        }
        11 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_browserGameType\x00" as *const u8 as *const libc::c_char,
                g_arenaservers.gametype.curvalue as libc::c_float,
            );
            g_gametype = g_arenaservers.gametype.curvalue;
            ArenaServers_UpdateMenu();
        }
        12 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_browserSortKey\x00" as *const u8 as *const libc::c_char,
                g_arenaservers.sortkey.curvalue as libc::c_float,
            );
            ArenaServers_Sort(g_arenaservers.sortkey.curvalue);
            ArenaServers_UpdateMenu();
        }
        13 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_browserShowFull\x00" as *const u8 as *const libc::c_char,
                g_arenaservers.showfull.curvalue as libc::c_float,
            );
            g_fullservers = g_arenaservers.showfull.curvalue;
            ArenaServers_UpdateMenu();
        }
        14 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_browserShowEmpty\x00" as *const u8 as *const libc::c_char,
                g_arenaservers.showempty.curvalue as libc::c_float,
            );
            g_emptyservers = g_arenaservers.showempty.curvalue;
            ArenaServers_UpdateMenu();
        }
        15 => {
            if event == 1 as libc::c_int {
                ArenaServers_UpdatePicture();
            }
        }
        16 => {
            crate::src::q3_ui::ui_qmenu::ScrollList_Key(
                &mut g_arenaservers.list as *mut _ as *mut crate::ui_local_h::menulist_s,
                crate::keycodes_h::K_UPARROW as libc::c_int,
            );
        }
        17 => {
            crate::src::q3_ui::ui_qmenu::ScrollList_Key(
                &mut g_arenaservers.list as *mut _ as *mut crate::ui_local_h::menulist_s,
                crate::keycodes_h::K_DOWNARROW as libc::c_int,
            );
        }
        18 => {
            ArenaServers_StopRefresh();
            ArenaServers_SaveChanges();
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        19 => {
            ArenaServers_StartRefresh();
        }
        20 => {
            crate::src::q3_ui::ui_specifyserver::UI_SpecifyServerMenu();
        }
        21 => {
            crate::src::q3_ui::ui_startserver::UI_StartServerMenu(
                crate::src::qcommon::q_shared::qtrue,
            );
        }
        22 => {
            ArenaServers_Go();
        }
        23 => {
            ArenaServers_Remove();
            ArenaServers_UpdateMenu();
        }
        24 => {
            if g_arenaservers.punkbuster.curvalue != 0 {
                crate::src::q3_ui::ui_confirm::UI_ConfirmMenu_Style(
                    b"Enable Punkbuster?\x00" as *const u8 as *const libc::c_char,
                    0x1 as libc::c_int | 0x2000 as libc::c_int | 0x10 as libc::c_int,
                    None,
                    Some(
                        Punkbuster_ConfirmEnable
                            as unsafe extern "C" fn(
                                _: crate::src::qcommon::q_shared::qboolean,
                            ) -> (),
                    ),
                );
            } else {
                crate::src::q3_ui::ui_confirm::UI_ConfirmMenu_Style(
                    b"Disable Punkbuster?\x00" as *const u8 as *const libc::c_char,
                    0x1 as libc::c_int | 0x2000 as libc::c_int | 0x10 as libc::c_int,
                    None,
                    Some(
                        Punkbuster_ConfirmDisable
                            as unsafe extern "C" fn(
                                _: crate::src::qcommon::q_shared::qboolean,
                            ) -> (),
                    ),
                );
            }
        }
        _ => {}
    };
}
/*
=================
ArenaServers_MenuDraw
=================
*/

unsafe extern "C" fn ArenaServers_MenuDraw() {
    if g_arenaservers.refreshservers as u64 != 0 {
        ArenaServers_DoRefresh();
    }
    crate::src::q3_ui::ui_qmenu::Menu_Draw(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}
/*
=================
ArenaServers_MenuKey
=================
*/

unsafe extern "C" fn ArenaServers_MenuKey(
    mut key: libc::c_int,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    if key == crate::keycodes_h::K_SPACE as libc::c_int
        && g_arenaservers.refreshservers as libc::c_uint != 0
    {
        ArenaServers_StopRefresh();
        return crate::src::q3_ui::ui_qmenu::menu_move_sound;
    }
    if (key == crate::keycodes_h::K_DEL as libc::c_int
        || key == crate::keycodes_h::K_KP_DEL as libc::c_int)
        && g_servertype == 7 as libc::c_int
        && crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor(
            &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        ) == &mut g_arenaservers.list as *mut crate::ui_local_h::menulist_s as *mut libc::c_void
    {
        ArenaServers_Remove();
        ArenaServers_UpdateMenu();
        return crate::src::q3_ui::ui_qmenu::menu_move_sound;
    }
    if key == crate::keycodes_h::K_MOUSE2 as libc::c_int
        || key == crate::keycodes_h::K_ESCAPE as libc::c_int
    {
        ArenaServers_StopRefresh();
        ArenaServers_SaveChanges();
    }
    return crate::src::q3_ui::ui_qmenu::Menu_DefaultKey(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        key,
    );
}
/*
=================
ArenaServers_MenuInit
=================
*/

unsafe extern "C" fn ArenaServers_MenuInit() {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    static mut statusbuffer: [libc::c_char; 64] = [0; 64];
    // zero set all our globals
    crate::stdlib::memset(
        &mut g_arenaservers as *mut arenaservers_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<arenaservers_t>() as libc::c_ulong,
    );
    ArenaServers_Cache();
    g_arenaservers.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    g_arenaservers.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    g_arenaservers.menu.draw = Some(ArenaServers_MenuDraw as unsafe extern "C" fn() -> ());
    g_arenaservers.menu.key = Some(
        ArenaServers_MenuKey
            as unsafe extern "C" fn(_: libc::c_int) -> crate::src::qcommon::q_shared::sfxHandle_t,
    );
    g_arenaservers.banner.generic.type_0 = 10 as libc::c_int;
    g_arenaservers.banner.generic.flags = 0x8 as libc::c_int as libc::c_uint;
    g_arenaservers.banner.generic.x = 320 as libc::c_int;
    g_arenaservers.banner.generic.y = 16 as libc::c_int;
    g_arenaservers.banner.string =
        b"ARENA SERVERS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.banner.style = 0x1 as libc::c_int;
    g_arenaservers.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    y = 80 as libc::c_int;
    g_arenaservers.master.generic.type_0 = 3 as libc::c_int;
    g_arenaservers.master.generic.name = b"Servers:\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.master.generic.flags =
        0x100 as libc::c_int as libc::c_uint | 0x2 as libc::c_int as libc::c_uint;
    g_arenaservers.master.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.master.generic.id = 10 as libc::c_int;
    g_arenaservers.master.generic.x = 320 as libc::c_int;
    g_arenaservers.master.generic.y = y;
    g_arenaservers.master.itemnames = master_items.as_mut_ptr();
    y += 16 as libc::c_int;
    g_arenaservers.gametype.generic.type_0 = 3 as libc::c_int;
    g_arenaservers.gametype.generic.name = b"Game Type:\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.gametype.generic.flags =
        0x100 as libc::c_int as libc::c_uint | 0x2 as libc::c_int as libc::c_uint;
    g_arenaservers.gametype.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.gametype.generic.id = 11 as libc::c_int;
    g_arenaservers.gametype.generic.x = 320 as libc::c_int;
    g_arenaservers.gametype.generic.y = y;
    g_arenaservers.gametype.itemnames = servertype_items.as_mut_ptr();
    y += 16 as libc::c_int;
    g_arenaservers.sortkey.generic.type_0 = 3 as libc::c_int;
    g_arenaservers.sortkey.generic.name = b"Sort By:\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.sortkey.generic.flags =
        0x100 as libc::c_int as libc::c_uint | 0x2 as libc::c_int as libc::c_uint;
    g_arenaservers.sortkey.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.sortkey.generic.id = 12 as libc::c_int;
    g_arenaservers.sortkey.generic.x = 320 as libc::c_int;
    g_arenaservers.sortkey.generic.y = y;
    g_arenaservers.sortkey.itemnames = sortkey_items.as_mut_ptr();
    y += 16 as libc::c_int;
    g_arenaservers.showfull.generic.type_0 = 5 as libc::c_int;
    g_arenaservers.showfull.generic.name = b"Show Full:\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.showfull.generic.flags =
        0x100 as libc::c_int as libc::c_uint | 0x2 as libc::c_int as libc::c_uint;
    g_arenaservers.showfull.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.showfull.generic.id = 13 as libc::c_int;
    g_arenaservers.showfull.generic.x = 320 as libc::c_int;
    g_arenaservers.showfull.generic.y = y;
    y += 16 as libc::c_int;
    g_arenaservers.showempty.generic.type_0 = 5 as libc::c_int;
    g_arenaservers.showempty.generic.name = b"Show Empty:\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.showempty.generic.flags =
        0x100 as libc::c_int as libc::c_uint | 0x2 as libc::c_int as libc::c_uint;
    g_arenaservers.showempty.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.showempty.generic.id = 14 as libc::c_int;
    g_arenaservers.showempty.generic.x = 320 as libc::c_int;
    g_arenaservers.showempty.generic.y = y;
    y += 3 as libc::c_int * 16 as libc::c_int;
    g_arenaservers.list.generic.type_0 = 8 as libc::c_int;
    g_arenaservers.list.generic.flags = 0x80 as libc::c_int as libc::c_uint;
    g_arenaservers.list.generic.id = 15 as libc::c_int;
    g_arenaservers.list.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.list.generic.x = 72 as libc::c_int;
    g_arenaservers.list.generic.y = y;
    g_arenaservers.list.width = 68 as libc::c_int;
    g_arenaservers.list.height = 11 as libc::c_int;
    g_arenaservers.list.itemnames = g_arenaservers.items.as_mut_ptr() as *mut *const libc::c_char;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        g_arenaservers.items[i as usize] = g_arenaservers.table[i as usize].buff.as_mut_ptr();
        i += 1
    }
    g_arenaservers.mappic.generic.type_0 = 6 as libc::c_int;
    g_arenaservers.mappic.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x4000 as libc::c_int as libc::c_uint;
    g_arenaservers.mappic.generic.x = 72 as libc::c_int;
    g_arenaservers.mappic.generic.y = 80 as libc::c_int;
    g_arenaservers.mappic.width = 128 as libc::c_int;
    g_arenaservers.mappic.height = 96 as libc::c_int;
    g_arenaservers.mappic.errorpic =
        b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.arrows.generic.type_0 = 6 as libc::c_int;
    g_arenaservers.arrows.generic.name =
        b"menu/art/arrows_vert_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.arrows.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x4000 as libc::c_int as libc::c_uint;
    g_arenaservers.arrows.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.arrows.generic.x = 512 as libc::c_int + 48 as libc::c_int;
    g_arenaservers.arrows.generic.y = 240 as libc::c_int - 64 as libc::c_int + 16 as libc::c_int;
    g_arenaservers.arrows.width = 64 as libc::c_int;
    g_arenaservers.arrows.height = 128 as libc::c_int;
    g_arenaservers.up.generic.type_0 = 6 as libc::c_int;
    g_arenaservers.up.generic.flags = 0x4 as libc::c_int as libc::c_uint
        | 0x100 as libc::c_int as libc::c_uint
        | 0x800 as libc::c_int as libc::c_uint;
    g_arenaservers.up.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.up.generic.id = 16 as libc::c_int;
    g_arenaservers.up.generic.x = 512 as libc::c_int + 48 as libc::c_int;
    g_arenaservers.up.generic.y = 240 as libc::c_int - 64 as libc::c_int + 16 as libc::c_int;
    g_arenaservers.up.width = 64 as libc::c_int;
    g_arenaservers.up.height = 64 as libc::c_int;
    g_arenaservers.up.focuspic =
        b"menu/art/arrows_vert_top\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.down.generic.type_0 = 6 as libc::c_int;
    g_arenaservers.down.generic.flags = 0x4 as libc::c_int as libc::c_uint
        | 0x100 as libc::c_int as libc::c_uint
        | 0x800 as libc::c_int as libc::c_uint;
    g_arenaservers.down.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.down.generic.id = 17 as libc::c_int;
    g_arenaservers.down.generic.x = 512 as libc::c_int + 48 as libc::c_int;
    g_arenaservers.down.generic.y = 240 as libc::c_int + 16 as libc::c_int;
    g_arenaservers.down.width = 64 as libc::c_int;
    g_arenaservers.down.height = 64 as libc::c_int;
    g_arenaservers.down.focuspic =
        b"menu/art/arrows_vert_bot\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    y = 376 as libc::c_int;
    g_arenaservers.status.generic.type_0 = 7 as libc::c_int;
    g_arenaservers.status.generic.x = 320 as libc::c_int;
    g_arenaservers.status.generic.y = y;
    g_arenaservers.status.string = statusbuffer.as_mut_ptr();
    g_arenaservers.status.style = 0x1 as libc::c_int | 0x10 as libc::c_int;
    g_arenaservers.status.color = crate::src::q3_ui::ui_qmenu::menu_text_color.as_mut_ptr();
    y += 16 as libc::c_int;
    g_arenaservers.statusbar.generic.type_0 = 7 as libc::c_int;
    g_arenaservers.statusbar.generic.x = 320 as libc::c_int;
    g_arenaservers.statusbar.generic.y = y;
    g_arenaservers.statusbar.string =
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.statusbar.style = 0x1 as libc::c_int | 0x10 as libc::c_int;
    g_arenaservers.statusbar.color = crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr();
    g_arenaservers.remove.generic.type_0 = 6 as libc::c_int;
    g_arenaservers.remove.generic.name =
        b"menu/art/delete_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.remove.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    g_arenaservers.remove.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.remove.generic.id = 23 as libc::c_int;
    g_arenaservers.remove.generic.x = 450 as libc::c_int;
    g_arenaservers.remove.generic.y = 86 as libc::c_int;
    g_arenaservers.remove.width = 96 as libc::c_int;
    g_arenaservers.remove.height = 48 as libc::c_int;
    g_arenaservers.remove.focuspic =
        b"menu/art/delete_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.back.generic.type_0 = 6 as libc::c_int;
    g_arenaservers.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.back.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    g_arenaservers.back.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.back.generic.id = 18 as libc::c_int;
    g_arenaservers.back.generic.x = 0 as libc::c_int;
    g_arenaservers.back.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    g_arenaservers.back.width = 128 as libc::c_int;
    g_arenaservers.back.height = 64 as libc::c_int;
    g_arenaservers.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.specify.generic.type_0 = 6 as libc::c_int;
    g_arenaservers.specify.generic.name =
        b"menu/art/specify_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.specify.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    g_arenaservers.specify.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.specify.generic.id = 20 as libc::c_int;
    g_arenaservers.specify.generic.x = 128 as libc::c_int;
    g_arenaservers.specify.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    g_arenaservers.specify.width = 128 as libc::c_int;
    g_arenaservers.specify.height = 64 as libc::c_int;
    g_arenaservers.specify.focuspic =
        b"menu/art/specify_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.refresh.generic.type_0 = 6 as libc::c_int;
    g_arenaservers.refresh.generic.name =
        b"menu/art/refresh_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.refresh.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    g_arenaservers.refresh.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.refresh.generic.id = 19 as libc::c_int;
    g_arenaservers.refresh.generic.x = 256 as libc::c_int;
    g_arenaservers.refresh.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    g_arenaservers.refresh.width = 128 as libc::c_int;
    g_arenaservers.refresh.height = 64 as libc::c_int;
    g_arenaservers.refresh.focuspic =
        b"menu/art/refresh_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.create.generic.type_0 = 6 as libc::c_int;
    g_arenaservers.create.generic.name =
        b"menu/art/create_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.create.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    g_arenaservers.create.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.create.generic.id = 21 as libc::c_int;
    g_arenaservers.create.generic.x = 384 as libc::c_int;
    g_arenaservers.create.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    g_arenaservers.create.width = 128 as libc::c_int;
    g_arenaservers.create.height = 64 as libc::c_int;
    g_arenaservers.create.focuspic =
        b"menu/art/create_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.go.generic.type_0 = 6 as libc::c_int;
    g_arenaservers.go.generic.name = b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.go.generic.flags =
        0x10 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    g_arenaservers.go.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.go.generic.id = 22 as libc::c_int;
    g_arenaservers.go.generic.x = 640 as libc::c_int;
    g_arenaservers.go.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    g_arenaservers.go.width = 128 as libc::c_int;
    g_arenaservers.go.height = 64 as libc::c_int;
    g_arenaservers.go.focuspic =
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.punkbuster.generic.type_0 = 3 as libc::c_int;
    g_arenaservers.punkbuster.generic.name = b"Punkbuster:\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.punkbuster.generic.flags =
        0x100 as libc::c_int as libc::c_uint | 0x2 as libc::c_int as libc::c_uint;
    g_arenaservers.punkbuster.generic.callback = Some(
        ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    g_arenaservers.punkbuster.generic.id = 24 as libc::c_int;
    g_arenaservers.punkbuster.generic.x = 480 as libc::c_int + 32 as libc::c_int;
    g_arenaservers.punkbuster.generic.y = 144 as libc::c_int;
    g_arenaservers.punkbuster.itemnames = punkbuster_items.as_mut_ptr();
    g_arenaservers.pblogo.generic.type_0 = 6 as libc::c_int;
    g_arenaservers.pblogo.generic.name = b"menu/art/pblogo\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.pblogo.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x4000 as libc::c_int as libc::c_uint;
    g_arenaservers.pblogo.generic.x = 526 as libc::c_int;
    g_arenaservers.pblogo.generic.y = 176 as libc::c_int;
    g_arenaservers.pblogo.width = 32 as libc::c_int;
    g_arenaservers.pblogo.height = 16 as libc::c_int;
    g_arenaservers.pblogo.errorpic =
        b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.master as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.gametype as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.sortkey as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.showfull as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.showempty as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.mappic as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.status as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.statusbar as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.arrows as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.up as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.down as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.list as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.remove as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.specify as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.refresh as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.create as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.go as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.punkbuster as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut g_arenaservers.pblogo as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    ArenaServers_LoadFavorites();
    g_servertype = crate::src::qcommon::q_shared::Com_Clamp(
        0 as libc::c_int as libc::c_float,
        (8 as libc::c_int - 1 as libc::c_int) as libc::c_float,
        crate::src::q3_ui::ui_main::ui_browserMaster.integer as libc::c_float,
    ) as libc::c_int;
    g_arenaservers.master.curvalue = g_servertype;
    g_gametype = crate::src::qcommon::q_shared::Com_Clamp(
        0 as libc::c_int as libc::c_float,
        (5 as libc::c_int - 1 as libc::c_int) as libc::c_float,
        crate::src::q3_ui::ui_main::ui_browserGameType.integer as libc::c_float,
    ) as libc::c_int;
    g_arenaservers.gametype.curvalue = g_gametype;
    g_sortkey = crate::src::qcommon::q_shared::Com_Clamp(
        0 as libc::c_int as libc::c_float,
        (5 as libc::c_int - 1 as libc::c_int) as libc::c_float,
        crate::src::q3_ui::ui_main::ui_browserSortKey.integer as libc::c_float,
    ) as libc::c_int;
    g_arenaservers.sortkey.curvalue = g_sortkey;
    g_fullservers = crate::src::qcommon::q_shared::Com_Clamp(
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        crate::src::q3_ui::ui_main::ui_browserShowFull.integer as libc::c_float,
    ) as libc::c_int;
    g_arenaservers.showfull.curvalue = g_fullservers;
    g_emptyservers = crate::src::qcommon::q_shared::Com_Clamp(
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        crate::src::q3_ui::ui_main::ui_browserShowEmpty.integer as libc::c_float,
    ) as libc::c_int;
    g_arenaservers.showempty.curvalue = g_emptyservers;
    g_arenaservers.punkbuster.curvalue = crate::src::qcommon::q_shared::Com_Clamp(
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"cl_punkbuster\x00" as *const u8 as *const libc::c_char,
        ),
    ) as libc::c_int;
    // force to initial state and refresh
    g_servertype = ArenaServers_SetType(g_servertype);
    g_arenaservers.master.curvalue = g_servertype;
    crate::src::ui::ui_syscalls::trap_Cvar_Register(
        0 as *mut crate::src::qcommon::q_shared::vmCvar_t
            as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"debug_protocol\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
}
/*
=================
ArenaServers_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ArenaServers_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/create_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/create_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/specify_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/specify_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/refresh_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/refresh_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/arrows_vert_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/arrows_vert_top\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/arrows_vert_bot\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/pblogo\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
UI_ArenaServersMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ArenaServersMenu() {
    ArenaServers_MenuInit();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut g_arenaservers.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}

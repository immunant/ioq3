use ::libc;

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::weapon_t;
pub use crate::bg_public_h::BOTH_DEAD1;
pub use crate::bg_public_h::BOTH_DEAD2;
pub use crate::bg_public_h::BOTH_DEAD3;
pub use crate::bg_public_h::BOTH_DEATH1;
pub use crate::bg_public_h::BOTH_DEATH2;
pub use crate::bg_public_h::BOTH_DEATH3;
pub use crate::bg_public_h::FLAG_RUN;
pub use crate::bg_public_h::FLAG_STAND;
pub use crate::bg_public_h::FLAG_STAND2RUN;
pub use crate::bg_public_h::LEGS_BACK;
pub use crate::bg_public_h::LEGS_BACKCR;
pub use crate::bg_public_h::LEGS_BACKWALK;
pub use crate::bg_public_h::LEGS_IDLE;
pub use crate::bg_public_h::LEGS_IDLECR;
pub use crate::bg_public_h::LEGS_JUMP;
pub use crate::bg_public_h::LEGS_JUMPB;
pub use crate::bg_public_h::LEGS_LAND;
pub use crate::bg_public_h::LEGS_LANDB;
pub use crate::bg_public_h::LEGS_RUN;
pub use crate::bg_public_h::LEGS_SWIM;
pub use crate::bg_public_h::LEGS_TURN;
pub use crate::bg_public_h::LEGS_WALK;
pub use crate::bg_public_h::LEGS_WALKCR;
pub use crate::bg_public_h::MAX_ANIMATIONS;
pub use crate::bg_public_h::MAX_TOTALANIMATIONS;
pub use crate::bg_public_h::TORSO_AFFIRMATIVE;
pub use crate::bg_public_h::TORSO_ATTACK;
pub use crate::bg_public_h::TORSO_ATTACK2;
pub use crate::bg_public_h::TORSO_DROP;
pub use crate::bg_public_h::TORSO_FOLLOWME;
pub use crate::bg_public_h::TORSO_GESTURE;
pub use crate::bg_public_h::TORSO_GETFLAG;
pub use crate::bg_public_h::TORSO_GUARDBASE;
pub use crate::bg_public_h::TORSO_NEGATIVE;
pub use crate::bg_public_h::TORSO_PATROL;
pub use crate::bg_public_h::TORSO_RAISE;
pub use crate::bg_public_h::TORSO_STAND;
pub use crate::bg_public_h::TORSO_STAND2;
pub use crate::bg_public_h::WP_BFG;
pub use crate::bg_public_h::WP_GAUNTLET;
pub use crate::bg_public_h::WP_GRAPPLING_HOOK;
pub use crate::bg_public_h::WP_GRENADE_LAUNCHER;
pub use crate::bg_public_h::WP_LIGHTNING;
pub use crate::bg_public_h::WP_MACHINEGUN;
pub use crate::bg_public_h::WP_NONE;
pub use crate::bg_public_h::WP_NUM_WEAPONS;
pub use crate::bg_public_h::WP_PLASMAGUN;
pub use crate::bg_public_h::WP_RAILGUN;
pub use crate::bg_public_h::WP_ROCKET_LAUNCHER;
pub use crate::bg_public_h::WP_SHOTGUN;
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
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_players::UI_DrawPlayer;
pub use crate::src::q3_ui::ui_players::UI_PlayerInfo_SetInfo;
pub use crate::src::q3_ui::ui_players::UI_PlayerInfo_SetModel;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::menu_buzz_sound;
pub use crate::src::q3_ui::ui_qmenu::menu_move_sound;
pub use crate::src::q3_ui::ui_qmenu::text_color_normal;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_DefaultKey;
pub use crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursor;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem;
pub use crate::src::qcommon::q_math::colorRed;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_stricmpn;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::Q_strupr;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_FS_GetFileList;
pub use crate::src::ui::ui_syscalls::trap_MemoryRemaining;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_S_RegisterSound;

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
pub use crate::ui_local_h::lerpFrame_t;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::playerInfo_t;
pub use crate::ui_local_h::uiStatic_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct playermodel_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub pics: [crate::ui_local_h::menubitmap_s; 16],
    pub picbuttons: [crate::ui_local_h::menubitmap_s; 16],
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub ports: crate::ui_local_h::menubitmap_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub player: crate::ui_local_h::menubitmap_s,
    pub arrows: crate::ui_local_h::menubitmap_s,
    pub left: crate::ui_local_h::menubitmap_s,
    pub right: crate::ui_local_h::menubitmap_s,
    pub modelname: crate::ui_local_h::menutext_s,
    pub skinname: crate::ui_local_h::menutext_s,
    pub playername: crate::ui_local_h::menutext_s,
    pub playerinfo: crate::ui_local_h::playerInfo_t,
    pub nummodels: i32,
    pub modelnames: [[libc::c_char; 128]; 256],
    pub modelpage: i32,
    pub numpages: i32,
    pub modelskin: [libc::c_char; 64],
    pub selectedmodel: i32,
}

static mut playermodel_artlist: [*mut libc::c_char; 11] = [
    b"menu/art/back_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/opponents_select\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/opponents_selected\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/frame1_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/player_models_ports\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/gs_arrows_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/gs_arrows_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/gs_arrows_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];

static mut s_playermodel: playermodel_t = playermodel_t {
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
    pics: [crate::ui_local_h::menubitmap_s {
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
        focuscolor: 0 as *const f32 as *mut f32,
    }; 16],
    picbuttons: [crate::ui_local_h::menubitmap_s {
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
        focuscolor: 0 as *const f32 as *mut f32,
    }; 16],
    framel: crate::ui_local_h::menubitmap_s {
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
        focuscolor: 0 as *const f32 as *mut f32,
    },
    framer: crate::ui_local_h::menubitmap_s {
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
        focuscolor: 0 as *const f32 as *mut f32,
    },
    ports: crate::ui_local_h::menubitmap_s {
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
        focuscolor: 0 as *const f32 as *mut f32,
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
        color: 0 as *const f32 as *mut f32,
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
        focuscolor: 0 as *const f32 as *mut f32,
    },
    player: crate::ui_local_h::menubitmap_s {
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
        focuscolor: 0 as *const f32 as *mut f32,
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
        focuscolor: 0 as *const f32 as *mut f32,
    },
    left: crate::ui_local_h::menubitmap_s {
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
        focuscolor: 0 as *const f32 as *mut f32,
    },
    right: crate::ui_local_h::menubitmap_s {
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
        focuscolor: 0 as *const f32 as *mut f32,
    },
    modelname: crate::ui_local_h::menutext_s {
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
        color: 0 as *const f32 as *mut f32,
    },
    skinname: crate::ui_local_h::menutext_s {
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
        color: 0 as *const f32 as *mut f32,
    },
    playername: crate::ui_local_h::menutext_s {
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
        color: 0 as *const f32 as *mut f32,
    },
    playerinfo: crate::ui_local_h::playerInfo_t {
        legsModel: 0,
        legsSkin: 0,
        legs: crate::ui_local_h::lerpFrame_t {
            oldFrame: 0,
            oldFrameTime: 0,
            frame: 0,
            frameTime: 0,
            backlerp: 0.,
            yawAngle: 0.,
            yawing: crate::src::qcommon::q_shared::qfalse,
            pitchAngle: 0.,
            pitching: crate::src::qcommon::q_shared::qfalse,
            animationNumber: 0,
            animation: 0 as *const crate::bg_public_h::animation_t
                as *mut crate::bg_public_h::animation_t,
            animationTime: 0,
        },
        torsoModel: 0,
        torsoSkin: 0,
        torso: crate::ui_local_h::lerpFrame_t {
            oldFrame: 0,
            oldFrameTime: 0,
            frame: 0,
            frameTime: 0,
            backlerp: 0.,
            yawAngle: 0.,
            yawing: crate::src::qcommon::q_shared::qfalse,
            pitchAngle: 0.,
            pitching: crate::src::qcommon::q_shared::qfalse,
            animationNumber: 0,
            animation: 0 as *const crate::bg_public_h::animation_t
                as *mut crate::bg_public_h::animation_t,
            animationTime: 0,
        },
        headModel: 0,
        headSkin: 0,
        animations: [crate::bg_public_h::animation_t {
            firstFrame: 0,
            numFrames: 0,
            loopFrames: 0,
            frameLerp: 0,
            initialLerp: 0,
            reversed: 0,
            flipflop: 0,
        }; 31],
        fixedlegs: crate::src::qcommon::q_shared::qfalse,
        fixedtorso: crate::src::qcommon::q_shared::qfalse,
        weaponModel: 0,
        barrelModel: 0,
        flashModel: 0,
        flashDlightColor: [0.; 3],
        muzzleFlashTime: 0,
        color1: [0.; 3],
        c1RGBA: [0; 4],
        viewAngles: [0.; 3],
        moveAngles: [0.; 3],
        currentWeapon: crate::bg_public_h::WP_NONE,
        legsAnim: 0,
        torsoAnim: 0,
        weapon: crate::bg_public_h::WP_NONE,
        lastWeapon: crate::bg_public_h::WP_NONE,
        pendingWeapon: crate::bg_public_h::WP_NONE,
        weaponTimer: 0,
        pendingLegsAnim: 0,
        torsoAnimationTimer: 0,
        pendingTorsoAnim: 0,
        legsAnimationTimer: 0,
        chat: crate::src::qcommon::q_shared::qfalse,
        newModel: crate::src::qcommon::q_shared::qfalse,
        barrelSpinning: crate::src::qcommon::q_shared::qfalse,
        barrelAngle: 0.,
        barrelTime: 0,
        realWeapon: 0,
    },
    nummodels: 0,
    modelnames: [[0; 128]; 256],
    modelpage: 0,
    numpages: 0,
    modelskin: [0; 64],
    selectedmodel: 0,
};
/*
=================
PlayerModel_UpdateGrid
=================
*/

unsafe extern "C" fn PlayerModel_UpdateGrid() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    j = s_playermodel.modelpage * (4 as i32 * 4 as i32);
    i = 0 as i32;
    while i < 4 as i32 * 4 as i32 {
        if j < s_playermodel.nummodels {
            // model/skin portrait
            s_playermodel.pics[i as usize].generic.name =
                s_playermodel.modelnames[j as usize].as_mut_ptr();
            s_playermodel.picbuttons[i as usize].generic.flags &=
                !(0x4000 as i32 as u32)
        } else {
            // dead slot
            s_playermodel.pics[i as usize].generic.name = 0 as *const libc::c_char;
            s_playermodel.picbuttons[i as usize].generic.flags |=
                0x4000 as i32 as u32
        }
        s_playermodel.pics[i as usize].generic.flags &= !(0x40 as i32 as u32);
        s_playermodel.pics[i as usize].shader = 0 as i32;
        s_playermodel.picbuttons[i as usize].generic.flags |= 0x100 as i32 as u32;
        i += 1;
        j += 1
    }
    if s_playermodel.selectedmodel / (4 as i32 * 4 as i32)
        == s_playermodel.modelpage
    {
        // set selected model
        i = s_playermodel.selectedmodel % (4 as i32 * 4 as i32);
        s_playermodel.pics[i as usize].generic.flags |= 0x40 as i32 as u32;
        s_playermodel.picbuttons[i as usize].generic.flags &=
            !(0x100 as i32 as u32)
    }
    if s_playermodel.numpages > 1 as i32 {
        if s_playermodel.modelpage > 0 as i32 {
            s_playermodel.left.generic.flags &= !(0x4000 as i32 as u32)
        } else {
            s_playermodel.left.generic.flags |= 0x4000 as i32 as u32
        }
        if s_playermodel.modelpage < s_playermodel.numpages - 1 as i32 {
            s_playermodel.right.generic.flags &= !(0x4000 as i32 as u32)
        } else {
            s_playermodel.right.generic.flags |= 0x4000 as i32 as u32
        }
    } else {
        // hide left/right markers
        s_playermodel.left.generic.flags |= 0x4000 as i32 as u32;
        s_playermodel.right.generic.flags |= 0x4000 as i32 as u32
    };
}
/*
=================
PlayerModel_UpdateModel
=================
*/

unsafe extern "C" fn PlayerModel_UpdateModel() {
    let mut viewangles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut moveangles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    crate::stdlib::memset(
        &mut s_playermodel.playerinfo as *mut crate::ui_local_h::playerInfo_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::ui_local_h::playerInfo_t>() as libc::c_ulong,
    );
    viewangles[1 as i32 as usize] =
        (180 as i32 - 30 as i32) as crate::src::qcommon::q_shared::vec_t;
    viewangles[0 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    viewangles[2 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    moveangles[2 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    moveangles[1 as i32 as usize] = moveangles[2 as i32 as usize];
    moveangles[0 as i32 as usize] = moveangles[1 as i32 as usize];
    crate::src::q3_ui::ui_players::UI_PlayerInfo_SetModel(
        &mut s_playermodel.playerinfo as *mut _ as *mut crate::ui_local_h::playerInfo_t,
        s_playermodel.modelskin.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_players::UI_PlayerInfo_SetInfo(
        &mut s_playermodel.playerinfo as *mut _ as *mut crate::ui_local_h::playerInfo_t,
        crate::bg_public_h::LEGS_IDLE as i32,
        crate::bg_public_h::TORSO_STAND as i32,
        viewangles.as_mut_ptr(),
        moveangles.as_mut_ptr(),
        crate::bg_public_h::WP_MACHINEGUN,
        crate::src::qcommon::q_shared::qfalse,
    );
}
/*
=================
PlayerModel_SaveChanges
=================
*/

unsafe extern "C" fn PlayerModel_SaveChanges() {
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"model\x00" as *const u8 as *const libc::c_char,
        s_playermodel.modelskin.as_mut_ptr(),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"headmodel\x00" as *const u8 as *const libc::c_char,
        s_playermodel.modelskin.as_mut_ptr(),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"team_model\x00" as *const u8 as *const libc::c_char,
        s_playermodel.modelskin.as_mut_ptr(),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"team_headmodel\x00" as *const u8 as *const libc::c_char,
        s_playermodel.modelskin.as_mut_ptr(),
    );
}
/*
=================
PlayerModel_MenuEvent
=================
*/

unsafe extern "C" fn PlayerModel_MenuEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        100 => {
            if s_playermodel.modelpage > 0 as i32 {
                s_playermodel.modelpage -= 1;
                PlayerModel_UpdateGrid();
            }
        }
        101 => {
            if s_playermodel.modelpage < s_playermodel.numpages - 1 as i32 {
                s_playermodel.modelpage += 1;
                PlayerModel_UpdateGrid();
            }
        }
        102 => {
            PlayerModel_SaveChanges();
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
PlayerModel_MenuKey
=================
*/

unsafe extern "C" fn PlayerModel_MenuKey(
    mut key: i32,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    let mut m: *mut crate::ui_local_h::menucommon_s = 0 as *mut crate::ui_local_h::menucommon_s;
    let mut picnum: i32 = 0;
    match key {
        163 | 134 => {
            m = crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor(
                &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
            ) as *mut crate::ui_local_h::menucommon_s;
            picnum = (*m).id - 0 as i32;
            if picnum >= 0 as i32 && picnum <= 15 as i32 {
                if picnum > 0 as i32 {
                    crate::src::q3_ui::ui_qmenu::Menu_SetCursor(
                        &mut s_playermodel.menu as *mut _
                            as *mut crate::ui_local_h::_tag_menuframework,
                        s_playermodel.menu.cursor - 1 as i32,
                    );
                    return crate::src::q3_ui::ui_qmenu::menu_move_sound;
                } else if s_playermodel.modelpage > 0 as i32 {
                    s_playermodel.modelpage -= 1;
                    crate::src::q3_ui::ui_qmenu::Menu_SetCursor(
                        &mut s_playermodel.menu as *mut _
                            as *mut crate::ui_local_h::_tag_menuframework,
                        s_playermodel.menu.cursor + 15 as i32,
                    );
                    PlayerModel_UpdateGrid();
                    return crate::src::q3_ui::ui_qmenu::menu_move_sound;
                } else {
                    return crate::src::q3_ui::ui_qmenu::menu_buzz_sound;
                }
            }
        }
        165 | 135 => {
            m = crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor(
                &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
            ) as *mut crate::ui_local_h::menucommon_s;
            picnum = (*m).id - 0 as i32;
            if picnum >= 0 as i32 && picnum <= 15 as i32 {
                if picnum < 15 as i32
                    && (s_playermodel.modelpage * (4 as i32 * 4 as i32)
                        + picnum
                        + 1 as i32)
                        < s_playermodel.nummodels
                {
                    crate::src::q3_ui::ui_qmenu::Menu_SetCursor(
                        &mut s_playermodel.menu as *mut _
                            as *mut crate::ui_local_h::_tag_menuframework,
                        s_playermodel.menu.cursor + 1 as i32,
                    );
                    return crate::src::q3_ui::ui_qmenu::menu_move_sound;
                } else if picnum == 15 as i32
                    && s_playermodel.modelpage < s_playermodel.numpages - 1 as i32
                {
                    s_playermodel.modelpage += 1;
                    crate::src::q3_ui::ui_qmenu::Menu_SetCursor(
                        &mut s_playermodel.menu as *mut _
                            as *mut crate::ui_local_h::_tag_menuframework,
                        s_playermodel.menu.cursor - 15 as i32,
                    );
                    PlayerModel_UpdateGrid();
                    return crate::src::q3_ui::ui_qmenu::menu_move_sound;
                } else {
                    return crate::src::q3_ui::ui_qmenu::menu_buzz_sound;
                }
            }
        }
        179 | 27 => {
            PlayerModel_SaveChanges();
        }
        _ => {}
    }
    return crate::src::q3_ui::ui_qmenu::Menu_DefaultKey(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        key,
    );
}
/*
=================
PlayerModel_PicEvent
=================
*/

unsafe extern "C" fn PlayerModel_PicEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut modelnum: i32 = 0;
    let mut maxlen: i32 = 0;
    let mut buffptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pdest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: i32 = 0;
    if event != 3 as i32 {
        return;
    }
    i = 0 as i32;
    while i < 4 as i32 * 4 as i32 {
        // reset
        s_playermodel.pics[i as usize].generic.flags &= !(0x40 as i32 as u32);
        s_playermodel.picbuttons[i as usize].generic.flags |= 0x100 as i32 as u32;
        i += 1
    }
    // set selected
    i = (*(ptr as *mut crate::ui_local_h::menucommon_s)).id - 0 as i32;
    s_playermodel.pics[i as usize].generic.flags |= 0x40 as i32 as u32;
    s_playermodel.picbuttons[i as usize].generic.flags &= !(0x100 as i32 as u32);
    // get model and strip icon_
    modelnum = s_playermodel.modelpage * (4 as i32 * 4 as i32) + i;
    buffptr = s_playermodel.modelnames[modelnum as usize]
        .as_mut_ptr()
        .offset(
            crate::stdlib::strlen(b"models/players/\x00" as *const u8 as *const libc::c_char)
                as isize,
        );
    pdest = ::libc::strstr(buffptr, b"icon_\x00" as *const u8 as *const libc::c_char);
    if !pdest.is_null() {
        // track the whole model/skin name
        crate::src::qcommon::q_shared::Q_strncpyz(
            s_playermodel.modelskin.as_mut_ptr(),
            buffptr,
            (pdest.offset_from(buffptr) as libc::c_long + 1 as i32 as libc::c_long)
                as i32,
        );
        ::libc::strcat(
            s_playermodel.modelskin.as_mut_ptr(),
            pdest.offset(5 as i32 as isize),
        );
        // separate the model name
        maxlen = pdest.offset_from(buffptr) as libc::c_long as i32;
        if maxlen > 16 as i32 {
            maxlen = 16 as i32
        }
        crate::src::qcommon::q_shared::Q_strncpyz(s_playermodel.modelname.string, buffptr, maxlen);
        crate::src::qcommon::q_shared::Q_strupr(s_playermodel.modelname.string);
        // separate the skin name
        maxlen = crate::stdlib::strlen(pdest.offset(5 as i32 as isize))
            .wrapping_add(1 as i32 as libc::c_ulong) as i32;
        if maxlen > 16 as i32 {
            maxlen = 16 as i32
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            s_playermodel.skinname.string,
            pdest.offset(5 as i32 as isize),
            maxlen,
        );
        crate::src::qcommon::q_shared::Q_strupr(s_playermodel.skinname.string);
        s_playermodel.selectedmodel = modelnum;
        if crate::src::ui::ui_syscalls::trap_MemoryRemaining()
            > 5 as i32 * 1024 as i32 * 1024 as i32
        {
            PlayerModel_UpdateModel();
        }
    };
}
/*
=================
PlayerModel_DrawPlayer
=================
*/

unsafe extern "C" fn PlayerModel_DrawPlayer(mut self_0: *mut libc::c_void) {
    let mut b: *mut crate::ui_local_h::menubitmap_s = 0 as *mut crate::ui_local_h::menubitmap_s;
    b = self_0 as *mut crate::ui_local_h::menubitmap_s;
    if crate::src::ui::ui_syscalls::trap_MemoryRemaining()
        <= 5 as i32 * 1024 as i32 * 1024 as i32
    {
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
            (*b).generic.x,
            (*b).generic.y + (*b).height / 2 as i32,
            b"LOW MEMORY\x00" as *const u8 as *const libc::c_char,
            0 as i32,
            crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr(),
        );
        return;
    }
    crate::src::q3_ui::ui_players::UI_DrawPlayer(
        (*b).generic.x as f32,
        (*b).generic.y as f32,
        (*b).width as f32,
        (*b).height as f32,
        &mut s_playermodel.playerinfo as *mut _ as *mut crate::ui_local_h::playerInfo_t,
        crate::src::q3_ui::ui_atoms::uis.realtime / 2 as i32,
    );
}
/*
=================
PlayerModel_BuildList
=================
*/

unsafe extern "C" fn PlayerModel_BuildList() {
    let mut numdirs: i32 = 0;
    let mut numfiles: i32 = 0;
    let mut dirlist: [libc::c_char; 2048] = [0; 2048];
    let mut filelist: [libc::c_char; 2048] = [0; 2048];
    let mut skinname: [libc::c_char; 64] = [0; 64];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fileptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut dirlen: i32 = 0;
    let mut filelen: i32 = 0;
    let mut precache: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    precache = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"com_buildscript\x00" as *const u8 as *const libc::c_char,
    ) as crate::src::qcommon::q_shared::qboolean;
    s_playermodel.modelpage = 0 as i32;
    s_playermodel.nummodels = 0 as i32;
    // iterate directory of all player models
    numdirs = crate::src::ui::ui_syscalls::trap_FS_GetFileList(
        b"models/players\x00" as *const u8 as *const libc::c_char,
        b"/\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        2048 as i32,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0 as i32;
    while i < numdirs && s_playermodel.nummodels < 256 as i32 {
        dirlen = crate::stdlib::strlen(dirptr) as i32;
        if dirlen != 0
            && *dirptr.offset((dirlen - 1 as i32) as isize) as i32 == '/' as i32
        {
            *dirptr.offset((dirlen - 1 as i32) as isize) = '\u{0}' as i32 as libc::c_char
        }
        if !(::libc::strcmp(dirptr, b".\x00" as *const u8 as *const libc::c_char) == 0
            || ::libc::strcmp(dirptr, b"..\x00" as *const u8 as *const libc::c_char) == 0)
        {
            // iterate all skin files in directory
            numfiles = crate::src::ui::ui_syscalls::trap_FS_GetFileList(
                crate::src::qcommon::q_shared::va(
                    b"models/players/%s\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    dirptr,
                ),
                b"tga\x00" as *const u8 as *const libc::c_char,
                filelist.as_mut_ptr(),
                2048 as i32,
            );
            fileptr = filelist.as_mut_ptr();
            j = 0 as i32;
            while j < numfiles && s_playermodel.nummodels < 256 as i32 {
                filelen = crate::stdlib::strlen(fileptr) as i32;
                crate::src::qcommon::q_shared::COM_StripExtension(
                    fileptr,
                    skinname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                );
                // look for icon_????
                if crate::src::qcommon::q_shared::Q_stricmpn(
                    skinname.as_mut_ptr(),
                    b"icon_\x00" as *const u8 as *const libc::c_char,
                    5 as i32,
                ) == 0
                {
                    let fresh0 = s_playermodel.nummodels;
                    s_playermodel.nummodels = s_playermodel.nummodels + 1;
                    crate::src::qcommon::q_shared::Com_sprintf(
                        s_playermodel.modelnames[fresh0 as usize].as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
                            as i32,
                        b"models/players/%s/%s\x00" as *const u8 as *const libc::c_char,
                        dirptr,
                        skinname.as_mut_ptr(),
                    );
                    //if (s_playermodel.nummodels >= MAX_PLAYERMODELS)
                    //	return;
                }
                if precache as u64 != 0 {
                    crate::src::ui::ui_syscalls::trap_S_RegisterSound(
                        crate::src::qcommon::q_shared::va(
                            b"sound/player/announce/%s_wins.wav\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            skinname.as_mut_ptr(),
                        ),
                        crate::src::qcommon::q_shared::qfalse,
                    );
                }
                j += 1;
                fileptr = fileptr.offset((filelen + 1 as i32) as isize)
            }
        }
        i += 1;
        dirptr = dirptr.offset((dirlen + 1 as i32) as isize)
    }
    //APSFIXME - Degenerate no models case
    s_playermodel.numpages = s_playermodel.nummodels / (4 as i32 * 4 as i32);
    if s_playermodel.nummodels % (4 as i32 * 4 as i32) != 0 {
        s_playermodel.numpages += 1
    };
}
/*
=================
PlayerModel_SetMenuItems
=================
*/

unsafe extern "C" fn PlayerModel_SetMenuItems() {
    let mut i: i32 = 0;
    let mut maxlen: i32 = 0;
    let mut modelskin: [libc::c_char; 64] = [0; 64];
    let mut buffptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pdest: *mut libc::c_char = 0 as *mut libc::c_char;
    // name
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"name\x00" as *const u8 as *const libc::c_char,
        s_playermodel.playername.string,
        16 as i32,
    );
    crate::src::qcommon::q_shared::Q_CleanStr(s_playermodel.playername.string);
    // model
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const libc::c_char,
        s_playermodel.modelskin.as_mut_ptr(),
        64 as i32,
    );
    // use default skin if none is set
    if ::libc::strchr(s_playermodel.modelskin.as_mut_ptr(), '/' as i32).is_null() {
        crate::src::qcommon::q_shared::Q_strcat(
            s_playermodel.modelskin.as_mut_ptr(),
            64 as i32,
            b"/default\x00" as *const u8 as *const libc::c_char,
        );
    }
    // find model in our list
    i = 0 as i32;
    while i < s_playermodel.nummodels {
        // strip icon_
        buffptr = s_playermodel.modelnames[i as usize]
            .as_mut_ptr()
            .offset(crate::stdlib::strlen(
                b"models/players/\x00" as *const u8 as *const libc::c_char,
            ) as isize);
        pdest = ::libc::strstr(buffptr, b"icon_\x00" as *const u8 as *const libc::c_char);
        if !pdest.is_null() {
            crate::src::qcommon::q_shared::Q_strncpyz(
                modelskin.as_mut_ptr(),
                buffptr,
                (pdest.offset_from(buffptr) as libc::c_long + 1 as i32 as libc::c_long)
                    as i32,
            );
            ::libc::strcat(
                modelskin.as_mut_ptr(),
                pdest.offset(5 as i32 as isize),
            );
            if crate::src::qcommon::q_shared::Q_stricmp(
                s_playermodel.modelskin.as_mut_ptr(),
                modelskin.as_mut_ptr(),
            ) == 0
            {
                // found pic, set selection here
                s_playermodel.selectedmodel = i;
                s_playermodel.modelpage = i / (4 as i32 * 4 as i32);
                // separate the model name
                maxlen = pdest.offset_from(buffptr) as libc::c_long as i32;
                if maxlen > 16 as i32 {
                    maxlen = 16 as i32
                }
                crate::src::qcommon::q_shared::Q_strncpyz(
                    s_playermodel.modelname.string,
                    buffptr,
                    maxlen,
                );
                crate::src::qcommon::q_shared::Q_strupr(s_playermodel.modelname.string);
                // separate the skin name
                maxlen = crate::stdlib::strlen(pdest.offset(5 as i32 as isize))
                    .wrapping_add(1 as i32 as libc::c_ulong)
                    as i32;
                if maxlen > 16 as i32 {
                    maxlen = 16 as i32
                }
                crate::src::qcommon::q_shared::Q_strncpyz(
                    s_playermodel.skinname.string,
                    pdest.offset(5 as i32 as isize),
                    maxlen,
                );
                crate::src::qcommon::q_shared::Q_strupr(s_playermodel.skinname.string);
                break;
            }
        }
        i += 1
    }
}
/*
=================
PlayerModel_MenuInit
=================
*/

unsafe extern "C" fn PlayerModel_MenuInit() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    static mut playername: [libc::c_char; 32] = [0; 32];
    static mut modelname: [libc::c_char; 32] = [0; 32];
    static mut skinname: [libc::c_char; 32] = [0; 32];
    // zero set all our globals
    crate::stdlib::memset(
        &mut s_playermodel as *mut playermodel_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<playermodel_t>() as libc::c_ulong,
    );
    PlayerModel_Cache();
    s_playermodel.menu.key = Some(
        PlayerModel_MenuKey
            as unsafe extern "C" fn(_: i32) -> crate::src::qcommon::q_shared::sfxHandle_t,
    );
    s_playermodel.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_playermodel.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_playermodel.banner.generic.type_0 = 10 as i32;
    s_playermodel.banner.generic.x = 320 as i32;
    s_playermodel.banner.generic.y = 16 as i32;
    s_playermodel.banner.string =
        b"PLAYER MODEL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_playermodel.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    s_playermodel.banner.style = 0x1 as i32;
    s_playermodel.framel.generic.type_0 = 6 as i32;
    s_playermodel.framel.generic.name =
        b"menu/art/frame1_l\x00" as *const u8 as *const libc::c_char;
    s_playermodel.framel.generic.flags =
        0x4 as i32 as u32 | 0x4000 as i32 as u32;
    s_playermodel.framel.generic.x = 0 as i32;
    s_playermodel.framel.generic.y = 78 as i32;
    s_playermodel.framel.width = 256 as i32;
    s_playermodel.framel.height = 329 as i32;
    s_playermodel.framer.generic.type_0 = 6 as i32;
    s_playermodel.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_playermodel.framer.generic.flags =
        0x4 as i32 as u32 | 0x4000 as i32 as u32;
    s_playermodel.framer.generic.x = 376 as i32;
    s_playermodel.framer.generic.y = 76 as i32;
    s_playermodel.framer.width = 256 as i32;
    s_playermodel.framer.height = 334 as i32;
    s_playermodel.ports.generic.type_0 = 6 as i32;
    s_playermodel.ports.generic.name =
        b"menu/art/player_models_ports\x00" as *const u8 as *const libc::c_char;
    s_playermodel.ports.generic.flags =
        0x4 as i32 as u32 | 0x4000 as i32 as u32;
    s_playermodel.ports.generic.x = 50 as i32;
    s_playermodel.ports.generic.y = 59 as i32;
    s_playermodel.ports.width = 274 as i32;
    s_playermodel.ports.height = 274 as i32;
    y = 59 as i32;
    i = 0 as i32;
    k = 0 as i32;
    while i < 4 as i32 {
        x = 50 as i32;
        j = 0 as i32;
        while j < 4 as i32 {
            s_playermodel.pics[k as usize].generic.type_0 = 6 as i32;
            s_playermodel.pics[k as usize].generic.flags =
                0x4 as i32 as u32 | 0x4000 as i32 as u32;
            s_playermodel.pics[k as usize].generic.x = x;
            s_playermodel.pics[k as usize].generic.y = y;
            s_playermodel.pics[k as usize].width = 64 as i32;
            s_playermodel.pics[k as usize].height = 64 as i32;
            s_playermodel.pics[k as usize].focuspic =
                b"menu/art/opponents_selected\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            s_playermodel.pics[k as usize].focuscolor =
                crate::src::qcommon::q_math::colorRed.as_mut_ptr();
            s_playermodel.picbuttons[k as usize].generic.type_0 = 6 as i32;
            s_playermodel.picbuttons[k as usize].generic.flags = 0x4 as i32 as u32
                | 0x8000 as i32 as u32
                | 0x100 as i32 as u32;
            s_playermodel.picbuttons[k as usize].generic.id = 0 as i32 + k;
            s_playermodel.picbuttons[k as usize].generic.callback = Some(
                PlayerModel_PicEvent
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
            );
            s_playermodel.picbuttons[k as usize].generic.x = x - 16 as i32;
            s_playermodel.picbuttons[k as usize].generic.y = y - 16 as i32;
            s_playermodel.picbuttons[k as usize].generic.left = x;
            s_playermodel.picbuttons[k as usize].generic.top = y;
            s_playermodel.picbuttons[k as usize].generic.right = x + 64 as i32;
            s_playermodel.picbuttons[k as usize].generic.bottom = y + 64 as i32;
            s_playermodel.picbuttons[k as usize].width = 128 as i32;
            s_playermodel.picbuttons[k as usize].height = 128 as i32;
            s_playermodel.picbuttons[k as usize].focuspic =
                b"menu/art/opponents_select\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            s_playermodel.picbuttons[k as usize].focuscolor =
                crate::src::qcommon::q_math::colorRed.as_mut_ptr();
            x += 64 as i32 + 6 as i32;
            j += 1;
            k += 1
        }
        y += 64 as i32 + 6 as i32;
        i += 1
    }
    s_playermodel.playername.generic.type_0 = 9 as i32;
    s_playermodel.playername.generic.flags =
        0x8 as i32 as u32 | 0x4000 as i32 as u32;
    s_playermodel.playername.generic.x = 320 as i32;
    s_playermodel.playername.generic.y = 440 as i32;
    s_playermodel.playername.string = playername.as_mut_ptr();
    s_playermodel.playername.style = 0x1 as i32;
    s_playermodel.playername.color = crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr();
    s_playermodel.modelname.generic.type_0 = 9 as i32;
    s_playermodel.modelname.generic.flags =
        0x8 as i32 as u32 | 0x4000 as i32 as u32;
    s_playermodel.modelname.generic.x = 497 as i32;
    s_playermodel.modelname.generic.y = 54 as i32;
    s_playermodel.modelname.string = modelname.as_mut_ptr();
    s_playermodel.modelname.style = 0x1 as i32;
    s_playermodel.modelname.color = crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr();
    s_playermodel.skinname.generic.type_0 = 9 as i32;
    s_playermodel.skinname.generic.flags =
        0x8 as i32 as u32 | 0x4000 as i32 as u32;
    s_playermodel.skinname.generic.x = 497 as i32;
    s_playermodel.skinname.generic.y = 394 as i32;
    s_playermodel.skinname.string = skinname.as_mut_ptr();
    s_playermodel.skinname.style = 0x1 as i32;
    s_playermodel.skinname.color = crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr();
    s_playermodel.player.generic.type_0 = 6 as i32;
    s_playermodel.player.generic.flags = 0x4000 as i32 as u32;
    s_playermodel.player.generic.ownerdraw =
        Some(PlayerModel_DrawPlayer as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_playermodel.player.generic.x = 400 as i32;
    s_playermodel.player.generic.y = -(40 as i32);
    s_playermodel.player.width = 32 as i32 * 10 as i32;
    s_playermodel.player.height = 56 as i32 * 10 as i32;
    s_playermodel.arrows.generic.type_0 = 6 as i32;
    s_playermodel.arrows.generic.name =
        b"menu/art/gs_arrows_0\x00" as *const u8 as *const libc::c_char;
    s_playermodel.arrows.generic.flags = 0x4000 as i32 as u32;
    s_playermodel.arrows.generic.x = 125 as i32;
    s_playermodel.arrows.generic.y = 340 as i32;
    s_playermodel.arrows.width = 128 as i32;
    s_playermodel.arrows.height = 32 as i32;
    s_playermodel.left.generic.type_0 = 6 as i32;
    s_playermodel.left.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_playermodel.left.generic.callback = Some(
        PlayerModel_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    s_playermodel.left.generic.id = 100 as i32;
    s_playermodel.left.generic.x = 125 as i32;
    s_playermodel.left.generic.y = 340 as i32;
    s_playermodel.left.width = 64 as i32;
    s_playermodel.left.height = 32 as i32;
    s_playermodel.left.focuspic =
        b"menu/art/gs_arrows_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_playermodel.right.generic.type_0 = 6 as i32;
    s_playermodel.right.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_playermodel.right.generic.callback = Some(
        PlayerModel_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    s_playermodel.right.generic.id = 101 as i32;
    s_playermodel.right.generic.x = 125 as i32 + 61 as i32;
    s_playermodel.right.generic.y = 340 as i32;
    s_playermodel.right.width = 64 as i32;
    s_playermodel.right.height = 32 as i32;
    s_playermodel.right.focuspic =
        b"menu/art/gs_arrows_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_playermodel.back.generic.type_0 = 6 as i32;
    s_playermodel.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_playermodel.back.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_playermodel.back.generic.callback = Some(
        PlayerModel_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    s_playermodel.back.generic.id = 102 as i32;
    s_playermodel.back.generic.x = 0 as i32;
    s_playermodel.back.generic.y = 480 as i32 - 64 as i32;
    s_playermodel.back.width = 128 as i32;
    s_playermodel.back.height = 64 as i32;
    s_playermodel.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playermodel.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playermodel.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playermodel.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playermodel.ports as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playermodel.playername as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playermodel.modelname as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playermodel.skinname as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    i = 0 as i32;
    while i < 4 as i32 * 4 as i32 {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
            &mut *s_playermodel.pics.as_mut_ptr().offset(i as isize)
                as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
        );
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
            &mut *s_playermodel.picbuttons.as_mut_ptr().offset(i as isize)
                as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
        );
        i += 1
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playermodel.player as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playermodel.arrows as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playermodel.left as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playermodel.right as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playermodel.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    // find all available models
    //	PlayerModel_BuildList();
    // set initial states
    PlayerModel_SetMenuItems();
    // update user interface
    PlayerModel_UpdateGrid();
    PlayerModel_UpdateModel();
}
/*
=================
PlayerModel_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn PlayerModel_Cache() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while !playermodel_artlist[i as usize].is_null() {
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(playermodel_artlist[i as usize]);
        i += 1
    }
    PlayerModel_BuildList();
    i = 0 as i32;
    while i < s_playermodel.nummodels {
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            s_playermodel.modelnames[i as usize].as_mut_ptr(),
        );
        i += 1
    }
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
//
//NOTE: include the ui_public.h from the new UI
//redefine to old API version
//
// ui_qmenu.c
//
// edit field is only numbers
// steady focus
// pulse if focus
// only mouse input allowed
// skips drawing
// grays and disables
// disables any input
// skip default initialization
// edit field is all lower case
// edit field is all upper case
// callback notifications
//
// ui_mfield.c
//
//
// ui_menu.c
//
//
// ui_credits.c
//
//
// ui_ingame.c
//
//
// ui_confirm.c
//
//
// ui_setup.c
//
//
// ui_team.c
//
//
// ui_connect.c
//
//
// ui_controls2.c
//
//
// ui_demo2.c
//
//
// ui_cinematics.c
//
//
// ui_mods.c
//
//
// ui_cdkey.c
//
//
// ui_playermodel.c
//
#[no_mangle]

pub unsafe extern "C" fn UI_PlayerModelMenu() {
    PlayerModel_MenuInit();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
    crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem(
        &mut s_playermodel.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut *s_playermodel
            .pics
            .as_mut_ptr()
            .offset((s_playermodel.selectedmodel % (4 as i32 * 4 as i32)) as isize)
            as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
}

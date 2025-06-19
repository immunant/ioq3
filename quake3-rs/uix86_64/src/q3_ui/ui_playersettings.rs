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
pub use crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawChar;
pub use crate::src::q3_ui::ui_atoms::UI_DrawHandlePic;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_playermodel::UI_PlayerModelMenu;
pub use crate::src::q3_ui::ui_players::UI_DrawPlayer;
pub use crate::src::q3_ui::ui_players::UI_PlayerInfo_SetInfo;
pub use crate::src::q3_ui::ui_players::UI_PlayerInfo_SetModel;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::text_color_highlight;
pub use crate::src::q3_ui::ui_qmenu::text_color_normal;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_DefaultKey;
pub use crate::src::qcommon::q_math::g_color_table;
pub use crate::src::qcommon::q_math::vec3_origin;
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
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_IsColorString;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_Key_GetOverstrikeMode;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

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
pub use crate::ui_local_h::menufield_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::mfield_t;
pub use crate::ui_local_h::playerInfo_t;
pub use crate::ui_local_h::uiStatic_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct playersettings_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub player: crate::ui_local_h::menubitmap_s,
    pub name: crate::ui_local_h::menufield_s,
    pub handicap: crate::ui_local_h::menulist_s,
    pub effects: crate::ui_local_h::menulist_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub model: crate::ui_local_h::menubitmap_s,
    pub item_null: crate::ui_local_h::menubitmap_s,
    pub fxBasePic: crate::src::qcommon::q_shared::qhandle_t,
    pub fxPic: [crate::src::qcommon::q_shared::qhandle_t; 7],
    pub playerinfo: crate::ui_local_h::playerInfo_t,
    pub current_fx: i32,
    pub playerModel: [libc::c_char; 64],
}

static mut s_playersettings: playersettings_t = playersettings_t {
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
        color: 0 as *const f32 as *mut f32,
    },
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
    name: crate::ui_local_h::menufield_s {
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
        field: crate::ui_local_h::mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    handicap: crate::ui_local_h::menulist_s {
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
    effects: crate::ui_local_h::menulist_s {
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
    model: crate::ui_local_h::menubitmap_s {
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
    item_null: crate::ui_local_h::menubitmap_s {
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
    fxBasePic: 0,
    fxPic: [0; 7],
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
    current_fx: 0,
    playerModel: [0; 64],
};

static mut gamecodetoui: [i32; 7] = [
    4 as i32,
    2 as i32,
    3 as i32,
    0 as i32,
    5 as i32,
    1 as i32,
    6 as i32,
];

static mut uitogamecode: [i32; 7] = [
    4 as i32,
    6 as i32,
    2 as i32,
    3 as i32,
    1 as i32,
    5 as i32,
    7 as i32,
];

static mut handicap_items: [*const libc::c_char; 21] = [
    b"None\x00" as *const u8 as *const libc::c_char,
    b"95\x00" as *const u8 as *const libc::c_char,
    b"90\x00" as *const u8 as *const libc::c_char,
    b"85\x00" as *const u8 as *const libc::c_char,
    b"80\x00" as *const u8 as *const libc::c_char,
    b"75\x00" as *const u8 as *const libc::c_char,
    b"70\x00" as *const u8 as *const libc::c_char,
    b"65\x00" as *const u8 as *const libc::c_char,
    b"60\x00" as *const u8 as *const libc::c_char,
    b"55\x00" as *const u8 as *const libc::c_char,
    b"50\x00" as *const u8 as *const libc::c_char,
    b"45\x00" as *const u8 as *const libc::c_char,
    b"40\x00" as *const u8 as *const libc::c_char,
    b"35\x00" as *const u8 as *const libc::c_char,
    b"30\x00" as *const u8 as *const libc::c_char,
    b"25\x00" as *const u8 as *const libc::c_char,
    b"20\x00" as *const u8 as *const libc::c_char,
    b"15\x00" as *const u8 as *const libc::c_char,
    b"10\x00" as *const u8 as *const libc::c_char,
    b"5\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
=================
PlayerSettings_DrawName
=================
*/

unsafe extern "C" fn PlayerSettings_DrawName(mut self_0: *mut libc::c_void) {
    let mut f: *mut crate::ui_local_h::menufield_s = 0 as *mut crate::ui_local_h::menufield_s;
    let mut focus: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut style: i32 = 0;
    let mut txt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut color: *mut f32 = 0 as *mut f32;
    let mut n: i32 = 0;
    let mut basex: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut name: [libc::c_char; 32] = [0; 32];
    f = self_0 as *mut crate::ui_local_h::menufield_s;
    basex = (*f).generic.x;
    y = (*f).generic.y;
    focus = ((*(*f).generic.parent).cursor == (*f).generic.menuPosition) as i32
        as crate::src::qcommon::q_shared::qboolean;
    style = 0 as i32 | 0x10 as i32;
    color = crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr();
    if focus as u64 != 0 {
        style |= 0x4000 as i32;
        color = crate::src::q3_ui::ui_qmenu::text_color_highlight.as_mut_ptr()
    }
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        basex,
        y,
        b"Name\x00" as *const u8 as *const libc::c_char,
        style,
        color,
    );
    // draw the actual name
    basex += 64 as i32;
    y += 27 as i32;
    txt = (*f).field.buffer.as_mut_ptr();
    color = crate::src::qcommon::q_math::g_color_table
        [('7' as i32 - '0' as i32 & 0x7 as i32) as usize]
        .as_mut_ptr();
    x = basex;
    loop {
        c = *txt;
        if !(c as i32 != 0 as i32) {
            break;
        }
        if focus as u64 == 0
            && crate::src::qcommon::q_shared::Q_IsColorString(txt) as u32 != 0
        {
            n = *txt.offset(1 as i32 as isize) as i32 - '0' as i32
                & 0x7 as i32;
            if n == 0 as i32 {
                n = 7 as i32
            }
            color = crate::src::qcommon::q_math::g_color_table[n as usize].as_mut_ptr();
            txt = txt.offset(2 as i32 as isize)
        } else {
            crate::src::q3_ui::ui_atoms::UI_DrawChar(x, y, c as i32, style, color);
            txt = txt.offset(1);
            x += 8 as i32
        }
    }
    // draw cursor if we have focus
    if focus as u64 != 0 {
        if crate::src::ui::ui_syscalls::trap_Key_GetOverstrikeMode() as u64 != 0 {
            c = 11 as i32 as libc::c_char
        } else {
            c = 10 as i32 as libc::c_char
        }
        style &= !(0x4000 as i32);
        style |= 0x1000 as i32;
        crate::src::q3_ui::ui_atoms::UI_DrawChar(
            basex + (*f).field.cursor * 8 as i32,
            y,
            c as i32,
            style,
            crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
        );
    }
    // draw at bottom also using proportional font
    crate::src::qcommon::q_shared::Q_strncpyz(
        name.as_mut_ptr(),
        (*f).field.buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_CleanStr(name.as_mut_ptr());
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        320 as i32,
        440 as i32,
        name.as_mut_ptr(),
        0x1 as i32 | 0x20 as i32,
        crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
    );
}
/*
=================
PlayerSettings_DrawHandicap
=================
*/

unsafe extern "C" fn PlayerSettings_DrawHandicap(mut self_0: *mut libc::c_void) {
    let mut item: *mut crate::ui_local_h::menulist_s = 0 as *mut crate::ui_local_h::menulist_s;
    let mut focus: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut style: i32 = 0;
    let mut color: *mut f32 = 0 as *mut f32;
    item = self_0 as *mut crate::ui_local_h::menulist_s;
    focus = ((*(*item).generic.parent).cursor == (*item).generic.menuPosition) as i32
        as crate::src::qcommon::q_shared::qboolean;
    style = 0 as i32 | 0x10 as i32;
    color = crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr();
    if focus as u64 != 0 {
        style |= 0x4000 as i32;
        color = crate::src::q3_ui::ui_qmenu::text_color_highlight.as_mut_ptr()
    }
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        (*item).generic.x,
        (*item).generic.y,
        b"Handicap\x00" as *const u8 as *const libc::c_char,
        style,
        color,
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        (*item).generic.x + 64 as i32,
        (*item).generic.y + 27 as i32,
        handicap_items[(*item).curvalue as usize],
        style,
        color,
    );
}
/*
=================
PlayerSettings_DrawEffects
=================
*/

unsafe extern "C" fn PlayerSettings_DrawEffects(mut self_0: *mut libc::c_void) {
    let mut item: *mut crate::ui_local_h::menulist_s = 0 as *mut crate::ui_local_h::menulist_s;
    let mut focus: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut style: i32 = 0;
    let mut color: *mut f32 = 0 as *mut f32;
    item = self_0 as *mut crate::ui_local_h::menulist_s;
    focus = ((*(*item).generic.parent).cursor == (*item).generic.menuPosition) as i32
        as crate::src::qcommon::q_shared::qboolean;
    style = 0 as i32 | 0x10 as i32;
    color = crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr();
    if focus as u64 != 0 {
        style |= 0x4000 as i32;
        color = crate::src::q3_ui::ui_qmenu::text_color_highlight.as_mut_ptr()
    }
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        (*item).generic.x,
        (*item).generic.y,
        b"Effects\x00" as *const u8 as *const libc::c_char,
        style,
        color,
    );
    crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
        ((*item).generic.x + 64 as i32) as f32,
        ((*item).generic.y + 27 as i32 + 8 as i32) as f32,
        128 as i32 as f32,
        8 as i32 as f32,
        s_playersettings.fxBasePic,
    );
    crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
        ((*item).generic.x
            + 64 as i32
            + (*item).curvalue * 16 as i32
            + 8 as i32) as f32,
        ((*item).generic.y + 27 as i32 + 6 as i32) as f32,
        16 as i32 as f32,
        12 as i32 as f32,
        s_playersettings.fxPic[(*item).curvalue as usize],
    );
}
/*
=================
PlayerSettings_DrawPlayer
=================
*/

unsafe extern "C" fn PlayerSettings_DrawPlayer(mut self_0: *mut libc::c_void) {
    let mut b: *mut crate::ui_local_h::menubitmap_s = 0 as *mut crate::ui_local_h::menubitmap_s;
    let mut viewangles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut buf: [libc::c_char; 64] = [0; 64];
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    if ::libc::strcmp(buf.as_mut_ptr(), s_playersettings.playerModel.as_mut_ptr())
        != 0 as i32
    {
        crate::src::q3_ui::ui_players::UI_PlayerInfo_SetModel(
            &mut s_playersettings.playerinfo as *mut _ as *mut crate::ui_local_h::playerInfo_t,
            buf.as_mut_ptr(),
        );
        ::libc::strcpy(s_playersettings.playerModel.as_mut_ptr(), buf.as_mut_ptr());
        viewangles[1 as i32 as usize] =
            (180 as i32 - 30 as i32) as crate::src::qcommon::q_shared::vec_t;
        viewangles[0 as i32 as usize] =
            0 as i32 as crate::src::qcommon::q_shared::vec_t;
        viewangles[2 as i32 as usize] =
            0 as i32 as crate::src::qcommon::q_shared::vec_t;
        crate::src::q3_ui::ui_players::UI_PlayerInfo_SetInfo(
            &mut s_playersettings.playerinfo as *mut _ as *mut crate::ui_local_h::playerInfo_t,
            crate::bg_public_h::LEGS_IDLE as i32,
            crate::bg_public_h::TORSO_STAND as i32,
            viewangles.as_mut_ptr(),
            crate::src::qcommon::q_math::vec3_origin.as_mut_ptr(),
            crate::bg_public_h::WP_MACHINEGUN,
            crate::src::qcommon::q_shared::qfalse,
        );
    }
    b = self_0 as *mut crate::ui_local_h::menubitmap_s;
    crate::src::q3_ui::ui_players::UI_DrawPlayer(
        (*b).generic.x as f32,
        (*b).generic.y as f32,
        (*b).width as f32,
        (*b).height as f32,
        &mut s_playersettings.playerinfo as *mut _ as *mut crate::ui_local_h::playerInfo_t,
        crate::src::q3_ui::ui_atoms::uis.realtime / 2 as i32,
    );
}
/*
=================
PlayerSettings_SaveChanges
=================
*/

unsafe extern "C" fn PlayerSettings_SaveChanges() {
    // name
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"name\x00" as *const u8 as *const libc::c_char,
        s_playersettings.name.field.buffer.as_mut_ptr(),
    );
    // handicap
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"handicap\x00" as *const u8 as *const libc::c_char,
        (100 as i32 - s_playersettings.handicap.curvalue * 5 as i32)
            as f32,
    );
    // effects color
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"color1\x00" as *const u8 as *const libc::c_char,
        uitogamecode[s_playersettings.effects.curvalue as usize] as f32,
    );
}
/*
=================
PlayerSettings_MenuKey
=================
*/

unsafe extern "C" fn PlayerSettings_MenuKey(
    mut key: i32,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    if key == crate::keycodes_h::K_MOUSE2 as i32
        || key == crate::keycodes_h::K_ESCAPE as i32
    {
        PlayerSettings_SaveChanges();
    }
    return crate::src::q3_ui::ui_qmenu::Menu_DefaultKey(
        &mut s_playersettings.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        key,
    );
}
/*
=================
PlayerSettings_SetMenuItems
=================
*/

unsafe extern "C" fn PlayerSettings_SetMenuItems() {
    let mut viewangles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut c: i32 = 0;
    let mut h: i32 = 0;
    // name
    crate::src::qcommon::q_shared::Q_strncpyz(
        s_playersettings.name.field.buffer.as_mut_ptr(),
        crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString(
            b"name\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
    );
    // effects color
    c = (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"color1\x00" as *const u8 as *const libc::c_char,
    ) - 1 as i32 as f32) as i32;
    if c < 0 as i32 || c > 6 as i32 {
        c = 6 as i32
    }
    s_playersettings.effects.curvalue = gamecodetoui[c as usize];
    // model/skin
    crate::stdlib::memset(
        &mut s_playersettings.playerinfo as *mut crate::ui_local_h::playerInfo_t
            as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::ui_local_h::playerInfo_t>() as libc::c_ulong,
    );
    viewangles[1 as i32 as usize] =
        (180 as i32 - 30 as i32) as crate::src::qcommon::q_shared::vec_t;
    viewangles[0 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    viewangles[2 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::q3_ui::ui_players::UI_PlayerInfo_SetModel(
        &mut s_playersettings.playerinfo as *mut _ as *mut crate::ui_local_h::playerInfo_t,
        crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString(
            b"model\x00" as *const u8 as *const libc::c_char,
        ),
    );
    crate::src::q3_ui::ui_players::UI_PlayerInfo_SetInfo(
        &mut s_playersettings.playerinfo as *mut _ as *mut crate::ui_local_h::playerInfo_t,
        crate::bg_public_h::LEGS_IDLE as i32,
        crate::bg_public_h::TORSO_STAND as i32,
        viewangles.as_mut_ptr(),
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr(),
        crate::bg_public_h::WP_MACHINEGUN,
        crate::src::qcommon::q_shared::qfalse,
    );
    // handicap
    h = crate::src::qcommon::q_shared::Com_Clamp(
        5 as i32 as f32,
        100 as i32 as f32,
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"handicap\x00" as *const u8 as *const libc::c_char,
        ),
    ) as i32;
    s_playersettings.handicap.curvalue = 20 as i32 - h / 5 as i32;
}
/*
=================
PlayerSettings_MenuEvent
=================
*/

unsafe extern "C" fn PlayerSettings_MenuEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        11 => {
            crate::src::ui::ui_syscalls::trap_Cvar_Set(
                b"handicap\x00" as *const u8 as *const libc::c_char,
                crate::src::qcommon::q_shared::va(
                    b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    100 as i32 - 25 as i32 * s_playersettings.handicap.curvalue,
                ),
            );
        }
        14 => {
            PlayerSettings_SaveChanges();
            crate::src::q3_ui::ui_playermodel::UI_PlayerModelMenu();
        }
        13 => {
            PlayerSettings_SaveChanges();
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
PlayerSettings_MenuInit
=================
*/

unsafe extern "C" fn PlayerSettings_MenuInit() {
    let mut y: i32 = 0;
    crate::stdlib::memset(
        &mut s_playersettings as *mut playersettings_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<playersettings_t>() as libc::c_ulong,
    );
    PlayerSettings_Cache();
    s_playersettings.menu.key = Some(
        PlayerSettings_MenuKey
            as unsafe extern "C" fn(_: i32) -> crate::src::qcommon::q_shared::sfxHandle_t,
    );
    s_playersettings.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_playersettings.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_playersettings.banner.generic.type_0 = 10 as i32;
    s_playersettings.banner.generic.x = 320 as i32;
    s_playersettings.banner.generic.y = 16 as i32;
    s_playersettings.banner.string =
        b"PLAYER SETTINGS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_playersettings.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    s_playersettings.banner.style = 0x1 as i32;
    s_playersettings.framel.generic.type_0 = 6 as i32;
    s_playersettings.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_playersettings.framel.generic.flags =
        0x4 as i32 as u32 | 0x4000 as i32 as u32;
    s_playersettings.framel.generic.x = 0 as i32;
    s_playersettings.framel.generic.y = 78 as i32;
    s_playersettings.framel.width = 256 as i32;
    s_playersettings.framel.height = 329 as i32;
    s_playersettings.framer.generic.type_0 = 6 as i32;
    s_playersettings.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_playersettings.framer.generic.flags =
        0x4 as i32 as u32 | 0x4000 as i32 as u32;
    s_playersettings.framer.generic.x = 376 as i32;
    s_playersettings.framer.generic.y = 76 as i32;
    s_playersettings.framer.width = 256 as i32;
    s_playersettings.framer.height = 334 as i32;
    y = 144 as i32;
    s_playersettings.name.generic.type_0 = 4 as i32;
    s_playersettings.name.generic.flags = 0x8000 as i32 as u32;
    s_playersettings.name.generic.ownerdraw =
        Some(PlayerSettings_DrawName as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_playersettings.name.field.widthInChars = 20 as i32;
    s_playersettings.name.field.maxchars = 20 as i32;
    s_playersettings.name.generic.x = 192 as i32;
    s_playersettings.name.generic.y = y;
    s_playersettings.name.generic.left = 192 as i32 - 8 as i32;
    s_playersettings.name.generic.top = y - 8 as i32;
    s_playersettings.name.generic.right = 192 as i32 + 200 as i32;
    s_playersettings.name.generic.bottom = y + 2 as i32 * 27 as i32;
    y += 3 as i32 * 27 as i32;
    s_playersettings.handicap.generic.type_0 = 3 as i32;
    s_playersettings.handicap.generic.flags = 0x8000 as i32 as u32;
    s_playersettings.handicap.generic.id = 11 as i32;
    s_playersettings.handicap.generic.ownerdraw =
        Some(PlayerSettings_DrawHandicap as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_playersettings.handicap.generic.x = 192 as i32;
    s_playersettings.handicap.generic.y = y;
    s_playersettings.handicap.generic.left = 192 as i32 - 8 as i32;
    s_playersettings.handicap.generic.top = y - 8 as i32;
    s_playersettings.handicap.generic.right = 192 as i32 + 200 as i32;
    s_playersettings.handicap.generic.bottom = y + 2 as i32 * 27 as i32;
    s_playersettings.handicap.numitems = 20 as i32;
    y += 3 as i32 * 27 as i32;
    s_playersettings.effects.generic.type_0 = 3 as i32;
    s_playersettings.effects.generic.flags = 0x8000 as i32 as u32;
    s_playersettings.effects.generic.id = 12 as i32;
    s_playersettings.effects.generic.ownerdraw =
        Some(PlayerSettings_DrawEffects as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_playersettings.effects.generic.x = 192 as i32;
    s_playersettings.effects.generic.y = y;
    s_playersettings.effects.generic.left = 192 as i32 - 8 as i32;
    s_playersettings.effects.generic.top = y - 8 as i32;
    s_playersettings.effects.generic.right = 192 as i32 + 200 as i32;
    s_playersettings.effects.generic.bottom = y + 2 as i32 * 27 as i32;
    s_playersettings.effects.numitems = 7 as i32;
    s_playersettings.model.generic.type_0 = 6 as i32;
    s_playersettings.model.generic.name =
        b"menu/art/model_0\x00" as *const u8 as *const libc::c_char;
    s_playersettings.model.generic.flags =
        0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_playersettings.model.generic.id = 14 as i32;
    s_playersettings.model.generic.callback = Some(
        PlayerSettings_MenuEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    s_playersettings.model.generic.x = 640 as i32;
    s_playersettings.model.generic.y = 480 as i32 - 64 as i32;
    s_playersettings.model.width = 128 as i32;
    s_playersettings.model.height = 64 as i32;
    s_playersettings.model.focuspic =
        b"menu/art/model_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_playersettings.player.generic.type_0 = 6 as i32;
    s_playersettings.player.generic.flags = 0x4000 as i32 as u32;
    s_playersettings.player.generic.ownerdraw =
        Some(PlayerSettings_DrawPlayer as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_playersettings.player.generic.x = 400 as i32;
    s_playersettings.player.generic.y = -(40 as i32);
    s_playersettings.player.width = 32 as i32 * 10 as i32;
    s_playersettings.player.height = 56 as i32 * 10 as i32;
    s_playersettings.back.generic.type_0 = 6 as i32;
    s_playersettings.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_playersettings.back.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_playersettings.back.generic.id = 13 as i32;
    s_playersettings.back.generic.callback = Some(
        PlayerSettings_MenuEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    s_playersettings.back.generic.x = 0 as i32;
    s_playersettings.back.generic.y = 480 as i32 - 64 as i32;
    s_playersettings.back.width = 128 as i32;
    s_playersettings.back.height = 64 as i32;
    s_playersettings.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_playersettings.item_null.generic.type_0 = 6 as i32;
    s_playersettings.item_null.generic.flags = 0x4 as i32 as u32
        | 0x800 as i32 as u32
        | 0x100000 as i32 as u32;
    s_playersettings.item_null.generic.x = 0 as i32;
    s_playersettings.item_null.generic.y = 0 as i32;
    s_playersettings.item_null.width = 640 as i32;
    s_playersettings.item_null.height = 480 as i32;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playersettings.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playersettings.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playersettings.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playersettings.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playersettings.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playersettings.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playersettings.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playersettings.name as *mut crate::ui_local_h::menufield_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playersettings.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playersettings.handicap as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playersettings.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playersettings.effects as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playersettings.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playersettings.model as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playersettings.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playersettings.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playersettings.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playersettings.player as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_playersettings.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_playersettings.item_null as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    PlayerSettings_SetMenuItems();
}
/*
=================
PlayerSettings_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn PlayerSettings_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/model_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/model_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char,
    );
    s_playersettings.fxBasePic = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/fx_base\x00" as *const u8 as *const libc::c_char,
    );
    s_playersettings.fxPic[0 as i32 as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/fx_red\x00" as *const u8 as *const libc::c_char,
        );
    s_playersettings.fxPic[1 as i32 as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/fx_yel\x00" as *const u8 as *const libc::c_char,
        );
    s_playersettings.fxPic[2 as i32 as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/fx_grn\x00" as *const u8 as *const libc::c_char,
        );
    s_playersettings.fxPic[3 as i32 as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/fx_teal\x00" as *const u8 as *const libc::c_char,
        );
    s_playersettings.fxPic[4 as i32 as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/fx_blue\x00" as *const u8 as *const libc::c_char,
        );
    s_playersettings.fxPic[5 as i32 as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/fx_cyan\x00" as *const u8 as *const libc::c_char,
        );
    s_playersettings.fxPic[6 as i32 as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/fx_white\x00" as *const u8 as *const libc::c_char,
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
//
// ui_playersettings.c
//
/*
=================
UI_PlayerSettingsMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_PlayerSettingsMenu() {
    PlayerSettings_MenuInit();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut s_playersettings.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}

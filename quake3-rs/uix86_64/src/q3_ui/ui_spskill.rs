use ::libc;

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
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_DefaultKey;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem;
pub use crate::src::q3_ui::ui_sparena::UI_SPArena_Start;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_S_RegisterSound;
pub use crate::src::ui::ui_syscalls::trap_S_StartLocalSound;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skillMenuInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub art_frame: crate::ui_local_h::menubitmap_s,
    pub art_banner: crate::ui_local_h::menutext_s,
    pub item_baby: crate::ui_local_h::menutext_s,
    pub item_easy: crate::ui_local_h::menutext_s,
    pub item_medium: crate::ui_local_h::menutext_s,
    pub item_hard: crate::ui_local_h::menutext_s,
    pub item_nightmare: crate::ui_local_h::menutext_s,
    pub art_skillPic: crate::ui_local_h::menubitmap_s,
    pub item_back: crate::ui_local_h::menubitmap_s,
    pub item_fight: crate::ui_local_h::menubitmap_s,
    pub arenaInfo: *const libc::c_char,
    pub skillpics: [crate::src::qcommon::q_shared::qhandle_t; 5],
    pub nightmareSound: crate::src::qcommon::q_shared::sfxHandle_t,
    pub silenceSound: crate::src::qcommon::q_shared::sfxHandle_t,
}

static mut skillMenuInfo: skillMenuInfo_t = skillMenuInfo_t {
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
    art_frame: crate::ui_local_h::menubitmap_s {
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
    art_banner: crate::ui_local_h::menutext_s {
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
    item_baby: crate::ui_local_h::menutext_s {
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
    item_easy: crate::ui_local_h::menutext_s {
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
    item_medium: crate::ui_local_h::menutext_s {
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
    item_hard: crate::ui_local_h::menutext_s {
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
    item_nightmare: crate::ui_local_h::menutext_s {
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
    art_skillPic: crate::ui_local_h::menubitmap_s {
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
    item_back: crate::ui_local_h::menubitmap_s {
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
    item_fight: crate::ui_local_h::menubitmap_s {
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
    arenaInfo: 0 as *const libc::c_char,
    skillpics: [0; 5],
    nightmareSound: 0,
    silenceSound: 0,
};

unsafe extern "C" fn SetSkillColor(
    mut skill: libc::c_int,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    match skill {
        1 => skillMenuInfo.item_baby.color = color,
        2 => skillMenuInfo.item_easy.color = color,
        3 => skillMenuInfo.item_medium.color = color,
        4 => skillMenuInfo.item_hard.color = color,
        5 => skillMenuInfo.item_nightmare.color = color,
        _ => {}
    };
}
/*
=================
UI_SPSkillMenu_SkillEvent
=================
*/

unsafe extern "C" fn UI_SPSkillMenu_SkillEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    let mut id: libc::c_int = 0;
    let mut skill: libc::c_int = 0;
    if notification != 3 as libc::c_int {
        return;
    }
    SetSkillColor(
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"g_spSkill\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_int,
        crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr(),
    );
    id = (*(ptr as *mut crate::ui_local_h::menucommon_s)).id;
    skill = id - 10 as libc::c_int + 1 as libc::c_int;
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"g_spSkill\x00" as *const u8 as *const libc::c_char,
        skill as libc::c_float,
    );
    SetSkillColor(skill, crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr());
    skillMenuInfo.art_skillPic.shader =
        skillMenuInfo.skillpics[(skill - 1 as libc::c_int) as usize];
    if id == 14 as libc::c_int {
        crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
            skillMenuInfo.nightmareSound,
            crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
        );
    } else {
        crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
            skillMenuInfo.silenceSound,
            crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
        );
    };
}
/*
=================
UI_SPSkillMenu_FightEvent
=================
*/

unsafe extern "C" fn UI_SPSkillMenu_FightEvent(
    mut _ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3 as libc::c_int {
        return;
    }
    crate::src::q3_ui::ui_sparena::UI_SPArena_Start(skillMenuInfo.arenaInfo);
}
/*
=================
UI_SPSkillMenu_BackEvent
=================
*/

unsafe extern "C" fn UI_SPSkillMenu_BackEvent(
    mut _ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3 as libc::c_int {
        return;
    }
    crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
        skillMenuInfo.silenceSound,
        crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
    );
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
}
/*
=================
UI_SPSkillMenu_Key
=================
*/

unsafe extern "C" fn UI_SPSkillMenu_Key(
    mut key: libc::c_int,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    if key == crate::keycodes_h::K_MOUSE2 as libc::c_int
        || key == crate::keycodes_h::K_ESCAPE as libc::c_int
    {
        crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
            skillMenuInfo.silenceSound,
            crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
        );
    }
    return crate::src::q3_ui::ui_qmenu::Menu_DefaultKey(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        key,
    );
}
/*
=================
UI_SPSkillMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPSkillMenu_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0.tga\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1.tga\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char,
    );
    skillMenuInfo.skillpics[0 as libc::c_int as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/level_complete1\x00" as *const u8 as *const libc::c_char,
        );
    skillMenuInfo.skillpics[1 as libc::c_int as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/level_complete2\x00" as *const u8 as *const libc::c_char,
        );
    skillMenuInfo.skillpics[2 as libc::c_int as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/level_complete3\x00" as *const u8 as *const libc::c_char,
        );
    skillMenuInfo.skillpics[3 as libc::c_int as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/level_complete4\x00" as *const u8 as *const libc::c_char,
        );
    skillMenuInfo.skillpics[4 as libc::c_int as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/level_complete5\x00" as *const u8 as *const libc::c_char,
        );
    skillMenuInfo.nightmareSound = crate::src::ui::ui_syscalls::trap_S_RegisterSound(
        b"sound/misc/nightmare.wav\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::qfalse,
    );
    skillMenuInfo.silenceSound = crate::src::ui::ui_syscalls::trap_S_RegisterSound(
        b"sound/misc/silence.wav\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::qfalse,
    );
}
/*
=================
UI_SPSkillMenu_Init
=================
*/

unsafe extern "C" fn UI_SPSkillMenu_Init() {
    let mut skill: libc::c_int = 0;
    crate::stdlib::memset(
        &mut skillMenuInfo as *mut skillMenuInfo_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<skillMenuInfo_t>() as libc::c_ulong,
    );
    skillMenuInfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    skillMenuInfo.menu.key = Some(
        UI_SPSkillMenu_Key
            as unsafe extern "C" fn(_: libc::c_int) -> crate::src::qcommon::q_shared::sfxHandle_t,
    );
    UI_SPSkillMenu_Cache();
    skillMenuInfo.art_frame.generic.type_0 = 6 as libc::c_int;
    skillMenuInfo.art_frame.generic.name =
        b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char;
    skillMenuInfo.art_frame.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x4000 as libc::c_int as libc::c_uint;
    skillMenuInfo.art_frame.generic.x = 142 as libc::c_int;
    skillMenuInfo.art_frame.generic.y = 118 as libc::c_int;
    skillMenuInfo.art_frame.width = 359 as libc::c_int;
    skillMenuInfo.art_frame.height = 256 as libc::c_int;
    skillMenuInfo.art_banner.generic.type_0 = 10 as libc::c_int;
    skillMenuInfo.art_banner.generic.flags = 0x8 as libc::c_int as libc::c_uint;
    skillMenuInfo.art_banner.generic.x = 320 as libc::c_int;
    skillMenuInfo.art_banner.generic.y = 16 as libc::c_int;
    skillMenuInfo.art_banner.string =
        b"DIFFICULTY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.art_banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    skillMenuInfo.art_banner.style = 0x1 as libc::c_int;
    skillMenuInfo.item_baby.generic.type_0 = 9 as libc::c_int;
    skillMenuInfo.item_baby.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    skillMenuInfo.item_baby.generic.x = 320 as libc::c_int;
    skillMenuInfo.item_baby.generic.y = 170 as libc::c_int;
    skillMenuInfo.item_baby.generic.callback = Some(
        UI_SPSkillMenu_SkillEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    skillMenuInfo.item_baby.generic.id = 10 as libc::c_int;
    skillMenuInfo.item_baby.string =
        b"I Can Win\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.item_baby.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    skillMenuInfo.item_baby.style = 0x1 as libc::c_int;
    skillMenuInfo.item_easy.generic.type_0 = 9 as libc::c_int;
    skillMenuInfo.item_easy.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    skillMenuInfo.item_easy.generic.x = 320 as libc::c_int;
    skillMenuInfo.item_easy.generic.y = 198 as libc::c_int;
    skillMenuInfo.item_easy.generic.callback = Some(
        UI_SPSkillMenu_SkillEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    skillMenuInfo.item_easy.generic.id = 11 as libc::c_int;
    skillMenuInfo.item_easy.string =
        b"Bring It On\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.item_easy.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    skillMenuInfo.item_easy.style = 0x1 as libc::c_int;
    skillMenuInfo.item_medium.generic.type_0 = 9 as libc::c_int;
    skillMenuInfo.item_medium.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    skillMenuInfo.item_medium.generic.x = 320 as libc::c_int;
    skillMenuInfo.item_medium.generic.y = 227 as libc::c_int;
    skillMenuInfo.item_medium.generic.callback = Some(
        UI_SPSkillMenu_SkillEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    skillMenuInfo.item_medium.generic.id = 12 as libc::c_int;
    skillMenuInfo.item_medium.string =
        b"Hurt Me Plenty\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.item_medium.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    skillMenuInfo.item_medium.style = 0x1 as libc::c_int;
    skillMenuInfo.item_hard.generic.type_0 = 9 as libc::c_int;
    skillMenuInfo.item_hard.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    skillMenuInfo.item_hard.generic.x = 320 as libc::c_int;
    skillMenuInfo.item_hard.generic.y = 255 as libc::c_int;
    skillMenuInfo.item_hard.generic.callback = Some(
        UI_SPSkillMenu_SkillEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    skillMenuInfo.item_hard.generic.id = 13 as libc::c_int;
    skillMenuInfo.item_hard.string =
        b"Hardcore\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.item_hard.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    skillMenuInfo.item_hard.style = 0x1 as libc::c_int;
    skillMenuInfo.item_nightmare.generic.type_0 = 9 as libc::c_int;
    skillMenuInfo.item_nightmare.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    skillMenuInfo.item_nightmare.generic.x = 320 as libc::c_int;
    skillMenuInfo.item_nightmare.generic.y = 283 as libc::c_int;
    skillMenuInfo.item_nightmare.generic.callback = Some(
        UI_SPSkillMenu_SkillEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    skillMenuInfo.item_nightmare.generic.id = 14 as libc::c_int;
    skillMenuInfo.item_nightmare.string =
        b"NIGHTMARE!\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.item_nightmare.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    skillMenuInfo.item_nightmare.style = 0x1 as libc::c_int;
    skillMenuInfo.item_back.generic.type_0 = 6 as libc::c_int;
    skillMenuInfo.item_back.generic.name =
        b"menu/art/back_0.tga\x00" as *const u8 as *const libc::c_char;
    skillMenuInfo.item_back.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    skillMenuInfo.item_back.generic.x = 0 as libc::c_int;
    skillMenuInfo.item_back.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    skillMenuInfo.item_back.generic.callback = Some(
        UI_SPSkillMenu_BackEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    skillMenuInfo.item_back.generic.id = 15 as libc::c_int;
    skillMenuInfo.item_back.width = 128 as libc::c_int;
    skillMenuInfo.item_back.height = 64 as libc::c_int;
    skillMenuInfo.item_back.focuspic =
        b"menu/art/back_1.tga\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.art_skillPic.generic.type_0 = 6 as libc::c_int;
    skillMenuInfo.art_skillPic.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x4000 as libc::c_int as libc::c_uint;
    skillMenuInfo.art_skillPic.generic.x = 320 as libc::c_int - 64 as libc::c_int;
    skillMenuInfo.art_skillPic.generic.y = 368 as libc::c_int;
    skillMenuInfo.art_skillPic.width = 128 as libc::c_int;
    skillMenuInfo.art_skillPic.height = 96 as libc::c_int;
    skillMenuInfo.item_fight.generic.type_0 = 6 as libc::c_int;
    skillMenuInfo.item_fight.generic.name =
        b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char;
    skillMenuInfo.item_fight.generic.flags =
        0x10 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    skillMenuInfo.item_fight.generic.callback = Some(
        UI_SPSkillMenu_FightEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    skillMenuInfo.item_fight.generic.id = 16 as libc::c_int;
    skillMenuInfo.item_fight.generic.x = 640 as libc::c_int;
    skillMenuInfo.item_fight.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    skillMenuInfo.item_fight.width = 128 as libc::c_int;
    skillMenuInfo.item_fight.height = 64 as libc::c_int;
    skillMenuInfo.item_fight.focuspic =
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut skillMenuInfo.art_frame as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut skillMenuInfo.art_banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut skillMenuInfo.item_baby as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut skillMenuInfo.item_easy as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut skillMenuInfo.item_medium as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut skillMenuInfo.item_hard as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut skillMenuInfo.item_nightmare as *mut crate::ui_local_h::menutext_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut skillMenuInfo.art_skillPic as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut skillMenuInfo.item_back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut skillMenuInfo.item_fight as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    skill = crate::src::qcommon::q_shared::Com_Clamp(
        1 as libc::c_int as libc::c_float,
        5 as libc::c_int as libc::c_float,
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"g_spSkill\x00" as *const u8 as *const libc::c_char,
        ),
    ) as libc::c_int;
    SetSkillColor(skill, crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr());
    skillMenuInfo.art_skillPic.shader =
        skillMenuInfo.skillpics[(skill - 1 as libc::c_int) as usize];
    if skill == 5 as libc::c_int {
        crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
            skillMenuInfo.nightmareSound,
            crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
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
//
// ui_preferences.c
//
//
// ui_specifyleague.c
//
//
// ui_specifyserver.c
//
//
// ui_servers2.c
//
//
// ui_startserver.c
//
//
// ui_serverinfo.c
//
//
// ui_video.c
//
//
// ui_players.c
//
//FIXME ripped from cg_local.h
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// model info
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// currently in use drawing parms
// animation vars
//
// ui_atoms.c
//
//
// ui_spLevel.c
//
//
// ui_spArena.c
//
//
// ui_spPostgame.c
//
//
// ui_spSkill.c
//
#[no_mangle]

pub unsafe extern "C" fn UI_SPSkillMenu(mut arenaInfo: *const libc::c_char) {
    UI_SPSkillMenu_Init();
    skillMenuInfo.arenaInfo = arenaInfo;
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
    crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem(
        &mut skillMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut skillMenuInfo.item_fight as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
}

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
pub use crate::src::q3_ui::ui_atoms::UI_ForceMenuOff;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::ScrollList_Key;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_FS_GetFileList;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menutext_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct demos_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub list: crate::ui_local_h::menulist_s,
    pub arrows: crate::ui_local_h::menubitmap_s,
    pub left: crate::ui_local_h::menubitmap_s,
    pub right: crate::ui_local_h::menubitmap_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub go: crate::ui_local_h::menubitmap_s,
    pub numDemos: libc::c_int,
    pub names: [libc::c_char; 32768],
    pub demolist: [*mut libc::c_char; 1024],
}

static mut s_demos: demos_t = demos_t {
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
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
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
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
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
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
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
    numDemos: 0,
    names: [0; 32768],
    demolist: [0 as *const libc::c_char as *mut libc::c_char; 1024],
};
/*
===============
Demos_MenuEvent
===============
*/

unsafe extern "C" fn Demos_MenuEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3 as libc::c_int {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        11 => {
            crate::src::q3_ui::ui_atoms::UI_ForceMenuOff();
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
                crate::src::qcommon::q_shared::va(
                    b"demo %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *s_demos
                        .list
                        .itemnames
                        .offset(s_demos.list.curvalue as isize),
                ),
            );
        }
        10 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        14 => {
            crate::src::q3_ui::ui_qmenu::ScrollList_Key(
                &mut s_demos.list as *mut _ as *mut crate::ui_local_h::menulist_s,
                crate::keycodes_h::K_LEFTARROW as libc::c_int,
            );
        }
        13 => {
            crate::src::q3_ui::ui_qmenu::ScrollList_Key(
                &mut s_demos.list as *mut _ as *mut crate::ui_local_h::menulist_s,
                crate::keycodes_h::K_RIGHTARROW as libc::c_int,
            );
        }
        _ => {}
    };
}
/*
===============
Demos_MenuInit
===============
*/

unsafe extern "C" fn Demos_MenuInit() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut demoname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut extension: [libc::c_char; 32] = [0; 32];
    let mut protocol: libc::c_int = 0;
    let mut protocolLegacy: libc::c_int = 0;
    crate::stdlib::memset(
        &mut s_demos as *mut demos_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<demos_t>() as libc::c_ulong,
    );
    Demos_Cache();
    s_demos.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_demos.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_demos.banner.generic.type_0 = 10 as libc::c_int;
    s_demos.banner.generic.x = 320 as libc::c_int;
    s_demos.banner.generic.y = 16 as libc::c_int;
    s_demos.banner.string = b"DEMOS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_demos.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    s_demos.banner.style = 0x1 as libc::c_int;
    s_demos.framel.generic.type_0 = 6 as libc::c_int;
    s_demos.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_demos.framel.generic.flags = 0x4000 as libc::c_int as libc::c_uint;
    s_demos.framel.generic.x = 0 as libc::c_int;
    s_demos.framel.generic.y = 78 as libc::c_int;
    s_demos.framel.width = 256 as libc::c_int;
    s_demos.framel.height = 329 as libc::c_int;
    s_demos.framer.generic.type_0 = 6 as libc::c_int;
    s_demos.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_demos.framer.generic.flags = 0x4000 as libc::c_int as libc::c_uint;
    s_demos.framer.generic.x = 376 as libc::c_int;
    s_demos.framer.generic.y = 76 as libc::c_int;
    s_demos.framer.width = 256 as libc::c_int;
    s_demos.framer.height = 334 as libc::c_int;
    s_demos.arrows.generic.type_0 = 6 as libc::c_int;
    s_demos.arrows.generic.name = b"menu/art/arrows_horz_0\x00" as *const u8 as *const libc::c_char;
    s_demos.arrows.generic.flags = 0x4000 as libc::c_int as libc::c_uint;
    s_demos.arrows.generic.x = 320 as libc::c_int - 128 as libc::c_int / 2 as libc::c_int;
    s_demos.arrows.generic.y = 400 as libc::c_int;
    s_demos.arrows.width = 128 as libc::c_int;
    s_demos.arrows.height = 48 as libc::c_int;
    s_demos.left.generic.type_0 = 6 as libc::c_int;
    s_demos.left.generic.flags = 0x4 as libc::c_int as libc::c_uint
        | 0x100 as libc::c_int as libc::c_uint
        | 0x800 as libc::c_int as libc::c_uint;
    s_demos.left.generic.x = 320 as libc::c_int - 128 as libc::c_int / 2 as libc::c_int;
    s_demos.left.generic.y = 400 as libc::c_int;
    s_demos.left.generic.id = 14 as libc::c_int;
    s_demos.left.generic.callback =
        Some(Demos_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_demos.left.width = 128 as libc::c_int / 2 as libc::c_int;
    s_demos.left.height = 48 as libc::c_int;
    s_demos.left.focuspic =
        b"menu/art/arrows_horz_left\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_demos.right.generic.type_0 = 6 as libc::c_int;
    s_demos.right.generic.flags = 0x4 as libc::c_int as libc::c_uint
        | 0x100 as libc::c_int as libc::c_uint
        | 0x800 as libc::c_int as libc::c_uint;
    s_demos.right.generic.x = 320 as libc::c_int;
    s_demos.right.generic.y = 400 as libc::c_int;
    s_demos.right.generic.id = 13 as libc::c_int;
    s_demos.right.generic.callback =
        Some(Demos_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_demos.right.width = 128 as libc::c_int / 2 as libc::c_int;
    s_demos.right.height = 48 as libc::c_int;
    s_demos.right.focuspic =
        b"menu/art/arrows_horz_right\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_demos.back.generic.type_0 = 6 as libc::c_int;
    s_demos.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_demos.back.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    s_demos.back.generic.id = 10 as libc::c_int;
    s_demos.back.generic.callback =
        Some(Demos_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_demos.back.generic.x = 0 as libc::c_int;
    s_demos.back.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    s_demos.back.width = 128 as libc::c_int;
    s_demos.back.height = 64 as libc::c_int;
    s_demos.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_demos.go.generic.type_0 = 6 as libc::c_int;
    s_demos.go.generic.name = b"menu/art/play_0\x00" as *const u8 as *const libc::c_char;
    s_demos.go.generic.flags =
        0x10 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    s_demos.go.generic.id = 11 as libc::c_int;
    s_demos.go.generic.callback =
        Some(Demos_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_demos.go.generic.x = 640 as libc::c_int;
    s_demos.go.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    s_demos.go.width = 128 as libc::c_int;
    s_demos.go.height = 64 as libc::c_int;
    s_demos.go.focuspic =
        b"menu/art/play_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_demos.list.generic.type_0 = 8 as libc::c_int;
    s_demos.list.generic.flags = 0x100 as libc::c_int as libc::c_uint;
    s_demos.list.generic.callback =
        Some(Demos_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ());
    s_demos.list.generic.id = 12 as libc::c_int;
    s_demos.list.generic.x = 118 as libc::c_int;
    s_demos.list.generic.y = 130 as libc::c_int;
    s_demos.list.width = 16 as libc::c_int;
    s_demos.list.height = 14 as libc::c_int;
    s_demos.list.itemnames = s_demos.demolist.as_mut_ptr() as *mut *const libc::c_char;
    s_demos.list.columns = 3 as libc::c_int;
    protocolLegacy = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"com_legacyprotocol\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    protocol = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"com_protocol\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if protocol == 0 {
        protocol = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"protocol\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_int
    }
    if protocolLegacy == protocol {
        protocolLegacy = 0 as libc::c_int
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        extension.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        b".%s%d\x00" as *const u8 as *const libc::c_char,
        b"dm_\x00" as *const u8 as *const libc::c_char,
        protocol,
    );
    s_demos.numDemos = crate::src::ui::ui_syscalls::trap_FS_GetFileList(
        b"demos\x00" as *const u8 as *const libc::c_char,
        extension.as_mut_ptr(),
        s_demos.names.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 32768]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int,
    );
    demoname = s_demos.names.as_mut_ptr();
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        if s_demos.numDemos > 1024 as libc::c_int {
            s_demos.numDemos = 1024 as libc::c_int
        }
        while i < s_demos.numDemos {
            let ref mut fresh0 = *s_demos.list.itemnames.offset(i as isize);
            *fresh0 = demoname;
            len = crate::stdlib::strlen(demoname) as libc::c_int;
            demoname = demoname.offset((len + 1 as libc::c_int) as isize);
            i += 1
        }
        if j == 0 {
            if !(protocolLegacy > 0 as libc::c_int && s_demos.numDemos < 1024 as libc::c_int) {
                break;
            }
            crate::src::qcommon::q_shared::Com_sprintf(
                extension.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
                b".%s%d\x00" as *const u8 as *const libc::c_char,
                b"dm_\x00" as *const u8 as *const libc::c_char,
                protocolLegacy,
            );
            s_demos.numDemos += crate::src::ui::ui_syscalls::trap_FS_GetFileList(
                b"demos\x00" as *const u8 as *const libc::c_char,
                extension.as_mut_ptr(),
                demoname,
                (::std::mem::size_of::<[libc::c_char; 32768]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(demoname.wrapping_offset_from(s_demos.names.as_mut_ptr())
                        as libc::c_long as libc::c_ulong) as libc::c_int,
            )
        }
        j += 1
    }
    s_demos.list.numitems = s_demos.numDemos;
    if s_demos.numDemos == 0 {
        let ref mut fresh1 = *s_demos.list.itemnames.offset(0 as libc::c_int as isize);
        *fresh1 = b"No Demos Found.\x00" as *const u8 as *const libc::c_char;
        s_demos.list.numitems = 1 as libc::c_int;
        //degenerate case, not selectable
        s_demos.go.generic.flags |=
            0x4000 as libc::c_int as libc::c_uint | 0x1000 as libc::c_int as libc::c_uint
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_demos.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_demos.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_demos.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_demos.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_demos.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_demos.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_demos.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_demos.list as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_demos.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_demos.arrows as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_demos.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_demos.left as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_demos.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_demos.right as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_demos.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_demos.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_demos.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_demos.go as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
Demos_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Demos_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/play_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/play_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/arrows_horz_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/arrows_horz_left\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/arrows_horz_right\x00" as *const u8 as *const libc::c_char,
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
/*
===============
UI_DemosMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DemosMenu() {
    Demos_MenuInit();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut s_demos.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}

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
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strupr;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
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
pub struct configs_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub list: menulist_s,
    pub arrows: menubitmap_s,
    pub left: menubitmap_s,
    pub right: menubitmap_s,
    pub back: menubitmap_s,
    pub go: menubitmap_s,
    pub names: [libc::c_char; 2048],
    pub configlist: [*mut libc::c_char; 128],
}

static mut s_configs: configs_t = configs_t {
    menu: menuframework_s {
        cursor: 0,
        cursor_prev: 0,
        nitems: 0,
        items: [std::ptr::null_mut(); 64],
        draw: None,
        key: None,
        wrapAround: qfalse,
        fullscreen: qfalse,
        showlogo: qfalse,
    },
    banner: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: std::ptr::null_mut(),
        style: 0,
        color: std::ptr::null_mut(),
    },
    framel: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    framer: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    list: menulist_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
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
        itemnames: std::ptr::null_mut(),
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    arrows: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    left: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    right: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    back: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    go: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: std::ptr::null(),
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: std::ptr::null_mut(),
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: std::ptr::null_mut(),
        errorpic: std::ptr::null_mut(),
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: std::ptr::null_mut(),
    },
    names: [0; 2048],
    configlist: [std::ptr::null_mut(); 128],
};
/*
===============
LoadConfig_MenuEvent
===============
*/

unsafe extern "C" fn LoadConfig_MenuEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        11 => {
            trap_Cmd_ExecuteText(
                EXEC_APPEND as i32,
                va(
                    b"exec %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *s_configs
                        .list
                        .itemnames
                        .offset(s_configs.list.curvalue as isize),
                ),
            );
            UI_PopMenu();
        }
        10 => {
            UI_PopMenu();
        }
        13 => {
            ScrollList_Key(
                &mut s_configs.list as *mut _ as *mut menulist_s,
                K_LEFTARROW as i32,
            );
        }
        14 => {
            ScrollList_Key(
                &mut s_configs.list as *mut _ as *mut menulist_s,
                K_RIGHTARROW as i32,
            );
        }
        _ => {}
    };
}
/*
===============
LoadConfig_MenuInit
===============
*/

unsafe extern "C" fn LoadConfig_MenuInit() {
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    let mut configname: *mut libc::c_char = std::ptr::null_mut();
    UI_LoadConfig_Cache();
    crate::stdlib::memset(
        &mut s_configs as *mut configs_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<configs_t>() as usize,
    );
    s_configs.menu.wrapAround = qtrue;
    s_configs.menu.fullscreen = qtrue;
    s_configs.banner.generic.type_0 = 10 as i32;
    s_configs.banner.generic.x = 320 as i32;
    s_configs.banner.generic.y = 16 as i32;
    s_configs.banner.string =
        b"LOAD CONFIG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_configs.banner.color = color_white.as_mut_ptr();
    s_configs.banner.style = 0x1 as i32;
    s_configs.framel.generic.type_0 = 6 as i32;
    s_configs.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_configs.framel.generic.flags = 0x4000 as i32 as u32;
    s_configs.framel.generic.x = 0 as i32;
    s_configs.framel.generic.y = 78 as i32;
    s_configs.framel.width = 256 as i32;
    s_configs.framel.height = 329 as i32;
    s_configs.framer.generic.type_0 = 6 as i32;
    s_configs.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_configs.framer.generic.flags = 0x4000 as i32 as u32;
    s_configs.framer.generic.x = 376 as i32;
    s_configs.framer.generic.y = 76 as i32;
    s_configs.framer.width = 256 as i32;
    s_configs.framer.height = 334 as i32;
    s_configs.arrows.generic.type_0 = 6 as i32;
    s_configs.arrows.generic.name =
        b"menu/art/arrows_horz_0\x00" as *const u8 as *const libc::c_char;
    s_configs.arrows.generic.flags = 0x4000 as i32 as u32;
    s_configs.arrows.generic.x = 320 as i32 - 128 as i32 / 2 as i32;
    s_configs.arrows.generic.y = 400 as i32;
    s_configs.arrows.width = 128 as i32;
    s_configs.arrows.height = 48 as i32;
    s_configs.left.generic.type_0 = 6 as i32;
    s_configs.left.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x800 as i32 as u32;
    s_configs.left.generic.x = 320 as i32 - 128 as i32 / 2 as i32;
    s_configs.left.generic.y = 400 as i32;
    s_configs.left.generic.id = 13 as i32;
    s_configs.left.generic.callback =
        Some(LoadConfig_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_configs.left.width = 128 as i32 / 2 as i32;
    s_configs.left.height = 48 as i32;
    s_configs.left.focuspic =
        b"menu/art/arrows_horz_left\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_configs.right.generic.type_0 = 6 as i32;
    s_configs.right.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32 | 0x800 as i32 as u32;
    s_configs.right.generic.x = 320 as i32;
    s_configs.right.generic.y = 400 as i32;
    s_configs.right.generic.id = 14 as i32;
    s_configs.right.generic.callback =
        Some(LoadConfig_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_configs.right.width = 128 as i32 / 2 as i32;
    s_configs.right.height = 48 as i32;
    s_configs.right.focuspic =
        b"menu/art/arrows_horz_right\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_configs.back.generic.type_0 = 6 as i32;
    s_configs.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_configs.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_configs.back.generic.id = 10 as i32;
    s_configs.back.generic.callback =
        Some(LoadConfig_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_configs.back.generic.x = 0 as i32;
    s_configs.back.generic.y = 480 as i32 - 64 as i32;
    s_configs.back.width = 128 as i32;
    s_configs.back.height = 64 as i32;
    s_configs.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_configs.go.generic.type_0 = 6 as i32;
    s_configs.go.generic.name = b"menu/art/load_0\x00" as *const u8 as *const libc::c_char;
    s_configs.go.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_configs.go.generic.id = 11 as i32;
    s_configs.go.generic.callback =
        Some(LoadConfig_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_configs.go.generic.x = 640 as i32;
    s_configs.go.generic.y = 480 as i32 - 64 as i32;
    s_configs.go.width = 128 as i32;
    s_configs.go.height = 64 as i32;
    s_configs.go.focuspic =
        b"menu/art/load_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    // scan for configs
    s_configs.list.generic.type_0 = 8 as i32;
    s_configs.list.generic.flags = 0x100 as i32 as u32;
    s_configs.list.generic.callback =
        Some(LoadConfig_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_configs.list.generic.id = 12 as i32;
    s_configs.list.generic.x = 118 as i32;
    s_configs.list.generic.y = 130 as i32;
    s_configs.list.width = 16 as i32;
    s_configs.list.height = 14 as i32;
    s_configs.list.numitems = trap_FS_GetFileList(
        b"\x00" as *const u8 as *const libc::c_char,
        b"cfg\x00" as *const u8 as *const libc::c_char,
        s_configs.names.as_mut_ptr(),
        128 as i32 * 16 as i32,
    );
    s_configs.list.itemnames = s_configs.configlist.as_mut_ptr() as *mut *const libc::c_char;
    s_configs.list.columns = 3 as i32;
    if s_configs.list.numitems == 0 {
        libc::strcpy(
            s_configs.names.as_mut_ptr(),
            b"No Files Found.\x00" as *const u8 as *const libc::c_char,
        );
        s_configs.list.numitems = 1 as i32;
        //degenerate case, not selectable
        s_configs.go.generic.flags |= 0x4000 as i32 as u32 | 0x1000 as i32 as u32
    } else if s_configs.list.numitems > 128 as i32 {
        s_configs.list.numitems = 128 as i32
    }
    configname = s_configs.names.as_mut_ptr();
    i = 0 as i32;
    while i < s_configs.list.numitems {
        let ref mut fresh0 = *s_configs.list.itemnames.offset(i as isize);
        *fresh0 = configname;
        // strip extension
        len = crate::stdlib::strlen(configname) as i32;
        if Q_stricmp(
            configname.offset(len as isize).offset(-(4 as i32 as isize)),
            b".cfg\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            *configname.offset((len - 4 as i32) as isize) = '\u{0}' as i32 as libc::c_char
        }
        Q_strupr(configname);
        configname = configname.offset((len + 1 as i32) as isize);
        i += 1
    }
    Menu_AddItem(
        &mut s_configs.menu as *mut _ as *mut _tag_menuframework,
        &mut s_configs.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu as *mut _ as *mut _tag_menuframework,
        &mut s_configs.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu as *mut _ as *mut _tag_menuframework,
        &mut s_configs.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu as *mut _ as *mut _tag_menuframework,
        &mut s_configs.list as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu as *mut _ as *mut _tag_menuframework,
        &mut s_configs.arrows as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu as *mut _ as *mut _tag_menuframework,
        &mut s_configs.left as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu as *mut _ as *mut _tag_menuframework,
        &mut s_configs.right as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu as *mut _ as *mut _tag_menuframework,
        &mut s_configs.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu as *mut _ as *mut _tag_menuframework,
        &mut s_configs.go as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
UI_LoadConfig_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_LoadConfig_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/load_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/load_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/arrows_horz_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(
        b"menu/art/arrows_horz_left\x00" as *const u8 as *const libc::c_char,
    );
    trap_R_RegisterShaderNoMip(
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
//
// ui_syscalls.c
//
// don't use EXEC_NOW!
// fsOrigin_t
//
// ui_addbots.c
//
//
// ui_removebots.c
//
//
// ui_teamorders.c
//
//
// ui_loadconfig.c
//
/*
===============
UI_LoadConfigMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_LoadConfigMenu() {
    LoadConfig_MenuInit();
    UI_PushMenu(&mut s_configs.menu as *mut _ as *mut _tag_menuframework);
}

use ::libc;

pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_cdkey::UI_CDKeyMenu;
pub use crate::src::q3_ui::ui_confirm::UI_ConfirmMenu;
pub use crate::src::q3_ui::ui_controls2::UI_ControlsMenu;
pub use crate::src::q3_ui::ui_playersettings::UI_PlayerSettingsMenu;
pub use crate::src::q3_ui::ui_preferences::UI_PreferencesMenu;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::color_yellow;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_video::UI_GraphicsOptionsMenu;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct setupMenuInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub setupplayer: crate::ui_local_h::menutext_s,
    pub setupcontrols: crate::ui_local_h::menutext_s,
    pub setupsystem: crate::ui_local_h::menutext_s,
    pub game: crate::ui_local_h::menutext_s,
    pub cdkey: crate::ui_local_h::menutext_s,
    pub defaults: crate::ui_local_h::menutext_s,
    pub back: crate::ui_local_h::menubitmap_s,
}

static mut setupMenuInfo: setupMenuInfo_t = setupMenuInfo_t {
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
    setupplayer: crate::ui_local_h::menutext_s {
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
    setupcontrols: crate::ui_local_h::menutext_s {
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
    setupsystem: crate::ui_local_h::menutext_s {
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
    game: crate::ui_local_h::menutext_s {
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
    cdkey: crate::ui_local_h::menutext_s {
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
    defaults: crate::ui_local_h::menutext_s {
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
};
/*
=================
Setup_ResetDefaults_Action
=================
*/

unsafe extern "C" fn Setup_ResetDefaults_Action(
    mut result: crate::src::qcommon::q_shared::qboolean,
) {
    if result as u64 == 0 {
        return;
    }
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
        b"exec default.cfg\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
        b"cvar_restart\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
        b"vid_restart\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
Setup_ResetDefaults_Draw
=================
*/

unsafe extern "C" fn Setup_ResetDefaults_Draw() {
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        640 as libc::c_int / 2 as libc::c_int,
        356 as libc::c_int + 27 as libc::c_int * 0 as libc::c_int,
        b"WARNING: This will reset *ALL*\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x10 as libc::c_int,
        crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        640 as libc::c_int / 2 as libc::c_int,
        356 as libc::c_int + 27 as libc::c_int * 1 as libc::c_int,
        b"options to their default values.\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x10 as libc::c_int,
        crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
    );
}
/*
===============
UI_SetupMenu_Event
===============
*/

unsafe extern "C" fn UI_SetupMenu_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3 as libc::c_int {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        10 => {
            crate::src::q3_ui::ui_playersettings::UI_PlayerSettingsMenu();
        }
        11 => {
            crate::src::q3_ui::ui_controls2::UI_ControlsMenu();
        }
        12 => {
            crate::src::q3_ui::ui_video::UI_GraphicsOptionsMenu();
        }
        13 => {
            crate::src::q3_ui::ui_preferences::UI_PreferencesMenu();
        }
        14 => {
            crate::src::q3_ui::ui_cdkey::UI_CDKeyMenu();
        }
        17 => {
            //	case ID_LOAD:
            //		UI_LoadConfigMenu();
            //		break;
            //	case ID_SAVE:
            //		UI_SaveConfigMenu();
            //		break;
            crate::src::q3_ui::ui_confirm::UI_ConfirmMenu(
                b"SET TO DEFAULTS?\x00" as *const u8 as *const libc::c_char,
                Some(Setup_ResetDefaults_Draw as unsafe extern "C" fn() -> ()),
                Some(
                    Setup_ResetDefaults_Action
                        as unsafe extern "C" fn(_: crate::src::qcommon::q_shared::qboolean) -> (),
                ),
            );
        }
        18 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        _ => {}
    };
}
/*
===============
UI_SetupMenu_Init
===============
*/

unsafe extern "C" fn UI_SetupMenu_Init() {
    let mut y: libc::c_int = 0;
    UI_SetupMenu_Cache();
    crate::stdlib::memset(
        &mut setupMenuInfo as *mut setupMenuInfo_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<setupMenuInfo_t>() as libc::c_ulong,
    );
    setupMenuInfo.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    setupMenuInfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    setupMenuInfo.banner.generic.type_0 = 10 as libc::c_int;
    setupMenuInfo.banner.generic.x = 320 as libc::c_int;
    setupMenuInfo.banner.generic.y = 16 as libc::c_int;
    setupMenuInfo.banner.string =
        b"SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    setupMenuInfo.banner.style = 0x1 as libc::c_int;
    setupMenuInfo.framel.generic.type_0 = 6 as libc::c_int;
    setupMenuInfo.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    setupMenuInfo.framel.generic.flags = 0x4000 as libc::c_int as libc::c_uint;
    setupMenuInfo.framel.generic.x = 0 as libc::c_int;
    setupMenuInfo.framel.generic.y = 78 as libc::c_int;
    setupMenuInfo.framel.width = 256 as libc::c_int;
    setupMenuInfo.framel.height = 329 as libc::c_int;
    setupMenuInfo.framer.generic.type_0 = 6 as libc::c_int;
    setupMenuInfo.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    setupMenuInfo.framer.generic.flags = 0x4000 as libc::c_int as libc::c_uint;
    setupMenuInfo.framer.generic.x = 376 as libc::c_int;
    setupMenuInfo.framer.generic.y = 76 as libc::c_int;
    setupMenuInfo.framer.width = 256 as libc::c_int;
    setupMenuInfo.framer.height = 334 as libc::c_int;
    y = 134 as libc::c_int;
    setupMenuInfo.setupplayer.generic.type_0 = 9 as libc::c_int;
    setupMenuInfo.setupplayer.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    setupMenuInfo.setupplayer.generic.x = 320 as libc::c_int;
    setupMenuInfo.setupplayer.generic.y = y;
    setupMenuInfo.setupplayer.generic.id = 10 as libc::c_int;
    setupMenuInfo.setupplayer.generic.callback = Some(
        UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    setupMenuInfo.setupplayer.string =
        b"PLAYER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.setupplayer.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    setupMenuInfo.setupplayer.style = 0x1 as libc::c_int;
    y += 34 as libc::c_int;
    setupMenuInfo.setupcontrols.generic.type_0 = 9 as libc::c_int;
    setupMenuInfo.setupcontrols.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    setupMenuInfo.setupcontrols.generic.x = 320 as libc::c_int;
    setupMenuInfo.setupcontrols.generic.y = y;
    setupMenuInfo.setupcontrols.generic.id = 11 as libc::c_int;
    setupMenuInfo.setupcontrols.generic.callback = Some(
        UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    setupMenuInfo.setupcontrols.string =
        b"CONTROLS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.setupcontrols.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    setupMenuInfo.setupcontrols.style = 0x1 as libc::c_int;
    y += 34 as libc::c_int;
    setupMenuInfo.setupsystem.generic.type_0 = 9 as libc::c_int;
    setupMenuInfo.setupsystem.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    setupMenuInfo.setupsystem.generic.x = 320 as libc::c_int;
    setupMenuInfo.setupsystem.generic.y = y;
    setupMenuInfo.setupsystem.generic.id = 12 as libc::c_int;
    setupMenuInfo.setupsystem.generic.callback = Some(
        UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    setupMenuInfo.setupsystem.string =
        b"SYSTEM\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.setupsystem.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    setupMenuInfo.setupsystem.style = 0x1 as libc::c_int;
    y += 34 as libc::c_int;
    setupMenuInfo.game.generic.type_0 = 9 as libc::c_int;
    setupMenuInfo.game.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    setupMenuInfo.game.generic.x = 320 as libc::c_int;
    setupMenuInfo.game.generic.y = y;
    setupMenuInfo.game.generic.id = 13 as libc::c_int;
    setupMenuInfo.game.generic.callback = Some(
        UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    setupMenuInfo.game.string =
        b"GAME OPTIONS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.game.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    setupMenuInfo.game.style = 0x1 as libc::c_int;
    y += 34 as libc::c_int;
    setupMenuInfo.cdkey.generic.type_0 = 9 as libc::c_int;
    setupMenuInfo.cdkey.generic.flags =
        0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    setupMenuInfo.cdkey.generic.x = 320 as libc::c_int;
    setupMenuInfo.cdkey.generic.y = y;
    setupMenuInfo.cdkey.generic.id = 14 as libc::c_int;
    setupMenuInfo.cdkey.generic.callback = Some(
        UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    setupMenuInfo.cdkey.string =
        b"CD Key\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.cdkey.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    setupMenuInfo.cdkey.style = 0x1 as libc::c_int;
    if crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cl_paused\x00" as *const u8 as *const libc::c_char,
    ) == 0.
    {
        y += 34 as libc::c_int;
        setupMenuInfo.defaults.generic.type_0 = 9 as libc::c_int;
        setupMenuInfo.defaults.generic.flags =
            0x8 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
        setupMenuInfo.defaults.generic.x = 320 as libc::c_int;
        setupMenuInfo.defaults.generic.y = y;
        setupMenuInfo.defaults.generic.id = 17 as libc::c_int;
        setupMenuInfo.defaults.generic.callback = Some(
            UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
        );
        setupMenuInfo.defaults.string =
            b"DEFAULTS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        setupMenuInfo.defaults.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
        setupMenuInfo.defaults.style = 0x1 as libc::c_int
    }
    setupMenuInfo.back.generic.type_0 = 6 as libc::c_int;
    setupMenuInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    setupMenuInfo.back.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    setupMenuInfo.back.generic.id = 18 as libc::c_int;
    setupMenuInfo.back.generic.callback = Some(
        UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    setupMenuInfo.back.generic.x = 0 as libc::c_int;
    setupMenuInfo.back.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    setupMenuInfo.back.width = 128 as libc::c_int;
    setupMenuInfo.back.height = 64 as libc::c_int;
    setupMenuInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut setupMenuInfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut setupMenuInfo.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut setupMenuInfo.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut setupMenuInfo.setupplayer as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut setupMenuInfo.setupcontrols as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut setupMenuInfo.setupsystem as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut setupMenuInfo.game as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut setupMenuInfo.cdkey as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    //	Menu_AddItem( &setupMenuInfo.menu, &setupMenuInfo.load );
    //	Menu_AddItem( &setupMenuInfo.menu, &setupMenuInfo.save );
    if crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cl_paused\x00" as *const u8 as *const libc::c_char,
    ) == 0.
    {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut setupMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
            &mut setupMenuInfo.defaults as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
        );
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut setupMenuInfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
UI_SetupMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SetupMenu_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char,
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
/*
===============
UI_SetupMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SetupMenu() {
    UI_SetupMenu_Init();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut setupMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}

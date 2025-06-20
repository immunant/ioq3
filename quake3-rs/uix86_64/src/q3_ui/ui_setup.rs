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
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub setupplayer: menutext_s,
    pub setupcontrols: menutext_s,
    pub setupsystem: menutext_s,
    pub game: menutext_s,
    pub cdkey: menutext_s,
    pub defaults: menutext_s,
    pub back: menubitmap_s,
}

static mut setupMenuInfo: setupMenuInfo_t = setupMenuInfo_t {
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
    setupplayer: menutext_s {
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
    setupcontrols: menutext_s {
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
    setupsystem: menutext_s {
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
    game: menutext_s {
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
    cdkey: menutext_s {
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
    defaults: menutext_s {
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
};
/*
=================
Setup_ResetDefaults_Action
=================
*/

unsafe extern "C" fn Setup_ResetDefaults_Action(mut result: qboolean) {
    if result as u64 == 0 {
        return;
    }
    trap_Cmd_ExecuteText(
        EXEC_APPEND as i32,
        b"exec default.cfg\n\x00" as *const u8 as *const libc::c_char,
    );
    trap_Cmd_ExecuteText(
        EXEC_APPEND as i32,
        b"cvar_restart\n\x00" as *const u8 as *const libc::c_char,
    );
    trap_Cmd_ExecuteText(
        EXEC_APPEND as i32,
        b"vid_restart\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
Setup_ResetDefaults_Draw
=================
*/

unsafe extern "C" fn Setup_ResetDefaults_Draw() {
    UI_DrawProportionalString(
        640 as i32 / 2 as i32,
        356 as i32 + 27 as i32 * 0 as i32,
        b"WARNING: This will reset *ALL*\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_yellow.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        640 as i32 / 2 as i32,
        356 as i32 + 27 as i32 * 1 as i32,
        b"options to their default values.\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_yellow.as_mut_ptr(),
    );
}
/*
===============
UI_SetupMenu_Event
===============
*/

unsafe extern "C" fn UI_SetupMenu_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        10 => {
            UI_PlayerSettingsMenu();
        }
        11 => {
            UI_ControlsMenu();
        }
        12 => {
            UI_GraphicsOptionsMenu();
        }
        13 => {
            UI_PreferencesMenu();
        }
        14 => {
            UI_CDKeyMenu();
        }
        17 => {
            //	case ID_LOAD:
            //		UI_LoadConfigMenu();
            //		break;
            //	case ID_SAVE:
            //		UI_SaveConfigMenu();
            //		break;
            UI_ConfirmMenu(
                b"SET TO DEFAULTS?\x00" as *const u8 as *const libc::c_char,
                Some(Setup_ResetDefaults_Draw as unsafe extern "C" fn() -> ()),
                Some(Setup_ResetDefaults_Action as unsafe extern "C" fn(_: qboolean) -> ()),
            );
        }
        18 => {
            UI_PopMenu();
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
    let mut y: i32 = 0;
    UI_SetupMenu_Cache();
    crate::stdlib::memset(
        &mut setupMenuInfo as *mut setupMenuInfo_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<setupMenuInfo_t>() as usize,
    );
    setupMenuInfo.menu.wrapAround = qtrue;
    setupMenuInfo.menu.fullscreen = qtrue;
    setupMenuInfo.banner.generic.type_0 = 10 as i32;
    setupMenuInfo.banner.generic.x = 320 as i32;
    setupMenuInfo.banner.generic.y = 16 as i32;
    setupMenuInfo.banner.string =
        b"SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.banner.color = color_white.as_mut_ptr();
    setupMenuInfo.banner.style = 0x1 as i32;
    setupMenuInfo.framel.generic.type_0 = 6 as i32;
    setupMenuInfo.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    setupMenuInfo.framel.generic.flags = 0x4000 as i32 as u32;
    setupMenuInfo.framel.generic.x = 0 as i32;
    setupMenuInfo.framel.generic.y = 78 as i32;
    setupMenuInfo.framel.width = 256 as i32;
    setupMenuInfo.framel.height = 329 as i32;
    setupMenuInfo.framer.generic.type_0 = 6 as i32;
    setupMenuInfo.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    setupMenuInfo.framer.generic.flags = 0x4000 as i32 as u32;
    setupMenuInfo.framer.generic.x = 376 as i32;
    setupMenuInfo.framer.generic.y = 76 as i32;
    setupMenuInfo.framer.width = 256 as i32;
    setupMenuInfo.framer.height = 334 as i32;
    y = 134 as i32;
    setupMenuInfo.setupplayer.generic.type_0 = 9 as i32;
    setupMenuInfo.setupplayer.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    setupMenuInfo.setupplayer.generic.x = 320 as i32;
    setupMenuInfo.setupplayer.generic.y = y;
    setupMenuInfo.setupplayer.generic.id = 10 as i32;
    setupMenuInfo.setupplayer.generic.callback =
        Some(UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    setupMenuInfo.setupplayer.string =
        b"PLAYER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.setupplayer.color = color_red.as_mut_ptr();
    setupMenuInfo.setupplayer.style = 0x1 as i32;
    y += 34 as i32;
    setupMenuInfo.setupcontrols.generic.type_0 = 9 as i32;
    setupMenuInfo.setupcontrols.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    setupMenuInfo.setupcontrols.generic.x = 320 as i32;
    setupMenuInfo.setupcontrols.generic.y = y;
    setupMenuInfo.setupcontrols.generic.id = 11 as i32;
    setupMenuInfo.setupcontrols.generic.callback =
        Some(UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    setupMenuInfo.setupcontrols.string =
        b"CONTROLS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.setupcontrols.color = color_red.as_mut_ptr();
    setupMenuInfo.setupcontrols.style = 0x1 as i32;
    y += 34 as i32;
    setupMenuInfo.setupsystem.generic.type_0 = 9 as i32;
    setupMenuInfo.setupsystem.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    setupMenuInfo.setupsystem.generic.x = 320 as i32;
    setupMenuInfo.setupsystem.generic.y = y;
    setupMenuInfo.setupsystem.generic.id = 12 as i32;
    setupMenuInfo.setupsystem.generic.callback =
        Some(UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    setupMenuInfo.setupsystem.string =
        b"SYSTEM\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.setupsystem.color = color_red.as_mut_ptr();
    setupMenuInfo.setupsystem.style = 0x1 as i32;
    y += 34 as i32;
    setupMenuInfo.game.generic.type_0 = 9 as i32;
    setupMenuInfo.game.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    setupMenuInfo.game.generic.x = 320 as i32;
    setupMenuInfo.game.generic.y = y;
    setupMenuInfo.game.generic.id = 13 as i32;
    setupMenuInfo.game.generic.callback =
        Some(UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    setupMenuInfo.game.string =
        b"GAME OPTIONS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.game.color = color_red.as_mut_ptr();
    setupMenuInfo.game.style = 0x1 as i32;
    y += 34 as i32;
    setupMenuInfo.cdkey.generic.type_0 = 9 as i32;
    setupMenuInfo.cdkey.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
    setupMenuInfo.cdkey.generic.x = 320 as i32;
    setupMenuInfo.cdkey.generic.y = y;
    setupMenuInfo.cdkey.generic.id = 14 as i32;
    setupMenuInfo.cdkey.generic.callback =
        Some(UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    setupMenuInfo.cdkey.string =
        b"CD Key\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.cdkey.color = color_red.as_mut_ptr();
    setupMenuInfo.cdkey.style = 0x1 as i32;
    if trap_Cvar_VariableValue(b"cl_paused\x00" as *const u8 as *const libc::c_char) == 0. {
        y += 34 as i32;
        setupMenuInfo.defaults.generic.type_0 = 9 as i32;
        setupMenuInfo.defaults.generic.flags = 0x8 as i32 as u32 | 0x100 as i32 as u32;
        setupMenuInfo.defaults.generic.x = 320 as i32;
        setupMenuInfo.defaults.generic.y = y;
        setupMenuInfo.defaults.generic.id = 17 as i32;
        setupMenuInfo.defaults.generic.callback =
            Some(UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
        setupMenuInfo.defaults.string =
            b"DEFAULTS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        setupMenuInfo.defaults.color = color_red.as_mut_ptr();
        setupMenuInfo.defaults.style = 0x1 as i32
    }
    setupMenuInfo.back.generic.type_0 = 6 as i32;
    setupMenuInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    setupMenuInfo.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    setupMenuInfo.back.generic.id = 18 as i32;
    setupMenuInfo.back.generic.callback =
        Some(UI_SetupMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    setupMenuInfo.back.generic.x = 0 as i32;
    setupMenuInfo.back.generic.y = 480 as i32 - 64 as i32;
    setupMenuInfo.back.width = 128 as i32;
    setupMenuInfo.back.height = 64 as i32;
    setupMenuInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut setupMenuInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut setupMenuInfo.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut setupMenuInfo.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut setupMenuInfo.setupplayer as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut setupMenuInfo.setupcontrols as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut setupMenuInfo.setupsystem as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut setupMenuInfo.game as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut setupMenuInfo.cdkey as *mut menutext_s as *mut libc::c_void,
    );
    //	Menu_AddItem( &setupMenuInfo.menu, &setupMenuInfo.load );
    //	Menu_AddItem( &setupMenuInfo.menu, &setupMenuInfo.save );
    if trap_Cvar_VariableValue(b"cl_paused\x00" as *const u8 as *const libc::c_char) == 0. {
        Menu_AddItem(
            &mut setupMenuInfo.menu as *mut _ as *mut _tag_menuframework,
            &mut setupMenuInfo.defaults as *mut menutext_s as *mut libc::c_void,
        );
    }
    Menu_AddItem(
        &mut setupMenuInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut setupMenuInfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
UI_SetupMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SetupMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
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
    UI_PushMenu(&mut setupMenuInfo.menu as *mut _ as *mut _tag_menuframework);
}

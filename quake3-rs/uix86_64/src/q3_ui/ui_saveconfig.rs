use ::libc;

pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_FillRect;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_mfield::MField_Draw;
pub use crate::src::q3_ui::ui_qmenu::color_orange;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::text_color_highlight;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor;
pub use crate::src::qcommon::q_math::colorBlack;
pub use crate::src::qcommon::q_math::colorRed;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menufield_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::mfield_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saveConfig_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub background: menubitmap_s,
    pub savename: menufield_s,
    pub back: menubitmap_s,
    pub save: menubitmap_s,
}

static mut saveConfig: saveConfig_t = saveConfig_t {
    menu: menuframework_s {
        cursor: 0,
        cursor_prev: 0,
        nitems: 0,
        items: [0 as *const libc::c_void as *mut libc::c_void; 64],
        draw: None,
        key: None,
        wrapAround: qfalse,
        fullscreen: qfalse,
        showlogo: qfalse,
    },
    banner: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    background: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    savename: menufield_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const menuframework_s as *mut menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        field: mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    back: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
    save: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const menuframework_s as *mut menuframework_s,
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
};
/*
===============
UI_SaveConfigMenu_BackEvent
===============
*/

unsafe extern "C" fn UI_SaveConfigMenu_BackEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    UI_PopMenu();
}
/*
===============
UI_SaveConfigMenu_SaveEvent
===============
*/

unsafe extern "C" fn UI_SaveConfigMenu_SaveEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    let mut configname: [libc::c_char; 64] = [0; 64];
    if event != 3 as i32 {
        return;
    }
    if saveConfig.savename.field.buffer[0 as i32 as usize] == 0 {
        return;
    }
    COM_StripExtension(
        saveConfig.savename.field.buffer.as_mut_ptr(),
        configname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    trap_Cmd_ExecuteText(
        EXEC_APPEND as i32,
        va(
            b"writeconfig %s.cfg\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            configname.as_mut_ptr(),
        ),
    );
    UI_PopMenu();
}
/*
===============
UI_SaveConfigMenu_SavenameDraw
===============
*/

unsafe extern "C" fn UI_SaveConfigMenu_SavenameDraw(mut self_0: *mut libc::c_void) {
    let mut f: *mut menufield_s = 0 as *mut menufield_s;
    let mut style: i32 = 0;
    let mut color: *mut f32 = 0 as *mut f32;
    f = self_0 as *mut menufield_s;
    if f == Menu_ItemAtCursor(&mut saveConfig.menu as *mut _ as *mut _tag_menuframework)
        as *mut menufield_s
    {
        style = 0 as i32 | 0x4000 as i32 | 0x10 as i32;
        color = text_color_highlight.as_mut_ptr()
    } else {
        style = 0 as i32 | 0x10 as i32;
        color = colorRed.as_mut_ptr()
    }
    UI_DrawProportionalString(
        320 as i32,
        192 as i32,
        b"Enter filename:\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_orange.as_mut_ptr(),
    );
    UI_FillRect(
        (*f).generic.x as f32,
        (*f).generic.y as f32,
        ((*f).field.widthInChars * 8 as i32) as f32,
        16 as i32 as f32,
        colorBlack.as_mut_ptr(),
    );
    MField_Draw(
        &mut (*f).field as *mut _ as *mut mfield_t,
        (*f).generic.x,
        (*f).generic.y,
        style,
        color,
    );
}
/*
=================
UI_SaveConfigMenu_Init
=================
*/

unsafe extern "C" fn UI_SaveConfigMenu_Init() {
    crate::stdlib::memset(
        &mut saveConfig as *mut saveConfig_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<saveConfig_t>() as libc::c_ulong,
    );
    UI_SaveConfigMenu_Cache();
    saveConfig.menu.wrapAround = qtrue;
    saveConfig.menu.fullscreen = qtrue;
    saveConfig.banner.generic.type_0 = 10 as i32;
    saveConfig.banner.generic.x = 320 as i32;
    saveConfig.banner.generic.y = 16 as i32;
    saveConfig.banner.string =
        b"SAVE CONFIG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    saveConfig.banner.color = color_white.as_mut_ptr();
    saveConfig.banner.style = 0x1 as i32;
    saveConfig.background.generic.type_0 = 6 as i32;
    saveConfig.background.generic.name =
        b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char;
    saveConfig.background.generic.flags = 0x4000 as i32 as u32;
    saveConfig.background.generic.x = 142 as i32;
    saveConfig.background.generic.y = 118 as i32;
    saveConfig.background.width = 359 as i32;
    saveConfig.background.height = 256 as i32;
    saveConfig.savename.generic.type_0 = 4 as i32;
    saveConfig.savename.generic.flags = 0x8000 as i32 as u32 | 0x80000 as i32 as u32;
    saveConfig.savename.generic.ownerdraw =
        Some(UI_SaveConfigMenu_SavenameDraw as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    saveConfig.savename.field.widthInChars = 20 as i32;
    saveConfig.savename.field.maxchars = 20 as i32;
    saveConfig.savename.generic.x = 240 as i32;
    saveConfig.savename.generic.y = 155 as i32 + 72 as i32;
    saveConfig.savename.generic.left = 240 as i32;
    saveConfig.savename.generic.top = 155 as i32 + 72 as i32;
    saveConfig.savename.generic.right = 233 as i32 + 20 as i32 * 8 as i32;
    saveConfig.savename.generic.bottom = 155 as i32 + 72 as i32 + 16 as i32 + 2 as i32;
    saveConfig.back.generic.type_0 = 6 as i32;
    saveConfig.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    saveConfig.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    saveConfig.back.generic.id = 11 as i32;
    saveConfig.back.generic.callback = Some(
        UI_SaveConfigMenu_BackEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    saveConfig.back.generic.x = 0 as i32;
    saveConfig.back.generic.y = 480 as i32 - 64 as i32;
    saveConfig.back.width = 128 as i32;
    saveConfig.back.height = 64 as i32;
    saveConfig.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    saveConfig.save.generic.type_0 = 6 as i32;
    saveConfig.save.generic.name = b"menu/art/save_0\x00" as *const u8 as *const libc::c_char;
    saveConfig.save.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    saveConfig.save.generic.id = 12 as i32;
    saveConfig.save.generic.callback = Some(
        UI_SaveConfigMenu_SaveEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    saveConfig.save.generic.x = 640 as i32;
    saveConfig.save.generic.y = 480 as i32 - 64 as i32;
    saveConfig.save.width = 128 as i32;
    saveConfig.save.height = 64 as i32;
    saveConfig.save.focuspic =
        b"menu/art/save_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut saveConfig.menu as *mut _ as *mut _tag_menuframework,
        &mut saveConfig.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut saveConfig.menu as *mut _ as *mut _tag_menuframework,
        &mut saveConfig.background as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut saveConfig.menu as *mut _ as *mut _tag_menuframework,
        &mut saveConfig.savename as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut saveConfig.menu as *mut _ as *mut _tag_menuframework,
        &mut saveConfig.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut saveConfig.menu as *mut _ as *mut _tag_menuframework,
        &mut saveConfig.save as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
UI_SaveConfigMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SaveConfigMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/save_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/save_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char);
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
//
// ui_saveconfig.c
//
/*
===============
UI_SaveConfigMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SaveConfigMenu() {
    UI_SaveConfigMenu_Init();
    UI_PushMenu(&mut saveConfig.menu as *mut _ as *mut _tag_menuframework);
}

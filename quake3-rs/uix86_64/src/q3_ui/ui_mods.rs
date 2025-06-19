use ::libc;

pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_FS_GetFileList;
pub use crate::src::ui::ui_syscalls::trap_Print;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menutext_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mods_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub list: crate::ui_local_h::menulist_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub go: crate::ui_local_h::menubitmap_s,
    pub description: [libc::c_char; 3072],
    pub fs_game: [libc::c_char; 1024],
    pub descriptionPtr: *mut libc::c_char,
    pub fs_gamePtr: *mut libc::c_char,
    pub descriptionList: [*mut libc::c_char; 64],
    pub fs_gameList: [*mut libc::c_char; 64],
}

static mut s_mods: mods_t = mods_t {
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
        focuscolor: 0 as *const f32 as *mut f32,
    },
    description: [0; 3072],
    fs_game: [0; 1024],
    descriptionPtr: 0 as *const libc::c_char as *mut libc::c_char,
    fs_gamePtr: 0 as *const libc::c_char as *mut libc::c_char,
    descriptionList: [0 as *const libc::c_char as *mut libc::c_char; 64],
    fs_gameList: [0 as *const libc::c_char as *mut libc::c_char; 64],
};
/*
===============
UI_Mods_MenuEvent
===============
*/

unsafe extern "C" fn UI_Mods_MenuEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        11 => {
            crate::src::ui::ui_syscalls::trap_Cvar_Set(
                b"fs_game\x00" as *const u8 as *const libc::c_char,
                s_mods.fs_gameList[s_mods.list.curvalue as usize],
            );
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                b"vid_restart;\x00" as *const u8 as *const libc::c_char,
            );
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        10 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        _ => {}
    };
}
/*
===============
UI_Mods_ParseInfos
===============
*/

unsafe extern "C" fn UI_Mods_ParseInfos(
    mut modDir: *mut libc::c_char,
    mut modDesc: *mut libc::c_char,
) {
    s_mods.fs_gameList[s_mods.list.numitems as usize] = s_mods.fs_gamePtr;
    crate::src::qcommon::q_shared::Q_strncpyz(s_mods.fs_gamePtr, modDir, 16 as i32);
    s_mods.descriptionList[s_mods.list.numitems as usize] = s_mods.descriptionPtr;
    crate::src::qcommon::q_shared::Q_strncpyz(s_mods.descriptionPtr, modDesc, 48 as i32);
    let ref mut fresh0 = *s_mods.list.itemnames.offset(s_mods.list.numitems as isize);
    *fresh0 = s_mods.descriptionPtr;
    s_mods.descriptionPtr = s_mods.descriptionPtr.offset(
        crate::stdlib::strlen(s_mods.descriptionPtr).wrapping_add(1 as i32 as libc::c_ulong)
            as isize,
    );
    s_mods.fs_gamePtr = s_mods.fs_gamePtr.offset(
        crate::stdlib::strlen(s_mods.fs_gamePtr).wrapping_add(1 as i32 as libc::c_ulong)
            as isize,
    );
    s_mods.list.numitems += 1;
}
/*
===============
UI_Mods_LoadMods
===============
*/

unsafe extern "C" fn UI_Mods_LoadMods() {
    let mut numdirs: i32 = 0;
    let mut dirlist: [libc::c_char; 2048] = [0; 2048];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut descptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: i32 = 0;
    let mut dirlen: i32 = 0;
    s_mods.list.itemnames = s_mods.descriptionList.as_mut_ptr() as *mut *const libc::c_char;
    s_mods.descriptionPtr = s_mods.description.as_mut_ptr();
    s_mods.fs_gamePtr = s_mods.fs_game.as_mut_ptr();
    // always start off with baseq3
    s_mods.list.numitems = 1 as i32;
    s_mods.descriptionList[0 as i32 as usize] =
        b"Quake III Arena\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let ref mut fresh1 = *s_mods.list.itemnames.offset(0 as i32 as isize);
    *fresh1 = s_mods.descriptionList[0 as i32 as usize];
    s_mods.fs_gameList[0 as i32 as usize] =
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    numdirs = crate::src::ui::ui_syscalls::trap_FS_GetFileList(
        b"$modlist\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong as i32,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0 as i32;
    while i < numdirs {
        dirlen = crate::stdlib::strlen(dirptr).wrapping_add(1 as i32 as libc::c_ulong)
            as i32;
        descptr = dirptr.offset(dirlen as isize);
        UI_Mods_ParseInfos(dirptr, descptr);
        dirptr = dirptr.offset(
            (dirlen as libc::c_ulong)
                .wrapping_add(crate::stdlib::strlen(descptr))
                .wrapping_add(1 as i32 as libc::c_ulong) as isize,
        );
        i += 1
    }
    crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
        b"%i mods parsed\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_mods.list.numitems,
    ));
    if s_mods.list.numitems > 64 as i32 {
        s_mods.list.numitems = 64 as i32
    };
}
/*
===============
UI_Mods_MenuInit
===============
*/

unsafe extern "C" fn UI_Mods_MenuInit() {
    UI_ModsMenu_Cache();
    crate::stdlib::memset(
        &mut s_mods as *mut mods_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<mods_t>() as libc::c_ulong,
    );
    s_mods.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_mods.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_mods.banner.generic.type_0 = 10 as i32;
    s_mods.banner.generic.x = 320 as i32;
    s_mods.banner.generic.y = 16 as i32;
    s_mods.banner.string = b"MODS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_mods.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    s_mods.banner.style = 0x1 as i32;
    s_mods.framel.generic.type_0 = 6 as i32;
    s_mods.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_mods.framel.generic.flags = 0x4000 as i32 as u32;
    s_mods.framel.generic.x = 0 as i32;
    s_mods.framel.generic.y = 78 as i32;
    s_mods.framel.width = 256 as i32;
    s_mods.framel.height = 329 as i32;
    s_mods.framer.generic.type_0 = 6 as i32;
    s_mods.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_mods.framer.generic.flags = 0x4000 as i32 as u32;
    s_mods.framer.generic.x = 376 as i32;
    s_mods.framer.generic.y = 76 as i32;
    s_mods.framer.width = 256 as i32;
    s_mods.framer.height = 334 as i32;
    s_mods.back.generic.type_0 = 6 as i32;
    s_mods.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_mods.back.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_mods.back.generic.id = 10 as i32;
    s_mods.back.generic.callback =
        Some(UI_Mods_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_mods.back.generic.x = 0 as i32;
    s_mods.back.generic.y = 480 as i32 - 64 as i32;
    s_mods.back.width = 128 as i32;
    s_mods.back.height = 64 as i32;
    s_mods.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_mods.go.generic.type_0 = 6 as i32;
    s_mods.go.generic.name = b"menu/art/load_0\x00" as *const u8 as *const libc::c_char;
    s_mods.go.generic.flags =
        0x10 as i32 as u32 | 0x100 as i32 as u32;
    s_mods.go.generic.id = 11 as i32;
    s_mods.go.generic.callback =
        Some(UI_Mods_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_mods.go.generic.x = 640 as i32;
    s_mods.go.generic.y = 480 as i32 - 64 as i32;
    s_mods.go.width = 128 as i32;
    s_mods.go.height = 64 as i32;
    s_mods.go.focuspic =
        b"menu/art/load_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    // scan for mods
    s_mods.list.generic.type_0 = 8 as i32;
    s_mods.list.generic.flags =
        0x100 as i32 as u32 | 0x8 as i32 as u32;
    s_mods.list.generic.callback =
        Some(UI_Mods_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_mods.list.generic.id = 12 as i32;
    s_mods.list.generic.x = 320 as i32;
    s_mods.list.generic.y = 130 as i32;
    s_mods.list.width = 48 as i32;
    s_mods.list.height = 14 as i32;
    UI_Mods_LoadMods();
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_mods.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_mods.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_mods.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_mods.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_mods.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_mods.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_mods.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_mods.list as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_mods.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_mods.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_mods.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_mods.go as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
UI_Mods_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ModsMenu_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/load_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/load_1\x00" as *const u8 as *const libc::c_char,
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
/*
===============
UI_ModsMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ModsMenu() {
    UI_Mods_MenuInit();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut s_mods.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}

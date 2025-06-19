use ::libc;

pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::text_color_normal;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_DefaultKey;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Info_NextPair;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_GetConfigString;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serverinfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub add: crate::ui_local_h::menutext_s,
    pub info: [libc::c_char; 1024],
    pub numlines: i32,
}

static mut serverinfo_artlist: [*mut libc::c_char; 5] = [
    b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];

static mut s_serverinfo: serverinfo_t = serverinfo_t {
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
    add: crate::ui_local_h::menutext_s {
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
    info: [0; 1024],
    numlines: 0,
};
/*
=================
Favorites_Add

Add current server to favorites
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Favorites_Add() {
    let mut adrstr: [libc::c_char; 128] = [0; 128];
    let mut serverbuff: [libc::c_char; 128] = [0; 128];
    let mut i: i32 = 0;
    let mut best: i32 = 0;
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"cl_currentServerAddress\x00" as *const u8 as *const libc::c_char,
        serverbuff.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as i32,
    );
    if serverbuff[0 as i32 as usize] == 0 {
        return;
    }
    best = 0 as i32;
    i = 0 as i32;
    while i < 16 as i32 {
        crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
            crate::src::qcommon::q_shared::va(
                b"server%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                i + 1 as i32,
            ),
            adrstr.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as i32,
        );
        if crate::src::qcommon::q_shared::Q_stricmp(serverbuff.as_mut_ptr(), adrstr.as_mut_ptr())
            == 0
        {
            // already in list
            return;
        }
        // use first empty available slot
        if adrstr[0 as i32 as usize] == 0 && best == 0 {
            best = i + 1 as i32
        }
        i += 1
    }
    if best != 0 {
        crate::src::ui::ui_syscalls::trap_Cvar_Set(
            crate::src::qcommon::q_shared::va(
                b"server%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                best,
            ),
            serverbuff.as_mut_ptr(),
        );
    };
}
/*
=================
ServerInfo_Event
=================
*/

unsafe extern "C" fn ServerInfo_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        100 => {
            if !(event != 3 as i32) {
                Favorites_Add();
                crate::src::q3_ui::ui_atoms::UI_PopMenu();
            }
        }
        101 => {
            if !(event != 3 as i32) {
                crate::src::q3_ui::ui_atoms::UI_PopMenu();
            }
        }
        _ => {}
    };
}
/*
=================
ServerInfo_MenuDraw
=================
*/

unsafe extern "C" fn ServerInfo_MenuDraw() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: [libc::c_char; 1024] = [0; 1024];
    let mut value: [libc::c_char; 1024] = [0; 1024];
    let mut i: i32 = 0 as i32;
    let mut y: i32 = 0;
    y = 480 as i32 / 2 as i32
        - s_serverinfo.numlines * 16 as i32 / 2 as i32
        - 20 as i32;
    s = s_serverinfo.info.as_mut_ptr();
    while !s.is_null() && i < s_serverinfo.numlines {
        crate::src::qcommon::q_shared::Info_NextPair(&mut s, key.as_mut_ptr(), value.as_mut_ptr());
        if key[0 as i32 as usize] == 0 {
            break;
        }
        crate::src::qcommon::q_shared::Q_strcat(
            key.as_mut_ptr(),
            1024 as i32,
            b":\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            (640 as i32 as f64 * 0.50f64 - 8 as i32 as f64)
                as i32,
            y,
            key.as_mut_ptr(),
            0x2 as i32 | 0x10 as i32,
            crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr(),
        );
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            (640 as i32 as f64 * 0.50f64 + 8 as i32 as f64)
                as i32,
            y,
            value.as_mut_ptr(),
            0 as i32 | 0x10 as i32,
            crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
        );
        y += 16 as i32;
        i += 1
    }
    crate::src::q3_ui::ui_qmenu::Menu_Draw(
        &mut s_serverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}
/*
=================
ServerInfo_MenuKey
=================
*/

unsafe extern "C" fn ServerInfo_MenuKey(
    mut key: i32,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    return crate::src::q3_ui::ui_qmenu::Menu_DefaultKey(
        &mut s_serverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        key,
    );
}
/*
=================
ServerInfo_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ServerInfo_Cache() {
    let mut i: i32 = 0;
    // touch all our pics
    i = 0 as i32;
    while !serverinfo_artlist[i as usize].is_null() {
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(serverinfo_artlist[i as usize]);
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
/*
=================
UI_ServerInfoMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ServerInfoMenu() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: [libc::c_char; 1024] = [0; 1024];
    let mut value: [libc::c_char; 1024] = [0; 1024];
    // zero set all our globals
    crate::stdlib::memset(
        &mut s_serverinfo as *mut serverinfo_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<serverinfo_t>() as libc::c_ulong,
    );
    ServerInfo_Cache();
    s_serverinfo.menu.draw = Some(ServerInfo_MenuDraw as unsafe extern "C" fn() -> ());
    s_serverinfo.menu.key = Some(
        ServerInfo_MenuKey
            as unsafe extern "C" fn(_: i32) -> crate::src::qcommon::q_shared::sfxHandle_t,
    );
    s_serverinfo.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_serverinfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_serverinfo.banner.generic.type_0 = 10 as i32;
    s_serverinfo.banner.generic.x = 320 as i32;
    s_serverinfo.banner.generic.y = 16 as i32;
    s_serverinfo.banner.string =
        b"SERVER INFO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_serverinfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    s_serverinfo.banner.style = 0x1 as i32;
    s_serverinfo.framel.generic.type_0 = 6 as i32;
    s_serverinfo.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_serverinfo.framel.generic.flags = 0x4000 as i32 as u32;
    s_serverinfo.framel.generic.x = 0 as i32;
    s_serverinfo.framel.generic.y = 78 as i32;
    s_serverinfo.framel.width = 256 as i32;
    s_serverinfo.framel.height = 329 as i32;
    s_serverinfo.framer.generic.type_0 = 6 as i32;
    s_serverinfo.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_serverinfo.framer.generic.flags = 0x4000 as i32 as u32;
    s_serverinfo.framer.generic.x = 376 as i32;
    s_serverinfo.framer.generic.y = 76 as i32;
    s_serverinfo.framer.width = 256 as i32;
    s_serverinfo.framer.height = 334 as i32;
    s_serverinfo.add.generic.type_0 = 9 as i32;
    s_serverinfo.add.generic.flags =
        0x8 as i32 as u32 | 0x100 as i32 as u32;
    s_serverinfo.add.generic.callback =
        Some(ServerInfo_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_serverinfo.add.generic.id = 100 as i32;
    s_serverinfo.add.generic.x = 320 as i32;
    s_serverinfo.add.generic.y = 371 as i32;
    s_serverinfo.add.string =
        b"ADD TO FAVORITES\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_serverinfo.add.style = 0x1 as i32 | 0x10 as i32;
    s_serverinfo.add.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    if crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"sv_running\x00" as *const u8 as *const libc::c_char,
    ) != 0.
    {
        s_serverinfo.add.generic.flags |= 0x2000 as i32 as u32
    }
    s_serverinfo.back.generic.type_0 = 6 as i32;
    s_serverinfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_serverinfo.back.generic.flags =
        0x4 as i32 as u32 | 0x100 as i32 as u32;
    s_serverinfo.back.generic.callback =
        Some(ServerInfo_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_serverinfo.back.generic.id = 101 as i32;
    s_serverinfo.back.generic.x = 0 as i32;
    s_serverinfo.back.generic.y = 480 as i32 - 64 as i32;
    s_serverinfo.back.width = 128 as i32;
    s_serverinfo.back.height = 64 as i32;
    s_serverinfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    crate::src::ui::ui_syscalls::trap_GetConfigString(
        0 as i32,
        s_serverinfo.info.as_mut_ptr(),
        1024 as i32,
    );
    s_serverinfo.numlines = 0 as i32;
    s = s_serverinfo.info.as_mut_ptr();
    while !s.is_null() {
        crate::src::qcommon::q_shared::Info_NextPair(&mut s, key.as_mut_ptr(), value.as_mut_ptr());
        if key[0 as i32 as usize] == 0 {
            break;
        }
        s_serverinfo.numlines += 1
    }
    if s_serverinfo.numlines > 16 as i32 {
        s_serverinfo.numlines = 16 as i32
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_serverinfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_serverinfo.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_serverinfo.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_serverinfo.add as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut s_serverinfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut s_serverinfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}

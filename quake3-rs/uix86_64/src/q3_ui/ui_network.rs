use ::libc;

pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_display::UI_DisplayOptionsMenu;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem;
pub use crate::src::q3_ui::ui_sound::UI_SoundOptionsMenu;
pub use crate::src::q3_ui::ui_video::UI_GraphicsOptionsMenu;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menutext_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct networkOptionsInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub graphics: crate::ui_local_h::menutext_s,
    pub display: crate::ui_local_h::menutext_s,
    pub sound: crate::ui_local_h::menutext_s,
    pub network: crate::ui_local_h::menutext_s,
    pub rate: crate::ui_local_h::menulist_s,
    pub back: crate::ui_local_h::menubitmap_s,
}

static mut rate_items: [*const libc::c_char; 6] = [
    b"<= 28.8K\x00" as *const u8 as *const libc::c_char,
    b"33.6K\x00" as *const u8 as *const libc::c_char,
    b"56K\x00" as *const u8 as *const libc::c_char,
    b"ISDN\x00" as *const u8 as *const libc::c_char,
    b"LAN/Cable/xDSL\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

static mut networkOptionsInfo: networkOptionsInfo_t = networkOptionsInfo_t {
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
    graphics: crate::ui_local_h::menutext_s {
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
    display: crate::ui_local_h::menutext_s {
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
    sound: crate::ui_local_h::menutext_s {
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
    network: crate::ui_local_h::menutext_s {
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
    rate: crate::ui_local_h::menulist_s {
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
};
/*
=================
UI_NetworkOptionsMenu_Event
=================
*/

unsafe extern "C" fn UI_NetworkOptionsMenu_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        10 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_video::UI_GraphicsOptionsMenu();
        }
        11 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_display::UI_DisplayOptionsMenu();
        }
        12 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_sound::UI_SoundOptionsMenu();
        }
        14 => {
            if networkOptionsInfo.rate.curvalue == 0 as i32 {
                crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                    b"rate\x00" as *const u8 as *const libc::c_char,
                    2500 as i32 as f32,
                );
            } else if networkOptionsInfo.rate.curvalue == 1 as i32 {
                crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                    b"rate\x00" as *const u8 as *const libc::c_char,
                    3000 as i32 as f32,
                );
            } else if networkOptionsInfo.rate.curvalue == 2 as i32 {
                crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                    b"rate\x00" as *const u8 as *const libc::c_char,
                    4000 as i32 as f32,
                );
            } else if networkOptionsInfo.rate.curvalue == 3 as i32 {
                crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                    b"rate\x00" as *const u8 as *const libc::c_char,
                    5000 as i32 as f32,
                );
            } else if networkOptionsInfo.rate.curvalue == 4 as i32 {
                crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                    b"rate\x00" as *const u8 as *const libc::c_char,
                    25000 as i32 as f32,
                );
            }
        }
        15 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        13 | _ => {}
    };
}
/*
===============
UI_NetworkOptionsMenu_Init
===============
*/

unsafe extern "C" fn UI_NetworkOptionsMenu_Init() {
    let mut y: i32 = 0;
    let mut rate: i32 = 0;
    crate::stdlib::memset(
        &mut networkOptionsInfo as *mut networkOptionsInfo_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<networkOptionsInfo_t>() as libc::c_ulong,
    );
    UI_NetworkOptionsMenu_Cache();
    networkOptionsInfo.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    networkOptionsInfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    networkOptionsInfo.banner.generic.type_0 = 10 as i32;
    networkOptionsInfo.banner.generic.flags = 0x8 as i32 as u32;
    networkOptionsInfo.banner.generic.x = 320 as i32;
    networkOptionsInfo.banner.generic.y = 16 as i32;
    networkOptionsInfo.banner.string =
        b"SYSTEM SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    networkOptionsInfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    networkOptionsInfo.banner.style = 0x1 as i32;
    networkOptionsInfo.framel.generic.type_0 = 6 as i32;
    networkOptionsInfo.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    networkOptionsInfo.framel.generic.flags = 0x4000 as i32 as u32;
    networkOptionsInfo.framel.generic.x = 0 as i32;
    networkOptionsInfo.framel.generic.y = 78 as i32;
    networkOptionsInfo.framel.width = 256 as i32;
    networkOptionsInfo.framel.height = 329 as i32;
    networkOptionsInfo.framer.generic.type_0 = 6 as i32;
    networkOptionsInfo.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    networkOptionsInfo.framer.generic.flags = 0x4000 as i32 as u32;
    networkOptionsInfo.framer.generic.x = 376 as i32;
    networkOptionsInfo.framer.generic.y = 76 as i32;
    networkOptionsInfo.framer.width = 256 as i32;
    networkOptionsInfo.framer.height = 334 as i32;
    networkOptionsInfo.graphics.generic.type_0 = 9 as i32;
    networkOptionsInfo.graphics.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    networkOptionsInfo.graphics.generic.id = 10 as i32;
    networkOptionsInfo.graphics.generic.callback = Some(
        UI_NetworkOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    networkOptionsInfo.graphics.generic.x = 216 as i32;
    networkOptionsInfo.graphics.generic.y = 240 as i32 - 2 as i32 * 27 as i32;
    networkOptionsInfo.graphics.string =
        b"GRAPHICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    networkOptionsInfo.graphics.style = 0x2 as i32;
    networkOptionsInfo.graphics.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    networkOptionsInfo.display.generic.type_0 = 9 as i32;
    networkOptionsInfo.display.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    networkOptionsInfo.display.generic.id = 11 as i32;
    networkOptionsInfo.display.generic.callback = Some(
        UI_NetworkOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    networkOptionsInfo.display.generic.x = 216 as i32;
    networkOptionsInfo.display.generic.y = 240 as i32 - 27 as i32;
    networkOptionsInfo.display.string =
        b"DISPLAY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    networkOptionsInfo.display.style = 0x2 as i32;
    networkOptionsInfo.display.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    networkOptionsInfo.sound.generic.type_0 = 9 as i32;
    networkOptionsInfo.sound.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    networkOptionsInfo.sound.generic.id = 12 as i32;
    networkOptionsInfo.sound.generic.callback = Some(
        UI_NetworkOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    networkOptionsInfo.sound.generic.x = 216 as i32;
    networkOptionsInfo.sound.generic.y = 240 as i32;
    networkOptionsInfo.sound.string =
        b"SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    networkOptionsInfo.sound.style = 0x2 as i32;
    networkOptionsInfo.sound.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    networkOptionsInfo.network.generic.type_0 = 9 as i32;
    networkOptionsInfo.network.generic.flags = 0x10 as i32 as u32;
    networkOptionsInfo.network.generic.id = 13 as i32;
    networkOptionsInfo.network.generic.callback = Some(
        UI_NetworkOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    networkOptionsInfo.network.generic.x = 216 as i32;
    networkOptionsInfo.network.generic.y = 240 as i32 + 27 as i32;
    networkOptionsInfo.network.string =
        b"NETWORK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    networkOptionsInfo.network.style = 0x2 as i32;
    networkOptionsInfo.network.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    y = 240 as i32 - 1 as i32 * (16 as i32 + 2 as i32);
    networkOptionsInfo.rate.generic.type_0 = 3 as i32;
    networkOptionsInfo.rate.generic.name = b"Data Rate:\x00" as *const u8 as *const libc::c_char;
    networkOptionsInfo.rate.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    networkOptionsInfo.rate.generic.callback = Some(
        UI_NetworkOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    networkOptionsInfo.rate.generic.id = 14 as i32;
    networkOptionsInfo.rate.generic.x = 400 as i32;
    networkOptionsInfo.rate.generic.y = y;
    networkOptionsInfo.rate.itemnames = rate_items.as_mut_ptr();
    networkOptionsInfo.back.generic.type_0 = 6 as i32;
    networkOptionsInfo.back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    networkOptionsInfo.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    networkOptionsInfo.back.generic.callback = Some(
        UI_NetworkOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    networkOptionsInfo.back.generic.id = 15 as i32;
    networkOptionsInfo.back.generic.x = 0 as i32;
    networkOptionsInfo.back.generic.y = 480 as i32 - 64 as i32;
    networkOptionsInfo.back.width = 128 as i32;
    networkOptionsInfo.back.height = 64 as i32;
    networkOptionsInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut networkOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut networkOptionsInfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut networkOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut networkOptionsInfo.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut networkOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut networkOptionsInfo.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut networkOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut networkOptionsInfo.graphics as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut networkOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut networkOptionsInfo.display as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut networkOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut networkOptionsInfo.sound as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut networkOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut networkOptionsInfo.network as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut networkOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut networkOptionsInfo.rate as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut networkOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut networkOptionsInfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    rate = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"rate\x00" as *const u8 as *const libc::c_char,
    ) as i32;
    if rate <= 2500 as i32 {
        networkOptionsInfo.rate.curvalue = 0 as i32
    } else if rate <= 3000 as i32 {
        networkOptionsInfo.rate.curvalue = 1 as i32
    } else if rate <= 4000 as i32 {
        networkOptionsInfo.rate.curvalue = 2 as i32
    } else if rate <= 5000 as i32 {
        networkOptionsInfo.rate.curvalue = 3 as i32
    } else {
        networkOptionsInfo.rate.curvalue = 4 as i32
    };
}
/*
===============
UI_NetworkOptionsMenu_Cache
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_NetworkOptionsMenu_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char,
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
//
// ui_saveconfig.c
//
//
// ui_display.c
//
//
// ui_sound.c
//
//
// ui_network.c
//
/*
===============
UI_NetworkOptionsMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_NetworkOptionsMenu() {
    UI_NetworkOptionsMenu_Init();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut networkOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
    crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem(
        &mut networkOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut networkOptionsInfo.network as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
}

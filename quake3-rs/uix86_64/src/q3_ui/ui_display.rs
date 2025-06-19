use ::libc;

pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu;
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
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menuslider_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::uiStatic_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayOptionsInfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub graphics: menutext_s,
    pub display: menutext_s,
    pub sound: menutext_s,
    pub network: menutext_s,
    pub brightness: menuslider_s,
    pub screensize: menuslider_s,
    pub back: menubitmap_s,
}

static mut displayOptionsInfo: displayOptionsInfo_t = displayOptionsInfo_t {
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
    framel: menubitmap_s {
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
    framer: menubitmap_s {
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
    graphics: menutext_s {
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
    display: menutext_s {
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
    sound: menutext_s {
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
    network: menutext_s {
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
    brightness: menuslider_s {
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
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
    },
    screensize: menuslider_s {
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
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
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
};
/*
=================
UI_DisplayOptionsMenu_Event
=================
*/

unsafe extern "C" fn UI_DisplayOptionsMenu_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 as i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        10 => {
            UI_PopMenu();
            UI_GraphicsOptionsMenu();
        }
        12 => {
            UI_PopMenu();
            UI_SoundOptionsMenu();
        }
        13 => {
            UI_PopMenu();
            UI_NetworkOptionsMenu();
        }
        14 => {
            trap_Cvar_SetValue(
                b"r_gamma\x00" as *const u8 as *const libc::c_char,
                displayOptionsInfo.brightness.curvalue / 10.0f32,
            );
        }
        15 => {
            trap_Cvar_SetValue(
                b"cg_viewsize\x00" as *const u8 as *const libc::c_char,
                displayOptionsInfo.screensize.curvalue * 10 as i32 as f32,
            );
        }
        16 => {
            UI_PopMenu();
        }
        11 | _ => {}
    };
}
/*
===============
UI_DisplayOptionsMenu_Init
===============
*/

unsafe extern "C" fn UI_DisplayOptionsMenu_Init() {
    let mut y: i32 = 0;
    crate::stdlib::memset(
        &mut displayOptionsInfo as *mut displayOptionsInfo_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<displayOptionsInfo_t>() as usize,
    );
    UI_DisplayOptionsMenu_Cache();
    displayOptionsInfo.menu.wrapAround = qtrue;
    displayOptionsInfo.menu.fullscreen = qtrue;
    displayOptionsInfo.banner.generic.type_0 = 10 as i32;
    displayOptionsInfo.banner.generic.flags = 0x8 as i32 as u32;
    displayOptionsInfo.banner.generic.x = 320 as i32;
    displayOptionsInfo.banner.generic.y = 16 as i32;
    displayOptionsInfo.banner.string =
        b"SYSTEM SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    displayOptionsInfo.banner.color = color_white.as_mut_ptr();
    displayOptionsInfo.banner.style = 0x1 as i32;
    displayOptionsInfo.framel.generic.type_0 = 6 as i32;
    displayOptionsInfo.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    displayOptionsInfo.framel.generic.flags = 0x4000 as i32 as u32;
    displayOptionsInfo.framel.generic.x = 0 as i32;
    displayOptionsInfo.framel.generic.y = 78 as i32;
    displayOptionsInfo.framel.width = 256 as i32;
    displayOptionsInfo.framel.height = 329 as i32;
    displayOptionsInfo.framer.generic.type_0 = 6 as i32;
    displayOptionsInfo.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    displayOptionsInfo.framer.generic.flags = 0x4000 as i32 as u32;
    displayOptionsInfo.framer.generic.x = 376 as i32;
    displayOptionsInfo.framer.generic.y = 76 as i32;
    displayOptionsInfo.framer.width = 256 as i32;
    displayOptionsInfo.framer.height = 334 as i32;
    displayOptionsInfo.graphics.generic.type_0 = 9 as i32;
    displayOptionsInfo.graphics.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    displayOptionsInfo.graphics.generic.id = 10 as i32;
    displayOptionsInfo.graphics.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.graphics.generic.x = 216 as i32;
    displayOptionsInfo.graphics.generic.y = 240 as i32 - 2 as i32 * 27 as i32;
    displayOptionsInfo.graphics.string =
        b"GRAPHICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    displayOptionsInfo.graphics.style = 0x2 as i32;
    displayOptionsInfo.graphics.color = color_red.as_mut_ptr();
    displayOptionsInfo.display.generic.type_0 = 9 as i32;
    displayOptionsInfo.display.generic.flags = 0x10 as i32 as u32;
    displayOptionsInfo.display.generic.id = 11 as i32;
    displayOptionsInfo.display.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.display.generic.x = 216 as i32;
    displayOptionsInfo.display.generic.y = 240 as i32 - 27 as i32;
    displayOptionsInfo.display.string =
        b"DISPLAY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    displayOptionsInfo.display.style = 0x2 as i32;
    displayOptionsInfo.display.color = color_red.as_mut_ptr();
    displayOptionsInfo.sound.generic.type_0 = 9 as i32;
    displayOptionsInfo.sound.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    displayOptionsInfo.sound.generic.id = 12 as i32;
    displayOptionsInfo.sound.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.sound.generic.x = 216 as i32;
    displayOptionsInfo.sound.generic.y = 240 as i32;
    displayOptionsInfo.sound.string =
        b"SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    displayOptionsInfo.sound.style = 0x2 as i32;
    displayOptionsInfo.sound.color = color_red.as_mut_ptr();
    displayOptionsInfo.network.generic.type_0 = 9 as i32;
    displayOptionsInfo.network.generic.flags = 0x10 as i32 as u32 | 0x100 as i32 as u32;
    displayOptionsInfo.network.generic.id = 13 as i32;
    displayOptionsInfo.network.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.network.generic.x = 216 as i32;
    displayOptionsInfo.network.generic.y = 240 as i32 + 27 as i32;
    displayOptionsInfo.network.string =
        b"NETWORK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    displayOptionsInfo.network.style = 0x2 as i32;
    displayOptionsInfo.network.color = color_red.as_mut_ptr();
    y = 240 as i32 - 1 as i32 * (16 as i32 + 2 as i32);
    displayOptionsInfo.brightness.generic.type_0 = 1 as i32;
    displayOptionsInfo.brightness.generic.name =
        b"Brightness:\x00" as *const u8 as *const libc::c_char;
    displayOptionsInfo.brightness.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    displayOptionsInfo.brightness.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.brightness.generic.id = 14 as i32;
    displayOptionsInfo.brightness.generic.x = 400 as i32;
    displayOptionsInfo.brightness.generic.y = y;
    displayOptionsInfo.brightness.minvalue = 5 as i32 as f32;
    displayOptionsInfo.brightness.maxvalue = 20 as i32 as f32;
    if uis.glconfig.deviceSupportsGamma as u64 == 0 {
        displayOptionsInfo.brightness.generic.flags |= 0x2000 as i32 as u32
    }
    y += 16 as i32 + 2 as i32;
    displayOptionsInfo.screensize.generic.type_0 = 1 as i32;
    displayOptionsInfo.screensize.generic.name =
        b"Screen Size:\x00" as *const u8 as *const libc::c_char;
    displayOptionsInfo.screensize.generic.flags = 0x100 as i32 as u32 | 0x2 as i32 as u32;
    displayOptionsInfo.screensize.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.screensize.generic.id = 15 as i32;
    displayOptionsInfo.screensize.generic.x = 400 as i32;
    displayOptionsInfo.screensize.generic.y = y;
    displayOptionsInfo.screensize.minvalue = 3 as i32 as f32;
    displayOptionsInfo.screensize.maxvalue = 10 as i32 as f32;
    displayOptionsInfo.back.generic.type_0 = 6 as i32;
    displayOptionsInfo.back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    displayOptionsInfo.back.generic.flags = 0x4 as i32 as u32 | 0x100 as i32 as u32;
    displayOptionsInfo.back.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.back.generic.id = 16 as i32;
    displayOptionsInfo.back.generic.x = 0 as i32;
    displayOptionsInfo.back.generic.y = 480 as i32 - 64 as i32;
    displayOptionsInfo.back.width = 128 as i32;
    displayOptionsInfo.back.height = 64 as i32;
    displayOptionsInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut displayOptionsInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut displayOptionsInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut displayOptionsInfo.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut displayOptionsInfo.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut displayOptionsInfo.graphics as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut displayOptionsInfo.display as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut displayOptionsInfo.sound as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut displayOptionsInfo.network as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut displayOptionsInfo.brightness as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut displayOptionsInfo.screensize as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut displayOptionsInfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
    displayOptionsInfo.brightness.curvalue =
        trap_Cvar_VariableValue(b"r_gamma\x00" as *const u8 as *const libc::c_char)
            * 10 as i32 as f32;
    displayOptionsInfo.screensize.curvalue =
        trap_Cvar_VariableValue(b"cg_viewsize\x00" as *const u8 as *const libc::c_char)
            / 10 as i32 as f32;
}
/*
===============
UI_DisplayOptionsMenu_Cache
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DisplayOptionsMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
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
/*
===============
UI_DisplayOptionsMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DisplayOptionsMenu() {
    UI_DisplayOptionsMenu_Init();
    UI_PushMenu(&mut displayOptionsInfo.menu as *mut _ as *mut _tag_menuframework);
    Menu_SetCursorToItem(
        &mut displayOptionsInfo.menu as *mut _ as *mut _tag_menuframework,
        &mut displayOptionsInfo.display as *mut menutext_s as *mut libc::c_void,
    );
}

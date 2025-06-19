use ::libc;

pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menuframework_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct creditsmenu_t {
    pub menu: menuframework_s,
    pub frame: i32,
}

static mut s_credits: creditsmenu_t = creditsmenu_t {
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
    frame: 0,
};
/*
===============
UI_CreditMenu_Draw_ioq3
===============
*/

unsafe extern "C" fn UI_CreditMenu_Draw_ioq3() {
    let mut y: i32 = 0;
    let mut i: i32 = 0;
    // These are all people that have made commits to Subversion, and thus
    //  probably incomplete.
    // (These are in alphabetical order, for the defense of everyone's egos.)
    static mut names: [*const libc::c_char; 14] = [
        b"Tim Angus\x00" as *const u8 as *const libc::c_char,
        b"James Canete\x00" as *const u8 as *const libc::c_char,
        b"Vincent Cojot\x00" as *const u8 as *const libc::c_char,
        b"Ryan C. Gordon\x00" as *const u8 as *const libc::c_char,
        b"Aaron Gyes\x00" as *const u8 as *const libc::c_char,
        b"Zack Middleton\x00" as *const u8 as *const libc::c_char,
        b"Ludwig Nussel\x00" as *const u8 as *const libc::c_char,
        b"Julian Priestley\x00" as *const u8 as *const libc::c_char,
        b"Scirocco Six\x00" as *const u8 as *const libc::c_char,
        b"Thilo Schulz\x00" as *const u8 as *const libc::c_char,
        b"Zachary J. Slater\x00" as *const u8 as *const libc::c_char,
        b"Tony J. White\x00" as *const u8 as *const libc::c_char,
        b"...and many, many others!\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    // Center text vertically on the screen
    y = ((480 as i32 as f64
        - (::std::mem::size_of::<[*const libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as f64
            * (1.42f64 * 27 as i32 as f64 * 0.75f64))
        / 2 as i32 as f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"ioquake3 contributors:\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 1.42f64 * 27 as i32 as f64 * 0.75f64) as i32;
    i = 0 as i32;
    while !names[i as usize].is_null() {
        UI_DrawProportionalString(
            320 as i32,
            y,
            names[i as usize],
            0x1 as i32 | 0x10 as i32,
            color_white.as_mut_ptr(),
        );
        y = (y as f64 + 1.42f64 * 27 as i32 as f64 * 0.75f64) as i32;
        i += 1
    }
    UI_DrawString(
        320 as i32,
        459 as i32,
        b"http://www.ioquake3.org/\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_red.as_mut_ptr(),
    );
}
/*
=================
UI_CreditMenu_Key
=================
*/

unsafe extern "C" fn UI_CreditMenu_Key(mut key: i32) -> sfxHandle_t {
    if key & 1024 as i32 != 0 {
        return 0 as i32;
    }
    s_credits.frame += 1;
    if s_credits.frame == 1 as i32 {
        s_credits.menu.draw = Some(UI_CreditMenu_Draw_ioq3 as unsafe extern "C" fn() -> ())
    } else {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as i32,
            b"quit\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as i32;
}
/*
===============
UI_CreditMenu_Draw
===============
*/

unsafe extern "C" fn UI_CreditMenu_Draw() {
    let mut y: i32 = 0;
    y = 12 as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"id Software is:\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 1.42f64 * 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Programming\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"John Carmack, Robert A. Duffy, Jim Dose\'\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 1.42f64 * 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Art\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Adrian Carmack, Kevin Cloud,\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Kenneth Scott, Seneca Menard, Fred Nilsson\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 1.42f64 * 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Game Designer\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Graeme Devine\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 1.42f64 * 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Level Design\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Tim Willits, Christian Antkow, Paul Jaquays\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 1.42f64 * 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"CEO\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Todd Hollenshead\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 1.42f64 * 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Director of Business Development\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Marty Stratton\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 1.42f64 * 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Biz Assist and id Mom\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Donna Jackson\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 1.42f64 * 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Development Assistance\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawProportionalString(
        320 as i32,
        y,
        b"Eric Webb\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_white.as_mut_ptr(),
    );
    y = (y as f64 + 1.35f64 * 27 as i32 as f64 * 0.75f64) as i32;
    UI_DrawString(
        320 as i32,
        y,
        b"To order: 1-800-idgames     www.quake3arena.com     www.idsoftware.com\x00" as *const u8
            as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_red.as_mut_ptr(),
    );
    y += 16 as i32;
    UI_DrawString(
        320 as i32,
        y,
        b"Quake III Arena(c) 1999-2000, Id Software, Inc.  All Rights Reserved\x00" as *const u8
            as *const libc::c_char,
        0x1 as i32 | 0x10 as i32,
        color_red.as_mut_ptr(),
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
/*
===============
UI_CreditMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_CreditMenu() {
    crate::stdlib::memset(
        &mut s_credits as *mut creditsmenu_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<creditsmenu_t>() as libc::c_ulong,
    );
    s_credits.menu.draw = Some(UI_CreditMenu_Draw as unsafe extern "C" fn() -> ());
    s_credits.menu.key = Some(UI_CreditMenu_Key as unsafe extern "C" fn(_: i32) -> sfxHandle_t);
    s_credits.menu.fullscreen = qtrue;
    UI_PushMenu(&mut s_credits.menu as *mut _ as *mut _tag_menuframework);
}

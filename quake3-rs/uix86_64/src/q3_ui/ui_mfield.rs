use ::libc;

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}
pub use crate::stdlib::__int32_t;

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
pub use crate::src::q3_ui::ui_atoms::UI_DrawChar;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_FillRect;
pub use crate::src::q3_ui::ui_mfield::ctype_h::tolower;
pub use crate::src::q3_ui::ui_qmenu::listbar_color;
pub use crate::src::q3_ui::ui_qmenu::menu_buzz_sound;
pub use crate::src::q3_ui::ui_qmenu::text_color_disabled;
pub use crate::src::q3_ui::ui_qmenu::text_color_highlight;
pub use crate::src::q3_ui::ui_qmenu::text_color_normal;
pub use crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Q_isalpha;
pub use crate::src::qcommon::q_shared::Q_islower;
pub use crate::src::qcommon::q_shared::Q_isupper;
pub use crate::src::ui::ui_syscalls::trap_Error;
pub use crate::src::ui::ui_syscalls::trap_GetClipboardData;
pub use crate::src::ui::ui_syscalls::trap_Key_GetOverstrikeMode;
pub use crate::src::ui::ui_syscalls::trap_Key_IsDown;
pub use crate::src::ui::ui_syscalls::trap_Key_SetOverstrikeMode;
pub use crate::stdlib::__ctype_tolower_loc;

pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menufield_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::mfield_t;
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
/*
===================
MField_Draw

Handles horizontal scrolling and cursor blinking
x, y, are in pixels
===================
*/
#[no_mangle]

pub unsafe extern "C" fn MField_Draw(
    mut edit: *mut crate::ui_local_h::mfield_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut style: libc::c_int,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut len: libc::c_int = 0;
    let mut charw: libc::c_int = 0;
    let mut drawLen: libc::c_int = 0;
    let mut prestep: libc::c_int = 0;
    let mut cursorChar: libc::c_int = 0;
    let mut str: [libc::c_char; 1024] = [0; 1024];
    drawLen = (*edit).widthInChars;
    len = crate::stdlib::strlen((*edit).buffer.as_mut_ptr())
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    // guarantee that cursor will be visible
    if len <= drawLen {
        prestep = 0 as libc::c_int
    } else {
        if (*edit).scroll + drawLen > len {
            (*edit).scroll = len - drawLen;
            if (*edit).scroll < 0 as libc::c_int {
                (*edit).scroll = 0 as libc::c_int
            }
        }
        prestep = (*edit).scroll
    }
    if prestep + drawLen > len {
        drawLen = len - prestep
    }
    // extract <drawLen> characters from the field at <prestep>
    if drawLen >= 1024 as libc::c_int {
        crate::src::ui::ui_syscalls::trap_Error(
            b"drawLen >= MAX_STRING_CHARS\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::stdlib::memcpy(
        str.as_mut_ptr() as *mut libc::c_void,
        (*edit).buffer.as_mut_ptr().offset(prestep as isize) as *const libc::c_void,
        drawLen as libc::c_ulong,
    );
    str[drawLen as usize] = 0 as libc::c_int as libc::c_char;
    crate::src::q3_ui::ui_atoms::UI_DrawString(x, y, str.as_mut_ptr(), style, color);
    // draw the cursor
    if style & 0x4000 as libc::c_int == 0 {
        return;
    }
    if crate::src::ui::ui_syscalls::trap_Key_GetOverstrikeMode() as u64 != 0 {
        cursorChar = 11 as libc::c_int
    } else {
        cursorChar = 10 as libc::c_int
    }
    style &= !(0x4000 as libc::c_int);
    style |= 0x1000 as libc::c_int;
    if style & 0x10 as libc::c_int != 0 {
        charw = 8 as libc::c_int
    } else if style & 0x40 as libc::c_int != 0 {
        charw = 32 as libc::c_int
    } else {
        charw = 16 as libc::c_int
    }
    if style & 0x1 as libc::c_int != 0 {
        len = crate::stdlib::strlen(str.as_mut_ptr()) as libc::c_int;
        x = x - len * charw / 2 as libc::c_int
    } else if style & 0x2 as libc::c_int != 0 {
        len = crate::stdlib::strlen(str.as_mut_ptr()) as libc::c_int;
        x = x - len * charw
    }
    crate::src::q3_ui::ui_atoms::UI_DrawChar(
        x + ((*edit).cursor - prestep) * charw,
        y,
        cursorChar,
        style & !(0x1 as libc::c_int | 0x2 as libc::c_int),
        color,
    );
}
/*
================
MField_Paste
================
*/
#[no_mangle]

pub unsafe extern "C" fn MField_Paste(mut edit: *mut crate::ui_local_h::mfield_t) {
    let mut pasteBuffer: [libc::c_char; 64] = [0; 64];
    let mut pasteLen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    crate::src::ui::ui_syscalls::trap_GetClipboardData(pasteBuffer.as_mut_ptr(), 64 as libc::c_int);
    // send as if typed, so insert / overstrike works properly
    pasteLen = crate::stdlib::strlen(pasteBuffer.as_mut_ptr()) as libc::c_int;
    i = 0 as libc::c_int;
    while i < pasteLen {
        MField_CharEvent(edit, pasteBuffer[i as usize] as libc::c_int);
        i += 1
    }
}
/*
=================
MField_KeyDownEvent

Performs the basic line editing functions for the console,
in-game talk, and menu fields

Key events are used for non-printable characters, others are gotten from char events.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn MField_KeyDownEvent(
    mut edit: *mut crate::ui_local_h::mfield_t,
    mut key: libc::c_int,
) {
    let mut len: libc::c_int = 0;
    // shift-insert is paste
    if (key == crate::keycodes_h::K_INS as libc::c_int
        || key == crate::keycodes_h::K_KP_INS as libc::c_int)
        && crate::src::ui::ui_syscalls::trap_Key_IsDown(crate::keycodes_h::K_SHIFT as libc::c_int)
            as libc::c_uint
            != 0
    {
        MField_Paste(edit);
        return;
    }
    len = crate::stdlib::strlen((*edit).buffer.as_mut_ptr()) as libc::c_int;
    if key == crate::keycodes_h::K_DEL as libc::c_int
        || key == crate::keycodes_h::K_KP_DEL as libc::c_int
    {
        if (*edit).cursor < len {
            crate::stdlib::memmove(
                (*edit).buffer.as_mut_ptr().offset((*edit).cursor as isize) as *mut libc::c_void,
                (*edit)
                    .buffer
                    .as_mut_ptr()
                    .offset((*edit).cursor as isize)
                    .offset(1 as libc::c_int as isize) as *const libc::c_void,
                (len - (*edit).cursor) as libc::c_ulong,
            );
        }
        return;
    }
    if key == crate::keycodes_h::K_RIGHTARROW as libc::c_int
        || key == crate::keycodes_h::K_KP_RIGHTARROW as libc::c_int
    {
        if (*edit).cursor < len {
            (*edit).cursor += 1
        }
        if (*edit).cursor >= (*edit).scroll + (*edit).widthInChars && (*edit).cursor <= len {
            (*edit).scroll += 1
        }
        return;
    }
    if key == crate::keycodes_h::K_LEFTARROW as libc::c_int
        || key == crate::keycodes_h::K_KP_LEFTARROW as libc::c_int
    {
        if (*edit).cursor > 0 as libc::c_int {
            (*edit).cursor -= 1
        }
        if (*edit).cursor < (*edit).scroll {
            (*edit).scroll -= 1
        }
        return;
    }
    if key == crate::keycodes_h::K_HOME as libc::c_int
        || key == crate::keycodes_h::K_KP_HOME as libc::c_int
        || ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = key;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower(key)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_tolower_loc()).offset(key as isize)
            }
            __res
        }) == 'a' as i32
            && crate::src::ui::ui_syscalls::trap_Key_IsDown(
                crate::keycodes_h::K_CTRL as libc::c_int,
            ) as libc::c_uint
                != 0
    {
        (*edit).cursor = 0 as libc::c_int;
        (*edit).scroll = 0 as libc::c_int;
        return;
    }
    if key == crate::keycodes_h::K_END as libc::c_int
        || key == crate::keycodes_h::K_KP_END as libc::c_int
        || ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = key;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower(key)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_tolower_loc()).offset(key as isize)
            }
            __res
        }) == 'e' as i32
            && crate::src::ui::ui_syscalls::trap_Key_IsDown(
                crate::keycodes_h::K_CTRL as libc::c_int,
            ) as libc::c_uint
                != 0
    {
        (*edit).cursor = len;
        (*edit).scroll = len - (*edit).widthInChars + 1 as libc::c_int;
        if (*edit).scroll < 0 as libc::c_int {
            (*edit).scroll = 0 as libc::c_int
        }
        return;
    }
    if key == crate::keycodes_h::K_INS as libc::c_int
        || key == crate::keycodes_h::K_KP_INS as libc::c_int
    {
        crate::src::ui::ui_syscalls::trap_Key_SetOverstrikeMode(
            (crate::src::ui::ui_syscalls::trap_Key_GetOverstrikeMode() as u64 == 0) as libc::c_int
                as crate::src::qcommon::q_shared::qboolean,
        );
        return;
    };
}
/*
==================
MField_CharEvent
==================
*/
#[no_mangle]

pub unsafe extern "C" fn MField_CharEvent(
    mut edit: *mut crate::ui_local_h::mfield_t,
    mut ch: libc::c_int,
) {
    let mut len: libc::c_int = 0;
    if ch == 'v' as i32 - 'a' as i32 + 1 as libc::c_int {
        // ctrl-v is paste
        MField_Paste(edit);
        return;
    }
    if ch == 'c' as i32 - 'a' as i32 + 1 as libc::c_int {
        // ctrl-c clears the field
        MField_Clear(edit);
        return;
    }
    len = crate::stdlib::strlen((*edit).buffer.as_mut_ptr()) as libc::c_int;
    if ch == 'h' as i32 - 'a' as i32 + 1 as libc::c_int {
        // ctrl-h is backspace
        if (*edit).cursor > 0 as libc::c_int {
            crate::stdlib::memmove(
                (*edit)
                    .buffer
                    .as_mut_ptr()
                    .offset((*edit).cursor as isize)
                    .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                (*edit).buffer.as_mut_ptr().offset((*edit).cursor as isize) as *const libc::c_void,
                (len + 1 as libc::c_int - (*edit).cursor) as libc::c_ulong,
            );
            (*edit).cursor -= 1;
            if (*edit).cursor < (*edit).scroll {
                (*edit).scroll -= 1
            }
        }
        return;
    }
    if ch == 'a' as i32 - 'a' as i32 + 1 as libc::c_int {
        // ctrl-a is home
        (*edit).cursor = 0 as libc::c_int;
        (*edit).scroll = 0 as libc::c_int;
        return;
    }
    if ch == 'e' as i32 - 'a' as i32 + 1 as libc::c_int {
        // ctrl-e is end
        (*edit).cursor = len;
        (*edit).scroll = (*edit).cursor - (*edit).widthInChars + 1 as libc::c_int;
        if (*edit).scroll < 0 as libc::c_int {
            (*edit).scroll = 0 as libc::c_int
        }
        return;
    }
    //
    // ignore any other non printable chars
    //
    if ch < 32 as libc::c_int {
        return;
    }
    if crate::src::ui::ui_syscalls::trap_Key_GetOverstrikeMode() as u64 != 0 {
        if (*edit).cursor == 256 as libc::c_int - 1 as libc::c_int
            || (*edit).maxchars != 0 && (*edit).cursor >= (*edit).maxchars
        {
            return;
        }
    } else {
        // insert mode
        if len == 256 as libc::c_int - 1 as libc::c_int
            || (*edit).maxchars != 0 && len >= (*edit).maxchars
        {
            return;
        }
        crate::stdlib::memmove(
            (*edit)
                .buffer
                .as_mut_ptr()
                .offset((*edit).cursor as isize)
                .offset(1 as libc::c_int as isize) as *mut libc::c_void,
            (*edit).buffer.as_mut_ptr().offset((*edit).cursor as isize) as *const libc::c_void,
            (len + 1 as libc::c_int - (*edit).cursor) as libc::c_ulong,
        );
    }
    (*edit).buffer[(*edit).cursor as usize] = ch as libc::c_char;
    if (*edit).maxchars == 0 || (*edit).cursor < (*edit).maxchars - 1 as libc::c_int {
        (*edit).cursor += 1
    }
    if (*edit).cursor >= (*edit).widthInChars {
        (*edit).scroll += 1
    }
    if (*edit).cursor == len + 1 as libc::c_int {
        (*edit).buffer[(*edit).cursor as usize] = 0 as libc::c_int as libc::c_char
    };
}
/*
==================
MField_Clear
==================
*/
#[no_mangle]

pub unsafe extern "C" fn MField_Clear(mut edit: *mut crate::ui_local_h::mfield_t) {
    (*edit).buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    (*edit).cursor = 0 as libc::c_int;
    (*edit).scroll = 0 as libc::c_int;
}
/*
==================
MenuField_Init
==================
*/
#[no_mangle]

pub unsafe extern "C" fn MenuField_Init(mut m: *mut crate::ui_local_h::menufield_s) {
    let mut l: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    MField_Clear(&mut (*m).field);
    if (*m).generic.flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        w = 8 as libc::c_int;
        h = 16 as libc::c_int
    } else {
        w = 16 as libc::c_int;
        h = 16 as libc::c_int
    }
    if !(*m).generic.name.is_null() {
        l = crate::stdlib::strlen((*m).generic.name)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(w as libc::c_ulong) as libc::c_int
    } else {
        l = 0 as libc::c_int
    }
    (*m).generic.left = (*m).generic.x - l;
    (*m).generic.top = (*m).generic.y;
    (*m).generic.right = (*m).generic.x + w + (*m).field.widthInChars * w;
    (*m).generic.bottom = (*m).generic.y + h;
}
/*
==================
MenuField_Draw
==================
*/
#[no_mangle]

pub unsafe extern "C" fn MenuField_Draw(mut f: *mut crate::ui_local_h::menufield_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut focus: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    x = (*f).generic.x;
    y = (*f).generic.y;
    if (*f).generic.flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        w = 8 as libc::c_int;
        style = 0x10 as libc::c_int
    } else {
        w = 16 as libc::c_int;
        style = 0x20 as libc::c_int
    }
    if crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor(
        (*f).generic.parent as *mut crate::ui_local_h::_tag_menuframework,
    ) == f as *mut libc::c_void
    {
        focus = crate::src::qcommon::q_shared::qtrue;
        style |= 0x4000 as libc::c_int
    } else {
        focus = crate::src::qcommon::q_shared::qfalse
    }
    if (*f).generic.flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
        color = crate::src::q3_ui::ui_qmenu::text_color_disabled.as_mut_ptr()
    } else if focus as u64 != 0 {
        color = crate::src::q3_ui::ui_qmenu::text_color_highlight.as_mut_ptr()
    } else {
        color = crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr()
    }
    if focus as u64 != 0 {
        // draw cursor
        crate::src::q3_ui::ui_atoms::UI_FillRect(
            (*f).generic.left as libc::c_float,
            (*f).generic.top as libc::c_float,
            ((*f).generic.right - (*f).generic.left + 1 as libc::c_int) as libc::c_float,
            ((*f).generic.bottom - (*f).generic.top + 1 as libc::c_int) as libc::c_float,
            crate::src::q3_ui::ui_qmenu::listbar_color.as_mut_ptr(),
        );
        crate::src::q3_ui::ui_atoms::UI_DrawChar(
            x,
            y,
            13 as libc::c_int,
            0x1 as libc::c_int | 0x1000 as libc::c_int | style,
            color,
        );
    }
    if !(*f).generic.name.is_null() {
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            x - w,
            y,
            (*f).generic.name,
            style | 0x2 as libc::c_int,
            color,
        );
    }
    MField_Draw(&mut (*f).field, x + w, y, style, color);
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
/*
==================
MenuField_Key
==================
*/
#[no_mangle]

pub unsafe extern "C" fn MenuField_Key(
    mut m: *mut crate::ui_local_h::menufield_s,
    mut key: *mut libc::c_int,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    let mut keycode: libc::c_int = 0;
    keycode = *key;
    match keycode {
        169 | 13 | 185 | 186 | 187 | 188 => {
            // have enter go to next cursor point
            *key = crate::keycodes_h::K_TAB as libc::c_int
        }
        9 | 167 | 133 | 161 | 132 => {}
        _ => {
            if keycode & 1024 as libc::c_int != 0 {
                keycode &= !(1024 as libc::c_int);
                if (*m).generic.flags & 0x80000 as libc::c_int as libc::c_uint != 0
                    && crate::src::qcommon::q_shared::Q_islower(keycode) != 0
                {
                    keycode -= 'a' as i32 - 'A' as i32
                } else if (*m).generic.flags & 0x40000 as libc::c_int as libc::c_uint != 0
                    && crate::src::qcommon::q_shared::Q_isupper(keycode) != 0
                {
                    keycode -= 'A' as i32 - 'a' as i32
                } else if (*m).generic.flags & 0x20 as libc::c_int as libc::c_uint != 0
                    && crate::src::qcommon::q_shared::Q_isalpha(keycode) != 0
                {
                    return crate::src::q3_ui::ui_qmenu::menu_buzz_sound;
                }
                MField_CharEvent(&mut (*m).field, keycode);
            } else {
                MField_KeyDownEvent(&mut (*m).field, keycode);
            }
        }
    }
    return 0 as libc::c_int;
}

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
pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_CursorInRect;
pub use crate::src::q3_ui::ui_atoms::UI_DrawBannerString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawChar;
pub use crate::src::q3_ui::ui_atoms::UI_DrawHandlePic;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_FillRect;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_ProportionalSizeScale;
pub use crate::src::q3_ui::ui_atoms::UI_ProportionalStringWidth;
pub use crate::src::q3_ui::ui_atoms::UI_SetColor;
pub use crate::src::q3_ui::ui_mfield::MenuField_Draw;
pub use crate::src::q3_ui::ui_mfield::MenuField_Init;
pub use crate::src::q3_ui::ui_mfield::MenuField_Key;
pub use crate::src::qcommon::q_math::colorMdGrey;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Q_isprint;
pub use crate::src::qcommon::q_shared::Q_isupper;
pub use crate::src::ui::ui_syscalls::trap_Error;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_R_SetColor;
pub use crate::src::ui::ui_syscalls::trap_S_RegisterSound;

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
pub use crate::ui_local_h::menuaction_s;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menufield_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menuradiobutton_s;
pub use crate::ui_local_h::menuslider_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::mfield_t;
pub use crate::ui_local_h::uiStatic_t;

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
/* *********************************************************************
    UI_QMENU.C

    Quake's menu framework system.
**********************************************************************/
#[no_mangle]

pub static mut menu_in_sound: crate::src::qcommon::q_shared::sfxHandle_t = 0;
#[no_mangle]

pub static mut menu_move_sound: crate::src::qcommon::q_shared::sfxHandle_t = 0;
#[no_mangle]

pub static mut menu_out_sound: crate::src::qcommon::q_shared::sfxHandle_t = 0;
#[no_mangle]

pub static mut menu_buzz_sound: crate::src::qcommon::q_shared::sfxHandle_t = 0;
#[no_mangle]

pub static mut menu_null_sound: crate::src::qcommon::q_shared::sfxHandle_t = 0;
#[no_mangle]

pub static mut weaponChangeSound: crate::src::qcommon::q_shared::sfxHandle_t = 0;

static mut sliderBar: crate::src::qcommon::q_shared::qhandle_t = 0;

static mut sliderButton_0: crate::src::qcommon::q_shared::qhandle_t = 0;

static mut sliderButton_1: crate::src::qcommon::q_shared::qhandle_t = 0;
#[no_mangle]

pub static mut menu_text_color: crate::src::qcommon::q_shared::vec4_t =
    [1.0f32, 1.0f32, 1.0f32, 1.0f32];
#[no_mangle]

pub static mut menu_dim_color: crate::src::qcommon::q_shared::vec4_t =
    [0.0f32, 0.0f32, 0.0f32, 0.75f32];
#[no_mangle]

pub static mut color_black: crate::src::qcommon::q_shared::vec4_t =
    [0.00f32, 0.00f32, 0.00f32, 1.00f32];
#[no_mangle]

pub static mut color_white: crate::src::qcommon::q_shared::vec4_t =
    [1.00f32, 1.00f32, 1.00f32, 1.00f32];
#[no_mangle]

pub static mut color_yellow: crate::src::qcommon::q_shared::vec4_t =
    [1.00f32, 1.00f32, 0.00f32, 1.00f32];
#[no_mangle]

pub static mut color_blue: crate::src::qcommon::q_shared::vec4_t =
    [0.00f32, 0.00f32, 1.00f32, 1.00f32];
#[no_mangle]

pub static mut color_lightOrange: crate::src::qcommon::q_shared::vec4_t =
    [1.00f32, 0.68f32, 0.00f32, 1.00f32];
#[no_mangle]

pub static mut color_orange: crate::src::qcommon::q_shared::vec4_t =
    [1.00f32, 0.43f32, 0.00f32, 1.00f32];
#[no_mangle]

pub static mut color_red: crate::src::qcommon::q_shared::vec4_t =
    [1.00f32, 0.00f32, 0.00f32, 1.00f32];
#[no_mangle]

pub static mut color_dim: crate::src::qcommon::q_shared::vec4_t =
    [0.00f32, 0.00f32, 0.00f32, 0.25f32];
// current color scheme
#[no_mangle]

pub static mut pulse_color: crate::src::qcommon::q_shared::vec4_t =
    [1.00f32, 1.00f32, 1.00f32, 1.00f32];
#[no_mangle]

pub static mut text_color_disabled: crate::src::qcommon::q_shared::vec4_t =
    [0.50f32, 0.50f32, 0.50f32, 1.00f32];
// light gray
#[no_mangle]

pub static mut text_color_normal: crate::src::qcommon::q_shared::vec4_t =
    [1.00f32, 0.43f32, 0.00f32, 1.00f32];
// light orange
#[no_mangle]

pub static mut text_color_highlight: crate::src::qcommon::q_shared::vec4_t =
    [1.00f32, 1.00f32, 0.00f32, 1.00f32];
// bright yellow
#[no_mangle]

pub static mut listbar_color: crate::src::qcommon::q_shared::vec4_t =
    [1.00f32, 0.43f32, 0.00f32, 0.30f32];
// transluscent orange
#[no_mangle]

pub static mut text_color_status: crate::src::qcommon::q_shared::vec4_t =
    [1.00f32, 1.00f32, 1.00f32, 1.00f32];
// text widget
/*
=================
Text_Init
=================
*/

unsafe extern "C" fn Text_Init(mut t: *mut crate::ui_local_h::menutext_s) {
    (*t).generic.flags |= 0x4000 as libc::c_int as libc::c_uint;
}
/*
=================
Text_Draw
=================
*/

unsafe extern "C" fn Text_Draw(mut t: *mut crate::ui_local_h::menutext_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut buff: [libc::c_char; 512] = [0; 512];
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    x = (*t).generic.x;
    y = (*t).generic.y;
    buff[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    // possible label
    if !(*t).generic.name.is_null() {
        ::libc::strcpy(buff.as_mut_ptr(), (*t).generic.name);
    }
    // possible value
    if !(*t).string.is_null() {
        ::libc::strcat(buff.as_mut_ptr(), (*t).string);
    }
    if (*t).generic.flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
        color = text_color_disabled.as_mut_ptr()
    } else {
        color = (*t).color
    }
    crate::src::q3_ui::ui_atoms::UI_DrawString(x, y, buff.as_mut_ptr(), (*t).style, color);
}
// proportional banner text widget
/*
=================
BText_Init
=================
*/

unsafe extern "C" fn BText_Init(mut t: *mut crate::ui_local_h::menutext_s) {
    (*t).generic.flags |= 0x4000 as libc::c_int as libc::c_uint;
}
/*
=================
BText_Draw
=================
*/

unsafe extern "C" fn BText_Draw(mut t: *mut crate::ui_local_h::menutext_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    x = (*t).generic.x;
    y = (*t).generic.y;
    if (*t).generic.flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
        color = text_color_disabled.as_mut_ptr()
    } else {
        color = (*t).color
    }
    crate::src::q3_ui::ui_atoms::UI_DrawBannerString(x, y, (*t).string, (*t).style, color);
}
// proportional text widget
/*
=================
PText_Init
=================
*/

unsafe extern "C" fn PText_Init(mut t: *mut crate::ui_local_h::menutext_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut sizeScale: libc::c_float = 0.;
    sizeScale = crate::src::q3_ui::ui_atoms::UI_ProportionalSizeScale((*t).style);
    x = (*t).generic.x;
    y = (*t).generic.y;
    w = (crate::src::q3_ui::ui_atoms::UI_ProportionalStringWidth((*t).string) as libc::c_float
        * sizeScale) as libc::c_int;
    h = (27 as libc::c_int as libc::c_float * sizeScale) as libc::c_int;
    if (*t).generic.flags & 0x10 as libc::c_int as libc::c_uint != 0 {
        x -= w
    } else if (*t).generic.flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        x -= w / 2 as libc::c_int
    }
    (*t).generic.left =
        (x as libc::c_float - 3 as libc::c_int as libc::c_float * sizeScale) as libc::c_int;
    (*t).generic.right =
        ((x + w) as libc::c_float + 3 as libc::c_int as libc::c_float * sizeScale) as libc::c_int;
    (*t).generic.top = y;
    (*t).generic.bottom = y + h;
}
/*
=================
PText_Draw
=================
*/

unsafe extern "C" fn PText_Draw(mut t: *mut crate::ui_local_h::menutext_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut style: libc::c_int = 0;
    x = (*t).generic.x;
    y = (*t).generic.y;
    if (*t).generic.flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
        color = text_color_disabled.as_mut_ptr()
    } else {
        color = (*t).color
    }
    style = (*t).style;
    if (*t).generic.flags & 0x100 as libc::c_int as libc::c_uint != 0 {
        if Menu_ItemAtCursor((*t).generic.parent) == t as *mut libc::c_void {
            style |= 0x4000 as libc::c_int
        } else {
            style |= 0x2000 as libc::c_int
        }
    }
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(x, y, (*t).string, style, color);
}
/*
=================
Bitmap_Init
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Bitmap_Init(mut b: *mut crate::ui_local_h::menubitmap_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    x = (*b).generic.x;
    y = (*b).generic.y;
    w = (*b).width;
    h = (*b).height;
    if w < 0 as libc::c_int {
        w = -w
    }
    if h < 0 as libc::c_int {
        h = -h
    }
    if (*b).generic.flags & 0x10 as libc::c_int as libc::c_uint != 0 {
        x = x - w
    } else if (*b).generic.flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        x = x - w / 2 as libc::c_int
    }
    (*b).generic.left = x;
    (*b).generic.right = x + w;
    (*b).generic.top = y;
    (*b).generic.bottom = y + h;
    (*b).shader = 0 as libc::c_int;
    (*b).focusshader = 0 as libc::c_int;
}
/*
=================
Bitmap_Draw
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Bitmap_Draw(mut b: *mut crate::ui_local_h::menubitmap_s) {
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    let mut tempcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    x = (*b).generic.x as libc::c_float;
    y = (*b).generic.y as libc::c_float;
    w = (*b).width as libc::c_float;
    h = (*b).height as libc::c_float;
    if (*b).generic.flags & 0x10 as libc::c_int as libc::c_uint != 0 {
        x = x - w
    } else if (*b).generic.flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        x = x - w / 2 as libc::c_int as libc::c_float
    }
    // used to refresh shader
    if !(*b).generic.name.is_null() && (*b).shader == 0 {
        (*b).shader = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip((*b).generic.name);
        if (*b).shader == 0 && !(*b).errorpic.is_null() {
            (*b).shader = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip((*b).errorpic)
        }
    }
    if !(*b).focuspic.is_null() && (*b).focusshader == 0 {
        (*b).focusshader = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip((*b).focuspic)
    }
    if (*b).generic.flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
        if (*b).shader != 0 {
            crate::src::ui::ui_syscalls::trap_R_SetColor(
                crate::src::qcommon::q_math::colorMdGrey.as_mut_ptr(),
            );
            crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(x, y, w, h, (*b).shader);
            crate::src::ui::ui_syscalls::trap_R_SetColor(0 as *const libc::c_float);
        }
    } else {
        if (*b).shader != 0 {
            crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(x, y, w, h, (*b).shader);
        }
        if ((*b).generic.flags & 0x20000 as libc::c_int as libc::c_uint != 0
            || (*b).generic.flags & 0x100 as libc::c_int as libc::c_uint != 0)
            && Menu_ItemAtCursor((*b).generic.parent) == b as *mut libc::c_void
        {
            if !(*b).focuscolor.is_null() {
                tempcolor[0 as libc::c_int as usize] =
                    *(*b).focuscolor.offset(0 as libc::c_int as isize);
                tempcolor[1 as libc::c_int as usize] =
                    *(*b).focuscolor.offset(1 as libc::c_int as isize);
                tempcolor[2 as libc::c_int as usize] =
                    *(*b).focuscolor.offset(2 as libc::c_int as isize);
                color = tempcolor.as_mut_ptr()
            } else {
                color = pulse_color.as_mut_ptr()
            }
            *color.offset(3 as libc::c_int as isize) = (0.5f64
                + 0.5f64
                    * crate::stdlib::sin(
                        (crate::src::q3_ui::ui_atoms::uis.realtime / 75 as libc::c_int)
                            as libc::c_double,
                    )) as libc::c_float;
            crate::src::ui::ui_syscalls::trap_R_SetColor(color);
            crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(x, y, w, h, (*b).focusshader);
            crate::src::ui::ui_syscalls::trap_R_SetColor(0 as *const libc::c_float);
        } else if (*b).generic.flags & 0x40 as libc::c_int as libc::c_uint != 0
            || (*b).generic.flags & 0x80 as libc::c_int as libc::c_uint != 0
                && Menu_ItemAtCursor((*b).generic.parent) == b as *mut libc::c_void
        {
            if !(*b).focuscolor.is_null() {
                crate::src::ui::ui_syscalls::trap_R_SetColor((*b).focuscolor);
                crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(x, y, w, h, (*b).focusshader);
                crate::src::ui::ui_syscalls::trap_R_SetColor(0 as *const libc::c_float);
            } else {
                crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(x, y, w, h, (*b).focusshader);
            }
        }
    };
}
// bright white
// action widget
/*
=================
Action_Init
=================
*/

unsafe extern "C" fn Action_Init(mut a: *mut crate::ui_local_h::menuaction_s) {
    let mut len: libc::c_int = 0;
    // calculate bounds
    if !(*a).generic.name.is_null() {
        len = crate::stdlib::strlen((*a).generic.name) as libc::c_int
    } else {
        len = 0 as libc::c_int
    }
    // left justify text
    (*a).generic.left = (*a).generic.x;
    (*a).generic.right = (*a).generic.x + len * 16 as libc::c_int;
    (*a).generic.top = (*a).generic.y;
    (*a).generic.bottom = (*a).generic.y + 16 as libc::c_int;
}
/*
=================
Action_Draw
=================
*/

unsafe extern "C" fn Action_Draw(mut a: *mut crate::ui_local_h::menuaction_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    style = 0 as libc::c_int;
    color = menu_text_color.as_mut_ptr();
    if (*a).generic.flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
        color = text_color_disabled.as_mut_ptr()
    } else if (*a).generic.flags & 0x100 as libc::c_int as libc::c_uint != 0
        && (*(*a).generic.parent).cursor == (*a).generic.menuPosition
    {
        color = text_color_highlight.as_mut_ptr();
        style = 0x4000 as libc::c_int
    } else if (*a).generic.flags & 0x80 as libc::c_int as libc::c_uint != 0
        && (*(*a).generic.parent).cursor == (*a).generic.menuPosition
    {
        color = text_color_highlight.as_mut_ptr()
    } else if (*a).generic.flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        style = 0x1000 as libc::c_int;
        color = text_color_highlight.as_mut_ptr()
    }
    x = (*a).generic.x;
    y = (*a).generic.y;
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        x,
        y,
        (*a).generic.name,
        0 as libc::c_int | style,
        color,
    );
    if (*(*a).generic.parent).cursor == (*a).generic.menuPosition {
        // draw cursor
        crate::src::q3_ui::ui_atoms::UI_DrawChar(
            x - 16 as libc::c_int,
            y,
            13 as libc::c_int,
            0 as libc::c_int | 0x1000 as libc::c_int,
            color,
        );
    };
}
// radio button widget
/*
=================
RadioButton_Init
=================
*/

unsafe extern "C" fn RadioButton_Init(mut rb: *mut crate::ui_local_h::menuradiobutton_s) {
    let mut len: libc::c_int = 0;
    // calculate bounds
    if !(*rb).generic.name.is_null() {
        len = crate::stdlib::strlen((*rb).generic.name) as libc::c_int
    } else {
        len = 0 as libc::c_int
    }
    (*rb).generic.left = (*rb).generic.x - (len + 1 as libc::c_int) * 8 as libc::c_int;
    (*rb).generic.right = (*rb).generic.x + 6 as libc::c_int * 8 as libc::c_int;
    (*rb).generic.top = (*rb).generic.y;
    (*rb).generic.bottom = (*rb).generic.y + 16 as libc::c_int;
}
/*
=================
RadioButton_Key
=================
*/

unsafe extern "C" fn RadioButton_Key(
    mut rb: *mut crate::ui_local_h::menuradiobutton_s,
    mut key: libc::c_int,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    let mut current_block_3: u64;
    match key {
        178 => {
            if (*rb).generic.flags & 0x200 as libc::c_int as libc::c_uint == 0 {
                current_block_3 = 3640593987805443782;
            } else {
                current_block_3 = 3096415330105608827;
            }
        }
        185 | 186 | 187 | 188 | 13 | 169 | 163 | 134 | 165 | 135 => {
            current_block_3 = 3096415330105608827;
        }
        _ => {
            current_block_3 = 3640593987805443782;
        }
    }
    match current_block_3 {
        3640593987805443782 => {}
        _ => {
            (*rb).curvalue = ((*rb).curvalue == 0) as libc::c_int;
            if (*rb).generic.callback.is_some() {
                (*rb).generic.callback.expect("non-null function pointer")(
                    rb as *mut libc::c_void,
                    3 as libc::c_int,
                );
            }
            return menu_move_sound;
        }
    }
    // key not handled
    return 0 as libc::c_int;
}
/*
=================
RadioButton_Draw
=================
*/

unsafe extern "C" fn RadioButton_Draw(mut rb: *mut crate::ui_local_h::menuradiobutton_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut style: libc::c_int = 0;
    let mut focus: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    x = (*rb).generic.x;
    y = (*rb).generic.y;
    focus = ((*(*rb).generic.parent).cursor == (*rb).generic.menuPosition) as libc::c_int
        as crate::src::qcommon::q_shared::qboolean;
    if (*rb).generic.flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
        color = text_color_disabled.as_mut_ptr();
        style = 0 as libc::c_int | 0x10 as libc::c_int
    } else if focus as u64 != 0 {
        color = text_color_highlight.as_mut_ptr();
        style = 0 as libc::c_int | 0x4000 as libc::c_int | 0x10 as libc::c_int
    } else {
        color = text_color_normal.as_mut_ptr();
        style = 0 as libc::c_int | 0x10 as libc::c_int
    }
    if focus as u64 != 0 {
        // draw cursor
        crate::src::q3_ui::ui_atoms::UI_FillRect(
            (*rb).generic.left as libc::c_float,
            (*rb).generic.top as libc::c_float,
            ((*rb).generic.right - (*rb).generic.left + 1 as libc::c_int) as libc::c_float,
            ((*rb).generic.bottom - (*rb).generic.top + 1 as libc::c_int) as libc::c_float,
            listbar_color.as_mut_ptr(),
        );
        crate::src::q3_ui::ui_atoms::UI_DrawChar(
            x,
            y,
            13 as libc::c_int,
            0x1 as libc::c_int | 0x1000 as libc::c_int | 0x10 as libc::c_int,
            color,
        );
    }
    if !(*rb).generic.name.is_null() {
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            x - 8 as libc::c_int,
            y,
            (*rb).generic.name,
            0x2 as libc::c_int | 0x10 as libc::c_int,
            color,
        );
    }
    if (*rb).curvalue == 0 {
        crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
            (x + 8 as libc::c_int) as libc::c_float,
            (y + 2 as libc::c_int) as libc::c_float,
            16 as libc::c_int as libc::c_float,
            16 as libc::c_int as libc::c_float,
            crate::src::q3_ui::ui_atoms::uis.rb_off,
        );
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            x + 8 as libc::c_int + 16 as libc::c_int,
            y,
            b"off\x00" as *const u8 as *const libc::c_char,
            style,
            color,
        );
    } else {
        crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
            (x + 8 as libc::c_int) as libc::c_float,
            (y + 2 as libc::c_int) as libc::c_float,
            16 as libc::c_int as libc::c_float,
            16 as libc::c_int as libc::c_float,
            crate::src::q3_ui::ui_atoms::uis.rb_on,
        );
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            x + 8 as libc::c_int + 16 as libc::c_int,
            y,
            b"on\x00" as *const u8 as *const libc::c_char,
            style,
            color,
        );
    };
}
// slider widget
/*
=================
Slider_Init
=================
*/

unsafe extern "C" fn Slider_Init(mut s: *mut crate::ui_local_h::menuslider_s) {
    let mut len: libc::c_int = 0;
    // calculate bounds
    if !(*s).generic.name.is_null() {
        len = crate::stdlib::strlen((*s).generic.name) as libc::c_int
    } else {
        len = 0 as libc::c_int
    }
    (*s).generic.left = (*s).generic.x - (len + 1 as libc::c_int) * 8 as libc::c_int;
    (*s).generic.right = (*s).generic.x
        + (10 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int) * 8 as libc::c_int;
    (*s).generic.top = (*s).generic.y;
    (*s).generic.bottom = (*s).generic.y + 16 as libc::c_int;
}
/*
=================
Slider_Key
=================
*/

unsafe extern "C" fn Slider_Key(
    mut s: *mut crate::ui_local_h::menuslider_s,
    mut key: libc::c_int,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    let mut sound: crate::src::qcommon::q_shared::sfxHandle_t = 0;
    let mut x: libc::c_int = 0;
    let mut oldvalue: libc::c_int = 0;
    match key {
        178 => {
            x = crate::src::q3_ui::ui_atoms::uis.cursorx
                - (*s).generic.x
                - 2 as libc::c_int * 8 as libc::c_int;
            oldvalue = (*s).curvalue as libc::c_int;
            (*s).curvalue = x as libc::c_float
                / (10 as libc::c_int * 8 as libc::c_int) as libc::c_float
                * ((*s).maxvalue - (*s).minvalue)
                + (*s).minvalue;
            if (*s).curvalue < (*s).minvalue {
                (*s).curvalue = (*s).minvalue
            } else if (*s).curvalue > (*s).maxvalue {
                (*s).curvalue = (*s).maxvalue
            }
            if (*s).curvalue != oldvalue as libc::c_float {
                sound = menu_move_sound
            } else {
                sound = 0 as libc::c_int
            }
        }
        163 | 134 => {
            if (*s).curvalue > (*s).minvalue {
                (*s).curvalue -= 1.;
                sound = menu_move_sound
            } else {
                sound = menu_buzz_sound
            }
        }
        165 | 135 => {
            if (*s).curvalue < (*s).maxvalue {
                (*s).curvalue += 1.;
                sound = menu_move_sound
            } else {
                sound = menu_buzz_sound
            }
        }
        _ => {
            // key not handled
            sound = 0 as libc::c_int
        }
    }
    if sound != 0 && (*s).generic.callback.is_some() {
        (*s).generic.callback.expect("non-null function pointer")(
            s as *mut libc::c_void,
            3 as libc::c_int,
        );
    }
    return sound;
}
/*
=================
Slider_Draw
=================
*/

unsafe extern "C" fn Slider_Draw(mut s: *mut crate::ui_local_h::menuslider_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut button: libc::c_int = 0;
    let mut focus: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    x = (*s).generic.x;
    y = (*s).generic.y;
    focus = ((*(*s).generic.parent).cursor == (*s).generic.menuPosition) as libc::c_int
        as crate::src::qcommon::q_shared::qboolean;
    if (*s).generic.flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
        color = text_color_disabled.as_mut_ptr();
        style = 0x10 as libc::c_int
    } else if focus as u64 != 0 {
        color = text_color_highlight.as_mut_ptr();
        style = 0x10 as libc::c_int | 0x4000 as libc::c_int
    } else {
        color = text_color_normal.as_mut_ptr();
        style = 0x10 as libc::c_int
    }
    // draw label
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        x - 8 as libc::c_int,
        y,
        (*s).generic.name,
        0x2 as libc::c_int | style,
        color,
    );
    // draw slider
    crate::src::q3_ui::ui_atoms::UI_SetColor(color);
    crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
        (x + 8 as libc::c_int) as libc::c_float,
        y as libc::c_float,
        96 as libc::c_int as libc::c_float,
        16 as libc::c_int as libc::c_float,
        sliderBar,
    );
    crate::src::q3_ui::ui_atoms::UI_SetColor(0 as *const libc::c_float);
    // clamp thumb
    if (*s).maxvalue > (*s).minvalue {
        (*s).range = ((*s).curvalue - (*s).minvalue) / ((*s).maxvalue - (*s).minvalue);
        if (*s).range < 0 as libc::c_int as libc::c_float {
            (*s).range = 0 as libc::c_int as libc::c_float
        } else if (*s).range > 1 as libc::c_int as libc::c_float {
            (*s).range = 1 as libc::c_int as libc::c_float
        }
    } else {
        (*s).range = 0 as libc::c_int as libc::c_float
    }
    // draw thumb
    if style & 0x4000 as libc::c_int != 0 {
        button = sliderButton_1
    } else {
        button = sliderButton_0
    }
    crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
        (((x + 2 as libc::c_int * 8 as libc::c_int) as libc::c_float
            + ((10 as libc::c_int - 1 as libc::c_int) * 8 as libc::c_int) as libc::c_float
                * (*s).range) as libc::c_int
            - 2 as libc::c_int) as libc::c_float,
        (y - 2 as libc::c_int) as libc::c_float,
        12 as libc::c_int as libc::c_float,
        20 as libc::c_int as libc::c_float,
        button,
    );
}
// spin control widget
/*
=================
SpinControl_Init
=================
*/

unsafe extern "C" fn SpinControl_Init(mut s: *mut crate::ui_local_h::menulist_s) {
    let mut len: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    if !(*s).generic.name.is_null() {
        len = crate::stdlib::strlen((*s).generic.name)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int
    } else {
        len = 0 as libc::c_int
    }
    (*s).generic.left = (*s).generic.x - 8 as libc::c_int - len;
    (*s).numitems = 0 as libc::c_int;
    len = (*s).numitems;
    loop {
        str = *(*s).itemnames.offset((*s).numitems as isize);
        if str.is_null() {
            break;
        }
        l = crate::stdlib::strlen(str) as libc::c_int;
        if l > len {
            len = l
        }
        (*s).numitems += 1
    }
    (*s).generic.top = (*s).generic.y;
    (*s).generic.right = (*s).generic.x + (len + 1 as libc::c_int) * 8 as libc::c_int;
    (*s).generic.bottom = (*s).generic.y + 16 as libc::c_int;
}
/*
=================
SpinControl_Key
=================
*/

unsafe extern "C" fn SpinControl_Key(
    mut s: *mut crate::ui_local_h::menulist_s,
    mut key: libc::c_int,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    let mut sound: crate::src::qcommon::q_shared::sfxHandle_t = 0;
    sound = 0 as libc::c_int;
    match key {
        165 | 135 | 178 => {
            (*s).curvalue += 1;
            if (*s).curvalue >= (*s).numitems {
                (*s).curvalue = 0 as libc::c_int
            }
            sound = menu_move_sound
        }
        163 | 134 => {
            (*s).curvalue -= 1;
            if (*s).curvalue < 0 as libc::c_int {
                (*s).curvalue = (*s).numitems - 1 as libc::c_int
            }
            sound = menu_move_sound
        }
        _ => {}
    }
    if sound != 0 && (*s).generic.callback.is_some() {
        (*s).generic.callback.expect("non-null function pointer")(
            s as *mut libc::c_void,
            3 as libc::c_int,
        );
    }
    return sound;
}
/*
=================
SpinControl_Draw
=================
*/

unsafe extern "C" fn SpinControl_Draw(mut s: *mut crate::ui_local_h::menulist_s) {
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut focus: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    x = (*s).generic.x;
    y = (*s).generic.y;
    style = 0x10 as libc::c_int;
    focus = ((*(*s).generic.parent).cursor == (*s).generic.menuPosition) as libc::c_int
        as crate::src::qcommon::q_shared::qboolean;
    if (*s).generic.flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
        color = text_color_disabled.as_mut_ptr()
    } else if focus as u64 != 0 {
        color = text_color_highlight.as_mut_ptr();
        style |= 0x4000 as libc::c_int
    } else if (*s).generic.flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        color = text_color_highlight.as_mut_ptr();
        style |= 0x1000 as libc::c_int
    } else {
        color = text_color_normal.as_mut_ptr()
    }
    if focus as u64 != 0 {
        // draw cursor
        crate::src::q3_ui::ui_atoms::UI_FillRect(
            (*s).generic.left as libc::c_float,
            (*s).generic.top as libc::c_float,
            ((*s).generic.right - (*s).generic.left + 1 as libc::c_int) as libc::c_float,
            ((*s).generic.bottom - (*s).generic.top + 1 as libc::c_int) as libc::c_float,
            listbar_color.as_mut_ptr(),
        );
        crate::src::q3_ui::ui_atoms::UI_DrawChar(
            x,
            y,
            13 as libc::c_int,
            0x1 as libc::c_int | 0x1000 as libc::c_int | 0x10 as libc::c_int,
            color,
        );
    }
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        x - 8 as libc::c_int,
        y,
        (*s).generic.name,
        style | 0x2 as libc::c_int,
        color,
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        x + 8 as libc::c_int,
        y,
        *(*s).itemnames.offset((*s).curvalue as isize),
        style | 0 as libc::c_int,
        color,
    );
}
// scrolllist widget
/*
=================
ScrollList_Init
=================
*/

unsafe extern "C" fn ScrollList_Init(mut l: *mut crate::ui_local_h::menulist_s) {
    let mut w: libc::c_int = 0;
    (*l).oldvalue = 0 as libc::c_int;
    (*l).curvalue = 0 as libc::c_int;
    (*l).top = 0 as libc::c_int;
    if (*l).columns == 0 {
        (*l).columns = 1 as libc::c_int;
        (*l).separation = 0 as libc::c_int
    } else if (*l).separation == 0 {
        (*l).separation = 3 as libc::c_int
    }
    w = (((*l).width + (*l).separation) * (*l).columns - (*l).separation) * 8 as libc::c_int;
    (*l).generic.left = (*l).generic.x;
    (*l).generic.top = (*l).generic.y;
    (*l).generic.right = (*l).generic.x + w;
    (*l).generic.bottom = (*l).generic.y + (*l).height * 16 as libc::c_int;
    if (*l).generic.flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        (*l).generic.left -= w / 2 as libc::c_int;
        (*l).generic.right -= w / 2 as libc::c_int
    };
}
/*
=================
ScrollList_Key
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ScrollList_Key(
    mut l: *mut crate::ui_local_h::menulist_s,
    mut key: libc::c_int,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cursorx: libc::c_int = 0;
    let mut cursory: libc::c_int = 0;
    let mut column: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    match key {
        178 => {
            if (*l).generic.flags & 0x200 as libc::c_int as libc::c_uint != 0 {
                // check scroll region
                x = (*l).generic.x;
                y = (*l).generic.y;
                w = (((*l).width + (*l).separation) * (*l).columns - (*l).separation)
                    * 8 as libc::c_int;
                if (*l).generic.flags & 0x8 as libc::c_int as libc::c_uint != 0 {
                    x -= w / 2 as libc::c_int
                }
                if crate::src::q3_ui::ui_atoms::UI_CursorInRect(
                    x,
                    y,
                    w,
                    (*l).height * 16 as libc::c_int,
                ) as u64
                    != 0
                {
                    cursorx = (crate::src::q3_ui::ui_atoms::uis.cursorx - x) / 8 as libc::c_int;
                    column = cursorx / ((*l).width + (*l).separation);
                    cursory = (crate::src::q3_ui::ui_atoms::uis.cursory - y) / 16 as libc::c_int;
                    index = column * (*l).height + cursory;
                    if (*l).top + index < (*l).numitems {
                        (*l).oldvalue = (*l).curvalue;
                        (*l).curvalue = (*l).top + index;
                        if (*l).oldvalue != (*l).curvalue && (*l).generic.callback.is_some() {
                            (*l).generic.callback.expect("non-null function pointer")(
                                l as *mut libc::c_void,
                                1 as libc::c_int,
                            );
                            return menu_move_sound;
                        }
                    }
                }
                // absorbed, silent sound effect
                return menu_null_sound;
            }
        }
        160 | 143 => {
            (*l).oldvalue = (*l).curvalue;
            (*l).curvalue = 0 as libc::c_int;
            (*l).top = 0 as libc::c_int;
            if (*l).oldvalue != (*l).curvalue && (*l).generic.callback.is_some() {
                (*l).generic.callback.expect("non-null function pointer")(
                    l as *mut libc::c_void,
                    1 as libc::c_int,
                );
                return menu_move_sound;
            }
            return menu_buzz_sound;
        }
        166 | 144 => {
            (*l).oldvalue = (*l).curvalue;
            (*l).curvalue = (*l).numitems - 1 as libc::c_int;
            if (*l).columns > 1 as libc::c_int {
                c = ((*l).curvalue / (*l).height + 1 as libc::c_int) * (*l).height;
                (*l).top = c - (*l).columns * (*l).height
            } else {
                (*l).top = (*l).curvalue - ((*l).height - 1 as libc::c_int)
            }
            if (*l).top < 0 as libc::c_int {
                (*l).top = 0 as libc::c_int
            }
            if (*l).oldvalue != (*l).curvalue && (*l).generic.callback.is_some() {
                (*l).generic.callback.expect("non-null function pointer")(
                    l as *mut libc::c_void,
                    1 as libc::c_int,
                );
                return menu_move_sound;
            }
            return menu_buzz_sound;
        }
        142 | 162 => {
            if (*l).columns > 1 as libc::c_int {
                return menu_null_sound;
            }
            if (*l).curvalue > 0 as libc::c_int {
                (*l).oldvalue = (*l).curvalue;
                (*l).curvalue -= (*l).height - 1 as libc::c_int;
                if (*l).curvalue < 0 as libc::c_int {
                    (*l).curvalue = 0 as libc::c_int
                }
                (*l).top = (*l).curvalue;
                if (*l).top < 0 as libc::c_int {
                    (*l).top = 0 as libc::c_int
                }
                if (*l).generic.callback.is_some() {
                    (*l).generic.callback.expect("non-null function pointer")(
                        l as *mut libc::c_void,
                        1 as libc::c_int,
                    );
                }
                return menu_move_sound;
            }
            return menu_buzz_sound;
        }
        141 | 168 => {
            if (*l).columns > 1 as libc::c_int {
                return menu_null_sound;
            }
            if (*l).curvalue < (*l).numitems - 1 as libc::c_int {
                (*l).oldvalue = (*l).curvalue;
                (*l).curvalue += (*l).height - 1 as libc::c_int;
                if (*l).curvalue > (*l).numitems - 1 as libc::c_int {
                    (*l).curvalue = (*l).numitems - 1 as libc::c_int
                }
                (*l).top = (*l).curvalue - ((*l).height - 1 as libc::c_int);
                if (*l).top < 0 as libc::c_int {
                    (*l).top = 0 as libc::c_int
                }
                if (*l).generic.callback.is_some() {
                    (*l).generic.callback.expect("non-null function pointer")(
                        l as *mut libc::c_void,
                        1 as libc::c_int,
                    );
                }
                return menu_move_sound;
            }
            return menu_buzz_sound;
        }
        184 => {
            if (*l).columns > 1 as libc::c_int {
                return menu_null_sound;
            }
            if (*l).top > 0 as libc::c_int {
                // if scrolling 3 lines would replace over half of the
                // displayed items, only scroll 1 item at a time.
                let mut scroll: libc::c_int = if (*l).height < 6 as libc::c_int {
                    1 as libc::c_int
                } else {
                    3 as libc::c_int
                };
                (*l).top -= scroll;
                if (*l).top < 0 as libc::c_int {
                    (*l).top = 0 as libc::c_int
                }
                if (*l).generic.callback.is_some() {
                    (*l).generic.callback.expect("non-null function pointer")(
                        l as *mut libc::c_void,
                        1 as libc::c_int,
                    );
                }
                // make scrolling silent
                return menu_null_sound;
            }
            return menu_buzz_sound;
        }
        183 => {
            if (*l).columns > 1 as libc::c_int {
                return menu_null_sound;
            }
            if (*l).top < (*l).numitems - (*l).height {
                // if scrolling 3 items would replace over half of the
                // displayed items, only scroll 1 item at a time.
                let mut scroll_0: libc::c_int = if (*l).height < 6 as libc::c_int {
                    1 as libc::c_int
                } else {
                    3 as libc::c_int
                };
                (*l).top += scroll_0;
                if (*l).top > (*l).numitems - (*l).height {
                    (*l).top = (*l).numitems - (*l).height
                }
                if (*l).generic.callback.is_some() {
                    (*l).generic.callback.expect("non-null function pointer")(
                        l as *mut libc::c_void,
                        1 as libc::c_int,
                    );
                }
                // make scrolling silent
                return menu_null_sound;
            }
            return menu_buzz_sound;
        }
        161 | 132 => {
            if (*l).curvalue == 0 as libc::c_int {
                return menu_buzz_sound;
            }
            (*l).oldvalue = (*l).curvalue;
            (*l).curvalue -= 1;
            if (*l).curvalue < (*l).top {
                if (*l).columns == 1 as libc::c_int {
                    (*l).top -= 1
                } else {
                    (*l).top -= (*l).height
                }
            }
            if (*l).generic.callback.is_some() {
                (*l).generic.callback.expect("non-null function pointer")(
                    l as *mut libc::c_void,
                    1 as libc::c_int,
                );
            }
            return menu_move_sound;
        }
        167 | 133 => {
            if (*l).curvalue == (*l).numitems - 1 as libc::c_int {
                return menu_buzz_sound;
            }
            (*l).oldvalue = (*l).curvalue;
            (*l).curvalue += 1;
            if (*l).curvalue >= (*l).top + (*l).columns * (*l).height {
                if (*l).columns == 1 as libc::c_int {
                    (*l).top += 1
                } else {
                    (*l).top += (*l).height
                }
            }
            if (*l).generic.callback.is_some() {
                (*l).generic.callback.expect("non-null function pointer")(
                    l as *mut libc::c_void,
                    1 as libc::c_int,
                );
            }
            return menu_move_sound;
        }
        163 | 134 => {
            if (*l).columns == 1 as libc::c_int {
                return menu_null_sound;
            }
            if (*l).curvalue < (*l).height {
                return menu_buzz_sound;
            }
            (*l).oldvalue = (*l).curvalue;
            (*l).curvalue -= (*l).height;
            if (*l).curvalue < (*l).top {
                (*l).top -= (*l).height
            }
            if (*l).generic.callback.is_some() {
                (*l).generic.callback.expect("non-null function pointer")(
                    l as *mut libc::c_void,
                    1 as libc::c_int,
                );
            }
            return menu_move_sound;
        }
        165 | 135 => {
            if (*l).columns == 1 as libc::c_int {
                return menu_null_sound;
            }
            c = (*l).curvalue + (*l).height;
            if c >= (*l).numitems {
                return menu_buzz_sound;
            }
            (*l).oldvalue = (*l).curvalue;
            (*l).curvalue = c;
            if (*l).curvalue > (*l).top + (*l).columns * (*l).height - 1 as libc::c_int {
                (*l).top += (*l).height
            }
            if (*l).generic.callback.is_some() {
                (*l).generic.callback.expect("non-null function pointer")(
                    l as *mut libc::c_void,
                    1 as libc::c_int,
                );
            }
            return menu_move_sound;
        }
        _ => {}
    }
    // cycle look for ascii key inside list items
    if crate::src::qcommon::q_shared::Q_isprint(key) == 0 {
        return 0 as libc::c_int;
    }
    // force to lower for case insensitive compare
    if crate::src::qcommon::q_shared::Q_isupper(key) != 0 {
        key -= 'A' as i32 - 'a' as i32
    }
    // iterate list items
    i = 1 as libc::c_int;
    while i <= (*l).numitems {
        j = ((*l).curvalue + i) % (*l).numitems;
        c = *(*(*l).itemnames.offset(j as isize)).offset(0 as libc::c_int as isize) as libc::c_int;
        if crate::src::qcommon::q_shared::Q_isupper(c) != 0 {
            c -= 'A' as i32 - 'a' as i32
        }
        if c == key {
            // set current item, mimic windows listbox scroll behavior
            if j < (*l).top {
                // behind top most item, set this as new top
                (*l).top = j
            } else if j > (*l).top + (*l).height - 1 as libc::c_int {
                // past end of list box, do page down
                (*l).top = j + 1 as libc::c_int - (*l).height
            }
            if (*l).curvalue != j {
                (*l).oldvalue = (*l).curvalue;
                (*l).curvalue = j;
                if (*l).generic.callback.is_some() {
                    (*l).generic.callback.expect("non-null function pointer")(
                        l as *mut libc::c_void,
                        1 as libc::c_int,
                    );
                }
                return menu_move_sound;
            }
            return menu_buzz_sound;
        }
        i += 1
    }
    return menu_buzz_sound;
}
/*
=================
ScrollList_Draw
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ScrollList_Draw(mut l: *mut crate::ui_local_h::menulist_s) {
    let mut x: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    let mut column: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut hasfocus: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut style: libc::c_int = 0;
    hasfocus = ((*(*l).generic.parent).cursor == (*l).generic.menuPosition) as libc::c_int
        as crate::src::qcommon::q_shared::qboolean;
    x = (*l).generic.x;
    column = 0 as libc::c_int;
    while column < (*l).columns {
        y = (*l).generic.y;
        base = (*l).top + column * (*l).height;
        i = base;
        while i < base + (*l).height {
            if i >= (*l).numitems {
                break;
            }
            if i == (*l).curvalue {
                u = x - 2 as libc::c_int;
                if (*l).generic.flags & 0x8 as libc::c_int as libc::c_uint != 0 {
                    u -= (*l).width * 8 as libc::c_int / 2 as libc::c_int + 1 as libc::c_int
                }
                crate::src::q3_ui::ui_atoms::UI_FillRect(
                    u as libc::c_float,
                    y as libc::c_float,
                    ((*l).width * 8 as libc::c_int) as libc::c_float,
                    (16 as libc::c_int + 2 as libc::c_int) as libc::c_float,
                    listbar_color.as_mut_ptr(),
                );
                color = text_color_highlight.as_mut_ptr();
                if hasfocus as u64 != 0 {
                    style = 0x4000 as libc::c_int | 0 as libc::c_int | 0x10 as libc::c_int
                } else {
                    style = 0 as libc::c_int | 0x10 as libc::c_int
                }
            } else {
                color = text_color_normal.as_mut_ptr();
                style = 0 as libc::c_int | 0x10 as libc::c_int
            }
            if (*l).generic.flags & 0x8 as libc::c_int as libc::c_uint != 0 {
                style |= 0x1 as libc::c_int
            }
            crate::src::q3_ui::ui_atoms::UI_DrawString(
                x,
                y,
                *(*l).itemnames.offset(i as isize),
                style,
                color,
            );
            y += 16 as libc::c_int;
            i += 1
        }
        x += ((*l).width + (*l).separation) * 8 as libc::c_int;
        column += 1
    }
}
/*
=================
Menu_AddItem
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Menu_AddItem(
    mut menu: *mut crate::ui_local_h::menuframework_s,
    mut item: *mut libc::c_void,
) {
    let mut itemptr: *mut crate::ui_local_h::menucommon_s =
        0 as *mut crate::ui_local_h::menucommon_s;
    if (*menu).nitems >= 64 as libc::c_int {
        crate::src::ui::ui_syscalls::trap_Error(
            b"Menu_AddItem: excessive items\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*menu).items[(*menu).nitems as usize] = item;
    let ref mut fresh0 =
        (*((*menu).items[(*menu).nitems as usize] as *mut crate::ui_local_h::menucommon_s)).parent;
    *fresh0 = menu;
    (*((*menu).items[(*menu).nitems as usize] as *mut crate::ui_local_h::menucommon_s))
        .menuPosition = (*menu).nitems;
    (*((*menu).items[(*menu).nitems as usize] as *mut crate::ui_local_h::menucommon_s)).flags &=
        !(0x200 as libc::c_int as libc::c_uint);
    // perform any item specific initializations
    itemptr = item as *mut crate::ui_local_h::menucommon_s;
    if (*itemptr).flags & 0x8000 as libc::c_int as libc::c_uint == 0 {
        match (*itemptr).type_0 {
            2 => {
                Action_Init(item as *mut crate::ui_local_h::menuaction_s);
            }
            4 => {
                crate::src::q3_ui::ui_mfield::MenuField_Init(
                    item as *mut crate::ui_local_h::menufield_s
                        as *mut crate::ui_local_h::menufield_s,
                );
            }
            3 => {
                SpinControl_Init(item as *mut crate::ui_local_h::menulist_s);
            }
            5 => {
                RadioButton_Init(item as *mut crate::ui_local_h::menuradiobutton_s);
            }
            1 => {
                Slider_Init(item as *mut crate::ui_local_h::menuslider_s);
            }
            6 => {
                Bitmap_Init(item as *mut crate::ui_local_h::menubitmap_s);
            }
            7 => {
                Text_Init(item as *mut crate::ui_local_h::menutext_s);
            }
            8 => {
                ScrollList_Init(item as *mut crate::ui_local_h::menulist_s);
            }
            9 => {
                PText_Init(item as *mut crate::ui_local_h::menutext_s);
            }
            10 => {
                BText_Init(item as *mut crate::ui_local_h::menutext_s);
            }
            _ => {
                crate::src::ui::ui_syscalls::trap_Error(crate::src::qcommon::q_shared::va(
                    b"Menu_Init: unknown type %d\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*itemptr).type_0,
                ));
            }
        }
    }
    (*menu).nitems += 1;
}
/*
=================
Menu_CursorMoved
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Menu_CursorMoved(mut m: *mut crate::ui_local_h::menuframework_s) {
    let mut callback: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ()> =
        None;
    if (*m).cursor_prev == (*m).cursor {
        return;
    }
    if (*m).cursor_prev >= 0 as libc::c_int && (*m).cursor_prev < (*m).nitems {
        callback = (*((*m).items[(*m).cursor_prev as usize]
            as *mut crate::ui_local_h::menucommon_s))
            .callback;
        if callback.is_some() {
            callback.expect("non-null function pointer")(
                (*m).items[(*m).cursor_prev as usize],
                2 as libc::c_int,
            );
        }
    }
    if (*m).cursor >= 0 as libc::c_int && (*m).cursor < (*m).nitems {
        callback =
            (*((*m).items[(*m).cursor as usize] as *mut crate::ui_local_h::menucommon_s)).callback;
        if callback.is_some() {
            callback.expect("non-null function pointer")(
                (*m).items[(*m).cursor as usize],
                1 as libc::c_int,
            );
        }
    };
}
/*
=================
Menu_SetCursor
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Menu_SetCursor(
    mut m: *mut crate::ui_local_h::menuframework_s,
    mut cursor: libc::c_int,
) {
    if (*((*m).items[cursor as usize] as *mut crate::ui_local_h::menucommon_s)).flags
        & (0x2000 as libc::c_int as libc::c_uint | 0x4000 as libc::c_int as libc::c_uint)
        != 0
    {
        // cursor can't go there
        return;
    }
    (*m).cursor_prev = (*m).cursor;
    (*m).cursor = cursor;
    Menu_CursorMoved(m);
}
/*
=================
Menu_SetCursorToItem
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Menu_SetCursorToItem(
    mut m: *mut crate::ui_local_h::menuframework_s,
    mut ptr: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*m).nitems {
        if (*m).items[i as usize] == ptr {
            Menu_SetCursor(m, i);
            return;
        }
        i += 1
    }
}
/*
** Menu_AdjustCursor
**
** This function takes the given menu, the direction, and attempts
** to adjust the menu's cursor so that it's at the next available
** slot.
*/
#[no_mangle]

pub unsafe extern "C" fn Menu_AdjustCursor(
    mut m: *mut crate::ui_local_h::menuframework_s,
    mut dir: libc::c_int,
) {
    let mut item: *mut crate::ui_local_h::menucommon_s = 0 as *mut crate::ui_local_h::menucommon_s;
    let mut wrapped: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    loop {
        while (*m).cursor >= 0 as libc::c_int && (*m).cursor < (*m).nitems {
            item = (*m).items[(*m).cursor as usize] as *mut crate::ui_local_h::menucommon_s;
            if !((*item).flags
                & (0x2000 as libc::c_int as libc::c_uint
                    | 0x800 as libc::c_int as libc::c_uint
                    | 0x4000 as libc::c_int as libc::c_uint)
                != 0)
            {
                break;
            }
            (*m).cursor += dir
        }
        if dir == 1 as libc::c_int {
            if !((*m).cursor >= (*m).nitems) {
                break;
            }
            if (*m).wrapAround as u64 != 0 {
                if wrapped as u64 != 0 {
                    (*m).cursor = (*m).cursor_prev;
                    return;
                }
                (*m).cursor = 0 as libc::c_int;
                wrapped = crate::src::qcommon::q_shared::qtrue
            } else {
                (*m).cursor = (*m).cursor_prev;
                break;
            }
        } else {
            if !((*m).cursor < 0 as libc::c_int) {
                break;
            }
            if (*m).wrapAround as u64 != 0 {
                if wrapped as u64 != 0 {
                    (*m).cursor = (*m).cursor_prev;
                    return;
                }
                (*m).cursor = (*m).nitems - 1 as libc::c_int;
                wrapped = crate::src::qcommon::q_shared::qtrue
            } else {
                (*m).cursor = (*m).cursor_prev;
                break;
            }
        }
    }
}
/*
=================
Menu_Draw
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Menu_Draw(mut menu: *mut crate::ui_local_h::menuframework_s) {
    let mut i: libc::c_int = 0;
    let mut itemptr: *mut crate::ui_local_h::menucommon_s =
        0 as *mut crate::ui_local_h::menucommon_s;
    // draw menu
    i = 0 as libc::c_int;
    while i < (*menu).nitems {
        itemptr = (*menu).items[i as usize] as *mut crate::ui_local_h::menucommon_s;
        if !((*itemptr).flags & 0x1000 as libc::c_int as libc::c_uint != 0) {
            if (*itemptr).ownerdraw.is_some() {
                // total subclassing, owner draws everything
                (*itemptr).ownerdraw.expect("non-null function pointer")(
                    itemptr as *mut libc::c_void,
                );
            } else {
                match (*itemptr).type_0 {
                    5 => {
                        RadioButton_Draw(itemptr as *mut crate::ui_local_h::menuradiobutton_s);
                    }
                    4 => {
                        crate::src::q3_ui::ui_mfield::MenuField_Draw(
                            itemptr as *mut crate::ui_local_h::menufield_s
                                as *mut crate::ui_local_h::menufield_s,
                        );
                    }
                    1 => {
                        Slider_Draw(itemptr as *mut crate::ui_local_h::menuslider_s);
                    }
                    3 => {
                        SpinControl_Draw(itemptr as *mut crate::ui_local_h::menulist_s);
                    }
                    2 => {
                        Action_Draw(itemptr as *mut crate::ui_local_h::menuaction_s);
                    }
                    6 => {
                        Bitmap_Draw(itemptr as *mut crate::ui_local_h::menubitmap_s);
                    }
                    7 => {
                        Text_Draw(itemptr as *mut crate::ui_local_h::menutext_s);
                    }
                    8 => {
                        ScrollList_Draw(itemptr as *mut crate::ui_local_h::menulist_s);
                    }
                    9 => {
                        PText_Draw(itemptr as *mut crate::ui_local_h::menutext_s);
                    }
                    10 => {
                        BText_Draw(itemptr as *mut crate::ui_local_h::menutext_s);
                    }
                    _ => {
                        crate::src::ui::ui_syscalls::trap_Error(crate::src::qcommon::q_shared::va(
                            b"Menu_Draw: unknown type %d\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            (*itemptr).type_0,
                        ));
                    }
                }
            }
        }
        i += 1
    }
    itemptr = Menu_ItemAtCursor(menu) as *mut crate::ui_local_h::menucommon_s;
    if !itemptr.is_null() && (*itemptr).statusbar.is_some() {
        (*itemptr).statusbar.expect("non-null function pointer")(itemptr as *mut libc::c_void);
    };
}
/*
=================
Menu_ItemAtCursor
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Menu_ItemAtCursor(
    mut m: *mut crate::ui_local_h::menuframework_s,
) -> *mut libc::c_void {
    if (*m).cursor < 0 as libc::c_int || (*m).cursor >= (*m).nitems {
        return 0 as *mut libc::c_void;
    }
    return (*m).items[(*m).cursor as usize];
}
/*
=================
Menu_ActivateItem
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Menu_ActivateItem(
    mut _s: *mut crate::ui_local_h::menuframework_s,
    mut item: *mut crate::ui_local_h::menucommon_s,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    if (*item).callback.is_some() {
        (*item).callback.expect("non-null function pointer")(
            item as *mut libc::c_void,
            3 as libc::c_int,
        );
        if (*item).flags & 0x100000 as libc::c_int as libc::c_uint == 0 {
            return menu_move_sound;
        }
    }
    return 0 as libc::c_int;
}
/*
=================
Menu_DefaultKey
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Menu_DefaultKey(
    mut m: *mut crate::ui_local_h::menuframework_s,
    mut key: libc::c_int,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    let mut sound: crate::src::qcommon::q_shared::sfxHandle_t = 0 as libc::c_int;
    let mut item: *mut crate::ui_local_h::menucommon_s = 0 as *mut crate::ui_local_h::menucommon_s;
    let mut cursor_prev: libc::c_int = 0;
    // menu system keys
    match key {
        179 | 27 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            return menu_out_sound;
        }
        _ => {}
    }
    if m.is_null() || (*m).nitems == 0 {
        return 0 as libc::c_int;
    }
    // route key stimulus to widget
    item = Menu_ItemAtCursor(m) as *mut crate::ui_local_h::menucommon_s;
    if !item.is_null()
        && (*item).flags
            & (0x2000 as libc::c_int as libc::c_uint | 0x4000 as libc::c_int as libc::c_uint)
            == 0
    {
        match (*item).type_0 {
            3 => sound = SpinControl_Key(item as *mut crate::ui_local_h::menulist_s, key),
            5 => sound = RadioButton_Key(item as *mut crate::ui_local_h::menuradiobutton_s, key),
            1 => sound = Slider_Key(item as *mut crate::ui_local_h::menuslider_s, key),
            8 => sound = ScrollList_Key(item as *mut crate::ui_local_h::menulist_s, key),
            4 => {
                sound = crate::src::q3_ui::ui_mfield::MenuField_Key(
                    item as *mut crate::ui_local_h::menufield_s
                        as *mut crate::ui_local_h::menufield_s,
                    &mut key,
                )
            }
            _ => {}
        }
        if sound != 0 {
            // key was handled
            return sound;
        }
    }
    // default handling
    match key {
        161 | 132 => {
            cursor_prev = (*m).cursor;
            (*m).cursor_prev = (*m).cursor;
            (*m).cursor -= 1;
            Menu_AdjustCursor(m, -(1 as libc::c_int));
            if cursor_prev != (*m).cursor {
                Menu_CursorMoved(m);
                sound = menu_move_sound
            }
        }
        9 | 167 | 133 => {
            cursor_prev = (*m).cursor;
            (*m).cursor_prev = (*m).cursor;
            (*m).cursor += 1;
            Menu_AdjustCursor(m, 1 as libc::c_int);
            if cursor_prev != (*m).cursor {
                Menu_CursorMoved(m);
                sound = menu_move_sound
            }
        }
        178 | 180 => {
            if !item.is_null() {
                if (*item).flags & 0x200 as libc::c_int as libc::c_uint != 0
                    && (*item).flags
                        & (0x2000 as libc::c_int as libc::c_uint
                            | 0x4000 as libc::c_int as libc::c_uint)
                        == 0
                {
                    return Menu_ActivateItem(m, item);
                }
            }
        }
        185 | 186 | 187 | 188 | 217 | 218 | 219 | 220 | 221 | 222 | 223 | 224 | 225 | 226 | 227
        | 228 | 229 | 230 | 231 | 232 | 169 | 13 => {
            if !item.is_null() {
                if (*item).flags
                    & (0x800 as libc::c_int as libc::c_uint
                        | 0x2000 as libc::c_int as libc::c_uint
                        | 0x4000 as libc::c_int as libc::c_uint)
                    == 0
                {
                    return Menu_ActivateItem(m, item);
                }
            }
        }
        _ => {}
    }
    return sound;
}
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
/*
=================
Menu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Menu_Cache() {
    crate::src::q3_ui::ui_atoms::uis.charset =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"gfx/2d/bigchars\x00" as *const u8 as *const libc::c_char,
        );
    crate::src::q3_ui::ui_atoms::uis.charsetProp =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/font1_prop.tga\x00" as *const u8 as *const libc::c_char,
        );
    crate::src::q3_ui::ui_atoms::uis.charsetPropGlow =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/font1_prop_glo.tga\x00" as *const u8 as *const libc::c_char,
        );
    crate::src::q3_ui::ui_atoms::uis.charsetPropB =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/font2_prop.tga\x00" as *const u8 as *const libc::c_char,
        );
    crate::src::q3_ui::ui_atoms::uis.cursor =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/3_cursor2\x00" as *const u8 as *const libc::c_char,
        );
    crate::src::q3_ui::ui_atoms::uis.rb_on =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/switch_on\x00" as *const u8 as *const libc::c_char,
        );
    crate::src::q3_ui::ui_atoms::uis.rb_off =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/switch_off\x00" as *const u8 as *const libc::c_char,
        );
    crate::src::q3_ui::ui_atoms::uis.whiteShader =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"white\x00" as *const u8 as *const libc::c_char,
        );
    if crate::src::q3_ui::ui_atoms::uis.glconfig.hardwareType as libc::c_uint
        == crate::tr_types_h::GLHW_RAGEPRO as libc::c_int as libc::c_uint
    {
        // the blend effect turns to shit with the normal
        crate::src::q3_ui::ui_atoms::uis.menuBackShader =
            crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
                b"menubackRagePro\x00" as *const u8 as *const libc::c_char,
            )
    } else {
        crate::src::q3_ui::ui_atoms::uis.menuBackShader =
            crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
                b"menuback\x00" as *const u8 as *const libc::c_char,
            )
    }
    crate::src::q3_ui::ui_atoms::uis.menuBackNoLogoShader =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menubacknologo\x00" as *const u8 as *const libc::c_char,
        );
    menu_in_sound = crate::src::ui::ui_syscalls::trap_S_RegisterSound(
        b"sound/misc/menu1.wav\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::qfalse,
    );
    menu_move_sound = crate::src::ui::ui_syscalls::trap_S_RegisterSound(
        b"sound/misc/menu2.wav\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::qfalse,
    );
    menu_out_sound = crate::src::ui::ui_syscalls::trap_S_RegisterSound(
        b"sound/misc/menu3.wav\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::qfalse,
    );
    menu_buzz_sound = crate::src::ui::ui_syscalls::trap_S_RegisterSound(
        b"sound/misc/menu4.wav\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::qfalse,
    );
    weaponChangeSound = crate::src::ui::ui_syscalls::trap_S_RegisterSound(
        b"sound/weapons/change.wav\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::qfalse,
    );
    // need a nonzero sound, make an empty sound for this
    menu_null_sound = -(1 as libc::c_int);
    sliderBar = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/slider2\x00" as *const u8 as *const libc::c_char,
    );
    sliderButton_0 = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/sliderbutt_0\x00" as *const u8 as *const libc::c_char,
    );
    sliderButton_1 = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/sliderbutt_1\x00" as *const u8 as *const libc::c_char,
    );
}

use ::libc;

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::GENDER_FEMALE;
pub use crate::bg_public_h::GENDER_MALE;
pub use crate::bg_public_h::GENDER_NEUTER;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::STAT_ARMOR;
pub use crate::bg_public_h::STAT_CLIENTS_READY;
pub use crate::bg_public_h::STAT_DEAD_YAW;
pub use crate::bg_public_h::STAT_HEALTH;
pub use crate::bg_public_h::STAT_HOLDABLE_ITEM;
pub use crate::bg_public_h::STAT_MAX_HEALTH;
pub use crate::bg_public_h::STAT_WEAPONS;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::cg_local_h::FOOTSTEP_BOOT;
pub use crate::cg_local_h::FOOTSTEP_ENERGY;
pub use crate::cg_local_h::FOOTSTEP_FLESH;
pub use crate::cg_local_h::FOOTSTEP_MECH;
pub use crate::cg_local_h::FOOTSTEP_METAL;
pub use crate::cg_local_h::FOOTSTEP_NORMAL;
pub use crate::cg_local_h::FOOTSTEP_SPLASH;
pub use crate::cg_local_h::FOOTSTEP_TOTAL;
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic;
pub use crate::src::cgame::cg_syscalls::trap_R_SetColor;
pub use crate::src::qcommon::q_math::g_color_table;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Q_IsColorString;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::RT_BEAM;
pub use crate::tr_types_h::RT_LIGHTNING;
pub use crate::tr_types_h::RT_MAX_REF_ENTITY_TYPE;
pub use crate::tr_types_h::RT_MODEL;
pub use crate::tr_types_h::RT_POLY;
pub use crate::tr_types_h::RT_PORTALSURFACE;
pub use crate::tr_types_h::RT_RAIL_CORE;
pub use crate::tr_types_h::RT_RAIL_RINGS;
pub use crate::tr_types_h::RT_SPRITE;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
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
// cg_drawtools.c -- helper functions called by cg_draw, cg_scoreboard, cg_info, etc
/*
================
CG_AdjustFrom640

Adjusted for resolution and screen aspect ratio
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AdjustFrom640(
    mut x: *mut libc::c_float,
    mut y: *mut libc::c_float,
    mut w: *mut libc::c_float,
    mut h: *mut libc::c_float,
) {
    // scale for screen sizes
    *x *= crate::src::cgame::cg_main::cgs.screenXScale;
    *y *= crate::src::cgame::cg_main::cgs.screenYScale;
    *w *= crate::src::cgame::cg_main::cgs.screenXScale;
    *h *= crate::src::cgame::cg_main::cgs.screenYScale;
}
/*
================
CG_FillRect

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_FillRect(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut color: *const libc::c_float,
) {
    crate::src::cgame::cg_syscalls::trap_R_SetColor(color);
    CG_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x,
        y,
        width,
        height,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        crate::src::cgame::cg_main::cgs.media.whiteShader,
    );
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const libc::c_float);
}
/*
================
CG_DrawSides

Coords are virtual 640x480
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawSides(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut w: libc::c_float,
    mut h: libc::c_float,
    mut size: libc::c_float,
) {
    CG_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    size *= crate::src::cgame::cg_main::cgs.screenXScale;
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x,
        y,
        size,
        h,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        crate::src::cgame::cg_main::cgs.media.whiteShader,
    );
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x + w - size,
        y,
        size,
        h,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        crate::src::cgame::cg_main::cgs.media.whiteShader,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawTopBottom(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut w: libc::c_float,
    mut h: libc::c_float,
    mut size: libc::c_float,
) {
    CG_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    size *= crate::src::cgame::cg_main::cgs.screenYScale;
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x,
        y,
        w,
        size,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        crate::src::cgame::cg_main::cgs.media.whiteShader,
    );
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x,
        y + h - size,
        w,
        size,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        crate::src::cgame::cg_main::cgs.media.whiteShader,
    );
}
/*
================
UI_DrawRect

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawRect(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut size: libc::c_float,
    mut color: *const libc::c_float,
) {
    crate::src::cgame::cg_syscalls::trap_R_SetColor(color);
    CG_DrawTopBottom(x, y, width, height, size);
    CG_DrawSides(
        x,
        y + size,
        width,
        height - size * 2 as libc::c_int as libc::c_float,
        size,
    );
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const libc::c_float);
}
/*
================
CG_DrawPic

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawPic(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut hShader: crate::src::qcommon::q_shared::qhandle_t,
) {
    CG_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x,
        y,
        width,
        height,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        hShader,
    );
}
/*
===============
CG_DrawChar

Coordinates and size in 640*480 virtual screen size
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawChar(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut ch: libc::c_int,
) {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut frow: libc::c_float = 0.;
    let mut fcol: libc::c_float = 0.;
    let mut size: libc::c_float = 0.;
    let mut ax: libc::c_float = 0.;
    let mut ay: libc::c_float = 0.;
    let mut aw: libc::c_float = 0.;
    let mut ah: libc::c_float = 0.;
    ch &= 255 as libc::c_int;
    if ch == ' ' as i32 {
        return;
    }
    ax = x as libc::c_float;
    ay = y as libc::c_float;
    aw = width as libc::c_float;
    ah = height as libc::c_float;
    CG_AdjustFrom640(&mut ax, &mut ay, &mut aw, &mut ah);
    row = ch >> 4 as libc::c_int;
    col = ch & 15 as libc::c_int;
    frow = (row as libc::c_double * 0.0625f64) as libc::c_float;
    fcol = (col as libc::c_double * 0.0625f64) as libc::c_float;
    size = 0.0625f64 as libc::c_float;
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        ax,
        ay,
        aw,
        ah,
        fcol,
        frow,
        fcol + size,
        frow + size,
        crate::src::cgame::cg_main::cgs.media.charsetShader,
    );
}
/*
==================
CG_DrawStringExt

Draws a multi-colored string with a drop shadow, optionally forcing
to a fixed color.

Coordinates are at 640 by 480 virtual resolution
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawStringExt(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut string: *const libc::c_char,
    mut setColor: *const libc::c_float,
    mut forceColor: crate::src::qcommon::q_shared::qboolean,
    mut shadow: crate::src::qcommon::q_shared::qboolean,
    mut charWidth: libc::c_int,
    mut charHeight: libc::c_int,
    mut maxChars: libc::c_int,
) {
    let mut color: crate::src::qcommon::q_shared::vec4_t = [0.; 4]; // do them all!
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut xx: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    if maxChars <= 0 as libc::c_int {
        maxChars = 32767 as libc::c_int
    }
    // draw the drop shadow
    if shadow as u64 != 0 {
        color[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        color[1 as libc::c_int as usize] = color[2 as libc::c_int as usize];
        color[0 as libc::c_int as usize] = color[1 as libc::c_int as usize];
        color[3 as libc::c_int as usize] = *setColor.offset(3 as libc::c_int as isize);
        crate::src::cgame::cg_syscalls::trap_R_SetColor(color.as_mut_ptr());
        s = string;
        xx = x;
        cnt = 0 as libc::c_int;
        while *s as libc::c_int != 0 && cnt < maxChars {
            if crate::src::qcommon::q_shared::Q_IsColorString(s) as u64 != 0 {
                s = s.offset(2 as libc::c_int as isize)
            } else {
                CG_DrawChar(
                    xx + 2 as libc::c_int,
                    y + 2 as libc::c_int,
                    charWidth,
                    charHeight,
                    *s as libc::c_int,
                );
                cnt += 1;
                xx += charWidth;
                s = s.offset(1)
            }
        }
    }
    // draw the colored text
    s = string;
    xx = x;
    cnt = 0 as libc::c_int;
    crate::src::cgame::cg_syscalls::trap_R_SetColor(setColor);
    while *s as libc::c_int != 0 && cnt < maxChars {
        if crate::src::qcommon::q_shared::Q_IsColorString(s) as u64 != 0 {
            if forceColor as u64 == 0 {
                crate::stdlib::memcpy(
                    color.as_mut_ptr() as *mut libc::c_void,
                    crate::src::qcommon::q_math::g_color_table[(*s.offset(1 as libc::c_int as isize)
                        as libc::c_int
                        - '0' as i32
                        & 0x7 as libc::c_int)
                        as usize]
                        .as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<crate::src::qcommon::q_shared::vec4_t>() as libc::c_ulong,
                );
                color[3 as libc::c_int as usize] = *setColor.offset(3 as libc::c_int as isize);
                crate::src::cgame::cg_syscalls::trap_R_SetColor(color.as_mut_ptr());
            }
            s = s.offset(2 as libc::c_int as isize)
        } else {
            CG_DrawChar(xx, y, charWidth, charHeight, *s as libc::c_int);
            xx += charWidth;
            cnt += 1;
            s = s.offset(1)
        }
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const libc::c_float);
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawBigString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut s: *const libc::c_char,
    mut alpha: libc::c_float,
) {
    let mut color: [libc::c_float; 4] = [0.; 4];
    color[2 as libc::c_int as usize] = 1.0f64 as libc::c_float;
    color[1 as libc::c_int as usize] = color[2 as libc::c_int as usize];
    color[0 as libc::c_int as usize] = color[1 as libc::c_int as usize];
    color[3 as libc::c_int as usize] = alpha;
    CG_DrawStringExt(
        x,
        y,
        s,
        color.as_mut_ptr(),
        crate::src::qcommon::q_shared::qfalse,
        crate::src::qcommon::q_shared::qtrue,
        16 as libc::c_int,
        16 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawBigStringColor(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut s: *const libc::c_char,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    CG_DrawStringExt(
        x,
        y,
        s,
        color as *const libc::c_float,
        crate::src::qcommon::q_shared::qtrue,
        crate::src::qcommon::q_shared::qtrue,
        16 as libc::c_int,
        16 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawSmallString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut s: *const libc::c_char,
    mut alpha: libc::c_float,
) {
    let mut color: [libc::c_float; 4] = [0.; 4];
    color[2 as libc::c_int as usize] = 1.0f64 as libc::c_float;
    color[1 as libc::c_int as usize] = color[2 as libc::c_int as usize];
    color[0 as libc::c_int as usize] = color[1 as libc::c_int as usize];
    color[3 as libc::c_int as usize] = alpha;
    CG_DrawStringExt(
        x,
        y,
        s,
        color.as_mut_ptr(),
        crate::src::qcommon::q_shared::qfalse,
        crate::src::qcommon::q_shared::qfalse,
        8 as libc::c_int,
        16 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawSmallStringColor(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut s: *const libc::c_char,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    CG_DrawStringExt(
        x,
        y,
        s,
        color as *const libc::c_float,
        crate::src::qcommon::q_shared::qtrue,
        crate::src::qcommon::q_shared::qfalse,
        8 as libc::c_int,
        16 as libc::c_int,
        0 as libc::c_int,
    );
}
/*
=================
CG_DrawStrlen

Returns character count, skiping color escape codes
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawStrlen(mut str: *const libc::c_char) -> libc::c_int {
    let mut s: *const libc::c_char = str;
    let mut count: libc::c_int = 0 as libc::c_int;
    while *s != 0 {
        if crate::src::qcommon::q_shared::Q_IsColorString(s) as u64 != 0 {
            s = s.offset(2 as libc::c_int as isize)
        } else {
            count += 1;
            s = s.offset(1)
        }
    }
    return count;
}
/*
=============
CG_TileClearBox

This repeats a 64*64 tile graphic to fill the screen around a sized down
refresh window.
=============
*/

unsafe extern "C" fn CG_TileClearBox(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut hShader: crate::src::qcommon::q_shared::qhandle_t,
) {
    let mut s1: libc::c_float = 0.;
    let mut t1: libc::c_float = 0.;
    let mut s2: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    s1 = (x as libc::c_double / 64.0f64) as libc::c_float;
    t1 = (y as libc::c_double / 64.0f64) as libc::c_float;
    s2 = ((x + w) as libc::c_double / 64.0f64) as libc::c_float;
    t2 = ((y + h) as libc::c_double / 64.0f64) as libc::c_float;
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x as libc::c_float,
        y as libc::c_float,
        w as libc::c_float,
        h as libc::c_float,
        s1,
        t1,
        s2,
        t2,
        hShader,
    );
}
/*
==============
CG_TileClear

Clear around a sized down screen
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_TileClear() {
    let mut top: libc::c_int = 0;
    let mut bottom: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    w = crate::src::cgame::cg_main::cgs.glconfig.vidWidth;
    h = crate::src::cgame::cg_main::cgs.glconfig.vidHeight;
    if crate::src::cgame::cg_main::cg.refdef.x == 0 as libc::c_int
        && crate::src::cgame::cg_main::cg.refdef.y == 0 as libc::c_int
        && crate::src::cgame::cg_main::cg.refdef.width == w
        && crate::src::cgame::cg_main::cg.refdef.height == h
    {
        return;
        // full screen rendering
    }
    top = crate::src::cgame::cg_main::cg.refdef.y;
    bottom = top + crate::src::cgame::cg_main::cg.refdef.height - 1 as libc::c_int;
    left = crate::src::cgame::cg_main::cg.refdef.x;
    right = left + crate::src::cgame::cg_main::cg.refdef.width - 1 as libc::c_int;
    // clear above view screen
    CG_TileClearBox(
        0 as libc::c_int,
        0 as libc::c_int,
        w,
        top,
        crate::src::cgame::cg_main::cgs.media.backTileShader,
    );
    // clear below view screen
    CG_TileClearBox(
        0 as libc::c_int,
        bottom,
        w,
        h - bottom,
        crate::src::cgame::cg_main::cgs.media.backTileShader,
    );
    // clear left of view screen
    CG_TileClearBox(
        0 as libc::c_int,
        top,
        left,
        bottom - top + 1 as libc::c_int,
        crate::src::cgame::cg_main::cgs.media.backTileShader,
    );
    // clear right of view screen
    CG_TileClearBox(
        right,
        top,
        w - right,
        bottom - top + 1 as libc::c_int,
        crate::src::cgame::cg_main::cgs.media.backTileShader,
    );
}
/*
================
CG_FadeColor
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_FadeColor(
    mut startMsec: libc::c_int,
    mut totalMsec: libc::c_int,
) -> *mut libc::c_float {
    static mut color: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut t: libc::c_int = 0;
    if startMsec == 0 as libc::c_int {
        return 0 as *mut libc::c_float;
    }
    t = crate::src::cgame::cg_main::cg.time - startMsec;
    if t >= totalMsec {
        return 0 as *mut libc::c_float;
    }
    // fade out
    if totalMsec - t < 200 as libc::c_int {
        color[3 as libc::c_int as usize] = ((totalMsec - t) as libc::c_double * 1.0f64
            / 200 as libc::c_int as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t
    } else {
        color[3 as libc::c_int as usize] = 1.0f64 as crate::src::qcommon::q_shared::vec_t
    }
    color[2 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    color[1 as libc::c_int as usize] = color[2 as libc::c_int as usize];
    color[0 as libc::c_int as usize] = color[1 as libc::c_int as usize];
    return color.as_mut_ptr();
}
/*
================
CG_TeamColor
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_TeamColor(mut team: libc::c_int) -> *mut libc::c_float {
    static mut red: crate::src::qcommon::q_shared::vec4_t = [
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0.2f32,
        0.2f32,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    static mut blue: crate::src::qcommon::q_shared::vec4_t = [
        0.2f32,
        0.2f32,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    static mut other: crate::src::qcommon::q_shared::vec4_t = [
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    static mut spectator: crate::src::qcommon::q_shared::vec4_t = [
        0.7f32,
        0.7f32,
        0.7f32,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    match team {
        1 => return red.as_mut_ptr(),
        2 => return blue.as_mut_ptr(),
        3 => return spectator.as_mut_ptr(),
        _ => return other.as_mut_ptr(),
    };
}
/*
=================
CG_GetColorForHealth
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_GetColorForHealth(
    mut health: libc::c_int,
    mut armor: libc::c_int,
    mut hcolor: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut count: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    // calculate the total points of damage that can
    // be sustained at the current health / armor level
    if health <= 0 as libc::c_int {
        let ref mut fresh0 = *hcolor.offset(2 as libc::c_int as isize); // black
        *fresh0 = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        let ref mut fresh1 = *hcolor.offset(1 as libc::c_int as isize);
        *fresh1 = *fresh0;
        *hcolor.offset(0 as libc::c_int as isize) = *fresh1;
        *hcolor.offset(3 as libc::c_int as isize) =
            1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        return;
    }
    count = armor;
    max = (health as libc::c_double * 0.66f64 / (1.0f64 - 0.66f64)) as libc::c_int;
    if max < count {
        count = max
    }
    health += count;
    // set the color based on health
    *hcolor.offset(0 as libc::c_int as isize) = 1.0f64 as crate::src::qcommon::q_shared::vec_t;
    *hcolor.offset(3 as libc::c_int as isize) = 1.0f64 as crate::src::qcommon::q_shared::vec_t;
    if health >= 100 as libc::c_int {
        *hcolor.offset(2 as libc::c_int as isize) = 1.0f64 as crate::src::qcommon::q_shared::vec_t
    } else if health < 66 as libc::c_int {
        *hcolor.offset(2 as libc::c_int as isize) =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t
    } else {
        *hcolor.offset(2 as libc::c_int as isize) = ((health - 66 as libc::c_int) as libc::c_double
            / 33.0f64)
            as crate::src::qcommon::q_shared::vec_t
    }
    if health > 60 as libc::c_int {
        *hcolor.offset(1 as libc::c_int as isize) = 1.0f64 as crate::src::qcommon::q_shared::vec_t
    } else if health < 30 as libc::c_int {
        *hcolor.offset(1 as libc::c_int as isize) =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t
    } else {
        *hcolor.offset(1 as libc::c_int as isize) = ((health - 30 as libc::c_int) as libc::c_double
            / 30.0f64)
            as crate::src::qcommon::q_shared::vec_t
    };
}
/*
=================
CG_ColorForHealth
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ColorForHealth(mut hcolor: *mut crate::src::qcommon::q_shared::vec_t) {
    CG_GetColorForHealth(
        (*crate::src::cgame::cg_main::cg.snap).ps.stats
            [crate::bg_public_h::STAT_HEALTH as libc::c_int as usize],
        (*crate::src::cgame::cg_main::cg.snap).ps.stats
            [crate::bg_public_h::STAT_ARMOR as libc::c_int as usize],
        hcolor,
    );
}
/*
=================
UI_DrawProportionalString2
=================
*/

static mut propMap: [[libc::c_int; 3]; 128] = [
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
    [0 as libc::c_int, 0 as libc::c_int, 8 as libc::c_int],
    [11 as libc::c_int, 122 as libc::c_int, 7 as libc::c_int],
    [154 as libc::c_int, 181 as libc::c_int, 14 as libc::c_int],
    [55 as libc::c_int, 122 as libc::c_int, 17 as libc::c_int],
    [79 as libc::c_int, 122 as libc::c_int, 18 as libc::c_int],
    [101 as libc::c_int, 122 as libc::c_int, 23 as libc::c_int],
    [153 as libc::c_int, 122 as libc::c_int, 18 as libc::c_int],
    [9 as libc::c_int, 93 as libc::c_int, 7 as libc::c_int],
    [207 as libc::c_int, 122 as libc::c_int, 8 as libc::c_int],
    [230 as libc::c_int, 122 as libc::c_int, 9 as libc::c_int],
    [177 as libc::c_int, 122 as libc::c_int, 18 as libc::c_int],
    [30 as libc::c_int, 152 as libc::c_int, 18 as libc::c_int],
    [85 as libc::c_int, 181 as libc::c_int, 7 as libc::c_int],
    [34 as libc::c_int, 93 as libc::c_int, 11 as libc::c_int],
    [110 as libc::c_int, 181 as libc::c_int, 6 as libc::c_int],
    [130 as libc::c_int, 152 as libc::c_int, 14 as libc::c_int],
    [22 as libc::c_int, 64 as libc::c_int, 17 as libc::c_int],
    [41 as libc::c_int, 64 as libc::c_int, 12 as libc::c_int],
    [58 as libc::c_int, 64 as libc::c_int, 17 as libc::c_int],
    [78 as libc::c_int, 64 as libc::c_int, 18 as libc::c_int],
    [98 as libc::c_int, 64 as libc::c_int, 19 as libc::c_int],
    [120 as libc::c_int, 64 as libc::c_int, 18 as libc::c_int],
    [141 as libc::c_int, 64 as libc::c_int, 18 as libc::c_int],
    [204 as libc::c_int, 64 as libc::c_int, 16 as libc::c_int],
    [162 as libc::c_int, 64 as libc::c_int, 17 as libc::c_int],
    [182 as libc::c_int, 64 as libc::c_int, 18 as libc::c_int],
    [59 as libc::c_int, 181 as libc::c_int, 7 as libc::c_int],
    [35 as libc::c_int, 181 as libc::c_int, 7 as libc::c_int],
    [203 as libc::c_int, 152 as libc::c_int, 14 as libc::c_int],
    [56 as libc::c_int, 93 as libc::c_int, 14 as libc::c_int],
    [228 as libc::c_int, 152 as libc::c_int, 14 as libc::c_int],
    [177 as libc::c_int, 181 as libc::c_int, 18 as libc::c_int],
    [28 as libc::c_int, 122 as libc::c_int, 22 as libc::c_int],
    [5 as libc::c_int, 4 as libc::c_int, 18 as libc::c_int],
    [27 as libc::c_int, 4 as libc::c_int, 18 as libc::c_int],
    [48 as libc::c_int, 4 as libc::c_int, 18 as libc::c_int],
    [69 as libc::c_int, 4 as libc::c_int, 17 as libc::c_int],
    [90 as libc::c_int, 4 as libc::c_int, 13 as libc::c_int],
    [106 as libc::c_int, 4 as libc::c_int, 13 as libc::c_int],
    [121 as libc::c_int, 4 as libc::c_int, 18 as libc::c_int],
    [143 as libc::c_int, 4 as libc::c_int, 17 as libc::c_int],
    [164 as libc::c_int, 4 as libc::c_int, 8 as libc::c_int],
    [175 as libc::c_int, 4 as libc::c_int, 16 as libc::c_int],
    [195 as libc::c_int, 4 as libc::c_int, 18 as libc::c_int],
    [216 as libc::c_int, 4 as libc::c_int, 12 as libc::c_int],
    [230 as libc::c_int, 4 as libc::c_int, 23 as libc::c_int],
    [6 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [27 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [48 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [68 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [90 as libc::c_int, 34 as libc::c_int, 17 as libc::c_int],
    [110 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [130 as libc::c_int, 34 as libc::c_int, 14 as libc::c_int],
    [146 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [166 as libc::c_int, 34 as libc::c_int, 19 as libc::c_int],
    [185 as libc::c_int, 34 as libc::c_int, 29 as libc::c_int],
    [215 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [234 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [5 as libc::c_int, 64 as libc::c_int, 14 as libc::c_int],
    [60 as libc::c_int, 152 as libc::c_int, 7 as libc::c_int],
    [106 as libc::c_int, 151 as libc::c_int, 13 as libc::c_int],
    [83 as libc::c_int, 152 as libc::c_int, 7 as libc::c_int],
    [128 as libc::c_int, 122 as libc::c_int, 17 as libc::c_int],
    [4 as libc::c_int, 152 as libc::c_int, 21 as libc::c_int],
    [134 as libc::c_int, 181 as libc::c_int, 5 as libc::c_int],
    [5 as libc::c_int, 4 as libc::c_int, 18 as libc::c_int],
    [27 as libc::c_int, 4 as libc::c_int, 18 as libc::c_int],
    [48 as libc::c_int, 4 as libc::c_int, 18 as libc::c_int],
    [69 as libc::c_int, 4 as libc::c_int, 17 as libc::c_int],
    [90 as libc::c_int, 4 as libc::c_int, 13 as libc::c_int],
    [106 as libc::c_int, 4 as libc::c_int, 13 as libc::c_int],
    [121 as libc::c_int, 4 as libc::c_int, 18 as libc::c_int],
    [143 as libc::c_int, 4 as libc::c_int, 17 as libc::c_int],
    [164 as libc::c_int, 4 as libc::c_int, 8 as libc::c_int],
    [175 as libc::c_int, 4 as libc::c_int, 16 as libc::c_int],
    [195 as libc::c_int, 4 as libc::c_int, 18 as libc::c_int],
    [216 as libc::c_int, 4 as libc::c_int, 12 as libc::c_int],
    [230 as libc::c_int, 4 as libc::c_int, 23 as libc::c_int],
    [6 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [27 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [48 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [68 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [90 as libc::c_int, 34 as libc::c_int, 17 as libc::c_int],
    [110 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [130 as libc::c_int, 34 as libc::c_int, 14 as libc::c_int],
    [146 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [166 as libc::c_int, 34 as libc::c_int, 19 as libc::c_int],
    [185 as libc::c_int, 34 as libc::c_int, 29 as libc::c_int],
    [215 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [234 as libc::c_int, 34 as libc::c_int, 18 as libc::c_int],
    [5 as libc::c_int, 64 as libc::c_int, 14 as libc::c_int],
    [153 as libc::c_int, 152 as libc::c_int, 13 as libc::c_int],
    [11 as libc::c_int, 181 as libc::c_int, 5 as libc::c_int],
    [180 as libc::c_int, 152 as libc::c_int, 13 as libc::c_int],
    [79 as libc::c_int, 93 as libc::c_int, 17 as libc::c_int],
    [0 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int)],
];

static mut propMapB: [[libc::c_int; 3]; 26] = [
    [11 as libc::c_int, 12 as libc::c_int, 33 as libc::c_int],
    [49 as libc::c_int, 12 as libc::c_int, 31 as libc::c_int],
    [85 as libc::c_int, 12 as libc::c_int, 31 as libc::c_int],
    [120 as libc::c_int, 12 as libc::c_int, 30 as libc::c_int],
    [156 as libc::c_int, 12 as libc::c_int, 21 as libc::c_int],
    [183 as libc::c_int, 12 as libc::c_int, 21 as libc::c_int],
    [207 as libc::c_int, 12 as libc::c_int, 32 as libc::c_int],
    [13 as libc::c_int, 55 as libc::c_int, 30 as libc::c_int],
    [49 as libc::c_int, 55 as libc::c_int, 13 as libc::c_int],
    [66 as libc::c_int, 55 as libc::c_int, 29 as libc::c_int],
    [101 as libc::c_int, 55 as libc::c_int, 31 as libc::c_int],
    [135 as libc::c_int, 55 as libc::c_int, 21 as libc::c_int],
    [158 as libc::c_int, 55 as libc::c_int, 40 as libc::c_int],
    [204 as libc::c_int, 55 as libc::c_int, 32 as libc::c_int],
    [12 as libc::c_int, 97 as libc::c_int, 31 as libc::c_int],
    [48 as libc::c_int, 97 as libc::c_int, 31 as libc::c_int],
    [82 as libc::c_int, 97 as libc::c_int, 30 as libc::c_int],
    [118 as libc::c_int, 97 as libc::c_int, 30 as libc::c_int],
    [153 as libc::c_int, 97 as libc::c_int, 30 as libc::c_int],
    [185 as libc::c_int, 97 as libc::c_int, 25 as libc::c_int],
    [213 as libc::c_int, 97 as libc::c_int, 30 as libc::c_int],
    [11 as libc::c_int, 139 as libc::c_int, 32 as libc::c_int],
    [42 as libc::c_int, 139 as libc::c_int, 51 as libc::c_int],
    [93 as libc::c_int, 139 as libc::c_int, 32 as libc::c_int],
    [126 as libc::c_int, 139 as libc::c_int, 31 as libc::c_int],
    [158 as libc::c_int, 139 as libc::c_int, 25 as libc::c_int],
];
/*
=================
UI_DrawBannerString
=================
*/

unsafe extern "C" fn UI_DrawBannerString2(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_uchar = 0;
    let mut ax: libc::c_float = 0.;
    let mut ay: libc::c_float = 0.;
    let mut aw: libc::c_float = 0.;
    let mut ah: libc::c_float = 0.;
    let mut frow: libc::c_float = 0.;
    let mut fcol: libc::c_float = 0.;
    let mut fwidth: libc::c_float = 0.;
    let mut fheight: libc::c_float = 0.;
    // draw the colored text
    crate::src::cgame::cg_syscalls::trap_R_SetColor(color as *const libc::c_float);
    ax = x as libc::c_float * crate::src::cgame::cg_main::cgs.screenXScale
        + crate::src::cgame::cg_main::cgs.screenXBias;
    ay = y as libc::c_float * crate::src::cgame::cg_main::cgs.screenYScale;
    s = str;
    while *s != 0 {
        ch = (*s as libc::c_int & 127 as libc::c_int) as libc::c_uchar;
        if ch as libc::c_int == ' ' as i32 {
            ax += (12 as libc::c_int as libc::c_float + 4 as libc::c_int as libc::c_float)
                * crate::src::cgame::cg_main::cgs.screenXScale
        } else if ch as libc::c_int >= 'A' as i32 && ch as libc::c_int <= 'Z' as i32 {
            ch = (ch as libc::c_int - 'A' as i32) as libc::c_uchar;
            fcol = propMapB[ch as usize][0 as libc::c_int as usize] as libc::c_float / 256.0f32;
            frow = propMapB[ch as usize][1 as libc::c_int as usize] as libc::c_float / 256.0f32;
            fwidth = propMapB[ch as usize][2 as libc::c_int as usize] as libc::c_float / 256.0f32;
            fheight = 36 as libc::c_int as libc::c_float / 256.0f32;
            aw = propMapB[ch as usize][2 as libc::c_int as usize] as libc::c_float
                * crate::src::cgame::cg_main::cgs.screenXScale;
            ah = 36 as libc::c_int as libc::c_float * crate::src::cgame::cg_main::cgs.screenYScale;
            crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
                ax,
                ay,
                aw,
                ah,
                fcol,
                frow,
                fcol + fwidth,
                frow + fheight,
                crate::src::cgame::cg_main::cgs.media.charsetPropB,
            );
            ax += aw
                + 4 as libc::c_int as libc::c_float * crate::src::cgame::cg_main::cgs.screenXScale
        }
        s = s.offset(1)
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const libc::c_float);
}
#[no_mangle]

pub unsafe extern "C" fn UI_DrawBannerString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut style: libc::c_int,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut drawcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    // find the width of the drawn text
    s = str;
    width = 0 as libc::c_int;
    while *s != 0 {
        ch = *s as libc::c_int;
        if ch == ' ' as i32 {
            width += 12 as libc::c_int
        } else if ch >= 'A' as i32 && ch <= 'Z' as i32 {
            width +=
                propMapB[(ch - 'A' as i32) as usize][2 as libc::c_int as usize] + 4 as libc::c_int
        }
        s = s.offset(1)
    }
    width -= 4 as libc::c_int;
    match style & 0x7 as libc::c_int {
        1 => x -= width / 2 as libc::c_int,
        2 => x -= width,
        0 | _ => {}
    }
    if style & 0x800 as libc::c_int != 0 {
        drawcolor[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        drawcolor[1 as libc::c_int as usize] = drawcolor[2 as libc::c_int as usize];
        drawcolor[0 as libc::c_int as usize] = drawcolor[1 as libc::c_int as usize];
        drawcolor[3 as libc::c_int as usize] = *color.offset(3 as libc::c_int as isize);
        UI_DrawBannerString2(
            x + 2 as libc::c_int,
            y + 2 as libc::c_int,
            str,
            drawcolor.as_mut_ptr(),
        );
    }
    UI_DrawBannerString2(x, y, str, color);
}
#[no_mangle]

pub unsafe extern "C" fn UI_ProportionalStringWidth(mut str: *const libc::c_char) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_int = 0;
    let mut charWidth: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    s = str;
    width = 0 as libc::c_int;
    while *s != 0 {
        ch = *s as libc::c_int & 127 as libc::c_int;
        charWidth = propMap[ch as usize][2 as libc::c_int as usize];
        if charWidth != -(1 as libc::c_int) {
            width += charWidth;
            width += 3 as libc::c_int
        }
        s = s.offset(1)
    }
    width -= 3 as libc::c_int;
    return width;
}

unsafe extern "C" fn UI_DrawProportionalString2(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
    mut sizeScale: libc::c_float,
    mut charset: crate::src::qcommon::q_shared::qhandle_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_uchar = 0;
    let mut ax: libc::c_float = 0.;
    let mut ay: libc::c_float = 0.;
    let mut aw: libc::c_float = 0.;
    let mut ah: libc::c_float = 0.;
    let mut frow: libc::c_float = 0.;
    let mut fcol: libc::c_float = 0.;
    let mut fwidth: libc::c_float = 0.;
    let mut fheight: libc::c_float = 0.;
    // draw the colored text
    crate::src::cgame::cg_syscalls::trap_R_SetColor(color as *const libc::c_float);
    ax = x as libc::c_float * crate::src::cgame::cg_main::cgs.screenXScale
        + crate::src::cgame::cg_main::cgs.screenXBias;
    ay = y as libc::c_float * crate::src::cgame::cg_main::cgs.screenYScale;
    s = str;
    while *s != 0 {
        ch = (*s as libc::c_int & 127 as libc::c_int) as libc::c_uchar;
        if ch as libc::c_int == ' ' as i32 {
            aw = 8 as libc::c_int as libc::c_float
                * crate::src::cgame::cg_main::cgs.screenXScale
                * sizeScale
        } else if propMap[ch as usize][2 as libc::c_int as usize] != -(1 as libc::c_int) {
            fcol = propMap[ch as usize][0 as libc::c_int as usize] as libc::c_float / 256.0f32;
            frow = propMap[ch as usize][1 as libc::c_int as usize] as libc::c_float / 256.0f32;
            fwidth = propMap[ch as usize][2 as libc::c_int as usize] as libc::c_float / 256.0f32;
            fheight = 27 as libc::c_int as libc::c_float / 256.0f32;
            aw = propMap[ch as usize][2 as libc::c_int as usize] as libc::c_float
                * crate::src::cgame::cg_main::cgs.screenXScale
                * sizeScale;
            ah = 27 as libc::c_int as libc::c_float
                * crate::src::cgame::cg_main::cgs.screenYScale
                * sizeScale;
            crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
                ax,
                ay,
                aw,
                ah,
                fcol,
                frow,
                fcol + fwidth,
                frow + fheight,
                charset,
            );
        } else {
            aw = 0 as libc::c_int as libc::c_float
        }
        ax += aw
            + 3 as libc::c_int as libc::c_float
                * crate::src::cgame::cg_main::cgs.screenXScale
                * sizeScale;
        s = s.offset(1)
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const libc::c_float);
}
/*
=================
UI_ProportionalSizeScale
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ProportionalSizeScale(mut style: libc::c_int) -> libc::c_float {
    if style & 0x10 as libc::c_int != 0 {
        return 0.75f64 as libc::c_float;
    }
    return 1.00f64 as libc::c_float;
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
// The entire cgame module is unloaded and reloaded on each level change,
// so there is NO persistant data between levels on the client side.
// If you absolutely need something stored, it can either be kept
// by the server in the server stored userinfos, or stashed in a cvar.
// time for fragments to sink into ground before going away
// amount to scale up the icons when activating
// num frame for '-' stats digit
// very large characters
//=================================================
// player entities need to track more information
// than any other type of entity.
// note that not every player entity is a client entity,
// because corpses after respawn are outside the normal
// client numbering range
// when changing animation, set animationTime to frameTime + lerping time
// The current lerp will finish out, then it will lerp to the new animation
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// flip from 0 to 1
// machinegun spinning
//=================================================
// centity_t have a direct corespondence with gentity_t in the game, but
// only the entityState_t is directly communicated to the cgame
// from cg.frame
// from cg.nextFrame, if available
// true if next is valid to interpolate to
// true if cg.frame holds this entity
// move to playerEntity?
// so missile trails can handle dropped initial packets
// last time this entity was found in a snapshot
// decay the error from this time
// false if origin / angles is an interpolation
// exact interpolated position of entity on this frame
//======================================================================
// local entities are created as a result of events or predicted actions,
// and live independently from all server transmitted entities
// fade alpha instead of rgb
// do not scale size over time
// tumble over time, used for ejecting shells
// sound 1 for kamikaze
// sound 2 for kamikaze
// fragment local entities can leave marks on walls
// fragment local entities can make sounds on impacts
// 1.0 / (endTime - startTime)
// 0.0 = no bounce, 1.0 = perfect
// mark to leave on fragment impact
//======================================================================
// each client has an associated clientInfo_t
// that contains media references necessary to present the
// client model and other color coded effects
// this is regenerated each time a client's configstring changes,
// usually as a result of a userinfo (name, model, etc) change
// 0 = not bot, 1-5 = bot
// updated by score servercmds
// location index for team mode
// you only get this info about your teammates
// in tourney mode
// task in teamplay (offence/defence)
// true when this is a team leader
// so can display quad/flag status
// when clientinfo is changed, the loading of models/skins/sounds
// can be deferred until you are dead, to prevent hitches in
// gameplay
// true if using the new mission pack animations
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// move head in icon views
// from model
// each WP_* weapon enum has an associated weaponInfo_t
// that contains media references necessary to present the
// weapon and its effects
// the hands don't actually draw, they just position the weapon
// so it will rotate centered instead of by tag
// fast firing weapons randomly choose
// each IT_* item has an associated itemInfo_t
// that constains media references necessary to present the
// item and its effects
//======================================================================
// all cg.stepTime, cg.duckTime, cg.landTime, etc are set to cg.time when the action
// occurs, and they will have visible effects for #define STEP_TIME or whatever msec after
// incremented each frame
// taking a level menu screenshot
// don't defer players at initial startup
// don't play voice rewards, because game will end shortly
// there are only one or two snapshot_t that are relevant at a time
// the number of snapshots the client system has received
// the time from latestSnapshotNum, so we don't need to read the snapshot yet
// cg.snap->serverTime <= cg.time
// cg.nextSnap->serverTime > cg.time, or NULL
// (float)( cg.time - cg.frame->serverTime ) / (cg.nextFrame->serverTime - cg.frame->serverTime)
// cg.time - cg.oldTime
// this is the time value that the client
// is rendering at.
// time at last frame, used for missile trails and prediction checking
// either cg.snap->time or cg.nextSnap->time
// 5 min, 1 min, overtime
// set on a map restart to set back the weapon
// during deaths, chasecams, etc
// prediction state
// true if prediction has hit a trigger_teleport
// clear until the first call to CG_PredictPlayerState
// for stair up smoothing
// for duck viewheight smoothing
// for landing hard
// input state sent to server
// auto rotating items
// view rendering
// will be converted to refdef.viewaxis
// zoom key
// information screen text during loading
// scoreboard
// list of names
// length of list
// width in device units
// next time to offset
// current paint x
// current paint x
// current offset from start
// current offset from start
// centerprinting
// low ammo warning state
// 1 = low, 2 = empty
// crosshair client ID
// powerup active flashing
// attacking player
// reward medals
// sound buffer mainly for announcer sounds
// warmup countdown
//==========================
// the pulse around the crosshair is timed separately
// blend blobs
// status bar head
// view movement
// temp working variables for player view
//qboolean cameraMode;		// if rendering from a loaded camera
// development tool
// all of the model, shader, and sound references that are
// loaded at gamestate time are stored in cgMedia_t
// Other media that can be tied to clients, weapons, or items are
// stored in the clientInfo_t, itemInfo_t, weaponInfo_t, and powerupInfo_t
// gib explosions
// wall mark shaders
// powerup shaders
// weapon effect models
// weapon effect shaders
// special effects models
// scoreboard headers
// medals shown during gameplay
// sounds
//sfxHandle_t	sfx_railg;
// teamplay sounds
// tournament sounds
// The client game static (cgs) structure hold everything
// loaded or calculated from the gamestate.  It will NOT
// be cleared when a tournement restart is done, allowing
// all clients to begin playing instantly
// gamestate from server
// rendering configuration
// derived from glconfig
// reliable command stream counter
// the number of snapshots cgame has requested
// detected on startup by checking sv_running
// parsed from serverinfo
// beep whenever changed
// beep whenever changed
// from configstrings
// flag status from configstrings
//
// locally derived information from gamestate
//
// teamchat width is *3 because of embedded color codes
// orders
// media
//==============================================================================
//extern	vmCvar_t		cg_pmove_fixed;
//
// cg_main.c
//
//
// cg_view.c
//
//
// cg_drawtools.c
//
/*
=================
UI_DrawProportionalString
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawProportionalString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut style: libc::c_int,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut drawcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut width: libc::c_int = 0;
    let mut sizeScale: libc::c_float = 0.;
    sizeScale = UI_ProportionalSizeScale(style);
    match style & 0x7 as libc::c_int {
        1 => {
            width = (UI_ProportionalStringWidth(str) as libc::c_float * sizeScale) as libc::c_int;
            x -= width / 2 as libc::c_int
        }
        2 => {
            width = (UI_ProportionalStringWidth(str) as libc::c_float * sizeScale) as libc::c_int;
            x -= width
        }
        0 | _ => {}
    }
    if style & 0x800 as libc::c_int != 0 {
        drawcolor[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        drawcolor[1 as libc::c_int as usize] = drawcolor[2 as libc::c_int as usize];
        drawcolor[0 as libc::c_int as usize] = drawcolor[1 as libc::c_int as usize];
        drawcolor[3 as libc::c_int as usize] = *color.offset(3 as libc::c_int as isize);
        UI_DrawProportionalString2(
            x + 2 as libc::c_int,
            y + 2 as libc::c_int,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            crate::src::cgame::cg_main::cgs.media.charsetProp,
        );
    }
    if style & 0x2000 as libc::c_int != 0 {
        drawcolor[0 as libc::c_int as usize] =
            (*color.offset(0 as libc::c_int as isize) as libc::c_double * 0.8f64)
                as crate::src::qcommon::q_shared::vec_t;
        drawcolor[1 as libc::c_int as usize] =
            (*color.offset(1 as libc::c_int as isize) as libc::c_double * 0.8f64)
                as crate::src::qcommon::q_shared::vec_t;
        drawcolor[2 as libc::c_int as usize] =
            (*color.offset(2 as libc::c_int as isize) as libc::c_double * 0.8f64)
                as crate::src::qcommon::q_shared::vec_t;
        drawcolor[3 as libc::c_int as usize] = *color.offset(3 as libc::c_int as isize);
        UI_DrawProportionalString2(
            x,
            y,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            crate::src::cgame::cg_main::cgs.media.charsetProp,
        );
        return;
    }
    if style & 0x4000 as libc::c_int != 0 {
        drawcolor[0 as libc::c_int as usize] =
            (*color.offset(0 as libc::c_int as isize) as libc::c_double * 0.8f64)
                as crate::src::qcommon::q_shared::vec_t;
        drawcolor[1 as libc::c_int as usize] =
            (*color.offset(1 as libc::c_int as isize) as libc::c_double * 0.8f64)
                as crate::src::qcommon::q_shared::vec_t;
        drawcolor[2 as libc::c_int as usize] =
            (*color.offset(2 as libc::c_int as isize) as libc::c_double * 0.8f64)
                as crate::src::qcommon::q_shared::vec_t;
        drawcolor[3 as libc::c_int as usize] = *color.offset(3 as libc::c_int as isize);
        UI_DrawProportionalString2(
            x,
            y,
            str,
            color,
            sizeScale,
            crate::src::cgame::cg_main::cgs.media.charsetProp,
        );
        drawcolor[0 as libc::c_int as usize] = *color.offset(0 as libc::c_int as isize);
        drawcolor[1 as libc::c_int as usize] = *color.offset(1 as libc::c_int as isize);
        drawcolor[2 as libc::c_int as usize] = *color.offset(2 as libc::c_int as isize);
        drawcolor[3 as libc::c_int as usize] = (0.5f64
            + 0.5f64
                * crate::stdlib::sin(
                    (crate::src::cgame::cg_main::cg.time / 75 as libc::c_int) as libc::c_double,
                ))
            as crate::src::qcommon::q_shared::vec_t;
        UI_DrawProportionalString2(
            x,
            y,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            crate::src::cgame::cg_main::cgs.media.charsetPropGlow,
        );
        return;
    }
    UI_DrawProportionalString2(
        x,
        y,
        str,
        color,
        sizeScale,
        crate::src::cgame::cg_main::cgs.media.charsetProp,
    );
}

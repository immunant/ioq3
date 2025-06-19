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
    mut x: *mut f32,
    mut y: *mut f32,
    mut w: *mut f32,
    mut h: *mut f32,
) {
    // scale for screen sizes
    *x *= cgs.screenXScale;
    *y *= cgs.screenYScale;
    *w *= cgs.screenXScale;
    *h *= cgs.screenYScale;
}
/*
================
CG_FillRect

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_FillRect(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut color: *const f32,
) {
    trap_R_SetColor(color);
    CG_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    trap_R_DrawStretchPic(
        x,
        y,
        width,
        height,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        cgs.media.whiteShader,
    );
    trap_R_SetColor(0 as *const f32);
}
/*
================
CG_DrawSides

Coords are virtual 640x480
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawSides(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut size: f32,
) {
    CG_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    size *= cgs.screenXScale;
    trap_R_DrawStretchPic(
        x,
        y,
        size,
        h,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        cgs.media.whiteShader,
    );
    trap_R_DrawStretchPic(
        x + w - size,
        y,
        size,
        h,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        cgs.media.whiteShader,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawTopBottom(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut size: f32,
) {
    CG_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    size *= cgs.screenYScale;
    trap_R_DrawStretchPic(
        x,
        y,
        w,
        size,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        cgs.media.whiteShader,
    );
    trap_R_DrawStretchPic(
        x,
        y + h - size,
        w,
        size,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        cgs.media.whiteShader,
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
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut size: f32,
    mut color: *const f32,
) {
    trap_R_SetColor(color);
    CG_DrawTopBottom(x, y, width, height, size);
    CG_DrawSides(x, y + size, width, height - size * 2 as i32 as f32, size);
    trap_R_SetColor(0 as *const f32);
}
/*
================
CG_DrawPic

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawPic(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut hShader: qhandle_t,
) {
    CG_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    trap_R_DrawStretchPic(
        x,
        y,
        width,
        height,
        0 as i32 as f32,
        0 as i32 as f32,
        1 as i32 as f32,
        1 as i32 as f32,
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
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut height: i32,
    mut ch: i32,
) {
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    let mut size: f32 = 0.;
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0.;
    let mut ah: f32 = 0.;
    ch &= 255 as i32;
    if ch == ' ' as i32 {
        return;
    }
    ax = x as f32;
    ay = y as f32;
    aw = width as f32;
    ah = height as f32;
    CG_AdjustFrom640(&mut ax, &mut ay, &mut aw, &mut ah);
    row = ch >> 4 as i32;
    col = ch & 15 as i32;
    frow = (row as f64 * 0.0625f64) as f32;
    fcol = (col as f64 * 0.0625f64) as f32;
    size = 0.0625f64 as f32;
    trap_R_DrawStretchPic(
        ax,
        ay,
        aw,
        ah,
        fcol,
        frow,
        fcol + size,
        frow + size,
        cgs.media.charsetShader,
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
    mut x: i32,
    mut y: i32,
    mut string: *const libc::c_char,
    mut setColor: *const f32,
    mut forceColor: qboolean,
    mut shadow: qboolean,
    mut charWidth: i32,
    mut charHeight: i32,
    mut maxChars: i32,
) {
    let mut color: vec4_t = [0.; 4]; // do them all!
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut xx: i32 = 0;
    let mut cnt: i32 = 0;
    if maxChars <= 0 as i32 {
        maxChars = 32767 as i32
    }
    // draw the drop shadow
    if shadow as u64 != 0 {
        color[2 as i32 as usize] = 0 as i32 as vec_t;
        color[1 as i32 as usize] = color[2 as i32 as usize];
        color[0 as i32 as usize] = color[1 as i32 as usize];
        color[3 as i32 as usize] = *setColor.offset(3 as i32 as isize);
        trap_R_SetColor(color.as_mut_ptr());
        s = string;
        xx = x;
        cnt = 0 as i32;
        while *s as i32 != 0 && cnt < maxChars {
            if Q_IsColorString(s) as u64 != 0 {
                s = s.offset(2 as i32 as isize)
            } else {
                CG_DrawChar(
                    xx + 2 as i32,
                    y + 2 as i32,
                    charWidth,
                    charHeight,
                    *s as i32,
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
    cnt = 0 as i32;
    trap_R_SetColor(setColor);
    while *s as i32 != 0 && cnt < maxChars {
        if Q_IsColorString(s) as u64 != 0 {
            if forceColor as u64 == 0 {
                crate::stdlib::memcpy(
                    color.as_mut_ptr() as *mut libc::c_void,
                    g_color_table
                        [(*s.offset(1 as i32 as isize) as i32 - '0' as i32 & 0x7 as i32) as usize]
                        .as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<vec4_t>() as libc::c_ulong,
                );
                color[3 as i32 as usize] = *setColor.offset(3 as i32 as isize);
                trap_R_SetColor(color.as_mut_ptr());
            }
            s = s.offset(2 as i32 as isize)
        } else {
            CG_DrawChar(xx, y, charWidth, charHeight, *s as i32);
            xx += charWidth;
            cnt += 1;
            s = s.offset(1)
        }
    }
    trap_R_SetColor(0 as *const f32);
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawBigString(
    mut x: i32,
    mut y: i32,
    mut s: *const libc::c_char,
    mut alpha: f32,
) {
    let mut color: [f32; 4] = [0.; 4];
    color[2 as i32 as usize] = 1.0f64 as f32;
    color[1 as i32 as usize] = color[2 as i32 as usize];
    color[0 as i32 as usize] = color[1 as i32 as usize];
    color[3 as i32 as usize] = alpha;
    CG_DrawStringExt(
        x,
        y,
        s,
        color.as_mut_ptr(),
        qfalse,
        qtrue,
        16 as i32,
        16 as i32,
        0 as i32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawBigStringColor(
    mut x: i32,
    mut y: i32,
    mut s: *const libc::c_char,
    mut color: *mut vec_t,
) {
    CG_DrawStringExt(
        x,
        y,
        s,
        color as *const f32,
        qtrue,
        qtrue,
        16 as i32,
        16 as i32,
        0 as i32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawSmallString(
    mut x: i32,
    mut y: i32,
    mut s: *const libc::c_char,
    mut alpha: f32,
) {
    let mut color: [f32; 4] = [0.; 4];
    color[2 as i32 as usize] = 1.0f64 as f32;
    color[1 as i32 as usize] = color[2 as i32 as usize];
    color[0 as i32 as usize] = color[1 as i32 as usize];
    color[3 as i32 as usize] = alpha;
    CG_DrawStringExt(
        x,
        y,
        s,
        color.as_mut_ptr(),
        qfalse,
        qfalse,
        8 as i32,
        16 as i32,
        0 as i32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawSmallStringColor(
    mut x: i32,
    mut y: i32,
    mut s: *const libc::c_char,
    mut color: *mut vec_t,
) {
    CG_DrawStringExt(
        x,
        y,
        s,
        color as *const f32,
        qtrue,
        qfalse,
        8 as i32,
        16 as i32,
        0 as i32,
    );
}
/*
=================
CG_DrawStrlen

Returns character count, skiping color escape codes
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawStrlen(mut str: *const libc::c_char) -> i32 {
    let mut s: *const libc::c_char = str;
    let mut count: i32 = 0 as i32;
    while *s != 0 {
        if Q_IsColorString(s) as u64 != 0 {
            s = s.offset(2 as i32 as isize)
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
    mut x: i32,
    mut y: i32,
    mut w: i32,
    mut h: i32,
    mut hShader: qhandle_t,
) {
    let mut s1: f32 = 0.;
    let mut t1: f32 = 0.;
    let mut s2: f32 = 0.;
    let mut t2: f32 = 0.;
    s1 = (x as f64 / 64.0f64) as f32;
    t1 = (y as f64 / 64.0f64) as f32;
    s2 = ((x + w) as f64 / 64.0f64) as f32;
    t2 = ((y + h) as f64 / 64.0f64) as f32;
    trap_R_DrawStretchPic(
        x as f32, y as f32, w as f32, h as f32, s1, t1, s2, t2, hShader,
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
    let mut top: i32 = 0;
    let mut bottom: i32 = 0;
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    w = cgs.glconfig.vidWidth;
    h = cgs.glconfig.vidHeight;
    if cg.refdef.x == 0 as i32
        && cg.refdef.y == 0 as i32
        && cg.refdef.width == w
        && cg.refdef.height == h
    {
        return;
        // full screen rendering
    }
    top = cg.refdef.y;
    bottom = top + cg.refdef.height - 1 as i32;
    left = cg.refdef.x;
    right = left + cg.refdef.width - 1 as i32;
    // clear above view screen
    CG_TileClearBox(
        0 as i32,
        0 as i32,
        w,
        top,
        cgs.media.backTileShader,
    );
    // clear below view screen
    CG_TileClearBox(
        0 as i32,
        bottom,
        w,
        h - bottom,
        cgs.media.backTileShader,
    );
    // clear left of view screen
    CG_TileClearBox(
        0 as i32,
        top,
        left,
        bottom - top + 1 as i32,
        cgs.media.backTileShader,
    );
    // clear right of view screen
    CG_TileClearBox(
        right,
        top,
        w - right,
        bottom - top + 1 as i32,
        cgs.media.backTileShader,
    );
}
/*
================
CG_FadeColor
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_FadeColor(mut startMsec: i32, mut totalMsec: i32) -> *mut f32 {
    static mut color: vec4_t = [0.; 4];
    let mut t: i32 = 0;
    if startMsec == 0 as i32 {
        return 0 as *mut f32;
    }
    t = cg.time - startMsec;
    if t >= totalMsec {
        return 0 as *mut f32;
    }
    // fade out
    if totalMsec - t < 200 as i32 {
        color[3 as i32 as usize] = ((totalMsec - t) as f64 * 1.0f64 / 200 as i32 as f64)
            as vec_t
    } else {
        color[3 as i32 as usize] = 1.0f64 as vec_t
    }
    color[2 as i32 as usize] = 1 as i32 as vec_t;
    color[1 as i32 as usize] = color[2 as i32 as usize];
    color[0 as i32 as usize] = color[1 as i32 as usize];
    return color.as_mut_ptr();
}
/*
================
CG_TeamColor
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_TeamColor(mut team: i32) -> *mut f32 {
    static mut red: vec4_t = [
        1 as i32 as vec_t,
        0.2f32,
        0.2f32,
        1 as i32 as vec_t,
    ];
    static mut blue: vec4_t = [
        0.2f32,
        0.2f32,
        1 as i32 as vec_t,
        1 as i32 as vec_t,
    ];
    static mut other: vec4_t = [
        1 as i32 as vec_t,
        1 as i32 as vec_t,
        1 as i32 as vec_t,
        1 as i32 as vec_t,
    ];
    static mut spectator: vec4_t = [
        0.7f32,
        0.7f32,
        0.7f32,
        1 as i32 as vec_t,
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
    mut health: i32,
    mut armor: i32,
    mut hcolor: *mut vec_t,
) {
    let mut count: i32 = 0;
    let mut max: i32 = 0;
    // calculate the total points of damage that can
    // be sustained at the current health / armor level
    if health <= 0 as i32 {
        let ref mut fresh0 = *hcolor.offset(2 as i32 as isize); // black
        *fresh0 = 0 as i32 as vec_t;
        let ref mut fresh1 = *hcolor.offset(1 as i32 as isize);
        *fresh1 = *fresh0;
        *hcolor.offset(0 as i32 as isize) = *fresh1;
        *hcolor.offset(3 as i32 as isize) = 1 as i32 as vec_t;
        return;
    }
    count = armor;
    max = (health as f64 * 0.66f64 / (1.0f64 - 0.66f64)) as i32;
    if max < count {
        count = max
    }
    health += count;
    // set the color based on health
    *hcolor.offset(0 as i32 as isize) = 1.0f64 as vec_t;
    *hcolor.offset(3 as i32 as isize) = 1.0f64 as vec_t;
    if health >= 100 as i32 {
        *hcolor.offset(2 as i32 as isize) = 1.0f64 as vec_t
    } else if health < 66 as i32 {
        *hcolor.offset(2 as i32 as isize) = 0 as i32 as vec_t
    } else {
        *hcolor.offset(2 as i32 as isize) =
            ((health - 66 as i32) as f64 / 33.0f64) as vec_t
    }
    if health > 60 as i32 {
        *hcolor.offset(1 as i32 as isize) = 1.0f64 as vec_t
    } else if health < 30 as i32 {
        *hcolor.offset(1 as i32 as isize) = 0 as i32 as vec_t
    } else {
        *hcolor.offset(1 as i32 as isize) =
            ((health - 30 as i32) as f64 / 30.0f64) as vec_t
    };
}
/*
=================
CG_ColorForHealth
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ColorForHealth(mut hcolor: *mut vec_t) {
    CG_GetColorForHealth(
        (*cg.snap).ps.stats
            [STAT_HEALTH as i32 as usize],
        (*cg.snap).ps.stats
            [STAT_ARMOR as i32 as usize],
        hcolor,
    );
}
/*
=================
UI_DrawProportionalString2
=================
*/

static mut propMap: [[i32; 3]; 128] = [
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, -(1 as i32)],
    [0 as i32, 0 as i32, 8 as i32],
    [11 as i32, 122 as i32, 7 as i32],
    [154 as i32, 181 as i32, 14 as i32],
    [55 as i32, 122 as i32, 17 as i32],
    [79 as i32, 122 as i32, 18 as i32],
    [101 as i32, 122 as i32, 23 as i32],
    [153 as i32, 122 as i32, 18 as i32],
    [9 as i32, 93 as i32, 7 as i32],
    [207 as i32, 122 as i32, 8 as i32],
    [230 as i32, 122 as i32, 9 as i32],
    [177 as i32, 122 as i32, 18 as i32],
    [30 as i32, 152 as i32, 18 as i32],
    [85 as i32, 181 as i32, 7 as i32],
    [34 as i32, 93 as i32, 11 as i32],
    [110 as i32, 181 as i32, 6 as i32],
    [130 as i32, 152 as i32, 14 as i32],
    [22 as i32, 64 as i32, 17 as i32],
    [41 as i32, 64 as i32, 12 as i32],
    [58 as i32, 64 as i32, 17 as i32],
    [78 as i32, 64 as i32, 18 as i32],
    [98 as i32, 64 as i32, 19 as i32],
    [120 as i32, 64 as i32, 18 as i32],
    [141 as i32, 64 as i32, 18 as i32],
    [204 as i32, 64 as i32, 16 as i32],
    [162 as i32, 64 as i32, 17 as i32],
    [182 as i32, 64 as i32, 18 as i32],
    [59 as i32, 181 as i32, 7 as i32],
    [35 as i32, 181 as i32, 7 as i32],
    [203 as i32, 152 as i32, 14 as i32],
    [56 as i32, 93 as i32, 14 as i32],
    [228 as i32, 152 as i32, 14 as i32],
    [177 as i32, 181 as i32, 18 as i32],
    [28 as i32, 122 as i32, 22 as i32],
    [5 as i32, 4 as i32, 18 as i32],
    [27 as i32, 4 as i32, 18 as i32],
    [48 as i32, 4 as i32, 18 as i32],
    [69 as i32, 4 as i32, 17 as i32],
    [90 as i32, 4 as i32, 13 as i32],
    [106 as i32, 4 as i32, 13 as i32],
    [121 as i32, 4 as i32, 18 as i32],
    [143 as i32, 4 as i32, 17 as i32],
    [164 as i32, 4 as i32, 8 as i32],
    [175 as i32, 4 as i32, 16 as i32],
    [195 as i32, 4 as i32, 18 as i32],
    [216 as i32, 4 as i32, 12 as i32],
    [230 as i32, 4 as i32, 23 as i32],
    [6 as i32, 34 as i32, 18 as i32],
    [27 as i32, 34 as i32, 18 as i32],
    [48 as i32, 34 as i32, 18 as i32],
    [68 as i32, 34 as i32, 18 as i32],
    [90 as i32, 34 as i32, 17 as i32],
    [110 as i32, 34 as i32, 18 as i32],
    [130 as i32, 34 as i32, 14 as i32],
    [146 as i32, 34 as i32, 18 as i32],
    [166 as i32, 34 as i32, 19 as i32],
    [185 as i32, 34 as i32, 29 as i32],
    [215 as i32, 34 as i32, 18 as i32],
    [234 as i32, 34 as i32, 18 as i32],
    [5 as i32, 64 as i32, 14 as i32],
    [60 as i32, 152 as i32, 7 as i32],
    [106 as i32, 151 as i32, 13 as i32],
    [83 as i32, 152 as i32, 7 as i32],
    [128 as i32, 122 as i32, 17 as i32],
    [4 as i32, 152 as i32, 21 as i32],
    [134 as i32, 181 as i32, 5 as i32],
    [5 as i32, 4 as i32, 18 as i32],
    [27 as i32, 4 as i32, 18 as i32],
    [48 as i32, 4 as i32, 18 as i32],
    [69 as i32, 4 as i32, 17 as i32],
    [90 as i32, 4 as i32, 13 as i32],
    [106 as i32, 4 as i32, 13 as i32],
    [121 as i32, 4 as i32, 18 as i32],
    [143 as i32, 4 as i32, 17 as i32],
    [164 as i32, 4 as i32, 8 as i32],
    [175 as i32, 4 as i32, 16 as i32],
    [195 as i32, 4 as i32, 18 as i32],
    [216 as i32, 4 as i32, 12 as i32],
    [230 as i32, 4 as i32, 23 as i32],
    [6 as i32, 34 as i32, 18 as i32],
    [27 as i32, 34 as i32, 18 as i32],
    [48 as i32, 34 as i32, 18 as i32],
    [68 as i32, 34 as i32, 18 as i32],
    [90 as i32, 34 as i32, 17 as i32],
    [110 as i32, 34 as i32, 18 as i32],
    [130 as i32, 34 as i32, 14 as i32],
    [146 as i32, 34 as i32, 18 as i32],
    [166 as i32, 34 as i32, 19 as i32],
    [185 as i32, 34 as i32, 29 as i32],
    [215 as i32, 34 as i32, 18 as i32],
    [234 as i32, 34 as i32, 18 as i32],
    [5 as i32, 64 as i32, 14 as i32],
    [153 as i32, 152 as i32, 13 as i32],
    [11 as i32, 181 as i32, 5 as i32],
    [180 as i32, 152 as i32, 13 as i32],
    [79 as i32, 93 as i32, 17 as i32],
    [0 as i32, 0 as i32, -(1 as i32)],
];

static mut propMapB: [[i32; 3]; 26] = [
    [11 as i32, 12 as i32, 33 as i32],
    [49 as i32, 12 as i32, 31 as i32],
    [85 as i32, 12 as i32, 31 as i32],
    [120 as i32, 12 as i32, 30 as i32],
    [156 as i32, 12 as i32, 21 as i32],
    [183 as i32, 12 as i32, 21 as i32],
    [207 as i32, 12 as i32, 32 as i32],
    [13 as i32, 55 as i32, 30 as i32],
    [49 as i32, 55 as i32, 13 as i32],
    [66 as i32, 55 as i32, 29 as i32],
    [101 as i32, 55 as i32, 31 as i32],
    [135 as i32, 55 as i32, 21 as i32],
    [158 as i32, 55 as i32, 40 as i32],
    [204 as i32, 55 as i32, 32 as i32],
    [12 as i32, 97 as i32, 31 as i32],
    [48 as i32, 97 as i32, 31 as i32],
    [82 as i32, 97 as i32, 30 as i32],
    [118 as i32, 97 as i32, 30 as i32],
    [153 as i32, 97 as i32, 30 as i32],
    [185 as i32, 97 as i32, 25 as i32],
    [213 as i32, 97 as i32, 30 as i32],
    [11 as i32, 139 as i32, 32 as i32],
    [42 as i32, 139 as i32, 51 as i32],
    [93 as i32, 139 as i32, 32 as i32],
    [126 as i32, 139 as i32, 31 as i32],
    [158 as i32, 139 as i32, 25 as i32],
];
/*
=================
UI_DrawBannerString
=================
*/

unsafe extern "C" fn UI_DrawBannerString2(
    mut x: i32,
    mut y: i32,
    mut str: *const libc::c_char,
    mut color: *mut vec_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: u8 = 0;
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0.;
    let mut ah: f32 = 0.;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    let mut fwidth: f32 = 0.;
    let mut fheight: f32 = 0.;
    // draw the colored text
    trap_R_SetColor(color as *const f32);
    ax = x as f32 * cgs.screenXScale
        + cgs.screenXBias;
    ay = y as f32 * cgs.screenYScale;
    s = str;
    while *s != 0 {
        ch = (*s as i32 & 127 as i32) as u8;
        if ch as i32 == ' ' as i32 {
            ax +=
                (12 as i32 as f32 + 4 as i32 as f32) * cgs.screenXScale
        } else if ch as i32 >= 'A' as i32 && ch as i32 <= 'Z' as i32 {
            ch = (ch as i32 - 'A' as i32) as u8;
            fcol = propMapB[ch as usize][0 as i32 as usize] as f32 / 256.0f32;
            frow = propMapB[ch as usize][1 as i32 as usize] as f32 / 256.0f32;
            fwidth = propMapB[ch as usize][2 as i32 as usize] as f32 / 256.0f32;
            fheight = 36 as i32 as f32 / 256.0f32;
            aw = propMapB[ch as usize][2 as i32 as usize] as f32
                * cgs.screenXScale;
            ah = 36 as i32 as f32 * cgs.screenYScale;
            trap_R_DrawStretchPic(
                ax,
                ay,
                aw,
                ah,
                fcol,
                frow,
                fcol + fwidth,
                frow + fheight,
                cgs.media.charsetPropB,
            );
            ax += aw + 4 as i32 as f32 * cgs.screenXScale
        }
        s = s.offset(1)
    }
    trap_R_SetColor(0 as *const f32);
}
#[no_mangle]

pub unsafe extern "C" fn UI_DrawBannerString(
    mut x: i32,
    mut y: i32,
    mut str: *const libc::c_char,
    mut style: i32,
    mut color: *mut vec_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: i32 = 0;
    let mut width: i32 = 0;
    let mut drawcolor: vec4_t = [0.; 4];
    // find the width of the drawn text
    s = str;
    width = 0 as i32;
    while *s != 0 {
        ch = *s as i32;
        if ch == ' ' as i32 {
            width += 12 as i32
        } else if ch >= 'A' as i32 && ch <= 'Z' as i32 {
            width += propMapB[(ch - 'A' as i32) as usize][2 as i32 as usize] + 4 as i32
        }
        s = s.offset(1)
    }
    width -= 4 as i32;
    match style & 0x7 as i32 {
        1 => x -= width / 2 as i32,
        2 => x -= width,
        0 | _ => {}
    }
    if style & 0x800 as i32 != 0 {
        drawcolor[2 as i32 as usize] = 0 as i32 as vec_t;
        drawcolor[1 as i32 as usize] = drawcolor[2 as i32 as usize];
        drawcolor[0 as i32 as usize] = drawcolor[1 as i32 as usize];
        drawcolor[3 as i32 as usize] = *color.offset(3 as i32 as isize);
        UI_DrawBannerString2(x + 2 as i32, y + 2 as i32, str, drawcolor.as_mut_ptr());
    }
    UI_DrawBannerString2(x, y, str, color);
}
#[no_mangle]

pub unsafe extern "C" fn UI_ProportionalStringWidth(mut str: *const libc::c_char) -> i32 {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: i32 = 0;
    let mut charWidth: i32 = 0;
    let mut width: i32 = 0;
    s = str;
    width = 0 as i32;
    while *s != 0 {
        ch = *s as i32 & 127 as i32;
        charWidth = propMap[ch as usize][2 as i32 as usize];
        if charWidth != -(1 as i32) {
            width += charWidth;
            width += 3 as i32
        }
        s = s.offset(1)
    }
    width -= 3 as i32;
    return width;
}

unsafe extern "C" fn UI_DrawProportionalString2(
    mut x: i32,
    mut y: i32,
    mut str: *const libc::c_char,
    mut color: *mut vec_t,
    mut sizeScale: f32,
    mut charset: qhandle_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: u8 = 0;
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0.;
    let mut ah: f32 = 0.;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    let mut fwidth: f32 = 0.;
    let mut fheight: f32 = 0.;
    // draw the colored text
    trap_R_SetColor(color as *const f32);
    ax = x as f32 * cgs.screenXScale
        + cgs.screenXBias;
    ay = y as f32 * cgs.screenYScale;
    s = str;
    while *s != 0 {
        ch = (*s as i32 & 127 as i32) as u8;
        if ch as i32 == ' ' as i32 {
            aw = 8 as i32 as f32 * cgs.screenXScale * sizeScale
        } else if propMap[ch as usize][2 as i32 as usize] != -(1 as i32) {
            fcol = propMap[ch as usize][0 as i32 as usize] as f32 / 256.0f32;
            frow = propMap[ch as usize][1 as i32 as usize] as f32 / 256.0f32;
            fwidth = propMap[ch as usize][2 as i32 as usize] as f32 / 256.0f32;
            fheight = 27 as i32 as f32 / 256.0f32;
            aw = propMap[ch as usize][2 as i32 as usize] as f32
                * cgs.screenXScale
                * sizeScale;
            ah = 27 as i32 as f32 * cgs.screenYScale * sizeScale;
            trap_R_DrawStretchPic(
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
            aw = 0 as i32 as f32
        }
        ax += aw + 3 as i32 as f32 * cgs.screenXScale * sizeScale;
        s = s.offset(1)
    }
    trap_R_SetColor(0 as *const f32);
}
/*
=================
UI_ProportionalSizeScale
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ProportionalSizeScale(mut style: i32) -> f32 {
    if style & 0x10 as i32 != 0 {
        return 0.75f64 as f32;
    }
    return 1.00f64 as f32;
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
    mut x: i32,
    mut y: i32,
    mut str: *const libc::c_char,
    mut style: i32,
    mut color: *mut vec_t,
) {
    let mut drawcolor: vec4_t = [0.; 4];
    let mut width: i32 = 0;
    let mut sizeScale: f32 = 0.;
    sizeScale = UI_ProportionalSizeScale(style);
    match style & 0x7 as i32 {
        1 => {
            width = (UI_ProportionalStringWidth(str) as f32 * sizeScale) as i32;
            x -= width / 2 as i32
        }
        2 => {
            width = (UI_ProportionalStringWidth(str) as f32 * sizeScale) as i32;
            x -= width
        }
        0 | _ => {}
    }
    if style & 0x800 as i32 != 0 {
        drawcolor[2 as i32 as usize] = 0 as i32 as vec_t;
        drawcolor[1 as i32 as usize] = drawcolor[2 as i32 as usize];
        drawcolor[0 as i32 as usize] = drawcolor[1 as i32 as usize];
        drawcolor[3 as i32 as usize] = *color.offset(3 as i32 as isize);
        UI_DrawProportionalString2(
            x + 2 as i32,
            y + 2 as i32,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            cgs.media.charsetProp,
        );
    }
    if style & 0x2000 as i32 != 0 {
        drawcolor[0 as i32 as usize] = (*color.offset(0 as i32 as isize) as f64 * 0.8f64)
            as vec_t;
        drawcolor[1 as i32 as usize] = (*color.offset(1 as i32 as isize) as f64 * 0.8f64)
            as vec_t;
        drawcolor[2 as i32 as usize] = (*color.offset(2 as i32 as isize) as f64 * 0.8f64)
            as vec_t;
        drawcolor[3 as i32 as usize] = *color.offset(3 as i32 as isize);
        UI_DrawProportionalString2(
            x,
            y,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            cgs.media.charsetProp,
        );
        return;
    }
    if style & 0x4000 as i32 != 0 {
        drawcolor[0 as i32 as usize] = (*color.offset(0 as i32 as isize) as f64 * 0.8f64)
            as vec_t;
        drawcolor[1 as i32 as usize] = (*color.offset(1 as i32 as isize) as f64 * 0.8f64)
            as vec_t;
        drawcolor[2 as i32 as usize] = (*color.offset(2 as i32 as isize) as f64 * 0.8f64)
            as vec_t;
        drawcolor[3 as i32 as usize] = *color.offset(3 as i32 as isize);
        UI_DrawProportionalString2(
            x,
            y,
            str,
            color,
            sizeScale,
            cgs.media.charsetProp,
        );
        drawcolor[0 as i32 as usize] = *color.offset(0 as i32 as isize);
        drawcolor[1 as i32 as usize] = *color.offset(1 as i32 as isize);
        drawcolor[2 as i32 as usize] = *color.offset(2 as i32 as isize);
        drawcolor[3 as i32 as usize] = (0.5f64
            + 0.5f64 * crate::stdlib::sin((cg.time / 75 as i32) as f64))
            as vec_t;
        UI_DrawProportionalString2(
            x,
            y,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            cgs.media.charsetPropGlow,
        );
        return;
    }
    UI_DrawProportionalString2(
        x,
        y,
        str,
        color,
        sizeScale,
        cgs.media.charsetProp,
    );
}

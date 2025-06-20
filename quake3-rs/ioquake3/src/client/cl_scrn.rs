use ::libc;

pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::uint8_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::curl_h::CURL;
pub use crate::multi_h::CURLM;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::vm_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::qcommon_h::NS_CLIENT;
pub use crate::qcommon_h::NS_SERVER;
pub use crate::src::qcommon::common::com_dedicated;
pub use crate::src::qcommon::common::com_speeds;
pub use crate::src::qcommon::common::time_backend;
pub use crate::src::qcommon::common::time_frontend;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_VariableIntegerValue;
pub use crate::src::qcommon::files::FS_FTell;
pub use crate::src::qcommon::q_math::g_color_table;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fontInfo_t;
pub use crate::src::qcommon::q_shared::glyphInfo_t;
pub use crate::src::qcommon::q_shared::markFragment_t;
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_IsColorString;
pub use crate::src::qcommon::q_shared::CA_ACTIVE;
pub use crate::src::qcommon::q_shared::CA_AUTHORIZING;
pub use crate::src::qcommon::q_shared::CA_CHALLENGING;
pub use crate::src::qcommon::q_shared::CA_CINEMATIC;
pub use crate::src::qcommon::q_shared::CA_CONNECTED;
pub use crate::src::qcommon::q_shared::CA_CONNECTING;
pub use crate::src::qcommon::q_shared::CA_DISCONNECTED;
pub use crate::src::qcommon::q_shared::CA_LOADING;
pub use crate::src::qcommon::q_shared::CA_PRIMED;
pub use crate::src::qcommon::q_shared::CA_UNINITIALIZED;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::vm::VM_Call;
pub use crate::tr_public_h::refexport_t;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::polyVert_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
pub use crate::tr_types_h::stereoFrame_t;
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
pub use crate::tr_types_h::STEREO_CENTER;
pub use crate::tr_types_h::STEREO_LEFT;
pub use crate::tr_types_h::STEREO_RIGHT;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
pub use crate::ui_public_h::UIMENU_BAD_CD_KEY;
pub use crate::ui_public_h::UIMENU_INGAME;
pub use crate::ui_public_h::UIMENU_MAIN;
pub use crate::ui_public_h::UIMENU_NEED_CD;
pub use crate::ui_public_h::UIMENU_NONE;
pub use crate::ui_public_h::UIMENU_POSTGAME;
pub use crate::ui_public_h::UIMENU_TEAM;
pub use crate::ui_public_h::UI_CONSOLE_COMMAND;
pub use crate::ui_public_h::UI_DRAW_CONNECT_SCREEN;
pub use crate::ui_public_h::UI_GETAPIVERSION;
pub use crate::ui_public_h::UI_HASUNIQUECDKEY;
pub use crate::ui_public_h::UI_INIT;
pub use crate::ui_public_h::UI_IS_FULLSCREEN;
pub use crate::ui_public_h::UI_KEY_EVENT;
pub use crate::ui_public_h::UI_MOUSE_EVENT;
pub use crate::ui_public_h::UI_REFRESH;
pub use crate::ui_public_h::UI_SET_ACTIVE_MENU;
pub use crate::ui_public_h::UI_SHUTDOWN;
pub use crate::vm_local_h::vm_s;

pub use crate::client_h::clientConnection_t;
pub use crate::client_h::clientStatic_t;
pub use crate::client_h::serverInfo_t;
pub use crate::src::client::cl_cgame::CL_CGameRendering;
pub use crate::src::client::cl_cin::SCR_DrawCinematic;
pub use crate::src::client::cl_console::Con_DrawConsole;
pub use crate::src::client::cl_keys::Key_GetCatcher;
pub use crate::src::client::cl_main::cl_debugMove;
pub use crate::src::client::cl_main::cl_voip;
pub use crate::src::client::cl_main::cl_voipSend;
pub use crate::src::client::cl_main::cl_voipShowMeter;
pub use crate::src::client::cl_main::clc;
pub use crate::src::client::cl_main::cls;
pub use crate::src::client::cl_main::re;
pub use crate::src::client::cl_ui::uivm;

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
// cl_scrn.c -- master for refresh, status bar, console, chat, notify, etc
#[no_mangle]

pub static mut scr_initialized: qboolean = qfalse;
// ready to draw
#[no_mangle]

pub static mut cl_timegraph: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut cl_debuggraph: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut cl_graphheight: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut cl_graphscale: *mut cvar_t = std::ptr::null_mut();
#[no_mangle]

pub static mut cl_graphshift: *mut cvar_t = std::ptr::null_mut();
/*
================
SCR_DrawNamedPic

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_DrawNamedPic(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut picname: *const libc::c_char,
) {
    let mut hShader: qhandle_t = 0;
    hShader = re.RegisterShader.expect("non-null function pointer")(picname);
    SCR_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    re.DrawStretchPic.expect("non-null function pointer")(
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
================
SCR_AdjustFrom640

Adjusted for resolution and screen aspect ratio
================
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_AdjustFrom640(
    mut x: *mut f32,
    mut y: *mut f32,
    mut w: *mut f32,
    mut h: *mut f32,
) {
    let mut xscale: f32 = 0.;
    let mut yscale: f32 = 0.;
    // scale for screen sizes
    xscale = (cls.glconfig.vidWidth as f64 / 640.0f64) as f32;
    yscale = (cls.glconfig.vidHeight as f64 / 480.0f64) as f32;
    if !x.is_null() {
        *x *= xscale
    }
    if !y.is_null() {
        *y *= yscale
    }
    if !w.is_null() {
        *w *= xscale
    }
    if !h.is_null() {
        *h *= yscale
    };
}
/*
================
SCR_FillRect

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_FillRect(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut color: *const f32,
) {
    re.SetColor.expect("non-null function pointer")(color);
    SCR_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    re.DrawStretchPic.expect("non-null function pointer")(
        x,
        y,
        width,
        height,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        cls.whiteShader,
    );
    re.SetColor.expect("non-null function pointer")(0 as *const f32);
}
/*
================
SCR_DrawPic

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_DrawPic(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut hShader: qhandle_t,
) {
    SCR_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    re.DrawStretchPic.expect("non-null function pointer")(
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
** SCR_DrawChar
** chars are drawn at 640*480 virtual screen size
*/

unsafe extern "C" fn SCR_DrawChar(mut x: i32, mut y: i32, mut size: f32, mut ch: i32) {
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0.;
    let mut ah: f32 = 0.;
    ch &= 255 as i32;
    if ch == ' ' as i32 {
        return;
    }
    if (y as f32) < -size {
        return;
    }
    ax = x as f32;
    ay = y as f32;
    aw = size;
    ah = size;
    SCR_AdjustFrom640(&mut ax, &mut ay, &mut aw, &mut ah);
    row = ch >> 4 as i32;
    col = ch & 15 as i32;
    frow = (row as f64 * 0.0625f64) as f32;
    fcol = (col as f64 * 0.0625f64) as f32;
    size = 0.0625f64 as f32;
    re.DrawStretchPic.expect("non-null function pointer")(
        ax,
        ay,
        aw,
        ah,
        fcol,
        frow,
        fcol + size,
        frow + size,
        cls.charSetShader,
    );
}
/*
** SCR_DrawSmallChar
** small chars are drawn at native screen resolution
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_DrawSmallChar(mut x: i32, mut y: i32, mut ch: i32) {
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    let mut size: f32 = 0.;
    ch &= 255 as i32;
    if ch == ' ' as i32 {
        return;
    }
    if y < -(16 as i32) {
        return;
    }
    row = ch >> 4 as i32;
    col = ch & 15 as i32;
    frow = (row as f64 * 0.0625f64) as f32;
    fcol = (col as f64 * 0.0625f64) as f32;
    size = 0.0625f64 as f32;
    re.DrawStretchPic.expect("non-null function pointer")(
        x as f32,
        y as f32,
        8 as i32 as f32,
        16 as i32 as f32,
        fcol,
        frow,
        fcol + size,
        frow + size,
        cls.charSetShader,
    );
}
/*
==================
SCR_DrawBigString[Color]

Draws a multi-colored string with a drop shadow, optionally forcing
to a fixed color.

Coordinates are at 640 by 480 virtual resolution
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_DrawStringExt(
    mut x: i32,
    mut y: i32,
    mut size: f32,
    mut string: *const libc::c_char,
    mut setColor: *mut f32,
    mut forceColor: qboolean,
    mut noColorEscape: qboolean,
) {
    let mut color: vec4_t = [0.; 4];
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut xx: i32 = 0;
    // draw the drop shadow
    color[2 as i32 as usize] = 0 as i32 as vec_t;
    color[1 as i32 as usize] = color[2 as i32 as usize];
    color[0 as i32 as usize] = color[1 as i32 as usize];
    color[3 as i32 as usize] = *setColor.offset(3 as i32 as isize);
    re.SetColor.expect("non-null function pointer")(color.as_mut_ptr());
    s = string;
    xx = x;
    while *s != 0 {
        if noColorEscape as u64 == 0 && Q_IsColorString(s) as u32 != 0 {
            s = s.offset(2 as i32 as isize)
        } else {
            SCR_DrawChar(xx + 2 as i32, y + 2 as i32, size, *s as i32);
            xx = (xx as f32 + size) as i32;
            s = s.offset(1)
        }
    }
    // draw the colored text
    s = string;
    xx = x;
    re.SetColor.expect("non-null function pointer")(setColor);
    while *s != 0 {
        if Q_IsColorString(s) as u64 != 0 {
            if forceColor as u64 == 0 {
                crate::stdlib::memcpy(
                    color.as_mut_ptr() as *mut libc::c_void,
                    g_color_table
                        [(*s.offset(1 as i32 as isize) as i32 - '0' as i32 & 0x7 as i32) as usize]
                        .as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<vec4_t>() as usize,
                );
                color[3 as i32 as usize] = *setColor.offset(3 as i32 as isize);
                re.SetColor.expect("non-null function pointer")(color.as_mut_ptr());
            }
            if noColorEscape as u64 == 0 {
                s = s.offset(2 as i32 as isize);
                continue;
            }
        }
        SCR_DrawChar(xx, y, size, *s as i32);
        xx = (xx as f32 + size) as i32;
        s = s.offset(1)
    }
    re.SetColor.expect("non-null function pointer")(0 as *const f32);
}
#[no_mangle]

pub unsafe extern "C" fn SCR_DrawBigString(
    mut x: i32,
    mut y: i32,
    mut s: *const libc::c_char,
    mut alpha: f32,
    mut noColorEscape: qboolean,
) {
    let mut color: [f32; 4] = [0.; 4];
    color[2 as i32 as usize] = 1.0f64 as f32;
    color[1 as i32 as usize] = color[2 as i32 as usize];
    color[0 as i32 as usize] = color[1 as i32 as usize];
    color[3 as i32 as usize] = alpha;
    SCR_DrawStringExt(
        x,
        y,
        16 as i32 as f32,
        s,
        color.as_mut_ptr(),
        qfalse,
        noColorEscape,
    );
}
#[no_mangle]

pub unsafe extern "C" fn SCR_DrawBigStringColor(
    mut x: i32,
    mut y: i32,
    mut s: *const libc::c_char,
    mut color: *mut vec_t,
    mut noColorEscape: qboolean,
) {
    SCR_DrawStringExt(x, y, 16 as i32 as f32, s, color, qtrue, noColorEscape);
}
// returns in virtual 640x480 coordinates
// draws a string with embedded color control characters with fade
// ignores embedded color control characters
/*
==================
SCR_DrawSmallString[Color]

Draws a multi-colored string with a drop shadow, optionally forcing
to a fixed color.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_DrawSmallStringExt(
    mut x: i32,
    mut y: i32,
    mut string: *const libc::c_char,
    mut setColor: *mut f32,
    mut forceColor: qboolean,
    mut noColorEscape: qboolean,
) {
    let mut color: vec4_t = [0.; 4];
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut xx: i32 = 0;
    // draw the colored text
    s = string;
    xx = x;
    re.SetColor.expect("non-null function pointer")(setColor);
    while *s != 0 {
        if Q_IsColorString(s) as u64 != 0 {
            if forceColor as u64 == 0 {
                crate::stdlib::memcpy(
                    color.as_mut_ptr() as *mut libc::c_void,
                    g_color_table
                        [(*s.offset(1 as i32 as isize) as i32 - '0' as i32 & 0x7 as i32) as usize]
                        .as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<vec4_t>() as usize,
                );
                color[3 as i32 as usize] = *setColor.offset(3 as i32 as isize);
                re.SetColor.expect("non-null function pointer")(color.as_mut_ptr());
            }
            if noColorEscape as u64 == 0 {
                s = s.offset(2 as i32 as isize);
                continue;
            }
        }
        SCR_DrawSmallChar(xx, y, *s as i32);
        xx += 8 as i32;
        s = s.offset(1)
    }
    re.SetColor.expect("non-null function pointer")(0 as *const f32);
}
/*
** SCR_Strlen -- skips color escape codes
*/

unsafe extern "C" fn SCR_Strlen(mut str: *const libc::c_char) -> i32 {
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
** SCR_GetBigStringWidth
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_GetBigStringWidth(mut str: *const libc::c_char) -> i32 {
    return SCR_Strlen(str) * 16 as i32;
}
//===============================================================================
/*
=================
SCR_DrawDemoRecording
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_DrawDemoRecording() {
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut pos: i32 = 0;
    if clc.demorecording as u64 == 0 {
        return;
    }
    if clc.spDemoRecording as u64 != 0 {
        return;
    }
    pos = FS_FTell(clc.demofile);
    libc::sprintf(
        string.as_mut_ptr(),
        b"RECORDING %s: %ik\x00" as *const u8 as *const libc::c_char,
        clc.demoName.as_mut_ptr(),
        pos / 1024 as i32,
    );
    SCR_DrawStringExt(
        (320 as i32 as usize).wrapping_sub(
            crate::stdlib::strlen(string.as_mut_ptr()).wrapping_mul(4 as i32 as usize),
        ) as i32,
        20 as i32,
        8 as i32 as f32,
        string.as_mut_ptr(),
        g_color_table[7 as i32 as usize].as_mut_ptr(),
        qtrue,
        qfalse,
    );
}
/*
=================
SCR_DrawVoipMeter
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_DrawVoipMeter() {
    let mut buffer: [libc::c_char; 16] = [0; 16]; // client has VoIP support disabled.
    let mut string: [libc::c_char; 256] = [0; 256]; // player doesn't want to show meter at all.
    let mut limit: i32 = 0; // not recording at the moment.
    let mut i: i32 = 0; // not connected to a server.
    if (*cl_voipShowMeter).integer == 0 {
        return;
    } else {
        if (*cl_voipSend).integer == 0 {
            return;
        } else {
            if clc.state as u32 != CA_ACTIVE as i32 as u32 {
                return;
            } else {
                if clc.voipEnabled as u64 == 0 {
                    return;
                } else {
                    if clc.demoplaying as u64 != 0 {
                        // server doesn't support VoIP.
                        return;
                    } else {
                        if (*cl_voip).integer == 0 {
                            return;
                        }
                    }
                }
            }
        }
    } // playing back a demo.
    limit = (clc.voipPower * 10.0f32) as i32;
    if limit > 10 as i32 {
        limit = 10 as i32
    }
    i = 0 as i32;
    while i < limit {
        buffer[i as usize] = '*' as i32 as libc::c_char;
        i += 1
    }
    while i < 10 as i32 {
        let fresh0 = i;
        i = i + 1;
        buffer[fresh0 as usize] = ' ' as i32 as libc::c_char
    }
    buffer[i as usize] = '\u{0}' as i32 as libc::c_char;
    libc::sprintf(
        string.as_mut_ptr(),
        b"VoIP: [%s]\x00" as *const u8 as *const libc::c_char,
        buffer.as_mut_ptr(),
    );
    SCR_DrawStringExt(
        (320 as i32 as usize).wrapping_sub(
            crate::stdlib::strlen(string.as_mut_ptr()).wrapping_mul(4 as i32 as usize),
        ) as i32,
        10 as i32,
        8 as i32 as f32,
        string.as_mut_ptr(),
        g_color_table[7 as i32 as usize].as_mut_ptr(),
        qtrue,
        qfalse,
    );
}
/*
===============================================================================

DEBUG GRAPH

===============================================================================
*/

static mut current: i32 = 0;

static mut values: [f32; 1024] = [0.; 1024];
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
// qcommon.h -- definitions common between client and server, but not game.or ref modules
//Ignore __attribute__ on non-gcc platforms
//#define	PRE_RELEASE_DEMO
//============================================================================
//
// msg.c
//
// if false, do a Com_Error
// set to true if the buffer size failed (with allowoverflow set)
// set to true if the buffer size failed (with allowoverflow set)
// for bitwise reads and writes
// TTimo
// copy a msg_t in case we need to store it as is for a bit
// (as I needed this to keep an msg_t from a static var for later use)
// sets data buffer as MSG_Init does prior to do the copy
//============================================================================
/*
==============================================================

NET

==============================================================
*/
// if this flag is set, always attempt ipv6 connections instead of ipv4 if a v6 address is found.
// disables ipv6 multicast support if set.
// number of old messages that must be kept on client and
// server for delta comrpession and ping estimation
// max number of usercmd_t in a packet
// max string commands buffered for restransmit
// an address lookup failed
// maximum length of an IPv6 address string including trailing '\0'
// Needed for IPv6 link-local addresses
// max length of a message, which may
// be fragmented into multiple packets
// ACK window of 48 download chunks. Cannot set this higher, or clients
// will overflow the reliable commands buffer
// 896 byte block chunks
/*
Netchan handles packet fragmentation and out of order / duplicate suppression
*/
// between last packet and previous
// qport value to write when transmitting
// sequencing variables
// incoming fragment assembly buffer
// outgoing fragment buffer
// we need to space out the sending of large fragmented messages
/*
==============================================================

PROTOCOL

==============================================================
*/
// 1.31 - 67
// maintain a list of compatible protocols for demo playing
// NOTE: that stuff only works with two digits protocols
// override on command line, config files etc.
// broadcast scan this many ports after
// PORT_SERVER so a single machine can
// run multiple servers
// the svc_strings[] array in cl_parse.c should mirror this
//
// server to client
//
// [short] [string] only in gamestate messages
// only in gamestate messages
// [string] to be executed by client game module
// [short] size [size bytes]
// new commands, supported only by ioquake3 protocol but not legacy
// not wrapped in USE_VOIP, so this value is reserved.
//
//
// client to server
//
// [[usercmd_t]
// [[usercmd_t]
// [string] message
// new commands, supported only by ioquake3 protocol but not legacy
// not wrapped in USE_VOIP, so this value is reserved.
//
/*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
// module should be bare: "cgame", not "cgame.dll" or "vm/cgame.qvm"
/*
==============================================================

CMD

Command text buffering and command execution

==============================================================
*/
/*

Any number of commands can be added in a frame, from several different sources.
Most commands come from either keybindings or console line input, but entire text
files can be execed.

*/
// allocates an initial text buffer that will grow as needed
// Adds command text at the end of the buffer, does NOT add a final \n
// this can be used in place of either Cbuf_AddText or Cbuf_InsertText
// Pulls off \n terminated lines of text from the command buffer and sends
// them through Cmd_ExecuteString.  Stops when the buffer is empty.
// Normally called once per frame, but may be explicitly invoked.
// Do not call inside a command function, or current args will be destroyed.
//===========================================================================
/*

Command execution takes a null terminated string, breaks it into tokens,
then searches for a command or variable that matches the first token.

*/
// called by the init functions of other parts of the program to
// register commands and functions to call for them.
// The cmd_name is referenced later, so it should not be in temp memory
// if function is NULL, the command will be forwarded to the server
// as a clc_clientCommand instead of executed locally
// don't allow VMs to remove system commands
// callback with each valid string
// The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
// Takes a null terminated string.  Does not need to be /n terminated.
// breaks the string up into arg tokens.
// Parses a single line of text into arguments and tries to execute it
// as if it was typed at the console
/*
==============================================================

CVAR

==============================================================
*/
/*

cvar_t variables are used to hold scalar or string variables that can be changed
or displayed at the console or prog code as well as accessed directly
in C code.

The user can access cvars from the console in three ways:
r_draworder			prints the current value
r_draworder 0		sets the current value to 0
set r_draworder 0	as above, but creates the cvar if not present

Cvars are restricted from having the same names as commands to keep this
interface from being ambiguous.

The are also occasionally used to communicated information between different
modules of the program.

*/
// creates the variable if it doesn't exist, or returns the existing one
// if it exists, the value will not be changed, but flags will be ORed in
// that allows variables to be unarchived without needing bitflags
// if value is "", the value will not override a previously set value.
// basically a slightly modified Cvar_Get for the interpreted modules
// updates an interpreted modules' version of a cvar
// will create the variable with no flags if it doesn't exist
// same as Cvar_Set, but allows more control over setting of cvar
// sometimes we set variables from an untrusted source: fail if flags & CVAR_PROTECTED
// don't set the cvar immediately
// expands value to a string and calls Cvar_Set/Cvar_SetSafe
// returns 0 if not defined or non numeric
// returns an empty string if not defined
// returns CVAR_NONEXISTENT if cvar doesn't exist or the flags of that particular CVAR.
// callback with each valid string
// reset all testing vars to a safe value
// called by Cmd_ExecuteString when Cmd_Argv(0) doesn't match a known
// command.  Returns true if the command was a variable reference that
// was handled. (print or change)
// writes lines containing "set variable value" for all variables
// with the archive flag set to true.
// returns an info string containing all the cvars that have the given bit set
// in their flags ( CVAR_USERINFO, CVAR_SERVERINFO, CVAR_SYSTEMINFO, etc )
// whenever a cvar is modifed, its flags will be OR'd into this, so
// a single check can determine if any CVAR_USERINFO, CVAR_SERVERINFO,
// etc, variables have been modified since the last check.  The bit
// can then be cleared to allow another change detection.
/*
==============================================================

FILESYSTEM

No stdio calls should be used by any part of the game, because
we need to deal with all sorts of directory and seperator char
issues.
==============================================================
*/
// referenced flags
// these are in loop specific order so don't change the order
// number of id paks that will never be autodownloaded from baseq3/missionpack
// shutdown and restart the filesystem so changes to fs_gamedir can take effect
// directory should not have either a leading or trailing /
// if extension is "/", only subdirectories will be returned
// the returned files will not include any directories or /
// will properly create any needed paths and deal with seperater character issues
// if uniqueFILE is true, then a new FILE will be fopened even if the file
// is found in an already open pak file.  If uniqueFILE is false, you must call
// FS_FCloseFile instead of fclose, otherwise the pak FILE would be improperly closed
// It is generally safe to always set uniqueFILE to true, because the majority of
// file IO goes through FS_ReadFile, which Does The Right Thing already.
// returns 1 if a file is in the PAK file, otherwise -1
// properly handles partial reads and reads from other dlls
// note: you can't just fclose from another DLL, due to MS libc issues
// returns the length of the file
// a null buffer will just return the file length without loading
// as a quick check for existence. -1 length == not present
// A 0 byte will always be appended at the end, so string ops are safe.
// the buffer should be considered read-only, because it may be cached
// for other uses.
// forces flush on files we're writing to.
// frees the memory returned by FS_ReadFile
// writes a complete file, creating any subdirectories needed
// doesn't work for files that are opened from a pack file
// where are we?
// like fprintf
// opens a file for reading, writing, or appending depending on the value of mode
// seek on a file
// Returns a space separated string containing the checksums of all loaded pk3 files.
// Servers with sv_pure set will get this string and pass it to clients.
// Returns a space separated string containing the checksums of all loaded
// AND referenced pk3 files. Servers with sv_pure set will get this string
// back from clients for pure validation
// clears referenced booleans on loaded pk3s
// If the string is empty, all data sources will be allowed.
// If not empty, only pk3 files that match one of the space
// separated checksums will be checked for files, with the
// sole exception of .cfg files.
/*
==============================================================

Edit fields and command line history/completion

==============================================================
*/
/*
==============================================================

MISC

==============================================================
*/
// centralizing the declarations for cl_cdkey
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=470
// returned by Sys_GetProcessorFeatures
// centralized and cleaned, that's the max string you can send to a Com_Printf / Com_DPrintf (above gets truncated)
// SE_NONE must be zero
// evTime is still valid
// evValue is a key code, evValue2 is the down flag
// evValue is an ascii char
// evValue and evValue2 are relative signed x / y moves
// evValue is an axis number and evValue2 is the current state (-127 to 127)
// evPtr is a char*
// bytes of data pointed to by evPtr, for journaling
// this must be manually freed if not NULL
// will be journaled properly
// checks for and removes command line "+set var arg" constructs
// if match is NULL, all set commands will be executed, otherwise
// only a set with the exact name.  Only used during startup.
// for building release pak files
// both client and server must agree to pause
// com_speeds times
// renderer backend time
/*

--- low memory ----
server vm
server clipmap
---mark---
renderer initialization (shaders, etc)
UI vm
cgame vm
renderer map
renderer models

---free---

temp file loading
--- high memory ---

*/
// NOT 0 filled memory
// returns 0 filled memory
// NOT 0 filled memory only for small allocations
// commandLine should not include the executable name (argv[0])
/*
==============================================================

CLIENT / SERVER SYSTEMS

==============================================================
*/
//
// client interface
//
// the keyboard binding interface must be setup before execing
// config files, but the rest of client startup will happen later
// char events are for field typing, not game control
// do a screen update before starting to load a map
// when the server is going to load a new map, the entire hunk
// will be cleared, so the client must shutdown cgame, ui, and
// the renderer
// adds the current command line as a clc_clientCommand to the client message.
// things like godmode, noclip, etc, are commands directed to the server,
// so when they are typed in at the console, they will need to be forwarded.
// bring up the "need a cd to play" dialog
// dump all memory on an error
// shutdown client
// initialize renderer interface
// start all the client stuff using the hunk
// Restart sound subsystem
// for keyname autocompletion
// for writing the config files
// call before filesystem access
/*
==============
SCR_DebugGraph
==============
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_DebugGraph(mut value: f32) {
    values[current as usize] = value;
    current = ((current + 1 as i32) as usize).wrapping_rem(
        (::std::mem::size_of::<[f32; 1024]>() as usize)
            .wrapping_div(::std::mem::size_of::<f32>() as usize),
    ) as i32;
}
/*
==============
SCR_DrawDebugGraph
==============
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_DrawDebugGraph() {
    let mut a: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut w: i32 = 0;
    let mut i: i32 = 0;
    let mut h: i32 = 0;
    let mut v: f32 = 0.;
    //
    // draw the graph
    //
    w = cls.glconfig.vidWidth;
    x = 0 as i32;
    y = cls.glconfig.vidHeight;
    re.SetColor.expect("non-null function pointer")(g_color_table[0 as i32 as usize].as_mut_ptr());
    re.DrawStretchPic.expect("non-null function pointer")(
        x as f32,
        (y - (*cl_graphheight).integer) as f32,
        w as f32,
        (*cl_graphheight).integer as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        0 as i32 as f32,
        cls.whiteShader,
    );
    re.SetColor.expect("non-null function pointer")(0 as *const f32);
    a = 0 as i32;
    while a < w {
        i = (::std::mem::size_of::<[f32; 1024]>() as usize)
            .wrapping_div(::std::mem::size_of::<f32>() as usize)
            .wrapping_add(current as usize)
            .wrapping_sub(1 as i32 as usize)
            .wrapping_sub(
                (a as usize).wrapping_rem(
                    (::std::mem::size_of::<[f32; 1024]>() as usize)
                        .wrapping_div(::std::mem::size_of::<f32>() as usize),
                ),
            )
            .wrapping_rem(
                (::std::mem::size_of::<[f32; 1024]>() as usize)
                    .wrapping_div(::std::mem::size_of::<f32>() as usize),
            ) as i32;
        v = values[i as usize];
        v = v * (*cl_graphscale).integer as f32 + (*cl_graphshift).integer as f32;
        if v < 0 as i32 as f32 {
            v += ((*cl_graphheight).integer
                * (1 as i32 + (-v / (*cl_graphheight).integer as f32) as i32))
                as f32
        }
        h = v as i32 % (*cl_graphheight).integer;
        re.DrawStretchPic.expect("non-null function pointer")(
            (x + w - 1 as i32 - a) as f32,
            (y - h) as f32,
            1 as i32 as f32,
            h as f32,
            0 as i32 as f32,
            0 as i32 as f32,
            0 as i32 as f32,
            0 as i32 as f32,
            cls.whiteShader,
        );
        a += 1
    }
}
//=============================================================================
/*
==================
SCR_Init
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_Init() {
    cl_timegraph = Cvar_Get(
        b"timegraph\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s;
    cl_debuggraph = Cvar_Get(
        b"debuggraph\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s;
    cl_graphheight = Cvar_Get(
        b"graphheight\x00" as *const u8 as *const libc::c_char,
        b"32\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s;
    cl_graphscale = Cvar_Get(
        b"graphscale\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s;
    cl_graphshift = Cvar_Get(
        b"graphshift\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s;
    scr_initialized = qtrue;
}
//=======================================================
/*
==================
SCR_DrawScreenField

This will be called twice if rendering in stereo mode
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_DrawScreenField(mut stereoFrame: stereoFrame_t) {
    let mut uiFullscreen: qboolean = qfalse;
    re.BeginFrame.expect("non-null function pointer")(stereoFrame);
    uiFullscreen =
        (!uivm.is_null() && VM_Call(uivm, UI_IS_FULLSCREEN as i32) != 0) as i32 as qboolean;
    // wide aspect ratio screens need to have the sides cleared
    // unless they are displaying game renderings
    if uiFullscreen as u32 != 0 || (clc.state as u32) < CA_LOADING as i32 as u32 {
        if cls.glconfig.vidWidth * 480 as i32 > cls.glconfig.vidHeight * 640 as i32 {
            re.SetColor.expect("non-null function pointer")(
                g_color_table[0 as i32 as usize].as_mut_ptr(),
            );
            re.DrawStretchPic.expect("non-null function pointer")(
                0 as i32 as f32,
                0 as i32 as f32,
                cls.glconfig.vidWidth as f32,
                cls.glconfig.vidHeight as f32,
                0 as i32 as f32,
                0 as i32 as f32,
                0 as i32 as f32,
                0 as i32 as f32,
                cls.whiteShader,
            );
            re.SetColor.expect("non-null function pointer")(0 as *const f32);
        }
    }
    // if the menu is going to cover the entire screen, we
    // don't need to render anything under it
    if !uivm.is_null() && uiFullscreen as u64 == 0 {
        match clc.state as u32 {
            9 => {
                SCR_DrawCinematic();
            }
            1 => {
                // force menu up
                crate::src::client::snd_main::S_StopAllSounds();
                VM_Call(uivm, UI_SET_ACTIVE_MENU as i32, UIMENU_MAIN as i32);
            }
            3 | 4 | 5 => {
                // connecting clients will only show the connection dialog
                // refresh to update the time
                VM_Call(uivm, UI_REFRESH as i32, cls.realtime);
                VM_Call(uivm, UI_DRAW_CONNECT_SCREEN as i32, qfalse as i32);
            }
            6 | 7 => {
                // draw the game information screen and loading progress
                CL_CGameRendering(stereoFrame);
                // also draw the connection information, so it doesn't
                // flash away too briefly on local or lan games
                // refresh to update the time
                VM_Call(uivm, UI_REFRESH as i32, cls.realtime);
                VM_Call(uivm, UI_DRAW_CONNECT_SCREEN as i32, qtrue as i32);
            }
            8 => {
                // always supply STEREO_CENTER as vieworg offset is now done by the engine.
                CL_CGameRendering(stereoFrame);
                SCR_DrawDemoRecording();
                SCR_DrawVoipMeter();
            }
            _ => {
                Com_Error(
                    ERR_FATAL as i32,
                    b"SCR_DrawScreenField: bad clc.state\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    // the menu draws next
    if Key_GetCatcher() & 0x2 as i32 != 0 && !uivm.is_null() {
        VM_Call(uivm, UI_REFRESH as i32, cls.realtime);
    }
    // console draws next
    Con_DrawConsole();
    // debug graph can be drawn on top of anything
    if (*cl_debuggraph).integer != 0 || (*cl_timegraph).integer != 0 || (*cl_debugMove).integer != 0
    {
        SCR_DrawDebugGraph();
    };
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
// client.h -- primary header for client
/* USE_CURL */
// file full of random crap that gets used to create cl_guid
// time between connection packet retransmits
// snapshots are a view of the server at a given time
// cleared if delta parsing was invalid
// rate delayed and dropped commands
// server time the message is valid for (in msec)
// copied from netchan->incoming_sequence
// messageNum the delta is from
// time from when cmdNum-1 was sent to time packet was reeceived
// portalarea visibility bits
// the next cmdNum the server is expecting
// complete information about the current player at this time
// all of the entities that need to be presented
// at the time of this snapshot
// execute all commands up to this before
// making the snapshot current
/*
=============================================================================

the clientActive_t structure is wiped completely at every
new gamestate_t, potentially several times during an established connection

=============================================================================
*/
// cl.cmdNumber when packet was sent
// usercmd->serverTime when packet was sent
// cls.realtime when packet was sent
// the parseEntities array must be large enough to hold PACKET_BACKUP frames of
// entities, so that when a delta compressed message arives from the server
// it can be un-deltad from the original
// it requres several frames in a timeout condition
// to disconnect, preventing debugging breaks from
// causing immediate disconnects on continue
// latest received from server
// may be paused during play
// to prevent time from flowing bakcwards
// to check tournament restarts
// cl.serverTime = cls.realtime + cl.serverTimeDelta
// this value changes as net lag varies
// set if any cgame frame has been forced to extrapolate
// cleared when CL_AdjustTimeDelta looks at it
// set on parse of any valid packet
// configstrings
// extracted from CS_SERVERINFO
// index (not anded off) into cl_parse_entities[]
// added to by mouse events
// set by joystick events
// cgame communicates a few values to the client system
// current weapon to add to usercmd_t
// cmds[cmdNumber] is the predicted command, [cmdNumber-1] is the last
// properly generated command
// each mesage will send several old cmds
// incremented each frame, because multiple
// frames may need to be packed into a single packet
// information about each packet we have sent out
// the client maintains its own idea of view angles, which are
// sent to the server each frame.  It is cleared to 0 upon entering each level.
// the server sends a delta each frame which is added to the locally
// tracked view angles to account for standing on rotating objects,
// and teleport direction changes
// included in each client message so the server
// can tell if it is for a prior map_restart
// big stuff at end of structure so most offsets are 15 bits or less
// for delta compression when not in previous frame
/*
=============================================================================

the clientConnection_t structure is wiped when disconnecting from a server,
either to go to a full screen console, play a demo, or connect to a different server

A connection can be to either a server through the network layer or a
demo through a file.

=============================================================================
*/
// connection status
// for retransmits during connection
// for timeouts
// name of server from original connect (used by reconnect)
// for connection retransmits
// for display on connection dialog
// for display on connection dialog
// from the server to use for connecting
// from the server for checksum calculations
// these are our reliable messages that go to the server
// the last one the server has executed
// server message (unreliable) and command (reliable) sequence
// numbers are NOT cleared at level changes, but continue to
// increase as long as the connection is valid
// message sequence is used by both the network layer and the
// delta compression layer
// reliable messages received from server
// last server command grabbed or executed with CL_GetServerCommand
// file transfer from server
/* USE_CURL */
// block we are waiting for
// how many bytes we got
// how many bytes we got
// list of paks we need to download
// if true, we need to do another FS_Restart because we downloaded a pak
// demo information
// don't record until a non-delta message is received
// counter of rendered frames
// cls.realtime before first frame
// each frame will be at this time + frameNum * 50
// time the last frame was rendered
// minimum frame duration
// maximum frame duration
// log of frame durations
// incoming data...
// !!! FIXME: convert from parallel arrays to array of a struct.
// outgoing data...
// if voipTargets[i / 8] & (1 << (i % 8)),
// then we are sending to clientnum i.
// big stuff at end of structure so most offsets are 15 bits or less
/*
==================================================================

the clientStatic_t structure is never wiped, and is used even when
no client connection is active at all
(except when CL_Shutdown is called)

==================================================================
*/
// bring up the cd needed dialog next frame
// when the server clears the hunk, all of these must be restarted
// msec since last frame
// ignores pause
// ignoring pause, so console always works
// additional global servers
// source currently pinging or updating
// update server info
// rendering info
//=============================================================================
// interface to cgame dll or vm
// interface to ui dll or vm
// interface to refresh .dll
//
// cvars
//
// cl_voipSendTarget is a string: "all" to broadcast to everyone, "none" to
//  send to no one, or a comma-separated list of client numbers:
//  "0,7,2,23" ... an empty string is treated like "all".
// 20ms at 48k
// 3 frame is 60ms of audio, the max opus will encode at once
//=================================================
//
// cl_main
//
//
// cl_input
//
// key nums holding it down
// msec timestamp
// msec down this frame if both a down and up happened
// current state
// set when down, not cleared when up
//
// cl_parse.c
//
//====================================================================
//
// console
//
//
// cl_scrn.c
//
/*
==================
SCR_UpdateScreen

This is called every frame, and can also be called explicitly to flush
text to the screen.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SCR_UpdateScreen() {
    static mut recursive: i32 = 0;
    if scr_initialized as u64 == 0 {
        return;
        // not initialized yet
    }
    recursive += 1;
    if recursive > 2 as i32 {
        Com_Error(
            ERR_FATAL as i32,
            b"SCR_UpdateScreen: recursively called\x00" as *const u8 as *const libc::c_char,
        );
    }
    recursive = 1 as i32;
    // If there is no VM, there are also no rendering commands issued. Stop the renderer in
    // that case.
    if !uivm.is_null() || (*com_dedicated).integer != 0 {
        // XXX
        let mut in_anaglyphMode: i32 =
            Cvar_VariableIntegerValue(b"r_anaglyphMode\x00" as *const u8 as *const libc::c_char);
        // if running in stereo, we need to draw the frame twice
        if cls.glconfig.stereoEnabled as u32 != 0 || in_anaglyphMode != 0 {
            SCR_DrawScreenField(STEREO_LEFT);
            SCR_DrawScreenField(STEREO_RIGHT);
        } else {
            SCR_DrawScreenField(STEREO_CENTER);
        }
        if (*com_speeds).integer != 0 {
            re.EndFrame.expect("non-null function pointer")(&mut time_frontend, &mut time_backend);
        } else {
            re.EndFrame.expect("non-null function pointer")(0 as *mut i32, 0 as *mut i32);
        }
    }
    recursive = 0 as i32;
}

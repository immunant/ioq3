use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return ::libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
    }
}

pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::uint8_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::cg_public_h::CG_CONSOLE_COMMAND;
pub use crate::cg_public_h::CG_CROSSHAIR_PLAYER;
pub use crate::cg_public_h::CG_DRAW_ACTIVE_FRAME;
pub use crate::cg_public_h::CG_EVENT_HANDLING;
pub use crate::cg_public_h::CG_INIT;
pub use crate::cg_public_h::CG_KEY_EVENT;
pub use crate::cg_public_h::CG_LAST_ATTACKER;
pub use crate::cg_public_h::CG_MOUSE_EVENT;
pub use crate::cg_public_h::CG_SHUTDOWN;
pub use crate::client_h::clSnapshot_t;
pub use crate::client_h::clientActive_t;
pub use crate::client_h::clientConnection_t;
pub use crate::client_h::clientStatic_t;
pub use crate::client_h::kbutton_t;
pub use crate::client_h::outPacket_t;
pub use crate::client_h::serverInfo_t;
pub use crate::curl_h::CURL;
pub use crate::multi_h::CURLM;
pub use crate::qcommon_h::clc_EOF;
pub use crate::qcommon_h::clc_bad;
pub use crate::qcommon_h::clc_clientCommand;
pub use crate::qcommon_h::clc_move;
pub use crate::qcommon_h::clc_moveNoDelta;
pub use crate::qcommon_h::clc_nop;
pub use crate::qcommon_h::clc_ops_e;
pub use crate::qcommon_h::clc_voipOpus;
pub use crate::qcommon_h::clc_voipSpeex;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::svc_EOF;
pub use crate::qcommon_h::svc_bad;
pub use crate::qcommon_h::svc_baseline;
pub use crate::qcommon_h::svc_configstring;
pub use crate::qcommon_h::svc_download;
pub use crate::qcommon_h::svc_gamestate;
pub use crate::qcommon_h::svc_nop;
pub use crate::qcommon_h::svc_ops_e;
pub use crate::qcommon_h::svc_serverCommand;
pub use crate::qcommon_h::svc_snapshot;
pub use crate::qcommon_h::svc_voipOpus;
pub use crate::qcommon_h::svc_voipSpeex;
pub use crate::qcommon_h::vm_t;
pub use crate::qcommon_h::xcommand_t;
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
pub use crate::src::client::cl_keys::Key_GetCatcher;
pub use crate::src::client::cl_main::cgvm;
pub use crate::src::client::cl_main::cl;
pub use crate::src::client::cl_main::cl_debugMove;
pub use crate::src::client::cl_main::cl_freelook;
pub use crate::src::client::cl_main::cl_lanForcePackets;
pub use crate::src::client::cl_main::cl_maxpackets;
pub use crate::src::client::cl_main::cl_mouseAccel;
pub use crate::src::client::cl_main::cl_mouseAccelOffset;
pub use crate::src::client::cl_main::cl_mouseAccelStyle;
pub use crate::src::client::cl_main::cl_nodelta;
pub use crate::src::client::cl_main::cl_packetdup;
pub use crate::src::client::cl_main::cl_sensitivity;
pub use crate::src::client::cl_main::cl_showMouseRate;
pub use crate::src::client::cl_main::cl_showSend;
pub use crate::src::client::cl_main::clc;
pub use crate::src::client::cl_main::cls;
pub use crate::src::client::cl_main::j_forward;
pub use crate::src::client::cl_main::j_forward_axis;
pub use crate::src::client::cl_main::j_pitch;
pub use crate::src::client::cl_main::j_pitch_axis;
pub use crate::src::client::cl_main::j_side;
pub use crate::src::client::cl_main::j_side_axis;
pub use crate::src::client::cl_main::j_up;
pub use crate::src::client::cl_main::j_up_axis;
pub use crate::src::client::cl_main::j_yaw;
pub use crate::src::client::cl_main::j_yaw_axis;
pub use crate::src::client::cl_main::m_filter;
pub use crate::src::client::cl_main::m_forward;
pub use crate::src::client::cl_main::m_pitch;
pub use crate::src::client::cl_main::m_side;
pub use crate::src::client::cl_main::m_yaw;
pub use crate::src::client::cl_main::CL_WriteDemoMessage;
pub use crate::src::client::cl_net_chan::CL_Netchan_Transmit;
pub use crate::src::client::cl_scrn::SCR_DebugGraph;
pub use crate::src::client::cl_ui::uivm;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::cmd::Cmd_RemoveCommand;
pub use crate::src::qcommon::common::cl_paused;
pub use crate::src::qcommon::common::com_frameTime;
pub use crate::src::qcommon::common::com_sv_running;
pub use crate::src::qcommon::common::sv_paused;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_IsVoipTarget;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::msg::MSG_Bitstream;
pub use crate::src::qcommon::msg::MSG_HashKey;
pub use crate::src::qcommon::msg::MSG_Init;
pub use crate::src::qcommon::msg::MSG_WriteBits;
pub use crate::src::qcommon::msg::MSG_WriteByte;
pub use crate::src::qcommon::msg::MSG_WriteData;
pub use crate::src::qcommon::msg::MSG_WriteDeltaUsercmdKey;
pub use crate::src::qcommon::msg::MSG_WriteLong;
pub use crate::src::qcommon::msg::MSG_WriteShort;
pub use crate::src::qcommon::msg::MSG_WriteString;
pub use crate::src::qcommon::net_ip::Sys_IsLANAddress;
pub use crate::src::qcommon::q_math::ClampChar;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
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
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::qcommon::vm::VM_Call;
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

pub use crate::src::client::cl_input::stdlib_h::atoi;

pub use ::libc::strtol;
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
// cl.input.c  -- builds an intended movement command to send to the server
#[no_mangle]

pub static mut frame_msec: libc::c_uint = 0;
#[no_mangle]

pub static mut old_com_frameTime: libc::c_int = 0;
/*
===============================================================================

KEY BUTTONS

Continuous button event tracking is complicated by the fact that two different
input sources (say, mouse button 1 and the control key) can both press the
same button, but the button should only be released when both of the
pressing key have been released.

When a key event issues a button command (+forward, +attack, etc), it appends
its key number as argv(1) so it can be matched up with the release.

argv(2) will be set to the time the event happened, which allows exact
control even at low framerates when the down and up events may both get qued
at the same time.

===============================================================================
*/
#[no_mangle]

pub static mut in_left: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_right: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_forward: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_back: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_lookup: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_lookdown: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_moveleft: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_moveright: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_strafe: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_speed: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_up: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_down: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_voiprecord: crate::client_h::kbutton_t = crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut in_buttons: [crate::client_h::kbutton_t; 16] = [crate::client_h::kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    active: crate::src::qcommon::q_shared::qfalse,
    wasPressed: crate::src::qcommon::q_shared::qfalse,
}; 16];
#[no_mangle]

pub static mut in_mlooking: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub unsafe extern "C" fn IN_MLookDown() {
    in_mlooking = crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn IN_MLookUp() {
    in_mlooking = crate::src::qcommon::q_shared::qfalse;
    if (*crate::src::client::cl_main::cl_freelook).integer == 0 {
        IN_CenterView();
    };
}
#[no_mangle]

pub unsafe extern "C" fn IN_KeyDown(mut b: *mut crate::client_h::kbutton_t) {
    let mut k: libc::c_int = 0;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    c = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int);
    if *c.offset(0 as libc::c_int as isize) != 0 {
        k = atoi(c)
    } else {
        k = -(1 as libc::c_int)
        // typed manually at the console for continuous down
    }
    if k == (*b).down[0 as libc::c_int as usize] || k == (*b).down[1 as libc::c_int as usize] {
        return;
        // repeating key
    }
    if (*b).down[0 as libc::c_int as usize] == 0 {
        (*b).down[0 as libc::c_int as usize] = k
    } else if (*b).down[1 as libc::c_int as usize] == 0 {
        (*b).down[1 as libc::c_int as usize] = k
    } else {
        crate::src::qcommon::common::Com_Printf(
            b"Three keys down for a button!\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*b).active as u64 != 0 {
        return;
        // still down
    }
    // save timestamp for partial frame summing
    c = crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int);
    (*b).downtime = atoi(c) as libc::c_uint;
    (*b).active = crate::src::qcommon::q_shared::qtrue;
    (*b).wasPressed = crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn IN_KeyUp(mut b: *mut crate::client_h::kbutton_t) {
    let mut k: libc::c_int = 0;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uptime: libc::c_uint = 0;
    c = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int);
    if *c.offset(0 as libc::c_int as isize) != 0 {
        k = atoi(c)
    } else {
        // typed manually at the console, assume for unsticking, so clear all
        (*b).down[1 as libc::c_int as usize] = 0 as libc::c_int;
        (*b).down[0 as libc::c_int as usize] = (*b).down[1 as libc::c_int as usize];
        (*b).active = crate::src::qcommon::q_shared::qfalse;
        return;
    }
    if (*b).down[0 as libc::c_int as usize] == k {
        (*b).down[0 as libc::c_int as usize] = 0 as libc::c_int
    } else if (*b).down[1 as libc::c_int as usize] == k {
        (*b).down[1 as libc::c_int as usize] = 0 as libc::c_int
    } else {
        return;
        // key up without coresponding down (menu pass through)
    }
    if (*b).down[0 as libc::c_int as usize] != 0 || (*b).down[1 as libc::c_int as usize] != 0 {
        return;
        // some other key is still holding it down
    }
    (*b).active = crate::src::qcommon::q_shared::qfalse;
    // save timestamp for partial frame summing
    c = crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int);
    uptime = atoi(c) as libc::c_uint;
    if uptime != 0 {
        (*b).msec = (*b).msec.wrapping_add(uptime.wrapping_sub((*b).downtime))
    } else {
        (*b).msec = (*b)
            .msec
            .wrapping_add(frame_msec.wrapping_div(2 as libc::c_int as libc::c_uint))
    }
    (*b).active = crate::src::qcommon::q_shared::qfalse;
}
/*
===============
CL_KeyState

Returns the fraction of the frame that the key was down
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_KeyState(mut key: *mut crate::client_h::kbutton_t) -> libc::c_float {
    let mut val: libc::c_float = 0.;
    let mut msec: libc::c_int = 0;
    msec = (*key).msec as libc::c_int;
    (*key).msec = 0 as libc::c_int as libc::c_uint;
    if (*key).active as u64 != 0 {
        // still down
        if (*key).downtime == 0 {
            msec = crate::src::qcommon::common::com_frameTime
        } else {
            msec = (msec as libc::c_uint).wrapping_add(
                (crate::src::qcommon::common::com_frameTime as libc::c_uint)
                    .wrapping_sub((*key).downtime),
            ) as libc::c_int as libc::c_int
        }
        (*key).downtime = crate::src::qcommon::common::com_frameTime as libc::c_uint
    }
    val = msec as libc::c_float / frame_msec as libc::c_float;
    if val < 0 as libc::c_int as libc::c_float {
        val = 0 as libc::c_int as libc::c_float
    }
    if val > 1 as libc::c_int as libc::c_float {
        val = 1 as libc::c_int as libc::c_float
    }
    return val;
}
#[no_mangle]

pub unsafe extern "C" fn IN_UpDown() {
    IN_KeyDown(&mut in_up);
}
#[no_mangle]

pub unsafe extern "C" fn IN_UpUp() {
    IN_KeyUp(&mut in_up);
}
#[no_mangle]

pub unsafe extern "C" fn IN_DownDown() {
    IN_KeyDown(&mut in_down);
}
#[no_mangle]

pub unsafe extern "C" fn IN_DownUp() {
    IN_KeyUp(&mut in_down);
}
#[no_mangle]

pub unsafe extern "C" fn IN_LeftDown() {
    IN_KeyDown(&mut in_left);
}
#[no_mangle]

pub unsafe extern "C" fn IN_LeftUp() {
    IN_KeyUp(&mut in_left);
}
#[no_mangle]

pub unsafe extern "C" fn IN_RightDown() {
    IN_KeyDown(&mut in_right);
}
#[no_mangle]

pub unsafe extern "C" fn IN_RightUp() {
    IN_KeyUp(&mut in_right);
}
#[no_mangle]

pub unsafe extern "C" fn IN_ForwardDown() {
    IN_KeyDown(&mut in_forward);
}
#[no_mangle]

pub unsafe extern "C" fn IN_ForwardUp() {
    IN_KeyUp(&mut in_forward);
}
#[no_mangle]

pub unsafe extern "C" fn IN_BackDown() {
    IN_KeyDown(&mut in_back);
}
#[no_mangle]

pub unsafe extern "C" fn IN_BackUp() {
    IN_KeyUp(&mut in_back);
}
#[no_mangle]

pub unsafe extern "C" fn IN_LookupDown() {
    IN_KeyDown(&mut in_lookup);
}
#[no_mangle]

pub unsafe extern "C" fn IN_LookupUp() {
    IN_KeyUp(&mut in_lookup);
}
#[no_mangle]

pub unsafe extern "C" fn IN_LookdownDown() {
    IN_KeyDown(&mut in_lookdown);
}
#[no_mangle]

pub unsafe extern "C" fn IN_LookdownUp() {
    IN_KeyUp(&mut in_lookdown);
}
#[no_mangle]

pub unsafe extern "C" fn IN_MoveleftDown() {
    IN_KeyDown(&mut in_moveleft);
}
#[no_mangle]

pub unsafe extern "C" fn IN_MoveleftUp() {
    IN_KeyUp(&mut in_moveleft);
}
#[no_mangle]

pub unsafe extern "C" fn IN_MoverightDown() {
    IN_KeyDown(&mut in_moveright);
}
#[no_mangle]

pub unsafe extern "C" fn IN_MoverightUp() {
    IN_KeyUp(&mut in_moveright);
}
#[no_mangle]

pub unsafe extern "C" fn IN_SpeedDown() {
    IN_KeyDown(&mut in_speed);
}
#[no_mangle]

pub unsafe extern "C" fn IN_SpeedUp() {
    IN_KeyUp(&mut in_speed);
}
#[no_mangle]

pub unsafe extern "C" fn IN_StrafeDown() {
    IN_KeyDown(&mut in_strafe);
}
#[no_mangle]

pub unsafe extern "C" fn IN_StrafeUp() {
    IN_KeyUp(&mut in_strafe);
}
#[no_mangle]

pub unsafe extern "C" fn IN_VoipRecordDown() {
    IN_KeyDown(&mut in_voiprecord);
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn IN_VoipRecordUp() {
    IN_KeyUp(&mut in_voiprecord);
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_voipSend\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button0Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(0 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button0Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(0 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button1Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(1 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button1Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(1 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button2Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(2 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button2Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(2 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button3Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(3 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button3Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(3 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button4Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(4 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button4Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(4 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button5Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(5 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button5Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(5 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button6Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(6 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button6Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(6 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button7Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(7 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button7Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(7 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button8Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(8 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button8Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(8 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button9Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(9 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button9Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(9 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button10Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(10 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button10Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(10 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button11Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(11 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button11Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(11 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button12Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(12 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button12Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(12 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button13Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(13 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button13Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(13 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button14Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(14 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button14Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(14 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button15Down() {
    IN_KeyDown(&mut *in_buttons.as_mut_ptr().offset(15 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_Button15Up() {
    IN_KeyUp(&mut *in_buttons.as_mut_ptr().offset(15 as libc::c_int as isize));
}
#[no_mangle]

pub unsafe extern "C" fn IN_CenterView() {
    crate::src::client::cl_main::cl.viewangles[0 as libc::c_int as usize] =
        -(crate::src::client::cl_main::cl.snap.ps.delta_angles[0 as libc::c_int as usize]
            as libc::c_double
            * (360.0f64 / 65536 as libc::c_int as libc::c_double))
            as crate::src::qcommon::q_shared::vec_t;
}
//==========================================================================
#[no_mangle]

pub static mut cl_yawspeed: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_pitchspeed: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_run: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cl_anglespeedkey: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
/*
================
CL_AdjustAngles

Moves the local angle positions
================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_AdjustAngles() {
    let mut speed: libc::c_float = 0.;
    if in_speed.active as u64 != 0 {
        speed = (0.001f64
            * crate::src::client::cl_main::cls.frametime as libc::c_double
            * (*cl_anglespeedkey).value as libc::c_double) as libc::c_float
    } else {
        speed = (0.001f64 * crate::src::client::cl_main::cls.frametime as libc::c_double)
            as libc::c_float
    }
    if in_strafe.active as u64 == 0 {
        crate::src::client::cl_main::cl.viewangles[1 as libc::c_int as usize] -=
            speed * (*cl_yawspeed).value * CL_KeyState(&mut in_right);
        crate::src::client::cl_main::cl.viewangles[1 as libc::c_int as usize] +=
            speed * (*cl_yawspeed).value * CL_KeyState(&mut in_left)
    }
    crate::src::client::cl_main::cl.viewangles[0 as libc::c_int as usize] -=
        speed * (*cl_pitchspeed).value * CL_KeyState(&mut in_lookup);
    crate::src::client::cl_main::cl.viewangles[0 as libc::c_int as usize] +=
        speed * (*cl_pitchspeed).value * CL_KeyState(&mut in_lookdown);
}
/*
================
CL_KeyMove

Sets the usercmd_t based on key states
================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_KeyMove(mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t) {
    let mut movespeed: libc::c_int = 0;
    let mut forward: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut up: libc::c_int = 0;
    //
    // adjust for speed key / running
    // the walking flag is to keep animations consistent
    // even during acceleration and develeration
    //
    if in_speed.active as libc::c_uint ^ (*cl_run).integer as libc::c_uint != 0 {
        movespeed = 127 as libc::c_int;
        (*cmd).buttons &= !(16 as libc::c_int)
    } else {
        (*cmd).buttons |= 16 as libc::c_int;
        movespeed = 64 as libc::c_int
    }
    forward = 0 as libc::c_int;
    side = 0 as libc::c_int;
    up = 0 as libc::c_int;
    if in_strafe.active as u64 != 0 {
        side = (side as libc::c_float + movespeed as libc::c_float * CL_KeyState(&mut in_right))
            as libc::c_int;
        side = (side as libc::c_float - movespeed as libc::c_float * CL_KeyState(&mut in_left))
            as libc::c_int
    }
    side = (side as libc::c_float + movespeed as libc::c_float * CL_KeyState(&mut in_moveright))
        as libc::c_int;
    side = (side as libc::c_float - movespeed as libc::c_float * CL_KeyState(&mut in_moveleft))
        as libc::c_int;
    up =
        (up as libc::c_float + movespeed as libc::c_float * CL_KeyState(&mut in_up)) as libc::c_int;
    up = (up as libc::c_float - movespeed as libc::c_float * CL_KeyState(&mut in_down))
        as libc::c_int;
    forward = (forward as libc::c_float + movespeed as libc::c_float * CL_KeyState(&mut in_forward))
        as libc::c_int;
    forward = (forward as libc::c_float - movespeed as libc::c_float * CL_KeyState(&mut in_back))
        as libc::c_int;
    (*cmd).forwardmove = crate::src::qcommon::q_math::ClampChar(forward);
    (*cmd).rightmove = crate::src::qcommon::q_math::ClampChar(side);
    (*cmd).upmove = crate::src::qcommon::q_math::ClampChar(up);
}
/*
=================
CL_MouseEvent
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_MouseEvent(
    mut dx: libc::c_int,
    mut dy: libc::c_int,
    mut _time: libc::c_int,
) {
    if crate::src::client::cl_keys::Key_GetCatcher() & 0x2 as libc::c_int != 0 {
        crate::src::qcommon::vm::VM_Call(
            crate::src::client::cl_ui::uivm,
            crate::ui_public_h::UI_MOUSE_EVENT as libc::c_int,
            dx,
            dy,
        );
    } else if crate::src::client::cl_keys::Key_GetCatcher() & 0x8 as libc::c_int != 0 {
        crate::src::qcommon::vm::VM_Call(
            crate::src::client::cl_main::cgvm,
            crate::cg_public_h::CG_MOUSE_EVENT as libc::c_int,
            dx,
            dy,
        );
    } else {
        crate::src::client::cl_main::cl.mouseDx
            [crate::src::client::cl_main::cl.mouseIndex as usize] += dx;
        crate::src::client::cl_main::cl.mouseDy
            [crate::src::client::cl_main::cl.mouseIndex as usize] += dy
    };
}
/*
=================
CL_JoystickEvent

Joystick values stay set until changed
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_JoystickEvent(
    mut axis: libc::c_int,
    mut value: libc::c_int,
    mut _time: libc::c_int,
) {
    if axis < 0 as libc::c_int || axis >= 16 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CL_JoystickEvent: bad axis %i\x00" as *const u8 as *const libc::c_char,
            axis,
        );
    }
    crate::src::client::cl_main::cl.joystickAxis[axis as usize] = value;
}
/*
=================
CL_JoystickMove
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_JoystickMove(mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t) {
    let mut anglespeed: libc::c_float = 0.;
    let mut yaw: libc::c_float = (*crate::src::client::cl_main::j_yaw).value
        * crate::src::client::cl_main::cl.joystickAxis
            [(*crate::src::client::cl_main::j_yaw_axis).integer as usize]
            as libc::c_float;
    let mut right: libc::c_float = (*crate::src::client::cl_main::j_side).value
        * crate::src::client::cl_main::cl.joystickAxis
            [(*crate::src::client::cl_main::j_side_axis).integer as usize]
            as libc::c_float;
    let mut forward: libc::c_float = (*crate::src::client::cl_main::j_forward).value
        * crate::src::client::cl_main::cl.joystickAxis
            [(*crate::src::client::cl_main::j_forward_axis).integer as usize]
            as libc::c_float;
    let mut pitch: libc::c_float = (*crate::src::client::cl_main::j_pitch).value
        * crate::src::client::cl_main::cl.joystickAxis
            [(*crate::src::client::cl_main::j_pitch_axis).integer as usize]
            as libc::c_float;
    let mut up: libc::c_float = (*crate::src::client::cl_main::j_up).value
        * crate::src::client::cl_main::cl.joystickAxis
            [(*crate::src::client::cl_main::j_up_axis).integer as usize] as libc::c_float;
    if in_speed.active as libc::c_uint ^ (*cl_run).integer as libc::c_uint == 0 {
        (*cmd).buttons |= 16 as libc::c_int
    }
    if in_speed.active as u64 != 0 {
        anglespeed = (0.001f64
            * crate::src::client::cl_main::cls.frametime as libc::c_double
            * (*cl_anglespeedkey).value as libc::c_double) as libc::c_float
    } else {
        anglespeed = (0.001f64 * crate::src::client::cl_main::cls.frametime as libc::c_double)
            as libc::c_float
    }
    if in_strafe.active as u64 == 0 {
        crate::src::client::cl_main::cl.viewangles[1 as libc::c_int as usize] += anglespeed * yaw;
        (*cmd).rightmove = crate::src::qcommon::q_math::ClampChar(
            (*cmd).rightmove as libc::c_int + right as libc::c_int,
        )
    } else {
        crate::src::client::cl_main::cl.viewangles[1 as libc::c_int as usize] += anglespeed * right;
        (*cmd).rightmove = crate::src::qcommon::q_math::ClampChar(
            (*cmd).rightmove as libc::c_int + yaw as libc::c_int,
        )
    }
    if in_mlooking as u64 != 0 {
        crate::src::client::cl_main::cl.viewangles[0 as libc::c_int as usize] +=
            anglespeed * forward;
        (*cmd).forwardmove = crate::src::qcommon::q_math::ClampChar(
            (*cmd).forwardmove as libc::c_int + pitch as libc::c_int,
        )
    } else {
        crate::src::client::cl_main::cl.viewangles[0 as libc::c_int as usize] += anglespeed * pitch;
        (*cmd).forwardmove = crate::src::qcommon::q_math::ClampChar(
            (*cmd).forwardmove as libc::c_int + forward as libc::c_int,
        )
    }
    (*cmd).upmove =
        crate::src::qcommon::q_math::ClampChar((*cmd).upmove as libc::c_int + up as libc::c_int);
}
/*
=================
CL_MouseMove
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_MouseMove(mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t) {
    let mut mx: libc::c_float = 0.;
    let mut my: libc::c_float = 0.;
    // allow mouse smoothing
    if (*crate::src::client::cl_main::m_filter).integer != 0 {
        mx = (crate::src::client::cl_main::cl.mouseDx[0 as libc::c_int as usize]
            + crate::src::client::cl_main::cl.mouseDx[1 as libc::c_int as usize])
            as libc::c_float
            * 0.5f32;
        my = (crate::src::client::cl_main::cl.mouseDy[0 as libc::c_int as usize]
            + crate::src::client::cl_main::cl.mouseDy[1 as libc::c_int as usize])
            as libc::c_float
            * 0.5f32
    } else {
        mx = crate::src::client::cl_main::cl.mouseDx
            [crate::src::client::cl_main::cl.mouseIndex as usize] as libc::c_float;
        my = crate::src::client::cl_main::cl.mouseDy
            [crate::src::client::cl_main::cl.mouseIndex as usize] as libc::c_float
    }
    crate::src::client::cl_main::cl.mouseIndex ^= 1 as libc::c_int;
    crate::src::client::cl_main::cl.mouseDx[crate::src::client::cl_main::cl.mouseIndex as usize] =
        0 as libc::c_int;
    crate::src::client::cl_main::cl.mouseDy[crate::src::client::cl_main::cl.mouseIndex as usize] =
        0 as libc::c_int;
    if mx == 0.0f32 && my == 0.0f32 {
        return;
    }
    if (*crate::src::client::cl_main::cl_mouseAccel).value != 0.0f32 {
        if (*crate::src::client::cl_main::cl_mouseAccelStyle).integer == 0 as libc::c_int {
            let mut accelSensitivity: libc::c_float = 0.;
            let mut rate: libc::c_float = 0.;
            rate = (crate::stdlib::sqrt((mx * mx + my * my) as libc::c_double)
                / frame_msec as libc::c_float as libc::c_double)
                as libc::c_float;
            accelSensitivity = (*crate::src::client::cl_main::cl_sensitivity).value
                + rate * (*crate::src::client::cl_main::cl_mouseAccel).value;
            mx *= accelSensitivity;
            my *= accelSensitivity;
            if (*crate::src::client::cl_main::cl_showMouseRate).integer != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"rate: %f, accelSensitivity: %f\n\x00" as *const u8 as *const libc::c_char,
                    rate as libc::c_double,
                    accelSensitivity as libc::c_double,
                );
            }
        } else {
            let mut rate_0: [libc::c_float; 2] = [0.; 2];
            let mut power: [libc::c_float; 2] = [0.; 2];
            // sensitivity remains pretty much unchanged at low speeds
            // cl_mouseAccel is a power value to how the acceleration is shaped
            // cl_mouseAccelOffset is the rate for which the acceleration will have doubled the non accelerated amplification
            // NOTE: decouple the config cvars for independent acceleration setup along X and Y?
            rate_0[0 as libc::c_int as usize] = (crate::stdlib::fabs(mx as libc::c_double)
                / frame_msec as libc::c_float as libc::c_double)
                as libc::c_float;
            rate_0[1 as libc::c_int as usize] = (crate::stdlib::fabs(my as libc::c_double)
                / frame_msec as libc::c_float as libc::c_double)
                as libc::c_float;
            power[0 as libc::c_int as usize] = crate::stdlib::powf(
                rate_0[0 as libc::c_int as usize]
                    / (*crate::src::client::cl_main::cl_mouseAccelOffset).value,
                (*crate::src::client::cl_main::cl_mouseAccel).value,
            );
            power[1 as libc::c_int as usize] = crate::stdlib::powf(
                rate_0[1 as libc::c_int as usize]
                    / (*crate::src::client::cl_main::cl_mouseAccelOffset).value,
                (*crate::src::client::cl_main::cl_mouseAccel).value,
            );
            mx = (*crate::src::client::cl_main::cl_sensitivity).value
                * (mx
                    + (if mx < 0 as libc::c_int as libc::c_float {
                        -power[0 as libc::c_int as usize]
                    } else {
                        power[0 as libc::c_int as usize]
                    }) * (*crate::src::client::cl_main::cl_mouseAccelOffset).value);
            my = (*crate::src::client::cl_main::cl_sensitivity).value
                * (my
                    + (if my < 0 as libc::c_int as libc::c_float {
                        -power[1 as libc::c_int as usize]
                    } else {
                        power[1 as libc::c_int as usize]
                    }) * (*crate::src::client::cl_main::cl_mouseAccelOffset).value);
            if (*crate::src::client::cl_main::cl_showMouseRate).integer != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"ratex: %f, ratey: %f, powx: %f, powy: %f\n\x00" as *const u8
                        as *const libc::c_char,
                    rate_0[0 as libc::c_int as usize] as libc::c_double,
                    rate_0[1 as libc::c_int as usize] as libc::c_double,
                    power[0 as libc::c_int as usize] as libc::c_double,
                    power[1 as libc::c_int as usize] as libc::c_double,
                );
            }
        }
    } else {
        mx *= (*crate::src::client::cl_main::cl_sensitivity).value;
        my *= (*crate::src::client::cl_main::cl_sensitivity).value
    }
    // ingame FOV
    mx *= crate::src::client::cl_main::cl.cgameSensitivity;
    my *= crate::src::client::cl_main::cl.cgameSensitivity;
    // add mouse X/Y movement to cmd
    if in_strafe.active as u64 != 0 {
        (*cmd).rightmove = crate::src::qcommon::q_math::ClampChar(
            ((*cmd).rightmove as libc::c_int as libc::c_float
                + (*crate::src::client::cl_main::m_side).value * mx) as libc::c_int,
        )
    } else {
        crate::src::client::cl_main::cl.viewangles[1 as libc::c_int as usize] -=
            (*crate::src::client::cl_main::m_yaw).value * mx
    }
    if (in_mlooking as libc::c_uint != 0
        || (*crate::src::client::cl_main::cl_freelook).integer != 0)
        && in_strafe.active as u64 == 0
    {
        crate::src::client::cl_main::cl.viewangles[0 as libc::c_int as usize] +=
            (*crate::src::client::cl_main::m_pitch).value * my
    } else {
        (*cmd).forwardmove = crate::src::qcommon::q_math::ClampChar(
            ((*cmd).forwardmove as libc::c_int as libc::c_float
                - (*crate::src::client::cl_main::m_forward).value * my) as libc::c_int,
        )
    };
}
/*
==============
CL_CmdButtons
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CmdButtons(mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t) {
    let mut i: libc::c_int = 0;
    //
    // figure button bits
    // send a button bit even if the key was pressed and released in
    // less than a frame
    //
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        if in_buttons[i as usize].active as libc::c_uint != 0
            || in_buttons[i as usize].wasPressed as libc::c_uint != 0
        {
            (*cmd).buttons |= (1 as libc::c_int) << i
        }
        in_buttons[i as usize].wasPressed = crate::src::qcommon::q_shared::qfalse;
        i += 1
    }
    if crate::src::client::cl_keys::Key_GetCatcher() != 0 {
        (*cmd).buttons |= 2 as libc::c_int
    }
    // allow the game to know if any key at all is
    // currently pressed, even if it isn't bound to anything
    if crate::src::client::cl_keys::anykeydown != 0
        && crate::src::client::cl_keys::Key_GetCatcher() == 0 as libc::c_int
    {
        (*cmd).buttons |= 2048 as libc::c_int
    };
}
/*
==============
CL_FinishMove
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_FinishMove(mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t) {
    let mut i: libc::c_int = 0;
    // copy the state that the cgame is currently sending
    (*cmd).weapon =
        crate::src::client::cl_main::cl.cgameUserCmdValue as crate::src::qcommon::q_shared::byte;
    // send the current server time so the amount of movement
    // can be determined without allowing cheating
    (*cmd).serverTime = crate::src::client::cl_main::cl.serverTime;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*cmd).angles[i as usize] = (crate::src::client::cl_main::cl.viewangles[i as usize]
            * 65536 as libc::c_int as libc::c_float
            / 360 as libc::c_int as libc::c_float)
            as libc::c_int
            & 65535 as libc::c_int;
        i += 1
    }
}
/*
=================
CL_CreateCmd
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CreateCmd() -> crate::src::qcommon::q_shared::usercmd_t {
    let mut cmd: crate::src::qcommon::q_shared::usercmd_t =
        crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        };
    let mut oldAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    oldAngles[0 as libc::c_int as usize] =
        crate::src::client::cl_main::cl.viewangles[0 as libc::c_int as usize];
    oldAngles[1 as libc::c_int as usize] =
        crate::src::client::cl_main::cl.viewangles[1 as libc::c_int as usize];
    oldAngles[2 as libc::c_int as usize] =
        crate::src::client::cl_main::cl.viewangles[2 as libc::c_int as usize];
    // keyboard angle adjustment
    CL_AdjustAngles();
    crate::stdlib::memset(
        &mut cmd as *mut crate::src::qcommon::q_shared::usercmd_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::usercmd_t>() as libc::c_ulong,
    );
    CL_CmdButtons(&mut cmd);
    // get basic movement from keyboard
    CL_KeyMove(&mut cmd);
    // get basic movement from mouse
    CL_MouseMove(&mut cmd);
    // get basic movement from joystick
    CL_JoystickMove(&mut cmd);
    // check to make sure the angles haven't wrapped
    if crate::src::client::cl_main::cl.viewangles[0 as libc::c_int as usize]
        - oldAngles[0 as libc::c_int as usize]
        > 90 as libc::c_int as libc::c_float
    {
        crate::src::client::cl_main::cl.viewangles[0 as libc::c_int as usize] =
            oldAngles[0 as libc::c_int as usize] + 90 as libc::c_int as libc::c_float
    } else if oldAngles[0 as libc::c_int as usize]
        - crate::src::client::cl_main::cl.viewangles[0 as libc::c_int as usize]
        > 90 as libc::c_int as libc::c_float
    {
        crate::src::client::cl_main::cl.viewangles[0 as libc::c_int as usize] =
            oldAngles[0 as libc::c_int as usize] - 90 as libc::c_int as libc::c_float
    }
    // store out the final values
    CL_FinishMove(&mut cmd);
    // draw debug graphs of turning for mouse testing
    if (*crate::src::client::cl_main::cl_debugMove).integer != 0 {
        if (*crate::src::client::cl_main::cl_debugMove).integer == 1 as libc::c_int {
            crate::src::client::cl_scrn::SCR_DebugGraph(crate::stdlib::fabs(
                (crate::src::client::cl_main::cl.viewangles[1 as libc::c_int as usize]
                    - oldAngles[1 as libc::c_int as usize]) as libc::c_double,
            ) as libc::c_float);
        }
        if (*crate::src::client::cl_main::cl_debugMove).integer == 2 as libc::c_int {
            crate::src::client::cl_scrn::SCR_DebugGraph(crate::stdlib::fabs(
                (crate::src::client::cl_main::cl.viewangles[0 as libc::c_int as usize]
                    - oldAngles[0 as libc::c_int as usize]) as libc::c_double,
            ) as libc::c_float);
        }
    }
    return cmd;
}
/*
=================
CL_CreateNewCommands

Create a new usercmd_t structure for this frame
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_CreateNewCommands() {
    let mut cmdNum: libc::c_int = 0;
    // no need to create usercmds until we have a gamestate
    if (crate::src::client::cl_main::clc.state as libc::c_uint)
        < crate::src::qcommon::q_shared::CA_PRIMED as libc::c_int as libc::c_uint
    {
        return;
    }
    frame_msec = (crate::src::qcommon::common::com_frameTime - old_com_frameTime) as libc::c_uint;
    // if running over 1000fps, act as if each frame is 1ms
    // prevents divisions by zero
    if frame_msec < 1 as libc::c_int as libc::c_uint {
        frame_msec = 1 as libc::c_int as libc::c_uint
    }
    // if running less than 5fps, truncate the extra time to prevent
    // unexpected moves after a hitch
    if frame_msec > 200 as libc::c_int as libc::c_uint {
        frame_msec = 200 as libc::c_int as libc::c_uint
    }
    old_com_frameTime = crate::src::qcommon::common::com_frameTime;
    // generate a command for this frame
    crate::src::client::cl_main::cl.cmdNumber += 1;
    cmdNum = crate::src::client::cl_main::cl.cmdNumber & 64 as libc::c_int - 1 as libc::c_int;
    crate::src::client::cl_main::cl.cmds[cmdNum as usize] = CL_CreateCmd();
}
/*
=================
CL_ReadyToSendPacket

Returns qfalse if we are over the maxpackets limit
and should choke back the bandwidth a bit by not sending
a packet this frame.  All the commands will still get
delivered in the next packet, but saving a header and
getting more delta compression will reduce total bandwidth.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ReadyToSendPacket() -> crate::src::qcommon::q_shared::qboolean {
    let mut oldPacketNum: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    // don't send anything if playing back a demo
    if crate::src::client::cl_main::clc.demoplaying as libc::c_uint != 0
        || crate::src::client::cl_main::clc.state as libc::c_uint
            == crate::src::qcommon::q_shared::CA_CINEMATIC as libc::c_int as libc::c_uint
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // If we are downloading, we send no less than 50ms between packets
    if *crate::src::client::cl_main::clc
        .downloadTempName
        .as_mut_ptr() as libc::c_int
        != 0
        && crate::src::client::cl_main::cls.realtime
            - crate::src::client::cl_main::clc.lastPacketSentTime
            < 50 as libc::c_int
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // if we don't have a valid gamestate yet, only send
    // one packet a second
    if crate::src::client::cl_main::clc.state as libc::c_uint
        != crate::src::qcommon::q_shared::CA_ACTIVE as libc::c_int as libc::c_uint
        && crate::src::client::cl_main::clc.state as libc::c_uint
            != crate::src::qcommon::q_shared::CA_PRIMED as libc::c_int as libc::c_uint
        && *crate::src::client::cl_main::clc
            .downloadTempName
            .as_mut_ptr()
            == 0
        && crate::src::client::cl_main::cls.realtime
            - crate::src::client::cl_main::clc.lastPacketSentTime
            < 1000 as libc::c_int
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // send every frame for loopbacks
    if crate::src::client::cl_main::clc
        .netchan
        .remoteAddress
        .type_0 as libc::c_uint
        == crate::qcommon_h::NA_LOOPBACK as libc::c_int as libc::c_uint
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    // send every frame for LAN
    if (*crate::src::client::cl_main::cl_lanForcePackets).integer != 0
        && crate::src::qcommon::net_ip::Sys_IsLANAddress(
            crate::src::client::cl_main::clc.netchan.remoteAddress as crate::qcommon_h::netadr_t,
        ) as libc::c_uint
            != 0
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    // check for exceeding cl_maxpackets
    if (*crate::src::client::cl_main::cl_maxpackets).integer < 15 as libc::c_int {
        crate::src::qcommon::cvar::Cvar_Set(
            b"cl_maxpackets\x00" as *const u8 as *const libc::c_char,
            b"15\x00" as *const u8 as *const libc::c_char,
        );
    } else if (*crate::src::client::cl_main::cl_maxpackets).integer > 125 as libc::c_int {
        crate::src::qcommon::cvar::Cvar_Set(
            b"cl_maxpackets\x00" as *const u8 as *const libc::c_char,
            b"125\x00" as *const u8 as *const libc::c_char,
        );
    }
    oldPacketNum = crate::src::client::cl_main::clc.netchan.outgoingSequence - 1 as libc::c_int
        & 32 as libc::c_int - 1 as libc::c_int;
    delta = crate::src::client::cl_main::cls.realtime
        - crate::src::client::cl_main::cl.outPackets[oldPacketNum as usize].p_realtime;
    if delta < 1000 as libc::c_int / (*crate::src::client::cl_main::cl_maxpackets).integer {
        // the accumulated commands will go out in the next packet
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===================
CL_WritePacket

Create and send the command packet to the server
Including both the reliable commands and the usercmds

During normal gameplay, a client packet will contain something like:

4	sequence number
2	qport
4	serverid
4	acknowledged sequence number
4	clc.serverCommandSequence
<optional reliable commands>
1	clc_move or clc_moveNoDelta
1	command count
<count * usercmds>

===================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_WritePacket() {
    let mut buf: crate::qcommon_h::msg_t = crate::qcommon_h::msg_t {
        allowoverflow: crate::src::qcommon::q_shared::qfalse,
        overflowed: crate::src::qcommon::q_shared::qfalse,
        oob: crate::src::qcommon::q_shared::qfalse,
        data: 0 as *mut crate::src::qcommon::q_shared::byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    let mut data: [crate::src::qcommon::q_shared::byte; 16384] = [0; 16384];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t =
        0 as *mut crate::src::qcommon::q_shared::usercmd_t;
    let mut oldcmd: *mut crate::src::qcommon::q_shared::usercmd_t =
        0 as *mut crate::src::qcommon::q_shared::usercmd_t;
    let mut nullcmd: crate::src::qcommon::q_shared::usercmd_t =
        crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        };
    let mut packetNum: libc::c_int = 0;
    let mut oldPacketNum: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut key: libc::c_int = 0;
    // don't send anything if playing back a demo
    if crate::src::client::cl_main::clc.demoplaying as libc::c_uint != 0
        || crate::src::client::cl_main::clc.state as libc::c_uint
            == crate::src::qcommon::q_shared::CA_CINEMATIC as libc::c_int as libc::c_uint
    {
        return;
    }
    crate::stdlib::memset(
        &mut nullcmd as *mut crate::src::qcommon::q_shared::usercmd_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::usercmd_t>() as libc::c_ulong,
    );
    oldcmd = &mut nullcmd;
    crate::src::qcommon::msg::MSG_Init(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        data.as_mut_ptr(),
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16384]>() as libc::c_ulong
            as libc::c_int,
    );
    crate::src::qcommon::msg::MSG_Bitstream(&mut buf as *mut _ as *mut crate::qcommon_h::msg_t);
    // write the current serverId so the server
    // can tell if this is from the current gameState
    crate::src::qcommon::msg::MSG_WriteLong(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        crate::src::client::cl_main::cl.serverId,
    );
    // write the last message we received, which can
    // be used for delta compression, and is also used
    // to tell if we dropped a gamestate
    crate::src::qcommon::msg::MSG_WriteLong(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        crate::src::client::cl_main::clc.serverMessageSequence,
    );
    // write the last reliable message we received
    crate::src::qcommon::msg::MSG_WriteLong(
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
        crate::src::client::cl_main::clc.serverCommandSequence,
    );
    // write any unacknowledged clientCommands
    i = crate::src::client::cl_main::clc.reliableAcknowledge + 1 as libc::c_int;
    while i <= crate::src::client::cl_main::clc.reliableSequence {
        crate::src::qcommon::msg::MSG_WriteByte(
            &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
            crate::qcommon_h::clc_clientCommand as libc::c_int,
        );
        crate::src::qcommon::msg::MSG_WriteLong(
            &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
            i,
        );
        crate::src::qcommon::msg::MSG_WriteString(
            &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
            crate::src::client::cl_main::clc.reliableCommands
                [(i & 64 as libc::c_int - 1 as libc::c_int) as usize]
                .as_mut_ptr(),
        );
        i += 1
    }
    // we want to send all the usercmds that were generated in the last
    // few packet, so even if a couple packets are dropped in a row,
    // all the cmds will make it to the server
    if (*crate::src::client::cl_main::cl_packetdup).integer < 0 as libc::c_int {
        crate::src::qcommon::cvar::Cvar_Set(
            b"cl_packetdup\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    } else if (*crate::src::client::cl_main::cl_packetdup).integer > 5 as libc::c_int {
        crate::src::qcommon::cvar::Cvar_Set(
            b"cl_packetdup\x00" as *const u8 as *const libc::c_char,
            b"5\x00" as *const u8 as *const libc::c_char,
        );
    }
    oldPacketNum = crate::src::client::cl_main::clc.netchan.outgoingSequence
        - 1 as libc::c_int
        - (*crate::src::client::cl_main::cl_packetdup).integer
        & 32 as libc::c_int - 1 as libc::c_int;
    count = crate::src::client::cl_main::cl.cmdNumber
        - crate::src::client::cl_main::cl.outPackets[oldPacketNum as usize].p_cmdNumber;
    if count > 32 as libc::c_int {
        count = 32 as libc::c_int;
        crate::src::qcommon::common::Com_Printf(
            b"MAX_PACKET_USERCMDS\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if crate::src::client::cl_main::clc.voipOutgoingDataSize > 0 as libc::c_int {
        if crate::src::client::cl_main::clc.voipFlags as libc::c_int & 0x1 as libc::c_int != 0
            || crate::src::qcommon::common::Com_IsVoipTarget(
                crate::src::client::cl_main::clc.voipTargets.as_mut_ptr(),
                ::std::mem::size_of::<[crate::stdlib::uint8_t; 8]>() as libc::c_ulong
                    as libc::c_int,
                -(1 as libc::c_int),
            ) as libc::c_uint
                != 0
        {
            crate::src::qcommon::msg::MSG_WriteByte(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                crate::qcommon_h::clc_voipOpus as libc::c_int,
            );
            crate::src::qcommon::msg::MSG_WriteByte(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                crate::src::client::cl_main::clc.voipOutgoingGeneration as libc::c_int,
            );
            crate::src::qcommon::msg::MSG_WriteLong(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                crate::src::client::cl_main::clc.voipOutgoingSequence,
            );
            crate::src::qcommon::msg::MSG_WriteByte(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                crate::src::client::cl_main::clc.voipOutgoingDataFrames,
            );
            crate::src::qcommon::msg::MSG_WriteData(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                crate::src::client::cl_main::clc.voipTargets.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[crate::stdlib::uint8_t; 8]>() as libc::c_ulong
                    as libc::c_int,
            );
            crate::src::qcommon::msg::MSG_WriteByte(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                crate::src::client::cl_main::clc.voipFlags as libc::c_int,
            );
            crate::src::qcommon::msg::MSG_WriteShort(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                crate::src::client::cl_main::clc.voipOutgoingDataSize,
            );
            crate::src::qcommon::msg::MSG_WriteData(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                crate::src::client::cl_main::clc
                    .voipOutgoingData
                    .as_mut_ptr() as *const libc::c_void,
                crate::src::client::cl_main::clc.voipOutgoingDataSize,
            );
            // If we're recording a demo, we have to fake a server packet with
            //  this VoIP data so it gets to disk; the server doesn't send it
            //  back to us, and we might as well eliminate concerns about dropped
            //  and misordered packets here.
            if crate::src::client::cl_main::clc.demorecording as libc::c_uint != 0
                && crate::src::client::cl_main::clc.demowaiting as u64 == 0
            {
                let voipSize: libc::c_int = crate::src::client::cl_main::clc.voipOutgoingDataSize;
                let mut fakemsg: crate::qcommon_h::msg_t = crate::qcommon_h::msg_t {
                    allowoverflow: crate::src::qcommon::q_shared::qfalse,
                    overflowed: crate::src::qcommon::q_shared::qfalse,
                    oob: crate::src::qcommon::q_shared::qfalse,
                    data: 0 as *mut crate::src::qcommon::q_shared::byte,
                    maxsize: 0,
                    cursize: 0,
                    readcount: 0,
                    bit: 0,
                };
                let mut fakedata: [crate::src::qcommon::q_shared::byte; 16384] = [0; 16384];
                crate::src::qcommon::msg::MSG_Init(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                    fakedata.as_mut_ptr(),
                    ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16384]>()
                        as libc::c_ulong as libc::c_int,
                );
                crate::src::qcommon::msg::MSG_Bitstream(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                );
                crate::src::qcommon::msg::MSG_WriteLong(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                    crate::src::client::cl_main::clc.reliableAcknowledge,
                );
                crate::src::qcommon::msg::MSG_WriteByte(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                    crate::qcommon_h::svc_voipOpus as libc::c_int,
                );
                crate::src::qcommon::msg::MSG_WriteShort(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                    crate::src::client::cl_main::clc.clientNum,
                );
                crate::src::qcommon::msg::MSG_WriteByte(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                    crate::src::client::cl_main::clc.voipOutgoingGeneration as libc::c_int,
                );
                crate::src::qcommon::msg::MSG_WriteLong(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                    crate::src::client::cl_main::clc.voipOutgoingSequence,
                );
                crate::src::qcommon::msg::MSG_WriteByte(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                    crate::src::client::cl_main::clc.voipOutgoingDataFrames,
                );
                crate::src::qcommon::msg::MSG_WriteShort(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                    crate::src::client::cl_main::clc.voipOutgoingDataSize,
                );
                crate::src::qcommon::msg::MSG_WriteBits(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                    crate::src::client::cl_main::clc.voipFlags as libc::c_int,
                    2 as libc::c_int,
                );
                crate::src::qcommon::msg::MSG_WriteData(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                    crate::src::client::cl_main::clc
                        .voipOutgoingData
                        .as_mut_ptr() as *const libc::c_void,
                    voipSize,
                );
                crate::src::qcommon::msg::MSG_WriteByte(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                    crate::qcommon_h::svc_EOF as libc::c_int,
                );
                crate::src::client::cl_main::CL_WriteDemoMessage(
                    &mut fakemsg as *mut _ as *mut crate::qcommon_h::msg_t,
                    0 as libc::c_int,
                );
            }
            crate::src::client::cl_main::clc.voipOutgoingSequence +=
                crate::src::client::cl_main::clc.voipOutgoingDataFrames;
            crate::src::client::cl_main::clc.voipOutgoingDataSize = 0 as libc::c_int;
            crate::src::client::cl_main::clc.voipOutgoingDataFrames = 0 as libc::c_int
        } else {
            // We have data, but no targets. Silently discard all data
            crate::src::client::cl_main::clc.voipOutgoingDataSize = 0 as libc::c_int;
            crate::src::client::cl_main::clc.voipOutgoingDataFrames = 0 as libc::c_int
        }
    }
    if count >= 1 as libc::c_int {
        if (*crate::src::client::cl_main::cl_showSend).integer != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"(%i)\x00" as *const u8 as *const libc::c_char,
                count,
            );
        }
        // begin a client move command
        if (*crate::src::client::cl_main::cl_nodelta).integer != 0
            || crate::src::client::cl_main::cl.snap.valid as u64 == 0
            || crate::src::client::cl_main::clc.demowaiting as libc::c_uint != 0
            || crate::src::client::cl_main::clc.serverMessageSequence
                != crate::src::client::cl_main::cl.snap.messageNum
        {
            crate::src::qcommon::msg::MSG_WriteByte(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                crate::qcommon_h::clc_moveNoDelta as libc::c_int,
            );
        } else {
            crate::src::qcommon::msg::MSG_WriteByte(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                crate::qcommon_h::clc_move as libc::c_int,
            );
        }
        // write the command count
        crate::src::qcommon::msg::MSG_WriteByte(
            &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
            count,
        );
        // use the checksum feed in the key
        key = crate::src::client::cl_main::clc.checksumFeed;
        // also use the message acknowledge
        key ^= crate::src::client::cl_main::clc.serverMessageSequence;
        // also use the last acknowledged server command in the key
        key ^= crate::src::qcommon::msg::MSG_HashKey(
            crate::src::client::cl_main::clc.serverCommands[(crate::src::client::cl_main::clc
                .serverCommandSequence
                & 64 as libc::c_int - 1 as libc::c_int)
                as usize]
                .as_mut_ptr(),
            32 as libc::c_int,
        );
        // write all the commands, including the predicted command
        i = 0 as libc::c_int;
        while i < count {
            j = crate::src::client::cl_main::cl.cmdNumber - count + i + 1 as libc::c_int
                & 64 as libc::c_int - 1 as libc::c_int;
            cmd = &mut *crate::src::client::cl_main::cl
                .cmds
                .as_mut_ptr()
                .offset(j as isize)
                as *mut crate::src::qcommon::q_shared::usercmd_t;
            crate::src::qcommon::msg::MSG_WriteDeltaUsercmdKey(
                &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
                key,
                oldcmd as *mut crate::src::qcommon::q_shared::usercmd_s,
                cmd as *mut crate::src::qcommon::q_shared::usercmd_s,
            );
            oldcmd = cmd;
            i += 1
        }
    }
    //
    // deliver the message
    //
    packetNum = crate::src::client::cl_main::clc.netchan.outgoingSequence
        & 32 as libc::c_int - 1 as libc::c_int;
    crate::src::client::cl_main::cl.outPackets[packetNum as usize].p_realtime =
        crate::src::client::cl_main::cls.realtime;
    crate::src::client::cl_main::cl.outPackets[packetNum as usize].p_serverTime =
        (*oldcmd).serverTime;
    crate::src::client::cl_main::cl.outPackets[packetNum as usize].p_cmdNumber =
        crate::src::client::cl_main::cl.cmdNumber;
    crate::src::client::cl_main::clc.lastPacketSentTime = crate::src::client::cl_main::cls.realtime;
    if (*crate::src::client::cl_main::cl_showSend).integer != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"%i \x00" as *const u8 as *const libc::c_char,
            buf.cursize,
        );
    }
    crate::src::client::cl_net_chan::CL_Netchan_Transmit(
        &mut crate::src::client::cl_main::clc.netchan as *mut _ as *mut crate::qcommon_h::netchan_t,
        &mut buf as *mut _ as *mut crate::qcommon_h::msg_t,
    );
}
/*
=================
CL_SendCmd

Called every frame to builds and sends a command packet to the server.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_SendCmd() {
    // don't send any message if not connected
    if (crate::src::client::cl_main::clc.state as libc::c_uint)
        < crate::src::qcommon::q_shared::CA_CONNECTED as libc::c_int as libc::c_uint
    {
        return;
    }
    // don't send commands if paused
    if (*crate::src::qcommon::common::com_sv_running).integer != 0
        && (*crate::src::qcommon::common::sv_paused).integer != 0
        && (*crate::src::qcommon::common::cl_paused).integer != 0
    {
        return;
    }
    // we create commands even if a demo is playing,
    CL_CreateNewCommands();
    // don't send a packet if the last packet was sent too recently
    if CL_ReadyToSendPacket() as u64 == 0 {
        if (*crate::src::client::cl_main::cl_showSend).integer != 0 {
            crate::src::qcommon::common::Com_Printf(b". \x00" as *const u8 as *const libc::c_char);
        }
        return;
    }
    CL_WritePacket();
}
/*
============
CL_InitInput
============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_InitInput() {
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"centerview\x00" as *const u8 as *const libc::c_char,
        Some(IN_CenterView as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+moveup\x00" as *const u8 as *const libc::c_char,
        Some(IN_UpDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-moveup\x00" as *const u8 as *const libc::c_char,
        Some(IN_UpUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+movedown\x00" as *const u8 as *const libc::c_char,
        Some(IN_DownDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-movedown\x00" as *const u8 as *const libc::c_char,
        Some(IN_DownUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+left\x00" as *const u8 as *const libc::c_char,
        Some(IN_LeftDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-left\x00" as *const u8 as *const libc::c_char,
        Some(IN_LeftUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+right\x00" as *const u8 as *const libc::c_char,
        Some(IN_RightDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-right\x00" as *const u8 as *const libc::c_char,
        Some(IN_RightUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+forward\x00" as *const u8 as *const libc::c_char,
        Some(IN_ForwardDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-forward\x00" as *const u8 as *const libc::c_char,
        Some(IN_ForwardUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+back\x00" as *const u8 as *const libc::c_char,
        Some(IN_BackDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-back\x00" as *const u8 as *const libc::c_char,
        Some(IN_BackUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+lookup\x00" as *const u8 as *const libc::c_char,
        Some(IN_LookupDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-lookup\x00" as *const u8 as *const libc::c_char,
        Some(IN_LookupUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+lookdown\x00" as *const u8 as *const libc::c_char,
        Some(IN_LookdownDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-lookdown\x00" as *const u8 as *const libc::c_char,
        Some(IN_LookdownUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+strafe\x00" as *const u8 as *const libc::c_char,
        Some(IN_StrafeDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-strafe\x00" as *const u8 as *const libc::c_char,
        Some(IN_StrafeUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+moveleft\x00" as *const u8 as *const libc::c_char,
        Some(IN_MoveleftDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-moveleft\x00" as *const u8 as *const libc::c_char,
        Some(IN_MoveleftUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+moveright\x00" as *const u8 as *const libc::c_char,
        Some(IN_MoverightDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-moveright\x00" as *const u8 as *const libc::c_char,
        Some(IN_MoverightUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+speed\x00" as *const u8 as *const libc::c_char,
        Some(IN_SpeedDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-speed\x00" as *const u8 as *const libc::c_char,
        Some(IN_SpeedUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+attack\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button0Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-attack\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button0Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button0\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button0Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button0\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button0Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button1\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button1Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button1\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button1Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button2\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button2Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button2\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button2Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button3\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button3Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button3\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button3Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button4\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button4Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button4\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button4Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button5\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button5Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button5\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button5Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button6\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button6Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button6\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button6Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button7\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button7Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button7\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button7Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button8\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button8Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button8\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button8Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button9\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button9Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button9\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button9Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button10\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button10Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button10\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button10Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button11\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button11Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button11\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button11Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button12\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button12Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button12\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button12Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button13\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button13Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button13\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button13Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+button14\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button14Down as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-button14\x00" as *const u8 as *const libc::c_char,
        Some(IN_Button14Up as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+mlook\x00" as *const u8 as *const libc::c_char,
        Some(IN_MLookDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-mlook\x00" as *const u8 as *const libc::c_char,
        Some(IN_MLookUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"+voiprecord\x00" as *const u8 as *const libc::c_char,
        Some(IN_VoipRecordDown as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"-voiprecord\x00" as *const u8 as *const libc::c_char,
        Some(IN_VoipRecordUp as unsafe extern "C" fn() -> ()),
    );
    crate::src::client::cl_main::cl_nodelta = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_nodelta\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    crate::src::client::cl_main::cl_debugMove = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_debugMove\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
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
/*
============
CL_ShutdownInput
============
*/
#[no_mangle]

pub unsafe extern "C" fn CL_ShutdownInput() {
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"centerview\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"+moveup\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"-moveup\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+movedown\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-movedown\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"+left\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"-left\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"+right\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"-right\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+forward\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-forward\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"+back\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"-back\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"+lookup\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"-lookup\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+lookdown\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-lookdown\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"+strafe\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"-strafe\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+moveleft\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-moveleft\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+moveright\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-moveright\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"+speed\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"-speed\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"+attack\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"-attack\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button2\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button2\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button3\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button3\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button4\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button4\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button5\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button5\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button6\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button6\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button7\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button7\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button8\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button8\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button9\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button9\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button10\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button10\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button11\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button11\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button12\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button12\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button13\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button13\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+button14\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-button14\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"+mlook\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"-mlook\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"+voiprecord\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"-voiprecord\x00" as *const u8 as *const libc::c_char,
    );
}

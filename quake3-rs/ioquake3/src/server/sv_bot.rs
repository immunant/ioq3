use ::libc;

pub mod q_shared_h {
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
    // q_shared.h -- included first by ALL program modules.
    // A user mod should never modify this file
    // Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
    // When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
    // You shouldn't change this unless you know what you're doing
    // number of supported master servers
    // standard demo extension
    //Ignore __attribute__ on non-gcc platforms
    /* *********************************************************************
     VM Considerations

     The VM can not use the standard system headers because we aren't really
     using the compiler they were meant for.  We use bg_lib.h which contains
     prototypes for the functions we define for our own use in bg_lib.c.

     When writing mods, please add needed headers HERE, do not start including
     stuff like <stdio.h> in the various .c files that make up each of the VMs
     since you will be including system headers files can will have issues.

     Remember, if you use a C library function that is not defined in bg_lib.c,
     you will have to add your own version for support in the VM.

    **********************************************************************/
    //=============================================================

    // parameters to the main Error routine

    // pop up the need-cd dialog
    // client disconnected from the server

    // don't kill server

    // print to console and disconnect from game

    // exit the entire game with a popup window

    // font rendering values used by ui and cgame
    // default
    // default

    /*
    ==============================================================

    MATHLIB

    ==============================================================
    */

    /*
    // if your system does not have lrintf() and round() you can try this block. Please also open a bug report at bugzilla.icculus.org
    // or write a mail to the ioq3 mailing list.
    #else
      #define Q_ftol(v) ((long) (v))
      #define Q_round(v) do { if((v) < 0) (v) -= 0.5f; else (v) += 0.5f; (v) = Q_ftol((v)); } while(0)
      #define Q_SnapVector(vec) \
        do\
        {\
            vec3_t *temp = (vec);\
            \
            Q_round((*temp)[0]);\
            Q_round((*temp)[1]);\
            Q_round((*temp)[2]);\
        } while(0)
    #endif
    */
    // reciprocal square root
    // this isn't a real cheap function to call!
    // just in case you don't want to use the macros
    // fast vector normalize routine that does not check to make sure
    // that length != 0, nor does it return length, uses rsqrt approximation
    // returns vector length
    // perpendicular vector could be replaced by this
    //int	PlaneTypeForNormal (vec3_t normal);
    //=============================================
    //int		COM_ParseInfos( char *buf, int max, char infos[][MAX_INFO_STRING] );
    //token types
    // string
    // literal
    // number
    // name
    // punctuation
    // data is an in/out parm, returns a parsed out token
    // mode parm for FS_FOpenFile
    //=============================================
    // portable case insensitive compare
    // buffer size safe library replacements
    // strlen that discounts Quake color sequences
    // removes color sequences from string
    // Count the number of char tocount encountered in string
    //=============================================
    // 64-bit integers for global rankings interface
    // implemented as a struct for qvm compatibility
    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */
    //=============================================
    //
    // key / value info strings
    //
    // this is only here so the functions in q_shared.c and bg_*.c can link
    /*
    ==========================================================

    CVARS (console variables)

    Many variables can be used for cheating purposes, so when
    cheats is zero, force all unspecified variables to their
    default values.
    ==========================================================
    */
    // set to cause it to be saved to vars.rc
    // used for system variables, not for player
    // specific configurations
    // sent to server on connect or change
    // sent in response to front end requests
    // these cvars will be duplicated on all clients
    // don't allow change from console at all,
    // but can be set from the command line
    // will only change when C code next does
    // a Cvar_Get(), so it can't be changed
    // without proper initialization.  modified
    // will be set, even though the value hasn't
    // changed yet
    // display only, cannot be set by user at all
    // created by a set command
    // can be set even when cheats are disabled, but is not archived
    // can not be changed if cheats are disabled
    // do not clear when a cvar_restart is issued
    // cvar was created by a server the client connected to.
    // cvar was created exclusively in one of the VMs.
    // prevent modifying this var from VMs or the server
    // These flags are only returned by the Cvar_Flags() function
    // Cvar was modified
    // Cvar doesn't exist.
    // nothing outside the Cvar_*() functions should modify these fields!
    // cvar_restart will reset to this value
    // for CVAR_LATCH vars
    // set each time the cvar is changed
    // incremented each time the cvar is changed
    // atof( string )
    // atoi( string )
    // the modules that run in the virtual machine can't access the cvar_t directly,
    // so they must ask for structured updates
    /*
    ==============================================================

    VoIP

    ==============================================================
    */
    // if you change the count of flags be sure to also change VOIP_FLAGNUM
    // spatialized voip message
    // non-spatialized voip message
    // number of flags voip knows. You will have to bump protocol version number if you
    // change this.
    /*
    ==============================================================

    COLLISION DETECTION

    ==============================================================
    */
    // plane types are used to speed some tests
    // 0-2 are axial planes
    /*
    =================
    PlaneTypeForNormal
    =================
    */
    // plane_t structure
    // !!! if this is changed, it must be changed in asm code too !!!

    // a trace is returned when a box is swept through the world

    //=========================================================
    // bit field limits
    // playerState_t is the information needed by both the client and server
    // to predict player motion and actions
    // nothing outside of pmove should modify these, or some degree of prediction error
    // will occur
    // you can't add anything to this without modifying the code in msg.c
    // playerState_t is a full superset of entityState_t as it is used by players,
    // so if a playerState_t is transmitted, the entityState_t can be fully derived
    // from it.

    //====================================================================
    //
    // usercmd_t->button bits, many of which are generated by the client system,
    // so they aren't game/cgame only definitions
    //
    // displays talk balloon and disables actions
    // walking can't just be inferred from MOVE_RUN
    // because a key pressed late in the frame will
    // only generate a small move value for that frame
    // walking will use different animations and
    // won't generate footsteps
    // any key whatsoever
    // if forwardmove or rightmove are >= MOVE_RUN,
    // then BUTTON_WALKING should be set
    // usercmd_t is sent to the server each client frame

    //===================================================================
    // if entityState->solid == SOLID_BMODEL, modelindex is an inline model number

    // value = base + sin( time / duration ) * delta

    // non-parametric, but interpolate between snapshots

    // entityState_t is the information conveyed from the server
    // in an update message about entities that the client will
    // need to render in some way
    // Different eTypes may use the information in different ways
    // The messages are delta compressed, so it doesn't really matter if
    // the structure size is fairly large

    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0 as i32 as isize) = *v1.offset(1 as i32 as isize)
            * *v2.offset(2 as i32 as isize)
            - *v1.offset(2 as i32 as isize) * *v2.offset(1 as i32 as isize);
        *cross.offset(1 as i32 as isize) = *v1.offset(2 as i32 as isize)
            * *v2.offset(0 as i32 as isize)
            - *v1.offset(0 as i32 as isize) * *v2.offset(2 as i32 as isize);
        *cross.offset(2 as i32 as isize) = *v1.offset(0 as i32 as isize)
            * *v2.offset(1 as i32 as isize)
            - *v1.offset(1 as i32 as isize) * *v2.offset(0 as i32 as isize);
    }

    // __Q_SHARED_H
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stdlib::intptr_t;

pub use crate::be_aas_h::aas_altroutegoal_s;
pub use crate::be_aas_h::aas_areainfo_s;
pub use crate::be_aas_h::aas_clientmove_s;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_predictroute_s;
pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::botlib_h::aas_export_s;
pub use crate::botlib_h::aas_export_t;
pub use crate::botlib_h::ai_export_s;
pub use crate::botlib_h::ai_export_t;
pub use crate::botlib_h::bot_entitystate_s;
pub use crate::botlib_h::bot_entitystate_t;
pub use crate::botlib_h::bot_input_s;
pub use crate::botlib_h::bot_input_t;
pub use crate::botlib_h::botlib_export_s;
pub use crate::botlib_h::botlib_export_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::botlib_h::ea_export_s;
pub use crate::botlib_h::ea_export_t;
pub use crate::g_public_h::entityShared_t;
pub use crate::g_public_h::sharedEntity_t;
pub use crate::g_public_h::BOTAI_START_FRAME;
pub use crate::g_public_h::GAME_CLIENT_BEGIN;
pub use crate::g_public_h::GAME_CLIENT_COMMAND;
pub use crate::g_public_h::GAME_CLIENT_CONNECT;
pub use crate::g_public_h::GAME_CLIENT_DISCONNECT;
pub use crate::g_public_h::GAME_CLIENT_THINK;
pub use crate::g_public_h::GAME_CLIENT_USERINFO_CHANGED;
pub use crate::g_public_h::GAME_CONSOLE_COMMAND;
pub use crate::g_public_h::GAME_INIT;
pub use crate::g_public_h::GAME_RUN_FRAME;
pub use crate::g_public_h::GAME_SHUTDOWN;
pub use crate::qcommon_h::msg_t;
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
pub use crate::qcommon_h::TAG_BOTLIB;
pub use crate::qcommon_h::TAG_FREE;
pub use crate::qcommon_h::TAG_GENERAL;
pub use crate::qcommon_h::TAG_RENDERER;
pub use crate::qcommon_h::TAG_SMALL;
pub use crate::qcommon_h::TAG_STATIC;
pub use crate::server_h::challenge_t;
pub use crate::server_h::clientSnapshot_t;
pub use crate::server_h::clientState_t;
pub use crate::server_h::client_s;
pub use crate::server_h::client_t;
pub use crate::server_h::netchan_buffer_s;
pub use crate::server_h::netchan_buffer_t;
pub use crate::server_h::serverStatic_t;
pub use crate::server_h::voipServerPacket_s;
pub use crate::server_h::voipServerPacket_t;
pub use crate::server_h::CS_ACTIVE;
pub use crate::server_h::CS_CONNECTED;
pub use crate::server_h::CS_FREE;
pub use crate::server_h::CS_PRIMED;
pub use crate::server_h::CS_ZOMBIE;
pub use crate::src::botlib::be_ai_chat::bot_consolemessage_s;
pub use crate::src::botlib::be_ai_chat::bot_match_s;
pub use crate::src::botlib::be_ai_goal::bot_goal_s;
pub use crate::src::botlib::be_ai_move::bot_initmove_s;
pub use crate::src::botlib::be_ai_move::bot_moveresult_s;
pub use crate::src::botlib::be_ai_weap::weaponinfo_s;
pub use crate::src::botlib::be_interface::GetBotLibAPI;

pub use crate::src::qcommon::common::com_basegame;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Hunk_Alloc;
pub use crate::src::qcommon::common::Hunk_CheckMark;
pub use crate::src::qcommon::common::Z_AvailableMemory;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::common::Z_Malloc;
pub use crate::src::qcommon::common::Z_TagMalloc;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_VariableIntegerValue;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_FOpenFileByMode;
pub use crate::src::qcommon::files::FS_Read;
pub use crate::src::qcommon::files::FS_Seek;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::q_math::RadiusFromBounds;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::pc_token_s;
pub use crate::src::qcommon::q_shared::pc_token_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::qcommon::vm::VM_Call;
pub use crate::src::server::sv_bot::q_shared_h::CrossProduct;
pub use crate::src::server::sv_client::SV_ExecuteClientCommand;
pub use crate::src::server::sv_game::SV_GentityNum;
pub use crate::src::server::sv_game::SV_inPVS;
pub use crate::src::server::sv_main::gvm;
pub use crate::src::server::sv_main::sv_maxclients;
pub use crate::src::server::sv_main::svs;
pub use crate::src::server::sv_world::SV_ClipToEntity;
pub use crate::src::server::sv_world::SV_PointContents;
pub use crate::src::server::sv_world::SV_Trace;

pub use crate::vm_local_h::vm_s;
extern "C" {
    #[no_mangle]
    pub static mut botlib_export: *mut botlib_export_t;
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
// sv_bot.c

pub type bot_debugpoly_t = bot_debugpoly_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_debugpoly_s {
    pub inuse: i32,
    pub color: i32,
    pub numPoints: i32,
    pub points: [vec3_t; 128],
}

static mut debugpolygons: *mut bot_debugpoly_t =
    0 as *const bot_debugpoly_t as *mut bot_debugpoly_t;
#[no_mangle]

pub static mut bot_maxdebugpolys: i32 = 0;
#[no_mangle]

pub static mut bot_enable: i32 = 0;
/*
==================
SV_BotAllocateClient
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_BotAllocateClient() -> i32 {
    let mut i: i32 = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    // find a client slot
    i = 0 as i32;
    cl = svs.clients;
    while i < (*sv_maxclients).integer {
        if (*cl).state as u32 == CS_FREE as i32 as u32 {
            break;
        }
        i += 1;
        cl = cl.offset(1)
    }
    if i == (*sv_maxclients).integer {
        return -(1 as i32);
    }
    (*cl).gentity = SV_GentityNum(i) as *mut sharedEntity_t;
    (*(*cl).gentity).s.number = i;
    (*cl).state = CS_ACTIVE;
    (*cl).lastPacketTime = svs.time;
    (*cl).netchan.remoteAddress.type_0 = NA_BOT;
    (*cl).rate = 16384 as i32;
    return i;
}
/*
==================
SV_BotFreeClient
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_BotFreeClient(mut clientNum: i32) {
    let mut cl: *mut client_t = 0 as *mut client_t;
    if clientNum < 0 as i32 || clientNum >= (*sv_maxclients).integer {
        Com_Error(
            ERR_DROP as i32,
            b"SV_BotFreeClient: bad clientNum: %i\x00" as *const u8 as *const libc::c_char,
            clientNum,
        );
    }
    cl = &mut *svs.clients.offset(clientNum as isize) as *mut client_t;
    (*cl).state = CS_FREE;
    (*cl).name[0 as i32 as usize] = 0 as i32 as libc::c_char;
    if !(*cl).gentity.is_null() {
        (*(*cl).gentity).r.svFlags &= !(0x8 as i32)
    };
}
/*
==================
BotDrawDebugPolygons
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotDrawDebugPolygons(
    mut drawPoly: Option<unsafe extern "C" fn(_: i32, _: i32, _: *mut f32) -> ()>,
    mut _value: i32,
) {
    static mut bot_debug: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
    static mut bot_groundonly: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
    static mut bot_reachability: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
    static mut bot_highlightarea: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
    let mut poly: *mut bot_debugpoly_t = 0 as *mut bot_debugpoly_t;
    let mut i: i32 = 0;
    let mut parm0: i32 = 0;
    if debugpolygons.is_null() {
        return;
    }
    //bot debugging
    if bot_debug.is_null() {
        bot_debug = Cvar_Get(
            b"bot_debug\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            0 as i32,
        ) as *mut cvar_s
    }
    //
    if bot_enable != 0 && (*bot_debug).integer != 0 {
        //end if
        //show reachabilities
        if bot_reachability.is_null() {
            bot_reachability = Cvar_Get(
                b"bot_reachability\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
                0 as i32,
            ) as *mut cvar_s
        }
        //show ground faces only
        if bot_groundonly.is_null() {
            bot_groundonly = Cvar_Get(
                b"bot_groundonly\x00" as *const u8 as *const libc::c_char,
                b"1\x00" as *const u8 as *const libc::c_char,
                0 as i32,
            ) as *mut cvar_s
        }
        //get the hightlight area
        if bot_highlightarea.is_null() {
            bot_highlightarea = Cvar_Get(
                b"bot_highlightarea\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
                0 as i32,
            ) as *mut cvar_s
        }
        //
        parm0 = 0 as i32;
        if (*svs.clients.offset(0 as i32 as isize)).lastUsercmd.buttons & 1 as i32 != 0 {
            parm0 |= 1 as i32
        }
        if (*bot_reachability).integer != 0 {
            parm0 |= 2 as i32
        }
        if (*bot_groundonly).integer != 0 {
            parm0 |= 4 as i32
        }
        (*botlib_export)
            .BotLibVarSet
            .expect("non-null function pointer")(
            b"bot_highlightarea\x00" as *const u8 as *const libc::c_char,
            (*bot_highlightarea).string,
        );
        (*botlib_export).Test.expect("non-null function pointer")(
            parm0,
            0 as *mut libc::c_char,
            (*(*svs.clients.offset(0 as i32 as isize)).gentity)
                .r
                .currentOrigin
                .as_mut_ptr(),
            (*(*svs.clients.offset(0 as i32 as isize)).gentity)
                .r
                .currentAngles
                .as_mut_ptr(),
        );
    }
    //draw all debug polys
    i = 0 as i32;
    while i < bot_maxdebugpolys {
        poly = &mut *debugpolygons.offset(i as isize) as *mut bot_debugpoly_t;
        if !((*poly).inuse == 0) {
            drawPoly.expect("non-null function pointer")(
                (*poly).color,
                (*poly).numPoints,
                (*poly).points.as_mut_ptr() as *mut f32,
            );
        }
        i += 1
        //Com_Printf("poly %i, numpoints = %d\n", i, poly->numPoints);
    }
}
/*
==================
BotImport_Print
==================
*/

unsafe extern "C" fn BotImport_Print(mut type_0: i32, mut fmt: *mut libc::c_char, mut args: ...) {
    let mut str: [libc::c_char; 2048] = [0; 2048];
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    crate::stdlib::vsnprintf(
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2048]>() as usize,
        fmt,
        ap.as_va_list(),
    );
    match type_0 {
        1 => {
            Com_Printf(
                b"%s\x00" as *const u8 as *const libc::c_char,
                str.as_mut_ptr(),
            );
        }
        2 => {
            Com_Printf(
                b"^3Warning: %s\x00" as *const u8 as *const libc::c_char,
                str.as_mut_ptr(),
            );
        }
        3 => {
            Com_Printf(
                b"^1Error: %s\x00" as *const u8 as *const libc::c_char,
                str.as_mut_ptr(),
            );
        }
        4 => {
            Com_Printf(
                b"^1Fatal: %s\x00" as *const u8 as *const libc::c_char,
                str.as_mut_ptr(),
            );
        }
        5 => {
            Com_Error(
                ERR_DROP as i32,
                b"^1Exit: %s\x00" as *const u8 as *const libc::c_char,
                str.as_mut_ptr(),
            );
        }
        _ => {
            Com_Printf(b"unknown print type\n\x00" as *const u8 as *const libc::c_char);
        }
    };
}
/*
==================
BotImport_Trace
==================
*/

unsafe extern "C" fn BotImport_Trace(
    mut bsptrace: *mut bsp_trace_t,
    mut start: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut end: *mut vec_t,
    mut passent: i32,
    mut contentmask: i32,
) {
    let mut trace: trace_t = trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surfaceFlags: 0,
        contents: 0,
        entityNum: 0,
    };
    SV_Trace(
        &mut trace as *mut _ as *mut trace_t,
        start as *const vec_t,
        mins,
        maxs,
        end as *const vec_t,
        passent,
        contentmask,
        qfalse as i32,
    );
    //copy the trace information
    (*bsptrace).allsolid = trace.allsolid;
    (*bsptrace).startsolid = trace.startsolid;
    (*bsptrace).fraction = trace.fraction;
    (*bsptrace).endpos[0 as i32 as usize] = trace.endpos[0 as i32 as usize];
    (*bsptrace).endpos[1 as i32 as usize] = trace.endpos[1 as i32 as usize];
    (*bsptrace).endpos[2 as i32 as usize] = trace.endpos[2 as i32 as usize];
    (*bsptrace).plane.dist = trace.plane.dist;
    (*bsptrace).plane.normal[0 as i32 as usize] = trace.plane.normal[0 as i32 as usize];
    (*bsptrace).plane.normal[1 as i32 as usize] = trace.plane.normal[1 as i32 as usize];
    (*bsptrace).plane.normal[2 as i32 as usize] = trace.plane.normal[2 as i32 as usize];
    (*bsptrace).plane.signbits = trace.plane.signbits;
    (*bsptrace).plane.type_0 = trace.plane.type_0;
    (*bsptrace).surface.value = 0 as i32;
    (*bsptrace).surface.flags = trace.surfaceFlags;
    (*bsptrace).ent = trace.entityNum;
    (*bsptrace).exp_dist = 0 as i32 as f32;
    (*bsptrace).sidenum = 0 as i32;
    (*bsptrace).contents = 0 as i32;
}
/*
==================
BotImport_EntityTrace
==================
*/

unsafe extern "C" fn BotImport_EntityTrace(
    mut bsptrace: *mut bsp_trace_t,
    mut start: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut end: *mut vec_t,
    mut entnum: i32,
    mut contentmask: i32,
) {
    let mut trace: trace_t = trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surfaceFlags: 0,
        contents: 0,
        entityNum: 0,
    };
    SV_ClipToEntity(
        &mut trace as *mut _ as *mut trace_t,
        start as *const vec_t,
        mins as *const vec_t,
        maxs as *const vec_t,
        end as *const vec_t,
        entnum,
        contentmask,
        qfalse as i32,
    );
    //copy the trace information
    (*bsptrace).allsolid = trace.allsolid;
    (*bsptrace).startsolid = trace.startsolid;
    (*bsptrace).fraction = trace.fraction;
    (*bsptrace).endpos[0 as i32 as usize] = trace.endpos[0 as i32 as usize];
    (*bsptrace).endpos[1 as i32 as usize] = trace.endpos[1 as i32 as usize];
    (*bsptrace).endpos[2 as i32 as usize] = trace.endpos[2 as i32 as usize];
    (*bsptrace).plane.dist = trace.plane.dist;
    (*bsptrace).plane.normal[0 as i32 as usize] = trace.plane.normal[0 as i32 as usize];
    (*bsptrace).plane.normal[1 as i32 as usize] = trace.plane.normal[1 as i32 as usize];
    (*bsptrace).plane.normal[2 as i32 as usize] = trace.plane.normal[2 as i32 as usize];
    (*bsptrace).plane.signbits = trace.plane.signbits;
    (*bsptrace).plane.type_0 = trace.plane.type_0;
    (*bsptrace).surface.value = 0 as i32;
    (*bsptrace).surface.flags = trace.surfaceFlags;
    (*bsptrace).ent = trace.entityNum;
    (*bsptrace).exp_dist = 0 as i32 as f32;
    (*bsptrace).sidenum = 0 as i32;
    (*bsptrace).contents = 0 as i32;
}
/*
==================
BotImport_PointContents
==================
*/

unsafe extern "C" fn BotImport_PointContents(mut point: *mut vec_t) -> i32 {
    return SV_PointContents(point as *const vec_t, -(1 as i32));
}
/*
==================
BotImport_inPVS
==================
*/

unsafe extern "C" fn BotImport_inPVS(mut p1: *mut vec_t, mut p2: *mut vec_t) -> i32 {
    return SV_inPVS(p1 as *const vec_t, p2 as *const vec_t) as i32;
}
/*
==================
BotImport_BSPEntityData
==================
*/

unsafe extern "C" fn BotImport_BSPEntityData() -> *mut libc::c_char {
    return crate::src::qcommon::cm_load::CM_EntityString();
}
/*
==================
BotImport_BSPModelMinsMaxsOrigin
==================
*/

unsafe extern "C" fn BotImport_BSPModelMinsMaxsOrigin(
    mut modelnum: i32,
    mut angles: *mut vec_t,
    mut outmins: *mut vec_t,
    mut outmaxs: *mut vec_t,
    mut origin: *mut vec_t,
) {
    let mut h: clipHandle_t = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut max: f32 = 0.;
    let mut i: i32 = 0;
    h = crate::src::qcommon::cm_load::CM_InlineModel(modelnum);
    crate::src::qcommon::cm_load::CM_ModelBounds(h, mins.as_mut_ptr(), maxs.as_mut_ptr());
    //if the model is rotated
    if *angles.offset(0 as i32 as isize) != 0.
        || *angles.offset(1 as i32 as isize) != 0.
        || *angles.offset(2 as i32 as isize) != 0.
    {
        // expand for rotation
        max = RadiusFromBounds(
            mins.as_mut_ptr() as *const vec_t,
            maxs.as_mut_ptr() as *const vec_t,
        );
        i = 0 as i32;
        while i < 3 as i32 {
            mins[i as usize] = -max;
            maxs[i as usize] = max;
            i += 1
        }
    }
    if !outmins.is_null() {
        *outmins.offset(0 as i32 as isize) = mins[0 as i32 as usize];
        *outmins.offset(1 as i32 as isize) = mins[1 as i32 as usize];
        *outmins.offset(2 as i32 as isize) = mins[2 as i32 as usize]
    }
    if !outmaxs.is_null() {
        *outmaxs.offset(0 as i32 as isize) = maxs[0 as i32 as usize];
        *outmaxs.offset(1 as i32 as isize) = maxs[1 as i32 as usize];
        *outmaxs.offset(2 as i32 as isize) = maxs[2 as i32 as usize]
    }
    if !origin.is_null() {
        let ref mut fresh0 = *origin.offset(2 as i32 as isize);
        *fresh0 = 0 as i32 as vec_t;
        let ref mut fresh1 = *origin.offset(1 as i32 as isize);
        *fresh1 = *fresh0;
        *origin.offset(0 as i32 as isize) = *fresh1
    };
}
/*
==================
BotImport_GetMemory
==================
*/

unsafe extern "C" fn BotImport_GetMemory(mut size: i32) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    ptr = Z_TagMalloc(size, TAG_BOTLIB as i32);
    return ptr;
}
/*
==================
BotImport_FreeMemory
==================
*/

unsafe extern "C" fn BotImport_FreeMemory(mut ptr: *mut libc::c_void) {
    Z_Free(ptr);
}
/*
=================
BotImport_HunkAlloc
=================
*/

unsafe extern "C" fn BotImport_HunkAlloc(mut size: i32) -> *mut libc::c_void {
    if Hunk_CheckMark() as u64 != 0 {
        Com_Error(
            ERR_DROP as i32,
            b"SV_Bot_HunkAlloc: Alloc with marks already set\x00" as *const u8
                as *const libc::c_char,
        );
    }
    return Hunk_Alloc(size, h_high);
}
/*
==================
BotImport_DebugPolygonCreate
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotImport_DebugPolygonCreate(
    mut color: i32,
    mut numPoints: i32,
    mut points: *mut vec3_t,
) -> i32 {
    let mut poly: *mut bot_debugpoly_t = 0 as *mut bot_debugpoly_t;
    let mut i: i32 = 0;
    if debugpolygons.is_null() {
        return 0 as i32;
    }
    i = 1 as i32;
    while i < bot_maxdebugpolys {
        if (*debugpolygons.offset(i as isize)).inuse == 0 {
            break;
        }
        i += 1
    }
    if i >= bot_maxdebugpolys {
        return 0 as i32;
    }
    poly = &mut *debugpolygons.offset(i as isize) as *mut bot_debugpoly_t;
    (*poly).inuse = qtrue as i32;
    (*poly).color = color;
    (*poly).numPoints = numPoints;
    crate::stdlib::memcpy(
        (*poly).points.as_mut_ptr() as *mut libc::c_void,
        points as *const libc::c_void,
        (numPoints as usize).wrapping_mul(::std::mem::size_of::<vec3_t>() as usize),
    );
    //
    return i;
}
/*
==================
BotImport_DebugPolygonShow
==================
*/

unsafe extern "C" fn BotImport_DebugPolygonShow(
    mut id: i32,
    mut color: i32,
    mut numPoints: i32,
    mut points: *mut vec3_t,
) {
    let mut poly: *mut bot_debugpoly_t = 0 as *mut bot_debugpoly_t;
    if debugpolygons.is_null() {
        return;
    }
    poly = &mut *debugpolygons.offset(id as isize) as *mut bot_debugpoly_t;
    (*poly).inuse = qtrue as i32;
    (*poly).color = color;
    (*poly).numPoints = numPoints;
    crate::stdlib::memcpy(
        (*poly).points.as_mut_ptr() as *mut libc::c_void,
        points as *const libc::c_void,
        (numPoints as usize).wrapping_mul(::std::mem::size_of::<vec3_t>() as usize),
    );
}
/*
==================
BotImport_DebugPolygonDelete
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotImport_DebugPolygonDelete(mut id: i32) {
    if debugpolygons.is_null() {
        return;
    }
    (*debugpolygons.offset(id as isize)).inuse = qfalse as i32;
}
/*
==================
BotImport_DebugLineCreate
==================
*/

unsafe extern "C" fn BotImport_DebugLineCreate() -> i32 {
    let mut points: [vec3_t; 1] = [[0.; 3]; 1];
    return BotImport_DebugPolygonCreate(0 as i32, 0 as i32, points.as_mut_ptr());
}
/*
==================
BotImport_DebugLineDelete
==================
*/

unsafe extern "C" fn BotImport_DebugLineDelete(mut line: i32) {
    BotImport_DebugPolygonDelete(line);
}
/*
==================
BotImport_DebugLineShow
==================
*/

unsafe extern "C" fn BotImport_DebugLineShow(
    mut line: i32,
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut color: i32,
) {
    let mut points: [vec3_t; 4] = [[0.; 3]; 4];
    let mut dir: vec3_t = [0.; 3];
    let mut cross: vec3_t = [0.; 3];
    let mut up: vec3_t = [0 as i32 as vec_t, 0 as i32 as vec_t, 1 as i32 as vec_t];
    let mut dot: f32 = 0.;
    points[0 as i32 as usize][0 as i32 as usize] = *start.offset(0 as i32 as isize);
    points[0 as i32 as usize][1 as i32 as usize] = *start.offset(1 as i32 as isize);
    points[0 as i32 as usize][2 as i32 as usize] = *start.offset(2 as i32 as isize);
    points[1 as i32 as usize][0 as i32 as usize] = *start.offset(0 as i32 as isize);
    points[1 as i32 as usize][1 as i32 as usize] = *start.offset(1 as i32 as isize);
    points[1 as i32 as usize][2 as i32 as usize] = *start.offset(2 as i32 as isize);
    //points[1][2] -= 2;
    points[2 as i32 as usize][0 as i32 as usize] = *end.offset(0 as i32 as isize);
    points[2 as i32 as usize][1 as i32 as usize] = *end.offset(1 as i32 as isize);
    points[2 as i32 as usize][2 as i32 as usize] = *end.offset(2 as i32 as isize);
    //points[2][2] -= 2;
    points[3 as i32 as usize][0 as i32 as usize] = *end.offset(0 as i32 as isize);
    points[3 as i32 as usize][1 as i32 as usize] = *end.offset(1 as i32 as isize);
    points[3 as i32 as usize][2 as i32 as usize] = *end.offset(2 as i32 as isize);
    dir[0 as i32 as usize] = *end.offset(0 as i32 as isize) - *start.offset(0 as i32 as isize);
    dir[1 as i32 as usize] = *end.offset(1 as i32 as isize) - *start.offset(1 as i32 as isize);
    dir[2 as i32 as usize] = *end.offset(2 as i32 as isize) - *start.offset(2 as i32 as isize);
    VectorNormalize(dir.as_mut_ptr());
    dot = dir[0 as i32 as usize] * up[0 as i32 as usize]
        + dir[1 as i32 as usize] * up[1 as i32 as usize]
        + dir[2 as i32 as usize] * up[2 as i32 as usize];
    if dot as f64 > 0.99f64 || (dot as f64) < -0.99f64 {
        cross[0 as i32 as usize] = 1 as i32 as vec_t;
        cross[1 as i32 as usize] = 0 as i32 as vec_t;
        cross[2 as i32 as usize] = 0 as i32 as vec_t
    } else {
        CrossProduct(
            dir.as_mut_ptr() as *const vec_t,
            up.as_mut_ptr() as *const vec_t,
            cross.as_mut_ptr(),
        );
    }
    VectorNormalize(cross.as_mut_ptr());
    points[0 as i32 as usize][0 as i32 as usize] =
        points[0 as i32 as usize][0 as i32 as usize] + cross[0 as i32 as usize] * 2 as i32 as f32;
    points[0 as i32 as usize][1 as i32 as usize] =
        points[0 as i32 as usize][1 as i32 as usize] + cross[1 as i32 as usize] * 2 as i32 as f32;
    points[0 as i32 as usize][2 as i32 as usize] =
        points[0 as i32 as usize][2 as i32 as usize] + cross[2 as i32 as usize] * 2 as i32 as f32;
    points[1 as i32 as usize][0 as i32 as usize] = points[1 as i32 as usize][0 as i32 as usize]
        + cross[0 as i32 as usize] * -(2 as i32) as f32;
    points[1 as i32 as usize][1 as i32 as usize] = points[1 as i32 as usize][1 as i32 as usize]
        + cross[1 as i32 as usize] * -(2 as i32) as f32;
    points[1 as i32 as usize][2 as i32 as usize] = points[1 as i32 as usize][2 as i32 as usize]
        + cross[2 as i32 as usize] * -(2 as i32) as f32;
    points[2 as i32 as usize][0 as i32 as usize] = points[2 as i32 as usize][0 as i32 as usize]
        + cross[0 as i32 as usize] * -(2 as i32) as f32;
    points[2 as i32 as usize][1 as i32 as usize] = points[2 as i32 as usize][1 as i32 as usize]
        + cross[1 as i32 as usize] * -(2 as i32) as f32;
    points[2 as i32 as usize][2 as i32 as usize] = points[2 as i32 as usize][2 as i32 as usize]
        + cross[2 as i32 as usize] * -(2 as i32) as f32;
    points[3 as i32 as usize][0 as i32 as usize] =
        points[3 as i32 as usize][0 as i32 as usize] + cross[0 as i32 as usize] * 2 as i32 as f32;
    points[3 as i32 as usize][1 as i32 as usize] =
        points[3 as i32 as usize][1 as i32 as usize] + cross[1 as i32 as usize] * 2 as i32 as f32;
    points[3 as i32 as usize][2 as i32 as usize] =
        points[3 as i32 as usize][2 as i32 as usize] + cross[2 as i32 as usize] * 2 as i32 as f32;
    BotImport_DebugPolygonShow(line, color, 4 as i32, points.as_mut_ptr());
}
/*
==================
SV_BotClientCommand
==================
*/

unsafe extern "C" fn BotClientCommand(mut client: i32, mut command: *mut libc::c_char) {
    SV_ExecuteClientCommand(
        &mut *svs.clients.offset(client as isize) as *mut _ as *mut client_s,
        command,
        qtrue,
    );
}
/*
==================
SV_BotFrame
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_BotFrame(mut time: i32) {
    if bot_enable == 0 {
        return;
    }
    //NOTE: maybe the game is already shutdown
    if gvm.is_null() {
        return;
    }
    VM_Call(gvm, BOTAI_START_FRAME as i32, time);
}
/*
===============
SV_BotLibSetup
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_BotLibSetup() -> i32 {
    if bot_enable == 0 {
        return 0 as i32;
    }
    if botlib_export.is_null() {
        Com_Printf(
            b"^1Error: SV_BotLibSetup without SV_BotInitBotLib\n\x00" as *const u8
                as *const libc::c_char,
        );
        return -(1 as i32);
    }
    (*botlib_export)
        .BotLibVarSet
        .expect("non-null function pointer")(
        b"basegame\x00" as *const u8 as *const libc::c_char,
        (*com_basegame).string,
    );
    return (*botlib_export)
        .BotLibSetup
        .expect("non-null function pointer")();
}
/*
===============
SV_ShutdownBotLib

Called when either the entire server is being killed, or
it is changing to a different game directory.
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_BotLibShutdown() -> i32 {
    if botlib_export.is_null() {
        return -(1 as i32);
    }
    return (*botlib_export)
        .BotLibShutdown
        .expect("non-null function pointer")();
}
/*
==================
SV_BotInitCvars
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_BotInitCvars() {
    Cvar_Get(
        b"bot_enable\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //enable the bot

    Cvar_Get(
        b"bot_developer\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s; //bot developer mode

    Cvar_Get(
        b"bot_debug\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s; //enable bot debugging

    Cvar_Get(
        b"bot_maxdebugpolys\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //maximum number of debug polys

    Cvar_Get(
        b"bot_groundonly\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //only show ground faces of areas

    Cvar_Get(
        b"bot_reachability\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //show all reachabilities to other areas

    Cvar_Get(
        b"bot_visualizejumppads\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s; //show jumppads

    Cvar_Get(
        b"bot_forceclustering\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //force cluster calculations

    Cvar_Get(
        b"bot_forcereachability\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //force reachability calculations

    Cvar_Get(
        b"bot_forcewrite\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //force writing aas file

    Cvar_Get(
        b"bot_aasoptimize\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //no aas file optimisation

    Cvar_Get(
        b"bot_saveroutingcache\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //save routing cache

    Cvar_Get(
        b"bot_thinktime\x00" as *const u8 as *const libc::c_char,
        b"100\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s; //msec the bots thinks

    Cvar_Get(
        b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //reload the bot characters each time

    Cvar_Get(
        b"bot_testichat\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //test ichats

    Cvar_Get(
        b"bot_testrchat\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //test rchats

    Cvar_Get(
        b"bot_testsolid\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s; //test for solid areas

    Cvar_Get(
        b"bot_testclusters\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s; //test the AAS clusters

    Cvar_Get(
        b"bot_fastchat\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //fast chatting bots

    Cvar_Get(
        b"bot_nochat\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //disable chats

    Cvar_Get(
        b"bot_pause\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s; //pause the bots thinking

    Cvar_Get(
        b"bot_report\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s; //get a full report in ctf

    Cvar_Get(
        b"bot_grapple\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //enable grapple

    Cvar_Get(
        b"bot_rocketjump\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //enable rocket jumping

    Cvar_Get(
        b"bot_challenge\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //challenging bot

    Cvar_Get(
        b"bot_minplayers\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut cvar_s; //minimum players in a team or the game

    Cvar_Get(
        b"bot_interbreedchar\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s; //bot character used for interbreeding

    Cvar_Get(
        b"bot_interbreedbots\x00" as *const u8 as *const libc::c_char,
        b"10\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s; //number of bots used for interbreeding

    Cvar_Get(
        b"bot_interbreedcycle\x00" as *const u8 as *const libc::c_char,
        b"20\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s; //bot interbreeding cycle

    Cvar_Get(
        b"bot_interbreedwrite\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    ) as *mut cvar_s;
    //write interbreeded bots to this file
}
/*
==================
SV_BotInitBotLib
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_BotInitBotLib() {
    let mut botlib_import: botlib_import_t = botlib_import_t {
        Print: None,
        Trace: None,
        EntityTrace: None,
        PointContents: None,
        inPVS: None,
        BSPEntityData: None,
        BSPModelMinsMaxsOrigin: None,
        BotClientCommand: None,
        GetMemory: None,
        FreeMemory: None,
        AvailableMemory: None,
        HunkAlloc: None,
        FS_FOpenFile: None,
        FS_Read: None,
        FS_Write: None,
        FS_FCloseFile: None,
        FS_Seek: None,
        DebugLineCreate: None,
        DebugLineDelete: None,
        DebugLineShow: None,
        DebugPolygonCreate: None,
        DebugPolygonDelete: None,
    };
    if !debugpolygons.is_null() {
        Z_Free(debugpolygons as *mut libc::c_void);
    }
    bot_maxdebugpolys =
        Cvar_VariableIntegerValue(b"bot_maxdebugpolys\x00" as *const u8 as *const libc::c_char);
    debugpolygons = Z_Malloc(
        (::std::mem::size_of::<bot_debugpoly_t>() as usize).wrapping_mul(bot_maxdebugpolys as usize)
            as i32,
    ) as *mut bot_debugpoly_t;
    botlib_import.Print =
        Some(BotImport_Print as unsafe extern "C" fn(_: i32, _: *mut libc::c_char, _: ...) -> ());
    botlib_import.Trace = Some(
        BotImport_Trace
            as unsafe extern "C" fn(
                _: *mut bsp_trace_t,
                _: *mut vec_t,
                _: *mut vec_t,
                _: *mut vec_t,
                _: *mut vec_t,
                _: i32,
                _: i32,
            ) -> (),
    );
    botlib_import.EntityTrace = Some(
        BotImport_EntityTrace
            as unsafe extern "C" fn(
                _: *mut bsp_trace_t,
                _: *mut vec_t,
                _: *mut vec_t,
                _: *mut vec_t,
                _: *mut vec_t,
                _: i32,
                _: i32,
            ) -> (),
    );
    botlib_import.PointContents =
        Some(BotImport_PointContents as unsafe extern "C" fn(_: *mut vec_t) -> i32);
    botlib_import.inPVS =
        Some(BotImport_inPVS as unsafe extern "C" fn(_: *mut vec_t, _: *mut vec_t) -> i32);
    botlib_import.BSPEntityData =
        Some(BotImport_BSPEntityData as unsafe extern "C" fn() -> *mut libc::c_char);
    botlib_import.BSPModelMinsMaxsOrigin = Some(
        BotImport_BSPModelMinsMaxsOrigin
            as unsafe extern "C" fn(
                _: i32,
                _: *mut vec_t,
                _: *mut vec_t,
                _: *mut vec_t,
                _: *mut vec_t,
            ) -> (),
    );
    botlib_import.BotClientCommand =
        Some(BotClientCommand as unsafe extern "C" fn(_: i32, _: *mut libc::c_char) -> ());
    //memory management
    botlib_import.GetMemory =
        Some(BotImport_GetMemory as unsafe extern "C" fn(_: i32) -> *mut libc::c_void);
    botlib_import.FreeMemory =
        Some(BotImport_FreeMemory as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    botlib_import.AvailableMemory = Some(Z_AvailableMemory as unsafe extern "C" fn() -> i32);
    botlib_import.HunkAlloc =
        Some(BotImport_HunkAlloc as unsafe extern "C" fn(_: i32) -> *mut libc::c_void);
    // file system access
    botlib_import.FS_FOpenFile = Some(
        FS_FOpenFileByMode
            as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *mut fileHandle_t,
                _: fsMode_t,
            ) -> i32,
    );
    botlib_import.FS_Read =
        Some(FS_Read as unsafe extern "C" fn(_: *mut libc::c_void, _: i32, _: fileHandle_t) -> i32);
    botlib_import.FS_Write = Some(
        FS_Write as unsafe extern "C" fn(_: *const libc::c_void, _: i32, _: fileHandle_t) -> i32,
    );
    botlib_import.FS_FCloseFile =
        Some(FS_FCloseFile as unsafe extern "C" fn(_: fileHandle_t) -> ());
    botlib_import.FS_Seek =
        Some(FS_Seek as unsafe extern "C" fn(_: fileHandle_t, _: isize, _: i32) -> i32);
    //debug lines
    botlib_import.DebugLineCreate =
        Some(BotImport_DebugLineCreate as unsafe extern "C" fn() -> i32);
    botlib_import.DebugLineDelete =
        Some(BotImport_DebugLineDelete as unsafe extern "C" fn(_: i32) -> ());
    botlib_import.DebugLineShow = Some(
        BotImport_DebugLineShow
            as unsafe extern "C" fn(_: i32, _: *mut vec_t, _: *mut vec_t, _: i32) -> (),
    );
    //debug polygons
    botlib_import.DebugPolygonCreate = Some(
        BotImport_DebugPolygonCreate as unsafe extern "C" fn(_: i32, _: i32, _: *mut vec3_t) -> i32,
    );
    botlib_import.DebugPolygonDelete =
        Some(BotImport_DebugPolygonDelete as unsafe extern "C" fn(_: i32) -> ());
    botlib_export = GetBotLibAPI(
        2 as i32,
        &mut botlib_import as *mut _ as *mut botlib_import_s,
    ) as *mut botlib_export_s;
    // somehow we end up with a zero import.
}
//
//  * * * BOT AI CODE IS BELOW THIS POINT * * *
//
/*
==================
SV_BotGetConsoleMessage
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_BotGetConsoleMessage(
    mut client: i32,
    mut buf: *mut libc::c_char,
    mut size: i32,
) -> i32 {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut index: i32 = 0;
    cl = &mut *svs.clients.offset(client as isize) as *mut client_t;
    (*cl).lastPacketTime = svs.time;
    if (*cl).reliableAcknowledge == (*cl).reliableSequence {
        return qfalse as i32;
    }
    (*cl).reliableAcknowledge += 1;
    index = (*cl).reliableAcknowledge & 64 as i32 - 1 as i32;
    if (*cl).reliableCommands[index as usize][0 as i32 as usize] == 0 {
        return qfalse as i32;
    }
    Q_strncpyz(
        buf,
        (*cl).reliableCommands[index as usize].as_mut_ptr(),
        size,
    );
    return qtrue as i32;
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
// server.h
//=============================================================================
// !!! MUST NOT CHANGE, SERVER AND
// GAME BOTH REFERENCE !!!
// for delta compression of initial sighting
// if -1, use headnode instead
// if all the clusters don't fit in clusternums
// used to prevent double adding from portal views
// no map loaded
// spawning level entities
// actively running
// if true, send configstring changes during SS_LOADING
// changes each server start
// serverId before a map_restart
// the feed key that we use to compute the pure checksum strings
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=475
// the serverId associated with the current checksumFeed (always <= serverId)
// incremented for each snapshot built
// <= 1000 / sv_frame->value
// when time > nextFrameTime, process world
// used during game VM init
// the game virtual machine will update these on init and changes
// current number, <= MAX_GENTITIES
// will be > sizeof(playerState_t) due to game private data
// portalarea visibility bits
// into the circular sv_packet_entities[]
// the entities MUST be in increasing state number
// order, otherwise the delta compression will fail
// time the message was transmitted
// time the message was acked
// used to rate drop packets
// can be reused for a new connection
// client has been disconnected, but don't reuse
// connection for a couple seconds
// has been assigned to a client_t, but no gamestate yet
// gamestate has been sent, but client hasn't sent a usercmd
// client is fully in game
// valid command string for SV_Netchan_Encode
// name, etc
// last added reliable message, not necessarily sent or acknowledged yet
// last acknowledged reliable message
// last sent reliable message, not necessarily acknowledged yet
// netchan->outgoingSequence of gamestate
// for delta compression
// reliable client message sequence
// SV_GentityNum(clientnum)
// extracted from userinfo, high bits masked
// downloading
// if not empty string, we are downloading
// file being downloaded
// total bytes (can't use EOF because of paks)
// bytes sent
// last block we sent to the client, awaiting ack
// current block number
// last block we xmited
// the buffers for the download blocks
// We have sent the EOF block
// time we last got an ack from the client
// frame last client usercmd message
// svs.time when another reliable command will be allowed
// svs.time when packet was last received
// svs.time when connection started
// svs.time of last sent snapshot
// true if nextSnapshotTime was set based on rate instead of snapshotMsec
// must timeout a few frames in a row so debugging doesn't break
// updates can be delta'd from here
// bytes / second
// requests a snapshot every snapshotMsec unless rate choked
// TTimo - additional flag to distinguish between a bad pure checksum, and no cp command at all
// TTimo
// queuing outgoing fragmented messages to send them properly, without udp packet bursts
// in case large fragmented messages are stacking up
// buffer them into this queue, and hand them out to netchan as needed
//=============================================================================
// MAX_CHALLENGES is made large to prevent a denial
// of service attack that could cycle all of them
// out before legitimate users connected
// Allow a certain amount of challenges to have the same IP address
// to make it a bit harder to DOS one single IP address from connecting
// while not allowing a single ip to grab all challenge resources
// challenge number coming from the client
// time the last packet was sent to the autherize server
// time the challenge response was sent to client
// time the adr was first used, for authorize timeout checks
// this structure will be cleared only when the game dll changes
// sv_init has completed
// will be strictly increasing across level changes
// ^= SNAPFLAG_SERVERCOUNT every SV_SpawnServer()
// [sv_maxclients->integer];
// sv_maxclients->integer*PACKET_BACKUP*MAX_SNAPSHOT_ENTITIES
// next snapshotEntities to use
// [numSnapshotEntities]
// to prevent invalid IPs from connecting
// for rcon return messages
// authorize server address
// next svs.time that server should do dns lookup for master server
// Structure for managing bans
// For a CIDR-Notation type suffix
//=============================================================================
// persistant server info across maps
// cleared each map
// game virtual machine
//===========================================================
//
// sv_main.c
//
//
// sv_init.c
//
//
// sv_client.c
//
//
// sv_ccmds.c
//
//
// sv_snapshot.c
//
//
// sv_game.c
//
//
// sv_bot.c
//
/*
==================
SV_BotGetSnapshotEntity
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_BotGetSnapshotEntity(mut client: i32, mut sequence: i32) -> i32 {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut frame: *mut clientSnapshot_t = 0 as *mut clientSnapshot_t;
    cl = &mut *svs.clients.offset(client as isize) as *mut client_t;
    frame = &mut *(*cl)
        .frames
        .as_mut_ptr()
        .offset(((*cl).netchan.outgoingSequence & 32 as i32 - 1 as i32) as isize)
        as *mut clientSnapshot_t;
    if sequence < 0 as i32 || sequence >= (*frame).num_entities {
        return -(1 as i32);
    }
    return (*svs
        .snapshotEntities
        .offset((((*frame).first_entity + sequence) % svs.numSnapshotEntities) as isize))
    .number;
}

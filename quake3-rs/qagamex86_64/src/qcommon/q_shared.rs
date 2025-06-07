pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const qfalse: crate::src::qcommon::q_shared::qboolean = 0;
pub const qtrue: crate::src::qcommon::q_shared::qboolean = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub union floatint_t {
    pub f: libc::c_float,
    pub i: libc::c_int,
    pub ui: libc::c_uint,
}
pub type fileHandle_t = libc::c_int;
pub const EXEC_NOW: crate::bg_public_h::C2RustUnnamed_0 = 0;
// don't return until completed, a VM should NEVER use this,

// because some commands might cause the VM to be unloaded...
pub const EXEC_INSERT: crate::bg_public_h::C2RustUnnamed_0 = 1;
// add to end of the command buffer (normal case)

// insert at current position, but don't run yet
pub const EXEC_APPEND: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const ERR_FATAL: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const ERR_DROP: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const ERR_SERVERDISCONNECT: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const ERR_DISCONNECT: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const ERR_NEED_CD: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub type vec_t = libc::c_float;
pub type vec3_t = [crate::src::qcommon::q_shared::vec_t; 3];
pub type vec4_t = [crate::src::qcommon::q_shared::vec_t; 4];
pub type pc_token_t = crate::src::qcommon::q_shared::pc_token_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pc_token_s {
    pub type_0: libc::c_int,
    pub subtype: libc::c_int,
    pub intvalue: libc::c_int,
    pub floatvalue: libc::c_float,
    pub string: [libc::c_char; 1024],
}
pub type fsMode_t = libc::c_uint;
pub const FS_READ: crate::src::qcommon::q_shared::fsMode_t = 0;
pub const FS_WRITE: crate::src::qcommon::q_shared::fsMode_t = 1;
pub const FS_APPEND: crate::src::qcommon::q_shared::fsMode_t = 2;
pub const FS_APPEND_SYNC: crate::src::qcommon::q_shared::fsMode_t = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qint64 {
    pub b0: crate::src::qcommon::q_shared::byte,
    pub b1: crate::src::qcommon::q_shared::byte,
    pub b2: crate::src::qcommon::q_shared::byte,
    pub b3: crate::src::qcommon::q_shared::byte,
    pub b4: crate::src::qcommon::q_shared::byte,
    pub b5: crate::src::qcommon::q_shared::byte,
    pub b6: crate::src::qcommon::q_shared::byte,
    pub b7: crate::src::qcommon::q_shared::byte,
}
pub type cvarHandle_t = libc::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmCvar_t {
    pub handle: crate::src::qcommon::q_shared::cvarHandle_t,
    pub modificationCount: libc::c_int,
    pub value: libc::c_float,
    pub integer: libc::c_int,
    pub string: [libc::c_char; 256],
}
pub type cplane_t = crate::src::qcommon::q_shared::cplane_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cplane_s {
    pub normal: crate::src::qcommon::q_shared::vec3_t,
    pub dist: libc::c_float,
    pub type_0: crate::src::qcommon::q_shared::byte,
    pub signbits: crate::src::qcommon::q_shared::byte,
    pub pad: [crate::src::qcommon::q_shared::byte; 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_t {
    pub allsolid: crate::src::qcommon::q_shared::qboolean,
    pub startsolid: crate::src::qcommon::q_shared::qboolean,
    pub fraction: libc::c_float,
    pub endpos: crate::src::qcommon::q_shared::vec3_t,
    pub plane: crate::src::qcommon::q_shared::cplane_t,
    pub surfaceFlags: libc::c_int,
    pub contents: libc::c_int,
    pub entityNum: libc::c_int,
}
pub const CHAN_AUTO: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const CHAN_LOCAL: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const CHAN_WEAPON: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const CHAN_VOICE: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const CHAN_ITEM: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const CHAN_BODY: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const CHAN_LOCAL_SOUND: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const CHAN_ANNOUNCER: crate::bg_public_h::C2RustUnnamed_0 = 7;
pub type playerState_t = crate::src::qcommon::q_shared::playerState_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct playerState_s {
    pub commandTime: libc::c_int,
    pub pm_type: libc::c_int,
    pub bobCycle: libc::c_int,
    pub pm_flags: libc::c_int,
    pub pm_time: libc::c_int,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub velocity: crate::src::qcommon::q_shared::vec3_t,
    pub weaponTime: libc::c_int,
    pub gravity: libc::c_int,
    pub speed: libc::c_int,
    pub delta_angles: [libc::c_int; 3],
    pub groundEntityNum: libc::c_int,
    pub legsTimer: libc::c_int,
    pub legsAnim: libc::c_int,
    pub torsoTimer: libc::c_int,
    pub torsoAnim: libc::c_int,
    pub movementDir: libc::c_int,
    pub grapplePoint: crate::src::qcommon::q_shared::vec3_t,
    pub eFlags: libc::c_int,
    pub eventSequence: libc::c_int,
    pub events: [libc::c_int; 2],
    pub eventParms: [libc::c_int; 2],
    pub externalEvent: libc::c_int,
    pub externalEventParm: libc::c_int,
    pub externalEventTime: libc::c_int,
    pub clientNum: libc::c_int,
    pub weapon: libc::c_int,
    pub weaponstate: libc::c_int,
    pub viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub viewheight: libc::c_int,
    pub damageEvent: libc::c_int,
    pub damageYaw: libc::c_int,
    pub damagePitch: libc::c_int,
    pub damageCount: libc::c_int,
    pub stats: [libc::c_int; 16],
    pub persistant: [libc::c_int; 16],
    pub powerups: [libc::c_int; 16],
    pub ammo: [libc::c_int; 16],
    pub generic1: libc::c_int,
    pub loopSound: libc::c_int,
    pub jumppad_ent: libc::c_int,
    pub ping: libc::c_int,
    pub pmove_framecount: libc::c_int,
    pub jumppad_frame: libc::c_int,
    pub entityEventSequence: libc::c_int,
}
pub type usercmd_t = crate::src::qcommon::q_shared::usercmd_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usercmd_s {
    pub serverTime: libc::c_int,
    pub angles: [libc::c_int; 3],
    pub buttons: libc::c_int,
    pub weapon: crate::src::qcommon::q_shared::byte,
    pub forwardmove: libc::c_schar,
    pub rightmove: libc::c_schar,
    pub upmove: libc::c_schar,
}
pub type trType_t = libc::c_uint;
pub const TR_STATIONARY: crate::src::qcommon::q_shared::trType_t = 0;
pub const TR_INTERPOLATE: crate::src::qcommon::q_shared::trType_t = 1;
pub const TR_LINEAR: crate::src::qcommon::q_shared::trType_t = 2;
pub const TR_LINEAR_STOP: crate::src::qcommon::q_shared::trType_t = 3;
pub const TR_SINE: crate::src::qcommon::q_shared::trType_t = 4;
pub const TR_GRAVITY: crate::src::qcommon::q_shared::trType_t = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trajectory_t {
    pub trType: crate::src::qcommon::q_shared::trType_t,
    pub trTime: libc::c_int,
    pub trDuration: libc::c_int,
    pub trBase: crate::src::qcommon::q_shared::vec3_t,
    pub trDelta: crate::src::qcommon::q_shared::vec3_t,
}
pub type entityState_t = crate::src::qcommon::q_shared::entityState_s;
// these are sent over the net as 8 bits

// so they cannot be blindly increased

// these are the only configstrings that the system reserves, all the

// other ones are strictly for servergame to clientgame communication

// an info string with all the serverinfo cvars

// an info string for server system to client system configuration (timescale, etc)

// game can't modify below this, only the system can

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

// cmd->serverTime of last executed command

// for view bobbing and footstep generation

// ducked, jump_held, etc

// add to command angles to get view direction

// changed by spawns, rotating objects, and teleporters

// ENTITYNUM_NONE = in air

// don't change low priority animations until this runs out

// mask off ANIM_TOGGLEBIT

// don't change low priority animations until this runs out

// mask off ANIM_TOGGLEBIT

// a number 0 to 7 that represents the relative angle

// of movement to the view angle (axial and diagonals)

// when at rest, the value will remain unchanged

// used to twist the legs during strafing

// location of grapple to pull towards if PMF_GRAPPLE_PULL

// copied to entityState_t->eFlags

// pmove generated events

// events set on player from another source

// ranges from 0 to MAX_CLIENTS-1

// copied to entityState_t->weapon

// for fixed views

// damage feedback

// when it changes, latch the other parms

// stats that aren't cleared on death

// level.time that the powerup runs out

// jumppad entity hit this frame

// not communicated over the net at all

// server to game info for scoreboard

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

// weapon

//===================================================================

// if entityState->solid == SOLID_BMODEL, modelindex is an inline model number

// non-parametric, but interpolate between snapshots

// value = base + sin( time / duration ) * delta

// if non 0, trTime + trDuration = stop time

// velocity, etc

// entityState_t is the information conveyed from the server

// in an update message about entities that the client will

// need to render in some way

// Different eTypes may use the information in different ways

// The messages are delta compressed, so it doesn't really matter if

// the structure size is fairly large
#[repr(C)]
#[derive(Copy, Clone)]
pub struct entityState_s {
    pub number: libc::c_int,
    pub eType: libc::c_int,
    pub eFlags: libc::c_int,
    pub pos: crate::src::qcommon::q_shared::trajectory_t,
    pub apos: crate::src::qcommon::q_shared::trajectory_t,
    pub time: libc::c_int,
    pub time2: libc::c_int,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub origin2: crate::src::qcommon::q_shared::vec3_t,
    pub angles: crate::src::qcommon::q_shared::vec3_t,
    pub angles2: crate::src::qcommon::q_shared::vec3_t,
    pub otherEntityNum: libc::c_int,
    pub otherEntityNum2: libc::c_int,
    pub groundEntityNum: libc::c_int,
    pub constantLight: libc::c_int,
    pub loopSound: libc::c_int,
    pub modelindex: libc::c_int,
    pub modelindex2: libc::c_int,
    pub clientNum: libc::c_int,
    pub frame: libc::c_int,
    pub solid: libc::c_int,
    pub event: libc::c_int,
    pub eventParm: libc::c_int,
    pub powerups: libc::c_int,
    pub weapon: libc::c_int,
    pub legsAnim: libc::c_int,
    pub torsoAnim: libc::c_int,
    pub generic1: libc::c_int,
}
pub type qtime_t = crate::src::qcommon::q_shared::qtime_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtime_s {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
}
pub type flagStatus_t = crate::src::qcommon::q_shared::_flag_status;
pub type _flag_status = libc::c_uint;
pub const FLAG_ATBASE: crate::src::qcommon::q_shared::_flag_status = 0;
pub const FLAG_TAKEN: crate::src::qcommon::q_shared::_flag_status = 1;
pub const FLAG_TAKEN_RED: crate::src::qcommon::q_shared::_flag_status = 2;
pub const FLAG_TAKEN_BLUE: crate::src::qcommon::q_shared::_flag_status = 3;
pub const FLAG_DROPPED: crate::src::qcommon::q_shared::_flag_status = 4;
// check think function

// if it is in a nodrop volume, remove it

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

// __Q_SHARED_H

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

// bg_public.h -- definitions shared by both the server game and client game modules

// because games can change separately from the main system version, we need a

// second version that must match between game and cgame

// item sizes are needed for client side pickup detection

// for the CS_SCORES[12] when only one player is present

// 30 seconds before vote times out

//

// config strings are a general means of communicating variable length strings

// from the server to all connected clients.

//

// CS_SERVERINFO and CS_SYSTEMINFO are defined in q_shared.h

// from the map worldspawn's message field

// g_motd string for server message of the day

// server time when the match will be restarted

// so the timer only shows the current level

// when 1, fraglimit/timelimit has been hit and intermission will start in a second or two

// string indicating flag status in CTF

// string of 0's and 1's that tell which items are present

// capture the flag

// team deathmatch

// single player ffa

//-- team games go after this --

// one on one tournament

// free for all

// flip the togglebit every time an animation

// changes so a restart of the same anim can be detected

//---------------------------------------------------------

// gitem_t->type

// single use, holdable item

// EFX: rotate + bob

// instant on, timer based

// EFX: rotate + external ring that rotates

// EFX: static external sphere + rotating internal

// EFX: rotate + minlight

// EFX: rotate

// EFX: rotate + upscale + minlight

// string of all sounds this item will use

// g_dmflags->integer flags

// content masks

//

// entityState_t->eType

//

// any of the EV_* events can be added freestanding

// by setting eType to ET_EVENTS + eventNum

// this avoids having to set eFlags and eventNum

// grapple hooked on wall

// maximum radius of the models during the effect

// radius of the models without scaling

// 2nd shockwave times

// explosion/implosion times

// 1st shockwave times

// Kamikaze

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

// g_public.h -- game module information visible to server

// entity->svFlags

// the server does not know how to interpret most of the values

// in entityStates (level eType), so the game must explicitly flag

// special server behaviors

// don't send entity to clients, even if it has effects

// TTimo

// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=551

// set if the entity is a bot

// send to all connected clients

// merge a second pvs at origin2 into snapshots

// entity->r.currentOrigin instead of entity->s.origin

// for link position (missiles and movers)

// only send to a single client (entityShared_t->singleClient)

// don't send CS_SERVERINFO updates to this client

// so that it can be updated for ping tools without

// lagging clients

// use capsule for collision detection instead of bbox

// send entity to everyone but one client

// (entityShared_t->singleClient)

//===============================================================

//

// functions exported by the game subsystem

//

// ( int time );

// ( void );

// ConsoleCommand will be called when a command has been issued

// that is not recognized as a builtin function.

// The game can issue trap_argc() / trap_argv() commands to get the command

// and parameters.  Return qfalse if the game doesn't recognize it as a command.

// ( int levelTime );

// ( int clientNum );

// ( int clientNum );

// ( int clientNum );

// ( int clientNum );

// ( int clientNum );

// ( int clientNum, qboolean firstTime, qboolean isBot );

// return NULL if the client is allowed to connect, otherwise return

// a text string with the reason for denial

// (void);

// ( int levelTime, int randomSeed, int restart );

// init and shutdown will be called every single level

// The game should call G_GET_ENTITY_TOKEN to parse through all the

// entity configuration text and spawn gentities.

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

// Initialized in run_static_initializers

/*
================
vmMain

This is the only way control passes into the module.
This must be the very first function compiled into the .q3vm file
================
*/

/*
================
G_FindTeams

Chain together all entities with a matching team field.
Entity teams are used for item groups and multi-entity mover groups.

All but the first will have the FL_TEAMSLAVE flag set and teammaster field set
All but the last will have the teamchain field set to the next one
================
*/

// make sure that targets only point at the master

/*
=================
G_RegisterCvars
=================
*/

// check some things

/*
=================
G_UpdateCvars
=================
*/

/*
============
G_InitGame

============
*/

// set some level globals
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

    #[inline]

    pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}
pub mod stdlib_float_h {

    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
        return ::libc::strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    }
}
pub use crate::bg_public_h::C2RustUnnamed_0;
pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::src::game::g_main::Com_Error;
pub use crate::src::game::g_main::Com_Printf;
pub use crate::src::qcommon::q_shared::ctype_h::tolower;
pub use crate::src::qcommon::q_shared::ctype_h::toupper;
pub use crate::src::qcommon::q_shared::stdlib_float_h::atof;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_ISalnum;
pub use crate::stdlib::_ISalpha;
pub use crate::stdlib::_ISblank;
pub use crate::stdlib::_IScntrl;
pub use crate::stdlib::_ISdigit;
pub use crate::stdlib::_ISgraph;
pub use crate::stdlib::_ISlower;
pub use crate::stdlib::_ISprint;
pub use crate::stdlib::_ISpunct;
pub use crate::stdlib::_ISspace;
pub use crate::stdlib::_ISupper;
pub use crate::stdlib::_ISxdigit;
pub use crate::stdlib::__ctype_b_loc;
pub use crate::stdlib::__ctype_tolower_loc;
pub use crate::stdlib::__ctype_toupper_loc;
pub use crate::stdlib::__int32_t;

#[no_mangle]
pub unsafe extern "C" fn Q_IsColorString(
    mut p: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    if p.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int != '^' as i32 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if *p.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // isalnum expects a signed integer in the range -1 (EOF) to 255, or it might assert on undefined behaviour
    // a dereferenced char pointer has the range -128 to 127, so we just need to rangecheck the negative part
    if (*p.offset(1 as libc::c_int as isize) as libc::c_int) < 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if *(*crate::stdlib::__ctype_b_loc())
        .offset(*p.offset(1 as libc::c_int as isize) as libc::c_int as isize) as libc::c_int
        & crate::stdlib::_ISalnum as libc::c_int as libc::c_ushort as libc::c_int
        == 0 as libc::c_int
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn Com_Clamp(
    mut min: libc::c_float,
    mut max: libc::c_float,
    mut value: libc::c_float,
) -> libc::c_float {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn COM_SkipPath(mut pathname: *mut libc::c_char) -> *mut libc::c_char {
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    last = pathname;
    while *pathname != 0 {
        if *pathname as libc::c_int == '/' as i32 {
            last = pathname.offset(1 as libc::c_int as isize)
        }
        pathname = pathname.offset(1)
    }
    return last;
}
#[no_mangle]
pub unsafe extern "C" fn COM_GetExtension(mut name: *const libc::c_char) -> *const libc::c_char {
    let mut dot: *const libc::c_char = ::libc::strrchr(name, '.' as i32);
    let mut slash: *const libc::c_char = 0 as *const libc::c_char;
    if !dot.is_null() && {
        slash = ::libc::strrchr(name, '/' as i32);
        (slash.is_null()) || slash < dot
    } {
        return dot.offset(1 as libc::c_int as isize);
    } else {
        return b"\x00" as *const u8 as *const libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn COM_StripExtension(
    mut in_0: *const libc::c_char,
    mut out: *mut libc::c_char,
    mut destsize: libc::c_int,
) {
    let mut dot: *const libc::c_char = ::libc::strrchr(in_0, '.' as i32);
    let mut slash: *const libc::c_char = 0 as *const libc::c_char;
    if !dot.is_null() && {
        slash = ::libc::strrchr(in_0, '/' as i32);
        (slash.is_null()) || slash < dot
    } {
        destsize = if (destsize as libc::c_long)
            < dot.wrapping_offset_from(in_0) as libc::c_long + 1 as libc::c_int as libc::c_long
        {
            destsize as libc::c_long
        } else {
            (dot.wrapping_offset_from(in_0) as libc::c_long) + 1 as libc::c_int as libc::c_long
        } as libc::c_int
    }
    if in_0 == out as *const libc::c_char && destsize > 1 as libc::c_int {
        *out.offset((destsize - 1 as libc::c_int) as isize) = '\u{0}' as i32 as libc::c_char
    } else {
        Q_strncpyz(out, in_0, destsize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn COM_CompareExtension(
    mut in_0: *const libc::c_char,
    mut ext: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut inlen: libc::c_int = 0;
    let mut extlen: libc::c_int = 0;
    inlen = crate::stdlib::strlen(in_0) as libc::c_int;
    extlen = crate::stdlib::strlen(ext) as libc::c_int;
    if extlen <= inlen {
        in_0 = in_0.offset((inlen - extlen) as isize);
        if Q_stricmp(in_0, ext) == 0 {
            return crate::src::qcommon::q_shared::qtrue;
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn COM_DefaultExtension(
    mut path: *mut libc::c_char,
    mut maxSize: libc::c_int,
    mut extension: *const libc::c_char,
) {
    let mut dot: *const libc::c_char = ::libc::strrchr(path, '.' as i32);
    let mut slash: *const libc::c_char = 0 as *const libc::c_char;
    if !dot.is_null() && {
        slash = ::libc::strrchr(path, '/' as i32);
        (slash.is_null()) || slash < dot
    } {
        return;
    } else {
        Q_strcat(path, maxSize, extension);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CopyShortSwap(mut dest: *mut libc::c_void, mut src: *mut libc::c_void) {
    let mut to: *mut crate::src::qcommon::q_shared::byte =
        dest as *mut crate::src::qcommon::q_shared::byte;
    let mut from: *mut crate::src::qcommon::q_shared::byte =
        src as *mut crate::src::qcommon::q_shared::byte;
    *to.offset(0 as libc::c_int as isize) = *from.offset(1 as libc::c_int as isize);
    *to.offset(1 as libc::c_int as isize) = *from.offset(0 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CopyLongSwap(mut dest: *mut libc::c_void, mut src: *mut libc::c_void) {
    let mut to: *mut crate::src::qcommon::q_shared::byte =
        dest as *mut crate::src::qcommon::q_shared::byte;
    let mut from: *mut crate::src::qcommon::q_shared::byte =
        src as *mut crate::src::qcommon::q_shared::byte;
    *to.offset(0 as libc::c_int as isize) = *from.offset(3 as libc::c_int as isize);
    *to.offset(1 as libc::c_int as isize) = *from.offset(2 as libc::c_int as isize);
    *to.offset(2 as libc::c_int as isize) = *from.offset(1 as libc::c_int as isize);
    *to.offset(3 as libc::c_int as isize) = *from.offset(0 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ShortSwap(mut l: libc::c_short) -> libc::c_short {
    let mut b1: crate::src::qcommon::q_shared::byte = 0;
    let mut b2: crate::src::qcommon::q_shared::byte = 0;
    b1 = (l as libc::c_int & 255 as libc::c_int) as crate::src::qcommon::q_shared::byte;
    b2 = (l as libc::c_int >> 8 as libc::c_int & 255 as libc::c_int)
        as crate::src::qcommon::q_shared::byte;
    return (((b1 as libc::c_int) << 8 as libc::c_int) + b2 as libc::c_int) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ShortNoSwap(mut l: libc::c_short) -> libc::c_short {
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn LongSwap(mut l: libc::c_int) -> libc::c_int {
    let mut b1: crate::src::qcommon::q_shared::byte = 0;
    let mut b2: crate::src::qcommon::q_shared::byte = 0;
    let mut b3: crate::src::qcommon::q_shared::byte = 0;
    let mut b4: crate::src::qcommon::q_shared::byte = 0;
    b1 = (l & 255 as libc::c_int) as crate::src::qcommon::q_shared::byte;
    b2 = (l >> 8 as libc::c_int & 255 as libc::c_int) as crate::src::qcommon::q_shared::byte;
    b3 = (l >> 16 as libc::c_int & 255 as libc::c_int) as crate::src::qcommon::q_shared::byte;
    b4 = (l >> 24 as libc::c_int & 255 as libc::c_int) as crate::src::qcommon::q_shared::byte;
    return ((b1 as libc::c_int) << 24 as libc::c_int)
        + ((b2 as libc::c_int) << 16 as libc::c_int)
        + ((b3 as libc::c_int) << 8 as libc::c_int)
        + b4 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LongNoSwap(mut l: libc::c_int) -> libc::c_int {
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn Long64Swap(
    mut ll: crate::src::qcommon::q_shared::qint64,
) -> crate::src::qcommon::q_shared::qint64 {
    let mut result: crate::src::qcommon::q_shared::qint64 = crate::src::qcommon::q_shared::qint64 {
        b0: 0,
        b1: 0,
        b2: 0,
        b3: 0,
        b4: 0,
        b5: 0,
        b6: 0,
        b7: 0,
    };
    result.b0 = ll.b7;
    result.b1 = ll.b6;
    result.b2 = ll.b5;
    result.b3 = ll.b4;
    result.b4 = ll.b3;
    result.b5 = ll.b2;
    result.b6 = ll.b1;
    result.b7 = ll.b0;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Long64NoSwap(
    mut ll: crate::src::qcommon::q_shared::qint64,
) -> crate::src::qcommon::q_shared::qint64 {
    return ll;
}
#[no_mangle]
pub unsafe extern "C" fn FloatSwap(mut f: *const libc::c_float) -> libc::c_float {
    let mut out: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    out.f = *f;
    out.ui = LongSwap(out.ui as libc::c_int) as libc::c_uint;
    return out.f;
}
#[no_mangle]
pub unsafe extern "C" fn FloatNoSwap(mut f: *const libc::c_float) -> libc::c_float {
    return *f;
}
static mut com_token: [libc::c_char; 1024] = [0; 1024];
static mut com_parsename: [libc::c_char; 1024] = [0; 1024];
static mut com_lines: libc::c_int = 0;
static mut com_tokenline: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn COM_BeginParseSession(mut name: *const libc::c_char) {
    com_lines = 1 as libc::c_int;
    com_tokenline = 0 as libc::c_int;
    Com_sprintf(
        com_parsename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        b"%s\x00" as *const u8 as *const libc::c_char,
        name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn COM_GetCurrentParseLine() -> libc::c_int {
    if com_tokenline != 0 {
        return com_tokenline;
    }
    return com_lines;
}
#[no_mangle]
pub unsafe extern "C" fn COM_Parse(mut data_p: *mut *mut libc::c_char) -> *mut libc::c_char {
    return COM_ParseExt(data_p, crate::src::qcommon::q_shared::qtrue);
}
#[no_mangle]
pub unsafe extern "C" fn COM_ParseError(mut format: *mut libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut string: [libc::c_char; 4096] = [0; 4096];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        format,
        argptr.as_va_list(),
    );
    crate::src::game::g_main::Com_Printf(
        b"ERROR: %s, line %d: %s\n\x00" as *const u8 as *const libc::c_char,
        com_parsename.as_mut_ptr(),
        COM_GetCurrentParseLine(),
        string.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn COM_ParseWarning(mut format: *mut libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut string: [libc::c_char; 4096] = [0; 4096];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        format,
        argptr.as_va_list(),
    );
    crate::src::game::g_main::Com_Printf(
        b"WARNING: %s, line %d: %s\n\x00" as *const u8 as *const libc::c_char,
        com_parsename.as_mut_ptr(),
        COM_GetCurrentParseLine(),
        string.as_mut_ptr(),
    );
}
unsafe extern "C" fn SkipWhitespace(
    mut data: *mut libc::c_char,
    mut hasNewLines: *mut crate::src::qcommon::q_shared::qboolean,
) -> *mut libc::c_char {
    let mut c: libc::c_int = 0;
    loop {
        c = *data as libc::c_int;
        if !(c <= ' ' as i32) {
            break;
        }
        if c == 0 {
            return 0 as *mut libc::c_char;
        }
        if c == '\n' as i32 {
            com_lines += 1;
            *hasNewLines = crate::src::qcommon::q_shared::qtrue
        }
        data = data.offset(1)
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn COM_Compress(mut data_p: *mut libc::c_char) -> libc::c_int {
    let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut newline: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut whitespace: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    out = data_p;
    in_0 = out;
    if !in_0.is_null() {
        loop {
            c = *in_0 as libc::c_int;
            if !(c != 0 as libc::c_int) {
                break;
            }
            // skip double slash comments
            if c == '/' as i32
                && *in_0.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                while *in_0 as libc::c_int != 0 && *in_0 as libc::c_int != '\n' as i32 {
                    in_0 = in_0.offset(1)
                }
            // skip /* */ comments
            } else if c == '/' as i32
                && *in_0.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32
            {
                while *in_0 as libc::c_int != 0
                    && (*in_0 as libc::c_int != '*' as i32
                        || *in_0.offset(1 as libc::c_int as isize) as libc::c_int != '/' as i32)
                {
                    in_0 = in_0.offset(1)
                }
                if *in_0 != 0 {
                    in_0 = in_0.offset(2 as libc::c_int as isize)
                }
            // record when we hit a newline
            } else if c == '\n' as i32 || c == '\r' as i32 {
                newline = crate::src::qcommon::q_shared::qtrue;
                in_0 = in_0.offset(1)
            // record when we hit whitespace
            } else if c == ' ' as i32 || c == '\t' as i32 {
                whitespace = crate::src::qcommon::q_shared::qtrue;
                in_0 = in_0.offset(1)
            // an actual token
            } else {
                // if we have a pending newline, emit it (and it counts as whitespace)
                if newline as u64 != 0 {
                    let fresh0 = out;
                    out = out.offset(1);
                    *fresh0 = '\n' as i32 as libc::c_char;
                    newline = crate::src::qcommon::q_shared::qfalse;
                    whitespace = crate::src::qcommon::q_shared::qfalse
                }
                if whitespace as u64 != 0 {
                    let fresh1 = out;
                    out = out.offset(1);
                    *fresh1 = ' ' as i32 as libc::c_char;
                    whitespace = crate::src::qcommon::q_shared::qfalse
                }
                // copy quoted strings unmolested
                if c == '\"' as i32 {
                    let fresh2 = out;
                    out = out.offset(1);
                    *fresh2 = c as libc::c_char;
                    in_0 = in_0.offset(1);
                    loop {
                        c = *in_0 as libc::c_int;
                        if !(c != 0 && c != '\"' as i32) {
                            break;
                        }
                        let fresh3 = out;
                        out = out.offset(1);
                        *fresh3 = c as libc::c_char;
                        in_0 = in_0.offset(1)
                    }
                    if c == '\"' as i32 {
                        let fresh4 = out;
                        out = out.offset(1);
                        *fresh4 = c as libc::c_char;
                        in_0 = in_0.offset(1)
                    }
                } else {
                    *out = c as libc::c_char;
                    out = out.offset(1);
                    in_0 = in_0.offset(1)
                }
            }
        }
        *out = 0 as libc::c_int as libc::c_char
    }
    return out.wrapping_offset_from(data_p) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn COM_ParseExt(
    mut data_p: *mut *mut libc::c_char,
    mut allowLineBreaks: crate::src::qcommon::q_shared::qboolean,
) -> *mut libc::c_char {
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut hasNewLines: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    data = *data_p;
    len = 0 as libc::c_int;
    com_token[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    com_tokenline = 0 as libc::c_int;
    // make sure incoming data is valid
    if data.is_null() {
        *data_p = 0 as *mut libc::c_char;
        return com_token.as_mut_ptr();
    }
    loop {
        // skip whitespace
        data = SkipWhitespace(data, &mut hasNewLines);
        if data.is_null() {
            *data_p = 0 as *mut libc::c_char;
            return com_token.as_mut_ptr();
        }
        if hasNewLines as libc::c_uint != 0 && allowLineBreaks as u64 == 0 {
            *data_p = data;
            return com_token.as_mut_ptr();
        }
        c = *data as libc::c_int;
        // skip double slash comments
        if c == '/' as i32 && *data.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            data = data.offset(2 as libc::c_int as isize);
            while *data as libc::c_int != 0 && *data as libc::c_int != '\n' as i32 {
                data = data.offset(1)
            }
        } else {
            // skip /* */ comments
            if !(c == '/' as i32
                && *data.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32)
            {
                break;
            }
            data = data.offset(2 as libc::c_int as isize);
            while *data as libc::c_int != 0
                && (*data as libc::c_int != '*' as i32
                    || *data.offset(1 as libc::c_int as isize) as libc::c_int != '/' as i32)
            {
                if *data as libc::c_int == '\n' as i32 {
                    com_lines += 1
                }
                data = data.offset(1)
            }
            if *data != 0 {
                data = data.offset(2 as libc::c_int as isize)
            }
        }
    }
    // token starts on this line
    com_tokenline = com_lines;
    // handle quoted strings
    if c == '\"' as i32 {
        data = data.offset(1);
        loop {
            let fresh5 = data;
            data = data.offset(1);
            c = *fresh5 as libc::c_int;
            if c == '\"' as i32 || c == 0 {
                com_token[len as usize] = 0 as libc::c_int as libc::c_char;
                *data_p = data;
                return com_token.as_mut_ptr();
            }
            if c == '\n' as i32 {
                com_lines += 1
            }
            if len < 1024 as libc::c_int - 1 as libc::c_int {
                com_token[len as usize] = c as libc::c_char;
                len += 1
            }
        }
    }
    loop
    // parse a regular word
    {
        if len < 1024 as libc::c_int - 1 as libc::c_int {
            com_token[len as usize] = c as libc::c_char;
            len += 1
        }
        data = data.offset(1);
        c = *data as libc::c_int;
        if !(c > 32 as libc::c_int) {
            break;
        }
    }
    com_token[len as usize] = 0 as libc::c_int as libc::c_char;
    *data_p = data;
    return com_token.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn COM_MatchToken(
    mut buf_p: *mut *mut libc::c_char,
    mut match_0: *mut libc::c_char,
) {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    token = COM_Parse(buf_p);
    if ::libc::strcmp(token, match_0) != 0 {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MatchToken: %s != %s\x00" as *const u8 as *const libc::c_char,
            token,
            match_0,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn SkipBracedSection(
    mut program: *mut *mut libc::c_char,
    mut depth: libc::c_int,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        token = COM_ParseExt(program, crate::src::qcommon::q_shared::qtrue);
        if *token.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            if *token.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32 {
                depth += 1
            } else if *token.offset(0 as libc::c_int as isize) as libc::c_int == '}' as i32 {
                depth -= 1
            }
        }
        if !(depth != 0 && !(*program).is_null()) {
            break;
        }
    }
    return (depth == 0 as libc::c_int) as libc::c_int as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn SkipRestOfLine(mut data: *mut *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    p = *data;
    if *p == 0 {
        return;
    }
    loop {
        let fresh6 = p;
        p = p.offset(1);
        c = *fresh6 as libc::c_int;
        if !(c != 0 as libc::c_int) {
            break;
        }
        if !(c == '\n' as i32) {
            continue;
        }
        com_lines += 1;
        break;
    }
    *data = p;
}
#[no_mangle]
pub unsafe extern "C" fn Parse1DMatrix(
    mut buf_p: *mut *mut libc::c_char,
    mut x: libc::c_int,
    mut m: *mut libc::c_float,
) {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    COM_MatchToken(
        buf_p,
        b"(\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < x {
        token = COM_Parse(buf_p);
        *m.offset(i as isize) = atof(token) as libc::c_float;
        i += 1
    }
    COM_MatchToken(
        buf_p,
        b")\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Parse2DMatrix(
    mut buf_p: *mut *mut libc::c_char,
    mut y: libc::c_int,
    mut x: libc::c_int,
    mut m: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    COM_MatchToken(
        buf_p,
        b"(\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < y {
        Parse1DMatrix(buf_p, x, m.offset((i * x) as isize));
        i += 1
    }
    COM_MatchToken(
        buf_p,
        b")\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Parse3DMatrix(
    mut buf_p: *mut *mut libc::c_char,
    mut z: libc::c_int,
    mut y: libc::c_int,
    mut x: libc::c_int,
    mut m: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    COM_MatchToken(
        buf_p,
        b"(\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < z {
        Parse2DMatrix(buf_p, y, x, m.offset((i * x * y) as isize));
        i += 1
    }
    COM_MatchToken(
        buf_p,
        b")\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Com_HexStrToInt(mut str: *const libc::c_char) -> libc::c_int {
    if str.is_null() {
        return -(1 as libc::c_int);
    }
    // check for hex code
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        && *str.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
        && *str.offset(2 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
    {
        let mut i: libc::c_int = 0;
        let mut n: libc::c_int = 0 as libc::c_int;
        let mut len: libc::c_int = crate::stdlib::strlen(str) as libc::c_int;
        i = 2 as libc::c_int;
        while i < len {
            let mut digit: libc::c_char = 0;
            n *= 16 as libc::c_int;
            digit = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *str.offset(i as isize) as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = tolower(*str.offset(i as isize) as libc::c_int)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_tolower_loc())
                        .offset(*str.offset(i as isize) as libc::c_int as isize)
                }
                __res
            }) as libc::c_char;
            if digit as libc::c_int >= '0' as i32 && digit as libc::c_int <= '9' as i32 {
                digit = (digit as libc::c_int - '0' as i32) as libc::c_char
            } else if digit as libc::c_int >= 'a' as i32 && digit as libc::c_int <= 'f' as i32 {
                digit = (digit as libc::c_int - 'a' as i32 + 10 as libc::c_int) as libc::c_char
            } else {
                return -(1 as libc::c_int);
            }
            n += digit as libc::c_int;
            i += 1
        }
        return n;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Q_isprint(mut c: libc::c_int) -> libc::c_int {
    if c >= 0x20 as libc::c_int && c <= 0x7e as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Q_islower(mut c: libc::c_int) -> libc::c_int {
    if c >= 'a' as i32 && c <= 'z' as i32 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isupper(mut c: libc::c_int) -> libc::c_int {
    if c >= 'A' as i32 && c <= 'Z' as i32 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isalpha(mut c: libc::c_int) -> libc::c_int {
    if c >= 'a' as i32 && c <= 'z' as i32 || c >= 'A' as i32 && c <= 'Z' as i32 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isanumber(
    mut s: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: libc::c_double = 0.;
    if *s as libc::c_int == '\u{0}' as i32 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    d = ::libc::strtod(s, &mut p);
    return (*p as libc::c_int == '\u{0}' as i32) as libc::c_int
        as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isintegral(
    mut f: libc::c_float,
) -> crate::src::qcommon::q_shared::qboolean {
    return (f as libc::c_int as libc::c_float == f) as libc::c_int
        as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strncpyz(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut destsize: libc::c_int,
) {
    if dest.is_null() {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Q_strncpyz: NULL dest\x00" as *const u8 as *const libc::c_char,
        );
    }
    if src.is_null() {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Q_strncpyz: NULL src\x00" as *const u8 as *const libc::c_char,
        );
    }
    if destsize < 1 as libc::c_int {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Q_strncpyz: destsize < 1\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::stdlib::strncpy(dest, src, (destsize - 1 as libc::c_int) as libc::c_ulong);
    *dest.offset((destsize - 1 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Q_stricmpn(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    if s1.is_null() {
        if s2.is_null() {
            return 0 as libc::c_int;
        } else {
            return -(1 as libc::c_int);
        }
    } else {
        if s2.is_null() {
            return 1 as libc::c_int;
        }
    }
    loop {
        let fresh7 = s1;
        s1 = s1.offset(1);
        c1 = *fresh7 as libc::c_int;
        let fresh8 = s2;
        s2 = s2.offset(1);
        c2 = *fresh8 as libc::c_int;
        let fresh9 = n;
        n = n - 1;
        if fresh9 == 0 {
            return 0 as libc::c_int;
            // strings are equal until end point
        }
        if c1 != c2 {
            if c1 >= 'a' as i32 && c1 <= 'z' as i32 {
                c1 -= 'a' as i32 - 'A' as i32
            }
            if c2 >= 'a' as i32 && c2 <= 'z' as i32 {
                c2 -= 'a' as i32 - 'A' as i32
            }
            if c1 != c2 {
                return if c1 < c2 {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
            }
        }
        if !(c1 != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
    // strings are equal
}
#[no_mangle]
pub unsafe extern "C" fn Q_strncmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    loop {
        let fresh10 = s1;
        s1 = s1.offset(1);
        c1 = *fresh10 as libc::c_int;
        let fresh11 = s2;
        s2 = s2.offset(1);
        c2 = *fresh11 as libc::c_int;
        let fresh12 = n;
        n = n - 1;
        if fresh12 == 0 {
            return 0 as libc::c_int;
            // strings are equal until end point
        }
        if c1 != c2 {
            return if c1 < c2 {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            };
        }
        if !(c1 != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
    // strings are equal
}
#[no_mangle]
pub unsafe extern "C" fn Q_stricmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    return if !s1.is_null() && !s2.is_null() {
        Q_stricmpn(s1, s2, 99999 as libc::c_int)
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Q_strlwr(mut s1: *mut libc::c_char) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = s1;
    while *s != 0 {
        *s = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower(*s as libc::c_int)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_tolower_loc()).offset(*s as libc::c_int as isize)
            }
            __res
        }) as libc::c_char;
        s = s.offset(1)
    }
    return s1;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strupr(mut s1: *mut libc::c_char) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = s1;
    while *s != 0 {
        *s = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                    }
                } else {
                    __res = toupper(*s as libc::c_int)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_toupper_loc()).offset(*s as libc::c_int as isize)
            }
            __res
        }) as libc::c_char;
        s = s.offset(1)
    }
    return s1;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strcat(
    mut dest: *mut libc::c_char,
    mut size: libc::c_int,
    mut src: *const libc::c_char,
) {
    let mut l1: libc::c_int = 0;
    l1 = crate::stdlib::strlen(dest) as libc::c_int;
    if l1 >= size {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Q_strcat: already overflowed\x00" as *const u8 as *const libc::c_char,
        );
    }
    Q_strncpyz(dest.offset(l1 as isize), src, size - l1);
}
#[no_mangle]
pub unsafe extern "C" fn Q_stristr(
    mut s: *const libc::c_char,
    mut find: *const libc::c_char,
) -> *const libc::c_char {
    let mut c: libc::c_char = 0;
    let mut sc: libc::c_char = 0;
    let mut len: crate::stddef_h::size_t = 0;
    let fresh13 = find;
    find = find.offset(1);
    c = *fresh13;
    if c as libc::c_int != 0 as libc::c_int {
        if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
            c = (c as libc::c_int - ('a' as i32 - 'A' as i32)) as libc::c_char
        }
        len = crate::stdlib::strlen(find);
        loop {
            loop {
                let fresh14 = s;
                s = s.offset(1);
                sc = *fresh14;
                if sc as libc::c_int == 0 as libc::c_int {
                    return 0 as *const libc::c_char;
                }
                if sc as libc::c_int >= 'a' as i32 && sc as libc::c_int <= 'z' as i32 {
                    sc = (sc as libc::c_int - ('a' as i32 - 'A' as i32)) as libc::c_char
                }
                if !(sc as libc::c_int != c as libc::c_int) {
                    break;
                }
            }
            if !(Q_stricmpn(s, find, len as libc::c_int) != 0 as libc::c_int) {
                break;
            }
        }
        s = s.offset(-1)
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn Q_PrintStrlen(mut string: *const libc::c_char) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if string.is_null() {
        return 0 as libc::c_int;
    }
    len = 0 as libc::c_int;
    p = string;
    while *p != 0 {
        if Q_IsColorString(p) as u64 != 0 {
            p = p.offset(2 as libc::c_int as isize)
        } else {
            p = p.offset(1);
            len += 1
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn Q_CleanStr(mut string: *mut libc::c_char) -> *mut libc::c_char {
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    s = string;
    d = string;
    loop {
        c = *s as libc::c_int;
        if !(c != 0 as libc::c_int) {
            break;
        }
        if Q_IsColorString(s) as u64 != 0 {
            s = s.offset(1)
        } else if c >= 0x20 as libc::c_int && c <= 0x7e as libc::c_int {
            let fresh15 = d;
            d = d.offset(1);
            *fresh15 = c as libc::c_char
        }
        s = s.offset(1)
    }
    *d = '\u{0}' as i32 as libc::c_char;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn Q_CountChar(
    mut string: *const libc::c_char,
    mut tocount: libc::c_char,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    count = 0 as libc::c_int;
    while *string != 0 {
        if *string as libc::c_int == tocount as libc::c_int {
            count += 1
        }
        string = string.offset(1)
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn Com_sprintf(
    mut dest: *mut libc::c_char,
    mut size: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    len = crate::stdlib::vsnprintf(dest, size as libc::c_ulong, fmt, argptr.as_va_list());
    if len >= size {
        crate::src::game::g_main::Com_Printf(
            b"Com_sprintf: Output length %d too short, require %d bytes.\n\x00" as *const u8
                as *const libc::c_char,
            size,
            len + 1 as libc::c_int,
        );
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn va(mut format: *mut libc::c_char, mut args: ...) -> *mut libc::c_char {
    let mut argptr: ::std::ffi::VaListImpl; // in case va is called by nested functions
    static mut string: [[libc::c_char; 32000]; 2] = [[0; 32000]; 2];
    static mut index: libc::c_int = 0 as libc::c_int;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    buf = string[(index & 1 as libc::c_int) as usize].as_mut_ptr();
    index += 1;
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        buf,
        ::std::mem::size_of::<[libc::c_char; 32000]>() as libc::c_ulong,
        format,
        argptr.as_va_list(),
    );
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn Com_TruncateLongString(
    mut buffer: *mut libc::c_char,
    mut s: *const libc::c_char,
) {
    let mut length: libc::c_int = crate::stdlib::strlen(s) as libc::c_int;
    if length <= 64 as libc::c_int {
        Q_strncpyz(buffer, s, 64 as libc::c_int);
    } else {
        Q_strncpyz(
            buffer,
            s,
            64 as libc::c_int / 2 as libc::c_int - 3 as libc::c_int,
        );
        Q_strcat(
            buffer,
            64 as libc::c_int,
            b" ... \x00" as *const u8 as *const libc::c_char,
        );
        Q_strcat(
            buffer,
            64 as libc::c_int,
            s.offset(length as isize)
                .offset(-((64 as libc::c_int / 2 as libc::c_int) as isize))
                .offset(3 as libc::c_int as isize),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn Info_ValueForKey(
    mut s: *const libc::c_char,
    mut key: *const libc::c_char,
) -> *mut libc::c_char {
    let mut pkey: [libc::c_char; 8192] = [0; 8192]; // use two buffers so compares
    static mut value: [[libc::c_char; 8192]; 2] = [[0; 8192]; 2];
    // work without stomping on each other
    static mut valueindex: libc::c_int = 0 as libc::c_int;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    if s.is_null() || key.is_null() {
        return b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if crate::stdlib::strlen(s) >= 8192 as libc::c_int as libc::c_ulong {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Info_ValueForKey: oversize infostring\x00" as *const u8 as *const libc::c_char,
        );
    }
    valueindex ^= 1 as libc::c_int;
    if *s as libc::c_int == '\\' as i32 {
        s = s.offset(1)
    }
    loop {
        o = pkey.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 {
            if *s == 0 {
                return b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            let fresh16 = s;
            s = s.offset(1);
            let fresh17 = o;
            o = o.offset(1);
            *fresh17 = *fresh16
        }
        *o = 0 as libc::c_int as libc::c_char;
        s = s.offset(1);
        o = value[valueindex as usize].as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 && *s as libc::c_int != 0 {
            let fresh18 = s;
            s = s.offset(1);
            let fresh19 = o;
            o = o.offset(1);
            *fresh19 = *fresh18
        }
        *o = 0 as libc::c_int as libc::c_char;
        if Q_stricmp(key, pkey.as_mut_ptr()) == 0 {
            return value[valueindex as usize].as_mut_ptr();
        }
        if *s == 0 {
            break;
        }
        s = s.offset(1)
    }
    return b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Info_NextPair(
    mut head: *mut *const libc::c_char,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
) {
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    s = *head;
    if *s as libc::c_int == '\\' as i32 {
        s = s.offset(1)
    }
    *key.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    *value.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    o = key;
    while *s as libc::c_int != '\\' as i32 {
        if *s == 0 {
            *o = 0 as libc::c_int as libc::c_char;
            *head = s;
            return;
        }
        let fresh20 = s;
        s = s.offset(1);
        let fresh21 = o;
        o = o.offset(1);
        *fresh21 = *fresh20
    }
    *o = 0 as libc::c_int as libc::c_char;
    s = s.offset(1);
    o = value;
    while *s as libc::c_int != '\\' as i32 && *s as libc::c_int != 0 {
        let fresh22 = s;
        s = s.offset(1);
        let fresh23 = o;
        o = o.offset(1);
        *fresh23 = *fresh22
    }
    *o = 0 as libc::c_int as libc::c_char;
    *head = s;
}
#[no_mangle]
pub unsafe extern "C" fn Info_RemoveKey(mut s: *mut libc::c_char, mut key: *const libc::c_char) {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char; // remove this part
    let mut pkey: [libc::c_char; 1024] = [0; 1024];
    let mut value: [libc::c_char; 1024] = [0; 1024];
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    if crate::stdlib::strlen(s) >= 1024 as libc::c_int as libc::c_ulong {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Info_RemoveKey: oversize infostring\x00" as *const u8 as *const libc::c_char,
        );
    }
    if !::libc::strchr(key, '\\' as i32).is_null() {
        return;
    }
    loop {
        start = s;
        if *s as libc::c_int == '\\' as i32 {
            s = s.offset(1)
        }
        o = pkey.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 {
            if *s == 0 {
                return;
            }
            let fresh24 = s;
            s = s.offset(1);
            let fresh25 = o;
            o = o.offset(1);
            *fresh25 = *fresh24
        }
        *o = 0 as libc::c_int as libc::c_char;
        s = s.offset(1);
        o = value.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 && *s as libc::c_int != 0 {
            if *s == 0 {
                return;
            }
            let fresh26 = s;
            s = s.offset(1);
            let fresh27 = o;
            o = o.offset(1);
            *fresh27 = *fresh26
        }
        *o = 0 as libc::c_int as libc::c_char;
        if ::libc::strcmp(key, pkey.as_mut_ptr()) == 0 {
            crate::stdlib::memmove(
                start as *mut libc::c_void,
                s as *const libc::c_void,
                crate::stdlib::strlen(s).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            return;
        }
        if *s == 0 {
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Info_RemoveKey_Big(
    mut s: *mut libc::c_char,
    mut key: *const libc::c_char,
) {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char; // remove this part
    let mut pkey: [libc::c_char; 8192] = [0; 8192];
    let mut value: [libc::c_char; 8192] = [0; 8192];
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    if crate::stdlib::strlen(s) >= 8192 as libc::c_int as libc::c_ulong {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Info_RemoveKey_Big: oversize infostring\x00" as *const u8 as *const libc::c_char,
        );
    }
    if !::libc::strchr(key, '\\' as i32).is_null() {
        return;
    }
    loop {
        start = s;
        if *s as libc::c_int == '\\' as i32 {
            s = s.offset(1)
        }
        o = pkey.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 {
            if *s == 0 {
                return;
            }
            let fresh28 = s;
            s = s.offset(1);
            let fresh29 = o;
            o = o.offset(1);
            *fresh29 = *fresh28
        }
        *o = 0 as libc::c_int as libc::c_char;
        s = s.offset(1);
        o = value.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 && *s as libc::c_int != 0 {
            if *s == 0 {
                return;
            }
            let fresh30 = s;
            s = s.offset(1);
            let fresh31 = o;
            o = o.offset(1);
            *fresh31 = *fresh30
        }
        *o = 0 as libc::c_int as libc::c_char;
        if ::libc::strcmp(key, pkey.as_mut_ptr()) == 0 {
            crate::stdlib::memmove(
                start as *mut libc::c_void,
                s as *const libc::c_void,
                crate::stdlib::strlen(s).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            return;
        }
        if *s == 0 {
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Info_Validate(
    mut s: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    if !::libc::strchr(s, '\"' as i32).is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if !::libc::strchr(s, ';' as i32).is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn Info_SetValueForKey(
    mut s: *mut libc::c_char,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut newi: [libc::c_char; 1024] = [0; 1024];
    let mut blacklist: *const libc::c_char = b"\\;\"\x00" as *const u8 as *const libc::c_char;
    if crate::stdlib::strlen(s) >= 1024 as libc::c_int as libc::c_ulong {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Info_SetValueForKey: oversize infostring\x00" as *const u8 as *const libc::c_char,
        );
    }
    while *blacklist != 0 {
        if !::libc::strchr(key, *blacklist as libc::c_int).is_null()
            || !::libc::strchr(value, *blacklist as libc::c_int).is_null()
        {
            crate::src::game::g_main::Com_Printf(
                b"^3Can\'t use keys or values with a \'%c\': %s = %s\n\x00" as *const u8
                    as *const libc::c_char,
                *blacklist as libc::c_int,
                key,
                value,
            );
            return;
        }
        blacklist = blacklist.offset(1)
    }
    Info_RemoveKey(s, key);
    if value.is_null() || crate::stdlib::strlen(value) == 0 {
        return;
    }
    Com_sprintf(
        newi.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        b"\\%s\\%s\x00" as *const u8 as *const libc::c_char,
        key,
        value,
    );
    if crate::stdlib::strlen(newi.as_mut_ptr()).wrapping_add(crate::stdlib::strlen(s))
        >= 1024 as libc::c_int as libc::c_ulong
    {
        crate::src::game::g_main::Com_Printf(
            b"Info string length exceeded\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    ::libc::strcat(newi.as_mut_ptr(), s);
    ::libc::strcpy(s, newi.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Info_SetValueForKey_Big(
    mut s: *mut libc::c_char,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut newi: [libc::c_char; 8192] = [0; 8192];
    let mut blacklist: *const libc::c_char = b"\\;\"\x00" as *const u8 as *const libc::c_char;
    if crate::stdlib::strlen(s) >= 8192 as libc::c_int as libc::c_ulong {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Info_SetValueForKey: oversize infostring\x00" as *const u8 as *const libc::c_char,
        );
    }
    while *blacklist != 0 {
        if !::libc::strchr(key, *blacklist as libc::c_int).is_null()
            || !::libc::strchr(value, *blacklist as libc::c_int).is_null()
        {
            crate::src::game::g_main::Com_Printf(
                b"^3Can\'t use keys or values with a \'%c\': %s = %s\n\x00" as *const u8
                    as *const libc::c_char,
                *blacklist as libc::c_int,
                key,
                value,
            );
            return;
        }
        blacklist = blacklist.offset(1)
    }
    Info_RemoveKey_Big(s, key);
    if value.is_null() {
        return;
    }
    Com_sprintf(
        newi.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as libc::c_int,
        b"\\%s\\%s\x00" as *const u8 as *const libc::c_char,
        key,
        value,
    );
    if crate::stdlib::strlen(newi.as_mut_ptr()).wrapping_add(crate::stdlib::strlen(s))
        >= 8192 as libc::c_int as libc::c_ulong
    {
        crate::src::game::g_main::Com_Printf(
            b"BIG Info string length exceeded\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    ::libc::strcat(s, newi.as_mut_ptr());
}
unsafe extern "C" fn Com_CharIsOneOfCharset(
    mut c: libc::c_char,
    mut set: *mut libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < crate::stdlib::strlen(set) {
        if *set.offset(i as isize) as libc::c_int == c as libc::c_int {
            return crate::src::qcommon::q_shared::qtrue;
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn Com_SkipCharset(
    mut s: *mut libc::c_char,
    mut sep: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = s;
    while !p.is_null() {
        if !(Com_CharIsOneOfCharset(*p, sep) as u64 != 0) {
            break;
        }
        p = p.offset(1)
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn Com_SkipTokens(
    mut s: *mut libc::c_char,
    mut numTokens: libc::c_int,
    mut sep: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut sepCount: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = s;
    while sepCount < numTokens {
        let fresh32 = p;
        p = p.offset(1);
        if Com_CharIsOneOfCharset(*fresh32, sep) as u64 != 0 {
            sepCount += 1;
            while Com_CharIsOneOfCharset(*p, sep) as u64 != 0 {
                p = p.offset(1)
            }
        } else if *p as libc::c_int == '\u{0}' as i32 {
            break;
        }
    }
    if sepCount == numTokens {
        return p;
    } else {
        return s;
    };
}

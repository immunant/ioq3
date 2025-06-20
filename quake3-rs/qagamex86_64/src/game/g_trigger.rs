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

    //=====================================================================
    // in order from highest priority to lowest
    // if none of the catchers are active, bound key strings will be executed
    // sound channels
    // channel 0 never willingly overrides
    // other channels will allways override a playing sound on that channel

    // announcer voices, etc
    // chat messages, etc

    // menu sounds, etc

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

    pub unsafe extern "C" fn VectorCompare(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
    ) -> i32 {
        if *v1.offset(0 as i32 as isize) != *v2.offset(0 as i32 as isize)
            || *v1.offset(1 as i32 as isize) != *v2.offset(1 as i32 as isize)
            || *v1.offset(2 as i32 as isize) != *v2.offset(2 as i32 as isize)
        {
            return 0 as i32;
        }
        return 1 as i32;
    }

    // __Q_SHARED_H
}

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
pub use crate::bg_public_h::ET_BEAM;
pub use crate::bg_public_h::ET_EVENTS;
pub use crate::bg_public_h::ET_GENERAL;
pub use crate::bg_public_h::ET_GRAPPLE;
pub use crate::bg_public_h::ET_INVISIBLE;
pub use crate::bg_public_h::ET_ITEM;
pub use crate::bg_public_h::ET_MISSILE;
pub use crate::bg_public_h::ET_MOVER;
pub use crate::bg_public_h::ET_PLAYER;
pub use crate::bg_public_h::ET_PORTAL;
pub use crate::bg_public_h::ET_PUSH_TRIGGER;
pub use crate::bg_public_h::ET_SPEAKER;
pub use crate::bg_public_h::ET_TEAM;
pub use crate::bg_public_h::ET_TELEPORT_TRIGGER;
pub use crate::bg_public_h::IT_AMMO;
pub use crate::bg_public_h::IT_ARMOR;
pub use crate::bg_public_h::IT_BAD;
pub use crate::bg_public_h::IT_HEALTH;
pub use crate::bg_public_h::IT_HOLDABLE;
pub use crate::bg_public_h::IT_PERSISTANT_POWERUP;
pub use crate::bg_public_h::IT_POWERUP;
pub use crate::bg_public_h::IT_TEAM;
pub use crate::bg_public_h::IT_WEAPON;
pub use crate::bg_public_h::MOD_BFG;
pub use crate::bg_public_h::MOD_BFG_SPLASH;
pub use crate::bg_public_h::MOD_CRUSH;
pub use crate::bg_public_h::MOD_FALLING;
pub use crate::bg_public_h::MOD_GAUNTLET;
pub use crate::bg_public_h::MOD_GRAPPLE;
pub use crate::bg_public_h::MOD_GRENADE;
pub use crate::bg_public_h::MOD_GRENADE_SPLASH;
pub use crate::bg_public_h::MOD_LAVA;
pub use crate::bg_public_h::MOD_LIGHTNING;
pub use crate::bg_public_h::MOD_MACHINEGUN;
pub use crate::bg_public_h::MOD_PLASMA;
pub use crate::bg_public_h::MOD_PLASMA_SPLASH;
pub use crate::bg_public_h::MOD_RAILGUN;
pub use crate::bg_public_h::MOD_ROCKET;
pub use crate::bg_public_h::MOD_ROCKET_SPLASH;
pub use crate::bg_public_h::MOD_SHOTGUN;
pub use crate::bg_public_h::MOD_SLIME;
pub use crate::bg_public_h::MOD_SUICIDE;
pub use crate::bg_public_h::MOD_TARGET_LASER;
pub use crate::bg_public_h::MOD_TELEFRAG;
pub use crate::bg_public_h::MOD_TRIGGER_HURT;
pub use crate::bg_public_h::MOD_UNKNOWN;
pub use crate::bg_public_h::MOD_WATER;
pub use crate::bg_public_h::PM_DEAD;
pub use crate::bg_public_h::PM_FREEZE;
pub use crate::bg_public_h::PM_INTERMISSION;
pub use crate::bg_public_h::PM_NOCLIP;
pub use crate::bg_public_h::PM_NORMAL;
pub use crate::bg_public_h::PM_SPECTATOR;
pub use crate::bg_public_h::PM_SPINTERMISSION;
pub use crate::bg_public_h::PW_AMMOREGEN;
pub use crate::bg_public_h::PW_BATTLESUIT;
pub use crate::bg_public_h::PW_BLUEFLAG;
pub use crate::bg_public_h::PW_DOUBLER;
pub use crate::bg_public_h::PW_FLIGHT;
pub use crate::bg_public_h::PW_GUARD;
pub use crate::bg_public_h::PW_HASTE;
pub use crate::bg_public_h::PW_INVIS;
pub use crate::bg_public_h::PW_INVULNERABILITY;
pub use crate::bg_public_h::PW_NEUTRALFLAG;
pub use crate::bg_public_h::PW_NONE;
pub use crate::bg_public_h::PW_NUM_POWERUPS;
pub use crate::bg_public_h::PW_QUAD;
pub use crate::bg_public_h::PW_REDFLAG;
pub use crate::bg_public_h::PW_REGEN;
pub use crate::bg_public_h::PW_SCOUT;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::g_local_h::clientConnected_t;
pub use crate::g_local_h::clientPersistant_t;
pub use crate::g_local_h::clientSession_t;
pub use crate::g_local_h::gclient_s;
pub use crate::g_local_h::gentity_s;
pub use crate::g_local_h::gentity_t;
pub use crate::g_local_h::level_locals_t;
pub use crate::g_local_h::moverState_t;
pub use crate::g_local_h::playerTeamStateState_t;
pub use crate::g_local_h::playerTeamState_t;
pub use crate::g_local_h::spectatorState_t;
pub use crate::g_local_h::CON_CONNECTED;
pub use crate::g_local_h::CON_CONNECTING;
pub use crate::g_local_h::CON_DISCONNECTED;
pub use crate::g_local_h::MOVER_1TO2;
pub use crate::g_local_h::MOVER_2TO1;
pub use crate::g_local_h::MOVER_POS1;
pub use crate::g_local_h::MOVER_POS2;
pub use crate::g_local_h::SPECTATOR_FOLLOW;
pub use crate::g_local_h::SPECTATOR_FREE;
pub use crate::g_local_h::SPECTATOR_NOT;
pub use crate::g_local_h::SPECTATOR_SCOREBOARD;
pub use crate::g_local_h::TEAM_ACTIVE;
pub use crate::g_local_h::TEAM_BEGIN;
pub use crate::g_public_h::entityShared_t;
pub use crate::src::game::bg_misc::BG_TouchJumpPad;
pub use crate::src::game::g_combat::G_Damage;
pub use crate::src::game::g_main::g_gravity;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_misc::TeleportPlayer;
pub use crate::src::game::g_spawn::G_SpawnFloat;
pub use crate::src::game::g_syscalls::trap_LinkEntity;
pub use crate::src::game::g_syscalls::trap_SetBrushModel;
pub use crate::src::game::g_syscalls::trap_UnlinkEntity;
pub use crate::src::game::g_trigger::q_shared_h::VectorCompare;
pub use crate::src::game::g_utils::vtos;
pub use crate::src::game::g_utils::G_FreeEntity;
pub use crate::src::game::g_utils::G_PickTarget;
pub use crate::src::game::g_utils::G_SetMovedir;
pub use crate::src::game::g_utils::G_Sound;
pub use crate::src::game::g_utils::G_SoundIndex;
pub use crate::src::game::g_utils::G_UseTargets;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
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
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

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
#[no_mangle]

pub unsafe extern "C" fn InitTrigger(mut self_0: *mut gentity_t) {
    if VectorCompare(
        (*self_0).s.angles.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
    ) == 0
    {
        G_SetMovedir(
            (*self_0).s.angles.as_mut_ptr(),
            (*self_0).movedir.as_mut_ptr(),
        ); // replaces the -1 from trap_SetBrushModel
    }
    trap_SetBrushModel(self_0 as *mut gentity_s, (*self_0).model);
    (*self_0).r.contents = 0x40000000 as i32;
    (*self_0).r.svFlags = 0x1 as i32;
}
// the wait time has passed, so set back up for another activation
#[no_mangle]

pub unsafe extern "C" fn multi_wait(mut ent: *mut gentity_t) {
    (*ent).nextthink = 0 as i32;
}
// the trigger was just activated
// ent->activator should be set to the activator so it can be held through a delay
// so wait for the delay time before firing
#[no_mangle]

pub unsafe extern "C" fn multi_trigger(mut ent: *mut gentity_t, mut activator: *mut gentity_t) {
    (*ent).activator = activator;
    if (*ent).nextthink != 0 {
        return;
        // can't retrigger until the wait is over
    }
    if !(*activator).client.is_null() {
        if (*ent).spawnflags & 1 as i32 != 0
            && (*(*activator).client).sess.sessionTeam as u32 != TEAM_RED as i32 as u32
        {
            return;
        }
        if (*ent).spawnflags & 2 as i32 != 0
            && (*(*activator).client).sess.sessionTeam as u32 != TEAM_BLUE as i32 as u32
        {
            return;
        }
    }
    G_UseTargets(ent as *mut gentity_s, (*ent).activator as *mut gentity_s);
    if (*ent).wait > 0 as i32 as f32 {
        (*ent).think = Some(multi_wait as unsafe extern "C" fn(_: *mut gentity_t) -> ());
        (*ent).nextthink = (level.time as f64
            + ((*ent).wait as f64
                + (*ent).random as f64
                    * (2.0f64
                        * (((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64
                            - 0.5f64)))
                * 1000 as i32 as f64) as i32
    } else {
        // we can't just remove (self) here, because this is a touch function
        // called while looping through area links...
        (*ent).touch = None;
        (*ent).nextthink = level.time + 100 as i32;
        (*ent).think = Some(G_FreeEntity as unsafe extern "C" fn(_: *mut gentity_t) -> ())
    };
}
#[no_mangle]

pub unsafe extern "C" fn Use_Multi(
    mut ent: *mut gentity_t,
    mut _other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    multi_trigger(ent, activator);
}
#[no_mangle]

pub unsafe extern "C" fn Touch_Multi(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut _trace: *mut trace_t,
) {
    if (*other).client.is_null() {
        return;
    }
    multi_trigger(self_0, other);
}
/*QUAKED trigger_multiple (.5 .5 .5) ? RED_ONLY BLUE_ONLY
"wait" : Seconds between triggerings, 0.5 default, -1 = one time only.
"random"	wait variance, default is 0
Variable sized repeatable trigger.  Must be targeted at one or more entities.
so, the basic time between firing is a random time between
(wait - random) and (wait + random)
*/
#[no_mangle]

pub unsafe extern "C" fn SP_trigger_multiple(mut ent: *mut gentity_t) {
    G_SpawnFloat(
        b"wait\x00" as *const u8 as *const libc::c_char,
        b"0.5\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).wait,
    );
    G_SpawnFloat(
        b"random\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).random,
    );
    if (*ent).random >= (*ent).wait && (*ent).wait >= 0 as i32 as f32 {
        (*ent).random = (*ent).wait - 100 as i32 as f32;
        G_Printf(b"trigger_multiple has random >= wait\n\x00" as *const u8 as *const libc::c_char);
    }
    (*ent).touch = Some(
        Touch_Multi
            as unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut trace_t) -> (),
    );
    (*ent).use_0 = Some(
        Use_Multi
            as unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut gentity_t) -> (),
    );
    InitTrigger(ent);
    trap_LinkEntity(ent as *mut gentity_s);
}
/*
==============================================================================

trigger_always

==============================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn trigger_always_think(mut ent: *mut gentity_t) {
    G_UseTargets(ent as *mut gentity_s, ent as *mut gentity_s);
    G_FreeEntity(ent as *mut gentity_s);
}
/*QUAKED trigger_always (.5 .5 .5) (-8 -8 -8) (8 8 8)
This trigger will always fire.  It is activated by the world.
*/
#[no_mangle]

pub unsafe extern "C" fn SP_trigger_always(mut ent: *mut gentity_t) {
    // we must have some delay to make sure our use targets are present
    (*ent).nextthink = level.time + 300 as i32;
    (*ent).think = Some(trigger_always_think as unsafe extern "C" fn(_: *mut gentity_t) -> ());
}
/*
==============================================================================

trigger_push

==============================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn trigger_push_touch(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut _trace: *mut trace_t,
) {
    if (*other).client.is_null() {
        return;
    }
    BG_TouchJumpPad(
        &mut (*(*other).client).ps as *mut _ as *mut playerState_s,
        &mut (*self_0).s as *mut _ as *mut entityState_s,
    );
}
/*
=================
AimAtTarget

Calculate origin2 so the target apogee will be hit
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AimAtTarget(mut self_0: *mut gentity_t) {
    let mut ent: *mut gentity_t = std::ptr::null_mut();
    let mut origin: vec3_t = [0.; 3];
    let mut height: f32 = 0.;
    let mut gravity: f32 = 0.;
    let mut time: f32 = 0.;
    let mut forward: f32 = 0.;
    let mut dist: f32 = 0.;
    origin[0 as i32 as usize] =
        (*self_0).r.absmin[0 as i32 as usize] + (*self_0).r.absmax[0 as i32 as usize];
    origin[1 as i32 as usize] =
        (*self_0).r.absmin[1 as i32 as usize] + (*self_0).r.absmax[1 as i32 as usize];
    origin[2 as i32 as usize] =
        (*self_0).r.absmin[2 as i32 as usize] + (*self_0).r.absmax[2 as i32 as usize];
    origin[0 as i32 as usize] = (origin[0 as i32 as usize] as f64 * 0.5f64) as vec_t;
    origin[1 as i32 as usize] = (origin[1 as i32 as usize] as f64 * 0.5f64) as vec_t;
    origin[2 as i32 as usize] = (origin[2 as i32 as usize] as f64 * 0.5f64) as vec_t;
    ent = G_PickTarget((*self_0).target) as *mut gentity_s;
    if ent.is_null() {
        G_FreeEntity(self_0 as *mut gentity_s);
        return;
    }
    height = (*ent).s.origin[2 as i32 as usize] - origin[2 as i32 as usize];
    gravity = g_gravity.value;
    time = crate::stdlib::sqrt(height as f64 / (0.5f64 * gravity as f64)) as f32;
    if time == 0. {
        G_FreeEntity(self_0 as *mut gentity_s);
        return;
    }
    // set s.origin2 to the push velocity
    (*self_0).s.origin2[0 as i32 as usize] =
        (*ent).s.origin[0 as i32 as usize] - origin[0 as i32 as usize];
    (*self_0).s.origin2[1 as i32 as usize] =
        (*ent).s.origin[1 as i32 as usize] - origin[1 as i32 as usize];
    (*self_0).s.origin2[2 as i32 as usize] =
        (*ent).s.origin[2 as i32 as usize] - origin[2 as i32 as usize];
    (*self_0).s.origin2[2 as i32 as usize] = 0 as i32 as vec_t;
    dist = VectorNormalize((*self_0).s.origin2.as_mut_ptr());
    forward = dist / time;
    (*self_0).s.origin2[0 as i32 as usize] = (*self_0).s.origin2[0 as i32 as usize] * forward;
    (*self_0).s.origin2[1 as i32 as usize] = (*self_0).s.origin2[1 as i32 as usize] * forward;
    (*self_0).s.origin2[2 as i32 as usize] = (*self_0).s.origin2[2 as i32 as usize] * forward;
    (*self_0).s.origin2[2 as i32 as usize] = time * gravity;
}
/*QUAKED trigger_push (.5 .5 .5) ?
Must point at a target_position, which will be the apex of the leap.
This will be client side predicted, unlike target_push
*/
#[no_mangle]

pub unsafe extern "C" fn SP_trigger_push(mut self_0: *mut gentity_t) {
    InitTrigger(self_0);
    // unlike other triggers, we need to send this one to the client
    (*self_0).r.svFlags &= !(0x1 as i32);
    // make sure the client precaches this sound
    G_SoundIndex(
        b"sound/world/jumppad.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).s.eType = ET_PUSH_TRIGGER as i32;
    (*self_0).touch = Some(
        trigger_push_touch
            as unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut trace_t) -> (),
    );
    (*self_0).think = Some(AimAtTarget as unsafe extern "C" fn(_: *mut gentity_t) -> ());
    (*self_0).nextthink = level.time + 100 as i32;
    trap_LinkEntity(self_0 as *mut gentity_s);
}
#[no_mangle]

pub unsafe extern "C" fn Use_target_push(
    mut self_0: *mut gentity_t,
    mut _other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    if (*activator).client.is_null() {
        return;
    }
    if (*(*activator).client).ps.pm_type != PM_NORMAL as i32 {
        return;
    }
    if (*(*activator).client).ps.powerups[PW_FLIGHT as i32 as usize] != 0 {
        return;
    }
    (*(*activator).client).ps.velocity[0 as i32 as usize] = (*self_0).s.origin2[0 as i32 as usize];
    (*(*activator).client).ps.velocity[1 as i32 as usize] = (*self_0).s.origin2[1 as i32 as usize];
    (*(*activator).client).ps.velocity[2 as i32 as usize] = (*self_0).s.origin2[2 as i32 as usize];
    // play fly sound every 1.5 seconds
    if (*activator).fly_sound_debounce_time < level.time {
        (*activator).fly_sound_debounce_time = level.time + 1500 as i32;
        G_Sound(
            activator as *mut gentity_s,
            CHAN_AUTO as i32,
            (*self_0).noise_index,
        );
    };
}
/*QUAKED target_push (.5 .5 .5) (-8 -8 -8) (8 8 8) bouncepad
Pushes the activator in the direction.of angle, or towards a target apex.
"speed"		defaults to 1000
if "bouncepad", play bounce noise instead of windfly
*/
#[no_mangle]

pub unsafe extern "C" fn SP_target_push(mut self_0: *mut gentity_t) {
    if (*self_0).speed == 0. {
        (*self_0).speed = 1000 as i32 as f32
    }
    G_SetMovedir(
        (*self_0).s.angles.as_mut_ptr(),
        (*self_0).s.origin2.as_mut_ptr(),
    );
    (*self_0).s.origin2[0 as i32 as usize] =
        (*self_0).s.origin2[0 as i32 as usize] * (*self_0).speed;
    (*self_0).s.origin2[1 as i32 as usize] =
        (*self_0).s.origin2[1 as i32 as usize] * (*self_0).speed;
    (*self_0).s.origin2[2 as i32 as usize] =
        (*self_0).s.origin2[2 as i32 as usize] * (*self_0).speed;
    if (*self_0).spawnflags & 1 as i32 != 0 {
        (*self_0).noise_index = G_SoundIndex(
            b"sound/world/jumppad.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        )
    } else {
        (*self_0).noise_index = G_SoundIndex(
            b"sound/misc/windfly.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        )
    }
    if !(*self_0).target.is_null() {
        (*self_0).r.absmin[0 as i32 as usize] = (*self_0).s.origin[0 as i32 as usize];
        (*self_0).r.absmin[1 as i32 as usize] = (*self_0).s.origin[1 as i32 as usize];
        (*self_0).r.absmin[2 as i32 as usize] = (*self_0).s.origin[2 as i32 as usize];
        (*self_0).r.absmax[0 as i32 as usize] = (*self_0).s.origin[0 as i32 as usize];
        (*self_0).r.absmax[1 as i32 as usize] = (*self_0).s.origin[1 as i32 as usize];
        (*self_0).r.absmax[2 as i32 as usize] = (*self_0).s.origin[2 as i32 as usize];
        (*self_0).think = Some(AimAtTarget as unsafe extern "C" fn(_: *mut gentity_t) -> ());
        (*self_0).nextthink = level.time + 100 as i32
    }
    (*self_0).use_0 = Some(
        Use_target_push
            as unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut gentity_t) -> (),
    );
}
// armor, shields, invulnerability, and godmode have no effect
//
// g_missile.c
//
//
// g_mover.c
//
//
// g_trigger.c
//
/*
==============================================================================

trigger_teleport

==============================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn trigger_teleporter_touch(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut _trace: *mut trace_t,
) {
    let mut dest: *mut gentity_t = std::ptr::null_mut();
    if (*other).client.is_null() {
        return;
    }
    if (*(*other).client).ps.pm_type == PM_DEAD as i32 {
        return;
    }
    // Spectators only?
    if (*self_0).spawnflags & 1 as i32 != 0
        && (*(*other).client).sess.sessionTeam as u32 != TEAM_SPECTATOR as i32 as u32
    {
        return;
    }
    dest = G_PickTarget((*self_0).target) as *mut gentity_s;
    if dest.is_null() {
        G_Printf(
            b"Couldn\'t find teleporter destination\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    TeleportPlayer(
        other as *mut gentity_s,
        (*dest).s.origin.as_mut_ptr(),
        (*dest).s.angles.as_mut_ptr(),
    );
}
/*QUAKED trigger_teleport (.5 .5 .5) ? SPECTATOR
Allows client side prediction of teleportation events.
Must point at a target_position, which will be the teleport destination.

If spectator is set, only spectators can use this teleport
Spectator teleporters are not normally placed in the editor, but are created
automatically near doors to allow spectators to move through them
*/
#[no_mangle]

pub unsafe extern "C" fn SP_trigger_teleport(mut self_0: *mut gentity_t) {
    InitTrigger(self_0);
    // unlike other triggers, we need to send this one to the client
    // unless is a spectator trigger
    if (*self_0).spawnflags & 1 as i32 != 0 {
        (*self_0).r.svFlags |= 0x1 as i32
    } else {
        (*self_0).r.svFlags &= !(0x1 as i32)
    }
    // make sure the client precaches this sound
    G_SoundIndex(
        b"sound/world/jumppad.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).s.eType = ET_TELEPORT_TRIGGER as i32;
    (*self_0).touch = Some(
        trigger_teleporter_touch
            as unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut trace_t) -> (),
    );
    trap_LinkEntity(self_0 as *mut gentity_s);
}
/*
==============================================================================

trigger_hurt

==============================================================================
*/
/*QUAKED trigger_hurt (.5 .5 .5) ? START_OFF - SILENT NO_PROTECTION SLOW
Any entity that touches this will be hurt.
It does dmg points of damage each server frame
Targeting the trigger will toggle its on / off state.

SILENT			suppresses playing the sound
SLOW			changes the damage rate to once per second
NO_PROTECTION	*nothing* stops the damage

"dmg"			default 5 (whole numbers only)

*/
#[no_mangle]

pub unsafe extern "C" fn hurt_use(
    mut self_0: *mut gentity_t,
    mut _other: *mut gentity_t,
    mut _activator: *mut gentity_t,
) {
    if (*self_0).r.linked as u64 != 0 {
        trap_UnlinkEntity(self_0 as *mut gentity_s);
    } else {
        trap_LinkEntity(self_0 as *mut gentity_s);
    };
}
#[no_mangle]

pub unsafe extern "C" fn hurt_touch(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut _trace: *mut trace_t,
) {
    let mut dflags: i32 = 0;
    if (*other).takedamage as u64 == 0 {
        return;
    }
    if (*self_0).timestamp > level.time {
        return;
    }
    if (*self_0).spawnflags & 16 as i32 != 0 {
        (*self_0).timestamp = level.time + 1000 as i32
    } else {
        (*self_0).timestamp = level.time + 100 as i32
    }
    // play sound
    if (*self_0).spawnflags & 4 as i32 == 0 {
        G_Sound(
            other as *mut gentity_s,
            CHAN_AUTO as i32,
            (*self_0).noise_index,
        );
    }
    if (*self_0).spawnflags & 8 as i32 != 0 {
        dflags = 0x8 as i32
    } else {
        dflags = 0 as i32
    }
    G_Damage(
        other as *mut gentity_s,
        self_0 as *mut gentity_s,
        self_0 as *mut gentity_s,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        (*self_0).damage,
        dflags,
        MOD_TRIGGER_HURT as i32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn SP_trigger_hurt(mut self_0: *mut gentity_t) {
    InitTrigger(self_0);
    (*self_0).noise_index = G_SoundIndex(
        b"sound/world/electro.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).touch = Some(
        hurt_touch
            as unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut trace_t) -> (),
    );
    if (*self_0).damage == 0 {
        (*self_0).damage = 5 as i32
    }
    (*self_0).use_0 = Some(
        hurt_use
            as unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut gentity_t) -> (),
    );
    // link in to the world if starting active
    if (*self_0).spawnflags & 1 as i32 != 0 {
        trap_UnlinkEntity(self_0 as *mut gentity_s);
    } else {
        trap_LinkEntity(self_0 as *mut gentity_s);
    };
}
/*
==============================================================================

timer

==============================================================================
*/
/*QUAKED func_timer (0.3 0.1 0.6) (-8 -8 -8) (8 8 8) START_ON
This should be renamed trigger_timer...
Repeatedly fires its targets.
Can be turned on or off by using.

"wait"			base time between triggering all targets, default is 1
"random"		wait variance, default is 0
so, the basic time between firing is a random time between
(wait - random) and (wait + random)

*/
#[no_mangle]

pub unsafe extern "C" fn func_timer_think(mut self_0: *mut gentity_t) {
    G_UseTargets(
        self_0 as *mut gentity_s,
        (*self_0).activator as *mut gentity_s,
    );
    // set time before next firing
    (*self_0).nextthink = (level.time as f64
        + 1000 as i32 as f64
            * ((*self_0).wait as f64
                + 2.0f64
                    * (((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64
                        - 0.5f64)
                    * (*self_0).random as f64)) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn func_timer_use(
    mut self_0: *mut gentity_t,
    mut _other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    (*self_0).activator = activator;
    // if on, turn it off
    if (*self_0).nextthink != 0 {
        (*self_0).nextthink = 0 as i32;
        return;
    }
    // turn it on
    func_timer_think(self_0);
}
#[no_mangle]

pub unsafe extern "C" fn SP_func_timer(mut self_0: *mut gentity_t) {
    G_SpawnFloat(
        b"random\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        &mut (*self_0).random,
    );
    G_SpawnFloat(
        b"wait\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        &mut (*self_0).wait,
    );
    (*self_0).use_0 = Some(
        func_timer_use
            as unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut gentity_t) -> (),
    );
    (*self_0).think = Some(func_timer_think as unsafe extern "C" fn(_: *mut gentity_t) -> ());
    if (*self_0).random >= (*self_0).wait {
        (*self_0).random = (*self_0).wait - 100 as i32 as f32;
        G_Printf(
            b"func_timer at %s has random >= wait\n\x00" as *const u8 as *const libc::c_char,
            vtos((*self_0).s.origin.as_mut_ptr() as *const vec_t),
        );
    }
    if (*self_0).spawnflags & 1 as i32 != 0 {
        (*self_0).nextthink = level.time + 100 as i32;
        (*self_0).activator = self_0
    }
    (*self_0).r.svFlags = 0x1 as i32;
}

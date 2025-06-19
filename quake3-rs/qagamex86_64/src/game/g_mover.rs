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

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0 as i32 as isize) * *v.offset(0 as i32 as isize)
                + *v.offset(1 as i32 as isize) * *v.offset(1 as i32 as isize)
                + *v.offset(2 as i32 as isize) * *v.offset(2 as i32 as isize)) as f64,
        ) as crate::src::qcommon::q_shared::vec_t;
    }
    #[inline]

    pub unsafe extern "C" fn VectorInverse(mut v: *mut crate::src::qcommon::q_shared::vec_t) {
        *v.offset(0 as i32 as isize) = -*v.offset(0 as i32 as isize);
        *v.offset(1 as i32 as isize) = -*v.offset(1 as i32 as isize);
        *v.offset(2 as i32 as isize) = -*v.offset(2 as i32 as isize);
    }

    // __Q_SHARED_H
}

pub use crate::stddef_h::size_t;

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
pub use crate::bg_public_h::EV_BULLET;
pub use crate::bg_public_h::EV_BULLET_HIT_FLESH;
pub use crate::bg_public_h::EV_BULLET_HIT_WALL;
pub use crate::bg_public_h::EV_CHANGE_WEAPON;
pub use crate::bg_public_h::EV_DEATH1;
pub use crate::bg_public_h::EV_DEATH2;
pub use crate::bg_public_h::EV_DEATH3;
pub use crate::bg_public_h::EV_DEBUG_LINE;
pub use crate::bg_public_h::EV_FALL_FAR;
pub use crate::bg_public_h::EV_FALL_MEDIUM;
pub use crate::bg_public_h::EV_FALL_SHORT;
pub use crate::bg_public_h::EV_FIRE_WEAPON;
pub use crate::bg_public_h::EV_FOOTSPLASH;
pub use crate::bg_public_h::EV_FOOTSTEP;
pub use crate::bg_public_h::EV_FOOTSTEP_METAL;
pub use crate::bg_public_h::EV_FOOTWADE;
pub use crate::bg_public_h::EV_GENERAL_SOUND;
pub use crate::bg_public_h::EV_GIB_PLAYER;
pub use crate::bg_public_h::EV_GLOBAL_ITEM_PICKUP;
pub use crate::bg_public_h::EV_GLOBAL_SOUND;
pub use crate::bg_public_h::EV_GLOBAL_TEAM_SOUND;
pub use crate::bg_public_h::EV_GRENADE_BOUNCE;
pub use crate::bg_public_h::EV_INVUL_IMPACT;
pub use crate::bg_public_h::EV_ITEM_PICKUP;
pub use crate::bg_public_h::EV_ITEM_POP;
pub use crate::bg_public_h::EV_ITEM_RESPAWN;
pub use crate::bg_public_h::EV_JUICED;
pub use crate::bg_public_h::EV_JUMP;
pub use crate::bg_public_h::EV_JUMP_PAD;
pub use crate::bg_public_h::EV_KAMIKAZE;
pub use crate::bg_public_h::EV_LIGHTNINGBOLT;
pub use crate::bg_public_h::EV_MISSILE_HIT;
pub use crate::bg_public_h::EV_MISSILE_MISS;
pub use crate::bg_public_h::EV_MISSILE_MISS_METAL;
pub use crate::bg_public_h::EV_NOAMMO;
pub use crate::bg_public_h::EV_NONE;
pub use crate::bg_public_h::EV_OBELISKEXPLODE;
pub use crate::bg_public_h::EV_OBELISKPAIN;
pub use crate::bg_public_h::EV_OBITUARY;
pub use crate::bg_public_h::EV_PAIN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_IN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_OUT;
pub use crate::bg_public_h::EV_POWERUP_BATTLESUIT;
pub use crate::bg_public_h::EV_POWERUP_QUAD;
pub use crate::bg_public_h::EV_POWERUP_REGEN;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_STICK;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_TRIGGER;
pub use crate::bg_public_h::EV_RAILTRAIL;
pub use crate::bg_public_h::EV_SCOREPLUM;
pub use crate::bg_public_h::EV_SHOTGUN;
pub use crate::bg_public_h::EV_STEP_12;
pub use crate::bg_public_h::EV_STEP_16;
pub use crate::bg_public_h::EV_STEP_4;
pub use crate::bg_public_h::EV_STEP_8;
pub use crate::bg_public_h::EV_STOPLOOPINGSOUND;
pub use crate::bg_public_h::EV_SWIM;
pub use crate::bg_public_h::EV_TAUNT;
pub use crate::bg_public_h::EV_TAUNT_FOLLOWME;
pub use crate::bg_public_h::EV_TAUNT_GETFLAG;
pub use crate::bg_public_h::EV_TAUNT_GUARDBASE;
pub use crate::bg_public_h::EV_TAUNT_NO;
pub use crate::bg_public_h::EV_TAUNT_PATROL;
pub use crate::bg_public_h::EV_TAUNT_YES;
pub use crate::bg_public_h::EV_USE_ITEM0;
pub use crate::bg_public_h::EV_USE_ITEM1;
pub use crate::bg_public_h::EV_USE_ITEM10;
pub use crate::bg_public_h::EV_USE_ITEM11;
pub use crate::bg_public_h::EV_USE_ITEM12;
pub use crate::bg_public_h::EV_USE_ITEM13;
pub use crate::bg_public_h::EV_USE_ITEM14;
pub use crate::bg_public_h::EV_USE_ITEM15;
pub use crate::bg_public_h::EV_USE_ITEM2;
pub use crate::bg_public_h::EV_USE_ITEM3;
pub use crate::bg_public_h::EV_USE_ITEM4;
pub use crate::bg_public_h::EV_USE_ITEM5;
pub use crate::bg_public_h::EV_USE_ITEM6;
pub use crate::bg_public_h::EV_USE_ITEM7;
pub use crate::bg_public_h::EV_USE_ITEM8;
pub use crate::bg_public_h::EV_USE_ITEM9;
pub use crate::bg_public_h::EV_WATER_CLEAR;
pub use crate::bg_public_h::EV_WATER_LEAVE;
pub use crate::bg_public_h::EV_WATER_TOUCH;
pub use crate::bg_public_h::EV_WATER_UNDER;
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
pub use crate::g_public_h::entityShared_t;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectory;
pub use crate::src::game::g_mover::q_shared_h::VectorInverse;
pub use crate::src::game::g_mover::q_shared_h::VectorLength;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AddPointToBounds;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::RadiusFromBounds;
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
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

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
pub use crate::src::game::g_combat::G_Damage;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::g_gravity;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_Error;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_main::G_RunThink;
pub use crate::src::game::g_misc::TeleportPlayer;
pub use crate::src::game::g_spawn::G_SpawnFloat;
pub use crate::src::game::g_spawn::G_SpawnInt;
pub use crate::src::game::g_spawn::G_SpawnString;
pub use crate::src::game::g_spawn::G_SpawnVector;
pub use crate::src::game::g_syscalls::trap_AdjustAreaPortalState;
pub use crate::src::game::g_syscalls::trap_EntitiesInBox;
pub use crate::src::game::g_syscalls::trap_LinkEntity;
pub use crate::src::game::g_syscalls::trap_SetBrushModel;
pub use crate::src::game::g_syscalls::trap_Trace;
pub use crate::src::game::g_syscalls::trap_UnlinkEntity;

pub use crate::src::game::g_utils::tv;
pub use crate::src::game::g_utils::vtos;
pub use crate::src::game::g_utils::G_AddEvent;
pub use crate::src::game::g_utils::G_Find;
pub use crate::src::game::g_utils::G_FreeEntity;
pub use crate::src::game::g_utils::G_ModelIndex;
pub use crate::src::game::g_utils::G_SetMovedir;
pub use crate::src::game::g_utils::G_SoundIndex;
pub use crate::src::game::g_utils::G_Spawn;
pub use crate::src::game::g_utils::G_TempEntity;
pub use crate::src::game::g_utils::G_UseTargets;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pushed_t {
    pub ent: *mut crate::g_local_h::gentity_t,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub angles: crate::src::qcommon::q_shared::vec3_t,
    pub deltayaw: f32,
}
#[no_mangle]

pub static mut pushed: [pushed_t; 1024] = [pushed_t {
    ent: 0 as *const crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_t,
    origin: [0.; 3],
    angles: [0.; 3],
    deltayaw: 0.,
}; 1024];
#[no_mangle]

pub static mut pushed_p: *mut pushed_t = 0 as *const pushed_t as *mut pushed_t;
/*
============
G_TestEntityPosition

============
*/
#[no_mangle]

pub unsafe extern "C" fn G_TestEntityPosition(
    mut ent: *mut crate::g_local_h::gentity_t,
) -> *mut crate::g_local_h::gentity_t {
    let mut tr: crate::src::qcommon::q_shared::trace_t = crate::src::qcommon::q_shared::trace_t {
        allsolid: crate::src::qcommon::q_shared::qfalse,
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: crate::src::qcommon::q_shared::cplane_t {
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
    let mut mask: i32 = 0;
    if (*ent).clipmask != 0 {
        mask = (*ent).clipmask
    } else {
        mask = 1 as i32
    }
    if !(*ent).client.is_null() {
        crate::src::game::g_syscalls::trap_Trace(
            &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
            (*(*ent).client).ps.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).r.mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).r.maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*(*ent).client).ps.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).s.number,
            mask,
        );
    } else {
        crate::src::game::g_syscalls::trap_Trace(
            &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
            (*ent).s.pos.trBase.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).r.mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).r.maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).s.pos.trBase.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).s.number,
            mask,
        );
    }
    if tr.startsolid as u64 != 0 {
        return &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(tr.entityNum as isize) as *mut crate::g_local_h::gentity_t;
    }
    return 0 as *mut crate::g_local_h::gentity_t;
}
/*
================
G_CreateRotationMatrix
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_CreateRotationMatrix(
    mut angles: *mut crate::src::qcommon::q_shared::vec_t,
    mut matrix: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    crate::src::qcommon::q_math::AngleVectors(
        angles as *const crate::src::qcommon::q_shared::vec_t,
        (*matrix.offset(0 as i32 as isize)).as_mut_ptr(),
        (*matrix.offset(1 as i32 as isize)).as_mut_ptr(),
        (*matrix.offset(2 as i32 as isize)).as_mut_ptr(),
    );
    VectorInverse((*matrix.offset(1 as i32 as isize)).as_mut_ptr());
}
/*
================
G_TransposeMatrix
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_TransposeMatrix(
    mut matrix: *mut crate::src::qcommon::q_shared::vec3_t,
    mut transpose: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0 as i32;
    while i < 3 as i32 {
        j = 0 as i32;
        while j < 3 as i32 {
            (*transpose.offset(i as isize))[j as usize] = (*matrix.offset(j as isize))[i as usize];
            j += 1
        }
        i += 1
    }
}
/*
================
G_RotatePoint
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_RotatePoint(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut matrix: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    let mut tvec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    tvec[0 as i32 as usize] = *point.offset(0 as i32 as isize);
    tvec[1 as i32 as usize] = *point.offset(1 as i32 as isize);
    tvec[2 as i32 as usize] = *point.offset(2 as i32 as isize);
    *point.offset(0 as i32 as isize) = (*matrix.offset(0 as i32 as isize))[0 as i32 as usize]
        * tvec[0 as i32 as usize]
        + (*matrix.offset(0 as i32 as isize))[1 as i32 as usize] * tvec[1 as i32 as usize]
        + (*matrix.offset(0 as i32 as isize))[2 as i32 as usize] * tvec[2 as i32 as usize];
    *point.offset(1 as i32 as isize) = (*matrix.offset(1 as i32 as isize))[0 as i32 as usize]
        * tvec[0 as i32 as usize]
        + (*matrix.offset(1 as i32 as isize))[1 as i32 as usize] * tvec[1 as i32 as usize]
        + (*matrix.offset(1 as i32 as isize))[2 as i32 as usize] * tvec[2 as i32 as usize];
    *point.offset(2 as i32 as isize) = (*matrix.offset(2 as i32 as isize))[0 as i32 as usize]
        * tvec[0 as i32 as usize]
        + (*matrix.offset(2 as i32 as isize))[1 as i32 as usize] * tvec[1 as i32 as usize]
        + (*matrix.offset(2 as i32 as isize))[2 as i32 as usize] * tvec[2 as i32 as usize];
}
/*
==================
G_TryPushingEntity

Returns qfalse if the move is blocked
==================
*/
#[no_mangle]

pub unsafe extern "C" fn G_TryPushingEntity(
    mut check: *mut crate::g_local_h::gentity_t,
    mut pusher: *mut crate::g_local_h::gentity_t,
    mut move_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut amove: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut matrix: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    let mut transpose: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut org2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut move2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut block: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    // EF_MOVER_STOP will just stop when contacting another entity
    // instead of pushing it, but entities can still ride on top of it
    if (*pusher).s.eFlags & 0x400 as i32 != 0 && (*check).s.groundEntityNum != (*pusher).s.number {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // save off the old position
    if pushed_p
        > &mut *pushed
            .as_mut_ptr()
            .offset(((1 as i32) << 10 as i32) as isize) as *mut pushed_t
    {
        crate::src::game::g_main::G_Error(
            b"pushed_p > &pushed[MAX_GENTITIES]\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*pushed_p).ent = check;
    (*pushed_p).origin[0 as i32 as usize] = (*check).s.pos.trBase[0 as i32 as usize];
    (*pushed_p).origin[1 as i32 as usize] = (*check).s.pos.trBase[1 as i32 as usize];
    (*pushed_p).origin[2 as i32 as usize] = (*check).s.pos.trBase[2 as i32 as usize];
    (*pushed_p).angles[0 as i32 as usize] = (*check).s.apos.trBase[0 as i32 as usize];
    (*pushed_p).angles[1 as i32 as usize] = (*check).s.apos.trBase[1 as i32 as usize];
    (*pushed_p).angles[2 as i32 as usize] = (*check).s.apos.trBase[2 as i32 as usize];
    if !(*check).client.is_null() {
        (*pushed_p).deltayaw = (*(*check).client).ps.delta_angles[1 as i32 as usize] as f32;
        (*pushed_p).origin[0 as i32 as usize] = (*(*check).client).ps.origin[0 as i32 as usize];
        (*pushed_p).origin[1 as i32 as usize] = (*(*check).client).ps.origin[1 as i32 as usize];
        (*pushed_p).origin[2 as i32 as usize] = (*(*check).client).ps.origin[2 as i32 as usize]
    }
    pushed_p = pushed_p.offset(1);
    // try moving the contacted entity
    // figure movement due to the pusher's amove
    G_CreateRotationMatrix(amove, transpose.as_mut_ptr());
    G_TransposeMatrix(transpose.as_mut_ptr(), matrix.as_mut_ptr());
    if !(*check).client.is_null() {
        org[0 as i32 as usize] = (*(*check).client).ps.origin[0 as i32 as usize]
            - (*pusher).r.currentOrigin[0 as i32 as usize];
        org[1 as i32 as usize] = (*(*check).client).ps.origin[1 as i32 as usize]
            - (*pusher).r.currentOrigin[1 as i32 as usize];
        org[2 as i32 as usize] = (*(*check).client).ps.origin[2 as i32 as usize]
            - (*pusher).r.currentOrigin[2 as i32 as usize]
    } else {
        org[0 as i32 as usize] =
            (*check).s.pos.trBase[0 as i32 as usize] - (*pusher).r.currentOrigin[0 as i32 as usize];
        org[1 as i32 as usize] =
            (*check).s.pos.trBase[1 as i32 as usize] - (*pusher).r.currentOrigin[1 as i32 as usize];
        org[2 as i32 as usize] =
            (*check).s.pos.trBase[2 as i32 as usize] - (*pusher).r.currentOrigin[2 as i32 as usize]
    }
    org2[0 as i32 as usize] = org[0 as i32 as usize];
    org2[1 as i32 as usize] = org[1 as i32 as usize];
    org2[2 as i32 as usize] = org[2 as i32 as usize];
    G_RotatePoint(org2.as_mut_ptr(), matrix.as_mut_ptr());
    move2[0 as i32 as usize] = org2[0 as i32 as usize] - org[0 as i32 as usize];
    move2[1 as i32 as usize] = org2[1 as i32 as usize] - org[1 as i32 as usize];
    move2[2 as i32 as usize] = org2[2 as i32 as usize] - org[2 as i32 as usize];
    // add movement
    (*check).s.pos.trBase[0 as i32 as usize] =
        (*check).s.pos.trBase[0 as i32 as usize] + *move_0.offset(0 as i32 as isize);
    (*check).s.pos.trBase[1 as i32 as usize] =
        (*check).s.pos.trBase[1 as i32 as usize] + *move_0.offset(1 as i32 as isize);
    (*check).s.pos.trBase[2 as i32 as usize] =
        (*check).s.pos.trBase[2 as i32 as usize] + *move_0.offset(2 as i32 as isize);
    (*check).s.pos.trBase[0 as i32 as usize] =
        (*check).s.pos.trBase[0 as i32 as usize] + move2[0 as i32 as usize];
    (*check).s.pos.trBase[1 as i32 as usize] =
        (*check).s.pos.trBase[1 as i32 as usize] + move2[1 as i32 as usize];
    (*check).s.pos.trBase[2 as i32 as usize] =
        (*check).s.pos.trBase[2 as i32 as usize] + move2[2 as i32 as usize];
    if !(*check).client.is_null() {
        (*(*check).client).ps.origin[0 as i32 as usize] =
            (*(*check).client).ps.origin[0 as i32 as usize] + *move_0.offset(0 as i32 as isize);
        (*(*check).client).ps.origin[1 as i32 as usize] =
            (*(*check).client).ps.origin[1 as i32 as usize] + *move_0.offset(1 as i32 as isize);
        (*(*check).client).ps.origin[2 as i32 as usize] =
            (*(*check).client).ps.origin[2 as i32 as usize] + *move_0.offset(2 as i32 as isize);
        (*(*check).client).ps.origin[0 as i32 as usize] =
            (*(*check).client).ps.origin[0 as i32 as usize] + move2[0 as i32 as usize];
        (*(*check).client).ps.origin[1 as i32 as usize] =
            (*(*check).client).ps.origin[1 as i32 as usize] + move2[1 as i32 as usize];
        (*(*check).client).ps.origin[2 as i32 as usize] =
            (*(*check).client).ps.origin[2 as i32 as usize] + move2[2 as i32 as usize];
        // make sure the client's view rotates when on a rotating mover
        (*(*check).client).ps.delta_angles[1 as i32 as usize] +=
            (*amove.offset(1 as i32 as isize) * 65536 as i32 as f32 / 360 as i32 as f32) as i32
                & 65535 as i32
    }
    // may have pushed them off an edge
    if (*check).s.groundEntityNum != (*pusher).s.number {
        (*check).s.groundEntityNum = ((1 as i32) << 10 as i32) - 1 as i32
    }
    block = G_TestEntityPosition(check);
    if block.is_null() {
        // pushed ok
        if !(*check).client.is_null() {
            (*check).r.currentOrigin[0 as i32 as usize] =
                (*(*check).client).ps.origin[0 as i32 as usize];
            (*check).r.currentOrigin[1 as i32 as usize] =
                (*(*check).client).ps.origin[1 as i32 as usize];
            (*check).r.currentOrigin[2 as i32 as usize] =
                (*(*check).client).ps.origin[2 as i32 as usize]
        } else {
            (*check).r.currentOrigin[0 as i32 as usize] = (*check).s.pos.trBase[0 as i32 as usize];
            (*check).r.currentOrigin[1 as i32 as usize] = (*check).s.pos.trBase[1 as i32 as usize];
            (*check).r.currentOrigin[2 as i32 as usize] = (*check).s.pos.trBase[2 as i32 as usize]
        }
        crate::src::game::g_syscalls::trap_LinkEntity(check as *mut crate::g_local_h::gentity_s);
        return crate::src::qcommon::q_shared::qtrue;
    }
    // if it is ok to leave in the old position, do it
    // this is only relevant for riding entities, not pushed
    // Sliding trapdoors can cause this.
    (*check).s.pos.trBase[0 as i32 as usize] =
        (*pushed_p.offset(-(1 as i32 as isize))).origin[0 as i32 as usize];
    (*check).s.pos.trBase[1 as i32 as usize] =
        (*pushed_p.offset(-(1 as i32 as isize))).origin[1 as i32 as usize];
    (*check).s.pos.trBase[2 as i32 as usize] =
        (*pushed_p.offset(-(1 as i32 as isize))).origin[2 as i32 as usize];
    if !(*check).client.is_null() {
        (*(*check).client).ps.origin[0 as i32 as usize] =
            (*pushed_p.offset(-(1 as i32 as isize))).origin[0 as i32 as usize];
        (*(*check).client).ps.origin[1 as i32 as usize] =
            (*pushed_p.offset(-(1 as i32 as isize))).origin[1 as i32 as usize];
        (*(*check).client).ps.origin[2 as i32 as usize] =
            (*pushed_p.offset(-(1 as i32 as isize))).origin[2 as i32 as usize]
    }
    (*check).s.apos.trBase[0 as i32 as usize] =
        (*pushed_p.offset(-(1 as i32 as isize))).angles[0 as i32 as usize];
    (*check).s.apos.trBase[1 as i32 as usize] =
        (*pushed_p.offset(-(1 as i32 as isize))).angles[1 as i32 as usize];
    (*check).s.apos.trBase[2 as i32 as usize] =
        (*pushed_p.offset(-(1 as i32 as isize))).angles[2 as i32 as usize];
    block = G_TestEntityPosition(check);
    if block.is_null() {
        (*check).s.groundEntityNum = ((1 as i32) << 10 as i32) - 1 as i32;
        pushed_p = pushed_p.offset(-1);
        return crate::src::qcommon::q_shared::qtrue;
    }
    // blocked
    return crate::src::qcommon::q_shared::qfalse;
}
/*
==================
G_CheckProxMinePosition
==================
*/
#[no_mangle]

pub unsafe extern "C" fn G_CheckProxMinePosition(
    mut check: *mut crate::g_local_h::gentity_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut tr: crate::src::qcommon::q_shared::trace_t = crate::src::qcommon::q_shared::trace_t {
        allsolid: crate::src::qcommon::q_shared::qfalse,
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: crate::src::qcommon::q_shared::cplane_t {
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
    start[0 as i32 as usize] = ((*check).s.pos.trBase[0 as i32 as usize] as f64
        + (*check).movedir[0 as i32 as usize] as f64 * 0.125f64)
        as crate::src::qcommon::q_shared::vec_t;
    start[1 as i32 as usize] = ((*check).s.pos.trBase[1 as i32 as usize] as f64
        + (*check).movedir[1 as i32 as usize] as f64 * 0.125f64)
        as crate::src::qcommon::q_shared::vec_t;
    start[2 as i32 as usize] = ((*check).s.pos.trBase[2 as i32 as usize] as f64
        + (*check).movedir[2 as i32 as usize] as f64 * 0.125f64)
        as crate::src::qcommon::q_shared::vec_t;
    end[0 as i32 as usize] = (*check).s.pos.trBase[0 as i32 as usize]
        + (*check).movedir[0 as i32 as usize] * 2 as i32 as f32;
    end[1 as i32 as usize] = (*check).s.pos.trBase[1 as i32 as usize]
        + (*check).movedir[1 as i32 as usize] * 2 as i32 as f32;
    end[2 as i32 as usize] = (*check).s.pos.trBase[2 as i32 as usize]
        + (*check).movedir[2 as i32 as usize] * 2 as i32 as f32;
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        start.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*check).s.number,
        1 as i32,
    );
    if tr.startsolid as u32 != 0 || tr.fraction < 1 as i32 as f32 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
==================
G_TryPushingProxMine
==================
*/
#[no_mangle]

pub unsafe extern "C" fn G_TryPushingProxMine(
    mut check: *mut crate::g_local_h::gentity_t,
    mut pusher: *mut crate::g_local_h::gentity_t,
    mut move_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut amove: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut org2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut move2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut ret: i32 = 0;
    // we need this for pushing things later
    org[0 as i32 as usize] = crate::src::qcommon::q_math::vec3_origin[0 as i32 as usize]
        - *amove.offset(0 as i32 as isize);
    org[1 as i32 as usize] = crate::src::qcommon::q_math::vec3_origin[1 as i32 as usize]
        - *amove.offset(1 as i32 as isize);
    org[2 as i32 as usize] = crate::src::qcommon::q_math::vec3_origin[2 as i32 as usize]
        - *amove.offset(2 as i32 as isize);
    crate::src::qcommon::q_math::AngleVectors(
        org.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    // try moving the contacted entity
    (*check).s.pos.trBase[0 as i32 as usize] =
        (*check).s.pos.trBase[0 as i32 as usize] + *move_0.offset(0 as i32 as isize);
    (*check).s.pos.trBase[1 as i32 as usize] =
        (*check).s.pos.trBase[1 as i32 as usize] + *move_0.offset(1 as i32 as isize);
    (*check).s.pos.trBase[2 as i32 as usize] =
        (*check).s.pos.trBase[2 as i32 as usize] + *move_0.offset(2 as i32 as isize);
    // figure movement due to the pusher's amove
    org[0 as i32 as usize] =
        (*check).s.pos.trBase[0 as i32 as usize] - (*pusher).r.currentOrigin[0 as i32 as usize];
    org[1 as i32 as usize] =
        (*check).s.pos.trBase[1 as i32 as usize] - (*pusher).r.currentOrigin[1 as i32 as usize];
    org[2 as i32 as usize] =
        (*check).s.pos.trBase[2 as i32 as usize] - (*pusher).r.currentOrigin[2 as i32 as usize];
    org2[0 as i32 as usize] = org[0 as i32 as usize] * forward[0 as i32 as usize]
        + org[1 as i32 as usize] * forward[1 as i32 as usize]
        + org[2 as i32 as usize] * forward[2 as i32 as usize];
    org2[1 as i32 as usize] = -(org[0 as i32 as usize] * right[0 as i32 as usize]
        + org[1 as i32 as usize] * right[1 as i32 as usize]
        + org[2 as i32 as usize] * right[2 as i32 as usize]);
    org2[2 as i32 as usize] = org[0 as i32 as usize] * up[0 as i32 as usize]
        + org[1 as i32 as usize] * up[1 as i32 as usize]
        + org[2 as i32 as usize] * up[2 as i32 as usize];
    move2[0 as i32 as usize] = org2[0 as i32 as usize] - org[0 as i32 as usize];
    move2[1 as i32 as usize] = org2[1 as i32 as usize] - org[1 as i32 as usize];
    move2[2 as i32 as usize] = org2[2 as i32 as usize] - org[2 as i32 as usize];
    (*check).s.pos.trBase[0 as i32 as usize] =
        (*check).s.pos.trBase[0 as i32 as usize] + move2[0 as i32 as usize];
    (*check).s.pos.trBase[1 as i32 as usize] =
        (*check).s.pos.trBase[1 as i32 as usize] + move2[1 as i32 as usize];
    (*check).s.pos.trBase[2 as i32 as usize] =
        (*check).s.pos.trBase[2 as i32 as usize] + move2[2 as i32 as usize];
    ret = G_CheckProxMinePosition(check) as i32;
    if ret != 0 {
        (*check).r.currentOrigin[0 as i32 as usize] = (*check).s.pos.trBase[0 as i32 as usize];
        (*check).r.currentOrigin[1 as i32 as usize] = (*check).s.pos.trBase[1 as i32 as usize];
        (*check).r.currentOrigin[2 as i32 as usize] = (*check).s.pos.trBase[2 as i32 as usize];
        crate::src::game::g_syscalls::trap_LinkEntity(check as *mut crate::g_local_h::gentity_s);
    }
    return ret as crate::src::qcommon::q_shared::qboolean;
}
/*
============
G_MoverPush

Objects need to be moved back on a failed push,
otherwise riders would continue to slide.
If qfalse is returned, *obstacle will be the blocking entity
============
*/
#[no_mangle]

pub unsafe extern "C" fn G_MoverPush(
    mut pusher: *mut crate::g_local_h::gentity_t,
    mut move_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut amove: *mut crate::src::qcommon::q_shared::vec_t,
    mut obstacle: *mut *mut crate::g_local_h::gentity_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    let mut e: i32 = 0;
    let mut check: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p: *mut pushed_t = 0 as *mut pushed_t;
    let mut entityList: [i32; 1024] = [0; 1024];
    let mut listedEntities: i32 = 0;
    let mut totalMins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut totalMaxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    *obstacle = 0 as *mut crate::g_local_h::gentity_t;
    // mins/maxs are the bounds at the destination
    // totalMins / totalMaxs are the bounds for the entire move
    if (*pusher).r.currentAngles[0 as i32 as usize] != 0.
        || (*pusher).r.currentAngles[1 as i32 as usize] != 0.
        || (*pusher).r.currentAngles[2 as i32 as usize] != 0.
        || *amove.offset(0 as i32 as isize) != 0.
        || *amove.offset(1 as i32 as isize) != 0.
        || *amove.offset(2 as i32 as isize) != 0.
    {
        let mut radius: f32 = 0.;
        radius = crate::src::qcommon::q_math::RadiusFromBounds(
            (*pusher).r.mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*pusher).r.maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        );
        i = 0 as i32;
        while i < 3 as i32 {
            mins[i as usize] =
                (*pusher).r.currentOrigin[i as usize] + *move_0.offset(i as isize) - radius;
            maxs[i as usize] =
                (*pusher).r.currentOrigin[i as usize] + *move_0.offset(i as isize) + radius;
            totalMins[i as usize] = mins[i as usize] - *move_0.offset(i as isize);
            totalMaxs[i as usize] = maxs[i as usize] - *move_0.offset(i as isize);
            i += 1
        }
    } else {
        i = 0 as i32;
        while i < 3 as i32 {
            mins[i as usize] = (*pusher).r.absmin[i as usize] + *move_0.offset(i as isize);
            maxs[i as usize] = (*pusher).r.absmax[i as usize] + *move_0.offset(i as isize);
            i += 1
        }
        totalMins[0 as i32 as usize] = (*pusher).r.absmin[0 as i32 as usize];
        totalMins[1 as i32 as usize] = (*pusher).r.absmin[1 as i32 as usize];
        totalMins[2 as i32 as usize] = (*pusher).r.absmin[2 as i32 as usize];
        totalMaxs[0 as i32 as usize] = (*pusher).r.absmax[0 as i32 as usize];
        totalMaxs[1 as i32 as usize] = (*pusher).r.absmax[1 as i32 as usize];
        totalMaxs[2 as i32 as usize] = (*pusher).r.absmax[2 as i32 as usize];
        i = 0 as i32;
        while i < 3 as i32 {
            if *move_0.offset(i as isize) > 0 as i32 as f32 {
                totalMaxs[i as usize] += *move_0.offset(i as isize)
            } else {
                totalMins[i as usize] += *move_0.offset(i as isize)
            }
            i += 1
        }
    }
    // unlink the pusher so we don't get it in the entityList
    crate::src::game::g_syscalls::trap_UnlinkEntity(pusher as *mut crate::g_local_h::gentity_s);
    listedEntities = crate::src::game::g_syscalls::trap_EntitiesInBox(
        totalMins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        totalMaxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        entityList.as_mut_ptr(),
        (1 as i32) << 10 as i32,
    );
    // move the pusher to its final position
    (*pusher).r.currentOrigin[0 as i32 as usize] =
        (*pusher).r.currentOrigin[0 as i32 as usize] + *move_0.offset(0 as i32 as isize);
    (*pusher).r.currentOrigin[1 as i32 as usize] =
        (*pusher).r.currentOrigin[1 as i32 as usize] + *move_0.offset(1 as i32 as isize);
    (*pusher).r.currentOrigin[2 as i32 as usize] =
        (*pusher).r.currentOrigin[2 as i32 as usize] + *move_0.offset(2 as i32 as isize);
    (*pusher).r.currentAngles[0 as i32 as usize] =
        (*pusher).r.currentAngles[0 as i32 as usize] + *amove.offset(0 as i32 as isize);
    (*pusher).r.currentAngles[1 as i32 as usize] =
        (*pusher).r.currentAngles[1 as i32 as usize] + *amove.offset(1 as i32 as isize);
    (*pusher).r.currentAngles[2 as i32 as usize] =
        (*pusher).r.currentAngles[2 as i32 as usize] + *amove.offset(2 as i32 as isize);
    crate::src::game::g_syscalls::trap_LinkEntity(pusher as *mut crate::g_local_h::gentity_s);
    let mut current_block_46: u64;
    // see if any solid entities are inside the final position
    e = 0 as i32;
    while e < listedEntities {
        check = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(*entityList.as_mut_ptr().offset(e as isize) as isize)
            as *mut crate::g_local_h::gentity_t;
        // only push items and players
        if !((*check).s.eType != crate::bg_public_h::ET_ITEM as i32
            && (*check).s.eType != crate::bg_public_h::ET_PLAYER as i32
            && (*check).physicsObject as u64 == 0)
        {
            // if the entity is standing on the pusher, it will definitely be moved
            if (*check).s.groundEntityNum != (*pusher).s.number {
                // see if the ent needs to be tested
                if (*check).r.absmin[0 as i32 as usize] >= maxs[0 as i32 as usize]
                    || (*check).r.absmin[1 as i32 as usize] >= maxs[1 as i32 as usize]
                    || (*check).r.absmin[2 as i32 as usize] >= maxs[2 as i32 as usize]
                    || (*check).r.absmax[0 as i32 as usize] <= mins[0 as i32 as usize]
                    || (*check).r.absmax[1 as i32 as usize] <= mins[1 as i32 as usize]
                    || (*check).r.absmax[2 as i32 as usize] <= mins[2 as i32 as usize]
                {
                    current_block_46 = 9520865839495247062;
                } else if G_TestEntityPosition(check).is_null() {
                    current_block_46 = 9520865839495247062;
                } else {
                    current_block_46 = 790185930182612747;
                }
            } else {
                current_block_46 = 790185930182612747;
            }
            match current_block_46 {
                9520865839495247062 => {}
                _ =>
                // see if the ent's bbox is inside the pusher's final position
                // this does allow a fast moving object to pass through a thin entity...
                // the entity needs to be pushed
                {
                    if !(G_TryPushingEntity(check, pusher, move_0, amove) as u64 != 0) {
                        // the move was blocked an entity
                        // bobbing entities are instant-kill and never get blocked
                        if (*pusher).s.pos.trType as u32
                            == crate::src::qcommon::q_shared::TR_SINE as i32 as u32
                            || (*pusher).s.apos.trType as u32
                                == crate::src::qcommon::q_shared::TR_SINE as i32 as u32
                        {
                            crate::src::game::g_combat::G_Damage(
                                check as *mut crate::g_local_h::gentity_s,
                                pusher as *mut crate::g_local_h::gentity_s,
                                pusher as *mut crate::g_local_h::gentity_s,
                                0 as *mut crate::src::qcommon::q_shared::vec_t,
                                0 as *mut crate::src::qcommon::q_shared::vec_t,
                                99999 as i32,
                                0 as i32,
                                crate::bg_public_h::MOD_CRUSH as i32,
                            );
                        } else {
                            // save off the obstacle so we can call the block function (crush, etc)
                            *obstacle = check;
                            // move back any entities we already moved
                            // go backwards, so if the same entity was pushed
                            // twice, it goes back to the original position
                            p = pushed_p.offset(-(1 as i32 as isize));
                            while p >= pushed.as_mut_ptr() {
                                (*(*p).ent).s.pos.trBase[0 as i32 as usize] =
                                    (*p).origin[0 as i32 as usize];
                                (*(*p).ent).s.pos.trBase[1 as i32 as usize] =
                                    (*p).origin[1 as i32 as usize];
                                (*(*p).ent).s.pos.trBase[2 as i32 as usize] =
                                    (*p).origin[2 as i32 as usize];
                                (*(*p).ent).s.apos.trBase[0 as i32 as usize] =
                                    (*p).angles[0 as i32 as usize];
                                (*(*p).ent).s.apos.trBase[1 as i32 as usize] =
                                    (*p).angles[1 as i32 as usize];
                                (*(*p).ent).s.apos.trBase[2 as i32 as usize] =
                                    (*p).angles[2 as i32 as usize];
                                if !(*(*p).ent).client.is_null() {
                                    (*(*(*p).ent).client).ps.delta_angles[1 as i32 as usize] =
                                        (*p).deltayaw as i32;
                                    (*(*(*p).ent).client).ps.origin[0 as i32 as usize] =
                                        (*p).origin[0 as i32 as usize];
                                    (*(*(*p).ent).client).ps.origin[1 as i32 as usize] =
                                        (*p).origin[1 as i32 as usize];
                                    (*(*(*p).ent).client).ps.origin[2 as i32 as usize] =
                                        (*p).origin[2 as i32 as usize]
                                }
                                crate::src::game::g_syscalls::trap_LinkEntity(
                                    (*p).ent as *mut crate::g_local_h::gentity_s,
                                );
                                p = p.offset(-1)
                            }
                            return crate::src::qcommon::q_shared::qfalse;
                        }
                    }
                }
            }
        }
        e += 1
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=================
G_MoverTeam
=================
*/
#[no_mangle]

pub unsafe extern "C" fn G_MoverTeam(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut move_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut amove: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut part: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut obstacle: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    obstacle = 0 as *mut crate::g_local_h::gentity_t;
    // make sure all team slaves can move before committing
    // any moves or calling any think functions
    // if the move is blocked, all moved objects will be backed out
    pushed_p = pushed.as_mut_ptr();
    part = ent;
    while !part.is_null() {
        // get current position
        crate::src::game::bg_misc::BG_EvaluateTrajectory(
            &mut (*part).s.pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
            crate::src::game::g_main::level.time,
            origin.as_mut_ptr(),
        );
        crate::src::game::bg_misc::BG_EvaluateTrajectory(
            &mut (*part).s.apos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
            crate::src::game::g_main::level.time,
            angles.as_mut_ptr(),
        );
        move_0[0 as i32 as usize] =
            origin[0 as i32 as usize] - (*part).r.currentOrigin[0 as i32 as usize];
        move_0[1 as i32 as usize] =
            origin[1 as i32 as usize] - (*part).r.currentOrigin[1 as i32 as usize];
        move_0[2 as i32 as usize] =
            origin[2 as i32 as usize] - (*part).r.currentOrigin[2 as i32 as usize];
        amove[0 as i32 as usize] =
            angles[0 as i32 as usize] - (*part).r.currentAngles[0 as i32 as usize];
        amove[1 as i32 as usize] =
            angles[1 as i32 as usize] - (*part).r.currentAngles[1 as i32 as usize];
        amove[2 as i32 as usize] =
            angles[2 as i32 as usize] - (*part).r.currentAngles[2 as i32 as usize];
        if G_MoverPush(part, move_0.as_mut_ptr(), amove.as_mut_ptr(), &mut obstacle) as u64 == 0 {
            break;
        }
        part = (*part).teamchain
    }
    if !part.is_null() {
        // go back to the previous position
        part = ent;
        while !part.is_null() {
            (*part).s.pos.trTime +=
                crate::src::game::g_main::level.time - crate::src::game::g_main::level.previousTime;
            (*part).s.apos.trTime +=
                crate::src::game::g_main::level.time - crate::src::game::g_main::level.previousTime;
            crate::src::game::bg_misc::BG_EvaluateTrajectory(
                &mut (*part).s.pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
                crate::src::game::g_main::level.time,
                (*part).r.currentOrigin.as_mut_ptr(),
            );
            crate::src::game::bg_misc::BG_EvaluateTrajectory(
                &mut (*part).s.apos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
                crate::src::game::g_main::level.time,
                (*part).r.currentAngles.as_mut_ptr(),
            );
            crate::src::game::g_syscalls::trap_LinkEntity(part as *mut crate::g_local_h::gentity_s);
            part = (*part).teamchain
        }
        // if the pusher has a "blocked" function, call it
        if (*ent).blocked.is_some() {
            (*ent).blocked.expect("non-null function pointer")(ent, obstacle);
        }
        return;
    }
    // the move succeeded
    part = ent;
    while !part.is_null() {
        // call the reached function if time is at or past end point
        if (*part).s.pos.trType as u32
            == crate::src::qcommon::q_shared::TR_LINEAR_STOP as i32 as u32
        {
            if crate::src::game::g_main::level.time
                >= (*part).s.pos.trTime + (*part).s.pos.trDuration
            {
                if (*part).reached.is_some() {
                    (*part).reached.expect("non-null function pointer")(part);
                }
            }
        }
        part = (*part).teamchain
    }
}
/*
================
G_RunMover

================
*/
#[no_mangle]

pub unsafe extern "C" fn G_RunMover(mut ent: *mut crate::g_local_h::gentity_t) {
    // if not a team captain, don't do anything, because
    // the captain will handle everything
    if (*ent).flags & 0x400 as i32 != 0 {
        return;
    }
    // if stationary at one of the positions, don't move anything
    if (*ent).s.pos.trType as u32 != crate::src::qcommon::q_shared::TR_STATIONARY as i32 as u32
        || (*ent).s.apos.trType as u32 != crate::src::qcommon::q_shared::TR_STATIONARY as i32 as u32
    {
        G_MoverTeam(ent);
    }
    // check think function
    crate::src::game::g_main::G_RunThink(ent as *mut crate::g_local_h::gentity_s);
}
/*
============================================================================

GENERAL MOVERS

Doors, plats, and buttons are all binary (two position) movers
Pos1 is "at rest", pos2 is "activated"
============================================================================
*/
/*
===============
SetMoverState
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SetMoverState(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut moverState: crate::g_local_h::moverState_t,
    mut time: i32,
) {
    let mut delta: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut f: f32 = 0.;
    (*ent).moverState = moverState;
    (*ent).s.pos.trTime = time;
    match moverState as u32 {
        0 => {
            (*ent).s.pos.trBase[0 as i32 as usize] = (*ent).pos1[0 as i32 as usize];
            (*ent).s.pos.trBase[1 as i32 as usize] = (*ent).pos1[1 as i32 as usize];
            (*ent).s.pos.trBase[2 as i32 as usize] = (*ent).pos1[2 as i32 as usize];
            (*ent).s.pos.trType = crate::src::qcommon::q_shared::TR_STATIONARY
        }
        1 => {
            (*ent).s.pos.trBase[0 as i32 as usize] = (*ent).pos2[0 as i32 as usize];
            (*ent).s.pos.trBase[1 as i32 as usize] = (*ent).pos2[1 as i32 as usize];
            (*ent).s.pos.trBase[2 as i32 as usize] = (*ent).pos2[2 as i32 as usize];
            (*ent).s.pos.trType = crate::src::qcommon::q_shared::TR_STATIONARY
        }
        2 => {
            (*ent).s.pos.trBase[0 as i32 as usize] = (*ent).pos1[0 as i32 as usize];
            (*ent).s.pos.trBase[1 as i32 as usize] = (*ent).pos1[1 as i32 as usize];
            (*ent).s.pos.trBase[2 as i32 as usize] = (*ent).pos1[2 as i32 as usize];
            delta[0 as i32 as usize] =
                (*ent).pos2[0 as i32 as usize] - (*ent).pos1[0 as i32 as usize];
            delta[1 as i32 as usize] =
                (*ent).pos2[1 as i32 as usize] - (*ent).pos1[1 as i32 as usize];
            delta[2 as i32 as usize] =
                (*ent).pos2[2 as i32 as usize] - (*ent).pos1[2 as i32 as usize];
            f = (1000.0f64 / (*ent).s.pos.trDuration as f64) as f32;
            (*ent).s.pos.trDelta[0 as i32 as usize] = delta[0 as i32 as usize] * f;
            (*ent).s.pos.trDelta[1 as i32 as usize] = delta[1 as i32 as usize] * f;
            (*ent).s.pos.trDelta[2 as i32 as usize] = delta[2 as i32 as usize] * f;
            (*ent).s.pos.trType = crate::src::qcommon::q_shared::TR_LINEAR_STOP
        }
        3 => {
            (*ent).s.pos.trBase[0 as i32 as usize] = (*ent).pos2[0 as i32 as usize];
            (*ent).s.pos.trBase[1 as i32 as usize] = (*ent).pos2[1 as i32 as usize];
            (*ent).s.pos.trBase[2 as i32 as usize] = (*ent).pos2[2 as i32 as usize];
            delta[0 as i32 as usize] =
                (*ent).pos1[0 as i32 as usize] - (*ent).pos2[0 as i32 as usize];
            delta[1 as i32 as usize] =
                (*ent).pos1[1 as i32 as usize] - (*ent).pos2[1 as i32 as usize];
            delta[2 as i32 as usize] =
                (*ent).pos1[2 as i32 as usize] - (*ent).pos2[2 as i32 as usize];
            f = (1000.0f64 / (*ent).s.pos.trDuration as f64) as f32;
            (*ent).s.pos.trDelta[0 as i32 as usize] = delta[0 as i32 as usize] * f;
            (*ent).s.pos.trDelta[1 as i32 as usize] = delta[1 as i32 as usize] * f;
            (*ent).s.pos.trDelta[2 as i32 as usize] = delta[2 as i32 as usize] * f;
            (*ent).s.pos.trType = crate::src::qcommon::q_shared::TR_LINEAR_STOP
        }
        _ => {}
    }
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*ent).s.pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
        crate::src::game::g_main::level.time,
        (*ent).r.currentOrigin.as_mut_ptr(),
    );
    crate::src::game::g_syscalls::trap_LinkEntity(ent as *mut crate::g_local_h::gentity_s);
}
/*
================
MatchTeam

All entities in a mover team will move from pos1 to pos2
in the same amount of time
================
*/
#[no_mangle]

pub unsafe extern "C" fn MatchTeam(
    mut teamLeader: *mut crate::g_local_h::gentity_t,
    mut moverState: i32,
    mut time: i32,
) {
    let mut slave: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    slave = teamLeader;
    while !slave.is_null() {
        SetMoverState(slave, moverState as crate::g_local_h::moverState_t, time);
        slave = (*slave).teamchain
    }
}
/*
================
ReturnToPos1
================
*/
#[no_mangle]

pub unsafe extern "C" fn ReturnToPos1(mut ent: *mut crate::g_local_h::gentity_t) {
    MatchTeam(
        ent,
        crate::g_local_h::MOVER_2TO1 as i32,
        crate::src::game::g_main::level.time,
    );
    // looping sound
    (*ent).s.loopSound = (*ent).soundLoop;
    // starting sound
    if (*ent).sound2to1 != 0 {
        crate::src::game::g_utils::G_AddEvent(
            ent as *mut crate::g_local_h::gentity_s,
            crate::bg_public_h::EV_GENERAL_SOUND as i32,
            (*ent).sound2to1,
        );
    };
}
/*
================
Reached_BinaryMover
================
*/
#[no_mangle]

pub unsafe extern "C" fn Reached_BinaryMover(mut ent: *mut crate::g_local_h::gentity_t) {
    // stop the looping sound
    (*ent).s.loopSound = (*ent).soundLoop;
    if (*ent).moverState as u32 == crate::g_local_h::MOVER_1TO2 as i32 as u32 {
        // reached pos2
        SetMoverState(
            ent,
            crate::g_local_h::MOVER_POS2,
            crate::src::game::g_main::level.time,
        );
        // play sound
        if (*ent).soundPos2 != 0 {
            crate::src::game::g_utils::G_AddEvent(
                ent as *mut crate::g_local_h::gentity_s,
                crate::bg_public_h::EV_GENERAL_SOUND as i32,
                (*ent).soundPos2,
            );
        }
        // return to pos1 after a delay
        (*ent).think =
            Some(ReturnToPos1 as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
        (*ent).nextthink = (crate::src::game::g_main::level.time as f32 + (*ent).wait) as i32;
        // fire targets
        if (*ent).activator.is_null() {
            (*ent).activator = ent
        }
        crate::src::game::g_utils::G_UseTargets(
            ent as *mut crate::g_local_h::gentity_s,
            (*ent).activator as *mut crate::g_local_h::gentity_s,
        );
    } else if (*ent).moverState as u32 == crate::g_local_h::MOVER_2TO1 as i32 as u32 {
        // reached pos1
        SetMoverState(
            ent,
            crate::g_local_h::MOVER_POS1,
            crate::src::game::g_main::level.time,
        );
        // play sound
        if (*ent).soundPos1 != 0 {
            crate::src::game::g_utils::G_AddEvent(
                ent as *mut crate::g_local_h::gentity_s,
                crate::bg_public_h::EV_GENERAL_SOUND as i32,
                (*ent).soundPos1,
            );
        }
        // close areaportals
        if (*ent).teammaster == ent || (*ent).teammaster.is_null() {
            crate::src::game::g_syscalls::trap_AdjustAreaPortalState(
                ent as *mut crate::g_local_h::gentity_s,
                crate::src::qcommon::q_shared::qfalse,
            );
        }
    } else {
        crate::src::game::g_main::G_Error(
            b"Reached_BinaryMover: bad moverState\x00" as *const u8 as *const libc::c_char,
        );
    };
}
/*
================
Use_BinaryMover
================
*/
#[no_mangle]

pub unsafe extern "C" fn Use_BinaryMover(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
    mut activator: *mut crate::g_local_h::gentity_t,
) {
    let mut total: i32 = 0;
    let mut partial: i32 = 0;
    // only the master should be used
    if (*ent).flags & 0x400 as i32 != 0 {
        Use_BinaryMover((*ent).teammaster, other, activator);
        return;
    }
    (*ent).activator = activator;
    if (*ent).moverState as u32 == crate::g_local_h::MOVER_POS1 as i32 as u32 {
        // start moving 50 msec later, becase if this was player
        // triggered, level.time hasn't been advanced yet
        MatchTeam(
            ent,
            crate::g_local_h::MOVER_1TO2 as i32,
            crate::src::game::g_main::level.time + 50 as i32,
        );
        // starting sound
        if (*ent).sound1to2 != 0 {
            crate::src::game::g_utils::G_AddEvent(
                ent as *mut crate::g_local_h::gentity_s,
                crate::bg_public_h::EV_GENERAL_SOUND as i32,
                (*ent).sound1to2,
            );
        }
        // looping sound
        (*ent).s.loopSound = (*ent).soundLoop;
        // open areaportal
        if (*ent).teammaster == ent || (*ent).teammaster.is_null() {
            crate::src::game::g_syscalls::trap_AdjustAreaPortalState(
                ent as *mut crate::g_local_h::gentity_s,
                crate::src::qcommon::q_shared::qtrue,
            );
        }
        return;
    }
    // if all the way up, just delay before coming down
    if (*ent).moverState as u32 == crate::g_local_h::MOVER_POS2 as i32 as u32 {
        (*ent).nextthink = (crate::src::game::g_main::level.time as f32 + (*ent).wait) as i32;
        return;
    }
    // only partway down before reversing
    if (*ent).moverState as u32 == crate::g_local_h::MOVER_2TO1 as i32 as u32 {
        total = (*ent).s.pos.trDuration;
        partial = crate::src::game::g_main::level.time - (*ent).s.pos.trTime;
        if partial > total {
            partial = total
        }
        MatchTeam(
            ent,
            crate::g_local_h::MOVER_1TO2 as i32,
            crate::src::game::g_main::level.time - (total - partial),
        );
        if (*ent).sound1to2 != 0 {
            crate::src::game::g_utils::G_AddEvent(
                ent as *mut crate::g_local_h::gentity_s,
                crate::bg_public_h::EV_GENERAL_SOUND as i32,
                (*ent).sound1to2,
            );
        }
        return;
    }
    // only partway up before reversing
    if (*ent).moverState as u32 == crate::g_local_h::MOVER_1TO2 as i32 as u32 {
        total = (*ent).s.pos.trDuration;
        partial = crate::src::game::g_main::level.time - (*ent).s.pos.trTime;
        if partial > total {
            partial = total
        }
        MatchTeam(
            ent,
            crate::g_local_h::MOVER_2TO1 as i32,
            crate::src::game::g_main::level.time - (total - partial),
        );
        if (*ent).sound2to1 != 0 {
            crate::src::game::g_utils::G_AddEvent(
                ent as *mut crate::g_local_h::gentity_s,
                crate::bg_public_h::EV_GENERAL_SOUND as i32,
                (*ent).sound2to1,
            );
        }
        return;
    };
}
/*
================
InitMover

"pos1", "pos2", and "speed" should be set before calling,
so the movement delta can be calculated
================
*/
#[no_mangle]

pub unsafe extern "C" fn InitMover(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut move_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut distance: f32 = 0.;
    let mut light: f32 = 0.;
    let mut color: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lightSet: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut colorSet: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut sound: *mut libc::c_char = 0 as *mut libc::c_char;
    // if the "model2" key is set, use a separate model
    // for drawing, but clip against the brushes
    if !(*ent).model2.is_null() {
        (*ent).s.modelindex2 = crate::src::game::g_utils::G_ModelIndex((*ent).model2)
    }
    // if the "loopsound" key is set, use a constant looping sound when moving
    if crate::src::game::g_spawn::G_SpawnString(
        b"noise\x00" as *const u8 as *const libc::c_char,
        b"100\x00" as *const u8 as *const libc::c_char,
        &mut sound,
    ) as u64
        != 0
    {
        (*ent).s.loopSound = crate::src::game::g_utils::G_SoundIndex(sound)
    }
    // if the "color" or "light" keys are set, setup constantLight
    lightSet = crate::src::game::g_spawn::G_SpawnFloat(
        b"light\x00" as *const u8 as *const libc::c_char,
        b"100\x00" as *const u8 as *const libc::c_char,
        &mut light,
    );
    colorSet = crate::src::game::g_spawn::G_SpawnVector(
        b"color\x00" as *const u8 as *const libc::c_char,
        b"1 1 1\x00" as *const u8 as *const libc::c_char,
        color.as_mut_ptr(),
    );
    if lightSet as u32 != 0 || colorSet as u32 != 0 {
        let mut r: i32 = 0;
        let mut g: i32 = 0;
        let mut b: i32 = 0;
        let mut i: i32 = 0;
        r = (color[0 as i32 as usize] * 255 as i32 as f32) as i32;
        if r > 255 as i32 {
            r = 255 as i32
        }
        g = (color[1 as i32 as usize] * 255 as i32 as f32) as i32;
        if g > 255 as i32 {
            g = 255 as i32
        }
        b = (color[2 as i32 as usize] * 255 as i32 as f32) as i32;
        if b > 255 as i32 {
            b = 255 as i32
        }
        i = (light / 4 as i32 as f32) as i32;
        if i > 255 as i32 {
            i = 255 as i32
        }
        (*ent).s.constantLight = r | g << 8 as i32 | b << 16 as i32 | i << 24 as i32
    }
    (*ent).use_0 = Some(
        Use_BinaryMover
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
    (*ent).reached = Some(
        Reached_BinaryMover as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
    );
    (*ent).moverState = crate::g_local_h::MOVER_POS1;
    (*ent).r.svFlags = 0x80 as i32;
    (*ent).s.eType = crate::bg_public_h::ET_MOVER as i32;
    (*ent).r.currentOrigin[0 as i32 as usize] = (*ent).pos1[0 as i32 as usize];
    (*ent).r.currentOrigin[1 as i32 as usize] = (*ent).pos1[1 as i32 as usize];
    (*ent).r.currentOrigin[2 as i32 as usize] = (*ent).pos1[2 as i32 as usize];
    crate::src::game::g_syscalls::trap_LinkEntity(ent as *mut crate::g_local_h::gentity_s);
    (*ent).s.pos.trType = crate::src::qcommon::q_shared::TR_STATIONARY;
    (*ent).s.pos.trBase[0 as i32 as usize] = (*ent).pos1[0 as i32 as usize];
    (*ent).s.pos.trBase[1 as i32 as usize] = (*ent).pos1[1 as i32 as usize];
    (*ent).s.pos.trBase[2 as i32 as usize] = (*ent).pos1[2 as i32 as usize];
    // calculate time to reach second position from speed
    move_0[0 as i32 as usize] = (*ent).pos2[0 as i32 as usize] - (*ent).pos1[0 as i32 as usize];
    move_0[1 as i32 as usize] = (*ent).pos2[1 as i32 as usize] - (*ent).pos1[1 as i32 as usize];
    move_0[2 as i32 as usize] = (*ent).pos2[2 as i32 as usize] - (*ent).pos1[2 as i32 as usize];
    distance = VectorLength(move_0.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    if (*ent).speed == 0. {
        (*ent).speed = 100 as i32 as f32
    }
    (*ent).s.pos.trDelta[0 as i32 as usize] = move_0[0 as i32 as usize] * (*ent).speed;
    (*ent).s.pos.trDelta[1 as i32 as usize] = move_0[1 as i32 as usize] * (*ent).speed;
    (*ent).s.pos.trDelta[2 as i32 as usize] = move_0[2 as i32 as usize] * (*ent).speed;
    (*ent).s.pos.trDuration = (distance * 1000 as i32 as f32 / (*ent).speed) as i32;
    if (*ent).s.pos.trDuration <= 0 as i32 {
        (*ent).s.pos.trDuration = 1 as i32
    };
}
/*
===============================================================================

DOOR

A use can be triggered either by a touch function, by being shot, or by being
targeted by another entity.

===============================================================================
*/
/*
================
Blocked_Door
================
*/
#[no_mangle]

pub unsafe extern "C" fn Blocked_Door(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
) {
    // remove anything other than a client
    if (*other).client.is_null() {
        // except CTF flags!!!!
        if (*other).s.eType == crate::bg_public_h::ET_ITEM as i32
            && (*(*other).item).giType as u32 == crate::bg_public_h::IT_TEAM as i32 as u32
        {
            crate::src::game::g_team::Team_DroppedFlagThink(
                other as *mut crate::g_local_h::gentity_s,
            );
            return;
        }

        crate::src::game::g_utils::G_TempEntity(
            (*other).s.origin.as_mut_ptr(),
            crate::bg_public_h::EV_ITEM_POP as i32,
        ) as *mut crate::g_local_h::gentity_s;
        crate::src::game::g_utils::G_FreeEntity(other as *mut crate::g_local_h::gentity_s);
        return;
    }
    if (*ent).damage != 0 {
        crate::src::game::g_combat::G_Damage(
            other as *mut crate::g_local_h::gentity_s,
            ent as *mut crate::g_local_h::gentity_s,
            ent as *mut crate::g_local_h::gentity_s,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            (*ent).damage,
            0 as i32,
            crate::bg_public_h::MOD_CRUSH as i32,
        );
    }
    if (*ent).spawnflags & 4 as i32 != 0 {
        return;
        // crushers don't reverse
    }
    // reverse direction
    Use_BinaryMover(ent, ent, other);
}
/*
================
Touch_DoorTriggerSpectator
================
*/

unsafe extern "C" fn Touch_DoorTriggerSpectator(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
    mut _trace: *mut crate::src::qcommon::q_shared::trace_t,
) {
    let mut axis: i32 = 0;
    let mut doorMin: f32 = 0.;
    let mut doorMax: f32 = 0.;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    axis = (*ent).count;
    // the constants below relate to constants in Think_SpawnNewDoorTrigger()
    doorMin = (*ent).r.absmin[axis as usize] + 100 as i32 as f32;
    doorMax = (*ent).r.absmax[axis as usize] - 100 as i32 as f32;
    origin[0 as i32 as usize] = (*(*other).client).ps.origin[0 as i32 as usize];
    origin[1 as i32 as usize] = (*(*other).client).ps.origin[1 as i32 as usize];
    origin[2 as i32 as usize] = (*(*other).client).ps.origin[2 as i32 as usize];
    if origin[axis as usize] < doorMin || origin[axis as usize] > doorMax {
        return;
    }
    if crate::stdlib::fabs((origin[axis as usize] - doorMax) as f64)
        < crate::stdlib::fabs((origin[axis as usize] - doorMin) as f64)
    {
        origin[axis as usize] = doorMin - 10 as i32 as f32
    } else {
        origin[axis as usize] = doorMax + 10 as i32 as f32
    }
    crate::src::game::g_misc::TeleportPlayer(
        other as *mut crate::g_local_h::gentity_s,
        origin.as_mut_ptr(),
        crate::src::game::g_utils::tv(10000000.0f64 as f32, 0 as i32 as f32, 0 as i32 as f32),
    );
}
/*
================
Touch_DoorTrigger
================
*/
#[no_mangle]

pub unsafe extern "C" fn Touch_DoorTrigger(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
    mut trace: *mut crate::src::qcommon::q_shared::trace_t,
) {
    if !(*other).client.is_null()
        && (*(*other).client).sess.sessionTeam as u32
            == crate::bg_public_h::TEAM_SPECTATOR as i32 as u32
    {
        // if the door is not open and not opening
        if (*(*ent).parent).moverState as u32 != crate::g_local_h::MOVER_1TO2 as i32 as u32
            && (*(*ent).parent).moverState as u32 != crate::g_local_h::MOVER_POS2 as i32 as u32
        {
            Touch_DoorTriggerSpectator(ent, other, trace);
        }
    } else if (*(*ent).parent).moverState as u32 != crate::g_local_h::MOVER_1TO2 as i32 as u32 {
        Use_BinaryMover((*ent).parent, ent, other);
    };
}
/*
======================
Think_SpawnNewDoorTrigger

All of the parts of a door have been spawned, so create
a trigger that encloses all of them
======================
*/
#[no_mangle]

pub unsafe extern "C" fn Think_SpawnNewDoorTrigger(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut other: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut i: i32 = 0;
    let mut best: i32 = 0;
    if ent.is_null() {
        return;
    }
    // set all of the slaves as shootable
    other = ent;
    while !other.is_null() {
        (*other).takedamage = crate::src::qcommon::q_shared::qtrue;
        other = (*other).teamchain
    }
    // find the bounds of everything on the team
    mins[0 as i32 as usize] = (*ent).r.absmin[0 as i32 as usize];
    mins[1 as i32 as usize] = (*ent).r.absmin[1 as i32 as usize];
    mins[2 as i32 as usize] = (*ent).r.absmin[2 as i32 as usize];
    maxs[0 as i32 as usize] = (*ent).r.absmax[0 as i32 as usize];
    maxs[1 as i32 as usize] = (*ent).r.absmax[1 as i32 as usize];
    maxs[2 as i32 as usize] = (*ent).r.absmax[2 as i32 as usize];
    other = (*ent).teamchain;
    while !other.is_null() {
        crate::src::qcommon::q_math::AddPointToBounds(
            (*other).r.absmin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        crate::src::qcommon::q_math::AddPointToBounds(
            (*other).r.absmax.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        other = (*other).teamchain
    }
    // find the thinnest axis, which will be the one we expand
    best = 0 as i32;
    i = 1 as i32;
    while i < 3 as i32 {
        if maxs[i as usize] - mins[i as usize] < maxs[best as usize] - mins[best as usize] {
            best = i
        }
        i += 1
    }
    maxs[best as usize] += 120 as i32 as f32;
    mins[best as usize] -= 120 as i32 as f32;
    // create a trigger with this size
    other = crate::src::game::g_utils::G_Spawn() as *mut crate::g_local_h::gentity_s;
    (*other).classname =
        b"door_trigger\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*other).r.mins[0 as i32 as usize] = mins[0 as i32 as usize];
    (*other).r.mins[1 as i32 as usize] = mins[1 as i32 as usize];
    (*other).r.mins[2 as i32 as usize] = mins[2 as i32 as usize];
    (*other).r.maxs[0 as i32 as usize] = maxs[0 as i32 as usize];
    (*other).r.maxs[1 as i32 as usize] = maxs[1 as i32 as usize];
    (*other).r.maxs[2 as i32 as usize] = maxs[2 as i32 as usize];
    (*other).parent = ent;
    (*other).r.contents = 0x40000000 as i32;
    (*other).touch = Some(
        Touch_DoorTrigger
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::src::qcommon::q_shared::trace_t,
            ) -> (),
    );
    // remember the thinnest axis
    (*other).count = best;
    crate::src::game::g_syscalls::trap_LinkEntity(other as *mut crate::g_local_h::gentity_s);
    MatchTeam(
        ent,
        (*ent).moverState as i32,
        crate::src::game::g_main::level.time,
    );
}
#[no_mangle]

pub unsafe extern "C" fn Think_MatchTeam(mut ent: *mut crate::g_local_h::gentity_t) {
    MatchTeam(
        ent,
        (*ent).moverState as i32,
        crate::src::game::g_main::level.time,
    );
}
/*QUAKED func_door (0 .5 .8) ? START_OPEN x CRUSHER
TOGGLE		wait in both the start and end states for a trigger event.
START_OPEN	the door to moves to its destination when spawned, and operate in reverse.  It is used to temporarily or permanently close off an area when triggered (not useful for touch or takedamage doors).
NOMONSTER	monsters will not trigger this door

"model2"	.md3 model to also draw
"angle"		determines the opening direction
"targetname" if set, no touch field will be spawned and a remote button or trigger field activates the door.
"speed"		movement speed (100 default)
"wait"		wait before returning (3 default, -1 = never return)
"lip"		lip remaining at end of move (8 default)
"dmg"		damage to inflict when blocked (2 default)
"color"		constantLight color
"light"		constantLight radius
"health"	if set, the door must be shot open
*/
#[no_mangle]

pub unsafe extern "C" fn SP_func_door(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut abs_movedir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut distance: f32 = 0.;
    let mut size: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lip: f32 = 0.;
    (*ent).sound2to1 = crate::src::game::g_utils::G_SoundIndex(
        b"sound/movers/doors/dr1_strt.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).sound1to2 = (*ent).sound2to1;
    (*ent).soundPos2 = crate::src::game::g_utils::G_SoundIndex(
        b"sound/movers/doors/dr1_end.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).soundPos1 = (*ent).soundPos2;
    (*ent).blocked = Some(
        Blocked_Door
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
    // default speed of 400
    if (*ent).speed == 0. {
        (*ent).speed = 400 as i32 as f32
    }
    // default wait of 2 seconds
    if (*ent).wait == 0. {
        (*ent).wait = 2 as i32 as f32
    }
    (*ent).wait *= 1000 as i32 as f32;
    // default lip of 8 units
    crate::src::game::g_spawn::G_SpawnFloat(
        b"lip\x00" as *const u8 as *const libc::c_char,
        b"8\x00" as *const u8 as *const libc::c_char,
        &mut lip,
    );
    // default damage of 2 points
    crate::src::game::g_spawn::G_SpawnInt(
        b"dmg\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).damage,
    );
    // first position at start
    (*ent).pos1[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
    (*ent).pos1[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
    (*ent).pos1[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize];
    // calculate second position
    crate::src::game::g_syscalls::trap_SetBrushModel(
        ent as *mut crate::g_local_h::gentity_s,
        (*ent).model,
    );
    crate::src::game::g_utils::G_SetMovedir(
        (*ent).s.angles.as_mut_ptr(),
        (*ent).movedir.as_mut_ptr(),
    );
    abs_movedir[0 as i32 as usize] = crate::stdlib::fabs((*ent).movedir[0 as i32 as usize] as f64)
        as crate::src::qcommon::q_shared::vec_t;
    abs_movedir[1 as i32 as usize] = crate::stdlib::fabs((*ent).movedir[1 as i32 as usize] as f64)
        as crate::src::qcommon::q_shared::vec_t;
    abs_movedir[2 as i32 as usize] = crate::stdlib::fabs((*ent).movedir[2 as i32 as usize] as f64)
        as crate::src::qcommon::q_shared::vec_t;
    size[0 as i32 as usize] = (*ent).r.maxs[0 as i32 as usize] - (*ent).r.mins[0 as i32 as usize];
    size[1 as i32 as usize] = (*ent).r.maxs[1 as i32 as usize] - (*ent).r.mins[1 as i32 as usize];
    size[2 as i32 as usize] = (*ent).r.maxs[2 as i32 as usize] - (*ent).r.mins[2 as i32 as usize];
    distance = abs_movedir[0 as i32 as usize] * size[0 as i32 as usize]
        + abs_movedir[1 as i32 as usize] * size[1 as i32 as usize]
        + abs_movedir[2 as i32 as usize] * size[2 as i32 as usize]
        - lip;
    (*ent).pos2[0 as i32 as usize] =
        (*ent).pos1[0 as i32 as usize] + (*ent).movedir[0 as i32 as usize] * distance;
    (*ent).pos2[1 as i32 as usize] =
        (*ent).pos1[1 as i32 as usize] + (*ent).movedir[1 as i32 as usize] * distance;
    (*ent).pos2[2 as i32 as usize] =
        (*ent).pos1[2 as i32 as usize] + (*ent).movedir[2 as i32 as usize] * distance;
    // if "start_open", reverse position 1 and 2
    if (*ent).spawnflags & 1 as i32 != 0 {
        let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        temp[0 as i32 as usize] = (*ent).pos2[0 as i32 as usize];
        temp[1 as i32 as usize] = (*ent).pos2[1 as i32 as usize];
        temp[2 as i32 as usize] = (*ent).pos2[2 as i32 as usize];
        (*ent).pos2[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
        (*ent).pos2[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
        (*ent).pos2[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize];
        (*ent).pos1[0 as i32 as usize] = temp[0 as i32 as usize];
        (*ent).pos1[1 as i32 as usize] = temp[1 as i32 as usize];
        (*ent).pos1[2 as i32 as usize] = temp[2 as i32 as usize]
    }
    InitMover(ent);
    (*ent).nextthink = crate::src::game::g_main::level.time + 100 as i32;
    if (*ent).flags & 0x400 as i32 == 0 {
        let mut health: i32 = 0;
        crate::src::game::g_spawn::G_SpawnInt(
            b"health\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            &mut health,
        );
        if health != 0 {
            (*ent).takedamage = crate::src::qcommon::q_shared::qtrue
        }
        if !(*ent).targetname.is_null() || health != 0 {
            // non touch/shoot doors
            (*ent).think = Some(
                Think_MatchTeam as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
            )
        } else {
            (*ent).think = Some(
                Think_SpawnNewDoorTrigger
                    as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
            )
        }
    };
}
/*
===============================================================================

PLAT

===============================================================================
*/
/*
==============
Touch_Plat

Don't allow decent if a living player is on it
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Touch_Plat(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
    mut _trace: *mut crate::src::qcommon::q_shared::trace_t,
) {
    if (*other).client.is_null()
        || (*(*other).client).ps.stats[crate::bg_public_h::STAT_HEALTH as i32 as usize] <= 0 as i32
    {
        return;
    }
    // delay return-to-pos1 by one second
    if (*ent).moverState as u32 == crate::g_local_h::MOVER_POS2 as i32 as u32 {
        (*ent).nextthink = crate::src::game::g_main::level.time + 1000 as i32
    };
}
/*
==============
Touch_PlatCenterTrigger

If the plat is at the bottom position, start it going up
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Touch_PlatCenterTrigger(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
    mut _trace: *mut crate::src::qcommon::q_shared::trace_t,
) {
    if (*other).client.is_null() {
        return;
    }
    if (*(*ent).parent).moverState as u32 == crate::g_local_h::MOVER_POS1 as i32 as u32 {
        Use_BinaryMover((*ent).parent, ent, other);
    };
}
/*
================
SpawnPlatTrigger

Spawn a trigger in the middle of the plat's low position
Elevator cars require that the trigger extend through the entire low position,
not just sit on top of it.
================
*/
#[no_mangle]

pub unsafe extern "C" fn SpawnPlatTrigger(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut trigger: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut tmin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut tmax: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // the middle trigger will be a thin trigger just
    // above the starting position
    trigger = crate::src::game::g_utils::G_Spawn() as *mut crate::g_local_h::gentity_s;
    (*trigger).classname =
        b"plat_trigger\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*trigger).touch = Some(
        Touch_PlatCenterTrigger
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::src::qcommon::q_shared::trace_t,
            ) -> (),
    );
    (*trigger).r.contents = 0x40000000 as i32;
    (*trigger).parent = ent;
    tmin[0 as i32 as usize] =
        (*ent).pos1[0 as i32 as usize] + (*ent).r.mins[0 as i32 as usize] + 33 as i32 as f32;
    tmin[1 as i32 as usize] =
        (*ent).pos1[1 as i32 as usize] + (*ent).r.mins[1 as i32 as usize] + 33 as i32 as f32;
    tmin[2 as i32 as usize] = (*ent).pos1[2 as i32 as usize] + (*ent).r.mins[2 as i32 as usize];
    tmax[0 as i32 as usize] =
        (*ent).pos1[0 as i32 as usize] + (*ent).r.maxs[0 as i32 as usize] - 33 as i32 as f32;
    tmax[1 as i32 as usize] =
        (*ent).pos1[1 as i32 as usize] + (*ent).r.maxs[1 as i32 as usize] - 33 as i32 as f32;
    tmax[2 as i32 as usize] =
        (*ent).pos1[2 as i32 as usize] + (*ent).r.maxs[2 as i32 as usize] + 8 as i32 as f32;
    if tmax[0 as i32 as usize] <= tmin[0 as i32 as usize] {
        tmin[0 as i32 as usize] = ((*ent).pos1[0 as i32 as usize] as f64
            + ((*ent).r.mins[0 as i32 as usize] + (*ent).r.maxs[0 as i32 as usize]) as f64 * 0.5f64)
            as crate::src::qcommon::q_shared::vec_t;
        tmax[0 as i32 as usize] = tmin[0 as i32 as usize] + 1 as i32 as f32
    }
    if tmax[1 as i32 as usize] <= tmin[1 as i32 as usize] {
        tmin[1 as i32 as usize] = ((*ent).pos1[1 as i32 as usize] as f64
            + ((*ent).r.mins[1 as i32 as usize] + (*ent).r.maxs[1 as i32 as usize]) as f64 * 0.5f64)
            as crate::src::qcommon::q_shared::vec_t;
        tmax[1 as i32 as usize] = tmin[1 as i32 as usize] + 1 as i32 as f32
    }
    (*trigger).r.mins[0 as i32 as usize] = tmin[0 as i32 as usize];
    (*trigger).r.mins[1 as i32 as usize] = tmin[1 as i32 as usize];
    (*trigger).r.mins[2 as i32 as usize] = tmin[2 as i32 as usize];
    (*trigger).r.maxs[0 as i32 as usize] = tmax[0 as i32 as usize];
    (*trigger).r.maxs[1 as i32 as usize] = tmax[1 as i32 as usize];
    (*trigger).r.maxs[2 as i32 as usize] = tmax[2 as i32 as usize];
    crate::src::game::g_syscalls::trap_LinkEntity(trigger as *mut crate::g_local_h::gentity_s);
}
/*QUAKED func_plat (0 .5 .8) ?
Plats are always drawn in the extended position so they will light correctly.

"lip"		default 8, protrusion above rest position
"height"	total height of movement, defaults to model height
"speed"		overrides default 200.
"dmg"		overrides default 2
"model2"	.md3 model to also draw
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]

pub unsafe extern "C" fn SP_func_plat(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut lip: f32 = 0.;
    let mut height: f32 = 0.;
    (*ent).sound2to1 = crate::src::game::g_utils::G_SoundIndex(
        b"sound/movers/plats/pt1_strt.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).sound1to2 = (*ent).sound2to1;
    (*ent).soundPos2 = crate::src::game::g_utils::G_SoundIndex(
        b"sound/movers/plats/pt1_end.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).soundPos1 = (*ent).soundPos2;
    (*ent).s.angles[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    (*ent).s.angles[1 as i32 as usize] = (*ent).s.angles[2 as i32 as usize];
    (*ent).s.angles[0 as i32 as usize] = (*ent).s.angles[1 as i32 as usize];
    crate::src::game::g_spawn::G_SpawnFloat(
        b"speed\x00" as *const u8 as *const libc::c_char,
        b"200\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).speed,
    );
    crate::src::game::g_spawn::G_SpawnInt(
        b"dmg\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).damage,
    );
    crate::src::game::g_spawn::G_SpawnFloat(
        b"wait\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).wait,
    );
    crate::src::game::g_spawn::G_SpawnFloat(
        b"lip\x00" as *const u8 as *const libc::c_char,
        b"8\x00" as *const u8 as *const libc::c_char,
        &mut lip,
    );
    (*ent).wait = 1000 as i32 as f32;
    // create second position
    crate::src::game::g_syscalls::trap_SetBrushModel(
        ent as *mut crate::g_local_h::gentity_s,
        (*ent).model,
    );
    if crate::src::game::g_spawn::G_SpawnFloat(
        b"height\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut height,
    ) as u64
        == 0
    {
        height = (*ent).r.maxs[2 as i32 as usize] - (*ent).r.mins[2 as i32 as usize] - lip
    }
    // pos1 is the rest (bottom) position, pos2 is the top
    (*ent).pos2[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
    (*ent).pos2[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
    (*ent).pos2[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize];
    (*ent).pos1[0 as i32 as usize] = (*ent).pos2[0 as i32 as usize];
    (*ent).pos1[1 as i32 as usize] = (*ent).pos2[1 as i32 as usize];
    (*ent).pos1[2 as i32 as usize] = (*ent).pos2[2 as i32 as usize];
    (*ent).pos1[2 as i32 as usize] -= height;
    InitMover(ent);
    // touch function keeps the plat from returning while
    // a live player is standing on it
    (*ent).touch = Some(
        Touch_Plat
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::src::qcommon::q_shared::trace_t,
            ) -> (),
    ); // so it can be treated as a door
    (*ent).blocked = Some(
        Blocked_Door
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
    (*ent).parent = ent;
    // spawn the trigger if one hasn't been custom made
    if (*ent).targetname.is_null() {
        SpawnPlatTrigger(ent);
    };
}
/*
===============================================================================

BUTTON

===============================================================================
*/
/*
==============
Touch_Button

===============
*/
#[no_mangle]

pub unsafe extern "C" fn Touch_Button(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut other: *mut crate::g_local_h::gentity_t,
    mut _trace: *mut crate::src::qcommon::q_shared::trace_t,
) {
    if (*other).client.is_null() {
        return;
    }
    if (*ent).moverState as u32 == crate::g_local_h::MOVER_POS1 as i32 as u32 {
        Use_BinaryMover(ent, other, other);
    };
}
/*QUAKED func_button (0 .5 .8) ?
When a button is touched, it moves some distance in the direction of its angle, triggers all of its targets, waits some time, then returns to its original position where it can be triggered again.

"model2"	.md3 model to also draw
"angle"		determines the opening direction
"target"	all entities with a matching targetname will be used
"speed"		override the default 40 speed
"wait"		override the default 1 second wait (-1 = never return)
"lip"		override the default 4 pixel lip remaining at end of move
"health"	if set, the button must be killed instead of touched
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]

pub unsafe extern "C" fn SP_func_button(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut abs_movedir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut distance: f32 = 0.;
    let mut size: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lip: f32 = 0.;
    (*ent).sound1to2 = crate::src::game::g_utils::G_SoundIndex(
        b"sound/movers/switches/butn2.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if (*ent).speed == 0. {
        (*ent).speed = 40 as i32 as f32
    }
    if (*ent).wait == 0. {
        (*ent).wait = 1 as i32 as f32
    }
    (*ent).wait *= 1000 as i32 as f32;
    // first position
    (*ent).pos1[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
    (*ent).pos1[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
    (*ent).pos1[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize];
    // calculate second position
    crate::src::game::g_syscalls::trap_SetBrushModel(
        ent as *mut crate::g_local_h::gentity_s,
        (*ent).model,
    );
    crate::src::game::g_spawn::G_SpawnFloat(
        b"lip\x00" as *const u8 as *const libc::c_char,
        b"4\x00" as *const u8 as *const libc::c_char,
        &mut lip,
    );
    crate::src::game::g_utils::G_SetMovedir(
        (*ent).s.angles.as_mut_ptr(),
        (*ent).movedir.as_mut_ptr(),
    );
    abs_movedir[0 as i32 as usize] = crate::stdlib::fabs((*ent).movedir[0 as i32 as usize] as f64)
        as crate::src::qcommon::q_shared::vec_t;
    abs_movedir[1 as i32 as usize] = crate::stdlib::fabs((*ent).movedir[1 as i32 as usize] as f64)
        as crate::src::qcommon::q_shared::vec_t;
    abs_movedir[2 as i32 as usize] = crate::stdlib::fabs((*ent).movedir[2 as i32 as usize] as f64)
        as crate::src::qcommon::q_shared::vec_t;
    size[0 as i32 as usize] = (*ent).r.maxs[0 as i32 as usize] - (*ent).r.mins[0 as i32 as usize];
    size[1 as i32 as usize] = (*ent).r.maxs[1 as i32 as usize] - (*ent).r.mins[1 as i32 as usize];
    size[2 as i32 as usize] = (*ent).r.maxs[2 as i32 as usize] - (*ent).r.mins[2 as i32 as usize];
    distance = abs_movedir[0 as i32 as usize] * size[0 as i32 as usize]
        + abs_movedir[1 as i32 as usize] * size[1 as i32 as usize]
        + abs_movedir[2 as i32 as usize] * size[2 as i32 as usize]
        - lip;
    (*ent).pos2[0 as i32 as usize] =
        (*ent).pos1[0 as i32 as usize] + (*ent).movedir[0 as i32 as usize] * distance;
    (*ent).pos2[1 as i32 as usize] =
        (*ent).pos1[1 as i32 as usize] + (*ent).movedir[1 as i32 as usize] * distance;
    (*ent).pos2[2 as i32 as usize] =
        (*ent).pos1[2 as i32 as usize] + (*ent).movedir[2 as i32 as usize] * distance;
    if (*ent).health != 0 {
        // shootable button
        (*ent).takedamage = crate::src::qcommon::q_shared::qtrue
    } else {
        // touchable button
        (*ent).touch = Some(
            Touch_Button
                as unsafe extern "C" fn(
                    _: *mut crate::g_local_h::gentity_t,
                    _: *mut crate::g_local_h::gentity_t,
                    _: *mut crate::src::qcommon::q_shared::trace_t,
                ) -> (),
        )
    }
    InitMover(ent);
}
/*
===============
Think_BeginMoving

The wait time at a corner has completed, so start moving again
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Think_BeginMoving(mut ent: *mut crate::g_local_h::gentity_t) {
    (*ent).s.pos.trTime = crate::src::game::g_main::level.time;
    (*ent).s.pos.trType = crate::src::qcommon::q_shared::TR_LINEAR_STOP;
}
/*
===============
Reached_Train
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Reached_Train(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut next: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut speed: f32 = 0.;
    let mut move_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut length: f32 = 0.;
    // copy the appropriate values
    next = (*ent).nextTrain;
    if next.is_null() || (*next).nextTrain.is_null() {
        return;
        // just stop
    }
    // fire all other targets
    crate::src::game::g_utils::G_UseTargets(
        next as *mut crate::g_local_h::gentity_s,
        0 as *mut crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_s,
    );
    // set the new trajectory
    (*ent).nextTrain = (*next).nextTrain;
    (*ent).pos1[0 as i32 as usize] = (*next).s.origin[0 as i32 as usize];
    (*ent).pos1[1 as i32 as usize] = (*next).s.origin[1 as i32 as usize];
    (*ent).pos1[2 as i32 as usize] = (*next).s.origin[2 as i32 as usize];
    (*ent).pos2[0 as i32 as usize] = (*(*next).nextTrain).s.origin[0 as i32 as usize];
    (*ent).pos2[1 as i32 as usize] = (*(*next).nextTrain).s.origin[1 as i32 as usize];
    (*ent).pos2[2 as i32 as usize] = (*(*next).nextTrain).s.origin[2 as i32 as usize];
    // if the path_corner has a speed, use that
    if (*next).speed != 0. {
        speed = (*next).speed
    } else {
        // otherwise use the train's speed
        speed = (*ent).speed
    }
    if speed < 1 as i32 as f32 {
        speed = 1 as i32 as f32
    }
    // calculate duration
    move_0[0 as i32 as usize] = (*ent).pos2[0 as i32 as usize] - (*ent).pos1[0 as i32 as usize];
    move_0[1 as i32 as usize] = (*ent).pos2[1 as i32 as usize] - (*ent).pos1[1 as i32 as usize];
    move_0[2 as i32 as usize] = (*ent).pos2[2 as i32 as usize] - (*ent).pos1[2 as i32 as usize];
    length = VectorLength(move_0.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    (*ent).s.pos.trDuration = (length * 1000 as i32 as f32 / speed) as i32;
    // Tequila comment: Be sure to send to clients after any fast move case
    (*ent).r.svFlags &= !(0x1 as i32);
    // Tequila comment: Fast move case
    if (*ent).s.pos.trDuration < 1 as i32 {
        // Tequila comment: As trDuration is used later in a division, we need to avoid that case now
        // With null trDuration,
        // the calculated rocks bounding box becomes infinite and the engine think for a short time
        // any entity is riding that mover but not the world entity... In rare case, I found it
        // can also stuck every map entities after func_door are used.
        // The desired effect with very very big speed is to have instant move, so any not null duration
        // lower than a frame duration should be sufficient.
        // Afaik, the negative case don't have to be supported.
        (*ent).s.pos.trDuration = 1 as i32;
        // Tequila comment: Don't send entity to clients so it becomes really invisible
        (*ent).r.svFlags |= 0x1 as i32
    }
    // looping sound
    (*ent).s.loopSound = (*next).soundLoop;
    // start it going
    SetMoverState(
        ent,
        crate::g_local_h::MOVER_1TO2,
        crate::src::game::g_main::level.time,
    );
    // if there is a "wait" value on the target, don't start moving yet
    if (*next).wait != 0. {
        (*ent).nextthink = (crate::src::game::g_main::level.time as f32
            + (*next).wait * 1000 as i32 as f32) as i32;
        (*ent).think = Some(
            Think_BeginMoving as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
        );
        (*ent).s.pos.trType = crate::src::qcommon::q_shared::TR_STATIONARY
    };
}
/*
===============
Think_SetupTrainTargets

Link all the corners together
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Think_SetupTrainTargets(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut path: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut next: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut start: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    (*ent).nextTrain = crate::src::game::g_utils::G_Find(
        0 as *mut crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_s,
        &mut (*(0 as *mut crate::g_local_h::gentity_t)).targetname as *mut *mut libc::c_char
            as crate::stddef_h::size_t as i32,
        (*ent).target,
    ) as *mut crate::g_local_h::gentity_s;
    if (*ent).nextTrain.is_null() {
        crate::src::game::g_main::G_Printf(
            b"func_train at %s with an unfound target\n\x00" as *const u8 as *const libc::c_char,
            crate::src::game::g_utils::vtos(
                (*ent).r.absmin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
            ),
        );
        return;
    }
    start = 0 as *mut crate::g_local_h::gentity_t;
    path = (*ent).nextTrain;
    while path != start {
        if start.is_null() {
            start = path
        }
        if (*path).target.is_null() {
            crate::src::game::g_main::G_Printf(
                b"Train corner at %s without a target\n\x00" as *const u8 as *const libc::c_char,
                crate::src::game::g_utils::vtos(
                    (*path).s.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                ),
            );
            return;
        }
        // find a path_corner among the targets
        // there may also be other targets that get fired when the corner
        // is reached
        next = 0 as *mut crate::g_local_h::gentity_t;
        loop {
            next = crate::src::game::g_utils::G_Find(
                next as *mut crate::g_local_h::gentity_s,
                &mut (*(0 as *mut crate::g_local_h::gentity_t)).targetname as *mut *mut libc::c_char
                    as crate::stddef_h::size_t as i32,
                (*path).target,
            ) as *mut crate::g_local_h::gentity_s;
            if next.is_null() {
                crate::src::game::g_main::G_Printf(
                    b"Train corner at %s without a target path_corner\n\x00" as *const u8
                        as *const libc::c_char,
                    crate::src::game::g_utils::vtos((*path).s.origin.as_mut_ptr()
                        as *const crate::src::qcommon::q_shared::vec_t),
                );
                return;
            }
            if !(::libc::strcmp(
                (*next).classname,
                b"path_corner\x00" as *const u8 as *const libc::c_char,
            ) != 0)
            {
                break;
            }
        }
        (*path).nextTrain = next;
        path = next
    }
    // start the train moving from the first corner
    Reached_Train(ent);
}
/*QUAKED path_corner (.5 .3 0) (-8 -8 -8) (8 8 8)
Train path corners.
Target: next path corner and other targets to fire
"speed" speed to move to the next corner
"wait" seconds to wait before behining move to next corner
*/
#[no_mangle]

pub unsafe extern "C" fn SP_path_corner(mut self_0: *mut crate::g_local_h::gentity_t) {
    if (*self_0).targetname.is_null() {
        crate::src::game::g_main::G_Printf(
            b"path_corner with no targetname at %s\n\x00" as *const u8 as *const libc::c_char,
            crate::src::game::g_utils::vtos(
                (*self_0).s.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
            ),
        );
        crate::src::game::g_utils::G_FreeEntity(self_0 as *mut crate::g_local_h::gentity_s);
        return;
    };
    // path corners don't need to be linked in
}
/*QUAKED func_train (0 .5 .8) ? START_ON TOGGLE BLOCK_STOPS
A train is a mover that moves between path_corner target points.
Trains MUST HAVE AN ORIGIN BRUSH.
The train spawns at the first target it is pointing at.
"model2"	.md3 model to also draw
"speed"		default 100
"dmg"		default	2
"noise"		looping sound to play when the train is in motion
"target"	next path corner
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]

pub unsafe extern "C" fn SP_func_train(mut self_0: *mut crate::g_local_h::gentity_t) {
    (*self_0).s.angles[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    (*self_0).s.angles[1 as i32 as usize] = (*self_0).s.angles[2 as i32 as usize];
    (*self_0).s.angles[0 as i32 as usize] = (*self_0).s.angles[1 as i32 as usize];
    if (*self_0).spawnflags & 4 as i32 != 0 {
        (*self_0).damage = 0 as i32
    } else if (*self_0).damage == 0 {
        (*self_0).damage = 2 as i32
    }
    if (*self_0).speed == 0. {
        (*self_0).speed = 100 as i32 as f32
    }
    if (*self_0).target.is_null() {
        crate::src::game::g_main::G_Printf(
            b"func_train without a target at %s\n\x00" as *const u8 as *const libc::c_char,
            crate::src::game::g_utils::vtos(
                (*self_0).r.absmin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
            ),
        );
        crate::src::game::g_utils::G_FreeEntity(self_0 as *mut crate::g_local_h::gentity_s);
        return;
    }
    crate::src::game::g_syscalls::trap_SetBrushModel(
        self_0 as *mut crate::g_local_h::gentity_s,
        (*self_0).model,
    );
    InitMover(self_0);
    (*self_0).reached =
        Some(Reached_Train as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
    // start trains on the second frame, to make sure their targets have had
    // a chance to spawn
    (*self_0).nextthink = crate::src::game::g_main::level.time + 100 as i32;
    (*self_0).think = Some(
        Think_SetupTrainTargets as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
    );
}
/*
===============================================================================

STATIC

===============================================================================
*/
/*QUAKED func_static (0 .5 .8) ?
A bmodel that just sits there, doing nothing.  Can be used for conditional walls and models.
"model2"	.md3 model to also draw
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]

pub unsafe extern "C" fn SP_func_static(mut ent: *mut crate::g_local_h::gentity_t) {
    crate::src::game::g_syscalls::trap_SetBrushModel(
        ent as *mut crate::g_local_h::gentity_s,
        (*ent).model,
    );
    InitMover(ent);
    (*ent).s.pos.trBase[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
    (*ent).s.pos.trBase[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
    (*ent).s.pos.trBase[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize];
    (*ent).r.currentOrigin[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
    (*ent).r.currentOrigin[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
    (*ent).r.currentOrigin[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize];
}
/*
===============================================================================

ROTATING

===============================================================================
*/
/*QUAKED func_rotating (0 .5 .8) ? START_ON - X_AXIS Y_AXIS
You need to have an origin brush as part of this entity.  The center of that brush will be
the point around which it is rotated. It will rotate around the Z axis by default.  You can
check either the X_AXIS or Y_AXIS box to change that.

"model2"	.md3 model to also draw
"speed"		determines how fast it moves; default value is 100.
"dmg"		damage to inflict when blocked (2 default)
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]

pub unsafe extern "C" fn SP_func_rotating(mut ent: *mut crate::g_local_h::gentity_t) {
    if (*ent).speed == 0. {
        (*ent).speed = 100 as i32 as f32
    }
    // set the axis of rotation
    (*ent).s.apos.trType = crate::src::qcommon::q_shared::TR_LINEAR;
    if (*ent).spawnflags & 4 as i32 != 0 {
        (*ent).s.apos.trDelta[2 as i32 as usize] = (*ent).speed
    } else if (*ent).spawnflags & 8 as i32 != 0 {
        (*ent).s.apos.trDelta[0 as i32 as usize] = (*ent).speed
    } else {
        (*ent).s.apos.trDelta[1 as i32 as usize] = (*ent).speed
    }
    if (*ent).damage == 0 {
        (*ent).damage = 2 as i32
    }
    crate::src::game::g_syscalls::trap_SetBrushModel(
        ent as *mut crate::g_local_h::gentity_s,
        (*ent).model,
    );
    InitMover(ent);
    (*ent).s.pos.trBase[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
    (*ent).s.pos.trBase[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
    (*ent).s.pos.trBase[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize];
    (*ent).r.currentOrigin[0 as i32 as usize] = (*ent).s.pos.trBase[0 as i32 as usize];
    (*ent).r.currentOrigin[1 as i32 as usize] = (*ent).s.pos.trBase[1 as i32 as usize];
    (*ent).r.currentOrigin[2 as i32 as usize] = (*ent).s.pos.trBase[2 as i32 as usize];
    (*ent).r.currentAngles[0 as i32 as usize] = (*ent).s.apos.trBase[0 as i32 as usize];
    (*ent).r.currentAngles[1 as i32 as usize] = (*ent).s.apos.trBase[1 as i32 as usize];
    (*ent).r.currentAngles[2 as i32 as usize] = (*ent).s.apos.trBase[2 as i32 as usize];
    crate::src::game::g_syscalls::trap_LinkEntity(ent as *mut crate::g_local_h::gentity_s);
}
/*
===============================================================================

BOBBING

===============================================================================
*/
/*QUAKED func_bobbing (0 .5 .8) ? X_AXIS Y_AXIS
Normally bobs on the Z axis
"model2"	.md3 model to also draw
"height"	amplitude of bob (32 default)
"speed"		seconds to complete a bob cycle (4 default)
"phase"		the 0.0 to 1.0 offset in the cycle to start at
"dmg"		damage to inflict when blocked (2 default)
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]

pub unsafe extern "C" fn SP_func_bobbing(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut height: f32 = 0.;
    let mut phase: f32 = 0.;
    crate::src::game::g_spawn::G_SpawnFloat(
        b"speed\x00" as *const u8 as *const libc::c_char,
        b"4\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).speed,
    );
    crate::src::game::g_spawn::G_SpawnFloat(
        b"height\x00" as *const u8 as *const libc::c_char,
        b"32\x00" as *const u8 as *const libc::c_char,
        &mut height,
    );
    crate::src::game::g_spawn::G_SpawnInt(
        b"dmg\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).damage,
    );
    crate::src::game::g_spawn::G_SpawnFloat(
        b"phase\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut phase,
    );
    crate::src::game::g_syscalls::trap_SetBrushModel(
        ent as *mut crate::g_local_h::gentity_s,
        (*ent).model,
    );
    InitMover(ent);
    (*ent).s.pos.trBase[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
    (*ent).s.pos.trBase[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
    (*ent).s.pos.trBase[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize];
    (*ent).r.currentOrigin[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
    (*ent).r.currentOrigin[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
    (*ent).r.currentOrigin[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize];
    (*ent).s.pos.trDuration = ((*ent).speed * 1000 as i32 as f32) as i32;
    (*ent).s.pos.trTime = ((*ent).s.pos.trDuration as f32 * phase) as i32;
    (*ent).s.pos.trType = crate::src::qcommon::q_shared::TR_SINE;
    // set the axis of bobbing
    if (*ent).spawnflags & 1 as i32 != 0 {
        (*ent).s.pos.trDelta[0 as i32 as usize] = height
    } else if (*ent).spawnflags & 2 as i32 != 0 {
        (*ent).s.pos.trDelta[1 as i32 as usize] = height
    } else {
        (*ent).s.pos.trDelta[2 as i32 as usize] = height
    };
}
/*
===============================================================================

PENDULUM

===============================================================================
*/
/*QUAKED func_pendulum (0 .5 .8) ?
You need to have an origin brush as part of this entity.
Pendulums always swing north / south on unrotated models.  Add an angles field to the model to allow rotation in other directions.
Pendulum frequency is a physical constant based on the length of the beam and gravity.
"model2"	.md3 model to also draw
"speed"		the number of degrees each way the pendulum swings, (30 default)
"phase"		the 0.0 to 1.0 offset in the cycle to start at
"dmg"		damage to inflict when blocked (2 default)
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]

pub unsafe extern "C" fn SP_func_pendulum(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut freq: f32 = 0.;
    let mut length: f32 = 0.;
    let mut phase: f32 = 0.;
    let mut speed: f32 = 0.;
    crate::src::game::g_spawn::G_SpawnFloat(
        b"speed\x00" as *const u8 as *const libc::c_char,
        b"30\x00" as *const u8 as *const libc::c_char,
        &mut speed,
    );
    crate::src::game::g_spawn::G_SpawnInt(
        b"dmg\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).damage,
    );
    crate::src::game::g_spawn::G_SpawnFloat(
        b"phase\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut phase,
    );
    crate::src::game::g_syscalls::trap_SetBrushModel(
        ent as *mut crate::g_local_h::gentity_s,
        (*ent).model,
    );
    // find pendulum length
    length = crate::stdlib::fabs((*ent).r.mins[2 as i32 as usize] as f64) as f32;
    if length < 8 as i32 as f32 {
        length = 8 as i32 as f32
    }
    freq = (1 as i32 as f64 / (3.14159265358979323846f64 * 2 as i32 as f64)
        * crate::stdlib::sqrt(
            (crate::src::game::g_main::g_gravity.value / (3 as i32 as f32 * length)) as f64,
        )) as f32;
    (*ent).s.pos.trDuration = (1000 as i32 as f32 / freq) as i32;
    InitMover(ent);
    (*ent).s.pos.trBase[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
    (*ent).s.pos.trBase[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
    (*ent).s.pos.trBase[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize];
    (*ent).r.currentOrigin[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
    (*ent).r.currentOrigin[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
    (*ent).r.currentOrigin[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize];
    (*ent).s.apos.trBase[0 as i32 as usize] = (*ent).s.angles[0 as i32 as usize];
    (*ent).s.apos.trBase[1 as i32 as usize] = (*ent).s.angles[1 as i32 as usize];
    (*ent).s.apos.trBase[2 as i32 as usize] = (*ent).s.angles[2 as i32 as usize];
    (*ent).s.apos.trDuration = (1000 as i32 as f32 / freq) as i32;
    (*ent).s.apos.trTime = ((*ent).s.apos.trDuration as f32 * phase) as i32;
    (*ent).s.apos.trType = crate::src::qcommon::q_shared::TR_SINE;
    (*ent).s.apos.trDelta[2 as i32 as usize] = speed;
}

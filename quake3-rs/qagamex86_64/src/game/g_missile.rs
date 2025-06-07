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
            (*v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
                + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
                + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize))
                as libc::c_double,
        ) as crate::src::qcommon::q_shared::vec_t;
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
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::bg_public_h::WP_BFG;
pub use crate::bg_public_h::WP_GAUNTLET;
pub use crate::bg_public_h::WP_GRAPPLING_HOOK;
pub use crate::bg_public_h::WP_GRENADE_LAUNCHER;
pub use crate::bg_public_h::WP_LIGHTNING;
pub use crate::bg_public_h::WP_MACHINEGUN;
pub use crate::bg_public_h::WP_NONE;
pub use crate::bg_public_h::WP_NUM_WEAPONS;
pub use crate::bg_public_h::WP_PLASMAGUN;
pub use crate::bg_public_h::WP_RAILGUN;
pub use crate::bg_public_h::WP_ROCKET_LAUNCHER;
pub use crate::bg_public_h::WP_SHOTGUN;
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
pub use crate::src::game::bg_misc::BG_EvaluateTrajectory;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectoryDelta;
pub use crate::src::game::g_combat::G_Damage;
pub use crate::src::game::g_combat::G_RadiusDamage;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_RunThink;
pub use crate::src::game::g_missile::q_shared_h::VectorLength;
pub use crate::src::game::g_syscalls::trap_LinkEntity;
pub use crate::src::game::g_syscalls::trap_Trace;
pub use crate::src::game::g_utils::G_AddEvent;
pub use crate::src::game::g_utils::G_FreeEntity;
pub use crate::src::game::g_utils::G_SetOrigin;
pub use crate::src::game::g_utils::G_Spawn;
pub use crate::src::game::g_weapon::LogAccuracyHit;
pub use crate::src::game::g_weapon::SnapVectorTowards;
pub use crate::src::game::g_weapon::Weapon_HookFree;
pub use crate::src::game::g_weapon::Weapon_HookThink;
pub use crate::src::qcommon::q_math::DirToByte;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
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
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

/*
================
G_BounceMissile

================
*/
#[no_mangle]

pub unsafe extern "C" fn G_BounceMissile(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut trace: *mut crate::src::qcommon::q_shared::trace_t,
) {
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dot: libc::c_float = 0.;
    let mut hitTime: libc::c_int = 0;
    // reflect the velocity on the trace plane
    hitTime = (crate::src::game::g_main::level.previousTime as libc::c_float
        + (crate::src::game::g_main::level.time - crate::src::game::g_main::level.previousTime)
            as libc::c_float
            * (*trace).fraction) as libc::c_int;
    crate::src::game::bg_misc::BG_EvaluateTrajectoryDelta(
        &mut (*ent).s.pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
        hitTime,
        velocity.as_mut_ptr(),
    );
    dot = velocity[0 as libc::c_int as usize] * (*trace).plane.normal[0 as libc::c_int as usize]
        + velocity[1 as libc::c_int as usize] * (*trace).plane.normal[1 as libc::c_int as usize]
        + velocity[2 as libc::c_int as usize] * (*trace).plane.normal[2 as libc::c_int as usize];
    (*ent).s.pos.trDelta[0 as libc::c_int as usize] = velocity[0 as libc::c_int as usize]
        + (*trace).plane.normal[0 as libc::c_int as usize]
            * (-(2 as libc::c_int) as libc::c_float * dot);
    (*ent).s.pos.trDelta[1 as libc::c_int as usize] = velocity[1 as libc::c_int as usize]
        + (*trace).plane.normal[1 as libc::c_int as usize]
            * (-(2 as libc::c_int) as libc::c_float * dot);
    (*ent).s.pos.trDelta[2 as libc::c_int as usize] = velocity[2 as libc::c_int as usize]
        + (*trace).plane.normal[2 as libc::c_int as usize]
            * (-(2 as libc::c_int) as libc::c_float * dot);
    if (*ent).s.eFlags & 0x20 as libc::c_int != 0 {
        (*ent).s.pos.trDelta[0 as libc::c_int as usize] =
            ((*ent).s.pos.trDelta[0 as libc::c_int as usize] as libc::c_double * 0.65f64)
                as crate::src::qcommon::q_shared::vec_t;
        (*ent).s.pos.trDelta[1 as libc::c_int as usize] =
            ((*ent).s.pos.trDelta[1 as libc::c_int as usize] as libc::c_double * 0.65f64)
                as crate::src::qcommon::q_shared::vec_t;
        (*ent).s.pos.trDelta[2 as libc::c_int as usize] =
            ((*ent).s.pos.trDelta[2 as libc::c_int as usize] as libc::c_double * 0.65f64)
                as crate::src::qcommon::q_shared::vec_t;
        // check for stop
        if (*trace).plane.normal[2 as libc::c_int as usize] as libc::c_double > 0.2f64
            && VectorLength(
                (*ent).s.pos.trDelta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
            ) < 40 as libc::c_int as libc::c_float
        {
            crate::src::game::g_utils::G_SetOrigin(
                ent as *mut crate::g_local_h::gentity_s,
                (*trace).endpos.as_mut_ptr(),
            );
            (*ent).s.time = crate::src::game::g_main::level.time / 4 as libc::c_int;
            return;
        }
    }
    (*ent).r.currentOrigin[0 as libc::c_int as usize] = (*ent).r.currentOrigin
        [0 as libc::c_int as usize]
        + (*trace).plane.normal[0 as libc::c_int as usize];
    (*ent).r.currentOrigin[1 as libc::c_int as usize] = (*ent).r.currentOrigin
        [1 as libc::c_int as usize]
        + (*trace).plane.normal[1 as libc::c_int as usize];
    (*ent).r.currentOrigin[2 as libc::c_int as usize] = (*ent).r.currentOrigin
        [2 as libc::c_int as usize]
        + (*trace).plane.normal[2 as libc::c_int as usize];
    (*ent).s.pos.trBase[0 as libc::c_int as usize] =
        (*ent).r.currentOrigin[0 as libc::c_int as usize];
    (*ent).s.pos.trBase[1 as libc::c_int as usize] =
        (*ent).r.currentOrigin[1 as libc::c_int as usize];
    (*ent).s.pos.trBase[2 as libc::c_int as usize] =
        (*ent).r.currentOrigin[2 as libc::c_int as usize];
    (*ent).s.pos.trTime = crate::src::game::g_main::level.time;
}
/*
================
G_ExplodeMissile

Explode a missile without an impact
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_ExplodeMissile(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*ent).s.pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
        crate::src::game::g_main::level.time,
        origin.as_mut_ptr(),
    );
    origin[0 as libc::c_int as usize] =
        origin[0 as libc::c_int as usize] as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    origin[1 as libc::c_int as usize] =
        origin[1 as libc::c_int as usize] as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    origin[2 as libc::c_int as usize] =
        origin[2 as libc::c_int as usize] as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    crate::src::game::g_utils::G_SetOrigin(
        ent as *mut crate::g_local_h::gentity_s,
        origin.as_mut_ptr(),
    );
    // we don't have a valid direction, so just point straight up
    dir[1 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    dir[0 as libc::c_int as usize] = dir[1 as libc::c_int as usize];
    dir[2 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*ent).s.eType = crate::bg_public_h::ET_GENERAL as libc::c_int;
    crate::src::game::g_utils::G_AddEvent(
        ent as *mut crate::g_local_h::gentity_s,
        crate::bg_public_h::EV_MISSILE_MISS as libc::c_int,
        crate::src::qcommon::q_math::DirToByte(dir.as_mut_ptr()),
    );
    (*ent).freeAfterEvent = crate::src::qcommon::q_shared::qtrue;
    // splash damage
    if (*ent).splashDamage != 0 {
        if crate::src::game::g_combat::G_RadiusDamage(
            (*ent).r.currentOrigin.as_mut_ptr(),
            (*ent).parent as *mut crate::g_local_h::gentity_s,
            (*ent).splashDamage as libc::c_float,
            (*ent).splashRadius as libc::c_float,
            ent as *mut crate::g_local_h::gentity_s,
            (*ent).splashMethodOfDeath,
        ) as u64
            != 0
        {
            (*crate::src::game::g_main::g_entities[(*ent).r.ownerNum as usize].client)
                .accuracy_hits += 1
        }
    }
    crate::src::game::g_syscalls::trap_LinkEntity(ent as *mut crate::g_local_h::gentity_s);
}
/*
================
G_MissileImpact
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_MissileImpact(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut trace: *mut crate::src::qcommon::q_shared::trace_t,
) {
    let mut other: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut hitClient: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    other = &mut *crate::src::game::g_main::g_entities
        .as_mut_ptr()
        .offset((*trace).entityNum as isize) as *mut crate::g_local_h::gentity_t;
    // check for bounce
    if (*other).takedamage as u64 == 0
        && (*ent).s.eFlags & (0x10 as libc::c_int | 0x20 as libc::c_int) != 0
    {
        G_BounceMissile(ent, trace);
        crate::src::game::g_utils::G_AddEvent(
            ent as *mut crate::g_local_h::gentity_s,
            crate::bg_public_h::EV_GRENADE_BOUNCE as libc::c_int,
            0 as libc::c_int,
        );
        return;
    }
    // impact damage
    if (*other).takedamage as u64 != 0 {
        // FIXME: wrong damage direction?
        if (*ent).damage != 0 {
            let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
            if crate::src::game::g_weapon::LogAccuracyHit(
                other as *mut crate::g_local_h::gentity_s,
                &mut *crate::src::game::g_main::g_entities
                    .as_mut_ptr()
                    .offset((*ent).r.ownerNum as isize) as *mut _
                    as *mut crate::g_local_h::gentity_s,
            ) as u64
                != 0
            {
                (*crate::src::game::g_main::g_entities[(*ent).r.ownerNum as usize].client)
                    .accuracy_hits += 1;
                hitClient = crate::src::qcommon::q_shared::qtrue
            }
            crate::src::game::bg_misc::BG_EvaluateTrajectoryDelta(
                &mut (*ent).s.pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
                crate::src::game::g_main::level.time,
                velocity.as_mut_ptr(),
            );
            if VectorLength(velocity.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
                == 0 as libc::c_int as libc::c_float
            {
                velocity[2 as libc::c_int as usize] =
                    1 as libc::c_int as crate::src::qcommon::q_shared::vec_t
                // stepped on a grenade
            }
            crate::src::game::g_combat::G_Damage(
                other as *mut crate::g_local_h::gentity_s,
                ent as *mut crate::g_local_h::gentity_s,
                &mut *crate::src::game::g_main::g_entities
                    .as_mut_ptr()
                    .offset((*ent).r.ownerNum as isize) as *mut _
                    as *mut crate::g_local_h::gentity_s,
                velocity.as_mut_ptr(),
                (*ent).s.origin.as_mut_ptr(),
                (*ent).damage,
                0 as libc::c_int,
                (*ent).methodOfDeath,
            );
        }
    }
    if ::libc::strcmp(
        (*ent).classname,
        b"hook\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut nent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
        let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        nent = crate::src::game::g_utils::G_Spawn() as *mut crate::g_local_h::gentity_s;
        if (*other).takedamage as libc::c_uint != 0 && !(*other).client.is_null() {
            crate::src::game::g_utils::G_AddEvent(
                nent as *mut crate::g_local_h::gentity_s,
                crate::bg_public_h::EV_MISSILE_HIT as libc::c_int,
                crate::src::qcommon::q_math::DirToByte((*trace).plane.normal.as_mut_ptr()),
            );
            (*nent).s.otherEntityNum = (*other).s.number;
            (*ent).enemy = other;
            v[0 as libc::c_int as usize] =
                ((*other).r.currentOrigin[0 as libc::c_int as usize] as libc::c_double
                    + ((*other).r.mins[0 as libc::c_int as usize]
                        + (*other).r.maxs[0 as libc::c_int as usize])
                        as libc::c_double
                        * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
            v[1 as libc::c_int as usize] =
                ((*other).r.currentOrigin[1 as libc::c_int as usize] as libc::c_double
                    + ((*other).r.mins[1 as libc::c_int as usize]
                        + (*other).r.maxs[1 as libc::c_int as usize])
                        as libc::c_double
                        * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
            v[2 as libc::c_int as usize] =
                ((*other).r.currentOrigin[2 as libc::c_int as usize] as libc::c_double
                    + ((*other).r.mins[2 as libc::c_int as usize]
                        + (*other).r.maxs[2 as libc::c_int as usize])
                        as libc::c_double
                        * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
            crate::src::game::g_weapon::SnapVectorTowards(
                v.as_mut_ptr(),
                (*ent).s.pos.trBase.as_mut_ptr(),
            );
        // save net bandwidth
        } else {
            v[0 as libc::c_int as usize] = (*trace).endpos[0 as libc::c_int as usize]; // save net bandwidth
            v[1 as libc::c_int as usize] = (*trace).endpos[1 as libc::c_int as usize];
            v[2 as libc::c_int as usize] = (*trace).endpos[2 as libc::c_int as usize];
            crate::src::game::g_utils::G_AddEvent(
                nent as *mut crate::g_local_h::gentity_s,
                crate::bg_public_h::EV_MISSILE_MISS as libc::c_int,
                crate::src::qcommon::q_math::DirToByte((*trace).plane.normal.as_mut_ptr()),
            );
            (*ent).enemy = 0 as *mut crate::g_local_h::gentity_t
        }
        crate::src::game::g_weapon::SnapVectorTowards(
            v.as_mut_ptr(),
            (*ent).s.pos.trBase.as_mut_ptr(),
        );
        (*nent).freeAfterEvent = crate::src::qcommon::q_shared::qtrue;
        // change over to a normal entity right at the point of impact
        (*nent).s.eType = crate::bg_public_h::ET_GENERAL as libc::c_int;
        (*ent).s.eType = crate::bg_public_h::ET_GRAPPLE as libc::c_int;
        crate::src::game::g_utils::G_SetOrigin(
            ent as *mut crate::g_local_h::gentity_s,
            v.as_mut_ptr(),
        );
        crate::src::game::g_utils::G_SetOrigin(
            nent as *mut crate::g_local_h::gentity_s,
            v.as_mut_ptr(),
        );
        (*ent).think = Some(
            crate::src::game::g_weapon::Weapon_HookThink
                as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
        );
        (*ent).nextthink = crate::src::game::g_main::level.time + 100 as libc::c_int;
        (*(*(*ent).parent).client).ps.pm_flags |= 2048 as libc::c_int;
        (*(*(*ent).parent).client).ps.grapplePoint[0 as libc::c_int as usize] =
            (*ent).r.currentOrigin[0 as libc::c_int as usize];
        (*(*(*ent).parent).client).ps.grapplePoint[1 as libc::c_int as usize] =
            (*ent).r.currentOrigin[1 as libc::c_int as usize];
        (*(*(*ent).parent).client).ps.grapplePoint[2 as libc::c_int as usize] =
            (*ent).r.currentOrigin[2 as libc::c_int as usize];
        crate::src::game::g_syscalls::trap_LinkEntity(ent as *mut crate::g_local_h::gentity_s);
        crate::src::game::g_syscalls::trap_LinkEntity(nent as *mut crate::g_local_h::gentity_s);
        return;
    }
    // is it cheaper in bandwidth to just remove this ent and create a new
    // one, rather than changing the missile into the explosion?
    if (*other).takedamage as libc::c_uint != 0 && !(*other).client.is_null() {
        crate::src::game::g_utils::G_AddEvent(
            ent as *mut crate::g_local_h::gentity_s,
            crate::bg_public_h::EV_MISSILE_HIT as libc::c_int,
            crate::src::qcommon::q_math::DirToByte((*trace).plane.normal.as_mut_ptr()),
        );
        (*ent).s.otherEntityNum = (*other).s.number
    } else if (*trace).surfaceFlags & 0x1000 as libc::c_int != 0 {
        crate::src::game::g_utils::G_AddEvent(
            ent as *mut crate::g_local_h::gentity_s,
            crate::bg_public_h::EV_MISSILE_MISS_METAL as libc::c_int,
            crate::src::qcommon::q_math::DirToByte((*trace).plane.normal.as_mut_ptr()),
        );
    } else {
        crate::src::game::g_utils::G_AddEvent(
            ent as *mut crate::g_local_h::gentity_s,
            crate::bg_public_h::EV_MISSILE_MISS as libc::c_int,
            crate::src::qcommon::q_math::DirToByte((*trace).plane.normal.as_mut_ptr()),
        );
    }
    (*ent).freeAfterEvent = crate::src::qcommon::q_shared::qtrue;
    // change over to a normal entity right at the point of impact
    (*ent).s.eType = crate::bg_public_h::ET_GENERAL as libc::c_int; // save net bandwidth
    crate::src::game::g_weapon::SnapVectorTowards(
        (*trace).endpos.as_mut_ptr(),
        (*ent).s.pos.trBase.as_mut_ptr(),
    );
    crate::src::game::g_utils::G_SetOrigin(
        ent as *mut crate::g_local_h::gentity_s,
        (*trace).endpos.as_mut_ptr(),
    );
    // splash damage (doesn't apply to person directly hit)
    if (*ent).splashDamage != 0 {
        if crate::src::game::g_combat::G_RadiusDamage(
            (*trace).endpos.as_mut_ptr(),
            (*ent).parent as *mut crate::g_local_h::gentity_s,
            (*ent).splashDamage as libc::c_float,
            (*ent).splashRadius as libc::c_float,
            other as *mut crate::g_local_h::gentity_s,
            (*ent).splashMethodOfDeath,
        ) as u64
            != 0
        {
            if hitClient as u64 == 0 {
                (*crate::src::game::g_main::g_entities[(*ent).r.ownerNum as usize].client)
                    .accuracy_hits += 1
            }
        }
    }
    crate::src::game::g_syscalls::trap_LinkEntity(ent as *mut crate::g_local_h::gentity_s);
}
/*
================
G_RunMissile
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_RunMissile(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
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
    let mut passent: libc::c_int = 0;
    // get current position
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*ent).s.pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
        crate::src::game::g_main::level.time,
        origin.as_mut_ptr(),
    );
    // if this missile bounced off an invulnerability sphere
    if !(*ent).target_ent.is_null() {
        passent = (*(*ent).target_ent).s.number
    } else {
        // ignore interactions with the missile owner
        passent = (*ent).r.ownerNum
    }
    // trace a line from the previous position to the current position
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        (*ent).r.currentOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*ent).r.mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*ent).r.maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        passent,
        (*ent).clipmask,
    );
    if tr.startsolid as libc::c_uint != 0 || tr.allsolid as libc::c_uint != 0 {
        // make sure the tr.entityNum is set to the entity we're stuck in
        crate::src::game::g_syscalls::trap_Trace(
            &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
            (*ent).r.currentOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).r.mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).r.maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*ent).r.currentOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            passent,
            (*ent).clipmask,
        );
        tr.fraction = 0 as libc::c_int as libc::c_float
    } else {
        (*ent).r.currentOrigin[0 as libc::c_int as usize] = tr.endpos[0 as libc::c_int as usize];
        (*ent).r.currentOrigin[1 as libc::c_int as usize] = tr.endpos[1 as libc::c_int as usize];
        (*ent).r.currentOrigin[2 as libc::c_int as usize] = tr.endpos[2 as libc::c_int as usize]
    }
    crate::src::game::g_syscalls::trap_LinkEntity(ent as *mut crate::g_local_h::gentity_s);
    if tr.fraction != 1 as libc::c_int as libc::c_float {
        // never explode or bounce on sky
        if tr.surfaceFlags & 0x10 as libc::c_int != 0 {
            // If grapple, reset owner
            if !(*ent).parent.is_null()
                && !(*(*ent).parent).client.is_null()
                && (*(*(*ent).parent).client).hook == ent
            {
                (*(*(*ent).parent).client).hook = 0 as *mut crate::g_local_h::gentity_t
            }
            crate::src::game::g_utils::G_FreeEntity(ent as *mut crate::g_local_h::gentity_s);
            return;
        }
        G_MissileImpact(ent, &mut tr);
        if (*ent).s.eType != crate::bg_public_h::ET_MISSILE as libc::c_int {
            return;
            // exploded
        }
    }
    // check think function after bouncing
    crate::src::game::g_main::G_RunThink(ent as *mut crate::g_local_h::gentity_s);
}
//=============================================================================
/*
=================
fire_plasma

=================
*/
#[no_mangle]

pub unsafe extern "C" fn fire_plasma(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) -> *mut crate::g_local_h::gentity_t {
    let mut bolt: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t; // move a bit on the very first frame
    crate::src::qcommon::q_math::VectorNormalize(dir);
    bolt = crate::src::game::g_utils::G_Spawn() as *mut crate::g_local_h::gentity_s;
    (*bolt).classname = b"plasma\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*bolt).nextthink = crate::src::game::g_main::level.time + 10000 as libc::c_int;
    (*bolt).think =
        Some(G_ExplodeMissile as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
    (*bolt).s.eType = crate::bg_public_h::ET_MISSILE as libc::c_int;
    (*bolt).r.svFlags = 0x80 as libc::c_int;
    (*bolt).s.weapon = crate::bg_public_h::WP_PLASMAGUN as libc::c_int;
    (*bolt).r.ownerNum = (*self_0).s.number;
    (*bolt).parent = self_0;
    (*bolt).damage = 20 as libc::c_int;
    (*bolt).splashDamage = 15 as libc::c_int;
    (*bolt).splashRadius = 20 as libc::c_int;
    (*bolt).methodOfDeath = crate::bg_public_h::MOD_PLASMA as libc::c_int;
    (*bolt).splashMethodOfDeath = crate::bg_public_h::MOD_PLASMA_SPLASH as libc::c_int;
    (*bolt).clipmask = 1 as libc::c_int | 0x2000000 as libc::c_int | 0x4000000 as libc::c_int;
    (*bolt).target_ent = 0 as *mut crate::g_local_h::gentity_t;
    (*bolt).s.pos.trType = crate::src::qcommon::q_shared::TR_LINEAR;
    (*bolt).s.pos.trTime = crate::src::game::g_main::level.time - 50 as libc::c_int;
    (*bolt).s.pos.trBase[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*bolt).s.pos.trBase[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*bolt).s.pos.trBase[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    (*bolt).s.pos.trDelta[0 as libc::c_int as usize] =
        *dir.offset(0 as libc::c_int as isize) * 2000 as libc::c_int as libc::c_float;
    (*bolt).s.pos.trDelta[1 as libc::c_int as usize] =
        *dir.offset(1 as libc::c_int as isize) * 2000 as libc::c_int as libc::c_float;
    (*bolt).s.pos.trDelta[2 as libc::c_int as usize] =
        *dir.offset(2 as libc::c_int as isize) * 2000 as libc::c_int as libc::c_float;
    (*bolt).s.pos.trDelta[0 as libc::c_int as usize] =
        (*bolt).s.pos.trDelta[0 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    (*bolt).s.pos.trDelta[1 as libc::c_int as usize] =
        (*bolt).s.pos.trDelta[1 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    (*bolt).s.pos.trDelta[2 as libc::c_int as usize] =
        (*bolt).s.pos.trDelta[2 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    // save net bandwidth
    (*bolt).r.currentOrigin[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*bolt).r.currentOrigin[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*bolt).r.currentOrigin[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    return bolt;
}
//=============================================================================
/*
=================
fire_grenade
=================
*/
#[no_mangle]

pub unsafe extern "C" fn fire_grenade(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) -> *mut crate::g_local_h::gentity_t {
    let mut bolt: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t; // move a bit on the very first frame
    crate::src::qcommon::q_math::VectorNormalize(dir);
    bolt = crate::src::game::g_utils::G_Spawn() as *mut crate::g_local_h::gentity_s;
    (*bolt).classname = b"grenade\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*bolt).nextthink = crate::src::game::g_main::level.time + 2500 as libc::c_int;
    (*bolt).think =
        Some(G_ExplodeMissile as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
    (*bolt).s.eType = crate::bg_public_h::ET_MISSILE as libc::c_int;
    (*bolt).r.svFlags = 0x80 as libc::c_int;
    (*bolt).s.weapon = crate::bg_public_h::WP_GRENADE_LAUNCHER as libc::c_int;
    (*bolt).s.eFlags = 0x20 as libc::c_int;
    (*bolt).r.ownerNum = (*self_0).s.number;
    (*bolt).parent = self_0;
    (*bolt).damage = 100 as libc::c_int;
    (*bolt).splashDamage = 100 as libc::c_int;
    (*bolt).splashRadius = 150 as libc::c_int;
    (*bolt).methodOfDeath = crate::bg_public_h::MOD_GRENADE as libc::c_int;
    (*bolt).splashMethodOfDeath = crate::bg_public_h::MOD_GRENADE_SPLASH as libc::c_int;
    (*bolt).clipmask = 1 as libc::c_int | 0x2000000 as libc::c_int | 0x4000000 as libc::c_int;
    (*bolt).target_ent = 0 as *mut crate::g_local_h::gentity_t;
    (*bolt).s.pos.trType = crate::src::qcommon::q_shared::TR_GRAVITY;
    (*bolt).s.pos.trTime = crate::src::game::g_main::level.time - 50 as libc::c_int;
    (*bolt).s.pos.trBase[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*bolt).s.pos.trBase[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*bolt).s.pos.trBase[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    (*bolt).s.pos.trDelta[0 as libc::c_int as usize] =
        *dir.offset(0 as libc::c_int as isize) * 700 as libc::c_int as libc::c_float;
    (*bolt).s.pos.trDelta[1 as libc::c_int as usize] =
        *dir.offset(1 as libc::c_int as isize) * 700 as libc::c_int as libc::c_float;
    (*bolt).s.pos.trDelta[2 as libc::c_int as usize] =
        *dir.offset(2 as libc::c_int as isize) * 700 as libc::c_int as libc::c_float;
    (*bolt).s.pos.trDelta[0 as libc::c_int as usize] =
        (*bolt).s.pos.trDelta[0 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    (*bolt).s.pos.trDelta[1 as libc::c_int as usize] =
        (*bolt).s.pos.trDelta[1 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    (*bolt).s.pos.trDelta[2 as libc::c_int as usize] =
        (*bolt).s.pos.trDelta[2 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    // save net bandwidth
    (*bolt).r.currentOrigin[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*bolt).r.currentOrigin[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*bolt).r.currentOrigin[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    return bolt;
}
//=============================================================================
/*
=================
fire_bfg
=================
*/
#[no_mangle]

pub unsafe extern "C" fn fire_bfg(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) -> *mut crate::g_local_h::gentity_t {
    let mut bolt: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t; // move a bit on the very first frame
    crate::src::qcommon::q_math::VectorNormalize(dir);
    bolt = crate::src::game::g_utils::G_Spawn() as *mut crate::g_local_h::gentity_s;
    (*bolt).classname = b"bfg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*bolt).nextthink = crate::src::game::g_main::level.time + 10000 as libc::c_int;
    (*bolt).think =
        Some(G_ExplodeMissile as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
    (*bolt).s.eType = crate::bg_public_h::ET_MISSILE as libc::c_int;
    (*bolt).r.svFlags = 0x80 as libc::c_int;
    (*bolt).s.weapon = crate::bg_public_h::WP_BFG as libc::c_int;
    (*bolt).r.ownerNum = (*self_0).s.number;
    (*bolt).parent = self_0;
    (*bolt).damage = 100 as libc::c_int;
    (*bolt).splashDamage = 100 as libc::c_int;
    (*bolt).splashRadius = 120 as libc::c_int;
    (*bolt).methodOfDeath = crate::bg_public_h::MOD_BFG as libc::c_int;
    (*bolt).splashMethodOfDeath = crate::bg_public_h::MOD_BFG_SPLASH as libc::c_int;
    (*bolt).clipmask = 1 as libc::c_int | 0x2000000 as libc::c_int | 0x4000000 as libc::c_int;
    (*bolt).target_ent = 0 as *mut crate::g_local_h::gentity_t;
    (*bolt).s.pos.trType = crate::src::qcommon::q_shared::TR_LINEAR;
    (*bolt).s.pos.trTime = crate::src::game::g_main::level.time - 50 as libc::c_int;
    (*bolt).s.pos.trBase[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*bolt).s.pos.trBase[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*bolt).s.pos.trBase[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    (*bolt).s.pos.trDelta[0 as libc::c_int as usize] =
        *dir.offset(0 as libc::c_int as isize) * 2000 as libc::c_int as libc::c_float;
    (*bolt).s.pos.trDelta[1 as libc::c_int as usize] =
        *dir.offset(1 as libc::c_int as isize) * 2000 as libc::c_int as libc::c_float;
    (*bolt).s.pos.trDelta[2 as libc::c_int as usize] =
        *dir.offset(2 as libc::c_int as isize) * 2000 as libc::c_int as libc::c_float;
    (*bolt).s.pos.trDelta[0 as libc::c_int as usize] =
        (*bolt).s.pos.trDelta[0 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    (*bolt).s.pos.trDelta[1 as libc::c_int as usize] =
        (*bolt).s.pos.trDelta[1 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    (*bolt).s.pos.trDelta[2 as libc::c_int as usize] =
        (*bolt).s.pos.trDelta[2 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    // save net bandwidth
    (*bolt).r.currentOrigin[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*bolt).r.currentOrigin[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*bolt).r.currentOrigin[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    return bolt;
}
//=============================================================================
/*
=================
fire_rocket
=================
*/
#[no_mangle]

pub unsafe extern "C" fn fire_rocket(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) -> *mut crate::g_local_h::gentity_t {
    let mut bolt: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t; // move a bit on the very first frame
    crate::src::qcommon::q_math::VectorNormalize(dir);
    bolt = crate::src::game::g_utils::G_Spawn() as *mut crate::g_local_h::gentity_s;
    (*bolt).classname = b"rocket\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*bolt).nextthink = crate::src::game::g_main::level.time + 15000 as libc::c_int;
    (*bolt).think =
        Some(G_ExplodeMissile as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
    (*bolt).s.eType = crate::bg_public_h::ET_MISSILE as libc::c_int;
    (*bolt).r.svFlags = 0x80 as libc::c_int;
    (*bolt).s.weapon = crate::bg_public_h::WP_ROCKET_LAUNCHER as libc::c_int;
    (*bolt).r.ownerNum = (*self_0).s.number;
    (*bolt).parent = self_0;
    (*bolt).damage = 100 as libc::c_int;
    (*bolt).splashDamage = 100 as libc::c_int;
    (*bolt).splashRadius = 120 as libc::c_int;
    (*bolt).methodOfDeath = crate::bg_public_h::MOD_ROCKET as libc::c_int;
    (*bolt).splashMethodOfDeath = crate::bg_public_h::MOD_ROCKET_SPLASH as libc::c_int;
    (*bolt).clipmask = 1 as libc::c_int | 0x2000000 as libc::c_int | 0x4000000 as libc::c_int;
    (*bolt).target_ent = 0 as *mut crate::g_local_h::gentity_t;
    (*bolt).s.pos.trType = crate::src::qcommon::q_shared::TR_LINEAR;
    (*bolt).s.pos.trTime = crate::src::game::g_main::level.time - 50 as libc::c_int;
    (*bolt).s.pos.trBase[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*bolt).s.pos.trBase[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*bolt).s.pos.trBase[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    (*bolt).s.pos.trDelta[0 as libc::c_int as usize] =
        *dir.offset(0 as libc::c_int as isize) * 900 as libc::c_int as libc::c_float;
    (*bolt).s.pos.trDelta[1 as libc::c_int as usize] =
        *dir.offset(1 as libc::c_int as isize) * 900 as libc::c_int as libc::c_float;
    (*bolt).s.pos.trDelta[2 as libc::c_int as usize] =
        *dir.offset(2 as libc::c_int as isize) * 900 as libc::c_int as libc::c_float;
    (*bolt).s.pos.trDelta[0 as libc::c_int as usize] =
        (*bolt).s.pos.trDelta[0 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    (*bolt).s.pos.trDelta[1 as libc::c_int as usize] =
        (*bolt).s.pos.trDelta[1 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    (*bolt).s.pos.trDelta[2 as libc::c_int as usize] =
        (*bolt).s.pos.trDelta[2 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    // save net bandwidth
    (*bolt).r.currentOrigin[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*bolt).r.currentOrigin[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*bolt).r.currentOrigin[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    return bolt;
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
// g_local.h -- local definitions for game module
//==================================================================
// the "gameversion" client command will print this plus compile date
// msec
// gentity->flags
// not the first on the team
// spawn point not for bot use
// spawn point just for bots
// force gesture on client
// movers are things like doors, plats, buttons, etc
//============================================================================
// communicated by server to clients
// shared by both the server system and game
// DO NOT MODIFY ANYTHING ABOVE THIS, THE SERVER
// EXPECTS THE FIELDS IN THAT ORDER!
//================================
// NULL if not a client
// set in QuakeEd
// set in QuakeEd
// if true, FreeEntity will only unlink
// bodyque uses this
// FL_* variables
// level.time when the object was freed
// events will be cleared EVENT_VALID_MSEC after set
// if true, it can be pushed by movers and fall off edges
// all game items are physicsObjects,
// 1.0 = continuous bounce, 0.0 = no bounce
// brushes with this content value will be collided against
// when moving.  items and corpses do not collide against
// players, for instance
// movers
// body queue sinking, etc
// movers call this when hitting endpoint
// wind tunnel
// quad will increase this without increasing radius
// next entity in team
// master of the team
// timing variables
// for bonus items
// Beginning a team game, spawn at base
// Now actively playing
// client data that stays across multiple levels or tournament restarts
// this is achieved by writing all the data to cvar strings at game shutdown
// time and reading them back at connection time.  Anything added here
// MUST be dealt with in G_InitSessionData() / G_ReadSessionData() / G_WriteSessionData()
// for determining next-in-line to play
// for chasecam and follow mode
// tournament stats
// true when this client is a team leader
//
// client data that stays across multiple respawns, but is cleared
// on each level change or team change at ClientBegin()
// we would lose angles if not persistant
// true if "ip" info key is "localhost"
// the first spawn should be at a cool location
// based on cg_predictItems userinfo
//
// for handicapping
// level.time the client entered the game
// status in teamplay games
// to prevent people from constantly calling votes
// to prevent people from constantly calling votes
// send team overlay updates?
// this structure is cleared on each ClientSpawn(),
// except for 'client->pers' and 'client->sess'
// ps MUST be the first element, because the server expects it
// communicated by server to clients
// the rest of the structure is private to game
// wishes to leave the intermission
// level.time of last usercmd_t, for EF_CONNECTION
// we can't just use pers.lastCommand.time, because
// of the g_sycronousclients case
// sum up damage over an entire frame, so
// shotgun blasts give a single big kick
// damage absorbed by armor
// damage taken out of health
// impact damage
// origin for vector calculation
// if true, don't use the damage_from vector
// for "impressive" reward sound
// total number of shots
// total number of hits
//
// last client that this client killed
// last client that damaged this client
// type of damage the client did
// timers
// can respawn when time > this, force after g_forcerespwan
// kick players when time > this
// qtrue if the five seoond warning has been given
// clear the EF_AWARD_IMPRESSIVE, etc when time > this
// for multiple kill rewards
// used for hook
// grapple hook if out
// time the player switched teams
// timeResidual is used to handle events that happen every second
// like health / armor countdowns and regeneration
//
// this structure is cleared as each map is entered
//
// [maxclients]
// MAX_CLIENTS <= num_entities <= ENTITYNUM_MAX_NORMAL
// restart match at this time
// store latched cvars here that we want to get at often
// in msec
// so movers can back up when blocked
// level.time the map was started
// last time of client team location update
// don't use any old session data, because
// we changed gametype
// waiting for a map_restart to fire
// includes connecting clients
// connected, non-spectators
// sorted by score
// clientNums for auto-follow spectators
// sound index for standing in lava
// for detecting if g_warmup is changed
// voting state
// level.time vote was called
// time the vote is executed
// set by CalculateRanks
// team voting state
// level.time vote was called
// set by CalculateRanks
// spawn variables
// the G_Spawn*() functions are valid
// key / value pairs
// intermission state
// intermission was qualified, but
// wait INTERMISSION_DELAY_TIME before
// actually going there so the last
// frag can be watched.  Disable future
// kills during this delay
// time the intermission was started
// at least one client wants to exit
// also used for spectator spawns
// target_locations get linked
// head of the location list
// dead bodies
//
// g_spawn.c
//
// spawn string returns a temporary reference, you must CopyString() if you want to keep it
//
// g_cmds.c
//
//
// g_items.c
//
//
// g_utils.c
//
//
// g_combat.c
//
// damage flags
// damage was indirect
// armour does not protect from this damage
// do not affect velocity, just view angles
// armor, shields, invulnerability, and godmode have no effect
//
// g_missile.c
//
/*
=================
fire_grapple
=================
*/
#[no_mangle]

pub unsafe extern "C" fn fire_grapple(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) -> *mut crate::g_local_h::gentity_t {
    let mut hook: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t; // move a bit on the very first frame
    crate::src::qcommon::q_math::VectorNormalize(dir); // use to match beam in client
    hook = crate::src::game::g_utils::G_Spawn() as *mut crate::g_local_h::gentity_s;
    (*hook).classname = b"hook\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*hook).nextthink = crate::src::game::g_main::level.time + 10000 as libc::c_int;
    (*hook).think = Some(
        crate::src::game::g_weapon::Weapon_HookFree
            as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
    );
    (*hook).s.eType = crate::bg_public_h::ET_MISSILE as libc::c_int;
    (*hook).r.svFlags = 0x80 as libc::c_int;
    (*hook).s.weapon = crate::bg_public_h::WP_GRAPPLING_HOOK as libc::c_int;
    (*hook).r.ownerNum = (*self_0).s.number;
    (*hook).methodOfDeath = crate::bg_public_h::MOD_GRAPPLE as libc::c_int;
    (*hook).clipmask = 1 as libc::c_int | 0x2000000 as libc::c_int | 0x4000000 as libc::c_int;
    (*hook).parent = self_0;
    (*hook).target_ent = 0 as *mut crate::g_local_h::gentity_t;
    (*hook).s.pos.trType = crate::src::qcommon::q_shared::TR_LINEAR;
    (*hook).s.pos.trTime = crate::src::game::g_main::level.time - 50 as libc::c_int;
    (*hook).s.otherEntityNum = (*self_0).s.number;
    (*hook).s.pos.trBase[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*hook).s.pos.trBase[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*hook).s.pos.trBase[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    (*hook).s.pos.trDelta[0 as libc::c_int as usize] =
        *dir.offset(0 as libc::c_int as isize) * 800 as libc::c_int as libc::c_float;
    (*hook).s.pos.trDelta[1 as libc::c_int as usize] =
        *dir.offset(1 as libc::c_int as isize) * 800 as libc::c_int as libc::c_float;
    (*hook).s.pos.trDelta[2 as libc::c_int as usize] =
        *dir.offset(2 as libc::c_int as isize) * 800 as libc::c_int as libc::c_float;
    (*hook).s.pos.trDelta[0 as libc::c_int as usize] =
        (*hook).s.pos.trDelta[0 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    (*hook).s.pos.trDelta[1 as libc::c_int as usize] =
        (*hook).s.pos.trDelta[1 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    (*hook).s.pos.trDelta[2 as libc::c_int as usize] =
        (*hook).s.pos.trDelta[2 as libc::c_int as usize] as libc::c_int
            as crate::src::qcommon::q_shared::vec_t;
    // save net bandwidth
    (*hook).r.currentOrigin[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*hook).r.currentOrigin[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*hook).r.currentOrigin[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    (*(*self_0).client).hook = hook;
    return hook;
}

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

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::weapon_t;
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
pub use crate::src::game::bg_misc::BG_FindItemForWeapon;
pub use crate::src::game::bg_misc::BG_PlayerStateToEntityState;
pub use crate::src::game::g_client::SetClientViewAngle;
pub use crate::src::game::g_items::RegisterItem;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_misc::q_shared_h::CrossProduct;
pub use crate::src::game::g_missile::fire_grenade;
pub use crate::src::game::g_missile::fire_plasma;
pub use crate::src::game::g_missile::fire_rocket;
pub use crate::src::game::g_spawn::G_SpawnFloat;
pub use crate::src::game::g_syscalls::trap_LinkEntity;
pub use crate::src::game::g_syscalls::trap_UnlinkEntity;
pub use crate::src::game::g_utils::G_AddEvent;
pub use crate::src::game::g_utils::G_FreeEntity;
pub use crate::src::game::g_utils::G_KillBox;
pub use crate::src::game::g_utils::G_PickTarget;
pub use crate::src::game::g_utils::G_SetMovedir;
pub use crate::src::game::g_utils::G_SetOrigin;
pub use crate::src::game::g_utils::G_TempEntity;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::DirToByte;
pub use crate::src::qcommon::q_math::PerpendicularVector;
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
// g_misc.c
/*QUAKED func_group (0 0 0) ?
Used to group brushes together just for editor convenience.  They are turned into normal brushes by the utilities.
*/
/*QUAKED info_camp (0 0.5 0) (-4 -4 -4) (4 4 4)
Used as a positional target for calculations in the utilities (spotlights, etc), but removed during gameplay.
*/
#[no_mangle]

pub unsafe extern "C" fn SP_info_camp(mut self_0: *mut gentity_t) {
    G_SetOrigin(
        self_0 as *mut gentity_s,
        (*self_0).s.origin.as_mut_ptr(),
    );
}
/*QUAKED info_null (0 0.5 0) (-4 -4 -4) (4 4 4)
Used as a positional target for calculations in the utilities (spotlights, etc), but removed during gameplay.
*/
#[no_mangle]

pub unsafe extern "C" fn SP_info_null(mut self_0: *mut gentity_t) {
    G_FreeEntity(self_0 as *mut gentity_s);
}
/*QUAKED info_notnull (0 0.5 0) (-4 -4 -4) (4 4 4)
Used as a positional target for in-game calculation, like jumppad targets.
target_position does the same thing
*/
#[no_mangle]

pub unsafe extern "C" fn SP_info_notnull(mut self_0: *mut gentity_t) {
    G_SetOrigin(
        self_0 as *mut gentity_s,
        (*self_0).s.origin.as_mut_ptr(),
    );
}
/*QUAKED light (0 1 0) (-8 -8 -8) (8 8 8) linear
Non-displayed light.
"light" overrides the default 300 intensity.
Linear checbox gives linear falloff instead of inverse square
Lights pointed at a target will be spotlights.
"radius" overrides the default 64 unit radius of a spotlight at the target point.
*/
#[no_mangle]

pub unsafe extern "C" fn SP_light(mut self_0: *mut gentity_t) {
    G_FreeEntity(self_0 as *mut gentity_s);
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
//
// g_mover.c
//
//
// g_trigger.c
//
//
// g_misc.c
//
/*
=================================================================================

TELEPORTERS

=================================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn TeleportPlayer(
    mut player: *mut gentity_t,
    mut origin: *mut vec_t,
    mut angles: *mut vec_t,
) {
    let mut tent: *mut gentity_t = 0 as *mut gentity_t;
    let mut noAngles: qboolean =
        qfalse;
    noAngles = (*angles.offset(0 as i32 as isize) as f64 > 999999.0f64) as i32
        as qboolean;
    // use temp events at source and destination to prevent the effect
    // from getting dropped by a second player event
    if (*(*player).client).sess.sessionTeam as u32
        != TEAM_SPECTATOR as i32 as u32
    {
        tent = G_TempEntity(
            (*(*player).client).ps.origin.as_mut_ptr(),
            EV_PLAYER_TELEPORT_OUT as i32,
        ) as *mut gentity_s;
        (*tent).s.clientNum = (*player).s.clientNum;
        tent = G_TempEntity(
            origin,
            EV_PLAYER_TELEPORT_IN as i32,
        ) as *mut gentity_s;
        (*tent).s.clientNum = (*player).s.clientNum
    }
    // unlink to make sure it can't possibly interfere with G_KillBox
    trap_UnlinkEntity(player as *mut gentity_s);
    (*(*player).client).ps.origin[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    (*(*player).client).ps.origin[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    (*(*player).client).ps.origin[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
    (*(*player).client).ps.origin[2 as i32 as usize] += 1 as i32 as f32;
    if noAngles as u64 == 0 {
        // spit the player out
        AngleVectors(
            angles as *const vec_t,
            (*(*player).client).ps.velocity.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
        ); // hold time
        (*(*player).client).ps.velocity[0 as i32 as usize] =
            (*(*player).client).ps.velocity[0 as i32 as usize] * 400 as i32 as f32;
        (*(*player).client).ps.velocity[1 as i32 as usize] =
            (*(*player).client).ps.velocity[1 as i32 as usize] * 400 as i32 as f32;
        (*(*player).client).ps.velocity[2 as i32 as usize] =
            (*(*player).client).ps.velocity[2 as i32 as usize] * 400 as i32 as f32;
        (*(*player).client).ps.pm_time = 160 as i32;
        (*(*player).client).ps.pm_flags |= 64 as i32;
        // set angles
        SetClientViewAngle(
            player as *mut gentity_s,
            angles,
        );
    }
    // toggle the teleport bit so the client knows to not lerp
    (*(*player).client).ps.eFlags ^= 0x4 as i32;
    // kill anything at the destination
    if (*(*player).client).sess.sessionTeam as u32
        != TEAM_SPECTATOR as i32 as u32
    {
        G_KillBox(player as *mut gentity_s);
    }
    // save results of pmove
    BG_PlayerStateToEntityState(
        &mut (*(*player).client).ps as *mut _ as *mut playerState_s,
        &mut (*player).s as *mut _ as *mut entityState_s,
        qtrue,
    );
    // use the precise origin for linking
    (*player).r.currentOrigin[0 as i32 as usize] = (*(*player).client).ps.origin[0 as i32 as usize];
    (*player).r.currentOrigin[1 as i32 as usize] = (*(*player).client).ps.origin[1 as i32 as usize];
    (*player).r.currentOrigin[2 as i32 as usize] = (*(*player).client).ps.origin[2 as i32 as usize];
    if (*(*player).client).sess.sessionTeam as u32
        != TEAM_SPECTATOR as i32 as u32
    {
        trap_LinkEntity(player as *mut gentity_s);
    };
}
/*QUAKED misc_teleporter_dest (1 0 0) (-32 -32 -24) (32 32 -16)
Point teleporters at these.
Now that we don't have teleport destination pads, this is just
an info_notnull
*/
#[no_mangle]

pub unsafe extern "C" fn SP_misc_teleporter_dest(mut _ent: *mut gentity_t) {}
//===========================================================
/*QUAKED misc_model (1 0 0) (-16 -16 -16) (16 16 16)
"model"		arbitrary .md3 file to display
*/
#[no_mangle]

pub unsafe extern "C" fn SP_misc_model(mut ent: *mut gentity_t) {
    G_FreeEntity(ent as *mut gentity_s);
}
//===========================================================
#[no_mangle]

pub unsafe extern "C" fn locateCamera(mut ent: *mut gentity_t) {
    let mut dir: vec3_t = [0.; 3];
    let mut target: *mut gentity_t = 0 as *mut gentity_t;
    let mut owner: *mut gentity_t = 0 as *mut gentity_t;
    owner =
        G_PickTarget((*ent).target) as *mut gentity_s;
    if owner.is_null() {
        G_Printf(
            b"Couldn\'t find target for misc_partal_surface\n\x00" as *const u8
                as *const libc::c_char,
        );
        G_FreeEntity(ent as *mut gentity_s);
        return;
    }
    (*ent).r.ownerNum = (*owner).s.number;
    // frame holds the rotate speed
    if (*owner).spawnflags & 1 as i32 != 0 {
        (*ent).s.frame = 25 as i32
    } else if (*owner).spawnflags & 2 as i32 != 0 {
        (*ent).s.frame = 75 as i32
    }
    // swing camera ?
    if (*owner).spawnflags & 4 as i32 != 0 {
        // set to 0 for no rotation at all
        (*ent).s.powerups = 0 as i32
    } else {
        (*ent).s.powerups = 1 as i32
    }
    // clientNum holds the rotate offset
    (*ent).s.clientNum = (*owner).s.clientNum;
    (*ent).s.origin2[0 as i32 as usize] = (*owner).s.origin[0 as i32 as usize];
    (*ent).s.origin2[1 as i32 as usize] = (*owner).s.origin[1 as i32 as usize];
    (*ent).s.origin2[2 as i32 as usize] = (*owner).s.origin[2 as i32 as usize];
    // see if the portal_camera has a target
    target = G_PickTarget((*owner).target)
        as *mut gentity_s;
    if !target.is_null() {
        dir[0 as i32 as usize] =
            (*target).s.origin[0 as i32 as usize] - (*owner).s.origin[0 as i32 as usize];
        dir[1 as i32 as usize] =
            (*target).s.origin[1 as i32 as usize] - (*owner).s.origin[1 as i32 as usize];
        dir[2 as i32 as usize] =
            (*target).s.origin[2 as i32 as usize] - (*owner).s.origin[2 as i32 as usize];
        VectorNormalize(dir.as_mut_ptr());
    } else {
        G_SetMovedir((*owner).s.angles.as_mut_ptr(), dir.as_mut_ptr());
    }
    (*ent).s.eventParm = DirToByte(dir.as_mut_ptr());
}
/*QUAKED misc_portal_surface (0 0 1) (-8 -8 -8) (8 8 8)
The portal surface nearest this entity will show a view from the targeted misc_portal_camera, or a mirror view if untargeted.
This must be within 64 world units of the surface!
*/
#[no_mangle]

pub unsafe extern "C" fn SP_misc_portal_surface(mut ent: *mut gentity_t) {
    (*ent).r.mins[2 as i32 as usize] = 0 as i32 as vec_t;
    (*ent).r.mins[1 as i32 as usize] = (*ent).r.mins[2 as i32 as usize];
    (*ent).r.mins[0 as i32 as usize] = (*ent).r.mins[1 as i32 as usize];
    (*ent).r.maxs[2 as i32 as usize] = 0 as i32 as vec_t;
    (*ent).r.maxs[1 as i32 as usize] = (*ent).r.maxs[2 as i32 as usize];
    (*ent).r.maxs[0 as i32 as usize] = (*ent).r.maxs[1 as i32 as usize];
    trap_LinkEntity(ent as *mut gentity_s);
    (*ent).r.svFlags = 0x40 as i32;
    (*ent).s.eType = ET_PORTAL as i32;
    if (*ent).target.is_null() {
        (*ent).s.origin2[0 as i32 as usize] = (*ent).s.origin[0 as i32 as usize];
        (*ent).s.origin2[1 as i32 as usize] = (*ent).s.origin[1 as i32 as usize];
        (*ent).s.origin2[2 as i32 as usize] = (*ent).s.origin[2 as i32 as usize]
    } else {
        (*ent).think =
            Some(locateCamera as unsafe extern "C" fn(_: *mut gentity_t) -> ());
        (*ent).nextthink = level.time + 100 as i32
    };
}
/*QUAKED misc_portal_camera (0 0 1) (-8 -8 -8) (8 8 8) slowrotate fastrotate noswing
The target for a misc_portal_director.  You can set either angles or target another entity to determine the direction of view.
"roll" an angle modifier to orient the camera around the target vector;
*/
#[no_mangle]

pub unsafe extern "C" fn SP_misc_portal_camera(mut ent: *mut gentity_t) {
    let mut roll: f32 = 0.;
    (*ent).r.mins[2 as i32 as usize] = 0 as i32 as vec_t;
    (*ent).r.mins[1 as i32 as usize] = (*ent).r.mins[2 as i32 as usize];
    (*ent).r.mins[0 as i32 as usize] = (*ent).r.mins[1 as i32 as usize];
    (*ent).r.maxs[2 as i32 as usize] = 0 as i32 as vec_t;
    (*ent).r.maxs[1 as i32 as usize] = (*ent).r.maxs[2 as i32 as usize];
    (*ent).r.maxs[0 as i32 as usize] = (*ent).r.maxs[1 as i32 as usize];
    trap_LinkEntity(ent as *mut gentity_s);
    G_SpawnFloat(
        b"roll\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut roll,
    );
    (*ent).s.clientNum = (roll as f64 / 360.0f64 * 256 as i32 as f64) as i32;
}
/*
======================================================================

  SHOOTERS

======================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn Use_Shooter(
    mut ent: *mut gentity_t,
    mut _other: *mut gentity_t,
    mut _activator: *mut gentity_t,
) {
    let mut dir: vec3_t = [0.; 3];
    let mut deg: f32 = 0.;
    let mut up: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    // see if we have a target
    if !(*ent).enemy.is_null() {
        dir[0 as i32 as usize] =
            (*(*ent).enemy).r.currentOrigin[0 as i32 as usize] - (*ent).s.origin[0 as i32 as usize];
        dir[1 as i32 as usize] =
            (*(*ent).enemy).r.currentOrigin[1 as i32 as usize] - (*ent).s.origin[1 as i32 as usize];
        dir[2 as i32 as usize] =
            (*(*ent).enemy).r.currentOrigin[2 as i32 as usize] - (*ent).s.origin[2 as i32 as usize];
        VectorNormalize(dir.as_mut_ptr());
    } else {
        dir[0 as i32 as usize] = (*ent).movedir[0 as i32 as usize];
        dir[1 as i32 as usize] = (*ent).movedir[1 as i32 as usize];
        dir[2 as i32 as usize] = (*ent).movedir[2 as i32 as usize]
    }
    // randomize a bit
    PerpendicularVector(
        up.as_mut_ptr(),
        dir.as_mut_ptr() as *const vec_t,
    );
    CrossProduct(
        up.as_mut_ptr() as *const vec_t,
        dir.as_mut_ptr() as *const vec_t,
        right.as_mut_ptr(),
    );
    deg = (2.0f64
        * (((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
        * (*ent).random as f64) as f32;
    dir[0 as i32 as usize] = dir[0 as i32 as usize] + up[0 as i32 as usize] * deg;
    dir[1 as i32 as usize] = dir[1 as i32 as usize] + up[1 as i32 as usize] * deg;
    dir[2 as i32 as usize] = dir[2 as i32 as usize] + up[2 as i32 as usize] * deg;
    deg = (2.0f64
        * (((libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
        * (*ent).random as f64) as f32;
    dir[0 as i32 as usize] = dir[0 as i32 as usize] + right[0 as i32 as usize] * deg;
    dir[1 as i32 as usize] = dir[1 as i32 as usize] + right[1 as i32 as usize] * deg;
    dir[2 as i32 as usize] = dir[2 as i32 as usize] + right[2 as i32 as usize] * deg;
    VectorNormalize(dir.as_mut_ptr());
    match (*ent).s.weapon {
        4 => {
            fire_grenade(
                ent as *mut gentity_s,
                (*ent).s.origin.as_mut_ptr(),
                dir.as_mut_ptr(),
            ) as *mut gentity_s;
        }
        5 => {
            fire_rocket(
                ent as *mut gentity_s,
                (*ent).s.origin.as_mut_ptr(),
                dir.as_mut_ptr(),
            ) as *mut gentity_s;
        }
        8 => {
            fire_plasma(
                ent as *mut gentity_s,
                (*ent).s.origin.as_mut_ptr(),
                dir.as_mut_ptr(),
            ) as *mut gentity_s;
        }
        _ => {}
    }
    G_AddEvent(
        ent as *mut gentity_s,
        EV_FIRE_WEAPON as i32,
        0 as i32,
    );
}

unsafe extern "C" fn InitShooter_Finish(mut ent: *mut gentity_t) {
    (*ent).enemy =
        G_PickTarget((*ent).target) as *mut gentity_s;
    (*ent).think = None;
    (*ent).nextthink = 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn InitShooter(mut ent: *mut gentity_t, mut weapon: i32) {
    (*ent).use_0 = Some(
        Use_Shooter
            as unsafe extern "C" fn(
                _: *mut gentity_t,
                _: *mut gentity_t,
                _: *mut gentity_t,
            ) -> (),
    );
    (*ent).s.weapon = weapon;
    RegisterItem(BG_FindItemForWeapon(
        weapon as weapon_t,
    ) as *mut gitem_s
        as *mut gitem_s);
    G_SetMovedir(
        (*ent).s.angles.as_mut_ptr(),
        (*ent).movedir.as_mut_ptr(),
    );
    if (*ent).random == 0. {
        (*ent).random = 1.0f64 as f32
    }
    (*ent).random =
        crate::stdlib::sin(3.14159265358979323846f64 * (*ent).random as f64 / 180 as i32 as f64)
            as f32;
    // target might be a moving object, so we can't set movedir for it
    if !(*ent).target.is_null() {
        (*ent).think = Some(
            InitShooter_Finish as unsafe extern "C" fn(_: *mut gentity_t) -> (),
        );
        (*ent).nextthink = level.time + 500 as i32
    }
    trap_LinkEntity(ent as *mut gentity_s);
}
/*QUAKED shooter_rocket (1 0 0) (-16 -16 -16) (16 16 16)
Fires at either the target or the current direction.
"random" the number of degrees of deviance from the taget. (1.0 default)
*/
#[no_mangle]

pub unsafe extern "C" fn SP_shooter_rocket(mut ent: *mut gentity_t) {
    InitShooter(ent, WP_ROCKET_LAUNCHER as i32);
}
/*QUAKED shooter_plasma (1 0 0) (-16 -16 -16) (16 16 16)
Fires at either the target or the current direction.
"random" is the number of degrees of deviance from the taget. (1.0 default)
*/
#[no_mangle]

pub unsafe extern "C" fn SP_shooter_plasma(mut ent: *mut gentity_t) {
    InitShooter(ent, WP_PLASMAGUN as i32);
}
/*QUAKED shooter_grenade (1 0 0) (-16 -16 -16) (16 16 16)
Fires at either the target or the current direction.
"random" is the number of degrees of deviance from the taget. (1.0 default)
*/
#[no_mangle]

pub unsafe extern "C" fn SP_shooter_grenade(mut ent: *mut gentity_t) {
    InitShooter(ent, WP_GRENADE_LAUNCHER as i32);
}

use ::libc;

pub mod q_shared_h {

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
        *cross.offset(0 as libc::c_int as isize) = *v1.offset(1 as libc::c_int as isize)
            * *v2.offset(2 as libc::c_int as isize)
            - *v1.offset(2 as libc::c_int as isize) * *v2.offset(1 as libc::c_int as isize);
        *cross.offset(1 as libc::c_int as isize) = *v1.offset(2 as libc::c_int as isize)
            * *v2.offset(0 as libc::c_int as isize)
            - *v1.offset(0 as libc::c_int as isize) * *v2.offset(2 as libc::c_int as isize);
        *cross.offset(2 as libc::c_int as isize) = *v1.offset(0 as libc::c_int as isize)
            * *v2.offset(1 as libc::c_int as isize)
            - *v1.offset(1 as libc::c_int as isize) * *v2.offset(0 as libc::c_int as isize);
    }

    // __Q_SHARED_H
}

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
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
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
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
pub use crate::bg_public_h::PERS_ASSIST_COUNT;
pub use crate::bg_public_h::PERS_ATTACKEE_ARMOR;
pub use crate::bg_public_h::PERS_ATTACKER;
pub use crate::bg_public_h::PERS_CAPTURES;
pub use crate::bg_public_h::PERS_DEFEND_COUNT;
pub use crate::bg_public_h::PERS_EXCELLENT_COUNT;
pub use crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT;
pub use crate::bg_public_h::PERS_HITS;
pub use crate::bg_public_h::PERS_IMPRESSIVE_COUNT;
pub use crate::bg_public_h::PERS_KILLED;
pub use crate::bg_public_h::PERS_PLAYEREVENTS;
pub use crate::bg_public_h::PERS_RANK;
pub use crate::bg_public_h::PERS_SCORE;
pub use crate::bg_public_h::PERS_SPAWN_COUNT;
pub use crate::bg_public_h::PERS_TEAM;
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
pub use crate::g_public_h::entityShared_t;
pub use crate::src::game::g_weapon::q_shared_h::CrossProduct;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::DirToByte;
pub use crate::src::qcommon::q_math::PerpendicularVector;
pub use crate::src::qcommon::q_math::Q_crandom;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_math::VectorNormalize2;
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
pub use crate::src::game::g_main::g_gametype;
pub use crate::src::game::g_main::g_quadfactor;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_missile::fire_bfg;
pub use crate::src::game::g_missile::fire_grapple;
pub use crate::src::game::g_missile::fire_grenade;
pub use crate::src::game::g_missile::fire_plasma;
pub use crate::src::game::g_missile::fire_rocket;
pub use crate::src::game::g_syscalls::trap_LinkEntity;
pub use crate::src::game::g_syscalls::trap_Trace;
pub use crate::src::game::g_syscalls::trap_UnlinkEntity;
pub use crate::src::game::g_team::OnSameTeam;
pub use crate::src::game::g_utils::G_AddEvent;
pub use crate::src::game::g_utils::G_FreeEntity;
pub use crate::src::game::g_utils::G_SetOrigin;
pub use crate::src::game::g_utils::G_TempEntity;

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
// g_weapon.c
// perform the server side effects of a weapon firing

static mut s_quadFactor: libc::c_float = 0.;

static mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];

static mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];

static mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];

static mut muzzle: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
/*
================
G_BounceProjectile
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_BounceProjectile(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut impact: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut endout: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut newv: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dot: libc::c_float = 0.;
    v[0 as libc::c_int as usize] =
        *impact.offset(0 as libc::c_int as isize) - *start.offset(0 as libc::c_int as isize);
    v[1 as libc::c_int as usize] =
        *impact.offset(1 as libc::c_int as isize) - *start.offset(1 as libc::c_int as isize);
    v[2 as libc::c_int as usize] =
        *impact.offset(2 as libc::c_int as isize) - *start.offset(2 as libc::c_int as isize);
    dot = v[0 as libc::c_int as usize] * *dir.offset(0 as libc::c_int as isize)
        + v[1 as libc::c_int as usize] * *dir.offset(1 as libc::c_int as isize)
        + v[2 as libc::c_int as usize] * *dir.offset(2 as libc::c_int as isize);
    newv[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        + *dir.offset(0 as libc::c_int as isize) * (-(2 as libc::c_int) as libc::c_float * dot);
    newv[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        + *dir.offset(1 as libc::c_int as isize) * (-(2 as libc::c_int) as libc::c_float * dot);
    newv[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        + *dir.offset(2 as libc::c_int as isize) * (-(2 as libc::c_int) as libc::c_float * dot);
    crate::src::qcommon::q_math::VectorNormalize(newv.as_mut_ptr());
    *endout.offset(0 as libc::c_int as isize) = *impact.offset(0 as libc::c_int as isize)
        + newv[0 as libc::c_int as usize] * 8192 as libc::c_int as libc::c_float;
    *endout.offset(1 as libc::c_int as isize) = *impact.offset(1 as libc::c_int as isize)
        + newv[1 as libc::c_int as usize] * 8192 as libc::c_int as libc::c_float;
    *endout.offset(2 as libc::c_int as isize) = *impact.offset(2 as libc::c_int as isize)
        + newv[2 as libc::c_int as usize] * 8192 as libc::c_int as libc::c_float;
}
/*
======================================================================

GAUNTLET

======================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn Weapon_Gauntlet(mut _ent: *mut crate::g_local_h::gentity_t) {}
/*
===============
CheckGauntletAttack
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CheckGauntletAttack(
    mut ent: *mut crate::g_local_h::gentity_t,
) -> crate::src::qcommon::q_shared::qboolean {
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
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut tent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut traceEnt: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut damage: libc::c_int = 0;
    // set aiming directions
    crate::src::qcommon::q_math::AngleVectors(
        (*(*ent).client).ps.viewangles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    CalcMuzzlePoint(
        ent,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
        muzzle.as_mut_ptr(),
    );
    end[0 as libc::c_int as usize] = muzzle[0 as libc::c_int as usize]
        + forward[0 as libc::c_int as usize] * 32 as libc::c_int as libc::c_float;
    end[1 as libc::c_int as usize] = muzzle[1 as libc::c_int as usize]
        + forward[1 as libc::c_int as usize] * 32 as libc::c_int as libc::c_float;
    end[2 as libc::c_int as usize] = muzzle[2 as libc::c_int as usize]
        + forward[2 as libc::c_int as usize] * 32 as libc::c_int as libc::c_float;
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        muzzle.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*ent).s.number,
        1 as libc::c_int | 0x2000000 as libc::c_int | 0x4000000 as libc::c_int,
    );
    if tr.surfaceFlags & 0x10 as libc::c_int != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*(*ent).client).noclip as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    traceEnt = &mut *crate::src::game::g_main::g_entities
        .as_mut_ptr()
        .offset(tr.entityNum as isize) as *mut crate::g_local_h::gentity_t;
    // send blood impact
    if (*traceEnt).takedamage as libc::c_uint != 0 && !(*traceEnt).client.is_null() {
        tent = crate::src::game::g_utils::G_TempEntity(
            tr.endpos.as_mut_ptr(),
            crate::bg_public_h::EV_MISSILE_HIT as libc::c_int,
        ) as *mut crate::g_local_h::gentity_s;
        (*tent).s.otherEntityNum = (*traceEnt).s.number;
        (*tent).s.eventParm = crate::src::qcommon::q_math::DirToByte(tr.plane.normal.as_mut_ptr());
        (*tent).s.weapon = (*ent).s.weapon
    }
    if (*traceEnt).takedamage as u64 == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*(*ent).client).ps.powerups[crate::bg_public_h::PW_QUAD as libc::c_int as usize] != 0 {
        crate::src::game::g_utils::G_AddEvent(
            ent as *mut crate::g_local_h::gentity_s,
            crate::bg_public_h::EV_POWERUP_QUAD as libc::c_int,
            0 as libc::c_int,
        );
        s_quadFactor = crate::src::game::g_main::g_quadfactor.value
    } else {
        s_quadFactor = 1 as libc::c_int as libc::c_float
    }
    damage = (50 as libc::c_int as libc::c_float * s_quadFactor) as libc::c_int;
    crate::src::game::g_combat::G_Damage(
        traceEnt as *mut crate::g_local_h::gentity_s,
        ent as *mut crate::g_local_h::gentity_s,
        ent as *mut crate::g_local_h::gentity_s,
        forward.as_mut_ptr(),
        tr.endpos.as_mut_ptr(),
        damage,
        0 as libc::c_int,
        crate::bg_public_h::MOD_GAUNTLET as libc::c_int,
    );
    return crate::src::qcommon::q_shared::qtrue;
}
/*
======================================================================

MACHINEGUN

======================================================================
*/
/*
======================
SnapVectorTowards

Round a vector to integers for more efficient network
transmission, but make sure that it rounds towards a given point
rather than blindly truncating.  This prevents it from truncating
into a wall.
======================
*/
#[no_mangle]

pub unsafe extern "C" fn SnapVectorTowards(
    mut v: *mut crate::src::qcommon::q_shared::vec_t,
    mut to: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *to.offset(i as isize) <= *v.offset(i as isize) {
            *v.offset(i as isize) = crate::stdlib::floor(*v.offset(i as isize) as libc::c_double)
                as crate::src::qcommon::q_shared::vec_t
        } else {
            *v.offset(i as isize) = crate::stdlib::ceil(*v.offset(i as isize) as libc::c_double)
                as crate::src::qcommon::q_shared::vec_t
        }
        i += 1
    }
}
// wimpier MG in teamplay
#[no_mangle]

pub unsafe extern "C" fn Bullet_Fire(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut spread: libc::c_float,
    mut damage: libc::c_int,
    mut mod_0: libc::c_int,
) {
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
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut r: libc::c_float = 0.;
    let mut u: libc::c_float = 0.;
    let mut tent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut traceEnt: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut i: libc::c_int = 0;
    let mut passent: libc::c_int = 0;
    damage = (damage as libc::c_float * s_quadFactor) as libc::c_int;
    r = (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
        * 3.14159265358979323846f64
        * 2.0f32 as libc::c_double) as libc::c_float;
    u = (crate::stdlib::sin(r as libc::c_double)
        * (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64))
        * spread as libc::c_double
        * 16 as libc::c_int as libc::c_double) as libc::c_float;
    r = (crate::stdlib::cos(r as libc::c_double)
        * (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64))
        * spread as libc::c_double
        * 16 as libc::c_int as libc::c_double) as libc::c_float;
    end[0 as libc::c_int as usize] = muzzle[0 as libc::c_int as usize]
        + forward[0 as libc::c_int as usize]
            * (8192 as libc::c_int * 16 as libc::c_int) as libc::c_float;
    end[1 as libc::c_int as usize] = muzzle[1 as libc::c_int as usize]
        + forward[1 as libc::c_int as usize]
            * (8192 as libc::c_int * 16 as libc::c_int) as libc::c_float;
    end[2 as libc::c_int as usize] = muzzle[2 as libc::c_int as usize]
        + forward[2 as libc::c_int as usize]
            * (8192 as libc::c_int * 16 as libc::c_int) as libc::c_float;
    end[0 as libc::c_int as usize] =
        end[0 as libc::c_int as usize] + right[0 as libc::c_int as usize] * r;
    end[1 as libc::c_int as usize] =
        end[1 as libc::c_int as usize] + right[1 as libc::c_int as usize] * r;
    end[2 as libc::c_int as usize] =
        end[2 as libc::c_int as usize] + right[2 as libc::c_int as usize] * r;
    end[0 as libc::c_int as usize] =
        end[0 as libc::c_int as usize] + up[0 as libc::c_int as usize] * u;
    end[1 as libc::c_int as usize] =
        end[1 as libc::c_int as usize] + up[1 as libc::c_int as usize] * u;
    end[2 as libc::c_int as usize] =
        end[2 as libc::c_int as usize] + up[2 as libc::c_int as usize] * u;
    passent = (*ent).s.number;
    i = 0 as libc::c_int;
    if i < 10 as libc::c_int {
        crate::src::game::g_syscalls::trap_Trace(
            &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
            muzzle.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            0 as *const crate::src::qcommon::q_shared::vec_t,
            0 as *const crate::src::qcommon::q_shared::vec_t,
            end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            passent,
            1 as libc::c_int | 0x2000000 as libc::c_int | 0x4000000 as libc::c_int,
        );
        if tr.surfaceFlags & 0x10 as libc::c_int != 0 {
            return;
        }
        traceEnt = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(tr.entityNum as isize) as *mut crate::g_local_h::gentity_t;
        // snap the endpos to integers, but nudged towards the line
        SnapVectorTowards(tr.endpos.as_mut_ptr(), muzzle.as_mut_ptr());
        // send bullet impact
        if (*traceEnt).takedamage as libc::c_uint != 0 && !(*traceEnt).client.is_null() {
            tent = crate::src::game::g_utils::G_TempEntity(
                tr.endpos.as_mut_ptr(),
                crate::bg_public_h::EV_BULLET_HIT_FLESH as libc::c_int,
            ) as *mut crate::g_local_h::gentity_s;
            (*tent).s.eventParm = (*traceEnt).s.number;
            if LogAccuracyHit(traceEnt, ent) as u64 != 0 {
                (*(*ent).client).accuracy_hits += 1
            }
        } else {
            tent = crate::src::game::g_utils::G_TempEntity(
                tr.endpos.as_mut_ptr(),
                crate::bg_public_h::EV_BULLET_HIT_WALL as libc::c_int,
            ) as *mut crate::g_local_h::gentity_s;
            (*tent).s.eventParm =
                crate::src::qcommon::q_math::DirToByte(tr.plane.normal.as_mut_ptr())
        }
        (*tent).s.otherEntityNum = (*ent).s.number;
        if (*traceEnt).takedamage as u64 != 0 {
            crate::src::game::g_combat::G_Damage(
                traceEnt as *mut crate::g_local_h::gentity_s,
                ent as *mut crate::g_local_h::gentity_s,
                ent as *mut crate::g_local_h::gentity_s,
                forward.as_mut_ptr(),
                tr.endpos.as_mut_ptr(),
                damage,
                0 as libc::c_int,
                mod_0,
            );
        }
    };
}
/*
======================================================================

BFG

======================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn BFG_Fire(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut m: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    m = crate::src::game::g_missile::fire_bfg(
        ent as *mut crate::g_local_h::gentity_s,
        muzzle.as_mut_ptr(),
        forward.as_mut_ptr(),
    ) as *mut crate::g_local_h::gentity_s;
    (*m).damage = ((*m).damage as libc::c_float * s_quadFactor) as libc::c_int;
    (*m).splashDamage = ((*m).splashDamage as libc::c_float * s_quadFactor) as libc::c_int;
    //	VectorAdd( m->s.pos.trDelta, ent->client->ps.velocity, m->s.pos.trDelta );	// "real" physics
}
#[no_mangle]

pub unsafe extern "C" fn ShotgunPellet(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut ent: *mut crate::g_local_h::gentity_t,
) -> crate::src::qcommon::q_shared::qboolean {
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
    let mut damage: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut passent: libc::c_int = 0;
    let mut traceEnt: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut tr_start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut tr_end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut hitClient: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    passent = (*ent).s.number;
    tr_start[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    tr_start[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    tr_start[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    tr_end[0 as libc::c_int as usize] = *end.offset(0 as libc::c_int as isize);
    tr_end[1 as libc::c_int as usize] = *end.offset(1 as libc::c_int as isize);
    tr_end[2 as libc::c_int as usize] = *end.offset(2 as libc::c_int as isize);
    i = 0 as libc::c_int;
    if i < 10 as libc::c_int {
        crate::src::game::g_syscalls::trap_Trace(
            &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
            tr_start.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            0 as *const crate::src::qcommon::q_shared::vec_t,
            0 as *const crate::src::qcommon::q_shared::vec_t,
            tr_end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            passent,
            1 as libc::c_int | 0x2000000 as libc::c_int | 0x4000000 as libc::c_int,
        );
        traceEnt = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(tr.entityNum as isize) as *mut crate::g_local_h::gentity_t;
        // send bullet impact
        if tr.surfaceFlags & 0x10 as libc::c_int != 0 {
            return crate::src::qcommon::q_shared::qfalse;
        }
        if (*traceEnt).takedamage as u64 != 0 {
            damage = (10 as libc::c_int as libc::c_float * s_quadFactor) as libc::c_int;
            if LogAccuracyHit(traceEnt, ent) as u64 != 0 {
                hitClient = crate::src::qcommon::q_shared::qtrue
            }
            crate::src::game::g_combat::G_Damage(
                traceEnt as *mut crate::g_local_h::gentity_s,
                ent as *mut crate::g_local_h::gentity_s,
                ent as *mut crate::g_local_h::gentity_s,
                forward.as_mut_ptr(),
                tr.endpos.as_mut_ptr(),
                damage,
                0 as libc::c_int,
                crate::bg_public_h::MOD_SHOTGUN as libc::c_int,
            );
            return hitClient;
        }
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
// this should match CG_ShotgunPattern
#[no_mangle]

pub unsafe extern "C" fn ShotgunPattern(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut origin2: *mut crate::src::qcommon::q_shared::vec_t,
    mut seed: libc::c_int,
    mut ent: *mut crate::g_local_h::gentity_t,
) {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_float = 0.;
    let mut u: libc::c_float = 0.;
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut forward_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut hitClient: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    // derive the right and up vectors from the forward vector, because
    // the client won't have any other information
    crate::src::qcommon::q_math::VectorNormalize2(
        origin2 as *const crate::src::qcommon::q_shared::vec_t,
        forward_0.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::PerpendicularVector(
        right_0.as_mut_ptr(),
        forward_0.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    );
    CrossProduct(
        forward_0.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        right_0.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        up_0.as_mut_ptr(),
    );
    // generate the "random" spread pattern
    i = 0 as libc::c_int;
    while i < 11 as libc::c_int {
        r = crate::src::qcommon::q_math::Q_crandom(&mut seed)
            * 700 as libc::c_int as libc::c_float
            * 16 as libc::c_int as libc::c_float;
        u = crate::src::qcommon::q_math::Q_crandom(&mut seed)
            * 700 as libc::c_int as libc::c_float
            * 16 as libc::c_int as libc::c_float;
        end[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize)
            + forward_0[0 as libc::c_int as usize]
                * (8192 as libc::c_int * 16 as libc::c_int) as libc::c_float;
        end[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize)
            + forward_0[1 as libc::c_int as usize]
                * (8192 as libc::c_int * 16 as libc::c_int) as libc::c_float;
        end[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize)
            + forward_0[2 as libc::c_int as usize]
                * (8192 as libc::c_int * 16 as libc::c_int) as libc::c_float;
        end[0 as libc::c_int as usize] =
            end[0 as libc::c_int as usize] + right_0[0 as libc::c_int as usize] * r;
        end[1 as libc::c_int as usize] =
            end[1 as libc::c_int as usize] + right_0[1 as libc::c_int as usize] * r;
        end[2 as libc::c_int as usize] =
            end[2 as libc::c_int as usize] + right_0[2 as libc::c_int as usize] * r;
        end[0 as libc::c_int as usize] =
            end[0 as libc::c_int as usize] + up_0[0 as libc::c_int as usize] * u;
        end[1 as libc::c_int as usize] =
            end[1 as libc::c_int as usize] + up_0[1 as libc::c_int as usize] * u;
        end[2 as libc::c_int as usize] =
            end[2 as libc::c_int as usize] + up_0[2 as libc::c_int as usize] * u;
        if ShotgunPellet(origin, end.as_mut_ptr(), ent) as libc::c_uint != 0
            && hitClient as u64 == 0
        {
            hitClient = crate::src::qcommon::q_shared::qtrue;
            (*(*ent).client).accuracy_hits += 1
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn weapon_supershotgun_fire(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut tent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    // send shotgun blast
    tent = crate::src::game::g_utils::G_TempEntity(
        muzzle.as_mut_ptr(),
        crate::bg_public_h::EV_SHOTGUN as libc::c_int,
    ) as *mut crate::g_local_h::gentity_s; // seed for spread pattern
    (*tent).s.origin2[0 as libc::c_int as usize] =
        forward[0 as libc::c_int as usize] * 4096 as libc::c_int as libc::c_float;
    (*tent).s.origin2[1 as libc::c_int as usize] =
        forward[1 as libc::c_int as usize] * 4096 as libc::c_int as libc::c_float;
    (*tent).s.origin2[2 as libc::c_int as usize] =
        forward[2 as libc::c_int as usize] * 4096 as libc::c_int as libc::c_float;
    (*tent).s.origin2[0 as libc::c_int as usize] = (*tent).s.origin2[0 as libc::c_int as usize]
        as libc::c_int
        as crate::src::qcommon::q_shared::vec_t;
    (*tent).s.origin2[1 as libc::c_int as usize] = (*tent).s.origin2[1 as libc::c_int as usize]
        as libc::c_int
        as crate::src::qcommon::q_shared::vec_t;
    (*tent).s.origin2[2 as libc::c_int as usize] = (*tent).s.origin2[2 as libc::c_int as usize]
        as libc::c_int
        as crate::src::qcommon::q_shared::vec_t;
    (*tent).s.eventParm = ::libc::rand() & 255 as libc::c_int;
    (*tent).s.otherEntityNum = (*ent).s.number;
    ShotgunPattern(
        (*tent).s.pos.trBase.as_mut_ptr(),
        (*tent).s.origin2.as_mut_ptr(),
        (*tent).s.eventParm,
        ent,
    );
}
/*
======================================================================

GRENADE LAUNCHER

======================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn weapon_grenadelauncher_fire(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut m: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    // extra vertical velocity
    forward[2 as libc::c_int as usize] += 0.2f32;
    crate::src::qcommon::q_math::VectorNormalize(forward.as_mut_ptr());
    m = crate::src::game::g_missile::fire_grenade(
        ent as *mut crate::g_local_h::gentity_s,
        muzzle.as_mut_ptr(),
        forward.as_mut_ptr(),
    ) as *mut crate::g_local_h::gentity_s;
    (*m).damage = ((*m).damage as libc::c_float * s_quadFactor) as libc::c_int;
    (*m).splashDamage = ((*m).splashDamage as libc::c_float * s_quadFactor) as libc::c_int;
    //	VectorAdd( m->s.pos.trDelta, ent->client->ps.velocity, m->s.pos.trDelta );	// "real" physics
}
/*
======================================================================

ROCKET

======================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn Weapon_RocketLauncher_Fire(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut m: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    m = crate::src::game::g_missile::fire_rocket(
        ent as *mut crate::g_local_h::gentity_s,
        muzzle.as_mut_ptr(),
        forward.as_mut_ptr(),
    ) as *mut crate::g_local_h::gentity_s;
    (*m).damage = ((*m).damage as libc::c_float * s_quadFactor) as libc::c_int;
    (*m).splashDamage = ((*m).splashDamage as libc::c_float * s_quadFactor) as libc::c_int;
    //	VectorAdd( m->s.pos.trDelta, ent->client->ps.velocity, m->s.pos.trDelta );	// "real" physics
}
/*
======================================================================

PLASMA GUN

======================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn Weapon_Plasmagun_Fire(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut m: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    m = crate::src::game::g_missile::fire_plasma(
        ent as *mut crate::g_local_h::gentity_s,
        muzzle.as_mut_ptr(),
        forward.as_mut_ptr(),
    ) as *mut crate::g_local_h::gentity_s;
    (*m).damage = ((*m).damage as libc::c_float * s_quadFactor) as libc::c_int;
    (*m).splashDamage = ((*m).splashDamage as libc::c_float * s_quadFactor) as libc::c_int;
    //	VectorAdd( m->s.pos.trDelta, ent->client->ps.velocity, m->s.pos.trDelta );	// "real" physics
}
#[no_mangle]

pub unsafe extern "C" fn weapon_railgun_fire(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut trace: crate::src::qcommon::q_shared::trace_t =
        crate::src::qcommon::q_shared::trace_t {
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
    let mut tent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut traceEnt: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut damage: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut hits: libc::c_int = 0;
    let mut unlinked: libc::c_int = 0;
    let mut passent: libc::c_int = 0;
    let mut unlinkedEntities: [*mut crate::g_local_h::gentity_t; 4] =
        [0 as *mut crate::g_local_h::gentity_t; 4];
    damage = (100 as libc::c_int as libc::c_float * s_quadFactor) as libc::c_int;
    end[0 as libc::c_int as usize] = muzzle[0 as libc::c_int as usize]
        + forward[0 as libc::c_int as usize] * 8192 as libc::c_int as libc::c_float;
    end[1 as libc::c_int as usize] = muzzle[1 as libc::c_int as usize]
        + forward[1 as libc::c_int as usize] * 8192 as libc::c_int as libc::c_float;
    end[2 as libc::c_int as usize] = muzzle[2 as libc::c_int as usize]
        + forward[2 as libc::c_int as usize] * 8192 as libc::c_int as libc::c_float;
    // trace only against the solids, so the railgun will go through people
    unlinked = 0 as libc::c_int;
    hits = 0 as libc::c_int;
    passent = (*ent).s.number;
    loop {
        crate::src::game::g_syscalls::trap_Trace(
            &mut trace as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
            muzzle.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            0 as *const crate::src::qcommon::q_shared::vec_t,
            0 as *const crate::src::qcommon::q_shared::vec_t,
            end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            passent,
            1 as libc::c_int | 0x2000000 as libc::c_int | 0x4000000 as libc::c_int,
        );
        if trace.entityNum >= ((1 as libc::c_int) << 10 as libc::c_int) - 2 as libc::c_int {
            break;
        }
        traceEnt = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(trace.entityNum as isize)
            as *mut crate::g_local_h::gentity_t;
        if (*traceEnt).takedamage as u64 != 0 {
            if LogAccuracyHit(traceEnt, ent) as u64 != 0 {
                hits += 1
            }
            crate::src::game::g_combat::G_Damage(
                traceEnt as *mut crate::g_local_h::gentity_s,
                ent as *mut crate::g_local_h::gentity_s,
                ent as *mut crate::g_local_h::gentity_s,
                forward.as_mut_ptr(),
                trace.endpos.as_mut_ptr(),
                damage,
                0 as libc::c_int,
                crate::bg_public_h::MOD_RAILGUN as libc::c_int,
            );
        }
        if trace.contents & 1 as libc::c_int != 0 {
            break;
        }
        // unlink this entity, so the next trace will go past it
        crate::src::game::g_syscalls::trap_UnlinkEntity(
            traceEnt as *mut crate::g_local_h::gentity_s,
        );
        unlinkedEntities[unlinked as usize] = traceEnt;
        unlinked += 1;
        if !(unlinked < 4 as libc::c_int) {
            break;
        }
    }
    // link back in any entities we unlinked
    i = 0 as libc::c_int;
    while i < unlinked {
        crate::src::game::g_syscalls::trap_LinkEntity(
            unlinkedEntities[i as usize] as *mut crate::g_local_h::gentity_s,
        );
        i += 1
    }
    // the final trace endpos will be the terminal point of the rail trail
    // snap the endpos to integers to save net bandwidth, but nudged towards the line
    SnapVectorTowards(trace.endpos.as_mut_ptr(), muzzle.as_mut_ptr());
    // send railgun beam effect
    tent = crate::src::game::g_utils::G_TempEntity(
        trace.endpos.as_mut_ptr(),
        crate::bg_public_h::EV_RAILTRAIL as libc::c_int,
    ) as *mut crate::g_local_h::gentity_s;
    // set player number for custom colors on the railtrail
    (*tent).s.clientNum = (*ent).s.clientNum;
    (*tent).s.origin2[0 as libc::c_int as usize] = muzzle[0 as libc::c_int as usize];
    (*tent).s.origin2[1 as libc::c_int as usize] = muzzle[1 as libc::c_int as usize];
    (*tent).s.origin2[2 as libc::c_int as usize] = muzzle[2 as libc::c_int as usize];
    // move origin a bit to come closer to the drawn gun muzzle
    (*tent).s.origin2[0 as libc::c_int as usize] = (*tent).s.origin2[0 as libc::c_int as usize]
        + right[0 as libc::c_int as usize] * 4 as libc::c_int as libc::c_float;
    (*tent).s.origin2[1 as libc::c_int as usize] = (*tent).s.origin2[1 as libc::c_int as usize]
        + right[1 as libc::c_int as usize] * 4 as libc::c_int as libc::c_float;
    (*tent).s.origin2[2 as libc::c_int as usize] = (*tent).s.origin2[2 as libc::c_int as usize]
        + right[2 as libc::c_int as usize] * 4 as libc::c_int as libc::c_float;
    (*tent).s.origin2[0 as libc::c_int as usize] = (*tent).s.origin2[0 as libc::c_int as usize]
        + up[0 as libc::c_int as usize] * -(1 as libc::c_int) as libc::c_float;
    (*tent).s.origin2[1 as libc::c_int as usize] = (*tent).s.origin2[1 as libc::c_int as usize]
        + up[1 as libc::c_int as usize] * -(1 as libc::c_int) as libc::c_float;
    (*tent).s.origin2[2 as libc::c_int as usize] = (*tent).s.origin2[2 as libc::c_int as usize]
        + up[2 as libc::c_int as usize] * -(1 as libc::c_int) as libc::c_float;
    // no explosion at end if SURF_NOIMPACT, but still make the trail
    if trace.surfaceFlags & 0x10 as libc::c_int != 0 {
        (*tent).s.eventParm = 255 as libc::c_int
    // don't make the explosion at the end
    } else {
        (*tent).s.eventParm =
            crate::src::qcommon::q_math::DirToByte(trace.plane.normal.as_mut_ptr())
    }
    (*tent).s.clientNum = (*ent).s.clientNum;
    // give the shooter a reward sound if they have made two railgun hits in a row
    if hits == 0 as libc::c_int {
        // complete miss
        (*(*ent).client).accurateCount = 0 as libc::c_int
    } else {
        // check for "impressive" reward sound
        (*(*ent).client).accurateCount += hits;
        if (*(*ent).client).accurateCount >= 2 as libc::c_int {
            (*(*ent).client).accurateCount -= 2 as libc::c_int;
            (*(*ent).client).ps.persistant
                [crate::bg_public_h::PERS_IMPRESSIVE_COUNT as libc::c_int as usize] += 1;
            // add the sprite over the player's head
            (*(*ent).client).ps.eFlags &= !(0x8000 as libc::c_int
                | 0x8 as libc::c_int
                | 0x40 as libc::c_int
                | 0x20000 as libc::c_int
                | 0x10000 as libc::c_int
                | 0x800 as libc::c_int);
            (*(*ent).client).ps.eFlags |= 0x8000 as libc::c_int;
            (*(*ent).client).rewardTime = crate::src::game::g_main::level.time + 2000 as libc::c_int
        }
        (*(*ent).client).accuracy_hits += 1
    };
}
/*
======================================================================

GRAPPLING HOOK

======================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn Weapon_GrapplingHook_Fire(mut ent: *mut crate::g_local_h::gentity_t) {
    if (*(*ent).client).fireHeld as u64 == 0 && (*(*ent).client).hook.is_null() {
        crate::src::game::g_missile::fire_grapple(
            ent as *mut crate::g_local_h::gentity_s,
            muzzle.as_mut_ptr(),
            forward.as_mut_ptr(),
        ) as *mut crate::g_local_h::gentity_s; // save net bandwidth
    }
    (*(*ent).client).fireHeld = crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn Weapon_HookFree(mut ent: *mut crate::g_local_h::gentity_t) {
    (*(*(*ent).parent).client).hook = 0 as *mut crate::g_local_h::gentity_t;
    (*(*(*ent).parent).client).ps.pm_flags &= !(2048 as libc::c_int);
    crate::src::game::g_utils::G_FreeEntity(ent as *mut crate::g_local_h::gentity_s);
}
#[no_mangle]

pub unsafe extern "C" fn Weapon_HookThink(mut ent: *mut crate::g_local_h::gentity_t) {
    if !(*ent).enemy.is_null() {
        let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut oldorigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        oldorigin[0 as libc::c_int as usize] = (*ent).r.currentOrigin[0 as libc::c_int as usize];
        oldorigin[1 as libc::c_int as usize] = (*ent).r.currentOrigin[1 as libc::c_int as usize];
        oldorigin[2 as libc::c_int as usize] = (*ent).r.currentOrigin[2 as libc::c_int as usize];
        v[0 as libc::c_int as usize] =
            ((*(*ent).enemy).r.currentOrigin[0 as libc::c_int as usize] as libc::c_double
                + ((*(*ent).enemy).r.mins[0 as libc::c_int as usize]
                    + (*(*ent).enemy).r.maxs[0 as libc::c_int as usize])
                    as libc::c_double
                    * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
        v[1 as libc::c_int as usize] =
            ((*(*ent).enemy).r.currentOrigin[1 as libc::c_int as usize] as libc::c_double
                + ((*(*ent).enemy).r.mins[1 as libc::c_int as usize]
                    + (*(*ent).enemy).r.maxs[1 as libc::c_int as usize])
                    as libc::c_double
                    * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
        v[2 as libc::c_int as usize] =
            ((*(*ent).enemy).r.currentOrigin[2 as libc::c_int as usize] as libc::c_double
                + ((*(*ent).enemy).r.mins[2 as libc::c_int as usize]
                    + (*(*ent).enemy).r.maxs[2 as libc::c_int as usize])
                    as libc::c_double
                    * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
        SnapVectorTowards(v.as_mut_ptr(), oldorigin.as_mut_ptr());
        crate::src::game::g_utils::G_SetOrigin(
            ent as *mut crate::g_local_h::gentity_s,
            v.as_mut_ptr(),
        );
    }
    (*(*(*ent).parent).client).ps.grapplePoint[0 as libc::c_int as usize] =
        (*ent).r.currentOrigin[0 as libc::c_int as usize];
    (*(*(*ent).parent).client).ps.grapplePoint[1 as libc::c_int as usize] =
        (*ent).r.currentOrigin[1 as libc::c_int as usize];
    (*(*(*ent).parent).client).ps.grapplePoint[2 as libc::c_int as usize] =
        (*ent).r.currentOrigin[2 as libc::c_int as usize];
}
/*
======================================================================

LIGHTNING GUN

======================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn Weapon_LightningFire(mut ent: *mut crate::g_local_h::gentity_t) {
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
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut traceEnt: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut tent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut damage: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut passent: libc::c_int = 0;
    damage = (8 as libc::c_int as libc::c_float * s_quadFactor) as libc::c_int;
    passent = (*ent).s.number;
    i = 0 as libc::c_int;
    if i < 10 as libc::c_int {
        end[0 as libc::c_int as usize] = muzzle[0 as libc::c_int as usize]
            + forward[0 as libc::c_int as usize] * 768 as libc::c_int as libc::c_float;
        end[1 as libc::c_int as usize] = muzzle[1 as libc::c_int as usize]
            + forward[1 as libc::c_int as usize] * 768 as libc::c_int as libc::c_float;
        end[2 as libc::c_int as usize] = muzzle[2 as libc::c_int as usize]
            + forward[2 as libc::c_int as usize] * 768 as libc::c_int as libc::c_float;
        crate::src::game::g_syscalls::trap_Trace(
            &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
            muzzle.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            0 as *const crate::src::qcommon::q_shared::vec_t,
            0 as *const crate::src::qcommon::q_shared::vec_t,
            end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            passent,
            1 as libc::c_int | 0x2000000 as libc::c_int | 0x4000000 as libc::c_int,
        );
        if tr.entityNum == ((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int {
            return;
        }
        traceEnt = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(tr.entityNum as isize) as *mut crate::g_local_h::gentity_t;
        if (*traceEnt).takedamage as u64 != 0 {
            if LogAccuracyHit(traceEnt, ent) as u64 != 0 {
                (*(*ent).client).accuracy_hits += 1
            }
            crate::src::game::g_combat::G_Damage(
                traceEnt as *mut crate::g_local_h::gentity_s,
                ent as *mut crate::g_local_h::gentity_s,
                ent as *mut crate::g_local_h::gentity_s,
                forward.as_mut_ptr(),
                tr.endpos.as_mut_ptr(),
                damage,
                0 as libc::c_int,
                crate::bg_public_h::MOD_LIGHTNING as libc::c_int,
            );
        }
        if (*traceEnt).takedamage as libc::c_uint != 0 && !(*traceEnt).client.is_null() {
            tent = crate::src::game::g_utils::G_TempEntity(
                tr.endpos.as_mut_ptr(),
                crate::bg_public_h::EV_MISSILE_HIT as libc::c_int,
            ) as *mut crate::g_local_h::gentity_s;
            (*tent).s.otherEntityNum = (*traceEnt).s.number;
            (*tent).s.eventParm =
                crate::src::qcommon::q_math::DirToByte(tr.plane.normal.as_mut_ptr());
            (*tent).s.weapon = (*ent).s.weapon
        } else if tr.surfaceFlags & 0x10 as libc::c_int == 0 {
            tent = crate::src::game::g_utils::G_TempEntity(
                tr.endpos.as_mut_ptr(),
                crate::bg_public_h::EV_MISSILE_MISS as libc::c_int,
            ) as *mut crate::g_local_h::gentity_s;
            (*tent).s.eventParm =
                crate::src::qcommon::q_math::DirToByte(tr.plane.normal.as_mut_ptr())
        }
    };
}
//======================================================================
/*
===============
LogAccuracyHit
===============
*/
#[no_mangle]

pub unsafe extern "C" fn LogAccuracyHit(
    mut target: *mut crate::g_local_h::gentity_t,
    mut attacker: *mut crate::g_local_h::gentity_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*target).takedamage as u64 == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if target == attacker {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*target).client.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*attacker).client.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*(*target).client).ps.stats[crate::bg_public_h::STAT_HEALTH as libc::c_int as usize]
        <= 0 as libc::c_int
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if crate::src::game::g_team::OnSameTeam(
        target as *mut crate::g_local_h::gentity_s,
        attacker as *mut crate::g_local_h::gentity_s,
    ) as u64
        != 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
CalcMuzzlePoint

set muzzle location relative to pivoting eye
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CalcMuzzlePoint(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut forward_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut _right_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut _up_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut muzzlePoint: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *muzzlePoint.offset(0 as libc::c_int as isize) = (*ent).s.pos.trBase[0 as libc::c_int as usize];
    *muzzlePoint.offset(1 as libc::c_int as isize) = (*ent).s.pos.trBase[1 as libc::c_int as usize];
    *muzzlePoint.offset(2 as libc::c_int as isize) = (*ent).s.pos.trBase[2 as libc::c_int as usize];
    let ref mut fresh0 = *muzzlePoint.offset(2 as libc::c_int as isize);
    *fresh0 += (*(*ent).client).ps.viewheight as libc::c_float;
    *muzzlePoint.offset(0 as libc::c_int as isize) = *muzzlePoint.offset(0 as libc::c_int as isize)
        + *forward_0.offset(0 as libc::c_int as isize) * 14 as libc::c_int as libc::c_float;
    *muzzlePoint.offset(1 as libc::c_int as isize) = *muzzlePoint.offset(1 as libc::c_int as isize)
        + *forward_0.offset(1 as libc::c_int as isize) * 14 as libc::c_int as libc::c_float;
    *muzzlePoint.offset(2 as libc::c_int as isize) = *muzzlePoint.offset(2 as libc::c_int as isize)
        + *forward_0.offset(2 as libc::c_int as isize) * 14 as libc::c_int as libc::c_float;
    // snap to integer coordinates for more efficient network bandwidth usage
    *muzzlePoint.offset(0 as libc::c_int as isize) = *muzzlePoint.offset(0 as libc::c_int as isize)
        as libc::c_int
        as crate::src::qcommon::q_shared::vec_t;
    *muzzlePoint.offset(1 as libc::c_int as isize) = *muzzlePoint.offset(1 as libc::c_int as isize)
        as libc::c_int
        as crate::src::qcommon::q_shared::vec_t;
    *muzzlePoint.offset(2 as libc::c_int as isize) = *muzzlePoint.offset(2 as libc::c_int as isize)
        as libc::c_int
        as crate::src::qcommon::q_shared::vec_t;
}
/*
===============
CalcMuzzlePointOrigin

set muzzle location relative to pivoting eye
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CalcMuzzlePointOrigin(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut _origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut forward_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut _right_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut _up_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut muzzlePoint: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *muzzlePoint.offset(0 as libc::c_int as isize) = (*ent).s.pos.trBase[0 as libc::c_int as usize];
    *muzzlePoint.offset(1 as libc::c_int as isize) = (*ent).s.pos.trBase[1 as libc::c_int as usize];
    *muzzlePoint.offset(2 as libc::c_int as isize) = (*ent).s.pos.trBase[2 as libc::c_int as usize];
    let ref mut fresh1 = *muzzlePoint.offset(2 as libc::c_int as isize);
    *fresh1 += (*(*ent).client).ps.viewheight as libc::c_float;
    *muzzlePoint.offset(0 as libc::c_int as isize) = *muzzlePoint.offset(0 as libc::c_int as isize)
        + *forward_0.offset(0 as libc::c_int as isize) * 14 as libc::c_int as libc::c_float;
    *muzzlePoint.offset(1 as libc::c_int as isize) = *muzzlePoint.offset(1 as libc::c_int as isize)
        + *forward_0.offset(1 as libc::c_int as isize) * 14 as libc::c_int as libc::c_float;
    *muzzlePoint.offset(2 as libc::c_int as isize) = *muzzlePoint.offset(2 as libc::c_int as isize)
        + *forward_0.offset(2 as libc::c_int as isize) * 14 as libc::c_int as libc::c_float;
    // snap to integer coordinates for more efficient network bandwidth usage
    *muzzlePoint.offset(0 as libc::c_int as isize) = *muzzlePoint.offset(0 as libc::c_int as isize)
        as libc::c_int
        as crate::src::qcommon::q_shared::vec_t;
    *muzzlePoint.offset(1 as libc::c_int as isize) = *muzzlePoint.offset(1 as libc::c_int as isize)
        as libc::c_int
        as crate::src::qcommon::q_shared::vec_t;
    *muzzlePoint.offset(2 as libc::c_int as isize) = *muzzlePoint.offset(2 as libc::c_int as isize)
        as libc::c_int
        as crate::src::qcommon::q_shared::vec_t;
}
/*
===============
FireWeapon
===============
*/
#[no_mangle]

pub unsafe extern "C" fn FireWeapon(mut ent: *mut crate::g_local_h::gentity_t) {
    if (*(*ent).client).ps.powerups[crate::bg_public_h::PW_QUAD as libc::c_int as usize] != 0 {
        s_quadFactor = crate::src::game::g_main::g_quadfactor.value
    } else {
        s_quadFactor = 1 as libc::c_int as libc::c_float
    }
    // track shots taken for accuracy tracking.  Grapple is not a weapon and gauntet is just not tracked
    if (*ent).s.weapon != crate::bg_public_h::WP_GRAPPLING_HOOK as libc::c_int
        && (*ent).s.weapon != crate::bg_public_h::WP_GAUNTLET as libc::c_int
    {
        (*(*ent).client).accuracy_shots += 1
    }
    // set aiming directions
    crate::src::qcommon::q_math::AngleVectors(
        (*(*ent).client).ps.viewangles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    CalcMuzzlePointOrigin(
        ent,
        (*(*ent).client).oldOrigin.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
        muzzle.as_mut_ptr(),
    );
    // fire the specific weapon
    match (*ent).s.weapon {
        1 => {
            Weapon_Gauntlet(ent);
        }
        6 => {
            Weapon_LightningFire(ent);
        }
        3 => {
            weapon_supershotgun_fire(ent);
        }
        2 => {
            if crate::src::game::g_main::g_gametype.integer
                != crate::bg_public_h::GT_TEAM as libc::c_int
            {
                Bullet_Fire(
                    ent,
                    200 as libc::c_int as libc::c_float,
                    7 as libc::c_int,
                    crate::bg_public_h::MOD_MACHINEGUN as libc::c_int,
                );
            } else {
                Bullet_Fire(
                    ent,
                    200 as libc::c_int as libc::c_float,
                    5 as libc::c_int,
                    crate::bg_public_h::MOD_MACHINEGUN as libc::c_int,
                );
            }
        }
        4 => {
            weapon_grenadelauncher_fire(ent);
        }
        5 => {
            Weapon_RocketLauncher_Fire(ent);
        }
        8 => {
            Weapon_Plasmagun_Fire(ent);
        }
        7 => {
            weapon_railgun_fire(ent);
        }
        9 => {
            BFG_Fire(ent);
        }
        10 => {
            Weapon_GrapplingHook_Fire(ent);
        }
        _ => {}
    };
}

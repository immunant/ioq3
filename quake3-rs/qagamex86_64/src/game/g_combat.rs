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
                + *v.offset(2 as i32 as isize) * *v.offset(2 as i32 as isize))
                as f64,
        ) as crate::src::qcommon::q_shared::vec_t;
    }

    // __Q_SHARED_H
}

pub use crate::stddef_h::size_t;

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::powerup_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::weapon_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
pub use crate::bg_public_h::BOTH_DEAD1;
pub use crate::bg_public_h::BOTH_DEAD2;
pub use crate::bg_public_h::BOTH_DEAD3;
pub use crate::bg_public_h::BOTH_DEATH1;
pub use crate::bg_public_h::BOTH_DEATH2;
pub use crate::bg_public_h::BOTH_DEATH3;
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
pub use crate::bg_public_h::FLAG_RUN;
pub use crate::bg_public_h::FLAG_STAND;
pub use crate::bg_public_h::FLAG_STAND2RUN;
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
pub use crate::bg_public_h::LEGS_BACK;
pub use crate::bg_public_h::LEGS_BACKCR;
pub use crate::bg_public_h::LEGS_BACKWALK;
pub use crate::bg_public_h::LEGS_IDLE;
pub use crate::bg_public_h::LEGS_IDLECR;
pub use crate::bg_public_h::LEGS_JUMP;
pub use crate::bg_public_h::LEGS_JUMPB;
pub use crate::bg_public_h::LEGS_LAND;
pub use crate::bg_public_h::LEGS_LANDB;
pub use crate::bg_public_h::LEGS_RUN;
pub use crate::bg_public_h::LEGS_SWIM;
pub use crate::bg_public_h::LEGS_TURN;
pub use crate::bg_public_h::LEGS_WALK;
pub use crate::bg_public_h::LEGS_WALKCR;
pub use crate::bg_public_h::MAX_ANIMATIONS;
pub use crate::bg_public_h::MAX_TOTALANIMATIONS;
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
pub use crate::bg_public_h::TORSO_AFFIRMATIVE;
pub use crate::bg_public_h::TORSO_ATTACK;
pub use crate::bg_public_h::TORSO_ATTACK2;
pub use crate::bg_public_h::TORSO_DROP;
pub use crate::bg_public_h::TORSO_FOLLOWME;
pub use crate::bg_public_h::TORSO_GESTURE;
pub use crate::bg_public_h::TORSO_GETFLAG;
pub use crate::bg_public_h::TORSO_GUARDBASE;
pub use crate::bg_public_h::TORSO_NEGATIVE;
pub use crate::bg_public_h::TORSO_PATROL;
pub use crate::bg_public_h::TORSO_RAISE;
pub use crate::bg_public_h::TORSO_STAND;
pub use crate::bg_public_h::TORSO_STAND2;
pub use crate::bg_public_h::WEAPON_DROPPING;
pub use crate::bg_public_h::WEAPON_FIRING;
pub use crate::bg_public_h::WEAPON_RAISING;
pub use crate::bg_public_h::WEAPON_READY;
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
pub use crate::src::game::bg_misc::BG_FindItemForPowerup;
pub use crate::src::game::bg_misc::BG_FindItemForWeapon;
pub use crate::src::game::g_combat::q_shared_h::VectorLength;
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
pub use crate::g_local_h::gclient_t;
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
pub use crate::src::game::g_cmds::Cmd_Score_f;
pub use crate::src::game::g_items::Drop_Item;
pub use crate::src::game::g_main::g_blood;
pub use crate::src::game::g_main::g_debugDamage;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::g_friendlyFire;
pub use crate::src::game::g_main::g_gametype;
pub use crate::src::game::g_main::g_knockback;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::CalculateRanks;
pub use crate::src::game::g_main::G_LogPrintf;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_syscalls::trap_EntitiesInBox;
pub use crate::src::game::g_syscalls::trap_LinkEntity;
pub use crate::src::game::g_syscalls::trap_PointContents;
pub use crate::src::game::g_syscalls::trap_Trace;
pub use crate::src::game::g_team::OnSameTeam;

pub use crate::src::game::g_utils::vectoyaw;
pub use crate::src::game::g_utils::G_AddEvent;
pub use crate::src::game::g_utils::G_Find;
pub use crate::src::game::g_utils::G_FreeEntity;
pub use crate::src::game::g_utils::G_TempEntity;
pub use crate::src::game::g_weapon::LogAccuracyHit;
pub use crate::src::game::g_weapon::Weapon_HookFree;

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
// g_combat.c
/*
============
ScorePlum
============
*/
#[no_mangle]

pub unsafe extern "C" fn ScorePlum(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut score: i32,
) {
    let mut plum: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    plum = crate::src::game::g_utils::G_TempEntity(
        origin,
        crate::bg_public_h::EV_SCOREPLUM as i32,
    ) as *mut crate::g_local_h::gentity_s;
    // only send this temp entity to a single client
    (*plum).r.svFlags |= 0x100 as i32;
    (*plum).r.singleClient = (*ent).s.number;
    //
    (*plum).s.otherEntityNum = (*ent).s.number;
    (*plum).s.time = score;
}
/*
============
AddScore

Adds score to both the client and his team
============
*/
#[no_mangle]

pub unsafe extern "C" fn AddScore(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut score: i32,
) {
    if (*ent).client.is_null() {
        return;
    }
    // no scoring during pre-match warmup
    if crate::src::game::g_main::level.warmupTime != 0 {
        return;
    }
    // show score plum
    ScorePlum(ent, origin, score);
    //
    (*(*ent).client).ps.persistant[crate::bg_public_h::PERS_SCORE as i32 as usize] += score;
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_TEAM as i32 {
        crate::src::game::g_main::level.teamScores[(*(*ent).client).ps.persistant
            [crate::bg_public_h::PERS_TEAM as i32 as usize]
            as usize] += score
    }
    crate::src::game::g_main::CalculateRanks();
}
/*
=================
TossClientItems

Toss the weapon and powerups for the killed player
=================
*/
#[no_mangle]

pub unsafe extern "C" fn TossClientItems(mut self_0: *mut crate::g_local_h::gentity_t) {
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    let mut weapon: i32 = 0;
    let mut angle: f32 = 0.;
    let mut i: i32 = 0;
    let mut drop_0: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    // drop the weapon if not a gauntlet or machinegun
    weapon = (*self_0).s.weapon;
    // make a special check to see if they are changing to a new
    // weapon that isn't the mg or gauntlet.  Without this, a client
    // can pick up a weapon, be killed, and not drop the weapon because
    // their weapon change hasn't completed yet and they are still holding the MG.
    if weapon == crate::bg_public_h::WP_MACHINEGUN as i32
        || weapon == crate::bg_public_h::WP_GRAPPLING_HOOK as i32
    {
        if (*(*self_0).client).ps.weaponstate == crate::bg_public_h::WEAPON_DROPPING as i32
        {
            weapon = (*(*self_0).client).pers.cmd.weapon as i32
        }
        if (*(*self_0).client).ps.stats[crate::bg_public_h::STAT_WEAPONS as i32 as usize]
            & (1 as i32) << weapon
            == 0
        {
            weapon = crate::bg_public_h::WP_NONE as i32
        }
    }
    if weapon > crate::bg_public_h::WP_MACHINEGUN as i32
        && weapon != crate::bg_public_h::WP_GRAPPLING_HOOK as i32
        && (*(*self_0).client).ps.ammo[weapon as usize] != 0
    {
        // find the item type for this weapon
        item =
            crate::src::game::bg_misc::BG_FindItemForWeapon(weapon as crate::bg_public_h::weapon_t)
                as *mut crate::bg_public_h::gitem_s;
        // spawn the item

        crate::src::game::g_items::Drop_Item(
            self_0 as *mut crate::g_local_h::gentity_s,
            item as *mut crate::bg_public_h::gitem_s,
            0 as i32 as f32,
        ) as *mut crate::g_local_h::gentity_s;
    }
    // drop all the powerups if not in teamplay
    if crate::src::game::g_main::g_gametype.integer != crate::bg_public_h::GT_TEAM as i32 {
        angle = 45 as i32 as f32;
        i = 1 as i32;
        while i < crate::bg_public_h::PW_NUM_POWERUPS as i32 {
            if (*(*self_0).client).ps.powerups[i as usize] > crate::src::game::g_main::level.time {
                item = crate::src::game::bg_misc::BG_FindItemForPowerup(
                    i as crate::bg_public_h::powerup_t,
                ) as *mut crate::bg_public_h::gitem_s;
                if !item.is_null() {
                    drop_0 = crate::src::game::g_items::Drop_Item(
                        self_0 as *mut crate::g_local_h::gentity_s,
                        item as *mut crate::bg_public_h::gitem_s,
                        angle,
                    ) as *mut crate::g_local_h::gentity_s;
                    // decide how many seconds it has left
                    (*drop_0).count = ((*(*self_0).client).ps.powerups[i as usize]
                        - crate::src::game::g_main::level.time)
                        / 1000 as i32;
                    if (*drop_0).count < 1 as i32 {
                        (*drop_0).count = 1 as i32
                    }
                    angle += 45 as i32 as f32
                }
            }
            i += 1
        }
    };
}
/*
==================
LookAtKiller
==================
*/
#[no_mangle]

pub unsafe extern "C" fn LookAtKiller(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut inflictor: *mut crate::g_local_h::gentity_t,
    mut attacker: *mut crate::g_local_h::gentity_t,
) {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if !attacker.is_null() && attacker != self_0 {
        dir[0 as i32 as usize] = (*attacker).s.pos.trBase[0 as i32 as usize]
            - (*self_0).s.pos.trBase[0 as i32 as usize];
        dir[1 as i32 as usize] = (*attacker).s.pos.trBase[1 as i32 as usize]
            - (*self_0).s.pos.trBase[1 as i32 as usize];
        dir[2 as i32 as usize] = (*attacker).s.pos.trBase[2 as i32 as usize]
            - (*self_0).s.pos.trBase[2 as i32 as usize]
    } else if !inflictor.is_null() && inflictor != self_0 {
        dir[0 as i32 as usize] = (*inflictor).s.pos.trBase[0 as i32 as usize]
            - (*self_0).s.pos.trBase[0 as i32 as usize];
        dir[1 as i32 as usize] = (*inflictor).s.pos.trBase[1 as i32 as usize]
            - (*self_0).s.pos.trBase[1 as i32 as usize];
        dir[2 as i32 as usize] = (*inflictor).s.pos.trBase[2 as i32 as usize]
            - (*self_0).s.pos.trBase[2 as i32 as usize]
    } else {
        (*(*self_0).client).ps.stats[crate::bg_public_h::STAT_DEAD_YAW as i32 as usize] =
            (*self_0).s.angles[1 as i32 as usize] as i32;
        return;
    }
    (*(*self_0).client).ps.stats[crate::bg_public_h::STAT_DEAD_YAW as i32 as usize] =
        crate::src::game::g_utils::vectoyaw(
            dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
        ) as i32;
}
/*
==================
GibEntity
==================
*/
#[no_mangle]

pub unsafe extern "C" fn GibEntity(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut killer: i32,
) {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut i: i32 = 0;
    //if this entity still has kamikaze
    if (*self_0).s.eFlags & 0x200 as i32 != 0 {
        // check if there is a kamikaze timer around for this owner
        i = 0 as i32;
        while i < crate::src::game::g_main::level.num_entities {
            ent = &mut *crate::src::game::g_main::g_entities
                .as_mut_ptr()
                .offset(i as isize) as *mut crate::g_local_h::gentity_t;
            if !((*ent).inuse as u64 == 0) {
                if !((*ent).activator != self_0) {
                    if !(::libc::strcmp(
                        (*ent).classname,
                        b"kamikaze timer\x00" as *const u8 as *const libc::c_char,
                    ) != 0)
                    {
                        crate::src::game::g_utils::G_FreeEntity(
                            ent as *mut crate::g_local_h::gentity_s,
                        );
                        break;
                    }
                }
            }
            i += 1
        }
    }
    crate::src::game::g_utils::G_AddEvent(
        self_0 as *mut crate::g_local_h::gentity_s,
        crate::bg_public_h::EV_GIB_PLAYER as i32,
        killer,
    );
    (*self_0).takedamage = crate::src::qcommon::q_shared::qfalse;
    (*self_0).s.eType = crate::bg_public_h::ET_INVISIBLE as i32;
    (*self_0).r.contents = 0 as i32;
}
/*
==================
body_die
==================
*/
#[no_mangle]

pub unsafe extern "C" fn body_die(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut _inflictor: *mut crate::g_local_h::gentity_t,
    mut _attacker: *mut crate::g_local_h::gentity_t,
    mut _damage: i32,
    mut _meansOfDeath: i32,
) {
    if (*self_0).health > -(40 as i32) {
        return;
    }
    if crate::src::game::g_main::g_blood.integer == 0 {
        (*self_0).health = -(40 as i32) + 1 as i32;
        return;
    }
    GibEntity(self_0, 0 as i32);
}
// these are just for logging, the client prints its own messages
#[no_mangle]

pub static mut modNames: [*mut libc::c_char; 24] = [
    b"MOD_UNKNOWN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_SHOTGUN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_GAUNTLET\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_MACHINEGUN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_GRENADE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_GRENADE_SPLASH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_ROCKET\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_ROCKET_SPLASH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_PLASMA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_PLASMA_SPLASH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_RAILGUN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_LIGHTNING\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_BFG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_BFG_SPLASH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_WATER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_SLIME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_LAVA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_CRUSH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_TELEFRAG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_FALLING\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_SUICIDE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_TARGET_LASER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_TRIGGER_HURT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_GRAPPLE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
/*
==================
CheckAlmostCapture
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CheckAlmostCapture(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut attacker: *mut crate::g_local_h::gentity_t,
) {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut classname: *mut libc::c_char = 0 as *mut libc::c_char;
    // if this player was carrying a flag
    if (*(*self_0).client).ps.powerups[crate::bg_public_h::PW_REDFLAG as i32 as usize] != 0
        || (*(*self_0).client).ps.powerups[crate::bg_public_h::PW_BLUEFLAG as i32 as usize]
            != 0
        || (*(*self_0).client).ps.powerups
            [crate::bg_public_h::PW_NEUTRALFLAG as i32 as usize]
            != 0
    {
        // get the goal flag this player should have been going for
        if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_CTF as i32
        {
            if (*(*self_0).client).sess.sessionTeam as u32
                == crate::bg_public_h::TEAM_BLUE as i32 as u32
            {
                classname = b"team_CTF_blueflag\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
            } else {
                classname =
                    b"team_CTF_redflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
        } else if (*(*self_0).client).sess.sessionTeam as u32
            == crate::bg_public_h::TEAM_BLUE as i32 as u32
        {
            classname =
                b"team_CTF_redflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        } else {
            classname =
                b"team_CTF_blueflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        ent = 0 as *mut crate::g_local_h::gentity_t;
        loop {
            ent = crate::src::game::g_utils::G_Find(
                ent as *mut crate::g_local_h::gentity_s,
                &mut (*(0 as *mut crate::g_local_h::gentity_t)).classname as *mut *mut libc::c_char
                    as crate::stddef_h::size_t as i32,
                classname,
            ) as *mut crate::g_local_h::gentity_s;
            if !(!ent.is_null() && (*ent).flags & 0x1000 as i32 != 0) {
                break;
            }
        }
        // if we found the destination flag and it's not picked up
        if !ent.is_null() && (*ent).r.svFlags & 0x1 as i32 == 0 {
            // if the player was *very* close
            dir[0 as i32 as usize] = (*(*self_0).client).ps.origin
                [0 as i32 as usize]
                - (*ent).s.origin[0 as i32 as usize];
            dir[1 as i32 as usize] = (*(*self_0).client).ps.origin
                [1 as i32 as usize]
                - (*ent).s.origin[1 as i32 as usize];
            dir[2 as i32 as usize] = (*(*self_0).client).ps.origin
                [2 as i32 as usize]
                - (*ent).s.origin[2 as i32 as usize];
            if VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
                < 200 as i32 as f32
            {
                (*(*self_0).client).ps.persistant
                    [crate::bg_public_h::PERS_PLAYEREVENTS as i32 as usize] ^=
                    0x4 as i32;
                if !(*attacker).client.is_null() {
                    (*(*attacker).client).ps.persistant
                        [crate::bg_public_h::PERS_PLAYEREVENTS as i32 as usize] ^=
                        0x4 as i32
                }
            }
        }
    };
}
/*
==================
CheckAlmostScored
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CheckAlmostScored(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut attacker: *mut crate::g_local_h::gentity_t,
) {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut classname: *mut libc::c_char = 0 as *mut libc::c_char;
    // if the player was carrying cubes
    if (*(*self_0).client).ps.generic1 != 0 {
        if (*(*self_0).client).sess.sessionTeam as u32
            == crate::bg_public_h::TEAM_BLUE as i32 as u32
        {
            classname =
                b"team_redobelisk\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        } else {
            classname =
                b"team_blueobelisk\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        ent = crate::src::game::g_utils::G_Find(
            0 as *mut crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_s,
            &mut (*(0 as *mut crate::g_local_h::gentity_t)).classname as *mut *mut libc::c_char
                as crate::stddef_h::size_t as i32,
            classname,
        ) as *mut crate::g_local_h::gentity_s;
        // if we found the destination obelisk
        if !ent.is_null() {
            // if the player was *very* close
            dir[0 as i32 as usize] = (*(*self_0).client).ps.origin
                [0 as i32 as usize]
                - (*ent).s.origin[0 as i32 as usize];
            dir[1 as i32 as usize] = (*(*self_0).client).ps.origin
                [1 as i32 as usize]
                - (*ent).s.origin[1 as i32 as usize];
            dir[2 as i32 as usize] = (*(*self_0).client).ps.origin
                [2 as i32 as usize]
                - (*ent).s.origin[2 as i32 as usize];
            if VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
                < 200 as i32 as f32
            {
                (*(*self_0).client).ps.persistant
                    [crate::bg_public_h::PERS_PLAYEREVENTS as i32 as usize] ^=
                    0x4 as i32;
                if !(*attacker).client.is_null() {
                    (*(*attacker).client).ps.persistant
                        [crate::bg_public_h::PERS_PLAYEREVENTS as i32 as usize] ^=
                        0x4 as i32
                }
            }
        }
    };
}
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
//
// g_weapon.c
//
//
// g_client.c
//
/*
==================
player_die
==================
*/
#[no_mangle]

pub unsafe extern "C" fn player_die(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut inflictor: *mut crate::g_local_h::gentity_t,
    mut attacker: *mut crate::g_local_h::gentity_t,
    mut _damage: i32,
    mut meansOfDeath: i32,
) {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut anim: i32 = 0;
    let mut contents: i32 = 0;
    let mut killer: i32 = 0;
    let mut i: i32 = 0;
    let mut killerName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obit: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(*self_0).client).ps.pm_type == crate::bg_public_h::PM_DEAD as i32 {
        return;
    }
    if crate::src::game::g_main::level.intermissiontime != 0 {
        return;
    }
    // check for an almost capture
    CheckAlmostCapture(self_0, attacker);
    // check for a player that almost brought in cubes
    CheckAlmostScored(self_0, attacker);
    if !(*self_0).client.is_null() && !(*(*self_0).client).hook.is_null() {
        crate::src::game::g_weapon::Weapon_HookFree(
            (*(*self_0).client).hook as *mut crate::g_local_h::gentity_s,
        );
    }
    (*(*self_0).client).ps.pm_type = crate::bg_public_h::PM_DEAD as i32;
    if !attacker.is_null() {
        killer = (*attacker).s.number;
        if !(*attacker).client.is_null() {
            killerName = (*(*attacker).client).pers.netname.as_mut_ptr()
        } else {
            killerName =
                b"<non-client>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
    } else {
        killer = ((1 as i32) << 10 as i32) - 2 as i32;
        killerName = b"<world>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if killer < 0 as i32 || killer >= 64 as i32 {
        killer = ((1 as i32) << 10 as i32) - 2 as i32;
        killerName = b"<world>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if meansOfDeath < 0 as i32
        || meansOfDeath as libc::c_ulong
            >= (::std::mem::size_of::<[*mut libc::c_char; 24]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        obit = b"<bad obituary>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        obit = modNames[meansOfDeath as usize]
    }
    crate::src::game::g_main::G_LogPrintf(
        b"Kill: %i %i %i: %s killed %s by %s\n\x00" as *const u8 as *const libc::c_char,
        killer,
        (*self_0).s.number,
        meansOfDeath,
        killerName,
        (*(*self_0).client).pers.netname.as_mut_ptr(),
        obit,
    );
    // broadcast the death event to everyone
    ent = crate::src::game::g_utils::G_TempEntity(
        (*self_0).r.currentOrigin.as_mut_ptr(),
        crate::bg_public_h::EV_OBITUARY as i32,
    ) as *mut crate::g_local_h::gentity_s; // send to everyone
    (*ent).s.eventParm = meansOfDeath;
    (*ent).s.otherEntityNum = (*self_0).s.number;
    (*ent).s.otherEntityNum2 = killer;
    (*ent).r.svFlags = 0x20 as i32;
    (*self_0).enemy = attacker;
    (*(*self_0).client).ps.persistant[crate::bg_public_h::PERS_KILLED as i32 as usize] += 1;
    if !attacker.is_null() && !(*attacker).client.is_null() {
        (*(*attacker).client).lastkilled_client = (*self_0).s.number;
        if attacker == self_0
            || crate::src::game::g_team::OnSameTeam(
                self_0 as *mut crate::g_local_h::gentity_s,
                attacker as *mut crate::g_local_h::gentity_s,
            ) as u32
                != 0
        {
            AddScore(
                attacker,
                (*self_0).r.currentOrigin.as_mut_ptr(),
                -(1 as i32),
            );
        } else {
            AddScore(
                attacker,
                (*self_0).r.currentOrigin.as_mut_ptr(),
                1 as i32,
            );
            if meansOfDeath == crate::bg_public_h::MOD_GAUNTLET as i32 {
                // play humiliation on player
                (*(*attacker).client).ps.persistant
                    [crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT as i32 as usize] += 1;
                // add the sprite over the player's head
                (*(*attacker).client).ps.eFlags &= !(0x8000 as i32
                    | 0x8 as i32
                    | 0x40 as i32
                    | 0x20000 as i32
                    | 0x10000 as i32
                    | 0x800 as i32);
                (*(*attacker).client).ps.eFlags |= 0x40 as i32;
                (*(*attacker).client).rewardTime =
                    crate::src::game::g_main::level.time + 2000 as i32;
                // also play humiliation on target
                (*(*self_0).client).ps.persistant
                    [crate::bg_public_h::PERS_PLAYEREVENTS as i32 as usize] ^=
                    0x2 as i32
            }
            // check for two kills in a short amount of time
            // if this is close enough to the last kill, give a reward sound
            if crate::src::game::g_main::level.time - (*(*attacker).client).lastKillTime
                < 3000 as i32
            {
                // play excellent on player
                (*(*attacker).client).ps.persistant
                    [crate::bg_public_h::PERS_EXCELLENT_COUNT as i32 as usize] += 1;
                // add the sprite over the player's head
                (*(*attacker).client).ps.eFlags &= !(0x8000 as i32
                    | 0x8 as i32
                    | 0x40 as i32
                    | 0x20000 as i32
                    | 0x10000 as i32
                    | 0x800 as i32);
                (*(*attacker).client).ps.eFlags |= 0x8 as i32;
                (*(*attacker).client).rewardTime =
                    crate::src::game::g_main::level.time + 2000 as i32
            }
            (*(*attacker).client).lastKillTime = crate::src::game::g_main::level.time
        }
    } else {
        AddScore(
            self_0,
            (*self_0).r.currentOrigin.as_mut_ptr(),
            -(1 as i32),
        );
    }
    // Add team bonuses
    crate::src::game::g_team::Team_FragBonuses(
        self_0 as *mut crate::g_local_h::gentity_s,
        inflictor as *mut crate::g_local_h::gentity_s,
        attacker as *mut crate::g_local_h::gentity_s,
    );
    // if I committed suicide, the flag does not fall, it returns.
    if meansOfDeath == crate::bg_public_h::MOD_SUICIDE as i32 {
        if (*(*self_0).client).ps.powerups
            [crate::bg_public_h::PW_NEUTRALFLAG as i32 as usize]
            != 0
        {
            // only happens in One Flag CTF
            crate::src::game::g_team::Team_ReturnFlag(crate::bg_public_h::TEAM_FREE as i32);
            (*(*self_0).client).ps.powerups
                [crate::bg_public_h::PW_NEUTRALFLAG as i32 as usize] = 0 as i32
        } else if (*(*self_0).client).ps.powerups
            [crate::bg_public_h::PW_REDFLAG as i32 as usize]
            != 0
        {
            // only happens in standard CTF
            crate::src::game::g_team::Team_ReturnFlag(crate::bg_public_h::TEAM_RED as i32);
            (*(*self_0).client).ps.powerups
                [crate::bg_public_h::PW_REDFLAG as i32 as usize] = 0 as i32
        } else if (*(*self_0).client).ps.powerups
            [crate::bg_public_h::PW_BLUEFLAG as i32 as usize]
            != 0
        {
            // only happens in standard CTF
            crate::src::game::g_team::Team_ReturnFlag(crate::bg_public_h::TEAM_BLUE as i32); // show scores
            (*(*self_0).client).ps.powerups
                [crate::bg_public_h::PW_BLUEFLAG as i32 as usize] = 0 as i32
        }
    }
    TossClientItems(self_0);
    crate::src::game::g_cmds::Cmd_Score_f(self_0 as *mut crate::g_local_h::gentity_s);
    // send updated scores to any clients that are following this one,
    // or they would get stale scoreboards
    i = 0 as i32; // can still be gibbed
    while i < crate::src::game::g_main::level.maxclients {
        let mut client: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
        client = &mut *crate::src::game::g_main::level.clients.offset(i as isize)
            as *mut crate::g_local_h::gclient_s;
        if !((*client).pers.connected as u32
            != crate::g_local_h::CON_CONNECTED as i32 as u32)
        {
            if !((*client).sess.sessionTeam as u32
                != crate::bg_public_h::TEAM_SPECTATOR as i32 as u32)
            {
                if (*client).sess.spectatorClient == (*self_0).s.number {
                    crate::src::game::g_cmds::Cmd_Score_f(
                        crate::src::game::g_main::g_entities
                            .as_mut_ptr()
                            .offset(i as isize)
                            as *mut crate::g_local_h::gentity_s,
                    );
                }
            }
        }
        i += 1
    }
    (*self_0).takedamage = crate::src::qcommon::q_shared::qtrue;
    (*self_0).s.weapon = crate::bg_public_h::WP_NONE as i32;
    (*self_0).s.powerups = 0 as i32;
    (*self_0).r.contents = 0x4000000 as i32;
    (*self_0).s.angles[0 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    (*self_0).s.angles[2 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    LookAtKiller(self_0, inflictor, attacker);
    (*(*self_0).client).ps.viewangles[0 as i32 as usize] =
        (*self_0).s.angles[0 as i32 as usize];
    (*(*self_0).client).ps.viewangles[1 as i32 as usize] =
        (*self_0).s.angles[1 as i32 as usize];
    (*(*self_0).client).ps.viewangles[2 as i32 as usize] =
        (*self_0).s.angles[2 as i32 as usize];
    (*self_0).s.loopSound = 0 as i32;
    (*self_0).r.maxs[2 as i32 as usize] =
        -(8 as i32) as crate::src::qcommon::q_shared::vec_t;
    // don't allow respawn until the death anim is done
    // g_forcerespawn may force spawning at some later time
    (*(*self_0).client).respawnTime = crate::src::game::g_main::level.time + 1700 as i32;
    // remove powerups
    crate::stdlib::memset(
        (*(*self_0).client).ps.powerups.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[i32; 16]>() as libc::c_ulong,
    );
    // never gib in a nodrop
    contents = crate::src::game::g_syscalls::trap_PointContents(
        (*self_0).r.currentOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        -(1 as i32),
    );
    if (*self_0).health <= -(40 as i32)
        && contents as u32 & 0x80000000 as u32 == 0
        && crate::src::game::g_main::g_blood.integer != 0
        || meansOfDeath == crate::bg_public_h::MOD_SUICIDE as i32
    {
        // gib death
        GibEntity(self_0, killer);
    } else {
        // normal death
        static mut i_0: i32 = 0;
        match i_0 {
            0 => anim = crate::bg_public_h::BOTH_DEATH1 as i32,
            1 => anim = crate::bg_public_h::BOTH_DEATH2 as i32,
            2 | _ => anim = crate::bg_public_h::BOTH_DEATH3 as i32,
        }
        // for the no-blood option, we need to prevent the health
        // from going to gib level
        if (*self_0).health <= -(40 as i32) {
            (*self_0).health = -(40 as i32) + 1 as i32
        }
        (*(*self_0).client).ps.legsAnim =
            (*(*self_0).client).ps.legsAnim & 128 as i32 ^ 128 as i32 | anim;
        (*(*self_0).client).ps.torsoAnim =
            (*(*self_0).client).ps.torsoAnim & 128 as i32 ^ 128 as i32 | anim;
        crate::src::game::g_utils::G_AddEvent(
            self_0 as *mut crate::g_local_h::gentity_s,
            crate::bg_public_h::EV_DEATH1 as i32 + i_0,
            killer,
        );
        // the body can still be gibbed
        (*self_0).die = Some(
            body_die
                as unsafe extern "C" fn(
                    _: *mut crate::g_local_h::gentity_t,
                    _: *mut crate::g_local_h::gentity_t,
                    _: *mut crate::g_local_h::gentity_t,
                    _: i32,
                    _: i32,
                ) -> (),
        );
        // globally cycle through the different death animations
        i_0 = (i_0 + 1 as i32) % 3 as i32
    }
    crate::src::game::g_syscalls::trap_LinkEntity(self_0 as *mut crate::g_local_h::gentity_s);
}
/*
================
CheckArmor
================
*/
#[no_mangle]

pub unsafe extern "C" fn CheckArmor(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut damage: i32,
    mut dflags: i32,
) -> i32 {
    let mut client: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    let mut save: i32 = 0;
    let mut count: i32 = 0;
    if damage == 0 {
        return 0 as i32;
    }
    client = (*ent).client;
    if client.is_null() {
        return 0 as i32;
    }
    if dflags & 0x2 as i32 != 0 {
        return 0 as i32;
    }
    // armor
    count = (*client).ps.stats[crate::bg_public_h::STAT_ARMOR as i32 as usize];
    save = crate::stdlib::ceil(damage as f64 * 0.66f64) as i32;
    if save >= count {
        save = count
    }
    if save == 0 {
        return 0 as i32;
    }
    (*client).ps.stats[crate::bg_public_h::STAT_ARMOR as i32 as usize] -= save;
    return save;
}
/*
================
RaySphereIntersections
================
*/
#[no_mangle]

pub unsafe extern "C" fn RaySphereIntersections(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut radius: f32,
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut intersections: *mut crate::src::qcommon::q_shared::vec3_t,
) -> i32 {
    let mut b: f32 = 0.;
    let mut c: f32 = 0.;
    let mut d: f32 = 0.;
    let mut t: f32 = 0.;
    //	| origin - (point + t * dir) | = radius
    //	a = dir[0]^2 + dir[1]^2 + dir[2]^2;
    //	b = 2 * (dir[0] * (point[0] - origin[0]) + dir[1] * (point[1] - origin[1]) + dir[2] * (point[2] - origin[2]));
    //	c = (point[0] - origin[0])^2 + (point[1] - origin[1])^2 + (point[2] - origin[2])^2 - radius^2;
    // normalize dir so a = 1
    crate::src::qcommon::q_math::VectorNormalize(dir);
    b = 2 as i32 as f32
        * (*dir.offset(0 as i32 as isize)
            * (*point.offset(0 as i32 as isize)
                - *origin.offset(0 as i32 as isize))
            + *dir.offset(1 as i32 as isize)
                * (*point.offset(1 as i32 as isize)
                    - *origin.offset(1 as i32 as isize))
            + *dir.offset(2 as i32 as isize)
                * (*point.offset(2 as i32 as isize)
                    - *origin.offset(2 as i32 as isize)));
    c = (*point.offset(0 as i32 as isize) - *origin.offset(0 as i32 as isize))
        * (*point.offset(0 as i32 as isize) - *origin.offset(0 as i32 as isize))
        + (*point.offset(1 as i32 as isize) - *origin.offset(1 as i32 as isize))
            * (*point.offset(1 as i32 as isize)
                - *origin.offset(1 as i32 as isize))
        + (*point.offset(2 as i32 as isize) - *origin.offset(2 as i32 as isize))
            * (*point.offset(2 as i32 as isize)
                - *origin.offset(2 as i32 as isize))
        - radius * radius;
    d = b * b - 4 as i32 as f32 * c;
    if d > 0 as i32 as f32 {
        t = ((-b as f64 + crate::stdlib::sqrt(d as f64))
            / 2 as i32 as f64) as f32;
        (*intersections.offset(0 as i32 as isize))[0 as i32 as usize] =
            *point.offset(0 as i32 as isize) + *dir.offset(0 as i32 as isize) * t;
        (*intersections.offset(0 as i32 as isize))[1 as i32 as usize] =
            *point.offset(1 as i32 as isize) + *dir.offset(1 as i32 as isize) * t;
        (*intersections.offset(0 as i32 as isize))[2 as i32 as usize] =
            *point.offset(2 as i32 as isize) + *dir.offset(2 as i32 as isize) * t;
        t = ((-b as f64 - crate::stdlib::sqrt(d as f64))
            / 2 as i32 as f64) as f32;
        (*intersections.offset(1 as i32 as isize))[0 as i32 as usize] =
            *point.offset(0 as i32 as isize) + *dir.offset(0 as i32 as isize) * t;
        (*intersections.offset(1 as i32 as isize))[1 as i32 as usize] =
            *point.offset(1 as i32 as isize) + *dir.offset(1 as i32 as isize) * t;
        (*intersections.offset(1 as i32 as isize))[2 as i32 as usize] =
            *point.offset(2 as i32 as isize) + *dir.offset(2 as i32 as isize) * t;
        return 2 as i32;
    } else {
        if d == 0 as i32 as f32 {
            t = -b / 2 as i32 as f32;
            (*intersections.offset(0 as i32 as isize))[0 as i32 as usize] = *point
                .offset(0 as i32 as isize)
                + *dir.offset(0 as i32 as isize) * t;
            (*intersections.offset(0 as i32 as isize))[1 as i32 as usize] = *point
                .offset(1 as i32 as isize)
                + *dir.offset(1 as i32 as isize) * t;
            (*intersections.offset(0 as i32 as isize))[2 as i32 as usize] = *point
                .offset(2 as i32 as isize)
                + *dir.offset(2 as i32 as isize) * t;
            return 1 as i32;
        }
    }
    return 0 as i32;
}
/*
============
G_Damage

targ		entity that is being damaged
inflictor	entity that is causing the damage
attacker	entity that caused the inflictor to damage targ
    example: targ=monster, inflictor=rocket, attacker=player

dir			direction of the attack for knockback
point		point at which the damage is being inflicted, used for headshots
damage		amount of damage being inflicted
knockback	force to be applied against targ as a result of the damage

inflictor, attacker, dir, and point can be NULL for environmental effects

dflags		these flags are used to control how T_Damage works
    DAMAGE_RADIUS			damage was indirect (from a nearby explosion)
    DAMAGE_NO_ARMOR			armor does not protect from this damage
    DAMAGE_NO_KNOCKBACK		do not affect velocity, just view angles
    DAMAGE_NO_PROTECTION	kills godmode, armor, everything
============
*/
#[no_mangle]

pub unsafe extern "C" fn G_Damage(
    mut targ: *mut crate::g_local_h::gentity_t,
    mut inflictor: *mut crate::g_local_h::gentity_t,
    mut attacker: *mut crate::g_local_h::gentity_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut _point: *mut crate::src::qcommon::q_shared::vec_t,
    mut damage: i32,
    mut dflags: i32,
    mut mod_0: i32,
) {
    let mut client: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    let mut take: i32 = 0;
    let mut asave: i32 = 0;
    let mut knockback: i32 = 0;
    let mut max: i32 = 0;
    if (*targ).takedamage as u64 == 0 {
        return;
    }
    // the intermission has already been qualified for, so don't
    // allow any extra scoring
    if crate::src::game::g_main::level.intermissionQueued != 0 {
        return;
    }
    if inflictor.is_null() {
        inflictor = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset((((1 as i32) << 10 as i32) - 2 as i32) as isize)
            as *mut crate::g_local_h::gentity_t
    }
    if attacker.is_null() {
        attacker = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset((((1 as i32) << 10 as i32) - 2 as i32) as isize)
            as *mut crate::g_local_h::gentity_t
    }
    // shootable doors / buttons don't actually have any health
    if (*targ).s.eType == crate::bg_public_h::ET_MOVER as i32 {
        if (*targ).use_0.is_some()
            && (*targ).moverState as u32
                == crate::g_local_h::MOVER_POS1 as i32 as u32
        {
            (*targ).use_0.expect("non-null function pointer")(targ, inflictor, attacker);
        }
        return;
    }
    // reduce damage by the attacker's handicap value
    // unless they are rocket jumping
    if !(*attacker).client.is_null() && attacker != targ {
        max = (*(*attacker).client).ps.stats
            [crate::bg_public_h::STAT_MAX_HEALTH as i32 as usize];
        damage = damage * max / 100 as i32
    }
    client = (*targ).client;
    if !client.is_null() {
        if (*client).noclip as u64 != 0 {
            return;
        }
    }
    if dir.is_null() {
        dflags |= 0x4 as i32
    } else {
        crate::src::qcommon::q_math::VectorNormalize(dir);
    }
    knockback = damage;
    if knockback > 200 as i32 {
        knockback = 200 as i32
    }
    if (*targ).flags & 0x800 as i32 != 0 {
        knockback = 0 as i32
    }
    if dflags & 0x4 as i32 != 0 {
        knockback = 0 as i32
    }
    // figure momentum add, even if the damage won't be taken
    if knockback != 0 && !(*targ).client.is_null() {
        let mut kvel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut mass: f32 = 0.;
        mass = 200 as i32 as f32;
        kvel[0 as i32 as usize] = *dir.offset(0 as i32 as isize)
            * (crate::src::game::g_main::g_knockback.value * knockback as f32 / mass);
        kvel[1 as i32 as usize] = *dir.offset(1 as i32 as isize)
            * (crate::src::game::g_main::g_knockback.value * knockback as f32 / mass);
        kvel[2 as i32 as usize] = *dir.offset(2 as i32 as isize)
            * (crate::src::game::g_main::g_knockback.value * knockback as f32 / mass);
        (*(*targ).client).ps.velocity[0 as i32 as usize] = (*(*targ).client).ps.velocity
            [0 as i32 as usize]
            + kvel[0 as i32 as usize];
        (*(*targ).client).ps.velocity[1 as i32 as usize] = (*(*targ).client).ps.velocity
            [1 as i32 as usize]
            + kvel[1 as i32 as usize];
        (*(*targ).client).ps.velocity[2 as i32 as usize] = (*(*targ).client).ps.velocity
            [2 as i32 as usize]
            + kvel[2 as i32 as usize];
        // set the timer so that the other client can't cancel
        // out the movement immediately
        if (*(*targ).client).ps.pm_time == 0 {
            let mut t: i32 = 0;
            t = knockback * 2 as i32;
            if t < 50 as i32 {
                t = 50 as i32
            }
            if t > 200 as i32 {
                t = 200 as i32
            }
            (*(*targ).client).ps.pm_time = t;
            (*(*targ).client).ps.pm_flags |= 64 as i32
        }
    }
    // check for completely getting out of the damage
    if dflags & 0x8 as i32 == 0 {
        // if TF_NO_FRIENDLY_FIRE is set, don't do damage to the target
        // if the attacker was on the same team
        if targ != attacker
            && crate::src::game::g_team::OnSameTeam(
                targ as *mut crate::g_local_h::gentity_s,
                attacker as *mut crate::g_local_h::gentity_s,
            ) as u32
                != 0
        {
            if crate::src::game::g_main::g_friendlyFire.integer == 0 {
                return;
            }
        }
        // check for godmode
        if (*targ).flags & 0x10 as i32 != 0 {
            return;
        }
    }
    // battlesuit protects from all radius damage (but takes knockback)
    // and protects 50% against all damage
    if !client.is_null()
        && (*client).ps.powerups[crate::bg_public_h::PW_BATTLESUIT as i32 as usize] != 0
    {
        crate::src::game::g_utils::G_AddEvent(
            targ as *mut crate::g_local_h::gentity_s,
            crate::bg_public_h::EV_POWERUP_BATTLESUIT as i32,
            0 as i32,
        );
        if dflags & 0x1 as i32 != 0
            || mod_0 == crate::bg_public_h::MOD_FALLING as i32
        {
            return;
        }
        damage = (damage as f64 * 0.5f64) as i32
    }
    // add to the attacker's hit counter (if the target isn't a general entity like a prox mine)
    if !(*attacker).client.is_null()
        && !client.is_null()
        && targ != attacker
        && (*targ).health > 0 as i32
        && (*targ).s.eType != crate::bg_public_h::ET_MISSILE as i32
        && (*targ).s.eType != crate::bg_public_h::ET_GENERAL as i32
    {
        if crate::src::game::g_team::OnSameTeam(
            targ as *mut crate::g_local_h::gentity_s,
            attacker as *mut crate::g_local_h::gentity_s,
        ) as u64
            != 0
        {
            (*(*attacker).client).ps.persistant
                [crate::bg_public_h::PERS_HITS as i32 as usize] -= 1
        } else {
            (*(*attacker).client).ps.persistant
                [crate::bg_public_h::PERS_HITS as i32 as usize] += 1
        }
        (*(*attacker).client).ps.persistant
            [crate::bg_public_h::PERS_ATTACKEE_ARMOR as i32 as usize] = (*targ).health
            << 8 as i32
            | (*client).ps.stats[crate::bg_public_h::STAT_ARMOR as i32 as usize]
    }
    // always give half damage if hurting self
    // calculated after knockback, so rocket jumping works
    if targ == attacker {
        damage = (damage as f64 * 0.5f64) as i32
    }
    if damage < 1 as i32 {
        damage = 1 as i32
    }
    take = damage;
    // save some from armor
    asave = CheckArmor(targ, take, dflags);
    take -= asave;
    if crate::src::game::g_main::g_debugDamage.integer != 0 {
        crate::src::game::g_main::G_Printf(
            b"%i: client:%i health:%i damage:%i armor:%i\n\x00" as *const u8 as *const libc::c_char,
            crate::src::game::g_main::level.time,
            (*targ).s.number,
            (*targ).health,
            take,
            asave,
        );
    }
    // add to the damage inflicted on a player this frame
    // the total will be turned into screen blends and view angle kicks
    // at the end of the frame
    if !client.is_null() {
        if !attacker.is_null() {
            (*client).ps.persistant[crate::bg_public_h::PERS_ATTACKER as i32 as usize] =
                (*attacker).s.number
        } else {
            (*client).ps.persistant[crate::bg_public_h::PERS_ATTACKER as i32 as usize] =
                ((1 as i32) << 10 as i32) - 2 as i32
        }
        (*client).damage_armor += asave;
        (*client).damage_blood += take;
        (*client).damage_knockback += knockback;
        if !dir.is_null() {
            (*client).damage_from[0 as i32 as usize] =
                *dir.offset(0 as i32 as isize);
            (*client).damage_from[1 as i32 as usize] =
                *dir.offset(1 as i32 as isize);
            (*client).damage_from[2 as i32 as usize] =
                *dir.offset(2 as i32 as isize);
            (*client).damage_fromWorld = crate::src::qcommon::q_shared::qfalse
        } else {
            (*client).damage_from[0 as i32 as usize] =
                (*targ).r.currentOrigin[0 as i32 as usize];
            (*client).damage_from[1 as i32 as usize] =
                (*targ).r.currentOrigin[1 as i32 as usize];
            (*client).damage_from[2 as i32 as usize] =
                (*targ).r.currentOrigin[2 as i32 as usize];
            (*client).damage_fromWorld = crate::src::qcommon::q_shared::qtrue
        }
    }
    // See if it's the player hurting the emeny flag carrier
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_CTF as i32 {
        crate::src::game::g_team::Team_CheckHurtCarrier(
            targ as *mut crate::g_local_h::gentity_s,
            attacker as *mut crate::g_local_h::gentity_s,
        );
    }
    if !(*targ).client.is_null() {
        // set the last client who damaged the target
        (*(*targ).client).lasthurt_client = (*attacker).s.number;
        (*(*targ).client).lasthurt_mod = mod_0
    }
    // do the damage
    if take != 0 {
        (*targ).health = (*targ).health - take;
        if !(*targ).client.is_null() {
            (*(*targ).client).ps.stats[crate::bg_public_h::STAT_HEALTH as i32 as usize] =
                (*targ).health
        }
        if (*targ).health <= 0 as i32 {
            if !client.is_null() {
                (*targ).flags |= 0x800 as i32
            }
            if (*targ).health < -(999 as i32) {
                (*targ).health = -(999 as i32)
            }
            (*targ).enemy = attacker;
            (*targ).die.expect("non-null function pointer")(targ, inflictor, attacker, take, mod_0);
            return;
        } else {
            if (*targ).pain.is_some() {
                (*targ).pain.expect("non-null function pointer")(targ, attacker, take);
            }
        }
    };
}
/*
============
CanDamage

Returns qtrue if the inflictor can directly damage the target.  Used for
explosions and melee attacks.
============
*/
#[no_mangle]

pub unsafe extern "C" fn CanDamage(
    mut targ: *mut crate::g_local_h::gentity_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut dest: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
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
    let mut midpoint: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut offsetmins: crate::src::qcommon::q_shared::vec3_t = [
        -(15 as i32) as crate::src::qcommon::q_shared::vec_t,
        -(15 as i32) as crate::src::qcommon::q_shared::vec_t,
        -(15 as i32) as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut offsetmaxs: crate::src::qcommon::q_shared::vec3_t = [
        15 as i32 as crate::src::qcommon::q_shared::vec_t,
        15 as i32 as crate::src::qcommon::q_shared::vec_t,
        15 as i32 as crate::src::qcommon::q_shared::vec_t,
    ];
    // use the midpoint of the bounds instead of the origin, because
    // bmodels may have their origin is 0,0,0
    midpoint[0 as i32 as usize] =
        (*targ).r.absmin[0 as i32 as usize] + (*targ).r.absmax[0 as i32 as usize];
    midpoint[1 as i32 as usize] =
        (*targ).r.absmin[1 as i32 as usize] + (*targ).r.absmax[1 as i32 as usize];
    midpoint[2 as i32 as usize] =
        (*targ).r.absmin[2 as i32 as usize] + (*targ).r.absmax[2 as i32 as usize];
    midpoint[0 as i32 as usize] = (midpoint[0 as i32 as usize] as f64
        * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
    midpoint[1 as i32 as usize] = (midpoint[1 as i32 as usize] as f64
        * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
    midpoint[2 as i32 as usize] = (midpoint[2 as i32 as usize] as f64
        * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
    dest[0 as i32 as usize] = midpoint[0 as i32 as usize];
    dest[1 as i32 as usize] = midpoint[1 as i32 as usize];
    dest[2 as i32 as usize] = midpoint[2 as i32 as usize];
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        origin as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        dest.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ((1 as i32) << 10 as i32) - 1 as i32,
        1 as i32,
    );
    if tr.fraction as f64 == 1.0f64 || tr.entityNum == (*targ).s.number {
        return crate::src::qcommon::q_shared::qtrue;
    }
    // this should probably check in the plane of projection,
    // rather than in world coordinate
    dest[0 as i32 as usize] = midpoint[0 as i32 as usize];
    dest[1 as i32 as usize] = midpoint[1 as i32 as usize];
    dest[2 as i32 as usize] = midpoint[2 as i32 as usize];
    dest[0 as i32 as usize] += offsetmaxs[0 as i32 as usize];
    dest[1 as i32 as usize] += offsetmaxs[1 as i32 as usize];
    dest[2 as i32 as usize] += offsetmaxs[2 as i32 as usize];
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        origin as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        dest.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ((1 as i32) << 10 as i32) - 1 as i32,
        1 as i32,
    );
    if tr.fraction as f64 == 1.0f64 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    dest[0 as i32 as usize] = midpoint[0 as i32 as usize];
    dest[1 as i32 as usize] = midpoint[1 as i32 as usize];
    dest[2 as i32 as usize] = midpoint[2 as i32 as usize];
    dest[0 as i32 as usize] += offsetmaxs[0 as i32 as usize];
    dest[1 as i32 as usize] += offsetmins[1 as i32 as usize];
    dest[2 as i32 as usize] += offsetmaxs[2 as i32 as usize];
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        origin as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        dest.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ((1 as i32) << 10 as i32) - 1 as i32,
        1 as i32,
    );
    if tr.fraction as f64 == 1.0f64 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    dest[0 as i32 as usize] = midpoint[0 as i32 as usize];
    dest[1 as i32 as usize] = midpoint[1 as i32 as usize];
    dest[2 as i32 as usize] = midpoint[2 as i32 as usize];
    dest[0 as i32 as usize] += offsetmins[0 as i32 as usize];
    dest[1 as i32 as usize] += offsetmaxs[1 as i32 as usize];
    dest[2 as i32 as usize] += offsetmaxs[2 as i32 as usize];
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        origin as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        dest.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ((1 as i32) << 10 as i32) - 1 as i32,
        1 as i32,
    );
    if tr.fraction as f64 == 1.0f64 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    dest[0 as i32 as usize] = midpoint[0 as i32 as usize];
    dest[1 as i32 as usize] = midpoint[1 as i32 as usize];
    dest[2 as i32 as usize] = midpoint[2 as i32 as usize];
    dest[0 as i32 as usize] += offsetmins[0 as i32 as usize];
    dest[1 as i32 as usize] += offsetmins[1 as i32 as usize];
    dest[2 as i32 as usize] += offsetmaxs[2 as i32 as usize];
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        origin as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        dest.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ((1 as i32) << 10 as i32) - 1 as i32,
        1 as i32,
    );
    if tr.fraction as f64 == 1.0f64 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    dest[0 as i32 as usize] = midpoint[0 as i32 as usize];
    dest[1 as i32 as usize] = midpoint[1 as i32 as usize];
    dest[2 as i32 as usize] = midpoint[2 as i32 as usize];
    dest[0 as i32 as usize] += offsetmaxs[0 as i32 as usize];
    dest[1 as i32 as usize] += offsetmaxs[1 as i32 as usize];
    dest[2 as i32 as usize] += offsetmins[2 as i32 as usize];
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        origin as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        dest.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ((1 as i32) << 10 as i32) - 1 as i32,
        1 as i32,
    );
    if tr.fraction as f64 == 1.0f64 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    dest[0 as i32 as usize] = midpoint[0 as i32 as usize];
    dest[1 as i32 as usize] = midpoint[1 as i32 as usize];
    dest[2 as i32 as usize] = midpoint[2 as i32 as usize];
    dest[0 as i32 as usize] += offsetmaxs[0 as i32 as usize];
    dest[1 as i32 as usize] += offsetmins[1 as i32 as usize];
    dest[2 as i32 as usize] += offsetmins[2 as i32 as usize];
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        origin as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        dest.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ((1 as i32) << 10 as i32) - 1 as i32,
        1 as i32,
    );
    if tr.fraction as f64 == 1.0f64 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    dest[0 as i32 as usize] = midpoint[0 as i32 as usize];
    dest[1 as i32 as usize] = midpoint[1 as i32 as usize];
    dest[2 as i32 as usize] = midpoint[2 as i32 as usize];
    dest[0 as i32 as usize] += offsetmins[0 as i32 as usize];
    dest[1 as i32 as usize] += offsetmaxs[1 as i32 as usize];
    dest[2 as i32 as usize] += offsetmins[2 as i32 as usize];
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        origin as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        dest.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ((1 as i32) << 10 as i32) - 1 as i32,
        1 as i32,
    );
    if tr.fraction as f64 == 1.0f64 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    dest[0 as i32 as usize] = midpoint[0 as i32 as usize];
    dest[1 as i32 as usize] = midpoint[1 as i32 as usize];
    dest[2 as i32 as usize] = midpoint[2 as i32 as usize];
    dest[0 as i32 as usize] += offsetmins[0 as i32 as usize];
    dest[1 as i32 as usize] += offsetmins[1 as i32 as usize];
    dest[2 as i32 as usize] += offsetmins[2 as i32 as usize];
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        origin as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        dest.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ((1 as i32) << 10 as i32) - 1 as i32,
        1 as i32,
    );
    if tr.fraction as f64 == 1.0f64 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
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
/*
============
G_RadiusDamage
============
*/
#[no_mangle]

pub unsafe extern "C" fn G_RadiusDamage(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut attacker: *mut crate::g_local_h::gentity_t,
    mut damage: f32,
    mut radius: f32,
    mut ignore: *mut crate::g_local_h::gentity_t,
    mut mod_0: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut points: f32 = 0.;
    let mut dist: f32 = 0.;
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut entityList: [i32; 1024] = [0; 1024];
    let mut numListedEntities: i32 = 0;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut i: i32 = 0;
    let mut e: i32 = 0;
    let mut hitClient: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if radius < 1 as i32 as f32 {
        radius = 1 as i32 as f32
    }
    i = 0 as i32;
    while i < 3 as i32 {
        mins[i as usize] = *origin.offset(i as isize) - radius;
        maxs[i as usize] = *origin.offset(i as isize) + radius;
        i += 1
    }
    numListedEntities = crate::src::game::g_syscalls::trap_EntitiesInBox(
        mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        entityList.as_mut_ptr(),
        (1 as i32) << 10 as i32,
    );
    e = 0 as i32;
    while e < numListedEntities {
        ent = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(*entityList.as_mut_ptr().offset(e as isize) as isize)
            as *mut crate::g_local_h::gentity_t;
        if !(ent == ignore) {
            if !((*ent).takedamage as u64 == 0) {
                // find the distance from the edge of the bounding box
                i = 0 as i32;
                while i < 3 as i32 {
                    if *origin.offset(i as isize) < (*ent).r.absmin[i as usize] {
                        v[i as usize] = (*ent).r.absmin[i as usize] - *origin.offset(i as isize)
                    } else if *origin.offset(i as isize) > (*ent).r.absmax[i as usize] {
                        v[i as usize] = *origin.offset(i as isize) - (*ent).r.absmax[i as usize]
                    } else {
                        v[i as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t
                    }
                    i += 1
                }
                dist = VectorLength(v.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
                if !(dist >= radius) {
                    points = (damage as f64
                        * (1.0f64 - (dist / radius) as f64))
                        as f32;
                    if CanDamage(ent, origin) as u64 != 0 {
                        if crate::src::game::g_weapon::LogAccuracyHit(
                            ent as *mut crate::g_local_h::gentity_s,
                            attacker as *mut crate::g_local_h::gentity_s,
                        ) as u64
                            != 0
                        {
                            hitClient = crate::src::qcommon::q_shared::qtrue
                        }
                        dir[0 as i32 as usize] = (*ent).r.currentOrigin
                            [0 as i32 as usize]
                            - *origin.offset(0 as i32 as isize);
                        dir[1 as i32 as usize] = (*ent).r.currentOrigin
                            [1 as i32 as usize]
                            - *origin.offset(1 as i32 as isize);
                        dir[2 as i32 as usize] = (*ent).r.currentOrigin
                            [2 as i32 as usize]
                            - *origin.offset(2 as i32 as isize);
                        // push the center of mass higher than the origin so players
                        // get knocked into the air more
                        dir[2 as i32 as usize] += 24 as i32 as f32;
                        G_Damage(
                            ent,
                            0 as *mut crate::g_local_h::gentity_t,
                            attacker,
                            dir.as_mut_ptr(),
                            origin,
                            points as i32,
                            0x1 as i32,
                            mod_0,
                        );
                    }
                }
            }
        }
        e += 1
    }
    return hitClient;
}

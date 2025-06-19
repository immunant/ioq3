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
pub use crate::src::game::bg_misc::BG_AddPredictableEventToPlayerstate;
pub use crate::src::game::g_combat::G_Damage;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_Error;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_syscalls::trap_DebugPolygonCreate;
pub use crate::src::game::g_syscalls::trap_EntitiesInBox;
pub use crate::src::game::g_syscalls::trap_GetConfigstring;
pub use crate::src::game::g_syscalls::trap_LinkEntity;
pub use crate::src::game::g_syscalls::trap_LocateGameData;
pub use crate::src::game::g_syscalls::trap_SendServerCommand;
pub use crate::src::game::g_syscalls::trap_SetConfigstring;
pub use crate::src::game::g_syscalls::trap_UnlinkEntity;
pub use crate::src::game::g_utils::q_shared_h::CrossProduct;
pub use crate::src::game::g_utils::q_shared_h::VectorCompare;
pub use crate::src::qcommon::q_math::AngleVectors;
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
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shaderRemap_t {
    pub oldShader: [libc::c_char; 64],
    pub newShader: [libc::c_char; 64],
    pub timeOffset: f32,
}
#[no_mangle]

pub static mut remapCount: i32 = 0 as i32;
#[no_mangle]

pub static mut remappedShaders: [shaderRemap_t; 128] = [shaderRemap_t {
    oldShader: [0; 64],
    newShader: [0; 64],
    timeOffset: 0.,
}; 128];
#[no_mangle]

pub unsafe extern "C" fn AddRemap(
    mut oldShader: *const libc::c_char,
    mut newShader: *const libc::c_char,
    mut timeOffset: f32,
) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < remapCount {
        if crate::src::qcommon::q_shared::Q_stricmp(
            oldShader,
            remappedShaders[i as usize].oldShader.as_mut_ptr(),
        ) == 0 as i32
        {
            // found it, just update this one
            ::libc::strcpy(
                remappedShaders[i as usize].newShader.as_mut_ptr(),
                newShader,
            );
            remappedShaders[i as usize].timeOffset = timeOffset;
            return;
        }
        i += 1
    }
    if remapCount < 128 as i32 {
        ::libc::strcpy(
            remappedShaders[remapCount as usize].newShader.as_mut_ptr(),
            newShader,
        );
        ::libc::strcpy(
            remappedShaders[remapCount as usize].oldShader.as_mut_ptr(),
            oldShader,
        );
        remappedShaders[remapCount as usize].timeOffset = timeOffset;
        remapCount += 1
    };
}
#[no_mangle]

pub unsafe extern "C" fn BuildShaderStateConfig() -> *const libc::c_char {
    static mut buff: [libc::c_char; 4096] = [0; 4096];
    let mut out: [libc::c_char; 133] = [0; 133];
    let mut i: i32 = 0;
    crate::stdlib::memset(
        buff.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        1024 as i32 as libc::c_ulong,
    );
    i = 0 as i32;
    while i < remapCount {
        crate::src::qcommon::q_shared::Com_sprintf(
            out.as_mut_ptr(),
            64 as i32 * 2 as i32 + 5 as i32,
            b"%s=%s:%5.2f@\x00" as *const u8 as *const libc::c_char,
            remappedShaders[i as usize].oldShader.as_mut_ptr(),
            remappedShaders[i as usize].newShader.as_mut_ptr(),
            remappedShaders[i as usize].timeOffset as f64,
        );
        crate::src::qcommon::q_shared::Q_strcat(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
            out.as_mut_ptr(),
        );
        i += 1
    }
    return buff.as_mut_ptr();
}
/*
=========================================================================

model / sound configstring indexes

=========================================================================
*/
/*
================
G_FindConfigstringIndex

================
*/
#[no_mangle]

pub unsafe extern "C" fn G_FindConfigstringIndex(
    mut name: *mut libc::c_char,
    mut start: i32,
    mut max: i32,
    mut create: crate::src::qcommon::q_shared::qboolean,
) -> i32 {
    let mut i: i32 = 0;
    let mut s: [libc::c_char; 1024] = [0; 1024];
    if name.is_null() || *name.offset(0 as i32 as isize) == 0 {
        return 0 as i32;
    }
    i = 1 as i32;
    while i < max {
        crate::src::game::g_syscalls::trap_GetConfigstring(
            start + i,
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        if s[0 as i32 as usize] == 0 {
            break;
        }
        if ::libc::strcmp(s.as_mut_ptr(), name) == 0 {
            return i;
        }
        i += 1
    }
    if create as u64 == 0 {
        return 0 as i32;
    }
    if i == max {
        crate::src::game::g_main::G_Error(
            b"G_FindConfigstringIndex: overflow\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::src::game::g_syscalls::trap_SetConfigstring(start + i, name);
    return i;
}
#[no_mangle]

pub unsafe extern "C" fn G_ModelIndex(mut name: *mut libc::c_char) -> i32 {
    return G_FindConfigstringIndex(
        name,
        32 as i32,
        256 as i32,
        crate::src::qcommon::q_shared::qtrue,
    );
}
#[no_mangle]

pub unsafe extern "C" fn G_SoundIndex(mut name: *mut libc::c_char) -> i32 {
    return G_FindConfigstringIndex(
        name,
        32 as i32 + 256 as i32,
        256 as i32,
        crate::src::qcommon::q_shared::qtrue,
    );
}
//=====================================================================
/*
================
G_TeamCommand

Broadcasts a command to only a specific team
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_TeamCommand(
    mut team: crate::bg_public_h::team_t,
    mut cmd: *mut libc::c_char,
) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < crate::src::game::g_main::level.maxclients {
        if (*crate::src::game::g_main::level.clients.offset(i as isize))
            .pers
            .connected as u32
            == crate::g_local_h::CON_CONNECTED as i32 as u32
        {
            if (*crate::src::game::g_main::level.clients.offset(i as isize))
                .sess
                .sessionTeam as u32
                == team as u32
            {
                crate::src::game::g_syscalls::trap_SendServerCommand(
                    i,
                    crate::src::qcommon::q_shared::va(
                        b"%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        cmd,
                    ),
                );
            }
        }
        i += 1
    }
}
/*
=============
G_Find

Searches all active entities for the next one that holds
the matching string at fieldofs (use the FOFS() macro) in the structure.

Searches beginning at the entity after from, or the beginning if NULL
NULL will be returned if the end of the list is reached.

=============
*/
#[no_mangle]

pub unsafe extern "C" fn G_Find(
    mut from: *mut crate::g_local_h::gentity_t,
    mut fieldofs: i32,
    mut match_0: *const libc::c_char,
) -> *mut crate::g_local_h::gentity_t {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if from.is_null() {
        from = crate::src::game::g_main::g_entities.as_mut_ptr()
    } else {
        from = from.offset(1)
    }
    while from
        < &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(crate::src::game::g_main::level.num_entities as isize)
            as *mut crate::g_local_h::gentity_t
    {
        if !((*from).inuse as u64 == 0) {
            s = *((from as *mut crate::src::qcommon::q_shared::byte).offset(fieldofs as isize)
                as *mut *mut libc::c_char);
            if !s.is_null() {
                if crate::src::qcommon::q_shared::Q_stricmp(s, match_0) == 0 {
                    return from;
                }
            }
        }
        from = from.offset(1)
    }
    return 0 as *mut crate::g_local_h::gentity_t;
}
#[no_mangle]

pub unsafe extern "C" fn G_PickTarget(
    mut targetname: *mut libc::c_char,
) -> *mut crate::g_local_h::gentity_t {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut num_choices: i32 = 0 as i32;
    let mut choice: [*mut crate::g_local_h::gentity_t; 32] =
        [0 as *mut crate::g_local_h::gentity_t; 32];
    if targetname.is_null() {
        crate::src::game::g_main::G_Printf(
            b"G_PickTarget called with NULL targetname\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut crate::g_local_h::gentity_t;
    }
    loop {
        ent = G_Find(
            ent,
            &mut (*(0 as *mut crate::g_local_h::gentity_t)).targetname as *mut *mut libc::c_char
                as crate::stddef_h::size_t as i32,
            targetname,
        );
        if ent.is_null() {
            break;
        }
        let fresh0 = num_choices;
        num_choices = num_choices + 1;
        choice[fresh0 as usize] = ent;
        if num_choices == 32 as i32 {
            break;
        }
    }
    if num_choices == 0 {
        crate::src::game::g_main::G_Printf(
            b"G_PickTarget: target %s not found\n\x00" as *const u8 as *const libc::c_char,
            targetname,
        );
        return 0 as *mut crate::g_local_h::gentity_t;
    }
    return choice[(::libc::rand() % num_choices) as usize];
}
/*
==============================
G_UseTargets

"activator" should be set to the entity that initiated the firing.

Search for (string)targetname in all entities that
match (string)self.target and call their .use function

==============================
*/
#[no_mangle]

pub unsafe extern "C" fn G_UseTargets(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut activator: *mut crate::g_local_h::gentity_t,
) {
    let mut t: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    if ent.is_null() {
        return;
    }
    if !(*ent).targetShaderName.is_null() && !(*ent).targetShaderNewName.is_null() {
        let mut f: f32 =
            (crate::src::game::g_main::level.time as f64 * 0.001f64) as f32;
        AddRemap((*ent).targetShaderName, (*ent).targetShaderNewName, f);
        crate::src::game::g_syscalls::trap_SetConfigstring(
            24 as i32,
            BuildShaderStateConfig(),
        );
    }
    if (*ent).target.is_null() {
        return;
    }
    t = 0 as *mut crate::g_local_h::gentity_t;
    loop {
        t = G_Find(
            t,
            &mut (*(0 as *mut crate::g_local_h::gentity_t)).targetname as *mut *mut libc::c_char
                as crate::stddef_h::size_t as i32,
            (*ent).target,
        );
        if t.is_null() {
            break;
        }
        if t == ent {
            crate::src::game::g_main::G_Printf(
                b"WARNING: Entity used itself.\n\x00" as *const u8 as *const libc::c_char,
            );
        } else if (*t).use_0.is_some() {
            (*t).use_0.expect("non-null function pointer")(t, ent, activator);
        }
        if (*ent).inuse as u64 == 0 {
            crate::src::game::g_main::G_Printf(
                b"entity was removed while using targets\n\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
}
/*
=============
TempVector

This is just a convenience function
for making temporary vectors for function calls
=============
*/
#[no_mangle]

pub unsafe extern "C" fn tv(
    mut x: f32,
    mut y: f32,
    mut z: f32,
) -> *mut f32 {
    static mut index: i32 = 0;
    static mut vecs: [crate::src::qcommon::q_shared::vec3_t; 8] = [[0.; 3]; 8];
    let mut v: *mut f32 = 0 as *mut f32;
    // use an array so that multiple tempvectors won't collide
    // for a while
    v = vecs[index as usize].as_mut_ptr();
    index = index + 1 as i32 & 7 as i32;
    *v.offset(0 as i32 as isize) = x;
    *v.offset(1 as i32 as isize) = y;
    *v.offset(2 as i32 as isize) = z;
    return v;
}
/*
=============
VectorToString

This is just a convenience function
for printing vectors
=============
*/
#[no_mangle]

pub unsafe extern "C" fn vtos(
    mut v: *const crate::src::qcommon::q_shared::vec_t,
) -> *mut libc::c_char {
    static mut index: i32 = 0;
    static mut str: [[libc::c_char; 32]; 8] = [[0; 32]; 8];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    // use an array so that multiple vtos won't collide
    s = str[index as usize].as_mut_ptr();
    index = index + 1 as i32 & 7 as i32;
    crate::src::qcommon::q_shared::Com_sprintf(
        s,
        32 as i32,
        b"(%i %i %i)\x00" as *const u8 as *const libc::c_char,
        *v.offset(0 as i32 as isize) as i32,
        *v.offset(1 as i32 as isize) as i32,
        *v.offset(2 as i32 as isize) as i32,
    );
    return s;
}
/*
===============
G_SetMovedir

The editor only specifies a single value for angles (yaw),
but we have special constants to generate an up or down direction.
Angles will be cleared, because it is being used to represent a direction
instead of an orientation.
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_SetMovedir(
    mut angles: *mut crate::src::qcommon::q_shared::vec_t,
    mut movedir: *mut crate::src::qcommon::q_shared::vec_t,
) {
    static mut VEC_UP: crate::src::qcommon::q_shared::vec3_t = [
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        -(1 as i32) as crate::src::qcommon::q_shared::vec_t,
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
    ];
    static mut MOVEDIR_UP: crate::src::qcommon::q_shared::vec3_t = [
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        1 as i32 as crate::src::qcommon::q_shared::vec_t,
    ];
    static mut VEC_DOWN: crate::src::qcommon::q_shared::vec3_t = [
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        -(2 as i32) as crate::src::qcommon::q_shared::vec_t,
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
    ];
    static mut MOVEDIR_DOWN: crate::src::qcommon::q_shared::vec3_t = [
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        -(1 as i32) as crate::src::qcommon::q_shared::vec_t,
    ];
    if VectorCompare(
        angles as *const crate::src::qcommon::q_shared::vec_t,
        VEC_UP.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    ) != 0
    {
        *movedir.offset(0 as i32 as isize) = MOVEDIR_UP[0 as i32 as usize];
        *movedir.offset(1 as i32 as isize) = MOVEDIR_UP[1 as i32 as usize];
        *movedir.offset(2 as i32 as isize) = MOVEDIR_UP[2 as i32 as usize]
    } else if VectorCompare(
        angles as *const crate::src::qcommon::q_shared::vec_t,
        VEC_DOWN.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    ) != 0
    {
        *movedir.offset(0 as i32 as isize) = MOVEDIR_DOWN[0 as i32 as usize];
        *movedir.offset(1 as i32 as isize) = MOVEDIR_DOWN[1 as i32 as usize];
        *movedir.offset(2 as i32 as isize) = MOVEDIR_DOWN[2 as i32 as usize]
    } else {
        crate::src::qcommon::q_math::AngleVectors(
            angles as *const crate::src::qcommon::q_shared::vec_t,
            movedir,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
        );
    }
    let ref mut fresh1 = *angles.offset(2 as i32 as isize);
    *fresh1 = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh2 = *angles.offset(1 as i32 as isize);
    *fresh2 = *fresh1;
    *angles.offset(0 as i32 as isize) = *fresh2;
}
#[no_mangle]

pub unsafe extern "C" fn vectoyaw(
    mut vec: *const crate::src::qcommon::q_shared::vec_t,
) -> f32 {
    let mut yaw: f32 = 0.;
    if *vec.offset(1 as i32 as isize) == 0 as i32 as f32
        && *vec.offset(0 as i32 as isize) == 0 as i32 as f32
    {
        yaw = 0 as i32 as f32
    } else {
        if *vec.offset(0 as i32 as isize) != 0. {
            yaw = (crate::stdlib::atan2(
                *vec.offset(1 as i32 as isize) as f64,
                *vec.offset(0 as i32 as isize) as f64,
            ) * 180 as i32 as f64
                / 3.14159265358979323846f64) as f32
        } else if *vec.offset(1 as i32 as isize) > 0 as i32 as f32 {
            yaw = 90 as i32 as f32
        } else {
            yaw = 270 as i32 as f32
        }
        if yaw < 0 as i32 as f32 {
            yaw += 360 as i32 as f32
        }
    }
    return yaw;
}
#[no_mangle]

pub unsafe extern "C" fn G_InitGentity(mut e: *mut crate::g_local_h::gentity_t) {
    (*e).inuse = crate::src::qcommon::q_shared::qtrue;
    (*e).classname = b"noclass\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*e).s.number = e.offset_from(crate::src::game::g_main::g_entities.as_mut_ptr()) as libc::c_long
        as i32;
    (*e).r.ownerNum = ((1 as i32) << 10 as i32) - 1 as i32;
}
/*
=================
G_Spawn

Either finds a free entity, or allocates a new one.

  The slots from 0 to MAX_CLIENTS-1 are always reserved for clients, and will
never be used by anything else.

Try to avoid reusing an entity that was recently freed, because it
can cause the client to think the entity morphed into something else
instead of being removed and recreated, which can cause interpolated
angles and bad trails.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn G_Spawn() -> *mut crate::g_local_h::gentity_t {
    let mut i: i32 = 0; // shut up warning
    let mut force: i32 = 0;
    let mut e: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    e = 0 as *mut crate::g_local_h::gentity_t;
    force = 0 as i32;
    while force < 2 as i32 {
        // if we go through all entities and can't find one to free,
        // override the normal minimum times before use
        e = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(64 as i32 as isize) as *mut crate::g_local_h::gentity_t;
        i = 64 as i32;
        while i < crate::src::game::g_main::level.num_entities {
            if !((*e).inuse as u64 != 0) {
                // the first couple seconds of server time can involve a lot of
                // freeing and allocating, so relax the replacement policy
                if !(force == 0
                    && (*e).freetime
                        > crate::src::game::g_main::level.startTime + 2000 as i32
                    && crate::src::game::g_main::level.time - (*e).freetime < 1000 as i32)
                {
                    // reuse this slot
                    G_InitGentity(e);
                    return e;
                }
            }
            i += 1;
            e = e.offset(1)
        }
        if crate::src::game::g_main::level.num_entities
            < ((1 as i32) << 10 as i32) - 2 as i32
        {
            break;
        }
        force += 1
    }
    if crate::src::game::g_main::level.num_entities
        == ((1 as i32) << 10 as i32) - 2 as i32
    {
        i = 0 as i32;
        while i < (1 as i32) << 10 as i32 {
            crate::src::game::g_main::G_Printf(
                b"%4i: %s\n\x00" as *const u8 as *const libc::c_char,
                i,
                crate::src::game::g_main::g_entities[i as usize].classname,
            );
            i += 1
        }
        crate::src::game::g_main::G_Error(
            b"G_Spawn: no free entities\x00" as *const u8 as *const libc::c_char,
        );
    }
    // open up a new slot
    crate::src::game::g_main::level.num_entities += 1;
    // let the server system know that there are more entities
    crate::src::game::g_syscalls::trap_LocateGameData(
        crate::src::game::g_main::level.gentities as *mut crate::g_local_h::gentity_s,
        crate::src::game::g_main::level.num_entities,
        ::std::mem::size_of::<crate::g_local_h::gentity_t>() as libc::c_ulong as i32,
        &mut (*crate::src::game::g_main::level
            .clients
            .offset(0 as i32 as isize))
        .ps as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
        ::std::mem::size_of::<crate::g_local_h::gclient_s>() as libc::c_ulong as i32,
    );
    G_InitGentity(e);
    return e;
}
/*
=================
G_EntitiesFree
=================
*/
#[no_mangle]

pub unsafe extern "C" fn G_EntitiesFree() -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    let mut e: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    if crate::src::game::g_main::level.num_entities
        < ((1 as i32) << 10 as i32) - 2 as i32
    {
        // can open a new slot if needed
        return crate::src::qcommon::q_shared::qtrue;
    }
    e = &mut *crate::src::game::g_main::g_entities
        .as_mut_ptr()
        .offset(64 as i32 as isize) as *mut crate::g_local_h::gentity_t;
    i = 64 as i32;
    while i < crate::src::game::g_main::level.num_entities {
        if (*e).inuse as u64 != 0 {
            i += 1;
            e = e.offset(1)
        } else {
            // slot available
            return crate::src::qcommon::q_shared::qtrue;
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
=================
G_FreeEntity

Marks the entity as free
=================
*/
#[no_mangle]

pub unsafe extern "C" fn G_FreeEntity(mut ed: *mut crate::g_local_h::gentity_t) {
    crate::src::game::g_syscalls::trap_UnlinkEntity(ed as *mut crate::g_local_h::gentity_s); // unlink from world
    if (*ed).neverFree as u64 != 0 {
        return;
    }
    crate::stdlib::memset(
        ed as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::g_local_h::gentity_t>() as libc::c_ulong,
    );
    (*ed).classname = b"freed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*ed).freetime = crate::src::game::g_main::level.time;
    (*ed).inuse = crate::src::qcommon::q_shared::qfalse;
}
/*
=================
G_TempEntity

Spawns an event entity that will be auto-removed
The origin will be snapped to save net bandwidth, so care
must be taken if the origin is right on a surface (snap towards start vector first)
=================
*/
#[no_mangle]

pub unsafe extern "C" fn G_TempEntity(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut event: i32,
) -> *mut crate::g_local_h::gentity_t {
    let mut e: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut snapped: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    e = G_Spawn();
    (*e).s.eType = crate::bg_public_h::ET_EVENTS as i32 + event;
    (*e).classname = b"tempEntity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*e).eventTime = crate::src::game::g_main::level.time;
    (*e).freeAfterEvent = crate::src::qcommon::q_shared::qtrue;
    snapped[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    snapped[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    snapped[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
    snapped[0 as i32 as usize] =
        snapped[0 as i32 as usize] as i32 as crate::src::qcommon::q_shared::vec_t;
    snapped[1 as i32 as usize] =
        snapped[1 as i32 as usize] as i32 as crate::src::qcommon::q_shared::vec_t;
    snapped[2 as i32 as usize] =
        snapped[2 as i32 as usize] as i32 as crate::src::qcommon::q_shared::vec_t;
    // save network bandwidth
    G_SetOrigin(e, snapped.as_mut_ptr());
    // find cluster for PVS
    crate::src::game::g_syscalls::trap_LinkEntity(e as *mut crate::g_local_h::gentity_s);
    return e;
}
/*
==============================================================================

Kill box

==============================================================================
*/
/*
=================
G_KillBox

Kills all entities that would touch the proposed new positioning
of ent.  Ent should be unlinked before calling this!
=================
*/
#[no_mangle]

pub unsafe extern "C" fn G_KillBox(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut i: i32 = 0;
    let mut num: i32 = 0;
    let mut touch: [i32; 1024] = [0; 1024];
    let mut hit: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    mins[0 as i32 as usize] = (*(*ent).client).ps.origin[0 as i32 as usize]
        + (*ent).r.mins[0 as i32 as usize];
    mins[1 as i32 as usize] = (*(*ent).client).ps.origin[1 as i32 as usize]
        + (*ent).r.mins[1 as i32 as usize];
    mins[2 as i32 as usize] = (*(*ent).client).ps.origin[2 as i32 as usize]
        + (*ent).r.mins[2 as i32 as usize];
    maxs[0 as i32 as usize] = (*(*ent).client).ps.origin[0 as i32 as usize]
        + (*ent).r.maxs[0 as i32 as usize];
    maxs[1 as i32 as usize] = (*(*ent).client).ps.origin[1 as i32 as usize]
        + (*ent).r.maxs[1 as i32 as usize];
    maxs[2 as i32 as usize] = (*(*ent).client).ps.origin[2 as i32 as usize]
        + (*ent).r.maxs[2 as i32 as usize];
    num = crate::src::game::g_syscalls::trap_EntitiesInBox(
        mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        touch.as_mut_ptr(),
        (1 as i32) << 10 as i32,
    );
    i = 0 as i32;
    while i < num {
        hit = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(*touch.as_mut_ptr().offset(i as isize) as isize)
            as *mut crate::g_local_h::gentity_t;
        if !(*hit).client.is_null() {
            // nail it
            crate::src::game::g_combat::G_Damage(
                hit as *mut crate::g_local_h::gentity_s,
                ent as *mut crate::g_local_h::gentity_s,
                ent as *mut crate::g_local_h::gentity_s,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                100000 as i32,
                0x8 as i32,
                crate::bg_public_h::MOD_TELEFRAG as i32,
            );
        }
        i += 1
    }
}
//==============================================================================
/*
===============
G_AddPredictableEvent

Use for non-pmove events that would also be predicted on the
client side: jumppads and item pickups
Adds an event+parm and twiddles the event counter
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_AddPredictableEvent(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut event: i32,
    mut eventParm: i32,
) {
    if (*ent).client.is_null() {
        return;
    }
    crate::src::game::bg_misc::BG_AddPredictableEventToPlayerstate(
        event,
        eventParm,
        &mut (*(*ent).client).ps as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
    );
}
/*
===============
G_AddEvent

Adds an event+parm and twiddles the event counter
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_AddEvent(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut event: i32,
    mut eventParm: i32,
) {
    let mut bits: i32 = 0;
    if event == 0 {
        crate::src::game::g_main::G_Printf(
            b"G_AddEvent: zero event added for entity %i\n\x00" as *const u8 as *const libc::c_char,
            (*ent).s.number,
        );
        return;
    }
    // clients need to add the event in playerState_t instead of entityState_t
    if !(*ent).client.is_null() {
        bits = (*(*ent).client).ps.externalEvent & (0x100 as i32 | 0x200 as i32);
        bits = bits + 0x100 as i32 & (0x100 as i32 | 0x200 as i32);
        (*(*ent).client).ps.externalEvent = event | bits;
        (*(*ent).client).ps.externalEventParm = eventParm;
        (*(*ent).client).ps.externalEventTime = crate::src::game::g_main::level.time
    } else {
        bits = (*ent).s.event & (0x100 as i32 | 0x200 as i32);
        bits = bits + 0x100 as i32 & (0x100 as i32 | 0x200 as i32);
        (*ent).s.event = event | bits;
        (*ent).s.eventParm = eventParm
    }
    (*ent).eventTime = crate::src::game::g_main::level.time;
}
/*
=============
G_Sound
=============
*/
#[no_mangle]

pub unsafe extern "C" fn G_Sound(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut _channel: i32,
    mut soundIndex: i32,
) {
    let mut te: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    te = G_TempEntity(
        (*ent).r.currentOrigin.as_mut_ptr(),
        crate::bg_public_h::EV_GENERAL_SOUND as i32,
    );
    (*te).s.eventParm = soundIndex;
}
//==============================================================================
/*
================
G_SetOrigin

Sets the pos trajectory for a fixed position
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_SetOrigin(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) {
    (*ent).s.pos.trBase[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    (*ent).s.pos.trBase[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    (*ent).s.pos.trBase[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
    (*ent).s.pos.trType = crate::src::qcommon::q_shared::TR_STATIONARY;
    (*ent).s.pos.trTime = 0 as i32;
    (*ent).s.pos.trDuration = 0 as i32;
    (*ent).s.pos.trDelta[2 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    (*ent).s.pos.trDelta[1 as i32 as usize] =
        (*ent).s.pos.trDelta[2 as i32 as usize];
    (*ent).s.pos.trDelta[0 as i32 as usize] =
        (*ent).s.pos.trDelta[1 as i32 as usize];
    (*ent).r.currentOrigin[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    (*ent).r.currentOrigin[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    (*ent).r.currentOrigin[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
}
/*
================
DebugLine

  debug polygons only work when running a local game
  with r_debugSurface set to 2
================
*/
#[no_mangle]

pub unsafe extern "C" fn DebugLine(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut color: i32,
) -> i32 {
    let mut points: [crate::src::qcommon::q_shared::vec3_t; 4] = [[0.; 3]; 4];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cross: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        1 as i32 as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut dot: f32 = 0.;
    points[0 as i32 as usize][0 as i32 as usize] =
        *start.offset(0 as i32 as isize);
    points[0 as i32 as usize][1 as i32 as usize] =
        *start.offset(1 as i32 as isize);
    points[0 as i32 as usize][2 as i32 as usize] =
        *start.offset(2 as i32 as isize);
    points[1 as i32 as usize][0 as i32 as usize] =
        *start.offset(0 as i32 as isize);
    points[1 as i32 as usize][1 as i32 as usize] =
        *start.offset(1 as i32 as isize);
    points[1 as i32 as usize][2 as i32 as usize] =
        *start.offset(2 as i32 as isize);
    //points[1][2] -= 2;
    points[2 as i32 as usize][0 as i32 as usize] =
        *end.offset(0 as i32 as isize);
    points[2 as i32 as usize][1 as i32 as usize] =
        *end.offset(1 as i32 as isize);
    points[2 as i32 as usize][2 as i32 as usize] =
        *end.offset(2 as i32 as isize);
    //points[2][2] -= 2;
    points[3 as i32 as usize][0 as i32 as usize] =
        *end.offset(0 as i32 as isize);
    points[3 as i32 as usize][1 as i32 as usize] =
        *end.offset(1 as i32 as isize);
    points[3 as i32 as usize][2 as i32 as usize] =
        *end.offset(2 as i32 as isize);
    dir[0 as i32 as usize] =
        *end.offset(0 as i32 as isize) - *start.offset(0 as i32 as isize);
    dir[1 as i32 as usize] =
        *end.offset(1 as i32 as isize) - *start.offset(1 as i32 as isize);
    dir[2 as i32 as usize] =
        *end.offset(2 as i32 as isize) - *start.offset(2 as i32 as isize);
    crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    dot = dir[0 as i32 as usize] * up[0 as i32 as usize]
        + dir[1 as i32 as usize] * up[1 as i32 as usize]
        + dir[2 as i32 as usize] * up[2 as i32 as usize];
    if dot as f64 > 0.99f64 || (dot as f64) < -0.99f64 {
        cross[0 as i32 as usize] = 1 as i32 as crate::src::qcommon::q_shared::vec_t;
        cross[1 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
        cross[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t
    } else {
        CrossProduct(
            dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            up.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            cross.as_mut_ptr(),
        );
    }
    crate::src::qcommon::q_math::VectorNormalize(cross.as_mut_ptr());
    points[0 as i32 as usize][0 as i32 as usize] = points
        [0 as i32 as usize][0 as i32 as usize]
        + cross[0 as i32 as usize] * 2 as i32 as f32;
    points[0 as i32 as usize][1 as i32 as usize] = points
        [0 as i32 as usize][1 as i32 as usize]
        + cross[1 as i32 as usize] * 2 as i32 as f32;
    points[0 as i32 as usize][2 as i32 as usize] = points
        [0 as i32 as usize][2 as i32 as usize]
        + cross[2 as i32 as usize] * 2 as i32 as f32;
    points[1 as i32 as usize][0 as i32 as usize] = points
        [1 as i32 as usize][0 as i32 as usize]
        + cross[0 as i32 as usize] * -(2 as i32) as f32;
    points[1 as i32 as usize][1 as i32 as usize] = points
        [1 as i32 as usize][1 as i32 as usize]
        + cross[1 as i32 as usize] * -(2 as i32) as f32;
    points[1 as i32 as usize][2 as i32 as usize] = points
        [1 as i32 as usize][2 as i32 as usize]
        + cross[2 as i32 as usize] * -(2 as i32) as f32;
    points[2 as i32 as usize][0 as i32 as usize] = points
        [2 as i32 as usize][0 as i32 as usize]
        + cross[0 as i32 as usize] * -(2 as i32) as f32;
    points[2 as i32 as usize][1 as i32 as usize] = points
        [2 as i32 as usize][1 as i32 as usize]
        + cross[1 as i32 as usize] * -(2 as i32) as f32;
    points[2 as i32 as usize][2 as i32 as usize] = points
        [2 as i32 as usize][2 as i32 as usize]
        + cross[2 as i32 as usize] * -(2 as i32) as f32;
    points[3 as i32 as usize][0 as i32 as usize] = points
        [3 as i32 as usize][0 as i32 as usize]
        + cross[0 as i32 as usize] * 2 as i32 as f32;
    points[3 as i32 as usize][1 as i32 as usize] = points
        [3 as i32 as usize][1 as i32 as usize]
        + cross[1 as i32 as usize] * 2 as i32 as f32;
    points[3 as i32 as usize][2 as i32 as usize] = points
        [3 as i32 as usize][2 as i32 as usize]
        + cross[2 as i32 as usize] * 2 as i32 as f32;
    return crate::src::game::g_syscalls::trap_DebugPolygonCreate(
        color,
        4 as i32,
        points.as_mut_ptr(),
    );
}

use ::libc;

pub mod q_shared_h {

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

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::GENDER_FEMALE;
pub use crate::bg_public_h::GENDER_MALE;
pub use crate::bg_public_h::GENDER_NEUTER;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
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
pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::cg_local_h::FOOTSTEP_BOOT;
pub use crate::cg_local_h::FOOTSTEP_ENERGY;
pub use crate::cg_local_h::FOOTSTEP_FLESH;
pub use crate::cg_local_h::FOOTSTEP_MECH;
pub use crate::cg_local_h::FOOTSTEP_METAL;
pub use crate::cg_local_h::FOOTSTEP_NORMAL;
pub use crate::cg_local_h::FOOTSTEP_SPLASH;
pub use crate::cg_local_h::FOOTSTEP_TOTAL;
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::cgame::cg_event::CG_EntityEvent;
pub use crate::src::cgame::cg_event::CG_PainEvent;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_entities;
pub use crate::src::cgame::cg_main::cg_showmiss;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_Printf;
pub use crate::src::cgame::cg_playerstate::q_shared_h::VectorLength;
pub use crate::src::cgame::cg_syscalls::trap_S_StartLocalSound;
pub use crate::src::cgame::cg_view::CG_AddBufferedSound;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
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

pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::RT_BEAM;
pub use crate::tr_types_h::RT_LIGHTNING;
pub use crate::tr_types_h::RT_MAX_REF_ENTITY_TYPE;
pub use crate::tr_types_h::RT_MODEL;
pub use crate::tr_types_h::RT_POLY;
pub use crate::tr_types_h::RT_PORTALSURFACE;
pub use crate::tr_types_h::RT_RAIL_CORE;
pub use crate::tr_types_h::RT_RAIL_RINGS;
pub use crate::tr_types_h::RT_SPRITE;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
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
// cg_playerstate.c -- this file acts on changes in a new playerState_t
// With normal play, this will be done after local prediction, but when
// following another player or playing back a demo, it will be checked
// when the snapshot transitions like all the other entities
/*
==============
CG_CheckAmmo

If the ammo has gone low enough to generate the warning, play a sound
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_CheckAmmo() {
    let mut i: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut previous: libc::c_int = 0;
    let mut weapons: libc::c_int = 0;
    // see about how many seconds of ammo we have remaining
    weapons = (*crate::src::cgame::cg_main::cg.snap).ps.stats
        [crate::bg_public_h::STAT_WEAPONS as libc::c_int as usize];
    total = 0 as libc::c_int;
    i = crate::bg_public_h::WP_MACHINEGUN as libc::c_int;
    while i < crate::bg_public_h::WP_NUM_WEAPONS as libc::c_int {
        if !(weapons & (1 as libc::c_int) << i == 0) {
            if !((*crate::src::cgame::cg_main::cg.snap).ps.ammo[i as usize] < 0 as libc::c_int) {
                match i {
                    5 | 4 | 7 | 3 => {
                        total += (*crate::src::cgame::cg_main::cg.snap).ps.ammo[i as usize]
                            * 1000 as libc::c_int
                    }
                    _ => {
                        total += (*crate::src::cgame::cg_main::cg.snap).ps.ammo[i as usize]
                            * 200 as libc::c_int
                    }
                }
                if total >= 5000 as libc::c_int {
                    crate::src::cgame::cg_main::cg.lowAmmoWarning = 0 as libc::c_int;
                    return;
                }
            }
        }
        i += 1
    }
    previous = crate::src::cgame::cg_main::cg.lowAmmoWarning;
    if total == 0 as libc::c_int {
        crate::src::cgame::cg_main::cg.lowAmmoWarning = 2 as libc::c_int
    } else {
        crate::src::cgame::cg_main::cg.lowAmmoWarning = 1 as libc::c_int
    }
    // play a sound on transitions
    if crate::src::cgame::cg_main::cg.lowAmmoWarning != previous {
        crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
            crate::src::cgame::cg_main::cgs.media.noAmmoSound,
            crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND as libc::c_int,
        );
    };
}
/*
==============
CG_DamageFeedback
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DamageFeedback(
    mut yawByte: libc::c_int,
    mut pitchByte: libc::c_int,
    mut damage: libc::c_int,
) {
    let mut left: libc::c_float = 0.;
    let mut front: libc::c_float = 0.;
    let mut up: libc::c_float = 0.;
    let mut kick: libc::c_float = 0.;
    let mut health: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut yaw: libc::c_float = 0.;
    let mut pitch: libc::c_float = 0.;
    // show the attacking player's head and name in corner
    crate::src::cgame::cg_main::cg.attackerTime = crate::src::cgame::cg_main::cg.time;
    // the lower on health you are, the greater the view kick will be
    health = (*crate::src::cgame::cg_main::cg.snap).ps.stats
        [crate::bg_public_h::STAT_HEALTH as libc::c_int as usize];
    if health < 40 as libc::c_int {
        scale = 1 as libc::c_int as libc::c_float
    } else {
        scale = (40.0f64 / health as libc::c_double) as libc::c_float
    }
    kick = damage as libc::c_float * scale;
    if kick < 5 as libc::c_int as libc::c_float {
        kick = 5 as libc::c_int as libc::c_float
    }
    if kick > 10 as libc::c_int as libc::c_float {
        kick = 10 as libc::c_int as libc::c_float
    }
    // if yaw and pitch are both 255, make the damage always centered (falling, etc)
    if yawByte == 255 as libc::c_int && pitchByte == 255 as libc::c_int {
        crate::src::cgame::cg_main::cg.damageX = 0 as libc::c_int as libc::c_float;
        crate::src::cgame::cg_main::cg.damageY = 0 as libc::c_int as libc::c_float;
        crate::src::cgame::cg_main::cg.v_dmg_roll = 0 as libc::c_int as libc::c_float;
        crate::src::cgame::cg_main::cg.v_dmg_pitch = -kick
    } else {
        // positional
        pitch = (pitchByte as libc::c_double / 255.0f64 * 360 as libc::c_int as libc::c_double)
            as libc::c_float;
        yaw = (yawByte as libc::c_double / 255.0f64 * 360 as libc::c_int as libc::c_double)
            as libc::c_float;
        angles[0 as libc::c_int as usize] = pitch;
        angles[1 as libc::c_int as usize] = yaw;
        angles[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        crate::src::qcommon::q_math::AngleVectors(
            angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            dir.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
        );
        dir[0 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
            [0 as libc::c_int as usize]
            - dir[0 as libc::c_int as usize];
        dir[1 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
            [1 as libc::c_int as usize]
            - dir[1 as libc::c_int as usize];
        dir[2 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
            [2 as libc::c_int as usize]
            - dir[2 as libc::c_int as usize];
        front = dir[0 as libc::c_int as usize]
            * crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as libc::c_int as usize]
                [0 as libc::c_int as usize]
            + dir[1 as libc::c_int as usize]
                * crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as libc::c_int as usize]
                    [1 as libc::c_int as usize]
            + dir[2 as libc::c_int as usize]
                * crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as libc::c_int as usize]
                    [2 as libc::c_int as usize];
        left = dir[0 as libc::c_int as usize]
            * crate::src::cgame::cg_main::cg.refdef.viewaxis[1 as libc::c_int as usize]
                [0 as libc::c_int as usize]
            + dir[1 as libc::c_int as usize]
                * crate::src::cgame::cg_main::cg.refdef.viewaxis[1 as libc::c_int as usize]
                    [1 as libc::c_int as usize]
            + dir[2 as libc::c_int as usize]
                * crate::src::cgame::cg_main::cg.refdef.viewaxis[1 as libc::c_int as usize]
                    [2 as libc::c_int as usize];
        up = dir[0 as libc::c_int as usize]
            * crate::src::cgame::cg_main::cg.refdef.viewaxis[2 as libc::c_int as usize]
                [0 as libc::c_int as usize]
            + dir[1 as libc::c_int as usize]
                * crate::src::cgame::cg_main::cg.refdef.viewaxis[2 as libc::c_int as usize]
                    [1 as libc::c_int as usize]
            + dir[2 as libc::c_int as usize]
                * crate::src::cgame::cg_main::cg.refdef.viewaxis[2 as libc::c_int as usize]
                    [2 as libc::c_int as usize];
        dir[0 as libc::c_int as usize] = front;
        dir[1 as libc::c_int as usize] = left;
        dir[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        dist = VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
        if (dist as libc::c_double) < 0.1f64 {
            dist = 0.1f32
        }
        crate::src::cgame::cg_main::cg.v_dmg_roll = kick * left;
        crate::src::cgame::cg_main::cg.v_dmg_pitch = -kick * front;
        if front as libc::c_double <= 0.1f64 {
            front = 0.1f32
        }
        crate::src::cgame::cg_main::cg.damageX = -left / front;
        crate::src::cgame::cg_main::cg.damageY = up / dist
    }
    // clamp the position
    if crate::src::cgame::cg_main::cg.damageX as libc::c_double > 1.0f64 {
        crate::src::cgame::cg_main::cg.damageX = 1.0f64 as libc::c_float
    }
    if (crate::src::cgame::cg_main::cg.damageX as libc::c_double) < -1.0f64 {
        crate::src::cgame::cg_main::cg.damageX = -1.0f64 as libc::c_float
    }
    if crate::src::cgame::cg_main::cg.damageY as libc::c_double > 1.0f64 {
        crate::src::cgame::cg_main::cg.damageY = 1.0f64 as libc::c_float
    }
    if (crate::src::cgame::cg_main::cg.damageY as libc::c_double) < -1.0f64 {
        crate::src::cgame::cg_main::cg.damageY = -1.0f64 as libc::c_float
    }
    // don't let the screen flashes vary as much
    if kick > 10 as libc::c_int as libc::c_float {
        kick = 10 as libc::c_int as libc::c_float
    }
    crate::src::cgame::cg_main::cg.damageValue = kick;
    crate::src::cgame::cg_main::cg.v_dmg_time =
        (crate::src::cgame::cg_main::cg.time + 500 as libc::c_int) as libc::c_float;
    crate::src::cgame::cg_main::cg.damageTime =
        (*crate::src::cgame::cg_main::cg.snap).serverTime as libc::c_float;
}
/*
================
CG_Respawn

A respawn happened this snapshot
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_Respawn() {
    // no error decay on player movement
    crate::src::cgame::cg_main::cg.thisFrameTeleport = crate::src::qcommon::q_shared::qtrue;
    // display weapons available
    crate::src::cgame::cg_main::cg.weaponSelectTime = crate::src::cgame::cg_main::cg.time;
    // select the weapon the server says we are using
    crate::src::cgame::cg_main::cg.weaponSelect = (*crate::src::cgame::cg_main::cg.snap).ps.weapon;
}
/*
==============
CG_CheckPlayerstateEvents
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_CheckPlayerstateEvents(
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
    mut ops: *mut crate::src::qcommon::q_shared::playerState_t,
) {
    let mut i: libc::c_int = 0; // cg_entities[ ps->clientNum ];
    let mut event: libc::c_int = 0;
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    if (*ps).externalEvent != 0 && (*ps).externalEvent != (*ops).externalEvent {
        cent = &mut *crate::src::cgame::cg_main::cg_entities
            .as_mut_ptr()
            .offset((*ps).clientNum as isize) as *mut crate::cg_local_h::centity_t;
        (*cent).currentState.event = (*ps).externalEvent;
        (*cent).currentState.eventParm = (*ps).externalEventParm;
        crate::src::cgame::cg_event::CG_EntityEvent(
            cent as *mut crate::cg_local_h::centity_s,
            (*cent).lerpOrigin.as_mut_ptr(),
        );
    }
    cent = &mut crate::src::cgame::cg_main::cg.predictedPlayerEntity;
    // go through the predictable events buffer
    i = (*ps).eventSequence - 2 as libc::c_int;
    while i < (*ps).eventSequence {
        // if we have a new predictable event
        if i >= (*ops).eventSequence
            || i > (*ops).eventSequence - 2 as libc::c_int
                && (*ps).events[(i & 2 as libc::c_int - 1 as libc::c_int) as usize]
                    != (*ops).events[(i & 2 as libc::c_int - 1 as libc::c_int) as usize]
        {
            event = (*ps).events[(i & 2 as libc::c_int - 1 as libc::c_int) as usize];
            (*cent).currentState.event = event;
            (*cent).currentState.eventParm =
                (*ps).eventParms[(i & 2 as libc::c_int - 1 as libc::c_int) as usize];
            crate::src::cgame::cg_event::CG_EntityEvent(
                cent as *mut crate::cg_local_h::centity_s,
                (*cent).lerpOrigin.as_mut_ptr(),
            );
            crate::src::cgame::cg_main::cg.predictableEvents
                [(i & 16 as libc::c_int - 1 as libc::c_int) as usize] = event;
            crate::src::cgame::cg_main::cg.eventSequence += 1
        }
        i += 1
    }
}
/*
==================
CG_CheckChangedPredictableEvents
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_CheckChangedPredictableEvents(
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
) {
    let mut i: libc::c_int = 0;
    let mut event: libc::c_int = 0;
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    cent = &mut crate::src::cgame::cg_main::cg.predictedPlayerEntity;
    i = (*ps).eventSequence - 2 as libc::c_int;
    while i < (*ps).eventSequence {
        //
        if !(i >= crate::src::cgame::cg_main::cg.eventSequence) {
            // if this event is not further back in than the maximum predictable events we remember
            if i > crate::src::cgame::cg_main::cg.eventSequence - 16 as libc::c_int {
                // if the new playerstate event is different from a previously predicted one
                if (*ps).events[(i & 2 as libc::c_int - 1 as libc::c_int) as usize]
                    != crate::src::cgame::cg_main::cg.predictableEvents
                        [(i & 16 as libc::c_int - 1 as libc::c_int) as usize]
                {
                    event = (*ps).events[(i & 2 as libc::c_int - 1 as libc::c_int) as usize];
                    (*cent).currentState.event = event;
                    (*cent).currentState.eventParm =
                        (*ps).eventParms[(i & 2 as libc::c_int - 1 as libc::c_int) as usize];
                    crate::src::cgame::cg_event::CG_EntityEvent(
                        cent as *mut crate::cg_local_h::centity_s,
                        (*cent).lerpOrigin.as_mut_ptr(),
                    );
                    crate::src::cgame::cg_main::cg.predictableEvents
                        [(i & 16 as libc::c_int - 1 as libc::c_int) as usize] = event;
                    if crate::src::cgame::cg_main::cg_showmiss.integer != 0 {
                        crate::src::cgame::cg_main::CG_Printf(
                            b"WARNING: changed predicted event\n\x00" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
            }
        }
        i += 1
    }
}
/*
==================
pushReward
==================
*/

unsafe extern "C" fn pushReward(
    mut sfx: crate::src::qcommon::q_shared::sfxHandle_t,
    mut shader: crate::src::qcommon::q_shared::qhandle_t,
    mut rewardCount: libc::c_int,
) {
    if crate::src::cgame::cg_main::cg.rewardStack < 10 as libc::c_int - 1 as libc::c_int {
        crate::src::cgame::cg_main::cg.rewardStack += 1;
        crate::src::cgame::cg_main::cg.rewardSound
            [crate::src::cgame::cg_main::cg.rewardStack as usize] = sfx;
        crate::src::cgame::cg_main::cg.rewardShader
            [crate::src::cgame::cg_main::cg.rewardStack as usize] = shader;
        crate::src::cgame::cg_main::cg.rewardCount
            [crate::src::cgame::cg_main::cg.rewardStack as usize] = rewardCount
    };
}
/*
==================
CG_CheckLocalSounds
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_CheckLocalSounds(
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
    mut ops: *mut crate::src::qcommon::q_shared::playerState_t,
) {
    let mut highScore: libc::c_int = 0;
    let mut reward: libc::c_int = 0;
    let mut sfx: crate::src::qcommon::q_shared::sfxHandle_t = 0;
    // don't play the sounds if the player just changed teams
    if (*ps).persistant[crate::bg_public_h::PERS_TEAM as libc::c_int as usize]
        != (*ops).persistant[crate::bg_public_h::PERS_TEAM as libc::c_int as usize]
    {
        return;
    }
    // hit changes
    if (*ps).persistant[crate::bg_public_h::PERS_HITS as libc::c_int as usize]
        > (*ops).persistant[crate::bg_public_h::PERS_HITS as libc::c_int as usize]
    {
        crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
            crate::src::cgame::cg_main::cgs.media.hitSound,
            crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND as libc::c_int,
        );
    } else if (*ps).persistant[crate::bg_public_h::PERS_HITS as libc::c_int as usize]
        < (*ops).persistant[crate::bg_public_h::PERS_HITS as libc::c_int as usize]
    {
        crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
            crate::src::cgame::cg_main::cgs.media.hitTeamSound,
            crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND as libc::c_int,
        );
    }
    // health changes of more than -1 should make pain sounds
    if (*ps).stats[crate::bg_public_h::STAT_HEALTH as libc::c_int as usize]
        < (*ops).stats[crate::bg_public_h::STAT_HEALTH as libc::c_int as usize] - 1 as libc::c_int
    {
        if (*ps).stats[crate::bg_public_h::STAT_HEALTH as libc::c_int as usize] > 0 as libc::c_int {
            crate::src::cgame::cg_event::CG_PainEvent(
                &mut crate::src::cgame::cg_main::cg.predictedPlayerEntity as *mut _
                    as *mut crate::cg_local_h::centity_s,
                (*ps).stats[crate::bg_public_h::STAT_HEALTH as libc::c_int as usize],
            );
        }
    }
    // if we are going into the intermission, don't start any voices
    if crate::src::cgame::cg_main::cg.intermissionStarted as u64 != 0 {
        return;
    }
    // reward sounds
    reward = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    if (*ps).persistant[crate::bg_public_h::PERS_CAPTURES as libc::c_int as usize]
        != (*ops).persistant[crate::bg_public_h::PERS_CAPTURES as libc::c_int as usize]
    {
        pushReward(
            crate::src::cgame::cg_main::cgs.media.captureAwardSound,
            crate::src::cgame::cg_main::cgs.media.medalCapture,
            (*ps).persistant[crate::bg_public_h::PERS_CAPTURES as libc::c_int as usize],
        );
        reward = crate::src::qcommon::q_shared::qtrue as libc::c_int
        //Com_Printf("capture\n");
    }
    if (*ps).persistant[crate::bg_public_h::PERS_IMPRESSIVE_COUNT as libc::c_int as usize]
        != (*ops).persistant[crate::bg_public_h::PERS_IMPRESSIVE_COUNT as libc::c_int as usize]
    {
        sfx = crate::src::cgame::cg_main::cgs.media.impressiveSound;
        pushReward(
            sfx,
            crate::src::cgame::cg_main::cgs.media.medalImpressive,
            (*ps).persistant[crate::bg_public_h::PERS_IMPRESSIVE_COUNT as libc::c_int as usize],
        );
        reward = crate::src::qcommon::q_shared::qtrue as libc::c_int
        //Com_Printf("impressive\n");
    }
    if (*ps).persistant[crate::bg_public_h::PERS_EXCELLENT_COUNT as libc::c_int as usize]
        != (*ops).persistant[crate::bg_public_h::PERS_EXCELLENT_COUNT as libc::c_int as usize]
    {
        sfx = crate::src::cgame::cg_main::cgs.media.excellentSound;
        pushReward(
            sfx,
            crate::src::cgame::cg_main::cgs.media.medalExcellent,
            (*ps).persistant[crate::bg_public_h::PERS_EXCELLENT_COUNT as libc::c_int as usize],
        );
        reward = crate::src::qcommon::q_shared::qtrue as libc::c_int
        //Com_Printf("excellent\n");
    }
    if (*ps).persistant[crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT as libc::c_int as usize]
        != (*ops).persistant[crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT as libc::c_int as usize]
    {
        sfx = crate::src::cgame::cg_main::cgs.media.humiliationSound;
        pushReward(
            sfx,
            crate::src::cgame::cg_main::cgs.media.medalGauntlet,
            (*ps).persistant[crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT as libc::c_int as usize],
        );
        reward = crate::src::qcommon::q_shared::qtrue as libc::c_int
        //Com_Printf("gauntlet frag\n");
    }
    if (*ps).persistant[crate::bg_public_h::PERS_DEFEND_COUNT as libc::c_int as usize]
        != (*ops).persistant[crate::bg_public_h::PERS_DEFEND_COUNT as libc::c_int as usize]
    {
        pushReward(
            crate::src::cgame::cg_main::cgs.media.defendSound,
            crate::src::cgame::cg_main::cgs.media.medalDefend,
            (*ps).persistant[crate::bg_public_h::PERS_DEFEND_COUNT as libc::c_int as usize],
        );
        reward = crate::src::qcommon::q_shared::qtrue as libc::c_int
        //Com_Printf("defend\n");
    }
    if (*ps).persistant[crate::bg_public_h::PERS_ASSIST_COUNT as libc::c_int as usize]
        != (*ops).persistant[crate::bg_public_h::PERS_ASSIST_COUNT as libc::c_int as usize]
    {
        pushReward(
            crate::src::cgame::cg_main::cgs.media.assistSound,
            crate::src::cgame::cg_main::cgs.media.medalAssist,
            (*ps).persistant[crate::bg_public_h::PERS_ASSIST_COUNT as libc::c_int as usize],
        );
        reward = crate::src::qcommon::q_shared::qtrue as libc::c_int
        //Com_Printf("assist\n");
    }
    // if any of the player event bits changed
    if (*ps).persistant[crate::bg_public_h::PERS_PLAYEREVENTS as libc::c_int as usize]
        != (*ops).persistant[crate::bg_public_h::PERS_PLAYEREVENTS as libc::c_int as usize]
    {
        if (*ps).persistant[crate::bg_public_h::PERS_PLAYEREVENTS as libc::c_int as usize]
            & 0x1 as libc::c_int
            != (*ops).persistant[crate::bg_public_h::PERS_PLAYEREVENTS as libc::c_int as usize]
                & 0x1 as libc::c_int
        {
            crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                crate::src::cgame::cg_main::cgs.media.deniedSound,
                crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
            );
        } else if (*ps).persistant[crate::bg_public_h::PERS_PLAYEREVENTS as libc::c_int as usize]
            & 0x2 as libc::c_int
            != (*ops).persistant[crate::bg_public_h::PERS_PLAYEREVENTS as libc::c_int as usize]
                & 0x2 as libc::c_int
        {
            crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                crate::src::cgame::cg_main::cgs.media.humiliationSound,
                crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
            );
        } else if (*ps).persistant[crate::bg_public_h::PERS_PLAYEREVENTS as libc::c_int as usize]
            & 0x4 as libc::c_int
            != (*ops).persistant[crate::bg_public_h::PERS_PLAYEREVENTS as libc::c_int as usize]
                & 0x4 as libc::c_int
        {
            crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                crate::src::cgame::cg_main::cgs.media.holyShitSound,
                crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
            );
        }
        reward = crate::src::qcommon::q_shared::qtrue as libc::c_int
    }
    // check for flag pickup
    if crate::src::cgame::cg_main::cgs.gametype as libc::c_uint
        > crate::bg_public_h::GT_TEAM as libc::c_int as libc::c_uint
    {
        if (*ps).powerups[crate::bg_public_h::PW_REDFLAG as libc::c_int as usize]
            != (*ops).powerups[crate::bg_public_h::PW_REDFLAG as libc::c_int as usize]
            && (*ps).powerups[crate::bg_public_h::PW_REDFLAG as libc::c_int as usize] != 0
            || (*ps).powerups[crate::bg_public_h::PW_BLUEFLAG as libc::c_int as usize]
                != (*ops).powerups[crate::bg_public_h::PW_BLUEFLAG as libc::c_int as usize]
                && (*ps).powerups[crate::bg_public_h::PW_BLUEFLAG as libc::c_int as usize] != 0
            || (*ps).powerups[crate::bg_public_h::PW_NEUTRALFLAG as libc::c_int as usize]
                != (*ops).powerups[crate::bg_public_h::PW_NEUTRALFLAG as libc::c_int as usize]
                && (*ps).powerups[crate::bg_public_h::PW_NEUTRALFLAG as libc::c_int as usize] != 0
        {
            crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                crate::src::cgame::cg_main::cgs.media.youHaveFlagSound,
                crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
            );
        }
    }
    // lead changes
    if reward == 0 {
        //
        if crate::src::cgame::cg_main::cg.warmup == 0 {
            // never play lead changes during warmup
            if (*ps).persistant[crate::bg_public_h::PERS_RANK as libc::c_int as usize]
                != (*ops).persistant[crate::bg_public_h::PERS_RANK as libc::c_int as usize]
            {
                if (crate::src::cgame::cg_main::cgs.gametype as libc::c_uint)
                    < crate::bg_public_h::GT_TEAM as libc::c_int as libc::c_uint
                {
                    if (*ps).persistant[crate::bg_public_h::PERS_RANK as libc::c_int as usize]
                        == 0 as libc::c_int
                    {
                        crate::src::cgame::cg_view::CG_AddBufferedSound(
                            crate::src::cgame::cg_main::cgs.media.takenLeadSound,
                        );
                    } else if (*ps).persistant
                        [crate::bg_public_h::PERS_RANK as libc::c_int as usize]
                        == 0x4000 as libc::c_int
                    {
                        crate::src::cgame::cg_view::CG_AddBufferedSound(
                            crate::src::cgame::cg_main::cgs.media.tiedLeadSound,
                        );
                    } else if (*ops).persistant
                        [crate::bg_public_h::PERS_RANK as libc::c_int as usize]
                        & !(0x4000 as libc::c_int)
                        == 0 as libc::c_int
                    {
                        crate::src::cgame::cg_view::CG_AddBufferedSound(
                            crate::src::cgame::cg_main::cgs.media.lostLeadSound,
                        );
                    }
                }
            }
        }
    }
    // timelimit warnings
    if crate::src::cgame::cg_main::cgs.timelimit > 0 as libc::c_int {
        let mut msec: libc::c_int = 0;
        msec = crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cgs.levelStartTime;
        if crate::src::cgame::cg_main::cg.timelimitWarnings & 4 as libc::c_int == 0
            && msec
                > (crate::src::cgame::cg_main::cgs.timelimit * 60 as libc::c_int + 2 as libc::c_int)
                    * 1000 as libc::c_int
        {
            crate::src::cgame::cg_main::cg.timelimitWarnings |=
                1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int;
            crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                crate::src::cgame::cg_main::cgs.media.suddenDeathSound,
                crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
            );
        } else if crate::src::cgame::cg_main::cg.timelimitWarnings & 2 as libc::c_int == 0
            && msec
                > (crate::src::cgame::cg_main::cgs.timelimit - 1 as libc::c_int)
                    * 60 as libc::c_int
                    * 1000 as libc::c_int
        {
            crate::src::cgame::cg_main::cg.timelimitWarnings |= 1 as libc::c_int | 2 as libc::c_int;
            crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                crate::src::cgame::cg_main::cgs.media.oneMinuteSound,
                crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
            );
        } else if crate::src::cgame::cg_main::cgs.timelimit > 5 as libc::c_int
            && crate::src::cgame::cg_main::cg.timelimitWarnings & 1 as libc::c_int == 0
            && msec
                > (crate::src::cgame::cg_main::cgs.timelimit - 5 as libc::c_int)
                    * 60 as libc::c_int
                    * 1000 as libc::c_int
        {
            crate::src::cgame::cg_main::cg.timelimitWarnings |= 1 as libc::c_int;
            crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                crate::src::cgame::cg_main::cgs.media.fiveMinuteSound,
                crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
            );
        }
    }
    // fraglimit warnings
    if crate::src::cgame::cg_main::cgs.fraglimit > 0 as libc::c_int
        && (crate::src::cgame::cg_main::cgs.gametype as libc::c_uint)
            < crate::bg_public_h::GT_CTF as libc::c_int as libc::c_uint
    {
        highScore = crate::src::cgame::cg_main::cgs.scores1;
        if crate::src::cgame::cg_main::cgs.gametype as libc::c_uint
            == crate::bg_public_h::GT_TEAM as libc::c_int as libc::c_uint
            && crate::src::cgame::cg_main::cgs.scores2 > highScore
        {
            highScore = crate::src::cgame::cg_main::cgs.scores2
        }
        if crate::src::cgame::cg_main::cg.fraglimitWarnings & 4 as libc::c_int == 0
            && highScore == crate::src::cgame::cg_main::cgs.fraglimit - 1 as libc::c_int
        {
            crate::src::cgame::cg_main::cg.fraglimitWarnings |=
                1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int;
            crate::src::cgame::cg_view::CG_AddBufferedSound(
                crate::src::cgame::cg_main::cgs.media.oneFragSound,
            );
        } else if crate::src::cgame::cg_main::cgs.fraglimit > 2 as libc::c_int
            && crate::src::cgame::cg_main::cg.fraglimitWarnings & 2 as libc::c_int == 0
            && highScore == crate::src::cgame::cg_main::cgs.fraglimit - 2 as libc::c_int
        {
            crate::src::cgame::cg_main::cg.fraglimitWarnings |= 1 as libc::c_int | 2 as libc::c_int;
            crate::src::cgame::cg_view::CG_AddBufferedSound(
                crate::src::cgame::cg_main::cgs.media.twoFragSound,
            );
        } else if crate::src::cgame::cg_main::cgs.fraglimit > 3 as libc::c_int
            && crate::src::cgame::cg_main::cg.fraglimitWarnings & 1 as libc::c_int == 0
            && highScore == crate::src::cgame::cg_main::cgs.fraglimit - 3 as libc::c_int
        {
            crate::src::cgame::cg_main::cg.fraglimitWarnings |= 1 as libc::c_int;
            crate::src::cgame::cg_view::CG_AddBufferedSound(
                crate::src::cgame::cg_main::cgs.media.threeFragSound,
            );
        }
    };
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
// The entire cgame module is unloaded and reloaded on each level change,
// so there is NO persistant data between levels on the client side.
// If you absolutely need something stored, it can either be kept
// by the server in the server stored userinfos, or stashed in a cvar.
// time for fragments to sink into ground before going away
// amount to scale up the icons when activating
// num frame for '-' stats digit
// very large characters
//=================================================
// player entities need to track more information
// than any other type of entity.
// note that not every player entity is a client entity,
// because corpses after respawn are outside the normal
// client numbering range
// when changing animation, set animationTime to frameTime + lerping time
// The current lerp will finish out, then it will lerp to the new animation
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// flip from 0 to 1
// machinegun spinning
//=================================================
// centity_t have a direct corespondence with gentity_t in the game, but
// only the entityState_t is directly communicated to the cgame
// from cg.frame
// from cg.nextFrame, if available
// true if next is valid to interpolate to
// true if cg.frame holds this entity
// move to playerEntity?
// so missile trails can handle dropped initial packets
// last time this entity was found in a snapshot
// decay the error from this time
// false if origin / angles is an interpolation
// exact interpolated position of entity on this frame
//======================================================================
// local entities are created as a result of events or predicted actions,
// and live independently from all server transmitted entities
// fade alpha instead of rgb
// do not scale size over time
// tumble over time, used for ejecting shells
// sound 1 for kamikaze
// sound 2 for kamikaze
// fragment local entities can leave marks on walls
// fragment local entities can make sounds on impacts
// 1.0 / (endTime - startTime)
// 0.0 = no bounce, 1.0 = perfect
// mark to leave on fragment impact
//======================================================================
// each client has an associated clientInfo_t
// that contains media references necessary to present the
// client model and other color coded effects
// this is regenerated each time a client's configstring changes,
// usually as a result of a userinfo (name, model, etc) change
// 0 = not bot, 1-5 = bot
// updated by score servercmds
// location index for team mode
// you only get this info about your teammates
// in tourney mode
// task in teamplay (offence/defence)
// true when this is a team leader
// so can display quad/flag status
// when clientinfo is changed, the loading of models/skins/sounds
// can be deferred until you are dead, to prevent hitches in
// gameplay
// true if using the new mission pack animations
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// move head in icon views
// from model
// each WP_* weapon enum has an associated weaponInfo_t
// that contains media references necessary to present the
// weapon and its effects
// the hands don't actually draw, they just position the weapon
// so it will rotate centered instead of by tag
// fast firing weapons randomly choose
// each IT_* item has an associated itemInfo_t
// that constains media references necessary to present the
// item and its effects
//======================================================================
// all cg.stepTime, cg.duckTime, cg.landTime, etc are set to cg.time when the action
// occurs, and they will have visible effects for #define STEP_TIME or whatever msec after
// incremented each frame
// taking a level menu screenshot
// don't defer players at initial startup
// don't play voice rewards, because game will end shortly
// there are only one or two snapshot_t that are relevant at a time
// the number of snapshots the client system has received
// the time from latestSnapshotNum, so we don't need to read the snapshot yet
// cg.snap->serverTime <= cg.time
// cg.nextSnap->serverTime > cg.time, or NULL
// (float)( cg.time - cg.frame->serverTime ) / (cg.nextFrame->serverTime - cg.frame->serverTime)
// cg.time - cg.oldTime
// this is the time value that the client
// is rendering at.
// time at last frame, used for missile trails and prediction checking
// either cg.snap->time or cg.nextSnap->time
// 5 min, 1 min, overtime
// set on a map restart to set back the weapon
// during deaths, chasecams, etc
// prediction state
// true if prediction has hit a trigger_teleport
// clear until the first call to CG_PredictPlayerState
// for stair up smoothing
// for duck viewheight smoothing
// for landing hard
// input state sent to server
// auto rotating items
// view rendering
// will be converted to refdef.viewaxis
// zoom key
// information screen text during loading
// scoreboard
// list of names
// length of list
// width in device units
// next time to offset
// current paint x
// current paint x
// current offset from start
// current offset from start
// centerprinting
// low ammo warning state
// 1 = low, 2 = empty
// crosshair client ID
// powerup active flashing
// attacking player
// reward medals
// sound buffer mainly for announcer sounds
// warmup countdown
//==========================
// the pulse around the crosshair is timed separately
// blend blobs
// status bar head
// view movement
// temp working variables for player view
//qboolean cameraMode;		// if rendering from a loaded camera
// development tool
// all of the model, shader, and sound references that are
// loaded at gamestate time are stored in cgMedia_t
// Other media that can be tied to clients, weapons, or items are
// stored in the clientInfo_t, itemInfo_t, weaponInfo_t, and powerupInfo_t
// gib explosions
// wall mark shaders
// powerup shaders
// weapon effect models
// weapon effect shaders
// special effects models
// scoreboard headers
// medals shown during gameplay
// sounds
//sfxHandle_t	sfx_railg;
// teamplay sounds
// tournament sounds
// The client game static (cgs) structure hold everything
// loaded or calculated from the gamestate.  It will NOT
// be cleared when a tournement restart is done, allowing
// all clients to begin playing instantly
// gamestate from server
// rendering configuration
// derived from glconfig
// reliable command stream counter
// the number of snapshots cgame has requested
// detected on startup by checking sv_running
// parsed from serverinfo
// beep whenever changed
// beep whenever changed
// from configstrings
// flag status from configstrings
//
// locally derived information from gamestate
//
// teamchat width is *3 because of embedded color codes
// orders
// media
//==============================================================================
//extern	vmCvar_t		cg_pmove_fixed;
//
// cg_main.c
//
//
// cg_view.c
//
//
// cg_drawtools.c
//
//
// cg_draw.c, cg_newDraw.c
//
//
// cg_player.c
//
//
// cg_predict.c
//
//
// cg_events.c
//
//
// cg_ents.c
//
//
// cg_weapons.c
//
// should this be in pmove?
//
// cg_marks.c
//
//
// cg_localents.c
//
//
// cg_effects.c
//
//
// cg_snapshot.c
//
//
// cg_info.c
//
//
// cg_scoreboard.c
//
//
// cg_consolecmds.c
//
//
// cg_servercmds.c
//
//
// cg_playerstate.c
//
/*
===============
CG_TransitionPlayerState

===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_TransitionPlayerState(
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
    mut ops: *mut crate::src::qcommon::q_shared::playerState_t,
) {
    // check for changing follow mode
    if (*ps).clientNum != (*ops).clientNum {
        crate::src::cgame::cg_main::cg.thisFrameTeleport = crate::src::qcommon::q_shared::qtrue;
        // make sure we don't get any unwanted transition effects
        *ops = *ps
    }
    // damage events (player is getting wounded)
    if (*ps).damageEvent != (*ops).damageEvent && (*ps).damageCount != 0 {
        CG_DamageFeedback((*ps).damageYaw, (*ps).damagePitch, (*ps).damageCount);
    }
    // respawning
    if (*ps).persistant[crate::bg_public_h::PERS_SPAWN_COUNT as libc::c_int as usize]
        != (*ops).persistant[crate::bg_public_h::PERS_SPAWN_COUNT as libc::c_int as usize]
    {
        CG_Respawn();
    }
    if crate::src::cgame::cg_main::cg.mapRestart as u64 != 0 {
        CG_Respawn();
        crate::src::cgame::cg_main::cg.mapRestart = crate::src::qcommon::q_shared::qfalse
    }
    if (*crate::src::cgame::cg_main::cg.snap).ps.pm_type
        != crate::bg_public_h::PM_INTERMISSION as libc::c_int
        && (*ps).persistant[crate::bg_public_h::PERS_TEAM as libc::c_int as usize]
            != crate::bg_public_h::TEAM_SPECTATOR as libc::c_int
    {
        CG_CheckLocalSounds(ps, ops);
    }
    // check for going low on ammo
    CG_CheckAmmo();
    // run events
    CG_CheckPlayerstateEvents(ps, ops);
    // smooth the ducking viewheight change
    if (*ps).viewheight != (*ops).viewheight {
        crate::src::cgame::cg_main::cg.duckChange =
            ((*ps).viewheight - (*ops).viewheight) as libc::c_float;
        crate::src::cgame::cg_main::cg.duckTime = crate::src::cgame::cg_main::cg.time
    };
}

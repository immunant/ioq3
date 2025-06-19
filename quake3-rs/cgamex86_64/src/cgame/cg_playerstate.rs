use ::libc;

pub mod q_shared_h {

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
    let mut i: i32 = 0;
    let mut total: i32 = 0;
    let mut previous: i32 = 0;
    let mut weapons: i32 = 0;
    // see about how many seconds of ammo we have remaining
    weapons = (*cg.snap).ps.stats
        [STAT_WEAPONS as i32 as usize];
    total = 0 as i32;
    i = WP_MACHINEGUN as i32;
    while i < WP_NUM_WEAPONS as i32 {
        if !(weapons & (1 as i32) << i == 0) {
            if !((*cg.snap).ps.ammo[i as usize] < 0 as i32) {
                match i {
                    5 | 4 | 7 | 3 => {
                        total +=
                            (*cg.snap).ps.ammo[i as usize] * 1000 as i32
                    }
                    _ => {
                        total +=
                            (*cg.snap).ps.ammo[i as usize] * 200 as i32
                    }
                }
                if total >= 5000 as i32 {
                    cg.lowAmmoWarning = 0 as i32;
                    return;
                }
            }
        }
        i += 1
    }
    previous = cg.lowAmmoWarning;
    if total == 0 as i32 {
        cg.lowAmmoWarning = 2 as i32
    } else {
        cg.lowAmmoWarning = 1 as i32
    }
    // play a sound on transitions
    if cg.lowAmmoWarning != previous {
        trap_S_StartLocalSound(
            cgs.media.noAmmoSound,
            CHAN_LOCAL_SOUND as i32,
        );
    };
}
/*
==============
CG_DamageFeedback
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DamageFeedback(mut yawByte: i32, mut pitchByte: i32, mut damage: i32) {
    let mut left: f32 = 0.;
    let mut front: f32 = 0.;
    let mut up: f32 = 0.;
    let mut kick: f32 = 0.;
    let mut health: i32 = 0;
    let mut scale: f32 = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut yaw: f32 = 0.;
    let mut pitch: f32 = 0.;
    // show the attacking player's head and name in corner
    cg.attackerTime = cg.time;
    // the lower on health you are, the greater the view kick will be
    health = (*cg.snap).ps.stats
        [STAT_HEALTH as i32 as usize];
    if health < 40 as i32 {
        scale = 1 as i32 as f32
    } else {
        scale = (40.0f64 / health as f64) as f32
    }
    kick = damage as f32 * scale;
    if kick < 5 as i32 as f32 {
        kick = 5 as i32 as f32
    }
    if kick > 10 as i32 as f32 {
        kick = 10 as i32 as f32
    }
    // if yaw and pitch are both 255, make the damage always centered (falling, etc)
    if yawByte == 255 as i32 && pitchByte == 255 as i32 {
        cg.damageX = 0 as i32 as f32;
        cg.damageY = 0 as i32 as f32;
        cg.v_dmg_roll = 0 as i32 as f32;
        cg.v_dmg_pitch = -kick
    } else {
        // positional
        pitch = (pitchByte as f64 / 255.0f64 * 360 as i32 as f64) as f32;
        yaw = (yawByte as f64 / 255.0f64 * 360 as i32 as f64) as f32;
        angles[0 as i32 as usize] = pitch;
        angles[1 as i32 as usize] = yaw;
        angles[2 as i32 as usize] = 0 as i32 as vec_t;
        AngleVectors(
            angles.as_mut_ptr() as *const vec_t,
            dir.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
        );
        dir[0 as i32 as usize] =
            vec3_origin[0 as i32 as usize] - dir[0 as i32 as usize];
        dir[1 as i32 as usize] =
            vec3_origin[1 as i32 as usize] - dir[1 as i32 as usize];
        dir[2 as i32 as usize] =
            vec3_origin[2 as i32 as usize] - dir[2 as i32 as usize];
        front = dir[0 as i32 as usize]
            * cg.refdef.viewaxis[0 as i32 as usize][0 as i32 as usize]
            + dir[1 as i32 as usize]
                * cg.refdef.viewaxis[0 as i32 as usize]
                    [1 as i32 as usize]
            + dir[2 as i32 as usize]
                * cg.refdef.viewaxis[0 as i32 as usize]
                    [2 as i32 as usize];
        left = dir[0 as i32 as usize]
            * cg.refdef.viewaxis[1 as i32 as usize][0 as i32 as usize]
            + dir[1 as i32 as usize]
                * cg.refdef.viewaxis[1 as i32 as usize]
                    [1 as i32 as usize]
            + dir[2 as i32 as usize]
                * cg.refdef.viewaxis[1 as i32 as usize]
                    [2 as i32 as usize];
        up = dir[0 as i32 as usize]
            * cg.refdef.viewaxis[2 as i32 as usize][0 as i32 as usize]
            + dir[1 as i32 as usize]
                * cg.refdef.viewaxis[2 as i32 as usize]
                    [1 as i32 as usize]
            + dir[2 as i32 as usize]
                * cg.refdef.viewaxis[2 as i32 as usize]
                    [2 as i32 as usize];
        dir[0 as i32 as usize] = front;
        dir[1 as i32 as usize] = left;
        dir[2 as i32 as usize] = 0 as i32 as vec_t;
        dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
        if (dist as f64) < 0.1f64 {
            dist = 0.1f32
        }
        cg.v_dmg_roll = kick * left;
        cg.v_dmg_pitch = -kick * front;
        if front as f64 <= 0.1f64 {
            front = 0.1f32
        }
        cg.damageX = -left / front;
        cg.damageY = up / dist
    }
    // clamp the position
    if cg.damageX as f64 > 1.0f64 {
        cg.damageX = 1.0f64 as f32
    }
    if (cg.damageX as f64) < -1.0f64 {
        cg.damageX = -1.0f64 as f32
    }
    if cg.damageY as f64 > 1.0f64 {
        cg.damageY = 1.0f64 as f32
    }
    if (cg.damageY as f64) < -1.0f64 {
        cg.damageY = -1.0f64 as f32
    }
    // don't let the screen flashes vary as much
    if kick > 10 as i32 as f32 {
        kick = 10 as i32 as f32
    }
    cg.damageValue = kick;
    cg.v_dmg_time =
        (cg.time + 500 as i32) as f32;
    cg.damageTime =
        (*cg.snap).serverTime as f32;
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
    cg.thisFrameTeleport = qtrue;
    // display weapons available
    cg.weaponSelectTime = cg.time;
    // select the weapon the server says we are using
    cg.weaponSelect = (*cg.snap).ps.weapon;
}
/*
==============
CG_CheckPlayerstateEvents
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_CheckPlayerstateEvents(
    mut ps: *mut playerState_t,
    mut ops: *mut playerState_t,
) {
    let mut i: i32 = 0; // cg_entities[ ps->clientNum ];
    let mut event: i32 = 0;
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    if (*ps).externalEvent != 0 && (*ps).externalEvent != (*ops).externalEvent {
        cent = &mut *cg_entities
            .as_mut_ptr()
            .offset((*ps).clientNum as isize) as *mut centity_t;
        (*cent).currentState.event = (*ps).externalEvent;
        (*cent).currentState.eventParm = (*ps).externalEventParm;
        CG_EntityEvent(
            cent as *mut centity_s,
            (*cent).lerpOrigin.as_mut_ptr(),
        );
    }
    cent = &mut cg.predictedPlayerEntity;
    // go through the predictable events buffer
    i = (*ps).eventSequence - 2 as i32;
    while i < (*ps).eventSequence {
        // if we have a new predictable event
        if i >= (*ops).eventSequence
            || i > (*ops).eventSequence - 2 as i32
                && (*ps).events[(i & 2 as i32 - 1 as i32) as usize]
                    != (*ops).events[(i & 2 as i32 - 1 as i32) as usize]
        {
            event = (*ps).events[(i & 2 as i32 - 1 as i32) as usize];
            (*cent).currentState.event = event;
            (*cent).currentState.eventParm = (*ps).eventParms[(i & 2 as i32 - 1 as i32) as usize];
            CG_EntityEvent(
                cent as *mut centity_s,
                (*cent).lerpOrigin.as_mut_ptr(),
            );
            cg.predictableEvents[(i & 16 as i32 - 1 as i32) as usize] =
                event;
            cg.eventSequence += 1
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
    mut ps: *mut playerState_t,
) {
    let mut i: i32 = 0;
    let mut event: i32 = 0;
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    cent = &mut cg.predictedPlayerEntity;
    i = (*ps).eventSequence - 2 as i32;
    while i < (*ps).eventSequence {
        //
        if !(i >= cg.eventSequence) {
            // if this event is not further back in than the maximum predictable events we remember
            if i > cg.eventSequence - 16 as i32 {
                // if the new playerstate event is different from a previously predicted one
                if (*ps).events[(i & 2 as i32 - 1 as i32) as usize]
                    != cg.predictableEvents
                        [(i & 16 as i32 - 1 as i32) as usize]
                {
                    event = (*ps).events[(i & 2 as i32 - 1 as i32) as usize];
                    (*cent).currentState.event = event;
                    (*cent).currentState.eventParm =
                        (*ps).eventParms[(i & 2 as i32 - 1 as i32) as usize];
                    CG_EntityEvent(
                        cent as *mut centity_s,
                        (*cent).lerpOrigin.as_mut_ptr(),
                    );
                    cg.predictableEvents
                        [(i & 16 as i32 - 1 as i32) as usize] = event;
                    if cg_showmiss.integer != 0 {
                        CG_Printf(
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
    mut sfx: sfxHandle_t,
    mut shader: qhandle_t,
    mut rewardCount: i32,
) {
    if cg.rewardStack < 10 as i32 - 1 as i32 {
        cg.rewardStack += 1;
        cg.rewardSound
            [cg.rewardStack as usize] = sfx;
        cg.rewardShader
            [cg.rewardStack as usize] = shader;
        cg.rewardCount
            [cg.rewardStack as usize] = rewardCount
    };
}
/*
==================
CG_CheckLocalSounds
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_CheckLocalSounds(
    mut ps: *mut playerState_t,
    mut ops: *mut playerState_t,
) {
    let mut highScore: i32 = 0;
    let mut reward: i32 = 0;
    let mut sfx: sfxHandle_t = 0;
    // don't play the sounds if the player just changed teams
    if (*ps).persistant[PERS_TEAM as i32 as usize]
        != (*ops).persistant[PERS_TEAM as i32 as usize]
    {
        return;
    }
    // hit changes
    if (*ps).persistant[PERS_HITS as i32 as usize]
        > (*ops).persistant[PERS_HITS as i32 as usize]
    {
        trap_S_StartLocalSound(
            cgs.media.hitSound,
            CHAN_LOCAL_SOUND as i32,
        );
    } else if (*ps).persistant[PERS_HITS as i32 as usize]
        < (*ops).persistant[PERS_HITS as i32 as usize]
    {
        trap_S_StartLocalSound(
            cgs.media.hitTeamSound,
            CHAN_LOCAL_SOUND as i32,
        );
    }
    // health changes of more than -1 should make pain sounds
    if (*ps).stats[STAT_HEALTH as i32 as usize]
        < (*ops).stats[STAT_HEALTH as i32 as usize] - 1 as i32
    {
        if (*ps).stats[STAT_HEALTH as i32 as usize] > 0 as i32 {
            CG_PainEvent(
                &mut cg.predictedPlayerEntity as *mut _
                    as *mut centity_s,
                (*ps).stats[STAT_HEALTH as i32 as usize],
            );
        }
    }
    // if we are going into the intermission, don't start any voices
    if cg.intermissionStarted as u64 != 0 {
        return;
    }
    // reward sounds
    reward = qfalse as i32;
    if (*ps).persistant[PERS_CAPTURES as i32 as usize]
        != (*ops).persistant[PERS_CAPTURES as i32 as usize]
    {
        pushReward(
            cgs.media.captureAwardSound,
            cgs.media.medalCapture,
            (*ps).persistant[PERS_CAPTURES as i32 as usize],
        );
        reward = qtrue as i32
        //Com_Printf("capture\n");
    }
    if (*ps).persistant[PERS_IMPRESSIVE_COUNT as i32 as usize]
        != (*ops).persistant[PERS_IMPRESSIVE_COUNT as i32 as usize]
    {
        sfx = cgs.media.impressiveSound;
        pushReward(
            sfx,
            cgs.media.medalImpressive,
            (*ps).persistant[PERS_IMPRESSIVE_COUNT as i32 as usize],
        );
        reward = qtrue as i32
        //Com_Printf("impressive\n");
    }
    if (*ps).persistant[PERS_EXCELLENT_COUNT as i32 as usize]
        != (*ops).persistant[PERS_EXCELLENT_COUNT as i32 as usize]
    {
        sfx = cgs.media.excellentSound;
        pushReward(
            sfx,
            cgs.media.medalExcellent,
            (*ps).persistant[PERS_EXCELLENT_COUNT as i32 as usize],
        );
        reward = qtrue as i32
        //Com_Printf("excellent\n");
    }
    if (*ps).persistant[PERS_GAUNTLET_FRAG_COUNT as i32 as usize]
        != (*ops).persistant[PERS_GAUNTLET_FRAG_COUNT as i32 as usize]
    {
        sfx = cgs.media.humiliationSound;
        pushReward(
            sfx,
            cgs.media.medalGauntlet,
            (*ps).persistant[PERS_GAUNTLET_FRAG_COUNT as i32 as usize],
        );
        reward = qtrue as i32
        //Com_Printf("gauntlet frag\n");
    }
    if (*ps).persistant[PERS_DEFEND_COUNT as i32 as usize]
        != (*ops).persistant[PERS_DEFEND_COUNT as i32 as usize]
    {
        pushReward(
            cgs.media.defendSound,
            cgs.media.medalDefend,
            (*ps).persistant[PERS_DEFEND_COUNT as i32 as usize],
        );
        reward = qtrue as i32
        //Com_Printf("defend\n");
    }
    if (*ps).persistant[PERS_ASSIST_COUNT as i32 as usize]
        != (*ops).persistant[PERS_ASSIST_COUNT as i32 as usize]
    {
        pushReward(
            cgs.media.assistSound,
            cgs.media.medalAssist,
            (*ps).persistant[PERS_ASSIST_COUNT as i32 as usize],
        );
        reward = qtrue as i32
        //Com_Printf("assist\n");
    }
    // if any of the player event bits changed
    if (*ps).persistant[PERS_PLAYEREVENTS as i32 as usize]
        != (*ops).persistant[PERS_PLAYEREVENTS as i32 as usize]
    {
        if (*ps).persistant[PERS_PLAYEREVENTS as i32 as usize] & 0x1 as i32
            != (*ops).persistant[PERS_PLAYEREVENTS as i32 as usize] & 0x1 as i32
        {
            trap_S_StartLocalSound(
                cgs.media.deniedSound,
                CHAN_ANNOUNCER as i32,
            );
        } else if (*ps).persistant[PERS_PLAYEREVENTS as i32 as usize]
            & 0x2 as i32
            != (*ops).persistant[PERS_PLAYEREVENTS as i32 as usize] & 0x2 as i32
        {
            trap_S_StartLocalSound(
                cgs.media.humiliationSound,
                CHAN_ANNOUNCER as i32,
            );
        } else if (*ps).persistant[PERS_PLAYEREVENTS as i32 as usize]
            & 0x4 as i32
            != (*ops).persistant[PERS_PLAYEREVENTS as i32 as usize] & 0x4 as i32
        {
            trap_S_StartLocalSound(
                cgs.media.holyShitSound,
                CHAN_ANNOUNCER as i32,
            );
        }
        reward = qtrue as i32
    }
    // check for flag pickup
    if cgs.gametype as u32 > GT_TEAM as i32 as u32 {
        if (*ps).powerups[PW_REDFLAG as i32 as usize]
            != (*ops).powerups[PW_REDFLAG as i32 as usize]
            && (*ps).powerups[PW_REDFLAG as i32 as usize] != 0
            || (*ps).powerups[PW_BLUEFLAG as i32 as usize]
                != (*ops).powerups[PW_BLUEFLAG as i32 as usize]
                && (*ps).powerups[PW_BLUEFLAG as i32 as usize] != 0
            || (*ps).powerups[PW_NEUTRALFLAG as i32 as usize]
                != (*ops).powerups[PW_NEUTRALFLAG as i32 as usize]
                && (*ps).powerups[PW_NEUTRALFLAG as i32 as usize] != 0
        {
            trap_S_StartLocalSound(
                cgs.media.youHaveFlagSound,
                CHAN_ANNOUNCER as i32,
            );
        }
    }
    // lead changes
    if reward == 0 {
        //
        if cg.warmup == 0 {
            // never play lead changes during warmup
            if (*ps).persistant[PERS_RANK as i32 as usize]
                != (*ops).persistant[PERS_RANK as i32 as usize]
            {
                if (cgs.gametype as u32)
                    < GT_TEAM as i32 as u32
                {
                    if (*ps).persistant[PERS_RANK as i32 as usize] == 0 as i32 {
                        CG_AddBufferedSound(
                            cgs.media.takenLeadSound,
                        );
                    } else if (*ps).persistant[PERS_RANK as i32 as usize]
                        == 0x4000 as i32
                    {
                        CG_AddBufferedSound(
                            cgs.media.tiedLeadSound,
                        );
                    } else if (*ops).persistant[PERS_RANK as i32 as usize]
                        & !(0x4000 as i32)
                        == 0 as i32
                    {
                        CG_AddBufferedSound(
                            cgs.media.lostLeadSound,
                        );
                    }
                }
            }
        }
    }
    // timelimit warnings
    if cgs.timelimit > 0 as i32 {
        let mut msec: i32 = 0;
        msec = cg.time - cgs.levelStartTime;
        if cg.timelimitWarnings & 4 as i32 == 0
            && msec
                > (cgs.timelimit * 60 as i32 + 2 as i32) * 1000 as i32
        {
            cg.timelimitWarnings |= 1 as i32 | 2 as i32 | 4 as i32;
            trap_S_StartLocalSound(
                cgs.media.suddenDeathSound,
                CHAN_ANNOUNCER as i32,
            );
        } else if cg.timelimitWarnings & 2 as i32 == 0
            && msec
                > (cgs.timelimit - 1 as i32) * 60 as i32 * 1000 as i32
        {
            cg.timelimitWarnings |= 1 as i32 | 2 as i32;
            trap_S_StartLocalSound(
                cgs.media.oneMinuteSound,
                CHAN_ANNOUNCER as i32,
            );
        } else if cgs.timelimit > 5 as i32
            && cg.timelimitWarnings & 1 as i32 == 0
            && msec
                > (cgs.timelimit - 5 as i32) * 60 as i32 * 1000 as i32
        {
            cg.timelimitWarnings |= 1 as i32;
            trap_S_StartLocalSound(
                cgs.media.fiveMinuteSound,
                CHAN_ANNOUNCER as i32,
            );
        }
    }
    // fraglimit warnings
    if cgs.fraglimit > 0 as i32
        && (cgs.gametype as u32)
            < GT_CTF as i32 as u32
    {
        highScore = cgs.scores1;
        if cgs.gametype as u32
            == GT_TEAM as i32 as u32
            && cgs.scores2 > highScore
        {
            highScore = cgs.scores2
        }
        if cg.fraglimitWarnings & 4 as i32 == 0
            && highScore == cgs.fraglimit - 1 as i32
        {
            cg.fraglimitWarnings |= 1 as i32 | 2 as i32 | 4 as i32;
            CG_AddBufferedSound(
                cgs.media.oneFragSound,
            );
        } else if cgs.fraglimit > 2 as i32
            && cg.fraglimitWarnings & 2 as i32 == 0
            && highScore == cgs.fraglimit - 2 as i32
        {
            cg.fraglimitWarnings |= 1 as i32 | 2 as i32;
            CG_AddBufferedSound(
                cgs.media.twoFragSound,
            );
        } else if cgs.fraglimit > 3 as i32
            && cg.fraglimitWarnings & 1 as i32 == 0
            && highScore == cgs.fraglimit - 3 as i32
        {
            cg.fraglimitWarnings |= 1 as i32;
            CG_AddBufferedSound(
                cgs.media.threeFragSound,
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
    mut ps: *mut playerState_t,
    mut ops: *mut playerState_t,
) {
    // check for changing follow mode
    if (*ps).clientNum != (*ops).clientNum {
        cg.thisFrameTeleport = qtrue;
        // make sure we don't get any unwanted transition effects
        *ops = *ps
    }
    // damage events (player is getting wounded)
    if (*ps).damageEvent != (*ops).damageEvent && (*ps).damageCount != 0 {
        CG_DamageFeedback((*ps).damageYaw, (*ps).damagePitch, (*ps).damageCount);
    }
    // respawning
    if (*ps).persistant[PERS_SPAWN_COUNT as i32 as usize]
        != (*ops).persistant[PERS_SPAWN_COUNT as i32 as usize]
    {
        CG_Respawn();
    }
    if cg.mapRestart as u64 != 0 {
        CG_Respawn();
        cg.mapRestart = qfalse
    }
    if (*cg.snap).ps.pm_type
        != PM_INTERMISSION as i32
        && (*ps).persistant[PERS_TEAM as i32 as usize]
            != TEAM_SPECTATOR as i32
    {
        CG_CheckLocalSounds(ps, ops);
    }
    // check for going low on ammo
    CG_CheckAmmo();
    // run events
    CG_CheckPlayerstateEvents(ps, ops);
    // smooth the ducking viewheight change
    if (*ps).viewheight != (*ops).viewheight {
        cg.duckChange = ((*ps).viewheight - (*ops).viewheight) as f32;
        cg.duckTime = cg.time
    };
}

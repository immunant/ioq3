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

pub use crate::bg_local_h::pml_t;
pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::pmove_t;
pub use crate::bg_public_h::BOTH_DEAD1;
pub use crate::bg_public_h::BOTH_DEAD2;
pub use crate::bg_public_h::BOTH_DEAD3;
pub use crate::bg_public_h::BOTH_DEATH1;
pub use crate::bg_public_h::BOTH_DEATH2;
pub use crate::bg_public_h::BOTH_DEATH3;
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
pub use crate::bg_public_h::HI_INVULNERABILITY;
pub use crate::bg_public_h::HI_KAMIKAZE;
pub use crate::bg_public_h::HI_MEDKIT;
pub use crate::bg_public_h::HI_NONE;
pub use crate::bg_public_h::HI_NUM_HOLDABLE;
pub use crate::bg_public_h::HI_PORTAL;
pub use crate::bg_public_h::HI_TELEPORTER;
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
pub use crate::src::cgame::cg_main::Com_Printf;
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::game::bg_misc::BG_AddPredictableEventToPlayerstate;
pub use crate::src::game::bg_pmove::q_shared_h::VectorLength;
pub use crate::src::game::bg_slidemove::PM_SlideMove;
pub use crate::src::game::bg_slidemove::PM_StepSlideMove;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;

extern "C" {
    /*
    ================
    PmoveSingle

    ================
    */
    #[no_mangle]
    pub fn trap_SnapVector(v: *mut f32);
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
// bg_pmove.c -- both games player movement code
// takes a playerstate and a usercmd as input and returns a modifed playerstate
#[no_mangle]

pub static mut pm: *mut pmove_t = std::ptr::null_mut();
#[no_mangle]

pub static mut pml: pml_t = pml_t {
    forward: [0.; 3],
    right: [0.; 3],
    up: [0.; 3],
    frametime: 0.,
    msec: 0,
    walking: qfalse,
    groundPlane: qfalse,
    groundTrace: trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surfaceFlags: 0,
        contents: 0,
        entityNum: 0,
    },
    impactSpeed: 0.,
    previous_origin: [0.; 3],
    previous_velocity: [0.; 3],
    previous_waterlevel: 0,
};
// movement parameters
#[no_mangle]

pub static mut pm_stopspeed: f32 = 100.0f32;
#[no_mangle]

pub static mut pm_duckScale: f32 = 0.25f32;
#[no_mangle]

pub static mut pm_swimScale: f32 = 0.50f32;
#[no_mangle]

pub static mut pm_accelerate: f32 = 10.0f32;
#[no_mangle]

pub static mut pm_airaccelerate: f32 = 1.0f32;
#[no_mangle]

pub static mut pm_wateraccelerate: f32 = 4.0f32;
#[no_mangle]

pub static mut pm_flyaccelerate: f32 = 8.0f32;
#[no_mangle]

pub static mut pm_friction: f32 = 6.0f32;
#[no_mangle]

pub static mut pm_waterfriction: f32 = 1.0f32;
#[no_mangle]

pub static mut pm_flightfriction: f32 = 3.0f32;
#[no_mangle]

pub static mut pm_spectatorfriction: f32 = 5.0f32;
#[no_mangle]

pub static mut c_pmove: i32 = 0 as i32;
/*
===============
PM_AddEvent

===============
*/
#[no_mangle]

pub unsafe extern "C" fn PM_AddEvent(mut newEvent: i32) {
    BG_AddPredictableEventToPlayerstate(newEvent, 0 as i32, (*pm).ps as *mut playerState_s);
}
/*
===============
PM_AddTouchEnt
===============
*/
#[no_mangle]

pub unsafe extern "C" fn PM_AddTouchEnt(mut entityNum: i32) {
    let mut i: i32 = 0;
    if entityNum == ((1 as i32) << 10 as i32) - 2 as i32 {
        return;
    }
    if (*pm).numtouch == 32 as i32 {
        return;
    }
    // see if it is already added
    i = 0 as i32;
    while i < (*pm).numtouch {
        if (*pm).touchents[i as usize] == entityNum {
            return;
        }
        i += 1
    }
    // add it
    (*pm).touchents[(*pm).numtouch as usize] = entityNum;
    (*pm).numtouch += 1;
}
/*
===================
PM_StartTorsoAnim
===================
*/

unsafe extern "C" fn PM_StartTorsoAnim(mut anim: i32) {
    if (*(*pm).ps).pm_type >= PM_DEAD as i32 {
        return;
    }
    (*(*pm).ps).torsoAnim = (*(*pm).ps).torsoAnim & 128 as i32 ^ 128 as i32 | anim;
}

unsafe extern "C" fn PM_StartLegsAnim(mut anim: i32) {
    if (*(*pm).ps).pm_type >= PM_DEAD as i32 {
        return;
    }
    if (*(*pm).ps).legsTimer > 0 as i32 {
        return;
        // a high priority animation is running
    }
    (*(*pm).ps).legsAnim = (*(*pm).ps).legsAnim & 128 as i32 ^ 128 as i32 | anim;
}

unsafe extern "C" fn PM_ContinueLegsAnim(mut anim: i32) {
    if (*(*pm).ps).legsAnim & !(128 as i32) == anim {
        return;
    }
    if (*(*pm).ps).legsTimer > 0 as i32 {
        return;
        // a high priority animation is running
    }
    PM_StartLegsAnim(anim);
}

unsafe extern "C" fn PM_ContinueTorsoAnim(mut anim: i32) {
    if (*(*pm).ps).torsoAnim & !(128 as i32) == anim {
        return;
    }
    if (*(*pm).ps).torsoTimer > 0 as i32 {
        return;
        // a high priority animation is running
    }
    PM_StartTorsoAnim(anim);
}

unsafe extern "C" fn PM_ForceLegsAnim(mut anim: i32) {
    (*(*pm).ps).legsTimer = 0 as i32;
    PM_StartLegsAnim(anim);
}
// movement parameters
/*
==================
PM_ClipVelocity

Slide off of the impacting surface
==================
*/
#[no_mangle]

pub unsafe extern "C" fn PM_ClipVelocity(
    mut in_0: *mut vec_t,
    mut normal: *mut vec_t,
    mut out: *mut vec_t,
    mut overbounce: f32,
) {
    let mut backoff: f32 = 0.;
    let mut change: f32 = 0.;
    let mut i: i32 = 0;
    backoff = *in_0.offset(0 as i32 as isize) * *normal.offset(0 as i32 as isize)
        + *in_0.offset(1 as i32 as isize) * *normal.offset(1 as i32 as isize)
        + *in_0.offset(2 as i32 as isize) * *normal.offset(2 as i32 as isize);
    if backoff < 0 as i32 as f32 {
        backoff *= overbounce
    } else {
        backoff /= overbounce
    }
    i = 0 as i32;
    while i < 3 as i32 {
        change = *normal.offset(i as isize) * backoff;
        *out.offset(i as isize) = *in_0.offset(i as isize) - change;
        i += 1
    }
}
/*
==================
PM_Friction

Handles both ground friction and water friction
==================
*/

unsafe extern "C" fn PM_Friction() {
    let mut vec: vec3_t = [0.; 3];
    let mut vel: *mut f32 = 0 as *mut f32;
    let mut speed: f32 = 0.;
    let mut newspeed: f32 = 0.;
    let mut control: f32 = 0.;
    let mut drop_0: f32 = 0.;
    vel = (*(*pm).ps).velocity.as_mut_ptr();
    vec[0 as i32 as usize] = *vel.offset(0 as i32 as isize);
    vec[1 as i32 as usize] = *vel.offset(1 as i32 as isize);
    vec[2 as i32 as usize] = *vel.offset(2 as i32 as isize);
    if pml.walking as u64 != 0 {
        vec[2 as i32 as usize] = 0 as i32 as vec_t
        // ignore slope movement
    } // allow sinking underwater
    speed = VectorLength(vec.as_mut_ptr() as *const vec_t);
    if speed < 1 as i32 as f32 {
        *vel.offset(0 as i32 as isize) = 0 as i32 as f32;
        *vel.offset(1 as i32 as isize) = 0 as i32 as f32;
        // FIXME: still have z friction underwater?
        return;
    }
    drop_0 = 0 as i32 as f32;
    // apply ground friction
    if (*pm).waterlevel <= 1 as i32 {
        if pml.walking as u32 != 0 && pml.groundTrace.surfaceFlags & 0x2 as i32 == 0 {
            // if getting knocked back, no friction
            if (*(*pm).ps).pm_flags & 64 as i32 == 0 {
                control = if speed < pm_stopspeed {
                    pm_stopspeed
                } else {
                    speed
                };
                drop_0 += control * pm_friction * pml.frametime
            }
        }
    }
    // apply water friction even if just wading
    if (*pm).waterlevel != 0 {
        drop_0 += speed * pm_waterfriction * (*pm).waterlevel as f32 * pml.frametime
    }
    // apply flying friction
    if (*(*pm).ps).powerups[PW_FLIGHT as i32 as usize] != 0 {
        drop_0 += speed * pm_flightfriction * pml.frametime
    }
    if (*(*pm).ps).pm_type == PM_SPECTATOR as i32 {
        drop_0 += speed * pm_spectatorfriction * pml.frametime
    }
    // scale the velocity
    newspeed = speed - drop_0;
    if newspeed < 0 as i32 as f32 {
        newspeed = 0 as i32 as f32
    }
    newspeed /= speed;
    *vel.offset(0 as i32 as isize) = *vel.offset(0 as i32 as isize) * newspeed;
    *vel.offset(1 as i32 as isize) = *vel.offset(1 as i32 as isize) * newspeed;
    *vel.offset(2 as i32 as isize) = *vel.offset(2 as i32 as isize) * newspeed;
}
/*
==============
PM_Accelerate

Handles user intended acceleration
==============
*/

unsafe extern "C" fn PM_Accelerate(mut wishdir: *mut vec_t, mut wishspeed: f32, mut accel: f32) {
    // q2 style
    let mut i: i32 = 0;
    let mut addspeed: f32 = 0.;
    let mut accelspeed: f32 = 0.;
    let mut currentspeed: f32 = 0.;
    currentspeed = (*(*pm).ps).velocity[0 as i32 as usize] * *wishdir.offset(0 as i32 as isize)
        + (*(*pm).ps).velocity[1 as i32 as usize] * *wishdir.offset(1 as i32 as isize)
        + (*(*pm).ps).velocity[2 as i32 as usize] * *wishdir.offset(2 as i32 as isize);
    addspeed = wishspeed - currentspeed;
    if addspeed <= 0 as i32 as f32 {
        return;
    }
    accelspeed = accel * pml.frametime * wishspeed;
    if accelspeed > addspeed {
        accelspeed = addspeed
    }
    i = 0 as i32;
    while i < 3 as i32 {
        (*(*pm).ps).velocity[i as usize] += accelspeed * *wishdir.offset(i as isize);
        i += 1
    }
}
/*
============
PM_CmdScale

Returns the scale factor to apply to cmd movements
This allows the clients to use axial -127 to 127 values for all directions
without getting a sqrt(2) distortion in speed.
============
*/

unsafe extern "C" fn PM_CmdScale(mut cmd: *mut usercmd_t) -> f32 {
    let mut max: i32 = 0;
    let mut total: f32 = 0.;
    let mut scale: f32 = 0.;
    max = libc::abs((*cmd).forwardmove as i32);
    if libc::abs((*cmd).rightmove as i32) > max {
        max = libc::abs((*cmd).rightmove as i32)
    }
    if libc::abs((*cmd).upmove as i32) > max {
        max = libc::abs((*cmd).upmove as i32)
    }
    if max == 0 {
        return 0 as i32 as f32;
    }
    total = crate::stdlib::sqrt(
        ((*cmd).forwardmove as i32 * (*cmd).forwardmove as i32
            + (*cmd).rightmove as i32 * (*cmd).rightmove as i32
            + (*cmd).upmove as i32 * (*cmd).upmove as i32) as f64,
    ) as f32;
    scale = (((*(*pm).ps).speed as f32 * max as f32) as f64 / (127.0f64 * total as f64)) as f32;
    return scale;
}
/*
================
PM_SetMovementDir

Determine the rotation of the legs relative
to the facing dir
================
*/

unsafe extern "C" fn PM_SetMovementDir() {
    if (*pm).cmd.forwardmove as i32 != 0 || (*pm).cmd.rightmove as i32 != 0 {
        if (*pm).cmd.rightmove as i32 == 0 as i32 && (*pm).cmd.forwardmove as i32 > 0 as i32 {
            (*(*pm).ps).movementDir = 0 as i32
        } else if ((*pm).cmd.rightmove as i32) < 0 as i32 && (*pm).cmd.forwardmove as i32 > 0 as i32
        {
            (*(*pm).ps).movementDir = 1 as i32
        } else if ((*pm).cmd.rightmove as i32) < 0 as i32
            && (*pm).cmd.forwardmove as i32 == 0 as i32
        {
            (*(*pm).ps).movementDir = 2 as i32
        } else if ((*pm).cmd.rightmove as i32) < 0 as i32
            && ((*pm).cmd.forwardmove as i32) < 0 as i32
        {
            (*(*pm).ps).movementDir = 3 as i32
        } else if (*pm).cmd.rightmove as i32 == 0 as i32
            && ((*pm).cmd.forwardmove as i32) < 0 as i32
        {
            (*(*pm).ps).movementDir = 4 as i32
        } else if (*pm).cmd.rightmove as i32 > 0 as i32 && ((*pm).cmd.forwardmove as i32) < 0 as i32
        {
            (*(*pm).ps).movementDir = 5 as i32
        } else if (*pm).cmd.rightmove as i32 > 0 as i32 && (*pm).cmd.forwardmove as i32 == 0 as i32
        {
            (*(*pm).ps).movementDir = 6 as i32
        } else if (*pm).cmd.rightmove as i32 > 0 as i32 && (*pm).cmd.forwardmove as i32 > 0 as i32 {
            (*(*pm).ps).movementDir = 7 as i32
        }
    } else if (*(*pm).ps).movementDir == 2 as i32 {
        (*(*pm).ps).movementDir = 1 as i32
    } else if (*(*pm).ps).movementDir == 6 as i32 {
        (*(*pm).ps).movementDir = 7 as i32
    };
}
// if they aren't actively going directly sideways,
// change the animation to the diagonal so they
// don't stop too crooked
/*
=============
PM_CheckJump
=============
*/

unsafe extern "C" fn PM_CheckJump() -> qboolean {
    if (*(*pm).ps).pm_flags & 512 as i32 != 0 {
        return qfalse;
        // don't allow jump until all buttons are up
    }
    if ((*pm).cmd.upmove as i32) < 10 as i32 {
        // not holding jump
        return qfalse;
    }
    // must wait for jump to be released
    if (*(*pm).ps).pm_flags & 2 as i32 != 0 {
        // clear upmove so cmdscale doesn't lower running speed
        (*pm).cmd.upmove = 0 as i32 as i8; // jumping away
        return qfalse;
    }
    pml.groundPlane = qfalse;
    pml.walking = qfalse;
    (*(*pm).ps).pm_flags |= 2 as i32;
    (*(*pm).ps).groundEntityNum = ((1 as i32) << 10 as i32) - 1 as i32;
    (*(*pm).ps).velocity[2 as i32 as usize] = 270 as i32 as vec_t;
    PM_AddEvent(EV_JUMP as i32);
    if (*pm).cmd.forwardmove as i32 >= 0 as i32 {
        PM_ForceLegsAnim(LEGS_JUMP as i32);
        (*(*pm).ps).pm_flags &= !(8 as i32)
    } else {
        PM_ForceLegsAnim(LEGS_JUMPB as i32);
        (*(*pm).ps).pm_flags |= 8 as i32
    }
    return qtrue;
}
/*
=============
PM_CheckWaterJump
=============
*/

unsafe extern "C" fn PM_CheckWaterJump() -> qboolean {
    let mut spot: vec3_t = [0.; 3];
    let mut cont: i32 = 0;
    let mut flatforward: vec3_t = [0.; 3];
    if (*(*pm).ps).pm_time != 0 {
        return qfalse;
    }
    // check for water jump
    if (*pm).waterlevel != 2 as i32 {
        return qfalse;
    }
    flatforward[0 as i32 as usize] = pml.forward[0 as i32 as usize];
    flatforward[1 as i32 as usize] = pml.forward[1 as i32 as usize];
    flatforward[2 as i32 as usize] = 0 as i32 as vec_t;
    VectorNormalize(flatforward.as_mut_ptr());
    spot[0 as i32 as usize] =
        (*(*pm).ps).origin[0 as i32 as usize] + flatforward[0 as i32 as usize] * 30 as i32 as f32;
    spot[1 as i32 as usize] =
        (*(*pm).ps).origin[1 as i32 as usize] + flatforward[1 as i32 as usize] * 30 as i32 as f32;
    spot[2 as i32 as usize] =
        (*(*pm).ps).origin[2 as i32 as usize] + flatforward[2 as i32 as usize] * 30 as i32 as f32;
    spot[2 as i32 as usize] += 4 as i32 as f32;
    cont = (*pm).pointcontents.expect("non-null function pointer")(
        spot.as_mut_ptr() as *const vec_t,
        (*(*pm).ps).clientNum,
    );
    if cont & 1 as i32 == 0 {
        return qfalse;
    }
    spot[2 as i32 as usize] += 16 as i32 as f32;
    cont = (*pm).pointcontents.expect("non-null function pointer")(
        spot.as_mut_ptr() as *const vec_t,
        (*(*pm).ps).clientNum,
    );
    if cont & (1 as i32 | 0x10000 as i32 | 0x2000000 as i32) != 0 {
        return qfalse;
    }
    // jump out of water
    (*(*pm).ps).velocity[0 as i32 as usize] = pml.forward[0 as i32 as usize] * 200 as i32 as f32;
    (*(*pm).ps).velocity[1 as i32 as usize] = pml.forward[1 as i32 as usize] * 200 as i32 as f32;
    (*(*pm).ps).velocity[2 as i32 as usize] = pml.forward[2 as i32 as usize] * 200 as i32 as f32;
    (*(*pm).ps).velocity[2 as i32 as usize] = 350 as i32 as vec_t;
    (*(*pm).ps).pm_flags |= 256 as i32;
    (*(*pm).ps).pm_time = 2000 as i32;
    return qtrue;
}
//============================================================================
/*
===================
PM_WaterJumpMove

Flying out of the water
===================
*/

unsafe extern "C" fn PM_WaterJumpMove() {
    // waterjump has no control, but falls
    PM_StepSlideMove(qtrue);
    (*(*pm).ps).velocity[2 as i32 as usize] -= (*(*pm).ps).gravity as f32 * pml.frametime;
    if (*(*pm).ps).velocity[2 as i32 as usize] < 0 as i32 as f32 {
        // cancel as soon as we are falling down again
        (*(*pm).ps).pm_flags &= !(256 as i32 | 32 as i32 | 64 as i32);
        (*(*pm).ps).pm_time = 0 as i32
    };
}
/*
===================
PM_WaterMove

===================
*/

unsafe extern "C" fn PM_WaterMove() {
    let mut i: i32 = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut wishspeed: f32 = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    let mut scale: f32 = 0.;
    let mut vel: f32 = 0.;
    if PM_CheckWaterJump() as u64 != 0 {
        PM_WaterJumpMove();
        return;
    }
    PM_Friction();
    scale = PM_CmdScale(&mut (*pm).cmd);
    //
    // user intentions
    //
    if scale == 0. {
        wishvel[0 as i32 as usize] = 0 as i32 as vec_t;
        wishvel[1 as i32 as usize] = 0 as i32 as vec_t;
        wishvel[2 as i32 as usize] = -(60 as i32) as vec_t
    // sink towards bottom
    } else {
        i = 0 as i32;
        while i < 3 as i32 {
            wishvel[i as usize] =
                scale * pml.forward[i as usize] * (*pm).cmd.forwardmove as i32 as f32
                    + scale * pml.right[i as usize] * (*pm).cmd.rightmove as i32 as f32;
            i += 1
        }
        wishvel[2 as i32 as usize] += scale * (*pm).cmd.upmove as i32 as f32
    }
    wishdir[0 as i32 as usize] = wishvel[0 as i32 as usize];
    wishdir[1 as i32 as usize] = wishvel[1 as i32 as usize];
    wishdir[2 as i32 as usize] = wishvel[2 as i32 as usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    if wishspeed > (*(*pm).ps).speed as f32 * pm_swimScale {
        wishspeed = (*(*pm).ps).speed as f32 * pm_swimScale
    }
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_wateraccelerate);
    // make sure we can go up slopes easily under water
    if pml.groundPlane as u32 != 0
        && (*(*pm).ps).velocity[0 as i32 as usize] * pml.groundTrace.plane.normal[0 as i32 as usize]
            + (*(*pm).ps).velocity[1 as i32 as usize]
                * pml.groundTrace.plane.normal[1 as i32 as usize]
            + (*(*pm).ps).velocity[2 as i32 as usize]
                * pml.groundTrace.plane.normal[2 as i32 as usize]
            < 0 as i32 as f32
    {
        vel = VectorLength((*(*pm).ps).velocity.as_mut_ptr() as *const vec_t);
        // slide along the ground plane
        PM_ClipVelocity(
            (*(*pm).ps).velocity.as_mut_ptr(),
            pml.groundTrace.plane.normal.as_mut_ptr(),
            (*(*pm).ps).velocity.as_mut_ptr(),
            1.001f32,
        );
        VectorNormalize((*(*pm).ps).velocity.as_mut_ptr());
        (*(*pm).ps).velocity[0 as i32 as usize] = (*(*pm).ps).velocity[0 as i32 as usize] * vel;
        (*(*pm).ps).velocity[1 as i32 as usize] = (*(*pm).ps).velocity[1 as i32 as usize] * vel;
        (*(*pm).ps).velocity[2 as i32 as usize] = (*(*pm).ps).velocity[2 as i32 as usize] * vel
    }
    PM_SlideMove(qfalse);
}
/*
===================
PM_FlyMove

Only with the flight powerup
===================
*/

unsafe extern "C" fn PM_FlyMove() {
    let mut i: i32 = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut wishspeed: f32 = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    let mut scale: f32 = 0.;
    // normal slowdown
    PM_Friction();
    scale = PM_CmdScale(&mut (*pm).cmd);
    //
    // user intentions
    //
    if scale == 0. {
        wishvel[0 as i32 as usize] = 0 as i32 as vec_t;
        wishvel[1 as i32 as usize] = 0 as i32 as vec_t;
        wishvel[2 as i32 as usize] = 0 as i32 as vec_t
    } else {
        i = 0 as i32;
        while i < 3 as i32 {
            wishvel[i as usize] =
                scale * pml.forward[i as usize] * (*pm).cmd.forwardmove as i32 as f32
                    + scale * pml.right[i as usize] * (*pm).cmd.rightmove as i32 as f32;
            i += 1
        }
        wishvel[2 as i32 as usize] += scale * (*pm).cmd.upmove as i32 as f32
    }
    wishdir[0 as i32 as usize] = wishvel[0 as i32 as usize];
    wishdir[1 as i32 as usize] = wishvel[1 as i32 as usize];
    wishdir[2 as i32 as usize] = wishvel[2 as i32 as usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_flyaccelerate);
    PM_StepSlideMove(qfalse);
}
/*
===================
PM_AirMove

===================
*/

unsafe extern "C" fn PM_AirMove() {
    let mut i: i32 = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut fmove: f32 = 0.;
    let mut smove: f32 = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    let mut wishspeed: f32 = 0.;
    let mut scale: f32 = 0.;
    let mut cmd: usercmd_t = usercmd_t {
        serverTime: 0,
        angles: [0; 3],
        buttons: 0,
        weapon: 0,
        forwardmove: 0,
        rightmove: 0,
        upmove: 0,
    };
    PM_Friction();
    fmove = (*pm).cmd.forwardmove as f32;
    smove = (*pm).cmd.rightmove as f32;
    cmd = (*pm).cmd;
    scale = PM_CmdScale(&mut cmd);
    // set the movementDir so clients can rotate the legs for strafing
    PM_SetMovementDir();
    // project moves down to flat plane
    pml.forward[2 as i32 as usize] = 0 as i32 as vec_t;
    pml.right[2 as i32 as usize] = 0 as i32 as vec_t;
    VectorNormalize(pml.forward.as_mut_ptr());
    VectorNormalize(pml.right.as_mut_ptr());
    i = 0 as i32;
    while i < 2 as i32 {
        wishvel[i as usize] = pml.forward[i as usize] * fmove + pml.right[i as usize] * smove;
        i += 1
    }
    wishvel[2 as i32 as usize] = 0 as i32 as vec_t;
    wishdir[0 as i32 as usize] = wishvel[0 as i32 as usize];
    wishdir[1 as i32 as usize] = wishvel[1 as i32 as usize];
    wishdir[2 as i32 as usize] = wishvel[2 as i32 as usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    wishspeed *= scale;
    // not on ground, so little effect on velocity
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_airaccelerate);
    // we may have a ground plane that is very steep, even
    // though we don't have a groundentity
    // slide along the steep plane
    if pml.groundPlane as u64 != 0 {
        PM_ClipVelocity(
            (*(*pm).ps).velocity.as_mut_ptr(),
            pml.groundTrace.plane.normal.as_mut_ptr(),
            (*(*pm).ps).velocity.as_mut_ptr(),
            1.001f32,
        );
    }
    PM_StepSlideMove(qtrue);
}
/*
===================
PM_GrappleMove

===================
*/

unsafe extern "C" fn PM_GrappleMove() {
    let mut vel: vec3_t = [0.; 3];
    let mut v: vec3_t = [0.; 3];
    let mut vlen: f32 = 0.;
    v[0 as i32 as usize] = pml.forward[0 as i32 as usize] * -(16 as i32) as f32;
    v[1 as i32 as usize] = pml.forward[1 as i32 as usize] * -(16 as i32) as f32;
    v[2 as i32 as usize] = pml.forward[2 as i32 as usize] * -(16 as i32) as f32;
    v[0 as i32 as usize] = (*(*pm).ps).grapplePoint[0 as i32 as usize] + v[0 as i32 as usize];
    v[1 as i32 as usize] = (*(*pm).ps).grapplePoint[1 as i32 as usize] + v[1 as i32 as usize];
    v[2 as i32 as usize] = (*(*pm).ps).grapplePoint[2 as i32 as usize] + v[2 as i32 as usize];
    vel[0 as i32 as usize] = v[0 as i32 as usize] - (*(*pm).ps).origin[0 as i32 as usize];
    vel[1 as i32 as usize] = v[1 as i32 as usize] - (*(*pm).ps).origin[1 as i32 as usize];
    vel[2 as i32 as usize] = v[2 as i32 as usize] - (*(*pm).ps).origin[2 as i32 as usize];
    vlen = VectorLength(vel.as_mut_ptr() as *const vec_t);
    VectorNormalize(vel.as_mut_ptr());
    if vlen <= 100 as i32 as f32 {
        vel[0 as i32 as usize] = vel[0 as i32 as usize] * (10 as i32 as f32 * vlen);
        vel[1 as i32 as usize] = vel[1 as i32 as usize] * (10 as i32 as f32 * vlen);
        vel[2 as i32 as usize] = vel[2 as i32 as usize] * (10 as i32 as f32 * vlen)
    } else {
        vel[0 as i32 as usize] = vel[0 as i32 as usize] * 800 as i32 as f32;
        vel[1 as i32 as usize] = vel[1 as i32 as usize] * 800 as i32 as f32;
        vel[2 as i32 as usize] = vel[2 as i32 as usize] * 800 as i32 as f32
    }
    (*(*pm).ps).velocity[0 as i32 as usize] = vel[0 as i32 as usize];
    (*(*pm).ps).velocity[1 as i32 as usize] = vel[1 as i32 as usize];
    (*(*pm).ps).velocity[2 as i32 as usize] = vel[2 as i32 as usize];
    pml.groundPlane = qfalse;
}
/*
===================
PM_WalkMove

===================
*/

unsafe extern "C" fn PM_WalkMove() {
    let mut i: i32 = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut fmove: f32 = 0.;
    let mut smove: f32 = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    let mut wishspeed: f32 = 0.;
    let mut scale: f32 = 0.;
    let mut cmd: usercmd_t = usercmd_t {
        serverTime: 0,
        angles: [0; 3],
        buttons: 0,
        weapon: 0,
        forwardmove: 0,
        rightmove: 0,
        upmove: 0,
    };
    let mut accelerate: f32 = 0.;
    let mut vel: f32 = 0.;
    if (*pm).waterlevel > 2 as i32
        && pml.forward[0 as i32 as usize] * pml.groundTrace.plane.normal[0 as i32 as usize]
            + pml.forward[1 as i32 as usize] * pml.groundTrace.plane.normal[1 as i32 as usize]
            + pml.forward[2 as i32 as usize] * pml.groundTrace.plane.normal[2 as i32 as usize]
            > 0 as i32 as f32
    {
        // begin swimming
        PM_WaterMove();
        return;
    }
    if PM_CheckJump() as u64 != 0 {
        // jumped away
        if (*pm).waterlevel > 1 as i32 {
            PM_WaterMove();
        } else {
            PM_AirMove();
        }
        return;
    }
    PM_Friction();
    fmove = (*pm).cmd.forwardmove as f32;
    smove = (*pm).cmd.rightmove as f32;
    cmd = (*pm).cmd;
    scale = PM_CmdScale(&mut cmd);
    // set the movementDir so clients can rotate the legs for strafing
    PM_SetMovementDir();
    // project moves down to flat plane
    pml.forward[2 as i32 as usize] = 0 as i32 as vec_t;
    pml.right[2 as i32 as usize] = 0 as i32 as vec_t;
    // project the forward and right directions onto the ground plane
    PM_ClipVelocity(
        pml.forward.as_mut_ptr(),
        pml.groundTrace.plane.normal.as_mut_ptr(),
        pml.forward.as_mut_ptr(),
        1.001f32,
    );
    PM_ClipVelocity(
        pml.right.as_mut_ptr(),
        pml.groundTrace.plane.normal.as_mut_ptr(),
        pml.right.as_mut_ptr(),
        1.001f32,
    );
    //
    VectorNormalize(pml.forward.as_mut_ptr());
    VectorNormalize(pml.right.as_mut_ptr());
    i = 0 as i32;
    while i < 3 as i32 {
        wishvel[i as usize] = pml.forward[i as usize] * fmove + pml.right[i as usize] * smove;
        i += 1
    }
    // when going up or down slopes the wish velocity should Not be zero
    //	wishvel[2] = 0;
    wishdir[0 as i32 as usize] = wishvel[0 as i32 as usize];
    wishdir[1 as i32 as usize] = wishvel[1 as i32 as usize];
    wishdir[2 as i32 as usize] = wishvel[2 as i32 as usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    wishspeed *= scale;
    // clamp the speed lower if ducking
    if (*(*pm).ps).pm_flags & 1 as i32 != 0 {
        if wishspeed > (*(*pm).ps).speed as f32 * pm_duckScale {
            wishspeed = (*(*pm).ps).speed as f32 * pm_duckScale
        }
    }
    // clamp the speed lower if wading or walking on the bottom
    if (*pm).waterlevel != 0 {
        let mut waterScale: f32 = 0.;
        waterScale = ((*pm).waterlevel as f64 / 3.0f64) as f32;
        waterScale = (1.0f64 - (1.0f64 - pm_swimScale as f64) * waterScale as f64) as f32;
        if wishspeed > (*(*pm).ps).speed as f32 * waterScale {
            wishspeed = (*(*pm).ps).speed as f32 * waterScale
        }
    }
    // when a player gets hit, they temporarily lose
    // full control, which allows them to be moved a bit
    if pml.groundTrace.surfaceFlags & 0x2 as i32 != 0 || (*(*pm).ps).pm_flags & 64 as i32 != 0 {
        accelerate = pm_airaccelerate
    } else {
        accelerate = pm_accelerate
    }
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, accelerate);
    //Com_Printf("velocity = %1.1f %1.1f %1.1f\n", pm->ps->velocity[0], pm->ps->velocity[1], pm->ps->velocity[2]);
    //Com_Printf("velocity1 = %1.1f\n", VectorLength(pm->ps->velocity));
    if pml.groundTrace.surfaceFlags & 0x2 as i32 != 0 || (*(*pm).ps).pm_flags & 64 as i32 != 0 {
        (*(*pm).ps).velocity[2 as i32 as usize] -= (*(*pm).ps).gravity as f32 * pml.frametime
    }
    vel = VectorLength((*(*pm).ps).velocity.as_mut_ptr() as *const vec_t);
    // slide along the ground plane
    PM_ClipVelocity(
        (*(*pm).ps).velocity.as_mut_ptr(),
        pml.groundTrace.plane.normal.as_mut_ptr(),
        (*(*pm).ps).velocity.as_mut_ptr(),
        1.001f32,
    );
    // don't decrease velocity when going up or down a slope
    VectorNormalize((*(*pm).ps).velocity.as_mut_ptr());
    (*(*pm).ps).velocity[0 as i32 as usize] = (*(*pm).ps).velocity[0 as i32 as usize] * vel;
    (*(*pm).ps).velocity[1 as i32 as usize] = (*(*pm).ps).velocity[1 as i32 as usize] * vel;
    (*(*pm).ps).velocity[2 as i32 as usize] = (*(*pm).ps).velocity[2 as i32 as usize] * vel;
    // don't do anything if standing still
    if (*(*pm).ps).velocity[0 as i32 as usize] == 0.
        && (*(*pm).ps).velocity[1 as i32 as usize] == 0.
    {
        return;
    }
    PM_StepSlideMove(qfalse);
    //Com_Printf("velocity2 = %1.1f\n", VectorLength(pm->ps->velocity));
}
/*
==============
PM_DeadMove
==============
*/

unsafe extern "C" fn PM_DeadMove() {
    let mut forward: f32 = 0.;
    if pml.walking as u64 == 0 {
        return;
    }
    // extra friction
    forward = VectorLength((*(*pm).ps).velocity.as_mut_ptr() as *const vec_t);
    forward -= 20 as i32 as f32;
    if forward <= 0 as i32 as f32 {
        (*(*pm).ps).velocity[2 as i32 as usize] = 0 as i32 as vec_t;
        (*(*pm).ps).velocity[1 as i32 as usize] = (*(*pm).ps).velocity[2 as i32 as usize];
        (*(*pm).ps).velocity[0 as i32 as usize] = (*(*pm).ps).velocity[1 as i32 as usize]
    } else {
        VectorNormalize((*(*pm).ps).velocity.as_mut_ptr());
        (*(*pm).ps).velocity[0 as i32 as usize] = (*(*pm).ps).velocity[0 as i32 as usize] * forward;
        (*(*pm).ps).velocity[1 as i32 as usize] = (*(*pm).ps).velocity[1 as i32 as usize] * forward;
        (*(*pm).ps).velocity[2 as i32 as usize] = (*(*pm).ps).velocity[2 as i32 as usize] * forward
    };
}
/*
===============
PM_NoclipMove
===============
*/

unsafe extern "C" fn PM_NoclipMove() {
    let mut speed: f32 = 0.;
    let mut drop_0: f32 = 0.;
    let mut friction: f32 = 0.;
    let mut control: f32 = 0.;
    let mut newspeed: f32 = 0.;
    let mut i: i32 = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut fmove: f32 = 0.;
    let mut smove: f32 = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    let mut wishspeed: f32 = 0.;
    let mut scale: f32 = 0.;
    (*(*pm).ps).viewheight = 26 as i32;
    // friction
    speed = VectorLength((*(*pm).ps).velocity.as_mut_ptr() as *const vec_t); // extra friction
    if speed < 1 as i32 as f32 {
        (*(*pm).ps).velocity[0 as i32 as usize] = vec3_origin[0 as i32 as usize];
        (*(*pm).ps).velocity[1 as i32 as usize] = vec3_origin[1 as i32 as usize];
        (*(*pm).ps).velocity[2 as i32 as usize] = vec3_origin[2 as i32 as usize]
    } else {
        drop_0 = 0 as i32 as f32;
        friction = (pm_friction as f64 * 1.5f64) as f32;
        control = if speed < pm_stopspeed {
            pm_stopspeed
        } else {
            speed
        };
        drop_0 += control * friction * pml.frametime;
        // scale the velocity
        newspeed = speed - drop_0;
        if newspeed < 0 as i32 as f32 {
            newspeed = 0 as i32 as f32
        }
        newspeed /= speed;
        (*(*pm).ps).velocity[0 as i32 as usize] =
            (*(*pm).ps).velocity[0 as i32 as usize] * newspeed;
        (*(*pm).ps).velocity[1 as i32 as usize] =
            (*(*pm).ps).velocity[1 as i32 as usize] * newspeed;
        (*(*pm).ps).velocity[2 as i32 as usize] = (*(*pm).ps).velocity[2 as i32 as usize] * newspeed
    }
    // accelerate
    scale = PM_CmdScale(&mut (*pm).cmd);
    fmove = (*pm).cmd.forwardmove as f32;
    smove = (*pm).cmd.rightmove as f32;
    i = 0 as i32;
    while i < 3 as i32 {
        wishvel[i as usize] = pml.forward[i as usize] * fmove + pml.right[i as usize] * smove;
        i += 1
    }
    wishvel[2 as i32 as usize] += (*pm).cmd.upmove as i32 as f32;
    wishdir[0 as i32 as usize] = wishvel[0 as i32 as usize];
    wishdir[1 as i32 as usize] = wishvel[1 as i32 as usize];
    wishdir[2 as i32 as usize] = wishvel[2 as i32 as usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    wishspeed *= scale;
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_accelerate);
    // move
    (*(*pm).ps).origin[0 as i32 as usize] = (*(*pm).ps).origin[0 as i32 as usize]
        + (*(*pm).ps).velocity[0 as i32 as usize] * pml.frametime;
    (*(*pm).ps).origin[1 as i32 as usize] = (*(*pm).ps).origin[1 as i32 as usize]
        + (*(*pm).ps).velocity[1 as i32 as usize] * pml.frametime;
    (*(*pm).ps).origin[2 as i32 as usize] = (*(*pm).ps).origin[2 as i32 as usize]
        + (*(*pm).ps).velocity[2 as i32 as usize] * pml.frametime;
}
//============================================================================
/*
================
PM_FootstepForSurface

Returns an event number appropriate for the groundsurface
================
*/

unsafe extern "C" fn PM_FootstepForSurface() -> i32 {
    if pml.groundTrace.surfaceFlags & 0x2000 as i32 != 0 {
        return 0 as i32;
    }
    if pml.groundTrace.surfaceFlags & 0x1000 as i32 != 0 {
        return EV_FOOTSTEP_METAL as i32;
    }
    return EV_FOOTSTEP as i32;
}
/*
=================
PM_CrashLand

Check for hard landings that generate sound events
=================
*/

unsafe extern "C" fn PM_CrashLand() {
    let mut delta: f32 = 0.;
    let mut dist: f32 = 0.;
    let mut vel: f32 = 0.;
    let mut acc: f32 = 0.;
    let mut t: f32 = 0.;
    let mut a: f32 = 0.;
    let mut b: f32 = 0.;
    let mut c: f32 = 0.;
    let mut den: f32 = 0.;
    // decide which landing animation to use
    if (*(*pm).ps).pm_flags & 8 as i32 != 0 {
        PM_ForceLegsAnim(LEGS_LANDB as i32);
    } else {
        PM_ForceLegsAnim(LEGS_LAND as i32);
    }
    (*(*pm).ps).legsTimer = 130 as i32;
    // calculate the exact velocity on landing
    dist = (*(*pm).ps).origin[2 as i32 as usize] - pml.previous_origin[2 as i32 as usize];
    vel = pml.previous_velocity[2 as i32 as usize];
    acc = -(*(*pm).ps).gravity as f32;
    a = acc / 2 as i32 as f32;
    b = vel;
    c = -dist;
    den = b * b - 4 as i32 as f32 * a * c;
    if den < 0 as i32 as f32 {
        return;
    }
    t = ((-b as f64 - crate::stdlib::sqrt(den as f64)) / (2 as i32 as f32 * a) as f64) as f32;
    delta = vel + t * acc;
    delta = ((delta * delta) as f64 * 0.0001f64) as f32;
    // ducking while falling doubles damage
    if (*(*pm).ps).pm_flags & 1 as i32 != 0 {
        delta *= 2 as i32 as f32
    }
    // never take falling damage if completely underwater
    if (*pm).waterlevel == 3 as i32 {
        return;
    }
    // reduce falling damage if there is standing water
    if (*pm).waterlevel == 2 as i32 {
        delta = (delta as f64 * 0.25f64) as f32
    }
    if (*pm).waterlevel == 1 as i32 {
        delta = (delta as f64 * 0.5f64) as f32
    }
    if delta < 1 as i32 as f32 {
        return;
    }
    // create a local entity event to play the sound
    // SURF_NODAMAGE is used for bounce pads where you don't ever
    // want to take damage or play a crunch sound
    if pml.groundTrace.surfaceFlags & 0x1 as i32 == 0 {
        if delta > 60 as i32 as f32 {
            PM_AddEvent(EV_FALL_FAR as i32);
        } else if delta > 40 as i32 as f32 {
            // this is a pain grunt, so don't play it if dead
            if (*(*pm).ps).stats[STAT_HEALTH as i32 as usize] > 0 as i32 {
                PM_AddEvent(EV_FALL_MEDIUM as i32);
            }
        } else if delta > 7 as i32 as f32 {
            PM_AddEvent(EV_FALL_SHORT as i32);
        } else {
            PM_AddEvent(PM_FootstepForSurface());
        }
    }
    // start footstep cycle over
    (*(*pm).ps).bobCycle = 0 as i32;
}
/*
=============
PM_CheckStuck
=============
*/
/*
void PM_CheckStuck(void) {
    trace_t trace;

    pm->trace (&trace, pm->ps->origin, pm->mins, pm->maxs, pm->ps->origin, pm->ps->clientNum, pm->tracemask);
    if (trace.allsolid) {
        //int shit = qtrue;
    }
}
*/
/*
=============
PM_CorrectAllSolid
=============
*/

unsafe extern "C" fn PM_CorrectAllSolid(mut trace: *mut trace_t) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut point: vec3_t = [0.; 3];
    if (*pm).debugLevel != 0 {
        Com_Printf(
            b"%i:allsolid\n\x00" as *const u8 as *const libc::c_char,
            c_pmove,
        );
    }
    // jitter around
    i = -(1 as i32);
    while i <= 1 as i32 {
        j = -(1 as i32);
        while j <= 1 as i32 {
            k = -(1 as i32);
            while k <= 1 as i32 {
                point[0 as i32 as usize] = (*(*pm).ps).origin[0 as i32 as usize];
                point[1 as i32 as usize] = (*(*pm).ps).origin[1 as i32 as usize];
                point[2 as i32 as usize] = (*(*pm).ps).origin[2 as i32 as usize];
                point[0 as i32 as usize] += i as f32;
                point[1 as i32 as usize] += j as f32;
                point[2 as i32 as usize] += k as f32;
                (*pm).trace.expect("non-null function pointer")(
                    trace,
                    point.as_mut_ptr() as *const vec_t,
                    (*pm).mins.as_mut_ptr() as *const vec_t,
                    (*pm).maxs.as_mut_ptr() as *const vec_t,
                    point.as_mut_ptr() as *const vec_t,
                    (*(*pm).ps).clientNum,
                    (*pm).tracemask,
                );
                if (*trace).allsolid as u64 == 0 {
                    point[0 as i32 as usize] = (*(*pm).ps).origin[0 as i32 as usize];
                    point[1 as i32 as usize] = (*(*pm).ps).origin[1 as i32 as usize];
                    point[2 as i32 as usize] =
                        ((*(*pm).ps).origin[2 as i32 as usize] as f64 - 0.25f64) as vec_t;
                    (*pm).trace.expect("non-null function pointer")(
                        trace,
                        (*(*pm).ps).origin.as_mut_ptr() as *const vec_t,
                        (*pm).mins.as_mut_ptr() as *const vec_t,
                        (*pm).maxs.as_mut_ptr() as *const vec_t,
                        point.as_mut_ptr() as *const vec_t,
                        (*(*pm).ps).clientNum,
                        (*pm).tracemask,
                    );
                    pml.groundTrace = *trace;
                    return qtrue as i32;
                }
                k += 1
            }
            j += 1
        }
        i += 1
    }
    (*(*pm).ps).groundEntityNum = ((1 as i32) << 10 as i32) - 1 as i32;
    pml.groundPlane = qfalse;
    pml.walking = qfalse;
    return qfalse as i32;
}
/*
=============
PM_GroundTraceMissed

The ground trace didn't hit a surface, so we are in freefall
=============
*/

unsafe extern "C" fn PM_GroundTraceMissed() {
    let mut trace: trace_t = trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
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
    let mut point: vec3_t = [0.; 3];
    if (*(*pm).ps).groundEntityNum != ((1 as i32) << 10 as i32) - 1 as i32 {
        // we just transitioned into freefall
        if (*pm).debugLevel != 0 {
            Com_Printf(
                b"%i:lift\n\x00" as *const u8 as *const libc::c_char,
                c_pmove,
            );
        }
        // if they aren't in a jumping animation and the ground is a ways away, force into it
        // if we didn't do the trace, the player would be backflipping down staircases
        point[0 as i32 as usize] = (*(*pm).ps).origin[0 as i32 as usize];
        point[1 as i32 as usize] = (*(*pm).ps).origin[1 as i32 as usize];
        point[2 as i32 as usize] = (*(*pm).ps).origin[2 as i32 as usize];
        point[2 as i32 as usize] -= 64 as i32 as f32;
        (*pm).trace.expect("non-null function pointer")(
            &mut trace,
            (*(*pm).ps).origin.as_mut_ptr() as *const vec_t,
            (*pm).mins.as_mut_ptr() as *const vec_t,
            (*pm).maxs.as_mut_ptr() as *const vec_t,
            point.as_mut_ptr() as *const vec_t,
            (*(*pm).ps).clientNum,
            (*pm).tracemask,
        );
        if trace.fraction as f64 == 1.0f64 {
            if (*pm).cmd.forwardmove as i32 >= 0 as i32 {
                PM_ForceLegsAnim(LEGS_JUMP as i32);
                (*(*pm).ps).pm_flags &= !(8 as i32)
            } else {
                PM_ForceLegsAnim(LEGS_JUMPB as i32);
                (*(*pm).ps).pm_flags |= 8 as i32
            }
        }
    }
    (*(*pm).ps).groundEntityNum = ((1 as i32) << 10 as i32) - 1 as i32;
    pml.groundPlane = qfalse;
    pml.walking = qfalse;
}
/*
=============
PM_GroundTrace
=============
*/

unsafe extern "C" fn PM_GroundTrace() {
    let mut point: vec3_t = [0.; 3];
    let mut trace: trace_t = trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
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
    point[0 as i32 as usize] = (*(*pm).ps).origin[0 as i32 as usize];
    point[1 as i32 as usize] = (*(*pm).ps).origin[1 as i32 as usize];
    point[2 as i32 as usize] = ((*(*pm).ps).origin[2 as i32 as usize] as f64 - 0.25f64) as vec_t;
    (*pm).trace.expect("non-null function pointer")(
        &mut trace,
        (*(*pm).ps).origin.as_mut_ptr() as *const vec_t,
        (*pm).mins.as_mut_ptr() as *const vec_t,
        (*pm).maxs.as_mut_ptr() as *const vec_t,
        point.as_mut_ptr() as *const vec_t,
        (*(*pm).ps).clientNum,
        (*pm).tracemask,
    );
    pml.groundTrace = trace;
    // do something corrective if the trace starts in a solid...
    if trace.allsolid as u64 != 0 {
        if PM_CorrectAllSolid(&mut trace) == 0 {
            return;
        }
    }
    // if the trace didn't hit anything, we are in free fall
    if trace.fraction as f64 == 1.0f64 {
        PM_GroundTraceMissed();
        pml.groundPlane = qfalse;
        pml.walking = qfalse;
        return;
    }
    // check if getting thrown off the ground
    if (*(*pm).ps).velocity[2 as i32 as usize] > 0 as i32 as f32
        && (*(*pm).ps).velocity[0 as i32 as usize] * trace.plane.normal[0 as i32 as usize]
            + (*(*pm).ps).velocity[1 as i32 as usize] * trace.plane.normal[1 as i32 as usize]
            + (*(*pm).ps).velocity[2 as i32 as usize] * trace.plane.normal[2 as i32 as usize]
            > 10 as i32 as f32
    {
        if (*pm).debugLevel != 0 {
            Com_Printf(
                b"%i:kickoff\n\x00" as *const u8 as *const libc::c_char,
                c_pmove,
            );
        }
        // go into jump animation
        if (*pm).cmd.forwardmove as i32 >= 0 as i32 {
            PM_ForceLegsAnim(LEGS_JUMP as i32);
            (*(*pm).ps).pm_flags &= !(8 as i32)
        } else {
            PM_ForceLegsAnim(LEGS_JUMPB as i32);
            (*(*pm).ps).pm_flags |= 8 as i32
        }
        (*(*pm).ps).groundEntityNum = ((1 as i32) << 10 as i32) - 1 as i32;
        pml.groundPlane = qfalse;
        pml.walking = qfalse;
        return;
    }
    // slopes that are too steep will not be considered onground
    if trace.plane.normal[2 as i32 as usize] < 0.7f32 {
        if (*pm).debugLevel != 0 {
            Com_Printf(
                b"%i:steep\n\x00" as *const u8 as *const libc::c_char,
                c_pmove,
            );
        }
        // FIXME: if they can't slide down the slope, let them
        // walk (sharp crevices)
        (*(*pm).ps).groundEntityNum = ((1 as i32) << 10 as i32) - 1 as i32;
        pml.groundPlane = qtrue;
        pml.walking = qfalse;
        return;
    }
    pml.groundPlane = qtrue;
    pml.walking = qtrue;
    // hitting solid ground will end a waterjump
    if (*(*pm).ps).pm_flags & 256 as i32 != 0 {
        (*(*pm).ps).pm_flags &= !(256 as i32 | 32 as i32);
        (*(*pm).ps).pm_time = 0 as i32
    }
    if (*(*pm).ps).groundEntityNum == ((1 as i32) << 10 as i32) - 1 as i32 {
        // just hit the ground
        if (*pm).debugLevel != 0 {
            Com_Printf(
                b"%i:Land\n\x00" as *const u8 as *const libc::c_char,
                c_pmove,
            );
        }
        PM_CrashLand();
        // don't do landing time if we were just going down a slope
        if pml.previous_velocity[2 as i32 as usize] < -(200 as i32) as f32 {
            // don't allow another jump for a little while
            (*(*pm).ps).pm_flags |= 32 as i32;
            (*(*pm).ps).pm_time = 250 as i32
        }
    }
    (*(*pm).ps).groundEntityNum = trace.entityNum;
    // don't reset the z velocity for slopes
    //	pm->ps->velocity[2] = 0;
    PM_AddTouchEnt(trace.entityNum);
}
/*
=============
PM_SetWaterLevel	FIXME: avoid this twice?  certainly if not moving
=============
*/

unsafe extern "C" fn PM_SetWaterLevel() {
    let mut point: vec3_t = [0.; 3];
    let mut cont: i32 = 0;
    let mut sample1: i32 = 0;
    let mut sample2: i32 = 0;
    //
    // get waterlevel, accounting for ducking
    //
    (*pm).waterlevel = 0 as i32;
    (*pm).watertype = 0 as i32;
    point[0 as i32 as usize] = (*(*pm).ps).origin[0 as i32 as usize];
    point[1 as i32 as usize] = (*(*pm).ps).origin[1 as i32 as usize];
    point[2 as i32 as usize] =
        (*(*pm).ps).origin[2 as i32 as usize] + -(24 as i32) as f32 + 1 as i32 as f32;
    cont = (*pm).pointcontents.expect("non-null function pointer")(
        point.as_mut_ptr() as *const vec_t,
        (*(*pm).ps).clientNum,
    );
    if cont & (32 as i32 | 8 as i32 | 16 as i32) != 0 {
        sample2 = (*(*pm).ps).viewheight - -(24 as i32);
        sample1 = sample2 / 2 as i32;
        (*pm).watertype = cont;
        (*pm).waterlevel = 1 as i32;
        point[2 as i32 as usize] =
            (*(*pm).ps).origin[2 as i32 as usize] + -(24 as i32) as f32 + sample1 as f32;
        cont = (*pm).pointcontents.expect("non-null function pointer")(
            point.as_mut_ptr() as *const vec_t,
            (*(*pm).ps).clientNum,
        );
        if cont & (32 as i32 | 8 as i32 | 16 as i32) != 0 {
            (*pm).waterlevel = 2 as i32;
            point[2 as i32 as usize] =
                (*(*pm).ps).origin[2 as i32 as usize] + -(24 as i32) as f32 + sample2 as f32;
            cont = (*pm).pointcontents.expect("non-null function pointer")(
                point.as_mut_ptr() as *const vec_t,
                (*(*pm).ps).clientNum,
            );
            if cont & (32 as i32 | 8 as i32 | 16 as i32) != 0 {
                (*pm).waterlevel = 3 as i32
            }
        }
    };
}
/*
==============
PM_CheckDuck

Sets mins, maxs, and pm->ps->viewheight
==============
*/

unsafe extern "C" fn PM_CheckDuck() {
    let mut trace: trace_t = trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
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
    if (*(*pm).ps).powerups[PW_INVULNERABILITY as i32 as usize] != 0 {
        if (*(*pm).ps).pm_flags & 16384 as i32 != 0 {
            // invulnerability sphere has a 42 units radius
            (*pm).mins[0 as i32 as usize] = -(42 as i32) as vec_t;
            (*pm).mins[1 as i32 as usize] = -(42 as i32) as vec_t;
            (*pm).mins[2 as i32 as usize] = -(42 as i32) as vec_t;
            (*pm).maxs[0 as i32 as usize] = 42 as i32 as vec_t;
            (*pm).maxs[1 as i32 as usize] = 42 as i32 as vec_t;
            (*pm).maxs[2 as i32 as usize] = 42 as i32 as vec_t
        } else {
            (*pm).mins[0 as i32 as usize] = -(15 as i32) as vec_t;
            (*pm).mins[1 as i32 as usize] = -(15 as i32) as vec_t;
            (*pm).mins[2 as i32 as usize] = -(24 as i32) as vec_t;
            (*pm).maxs[0 as i32 as usize] = 15 as i32 as vec_t;
            (*pm).maxs[1 as i32 as usize] = 15 as i32 as vec_t;
            (*pm).maxs[2 as i32 as usize] = 16 as i32 as vec_t
        }
        (*(*pm).ps).pm_flags |= 1 as i32;
        (*(*pm).ps).viewheight = 12 as i32;
        return;
    }
    (*(*pm).ps).pm_flags &= !(16384 as i32);
    (*pm).mins[0 as i32 as usize] = -(15 as i32) as vec_t;
    (*pm).mins[1 as i32 as usize] = -(15 as i32) as vec_t;
    (*pm).maxs[0 as i32 as usize] = 15 as i32 as vec_t;
    (*pm).maxs[1 as i32 as usize] = 15 as i32 as vec_t;
    (*pm).mins[2 as i32 as usize] = -(24 as i32) as vec_t;
    if (*(*pm).ps).pm_type == PM_DEAD as i32 {
        (*pm).maxs[2 as i32 as usize] = -(8 as i32) as vec_t;
        (*(*pm).ps).viewheight = -(16 as i32);
        return;
    }
    if ((*pm).cmd.upmove as i32) < 0 as i32 {
        // duck
        (*(*pm).ps).pm_flags |= 1 as i32
    } else if (*(*pm).ps).pm_flags & 1 as i32 != 0 {
        // stand up if possible
        // try to stand up
        (*pm).maxs[2 as i32 as usize] = 32 as i32 as vec_t;
        (*pm).trace.expect("non-null function pointer")(
            &mut trace,
            (*(*pm).ps).origin.as_mut_ptr() as *const vec_t,
            (*pm).mins.as_mut_ptr() as *const vec_t,
            (*pm).maxs.as_mut_ptr() as *const vec_t,
            (*(*pm).ps).origin.as_mut_ptr() as *const vec_t,
            (*(*pm).ps).clientNum,
            (*pm).tracemask,
        );
        if trace.allsolid as u64 == 0 {
            (*(*pm).ps).pm_flags &= !(1 as i32)
        }
    }
    if (*(*pm).ps).pm_flags & 1 as i32 != 0 {
        (*pm).maxs[2 as i32 as usize] = 16 as i32 as vec_t;
        (*(*pm).ps).viewheight = 12 as i32
    } else {
        (*pm).maxs[2 as i32 as usize] = 32 as i32 as vec_t;
        (*(*pm).ps).viewheight = 26 as i32
    };
}
//===================================================================
/*
===============
PM_Footsteps
===============
*/

unsafe extern "C" fn PM_Footsteps() {
    let mut bobmove: f32 = 0.;
    let mut old: i32 = 0;
    let mut footstep: qboolean = qfalse;
    //
    // calculate speed and cycle to be used for
    // all cyclic walking effects
    //
    (*pm).xyspeed = crate::stdlib::sqrt(
        ((*(*pm).ps).velocity[0 as i32 as usize] * (*(*pm).ps).velocity[0 as i32 as usize]
            + (*(*pm).ps).velocity[1 as i32 as usize] * (*(*pm).ps).velocity[1 as i32 as usize])
            as f64,
    ) as f32;
    if (*(*pm).ps).groundEntityNum == ((1 as i32) << 10 as i32) - 1 as i32 {
        if (*(*pm).ps).powerups[PW_INVULNERABILITY as i32 as usize] != 0 {
            PM_ContinueLegsAnim(LEGS_IDLECR as i32);
        }
        // airborne leaves position in cycle intact, but doesn't advance
        if (*pm).waterlevel > 1 as i32 {
            PM_ContinueLegsAnim(LEGS_SWIM as i32);
        }
        return;
    }
    // if not trying to move
    if (*pm).cmd.forwardmove == 0 && (*pm).cmd.rightmove == 0 {
        if (*pm).xyspeed < 5 as i32 as f32 {
            (*(*pm).ps).bobCycle = 0 as i32; // start at beginning of cycle again
            if (*(*pm).ps).pm_flags & 1 as i32 != 0 {
                PM_ContinueLegsAnim(LEGS_IDLECR as i32);
            // ducked characters bob much faster
            } else {
                PM_ContinueLegsAnim(LEGS_IDLE as i32);
            }
        }
        return;
    }
    footstep = qfalse;
    if (*(*pm).ps).pm_flags & 1 as i32 != 0 {
        bobmove = 0.5f64 as f32;
        if (*(*pm).ps).pm_flags & 16 as i32 != 0 {
            PM_ContinueLegsAnim(LEGS_BACKCR as i32);
        } else {
            PM_ContinueLegsAnim(LEGS_WALKCR as i32);
        }
    // ducked characters never play footsteps
    /*
    } else 	if ( pm->ps->pm_flags & PMF_BACKWARDS_RUN ) {
        if ( !( pm->cmd.buttons & BUTTON_WALKING ) ) {
            bobmove = 0.4;	// faster speeds bob faster
            footstep = qtrue;
        } else {
            bobmove = 0.3;
        }
        PM_ContinueLegsAnim( LEGS_BACK );
    */
    } else if (*pm).cmd.buttons & 16 as i32 == 0 {
        bobmove = 0.4f32; // faster speeds bob faster
        if (*(*pm).ps).pm_flags & 16 as i32 != 0 {
            PM_ContinueLegsAnim(LEGS_BACK as i32); // walking bobs slow
        } else {
            PM_ContinueLegsAnim(LEGS_RUN as i32);
        }
        footstep = qtrue
    } else {
        bobmove = 0.3f32;
        if (*(*pm).ps).pm_flags & 16 as i32 != 0 {
            PM_ContinueLegsAnim(LEGS_BACKWALK as i32);
        } else {
            PM_ContinueLegsAnim(LEGS_WALK as i32);
        }
    }
    // check for footstep / splash sounds
    old = (*(*pm).ps).bobCycle;
    (*(*pm).ps).bobCycle = (old as f32 + bobmove * pml.msec as f32) as i32 & 255 as i32;
    // if we just crossed a cycle boundary, play an appropriate footstep event
    if (old + 64 as i32 ^ (*(*pm).ps).bobCycle + 64 as i32) & 128 as i32 != 0 {
        if (*pm).waterlevel == 0 as i32 {
            // on ground will only play sounds if running
            if footstep as u32 != 0 && (*pm).noFootsteps as u64 == 0 {
                PM_AddEvent(PM_FootstepForSurface());
            }
        } else if (*pm).waterlevel == 1 as i32 {
            // splashing
            PM_AddEvent(EV_FOOTSPLASH as i32);
        } else if (*pm).waterlevel == 2 as i32 {
            // wading / swimming at surface
            PM_AddEvent(EV_SWIM as i32);
        } else {
            // waterlevel 3 case has no additional action
        }
    };
}
/*
==============
PM_WaterEvents

Generate sound events for entering and leaving water
==============
*/

unsafe extern "C" fn PM_WaterEvents() {
    // FIXME?
    //
    // if just entered a water volume, play a sound
    //
    if pml.previous_waterlevel == 0 && (*pm).waterlevel != 0 {
        PM_AddEvent(EV_WATER_TOUCH as i32);
    }
    //
    // if just completely exited a water volume, play a sound
    //
    if pml.previous_waterlevel != 0 && (*pm).waterlevel == 0 {
        PM_AddEvent(EV_WATER_LEAVE as i32);
    }
    //
    // check for head just going under water
    //
    if pml.previous_waterlevel != 3 as i32 && (*pm).waterlevel == 3 as i32 {
        PM_AddEvent(EV_WATER_UNDER as i32);
    }
    //
    // check for head just coming out of water
    //
    if pml.previous_waterlevel == 3 as i32 && (*pm).waterlevel != 3 as i32 {
        PM_AddEvent(EV_WATER_CLEAR as i32);
    };
}
/*
===============
PM_BeginWeaponChange
===============
*/

unsafe extern "C" fn PM_BeginWeaponChange(mut weapon: i32) {
    if weapon <= WP_NONE as i32 || weapon >= WP_NUM_WEAPONS as i32 {
        return;
    }
    if (*(*pm).ps).stats[STAT_WEAPONS as i32 as usize] & (1 as i32) << weapon == 0 {
        return;
    }
    if (*(*pm).ps).weaponstate == WEAPON_DROPPING as i32 {
        return;
    }
    PM_AddEvent(EV_CHANGE_WEAPON as i32);
    (*(*pm).ps).weaponstate = WEAPON_DROPPING as i32;
    (*(*pm).ps).weaponTime += 200 as i32;
    PM_StartTorsoAnim(TORSO_DROP as i32);
}
/*
===============
PM_FinishWeaponChange
===============
*/

unsafe extern "C" fn PM_FinishWeaponChange() {
    let mut weapon: i32 = 0;
    weapon = (*pm).cmd.weapon as i32;
    if weapon < WP_NONE as i32 || weapon >= WP_NUM_WEAPONS as i32 {
        weapon = WP_NONE as i32
    }
    if (*(*pm).ps).stats[STAT_WEAPONS as i32 as usize] & (1 as i32) << weapon == 0 {
        weapon = WP_NONE as i32
    }
    (*(*pm).ps).weapon = weapon;
    (*(*pm).ps).weaponstate = WEAPON_RAISING as i32;
    (*(*pm).ps).weaponTime += 250 as i32;
    PM_StartTorsoAnim(TORSO_RAISE as i32);
}
/*
==============
PM_TorsoAnimation

==============
*/

unsafe extern "C" fn PM_TorsoAnimation() {
    if (*(*pm).ps).weaponstate == WEAPON_READY as i32 {
        if (*(*pm).ps).weapon == WP_GAUNTLET as i32 {
            PM_ContinueTorsoAnim(TORSO_STAND2 as i32);
        } else {
            PM_ContinueTorsoAnim(TORSO_STAND as i32);
        }
        return;
    };
}
/*
==============
PM_Weapon

Generates weapon events and modifes the weapon counter
==============
*/

unsafe extern "C" fn PM_Weapon() {
    let mut addTime: i32 = 0;
    // don't allow attack until all buttons are up
    if (*(*pm).ps).pm_flags & 512 as i32 != 0 {
        return;
    }
    // ignore if spectator
    if (*(*pm).ps).persistant[PERS_TEAM as i32 as usize] == TEAM_SPECTATOR as i32 {
        return;
    }
    // check for dead player
    if (*(*pm).ps).stats[STAT_HEALTH as i32 as usize] <= 0 as i32 {
        (*(*pm).ps).weapon = WP_NONE as i32;
        return;
    }
    // check for item using
    if (*pm).cmd.buttons & 4 as i32 != 0 {
        if (*(*pm).ps).pm_flags & 1024 as i32 == 0 {
            if !((*bg_itemlist
                .as_mut_ptr()
                .offset((*(*pm).ps).stats[STAT_HOLDABLE_ITEM as i32 as usize] as isize))
            .giTag
                == HI_MEDKIT as i32
                && (*(*pm).ps).stats[STAT_HEALTH as i32 as usize]
                    >= (*(*pm).ps).stats[STAT_MAX_HEALTH as i32 as usize] + 25 as i32)
            {
                (*(*pm).ps).pm_flags |= 1024 as i32;
                PM_AddEvent(
                    EV_USE_ITEM0 as i32
                        + (*bg_itemlist.as_mut_ptr().offset(
                            (*(*pm).ps).stats[STAT_HOLDABLE_ITEM as i32 as usize] as isize,
                        ))
                        .giTag,
                );
                (*(*pm).ps).stats[STAT_HOLDABLE_ITEM as i32 as usize] = 0 as i32
            }
            return;
        }
    } else {
        (*(*pm).ps).pm_flags &= !(1024 as i32)
    }
    // make weapon function
    if (*(*pm).ps).weaponTime > 0 as i32 {
        (*(*pm).ps).weaponTime -= pml.msec
    }
    // check for weapon change
    // can't change if weapon is firing, but can change
    // again if lowering or raising
    if (*(*pm).ps).weaponTime <= 0 as i32 || (*(*pm).ps).weaponstate != WEAPON_FIRING as i32 {
        if (*(*pm).ps).weapon != (*pm).cmd.weapon as i32 {
            PM_BeginWeaponChange((*pm).cmd.weapon as i32);
        }
    }
    if (*(*pm).ps).weaponTime > 0 as i32 {
        return;
    }
    // change weapon if time
    if (*(*pm).ps).weaponstate == WEAPON_DROPPING as i32 {
        PM_FinishWeaponChange();
        return;
    }
    if (*(*pm).ps).weaponstate == WEAPON_RAISING as i32 {
        (*(*pm).ps).weaponstate = WEAPON_READY as i32;
        if (*(*pm).ps).weapon == WP_GAUNTLET as i32 {
            PM_StartTorsoAnim(TORSO_STAND2 as i32);
        } else {
            PM_StartTorsoAnim(TORSO_STAND as i32);
        }
        return;
    }
    // check for fire
    if (*pm).cmd.buttons & 1 as i32 == 0 {
        (*(*pm).ps).weaponTime = 0 as i32;
        (*(*pm).ps).weaponstate = WEAPON_READY as i32;
        return;
    }
    // start the animation even if out of ammo
    if (*(*pm).ps).weapon == WP_GAUNTLET as i32 {
        // the guantlet only "fires" when it actually hits something
        if (*pm).gauntletHit as u64 == 0 {
            (*(*pm).ps).weaponTime = 0 as i32;
            (*(*pm).ps).weaponstate = WEAPON_READY as i32;
            return;
        }
        PM_StartTorsoAnim(TORSO_ATTACK2 as i32);
    } else {
        PM_StartTorsoAnim(TORSO_ATTACK as i32);
    }
    (*(*pm).ps).weaponstate = WEAPON_FIRING as i32;
    // check for out of ammo
    if (*(*pm).ps).ammo[(*(*pm).ps).weapon as usize] == 0 {
        PM_AddEvent(EV_NOAMMO as i32);
        (*(*pm).ps).weaponTime += 500 as i32;
        return;
    }
    // take an ammo away if not infinite
    if (*(*pm).ps).ammo[(*(*pm).ps).weapon as usize] != -(1 as i32) {
        (*(*pm).ps).ammo[(*(*pm).ps).weapon as usize] -= 1
    }
    // fire weapon
    PM_AddEvent(EV_FIRE_WEAPON as i32);
    match (*(*pm).ps).weapon {
        6 => addTime = 50 as i32,
        3 => addTime = 1000 as i32,
        2 => addTime = 100 as i32,
        4 => addTime = 800 as i32,
        5 => addTime = 800 as i32,
        8 => addTime = 100 as i32,
        7 => addTime = 1500 as i32,
        9 => addTime = 200 as i32,
        10 => addTime = 400 as i32,
        1 | _ => addTime = 400 as i32,
    }
    if (*(*pm).ps).powerups[PW_HASTE as i32 as usize] != 0 {
        addTime = (addTime as f64 / 1.3f64) as i32
    }
    (*(*pm).ps).weaponTime += addTime;
}
/*
================
PM_Animate
================
*/

unsafe extern "C" fn PM_Animate() {
    if (*pm).cmd.buttons & 8 as i32 != 0 {
        if (*(*pm).ps).torsoTimer == 0 as i32 {
            PM_StartTorsoAnim(TORSO_GESTURE as i32);
            (*(*pm).ps).torsoTimer = 34 as i32 * 66 as i32 + 50 as i32;
            PM_AddEvent(EV_TAUNT as i32);
        }
    };
}
/*
================
PM_DropTimers
================
*/

unsafe extern "C" fn PM_DropTimers() {
    // drop misc timing counter
    if (*(*pm).ps).pm_time != 0 {
        if pml.msec >= (*(*pm).ps).pm_time {
            (*(*pm).ps).pm_flags &= !(256 as i32 | 32 as i32 | 64 as i32);
            (*(*pm).ps).pm_time = 0 as i32
        } else {
            (*(*pm).ps).pm_time -= pml.msec
        }
    }
    // drop animation counter
    if (*(*pm).ps).legsTimer > 0 as i32 {
        (*(*pm).ps).legsTimer -= pml.msec;
        if (*(*pm).ps).legsTimer < 0 as i32 {
            (*(*pm).ps).legsTimer = 0 as i32
        }
    }
    if (*(*pm).ps).torsoTimer > 0 as i32 {
        (*(*pm).ps).torsoTimer -= pml.msec;
        if (*(*pm).ps).torsoTimer < 0 as i32 {
            (*(*pm).ps).torsoTimer = 0 as i32
        }
    };
}
// if a full pmove isn't done on the client, you can just update the angles
/*
================
PM_UpdateViewAngles

This can be used as another entry point when only the viewangles
are being updated instead of a full move
================
*/
#[no_mangle]

pub unsafe extern "C" fn PM_UpdateViewAngles(
    mut ps: *mut playerState_t,
    mut cmd: *const usercmd_t,
) {
    let mut temp: i16 = 0;
    let mut i: i32 = 0;
    if (*ps).pm_type == PM_INTERMISSION as i32 || (*ps).pm_type == PM_SPINTERMISSION as i32 {
        return;
        // no view changes at all
    }
    if (*ps).pm_type != PM_SPECTATOR as i32 && (*ps).stats[STAT_HEALTH as i32 as usize] <= 0 as i32
    {
        return;
        // no view changes at all
    }
    // circularly clamp the angles with deltas
    i = 0 as i32;
    while i < 3 as i32 {
        temp = ((*cmd).angles[i as usize] + (*ps).delta_angles[i as usize]) as i16;
        if i == 0 as i32 {
            // don't let the player look up or down more than 90 degrees
            if temp as i32 > 16000 as i32 {
                (*ps).delta_angles[i as usize] = 16000 as i32 - (*cmd).angles[i as usize];
                temp = 16000 as i32 as i16
            } else if (temp as i32) < -(16000 as i32) {
                (*ps).delta_angles[i as usize] = -(16000 as i32) - (*cmd).angles[i as usize];
                temp = -(16000 as i32) as i16
            }
        }
        (*ps).viewangles[i as usize] =
            (temp as i32 as f64 * (360.0f64 / 65536 as i32 as f64)) as vec_t;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn PmoveSingle(mut pmove: *mut pmove_t) {
    pm = pmove;
    // this counter lets us debug movement problems with a journal
    // by setting a conditional breakpoint fot the previous frame
    c_pmove += 1;
    // clear results
    (*pm).numtouch = 0 as i32;
    (*pm).watertype = 0 as i32;
    (*pm).waterlevel = 0 as i32;
    if (*(*pm).ps).stats[STAT_HEALTH as i32 as usize] <= 0 as i32 {
        (*pm).tracemask &= !(0x2000000 as i32)
        // corpses can fly through bodies
    }
    // make sure walking button is clear if they are running, to avoid
    // proxy no-footsteps cheats
    if libc::abs((*pm).cmd.forwardmove as i32) > 64 as i32
        || libc::abs((*pm).cmd.rightmove as i32) > 64 as i32
    {
        (*pm).cmd.buttons &= !(16 as i32)
    }
    // set the talk balloon flag
    if (*pm).cmd.buttons & 2 as i32 != 0 {
        (*(*pm).ps).eFlags |= 0x1000 as i32
    } else {
        (*(*pm).ps).eFlags &= !(0x1000 as i32)
    }
    // set the firing flag for continuous beam weapons
    if (*(*pm).ps).pm_flags & 512 as i32 == 0
        && (*(*pm).ps).pm_type != PM_INTERMISSION as i32
        && (*(*pm).ps).pm_type != PM_NOCLIP as i32
        && (*pm).cmd.buttons & 1 as i32 != 0
        && (*(*pm).ps).ammo[(*(*pm).ps).weapon as usize] != 0
    {
        (*(*pm).ps).eFlags |= 0x100 as i32
    } else {
        (*(*pm).ps).eFlags &= !(0x100 as i32)
    }
    // clear the respawned flag if attack and use are cleared
    if (*(*pm).ps).stats[STAT_HEALTH as i32 as usize] > 0 as i32
        && (*pm).cmd.buttons & (1 as i32 | 4 as i32) == 0
    {
        (*(*pm).ps).pm_flags &= !(512 as i32)
    }
    // if talk button is down, dissallow all other input
    // this is to prevent any possible intercept proxy from
    // adding fake talk balloons
    if (*pmove).cmd.buttons & 2 as i32 != 0 {
        // keep the talk button set tho for when the cmd.serverTime > 66 msec
        // and the same cmd is used multiple times in Pmove
        (*pmove).cmd.buttons = 2 as i32;
        (*pmove).cmd.forwardmove = 0 as i32 as i8;
        (*pmove).cmd.rightmove = 0 as i32 as i8;
        (*pmove).cmd.upmove = 0 as i32 as i8
    }
    // clear all pmove local vars
    crate::stdlib::memset(
        &mut pml as *mut pml_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<pml_t>() as usize,
    );
    // determine the time
    pml.msec = (*pmove).cmd.serverTime - (*(*pm).ps).commandTime;
    if pml.msec < 1 as i32 {
        pml.msec = 1 as i32
    } else if pml.msec > 200 as i32 {
        pml.msec = 200 as i32
    }
    (*(*pm).ps).commandTime = (*pmove).cmd.serverTime;
    // save old org in case we get stuck
    pml.previous_origin[0 as i32 as usize] = (*(*pm).ps).origin[0 as i32 as usize];
    pml.previous_origin[1 as i32 as usize] = (*(*pm).ps).origin[1 as i32 as usize];
    pml.previous_origin[2 as i32 as usize] = (*(*pm).ps).origin[2 as i32 as usize];
    // save old velocity for crashlanding
    pml.previous_velocity[0 as i32 as usize] = (*(*pm).ps).velocity[0 as i32 as usize];
    pml.previous_velocity[1 as i32 as usize] = (*(*pm).ps).velocity[1 as i32 as usize];
    pml.previous_velocity[2 as i32 as usize] = (*(*pm).ps).velocity[2 as i32 as usize];
    pml.frametime = (pml.msec as f64 * 0.001f64) as f32;
    // update the viewangles
    PM_UpdateViewAngles((*pm).ps, &mut (*pm).cmd);
    AngleVectors(
        (*(*pm).ps).viewangles.as_mut_ptr() as *const vec_t,
        pml.forward.as_mut_ptr(),
        pml.right.as_mut_ptr(),
        pml.up.as_mut_ptr(),
    );
    if ((*pm).cmd.upmove as i32) < 10 as i32 {
        // not holding jump
        (*(*pm).ps).pm_flags &= !(2 as i32)
    }
    // decide if backpedaling animations should be used
    if ((*pm).cmd.forwardmove as i32) < 0 as i32 {
        (*(*pm).ps).pm_flags |= 16 as i32
    } else if (*pm).cmd.forwardmove as i32 > 0 as i32
        || (*pm).cmd.forwardmove as i32 == 0 as i32 && (*pm).cmd.rightmove as i32 != 0
    {
        (*(*pm).ps).pm_flags &= !(16 as i32)
    }
    if (*(*pm).ps).pm_type >= PM_DEAD as i32 {
        (*pm).cmd.forwardmove = 0 as i32 as i8;
        (*pm).cmd.rightmove = 0 as i32 as i8;
        (*pm).cmd.upmove = 0 as i32 as i8
    }
    if (*(*pm).ps).pm_type == PM_SPECTATOR as i32 {
        PM_CheckDuck();
        PM_FlyMove();
        PM_DropTimers();
        return;
    }
    if (*(*pm).ps).pm_type == PM_NOCLIP as i32 {
        PM_NoclipMove();
        PM_DropTimers();
        return;
    }
    if (*(*pm).ps).pm_type == PM_FREEZE as i32 {
        return;
        // no movement at all
    }
    if (*(*pm).ps).pm_type == PM_INTERMISSION as i32
        || (*(*pm).ps).pm_type == PM_SPINTERMISSION as i32
    {
        return;
        // no movement at all
    }
    // set watertype, and waterlevel
    PM_SetWaterLevel();
    pml.previous_waterlevel = (*pmove).waterlevel;
    // set mins, maxs, and viewheight
    PM_CheckDuck();
    // set groundentity
    PM_GroundTrace();
    if (*(*pm).ps).pm_type == PM_DEAD as i32 {
        PM_DeadMove();
    }
    PM_DropTimers();
    if (*(*pm).ps).powerups[PW_FLIGHT as i32 as usize] != 0 {
        // flight powerup doesn't allow jump and has different friction
        PM_FlyMove();
    } else if (*(*pm).ps).pm_flags & 2048 as i32 != 0 {
        PM_GrappleMove();
        // We can wiggle a bit
        PM_AirMove();
    } else if (*(*pm).ps).pm_flags & 256 as i32 != 0 {
        PM_WaterJumpMove();
    } else if (*pm).waterlevel > 1 as i32 {
        // swimming
        PM_WaterMove();
    } else if pml.walking as u64 != 0 {
        // walking on ground
        PM_WalkMove();
    } else {
        // airborne
        PM_AirMove();
    }
    PM_Animate();
    // set groundentity, watertype, and waterlevel
    PM_GroundTrace();
    PM_SetWaterLevel();
    // weapons
    PM_Weapon();
    // torso animation
    PM_TorsoAnimation();
    // footstep events / legs animations
    PM_Footsteps();
    // entering / leaving water splashes
    PM_WaterEvents();
    // snap some parts of playerstate to save network bandwidth
    trap_SnapVector((*(*pm).ps).velocity.as_mut_ptr());
}
/*
================
Pmove

Can be called by either the server or the client
================
*/
#[no_mangle]

pub unsafe extern "C" fn Pmove(mut pmove: *mut pmove_t) {
    let mut finalTime: i32 = 0;
    finalTime = (*pmove).cmd.serverTime;
    if finalTime < (*(*pmove).ps).commandTime {
        return;
        // should not happen
    }
    if finalTime > (*(*pmove).ps).commandTime + 1000 as i32 {
        (*(*pmove).ps).commandTime = finalTime - 1000 as i32
    }
    (*(*pmove).ps).pmove_framecount =
        (*(*pmove).ps).pmove_framecount + 1 as i32 & ((1 as i32) << 6 as i32) - 1 as i32;
    // chop the move up if it is too long, to prevent framerate
    // dependent behavior
    while (*(*pmove).ps).commandTime != finalTime {
        let mut msec: i32 = 0;
        msec = finalTime - (*(*pmove).ps).commandTime;
        if (*pmove).pmove_fixed != 0 {
            if msec > (*pmove).pmove_msec {
                msec = (*pmove).pmove_msec
            }
        } else if msec > 66 as i32 {
            msec = 66 as i32
        }
        (*pmove).cmd.serverTime = (*(*pmove).ps).commandTime + msec;
        PmoveSingle(pmove);
        if (*(*pmove).ps).pm_flags & 2 as i32 != 0 {
            (*pmove).cmd.upmove = 20 as i32 as i8
        }
    }
    //PM_CheckStuck();
}

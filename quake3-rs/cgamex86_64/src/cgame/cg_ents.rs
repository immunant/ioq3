use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorInverse(mut v: *mut crate::src::qcommon::q_shared::vec_t) {
        *v.offset(0 as i32 as isize) = -*v.offset(0 as i32 as isize);
        *v.offset(1 as i32 as isize) = -*v.offset(1 as i32 as isize);
        *v.offset(2 as i32 as isize) = -*v.offset(2 as i32 as isize);
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

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
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
pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::itemInfo_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::cg_local_h::weaponInfo_s;
pub use crate::cg_local_h::weaponInfo_t;
pub use crate::cg_local_h::FOOTSTEP_BOOT;
pub use crate::cg_local_h::FOOTSTEP_ENERGY;
pub use crate::cg_local_h::FOOTSTEP_FLESH;
pub use crate::cg_local_h::FOOTSTEP_MECH;
pub use crate::cg_local_h::FOOTSTEP_METAL;
pub use crate::cg_local_h::FOOTSTEP_NORMAL;
pub use crate::cg_local_h::FOOTSTEP_SPLASH;
pub use crate::cg_local_h::FOOTSTEP_TOTAL;
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::cgame::cg_ents::q_shared_h::CrossProduct;
pub use crate::src::cgame::cg_ents::q_shared_h::VectorInverse;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_entities;
pub use crate::src::cgame::cg_main::cg_items;
pub use crate::src::cgame::cg_main::cg_simpleItems;
pub use crate::src::cgame::cg_main::cg_smoothClients;
pub use crate::src::cgame::cg_main::cg_weapons;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_Error;
pub use crate::src::cgame::cg_players::CG_AddRefEntityWithPowerups;
pub use crate::src::cgame::cg_players::CG_Player;
pub use crate::src::cgame::cg_syscalls::trap_R_AddLightToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_LerpTag;
pub use crate::src::cgame::cg_syscalls::trap_S_AddLoopingSound;
pub use crate::src::cgame::cg_syscalls::trap_S_AddRealLoopingSound;
pub use crate::src::cgame::cg_syscalls::trap_S_StartSound;
pub use crate::src::cgame::cg_syscalls::trap_S_UpdateEntityPosition;
pub use crate::src::cgame::cg_weapons::CG_GrappleTrail;
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::game::bg_misc::bg_numItems;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectory;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectoryDelta;
pub use crate::src::game::bg_misc::BG_PlayerStateToEntityState;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::AnglesToAxis;
pub use crate::src::qcommon::q_math::AxisClear;
pub use crate::src::qcommon::q_math::AxisCopy;
pub use crate::src::qcommon::q_math::ByteToDir;
pub use crate::src::qcommon::q_math::LerpAngle;
pub use crate::src::qcommon::q_math::MatrixMultiply;
pub use crate::src::qcommon::q_math::PerpendicularVector;
pub use crate::src::qcommon::q_math::RotateAroundDirection;
pub use crate::src::qcommon::q_math::VectorNormalize2;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::orientation_t;
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
// cg_ents.c -- present snapshot entities, happens every single frame
/*
======================
CG_PositionEntityOnTag

Modifies the entities position and axis by the given
tag location
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_PositionEntityOnTag(
    mut entity: *mut crate::tr_types_h::refEntity_t,
    mut parent: *const crate::tr_types_h::refEntity_t,
    mut parentModel: crate::src::qcommon::q_shared::qhandle_t,
    mut tagName: *mut libc::c_char,
) {
    let mut i: i32 = 0;
    let mut lerped: crate::src::qcommon::q_shared::orientation_t =
        crate::src::qcommon::q_shared::orientation_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
        };
    // lerp the tag
    crate::src::cgame::cg_syscalls::trap_R_LerpTag(
        &mut lerped as *mut _ as *mut crate::src::qcommon::q_shared::orientation_t,
        parentModel,
        (*parent).oldframe,
        (*parent).frame,
        (1.0f64 - (*parent).backlerp as f64) as f32,
        tagName,
    );
    // FIXME: allow origin offsets along tag?
    (*entity).origin[0 as i32 as usize] = (*parent).origin[0 as i32 as usize];
    (*entity).origin[1 as i32 as usize] = (*parent).origin[1 as i32 as usize];
    (*entity).origin[2 as i32 as usize] = (*parent).origin[2 as i32 as usize];
    i = 0 as i32;
    while i < 3 as i32 {
        (*entity).origin[0 as i32 as usize] = (*entity).origin[0 as i32 as usize]
            + (*parent).axis[i as usize][0 as i32 as usize] * lerped.origin[i as usize];
        (*entity).origin[1 as i32 as usize] = (*entity).origin[1 as i32 as usize]
            + (*parent).axis[i as usize][1 as i32 as usize] * lerped.origin[i as usize];
        (*entity).origin[2 as i32 as usize] = (*entity).origin[2 as i32 as usize]
            + (*parent).axis[i as usize][2 as i32 as usize] * lerped.origin[i as usize];
        i += 1
    }
    // had to cast away the const to avoid compiler problems...
    crate::src::qcommon::q_math::MatrixMultiply(
        lerped.axis.as_mut_ptr(),
        (*(parent as *mut crate::tr_types_h::refEntity_t))
            .axis
            .as_mut_ptr(),
        (*entity).axis.as_mut_ptr(),
    );
    (*entity).backlerp = (*parent).backlerp;
}
/*
======================
CG_PositionRotatedEntityOnTag

Modifies the entities position and axis by the given
tag location
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_PositionRotatedEntityOnTag(
    mut entity: *mut crate::tr_types_h::refEntity_t,
    mut parent: *const crate::tr_types_h::refEntity_t,
    mut parentModel: crate::src::qcommon::q_shared::qhandle_t,
    mut tagName: *mut libc::c_char,
) {
    let mut i: i32 = 0;
    let mut lerped: crate::src::qcommon::q_shared::orientation_t =
        crate::src::qcommon::q_shared::orientation_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
        };
    let mut tempAxis: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    //AxisClear( entity->axis );
    // lerp the tag
    crate::src::cgame::cg_syscalls::trap_R_LerpTag(
        &mut lerped as *mut _ as *mut crate::src::qcommon::q_shared::orientation_t,
        parentModel,
        (*parent).oldframe,
        (*parent).frame,
        (1.0f64 - (*parent).backlerp as f64) as f32,
        tagName,
    );
    // FIXME: allow origin offsets along tag?
    (*entity).origin[0 as i32 as usize] = (*parent).origin[0 as i32 as usize];
    (*entity).origin[1 as i32 as usize] = (*parent).origin[1 as i32 as usize];
    (*entity).origin[2 as i32 as usize] = (*parent).origin[2 as i32 as usize];
    i = 0 as i32;
    while i < 3 as i32 {
        (*entity).origin[0 as i32 as usize] = (*entity).origin[0 as i32 as usize]
            + (*parent).axis[i as usize][0 as i32 as usize] * lerped.origin[i as usize];
        (*entity).origin[1 as i32 as usize] = (*entity).origin[1 as i32 as usize]
            + (*parent).axis[i as usize][1 as i32 as usize] * lerped.origin[i as usize];
        (*entity).origin[2 as i32 as usize] = (*entity).origin[2 as i32 as usize]
            + (*parent).axis[i as usize][2 as i32 as usize] * lerped.origin[i as usize];
        i += 1
    }
    // had to cast away the const to avoid compiler problems...
    crate::src::qcommon::q_math::MatrixMultiply(
        (*entity).axis.as_mut_ptr(),
        lerped.axis.as_mut_ptr(),
        tempAxis.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::MatrixMultiply(
        tempAxis.as_mut_ptr(),
        (*(parent as *mut crate::tr_types_h::refEntity_t))
            .axis
            .as_mut_ptr(),
        (*entity).axis.as_mut_ptr(),
    );
}
/*
==========================================================================

FUNCTIONS CALLED EACH FRAME

==========================================================================
*/
/*
======================
CG_SetEntitySoundPosition

Also called by event processing code
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_SetEntitySoundPosition(mut cent: *mut crate::cg_local_h::centity_t) {
    if (*cent).currentState.solid == 0xffffff as i32 {
        let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut v: *mut f32 = 0 as *mut f32;
        v = crate::src::cgame::cg_main::cgs.inlineModelMidpoints
            [(*cent).currentState.modelindex as usize]
            .as_mut_ptr();
        origin[0 as i32 as usize] =
            (*cent).lerpOrigin[0 as i32 as usize] + *v.offset(0 as i32 as isize);
        origin[1 as i32 as usize] =
            (*cent).lerpOrigin[1 as i32 as usize] + *v.offset(1 as i32 as isize);
        origin[2 as i32 as usize] =
            (*cent).lerpOrigin[2 as i32 as usize] + *v.offset(2 as i32 as isize);
        crate::src::cgame::cg_syscalls::trap_S_UpdateEntityPosition(
            (*cent).currentState.number,
            origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        );
    } else {
        crate::src::cgame::cg_syscalls::trap_S_UpdateEntityPosition(
            (*cent).currentState.number,
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        );
    };
}
/*
==================
CG_EntityEffects

Add continuous entity effects, like local entity emission and lighting
==================
*/

unsafe extern "C" fn CG_EntityEffects(mut cent: *mut crate::cg_local_h::centity_t) {
    // update sound origins
    CG_SetEntitySoundPosition(cent);
    // add loop sound
    if (*cent).currentState.loopSound != 0 {
        if (*cent).currentState.eType != crate::bg_public_h::ET_SPEAKER as i32 {
            crate::src::cgame::cg_syscalls::trap_S_AddLoopingSound(
                (*cent).currentState.number,
                (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::cgame::cg_main::cgs.gameSounds[(*cent).currentState.loopSound as usize],
            );
        } else {
            crate::src::cgame::cg_syscalls::trap_S_AddRealLoopingSound(
                (*cent).currentState.number,
                (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::cgame::cg_main::cgs.gameSounds[(*cent).currentState.loopSound as usize],
            );
        }
    }
    // constant light glow
    if (*cent).currentState.constantLight != 0 {
        let mut cl: i32 = 0;
        let mut i: f32 = 0.;
        let mut r: f32 = 0.;
        let mut g: f32 = 0.;
        let mut b: f32 = 0.;
        cl = (*cent).currentState.constantLight;
        r = ((cl & 0xff as i32) as f32 as f64 / 255.0f64)
            as f32;
        g = ((cl >> 8 as i32 & 0xff as i32) as f32 as f64
            / 255.0f64) as f32;
        b = ((cl >> 16 as i32 & 0xff as i32) as f32 as f64
            / 255.0f64) as f32;
        i = ((cl >> 24 as i32 & 0xff as i32) as f32 as f64
            * 4.0f64) as f32;
        crate::src::cgame::cg_syscalls::trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            i,
            r,
            g,
            b,
        );
    };
}
/*
==================
CG_General
==================
*/

unsafe extern "C" fn CG_General(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut s1: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    s1 = &mut (*cent).currentState;
    // if set to invisible, skip
    if (*s1).modelindex == 0 {
        return;
    }
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
    );
    // set frame
    ent.frame = (*s1).frame;
    ent.oldframe = ent.frame;
    ent.backlerp = 0 as i32 as f32;
    ent.origin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
    ent.origin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
    ent.origin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
    ent.oldorigin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
    ent.oldorigin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
    ent.oldorigin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
    ent.hModel = crate::src::cgame::cg_main::cgs.gameModels[(*s1).modelindex as usize];
    // player model
    if (*s1).number == (*crate::src::cgame::cg_main::cg.snap).ps.clientNum {
        ent.renderfx |= 0x2 as i32
        // only draw from mirrors
    }
    // convert angles to axis
    crate::src::qcommon::q_math::AnglesToAxis(
        (*cent).lerpAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis.as_mut_ptr(),
    );
    // add to refresh list
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        &mut ent as *mut _ as *const crate::tr_types_h::refEntity_t,
    );
}
/*
==================
CG_Speaker

Speaker entities can automatically play sounds
==================
*/

unsafe extern "C" fn CG_Speaker(mut cent: *mut crate::cg_local_h::centity_t) {
    if (*cent).currentState.clientNum == 0 {
        // FIXME: use something other than clientNum...
        return;
        // not auto triggering
    }
    if crate::src::cgame::cg_main::cg.time < (*cent).miscTime {
        return;
    }
    crate::src::cgame::cg_syscalls::trap_S_StartSound(
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        (*cent).currentState.number,
        crate::src::qcommon::q_shared::CHAN_ITEM as i32,
        crate::src::cgame::cg_main::cgs.gameSounds[(*cent).currentState.eventParm as usize],
    );
    //	ent->s.frame = ent->wait * 10;
    //	ent->s.clientNum = ent->random * 10;
    (*cent).miscTime = ((crate::src::cgame::cg_main::cg.time
        + (*cent).currentState.frame * 100 as i32)
        as f64
        + ((*cent).currentState.clientNum * 100 as i32) as f64
            * (2.0f64
                * (((::libc::rand() & 0x7fff as i32) as f32
                    / 0x7fff as i32 as f32)
                    as f64
                    - 0.5f64))) as i32;
}
/*
==================
CG_Item
==================
*/

unsafe extern "C" fn CG_Item(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut es: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    let mut msec: i32 = 0;
    let mut frac: f32 = 0.;
    let mut scale: f32 = 0.;
    let mut wi: *mut crate::cg_local_h::weaponInfo_t = 0 as *mut crate::cg_local_h::weaponInfo_t;
    es = &mut (*cent).currentState;
    if (*es).modelindex >= crate::src::game::bg_misc::bg_numItems {
        crate::src::cgame::cg_main::CG_Error(
            b"Bad item index %i on entity\x00" as *const u8 as *const libc::c_char,
            (*es).modelindex,
        );
    }
    // if set to invisible, skip
    if (*es).modelindex == 0 || (*es).eFlags & 0x80 as i32 != 0 {
        return;
    }
    item = &mut *crate::src::game::bg_misc::bg_itemlist
        .as_mut_ptr()
        .offset((*es).modelindex as isize) as *mut crate::bg_public_h::gitem_t;
    if crate::src::cgame::cg_main::cg_simpleItems.integer != 0
        && (*item).giType as u32
            != crate::bg_public_h::IT_TEAM as i32 as u32
    {
        crate::stdlib::memset(
            &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
        );
        ent.reType = crate::tr_types_h::RT_SPRITE;
        ent.origin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
        ent.origin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
        ent.origin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
        ent.radius = 14 as i32 as f32;
        ent.customShader = crate::src::cgame::cg_main::cg_items[(*es).modelindex as usize].icon;
        ent.shaderRGBA[0 as i32 as usize] =
            255 as i32 as crate::src::qcommon::q_shared::byte;
        ent.shaderRGBA[1 as i32 as usize] =
            255 as i32 as crate::src::qcommon::q_shared::byte;
        ent.shaderRGBA[2 as i32 as usize] =
            255 as i32 as crate::src::qcommon::q_shared::byte;
        ent.shaderRGBA[3 as i32 as usize] =
            255 as i32 as crate::src::qcommon::q_shared::byte;
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
            &mut ent as *mut _ as *const crate::tr_types_h::refEntity_t,
        );
        return;
    }
    // items bob up and down continuously
    scale =
        (0.005f64 + (*cent).currentState.number as f64 * 0.00001f64) as f32;
    (*cent).lerpOrigin[2 as i32 as usize] = ((*cent).lerpOrigin[2 as i32 as usize]
        as f64
        + (4 as i32 as f64
            + crate::stdlib::cos(
                ((crate::src::cgame::cg_main::cg.time + 1000 as i32) as f32
                    * scale) as f64,
            ) * 4 as i32 as f64))
        as crate::src::qcommon::q_shared::vec_t;
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
    );
    // autorotate at one of two speeds
    if (*item).giType as u32
        == crate::bg_public_h::IT_HEALTH as i32 as u32
    {
        (*cent).lerpAngles[0 as i32 as usize] =
            crate::src::cgame::cg_main::cg.autoAnglesFast[0 as i32 as usize];
        (*cent).lerpAngles[1 as i32 as usize] =
            crate::src::cgame::cg_main::cg.autoAnglesFast[1 as i32 as usize];
        (*cent).lerpAngles[2 as i32 as usize] =
            crate::src::cgame::cg_main::cg.autoAnglesFast[2 as i32 as usize];
        crate::src::qcommon::q_math::AxisCopy(
            crate::src::cgame::cg_main::cg.autoAxisFast.as_mut_ptr(),
            ent.axis.as_mut_ptr(),
        );
    } else {
        (*cent).lerpAngles[0 as i32 as usize] =
            crate::src::cgame::cg_main::cg.autoAngles[0 as i32 as usize];
        (*cent).lerpAngles[1 as i32 as usize] =
            crate::src::cgame::cg_main::cg.autoAngles[1 as i32 as usize];
        (*cent).lerpAngles[2 as i32 as usize] =
            crate::src::cgame::cg_main::cg.autoAngles[2 as i32 as usize];
        crate::src::qcommon::q_math::AxisCopy(
            crate::src::cgame::cg_main::cg.autoAxis.as_mut_ptr(),
            ent.axis.as_mut_ptr(),
        );
    }
    wi = 0 as *mut crate::cg_local_h::weaponInfo_t;
    // the weapons have their origin where they attatch to player
    // models, so we need to offset them or they will rotate
    // eccentricly
    if (*item).giType as u32
        == crate::bg_public_h::IT_WEAPON as i32 as u32
    {
        wi = &mut *crate::src::cgame::cg_main::cg_weapons
            .as_mut_ptr()
            .offset((*item).giTag as isize) as *mut crate::cg_local_h::weaponInfo_t;
        (*cent).lerpOrigin[0 as i32 as usize] -= (*wi).weaponMidpoint
            [0 as i32 as usize]
            * ent.axis[0 as i32 as usize][0 as i32 as usize]
            + (*wi).weaponMidpoint[1 as i32 as usize]
                * ent.axis[1 as i32 as usize][0 as i32 as usize]
            + (*wi).weaponMidpoint[2 as i32 as usize]
                * ent.axis[2 as i32 as usize][0 as i32 as usize];
        (*cent).lerpOrigin[1 as i32 as usize] -= (*wi).weaponMidpoint
            [0 as i32 as usize]
            * ent.axis[0 as i32 as usize][1 as i32 as usize]
            + (*wi).weaponMidpoint[1 as i32 as usize]
                * ent.axis[1 as i32 as usize][1 as i32 as usize]
            + (*wi).weaponMidpoint[2 as i32 as usize]
                * ent.axis[2 as i32 as usize][1 as i32 as usize];
        (*cent).lerpOrigin[2 as i32 as usize] -= (*wi).weaponMidpoint
            [0 as i32 as usize]
            * ent.axis[0 as i32 as usize][2 as i32 as usize]
            + (*wi).weaponMidpoint[1 as i32 as usize]
                * ent.axis[1 as i32 as usize][2 as i32 as usize]
            + (*wi).weaponMidpoint[2 as i32 as usize]
                * ent.axis[2 as i32 as usize][2 as i32 as usize];
        (*cent).lerpOrigin[2 as i32 as usize] += 8 as i32 as f32
        // an extra height boost
    }
    if (*item).giType as u32
        == crate::bg_public_h::IT_WEAPON as i32 as u32
        && (*item).giTag == crate::bg_public_h::WP_RAILGUN as i32
    {
        let mut ci: *mut crate::cg_local_h::clientInfo_t = &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset((*crate::src::cgame::cg_main::cg.snap).ps.clientNum as isize)
            as *mut crate::cg_local_h::clientInfo_t;
        ent.shaderRGBA[0 as i32 as usize] = (*ci).c1RGBA[0 as i32 as usize];
        ent.shaderRGBA[1 as i32 as usize] = (*ci).c1RGBA[1 as i32 as usize];
        ent.shaderRGBA[2 as i32 as usize] = (*ci).c1RGBA[2 as i32 as usize];
        ent.shaderRGBA[3 as i32 as usize] = (*ci).c1RGBA[3 as i32 as usize]
    }
    ent.hModel = crate::src::cgame::cg_main::cg_items[(*es).modelindex as usize].models
        [0 as i32 as usize];
    ent.origin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
    ent.origin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
    ent.origin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
    ent.oldorigin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
    ent.oldorigin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
    ent.oldorigin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
    ent.nonNormalizedAxes = crate::src::qcommon::q_shared::qfalse;
    // if just respawned, slowly scale up
    msec = crate::src::cgame::cg_main::cg.time - (*cent).miscTime;
    if msec >= 0 as i32 && msec < 1000 as i32 {
        frac = msec as f32 / 1000 as i32 as f32;
        ent.axis[0 as i32 as usize][0 as i32 as usize] =
            ent.axis[0 as i32 as usize][0 as i32 as usize] * frac;
        ent.axis[0 as i32 as usize][1 as i32 as usize] =
            ent.axis[0 as i32 as usize][1 as i32 as usize] * frac;
        ent.axis[0 as i32 as usize][2 as i32 as usize] =
            ent.axis[0 as i32 as usize][2 as i32 as usize] * frac;
        ent.axis[1 as i32 as usize][0 as i32 as usize] =
            ent.axis[1 as i32 as usize][0 as i32 as usize] * frac;
        ent.axis[1 as i32 as usize][1 as i32 as usize] =
            ent.axis[1 as i32 as usize][1 as i32 as usize] * frac;
        ent.axis[1 as i32 as usize][2 as i32 as usize] =
            ent.axis[1 as i32 as usize][2 as i32 as usize] * frac;
        ent.axis[2 as i32 as usize][0 as i32 as usize] =
            ent.axis[2 as i32 as usize][0 as i32 as usize] * frac;
        ent.axis[2 as i32 as usize][1 as i32 as usize] =
            ent.axis[2 as i32 as usize][1 as i32 as usize] * frac;
        ent.axis[2 as i32 as usize][2 as i32 as usize] =
            ent.axis[2 as i32 as usize][2 as i32 as usize] * frac;
        ent.nonNormalizedAxes = crate::src::qcommon::q_shared::qtrue
    } else {
        frac = 1.0f64 as f32
    }
    // items without glow textures need to keep a minimum light value
    // so they are always visible
    if (*item).giType as u32
        == crate::bg_public_h::IT_WEAPON as i32 as u32
        || (*item).giType as u32
            == crate::bg_public_h::IT_ARMOR as i32 as u32
    {
        ent.renderfx |= 0x1 as i32
    }
    // increase the size of the weapons when they are presented as items
    if (*item).giType as u32
        == crate::bg_public_h::IT_WEAPON as i32 as u32
    {
        ent.axis[0 as i32 as usize][0 as i32 as usize] =
            (ent.axis[0 as i32 as usize][0 as i32 as usize] as f64
                * 1.5f64) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[0 as i32 as usize][1 as i32 as usize] =
            (ent.axis[0 as i32 as usize][1 as i32 as usize] as f64
                * 1.5f64) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[0 as i32 as usize][2 as i32 as usize] =
            (ent.axis[0 as i32 as usize][2 as i32 as usize] as f64
                * 1.5f64) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[1 as i32 as usize][0 as i32 as usize] =
            (ent.axis[1 as i32 as usize][0 as i32 as usize] as f64
                * 1.5f64) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[1 as i32 as usize][1 as i32 as usize] =
            (ent.axis[1 as i32 as usize][1 as i32 as usize] as f64
                * 1.5f64) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[1 as i32 as usize][2 as i32 as usize] =
            (ent.axis[1 as i32 as usize][2 as i32 as usize] as f64
                * 1.5f64) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[2 as i32 as usize][0 as i32 as usize] =
            (ent.axis[2 as i32 as usize][0 as i32 as usize] as f64
                * 1.5f64) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[2 as i32 as usize][1 as i32 as usize] =
            (ent.axis[2 as i32 as usize][1 as i32 as usize] as f64
                * 1.5f64) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[2 as i32 as usize][2 as i32 as usize] =
            (ent.axis[2 as i32 as usize][2 as i32 as usize] as f64
                * 1.5f64) as crate::src::qcommon::q_shared::vec_t;
        ent.nonNormalizedAxes = crate::src::qcommon::q_shared::qtrue
    }
    // add to refresh list
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        &mut ent as *mut _ as *const crate::tr_types_h::refEntity_t,
    );
    if (*item).giType as u32
        == crate::bg_public_h::IT_WEAPON as i32 as u32
        && !wi.is_null()
        && (*wi).barrelModel != 0
    {
        let mut barrel: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
            reType: crate::tr_types_h::RT_MODEL,
            renderfx: 0,
            hModel: 0,
            lightingOrigin: [0.; 3],
            shadowPlane: 0.,
            axis: [[0.; 3]; 3],
            nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
            origin: [0.; 3],
            frame: 0,
            oldorigin: [0.; 3],
            oldframe: 0,
            backlerp: 0.,
            skinNum: 0,
            customSkin: 0,
            customShader: 0,
            shaderRGBA: [0; 4],
            shaderTexCoord: [0.; 2],
            shaderTime: 0.,
            radius: 0.,
            rotation: 0.,
        };
        let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        crate::stdlib::memset(
            &mut barrel as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
        );
        barrel.hModel = (*wi).barrelModel;
        barrel.lightingOrigin[0 as i32 as usize] =
            ent.lightingOrigin[0 as i32 as usize];
        barrel.lightingOrigin[1 as i32 as usize] =
            ent.lightingOrigin[1 as i32 as usize];
        barrel.lightingOrigin[2 as i32 as usize] =
            ent.lightingOrigin[2 as i32 as usize];
        barrel.shadowPlane = ent.shadowPlane;
        barrel.renderfx = ent.renderfx;
        angles[1 as i32 as usize] =
            0 as i32 as crate::src::qcommon::q_shared::vec_t;
        angles[0 as i32 as usize] =
            0 as i32 as crate::src::qcommon::q_shared::vec_t;
        angles[2 as i32 as usize] =
            0 as i32 as crate::src::qcommon::q_shared::vec_t;
        crate::src::qcommon::q_math::AnglesToAxis(
            angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            barrel.axis.as_mut_ptr(),
        );
        CG_PositionRotatedEntityOnTag(
            &mut barrel,
            &mut ent,
            (*wi).weaponModel,
            b"tag_barrel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        barrel.nonNormalizedAxes = ent.nonNormalizedAxes;
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
            &mut barrel as *mut _ as *const crate::tr_types_h::refEntity_t,
        );
    }
    // accompanying rings / spheres for powerups
    if crate::src::cgame::cg_main::cg_simpleItems.integer == 0 {
        let mut spinAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        spinAngles[2 as i32 as usize] =
            0 as i32 as crate::src::qcommon::q_shared::vec_t;
        spinAngles[1 as i32 as usize] = spinAngles[2 as i32 as usize];
        spinAngles[0 as i32 as usize] = spinAngles[1 as i32 as usize];
        if (*item).giType as u32
            == crate::bg_public_h::IT_HEALTH as i32 as u32
            || (*item).giType as u32
                == crate::bg_public_h::IT_POWERUP as i32 as u32
        {
            ent.hModel = crate::src::cgame::cg_main::cg_items[(*es).modelindex as usize].models
                [1 as i32 as usize];
            if ent.hModel != 0 as i32 {
                if (*item).giType as u32
                    == crate::bg_public_h::IT_POWERUP as i32 as u32
                {
                    ent.origin[2 as i32 as usize] += 12 as i32 as f32;
                    spinAngles[1 as i32 as usize] =
                        ((crate::src::cgame::cg_main::cg.time & 1023 as i32)
                            * 360 as i32) as f32
                            / -1024.0f32
                }
                crate::src::qcommon::q_math::AnglesToAxis(
                    spinAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    ent.axis.as_mut_ptr(),
                );
                // scale up if respawning
                if frac as f64 != 1.0f64 {
                    ent.axis[0 as i32 as usize][0 as i32 as usize] =
                        ent.axis[0 as i32 as usize][0 as i32 as usize] * frac;
                    ent.axis[0 as i32 as usize][1 as i32 as usize] =
                        ent.axis[0 as i32 as usize][1 as i32 as usize] * frac;
                    ent.axis[0 as i32 as usize][2 as i32 as usize] =
                        ent.axis[0 as i32 as usize][2 as i32 as usize] * frac;
                    ent.axis[1 as i32 as usize][0 as i32 as usize] =
                        ent.axis[1 as i32 as usize][0 as i32 as usize] * frac;
                    ent.axis[1 as i32 as usize][1 as i32 as usize] =
                        ent.axis[1 as i32 as usize][1 as i32 as usize] * frac;
                    ent.axis[1 as i32 as usize][2 as i32 as usize] =
                        ent.axis[1 as i32 as usize][2 as i32 as usize] * frac;
                    ent.axis[2 as i32 as usize][0 as i32 as usize] =
                        ent.axis[2 as i32 as usize][0 as i32 as usize] * frac;
                    ent.axis[2 as i32 as usize][1 as i32 as usize] =
                        ent.axis[2 as i32 as usize][1 as i32 as usize] * frac;
                    ent.axis[2 as i32 as usize][2 as i32 as usize] =
                        ent.axis[2 as i32 as usize][2 as i32 as usize] * frac;
                    ent.nonNormalizedAxes = crate::src::qcommon::q_shared::qtrue
                }
                crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
                    &mut ent as *mut _ as *const crate::tr_types_h::refEntity_t,
                );
            }
        }
    };
}
//============================================================================
/*
===============
CG_Missile
===============
*/

unsafe extern "C" fn CG_Missile(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut s1: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut weapon: *const crate::cg_local_h::weaponInfo_t =
        0 as *const crate::cg_local_h::weaponInfo_t;
    //	int	col;
    s1 = &mut (*cent).currentState;
    if (*s1).weapon >= crate::bg_public_h::WP_NUM_WEAPONS as i32 {
        (*s1).weapon = 0 as i32
    }
    weapon = &mut *crate::src::cgame::cg_main::cg_weapons
        .as_mut_ptr()
        .offset((*s1).weapon as isize) as *mut crate::cg_local_h::weaponInfo_t;
    // calculate the axis
    (*cent).lerpAngles[0 as i32 as usize] = (*s1).angles[0 as i32 as usize];
    (*cent).lerpAngles[1 as i32 as usize] = (*s1).angles[1 as i32 as usize];
    (*cent).lerpAngles[2 as i32 as usize] = (*s1).angles[2 as i32 as usize];
    // add trails
    if (*weapon).missileTrailFunc.is_some() {
        (*weapon)
            .missileTrailFunc
            .expect("non-null function pointer")(cent, weapon);
    }
    /*
        if ( cent->currentState.modelindex == TEAM_RED ) {
            col = 1;
        }
        else if ( cent->currentState.modelindex == TEAM_BLUE ) {
            col = 2;
        }
        else {
            col = 0;
        }

        // add dynamic light
        if ( weapon->missileDlight ) {
            trap_R_AddLightToScene(cent->lerpOrigin, weapon->missileDlight,
                weapon->missileDlightColor[col][0], weapon->missileDlightColor[col][1], weapon->missileDlightColor[col][2] );
        }
    */
    // add dynamic light
    if (*weapon).missileDlight != 0. {
        crate::src::cgame::cg_syscalls::trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*weapon).missileDlight,
            (*weapon).missileDlightColor[0 as i32 as usize],
            (*weapon).missileDlightColor[1 as i32 as usize],
            (*weapon).missileDlightColor[2 as i32 as usize],
        );
    }
    // add missile sound
    if (*weapon).missileSound != 0 {
        let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        crate::src::game::bg_misc::BG_EvaluateTrajectoryDelta(
            &mut (*cent).currentState.pos as *mut _
                as *const crate::src::qcommon::q_shared::trajectory_t,
            crate::src::cgame::cg_main::cg.time,
            velocity.as_mut_ptr(),
        );
        crate::src::cgame::cg_syscalls::trap_S_AddLoopingSound(
            (*cent).currentState.number,
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            velocity.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*weapon).missileSound,
        );
    }
    // create the render entity
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
    ent.origin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
    ent.origin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
    ent.oldorigin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
    ent.oldorigin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
    ent.oldorigin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
    if (*cent).currentState.weapon == crate::bg_public_h::WP_PLASMAGUN as i32 {
        ent.reType = crate::tr_types_h::RT_SPRITE;
        ent.radius = 16 as i32 as f32;
        ent.rotation = 0 as i32 as f32;
        ent.customShader = crate::src::cgame::cg_main::cgs.media.plasmaBallShader;
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
            &mut ent as *mut _ as *const crate::tr_types_h::refEntity_t,
        );
        return;
    }
    // flicker between two skins
    ent.skinNum = crate::src::cgame::cg_main::cg.clientFrame & 1 as i32;
    ent.hModel = (*weapon).missileModel;
    ent.renderfx = (*weapon).missileRenderfx | 0x40 as i32;
    // convert direction of travel into axis
    if crate::src::qcommon::q_math::VectorNormalize2(
        (*s1).pos.trDelta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis[0 as i32 as usize].as_mut_ptr(),
    ) == 0 as i32 as f32
    {
        ent.axis[0 as i32 as usize][2 as i32 as usize] =
            1 as i32 as crate::src::qcommon::q_shared::vec_t
    }
    // spin as it moves
    if (*s1).pos.trType as u32
        != crate::src::qcommon::q_shared::TR_STATIONARY as i32 as u32
    {
        crate::src::qcommon::q_math::RotateAroundDirection(
            ent.axis.as_mut_ptr(),
            (crate::src::cgame::cg_main::cg.time / 4 as i32) as f32,
        );
    } else {
        crate::src::qcommon::q_math::RotateAroundDirection(
            ent.axis.as_mut_ptr(),
            (*s1).time as f32,
        );
    }
    // add to refresh list, possibly with quad glow
    crate::src::cgame::cg_players::CG_AddRefEntityWithPowerups(
        &mut ent as *mut _ as *mut crate::tr_types_h::refEntity_t,
        s1 as *mut crate::src::qcommon::q_shared::entityState_s,
        crate::bg_public_h::TEAM_FREE as i32,
    );
}
/*
===============
CG_Grapple

This is called when the grapple is sitting up against the wall
===============
*/

unsafe extern "C" fn CG_Grapple(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut s1: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut weapon: *const crate::cg_local_h::weaponInfo_t =
        0 as *const crate::cg_local_h::weaponInfo_t;
    s1 = &mut (*cent).currentState;
    if (*s1).weapon >= crate::bg_public_h::WP_NUM_WEAPONS as i32 {
        (*s1).weapon = 0 as i32
    }
    weapon = &mut *crate::src::cgame::cg_main::cg_weapons
        .as_mut_ptr()
        .offset((*s1).weapon as isize) as *mut crate::cg_local_h::weaponInfo_t;
    // calculate the axis
    (*cent).lerpAngles[0 as i32 as usize] = (*s1).angles[0 as i32 as usize];
    (*cent).lerpAngles[1 as i32 as usize] = (*s1).angles[1 as i32 as usize];
    (*cent).lerpAngles[2 as i32 as usize] = (*s1).angles[2 as i32 as usize];
    // FIXME add grapple pull sound here..?
    // Will draw cable if needed
    crate::src::cgame::cg_weapons::CG_GrappleTrail(
        cent as *mut crate::cg_local_h::centity_s,
        weapon as *const crate::cg_local_h::weaponInfo_s,
    );
    // create the render entity
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
    ent.origin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
    ent.origin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
    ent.oldorigin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
    ent.oldorigin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
    ent.oldorigin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
    // flicker between two skins
    ent.skinNum = crate::src::cgame::cg_main::cg.clientFrame & 1 as i32;
    ent.hModel = (*weapon).missileModel;
    ent.renderfx = (*weapon).missileRenderfx | 0x40 as i32;
    // convert direction of travel into axis
    if crate::src::qcommon::q_math::VectorNormalize2(
        (*s1).pos.trDelta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis[0 as i32 as usize].as_mut_ptr(),
    ) == 0 as i32 as f32
    {
        ent.axis[0 as i32 as usize][2 as i32 as usize] =
            1 as i32 as crate::src::qcommon::q_shared::vec_t
    }
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        &mut ent as *mut _ as *const crate::tr_types_h::refEntity_t,
    );
}
/*
===============
CG_Mover
===============
*/

unsafe extern "C" fn CG_Mover(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut s1: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    s1 = &mut (*cent).currentState;
    // create the render entity
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
    ent.origin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
    ent.origin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
    ent.oldorigin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
    ent.oldorigin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
    ent.oldorigin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
    crate::src::qcommon::q_math::AnglesToAxis(
        (*cent).lerpAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis.as_mut_ptr(),
    );
    ent.renderfx = 0x40 as i32;
    // flicker between two skins (FIXME?)
    ent.skinNum = crate::src::cgame::cg_main::cg.time >> 6 as i32 & 1 as i32;
    // get the model, either as a bmodel or a modelindex
    if (*s1).solid == 0xffffff as i32 {
        ent.hModel = crate::src::cgame::cg_main::cgs.inlineDrawModel[(*s1).modelindex as usize]
    } else {
        ent.hModel = crate::src::cgame::cg_main::cgs.gameModels[(*s1).modelindex as usize]
    }
    // add to refresh list
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        &mut ent as *mut _ as *const crate::tr_types_h::refEntity_t,
    );
    // add the secondary model
    if (*s1).modelindex2 != 0 {
        ent.skinNum = 0 as i32;
        ent.hModel = crate::src::cgame::cg_main::cgs.gameModels[(*s1).modelindex2 as usize];
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
            &mut ent as *mut _ as *const crate::tr_types_h::refEntity_t,
        );
    };
}
/*
===============
CG_Beam

Also called as an event
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_Beam(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut s1: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    s1 = &mut (*cent).currentState;
    // create the render entity
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0 as i32 as usize] = (*s1).pos.trBase[0 as i32 as usize];
    ent.origin[1 as i32 as usize] = (*s1).pos.trBase[1 as i32 as usize];
    ent.origin[2 as i32 as usize] = (*s1).pos.trBase[2 as i32 as usize];
    ent.oldorigin[0 as i32 as usize] = (*s1).origin2[0 as i32 as usize];
    ent.oldorigin[1 as i32 as usize] = (*s1).origin2[1 as i32 as usize];
    ent.oldorigin[2 as i32 as usize] = (*s1).origin2[2 as i32 as usize];
    crate::src::qcommon::q_math::AxisClear(ent.axis.as_mut_ptr());
    ent.reType = crate::tr_types_h::RT_BEAM;
    ent.renderfx = 0x40 as i32;
    // add to refresh list
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        &mut ent as *mut _ as *const crate::tr_types_h::refEntity_t,
    );
}
/*
===============
CG_Portal
===============
*/

unsafe extern "C" fn CG_Portal(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut s1: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    s1 = &mut (*cent).currentState;
    // create the render entity
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
    ent.origin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
    ent.origin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
    ent.oldorigin[0 as i32 as usize] = (*s1).origin2[0 as i32 as usize];
    ent.oldorigin[1 as i32 as usize] = (*s1).origin2[1 as i32 as usize];
    ent.oldorigin[2 as i32 as usize] = (*s1).origin2[2 as i32 as usize];
    crate::src::qcommon::q_math::ByteToDir(
        (*s1).eventParm,
        ent.axis[0 as i32 as usize].as_mut_ptr(),
    );
    crate::src::qcommon::q_math::PerpendicularVector(
        ent.axis[1 as i32 as usize].as_mut_ptr(),
        ent.axis[0 as i32 as usize].as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
    );
    // negating this tends to get the directions like they want
    // we really should have a camera roll value
    ent.axis[1 as i32 as usize][0 as i32 as usize] =
        crate::src::qcommon::q_math::vec3_origin[0 as i32 as usize]
            - ent.axis[1 as i32 as usize][0 as i32 as usize]; // rotation speed
    ent.axis[1 as i32 as usize][1 as i32 as usize] =
        crate::src::qcommon::q_math::vec3_origin[1 as i32 as usize]
            - ent.axis[1 as i32 as usize][1 as i32 as usize]; // roll offset
    ent.axis[1 as i32 as usize][2 as i32 as usize] =
        crate::src::qcommon::q_math::vec3_origin[2 as i32 as usize]
            - ent.axis[1 as i32 as usize][2 as i32 as usize];
    CrossProduct(
        ent.axis[0 as i32 as usize].as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis[1 as i32 as usize].as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis[2 as i32 as usize].as_mut_ptr(),
    );
    ent.reType = crate::tr_types_h::RT_PORTALSURFACE;
    ent.oldframe = (*s1).powerups;
    ent.frame = (*s1).frame;
    ent.skinNum = ((*s1).clientNum as f64 / 256.0f64
        * 360 as i32 as f64) as i32;
    // add to refresh list
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        &mut ent as *mut _ as *const crate::tr_types_h::refEntity_t,
    );
}
/*
================
CG_CreateRotationMatrix
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_CreateRotationMatrix(
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
CG_TransposeMatrix
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_TransposeMatrix(
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
CG_RotatePoint
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_RotatePoint(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut matrix: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    let mut tvec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    tvec[0 as i32 as usize] = *point.offset(0 as i32 as isize);
    tvec[1 as i32 as usize] = *point.offset(1 as i32 as isize);
    tvec[2 as i32 as usize] = *point.offset(2 as i32 as isize);
    *point.offset(0 as i32 as isize) = (*matrix.offset(0 as i32 as isize))
        [0 as i32 as usize]
        * tvec[0 as i32 as usize]
        + (*matrix.offset(0 as i32 as isize))[1 as i32 as usize]
            * tvec[1 as i32 as usize]
        + (*matrix.offset(0 as i32 as isize))[2 as i32 as usize]
            * tvec[2 as i32 as usize];
    *point.offset(1 as i32 as isize) = (*matrix.offset(1 as i32 as isize))
        [0 as i32 as usize]
        * tvec[0 as i32 as usize]
        + (*matrix.offset(1 as i32 as isize))[1 as i32 as usize]
            * tvec[1 as i32 as usize]
        + (*matrix.offset(1 as i32 as isize))[2 as i32 as usize]
            * tvec[2 as i32 as usize];
    *point.offset(2 as i32 as isize) = (*matrix.offset(2 as i32 as isize))
        [0 as i32 as usize]
        * tvec[0 as i32 as usize]
        + (*matrix.offset(2 as i32 as isize))[1 as i32 as usize]
            * tvec[1 as i32 as usize]
        + (*matrix.offset(2 as i32 as isize))[2 as i32 as usize]
            * tvec[2 as i32 as usize];
}
/*
=========================
CG_AdjustPositionForMover

Also called by client movement prediction code
=========================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AdjustPositionForMover(
    mut in_0: *const crate::src::qcommon::q_shared::vec_t,
    mut moverNum: i32,
    mut fromTime: i32,
    mut toTime: i32,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
    mut angles_in: *mut crate::src::qcommon::q_shared::vec_t,
    mut angles_out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    let mut oldOrigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut deltaOrigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut oldAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut deltaAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut matrix: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    let mut transpose: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut org2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut move2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if moverNum <= 0 as i32
        || moverNum >= ((1 as i32) << 10 as i32) - 2 as i32
    {
        *out.offset(0 as i32 as isize) = *in_0.offset(0 as i32 as isize);
        *out.offset(1 as i32 as isize) = *in_0.offset(1 as i32 as isize);
        *out.offset(2 as i32 as isize) = *in_0.offset(2 as i32 as isize);
        *angles_out.offset(0 as i32 as isize) =
            *angles_in.offset(0 as i32 as isize);
        *angles_out.offset(1 as i32 as isize) =
            *angles_in.offset(1 as i32 as isize);
        *angles_out.offset(2 as i32 as isize) =
            *angles_in.offset(2 as i32 as isize);
        return;
    }
    cent = &mut *crate::src::cgame::cg_main::cg_entities
        .as_mut_ptr()
        .offset(moverNum as isize) as *mut crate::cg_local_h::centity_t;
    if (*cent).currentState.eType != crate::bg_public_h::ET_MOVER as i32 {
        *out.offset(0 as i32 as isize) = *in_0.offset(0 as i32 as isize);
        *out.offset(1 as i32 as isize) = *in_0.offset(1 as i32 as isize);
        *out.offset(2 as i32 as isize) = *in_0.offset(2 as i32 as isize);
        *angles_out.offset(0 as i32 as isize) =
            *angles_in.offset(0 as i32 as isize);
        *angles_out.offset(1 as i32 as isize) =
            *angles_in.offset(1 as i32 as isize);
        *angles_out.offset(2 as i32 as isize) =
            *angles_in.offset(2 as i32 as isize);
        return;
    }
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos as *mut _
            as *const crate::src::qcommon::q_shared::trajectory_t,
        fromTime,
        oldOrigin.as_mut_ptr(),
    );
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.apos as *mut _
            as *const crate::src::qcommon::q_shared::trajectory_t,
        fromTime,
        oldAngles.as_mut_ptr(),
    );
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos as *mut _
            as *const crate::src::qcommon::q_shared::trajectory_t,
        toTime,
        origin.as_mut_ptr(),
    );
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.apos as *mut _
            as *const crate::src::qcommon::q_shared::trajectory_t,
        toTime,
        angles.as_mut_ptr(),
    );
    deltaOrigin[0 as i32 as usize] =
        origin[0 as i32 as usize] - oldOrigin[0 as i32 as usize];
    deltaOrigin[1 as i32 as usize] =
        origin[1 as i32 as usize] - oldOrigin[1 as i32 as usize];
    deltaOrigin[2 as i32 as usize] =
        origin[2 as i32 as usize] - oldOrigin[2 as i32 as usize];
    deltaAngles[0 as i32 as usize] =
        angles[0 as i32 as usize] - oldAngles[0 as i32 as usize];
    deltaAngles[1 as i32 as usize] =
        angles[1 as i32 as usize] - oldAngles[1 as i32 as usize];
    deltaAngles[2 as i32 as usize] =
        angles[2 as i32 as usize] - oldAngles[2 as i32 as usize];
    // origin change when on a rotating object
    CG_CreateRotationMatrix(deltaAngles.as_mut_ptr(), transpose.as_mut_ptr());
    CG_TransposeMatrix(transpose.as_mut_ptr(), matrix.as_mut_ptr());
    org[0 as i32 as usize] =
        *in_0.offset(0 as i32 as isize) - oldOrigin[0 as i32 as usize];
    org[1 as i32 as usize] =
        *in_0.offset(1 as i32 as isize) - oldOrigin[1 as i32 as usize];
    org[2 as i32 as usize] =
        *in_0.offset(2 as i32 as isize) - oldOrigin[2 as i32 as usize];
    org2[0 as i32 as usize] = org[0 as i32 as usize];
    org2[1 as i32 as usize] = org[1 as i32 as usize];
    org2[2 as i32 as usize] = org[2 as i32 as usize];
    CG_RotatePoint(org2.as_mut_ptr(), matrix.as_mut_ptr());
    move2[0 as i32 as usize] =
        org2[0 as i32 as usize] - org[0 as i32 as usize];
    move2[1 as i32 as usize] =
        org2[1 as i32 as usize] - org[1 as i32 as usize];
    move2[2 as i32 as usize] =
        org2[2 as i32 as usize] - org[2 as i32 as usize];
    deltaOrigin[0 as i32 as usize] =
        deltaOrigin[0 as i32 as usize] + move2[0 as i32 as usize];
    deltaOrigin[1 as i32 as usize] =
        deltaOrigin[1 as i32 as usize] + move2[1 as i32 as usize];
    deltaOrigin[2 as i32 as usize] =
        deltaOrigin[2 as i32 as usize] + move2[2 as i32 as usize];
    *out.offset(0 as i32 as isize) =
        *in_0.offset(0 as i32 as isize) + deltaOrigin[0 as i32 as usize];
    *out.offset(1 as i32 as isize) =
        *in_0.offset(1 as i32 as isize) + deltaOrigin[1 as i32 as usize];
    *out.offset(2 as i32 as isize) =
        *in_0.offset(2 as i32 as isize) + deltaOrigin[2 as i32 as usize];
    *angles_out.offset(0 as i32 as isize) =
        *angles_in.offset(0 as i32 as isize) + deltaAngles[0 as i32 as usize];
    *angles_out.offset(1 as i32 as isize) =
        *angles_in.offset(1 as i32 as isize) + deltaAngles[1 as i32 as usize];
    *angles_out.offset(2 as i32 as isize) =
        *angles_in.offset(2 as i32 as isize) + deltaAngles[2 as i32 as usize];
}
/*
=============================
CG_InterpolateEntityPosition
=============================
*/

unsafe extern "C" fn CG_InterpolateEntityPosition(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut current: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut next: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut f: f32 = 0.;
    // it would be an internal error to find an entity that interpolates without
    // a snapshot ahead of the current one
    if crate::src::cgame::cg_main::cg.nextSnap.is_null() {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_InterpoateEntityPosition: cg.nextSnap == NULL\x00" as *const u8
                as *const libc::c_char,
        );
    }
    f = crate::src::cgame::cg_main::cg.frameInterpolation;
    // this will linearize a sine or parabolic curve, but it is important
    // to not extrapolate player positions if more recent data is available
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos as *mut _
            as *const crate::src::qcommon::q_shared::trajectory_t,
        (*crate::src::cgame::cg_main::cg.snap).serverTime,
        current.as_mut_ptr(),
    );
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).nextState.pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
        (*crate::src::cgame::cg_main::cg.nextSnap).serverTime,
        next.as_mut_ptr(),
    );
    (*cent).lerpOrigin[0 as i32 as usize] = current[0 as i32 as usize]
        + f * (next[0 as i32 as usize] - current[0 as i32 as usize]);
    (*cent).lerpOrigin[1 as i32 as usize] = current[1 as i32 as usize]
        + f * (next[1 as i32 as usize] - current[1 as i32 as usize]);
    (*cent).lerpOrigin[2 as i32 as usize] = current[2 as i32 as usize]
        + f * (next[2 as i32 as usize] - current[2 as i32 as usize]);
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.apos as *mut _
            as *const crate::src::qcommon::q_shared::trajectory_t,
        (*crate::src::cgame::cg_main::cg.snap).serverTime,
        current.as_mut_ptr(),
    );
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).nextState.apos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
        (*crate::src::cgame::cg_main::cg.nextSnap).serverTime,
        next.as_mut_ptr(),
    );
    (*cent).lerpAngles[0 as i32 as usize] = crate::src::qcommon::q_math::LerpAngle(
        current[0 as i32 as usize],
        next[0 as i32 as usize],
        f,
    );
    (*cent).lerpAngles[1 as i32 as usize] = crate::src::qcommon::q_math::LerpAngle(
        current[1 as i32 as usize],
        next[1 as i32 as usize],
        f,
    );
    (*cent).lerpAngles[2 as i32 as usize] = crate::src::qcommon::q_math::LerpAngle(
        current[2 as i32 as usize],
        next[2 as i32 as usize],
        f,
    );
}
/*
===============
CG_CalcEntityLerpPositions

===============
*/

unsafe extern "C" fn CG_CalcEntityLerpPositions(mut cent: *mut crate::cg_local_h::centity_t) {
    // if this player does not want to see extrapolated players
    if crate::src::cgame::cg_main::cg_smoothClients.integer == 0 {
        // make sure the clients use TR_INTERPOLATE
        if (*cent).currentState.number < 64 as i32 {
            (*cent).currentState.pos.trType = crate::src::qcommon::q_shared::TR_INTERPOLATE;
            (*cent).nextState.pos.trType = crate::src::qcommon::q_shared::TR_INTERPOLATE
        }
    }
    if (*cent).interpolate as u32 != 0
        && (*cent).currentState.pos.trType as u32
            == crate::src::qcommon::q_shared::TR_INTERPOLATE as i32 as u32
    {
        CG_InterpolateEntityPosition(cent);
        return;
    }
    // first see if we can interpolate between two snaps for
    // linear extrapolated clients
    if (*cent).interpolate as u32 != 0
        && (*cent).currentState.pos.trType as u32
            == crate::src::qcommon::q_shared::TR_LINEAR_STOP as i32 as u32
        && (*cent).currentState.number < 64 as i32
    {
        CG_InterpolateEntityPosition(cent);
        return;
    }
    // just use the current frame and evaluate as best we can
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos as *mut _
            as *const crate::src::qcommon::q_shared::trajectory_t,
        crate::src::cgame::cg_main::cg.time,
        (*cent).lerpOrigin.as_mut_ptr(),
    );
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.apos as *mut _
            as *const crate::src::qcommon::q_shared::trajectory_t,
        crate::src::cgame::cg_main::cg.time,
        (*cent).lerpAngles.as_mut_ptr(),
    );
    // adjust for riding a mover if it wasn't rolled into the predicted
    // player state
    if cent
        != &mut crate::src::cgame::cg_main::cg.predictedPlayerEntity
            as *mut crate::cg_local_h::centity_t
    {
        CG_AdjustPositionForMover(
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*cent).currentState.groundEntityNum,
            (*crate::src::cgame::cg_main::cg.snap).serverTime,
            crate::src::cgame::cg_main::cg.time,
            (*cent).lerpOrigin.as_mut_ptr(),
            (*cent).lerpAngles.as_mut_ptr(),
            (*cent).lerpAngles.as_mut_ptr(),
        );
    };
}
/*
===============
CG_TeamBase
===============
*/

unsafe extern "C" fn CG_TeamBase(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut model: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    if crate::src::cgame::cg_main::cgs.gametype as u32
        == crate::bg_public_h::GT_CTF as i32 as u32
    {
        // show the flag base
        crate::stdlib::memset(
            &mut model as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
        );
        model.reType = crate::tr_types_h::RT_MODEL;
        model.lightingOrigin[0 as i32 as usize] =
            (*cent).lerpOrigin[0 as i32 as usize];
        model.lightingOrigin[1 as i32 as usize] =
            (*cent).lerpOrigin[1 as i32 as usize];
        model.lightingOrigin[2 as i32 as usize] =
            (*cent).lerpOrigin[2 as i32 as usize];
        model.origin[0 as i32 as usize] = (*cent).lerpOrigin[0 as i32 as usize];
        model.origin[1 as i32 as usize] = (*cent).lerpOrigin[1 as i32 as usize];
        model.origin[2 as i32 as usize] = (*cent).lerpOrigin[2 as i32 as usize];
        crate::src::qcommon::q_math::AnglesToAxis(
            (*cent).currentState.angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            model.axis.as_mut_ptr(),
        );
        if (*cent).currentState.modelindex == crate::bg_public_h::TEAM_RED as i32 {
            model.hModel = crate::src::cgame::cg_main::cgs.media.redFlagBaseModel
        } else if (*cent).currentState.modelindex == crate::bg_public_h::TEAM_BLUE as i32 {
            model.hModel = crate::src::cgame::cg_main::cgs.media.blueFlagBaseModel
        } else {
            model.hModel = crate::src::cgame::cg_main::cgs.media.neutralFlagBaseModel
        }
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
            &mut model as *mut _ as *const crate::tr_types_h::refEntity_t,
        );
    };
}
/*
===============
CG_AddCEntity

===============
*/

unsafe extern "C" fn CG_AddCEntity(mut cent: *mut crate::cg_local_h::centity_t) {
    // event-only entities will have been dealt with already
    if (*cent).currentState.eType >= crate::bg_public_h::ET_EVENTS as i32 {
        return;
    }
    // calculate the current origin
    CG_CalcEntityLerpPositions(cent);
    // add automatic effects
    CG_EntityEffects(cent);
    match (*cent).currentState.eType {
        10 | 8 | 9 => {}
        0 => {
            CG_General(cent);
        }
        1 => {
            crate::src::cgame::cg_players::CG_Player(cent as *mut crate::cg_local_h::centity_s);
        }
        2 => {
            CG_Item(cent);
        }
        3 => {
            CG_Missile(cent);
        }
        4 => {
            CG_Mover(cent);
        }
        5 => {
            CG_Beam(cent);
        }
        6 => {
            CG_Portal(cent);
        }
        7 => {
            CG_Speaker(cent);
        }
        11 => {
            CG_Grapple(cent);
        }
        12 => {
            CG_TeamBase(cent);
        }
        _ => {
            crate::src::cgame::cg_main::CG_Error(
                b"Bad entity type: %i\x00" as *const u8 as *const libc::c_char,
                (*cent).currentState.eType,
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
/*
===============
CG_AddPacketEntities

===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddPacketEntities() {
    let mut num: i32 = 0;
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    let mut ps: *mut crate::src::qcommon::q_shared::playerState_t =
        0 as *mut crate::src::qcommon::q_shared::playerState_t;
    // set cg.frameInterpolation
    if !crate::src::cgame::cg_main::cg.nextSnap.is_null() {
        let mut delta: i32 = 0;
        delta = (*crate::src::cgame::cg_main::cg.nextSnap).serverTime
            - (*crate::src::cgame::cg_main::cg.snap).serverTime;
        if delta == 0 as i32 {
            crate::src::cgame::cg_main::cg.frameInterpolation = 0 as i32 as f32
        } else {
            crate::src::cgame::cg_main::cg.frameInterpolation = (crate::src::cgame::cg_main::cg
                .time
                - (*crate::src::cgame::cg_main::cg.snap).serverTime)
                as f32
                / delta as f32
        }
    } else {
        crate::src::cgame::cg_main::cg.frameInterpolation = 0 as i32 as f32
        // actually, it should never be used, because
        // no entities should be marked as interpolating
    }
    // the auto-rotating items will all have the same axis
    crate::src::cgame::cg_main::cg.autoAngles[0 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::cgame::cg_main::cg.autoAngles[1 as i32 as usize] =
        (((crate::src::cgame::cg_main::cg.time & 2047 as i32) * 360 as i32)
            as f64
            / 2048.0f64) as crate::src::qcommon::q_shared::vec_t;
    crate::src::cgame::cg_main::cg.autoAngles[2 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::cgame::cg_main::cg.autoAnglesFast[0 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::cgame::cg_main::cg.autoAnglesFast[1 as i32 as usize] =
        ((crate::src::cgame::cg_main::cg.time & 1023 as i32) * 360 as i32)
            as f32
            / 1024.0f32;
    crate::src::cgame::cg_main::cg.autoAnglesFast[2 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::qcommon::q_math::AnglesToAxis(
        crate::src::cgame::cg_main::cg.autoAngles.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::cgame::cg_main::cg.autoAxis.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::AnglesToAxis(
        crate::src::cgame::cg_main::cg.autoAnglesFast.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::cgame::cg_main::cg.autoAxisFast.as_mut_ptr(),
    );
    // generate and add the entity from the playerstate
    ps = &mut crate::src::cgame::cg_main::cg.predictedPlayerState;
    crate::src::game::bg_misc::BG_PlayerStateToEntityState(
        ps as *mut crate::src::qcommon::q_shared::playerState_s,
        &mut crate::src::cgame::cg_main::cg
            .predictedPlayerEntity
            .currentState as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
        crate::src::qcommon::q_shared::qfalse,
    );
    CG_AddCEntity(&mut crate::src::cgame::cg_main::cg.predictedPlayerEntity);
    // lerp the non-predicted value for lightning gun origins
    CG_CalcEntityLerpPositions(
        &mut *crate::src::cgame::cg_main::cg_entities
            .as_mut_ptr()
            .offset((*crate::src::cgame::cg_main::cg.snap).ps.clientNum as isize),
    );
    // add each entity sent over by the server
    num = 0 as i32;
    while num < (*crate::src::cgame::cg_main::cg.snap).numEntities {
        cent = &mut *crate::src::cgame::cg_main::cg_entities.as_mut_ptr().offset(
            (*(*crate::src::cgame::cg_main::cg.snap)
                .entities
                .as_mut_ptr()
                .offset(num as isize))
            .number as isize,
        ) as *mut crate::cg_local_h::centity_t;
        CG_AddCEntity(cent);
        num += 1
    }
}

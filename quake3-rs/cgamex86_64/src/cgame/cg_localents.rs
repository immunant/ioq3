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
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::cgame::cg_localents::q_shared_h::CrossProduct;
pub use crate::src::cgame::cg_localents::q_shared_h::VectorLength;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectory;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectoryDelta;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AnglesToAxis;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
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
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
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

pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::leBounceSoundType_t;
pub use crate::cg_local_h::leMarkType_t;
pub use crate::cg_local_h::leType_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::localEntity_s;
pub use crate::cg_local_h::localEntity_t;
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
pub use crate::cg_local_h::LEBS_BLOOD;
pub use crate::cg_local_h::LEBS_BRASS;
pub use crate::cg_local_h::LEBS_NONE;
pub use crate::cg_local_h::LEF_PUFF_DONT_SCALE;
pub use crate::cg_local_h::LEF_SOUND1;
pub use crate::cg_local_h::LEF_SOUND2;
pub use crate::cg_local_h::LEF_TUMBLE;
pub use crate::cg_local_h::LEMT_BLOOD;
pub use crate::cg_local_h::LEMT_BURN;
pub use crate::cg_local_h::LEMT_NONE;
pub use crate::cg_local_h::LE_EXPLOSION;
pub use crate::cg_local_h::LE_FADE_RGB;
pub use crate::cg_local_h::LE_FALL_SCALE_FADE;
pub use crate::cg_local_h::LE_FRAGMENT;
pub use crate::cg_local_h::LE_MARK;
pub use crate::cg_local_h::LE_MOVE_SCALE_FADE;
pub use crate::cg_local_h::LE_SCALE_FADE;
pub use crate::cg_local_h::LE_SCOREPLUM;
pub use crate::cg_local_h::LE_SPRITE_EXPLOSION;
pub use crate::src::cgame::cg_effects::CG_SmokePuff;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_Error;
pub use crate::src::cgame::cg_marks::CG_ImpactMark;
pub use crate::src::cgame::cg_predict::CG_PointContents;
pub use crate::src::cgame::cg_predict::CG_Trace;
pub use crate::src::cgame::cg_syscalls::trap_R_AddLightToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene;
pub use crate::src::cgame::cg_syscalls::trap_S_StartSound;

#[no_mangle]

pub static mut cg_localEntities: [crate::cg_local_h::localEntity_t; 512] =
    [crate::cg_local_h::localEntity_t {
        prev: 0 as *const crate::cg_local_h::localEntity_s as *mut crate::cg_local_h::localEntity_s,
        next: 0 as *const crate::cg_local_h::localEntity_s as *mut crate::cg_local_h::localEntity_s,
        leType: crate::cg_local_h::LE_MARK,
        leFlags: 0,
        startTime: 0,
        endTime: 0,
        fadeInTime: 0,
        lifeRate: 0.,
        pos: crate::src::qcommon::q_shared::trajectory_t {
            trType: crate::src::qcommon::q_shared::TR_STATIONARY,
            trTime: 0,
            trDuration: 0,
            trBase: [0.; 3],
            trDelta: [0.; 3],
        },
        angles: crate::src::qcommon::q_shared::trajectory_t {
            trType: crate::src::qcommon::q_shared::TR_STATIONARY,
            trTime: 0,
            trDuration: 0,
            trBase: [0.; 3],
            trDelta: [0.; 3],
        },
        bounceFactor: 0.,
        color: [0.; 4],
        radius: 0.,
        light: 0.,
        lightColor: [0.; 3],
        leMarkType: crate::cg_local_h::LEMT_NONE,
        leBounceSoundType: crate::cg_local_h::LEBS_NONE,
        refEntity: crate::tr_types_h::refEntity_t {
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
        },
    }; 512];
#[no_mangle]

pub static mut cg_activeLocalEntities: crate::cg_local_h::localEntity_t =
    crate::cg_local_h::localEntity_t {
        prev: 0 as *const crate::cg_local_h::localEntity_s as *mut crate::cg_local_h::localEntity_s,
        next: 0 as *const crate::cg_local_h::localEntity_s as *mut crate::cg_local_h::localEntity_s,
        leType: crate::cg_local_h::LE_MARK,
        leFlags: 0,
        startTime: 0,
        endTime: 0,
        fadeInTime: 0,
        lifeRate: 0.,
        pos: crate::src::qcommon::q_shared::trajectory_t {
            trType: crate::src::qcommon::q_shared::TR_STATIONARY,
            trTime: 0,
            trDuration: 0,
            trBase: [0.; 3],
            trDelta: [0.; 3],
        },
        angles: crate::src::qcommon::q_shared::trajectory_t {
            trType: crate::src::qcommon::q_shared::TR_STATIONARY,
            trTime: 0,
            trDuration: 0,
            trBase: [0.; 3],
            trDelta: [0.; 3],
        },
        bounceFactor: 0.,
        color: [0.; 4],
        radius: 0.,
        light: 0.,
        lightColor: [0.; 3],
        leMarkType: crate::cg_local_h::LEMT_NONE,
        leBounceSoundType: crate::cg_local_h::LEBS_NONE,
        refEntity: crate::tr_types_h::refEntity_t {
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
        },
    };
// double linked list
#[no_mangle]

pub static mut cg_freeLocalEntities: *mut crate::cg_local_h::localEntity_t =
    0 as *const crate::cg_local_h::localEntity_t as *mut crate::cg_local_h::localEntity_t;
// single linked list
/*
===================
CG_InitLocalEntities

This is called at startup and for tournement restarts
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_InitLocalEntities() {
    let mut i: i32 = 0;
    crate::stdlib::memset(
        cg_localEntities.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[crate::cg_local_h::localEntity_t; 512]>() as libc::c_ulong,
    );
    cg_activeLocalEntities.next = &mut cg_activeLocalEntities;
    cg_activeLocalEntities.prev = &mut cg_activeLocalEntities;
    cg_freeLocalEntities = cg_localEntities.as_mut_ptr();
    i = 0 as i32;
    while i < 512 as i32 - 1 as i32 {
        cg_localEntities[i as usize].next = &mut *cg_localEntities
            .as_mut_ptr()
            .offset((i + 1 as i32) as isize)
            as *mut crate::cg_local_h::localEntity_t;
        i += 1
    }
}
/*
==================
CG_FreeLocalEntity
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_FreeLocalEntity(mut le: *mut crate::cg_local_h::localEntity_t) {
    if (*le).prev.is_null() {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_FreeLocalEntity: not active\x00" as *const u8 as *const libc::c_char,
        );
    }
    // remove from the doubly linked active list
    (*(*le).prev).next = (*le).next;
    (*(*le).next).prev = (*le).prev;
    // the free list is only singly linked
    (*le).next = cg_freeLocalEntities;
    cg_freeLocalEntities = le;
}
/*
===================
CG_AllocLocalEntity

Will always succeed, even if it requires freeing an old active entity
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AllocLocalEntity() -> *mut crate::cg_local_h::localEntity_t {
    let mut le: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    if cg_freeLocalEntities.is_null() {
        // no free entities, so free the one at the end of the chain
        // remove the oldest active entity
        CG_FreeLocalEntity(cg_activeLocalEntities.prev);
    }
    le = cg_freeLocalEntities;
    cg_freeLocalEntities = (*cg_freeLocalEntities).next;
    crate::stdlib::memset(
        le as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::cg_local_h::localEntity_t>() as libc::c_ulong,
    );
    // link into the active list
    (*le).next = cg_activeLocalEntities.next;
    (*le).prev = &mut cg_activeLocalEntities;
    (*cg_activeLocalEntities.next).prev = le;
    cg_activeLocalEntities.next = le;
    return le;
}
/*
====================================================================================

FRAGMENT PROCESSING

A fragment localentity interacts with the environment in some way (hitting walls),
or generates more localentities along a trail.

====================================================================================
*/
/*
================
CG_BloodTrail

Leave expanding blood puffs behind gibs
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_BloodTrail(mut le: *mut crate::cg_local_h::localEntity_t) {
    let mut t: i32 = 0;
    let mut t2: i32 = 0;
    let mut step: i32 = 0;
    let mut newOrigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut blood: *mut crate::cg_local_h::localEntity_t =
        0 as *mut crate::cg_local_h::localEntity_t;
    step = 150 as i32;
    t = step
        * ((crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.frametime + step)
            / step);
    t2 = step * (crate::src::cgame::cg_main::cg.time / step);
    while t <= t2 {
        crate::src::game::bg_misc::BG_EvaluateTrajectory(
            &mut (*le).pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
            t,
            newOrigin.as_mut_ptr(),
        );
        blood = crate::src::cgame::cg_effects::CG_SmokePuff(
            newOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            20 as i32 as f32,
            1 as i32 as f32,
            1 as i32 as f32,
            1 as i32 as f32,
            1 as i32 as f32,
            2000 as i32 as f32,
            t,
            0 as i32,
            0 as i32,
            crate::src::cgame::cg_main::cgs.media.bloodTrailShader,
        ) as *mut crate::cg_local_h::localEntity_s;
        // use the optimized version
        (*blood).leType = crate::cg_local_h::LE_FALL_SCALE_FADE;
        // drop a total of 40 units over its lifetime
        (*blood).pos.trDelta[2 as i32 as usize] = 40 as i32 as crate::src::qcommon::q_shared::vec_t;
        t += step
    }
}
/*
================
CG_FragmentBounceMark
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_FragmentBounceMark(
    mut le: *mut crate::cg_local_h::localEntity_t,
    mut trace: *mut crate::src::qcommon::q_shared::trace_t,
) {
    let mut radius: i32 = 0;
    if (*le).leMarkType as u32 == crate::cg_local_h::LEMT_BLOOD as i32 as u32 {
        radius = 16 as i32 + (::libc::rand() & 31 as i32);
        crate::src::cgame::cg_marks::CG_ImpactMark(
            crate::src::cgame::cg_main::cgs.media.bloodMarkShader,
            (*trace).endpos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*trace).plane.normal.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 * 360 as i32 as f32,
            1 as i32 as f32,
            1 as i32 as f32,
            1 as i32 as f32,
            1 as i32 as f32,
            crate::src::qcommon::q_shared::qtrue,
            radius as f32,
            crate::src::qcommon::q_shared::qfalse,
        );
    } else if (*le).leMarkType as u32 == crate::cg_local_h::LEMT_BURN as i32 as u32 {
        radius = 8 as i32 + (::libc::rand() & 15 as i32);
        crate::src::cgame::cg_marks::CG_ImpactMark(
            crate::src::cgame::cg_main::cgs.media.burnMarkShader,
            (*trace).endpos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*trace).plane.normal.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (::libc::rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 * 360 as i32 as f32,
            1 as i32 as f32,
            1 as i32 as f32,
            1 as i32 as f32,
            1 as i32 as f32,
            crate::src::qcommon::q_shared::qtrue,
            radius as f32,
            crate::src::qcommon::q_shared::qfalse,
        );
    }
    // don't allow a fragment to make multiple marks, or they
    // pile up while settling
    (*le).leMarkType = crate::cg_local_h::LEMT_NONE;
}
/*
================
CG_FragmentBounceSound
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_FragmentBounceSound(
    mut le: *mut crate::cg_local_h::localEntity_t,
    mut trace: *mut crate::src::qcommon::q_shared::trace_t,
) {
    if (*le).leBounceSoundType as u32 == crate::cg_local_h::LEBS_BLOOD as i32 as u32 {
        // half the gibs will make splat sounds
        if ::libc::rand() & 1 as i32 != 0 {
            let mut r: i32 = ::libc::rand() & 3 as i32;
            let mut s: crate::src::qcommon::q_shared::sfxHandle_t = 0;
            if r == 0 as i32 {
                s = crate::src::cgame::cg_main::cgs.media.gibBounce1Sound
            } else if r == 1 as i32 {
                s = crate::src::cgame::cg_main::cgs.media.gibBounce2Sound
            } else {
                s = crate::src::cgame::cg_main::cgs.media.gibBounce3Sound
            }
            crate::src::cgame::cg_syscalls::trap_S_StartSound(
                (*trace).endpos.as_mut_ptr(),
                ((1 as i32) << 10 as i32) - 2 as i32,
                crate::src::qcommon::q_shared::CHAN_AUTO as i32,
                s,
            );
        }
    } else {
        // no bounce sound for non-brass types
    }
    // don't allow a fragment to make multiple bounce sounds,
    // or it gets too noisy as they settle
    (*le).leBounceSoundType = crate::cg_local_h::LEBS_NONE;
}
/*
================
CG_ReflectVelocity
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ReflectVelocity(
    mut le: *mut crate::cg_local_h::localEntity_t,
    mut trace: *mut crate::src::qcommon::q_shared::trace_t,
) {
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dot: f32 = 0.;
    let mut hitTime: i32 = 0;
    // reflect the velocity on the trace plane
    hitTime = ((crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.frametime)
        as f32
        + crate::src::cgame::cg_main::cg.frametime as f32 * (*trace).fraction) as i32;
    crate::src::game::bg_misc::BG_EvaluateTrajectoryDelta(
        &mut (*le).pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
        hitTime,
        velocity.as_mut_ptr(),
    );
    dot = velocity[0 as i32 as usize] * (*trace).plane.normal[0 as i32 as usize]
        + velocity[1 as i32 as usize] * (*trace).plane.normal[1 as i32 as usize]
        + velocity[2 as i32 as usize] * (*trace).plane.normal[2 as i32 as usize];
    (*le).pos.trDelta[0 as i32 as usize] = velocity[0 as i32 as usize]
        + (*trace).plane.normal[0 as i32 as usize] * (-(2 as i32) as f32 * dot);
    (*le).pos.trDelta[1 as i32 as usize] = velocity[1 as i32 as usize]
        + (*trace).plane.normal[1 as i32 as usize] * (-(2 as i32) as f32 * dot);
    (*le).pos.trDelta[2 as i32 as usize] = velocity[2 as i32 as usize]
        + (*trace).plane.normal[2 as i32 as usize] * (-(2 as i32) as f32 * dot);
    (*le).pos.trDelta[0 as i32 as usize] =
        (*le).pos.trDelta[0 as i32 as usize] * (*le).bounceFactor;
    (*le).pos.trDelta[1 as i32 as usize] =
        (*le).pos.trDelta[1 as i32 as usize] * (*le).bounceFactor;
    (*le).pos.trDelta[2 as i32 as usize] =
        (*le).pos.trDelta[2 as i32 as usize] * (*le).bounceFactor;
    (*le).pos.trBase[0 as i32 as usize] = (*trace).endpos[0 as i32 as usize];
    (*le).pos.trBase[1 as i32 as usize] = (*trace).endpos[1 as i32 as usize];
    (*le).pos.trBase[2 as i32 as usize] = (*trace).endpos[2 as i32 as usize];
    (*le).pos.trTime = crate::src::cgame::cg_main::cg.time;
    // check for stop, making sure that even on low FPS systems it doesn't bobble
    if (*trace).allsolid as u32 != 0
        || (*trace).plane.normal[2 as i32 as usize] > 0 as i32 as f32
            && ((*le).pos.trDelta[2 as i32 as usize] < 40 as i32 as f32
                || (*le).pos.trDelta[2 as i32 as usize]
                    < -crate::src::cgame::cg_main::cg.frametime as f32
                        * (*le).pos.trDelta[2 as i32 as usize])
    {
        (*le).pos.trType = crate::src::qcommon::q_shared::TR_STATIONARY
    };
}
/*
================
CG_AddFragment
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddFragment(mut le: *mut crate::cg_local_h::localEntity_t) {
    let mut newOrigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
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
    if (*le).pos.trType as u32 == crate::src::qcommon::q_shared::TR_STATIONARY as i32 as u32 {
        // sink into the ground if near the removal time
        let mut t: i32 = 0;
        let mut oldZ: f32 = 0.;
        t = (*le).endTime - crate::src::cgame::cg_main::cg.time;
        if t < 1000 as i32 {
            // we must use an explicit lighting origin, otherwise the
            // lighting would be lost as soon as the origin went
            // into the ground
            (*le).refEntity.lightingOrigin[0 as i32 as usize] =
                (*le).refEntity.origin[0 as i32 as usize];
            (*le).refEntity.lightingOrigin[1 as i32 as usize] =
                (*le).refEntity.origin[1 as i32 as usize];
            (*le).refEntity.lightingOrigin[2 as i32 as usize] =
                (*le).refEntity.origin[2 as i32 as usize];
            (*le).refEntity.renderfx |= 0x80 as i32;
            oldZ = (*le).refEntity.origin[2 as i32 as usize];
            (*le).refEntity.origin[2 as i32 as usize] = ((*le).refEntity.origin[2 as i32 as usize]
                as f64
                - 16 as i32 as f64 * (1.0f64 - (t as f32 / 1000 as i32 as f32) as f64))
                as f32;
            crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
                &mut (*le).refEntity as *mut _ as *const crate::tr_types_h::refEntity_t,
            );
            (*le).refEntity.origin[2 as i32 as usize] = oldZ
        } else {
            crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
                &mut (*le).refEntity as *mut _ as *const crate::tr_types_h::refEntity_t,
            );
        }
        return;
    }
    // calculate new position
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*le).pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
        crate::src::cgame::cg_main::cg.time,
        newOrigin.as_mut_ptr(),
    );
    // trace a line from previous position to new position
    crate::src::cgame::cg_predict::CG_Trace(
        &mut trace as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        (*le).refEntity.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        newOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        -(1 as i32),
        1 as i32,
    );
    if trace.fraction as f64 == 1.0f64 {
        // still in free fall
        (*le).refEntity.origin[0 as i32 as usize] = newOrigin[0 as i32 as usize];
        (*le).refEntity.origin[1 as i32 as usize] = newOrigin[1 as i32 as usize];
        (*le).refEntity.origin[2 as i32 as usize] = newOrigin[2 as i32 as usize];
        if (*le).leFlags & crate::cg_local_h::LEF_TUMBLE as i32 != 0 {
            let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
            crate::src::game::bg_misc::BG_EvaluateTrajectory(
                &mut (*le).angles as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
                crate::src::cgame::cg_main::cg.time,
                angles.as_mut_ptr(),
            );
            crate::src::qcommon::q_math::AnglesToAxis(
                angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*le).refEntity.axis.as_mut_ptr(),
            );
        }
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
            &mut (*le).refEntity as *mut _ as *const crate::tr_types_h::refEntity_t,
        );
        // add a blood trail
        if (*le).leBounceSoundType as u32 == crate::cg_local_h::LEBS_BLOOD as i32 as u32 {
            CG_BloodTrail(le);
        }
        return;
    }
    // if it is in a nodrop zone, remove it
    // this keeps gibs from waiting at the bottom of pits of death
    // and floating levels
    if crate::src::cgame::cg_predict::CG_PointContents(
        trace.endpos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0 as i32,
    ) as u32
        & 0x80000000 as u32
        != 0
    {
        CG_FreeLocalEntity(le);
        return;
    }
    // leave a mark
    CG_FragmentBounceMark(le, &mut trace);
    // do a bouncy sound
    CG_FragmentBounceSound(le, &mut trace);
    // reflect the velocity on the trace plane
    CG_ReflectVelocity(le, &mut trace);
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        &mut (*le).refEntity as *mut _ as *const crate::tr_types_h::refEntity_t,
    );
}
/*
=====================================================================

TRIVIAL LOCAL ENTITIES

These only do simple scaling or modulation before passing to the renderer
=====================================================================
*/
/*
====================
CG_AddFadeRGB
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddFadeRGB(mut le: *mut crate::cg_local_h::localEntity_t) {
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut c: f32 = 0.;
    re = &mut (*le).refEntity;
    c = ((*le).endTime - crate::src::cgame::cg_main::cg.time) as f32 * (*le).lifeRate;
    c *= 0xff as i32 as f32;
    (*re).shaderRGBA[0 as i32 as usize] =
        ((*le).color[0 as i32 as usize] * c) as crate::src::qcommon::q_shared::byte;
    (*re).shaderRGBA[1 as i32 as usize] =
        ((*le).color[1 as i32 as usize] * c) as crate::src::qcommon::q_shared::byte;
    (*re).shaderRGBA[2 as i32 as usize] =
        ((*le).color[2 as i32 as usize] * c) as crate::src::qcommon::q_shared::byte;
    (*re).shaderRGBA[3 as i32 as usize] =
        ((*le).color[3 as i32 as usize] * c) as crate::src::qcommon::q_shared::byte;
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        re as *const crate::tr_types_h::refEntity_t,
    );
}
/*
==================
CG_AddMoveScaleFade
==================
*/

unsafe extern "C" fn CG_AddMoveScaleFade(mut le: *mut crate::cg_local_h::localEntity_t) {
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut c: f32 = 0.;
    let mut delta: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut len: f32 = 0.;
    re = &mut (*le).refEntity;
    if (*le).fadeInTime > (*le).startTime && crate::src::cgame::cg_main::cg.time < (*le).fadeInTime
    {
        // fade / grow time
        c = (1.0f64
            - (((*le).fadeInTime - crate::src::cgame::cg_main::cg.time) as f32
                / ((*le).fadeInTime - (*le).startTime) as f32) as f64) as f32
    } else {
        // fade / grow time
        c = ((*le).endTime - crate::src::cgame::cg_main::cg.time) as f32 * (*le).lifeRate
    }
    (*re).shaderRGBA[3 as i32 as usize] = (0xff as i32 as f32 * c * (*le).color[3 as i32 as usize])
        as crate::src::qcommon::q_shared::byte;
    if (*le).leFlags & crate::cg_local_h::LEF_PUFF_DONT_SCALE as i32 == 0 {
        (*re).radius = ((*le).radius as f64 * (1.0f64 - c as f64) + 8 as i32 as f64) as f32
    }
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*le).pos as *mut _ as *const crate::src::qcommon::q_shared::trajectory_t,
        crate::src::cgame::cg_main::cg.time,
        (*re).origin.as_mut_ptr(),
    );
    // if the view would be "inside" the sprite, kill the sprite
    // so it doesn't add too much overdraw
    delta[0 as i32 as usize] = (*re).origin[0 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize];
    delta[1 as i32 as usize] = (*re).origin[1 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize];
    delta[2 as i32 as usize] = (*re).origin[2 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize];
    len = VectorLength(delta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    if len < (*le).radius {
        CG_FreeLocalEntity(le);
        return;
    }
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        re as *const crate::tr_types_h::refEntity_t,
    );
}
/*
===================
CG_AddScaleFade

For rocket smokes that hang in place, fade out, and are
removed if the view passes through them.
There are often many of these, so it needs to be simple.
===================
*/

unsafe extern "C" fn CG_AddScaleFade(mut le: *mut crate::cg_local_h::localEntity_t) {
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut c: f32 = 0.;
    let mut delta: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut len: f32 = 0.;
    re = &mut (*le).refEntity;
    // fade / grow time
    c = ((*le).endTime - crate::src::cgame::cg_main::cg.time) as f32 * (*le).lifeRate;
    (*re).shaderRGBA[3 as i32 as usize] = (0xff as i32 as f32 * c * (*le).color[3 as i32 as usize])
        as crate::src::qcommon::q_shared::byte;
    (*re).radius = ((*le).radius as f64 * (1.0f64 - c as f64) + 8 as i32 as f64) as f32;
    // if the view would be "inside" the sprite, kill the sprite
    // so it doesn't add too much overdraw
    delta[0 as i32 as usize] = (*re).origin[0 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize];
    delta[1 as i32 as usize] = (*re).origin[1 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize];
    delta[2 as i32 as usize] = (*re).origin[2 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize];
    len = VectorLength(delta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    if len < (*le).radius {
        CG_FreeLocalEntity(le);
        return;
    }
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        re as *const crate::tr_types_h::refEntity_t,
    );
}
/*
=================
CG_AddFallScaleFade

This is just an optimized CG_AddMoveScaleFade
For blood mists that drift down, fade out, and are
removed if the view passes through them.
There are often 100+ of these, so it needs to be simple.
=================
*/

unsafe extern "C" fn CG_AddFallScaleFade(mut le: *mut crate::cg_local_h::localEntity_t) {
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut c: f32 = 0.;
    let mut delta: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut len: f32 = 0.;
    re = &mut (*le).refEntity;
    // fade time
    c = ((*le).endTime - crate::src::cgame::cg_main::cg.time) as f32 * (*le).lifeRate;
    (*re).shaderRGBA[3 as i32 as usize] = (0xff as i32 as f32 * c * (*le).color[3 as i32 as usize])
        as crate::src::qcommon::q_shared::byte;
    (*re).origin[2 as i32 as usize] = ((*le).pos.trBase[2 as i32 as usize] as f64
        - (1.0f64 - c as f64) * (*le).pos.trDelta[2 as i32 as usize] as f64)
        as f32;
    (*re).radius = ((*le).radius as f64 * (1.0f64 - c as f64) + 16 as i32 as f64) as f32;
    // if the view would be "inside" the sprite, kill the sprite
    // so it doesn't add too much overdraw
    delta[0 as i32 as usize] = (*re).origin[0 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize];
    delta[1 as i32 as usize] = (*re).origin[1 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize];
    delta[2 as i32 as usize] = (*re).origin[2 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize];
    len = VectorLength(delta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    if len < (*le).radius {
        CG_FreeLocalEntity(le);
        return;
    }
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        re as *const crate::tr_types_h::refEntity_t,
    );
}
/*
================
CG_AddExplosion
================
*/

unsafe extern "C" fn CG_AddExplosion(mut ex: *mut crate::cg_local_h::localEntity_t) {
    let mut ent: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    ent = &mut (*ex).refEntity;
    // add the entity
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        ent as *const crate::tr_types_h::refEntity_t,
    );
    // add the dlight
    if (*ex).light != 0. {
        let mut light: f32 = 0.;
        light = (crate::src::cgame::cg_main::cg.time - (*ex).startTime) as f32
            / ((*ex).endTime - (*ex).startTime) as f32;
        if (light as f64) < 0.5f64 {
            light = 1.0f64 as f32
        } else {
            light = (1.0f64 - (light as f64 - 0.5f64) * 2 as i32 as f64) as f32
        }
        light = (*ex).light * light;
        crate::src::cgame::cg_syscalls::trap_R_AddLightToScene(
            (*ent).origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            light,
            (*ex).lightColor[0 as i32 as usize],
            (*ex).lightColor[1 as i32 as usize],
            (*ex).lightColor[2 as i32 as usize],
        );
    };
}
/*
================
CG_AddSpriteExplosion
================
*/

unsafe extern "C" fn CG_AddSpriteExplosion(mut le: *mut crate::cg_local_h::localEntity_t) {
    let mut re: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut c: f32 = 0.;
    re = (*le).refEntity;
    c = ((*le).endTime - crate::src::cgame::cg_main::cg.time) as f32
        / ((*le).endTime - (*le).startTime) as f32;
    if c > 1 as i32 as f32 {
        c = 1.0f64 as f32
        // can happen during connection problems
    }
    re.shaderRGBA[0 as i32 as usize] = 0xff as i32 as crate::src::qcommon::q_shared::byte;
    re.shaderRGBA[1 as i32 as usize] = 0xff as i32 as crate::src::qcommon::q_shared::byte;
    re.shaderRGBA[2 as i32 as usize] = 0xff as i32 as crate::src::qcommon::q_shared::byte;
    re.shaderRGBA[3 as i32 as usize] =
        ((0xff as i32 as f32 * c) as f64 * 0.33f64) as crate::src::qcommon::q_shared::byte;
    re.reType = crate::tr_types_h::RT_SPRITE;
    re.radius = (42 as i32 as f64 * (1.0f64 - c as f64) + 30 as i32 as f64) as f32;
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        &mut re as *mut _ as *const crate::tr_types_h::refEntity_t,
    );
    // add the dlight
    if (*le).light != 0. {
        let mut light: f32 = 0.;
        light = (crate::src::cgame::cg_main::cg.time - (*le).startTime) as f32
            / ((*le).endTime - (*le).startTime) as f32;
        if (light as f64) < 0.5f64 {
            light = 1.0f64 as f32
        } else {
            light = (1.0f64 - (light as f64 - 0.5f64) * 2 as i32 as f64) as f32
        }
        light = (*le).light * light;
        crate::src::cgame::cg_syscalls::trap_R_AddLightToScene(
            re.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            light,
            (*le).lightColor[0 as i32 as usize],
            (*le).lightColor[1 as i32 as usize],
            (*le).lightColor[2 as i32 as usize],
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn CG_AddScorePlum(mut le: *mut crate::cg_local_h::localEntity_t) {
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut delta: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        1 as i32 as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut c: f32 = 0.;
    let mut len: f32 = 0.;
    let mut i: i32 = 0;
    let mut score: i32 = 0;
    let mut digits: [i32; 10] = [0; 10];
    let mut numdigits: i32 = 0;
    let mut negative: i32 = 0;
    re = &mut (*le).refEntity;
    c = ((*le).endTime - crate::src::cgame::cg_main::cg.time) as f32 * (*le).lifeRate;
    score = (*le).radius as i32;
    if score < 0 as i32 {
        (*re).shaderRGBA[0 as i32 as usize] = 0xff as i32 as crate::src::qcommon::q_shared::byte;
        (*re).shaderRGBA[1 as i32 as usize] = 0x11 as i32 as crate::src::qcommon::q_shared::byte;
        (*re).shaderRGBA[2 as i32 as usize] = 0x11 as i32 as crate::src::qcommon::q_shared::byte
    } else {
        (*re).shaderRGBA[0 as i32 as usize] = 0xff as i32 as crate::src::qcommon::q_shared::byte;
        (*re).shaderRGBA[1 as i32 as usize] = 0xff as i32 as crate::src::qcommon::q_shared::byte;
        (*re).shaderRGBA[2 as i32 as usize] = 0xff as i32 as crate::src::qcommon::q_shared::byte;
        if score >= 50 as i32 {
            (*re).shaderRGBA[1 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::byte
        } else if score >= 20 as i32 {
            (*re).shaderRGBA[1 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::byte;
            (*re).shaderRGBA[0 as i32 as usize] = (*re).shaderRGBA[1 as i32 as usize]
        } else if score >= 10 as i32 {
            (*re).shaderRGBA[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::byte
        } else if score >= 2 as i32 {
            (*re).shaderRGBA[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::byte;
            (*re).shaderRGBA[0 as i32 as usize] = (*re).shaderRGBA[2 as i32 as usize]
        }
    }
    if (c as f64) < 0.25f64 {
        (*re).shaderRGBA[3 as i32 as usize] =
            ((0xff as i32 * 4 as i32) as f32 * c) as crate::src::qcommon::q_shared::byte
    } else {
        (*re).shaderRGBA[3 as i32 as usize] = 0xff as i32 as crate::src::qcommon::q_shared::byte
    }
    (*re).radius = (8 as i32 / 2 as i32) as f32;
    origin[0 as i32 as usize] = (*le).pos.trBase[0 as i32 as usize];
    origin[1 as i32 as usize] = (*le).pos.trBase[1 as i32 as usize];
    origin[2 as i32 as usize] = (*le).pos.trBase[2 as i32 as usize];
    origin[2 as i32 as usize] += 110 as i32 as f32 - c * 100 as i32 as f32;
    dir[0 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize]
        - origin[0 as i32 as usize];
    dir[1 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize]
        - origin[1 as i32 as usize];
    dir[2 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize]
        - origin[2 as i32 as usize];
    CrossProduct(
        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        up.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        vec.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::VectorNormalize(vec.as_mut_ptr());
    origin[0 as i32 as usize] = (origin[0 as i32 as usize] as f64
        + vec[0 as i32 as usize] as f64
            * (-(10 as i32) as f64
                + 20 as i32 as f64
                    * crate::stdlib::sin((c * 2 as i32 as f32) as f64 * 3.14159265358979323846f64)))
        as crate::src::qcommon::q_shared::vec_t;
    origin[1 as i32 as usize] = (origin[1 as i32 as usize] as f64
        + vec[1 as i32 as usize] as f64
            * (-(10 as i32) as f64
                + 20 as i32 as f64
                    * crate::stdlib::sin((c * 2 as i32 as f32) as f64 * 3.14159265358979323846f64)))
        as crate::src::qcommon::q_shared::vec_t;
    origin[2 as i32 as usize] = (origin[2 as i32 as usize] as f64
        + vec[2 as i32 as usize] as f64
            * (-(10 as i32) as f64
                + 20 as i32 as f64
                    * crate::stdlib::sin((c * 2 as i32 as f32) as f64 * 3.14159265358979323846f64)))
        as crate::src::qcommon::q_shared::vec_t;
    // if the view would be "inside" the sprite, kill the sprite
    // so it doesn't add too much overdraw
    delta[0 as i32 as usize] = origin[0 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize];
    delta[1 as i32 as usize] = origin[1 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize];
    delta[2 as i32 as usize] = origin[2 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize];
    len = VectorLength(delta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    if len < 20 as i32 as f32 {
        CG_FreeLocalEntity(le);
        return;
    }
    negative = crate::src::qcommon::q_shared::qfalse as i32;
    if score < 0 as i32 {
        negative = crate::src::qcommon::q_shared::qtrue as i32;
        score = -score
    }
    numdigits = 0 as i32;
    while !(numdigits != 0 && score == 0) {
        digits[numdigits as usize] = score % 10 as i32;
        score = score / 10 as i32;
        numdigits += 1
    }
    if negative != 0 {
        digits[numdigits as usize] = 10 as i32;
        numdigits += 1
    }
    i = 0 as i32;
    while i < numdigits {
        (*re).origin[0 as i32 as usize] = origin[0 as i32 as usize]
            + vec[0 as i32 as usize]
                * ((numdigits as f32 / 2 as i32 as f32 - i as f32) * 8 as i32 as f32);
        (*re).origin[1 as i32 as usize] = origin[1 as i32 as usize]
            + vec[1 as i32 as usize]
                * ((numdigits as f32 / 2 as i32 as f32 - i as f32) * 8 as i32 as f32);
        (*re).origin[2 as i32 as usize] = origin[2 as i32 as usize]
            + vec[2 as i32 as usize]
                * ((numdigits as f32 / 2 as i32 as f32 - i as f32) * 8 as i32 as f32);
        (*re).customShader = crate::src::cgame::cg_main::cgs.media.numberShaders
            [digits[(numdigits - 1 as i32 - i) as usize] as usize];
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
            re as *const crate::tr_types_h::refEntity_t,
        );
        i += 1
    }
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
//==============================================================================
/*
===================
CG_AddLocalEntities

===================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddLocalEntities() {
    let mut le: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    let mut next: *mut crate::cg_local_h::localEntity_t =
        0 as *mut crate::cg_local_h::localEntity_t;
    // walk the list backwards, so any new local entities generated
    // (trails, marks, etc) will be present this frame
    le = cg_activeLocalEntities.prev;
    while le != &mut cg_activeLocalEntities as *mut crate::cg_local_h::localEntity_t {
        // grab next now, so if the local entity is freed we
        // still have it
        next = (*le).prev;
        if crate::src::cgame::cg_main::cg.time >= (*le).endTime {
            CG_FreeLocalEntity(le);
        } else {
            match (*le).leType as u32 {
                0 => {}
                2 => {
                    CG_AddSpriteExplosion(le);
                }
                1 => {
                    CG_AddExplosion(le);
                }
                3 => {
                    // gibs and brass
                    CG_AddFragment(le);
                }
                4 => {
                    // water bubbles
                    CG_AddMoveScaleFade(le);
                }
                6 => {
                    // teleporters, railtrails
                    CG_AddFadeRGB(le);
                }
                5 => {
                    // gib blood trails
                    CG_AddFallScaleFade(le);
                }
                7 => {
                    // rocket trails
                    CG_AddScaleFade(le);
                }
                8 => {
                    CG_AddScorePlum(le);
                }
                _ => {
                    crate::src::cgame::cg_main::CG_Error(
                        b"Bad leType: %i\x00" as *const u8 as *const libc::c_char,
                        (*le).leType as u32,
                    );
                }
            }
        }
        le = next
    }
}

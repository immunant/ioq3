use ::libc;

pub mod q_shared_h {

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
pub use crate::src::cgame::cg_marks::q_shared_h::CrossProduct;
pub use crate::src::qcommon::q_math::PerpendicularVector;
pub use crate::src::qcommon::q_math::RotatePointAroundVector;
pub use crate::src::qcommon::q_math::VectorNormalize2;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::markFragment_t;
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
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::polyVert_t;
pub use crate::tr_types_h::poly_s;
pub use crate::tr_types_h::poly_t;
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
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::markPoly_s;
pub use crate::cg_local_h::markPoly_t;
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
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_addMarks;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_Error;
pub use crate::src::cgame::cg_syscalls::trap_CM_MarkFragments;
pub use crate::src::cgame::cg_syscalls::trap_R_AddPolyToScene;

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
// cg_marks.c -- wall marks
/*
===================================================================

MARK POLYS

===================================================================
*/
#[no_mangle]

pub static mut cg_activeMarkPolys: markPoly_t = markPoly_t {
    prevMark: std::ptr::null_mut(),
    nextMark: std::ptr::null_mut(),
    time: 0,
    markShader: 0,
    alphaFade: qfalse,
    color: [0.; 4],
    poly: poly_t {
        hShader: 0,
        numVerts: 0,
        verts: std::ptr::null_mut(),
    },
    verts: [polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 10],
};
// double linked list
#[no_mangle]

pub static mut cg_freeMarkPolys: *mut markPoly_t = std::ptr::null_mut();
// single linked list
#[no_mangle]

pub static mut cg_markPolys: [markPoly_t; 256] = [markPoly_t {
    prevMark: std::ptr::null_mut(),
    nextMark: std::ptr::null_mut(),
    time: 0,
    markShader: 0,
    alphaFade: qfalse,
    color: [0.; 4],
    poly: poly_t {
        hShader: 0,
        numVerts: 0,
        verts: std::ptr::null_mut(),
    },
    verts: [polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 10],
}; 256];

static mut markTotal: i32 = 0;
/*
===================
CG_InitMarkPolys

This is called at startup and for tournement restarts
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_InitMarkPolys() {
    let mut i: i32 = 0;
    crate::stdlib::memset(
        cg_markPolys.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[markPoly_t; 256]>() as usize,
    );
    cg_activeMarkPolys.nextMark = &mut cg_activeMarkPolys;
    cg_activeMarkPolys.prevMark = &mut cg_activeMarkPolys;
    cg_freeMarkPolys = cg_markPolys.as_mut_ptr();
    i = 0 as i32;
    while i < 256 as i32 - 1 as i32 {
        cg_markPolys[i as usize].nextMark =
            &mut *cg_markPolys.as_mut_ptr().offset((i + 1 as i32) as isize) as *mut markPoly_t;
        i += 1
    }
}
/*
==================
CG_FreeMarkPoly
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_FreeMarkPoly(mut le: *mut markPoly_t) {
    if (*le).prevMark.is_null() || (*le).nextMark.is_null() {
        CG_Error(b"CG_FreeLocalEntity: not active\x00" as *const u8 as *const libc::c_char);
    }
    // remove from the doubly linked active list
    (*(*le).prevMark).nextMark = (*le).nextMark;
    (*(*le).nextMark).prevMark = (*le).prevMark;
    // the free list is only singly linked
    (*le).nextMark = cg_freeMarkPolys;
    cg_freeMarkPolys = le;
}
/*
===================
CG_AllocMark

Will allways succeed, even if it requires freeing an old active mark
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AllocMark() -> *mut markPoly_t {
    let mut le: *mut markPoly_t = 0 as *mut markPoly_t;
    let mut time: i32 = 0;
    if cg_freeMarkPolys.is_null() {
        // no free entities, so free the one at the end of the chain
        // remove the oldest active entity
        time = (*cg_activeMarkPolys.prevMark).time;
        while !cg_activeMarkPolys.prevMark.is_null() && time == (*cg_activeMarkPolys.prevMark).time
        {
            CG_FreeMarkPoly(cg_activeMarkPolys.prevMark);
        }
    }
    le = cg_freeMarkPolys;
    cg_freeMarkPolys = (*cg_freeMarkPolys).nextMark;
    crate::stdlib::memset(
        le as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<markPoly_t>() as usize,
    );
    // link into the active list
    (*le).nextMark = cg_activeMarkPolys.nextMark;
    (*le).prevMark = &mut cg_activeMarkPolys;
    (*cg_activeMarkPolys.nextMark).prevMark = le;
    cg_activeMarkPolys.nextMark = le;
    return le;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ImpactMark(
    mut markShader: qhandle_t,
    mut origin: *const vec_t,
    mut dir: *const vec_t,
    mut orientation: f32,
    mut red: f32,
    mut green: f32,
    mut blue: f32,
    mut alpha: f32,
    mut alphaFade: qboolean,
    mut radius: f32,
    mut temporary: qboolean,
) {
    let mut axis: [vec3_t; 3] = [[0.; 3]; 3];
    let mut texCoordScale: f32 = 0.;
    let mut originalPoints: [vec3_t; 4] = [[0.; 3]; 4];
    let mut colors: [byte; 4] = [0; 4];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut numFragments: i32 = 0;
    let mut markFragments: [markFragment_t; 128] = [markFragment_t {
        firstPoint: 0,
        numPoints: 0,
    }; 128];
    let mut mf: *mut markFragment_t = 0 as *mut markFragment_t;
    let mut markPoints: [vec3_t; 384] = [[0.; 3]; 384];
    let mut projection: vec3_t = [0.; 3];
    if cg_addMarks.integer == 0 {
        return;
    }
    if radius <= 0 as i32 as f32 {
        CG_Error(b"CG_ImpactMark called with <= 0 radius\x00" as *const u8 as *const libc::c_char);
    }
    //if ( markTotal >= MAX_MARK_POLYS ) {
    //	return;
    //}
    // create the texture axis
    VectorNormalize2(dir, axis[0 as i32 as usize].as_mut_ptr());
    PerpendicularVector(
        axis[1 as i32 as usize].as_mut_ptr(),
        axis[0 as i32 as usize].as_mut_ptr() as *const vec_t,
    );
    RotatePointAroundVector(
        axis[2 as i32 as usize].as_mut_ptr(),
        axis[0 as i32 as usize].as_mut_ptr() as *const vec_t,
        axis[1 as i32 as usize].as_mut_ptr() as *const vec_t,
        orientation,
    );
    CrossProduct(
        axis[0 as i32 as usize].as_mut_ptr() as *const vec_t,
        axis[2 as i32 as usize].as_mut_ptr() as *const vec_t,
        axis[1 as i32 as usize].as_mut_ptr(),
    );
    texCoordScale = (0.5f64 * 1.0f64 / radius as f64) as f32;
    // create the full polygon
    i = 0 as i32;
    while i < 3 as i32 {
        originalPoints[0 as i32 as usize][i as usize] = *origin.offset(i as isize)
            - radius * axis[1 as i32 as usize][i as usize]
            - radius * axis[2 as i32 as usize][i as usize];
        originalPoints[1 as i32 as usize][i as usize] = *origin.offset(i as isize)
            + radius * axis[1 as i32 as usize][i as usize]
            - radius * axis[2 as i32 as usize][i as usize];
        originalPoints[2 as i32 as usize][i as usize] = *origin.offset(i as isize)
            + radius * axis[1 as i32 as usize][i as usize]
            + radius * axis[2 as i32 as usize][i as usize];
        originalPoints[3 as i32 as usize][i as usize] = *origin.offset(i as isize)
            - radius * axis[1 as i32 as usize][i as usize]
            + radius * axis[2 as i32 as usize][i as usize];
        i += 1
    }
    // get the fragments
    projection[0 as i32 as usize] = *dir.offset(0 as i32 as isize) * -(20 as i32) as f32;
    projection[1 as i32 as usize] = *dir.offset(1 as i32 as isize) * -(20 as i32) as f32;
    projection[2 as i32 as usize] = *dir.offset(2 as i32 as isize) * -(20 as i32) as f32;
    numFragments = trap_CM_MarkFragments(
        4 as i32,
        originalPoints.as_mut_ptr() as *mut libc::c_void as *const vec3_t,
        projection.as_mut_ptr() as *const vec_t,
        384 as i32,
        markPoints[0 as i32 as usize].as_mut_ptr(),
        128 as i32,
        markFragments.as_mut_ptr() as *mut markFragment_t,
    );
    colors[0 as i32 as usize] = (red * 255 as i32 as f32) as byte;
    colors[1 as i32 as usize] = (green * 255 as i32 as f32) as byte;
    colors[2 as i32 as usize] = (blue * 255 as i32 as f32) as byte;
    colors[3 as i32 as usize] = (alpha * 255 as i32 as f32) as byte;
    i = 0 as i32;
    mf = markFragments.as_mut_ptr();
    while i < numFragments {
        let mut v: *mut polyVert_t = 0 as *mut polyVert_t;
        let mut verts: [polyVert_t; 10] = [polyVert_t {
            xyz: [0.; 3],
            st: [0.; 2],
            modulate: [0; 4],
        }; 10];
        let mut mark: *mut markPoly_t = 0 as *mut markPoly_t;
        // we have an upper limit on the complexity of polygons
        // that we store persistantly
        if (*mf).numPoints > 10 as i32 {
            (*mf).numPoints = 10 as i32
        }
        j = 0 as i32;
        v = verts.as_mut_ptr();
        while j < (*mf).numPoints {
            let mut delta: vec3_t = [0.; 3];
            (*v).xyz[0 as i32 as usize] =
                markPoints[((*mf).firstPoint + j) as usize][0 as i32 as usize];
            (*v).xyz[1 as i32 as usize] =
                markPoints[((*mf).firstPoint + j) as usize][1 as i32 as usize];
            (*v).xyz[2 as i32 as usize] =
                markPoints[((*mf).firstPoint + j) as usize][2 as i32 as usize];
            delta[0 as i32 as usize] =
                (*v).xyz[0 as i32 as usize] - *origin.offset(0 as i32 as isize);
            delta[1 as i32 as usize] =
                (*v).xyz[1 as i32 as usize] - *origin.offset(1 as i32 as isize);
            delta[2 as i32 as usize] =
                (*v).xyz[2 as i32 as usize] - *origin.offset(2 as i32 as isize);
            (*v).st[0 as i32 as usize] = (0.5f64
                + ((delta[0 as i32 as usize] * axis[1 as i32 as usize][0 as i32 as usize]
                    + delta[1 as i32 as usize] * axis[1 as i32 as usize][1 as i32 as usize]
                    + delta[2 as i32 as usize] * axis[1 as i32 as usize][2 as i32 as usize])
                    * texCoordScale) as f64) as f32;
            (*v).st[1 as i32 as usize] = (0.5f64
                + ((delta[0 as i32 as usize] * axis[2 as i32 as usize][0 as i32 as usize]
                    + delta[1 as i32 as usize] * axis[2 as i32 as usize][1 as i32 as usize]
                    + delta[2 as i32 as usize] * axis[2 as i32 as usize][2 as i32 as usize])
                    * texCoordScale) as f64) as f32;
            *((*v).modulate.as_mut_ptr() as *mut i32) = *(colors.as_mut_ptr() as *mut i32);
            j += 1;
            v = v.offset(1)
        }
        // if it is a temporary (shadow) mark, add it immediately and forget about it
        if temporary as u64 != 0 {
            trap_R_AddPolyToScene(
                markShader,
                (*mf).numPoints,
                verts.as_mut_ptr() as *const polyVert_t,
            );
        } else {
            // otherwise save it persistantly
            mark = CG_AllocMark();
            (*mark).time = cg.time;
            (*mark).alphaFade = alphaFade;
            (*mark).markShader = markShader;
            (*mark).poly.numVerts = (*mf).numPoints;
            (*mark).color[0 as i32 as usize] = red;
            (*mark).color[1 as i32 as usize] = green;
            (*mark).color[2 as i32 as usize] = blue;
            (*mark).color[3 as i32 as usize] = alpha;
            crate::stdlib::memcpy(
                (*mark).verts.as_mut_ptr() as *mut libc::c_void,
                verts.as_mut_ptr() as *const libc::c_void,
                ((*mf).numPoints as usize)
                    .wrapping_mul(::std::mem::size_of::<polyVert_t>() as usize),
            );
            markTotal += 1
        }
        i += 1;
        mf = mf.offset(1)
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
#[no_mangle]

pub unsafe extern "C" fn CG_AddMarks() {
    let mut j: i32 = 0;
    let mut mp: *mut markPoly_t = 0 as *mut markPoly_t;
    let mut next: *mut markPoly_t = 0 as *mut markPoly_t;
    let mut t: i32 = 0;
    let mut fade: i32 = 0;
    if cg_addMarks.integer == 0 {
        return;
    }
    mp = cg_activeMarkPolys.nextMark;
    while mp != &mut cg_activeMarkPolys as *mut markPoly_t {
        // grab next now, so if the local entity is freed we
        // still have it
        next = (*mp).nextMark;
        // see if it is time to completely remove it
        if cg.time > (*mp).time + 10000 as i32 {
            CG_FreeMarkPoly(mp);
        } else {
            // fade out the energy bursts
            if (*mp).markShader == cgs.media.energyMarkShader {
                fade = (450 as i32 as f64
                    - 450 as i32 as f64 * ((cg.time - (*mp).time) as f64 / 3000.0f64))
                    as i32;
                if fade < 255 as i32 {
                    if fade < 0 as i32 {
                        fade = 0 as i32
                    }
                    if (*mp).verts[0 as i32 as usize].modulate[0 as i32 as usize] as i32 != 0 as i32
                    {
                        j = 0 as i32;
                        while j < (*mp).poly.numVerts {
                            (*mp).verts[j as usize].modulate[0 as i32 as usize] =
                                ((*mp).color[0 as i32 as usize] * fade as f32) as byte;
                            (*mp).verts[j as usize].modulate[1 as i32 as usize] =
                                ((*mp).color[1 as i32 as usize] * fade as f32) as byte;
                            (*mp).verts[j as usize].modulate[2 as i32 as usize] =
                                ((*mp).color[2 as i32 as usize] * fade as f32) as byte;
                            j += 1
                        }
                    }
                }
            }
            // fade all marks out with time
            t = (*mp).time + 10000 as i32 - cg.time;
            if t < 1000 as i32 {
                fade = 255 as i32 * t / 1000 as i32;
                if (*mp).alphaFade as u64 != 0 {
                    j = 0 as i32;
                    while j < (*mp).poly.numVerts {
                        (*mp).verts[j as usize].modulate[3 as i32 as usize] = fade as byte;
                        j += 1
                    }
                } else {
                    j = 0 as i32;
                    while j < (*mp).poly.numVerts {
                        (*mp).verts[j as usize].modulate[0 as i32 as usize] =
                            ((*mp).color[0 as i32 as usize] * fade as f32) as byte;
                        (*mp).verts[j as usize].modulate[1 as i32 as usize] =
                            ((*mp).color[1 as i32 as usize] * fade as f32) as byte;
                        (*mp).verts[j as usize].modulate[2 as i32 as usize] =
                            ((*mp).color[2 as i32 as usize] * fade as f32) as byte;
                        j += 1
                    }
                }
            }
            trap_R_AddPolyToScene(
                (*mp).markShader,
                (*mp).poly.numVerts,
                (*mp).verts.as_mut_ptr() as *const polyVert_t,
            );
        }
        mp = next
    }
}

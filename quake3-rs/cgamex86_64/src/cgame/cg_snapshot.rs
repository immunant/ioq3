use ::libc;

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
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
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
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
pub use crate::src::cgame::cg_draw::CG_AddLagometerSnapshotInfo;
pub use crate::src::cgame::cg_event::CG_CheckEvents;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_entities;
pub use crate::src::cgame::cg_main::cg_nopredict;
pub use crate::src::cgame::cg_main::cg_synchronousClients;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_Error;
pub use crate::src::cgame::cg_main::CG_Printf;
pub use crate::src::cgame::cg_players::CG_ResetPlayerEntity;
pub use crate::src::cgame::cg_playerstate::CG_Respawn;
pub use crate::src::cgame::cg_playerstate::CG_TransitionPlayerState;
pub use crate::src::cgame::cg_predict::CG_BuildSolidList;
pub use crate::src::cgame::cg_servercmds::CG_ExecuteNewServerCommands;
pub use crate::src::cgame::cg_syscalls::trap_GetCurrentSnapshotNumber;
pub use crate::src::cgame::cg_syscalls::trap_GetSnapshot;
pub use crate::src::game::bg_misc::BG_PlayerStateToEntityState;
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
// cg_snapshot.c -- things that happen on snapshot transition,
// not necessarily every single rendered frame
/*
==================
CG_ResetEntity
==================
*/

unsafe extern "C" fn CG_ResetEntity(mut cent: *mut crate::cg_local_h::centity_t) {
    // if the previous snapshot this entity was updated in is at least
    // an event window back in time then we can reset the previous event
    if (*cent).snapShotTime < crate::src::cgame::cg_main::cg.time - 300 as libc::c_int {
        (*cent).previousEvent = 0 as libc::c_int
    }
    (*cent).trailTime = (*crate::src::cgame::cg_main::cg.snap).serverTime;
    (*cent).lerpOrigin[0 as libc::c_int as usize] =
        (*cent).currentState.origin[0 as libc::c_int as usize];
    (*cent).lerpOrigin[1 as libc::c_int as usize] =
        (*cent).currentState.origin[1 as libc::c_int as usize];
    (*cent).lerpOrigin[2 as libc::c_int as usize] =
        (*cent).currentState.origin[2 as libc::c_int as usize];
    (*cent).lerpAngles[0 as libc::c_int as usize] =
        (*cent).currentState.angles[0 as libc::c_int as usize];
    (*cent).lerpAngles[1 as libc::c_int as usize] =
        (*cent).currentState.angles[1 as libc::c_int as usize];
    (*cent).lerpAngles[2 as libc::c_int as usize] =
        (*cent).currentState.angles[2 as libc::c_int as usize];
    if (*cent).currentState.eType == crate::bg_public_h::ET_PLAYER as libc::c_int {
        crate::src::cgame::cg_players::CG_ResetPlayerEntity(
            cent as *mut crate::cg_local_h::centity_s,
        );
    };
}
/*
===============
CG_TransitionEntity

cent->nextState is moved to cent->currentState and events are fired
===============
*/

unsafe extern "C" fn CG_TransitionEntity(mut cent: *mut crate::cg_local_h::centity_t) {
    (*cent).currentState = (*cent).nextState;
    (*cent).currentValid = crate::src::qcommon::q_shared::qtrue;
    // reset if the entity wasn't in the last frame or was teleported
    if (*cent).interpolate as u64 == 0 {
        CG_ResetEntity(cent);
    }
    // clear the next state.  if will be set by the next CG_SetNextSnap
    (*cent).interpolate = crate::src::qcommon::q_shared::qfalse;
    // check for events
    crate::src::cgame::cg_event::CG_CheckEvents(cent as *mut crate::cg_local_h::centity_s);
}
/*
==================
CG_SetInitialSnapshot

This will only happen on the very first snapshot.
All other times will use CG_TransitionSnapshot instead.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_SetInitialSnapshot(mut snap: *mut crate::cg_public_h::snapshot_t) {
    let mut i: libc::c_int = 0;
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    let mut state: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    crate::src::cgame::cg_main::cg.snap = snap;
    crate::src::game::bg_misc::BG_PlayerStateToEntityState(
        &mut (*snap).ps as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
        &mut (*crate::src::cgame::cg_main::cg_entities
            .as_mut_ptr()
            .offset((*snap).ps.clientNum as isize))
        .currentState as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
        crate::src::qcommon::q_shared::qfalse,
    );
    // sort out solid entities
    crate::src::cgame::cg_predict::CG_BuildSolidList();
    crate::src::cgame::cg_servercmds::CG_ExecuteNewServerCommands((*snap).serverCommandSequence);
    // set our local weapon selection pointer to
    // what the server has indicated the current weapon is
    crate::src::cgame::cg_playerstate::CG_Respawn();
    i = 0 as libc::c_int;
    while i < (*crate::src::cgame::cg_main::cg.snap).numEntities {
        state = &mut *(*crate::src::cgame::cg_main::cg.snap)
            .entities
            .as_mut_ptr()
            .offset(i as isize)
            as *mut crate::src::qcommon::q_shared::entityState_t;
        cent = &mut *crate::src::cgame::cg_main::cg_entities
            .as_mut_ptr()
            .offset((*state).number as isize) as *mut crate::cg_local_h::centity_t;
        crate::stdlib::memcpy(
            &mut (*cent).currentState as *mut crate::src::qcommon::q_shared::entityState_t
                as *mut libc::c_void,
            state as *const libc::c_void,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>() as libc::c_ulong,
        );
        //cent->currentState = *state;
        (*cent).interpolate = crate::src::qcommon::q_shared::qfalse;
        (*cent).currentValid = crate::src::qcommon::q_shared::qtrue;
        CG_ResetEntity(cent);
        // check for events
        crate::src::cgame::cg_event::CG_CheckEvents(cent as *mut crate::cg_local_h::centity_s);
        i += 1
    }
}
/*
===================
CG_TransitionSnapshot

The transition point from snap to nextSnap has passed
===================
*/

unsafe extern "C" fn CG_TransitionSnapshot() {
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    let mut oldFrame: *mut crate::cg_public_h::snapshot_t =
        0 as *mut crate::cg_public_h::snapshot_t;
    let mut i: libc::c_int = 0;
    if crate::src::cgame::cg_main::cg.snap.is_null() {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_TransitionSnapshot: NULL cg.snap\x00" as *const u8 as *const libc::c_char,
        );
    }
    if crate::src::cgame::cg_main::cg.nextSnap.is_null() {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_TransitionSnapshot: NULL cg.nextSnap\x00" as *const u8 as *const libc::c_char,
        );
    }
    // execute any server string commands before transitioning entities
    crate::src::cgame::cg_servercmds::CG_ExecuteNewServerCommands(
        (*crate::src::cgame::cg_main::cg.nextSnap).serverCommandSequence,
    );
    // if we had a map_restart, set everything with initial
    (crate::src::cgame::cg_main::cg.mapRestart as u64) != 0;
    // clear the currentValid flag for all entities in the existing snapshot
    i = 0 as libc::c_int;
    while i < (*crate::src::cgame::cg_main::cg.snap).numEntities {
        cent = &mut *crate::src::cgame::cg_main::cg_entities.as_mut_ptr().offset(
            (*(*crate::src::cgame::cg_main::cg.snap)
                .entities
                .as_mut_ptr()
                .offset(i as isize))
            .number as isize,
        ) as *mut crate::cg_local_h::centity_t;
        (*cent).currentValid = crate::src::qcommon::q_shared::qfalse;
        i += 1
    }
    // move nextSnap to snap and do the transitions
    oldFrame = crate::src::cgame::cg_main::cg.snap;
    crate::src::cgame::cg_main::cg.snap = crate::src::cgame::cg_main::cg.nextSnap;
    crate::src::game::bg_misc::BG_PlayerStateToEntityState(
        &mut (*crate::src::cgame::cg_main::cg.snap).ps as *mut _
            as *mut crate::src::qcommon::q_shared::playerState_s,
        &mut (*crate::src::cgame::cg_main::cg_entities
            .as_mut_ptr()
            .offset((*crate::src::cgame::cg_main::cg.snap).ps.clientNum as isize))
        .currentState as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
        crate::src::qcommon::q_shared::qfalse,
    );
    crate::src::cgame::cg_main::cg_entities
        [(*crate::src::cgame::cg_main::cg.snap).ps.clientNum as usize]
        .interpolate = crate::src::qcommon::q_shared::qfalse;
    i = 0 as libc::c_int;
    while i < (*crate::src::cgame::cg_main::cg.snap).numEntities {
        cent = &mut *crate::src::cgame::cg_main::cg_entities.as_mut_ptr().offset(
            (*(*crate::src::cgame::cg_main::cg.snap)
                .entities
                .as_mut_ptr()
                .offset(i as isize))
            .number as isize,
        ) as *mut crate::cg_local_h::centity_t;
        CG_TransitionEntity(cent);
        // remember time of snapshot this entity was last updated in
        (*cent).snapShotTime = (*crate::src::cgame::cg_main::cg.snap).serverTime;
        i += 1
    }
    crate::src::cgame::cg_main::cg.nextSnap = 0 as *mut crate::cg_public_h::snapshot_t;
    // check for playerstate transition events
    if !oldFrame.is_null() {
        let mut ops: *mut crate::src::qcommon::q_shared::playerState_t =
            0 as *mut crate::src::qcommon::q_shared::playerState_t;
        let mut ps: *mut crate::src::qcommon::q_shared::playerState_t =
            0 as *mut crate::src::qcommon::q_shared::playerState_t;
        ops = &mut (*oldFrame).ps;
        ps = &mut (*crate::src::cgame::cg_main::cg.snap).ps;
        // teleporting checks are irrespective of prediction
        if ((*ps).eFlags ^ (*ops).eFlags) & 0x4 as libc::c_int != 0 {
            crate::src::cgame::cg_main::cg.thisFrameTeleport = crate::src::qcommon::q_shared::qtrue
            // will be cleared by prediction code
        }
        // if we are not doing client side movement prediction for any
        // reason, then the client events and view changes will be issued now
        if crate::src::cgame::cg_main::cg.demoPlayback as libc::c_uint != 0
            || (*crate::src::cgame::cg_main::cg.snap).ps.pm_flags & 4096 as libc::c_int != 0
            || crate::src::cgame::cg_main::cg_nopredict.integer != 0
            || crate::src::cgame::cg_main::cg_synchronousClients.integer != 0
        {
            crate::src::cgame::cg_playerstate::CG_TransitionPlayerState(
                ps as *mut crate::src::qcommon::q_shared::playerState_s,
                ops as *mut crate::src::qcommon::q_shared::playerState_s,
            );
        }
    };
}
/*
===================
CG_SetNextSnap

A new snapshot has just been read in from the client system.
===================
*/

unsafe extern "C" fn CG_SetNextSnap(mut snap: *mut crate::cg_public_h::snapshot_t) {
    let mut num: libc::c_int = 0;
    let mut es: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    crate::src::cgame::cg_main::cg.nextSnap = snap;
    crate::src::game::bg_misc::BG_PlayerStateToEntityState(
        &mut (*snap).ps as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
        &mut (*crate::src::cgame::cg_main::cg_entities
            .as_mut_ptr()
            .offset((*snap).ps.clientNum as isize))
        .nextState as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
        crate::src::qcommon::q_shared::qfalse,
    );
    crate::src::cgame::cg_main::cg_entities
        [(*crate::src::cgame::cg_main::cg.snap).ps.clientNum as usize]
        .interpolate = crate::src::qcommon::q_shared::qtrue;
    // check for extrapolation errors
    num = 0 as libc::c_int;
    while num < (*snap).numEntities {
        es = &mut *(*snap).entities.as_mut_ptr().offset(num as isize)
            as *mut crate::src::qcommon::q_shared::entityState_t;
        cent = &mut *crate::src::cgame::cg_main::cg_entities
            .as_mut_ptr()
            .offset((*es).number as isize) as *mut crate::cg_local_h::centity_t;
        crate::stdlib::memcpy(
            &mut (*cent).nextState as *mut crate::src::qcommon::q_shared::entityState_t
                as *mut libc::c_void,
            es as *const libc::c_void,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>() as libc::c_ulong,
        );
        //cent->nextState = *es;
        // if this frame is a teleport, or the entity wasn't in the
        // previous frame, don't interpolate
        if (*cent).currentValid as u64 == 0
            || ((*cent).currentState.eFlags ^ (*es).eFlags) & 0x4 as libc::c_int != 0
        {
            (*cent).interpolate = crate::src::qcommon::q_shared::qfalse
        } else {
            (*cent).interpolate = crate::src::qcommon::q_shared::qtrue
        }
        num += 1
    }
    // if the next frame is a teleport for the playerstate, we
    // can't interpolate during demos
    if !crate::src::cgame::cg_main::cg.snap.is_null()
        && ((*snap).ps.eFlags ^ (*crate::src::cgame::cg_main::cg.snap).ps.eFlags)
            & 0x4 as libc::c_int
            != 0
    {
        crate::src::cgame::cg_main::cg.nextFrameTeleport = crate::src::qcommon::q_shared::qtrue
    } else {
        crate::src::cgame::cg_main::cg.nextFrameTeleport = crate::src::qcommon::q_shared::qfalse
    }
    // if changing follow mode, don't interpolate
    if (*crate::src::cgame::cg_main::cg.nextSnap).ps.clientNum
        != (*crate::src::cgame::cg_main::cg.snap).ps.clientNum
    {
        crate::src::cgame::cg_main::cg.nextFrameTeleport = crate::src::qcommon::q_shared::qtrue
    }
    // if changing server restarts, don't interpolate
    if ((*crate::src::cgame::cg_main::cg.nextSnap).snapFlags
        ^ (*crate::src::cgame::cg_main::cg.snap).snapFlags)
        & 4 as libc::c_int
        != 0
    {
        crate::src::cgame::cg_main::cg.nextFrameTeleport = crate::src::qcommon::q_shared::qtrue
    }
    // sort out solid entities
    crate::src::cgame::cg_predict::CG_BuildSolidList();
}
/*
========================
CG_ReadNextSnapshot

This is the only place new snapshots are requested
This may increment cgs.processedSnapshotNum multiple
times if the client system fails to return a
valid snapshot.
========================
*/

unsafe extern "C" fn CG_ReadNextSnapshot() -> *mut crate::cg_public_h::snapshot_t {
    let mut r: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut dest: *mut crate::cg_public_h::snapshot_t = 0 as *mut crate::cg_public_h::snapshot_t;
    if crate::src::cgame::cg_main::cg.latestSnapshotNum
        > crate::src::cgame::cg_main::cgs.processedSnapshotNum + 1000 as libc::c_int
    {
        crate::src::cgame::cg_main::CG_Printf(
            b"WARNING: CG_ReadNextSnapshot: way out of range, %i > %i\n\x00" as *const u8
                as *const libc::c_char,
            crate::src::cgame::cg_main::cg.latestSnapshotNum,
            crate::src::cgame::cg_main::cgs.processedSnapshotNum,
        );
    }
    while crate::src::cgame::cg_main::cgs.processedSnapshotNum
        < crate::src::cgame::cg_main::cg.latestSnapshotNum
    {
        // decide which of the two slots to load it into
        if crate::src::cgame::cg_main::cg.snap
            == &mut *crate::src::cgame::cg_main::cg
                .activeSnapshots
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize)
                as *mut crate::cg_public_h::snapshot_t
        {
            dest = &mut *crate::src::cgame::cg_main::cg
                .activeSnapshots
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize)
                as *mut crate::cg_public_h::snapshot_t
        } else {
            dest = &mut *crate::src::cgame::cg_main::cg
                .activeSnapshots
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize)
                as *mut crate::cg_public_h::snapshot_t
        }
        // If there are additional snapshots, continue trying to
        // read them.
        crate::src::cgame::cg_main::cgs.processedSnapshotNum += 1;
        r = crate::src::cgame::cg_syscalls::trap_GetSnapshot(
            crate::src::cgame::cg_main::cgs.processedSnapshotNum,
            dest as *mut crate::cg_public_h::snapshot_t,
        );
        (!crate::src::cgame::cg_main::cg.snap.is_null() && r as libc::c_uint != 0)
            && (*dest).serverTime == (*crate::src::cgame::cg_main::cg.snap).serverTime;
        if r as u64 != 0 {
            crate::src::cgame::cg_draw::CG_AddLagometerSnapshotInfo(
                dest as *mut crate::cg_public_h::snapshot_t,
            );
            return dest;
        }
        crate::src::cgame::cg_draw::CG_AddLagometerSnapshotInfo(
            0 as *mut crate::cg_public_h::snapshot_t as *mut crate::cg_public_h::snapshot_t,
        );
    }
    // try to read the snapshot from the client system
    // FIXME: why would trap_GetSnapshot return a snapshot with the same server time
    // if it succeeded, return
    // a GetSnapshot will return failure if the snapshot
    // never arrived, or  is so old that its entities
    // have been shoved off the end of the circular
    // buffer in the client system.
    // record as a dropped packet
    // nothing left to read
    return 0 as *mut crate::cg_public_h::snapshot_t;
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
/*
============
CG_ProcessSnapshots

We are trying to set up a renderable view, so determine
what the simulated time is, and try to get snapshots
both before and after that time if available.

If we don't have a valid cg.snap after exiting this function,
then a 3D game view cannot be rendered.  This should only happen
right after the initial connection.  After cg.snap has been valid
once, it will never turn invalid.

Even if cg.snap is valid, cg.nextSnap may not be, if the snapshot
hasn't arrived yet (it becomes an extrapolating situation instead
of an interpolating one)

============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ProcessSnapshots() {
    let mut snap: *mut crate::cg_public_h::snapshot_t = 0 as *mut crate::cg_public_h::snapshot_t;
    let mut n: libc::c_int = 0;
    // see what the latest snapshot the client system has is
    crate::src::cgame::cg_syscalls::trap_GetCurrentSnapshotNumber(
        &mut n,
        &mut crate::src::cgame::cg_main::cg.latestSnapshotTime,
    );
    if n != crate::src::cgame::cg_main::cg.latestSnapshotNum {
        if n < crate::src::cgame::cg_main::cg.latestSnapshotNum {
            // this should never happen
            crate::src::cgame::cg_main::CG_Error(
                b"CG_ProcessSnapshots: n < cg.latestSnapshotNum\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        crate::src::cgame::cg_main::cg.latestSnapshotNum = n
    }
    // If we have yet to receive a snapshot, check for it.
    // Once we have gotten the first snapshot, cg.snap will
    // always have valid data for the rest of the game
    while crate::src::cgame::cg_main::cg.snap.is_null() {
        snap = CG_ReadNextSnapshot();
        if snap.is_null() {
            // we can't continue until we get a snapshot
            return;
        }
        // set our weapon selection to what
        // the playerstate is currently using
        if (*snap).snapFlags & 2 as libc::c_int == 0 {
            CG_SetInitialSnapshot(snap);
        }
    }
    // loop until we either have a valid nextSnap with a serverTime
    // greater than cg.time to interpolate towards, or we run
    // out of available snapshots
    loop
    // if we don't have a nextframe, try and read a new one in
    {
        if crate::src::cgame::cg_main::cg.nextSnap.is_null() {
            snap = CG_ReadNextSnapshot();
            // if we still don't have a nextframe, we will just have to
            // extrapolate
            if snap.is_null() {
                break;
            }
            CG_SetNextSnap(snap);
            // if time went backwards, we have a level restart
            if (*crate::src::cgame::cg_main::cg.nextSnap).serverTime
                < (*crate::src::cgame::cg_main::cg.snap).serverTime
            {
                crate::src::cgame::cg_main::CG_Error(
                    b"CG_ProcessSnapshots: Server time went backwards\x00" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        // if our time is < nextFrame's, we have a nice interpolating state
        if crate::src::cgame::cg_main::cg.time >= (*crate::src::cgame::cg_main::cg.snap).serverTime
            && crate::src::cgame::cg_main::cg.time
                < (*crate::src::cgame::cg_main::cg.nextSnap).serverTime
        {
            break;
        }
        // we have passed the transition from nextFrame to frame
        CG_TransitionSnapshot();
    }
    // assert our valid conditions upon exiting
    if crate::src::cgame::cg_main::cg.snap.is_null() {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_ProcessSnapshots: cg.snap == NULL\x00" as *const u8 as *const libc::c_char,
        );
    }
    if crate::src::cgame::cg_main::cg.time < (*crate::src::cgame::cg_main::cg.snap).serverTime {
        // this can happen right after a vid_restart
        crate::src::cgame::cg_main::cg.time = (*crate::src::cgame::cg_main::cg.snap).serverTime
    }
    if !crate::src::cgame::cg_main::cg.nextSnap.is_null()
        && (*crate::src::cgame::cg_main::cg.nextSnap).serverTime
            <= crate::src::cgame::cg_main::cg.time
    {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_ProcessSnapshots: cg.nextSnap->serverTime <= cg.time\x00" as *const u8
                as *const libc::c_char,
        );
    };
}

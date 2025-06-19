use ::libc;

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> f64 {
        return ::libc::strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    }
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
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::AnglesToAxis;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
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
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
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
pub use crate::tr_types_h::stereoFrame_t;
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
pub use crate::tr_types_h::STEREO_CENTER;
pub use crate::tr_types_h::STEREO_LEFT;
pub use crate::tr_types_h::STEREO_RIGHT;
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
pub use crate::src::cgame::cg_draw::CG_AddLagometerFrameInfo;
pub use crate::src::cgame::cg_draw::CG_DrawActive;
pub use crate::src::cgame::cg_ents::CG_AddPacketEntities;
pub use crate::src::cgame::cg_info::CG_DrawInformation;
pub use crate::src::cgame::cg_localents::CG_AddLocalEntities;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_blood;
pub use crate::src::cgame::cg_main::cg_bobpitch;
pub use crate::src::cgame::cg_main::cg_bobroll;
pub use crate::src::cgame::cg_main::cg_bobup;
pub use crate::src::cgame::cg_main::cg_cameraMode;
pub use crate::src::cgame::cg_main::cg_cameraOrbit;
pub use crate::src::cgame::cg_main::cg_cameraOrbitDelay;
pub use crate::src::cgame::cg_main::cg_errorDecay;
pub use crate::src::cgame::cg_main::cg_fov;
pub use crate::src::cgame::cg_main::cg_gun_x;
pub use crate::src::cgame::cg_main::cg_gun_y;
pub use crate::src::cgame::cg_main::cg_gun_z;
pub use crate::src::cgame::cg_main::cg_runpitch;
pub use crate::src::cgame::cg_main::cg_runroll;
pub use crate::src::cgame::cg_main::cg_stats;
pub use crate::src::cgame::cg_main::cg_thirdPerson;
pub use crate::src::cgame::cg_main::cg_thirdPersonAngle;
pub use crate::src::cgame::cg_main::cg_thirdPersonRange;
pub use crate::src::cgame::cg_main::cg_timescale;
pub use crate::src::cgame::cg_main::cg_timescaleFadeEnd;
pub use crate::src::cgame::cg_main::cg_timescaleFadeSpeed;
pub use crate::src::cgame::cg_main::cg_viewsize;
pub use crate::src::cgame::cg_main::cg_zoomFov;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_Argv;
pub use crate::src::cgame::cg_main::CG_Printf;
pub use crate::src::cgame::cg_main::CG_UpdateCvars;
pub use crate::src::cgame::cg_marks::CG_AddMarks;
pub use crate::src::cgame::cg_particles::CG_AddParticles;
pub use crate::src::cgame::cg_predict::CG_PointContents;
pub use crate::src::cgame::cg_predict::CG_PredictPlayerState;
pub use crate::src::cgame::cg_predict::CG_Trace;
pub use crate::src::cgame::cg_snapshot::CG_ProcessSnapshots;
pub use crate::src::cgame::cg_syscalls::trap_Argc;
pub use crate::src::cgame::cg_syscalls::trap_Cvar_Set;
pub use crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_ClearScene;
pub use crate::src::cgame::cg_syscalls::trap_R_RegisterModel;
pub use crate::src::cgame::cg_syscalls::trap_S_ClearLoopingSounds;
pub use crate::src::cgame::cg_syscalls::trap_S_Respatialize;
pub use crate::src::cgame::cg_syscalls::trap_S_StartLocalSound;
pub use crate::src::cgame::cg_syscalls::trap_S_StartSound;
pub use crate::src::cgame::cg_syscalls::trap_SetUserCmdValue;
pub use crate::src::cgame::cg_view::stdlib_float_h::atof;
pub use crate::src::cgame::cg_weapons::CG_AddViewWeapon;

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
// cg_view.c -- setup all the parameters (position, angle, etc)
// for a 3D rendering
/*
=============================================================================

  MODEL TESTING

The viewthing and gun positioning tools from Q2 have been integrated and
enhanced into a single model testing facility.

Model viewing can begin with either "testmodel <modelname>" or "testgun <modelname>".

The names must be the full pathname after the basedir, like
"models/weapons/v_launch/tris.md3" or "players/male/tris.md3"

Testmodel will create a fake entity 100 units in front of the current view
position, directly facing the viewer.  It will remain immobile, so you can
move around it to view it from different angles.

Testgun will cause the model to follow the player around and suppress the real
view weapon model.  The default frame 0 of most guns is completely off screen,
so you will probably have to cycle a couple frames to see it.

"nextframe", "prevframe", "nextskin", and "prevskin" commands will change the
frame or skin of the testmodel.  These are bound to F5, F6, F7, and F8 in
q3default.cfg.

If a gun is being tested, the "gun_x", "gun_y", and "gun_z" variables will let
you adjust the positioning.

Note that none of the model testing features update while the game is paused, so
it may be convenient to test with deathmatch set to 1 so that bringing down the
console doesn't pause the game.

=============================================================================
*/
/*
=================
CG_TestModel_f

Creates an entity in front of the current position, which
can then be moved around
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_TestModel_f() {
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    crate::src::cgame::cg_main::cg.testGun = crate::src::qcommon::q_shared::qfalse;
    crate::stdlib::memset(
        &mut crate::src::cgame::cg_main::cg.testModelEntity as *mut crate::tr_types_h::refEntity_t
            as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
    );
    if crate::src::cgame::cg_syscalls::trap_Argc() < 2 as i32 {
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        crate::src::cgame::cg_main::cg.testModelName.as_mut_ptr(),
        crate::src::cgame::cg_main::CG_Argv(1 as i32),
        64 as i32,
    );
    crate::src::cgame::cg_main::cg.testModelEntity.hModel =
        crate::src::cgame::cg_syscalls::trap_R_RegisterModel(
            crate::src::cgame::cg_main::cg.testModelName.as_mut_ptr(),
        );
    if crate::src::cgame::cg_syscalls::trap_Argc() == 3 as i32 {
        crate::src::cgame::cg_main::cg.testModelEntity.backlerp =
            atof(crate::src::cgame::cg_main::CG_Argv(2 as i32)) as f32;
        crate::src::cgame::cg_main::cg.testModelEntity.frame = 1 as i32;
        crate::src::cgame::cg_main::cg.testModelEntity.oldframe = 0 as i32
    }
    if crate::src::cgame::cg_main::cg.testModelEntity.hModel == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"Can\'t register model\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    crate::src::cgame::cg_main::cg.testModelEntity.origin[0 as i32 as usize] =
        crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize]
            + crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as i32 as usize]
                [0 as i32 as usize]
                * 100 as i32 as f32;
    crate::src::cgame::cg_main::cg.testModelEntity.origin[1 as i32 as usize] =
        crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize]
            + crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as i32 as usize]
                [1 as i32 as usize]
                * 100 as i32 as f32;
    crate::src::cgame::cg_main::cg.testModelEntity.origin[2 as i32 as usize] =
        crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize]
            + crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as i32 as usize]
                [2 as i32 as usize]
                * 100 as i32 as f32;
    angles[0 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    angles[1 as i32 as usize] = 180 as i32 as f32
        + crate::src::cgame::cg_main::cg.refdefViewAngles[1 as i32 as usize];
    angles[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::qcommon::q_math::AnglesToAxis(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::cgame::cg_main::cg
            .testModelEntity
            .axis
            .as_mut_ptr(),
    );
}
/*
=================
CG_TestGun_f

Replaces the current view weapon with the given model
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_TestGun_f() {
    CG_TestModel_f();
    if crate::src::cgame::cg_main::cg.testModelEntity.hModel == 0 {
        return;
    }
    crate::src::cgame::cg_main::cg.testGun = crate::src::qcommon::q_shared::qtrue;
    crate::src::cgame::cg_main::cg.testModelEntity.renderfx =
        0x1 as i32 | 0x8 as i32 | 0x4 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn CG_TestModelNextFrame_f() {
    crate::src::cgame::cg_main::cg.testModelEntity.frame += 1;
    crate::src::cgame::cg_main::CG_Printf(
        b"frame %i\n\x00" as *const u8 as *const libc::c_char,
        crate::src::cgame::cg_main::cg.testModelEntity.frame,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_TestModelPrevFrame_f() {
    crate::src::cgame::cg_main::cg.testModelEntity.frame -= 1;
    if crate::src::cgame::cg_main::cg.testModelEntity.frame < 0 as i32 {
        crate::src::cgame::cg_main::cg.testModelEntity.frame = 0 as i32
    }
    crate::src::cgame::cg_main::CG_Printf(
        b"frame %i\n\x00" as *const u8 as *const libc::c_char,
        crate::src::cgame::cg_main::cg.testModelEntity.frame,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_TestModelNextSkin_f() {
    crate::src::cgame::cg_main::cg.testModelEntity.skinNum += 1;
    crate::src::cgame::cg_main::CG_Printf(
        b"skin %i\n\x00" as *const u8 as *const libc::c_char,
        crate::src::cgame::cg_main::cg.testModelEntity.skinNum,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_TestModelPrevSkin_f() {
    crate::src::cgame::cg_main::cg.testModelEntity.skinNum -= 1;
    if crate::src::cgame::cg_main::cg.testModelEntity.skinNum < 0 as i32 {
        crate::src::cgame::cg_main::cg.testModelEntity.skinNum = 0 as i32
    }
    crate::src::cgame::cg_main::CG_Printf(
        b"skin %i\n\x00" as *const u8 as *const libc::c_char,
        crate::src::cgame::cg_main::cg.testModelEntity.skinNum,
    );
}

unsafe extern "C" fn CG_AddTestModel() {
    let mut i: i32 = 0;
    // re-register the model, because the level may have changed
    crate::src::cgame::cg_main::cg.testModelEntity.hModel =
        crate::src::cgame::cg_syscalls::trap_R_RegisterModel(
            crate::src::cgame::cg_main::cg.testModelName.as_mut_ptr(),
        );
    if crate::src::cgame::cg_main::cg.testModelEntity.hModel == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"Can\'t register model\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    // if testing a gun, set the origin relative to the view origin
    if crate::src::cgame::cg_main::cg.testGun as u64 != 0 {
        crate::src::cgame::cg_main::cg.testModelEntity.origin[0 as i32 as usize] =
            crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize];
        crate::src::cgame::cg_main::cg.testModelEntity.origin[1 as i32 as usize] =
            crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize];
        crate::src::cgame::cg_main::cg.testModelEntity.origin[2 as i32 as usize] =
            crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize];
        crate::src::cgame::cg_main::cg.testModelEntity.axis[0 as i32 as usize]
            [0 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
            [0 as i32 as usize][0 as i32 as usize];
        crate::src::cgame::cg_main::cg.testModelEntity.axis[0 as i32 as usize]
            [1 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
            [0 as i32 as usize][1 as i32 as usize];
        crate::src::cgame::cg_main::cg.testModelEntity.axis[0 as i32 as usize]
            [2 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
            [0 as i32 as usize][2 as i32 as usize];
        crate::src::cgame::cg_main::cg.testModelEntity.axis[1 as i32 as usize]
            [0 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
            [1 as i32 as usize][0 as i32 as usize];
        crate::src::cgame::cg_main::cg.testModelEntity.axis[1 as i32 as usize]
            [1 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
            [1 as i32 as usize][1 as i32 as usize];
        crate::src::cgame::cg_main::cg.testModelEntity.axis[1 as i32 as usize]
            [2 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
            [1 as i32 as usize][2 as i32 as usize];
        crate::src::cgame::cg_main::cg.testModelEntity.axis[2 as i32 as usize]
            [0 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
            [2 as i32 as usize][0 as i32 as usize];
        crate::src::cgame::cg_main::cg.testModelEntity.axis[2 as i32 as usize]
            [1 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
            [2 as i32 as usize][1 as i32 as usize];
        crate::src::cgame::cg_main::cg.testModelEntity.axis[2 as i32 as usize]
            [2 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
            [2 as i32 as usize][2 as i32 as usize];
        // allow the position to be adjusted
        i = 0 as i32;
        while i < 3 as i32 {
            crate::src::cgame::cg_main::cg.testModelEntity.origin[i as usize] +=
                crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as i32 as usize]
                    [i as usize]
                    * crate::src::cgame::cg_main::cg_gun_x.value;
            crate::src::cgame::cg_main::cg.testModelEntity.origin[i as usize] +=
                crate::src::cgame::cg_main::cg.refdef.viewaxis[1 as i32 as usize]
                    [i as usize]
                    * crate::src::cgame::cg_main::cg_gun_y.value;
            crate::src::cgame::cg_main::cg.testModelEntity.origin[i as usize] +=
                crate::src::cgame::cg_main::cg.refdef.viewaxis[2 as i32 as usize]
                    [i as usize]
                    * crate::src::cgame::cg_main::cg_gun_z.value;
            i += 1
        }
    }
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        &mut crate::src::cgame::cg_main::cg.testModelEntity as *mut _
            as *const crate::tr_types_h::refEntity_t,
    );
}
//============================================================================
/*
=================
CG_CalcVrect

Sets the coordinates of the rendered window
=================
*/

unsafe extern "C" fn CG_CalcVrect() {
    let mut size: i32 = 0;
    // the intermission should allways be full screen
    if (*crate::src::cgame::cg_main::cg.snap).ps.pm_type
        == crate::bg_public_h::PM_INTERMISSION as i32
    {
        size = 100 as i32
    } else if crate::src::cgame::cg_main::cg_viewsize.integer < 30 as i32 {
        crate::src::cgame::cg_syscalls::trap_Cvar_Set(
            b"cg_viewsize\x00" as *const u8 as *const libc::c_char,
            b"30\x00" as *const u8 as *const libc::c_char,
        );
        size = 30 as i32
    } else if crate::src::cgame::cg_main::cg_viewsize.integer > 100 as i32 {
        crate::src::cgame::cg_syscalls::trap_Cvar_Set(
            b"cg_viewsize\x00" as *const u8 as *const libc::c_char,
            b"100\x00" as *const u8 as *const libc::c_char,
        );
        size = 100 as i32
    } else {
        size = crate::src::cgame::cg_main::cg_viewsize.integer
    }
    crate::src::cgame::cg_main::cg.refdef.width =
        crate::src::cgame::cg_main::cgs.glconfig.vidWidth * size / 100 as i32;
    crate::src::cgame::cg_main::cg.refdef.width &= !(1 as i32);
    crate::src::cgame::cg_main::cg.refdef.height =
        crate::src::cgame::cg_main::cgs.glconfig.vidHeight * size / 100 as i32;
    crate::src::cgame::cg_main::cg.refdef.height &= !(1 as i32);
    crate::src::cgame::cg_main::cg.refdef.x = (crate::src::cgame::cg_main::cgs.glconfig.vidWidth
        - crate::src::cgame::cg_main::cg.refdef.width)
        / 2 as i32;
    crate::src::cgame::cg_main::cg.refdef.y = (crate::src::cgame::cg_main::cgs.glconfig.vidHeight
        - crate::src::cgame::cg_main::cg.refdef.height)
        / 2 as i32;
}
// bound normal viewsize
//==============================================================================
/*
===============
CG_OffsetThirdPersonView

===============
*/

unsafe extern "C" fn CG_OffsetThirdPersonView() {
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut view: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut focusAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
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
    static mut mins: crate::src::qcommon::q_shared::vec3_t = [
        -(4 as i32) as crate::src::qcommon::q_shared::vec_t,
        -(4 as i32) as crate::src::qcommon::q_shared::vec_t,
        -(4 as i32) as crate::src::qcommon::q_shared::vec_t,
    ];
    static mut maxs: crate::src::qcommon::q_shared::vec3_t = [
        4 as i32 as crate::src::qcommon::q_shared::vec_t,
        4 as i32 as crate::src::qcommon::q_shared::vec_t,
        4 as i32 as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut focusPoint: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut focusDist: f32 = 0.;
    let mut forwardScale: f32 = 0.;
    let mut sideScale: f32 = 0.;
    crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize] +=
        crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .viewheight as f32;
    focusAngles[0 as i32 as usize] =
        crate::src::cgame::cg_main::cg.refdefViewAngles[0 as i32 as usize];
    focusAngles[1 as i32 as usize] =
        crate::src::cgame::cg_main::cg.refdefViewAngles[1 as i32 as usize];
    focusAngles[2 as i32 as usize] =
        crate::src::cgame::cg_main::cg.refdefViewAngles[2 as i32 as usize];
    // if dead, look at killer
    if crate::src::cgame::cg_main::cg.predictedPlayerState.stats
        [crate::bg_public_h::STAT_HEALTH as i32 as usize]
        <= 0 as i32
    {
        focusAngles[1 as i32 as usize] = crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .stats[crate::bg_public_h::STAT_DEAD_YAW as i32 as usize]
            as crate::src::qcommon::q_shared::vec_t;
        crate::src::cgame::cg_main::cg.refdefViewAngles[1 as i32 as usize] =
            crate::src::cgame::cg_main::cg.predictedPlayerState.stats
                [crate::bg_public_h::STAT_DEAD_YAW as i32 as usize]
                as crate::src::qcommon::q_shared::vec_t
    }
    if focusAngles[0 as i32 as usize] > 45 as i32 as f32 {
        focusAngles[0 as i32 as usize] =
            45 as i32 as crate::src::qcommon::q_shared::vec_t
        // don't go too far overhead
    }
    crate::src::qcommon::q_math::AngleVectors(
        focusAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    focusPoint[0 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.vieworg
        [0 as i32 as usize]
        + forward[0 as i32 as usize] * 512 as i32 as f32;
    focusPoint[1 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.vieworg
        [1 as i32 as usize]
        + forward[1 as i32 as usize] * 512 as i32 as f32;
    focusPoint[2 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.vieworg
        [2 as i32 as usize]
        + forward[2 as i32 as usize] * 512 as i32 as f32;
    view[0 as i32 as usize] =
        crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize];
    view[1 as i32 as usize] =
        crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize];
    view[2 as i32 as usize] =
        crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize];
    view[2 as i32 as usize] += 8 as i32 as f32;
    crate::src::cgame::cg_main::cg.refdefViewAngles[0 as i32 as usize] =
        (crate::src::cgame::cg_main::cg.refdefViewAngles[0 as i32 as usize]
            as f64
            * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
    crate::src::qcommon::q_math::AngleVectors(
        crate::src::cgame::cg_main::cg.refdefViewAngles.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    forwardScale = crate::stdlib::cos(
        (crate::src::cgame::cg_main::cg_thirdPersonAngle.value
            / 180 as i32 as f32) as f64
            * 3.14159265358979323846f64,
    ) as f32;
    sideScale = crate::stdlib::sin(
        (crate::src::cgame::cg_main::cg_thirdPersonAngle.value
            / 180 as i32 as f32) as f64
            * 3.14159265358979323846f64,
    ) as f32;
    view[0 as i32 as usize] = view[0 as i32 as usize]
        + forward[0 as i32 as usize]
            * (-crate::src::cgame::cg_main::cg_thirdPersonRange.value * forwardScale);
    view[1 as i32 as usize] = view[1 as i32 as usize]
        + forward[1 as i32 as usize]
            * (-crate::src::cgame::cg_main::cg_thirdPersonRange.value * forwardScale);
    view[2 as i32 as usize] = view[2 as i32 as usize]
        + forward[2 as i32 as usize]
            * (-crate::src::cgame::cg_main::cg_thirdPersonRange.value * forwardScale);
    view[0 as i32 as usize] = view[0 as i32 as usize]
        + right[0 as i32 as usize]
            * (-crate::src::cgame::cg_main::cg_thirdPersonRange.value * sideScale);
    view[1 as i32 as usize] = view[1 as i32 as usize]
        + right[1 as i32 as usize]
            * (-crate::src::cgame::cg_main::cg_thirdPersonRange.value * sideScale);
    view[2 as i32 as usize] = view[2 as i32 as usize]
        + right[2 as i32 as usize]
            * (-crate::src::cgame::cg_main::cg_thirdPersonRange.value * sideScale);
    // trace a ray from the origin to the viewpoint to make sure the view isn't
    // in a solid block.  Use an 8 by 8 block to prevent the view from near clipping anything
    if crate::src::cgame::cg_main::cg_cameraMode.integer == 0 {
        crate::src::cgame::cg_predict::CG_Trace(
            &mut trace as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
            crate::src::cgame::cg_main::cg.refdef.vieworg.as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            view.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            crate::src::cgame::cg_main::cg
                .predictedPlayerState
                .clientNum,
            1 as i32,
        );
        if trace.fraction as f64 != 1.0f64 {
            view[0 as i32 as usize] = trace.endpos[0 as i32 as usize];
            view[1 as i32 as usize] = trace.endpos[1 as i32 as usize];
            view[2 as i32 as usize] = trace.endpos[2 as i32 as usize];
            view[2 as i32 as usize] = (view[2 as i32 as usize] as f64
                + (1.0f64 - trace.fraction as f64) * 32 as i32 as f64)
                as crate::src::qcommon::q_shared::vec_t;
            // try another trace to this position, because a tunnel may have the ceiling
            // close enough that this is poking out
            crate::src::cgame::cg_predict::CG_Trace(
                &mut trace as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
                crate::src::cgame::cg_main::cg.refdef.vieworg.as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                view.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::cgame::cg_main::cg
                    .predictedPlayerState
                    .clientNum,
                1 as i32,
            );
            view[0 as i32 as usize] = trace.endpos[0 as i32 as usize];
            view[1 as i32 as usize] = trace.endpos[1 as i32 as usize];
            view[2 as i32 as usize] = trace.endpos[2 as i32 as usize]
        }
    }
    crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize] =
        view[0 as i32 as usize];
    crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize] =
        view[1 as i32 as usize];
    crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize] =
        view[2 as i32 as usize];
    // select pitch to look at focus point from vieword
    focusPoint[0 as i32 as usize] = focusPoint[0 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize];
    focusPoint[1 as i32 as usize] = focusPoint[1 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize];
    focusPoint[2 as i32 as usize] = focusPoint[2 as i32 as usize]
        - crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize];
    focusDist = crate::stdlib::sqrt(
        (focusPoint[0 as i32 as usize] * focusPoint[0 as i32 as usize]
            + focusPoint[1 as i32 as usize] * focusPoint[1 as i32 as usize])
            as f64,
    ) as f32;
    if focusDist < 1 as i32 as f32 {
        focusDist = 1 as i32 as f32
        // should never happen
    }
    crate::src::cgame::cg_main::cg.refdefViewAngles[0 as i32 as usize] =
        (-(180 as i32) as f64 / 3.14159265358979323846f64
            * crate::stdlib::atan2(
                focusPoint[2 as i32 as usize] as f64,
                focusDist as f64,
            )) as crate::src::qcommon::q_shared::vec_t;
    crate::src::cgame::cg_main::cg.refdefViewAngles[1 as i32 as usize] -=
        crate::src::cgame::cg_main::cg_thirdPersonAngle.value;
}
// this causes a compiler bug on mac MrC compiler

unsafe extern "C" fn CG_StepOffset() {
    let mut timeDelta: i32 = 0;
    // smooth out stair climbing
    timeDelta = crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.stepTime;
    if timeDelta < 200 as i32 {
        crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize] -=
            crate::src::cgame::cg_main::cg.stepChange
                * (200 as i32 - timeDelta) as f32
                / 200 as i32 as f32
    };
}
/*
===============
CG_OffsetFirstPersonView

===============
*/

unsafe extern "C" fn CG_OffsetFirstPersonView() {
    let mut origin: *mut f32 = 0 as *mut f32;
    let mut angles: *mut f32 = 0 as *mut f32;
    let mut bob: f32 = 0.;
    let mut ratio: f32 = 0.;
    let mut delta: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut f: f32 = 0.;
    let mut predictedVelocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut timeDelta: i32 = 0;
    if (*crate::src::cgame::cg_main::cg.snap).ps.pm_type
        == crate::bg_public_h::PM_INTERMISSION as i32
    {
        return;
    }
    origin = crate::src::cgame::cg_main::cg.refdef.vieworg.as_mut_ptr();
    angles = crate::src::cgame::cg_main::cg.refdefViewAngles.as_mut_ptr();
    // if dead, fix the angle and don't add any kick
    if (*crate::src::cgame::cg_main::cg.snap).ps.stats
        [crate::bg_public_h::STAT_HEALTH as i32 as usize]
        <= 0 as i32
    {
        *angles.offset(2 as i32 as isize) = 40 as i32 as f32;
        *angles.offset(0 as i32 as isize) = -(15 as i32) as f32;
        *angles.offset(1 as i32 as isize) = (*crate::src::cgame::cg_main::cg.snap).ps.stats
            [crate::bg_public_h::STAT_DEAD_YAW as i32 as usize]
            as f32;
        *origin.offset(2 as i32 as isize) += crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .viewheight as f32;
        return;
    }
    // add angles based on damage kick
    if crate::src::cgame::cg_main::cg.damageTime != 0. {
        ratio = crate::src::cgame::cg_main::cg.time as f32
            - crate::src::cgame::cg_main::cg.damageTime;
        if ratio < 100 as i32 as f32 {
            ratio /= 100 as i32 as f32;
            *angles.offset(0 as i32 as isize) +=
                ratio * crate::src::cgame::cg_main::cg.v_dmg_pitch;
            *angles.offset(2 as i32 as isize) +=
                ratio * crate::src::cgame::cg_main::cg.v_dmg_roll
        } else {
            ratio = (1.0f64
                - ((ratio - 100 as i32 as f32)
                    / 400 as i32 as f32) as f64)
                as f32;
            if ratio > 0 as i32 as f32 {
                *angles.offset(0 as i32 as isize) +=
                    ratio * crate::src::cgame::cg_main::cg.v_dmg_pitch;
                *angles.offset(2 as i32 as isize) +=
                    ratio * crate::src::cgame::cg_main::cg.v_dmg_roll
            }
        }
    }
    // add pitch based on fall kick
    // add angles based on velocity
    predictedVelocity[0 as i32 as usize] =
        crate::src::cgame::cg_main::cg.predictedPlayerState.velocity[0 as i32 as usize];
    predictedVelocity[1 as i32 as usize] =
        crate::src::cgame::cg_main::cg.predictedPlayerState.velocity[1 as i32 as usize];
    predictedVelocity[2 as i32 as usize] =
        crate::src::cgame::cg_main::cg.predictedPlayerState.velocity[2 as i32 as usize];
    delta = predictedVelocity[0 as i32 as usize]
        * crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as i32 as usize]
            [0 as i32 as usize]
        + predictedVelocity[1 as i32 as usize]
            * crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as i32 as usize]
                [1 as i32 as usize]
        + predictedVelocity[2 as i32 as usize]
            * crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as i32 as usize]
                [2 as i32 as usize];
    *angles.offset(0 as i32 as isize) +=
        delta * crate::src::cgame::cg_main::cg_runpitch.value;
    delta = predictedVelocity[0 as i32 as usize]
        * crate::src::cgame::cg_main::cg.refdef.viewaxis[1 as i32 as usize]
            [0 as i32 as usize]
        + predictedVelocity[1 as i32 as usize]
            * crate::src::cgame::cg_main::cg.refdef.viewaxis[1 as i32 as usize]
                [1 as i32 as usize]
        + predictedVelocity[2 as i32 as usize]
            * crate::src::cgame::cg_main::cg.refdef.viewaxis[1 as i32 as usize]
                [2 as i32 as usize];
    *angles.offset(2 as i32 as isize) -=
        delta * crate::src::cgame::cg_main::cg_runroll.value;
    // add angles based on bob
    // make sure the bob is visible even at low speeds
    speed = if crate::src::cgame::cg_main::cg.xyspeed > 200 as i32 as f32 {
        crate::src::cgame::cg_main::cg.xyspeed
    } else {
        200 as i32 as f32
    }; // crouching
    delta = crate::src::cgame::cg_main::cg.bobfracsin
        * crate::src::cgame::cg_main::cg_bobpitch.value
        * speed; // crouching accentuates roll
    if crate::src::cgame::cg_main::cg.predictedPlayerState.pm_flags & 1 as i32 != 0 {
        delta *= 3 as i32 as f32
    }
    *angles.offset(0 as i32 as isize) += delta;
    delta = crate::src::cgame::cg_main::cg.bobfracsin
        * crate::src::cgame::cg_main::cg_bobroll.value
        * speed;
    if crate::src::cgame::cg_main::cg.predictedPlayerState.pm_flags & 1 as i32 != 0 {
        delta *= 3 as i32 as f32
    }
    if crate::src::cgame::cg_main::cg.bobcycle & 1 as i32 != 0 {
        delta = -delta
    }
    *angles.offset(2 as i32 as isize) += delta;
    //===================================
    // add view height
    *origin.offset(2 as i32 as isize) += crate::src::cgame::cg_main::cg
        .predictedPlayerState
        .viewheight as f32;
    // smooth out duck height changes
    timeDelta = crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.duckTime;
    if timeDelta < 100 as i32 {
        crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize] -=
            crate::src::cgame::cg_main::cg.duckChange
                * (100 as i32 - timeDelta) as f32
                / 100 as i32 as f32
    }
    // add bob height
    bob = crate::src::cgame::cg_main::cg.bobfracsin
        * crate::src::cgame::cg_main::cg.xyspeed
        * crate::src::cgame::cg_main::cg_bobup.value;
    if bob > 6 as i32 as f32 {
        bob = 6 as i32 as f32
    }
    *origin.offset(2 as i32 as isize) += bob;
    // add fall height
    delta = (crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.landTime)
        as f32;
    if delta < 150 as i32 as f32 {
        f = delta / 150 as i32 as f32;
        crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize] +=
            crate::src::cgame::cg_main::cg.landChange * f
    } else if delta < (150 as i32 + 300 as i32) as f32 {
        delta -= 150 as i32 as f32;
        f = (1.0f64 - (delta / 300 as i32 as f32) as f64)
            as f32;
        crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize] +=
            crate::src::cgame::cg_main::cg.landChange * f
    }
    // add step offset
    CG_StepOffset();
    // pivot the eye based on a neck length
}
//======================================================================
#[no_mangle]

pub unsafe extern "C" fn CG_ZoomDown_f() {
    if crate::src::cgame::cg_main::cg.zoomed as u64 != 0 {
        return;
    }
    crate::src::cgame::cg_main::cg.zoomed = crate::src::qcommon::q_shared::qtrue;
    crate::src::cgame::cg_main::cg.zoomTime = crate::src::cgame::cg_main::cg.time;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ZoomUp_f() {
    if crate::src::cgame::cg_main::cg.zoomed as u64 == 0 {
        return;
    }
    crate::src::cgame::cg_main::cg.zoomed = crate::src::qcommon::q_shared::qfalse;
    crate::src::cgame::cg_main::cg.zoomTime = crate::src::cgame::cg_main::cg.time;
}

unsafe extern "C" fn CG_CalcFov() -> i32 {
    let mut x: f32 = 0.;
    let mut phase: f32 = 0.;
    let mut v: f32 = 0.;
    let mut contents: i32 = 0;
    let mut fov_x: f32 = 0.;
    let mut fov_y: f32 = 0.;
    let mut zoomFov: f32 = 0.;
    let mut f: f32 = 0.;
    let mut inwater: i32 = 0;
    if crate::src::cgame::cg_main::cg.predictedPlayerState.pm_type
        == crate::bg_public_h::PM_INTERMISSION as i32
    {
        // if in intermission, use a fixed value
        fov_x = 90 as i32 as f32
    } else {
        // user selectable
        if crate::src::cgame::cg_main::cgs.dmflags & 16 as i32 != 0 {
            // dmflag to prevent wide fov for all clients
            fov_x = 90 as i32 as f32
        } else {
            fov_x = crate::src::cgame::cg_main::cg_fov.value;
            if fov_x < 1 as i32 as f32 {
                fov_x = 1 as i32 as f32
            } else if fov_x > 160 as i32 as f32 {
                fov_x = 160 as i32 as f32
            }
        }
        // account for zooms
        zoomFov = crate::src::cgame::cg_main::cg_zoomFov.value;
        if zoomFov < 1 as i32 as f32 {
            zoomFov = 1 as i32 as f32
        } else if zoomFov > 160 as i32 as f32 {
            zoomFov = 160 as i32 as f32
        }
        if crate::src::cgame::cg_main::cg.zoomed as u64 != 0 {
            f = (crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.zoomTime)
                as f32
                / 150 as i32 as f32;
            if f as f64 > 1.0f64 {
                fov_x = zoomFov
            } else {
                fov_x = fov_x + f * (zoomFov - fov_x)
            }
        } else {
            f = (crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.zoomTime)
                as f32
                / 150 as i32 as f32;
            if f as f64 <= 1.0f64 {
                fov_x = zoomFov + f * (fov_x - zoomFov)
            }
        }
    }
    x = (crate::src::cgame::cg_main::cg.refdef.width as f64
        / crate::stdlib::tan(
            (fov_x / 360 as i32 as f32) as f64
                * 3.14159265358979323846f64,
        )) as f32;
    fov_y = crate::stdlib::atan2(
        crate::src::cgame::cg_main::cg.refdef.height as f64,
        x as f64,
    ) as f32;
    fov_y = ((fov_y * 360 as i32 as f32) as f64
        / 3.14159265358979323846f64) as f32;
    // warp if underwater
    contents = crate::src::cgame::cg_predict::CG_PointContents(
        crate::src::cgame::cg_main::cg.refdef.vieworg.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        -(1 as i32),
    );
    if contents & (32 as i32 | 16 as i32 | 8 as i32) != 0 {
        phase = (crate::src::cgame::cg_main::cg.time as f64 / 1000.0f64
            * 0.4f64
            * 3.14159265358979323846f64
            * 2 as i32 as f64) as f32;
        v = (1 as i32 as f64 * crate::stdlib::sin(phase as f64))
            as f32;
        fov_x += v;
        fov_y -= v;
        inwater = crate::src::qcommon::q_shared::qtrue as i32
    } else {
        inwater = crate::src::qcommon::q_shared::qfalse as i32
    }
    // set it
    crate::src::cgame::cg_main::cg.refdef.fov_x = fov_x;
    crate::src::cgame::cg_main::cg.refdef.fov_y = fov_y;
    if crate::src::cgame::cg_main::cg.zoomed as u64 == 0 {
        crate::src::cgame::cg_main::cg.zoomSensitivity = 1 as i32 as f32
    } else {
        crate::src::cgame::cg_main::cg.zoomSensitivity =
            (crate::src::cgame::cg_main::cg.refdef.fov_y as f64 / 75.0f64)
                as f32
    }
    return inwater;
}
/*
===============
CG_DamageBlendBlob

===============
*/

unsafe extern "C" fn CG_DamageBlendBlob() {
    let mut t: i32 = 0;
    let mut maxTime: i32 = 0;
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
    if crate::src::cgame::cg_main::cg_blood.integer == 0 {
        return;
    }
    if crate::src::cgame::cg_main::cg.damageValue == 0. {
        return;
    }
    //if (cg.cameraMode) {
    //	return;
    //}
    // ragePro systems can't fade blends, so don't obscure the screen
    if crate::src::cgame::cg_main::cgs.glconfig.hardwareType as u32
        == crate::tr_types_h::GLHW_RAGEPRO as i32 as u32
    {
        return;
    }
    maxTime = 500 as i32;
    t = (crate::src::cgame::cg_main::cg.time as f32
        - crate::src::cgame::cg_main::cg.damageTime) as i32;
    if t <= 0 as i32 || t >= maxTime {
        return;
    }
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>() as libc::c_ulong,
    );
    ent.reType = crate::tr_types_h::RT_SPRITE;
    ent.renderfx = 0x4 as i32;
    ent.origin[0 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.vieworg
        [0 as i32 as usize]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as i32 as usize]
            [0 as i32 as usize]
            * 8 as i32 as f32;
    ent.origin[1 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.vieworg
        [1 as i32 as usize]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as i32 as usize]
            [1 as i32 as usize]
            * 8 as i32 as f32;
    ent.origin[2 as i32 as usize] = crate::src::cgame::cg_main::cg.refdef.vieworg
        [2 as i32 as usize]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as i32 as usize]
            [2 as i32 as usize]
            * 8 as i32 as f32;
    ent.origin[0 as i32 as usize] = ent.origin[0 as i32 as usize]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[1 as i32 as usize]
            [0 as i32 as usize]
            * (crate::src::cgame::cg_main::cg.damageX * -(8 as i32) as f32);
    ent.origin[1 as i32 as usize] = ent.origin[1 as i32 as usize]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[1 as i32 as usize]
            [1 as i32 as usize]
            * (crate::src::cgame::cg_main::cg.damageX * -(8 as i32) as f32);
    ent.origin[2 as i32 as usize] = ent.origin[2 as i32 as usize]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[1 as i32 as usize]
            [2 as i32 as usize]
            * (crate::src::cgame::cg_main::cg.damageX * -(8 as i32) as f32);
    ent.origin[0 as i32 as usize] = ent.origin[0 as i32 as usize]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[2 as i32 as usize]
            [0 as i32 as usize]
            * (crate::src::cgame::cg_main::cg.damageY * 8 as i32 as f32);
    ent.origin[1 as i32 as usize] = ent.origin[1 as i32 as usize]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[2 as i32 as usize]
            [1 as i32 as usize]
            * (crate::src::cgame::cg_main::cg.damageY * 8 as i32 as f32);
    ent.origin[2 as i32 as usize] = ent.origin[2 as i32 as usize]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[2 as i32 as usize]
            [2 as i32 as usize]
            * (crate::src::cgame::cg_main::cg.damageY * 8 as i32 as f32);
    ent.radius = crate::src::cgame::cg_main::cg.damageValue * 3 as i32 as f32;
    ent.customShader = crate::src::cgame::cg_main::cgs.media.viewBloodShader;
    ent.shaderRGBA[0 as i32 as usize] =
        255 as i32 as crate::src::qcommon::q_shared::byte;
    ent.shaderRGBA[1 as i32 as usize] =
        255 as i32 as crate::src::qcommon::q_shared::byte;
    ent.shaderRGBA[2 as i32 as usize] =
        255 as i32 as crate::src::qcommon::q_shared::byte;
    ent.shaderRGBA[3 as i32 as usize] = (200 as i32 as f64
        * (1.0f64 - (t as f32 / maxTime as f32) as f64))
        as crate::src::qcommon::q_shared::byte;
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(
        &mut ent as *mut _ as *const crate::tr_types_h::refEntity_t,
    );
}
/*
===============
CG_CalcViewValues

Sets cg.refdef view values
===============
*/

unsafe extern "C" fn CG_CalcViewValues() -> i32 {
    let mut ps: *mut crate::src::qcommon::q_shared::playerState_t =
        0 as *mut crate::src::qcommon::q_shared::playerState_t;
    crate::stdlib::memset(
        &mut crate::src::cgame::cg_main::cg.refdef as *mut crate::tr_types_h::refdef_t
            as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_types_h::refdef_t>() as libc::c_ulong,
    );
    // strings for in game rendering
    // Q_strncpyz( cg.refdef.text[0], "Park Ranger", sizeof(cg.refdef.text[0]) );
    // Q_strncpyz( cg.refdef.text[1], "19", sizeof(cg.refdef.text[1]) );
    // calculate size of 3D view
    CG_CalcVrect();
    ps = &mut crate::src::cgame::cg_main::cg.predictedPlayerState;
    /*
        if (cg.cameraMode) {
            vec3_t origin, angles;
            if (trap_getCameraInfo(cg.time, &origin, &angles)) {
                VectorCopy(origin, cg.refdef.vieworg);
                angles[ROLL] = 0;
                VectorCopy(angles, cg.refdefViewAngles);
                AnglesToAxis( cg.refdefViewAngles, cg.refdef.viewaxis );
                return CG_CalcFov();
            } else {
                cg.cameraMode = qfalse;
            }
        }
    */
    // intermission view
    if (*ps).pm_type == crate::bg_public_h::PM_INTERMISSION as i32 {
        crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize] =
            (*ps).origin[0 as i32 as usize];
        crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize] =
            (*ps).origin[1 as i32 as usize];
        crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize] =
            (*ps).origin[2 as i32 as usize];
        crate::src::cgame::cg_main::cg.refdefViewAngles[0 as i32 as usize] =
            (*ps).viewangles[0 as i32 as usize];
        crate::src::cgame::cg_main::cg.refdefViewAngles[1 as i32 as usize] =
            (*ps).viewangles[1 as i32 as usize];
        crate::src::cgame::cg_main::cg.refdefViewAngles[2 as i32 as usize] =
            (*ps).viewangles[2 as i32 as usize];
        crate::src::qcommon::q_math::AnglesToAxis(
            crate::src::cgame::cg_main::cg.refdefViewAngles.as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            crate::src::cgame::cg_main::cg.refdef.viewaxis.as_mut_ptr(),
        );
        return CG_CalcFov();
    }
    crate::src::cgame::cg_main::cg.bobcycle =
        ((*ps).bobCycle & 128 as i32) >> 7 as i32;
    crate::src::cgame::cg_main::cg.bobfracsin = crate::stdlib::fabs(crate::stdlib::sin(
        ((*ps).bobCycle & 127 as i32) as f64 / 127.0f64
            * 3.14159265358979323846f64,
    )) as f32;
    crate::src::cgame::cg_main::cg.xyspeed = crate::stdlib::sqrt(
        ((*ps).velocity[0 as i32 as usize] * (*ps).velocity[0 as i32 as usize]
            + (*ps).velocity[1 as i32 as usize] * (*ps).velocity[1 as i32 as usize])
            as f64,
    ) as f32;
    crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize] =
        (*ps).origin[0 as i32 as usize];
    crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize] =
        (*ps).origin[1 as i32 as usize];
    crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize] =
        (*ps).origin[2 as i32 as usize];
    crate::src::cgame::cg_main::cg.refdefViewAngles[0 as i32 as usize] =
        (*ps).viewangles[0 as i32 as usize];
    crate::src::cgame::cg_main::cg.refdefViewAngles[1 as i32 as usize] =
        (*ps).viewangles[1 as i32 as usize];
    crate::src::cgame::cg_main::cg.refdefViewAngles[2 as i32 as usize] =
        (*ps).viewangles[2 as i32 as usize];
    if crate::src::cgame::cg_main::cg_cameraOrbit.integer != 0 {
        if crate::src::cgame::cg_main::cg.time > crate::src::cgame::cg_main::cg.nextOrbitTime {
            crate::src::cgame::cg_main::cg.nextOrbitTime = crate::src::cgame::cg_main::cg.time
                + crate::src::cgame::cg_main::cg_cameraOrbitDelay.integer;
            crate::src::cgame::cg_main::cg_thirdPersonAngle.value +=
                crate::src::cgame::cg_main::cg_cameraOrbit.value
        }
    }
    // add error decay
    if crate::src::cgame::cg_main::cg_errorDecay.value > 0 as i32 as f32 {
        let mut t: i32 = 0;
        let mut f: f32 = 0.;
        t = crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.predictedErrorTime;
        f = (crate::src::cgame::cg_main::cg_errorDecay.value - t as f32)
            / crate::src::cgame::cg_main::cg_errorDecay.value;
        if f > 0 as i32 as f32 && f < 1 as i32 as f32 {
            crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize] =
                crate::src::cgame::cg_main::cg.refdef.vieworg[0 as i32 as usize]
                    + crate::src::cgame::cg_main::cg.predictedError[0 as i32 as usize] * f;
            crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize] =
                crate::src::cgame::cg_main::cg.refdef.vieworg[1 as i32 as usize]
                    + crate::src::cgame::cg_main::cg.predictedError[1 as i32 as usize] * f;
            crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize] =
                crate::src::cgame::cg_main::cg.refdef.vieworg[2 as i32 as usize]
                    + crate::src::cgame::cg_main::cg.predictedError[2 as i32 as usize] * f
        } else {
            crate::src::cgame::cg_main::cg.predictedErrorTime = 0 as i32
        }
    }
    if crate::src::cgame::cg_main::cg.renderingThirdPerson as u64 != 0 {
        // back away from character
        CG_OffsetThirdPersonView();
    } else {
        // offset for local bobbing and kicks
        CG_OffsetFirstPersonView();
    }
    // position eye relative to origin
    crate::src::qcommon::q_math::AnglesToAxis(
        crate::src::cgame::cg_main::cg.refdefViewAngles.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::cgame::cg_main::cg.refdef.viewaxis.as_mut_ptr(),
    );
    if crate::src::cgame::cg_main::cg.hyperspace as u64 != 0 {
        crate::src::cgame::cg_main::cg.refdef.rdflags |= 0x1 as i32 | 0x4 as i32
    }
    // field of view
    return CG_CalcFov();
}
/*
=====================
CG_PowerupTimerSounds
=====================
*/

unsafe extern "C" fn CG_PowerupTimerSounds() {
    let mut i: i32 = 0;
    let mut t: i32 = 0;
    // powerup timers going away
    i = 0 as i32;
    while i < 16 as i32 {
        t = (*crate::src::cgame::cg_main::cg.snap).ps.powerups[i as usize];
        if !(t <= crate::src::cgame::cg_main::cg.time) {
            if !(t - crate::src::cgame::cg_main::cg.time >= 5 as i32 * 1000 as i32)
            {
                if (t - crate::src::cgame::cg_main::cg.time) / 1000 as i32
                    != (t - crate::src::cgame::cg_main::cg.oldTime) / 1000 as i32
                {
                    crate::src::cgame::cg_syscalls::trap_S_StartSound(
                        0 as *mut crate::src::qcommon::q_shared::vec_t,
                        (*crate::src::cgame::cg_main::cg.snap).ps.clientNum,
                        crate::src::qcommon::q_shared::CHAN_ITEM as i32,
                        crate::src::cgame::cg_main::cgs.media.wearOffSound,
                    );
                }
            }
        }
        i += 1
    }
}
/*
=====================
CG_AddBufferedSound
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddBufferedSound(mut sfx: crate::src::qcommon::q_shared::sfxHandle_t) {
    if sfx == 0 {
        return;
    }
    crate::src::cgame::cg_main::cg.soundBuffer
        [crate::src::cgame::cg_main::cg.soundBufferIn as usize] = sfx;
    crate::src::cgame::cg_main::cg.soundBufferIn =
        (crate::src::cgame::cg_main::cg.soundBufferIn + 1 as i32) % 20 as i32;
    if crate::src::cgame::cg_main::cg.soundBufferIn == crate::src::cgame::cg_main::cg.soundBufferOut
    {
        crate::src::cgame::cg_main::cg.soundBufferOut += 1
    };
}
/*
=====================
CG_PlayBufferedSounds
=====================
*/

unsafe extern "C" fn CG_PlayBufferedSounds() {
    if crate::src::cgame::cg_main::cg.soundTime < crate::src::cgame::cg_main::cg.time {
        if crate::src::cgame::cg_main::cg.soundBufferOut
            != crate::src::cgame::cg_main::cg.soundBufferIn
            && crate::src::cgame::cg_main::cg.soundBuffer
                [crate::src::cgame::cg_main::cg.soundBufferOut as usize]
                != 0
        {
            crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                crate::src::cgame::cg_main::cg.soundBuffer
                    [crate::src::cgame::cg_main::cg.soundBufferOut as usize],
                crate::src::qcommon::q_shared::CHAN_ANNOUNCER as i32,
            );
            crate::src::cgame::cg_main::cg.soundBuffer
                [crate::src::cgame::cg_main::cg.soundBufferOut as usize] = 0 as i32;
            crate::src::cgame::cg_main::cg.soundBufferOut =
                (crate::src::cgame::cg_main::cg.soundBufferOut + 1 as i32)
                    % 20 as i32;
            crate::src::cgame::cg_main::cg.soundTime =
                crate::src::cgame::cg_main::cg.time + 750 as i32
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
//=========================================================================
/*
=================
CG_DrawActiveFrame

Generates and draws a game scene and status information at the given time.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawActiveFrame(
    mut serverTime: i32,
    mut stereoView: crate::tr_types_h::stereoFrame_t,
    mut demoPlayback: crate::src::qcommon::q_shared::qboolean,
) {
    let mut inwater: i32 = 0;
    crate::src::cgame::cg_main::cg.time = serverTime;
    crate::src::cgame::cg_main::cg.demoPlayback = demoPlayback;
    // update cvars
    crate::src::cgame::cg_main::CG_UpdateCvars();
    // if we are only updating the screen as a loading
    // pacifier, don't even try to read snapshots
    if crate::src::cgame::cg_main::cg.infoScreenText[0 as i32 as usize] as i32
        != 0 as i32
    {
        crate::src::cgame::cg_info::CG_DrawInformation();
        return;
    }
    // any looped sounds will be respecified as entities
    // are added to the render list
    crate::src::cgame::cg_syscalls::trap_S_ClearLoopingSounds(
        crate::src::qcommon::q_shared::qfalse,
    );
    // clear all the render lists
    crate::src::cgame::cg_syscalls::trap_R_ClearScene();
    // set up cg.snap and possibly cg.nextSnap
    crate::src::cgame::cg_snapshot::CG_ProcessSnapshots();
    // if we haven't received any snapshots yet, all
    // we can draw is the information screen
    if crate::src::cgame::cg_main::cg.snap.is_null()
        || (*crate::src::cgame::cg_main::cg.snap).snapFlags & 2 as i32 != 0
    {
        crate::src::cgame::cg_info::CG_DrawInformation();
        return;
    }
    // let the client system know what our weapon and zoom settings are
    crate::src::cgame::cg_syscalls::trap_SetUserCmdValue(
        crate::src::cgame::cg_main::cg.weaponSelect,
        crate::src::cgame::cg_main::cg.zoomSensitivity,
    );
    // this counter will be bumped for every valid scene we generate
    crate::src::cgame::cg_main::cg.clientFrame += 1;
    // update cg.predictedPlayerState
    crate::src::cgame::cg_predict::CG_PredictPlayerState();
    // decide on third person view
    crate::src::cgame::cg_main::cg.renderingThirdPerson =
        ((*crate::src::cgame::cg_main::cg.snap).ps.persistant
            [crate::bg_public_h::PERS_TEAM as i32 as usize]
            != crate::bg_public_h::TEAM_SPECTATOR as i32
            && (crate::src::cgame::cg_main::cg_thirdPerson.integer != 0
                || (*crate::src::cgame::cg_main::cg.snap).ps.stats
                    [crate::bg_public_h::STAT_HEALTH as i32 as usize]
                    <= 0 as i32)) as i32
            as crate::src::qcommon::q_shared::qboolean;
    // build cg.refdef
    inwater = CG_CalcViewValues();
    // first person blend blobs, done after AnglesToAxis
    if crate::src::cgame::cg_main::cg.renderingThirdPerson as u64 == 0 {
        CG_DamageBlendBlob();
    }
    // build the render lists
    if crate::src::cgame::cg_main::cg.hyperspace as u64 == 0 {
        crate::src::cgame::cg_ents::CG_AddPacketEntities(); // adter calcViewValues, so predicted player state is correct
        crate::src::cgame::cg_marks::CG_AddMarks();
        crate::src::cgame::cg_particles::CG_AddParticles();
        crate::src::cgame::cg_localents::CG_AddLocalEntities();
    }
    crate::src::cgame::cg_weapons::CG_AddViewWeapon(
        &mut crate::src::cgame::cg_main::cg.predictedPlayerState as *mut _
            as *mut crate::src::qcommon::q_shared::playerState_s,
    );
    // add buffered sounds
    CG_PlayBufferedSounds();
    // finish up the rest of the refdef
    if crate::src::cgame::cg_main::cg.testModelEntity.hModel != 0 {
        CG_AddTestModel();
    }
    crate::src::cgame::cg_main::cg.refdef.time = crate::src::cgame::cg_main::cg.time;
    crate::stdlib::memcpy(
        crate::src::cgame::cg_main::cg.refdef.areamask.as_mut_ptr() as *mut libc::c_void,
        (*crate::src::cgame::cg_main::cg.snap).areamask.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 32]>() as libc::c_ulong,
    );
    // warning sounds when powerup is wearing off
    CG_PowerupTimerSounds();
    // update audio positions
    crate::src::cgame::cg_syscalls::trap_S_Respatialize(
        (*crate::src::cgame::cg_main::cg.snap).ps.clientNum,
        crate::src::cgame::cg_main::cg.refdef.vieworg.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::cgame::cg_main::cg.refdef.viewaxis.as_mut_ptr(),
        inwater,
    );
    // make sure the lagometerSample and frame timing isn't done twice when in stereo
    if stereoView as u32 != crate::tr_types_h::STEREO_RIGHT as i32 as u32
    {
        crate::src::cgame::cg_main::cg.frametime =
            crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.oldTime;
        if crate::src::cgame::cg_main::cg.frametime < 0 as i32 {
            crate::src::cgame::cg_main::cg.frametime = 0 as i32
        }
        crate::src::cgame::cg_main::cg.oldTime = crate::src::cgame::cg_main::cg.time;
        crate::src::cgame::cg_draw::CG_AddLagometerFrameInfo();
    }
    if crate::src::cgame::cg_main::cg_timescale.value
        != crate::src::cgame::cg_main::cg_timescaleFadeEnd.value
    {
        if crate::src::cgame::cg_main::cg_timescale.value
            < crate::src::cgame::cg_main::cg_timescaleFadeEnd.value
        {
            crate::src::cgame::cg_main::cg_timescale.value +=
                crate::src::cgame::cg_main::cg_timescaleFadeSpeed.value
                    * crate::src::cgame::cg_main::cg.frametime as f32
                    / 1000 as i32 as f32;
            if crate::src::cgame::cg_main::cg_timescale.value
                > crate::src::cgame::cg_main::cg_timescaleFadeEnd.value
            {
                crate::src::cgame::cg_main::cg_timescale.value =
                    crate::src::cgame::cg_main::cg_timescaleFadeEnd.value
            }
        } else {
            crate::src::cgame::cg_main::cg_timescale.value -=
                crate::src::cgame::cg_main::cg_timescaleFadeSpeed.value
                    * crate::src::cgame::cg_main::cg.frametime as f32
                    / 1000 as i32 as f32;
            if crate::src::cgame::cg_main::cg_timescale.value
                < crate::src::cgame::cg_main::cg_timescaleFadeEnd.value
            {
                crate::src::cgame::cg_main::cg_timescale.value =
                    crate::src::cgame::cg_main::cg_timescaleFadeEnd.value
            }
        }
        if crate::src::cgame::cg_main::cg_timescaleFadeSpeed.value != 0. {
            crate::src::cgame::cg_syscalls::trap_Cvar_Set(
                b"timescale\x00" as *const u8 as *const libc::c_char,
                crate::src::qcommon::q_shared::va(
                    b"%f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    crate::src::cgame::cg_main::cg_timescale.value as f64,
                ),
            );
        }
    }
    // actually issue the rendering calls
    crate::src::cgame::cg_draw::CG_DrawActive(stereoView);
    if crate::src::cgame::cg_main::cg_stats.integer != 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"cg.clientFrame:%i\n\x00" as *const u8 as *const libc::c_char,
            crate::src::cgame::cg_main::cg.clientFrame,
        );
    };
}

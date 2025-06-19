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

    #[inline]

    pub unsafe extern "C" fn VectorLengthSquared(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return *v.offset(0 as i32 as isize) * *v.offset(0 as i32 as isize)
            + *v.offset(1 as i32 as isize) * *v.offset(1 as i32 as isize)
            + *v.offset(2 as i32 as isize) * *v.offset(2 as i32 as isize);
    }
    #[inline]

    pub unsafe extern "C" fn VectorInverse(mut v: *mut crate::src::qcommon::q_shared::vec_t) {
        *v.offset(0 as i32 as isize) = -*v.offset(0 as i32 as isize);
        *v.offset(1 as i32 as isize) = -*v.offset(1 as i32 as isize);
        *v.offset(2 as i32 as isize) = -*v.offset(2 as i32 as isize);
    }

    // __Q_SHARED_H
}

pub use crate::cm_local_h::cArea_t;
pub use crate::cm_local_h::cLeaf_t;
pub use crate::cm_local_h::cNode_t;
pub use crate::cm_local_h::cPatch_t;
pub use crate::cm_local_h::cbrush_t;
pub use crate::cm_local_h::cbrushside_t;
pub use crate::cm_local_h::clipMap_t;
pub use crate::cm_local_h::cmodel_s;
pub use crate::cm_local_h::cmodel_t;
pub use crate::cm_local_h::leafList_s;
pub use crate::cm_local_h::leafList_t;
pub use crate::cm_local_h::sphere_t;
pub use crate::cm_local_h::traceWork_t;
pub use crate::qfiles_h::dshader_t;
pub use crate::src::qcommon::cm_load::c_brush_traces;
pub use crate::src::qcommon::cm_load::c_patch_traces;
pub use crate::src::qcommon::cm_load::c_traces;
pub use crate::src::qcommon::cm_load::cm;
pub use crate::src::qcommon::cm_load::cm_noCurves;
pub use crate::src::qcommon::cm_load::CM_ClipHandleToModel;

pub use crate::src::qcommon::cm_patch::patchCollide_s;
pub use crate::src::qcommon::cm_patch::CM_PositionTestInPatchCollide;
pub use crate::src::qcommon::cm_patch::CM_TraceThroughPatchCollide;
pub use crate::src::qcommon::cm_test::CM_BoundsIntersect;
pub use crate::src::qcommon::cm_test::CM_BoxLeafnums_r;
pub use crate::src::qcommon::cm_test::CM_StoreLeafs;
pub use crate::src::qcommon::cm_trace::q_shared_h::VectorInverse;
pub use crate::src::qcommon::cm_trace::q_shared_h::VectorLengthSquared;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::floatint_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;

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
// always use bbox vs. bbox collision and never capsule vs. bbox or vice versa
//#define ALWAYS_BBOX_VS_BBOX
// always use capsule vs. capsule collision and never capsule vs. bbox or vice versa
//#define ALWAYS_CAPSULE_VS_CAPSULE
//#define CAPSULE_DEBUG
/*
===============================================================================

BASIC MATH

===============================================================================
*/
/*
================
RotatePoint
================
*/
#[no_mangle]

pub unsafe extern "C" fn RotatePoint(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut matrix: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    // FIXME
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
================
TransposeMatrix
================
*/
#[no_mangle]

pub unsafe extern "C" fn TransposeMatrix(
    mut matrix: *mut crate::src::qcommon::q_shared::vec3_t,
    mut transpose: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    // FIXME
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
CreateRotationMatrix
================
*/
#[no_mangle]

pub unsafe extern "C" fn CreateRotationMatrix(
    mut angles: *const crate::src::qcommon::q_shared::vec_t,
    mut matrix: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    crate::src::qcommon::q_math::AngleVectors(
        angles,
        (*matrix.offset(0 as i32 as isize)).as_mut_ptr(),
        (*matrix.offset(1 as i32 as isize)).as_mut_ptr(),
        (*matrix.offset(2 as i32 as isize)).as_mut_ptr(),
    );
    VectorInverse((*matrix.offset(1 as i32 as isize)).as_mut_ptr());
}
/*
================
CM_ProjectPointOntoVector
================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_ProjectPointOntoVector(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut vStart: *mut crate::src::qcommon::q_shared::vec_t,
    mut vDir: *mut crate::src::qcommon::q_shared::vec_t,
    mut vProj: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut pVec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    pVec[0 as i32 as usize] =
        *point.offset(0 as i32 as isize) - *vStart.offset(0 as i32 as isize);
    pVec[1 as i32 as usize] =
        *point.offset(1 as i32 as isize) - *vStart.offset(1 as i32 as isize);
    pVec[2 as i32 as usize] =
        *point.offset(2 as i32 as isize) - *vStart.offset(2 as i32 as isize);
    // project onto the directional vector for this segment
    *vProj.offset(0 as i32 as isize) = *vStart.offset(0 as i32 as isize)
        + *vDir.offset(0 as i32 as isize)
            * (pVec[0 as i32 as usize] * *vDir.offset(0 as i32 as isize)
                + pVec[1 as i32 as usize] * *vDir.offset(1 as i32 as isize)
                + pVec[2 as i32 as usize] * *vDir.offset(2 as i32 as isize));
    *vProj.offset(1 as i32 as isize) = *vStart.offset(1 as i32 as isize)
        + *vDir.offset(1 as i32 as isize)
            * (pVec[0 as i32 as usize] * *vDir.offset(0 as i32 as isize)
                + pVec[1 as i32 as usize] * *vDir.offset(1 as i32 as isize)
                + pVec[2 as i32 as usize] * *vDir.offset(2 as i32 as isize));
    *vProj.offset(2 as i32 as isize) = *vStart.offset(2 as i32 as isize)
        + *vDir.offset(2 as i32 as isize)
            * (pVec[0 as i32 as usize] * *vDir.offset(0 as i32 as isize)
                + pVec[1 as i32 as usize] * *vDir.offset(1 as i32 as isize)
                + pVec[2 as i32 as usize] * *vDir.offset(2 as i32 as isize));
}
/*
================
CM_DistanceFromLineSquared
================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_DistanceFromLineSquared(
    mut p: *mut crate::src::qcommon::q_shared::vec_t,
    mut lp1: *mut crate::src::qcommon::q_shared::vec_t,
    mut lp2: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) -> f32 {
    let mut proj: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut t: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut j: i32 = 0;
    CM_ProjectPointOntoVector(p, lp1, dir, proj.as_mut_ptr());
    j = 0 as i32;
    while j < 3 as i32 {
        if proj[j as usize] > *lp1.offset(j as isize) && proj[j as usize] > *lp2.offset(j as isize)
            || proj[j as usize] < *lp1.offset(j as isize)
                && proj[j as usize] < *lp2.offset(j as isize)
        {
            break;
        }
        j += 1
    }
    if j < 3 as i32 {
        if crate::stdlib::fabs((proj[j as usize] - *lp1.offset(j as isize)) as f64)
            < crate::stdlib::fabs((proj[j as usize] - *lp2.offset(j as isize)) as f64)
        {
            t[0 as i32 as usize] =
                *p.offset(0 as i32 as isize) - *lp1.offset(0 as i32 as isize);
            t[1 as i32 as usize] =
                *p.offset(1 as i32 as isize) - *lp1.offset(1 as i32 as isize);
            t[2 as i32 as usize] =
                *p.offset(2 as i32 as isize) - *lp1.offset(2 as i32 as isize)
        } else {
            t[0 as i32 as usize] =
                *p.offset(0 as i32 as isize) - *lp2.offset(0 as i32 as isize);
            t[1 as i32 as usize] =
                *p.offset(1 as i32 as isize) - *lp2.offset(1 as i32 as isize);
            t[2 as i32 as usize] =
                *p.offset(2 as i32 as isize) - *lp2.offset(2 as i32 as isize)
        }
        return VectorLengthSquared(t.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    }
    t[0 as i32 as usize] =
        *p.offset(0 as i32 as isize) - proj[0 as i32 as usize];
    t[1 as i32 as usize] =
        *p.offset(1 as i32 as isize) - proj[1 as i32 as usize];
    t[2 as i32 as usize] =
        *p.offset(2 as i32 as isize) - proj[2 as i32 as usize];
    return VectorLengthSquared(t.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
}
/*
================
CM_VectorDistanceSquared
================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_VectorDistanceSquared(
    mut p1: *mut crate::src::qcommon::q_shared::vec_t,
    mut p2: *mut crate::src::qcommon::q_shared::vec_t,
) -> f32 {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    dir[0 as i32 as usize] =
        *p2.offset(0 as i32 as isize) - *p1.offset(0 as i32 as isize);
    dir[1 as i32 as usize] =
        *p2.offset(1 as i32 as isize) - *p1.offset(1 as i32 as isize);
    dir[2 as i32 as usize] =
        *p2.offset(2 as i32 as isize) - *p1.offset(2 as i32 as isize);
    return VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
}
/*
================
SquareRootFloat
================
*/
#[no_mangle]

pub unsafe extern "C" fn SquareRootFloat(mut number: f32) -> f32 {
    let mut t: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    let f: f32 = 1.5f32;
    x = number * 0.5f32;
    t.f = number;
    t.i = 0x5f3759df as i32 - (t.i >> 1 as i32);
    y = t.f;
    y = y * (f - x * y * y);
    y = y * (f - x * y * y);
    return number * y;
}
/*
===============================================================================

POSITION TESTING

===============================================================================
*/
/*
================
CM_TestBoxInBrush
================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TestBoxInBrush(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut brush: *mut crate::cm_local_h::cbrush_t,
) {
    let mut i: i32 = 0;
    let mut plane: *mut crate::src::qcommon::q_shared::cplane_t =
        0 as *mut crate::src::qcommon::q_shared::cplane_t;
    let mut dist: f32 = 0.;
    let mut d1: f32 = 0.;
    let mut side: *mut crate::cm_local_h::cbrushside_t = 0 as *mut crate::cm_local_h::cbrushside_t;
    let mut t: f32 = 0.;
    let mut startp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if (*brush).numsides == 0 {
        return;
    }
    // special test for axial
    if (*tw).bounds[0 as i32 as usize][0 as i32 as usize]
        > (*brush).bounds[1 as i32 as usize][0 as i32 as usize]
        || (*tw).bounds[0 as i32 as usize][1 as i32 as usize]
            > (*brush).bounds[1 as i32 as usize][1 as i32 as usize]
        || (*tw).bounds[0 as i32 as usize][2 as i32 as usize]
            > (*brush).bounds[1 as i32 as usize][2 as i32 as usize]
        || (*tw).bounds[1 as i32 as usize][0 as i32 as usize]
            < (*brush).bounds[0 as i32 as usize][0 as i32 as usize]
        || (*tw).bounds[1 as i32 as usize][1 as i32 as usize]
            < (*brush).bounds[0 as i32 as usize][1 as i32 as usize]
        || (*tw).bounds[1 as i32 as usize][2 as i32 as usize]
            < (*brush).bounds[0 as i32 as usize][2 as i32 as usize]
    {
        return;
    }
    if (*tw).sphere.use_0 as u64 != 0 {
        // the first six planes are the axial planes, so we only
        // need to test the remainder
        i = 6 as i32;
        while i < (*brush).numsides {
            side = (*brush).sides.offset(i as isize);
            plane = (*side).plane;
            // adjust the plane distance appropriately for radius
            dist = (*plane).dist + (*tw).sphere.radius;
            // find the closest point on the capsule to the plane
            t = (*plane).normal[0 as i32 as usize]
                * (*tw).sphere.offset[0 as i32 as usize]
                + (*plane).normal[1 as i32 as usize]
                    * (*tw).sphere.offset[1 as i32 as usize]
                + (*plane).normal[2 as i32 as usize]
                    * (*tw).sphere.offset[2 as i32 as usize];
            if t > 0 as i32 as f32 {
                startp[0 as i32 as usize] = (*tw).start[0 as i32 as usize]
                    - (*tw).sphere.offset[0 as i32 as usize];
                startp[1 as i32 as usize] = (*tw).start[1 as i32 as usize]
                    - (*tw).sphere.offset[1 as i32 as usize];
                startp[2 as i32 as usize] = (*tw).start[2 as i32 as usize]
                    - (*tw).sphere.offset[2 as i32 as usize]
            } else {
                startp[0 as i32 as usize] = (*tw).start[0 as i32 as usize]
                    + (*tw).sphere.offset[0 as i32 as usize];
                startp[1 as i32 as usize] = (*tw).start[1 as i32 as usize]
                    + (*tw).sphere.offset[1 as i32 as usize];
                startp[2 as i32 as usize] = (*tw).start[2 as i32 as usize]
                    + (*tw).sphere.offset[2 as i32 as usize]
            }
            d1 = startp[0 as i32 as usize] * (*plane).normal[0 as i32 as usize]
                + startp[1 as i32 as usize] * (*plane).normal[1 as i32 as usize]
                + startp[2 as i32 as usize] * (*plane).normal[2 as i32 as usize]
                - dist;
            // if completely in front of face, no intersection
            if d1 > 0 as i32 as f32 {
                return;
            }
            i += 1
        }
    } else {
        // the first six planes are the axial planes, so we only
        // need to test the remainder
        i = 6 as i32;
        while i < (*brush).numsides {
            side = (*brush).sides.offset(i as isize);
            plane = (*side).plane;
            // adjust the plane distance appropriately for mins/maxs
            dist = (*plane).dist
                - ((*tw).offsets[(*plane).signbits as usize][0 as i32 as usize]
                    * (*plane).normal[0 as i32 as usize]
                    + (*tw).offsets[(*plane).signbits as usize][1 as i32 as usize]
                        * (*plane).normal[1 as i32 as usize]
                    + (*tw).offsets[(*plane).signbits as usize][2 as i32 as usize]
                        * (*plane).normal[2 as i32 as usize]);
            d1 = (*tw).start[0 as i32 as usize]
                * (*plane).normal[0 as i32 as usize]
                + (*tw).start[1 as i32 as usize]
                    * (*plane).normal[1 as i32 as usize]
                + (*tw).start[2 as i32 as usize]
                    * (*plane).normal[2 as i32 as usize]
                - dist;
            // if completely in front of face, no intersection
            if d1 > 0 as i32 as f32 {
                return;
            }
            i += 1
        }
    }
    // inside this brush
    (*tw).trace.allsolid = crate::src::qcommon::q_shared::qtrue;
    (*tw).trace.startsolid = (*tw).trace.allsolid;
    (*tw).trace.fraction = 0 as i32 as f32;
    (*tw).trace.contents = (*brush).contents;
}
/*
================
CM_TestInLeaf
================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TestInLeaf(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut leaf: *mut crate::cm_local_h::cLeaf_t,
) {
    let mut k: i32 = 0;
    let mut brushnum: i32 = 0;
    let mut b: *mut crate::cm_local_h::cbrush_t = 0 as *mut crate::cm_local_h::cbrush_t;
    let mut patch: *mut crate::cm_local_h::cPatch_t = 0 as *mut crate::cm_local_h::cPatch_t;
    // test box position against all brushes in the leaf
    k = 0 as i32;
    while k < (*leaf).numLeafBrushes {
        brushnum = *crate::src::qcommon::cm_load::cm
            .leafbrushes
            .offset(((*leaf).firstLeafBrush + k) as isize);
        b = &mut *crate::src::qcommon::cm_load::cm
            .brushes
            .offset(brushnum as isize) as *mut crate::cm_local_h::cbrush_t;
        if !((*b).checkcount == crate::src::qcommon::cm_load::cm.checkcount) {
            (*b).checkcount = crate::src::qcommon::cm_load::cm.checkcount;
            if !((*b).contents & (*tw).contents == 0) {
                CM_TestBoxInBrush(tw, b);
                if (*tw).trace.allsolid as u64 != 0 {
                    return;
                }
            }
        }
        k += 1
        // already checked this brush in another leaf
    }
    // test against all patches
    if (*crate::src::qcommon::cm_load::cm_noCurves).integer == 0 {
        //BSPC
        k = 0 as i32;
        while k < (*leaf).numLeafSurfaces {
            patch = *crate::src::qcommon::cm_load::cm.surfaces.offset(
                *crate::src::qcommon::cm_load::cm
                    .leafsurfaces
                    .offset(((*leaf).firstLeafSurface + k) as isize) as isize,
            );
            if !patch.is_null() {
                if !((*patch).checkcount == crate::src::qcommon::cm_load::cm.checkcount) {
                    (*patch).checkcount = crate::src::qcommon::cm_load::cm.checkcount;
                    if !((*patch).contents & (*tw).contents == 0) {
                        if crate::src::qcommon::cm_patch::CM_PositionTestInPatchCollide(
                            tw as *mut crate::cm_local_h::traceWork_t,
                            (*patch).pc,
                        ) as u64
                            != 0
                        {
                            (*tw).trace.allsolid = crate::src::qcommon::q_shared::qtrue;
                            (*tw).trace.startsolid = (*tw).trace.allsolid;
                            (*tw).trace.fraction = 0 as i32 as f32;
                            (*tw).trace.contents = (*patch).contents;
                            return;
                        }
                    }
                }
            }
            k += 1
            // already checked this brush in another leaf
        }
    };
}
/*
==================
CM_TestCapsuleInCapsule

capsule inside capsule check
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TestCapsuleInCapsule(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
) {
    let mut i: i32 = 0;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut top: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bottom: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut tmp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut offset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut symetricSize: [crate::src::qcommon::q_shared::vec3_t; 2] = [[0.; 3]; 2];
    let mut radius: f32 = 0.;
    let mut halfwidth: f32 = 0.;
    let mut halfheight: f32 = 0.;
    let mut offs: f32 = 0.;
    let mut r: f32 = 0.;
    crate::src::qcommon::cm_load::CM_ModelBounds(model, mins.as_mut_ptr(), maxs.as_mut_ptr());
    top[0 as i32 as usize] =
        (*tw).start[0 as i32 as usize] + (*tw).sphere.offset[0 as i32 as usize];
    top[1 as i32 as usize] =
        (*tw).start[1 as i32 as usize] + (*tw).sphere.offset[1 as i32 as usize];
    top[2 as i32 as usize] =
        (*tw).start[2 as i32 as usize] + (*tw).sphere.offset[2 as i32 as usize];
    bottom[0 as i32 as usize] =
        (*tw).start[0 as i32 as usize] - (*tw).sphere.offset[0 as i32 as usize];
    bottom[1 as i32 as usize] =
        (*tw).start[1 as i32 as usize] - (*tw).sphere.offset[1 as i32 as usize];
    bottom[2 as i32 as usize] =
        (*tw).start[2 as i32 as usize] - (*tw).sphere.offset[2 as i32 as usize];
    i = 0 as i32;
    while i < 3 as i32 {
        offset[i as usize] = ((mins[i as usize] + maxs[i as usize]) as f64 * 0.5f64)
            as crate::src::qcommon::q_shared::vec_t;
        symetricSize[0 as i32 as usize][i as usize] = mins[i as usize] - offset[i as usize];
        symetricSize[1 as i32 as usize][i as usize] = maxs[i as usize] - offset[i as usize];
        i += 1
    }
    halfwidth = symetricSize[1 as i32 as usize][0 as i32 as usize];
    halfheight = symetricSize[1 as i32 as usize][2 as i32 as usize];
    radius = if halfwidth > halfheight {
        halfheight
    } else {
        halfwidth
    };
    offs = halfheight - radius;
    r = ((*tw).sphere.radius + radius) * ((*tw).sphere.radius + radius);
    // check if any of the spheres overlap
    p1[0 as i32 as usize] = offset[0 as i32 as usize];
    p1[1 as i32 as usize] = offset[1 as i32 as usize];
    p1[2 as i32 as usize] = offset[2 as i32 as usize];
    p1[2 as i32 as usize] += offs;
    tmp[0 as i32 as usize] = p1[0 as i32 as usize] - top[0 as i32 as usize];
    tmp[1 as i32 as usize] = p1[1 as i32 as usize] - top[1 as i32 as usize];
    tmp[2 as i32 as usize] = p1[2 as i32 as usize] - top[2 as i32 as usize];
    if VectorLengthSquared(tmp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) < r {
        (*tw).trace.allsolid = crate::src::qcommon::q_shared::qtrue;
        (*tw).trace.startsolid = (*tw).trace.allsolid;
        (*tw).trace.fraction = 0 as i32 as f32
    }
    tmp[0 as i32 as usize] =
        p1[0 as i32 as usize] - bottom[0 as i32 as usize];
    tmp[1 as i32 as usize] =
        p1[1 as i32 as usize] - bottom[1 as i32 as usize];
    tmp[2 as i32 as usize] =
        p1[2 as i32 as usize] - bottom[2 as i32 as usize];
    if VectorLengthSquared(tmp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) < r {
        (*tw).trace.allsolid = crate::src::qcommon::q_shared::qtrue;
        (*tw).trace.startsolid = (*tw).trace.allsolid;
        (*tw).trace.fraction = 0 as i32 as f32
    }
    p2[0 as i32 as usize] = offset[0 as i32 as usize];
    p2[1 as i32 as usize] = offset[1 as i32 as usize];
    p2[2 as i32 as usize] = offset[2 as i32 as usize];
    p2[2 as i32 as usize] -= offs;
    tmp[0 as i32 as usize] = p2[0 as i32 as usize] - top[0 as i32 as usize];
    tmp[1 as i32 as usize] = p2[1 as i32 as usize] - top[1 as i32 as usize];
    tmp[2 as i32 as usize] = p2[2 as i32 as usize] - top[2 as i32 as usize];
    if VectorLengthSquared(tmp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) < r {
        (*tw).trace.allsolid = crate::src::qcommon::q_shared::qtrue;
        (*tw).trace.startsolid = (*tw).trace.allsolid;
        (*tw).trace.fraction = 0 as i32 as f32
    }
    tmp[0 as i32 as usize] =
        p2[0 as i32 as usize] - bottom[0 as i32 as usize];
    tmp[1 as i32 as usize] =
        p2[1 as i32 as usize] - bottom[1 as i32 as usize];
    tmp[2 as i32 as usize] =
        p2[2 as i32 as usize] - bottom[2 as i32 as usize];
    if VectorLengthSquared(tmp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) < r {
        (*tw).trace.allsolid = crate::src::qcommon::q_shared::qtrue;
        (*tw).trace.startsolid = (*tw).trace.allsolid;
        (*tw).trace.fraction = 0 as i32 as f32
    }
    // if between cylinder up and lower bounds
    if top[2 as i32 as usize] >= p1[2 as i32 as usize]
        && top[2 as i32 as usize] <= p2[2 as i32 as usize]
        || bottom[2 as i32 as usize] >= p1[2 as i32 as usize]
            && bottom[2 as i32 as usize] <= p2[2 as i32 as usize]
    {
        // 2d coordinates
        p1[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
        top[2 as i32 as usize] = p1[2 as i32 as usize];
        // if the cylinders overlap
        tmp[0 as i32 as usize] =
            top[0 as i32 as usize] - p1[0 as i32 as usize];
        tmp[1 as i32 as usize] =
            top[1 as i32 as usize] - p1[1 as i32 as usize];
        tmp[2 as i32 as usize] =
            top[2 as i32 as usize] - p1[2 as i32 as usize];
        if VectorLengthSquared(tmp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) < r
        {
            (*tw).trace.allsolid = crate::src::qcommon::q_shared::qtrue;
            (*tw).trace.startsolid = (*tw).trace.allsolid;
            (*tw).trace.fraction = 0 as i32 as f32
        }
    };
}
/*
==================
CM_TestBoundingBoxInCapsule

bounding box inside capsule check
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TestBoundingBoxInCapsule(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
) {
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut offset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut size: [crate::src::qcommon::q_shared::vec3_t; 2] = [[0.; 3]; 2];
    let mut h: crate::src::qcommon::q_shared::clipHandle_t = 0;
    let mut cmod: *mut crate::cm_local_h::cmodel_t = 0 as *mut crate::cm_local_h::cmodel_t;
    let mut i: i32 = 0;
    // mins maxs of the capsule
    crate::src::qcommon::cm_load::CM_ModelBounds(model, mins.as_mut_ptr(), maxs.as_mut_ptr());
    // offset for capsule center
    i = 0 as i32;
    while i < 3 as i32 {
        offset[i as usize] = ((mins[i as usize] + maxs[i as usize]) as f64 * 0.5f64)
            as crate::src::qcommon::q_shared::vec_t;
        size[0 as i32 as usize][i as usize] = mins[i as usize] - offset[i as usize];
        size[1 as i32 as usize][i as usize] = maxs[i as usize] - offset[i as usize];
        (*tw).start[i as usize] -= offset[i as usize];
        (*tw).end[i as usize] -= offset[i as usize];
        i += 1
    }
    // replace the bounding box with the capsule
    (*tw).sphere.use_0 = crate::src::qcommon::q_shared::qtrue;
    (*tw).sphere.radius = if size[1 as i32 as usize][0 as i32 as usize]
        > size[1 as i32 as usize][2 as i32 as usize]
    {
        size[1 as i32 as usize][2 as i32 as usize]
    } else {
        size[1 as i32 as usize][0 as i32 as usize]
    };
    (*tw).sphere.halfheight = size[1 as i32 as usize][2 as i32 as usize];
    (*tw).sphere.offset[0 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    (*tw).sphere.offset[1 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    (*tw).sphere.offset[2 as i32 as usize] =
        size[1 as i32 as usize][2 as i32 as usize] - (*tw).sphere.radius;
    // replace the capsule with the bounding box
    h = crate::src::qcommon::cm_load::CM_TempBoxModel(
        (*tw).size[0 as i32 as usize].as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        (*tw).size[1 as i32 as usize].as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_shared::qfalse as i32,
    );
    // calculate collision
    cmod =
        crate::src::qcommon::cm_load::CM_ClipHandleToModel(h) as *mut crate::cm_local_h::cmodel_s;
    CM_TestInLeaf(tw, &mut (*cmod).leaf);
}
#[no_mangle]

pub unsafe extern "C" fn CM_PositionTest(mut tw: *mut crate::cm_local_h::traceWork_t) {
    let mut leafs: [i32; 1024] = [0; 1024];
    let mut i: i32 = 0;
    let mut ll: crate::cm_local_h::leafList_t = crate::cm_local_h::leafList_t {
        count: 0,
        maxcount: 0,
        overflowed: crate::src::qcommon::q_shared::qfalse,
        list: 0 as *mut i32,
        bounds: [[0.; 3]; 2],
        lastLeaf: 0,
        storeLeafs: None,
    };
    // identify the leafs we are touching
    ll.bounds[0 as i32 as usize][0 as i32 as usize] = (*tw).start
        [0 as i32 as usize]
        + (*tw).size[0 as i32 as usize][0 as i32 as usize];
    ll.bounds[0 as i32 as usize][1 as i32 as usize] = (*tw).start
        [1 as i32 as usize]
        + (*tw).size[0 as i32 as usize][1 as i32 as usize];
    ll.bounds[0 as i32 as usize][2 as i32 as usize] = (*tw).start
        [2 as i32 as usize]
        + (*tw).size[0 as i32 as usize][2 as i32 as usize];
    ll.bounds[1 as i32 as usize][0 as i32 as usize] = (*tw).start
        [0 as i32 as usize]
        + (*tw).size[1 as i32 as usize][0 as i32 as usize];
    ll.bounds[1 as i32 as usize][1 as i32 as usize] = (*tw).start
        [1 as i32 as usize]
        + (*tw).size[1 as i32 as usize][1 as i32 as usize];
    ll.bounds[1 as i32 as usize][2 as i32 as usize] = (*tw).start
        [2 as i32 as usize]
        + (*tw).size[1 as i32 as usize][2 as i32 as usize];
    i = 0 as i32;
    while i < 3 as i32 {
        ll.bounds[0 as i32 as usize][i as usize] -= 1 as i32 as f32;
        ll.bounds[1 as i32 as usize][i as usize] += 1 as i32 as f32;
        i += 1
    }
    ll.count = 0 as i32;
    ll.maxcount = 1024 as i32;
    ll.list = leafs.as_mut_ptr();
    ll.storeLeafs = Some(
        crate::src::qcommon::cm_test::CM_StoreLeafs
            as unsafe extern "C" fn(_: *mut crate::cm_local_h::leafList_t, _: i32) -> (),
    );
    ll.lastLeaf = 0 as i32;
    ll.overflowed = crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::cm_load::cm.checkcount += 1;
    crate::src::qcommon::cm_test::CM_BoxLeafnums_r(
        &mut ll as *mut _ as *mut crate::cm_local_h::leafList_s,
        0 as i32,
    );
    crate::src::qcommon::cm_load::cm.checkcount += 1;
    // test the contents of the leafs
    i = 0 as i32;
    while i < ll.count {
        CM_TestInLeaf(
            tw,
            &mut *crate::src::qcommon::cm_load::cm
                .leafs
                .offset(*leafs.as_mut_ptr().offset(i as isize) as isize),
        );
        if (*tw).trace.allsolid as u64 != 0 {
            break;
        }
        i += 1
    }
}
/*
===============================================================================

TRACING

===============================================================================
*/
/*
================
CM_TraceThroughPatch
================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TraceThroughPatch(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut patch: *mut crate::cm_local_h::cPatch_t,
) {
    let mut oldFrac: f32 = 0.;
    crate::src::qcommon::cm_load::c_patch_traces += 1;
    oldFrac = (*tw).trace.fraction;
    crate::src::qcommon::cm_patch::CM_TraceThroughPatchCollide(
        tw as *mut crate::cm_local_h::traceWork_t,
        (*patch).pc,
    );
    if (*tw).trace.fraction < oldFrac {
        (*tw).trace.surfaceFlags = (*patch).surfaceFlags;
        (*tw).trace.contents = (*patch).contents
    };
}
/*
================
CM_TraceThroughBrush
================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TraceThroughBrush(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut brush: *mut crate::cm_local_h::cbrush_t,
) {
    let mut i: i32 = 0;
    let mut plane: *mut crate::src::qcommon::q_shared::cplane_t =
        0 as *mut crate::src::qcommon::q_shared::cplane_t;
    let mut clipplane: *mut crate::src::qcommon::q_shared::cplane_t =
        0 as *mut crate::src::qcommon::q_shared::cplane_t;
    let mut dist: f32 = 0.;
    let mut enterFrac: f32 = 0.;
    let mut leaveFrac: f32 = 0.;
    let mut d1: f32 = 0.;
    let mut d2: f32 = 0.;
    let mut getout: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut startout: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut f: f32 = 0.;
    let mut side: *mut crate::cm_local_h::cbrushside_t = 0 as *mut crate::cm_local_h::cbrushside_t;
    let mut leadside: *mut crate::cm_local_h::cbrushside_t =
        0 as *mut crate::cm_local_h::cbrushside_t;
    let mut t: f32 = 0.;
    let mut startp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut endp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    enterFrac = -1.0f64 as f32;
    leaveFrac = 1.0f64 as f32;
    clipplane = 0 as *mut crate::src::qcommon::q_shared::cplane_t;
    if (*brush).numsides == 0 {
        return;
    }
    crate::src::qcommon::cm_load::c_brush_traces += 1;
    getout = crate::src::qcommon::q_shared::qfalse;
    startout = crate::src::qcommon::q_shared::qfalse;
    leadside = 0 as *mut crate::cm_local_h::cbrushside_t;
    if (*tw).sphere.use_0 as u64 != 0 {
        //
        // compare the trace against all planes of the brush
        // find the latest time the trace crosses a plane towards the interior
        // and the earliest time the trace crosses a plane towards the exterior
        //
        i = 0 as i32;
        while i < (*brush).numsides {
            side = (*brush).sides.offset(i as isize);
            plane = (*side).plane;
            // adjust the plane distance appropriately for radius
            dist = (*plane).dist + (*tw).sphere.radius;
            // find the closest point on the capsule to the plane
            t = (*plane).normal[0 as i32 as usize]
                * (*tw).sphere.offset[0 as i32 as usize]
                + (*plane).normal[1 as i32 as usize]
                    * (*tw).sphere.offset[1 as i32 as usize]
                + (*plane).normal[2 as i32 as usize]
                    * (*tw).sphere.offset[2 as i32 as usize];
            if t > 0 as i32 as f32 {
                startp[0 as i32 as usize] = (*tw).start[0 as i32 as usize]
                    - (*tw).sphere.offset[0 as i32 as usize];
                startp[1 as i32 as usize] = (*tw).start[1 as i32 as usize]
                    - (*tw).sphere.offset[1 as i32 as usize];
                startp[2 as i32 as usize] = (*tw).start[2 as i32 as usize]
                    - (*tw).sphere.offset[2 as i32 as usize];
                endp[0 as i32 as usize] = (*tw).end[0 as i32 as usize]
                    - (*tw).sphere.offset[0 as i32 as usize];
                endp[1 as i32 as usize] = (*tw).end[1 as i32 as usize]
                    - (*tw).sphere.offset[1 as i32 as usize];
                endp[2 as i32 as usize] = (*tw).end[2 as i32 as usize]
                    - (*tw).sphere.offset[2 as i32 as usize]
            } else {
                startp[0 as i32 as usize] = (*tw).start[0 as i32 as usize]
                    + (*tw).sphere.offset[0 as i32 as usize];
                startp[1 as i32 as usize] = (*tw).start[1 as i32 as usize]
                    + (*tw).sphere.offset[1 as i32 as usize];
                startp[2 as i32 as usize] = (*tw).start[2 as i32 as usize]
                    + (*tw).sphere.offset[2 as i32 as usize];
                endp[0 as i32 as usize] = (*tw).end[0 as i32 as usize]
                    + (*tw).sphere.offset[0 as i32 as usize];
                endp[1 as i32 as usize] = (*tw).end[1 as i32 as usize]
                    + (*tw).sphere.offset[1 as i32 as usize];
                endp[2 as i32 as usize] = (*tw).end[2 as i32 as usize]
                    + (*tw).sphere.offset[2 as i32 as usize]
            }
            d1 = startp[0 as i32 as usize] * (*plane).normal[0 as i32 as usize]
                + startp[1 as i32 as usize] * (*plane).normal[1 as i32 as usize]
                + startp[2 as i32 as usize] * (*plane).normal[2 as i32 as usize]
                - dist;
            d2 = endp[0 as i32 as usize] * (*plane).normal[0 as i32 as usize]
                + endp[1 as i32 as usize] * (*plane).normal[1 as i32 as usize]
                + endp[2 as i32 as usize] * (*plane).normal[2 as i32 as usize]
                - dist;
            if d2 > 0 as i32 as f32 {
                getout = crate::src::qcommon::q_shared::qtrue
                // endpoint is not in solid
            }
            if d1 > 0 as i32 as f32 {
                startout = crate::src::qcommon::q_shared::qtrue
            }
            // if completely in front of face, no intersection with the entire brush
            if d1 > 0 as i32 as f32
                && (d2 as f64 >= 0.125f64 || d2 >= d1)
            {
                return;
            }
            // if it doesn't cross the plane, the plane isn't relevant
            if !(d1 <= 0 as i32 as f32 && d2 <= 0 as i32 as f32)
            {
                // crosses face
                if d1 > d2 {
                    f = ((d1 as f64 - 0.125f64) / (d1 - d2) as f64)
                        as f32; // leave
                    if f < 0 as i32 as f32 {
                        f = 0 as i32 as f32
                    }
                    if f > enterFrac {
                        enterFrac = f;
                        clipplane = plane;
                        leadside = side
                    }
                } else {
                    f = ((d1 as f64 + 0.125f64) / (d1 - d2) as f64)
                        as f32;
                    if f > 1 as i32 as f32 {
                        f = 1 as i32 as f32
                    }
                    if f < leaveFrac {
                        leaveFrac = f
                    }
                }
            }
            i += 1
        }
    } else {
        // enter
        //
        // compare the trace against all planes of the brush
        // find the latest time the trace crosses a plane towards the interior
        // and the earliest time the trace crosses a plane towards the exterior
        //
        i = 0 as i32;
        while i < (*brush).numsides {
            side = (*brush).sides.offset(i as isize);
            plane = (*side).plane;
            // adjust the plane distance appropriately for mins/maxs
            dist = (*plane).dist
                - ((*tw).offsets[(*plane).signbits as usize][0 as i32 as usize]
                    * (*plane).normal[0 as i32 as usize]
                    + (*tw).offsets[(*plane).signbits as usize][1 as i32 as usize]
                        * (*plane).normal[1 as i32 as usize]
                    + (*tw).offsets[(*plane).signbits as usize][2 as i32 as usize]
                        * (*plane).normal[2 as i32 as usize]);
            d1 = (*tw).start[0 as i32 as usize]
                * (*plane).normal[0 as i32 as usize]
                + (*tw).start[1 as i32 as usize]
                    * (*plane).normal[1 as i32 as usize]
                + (*tw).start[2 as i32 as usize]
                    * (*plane).normal[2 as i32 as usize]
                - dist;
            d2 = (*tw).end[0 as i32 as usize] * (*plane).normal[0 as i32 as usize]
                + (*tw).end[1 as i32 as usize] * (*plane).normal[1 as i32 as usize]
                + (*tw).end[2 as i32 as usize] * (*plane).normal[2 as i32 as usize]
                - dist;
            if d2 > 0 as i32 as f32 {
                getout = crate::src::qcommon::q_shared::qtrue
                // endpoint is not in solid
            }
            if d1 > 0 as i32 as f32 {
                startout = crate::src::qcommon::q_shared::qtrue
            }
            // if completely in front of face, no intersection with the entire brush
            if d1 > 0 as i32 as f32
                && (d2 as f64 >= 0.125f64 || d2 >= d1)
            {
                return;
            }
            // if it doesn't cross the plane, the plane isn't relevant
            if !(d1 <= 0 as i32 as f32 && d2 <= 0 as i32 as f32)
            {
                // crosses face
                if d1 > d2 {
                    f = ((d1 as f64 - 0.125f64) / (d1 - d2) as f64)
                        as f32; // leave
                    if f < 0 as i32 as f32 {
                        f = 0 as i32 as f32
                    }
                    if f > enterFrac {
                        enterFrac = f;
                        clipplane = plane;
                        leadside = side
                    }
                } else {
                    f = ((d1 as f64 + 0.125f64) / (d1 - d2) as f64)
                        as f32;
                    if f > 1 as i32 as f32 {
                        f = 1 as i32 as f32
                    }
                    if f < leaveFrac {
                        leaveFrac = f
                    }
                }
            }
            i += 1
        }
    }
    // enter
    //
    // all planes have been checked, and the trace was not
    // completely outside the brush
    //
    if startout as u64 == 0 {
        // original point was inside brush
        (*tw).trace.startsolid = crate::src::qcommon::q_shared::qtrue;
        if getout as u64 == 0 {
            (*tw).trace.allsolid = crate::src::qcommon::q_shared::qtrue;
            (*tw).trace.fraction = 0 as i32 as f32;
            (*tw).trace.contents = (*brush).contents
        }
        return;
    }
    if enterFrac < leaveFrac {
        if enterFrac > -(1 as i32) as f32 && enterFrac < (*tw).trace.fraction {
            if enterFrac < 0 as i32 as f32 {
                enterFrac = 0 as i32 as f32
            }
            (*tw).trace.fraction = enterFrac;
            if !clipplane.is_null() {
                (*tw).trace.plane = *clipplane
            }
            if !leadside.is_null() {
                (*tw).trace.surfaceFlags = (*leadside).surfaceFlags
            }
            (*tw).trace.contents = (*brush).contents
        }
    };
}
/*
================
CM_TraceThroughLeaf
================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TraceThroughLeaf(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut leaf: *mut crate::cm_local_h::cLeaf_t,
) {
    let mut k: i32 = 0;
    let mut brushnum: i32 = 0;
    let mut b: *mut crate::cm_local_h::cbrush_t = 0 as *mut crate::cm_local_h::cbrush_t;
    let mut patch: *mut crate::cm_local_h::cPatch_t = 0 as *mut crate::cm_local_h::cPatch_t;
    // trace line against all brushes in the leaf
    k = 0 as i32;
    while k < (*leaf).numLeafBrushes {
        brushnum = *crate::src::qcommon::cm_load::cm
            .leafbrushes
            .offset(((*leaf).firstLeafBrush + k) as isize);
        b = &mut *crate::src::qcommon::cm_load::cm
            .brushes
            .offset(brushnum as isize) as *mut crate::cm_local_h::cbrush_t;
        if !((*b).checkcount == crate::src::qcommon::cm_load::cm.checkcount) {
            (*b).checkcount = crate::src::qcommon::cm_load::cm.checkcount;
            if !((*b).contents & (*tw).contents == 0) {
                if !(crate::src::qcommon::cm_test::CM_BoundsIntersect(
                    (*tw).bounds[0 as i32 as usize].as_mut_ptr()
                        as *const crate::src::qcommon::q_shared::vec_t,
                    (*tw).bounds[1 as i32 as usize].as_mut_ptr()
                        as *const crate::src::qcommon::q_shared::vec_t,
                    (*b).bounds[0 as i32 as usize].as_mut_ptr()
                        as *const crate::src::qcommon::q_shared::vec_t,
                    (*b).bounds[1 as i32 as usize].as_mut_ptr()
                        as *const crate::src::qcommon::q_shared::vec_t,
                ) as u64
                    == 0)
                {
                    CM_TraceThroughBrush(tw, b);
                    if (*tw).trace.fraction == 0. {
                        return;
                    }
                }
            }
        }
        k += 1
        // already checked this brush in another leaf
    }
    // trace line against all patches in the leaf
    if (*crate::src::qcommon::cm_load::cm_noCurves).integer == 0 {
        k = 0 as i32;
        while k < (*leaf).numLeafSurfaces {
            patch = *crate::src::qcommon::cm_load::cm.surfaces.offset(
                *crate::src::qcommon::cm_load::cm
                    .leafsurfaces
                    .offset(((*leaf).firstLeafSurface + k) as isize) as isize,
            );
            if !patch.is_null() {
                if !((*patch).checkcount == crate::src::qcommon::cm_load::cm.checkcount) {
                    (*patch).checkcount = crate::src::qcommon::cm_load::cm.checkcount;
                    if !((*patch).contents & (*tw).contents == 0) {
                        CM_TraceThroughPatch(tw, patch);
                        if (*tw).trace.fraction == 0. {
                            return;
                        }
                    }
                }
            }
            k += 1
            // already checked this patch in another leaf
        }
    };
}
/*
================
CM_TraceThroughSphere

get the first intersection of the ray with the sphere
================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TraceThroughSphere(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut radius: f32,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut l1: f32 = 0.;
    let mut l2: f32 = 0.;
    let mut length: f32 = 0.;
    let mut scale: f32 = 0.;
    let mut fraction: f32 = 0.;
    //float a;
    let mut b: f32 = 0.;
    let mut c: f32 = 0.;
    let mut d: f32 = 0.;
    let mut sqrtd: f32 = 0.;
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut intersection: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // if inside the sphere
    dir[0 as i32 as usize] =
        *start.offset(0 as i32 as isize) - *origin.offset(0 as i32 as isize);
    dir[1 as i32 as usize] =
        *start.offset(1 as i32 as isize) - *origin.offset(1 as i32 as isize);
    dir[2 as i32 as usize] =
        *start.offset(2 as i32 as isize) - *origin.offset(2 as i32 as isize);
    l1 = VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    if l1 < radius * radius {
        (*tw).trace.fraction = 0 as i32 as f32;
        (*tw).trace.startsolid = crate::src::qcommon::q_shared::qtrue;
        // test for allsolid
        dir[0 as i32 as usize] =
            *end.offset(0 as i32 as isize) - *origin.offset(0 as i32 as isize);
        dir[1 as i32 as usize] =
            *end.offset(1 as i32 as isize) - *origin.offset(1 as i32 as isize);
        dir[2 as i32 as usize] =
            *end.offset(2 as i32 as isize) - *origin.offset(2 as i32 as isize);
        l1 = VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
        if l1 < radius * radius {
            (*tw).trace.allsolid = crate::src::qcommon::q_shared::qtrue
        }
        return;
    }
    //
    dir[0 as i32 as usize] =
        *end.offset(0 as i32 as isize) - *start.offset(0 as i32 as isize);
    dir[1 as i32 as usize] =
        *end.offset(1 as i32 as isize) - *start.offset(1 as i32 as isize);
    dir[2 as i32 as usize] =
        *end.offset(2 as i32 as isize) - *start.offset(2 as i32 as isize);
    length = crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    //
    l1 = CM_DistanceFromLineSquared(origin, start, end, dir.as_mut_ptr());
    v1[0 as i32 as usize] =
        *end.offset(0 as i32 as isize) - *origin.offset(0 as i32 as isize);
    v1[1 as i32 as usize] =
        *end.offset(1 as i32 as isize) - *origin.offset(1 as i32 as isize);
    v1[2 as i32 as usize] =
        *end.offset(2 as i32 as isize) - *origin.offset(2 as i32 as isize);
    l2 = VectorLengthSquared(v1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    // if no intersection with the sphere and the end point is at least an epsilon away
    if l1 >= radius * radius
        && l2 as f64
            > (radius as f64 + 0.125f64) * (radius as f64 + 0.125f64)
    {
        return;
    }
    //
    //	| origin - (start + t * dir) | = radius
    //	a = dir[0]^2 + dir[1]^2 + dir[2]^2;
    //	b = 2 * (dir[0] * (start[0] - origin[0]) + dir[1] * (start[1] - origin[1]) + dir[2] * (start[2] - origin[2]));
    //	c = (start[0] - origin[0])^2 + (start[1] - origin[1])^2 + (start[2] - origin[2])^2 - radius^2;
    //
    v1[0 as i32 as usize] =
        *start.offset(0 as i32 as isize) - *origin.offset(0 as i32 as isize);
    v1[1 as i32 as usize] =
        *start.offset(1 as i32 as isize) - *origin.offset(1 as i32 as isize);
    v1[2 as i32 as usize] =
        *start.offset(2 as i32 as isize) - *origin.offset(2 as i32 as isize);
    // dir is normalized so a = 1
    //a = 1.0f;//dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2];
    b = 2.0f32
        * (dir[0 as i32 as usize] * v1[0 as i32 as usize]
            + dir[1 as i32 as usize] * v1[1 as i32 as usize]
            + dir[2 as i32 as usize] * v1[2 as i32 as usize]); // * a;
    c = v1[0 as i32 as usize] * v1[0 as i32 as usize]
        + v1[1 as i32 as usize] * v1[1 as i32 as usize]
        + v1[2 as i32 as usize] * v1[2 as i32 as usize]
        - (radius + 1.0f32) * (radius + 1.0f32);
    d = b * b - 4.0f32 * c;
    if d > 0 as i32 as f32 {
        sqrtd = SquareRootFloat(d);
        // = (- b + sqrtd) * 0.5f; // / (2.0f * a);
        fraction = (-b - sqrtd) * 0.5f32; // / (2.0f * a);
                                          //
        if fraction < 0 as i32 as f32 {
            fraction = 0 as i32 as f32
        } else {
            fraction /= length
        }
        if fraction < (*tw).trace.fraction {
            (*tw).trace.fraction = fraction;
            dir[0 as i32 as usize] =
                *end.offset(0 as i32 as isize) - *start.offset(0 as i32 as isize);
            dir[1 as i32 as usize] =
                *end.offset(1 as i32 as isize) - *start.offset(1 as i32 as isize);
            dir[2 as i32 as usize] =
                *end.offset(2 as i32 as isize) - *start.offset(2 as i32 as isize);
            intersection[0 as i32 as usize] = *start.offset(0 as i32 as isize)
                + dir[0 as i32 as usize] * fraction;
            intersection[1 as i32 as usize] = *start.offset(1 as i32 as isize)
                + dir[1 as i32 as usize] * fraction;
            intersection[2 as i32 as usize] = *start.offset(2 as i32 as isize)
                + dir[2 as i32 as usize] * fraction;
            dir[0 as i32 as usize] =
                intersection[0 as i32 as usize] - *origin.offset(0 as i32 as isize);
            dir[1 as i32 as usize] =
                intersection[1 as i32 as usize] - *origin.offset(1 as i32 as isize);
            dir[2 as i32 as usize] =
                intersection[2 as i32 as usize] - *origin.offset(2 as i32 as isize);
            scale = 1 as i32 as f32 / (radius + 1.0f32);
            dir[0 as i32 as usize] = dir[0 as i32 as usize] * scale;
            dir[1 as i32 as usize] = dir[1 as i32 as usize] * scale;
            dir[2 as i32 as usize] = dir[2 as i32 as usize] * scale;
            (*tw).trace.plane.normal[0 as i32 as usize] = dir[0 as i32 as usize];
            (*tw).trace.plane.normal[1 as i32 as usize] = dir[1 as i32 as usize];
            (*tw).trace.plane.normal[2 as i32 as usize] = dir[2 as i32 as usize];
            intersection[0 as i32 as usize] = (*tw).modelOrigin[0 as i32 as usize]
                + intersection[0 as i32 as usize];
            intersection[1 as i32 as usize] = (*tw).modelOrigin[1 as i32 as usize]
                + intersection[1 as i32 as usize];
            intersection[2 as i32 as usize] = (*tw).modelOrigin[2 as i32 as usize]
                + intersection[2 as i32 as usize];
            (*tw).trace.plane.dist = (*tw).trace.plane.normal[0 as i32 as usize]
                * intersection[0 as i32 as usize]
                + (*tw).trace.plane.normal[1 as i32 as usize]
                    * intersection[1 as i32 as usize]
                + (*tw).trace.plane.normal[2 as i32 as usize]
                    * intersection[2 as i32 as usize];
            (*tw).trace.contents = 0x2000000 as i32
        }
    } else {
        // plane distance is zero
    };
    // no intersection at all
}
/*
================
CM_TraceThroughVerticalCylinder

get the first intersection of the ray with the cylinder
the cylinder extends halfheight above and below the origin
================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TraceThroughVerticalCylinder(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut radius: f32,
    mut halfheight: f32,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut length: f32 = 0.;
    let mut scale: f32 = 0.;
    let mut fraction: f32 = 0.;
    let mut l1: f32 = 0.;
    let mut l2: f32 = 0.;
    //float a;
    let mut b: f32 = 0.;
    let mut c: f32 = 0.;
    let mut d: f32 = 0.;
    let mut sqrtd: f32 = 0.;
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start2d: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end2d: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut org2d: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut intersection: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // 2d coordinates
    start2d[0 as i32 as usize] = *start.offset(0 as i32 as isize);
    start2d[1 as i32 as usize] = *start.offset(1 as i32 as isize);
    start2d[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    end2d[0 as i32 as usize] = *end.offset(0 as i32 as isize);
    end2d[1 as i32 as usize] = *end.offset(1 as i32 as isize);
    end2d[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    org2d[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    org2d[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    org2d[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    // if between lower and upper cylinder bounds
    if *start.offset(2 as i32 as isize)
        <= *origin.offset(2 as i32 as isize) + halfheight
        && *start.offset(2 as i32 as isize)
            >= *origin.offset(2 as i32 as isize) - halfheight
    {
        // if inside the cylinder
        dir[0 as i32 as usize] =
            start2d[0 as i32 as usize] - org2d[0 as i32 as usize];
        dir[1 as i32 as usize] =
            start2d[1 as i32 as usize] - org2d[1 as i32 as usize];
        dir[2 as i32 as usize] =
            start2d[2 as i32 as usize] - org2d[2 as i32 as usize];
        l1 = VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
        if l1 < radius * radius {
            (*tw).trace.fraction = 0 as i32 as f32;
            (*tw).trace.startsolid = crate::src::qcommon::q_shared::qtrue;
            dir[0 as i32 as usize] =
                end2d[0 as i32 as usize] - org2d[0 as i32 as usize];
            dir[1 as i32 as usize] =
                end2d[1 as i32 as usize] - org2d[1 as i32 as usize];
            dir[2 as i32 as usize] =
                end2d[2 as i32 as usize] - org2d[2 as i32 as usize];
            l1 = VectorLengthSquared(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
            );
            if l1 < radius * radius {
                (*tw).trace.allsolid = crate::src::qcommon::q_shared::qtrue
            }
            return;
        }
    }
    //
    dir[0 as i32 as usize] =
        end2d[0 as i32 as usize] - start2d[0 as i32 as usize];
    dir[1 as i32 as usize] =
        end2d[1 as i32 as usize] - start2d[1 as i32 as usize];
    dir[2 as i32 as usize] =
        end2d[2 as i32 as usize] - start2d[2 as i32 as usize];
    length = crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    //
    l1 = CM_DistanceFromLineSquared(
        org2d.as_mut_ptr(),
        start2d.as_mut_ptr(),
        end2d.as_mut_ptr(),
        dir.as_mut_ptr(),
    );
    v1[0 as i32 as usize] =
        end2d[0 as i32 as usize] - org2d[0 as i32 as usize];
    v1[1 as i32 as usize] =
        end2d[1 as i32 as usize] - org2d[1 as i32 as usize];
    v1[2 as i32 as usize] =
        end2d[2 as i32 as usize] - org2d[2 as i32 as usize];
    l2 = VectorLengthSquared(v1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    // if no intersection with the cylinder and the end point is at least an epsilon away
    if l1 >= radius * radius
        && l2 as f64
            > (radius as f64 + 0.125f64) * (radius as f64 + 0.125f64)
    {
        return;
    }
    //
    //
    // (start[0] - origin[0] - t * dir[0]) ^ 2 + (start[1] - origin[1] - t * dir[1]) ^ 2 = radius ^ 2
    // (v1[0] + t * dir[0]) ^ 2 + (v1[1] + t * dir[1]) ^ 2 = radius ^ 2;
    // v1[0] ^ 2 + 2 * v1[0] * t * dir[0] + (t * dir[0]) ^ 2 +
    //						v1[1] ^ 2 + 2 * v1[1] * t * dir[1] + (t * dir[1]) ^ 2 = radius ^ 2
    // t ^ 2 * (dir[0] ^ 2 + dir[1] ^ 2) + t * (2 * v1[0] * dir[0] + 2 * v1[1] * dir[1]) +
    //						v1[0] ^ 2 + v1[1] ^ 2 - radius ^ 2 = 0
    //
    v1[0 as i32 as usize] =
        *start.offset(0 as i32 as isize) - *origin.offset(0 as i32 as isize);
    v1[1 as i32 as usize] =
        *start.offset(1 as i32 as isize) - *origin.offset(1 as i32 as isize);
    v1[2 as i32 as usize] =
        *start.offset(2 as i32 as isize) - *origin.offset(2 as i32 as isize);
    // dir is normalized so we can use a = 1
    //a = 1.0f;// * (dir[0] * dir[0] + dir[1] * dir[1]);
    b = 2.0f32
        * (v1[0 as i32 as usize] * dir[0 as i32 as usize]
            + v1[1 as i32 as usize] * dir[1 as i32 as usize]); // * a;
    c = v1[0 as i32 as usize] * v1[0 as i32 as usize]
        + v1[1 as i32 as usize] * v1[1 as i32 as usize]
        - (radius + 1.0f32) * (radius + 1.0f32);
    d = b * b - 4.0f32 * c;
    if d > 0 as i32 as f32 {
        sqrtd = SquareRootFloat(d);
        // = (- b + sqrtd) * 0.5f;// / (2.0f * a);
        fraction = (-b - sqrtd) * 0.5f32; // / (2.0f * a);
                                          //
        if fraction < 0 as i32 as f32 {
            fraction = 0 as i32 as f32
        } else {
            fraction /= length
        }
        if fraction < (*tw).trace.fraction {
            dir[0 as i32 as usize] =
                *end.offset(0 as i32 as isize) - *start.offset(0 as i32 as isize);
            dir[1 as i32 as usize] =
                *end.offset(1 as i32 as isize) - *start.offset(1 as i32 as isize);
            dir[2 as i32 as usize] =
                *end.offset(2 as i32 as isize) - *start.offset(2 as i32 as isize);
            intersection[0 as i32 as usize] = *start.offset(0 as i32 as isize)
                + dir[0 as i32 as usize] * fraction;
            intersection[1 as i32 as usize] = *start.offset(1 as i32 as isize)
                + dir[1 as i32 as usize] * fraction;
            intersection[2 as i32 as usize] = *start.offset(2 as i32 as isize)
                + dir[2 as i32 as usize] * fraction;
            // if the intersection is between the cylinder lower and upper bound
            if intersection[2 as i32 as usize]
                <= *origin.offset(2 as i32 as isize) + halfheight
                && intersection[2 as i32 as usize]
                    >= *origin.offset(2 as i32 as isize) - halfheight
            {
                //
                (*tw).trace.fraction = fraction;
                dir[0 as i32 as usize] = intersection[0 as i32 as usize]
                    - *origin.offset(0 as i32 as isize);
                dir[1 as i32 as usize] = intersection[1 as i32 as usize]
                    - *origin.offset(1 as i32 as isize);
                dir[2 as i32 as usize] = intersection[2 as i32 as usize]
                    - *origin.offset(2 as i32 as isize);
                dir[2 as i32 as usize] =
                    0 as i32 as crate::src::qcommon::q_shared::vec_t;
                scale = 1 as i32 as f32 / (radius + 1.0f32);
                dir[0 as i32 as usize] = dir[0 as i32 as usize] * scale;
                dir[1 as i32 as usize] = dir[1 as i32 as usize] * scale;
                dir[2 as i32 as usize] = dir[2 as i32 as usize] * scale;
                (*tw).trace.plane.normal[0 as i32 as usize] =
                    dir[0 as i32 as usize];
                (*tw).trace.plane.normal[1 as i32 as usize] =
                    dir[1 as i32 as usize];
                (*tw).trace.plane.normal[2 as i32 as usize] =
                    dir[2 as i32 as usize];
                intersection[0 as i32 as usize] = (*tw).modelOrigin
                    [0 as i32 as usize]
                    + intersection[0 as i32 as usize];
                intersection[1 as i32 as usize] = (*tw).modelOrigin
                    [1 as i32 as usize]
                    + intersection[1 as i32 as usize];
                intersection[2 as i32 as usize] = (*tw).modelOrigin
                    [2 as i32 as usize]
                    + intersection[2 as i32 as usize];
                (*tw).trace.plane.dist = (*tw).trace.plane.normal[0 as i32 as usize]
                    * intersection[0 as i32 as usize]
                    + (*tw).trace.plane.normal[1 as i32 as usize]
                        * intersection[1 as i32 as usize]
                    + (*tw).trace.plane.normal[2 as i32 as usize]
                        * intersection[2 as i32 as usize];
                (*tw).trace.contents = 0x2000000 as i32
            }
        }
    } else {
        // plane distance is zero
    };
    // no intersection at all
}
/*
================
CM_TraceCapsuleThroughCapsule

capsule vs. capsule collision (not rotated)
================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TraceCapsuleThroughCapsule(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
) {
    let mut i: i32 = 0;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut top: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bottom: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut starttop: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut startbottom: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut endtop: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut endbottom: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut offset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut symetricSize: [crate::src::qcommon::q_shared::vec3_t; 2] = [[0.; 3]; 2];
    let mut radius: f32 = 0.;
    let mut halfwidth: f32 = 0.;
    let mut halfheight: f32 = 0.;
    let mut offs: f32 = 0.;
    let mut h: f32 = 0.;
    crate::src::qcommon::cm_load::CM_ModelBounds(model, mins.as_mut_ptr(), maxs.as_mut_ptr());
    // test trace bounds vs. capsule bounds
    if (*tw).bounds[0 as i32 as usize][0 as i32 as usize]
        > maxs[0 as i32 as usize] + 1.0f32
        || (*tw).bounds[0 as i32 as usize][1 as i32 as usize]
            > maxs[1 as i32 as usize] + 1.0f32
        || (*tw).bounds[0 as i32 as usize][2 as i32 as usize]
            > maxs[2 as i32 as usize] + 1.0f32
        || (*tw).bounds[1 as i32 as usize][0 as i32 as usize]
            < mins[0 as i32 as usize] - 1.0f32
        || (*tw).bounds[1 as i32 as usize][1 as i32 as usize]
            < mins[1 as i32 as usize] - 1.0f32
        || (*tw).bounds[1 as i32 as usize][2 as i32 as usize]
            < mins[2 as i32 as usize] - 1.0f32
    {
        return;
    }
    // top origin and bottom origin of each sphere at start and end of trace
    starttop[0 as i32 as usize] =
        (*tw).start[0 as i32 as usize] + (*tw).sphere.offset[0 as i32 as usize];
    starttop[1 as i32 as usize] =
        (*tw).start[1 as i32 as usize] + (*tw).sphere.offset[1 as i32 as usize];
    starttop[2 as i32 as usize] =
        (*tw).start[2 as i32 as usize] + (*tw).sphere.offset[2 as i32 as usize];
    startbottom[0 as i32 as usize] =
        (*tw).start[0 as i32 as usize] - (*tw).sphere.offset[0 as i32 as usize];
    startbottom[1 as i32 as usize] =
        (*tw).start[1 as i32 as usize] - (*tw).sphere.offset[1 as i32 as usize];
    startbottom[2 as i32 as usize] =
        (*tw).start[2 as i32 as usize] - (*tw).sphere.offset[2 as i32 as usize];
    endtop[0 as i32 as usize] =
        (*tw).end[0 as i32 as usize] + (*tw).sphere.offset[0 as i32 as usize];
    endtop[1 as i32 as usize] =
        (*tw).end[1 as i32 as usize] + (*tw).sphere.offset[1 as i32 as usize];
    endtop[2 as i32 as usize] =
        (*tw).end[2 as i32 as usize] + (*tw).sphere.offset[2 as i32 as usize];
    endbottom[0 as i32 as usize] =
        (*tw).end[0 as i32 as usize] - (*tw).sphere.offset[0 as i32 as usize];
    endbottom[1 as i32 as usize] =
        (*tw).end[1 as i32 as usize] - (*tw).sphere.offset[1 as i32 as usize];
    endbottom[2 as i32 as usize] =
        (*tw).end[2 as i32 as usize] - (*tw).sphere.offset[2 as i32 as usize];
    // calculate top and bottom of the capsule spheres to collide with
    i = 0 as i32;
    while i < 3 as i32 {
        offset[i as usize] = ((mins[i as usize] + maxs[i as usize]) as f64 * 0.5f64)
            as crate::src::qcommon::q_shared::vec_t;
        symetricSize[0 as i32 as usize][i as usize] = mins[i as usize] - offset[i as usize];
        symetricSize[1 as i32 as usize][i as usize] = maxs[i as usize] - offset[i as usize];
        i += 1
    }
    halfwidth = symetricSize[1 as i32 as usize][0 as i32 as usize];
    halfheight = symetricSize[1 as i32 as usize][2 as i32 as usize];
    radius = if halfwidth > halfheight {
        halfheight
    } else {
        halfwidth
    };
    offs = halfheight - radius;
    top[0 as i32 as usize] = offset[0 as i32 as usize];
    top[1 as i32 as usize] = offset[1 as i32 as usize];
    top[2 as i32 as usize] = offset[2 as i32 as usize];
    top[2 as i32 as usize] += offs;
    bottom[0 as i32 as usize] = offset[0 as i32 as usize];
    bottom[1 as i32 as usize] = offset[1 as i32 as usize];
    bottom[2 as i32 as usize] = offset[2 as i32 as usize];
    bottom[2 as i32 as usize] -= offs;
    // expand radius of spheres
    radius += (*tw).sphere.radius;
    // if there is horizontal movement
    if (*tw).start[0 as i32 as usize] != (*tw).end[0 as i32 as usize]
        || (*tw).start[1 as i32 as usize] != (*tw).end[1 as i32 as usize]
    {
        // height of the expanded cylinder is the height of both cylinders minus the radius of both spheres
        h = halfheight + (*tw).sphere.halfheight - radius;
        // if the cylinder has a height
        if h > 0 as i32 as f32 {
            // test for collisions between the cylinders
            CM_TraceThroughVerticalCylinder(
                tw,
                offset.as_mut_ptr(),
                radius,
                h,
                (*tw).start.as_mut_ptr(),
                (*tw).end.as_mut_ptr(),
            );
        }
    }
    // test for collision between the spheres
    CM_TraceThroughSphere(
        tw,
        top.as_mut_ptr(),
        radius,
        startbottom.as_mut_ptr(),
        endbottom.as_mut_ptr(),
    );
    CM_TraceThroughSphere(
        tw,
        bottom.as_mut_ptr(),
        radius,
        starttop.as_mut_ptr(),
        endtop.as_mut_ptr(),
    );
}
/*
================
CM_TraceBoundingBoxThroughCapsule

bounding box vs. capsule collision
================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TraceBoundingBoxThroughCapsule(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
) {
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut offset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut size: [crate::src::qcommon::q_shared::vec3_t; 2] = [[0.; 3]; 2];
    let mut h: crate::src::qcommon::q_shared::clipHandle_t = 0;
    let mut cmod: *mut crate::cm_local_h::cmodel_t = 0 as *mut crate::cm_local_h::cmodel_t;
    let mut i: i32 = 0;
    // mins maxs of the capsule
    crate::src::qcommon::cm_load::CM_ModelBounds(model, mins.as_mut_ptr(), maxs.as_mut_ptr());
    // offset for capsule center
    i = 0 as i32;
    while i < 3 as i32 {
        offset[i as usize] = ((mins[i as usize] + maxs[i as usize]) as f64 * 0.5f64)
            as crate::src::qcommon::q_shared::vec_t;
        size[0 as i32 as usize][i as usize] = mins[i as usize] - offset[i as usize];
        size[1 as i32 as usize][i as usize] = maxs[i as usize] - offset[i as usize];
        (*tw).start[i as usize] -= offset[i as usize];
        (*tw).end[i as usize] -= offset[i as usize];
        i += 1
    }
    // replace the bounding box with the capsule
    (*tw).sphere.use_0 = crate::src::qcommon::q_shared::qtrue;
    (*tw).sphere.radius = if size[1 as i32 as usize][0 as i32 as usize]
        > size[1 as i32 as usize][2 as i32 as usize]
    {
        size[1 as i32 as usize][2 as i32 as usize]
    } else {
        size[1 as i32 as usize][0 as i32 as usize]
    };
    (*tw).sphere.halfheight = size[1 as i32 as usize][2 as i32 as usize];
    (*tw).sphere.offset[0 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    (*tw).sphere.offset[1 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    (*tw).sphere.offset[2 as i32 as usize] =
        size[1 as i32 as usize][2 as i32 as usize] - (*tw).sphere.radius;
    // replace the capsule with the bounding box
    h = crate::src::qcommon::cm_load::CM_TempBoxModel(
        (*tw).size[0 as i32 as usize].as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        (*tw).size[1 as i32 as usize].as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_shared::qfalse as i32,
    );
    // calculate collision
    cmod =
        crate::src::qcommon::cm_load::CM_ClipHandleToModel(h) as *mut crate::cm_local_h::cmodel_s;
    CM_TraceThroughLeaf(tw, &mut (*cmod).leaf);
}
//=========================================================================================
/*
==================
CM_TraceThroughTree

Traverse all the contacted leafs from the start to the end position.
If the trace is a point, they will be exactly in order, but for larger
trace volumes it is possible to hit something in a later leaf with
a smaller intercept fraction.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TraceThroughTree(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut num: i32,
    mut p1f: f32,
    mut p2f: f32,
    mut p1: *mut crate::src::qcommon::q_shared::vec_t,
    mut p2: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut node: *mut crate::cm_local_h::cNode_t = 0 as *mut crate::cm_local_h::cNode_t;
    let mut plane: *mut crate::src::qcommon::q_shared::cplane_t =
        0 as *mut crate::src::qcommon::q_shared::cplane_t;
    let mut t1: f32 = 0.;
    let mut t2: f32 = 0.;
    let mut offset: f32 = 0.;
    let mut frac: f32 = 0.;
    let mut frac2: f32 = 0.;
    let mut idist: f32 = 0.;
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut side: i32 = 0;
    let mut midf: f32 = 0.;
    if (*tw).trace.fraction <= p1f {
        return;
        // already hit something nearer
    }
    // if < 0, we are in a leaf node
    if num < 0 as i32 {
        CM_TraceThroughLeaf(
            tw,
            &mut *crate::src::qcommon::cm_load::cm
                .leafs
                .offset((-(1 as i32) - num) as isize),
        );
        return;
    }
    //
    // find the point distances to the separating plane
    // and the offset for the size of the box
    //
    node = crate::src::qcommon::cm_load::cm.nodes.offset(num as isize);
    plane = (*node).plane;
    // adjust the plane distance appropriately for mins/maxs
    if ((*plane).type_0 as i32) < 3 as i32 {
        t1 = *p1.offset((*plane).type_0 as isize) - (*plane).dist;
        t2 = *p2.offset((*plane).type_0 as isize) - (*plane).dist;
        offset = (*tw).extents[(*plane).type_0 as usize]
    } else {
        t1 = (*plane).normal[0 as i32 as usize] * *p1.offset(0 as i32 as isize)
            + (*plane).normal[1 as i32 as usize] * *p1.offset(1 as i32 as isize)
            + (*plane).normal[2 as i32 as usize] * *p1.offset(2 as i32 as isize)
            - (*plane).dist;
        t2 = (*plane).normal[0 as i32 as usize] * *p2.offset(0 as i32 as isize)
            + (*plane).normal[1 as i32 as usize] * *p2.offset(1 as i32 as isize)
            + (*plane).normal[2 as i32 as usize] * *p2.offset(2 as i32 as isize)
            - (*plane).dist;
        if (*tw).isPoint as u64 != 0 {
            offset = 0 as i32 as f32
        } else {
            // this is silly
            offset = 2048 as i32 as f32
        }
    }
    // see which sides we need to consider
    if t1 >= offset + 1 as i32 as f32
        && t2 >= offset + 1 as i32 as f32
    {
        CM_TraceThroughTree(
            tw,
            (*node).children[0 as i32 as usize],
            p1f,
            p2f,
            p1,
            p2,
        );
        return;
    }
    if t1 < -offset - 1 as i32 as f32
        && t2 < -offset - 1 as i32 as f32
    {
        CM_TraceThroughTree(
            tw,
            (*node).children[1 as i32 as usize],
            p1f,
            p2f,
            p1,
            p2,
        );
        return;
    }
    // put the crosspoint SURFACE_CLIP_EPSILON pixels on the near side
    if t1 < t2 {
        idist = (1.0f64 / (t1 - t2) as f64) as f32;
        side = 1 as i32;
        frac2 = (((t1 + offset) as f64 + 0.125f64) * idist as f64)
            as f32;
        frac = (((t1 - offset) as f64 + 0.125f64) * idist as f64)
            as f32
    } else if t1 > t2 {
        idist = (1.0f64 / (t1 - t2) as f64) as f32;
        side = 0 as i32;
        frac2 = (((t1 - offset) as f64 - 0.125f64) * idist as f64)
            as f32;
        frac = (((t1 + offset) as f64 + 0.125f64) * idist as f64)
            as f32
    } else {
        side = 0 as i32;
        frac = 1 as i32 as f32;
        frac2 = 0 as i32 as f32
    }
    // move up to the node
    if frac < 0 as i32 as f32 {
        frac = 0 as i32 as f32
    }
    if frac > 1 as i32 as f32 {
        frac = 1 as i32 as f32
    }
    midf = p1f + (p2f - p1f) * frac;
    mid[0 as i32 as usize] = *p1.offset(0 as i32 as isize)
        + frac * (*p2.offset(0 as i32 as isize) - *p1.offset(0 as i32 as isize));
    mid[1 as i32 as usize] = *p1.offset(1 as i32 as isize)
        + frac * (*p2.offset(1 as i32 as isize) - *p1.offset(1 as i32 as isize));
    mid[2 as i32 as usize] = *p1.offset(2 as i32 as isize)
        + frac * (*p2.offset(2 as i32 as isize) - *p1.offset(2 as i32 as isize));
    CM_TraceThroughTree(
        tw,
        (*node).children[side as usize],
        p1f,
        midf,
        p1,
        mid.as_mut_ptr(),
    );
    // go past the node
    if frac2 < 0 as i32 as f32 {
        frac2 = 0 as i32 as f32
    }
    if frac2 > 1 as i32 as f32 {
        frac2 = 1 as i32 as f32
    }
    midf = p1f + (p2f - p1f) * frac2;
    mid[0 as i32 as usize] = *p1.offset(0 as i32 as isize)
        + frac2 * (*p2.offset(0 as i32 as isize) - *p1.offset(0 as i32 as isize));
    mid[1 as i32 as usize] = *p1.offset(1 as i32 as isize)
        + frac2 * (*p2.offset(1 as i32 as isize) - *p1.offset(1 as i32 as isize));
    mid[2 as i32 as usize] = *p1.offset(2 as i32 as isize)
        + frac2 * (*p2.offset(2 as i32 as isize) - *p1.offset(2 as i32 as isize));
    CM_TraceThroughTree(
        tw,
        (*node).children[(side ^ 1 as i32) as usize],
        midf,
        p2f,
        mid.as_mut_ptr(),
        p2,
    );
}
//======================================================================
/*
==================
CM_Trace
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_Trace(
    mut results: *mut crate::src::qcommon::q_shared::trace_t,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut brushmask: i32,
    mut capsule: i32,
    mut sphere: *mut crate::cm_local_h::sphere_t,
) {
    let mut i: i32 = 0; // for multi-check avoidance
    let mut tw: crate::cm_local_h::traceWork_t = crate::cm_local_h::traceWork_t {
        start: [0.; 3],
        end: [0.; 3],
        size: [[0.; 3]; 2],
        offsets: [[0.; 3]; 8],
        maxOffset: 0.,
        extents: [0.; 3],
        bounds: [[0.; 3]; 2],
        modelOrigin: [0.; 3],
        contents: 0,
        isPoint: crate::src::qcommon::q_shared::qfalse,
        trace: crate::src::qcommon::q_shared::trace_t {
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
        },
        sphere: crate::cm_local_h::sphere_t {
            use_0: crate::src::qcommon::q_shared::qfalse,
            radius: 0.,
            halfheight: 0.,
            offset: [0.; 3],
        },
    }; // for statistics, may be zeroed
    let mut offset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cmod: *mut crate::cm_local_h::cmodel_t = 0 as *mut crate::cm_local_h::cmodel_t;
    cmod = crate::src::qcommon::cm_load::CM_ClipHandleToModel(model)
        as *mut crate::cm_local_h::cmodel_s;
    crate::src::qcommon::cm_load::cm.checkcount += 1;
    crate::src::qcommon::cm_load::c_traces += 1;
    // fill in a default trace
    crate::stdlib::memset(
        &mut tw as *mut crate::cm_local_h::traceWork_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::cm_local_h::traceWork_t>() as libc::c_ulong,
    ); // assume it goes the entire distance until shown otherwise
    tw.trace.fraction = 1 as i32 as f32;
    tw.modelOrigin[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    tw.modelOrigin[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    tw.modelOrigin[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
    if crate::src::qcommon::cm_load::cm.numNodes == 0 {
        *results = tw.trace;
        return;
        // map not loaded, shouldn't happen
    }
    // allow NULL to be passed in for 0,0,0
    if mins.is_null() {
        mins = crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
    }
    if maxs.is_null() {
        maxs = crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
    }
    // set basic parms
    tw.contents = brushmask;
    // adjust so that mins and maxs are always symetric, which
    // avoids some complications with plane expanding of rotated
    // bmodels
    i = 0 as i32;
    while i < 3 as i32 {
        offset[i as usize] = ((*mins.offset(i as isize) + *maxs.offset(i as isize))
            as f64
            * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
        tw.size[0 as i32 as usize][i as usize] =
            *mins.offset(i as isize) - offset[i as usize];
        tw.size[1 as i32 as usize][i as usize] =
            *maxs.offset(i as isize) - offset[i as usize];
        tw.start[i as usize] = *start.offset(i as isize) + offset[i as usize];
        tw.end[i as usize] = *end.offset(i as isize) + offset[i as usize];
        i += 1
    }
    // if a sphere is already specified
    if !sphere.is_null() {
        tw.sphere = *sphere
    } else {
        tw.sphere.use_0 = capsule as crate::src::qcommon::q_shared::qboolean;
        tw.sphere.radius = if tw.size[1 as i32 as usize][0 as i32 as usize]
            > tw.size[1 as i32 as usize][2 as i32 as usize]
        {
            tw.size[1 as i32 as usize][2 as i32 as usize]
        } else {
            tw.size[1 as i32 as usize][0 as i32 as usize]
        };
        tw.sphere.halfheight = tw.size[1 as i32 as usize][2 as i32 as usize];
        tw.sphere.offset[0 as i32 as usize] =
            0 as i32 as crate::src::qcommon::q_shared::vec_t;
        tw.sphere.offset[1 as i32 as usize] =
            0 as i32 as crate::src::qcommon::q_shared::vec_t;
        tw.sphere.offset[2 as i32 as usize] =
            tw.size[1 as i32 as usize][2 as i32 as usize] - tw.sphere.radius
    }
    tw.maxOffset = tw.size[1 as i32 as usize][0 as i32 as usize]
        + tw.size[1 as i32 as usize][1 as i32 as usize]
        + tw.size[1 as i32 as usize][2 as i32 as usize];
    // tw.offsets[signbits] = vector to appropriate corner from origin
    tw.offsets[0 as i32 as usize][0 as i32 as usize] =
        tw.size[0 as i32 as usize][0 as i32 as usize];
    tw.offsets[0 as i32 as usize][1 as i32 as usize] =
        tw.size[0 as i32 as usize][1 as i32 as usize];
    tw.offsets[0 as i32 as usize][2 as i32 as usize] =
        tw.size[0 as i32 as usize][2 as i32 as usize];
    tw.offsets[1 as i32 as usize][0 as i32 as usize] =
        tw.size[1 as i32 as usize][0 as i32 as usize];
    tw.offsets[1 as i32 as usize][1 as i32 as usize] =
        tw.size[0 as i32 as usize][1 as i32 as usize];
    tw.offsets[1 as i32 as usize][2 as i32 as usize] =
        tw.size[0 as i32 as usize][2 as i32 as usize];
    tw.offsets[2 as i32 as usize][0 as i32 as usize] =
        tw.size[0 as i32 as usize][0 as i32 as usize];
    tw.offsets[2 as i32 as usize][1 as i32 as usize] =
        tw.size[1 as i32 as usize][1 as i32 as usize];
    tw.offsets[2 as i32 as usize][2 as i32 as usize] =
        tw.size[0 as i32 as usize][2 as i32 as usize];
    tw.offsets[3 as i32 as usize][0 as i32 as usize] =
        tw.size[1 as i32 as usize][0 as i32 as usize];
    tw.offsets[3 as i32 as usize][1 as i32 as usize] =
        tw.size[1 as i32 as usize][1 as i32 as usize];
    tw.offsets[3 as i32 as usize][2 as i32 as usize] =
        tw.size[0 as i32 as usize][2 as i32 as usize];
    tw.offsets[4 as i32 as usize][0 as i32 as usize] =
        tw.size[0 as i32 as usize][0 as i32 as usize];
    tw.offsets[4 as i32 as usize][1 as i32 as usize] =
        tw.size[0 as i32 as usize][1 as i32 as usize];
    tw.offsets[4 as i32 as usize][2 as i32 as usize] =
        tw.size[1 as i32 as usize][2 as i32 as usize];
    tw.offsets[5 as i32 as usize][0 as i32 as usize] =
        tw.size[1 as i32 as usize][0 as i32 as usize];
    tw.offsets[5 as i32 as usize][1 as i32 as usize] =
        tw.size[0 as i32 as usize][1 as i32 as usize];
    tw.offsets[5 as i32 as usize][2 as i32 as usize] =
        tw.size[1 as i32 as usize][2 as i32 as usize];
    tw.offsets[6 as i32 as usize][0 as i32 as usize] =
        tw.size[0 as i32 as usize][0 as i32 as usize];
    tw.offsets[6 as i32 as usize][1 as i32 as usize] =
        tw.size[1 as i32 as usize][1 as i32 as usize];
    tw.offsets[6 as i32 as usize][2 as i32 as usize] =
        tw.size[1 as i32 as usize][2 as i32 as usize];
    tw.offsets[7 as i32 as usize][0 as i32 as usize] =
        tw.size[1 as i32 as usize][0 as i32 as usize];
    tw.offsets[7 as i32 as usize][1 as i32 as usize] =
        tw.size[1 as i32 as usize][1 as i32 as usize];
    tw.offsets[7 as i32 as usize][2 as i32 as usize] =
        tw.size[1 as i32 as usize][2 as i32 as usize];
    //
    // calculate bounds
    //
    if tw.sphere.use_0 as u64 != 0 {
        i = 0 as i32;
        while i < 3 as i32 {
            if tw.start[i as usize] < tw.end[i as usize] {
                tw.bounds[0 as i32 as usize][i as usize] = (tw.start[i as usize]
                    as f64
                    - crate::stdlib::fabs(tw.sphere.offset[i as usize] as f64)
                    - tw.sphere.radius as f64)
                    as crate::src::qcommon::q_shared::vec_t;
                tw.bounds[1 as i32 as usize][i as usize] = (tw.end[i as usize]
                    as f64
                    + crate::stdlib::fabs(tw.sphere.offset[i as usize] as f64)
                    + tw.sphere.radius as f64)
                    as crate::src::qcommon::q_shared::vec_t
            } else {
                tw.bounds[0 as i32 as usize][i as usize] = (tw.end[i as usize]
                    as f64
                    - crate::stdlib::fabs(tw.sphere.offset[i as usize] as f64)
                    - tw.sphere.radius as f64)
                    as crate::src::qcommon::q_shared::vec_t;
                tw.bounds[1 as i32 as usize][i as usize] = (tw.start[i as usize]
                    as f64
                    + crate::stdlib::fabs(tw.sphere.offset[i as usize] as f64)
                    + tw.sphere.radius as f64)
                    as crate::src::qcommon::q_shared::vec_t
            }
            i += 1
        }
    } else {
        i = 0 as i32;
        while i < 3 as i32 {
            if tw.start[i as usize] < tw.end[i as usize] {
                tw.bounds[0 as i32 as usize][i as usize] =
                    tw.start[i as usize] + tw.size[0 as i32 as usize][i as usize];
                tw.bounds[1 as i32 as usize][i as usize] =
                    tw.end[i as usize] + tw.size[1 as i32 as usize][i as usize]
            } else {
                tw.bounds[0 as i32 as usize][i as usize] =
                    tw.end[i as usize] + tw.size[0 as i32 as usize][i as usize];
                tw.bounds[1 as i32 as usize][i as usize] =
                    tw.start[i as usize] + tw.size[1 as i32 as usize][i as usize]
            }
            i += 1
        }
    }
    //
    // check for position test special case
    //
    if *start.offset(0 as i32 as isize) == *end.offset(0 as i32 as isize)
        && *start.offset(1 as i32 as isize) == *end.offset(1 as i32 as isize)
        && *start.offset(2 as i32 as isize) == *end.offset(2 as i32 as isize)
    {
        if model != 0 {
            // FIXME - compile time flag?
            if model == 254 as i32 {
                if tw.sphere.use_0 as u64 != 0 {
                    CM_TestCapsuleInCapsule(&mut tw, model);
                } else {
                    CM_TestBoundingBoxInCapsule(&mut tw, model);
                }
            } else {
                CM_TestInLeaf(&mut tw, &mut (*cmod).leaf);
            }
        } else {
            CM_PositionTest(&mut tw);
        }
    } else {
        //
        // check for point special case
        //
        if tw.size[0 as i32 as usize][0 as i32 as usize]
            == 0 as i32 as f32
            && tw.size[0 as i32 as usize][1 as i32 as usize]
                == 0 as i32 as f32
            && tw.size[0 as i32 as usize][2 as i32 as usize]
                == 0 as i32 as f32
        {
            tw.isPoint = crate::src::qcommon::q_shared::qtrue;
            tw.extents[2 as i32 as usize] =
                0 as i32 as crate::src::qcommon::q_shared::vec_t;
            tw.extents[1 as i32 as usize] = tw.extents[2 as i32 as usize];
            tw.extents[0 as i32 as usize] = tw.extents[1 as i32 as usize]
        } else {
            tw.isPoint = crate::src::qcommon::q_shared::qfalse;
            tw.extents[0 as i32 as usize] =
                tw.size[1 as i32 as usize][0 as i32 as usize];
            tw.extents[1 as i32 as usize] =
                tw.size[1 as i32 as usize][1 as i32 as usize];
            tw.extents[2 as i32 as usize] =
                tw.size[1 as i32 as usize][2 as i32 as usize]
        }
        //
        // general sweeping through world
        //
        if model != 0 {
            if model == 254 as i32 {
                if tw.sphere.use_0 as u64 != 0 {
                    CM_TraceCapsuleThroughCapsule(&mut tw, model);
                } else {
                    CM_TraceBoundingBoxThroughCapsule(&mut tw, model);
                }
            } else {
                CM_TraceThroughLeaf(&mut tw, &mut (*cmod).leaf);
            }
        } else {
            CM_TraceThroughTree(
                &mut tw,
                0 as i32,
                0 as i32 as f32,
                1 as i32 as f32,
                tw.start.as_mut_ptr(),
                tw.end.as_mut_ptr(),
            );
        }
    }
    // generate endpos from the original, unmodified start/end
    if tw.trace.fraction == 1 as i32 as f32 {
        tw.trace.endpos[0 as i32 as usize] = *end.offset(0 as i32 as isize);
        tw.trace.endpos[1 as i32 as usize] = *end.offset(1 as i32 as isize);
        tw.trace.endpos[2 as i32 as usize] = *end.offset(2 as i32 as isize)
    } else {
        i = 0 as i32;
        while i < 3 as i32 {
            tw.trace.endpos[i as usize] = *start.offset(i as isize)
                + tw.trace.fraction * (*end.offset(i as isize) - *start.offset(i as isize));
            i += 1
        }
    }
    // If allsolid is set (was entirely inside something solid), the plane is not valid.
    // If fraction == 1.0, we never hit anything, and thus the plane is not valid.
    // Otherwise, the normal on the plane should have unit length
    *results = tw.trace;
}
/*
==================
CM_BoxTrace
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_BoxTrace(
    mut results: *mut crate::src::qcommon::q_shared::trace_t,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
    mut brushmask: i32,
    mut capsule: i32,
) {
    CM_Trace(
        results,
        start,
        end,
        mins,
        maxs,
        model,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        brushmask,
        capsule,
        0 as *mut crate::cm_local_h::sphere_t,
    );
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
// 0 = world, 1 + are bmodels
// returns an ORed contents mask
/*
==================
CM_TransformedBoxTrace

Handles offseting and rotation of the end points for moving and
rotating entities
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TransformedBoxTrace(
    mut results: *mut crate::src::qcommon::q_shared::trace_t,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
    mut brushmask: i32,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut angles: *const crate::src::qcommon::q_shared::vec_t,
    mut capsule: i32,
) {
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
    let mut start_l: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end_l: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut rotated: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut offset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut symetricSize: [crate::src::qcommon::q_shared::vec3_t; 2] = [[0.; 3]; 2];
    let mut matrix: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    let mut transpose: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    let mut i: i32 = 0;
    let mut halfwidth: f32 = 0.;
    let mut halfheight: f32 = 0.;
    let mut t: f32 = 0.;
    let mut sphere: crate::cm_local_h::sphere_t = crate::cm_local_h::sphere_t {
        use_0: crate::src::qcommon::q_shared::qfalse,
        radius: 0.,
        halfheight: 0.,
        offset: [0.; 3],
    };
    if mins.is_null() {
        mins = crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
    }
    if maxs.is_null() {
        maxs = crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
    }
    // adjust so that mins and maxs are always symetric, which
    // avoids some complications with plane expanding of rotated
    // bmodels
    i = 0 as i32;
    while i < 3 as i32 {
        offset[i as usize] = ((*mins.offset(i as isize) + *maxs.offset(i as isize))
            as f64
            * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
        symetricSize[0 as i32 as usize][i as usize] =
            *mins.offset(i as isize) - offset[i as usize];
        symetricSize[1 as i32 as usize][i as usize] =
            *maxs.offset(i as isize) - offset[i as usize];
        start_l[i as usize] = *start.offset(i as isize) + offset[i as usize];
        end_l[i as usize] = *end.offset(i as isize) + offset[i as usize];
        i += 1
    }
    // subtract origin offset
    start_l[0 as i32 as usize] =
        start_l[0 as i32 as usize] - *origin.offset(0 as i32 as isize);
    start_l[1 as i32 as usize] =
        start_l[1 as i32 as usize] - *origin.offset(1 as i32 as isize);
    start_l[2 as i32 as usize] =
        start_l[2 as i32 as usize] - *origin.offset(2 as i32 as isize);
    end_l[0 as i32 as usize] =
        end_l[0 as i32 as usize] - *origin.offset(0 as i32 as isize);
    end_l[1 as i32 as usize] =
        end_l[1 as i32 as usize] - *origin.offset(1 as i32 as isize);
    end_l[2 as i32 as usize] =
        end_l[2 as i32 as usize] - *origin.offset(2 as i32 as isize);
    // rotate start and end into the models frame of reference
    if model != 255 as i32
        && (*angles.offset(0 as i32 as isize) != 0.
            || *angles.offset(1 as i32 as isize) != 0.
            || *angles.offset(2 as i32 as isize) != 0.)
    {
        rotated = crate::src::qcommon::q_shared::qtrue
    } else {
        rotated = crate::src::qcommon::q_shared::qfalse
    }
    halfwidth = symetricSize[1 as i32 as usize][0 as i32 as usize];
    halfheight = symetricSize[1 as i32 as usize][2 as i32 as usize];
    sphere.use_0 = capsule as crate::src::qcommon::q_shared::qboolean;
    sphere.radius = if halfwidth > halfheight {
        halfheight
    } else {
        halfwidth
    };
    sphere.halfheight = halfheight;
    t = halfheight - sphere.radius;
    if rotated as u64 != 0 {
        // rotation on trace line (start-end) instead of rotating the bmodel
        // NOTE: This is still incorrect for bounding boxes because the actual bounding
        //		 box that is swept through the model is not rotated. We cannot rotate
        //		 the bounding box or the bmodel because that would make all the brush
        //		 bevels invalid.
        //		 However this is correct for capsules since a capsule itself is rotated too.
        CreateRotationMatrix(angles, matrix.as_mut_ptr());
        RotatePoint(start_l.as_mut_ptr(), matrix.as_mut_ptr());
        RotatePoint(end_l.as_mut_ptr(), matrix.as_mut_ptr());
        // rotated sphere offset for capsule
        sphere.offset[0 as i32 as usize] =
            matrix[0 as i32 as usize][2 as i32 as usize] * t;
        sphere.offset[1 as i32 as usize] =
            -matrix[1 as i32 as usize][2 as i32 as usize] * t;
        sphere.offset[2 as i32 as usize] =
            matrix[2 as i32 as usize][2 as i32 as usize] * t
    } else {
        sphere.offset[0 as i32 as usize] =
            0 as i32 as crate::src::qcommon::q_shared::vec_t;
        sphere.offset[1 as i32 as usize] =
            0 as i32 as crate::src::qcommon::q_shared::vec_t;
        sphere.offset[2 as i32 as usize] = t
    }
    // sweep the box through the model
    CM_Trace(
        &mut trace,
        start_l.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        end_l.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        symetricSize[0 as i32 as usize].as_mut_ptr(),
        symetricSize[1 as i32 as usize].as_mut_ptr(),
        model,
        origin,
        brushmask,
        capsule,
        &mut sphere,
    );
    // if the bmodel was rotated and there was a collision
    if rotated as u32 != 0 && trace.fraction as f64 != 1.0f64 {
        // rotation of bmodel collision plane
        TransposeMatrix(matrix.as_mut_ptr(), transpose.as_mut_ptr());
        RotatePoint(trace.plane.normal.as_mut_ptr(), transpose.as_mut_ptr());
    }
    // re-calculate the end position of the trace because the trace.endpos
    // calculated by CM_Trace could be rotated and have an offset
    trace.endpos[0 as i32 as usize] = *start.offset(0 as i32 as isize)
        + trace.fraction
            * (*end.offset(0 as i32 as isize) - *start.offset(0 as i32 as isize));
    trace.endpos[1 as i32 as usize] = *start.offset(1 as i32 as isize)
        + trace.fraction
            * (*end.offset(1 as i32 as isize) - *start.offset(1 as i32 as isize));
    trace.endpos[2 as i32 as usize] = *start.offset(2 as i32 as isize)
        + trace.fraction
            * (*end.offset(2 as i32 as isize) - *start.offset(2 as i32 as isize));
    *results = trace;
}

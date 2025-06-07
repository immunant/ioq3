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

    // trace->entityNum can also be 0 to (MAX_GENTITIES-1)
    // or ENTITYNUM_NONE, ENTITYNUM_WORLD
    // markfragments are returned by R_MarkFragments()

    #[inline]

    pub unsafe extern "C" fn VectorNormalizeFast(mut v: *mut crate::src::qcommon::q_shared::vec_t) {
        let mut ilength: libc::c_float = 0.;
        ilength = crate::src::qcommon::q_math::Q_rsqrt(
            *v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
                + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
                + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize),
        );
        let ref mut fresh0 = *v.offset(0 as libc::c_int as isize);
        *fresh0 *= ilength;
        let ref mut fresh1 = *v.offset(1 as libc::c_int as isize);
        *fresh1 *= ilength;
        let ref mut fresh2 = *v.offset(2 as libc::c_int as isize);
        *fresh2 *= ilength;
    }
    #[inline]

    pub unsafe extern "C" fn VectorInverse(mut v: *mut crate::src::qcommon::q_shared::vec_t) {
        *v.offset(0 as libc::c_int as isize) = -*v.offset(0 as libc::c_int as isize);
        *v.offset(1 as libc::c_int as isize) = -*v.offset(1 as libc::c_int as isize);
        *v.offset(2 as libc::c_int as isize) = -*v.offset(2 as libc::c_int as isize);
    }
    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0 as libc::c_int as isize) = *v1.offset(1 as libc::c_int as isize)
            * *v2.offset(2 as libc::c_int as isize)
            - *v1.offset(2 as libc::c_int as isize) * *v2.offset(1 as libc::c_int as isize);
        *cross.offset(1 as libc::c_int as isize) = *v1.offset(2 as libc::c_int as isize)
            * *v2.offset(0 as libc::c_int as isize)
            - *v1.offset(0 as libc::c_int as isize) * *v2.offset(2 as libc::c_int as isize);
        *cross.offset(2 as libc::c_int as isize) = *v1.offset(0 as libc::c_int as isize)
            * *v2.offset(1 as libc::c_int as isize)
            - *v1.offset(1 as libc::c_int as isize) * *v2.offset(0 as libc::c_int as isize);
    }

    // __Q_SHARED_H
}

pub use crate::qfiles_h::drawVert_t;
pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::src::qcommon::q_math::AddPointToBounds;
pub use crate::src::qcommon::q_math::BoxOnPlaneSide;
pub use crate::src::qcommon::q_math::ClearBounds;
pub use crate::src::qcommon::q_math::Q_rsqrt;
pub use crate::src::qcommon::q_math::VectorNormalize2;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::markFragment_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::renderergl1::tr_marks::q_shared_h::CrossProduct;
pub use crate::src::renderergl1::tr_marks::q_shared_h::VectorInverse;
pub use crate::src::renderergl1::tr_marks::q_shared_h::VectorNormalizeFast;
pub use crate::stdlib::GLuint;
pub use crate::tr_types_h::polyVert_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::stereoFrame_t;
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

pub use crate::src::renderergl1::tr_init::r_marksOnTriangleMeshes;
pub use crate::src::renderergl1::tr_main::tr;

pub use crate::tr_common_h::image_s;
pub use crate::tr_common_h::image_t;
pub use crate::tr_common_h::imgFlags_t;
pub use crate::tr_common_h::imgType_t;
pub use crate::tr_common_h::IMGFLAG_CLAMPTOEDGE;
pub use crate::tr_common_h::IMGFLAG_CUBEMAP;
pub use crate::tr_common_h::IMGFLAG_GENNORMALMAP;
pub use crate::tr_common_h::IMGFLAG_MIPMAP;
pub use crate::tr_common_h::IMGFLAG_NOLIGHTSCALE;
pub use crate::tr_common_h::IMGFLAG_NONE;
pub use crate::tr_common_h::IMGFLAG_NO_COMPRESSION;
pub use crate::tr_common_h::IMGFLAG_PICMIP;
pub use crate::tr_common_h::IMGTYPE_COLORALPHA;
pub use crate::tr_common_h::IMGTYPE_DELUXE;
pub use crate::tr_common_h::IMGTYPE_NORMAL;
pub use crate::tr_common_h::IMGTYPE_NORMALHEIGHT;
pub use crate::tr_local_h::acff_t;
pub use crate::tr_local_h::alphaGen_t;
pub use crate::tr_local_h::bmodel_t;
pub use crate::tr_local_h::colorGen_t;
pub use crate::tr_local_h::cullType_t;
pub use crate::tr_local_h::deformStage_t;
pub use crate::tr_local_h::deform_t;
pub use crate::tr_local_h::dlight_s;
pub use crate::tr_local_h::drawSurf_s;
pub use crate::tr_local_h::fogParms_t;
pub use crate::tr_local_h::fogPass_t;
pub use crate::tr_local_h::fog_t;
pub use crate::tr_local_h::frontEndCounters_t;
pub use crate::tr_local_h::genFunc_t;
pub use crate::tr_local_h::mnode_s;
pub use crate::tr_local_h::mnode_t;
pub use crate::tr_local_h::model_s;
pub use crate::tr_local_h::model_t;
pub use crate::tr_local_h::modtype_t;
pub use crate::tr_local_h::msurface_s;
pub use crate::tr_local_h::msurface_t;
pub use crate::tr_local_h::orientationr_t;
pub use crate::tr_local_h::shaderStage_t;
pub use crate::tr_local_h::shader_s;
pub use crate::tr_local_h::shader_t;
pub use crate::tr_local_h::skinSurface_t;
pub use crate::tr_local_h::skin_s;
pub use crate::tr_local_h::skin_t;
pub use crate::tr_local_h::skyParms_t;
pub use crate::tr_local_h::srfGridMesh_s;
pub use crate::tr_local_h::srfGridMesh_t;
pub use crate::tr_local_h::srfPoly_s;
pub use crate::tr_local_h::srfSurfaceFace_t;
pub use crate::tr_local_h::srfTriangles_t;
pub use crate::tr_local_h::surfaceType_t;
pub use crate::tr_local_h::texCoordGen_t;
pub use crate::tr_local_h::texModInfo_t;
pub use crate::tr_local_h::texMod_t;
pub use crate::tr_local_h::textureBundle_t;
pub use crate::tr_local_h::trGlobals_t;
pub use crate::tr_local_h::trRefEntity_t;
pub use crate::tr_local_h::trRefdef_t;
pub use crate::tr_local_h::viewParms_t;
pub use crate::tr_local_h::waveForm_t;
pub use crate::tr_local_h::world_t;
pub use crate::tr_local_h::ACFF_MODULATE_ALPHA;
pub use crate::tr_local_h::ACFF_MODULATE_RGB;
pub use crate::tr_local_h::ACFF_MODULATE_RGBA;
pub use crate::tr_local_h::ACFF_NONE;
pub use crate::tr_local_h::AGEN_CONST;
pub use crate::tr_local_h::AGEN_ENTITY;
pub use crate::tr_local_h::AGEN_IDENTITY;
pub use crate::tr_local_h::AGEN_LIGHTING_SPECULAR;
pub use crate::tr_local_h::AGEN_ONE_MINUS_ENTITY;
pub use crate::tr_local_h::AGEN_ONE_MINUS_VERTEX;
pub use crate::tr_local_h::AGEN_PORTAL;
pub use crate::tr_local_h::AGEN_SKIP;
pub use crate::tr_local_h::AGEN_VERTEX;
pub use crate::tr_local_h::AGEN_WAVEFORM;
pub use crate::tr_local_h::CGEN_BAD;
pub use crate::tr_local_h::CGEN_CONST;
pub use crate::tr_local_h::CGEN_ENTITY;
pub use crate::tr_local_h::CGEN_EXACT_VERTEX;
pub use crate::tr_local_h::CGEN_FOG;
pub use crate::tr_local_h::CGEN_IDENTITY;
pub use crate::tr_local_h::CGEN_IDENTITY_LIGHTING;
pub use crate::tr_local_h::CGEN_LIGHTING_DIFFUSE;
pub use crate::tr_local_h::CGEN_ONE_MINUS_ENTITY;
pub use crate::tr_local_h::CGEN_ONE_MINUS_VERTEX;
pub use crate::tr_local_h::CGEN_VERTEX;
pub use crate::tr_local_h::CGEN_WAVEFORM;
pub use crate::tr_local_h::CT_BACK_SIDED;
pub use crate::tr_local_h::CT_FRONT_SIDED;
pub use crate::tr_local_h::CT_TWO_SIDED;
pub use crate::tr_local_h::DEFORM_AUTOSPRITE;
pub use crate::tr_local_h::DEFORM_AUTOSPRITE2;
pub use crate::tr_local_h::DEFORM_BULGE;
pub use crate::tr_local_h::DEFORM_MOVE;
pub use crate::tr_local_h::DEFORM_NONE;
pub use crate::tr_local_h::DEFORM_NORMALS;
pub use crate::tr_local_h::DEFORM_PROJECTION_SHADOW;
pub use crate::tr_local_h::DEFORM_TEXT0;
pub use crate::tr_local_h::DEFORM_TEXT1;
pub use crate::tr_local_h::DEFORM_TEXT2;
pub use crate::tr_local_h::DEFORM_TEXT3;
pub use crate::tr_local_h::DEFORM_TEXT4;
pub use crate::tr_local_h::DEFORM_TEXT5;
pub use crate::tr_local_h::DEFORM_TEXT6;
pub use crate::tr_local_h::DEFORM_TEXT7;
pub use crate::tr_local_h::DEFORM_WAVE;
pub use crate::tr_local_h::FP_EQUAL;
pub use crate::tr_local_h::FP_LE;
pub use crate::tr_local_h::FP_NONE;
pub use crate::tr_local_h::GF_INVERSE_SAWTOOTH;
pub use crate::tr_local_h::GF_NOISE;
pub use crate::tr_local_h::GF_NONE;
pub use crate::tr_local_h::GF_SAWTOOTH;
pub use crate::tr_local_h::GF_SIN;
pub use crate::tr_local_h::GF_SQUARE;
pub use crate::tr_local_h::GF_TRIANGLE;
pub use crate::tr_local_h::MOD_BAD;
pub use crate::tr_local_h::MOD_BRUSH;
pub use crate::tr_local_h::MOD_IQM;
pub use crate::tr_local_h::MOD_MDR;
pub use crate::tr_local_h::MOD_MESH;
pub use crate::tr_local_h::SF_BAD;
pub use crate::tr_local_h::SF_ENTITY;
pub use crate::tr_local_h::SF_FACE;
pub use crate::tr_local_h::SF_FLARE;
pub use crate::tr_local_h::SF_GRID;
pub use crate::tr_local_h::SF_IQM;
pub use crate::tr_local_h::SF_MAX;
pub use crate::tr_local_h::SF_MD3;
pub use crate::tr_local_h::SF_MDR;
pub use crate::tr_local_h::SF_NUM_SURFACE_TYPES;
pub use crate::tr_local_h::SF_POLY;
pub use crate::tr_local_h::SF_SKIP;
pub use crate::tr_local_h::SF_TRIANGLES;
pub use crate::tr_local_h::TCGEN_BAD;
pub use crate::tr_local_h::TCGEN_ENVIRONMENT_MAPPED;
pub use crate::tr_local_h::TCGEN_FOG;
pub use crate::tr_local_h::TCGEN_IDENTITY;
pub use crate::tr_local_h::TCGEN_LIGHTMAP;
pub use crate::tr_local_h::TCGEN_TEXTURE;
pub use crate::tr_local_h::TCGEN_VECTOR;
pub use crate::tr_local_h::TMOD_ENTITY_TRANSLATE;
pub use crate::tr_local_h::TMOD_NONE;
pub use crate::tr_local_h::TMOD_ROTATE;
pub use crate::tr_local_h::TMOD_SCALE;
pub use crate::tr_local_h::TMOD_SCROLL;
pub use crate::tr_local_h::TMOD_STRETCH;
pub use crate::tr_local_h::TMOD_TRANSFORM;
pub use crate::tr_local_h::TMOD_TURBULENT;

unsafe extern "C" fn R_ChopPolyBehindPlane(
    mut numInPoints: libc::c_int,
    mut inPoints: *mut crate::src::qcommon::q_shared::vec3_t,
    mut numOutPoints: *mut libc::c_int,
    mut outPoints: *mut crate::src::qcommon::q_shared::vec3_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: crate::src::qcommon::q_shared::vec_t,
    mut epsilon: crate::src::qcommon::q_shared::vec_t,
) {
    let mut dists: [libc::c_float; 68] = [
        0 as libc::c_int as libc::c_float,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut sides: [libc::c_int; 68] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut counts: [libc::c_int; 3] = [0; 3];
    let mut dot: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut p2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut clip: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut d: libc::c_float = 0.;
    // don't clip if it might overflow
    if numInPoints >= 64 as libc::c_int - 2 as libc::c_int {
        *numOutPoints = 0 as libc::c_int;
        return;
    }
    counts[2 as libc::c_int as usize] = 0 as libc::c_int;
    counts[1 as libc::c_int as usize] = counts[2 as libc::c_int as usize];
    counts[0 as libc::c_int as usize] = counts[1 as libc::c_int as usize];
    // determine sides for each point
    i = 0 as libc::c_int;
    while i < numInPoints {
        dot = (*inPoints.offset(i as isize))[0 as libc::c_int as usize]
            * *normal.offset(0 as libc::c_int as isize)
            + (*inPoints.offset(i as isize))[1 as libc::c_int as usize]
                * *normal.offset(1 as libc::c_int as isize)
            + (*inPoints.offset(i as isize))[2 as libc::c_int as usize]
                * *normal.offset(2 as libc::c_int as isize);
        dot -= dist;
        dists[i as usize] = dot;
        if dot > epsilon {
            sides[i as usize] = 0 as libc::c_int
        } else if dot < -epsilon {
            sides[i as usize] = 1 as libc::c_int
        } else {
            sides[i as usize] = 2 as libc::c_int
        }
        counts[sides[i as usize] as usize] += 1;
        i += 1
    }
    sides[i as usize] = sides[0 as libc::c_int as usize];
    dists[i as usize] = dists[0 as libc::c_int as usize];
    *numOutPoints = 0 as libc::c_int;
    if counts[0 as libc::c_int as usize] == 0 {
        return;
    }
    if counts[1 as libc::c_int as usize] == 0 {
        *numOutPoints = numInPoints;
        crate::stdlib::memcpy(
            outPoints as *mut libc::c_void,
            inPoints as *const libc::c_void,
            (numInPoints as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::src::qcommon::q_shared::vec3_t,
            >() as libc::c_ulong),
        );
        return;
    }
    i = 0 as libc::c_int;
    while i < numInPoints {
        p1 = (*inPoints.offset(i as isize)).as_mut_ptr();
        clip = (*outPoints.offset(*numOutPoints as isize)).as_mut_ptr();
        if sides[i as usize] == 2 as libc::c_int {
            *clip.offset(0 as libc::c_int as isize) = *p1.offset(0 as libc::c_int as isize);
            *clip.offset(1 as libc::c_int as isize) = *p1.offset(1 as libc::c_int as isize);
            *clip.offset(2 as libc::c_int as isize) = *p1.offset(2 as libc::c_int as isize);
            *numOutPoints += 1
        } else {
            if sides[i as usize] == 0 as libc::c_int {
                *clip.offset(0 as libc::c_int as isize) = *p1.offset(0 as libc::c_int as isize);
                *clip.offset(1 as libc::c_int as isize) = *p1.offset(1 as libc::c_int as isize);
                *clip.offset(2 as libc::c_int as isize) = *p1.offset(2 as libc::c_int as isize);
                *numOutPoints += 1;
                clip = (*outPoints.offset(*numOutPoints as isize)).as_mut_ptr()
            }
            if !(sides[(i + 1 as libc::c_int) as usize] == 2 as libc::c_int
                || sides[(i + 1 as libc::c_int) as usize] == sides[i as usize])
            {
                // generate a split point
                p2 = (*inPoints.offset(((i + 1 as libc::c_int) % numInPoints) as isize))
                    .as_mut_ptr();
                d = dists[i as usize] - dists[(i + 1 as libc::c_int) as usize];
                if d == 0 as libc::c_int as libc::c_float {
                    dot = 0 as libc::c_int as libc::c_float
                } else {
                    dot = dists[i as usize] / d
                }
                // clip xyz
                j = 0 as libc::c_int;
                while j < 3 as libc::c_int {
                    *clip.offset(j as isize) = *p1.offset(j as isize)
                        + dot * (*p2.offset(j as isize) - *p1.offset(j as isize));
                    j += 1
                }
                *numOutPoints += 1
            }
        }
        i += 1
    }
}
/*
=================
R_BoxSurfaces_r

=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_BoxSurfaces_r(
    mut node: *mut crate::tr_local_h::mnode_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut list: *mut *mut crate::tr_local_h::surfaceType_t,
    mut listsize: libc::c_int,
    mut listlength: *mut libc::c_int,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut s: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut surf: *mut crate::tr_local_h::msurface_t = 0 as *mut crate::tr_local_h::msurface_t;
    let mut mark: *mut *mut crate::tr_local_h::msurface_t =
        0 as *mut *mut crate::tr_local_h::msurface_t;
    // do the tail recursion in a loop
    while (*node).contents == -(1 as libc::c_int) {
        s = crate::src::qcommon::q_math::BoxOnPlaneSide(
            mins,
            maxs,
            (*node).plane as *mut crate::src::qcommon::q_shared::cplane_s,
        );
        if s == 1 as libc::c_int {
            node = (*node).children[0 as libc::c_int as usize]
        } else if s == 2 as libc::c_int {
            node = (*node).children[1 as libc::c_int as usize]
        } else {
            R_BoxSurfaces_r(
                (*node).children[0 as libc::c_int as usize],
                mins,
                maxs,
                list,
                listsize,
                listlength,
                dir,
            );
            node = (*node).children[1 as libc::c_int as usize]
        }
    }
    // add the individual surfaces
    mark = (*node).firstmarksurface;
    c = (*node).nummarksurfaces;
    loop {
        let fresh3 = c;
        c = c - 1;
        if !(fresh3 != 0) {
            break;
        }
        //
        if *listlength >= listsize {
            break;
        }
        //
        surf = *mark;
        // check if the surface has NOIMPACT or NOMARKS set
        if (*(*surf).shader).surfaceFlags & (0x10 as libc::c_int | 0x20 as libc::c_int) != 0
            || (*(*surf).shader).contentFlags & 64 as libc::c_int != 0
        {
            (*surf).viewCount = crate::src::renderergl1::tr_main::tr.viewCount
        } else if *(*surf).data as libc::c_uint
            == crate::tr_local_h::SF_FACE as libc::c_int as libc::c_uint
        {
            // extra check for surfaces to avoid list overflows
            // the face plane should go through the box
            s = crate::src::qcommon::q_math::BoxOnPlaneSide(
                mins,
                maxs,
                &mut (*((*surf).data as *mut crate::tr_local_h::srfSurfaceFace_t)).plane as *mut _
                    as *mut crate::src::qcommon::q_shared::cplane_s,
            );
            if s == 1 as libc::c_int || s == 2 as libc::c_int {
                (*surf).viewCount = crate::src::renderergl1::tr_main::tr.viewCount
            } else if ((*((*surf).data as *mut crate::tr_local_h::srfSurfaceFace_t))
                .plane
                .normal[0 as libc::c_int as usize]
                * *dir.offset(0 as libc::c_int as isize)
                + (*((*surf).data as *mut crate::tr_local_h::srfSurfaceFace_t))
                    .plane
                    .normal[1 as libc::c_int as usize]
                    * *dir.offset(1 as libc::c_int as isize)
                + (*((*surf).data as *mut crate::tr_local_h::srfSurfaceFace_t))
                    .plane
                    .normal[2 as libc::c_int as usize]
                    * *dir.offset(2 as libc::c_int as isize))
                as libc::c_double
                > -0.5f64
            {
                // don't add faces that make sharp angles with the projection direction
                (*surf).viewCount = crate::src::renderergl1::tr_main::tr.viewCount
            }
        } else if *(*surf).data as libc::c_uint
            != crate::tr_local_h::SF_GRID as libc::c_int as libc::c_uint
            && *(*surf).data as libc::c_uint
                != crate::tr_local_h::SF_TRIANGLES as libc::c_int as libc::c_uint
        {
            (*surf).viewCount = crate::src::renderergl1::tr_main::tr.viewCount
        }
        // check the viewCount because the surface may have
        // already been added if it spans multiple leafs
        if (*surf).viewCount != crate::src::renderergl1::tr_main::tr.viewCount {
            (*surf).viewCount = crate::src::renderergl1::tr_main::tr.viewCount;
            let ref mut fresh4 = *list.offset(*listlength as isize);
            *fresh4 = (*surf).data;
            *listlength += 1
        }
        mark = mark.offset(1)
    }
}
/*
=================
R_AddMarkFragments

=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_AddMarkFragments(
    mut numClipPoints: libc::c_int,
    mut clipPoints: *mut [crate::src::qcommon::q_shared::vec3_t; 64],
    mut numPlanes: libc::c_int,
    mut normals: *mut crate::src::qcommon::q_shared::vec3_t,
    mut dists: *mut libc::c_float,
    mut maxPoints: libc::c_int,
    mut pointBuffer: *mut crate::src::qcommon::q_shared::vec_t,
    mut _maxFragments: libc::c_int,
    mut fragmentBuffer: *mut crate::src::qcommon::q_shared::markFragment_t,
    mut returnedPoints: *mut libc::c_int,
    mut returnedFragments: *mut libc::c_int,
    mut _mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut _maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut pingPong: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut mf: *mut crate::src::qcommon::q_shared::markFragment_t =
        0 as *mut crate::src::qcommon::q_shared::markFragment_t;
    // chop the surface by all the bounding planes of the to be projected polygon
    pingPong = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < numPlanes {
        R_ChopPolyBehindPlane(
            numClipPoints,
            (*clipPoints.offset(pingPong as isize)).as_mut_ptr(),
            &mut numClipPoints,
            (*clipPoints.offset((pingPong == 0) as libc::c_int as isize)).as_mut_ptr(),
            (*normals.offset(i as isize)).as_mut_ptr(),
            *dists.offset(i as isize),
            0.5f64 as crate::src::qcommon::q_shared::vec_t,
        );
        pingPong ^= 1 as libc::c_int;
        if numClipPoints == 0 as libc::c_int {
            break;
        }
        i += 1
    }
    // completely clipped away?
    if numClipPoints == 0 as libc::c_int {
        return;
    }
    // add this fragment to the returned list
    if numClipPoints + *returnedPoints > maxPoints {
        return;
        // not enough space for this polygon
    }
    /*
    // all the clip points should be within the bounding box
    for ( i = 0 ; i < numClipPoints ; i++ ) {
        int j;
        for ( j = 0 ; j < 3 ; j++ ) {
            if (clipPoints[pingPong][i][j] < mins[j] - 0.5) break;
            if (clipPoints[pingPong][i][j] > maxs[j] + 0.5) break;
        }
        if (j < 3) break;
    }
    if (i < numClipPoints) return;
    */
    mf = fragmentBuffer.offset(*returnedFragments as isize);
    (*mf).firstPoint = *returnedPoints;
    (*mf).numPoints = numClipPoints;
    crate::stdlib::memcpy(
        pointBuffer.offset((*returnedPoints * 3 as libc::c_int) as isize) as *mut libc::c_void,
        (*clipPoints.offset(pingPong as isize)).as_mut_ptr() as *const libc::c_void,
        (numClipPoints as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::src::qcommon::q_shared::vec3_t,
        >() as libc::c_ulong),
    );
    *returnedPoints += numClipPoints;
    *returnedFragments += 1;
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
// 14 bits
// can't be increased without changing bit packing for drawsurfs
// see QSORT_SHADERNUM_SHIFT
// range from 0.0 to 1.0, should be color normalized
// origin in local coordinate system
// texture detail is lost tho when the lightmap is dark
// a trRefEntity_t has all the information passed in by
// the client game, as well as some locally derived info
// compensate for non-normalized axis
// true for bmodels that touch a dlight
// normalized direction towards light
// color normalized to 0-255
// 32 bit rgba packed
// in world coordinates
// orientation in world
// viewParms->or.origin in local coordinates
//===============================================================================
// mirrors, portals, viewscreens
// sky box
// opaque
// scorch marks, etc.
// ladders, grates, grills that may have small blended edges
// in addition to alpha test
// for items that should be drawn in front of the water plane
// regular transparency and filters
// generally only used for additive type effects
// gun smoke puffs
// blood blobs
// tr.identityLight
// always (1,1,1,1)
// grabbed from entity's modulate field
// grabbed from 1 - entity.modulate
// tess.vertexColors
// tess.vertexColors * tr.identityLight
// programmatically generated
// standard fog
// fixed color
// clear to 0,0
// S and T from world coordinates
// vertex coordinate modification type
// used for TMOD_TURBULENT and TMOD_STRETCH
// used for TMOD_TRANSFORM
// s' = s * m[0][0] + t * m[1][0] + trans[0]
// t' = s * m[0][1] + t * m[0][1] + trans[1]
// used for TMOD_SCALE
// s *= scale[0]
// t *= scale[1]
// used for TMOD_SCROLL
// s' = s + scroll[0] * time
// t' = t + scroll[1] * time
// + = clockwise
// - = counterclockwise
// for CGEN_CONST and AGEN_CONST
// GLS_xxxx mask
// surface is translucent and will just be adjusted properly
// surface is opaque but possibly alpha tested
// surface is trnaslucent, but still needs a fog pass (fog surface)
// game path, including extension
// for a shader to match, both name and lightmapIndex must match
// this shader == tr.shaders[index]
// this shader == tr.sortedShaders[sortedIndex]
// lower numbered shaders draw before higher numbered
// we want to return index 0 if the shader failed to
// load for some reason, but R_FindShader should
// still keep a name allocated for it, so if
// something calls RE_RegisterShader again with
// the same name, we don't try looking for it again
// found in a .shader file
// if explicitlyDefined, this will have SURF_* flags
// merge across entites optimizable (smoke, blood)
// distance to fog out at
// 0, GL_MODULATE, GL_ADD (FIXME: put in stage)
// CT_FRONT_SIDED, CT_BACK_SIDED, or CT_TWO_SIDED
// set for decals and other items that must be offset
// for console fonts, 2D elements, etc.
// for images that must always be full resolution
// draw a blended pass, possibly with depth test equals
// not all shaders will need all data to be gathered
// time this shader is clamped to
// current time offset for this shader
// current shader this one is remapped too
// trRefdef_t holds everything that comes in refdef_t,
// as well as the locally generated scene information
// transformation matrix
// time in milliseconds for shader effects and other time dependent rendering issues
// RDF_NOWORLDMODEL, etc
// 1 bits will prevent the associated area from rendering at all
// qtrue if areamask changed since last scene
// tr.refdef.time / 1000.0
// text messages for deform text shaders
//=================================================================================
// max surfaces per-skin
// This is an arbitry limit. Vanilla Q3 only supported 32 surfaces in skins but failed to
// enforce the maximum limit when reading skin files. It was possile to use more than 32
// surfaces which accessed out of bounds memory past end of skin->surfaces hunk block.
// skins allow models to be retextured without modifying the model file
// game path, including extension
// dynamically allocated array of surfaces
// in packed byte format
// texture coordinate vector scales
// for clipping distance in fog when outside
// may be different than or.origin for portals
// true if this view is through a portal
// the portal is a mirror, invert the face culling
// copied from tr.frameSceneNum
// copied from tr.frameCount
// clip anything behind this if mirroring
/*
==============================================================================

SURFACES

==============================================================================
*/
// any changes in surfaceType must be mirrored in rb_surfaceTable[]
// ignore
// beams, rails, lightning, etc that can be determined by entity
// ensures that sizeof( surfaceType_t ) == sizeof( int )
// bit combination for fast compares
// any of surface*_t
// max dimensions of a patch mesh in map file
// max dimensions of a grid mesh in memory
// when cgame directly specifies a polygon, it becomes a srfPoly_t
// as soon as it is called
// dynamic lighting information
// culling information
// lod information, which may be different
// than the culling information to allow for
// groups of curves that LOD as a unit
// vertexes
// variable sized
// dynamic lighting information
// triangle definitions (no normals at points)
// variable sized
// there is a variable length list of indices here also
// misc_models in maps are turned into direct geometry by q3map
// dynamic lighting information
// culling information (FIXME: use this!)
// triangle definitions
// inter-quake-model
// vertex arrays
// [num_vertexes] indexes into influenceBlendVertexes
// unique list of vertex blend indexes/weights for faster CPU vertex skinning
// [num_influences]
// [num_influences]
// depending upon the exporter, blend indices and weights might be int/float
// as opposed to the recommended byte/byte, for example Noesis exports
// int/float whereas the official IQM tool exports byte/byte
// IQM_UBYTE or IQM_FLOAT
// inter-quake-model surface
/*
==============================================================================

BRUSH MODELS

==============================================================================
*/
//
// in memory representation
//
// if == tr.viewCount, already added
// any of srf*_t
// common with leaf and node
// -1 for nodes, to differentiate from leafs
// node needs to be traversed if current
// for bounding box culling
// node specific
// leaf specific
// for culling
// ie: maps/tim_dm2.bsp
// ie: tim_dm2
// includes leafs
// may be passed in by CM_LoadMap to save space
// clusterBytes of 0xff
//======================================================================
// model = tr.models[model->index]
// just for listing purposes
// only if type == MOD_BRUSH
// only if type == MOD_MESH
// only if type == (MOD_MDR | MOD_IQM)
//====================================================
/*

the drawsurf sort data is packed into a single 32 bit value so it can be
compared quickly during the qsorting process

the bits are allocated as follows:

0 - 1	: dlightmap index
//2		: used to be clipped flag REMOVED - 03.21.00 rad
2 - 6	: fog index
11 - 20	: entity index
21 - 31	: sorted shader index

    TTimo - 1.32
0-1   : dlightmap index
2-6   : fog index
7-16  : entity index
17-30 : sorted shader index
*/
/*
** performanceCounters_t
*/
// the renderer front end should never modify glstate_t
// total msec for backend run
// all state modified by the back end is separated
// from the front end state
// flag for drawing sun
// if qtrue, drawstretchpic doesn't need to change modes
// shader needs to be finished
// currentEntity will point at this when doing 2D rendering
/*
** trGlobals_t
**
** Most renderer globals are defined here.
** backend functions should never modify any of these fields,
** but may read fields that aren't dynamically modified
** by the frontend.
*/
// cleared at shutdown, set at beginRegistration
// incremented every time a new vis cluster is entered
// incremented every frame
// incremented every scene
// incremented every view (twice a scene if portaled)
// and every R_MarkFragments call
// zeroed at RE_BeginFrame
// from RE_SetWorldVisData, shared with CM_Load
// inverse-quare highlight for projective adding
// full of 0xff
// full of tr.identityLightByte
// point currentEntity at this when rendering world
// currentEntityNum << QSORT_REFENTITYNUM_SHIFT
// 1.0 / ( 1 << overbrightBits )
// identityLight * 255
// r_overbrightBits->integer, but set to 0 if no hw gamma
// for current entity
// from the sky shader for this level
// not in pc due to clearing issue
//
// put large tables at the end, so most elements will be
// within the +/32K indexed range on risc processors
//
// shader indexes from other modules will be looked up in tr.shaders[]
// shader indexes from drawsurfs will be looked up in sortedShaders[]
// lower indexed sortedShaders must be rendered first (opaque surfaces before translucent)
// outside of TR since it shouldn't be cleared during ref re-init
//
// cvars
//
// coefficient for the flare intensity falloff function.
// used for debugging anything
// used for verbose debug spew
// allows us to ignore our Tess fast paths
// near Z clip plane
// z distance of projection plane
// separation of cameras for stereo rendering
// enables stencil buffer overdraw measurement
// push/pull LOD transitions
// "0" = based on compiled vertex array existence
// "1" = glDrawElemet tristrips
// "2" = glDrawElements triangles
// "-1" = no drawing
// controls whether in game video should be draw
// controls whether sky should be cleared or drawn
// controls drawing of sun quad
// dynamic lights enabled/disabled
// dlight non-facing surfaces for continuity
// bypasses the ref rendering
// disable/enable entity rendering
// disable/enable world rendering
// various levels of information display
// enables/disables detail texturing stages
// disable/enable usage of PVS
// enables culling of planar surfaces with back side test
// optional display refresh option
// turns off binding to appropriate textures
// make most world faces use default shader
// development aid to see texture mip usage
// controls picmip values
// avoid lightmap pass
// render lightmaps only
// vertex lighting mode for better performance
// ui is running fullscreen
// number of frames to emit GL logs
// enables wireframe rendering of the world
// forces sky in front of all surfaces
// draws wireframe normals
// force screen clear every frame
// controls shadows: 0 = none, 1 = blur, 2 = stencil, 3 = black planar projection
// light flares
//====================================================================
// completely unclipped
// clipped by one or more planes
// completely outside the clipping planes
/*
** GL wrapper/helper functions
*/
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=516
//
// tr_shader.c
//
/*
====================================================================

TESSELATOR/SHADER DECLARATIONS

====================================================================
*/
// or together of all vertexDlightBits
// info extracted from current shader
/*
============================================================

WORLD MAP

============================================================
*/
/*
============================================================

FLARES

============================================================
*/
/*
============================================================

LIGHTS

============================================================
*/
/*
============================================================

SHADOWS

============================================================
*/
/*
============================================================

SKIES

============================================================
*/
/*
============================================================

CURVE TESSELATION

============================================================
*/
/*
============================================================

MARKERS, POLYGON PROJECTION ON WORLD POLYGONS

============================================================
*/
/*
=================
R_MarkFragments

=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_MarkFragments(
    mut numPoints: libc::c_int,
    mut points: *const crate::src::qcommon::q_shared::vec3_t,
    mut projection: *const crate::src::qcommon::q_shared::vec_t,
    mut maxPoints: libc::c_int,
    mut pointBuffer: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxFragments: libc::c_int,
    mut fragmentBuffer: *mut crate::src::qcommon::q_shared::markFragment_t,
) -> libc::c_int {
    let mut numsurfaces: libc::c_int = 0;
    let mut numPlanes: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut surfaces: [*mut crate::tr_local_h::surfaceType_t; 64] =
        [0 as *mut crate::tr_local_h::surfaceType_t; 64];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut returnedFragments: libc::c_int = 0;
    let mut returnedPoints: libc::c_int = 0;
    let mut normals: [crate::src::qcommon::q_shared::vec3_t; 66] = [[0.; 3]; 66];
    let mut dists: [libc::c_float; 66] = [0.; 66];
    let mut clipPoints: [[crate::src::qcommon::q_shared::vec3_t; 64]; 2] = [[[0.; 3]; 64]; 2];
    let mut numClipPoints: libc::c_int = 0;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut cv: *mut crate::tr_local_h::srfGridMesh_t = 0 as *mut crate::tr_local_h::srfGridMesh_t;
    let mut dv: *mut crate::qfiles_h::drawVert_t = 0 as *mut crate::qfiles_h::drawVert_t;
    let mut normal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut projectionDir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut indexes: *mut libc::c_int = 0 as *mut libc::c_int;
    if numPoints <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    //increment view count for double check prevention
    crate::src::renderergl1::tr_main::tr.viewCount += 1;
    //
    crate::src::qcommon::q_math::VectorNormalize2(projection, projectionDir.as_mut_ptr());
    // find all the brushes that are to be considered
    crate::src::qcommon::q_math::ClearBounds(mins.as_mut_ptr(), maxs.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < numPoints {
        let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        crate::src::qcommon::q_math::AddPointToBounds(
            (*points.offset(i as isize)).as_ptr(),
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        temp[0 as libc::c_int as usize] = (*points.offset(i as isize))[0 as libc::c_int as usize]
            + *projection.offset(0 as libc::c_int as isize);
        temp[1 as libc::c_int as usize] = (*points.offset(i as isize))[1 as libc::c_int as usize]
            + *projection.offset(1 as libc::c_int as isize);
        temp[2 as libc::c_int as usize] = (*points.offset(i as isize))[2 as libc::c_int as usize]
            + *projection.offset(2 as libc::c_int as isize);
        crate::src::qcommon::q_math::AddPointToBounds(
            temp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        // make sure we get all the leafs (also the one(s) in front of the hit surface)
        temp[0 as libc::c_int as usize] = (*points.offset(i as isize))[0 as libc::c_int as usize]
            + projectionDir[0 as libc::c_int as usize] * -(20 as libc::c_int) as libc::c_float;
        temp[1 as libc::c_int as usize] = (*points.offset(i as isize))[1 as libc::c_int as usize]
            + projectionDir[1 as libc::c_int as usize] * -(20 as libc::c_int) as libc::c_float;
        temp[2 as libc::c_int as usize] = (*points.offset(i as isize))[2 as libc::c_int as usize]
            + projectionDir[2 as libc::c_int as usize] * -(20 as libc::c_int) as libc::c_float;
        crate::src::qcommon::q_math::AddPointToBounds(
            temp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        i += 1
    }
    if numPoints > 64 as libc::c_int {
        numPoints = 64 as libc::c_int
    }
    // create the bounding planes for the to be projected polygon
    i = 0 as libc::c_int;
    while i < numPoints {
        v1[0 as libc::c_int as usize] = (*points
            .offset(((i + 1 as libc::c_int) % numPoints) as isize))[0 as libc::c_int as usize]
            - (*points.offset(i as isize))[0 as libc::c_int as usize];
        v1[1 as libc::c_int as usize] = (*points
            .offset(((i + 1 as libc::c_int) % numPoints) as isize))[1 as libc::c_int as usize]
            - (*points.offset(i as isize))[1 as libc::c_int as usize];
        v1[2 as libc::c_int as usize] = (*points
            .offset(((i + 1 as libc::c_int) % numPoints) as isize))[2 as libc::c_int as usize]
            - (*points.offset(i as isize))[2 as libc::c_int as usize];
        v2[0 as libc::c_int as usize] = (*points.offset(i as isize))[0 as libc::c_int as usize]
            + *projection.offset(0 as libc::c_int as isize);
        v2[1 as libc::c_int as usize] = (*points.offset(i as isize))[1 as libc::c_int as usize]
            + *projection.offset(1 as libc::c_int as isize);
        v2[2 as libc::c_int as usize] = (*points.offset(i as isize))[2 as libc::c_int as usize]
            + *projection.offset(2 as libc::c_int as isize);
        v2[0 as libc::c_int as usize] =
            (*points.offset(i as isize))[0 as libc::c_int as usize] - v2[0 as libc::c_int as usize];
        v2[1 as libc::c_int as usize] =
            (*points.offset(i as isize))[1 as libc::c_int as usize] - v2[1 as libc::c_int as usize];
        v2[2 as libc::c_int as usize] =
            (*points.offset(i as isize))[2 as libc::c_int as usize] - v2[2 as libc::c_int as usize];
        CrossProduct(
            v1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            v2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            normals[i as usize].as_mut_ptr(),
        );
        VectorNormalizeFast(normals[i as usize].as_mut_ptr());
        dists[i as usize] = normals[i as usize][0 as libc::c_int as usize]
            * (*points.offset(i as isize))[0 as libc::c_int as usize]
            + normals[i as usize][1 as libc::c_int as usize]
                * (*points.offset(i as isize))[1 as libc::c_int as usize]
            + normals[i as usize][2 as libc::c_int as usize]
                * (*points.offset(i as isize))[2 as libc::c_int as usize];
        i += 1
    }
    // add near and far clipping planes for projection
    normals[numPoints as usize][0 as libc::c_int as usize] =
        projectionDir[0 as libc::c_int as usize];
    normals[numPoints as usize][1 as libc::c_int as usize] =
        projectionDir[1 as libc::c_int as usize];
    normals[numPoints as usize][2 as libc::c_int as usize] =
        projectionDir[2 as libc::c_int as usize];
    dists[numPoints as usize] = normals[numPoints as usize][0 as libc::c_int as usize]
        * (*points.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + normals[numPoints as usize][1 as libc::c_int as usize]
            * (*points.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + normals[numPoints as usize][2 as libc::c_int as usize]
            * (*points.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        - 32 as libc::c_int as libc::c_float;
    normals[(numPoints + 1 as libc::c_int) as usize][0 as libc::c_int as usize] =
        projectionDir[0 as libc::c_int as usize];
    normals[(numPoints + 1 as libc::c_int) as usize][1 as libc::c_int as usize] =
        projectionDir[1 as libc::c_int as usize];
    normals[(numPoints + 1 as libc::c_int) as usize][2 as libc::c_int as usize] =
        projectionDir[2 as libc::c_int as usize];
    VectorInverse(normals[(numPoints + 1 as libc::c_int) as usize].as_mut_ptr());
    dists[(numPoints + 1 as libc::c_int) as usize] = normals
        [(numPoints + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
        * (*points.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + normals[(numPoints + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
            * (*points.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + normals[(numPoints + 1 as libc::c_int) as usize][2 as libc::c_int as usize]
            * (*points.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        - 20 as libc::c_int as libc::c_float;
    numPlanes = numPoints + 2 as libc::c_int;
    numsurfaces = 0 as libc::c_int;
    R_BoxSurfaces_r(
        (*crate::src::renderergl1::tr_main::tr.world).nodes,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        surfaces.as_mut_ptr(),
        64 as libc::c_int,
        &mut numsurfaces,
        projectionDir.as_mut_ptr(),
    );
    //assert(numsurfaces <= 64);
    //assert(numsurfaces != 64);
    returnedPoints = 0 as libc::c_int;
    returnedFragments = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < numsurfaces {
        if *surfaces[i as usize] as libc::c_uint
            == crate::tr_local_h::SF_GRID as libc::c_int as libc::c_uint
        {
            cv = surfaces[i as usize] as *mut crate::tr_local_h::srfGridMesh_t;
            m = 0 as libc::c_int;
            while m < (*cv).height - 1 as libc::c_int {
                n = 0 as libc::c_int;
                while n < (*cv).width - 1 as libc::c_int {
                    // We triangulate the grid and chop all triangles within
                    // the bounding planes of the to be projected polygon.
                    // LOD is not taken into account, not such a big deal though.
                    //
                    // It's probably much nicer to chop the grid itself and deal
                    // with this grid as a normal SF_GRID surface so LOD will
                    // be applied. However the LOD of that chopped grid must
                    // be synced with the LOD of the original curve.
                    // One way to do this; the chopped grid shares vertices with
                    // the original curve. When LOD is applied to the original
                    // curve the unused vertices are flagged. Now the chopped curve
                    // should skip the flagged vertices. This still leaves the
                    // problems with the vertices at the chopped grid edges.
                    //
                    // To avoid issues when LOD applied to "hollow curves" (like
                    // the ones around many jump pads) we now just add a 2 unit
                    // offset to the triangle vertices.
                    // The offset is added in the vertex normal vector direction
                    // so all triangles will still fit together.
                    // The 2 unit offset should avoid pretty much all LOD problems.
                    numClipPoints = 3 as libc::c_int;
                    dv = (*cv)
                        .verts
                        .as_mut_ptr()
                        .offset((m * (*cv).width) as isize)
                        .offset(n as isize);
                    clipPoints[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [0 as libc::c_int as usize] =
                        (*dv.offset(0 as libc::c_int as isize)).xyz[0 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [1 as libc::c_int as usize] =
                        (*dv.offset(0 as libc::c_int as isize)).xyz[1 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [2 as libc::c_int as usize] =
                        (*dv.offset(0 as libc::c_int as isize)).xyz[2 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [0 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [0 as libc::c_int as usize][0 as libc::c_int as usize]
                        + (*dv.offset(0 as libc::c_int as isize)).normal[0 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [1 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [0 as libc::c_int as usize][1 as libc::c_int as usize]
                        + (*dv.offset(0 as libc::c_int as isize)).normal[1 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [2 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [0 as libc::c_int as usize][2 as libc::c_int as usize]
                        + (*dv.offset(0 as libc::c_int as isize)).normal[2 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        [0 as libc::c_int as usize] =
                        (*dv.offset((*cv).width as isize)).xyz[0 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        [1 as libc::c_int as usize] =
                        (*dv.offset((*cv).width as isize)).xyz[1 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        [2 as libc::c_int as usize] =
                        (*dv.offset((*cv).width as isize)).xyz[2 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        [0 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [1 as libc::c_int as usize][0 as libc::c_int as usize]
                        + (*dv.offset((*cv).width as isize)).normal[0 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        [1 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [1 as libc::c_int as usize][1 as libc::c_int as usize]
                        + (*dv.offset((*cv).width as isize)).normal[1 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        [2 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [1 as libc::c_int as usize][2 as libc::c_int as usize]
                        + (*dv.offset((*cv).width as isize)).normal[2 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        [0 as libc::c_int as usize] =
                        (*dv.offset(1 as libc::c_int as isize)).xyz[0 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        [1 as libc::c_int as usize] =
                        (*dv.offset(1 as libc::c_int as isize)).xyz[1 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        [2 as libc::c_int as usize] =
                        (*dv.offset(1 as libc::c_int as isize)).xyz[2 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        [0 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [2 as libc::c_int as usize][0 as libc::c_int as usize]
                        + (*dv.offset(1 as libc::c_int as isize)).normal[0 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        [1 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [2 as libc::c_int as usize][1 as libc::c_int as usize]
                        + (*dv.offset(1 as libc::c_int as isize)).normal[1 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        [2 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [2 as libc::c_int as usize][2 as libc::c_int as usize]
                        + (*dv.offset(1 as libc::c_int as isize)).normal[2 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    // check the normal of this triangle
                    v1[0 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [0 as libc::c_int as usize][0 as libc::c_int as usize]
                        - clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            [0 as libc::c_int as usize];
                    v1[1 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [0 as libc::c_int as usize][1 as libc::c_int as usize]
                        - clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            [1 as libc::c_int as usize];
                    v1[2 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [0 as libc::c_int as usize][2 as libc::c_int as usize]
                        - clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            [2 as libc::c_int as usize];
                    v2[0 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [2 as libc::c_int as usize][0 as libc::c_int as usize]
                        - clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            [0 as libc::c_int as usize];
                    v2[1 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [2 as libc::c_int as usize][1 as libc::c_int as usize]
                        - clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            [1 as libc::c_int as usize];
                    v2[2 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [2 as libc::c_int as usize][2 as libc::c_int as usize]
                        - clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            [2 as libc::c_int as usize];
                    CrossProduct(
                        v1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        v2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        normal.as_mut_ptr(),
                    );
                    VectorNormalizeFast(normal.as_mut_ptr());
                    if ((normal[0 as libc::c_int as usize]
                        * projectionDir[0 as libc::c_int as usize]
                        + normal[1 as libc::c_int as usize]
                            * projectionDir[1 as libc::c_int as usize]
                        + normal[2 as libc::c_int as usize]
                            * projectionDir[2 as libc::c_int as usize])
                        as libc::c_double)
                        < -0.1f64
                    {
                        // add the fragments of this triangle
                        R_AddMarkFragments(
                            numClipPoints,
                            clipPoints.as_mut_ptr(),
                            numPlanes,
                            normals.as_mut_ptr(),
                            dists.as_mut_ptr(),
                            maxPoints,
                            pointBuffer,
                            maxFragments,
                            fragmentBuffer,
                            &mut returnedPoints,
                            &mut returnedFragments,
                            mins.as_mut_ptr(),
                            maxs.as_mut_ptr(),
                        );
                        if returnedFragments == maxFragments {
                            return returnedFragments;
                            // not enough space for more fragments
                        }
                    }
                    clipPoints[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [0 as libc::c_int as usize] =
                        (*dv.offset(1 as libc::c_int as isize)).xyz[0 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [1 as libc::c_int as usize] =
                        (*dv.offset(1 as libc::c_int as isize)).xyz[1 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [2 as libc::c_int as usize] =
                        (*dv.offset(1 as libc::c_int as isize)).xyz[2 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [0 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [0 as libc::c_int as usize][0 as libc::c_int as usize]
                        + (*dv.offset(1 as libc::c_int as isize)).normal[0 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [1 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [0 as libc::c_int as usize][1 as libc::c_int as usize]
                        + (*dv.offset(1 as libc::c_int as isize)).normal[1 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [2 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [0 as libc::c_int as usize][2 as libc::c_int as usize]
                        + (*dv.offset(1 as libc::c_int as isize)).normal[2 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        [0 as libc::c_int as usize] =
                        (*dv.offset((*cv).width as isize)).xyz[0 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        [1 as libc::c_int as usize] =
                        (*dv.offset((*cv).width as isize)).xyz[1 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        [2 as libc::c_int as usize] =
                        (*dv.offset((*cv).width as isize)).xyz[2 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        [0 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [1 as libc::c_int as usize][0 as libc::c_int as usize]
                        + (*dv.offset((*cv).width as isize)).normal[0 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        [1 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [1 as libc::c_int as usize][1 as libc::c_int as usize]
                        + (*dv.offset((*cv).width as isize)).normal[1 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        [2 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [1 as libc::c_int as usize][2 as libc::c_int as usize]
                        + (*dv.offset((*cv).width as isize)).normal[2 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        [0 as libc::c_int as usize] = (*dv
                        .offset(((*cv).width + 1 as libc::c_int) as isize))
                    .xyz[0 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        [1 as libc::c_int as usize] = (*dv
                        .offset(((*cv).width + 1 as libc::c_int) as isize))
                    .xyz[1 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        [2 as libc::c_int as usize] = (*dv
                        .offset(((*cv).width + 1 as libc::c_int) as isize))
                    .xyz[2 as libc::c_int as usize];
                    clipPoints[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        [0 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [2 as libc::c_int as usize][0 as libc::c_int as usize]
                        + (*dv.offset(((*cv).width + 1 as libc::c_int) as isize)).normal
                            [0 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        [1 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [2 as libc::c_int as usize][1 as libc::c_int as usize]
                        + (*dv.offset(((*cv).width + 1 as libc::c_int) as isize)).normal
                            [1 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        [2 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [2 as libc::c_int as usize][2 as libc::c_int as usize]
                        + (*dv.offset(((*cv).width + 1 as libc::c_int) as isize)).normal
                            [2 as libc::c_int as usize]
                            * 0 as libc::c_int as libc::c_float;
                    // check the normal of this triangle
                    v1[0 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [0 as libc::c_int as usize][0 as libc::c_int as usize]
                        - clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            [0 as libc::c_int as usize];
                    v1[1 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [0 as libc::c_int as usize][1 as libc::c_int as usize]
                        - clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            [1 as libc::c_int as usize];
                    v1[2 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [0 as libc::c_int as usize][2 as libc::c_int as usize]
                        - clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            [2 as libc::c_int as usize];
                    v2[0 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [2 as libc::c_int as usize][0 as libc::c_int as usize]
                        - clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            [0 as libc::c_int as usize];
                    v2[1 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [2 as libc::c_int as usize][1 as libc::c_int as usize]
                        - clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            [1 as libc::c_int as usize];
                    v2[2 as libc::c_int as usize] = clipPoints[0 as libc::c_int as usize]
                        [2 as libc::c_int as usize][2 as libc::c_int as usize]
                        - clipPoints[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            [2 as libc::c_int as usize];
                    CrossProduct(
                        v1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        v2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        normal.as_mut_ptr(),
                    );
                    VectorNormalizeFast(normal.as_mut_ptr());
                    if ((normal[0 as libc::c_int as usize]
                        * projectionDir[0 as libc::c_int as usize]
                        + normal[1 as libc::c_int as usize]
                            * projectionDir[1 as libc::c_int as usize]
                        + normal[2 as libc::c_int as usize]
                            * projectionDir[2 as libc::c_int as usize])
                        as libc::c_double)
                        < -0.05f64
                    {
                        // add the fragments of this triangle
                        R_AddMarkFragments(
                            numClipPoints,
                            clipPoints.as_mut_ptr(),
                            numPlanes,
                            normals.as_mut_ptr(),
                            dists.as_mut_ptr(),
                            maxPoints,
                            pointBuffer,
                            maxFragments,
                            fragmentBuffer,
                            &mut returnedPoints,
                            &mut returnedFragments,
                            mins.as_mut_ptr(),
                            maxs.as_mut_ptr(),
                        );
                        if returnedFragments == maxFragments {
                            return returnedFragments;
                            // not enough space for more fragments
                        }
                    }
                    n += 1
                }
                m += 1
            }
        } else if *surfaces[i as usize] as libc::c_uint
            == crate::tr_local_h::SF_FACE as libc::c_int as libc::c_uint
        {
            let mut surf: *mut crate::tr_local_h::srfSurfaceFace_t =
                surfaces[i as usize] as *mut crate::tr_local_h::srfSurfaceFace_t;
            // check the normal of this face
            if !(((*surf).plane.normal[0 as libc::c_int as usize]
                * projectionDir[0 as libc::c_int as usize]
                + (*surf).plane.normal[1 as libc::c_int as usize]
                    * projectionDir[1 as libc::c_int as usize]
                + (*surf).plane.normal[2 as libc::c_int as usize]
                    * projectionDir[2 as libc::c_int as usize]) as libc::c_double
                > -0.5f64)
            {
                indexes = (surf as *mut crate::src::qcommon::q_shared::byte)
                    .offset((*surf).ofsIndices as isize)
                    as *mut libc::c_int;
                k = 0 as libc::c_int;
                while k < (*surf).numIndices {
                    j = 0 as libc::c_int;
                    while j < 3 as libc::c_int {
                        v = (&mut *(*(*surf)
                            .points
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize)
                            as *mut libc::c_float)
                            .offset(
                                (8 as libc::c_int * *indexes.offset((k + j) as isize)) as isize,
                            );
                        clipPoints[0 as libc::c_int as usize][j as usize]
                            [0 as libc::c_int as usize] = *v.offset(0 as libc::c_int as isize)
                            + (*surf).plane.normal[0 as libc::c_int as usize]
                                * 0 as libc::c_int as libc::c_float;
                        clipPoints[0 as libc::c_int as usize][j as usize]
                            [1 as libc::c_int as usize] = *v.offset(1 as libc::c_int as isize)
                            + (*surf).plane.normal[1 as libc::c_int as usize]
                                * 0 as libc::c_int as libc::c_float;
                        clipPoints[0 as libc::c_int as usize][j as usize]
                            [2 as libc::c_int as usize] = *v.offset(2 as libc::c_int as isize)
                            + (*surf).plane.normal[2 as libc::c_int as usize]
                                * 0 as libc::c_int as libc::c_float;
                        j += 1
                    }
                    // add the fragments of this face
                    R_AddMarkFragments(
                        3 as libc::c_int,
                        clipPoints.as_mut_ptr(),
                        numPlanes,
                        normals.as_mut_ptr(),
                        dists.as_mut_ptr(),
                        maxPoints,
                        pointBuffer,
                        maxFragments,
                        fragmentBuffer,
                        &mut returnedPoints,
                        &mut returnedFragments,
                        mins.as_mut_ptr(),
                        maxs.as_mut_ptr(),
                    );
                    if returnedFragments == maxFragments {
                        return returnedFragments;
                        // not enough space for more fragments
                    }
                    k += 3 as libc::c_int
                }
            }
        } else if *surfaces[i as usize] as libc::c_uint
            == crate::tr_local_h::SF_TRIANGLES as libc::c_int as libc::c_uint
            && (*crate::src::renderergl1::tr_init::r_marksOnTriangleMeshes).integer != 0
        {
            let mut surf_0: *mut crate::tr_local_h::srfTriangles_t =
                surfaces[i as usize] as *mut crate::tr_local_h::srfTriangles_t;
            k = 0 as libc::c_int;
            while k < (*surf_0).numIndexes {
                j = 0 as libc::c_int;
                while j < 3 as libc::c_int {
                    v = (*(*surf_0)
                        .verts
                        .offset(*(*surf_0).indexes.offset((k + j) as isize) as isize))
                    .xyz
                    .as_mut_ptr();
                    clipPoints[0 as libc::c_int as usize][j as usize][0 as libc::c_int as usize] =
                        *v.offset(0 as libc::c_int as isize)
                            + (*(*surf_0)
                                .verts
                                .offset(*(*surf_0).indexes.offset((k + j) as isize) as isize))
                            .normal[0 as libc::c_int as usize]
                                * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][j as usize][1 as libc::c_int as usize] =
                        *v.offset(1 as libc::c_int as isize)
                            + (*(*surf_0)
                                .verts
                                .offset(*(*surf_0).indexes.offset((k + j) as isize) as isize))
                            .normal[1 as libc::c_int as usize]
                                * 0 as libc::c_int as libc::c_float;
                    clipPoints[0 as libc::c_int as usize][j as usize][2 as libc::c_int as usize] =
                        *v.offset(2 as libc::c_int as isize)
                            + (*(*surf_0)
                                .verts
                                .offset(*(*surf_0).indexes.offset((k + j) as isize) as isize))
                            .normal[2 as libc::c_int as usize]
                                * 0 as libc::c_int as libc::c_float;
                    j += 1
                }
                // add the fragments of this face
                R_AddMarkFragments(
                    3 as libc::c_int,
                    clipPoints.as_mut_ptr(),
                    numPlanes,
                    normals.as_mut_ptr(),
                    dists.as_mut_ptr(),
                    maxPoints,
                    pointBuffer,
                    maxFragments,
                    fragmentBuffer,
                    &mut returnedPoints,
                    &mut returnedFragments,
                    mins.as_mut_ptr(),
                    maxs.as_mut_ptr(),
                );
                if returnedFragments == maxFragments {
                    return returnedFragments;
                    // not enough space for more fragments
                }
                k += 3 as libc::c_int
            }
        }
        i += 1
    }
    return returnedFragments;
}

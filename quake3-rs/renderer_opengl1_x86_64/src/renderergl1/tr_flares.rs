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

    #[inline]

    pub unsafe extern "C" fn VectorNormalizeFast(mut v: *mut crate::src::qcommon::q_shared::vec_t) {
        let mut ilength: f32 = 0.;
        ilength = crate::src::qcommon::q_math::Q_rsqrt(
            *v.offset(0 as i32 as isize) * *v.offset(0 as i32 as isize)
                + *v.offset(1 as i32 as isize) * *v.offset(1 as i32 as isize)
                + *v.offset(2 as i32 as isize) * *v.offset(2 as i32 as isize),
        );
        let ref mut fresh0 = *v.offset(0 as i32 as isize);
        *fresh0 *= ilength;
        let ref mut fresh1 = *v.offset(1 as i32 as isize);
        *fresh1 *= ilength;
        let ref mut fresh2 = *v.offset(2 as i32 as isize);
        *fresh2 *= ilength;
    }

    // __Q_SHARED_H
}

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> f64 {
        return ::libc::strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    }
}

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::qgl_h::Disableproc;
pub use crate::qgl_h::LoadIdentityproc;
pub use crate::qgl_h::MatrixModeproc;
pub use crate::qgl_h::Orthoproc;
pub use crate::qgl_h::PopMatrixproc;
pub use crate::qgl_h::PushMatrixproc;
pub use crate::qgl_h::ReadPixelsproc;
pub use crate::src::qcommon::q_math::Q_rsqrt;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec2_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::renderergl1::tr_backend::backEnd;
pub use crate::src::renderergl1::tr_flares::q_shared_h::VectorNormalizeFast;
pub use crate::src::renderergl1::tr_flares::stdlib_float_h::atof;
pub use crate::src::renderergl1::tr_init::glState;
pub use crate::src::renderergl1::tr_init::r_flareCoeff;
pub use crate::src::renderergl1::tr_init::r_flareFade;
pub use crate::src::renderergl1::tr_init::r_flareSize;
pub use crate::src::renderergl1::tr_init::r_flares;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_main::R_TransformClipToWindow;
pub use crate::src::renderergl1::tr_main::R_TransformModelToClip;
pub use crate::src::renderergl1::tr_shade::tess;
pub use crate::src::renderergl1::tr_shade::RB_BeginSurface;
pub use crate::src::renderergl1::tr_shade::RB_EndSurface;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcModulateColorsByFog;
pub use crate::src::sdl::sdl_glimp::qglDisable;
pub use crate::src::sdl::sdl_glimp::qglLoadIdentity;
pub use crate::src::sdl::sdl_glimp::qglMatrixMode;
pub use crate::src::sdl::sdl_glimp::qglOrtho;
pub use crate::src::sdl::sdl_glimp::qglPopMatrix;
pub use crate::src::sdl::sdl_glimp::qglPushMatrix;
pub use crate::src::sdl::sdl_glimp::qglReadPixels;

pub use crate::stdlib::GLdouble;
pub use crate::stdlib::GLenum;
pub use crate::stdlib::GLint;
pub use crate::stdlib::GLsizei;
pub use crate::stdlib::GLuint;
pub use crate::stdlib::GLvoid;
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
pub use crate::tr_local_h::backEndCounters_t;
pub use crate::tr_local_h::backEndState_t;
pub use crate::tr_local_h::bmodel_t;
pub use crate::tr_local_h::color4ub_t;
pub use crate::tr_local_h::colorGen_t;
pub use crate::tr_local_h::cullType_t;
pub use crate::tr_local_h::deformStage_t;
pub use crate::tr_local_h::deform_t;
pub use crate::tr_local_h::dlight_s;
pub use crate::tr_local_h::dlight_t;
pub use crate::tr_local_h::drawSurf_s;
pub use crate::tr_local_h::fogParms_t;
pub use crate::tr_local_h::fogPass_t;
pub use crate::tr_local_h::fog_t;
pub use crate::tr_local_h::frontEndCounters_t;
pub use crate::tr_local_h::genFunc_t;
pub use crate::tr_local_h::glIndex_t;
pub use crate::tr_local_h::glstate_t;
pub use crate::tr_local_h::mnode_s;
pub use crate::tr_local_h::mnode_t;
pub use crate::tr_local_h::model_s;
pub use crate::tr_local_h::model_t;
pub use crate::tr_local_h::modtype_t;
pub use crate::tr_local_h::msurface_s;
pub use crate::tr_local_h::msurface_t;
pub use crate::tr_local_h::orientationr_t;
pub use crate::tr_local_h::shaderCommands_s;
pub use crate::tr_local_h::shaderCommands_t;
pub use crate::tr_local_h::shaderStage_t;
pub use crate::tr_local_h::shader_s;
pub use crate::tr_local_h::shader_t;
pub use crate::tr_local_h::skinSurface_t;
pub use crate::tr_local_h::skin_s;
pub use crate::tr_local_h::skin_t;
pub use crate::tr_local_h::skyParms_t;
pub use crate::tr_local_h::srfPoly_s;
pub use crate::tr_local_h::stageVars;
pub use crate::tr_local_h::stageVars_t;
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
// tr_flares.c
/*
=============================================================================

LIGHT FLARES

A light flare is an effect that takes place inside the eye when bright light
sources are visible.  The size of the flare relative to the screen is nearly
constant, irrespective of distance, but the intensity should be proportional to the
projected area of the light source.

A surface that has been flagged as having a light flare will calculate the depth
buffer value that its midpoint should have when the surface is added.

After all opaque surfaces have been rendered, the depth buffer is read back for
each flare in view.  If the point has not been obscured by a closer surface, the
flare should be drawn.

Surfaces that have a repeated texture should never be flagged as flaring, because
there will only be a single flare added at the midpoint of the polygon.

To prevent abrupt popping, the intensity of the flare is interpolated up and
down as it changes visibility.  This involves scene to scene state, unlike almost
all other aspects of the renderer, and is complicated by the fact that a single
frame may have multiple scenes.

RB_RenderFlares() will be called once per view (twice in a mirrored scene, potentially
up to five or more times in a frame with 3D status bar icons).

=============================================================================
*/
// flare states maintain visibility over multiple frames for fading
// layers: view, mirror, menu

pub type flare_t = flare_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flare_s {
    pub next: *mut flare_s,
    pub addedFrame: i32,
    pub inPortal: crate::src::qcommon::q_shared::qboolean,
    pub frameSceneNum: i32,
    pub surface: *mut libc::c_void,
    pub fogNum: i32,
    pub fadeTime: i32,
    pub visible: crate::src::qcommon::q_shared::qboolean,
    pub drawIntensity: f32,
    pub windowX: i32,
    pub windowY: i32,
    pub eyeZ: f32,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub color: crate::src::qcommon::q_shared::vec3_t,
}
#[no_mangle]

pub static mut r_flareStructs: [flare_t; 256] = [flare_t {
    next: 0 as *const flare_s as *mut flare_s,
    addedFrame: 0,
    inPortal: crate::src::qcommon::q_shared::qfalse,
    frameSceneNum: 0,
    surface: 0 as *const libc::c_void as *mut libc::c_void,
    fogNum: 0,
    fadeTime: 0,
    visible: crate::src::qcommon::q_shared::qfalse,
    drawIntensity: 0.,
    windowX: 0,
    windowY: 0,
    eyeZ: 0.,
    origin: [0.; 3],
    color: [0.; 3],
}; 256];
#[no_mangle]

pub static mut r_activeFlares: *mut flare_t = 0 as *const flare_t as *mut flare_t;
#[no_mangle]

pub static mut r_inactiveFlares: *mut flare_t = 0 as *const flare_t as *mut flare_t;
#[no_mangle]

pub static mut flareCoeff: i32 = 0;
// for active chain
// true if in a portal view of the scene
// state of last test
// may be non 0 even if !visible due to fading
/*
==================
R_SetFlareCoeff
==================
*/

unsafe extern "C" fn R_SetFlareCoeff() {
    if (*crate::src::renderergl1::tr_init::r_flareCoeff).value == 0.0f32 {
        flareCoeff = atof(b"150\x00" as *const u8 as *const libc::c_char) as i32
    } else {
        flareCoeff = (*crate::src::renderergl1::tr_init::r_flareCoeff).value as i32
    };
}
/*
==================
R_ClearFlares
==================
*/
#[no_mangle]

pub unsafe extern "C" fn R_ClearFlares() {
    let mut i: i32 = 0;
    crate::stdlib::memset(
        r_flareStructs.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[flare_t; 256]>() as libc::c_ulong,
    );
    r_activeFlares = 0 as *mut flare_t;
    r_inactiveFlares = 0 as *mut flare_t;
    i = 0 as i32;
    while i < 256 as i32 {
        r_flareStructs[i as usize].next = r_inactiveFlares;
        r_inactiveFlares = &mut *r_flareStructs.as_mut_ptr().offset(i as isize) as *mut flare_t;
        i += 1
    }
    R_SetFlareCoeff();
}
/*
==================
RB_AddFlare

This is called at surface tesselation time
==================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_AddFlare(
    mut surface: *mut libc::c_void,
    mut fogNum: i32,
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut i: i32 = 0;
    let mut f: *mut flare_t = 0 as *mut flare_t;
    let mut local: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d: f32 = 1 as i32 as f32;
    let mut eye: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut clip: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut normalized: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut window: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    crate::src::renderergl1::tr_backend::backEnd.pc.c_flareAdds += 1;
    if !normal.is_null()
        && (*normal.offset(0 as i32 as isize) != 0.
            || *normal.offset(1 as i32 as isize) != 0.
            || *normal.offset(2 as i32 as isize) != 0.)
    {
        local[0 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[0 as i32 as usize]
            - *point.offset(0 as i32 as isize);
        local[1 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[1 as i32 as usize]
            - *point.offset(1 as i32 as isize);
        local[2 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[2 as i32 as usize]
            - *point.offset(2 as i32 as isize);
        VectorNormalizeFast(local.as_mut_ptr());
        d = local[0 as i32 as usize] * *normal.offset(0 as i32 as isize)
            + local[1 as i32 as usize] * *normal.offset(1 as i32 as isize)
            + local[2 as i32 as usize] * *normal.offset(2 as i32 as isize);
        // If the viewer is behind the flare don't add it.
        if d < 0 as i32 as f32 {
            return;
        }
    }
    // if the point is off the screen, don't bother adding it
    // calculate screen coordinates and depth
    crate::src::renderergl1::tr_main::R_TransformModelToClip(
        point as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::renderergl1::tr_backend::backEnd
            .or
            .modelMatrix
            .as_mut_ptr(),
        crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .projectionMatrix
            .as_mut_ptr(),
        eye.as_mut_ptr(),
        clip.as_mut_ptr(),
    );
    // check to see if the point is completely off screen
    i = 0 as i32;
    while i < 3 as i32 {
        if clip[i as usize] >= clip[3 as i32 as usize]
            || clip[i as usize] <= -clip[3 as i32 as usize]
        {
            return;
        }
        i += 1
    }
    crate::src::renderergl1::tr_main::R_TransformClipToWindow(
        clip.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        &mut crate::src::renderergl1::tr_backend::backEnd.viewParms as *mut _
            as *const crate::tr_local_h::viewParms_t,
        normalized.as_mut_ptr(),
        window.as_mut_ptr(),
    );
    if window[0 as i32 as usize] < 0 as i32 as f32
        || window[0 as i32 as usize]
            >= crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .viewportWidth as f32
        || window[1 as i32 as usize] < 0 as i32 as f32
        || window[1 as i32 as usize]
            >= crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .viewportHeight as f32
    {
        return;
        // shouldn't happen, since we check the clip[] above, except for FP rounding
    }
    // see if a flare with a matching surface, scene, and view exists
    f = r_activeFlares;
    while !f.is_null() {
        if (*f).surface == surface
            && (*f).frameSceneNum
                == crate::src::renderergl1::tr_backend::backEnd
                    .viewParms
                    .frameSceneNum
            && (*f).inPortal as u32
                == crate::src::renderergl1::tr_backend::backEnd
                    .viewParms
                    .isPortal as u32
        {
            break;
        }
        f = (*f).next
    }
    // allocate a new one
    if f.is_null() {
        if r_inactiveFlares.is_null() {
            // the list is completely full
            return;
        }
        f = r_inactiveFlares;
        r_inactiveFlares = (*r_inactiveFlares).next;
        (*f).next = r_activeFlares;
        r_activeFlares = f;
        (*f).surface = surface;
        (*f).frameSceneNum = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .frameSceneNum;
        (*f).inPortal = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .isPortal;
        (*f).addedFrame = -(1 as i32)
    }
    if (*f).addedFrame
        != crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .frameCount
            - 1 as i32
    {
        (*f).visible = crate::src::qcommon::q_shared::qfalse;
        (*f).fadeTime = crate::src::renderergl1::tr_backend::backEnd.refdef.time - 2000 as i32
    }
    (*f).addedFrame = crate::src::renderergl1::tr_backend::backEnd
        .viewParms
        .frameCount;
    (*f).fogNum = fogNum;
    (*f).origin[0 as i32 as usize] = *point.offset(0 as i32 as isize);
    (*f).origin[1 as i32 as usize] = *point.offset(1 as i32 as isize);
    (*f).origin[2 as i32 as usize] = *point.offset(2 as i32 as isize);
    (*f).color[0 as i32 as usize] = *color.offset(0 as i32 as isize);
    (*f).color[1 as i32 as usize] = *color.offset(1 as i32 as isize);
    (*f).color[2 as i32 as usize] = *color.offset(2 as i32 as isize);
    // fade the intensity of the flare down as the
    // light surface turns away from the viewer
    (*f).color[0 as i32 as usize] = (*f).color[0 as i32 as usize] * d;
    (*f).color[1 as i32 as usize] = (*f).color[1 as i32 as usize] * d;
    (*f).color[2 as i32 as usize] = (*f).color[2 as i32 as usize] * d;
    // save info needed to test
    (*f).windowX = (crate::src::renderergl1::tr_backend::backEnd
        .viewParms
        .viewportX as f32
        + window[0 as i32 as usize]) as i32;
    (*f).windowY = (crate::src::renderergl1::tr_backend::backEnd
        .viewParms
        .viewportY as f32
        + window[1 as i32 as usize]) as i32;
    (*f).eyeZ = eye[2 as i32 as usize];
}
/*
==================
RB_AddDlightFlares
==================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_AddDlightFlares() {
    let mut l: *mut crate::tr_local_h::dlight_t = 0 as *mut crate::tr_local_h::dlight_t;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut fog: *mut crate::tr_local_h::fog_t = 0 as *mut crate::tr_local_h::fog_t;
    if (*crate::src::renderergl1::tr_init::r_flares).integer == 0 {
        return;
    }
    l = crate::src::renderergl1::tr_backend::backEnd.refdef.dlights;
    if !crate::src::renderergl1::tr_main::tr.world.is_null() {
        fog = (*crate::src::renderergl1::tr_main::tr.world).fogs
    }
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_backend::backEnd
        .refdef
        .num_dlights
    {
        if !fog.is_null() {
            // find which fog volume the light is in
            j = 1 as i32;
            while j < (*crate::src::renderergl1::tr_main::tr.world).numfogs {
                fog = &mut *(*crate::src::renderergl1::tr_main::tr.world)
                    .fogs
                    .offset(j as isize) as *mut crate::tr_local_h::fog_t;
                k = 0 as i32;
                while k < 3 as i32 {
                    if (*l).origin[k as usize] < (*fog).bounds[0 as i32 as usize][k as usize]
                        || (*l).origin[k as usize] > (*fog).bounds[1 as i32 as usize][k as usize]
                    {
                        break;
                    }
                    k += 1
                }
                if k == 3 as i32 {
                    break;
                }
                j += 1
            }
            if j == (*crate::src::renderergl1::tr_main::tr.world).numfogs {
                j = 0 as i32
            }
        } else {
            j = 0 as i32
        }
        RB_AddFlare(
            l as *mut libc::c_void,
            j,
            (*l).origin.as_mut_ptr(),
            (*l).color.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec_t,
        );
        i += 1;
        l = l.offset(1)
    }
}
/*
===============================================================================

FLARE BACK END

===============================================================================
*/
/*
==================
RB_TestFlare
==================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_TestFlare(mut f: *mut flare_t) {
    let mut depth: f32 = 0.;
    let mut visible: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut fade: f32 = 0.;
    let mut screenZ: f32 = 0.;
    crate::src::renderergl1::tr_backend::backEnd.pc.c_flareTests += 1;
    // doing a readpixels is as good as doing a glFinish(), so
    // don't bother with another sync
    crate::src::renderergl1::tr_init::glState.finishCalled = crate::src::qcommon::q_shared::qfalse;
    // read back the z buffer contents
    crate::src::sdl::sdl_glimp::qglReadPixels.expect("non-null function pointer")(
        (*f).windowX,
        (*f).windowY,
        1 as i32,
        1 as i32,
        0x1902 as i32 as crate::stdlib::GLenum,
        0x1406 as i32 as crate::stdlib::GLenum,
        &mut depth as *mut f32 as *mut libc::c_void,
    );
    screenZ = crate::src::renderergl1::tr_backend::backEnd
        .viewParms
        .projectionMatrix[14 as i32 as usize]
        / ((2 as i32 as f32 * depth - 1 as i32 as f32)
            * crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .projectionMatrix[11 as i32 as usize]
            - crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .projectionMatrix[10 as i32 as usize]);
    visible = (-(*f).eyeZ - -screenZ < 24 as i32 as f32) as i32
        as crate::src::qcommon::q_shared::qboolean;
    if visible as u64 != 0 {
        if (*f).visible as u64 == 0 {
            (*f).visible = crate::src::qcommon::q_shared::qtrue;
            (*f).fadeTime = crate::src::renderergl1::tr_backend::backEnd.refdef.time - 1 as i32
        }
        fade = (crate::src::renderergl1::tr_backend::backEnd.refdef.time - (*f).fadeTime) as f32
            / 1000.0f32
            * (*crate::src::renderergl1::tr_init::r_flareFade).value
    } else {
        if (*f).visible as u64 != 0 {
            (*f).visible = crate::src::qcommon::q_shared::qfalse;
            (*f).fadeTime = crate::src::renderergl1::tr_backend::backEnd.refdef.time - 1 as i32
        }
        fade = 1.0f32
            - (crate::src::renderergl1::tr_backend::backEnd.refdef.time - (*f).fadeTime) as f32
                / 1000.0f32
                * (*crate::src::renderergl1::tr_init::r_flareFade).value
    }
    if fade < 0 as i32 as f32 {
        fade = 0 as i32 as f32
    }
    if fade > 1 as i32 as f32 {
        fade = 1 as i32 as f32
    }
    (*f).drawIntensity = fade;
}
/*
==================
RB_RenderFlare
==================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_RenderFlare(mut f: *mut flare_t) {
    let mut size: f32 = 0.;
    let mut color: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut iColor: [i32; 3] = [0; 3];
    let mut distance: f32 = 0.;
    let mut intensity: f32 = 0.;
    let mut factor: f32 = 0.;
    let mut fogFactors: [crate::src::qcommon::q_shared::byte; 3] = [
        255 as i32 as crate::src::qcommon::q_shared::byte,
        255 as i32 as crate::src::qcommon::q_shared::byte,
        255 as i32 as crate::src::qcommon::q_shared::byte,
    ];
    crate::src::renderergl1::tr_backend::backEnd
        .pc
        .c_flareRenders += 1;
    // We don't want too big values anyways when dividing by distance.
    if (*f).eyeZ > -1.0f32 {
        distance = 1.0f32
    } else {
        distance = -(*f).eyeZ
    }
    // calculate the flare size..
    size = crate::src::renderergl1::tr_backend::backEnd
        .viewParms
        .viewportWidth as f32
        * ((*crate::src::renderergl1::tr_init::r_flareSize).value / 640.0f32
            + 8 as i32 as f32 / distance);
    /*
     * This is an alternative to intensity scaling. It changes the size of the flare on screen instead
     * with growing distance. See in the description at the top why this is not the way to go.
        // size will change ~ 1/r.
        size = backEnd.viewParms.viewportWidth * (r_flareSize->value / (distance * -2.0f));
    */
    /*
     * As flare sizes stay nearly constant with increasing distance we must decrease the intensity
     * to achieve a reasonable visual result. The intensity is ~ (size^2 / distance^2) which can be
     * got by considering the ratio of
     * (flaresurface on screen) : (Surface of sphere defined by flare origin and distance from flare)
     * An important requirement is:
     * intensity <= 1 for all distances.
     *
     * The formula used here to compute the intensity is as follows:
     * intensity = flareCoeff * size^2 / (distance + size*sqrt(flareCoeff))^2
     * As you can see, the intensity will have a max. of 1 when the distance is 0.
     * The coefficient flareCoeff will determine the falloff speed with increasing distance.
     */
    factor = (distance as f64 + size as f64 * crate::stdlib::sqrt(flareCoeff as f64)) as f32;
    intensity = flareCoeff as f32 * size * size / (factor * factor);
    color[0 as i32 as usize] = (*f).color[0 as i32 as usize] * ((*f).drawIntensity * intensity);
    color[1 as i32 as usize] = (*f).color[1 as i32 as usize] * ((*f).drawIntensity * intensity);
    color[2 as i32 as usize] = (*f).color[2 as i32 as usize] * ((*f).drawIntensity * intensity);
    // Calculations for fogging
    if !crate::src::renderergl1::tr_main::tr.world.is_null()
        && (*f).fogNum > 0 as i32
        && (*f).fogNum < (*crate::src::renderergl1::tr_main::tr.world).numfogs
    {
        crate::src::renderergl1::tr_shade::tess.numVertexes = 1 as i32;
        crate::src::renderergl1::tr_shade::tess.xyz[0 as i32 as usize][0 as i32 as usize] =
            (*f).origin[0 as i32 as usize];
        crate::src::renderergl1::tr_shade::tess.xyz[0 as i32 as usize][1 as i32 as usize] =
            (*f).origin[1 as i32 as usize];
        crate::src::renderergl1::tr_shade::tess.xyz[0 as i32 as usize][2 as i32 as usize] =
            (*f).origin[2 as i32 as usize];
        crate::src::renderergl1::tr_shade::tess.fogNum = (*f).fogNum;
        crate::src::renderergl1::tr_shade_calc::RB_CalcModulateColorsByFog(fogFactors.as_mut_ptr());
        // We don't need to render the flare if colors are 0 anyways.
        if !(fogFactors[0 as i32 as usize] as i32 != 0
            || fogFactors[1 as i32 as usize] as i32 != 0
            || fogFactors[2 as i32 as usize] as i32 != 0)
        {
            return;
        }
    }
    iColor[0 as i32 as usize] =
        (color[0 as i32 as usize] * fogFactors[0 as i32 as usize] as i32 as f32) as i32;
    iColor[1 as i32 as usize] =
        (color[1 as i32 as usize] * fogFactors[1 as i32 as usize] as i32 as f32) as i32;
    iColor[2 as i32 as usize] =
        (color[2 as i32 as usize] * fogFactors[2 as i32 as usize] as i32 as f32) as i32;
    crate::src::renderergl1::tr_shade::RB_BeginSurface(
        crate::src::renderergl1::tr_main::tr.flareShader as *mut crate::tr_local_h::shader_s,
        (*f).fogNum,
    );
    // FIXME: use quadstamp?
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize] =
        (*f).windowX as f32 - size;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1 as i32 as usize] =
        (*f).windowY as f32 - size;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize]
        [0 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize]
        [1 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize] =
        iColor[0 as i32 as usize] as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1 as i32 as usize] =
        iColor[1 as i32 as usize] as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2 as i32 as usize] =
        iColor[2 as i32 as usize] as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][3 as i32 as usize] =
        255 as i32 as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.numVertexes += 1;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize] =
        (*f).windowX as f32 - size;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1 as i32 as usize] =
        (*f).windowY as f32 + size;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize]
        [0 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize]
        [1 as i32 as usize] = 1 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize] =
        iColor[0 as i32 as usize] as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1 as i32 as usize] =
        iColor[1 as i32 as usize] as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2 as i32 as usize] =
        iColor[2 as i32 as usize] as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][3 as i32 as usize] =
        255 as i32 as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.numVertexes += 1;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize] =
        (*f).windowX as f32 + size;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1 as i32 as usize] =
        (*f).windowY as f32 + size;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize]
        [0 as i32 as usize] = 1 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize]
        [1 as i32 as usize] = 1 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize] =
        iColor[0 as i32 as usize] as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1 as i32 as usize] =
        iColor[1 as i32 as usize] as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2 as i32 as usize] =
        iColor[2 as i32 as usize] as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][3 as i32 as usize] =
        255 as i32 as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.numVertexes += 1;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize] =
        (*f).windowX as f32 + size;
    crate::src::renderergl1::tr_shade::tess.xyz
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1 as i32 as usize] =
        (*f).windowY as f32 - size;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize]
        [0 as i32 as usize] = 1 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::renderergl1::tr_shade::tess.texCoords
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize]
        [1 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][0 as i32 as usize] =
        iColor[0 as i32 as usize] as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][1 as i32 as usize] =
        iColor[1 as i32 as usize] as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][2 as i32 as usize] =
        iColor[2 as i32 as usize] as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.vertexColors
        [crate::src::renderergl1::tr_shade::tess.numVertexes as usize][3 as i32 as usize] =
        255 as i32 as crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_shade::tess.numVertexes += 1;
    let fresh3 = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numIndexes =
        crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
    crate::src::renderergl1::tr_shade::tess.indexes[fresh3 as usize] =
        0 as i32 as crate::tr_local_h::glIndex_t;
    let fresh4 = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numIndexes =
        crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
    crate::src::renderergl1::tr_shade::tess.indexes[fresh4 as usize] =
        1 as i32 as crate::tr_local_h::glIndex_t;
    let fresh5 = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numIndexes =
        crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
    crate::src::renderergl1::tr_shade::tess.indexes[fresh5 as usize] =
        2 as i32 as crate::tr_local_h::glIndex_t;
    let fresh6 = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numIndexes =
        crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
    crate::src::renderergl1::tr_shade::tess.indexes[fresh6 as usize] =
        0 as i32 as crate::tr_local_h::glIndex_t;
    let fresh7 = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numIndexes =
        crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
    crate::src::renderergl1::tr_shade::tess.indexes[fresh7 as usize] =
        2 as i32 as crate::tr_local_h::glIndex_t;
    let fresh8 = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numIndexes =
        crate::src::renderergl1::tr_shade::tess.numIndexes + 1;
    crate::src::renderergl1::tr_shade::tess.indexes[fresh8 as usize] =
        3 as i32 as crate::tr_local_h::glIndex_t;
    crate::src::renderergl1::tr_shade::RB_EndSurface();
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
==================
RB_RenderFlares

Because flares are simulating an occular effect, they should be drawn after
everything (all views) in the entire frame has been drawn.

Because of the way portals use the depth buffer to mark off areas, the
needed information would be lost after each view, so we are forced to draw
flares after each view.

The resulting artifact is that flares in mirrors or portals don't dim properly
when occluded by something in the main view, and portal flares that should
extend past the portal edge will be overwritten.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_RenderFlares() {
    let mut f: *mut flare_t = 0 as *mut flare_t;
    let mut prev: *mut *mut flare_t = 0 as *mut *mut flare_t;
    let mut draw: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    if (*crate::src::renderergl1::tr_init::r_flares).integer == 0 {
        return;
    }
    if (*crate::src::renderergl1::tr_init::r_flareCoeff).modified as u64 != 0 {
        R_SetFlareCoeff();
        (*crate::src::renderergl1::tr_init::r_flareCoeff).modified =
            crate::src::qcommon::q_shared::qfalse
    }
    // Reset currentEntity to world so that any previously referenced entities
    // don't have influence on the rendering of these flares (i.e. RF_ renderer flags).
    crate::src::renderergl1::tr_backend::backEnd.currentEntity =
        &mut crate::src::renderergl1::tr_main::tr.worldEntity;
    crate::src::renderergl1::tr_backend::backEnd.or =
        crate::src::renderergl1::tr_backend::backEnd.viewParms.world;
    //	RB_AddDlightFlares();
    // perform z buffer readback on each flare in this view
    draw = crate::src::qcommon::q_shared::qfalse;
    prev = &mut r_activeFlares;
    loop {
        f = *prev;
        if f.is_null() {
            break;
        }
        // throw out any flares that weren't added last frame
        if (*f).addedFrame
            < crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .frameCount
                - 1 as i32
        {
            *prev = (*f).next;
            (*f).next = r_inactiveFlares;
            r_inactiveFlares = f
        } else {
            // don't draw any here that aren't from this scene / portal
            (*f).drawIntensity = 0 as i32 as f32;
            if (*f).frameSceneNum
                == crate::src::renderergl1::tr_backend::backEnd
                    .viewParms
                    .frameSceneNum
                && (*f).inPortal as u32
                    == crate::src::renderergl1::tr_backend::backEnd
                        .viewParms
                        .isPortal as u32
            {
                RB_TestFlare(f);
                if (*f).drawIntensity != 0. {
                    draw = crate::src::qcommon::q_shared::qtrue
                } else {
                    // this flare has completely faded out, so remove it from the chain
                    *prev = (*f).next;
                    (*f).next = r_inactiveFlares;
                    r_inactiveFlares = f;
                    continue;
                }
            }
            prev = &mut (*f).next
        }
    }
    if draw as u64 == 0 {
        return;
        // none visible
    }
    if crate::src::renderergl1::tr_backend::backEnd
        .viewParms
        .isPortal as u64
        != 0
    {
        crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
            0x3000 as i32 as crate::stdlib::GLenum,
        );
    }
    crate::src::sdl::sdl_glimp::qglPushMatrix.expect("non-null function pointer")();
    crate::src::sdl::sdl_glimp::qglLoadIdentity.expect("non-null function pointer")();
    crate::src::sdl::sdl_glimp::qglMatrixMode.expect("non-null function pointer")(
        0x1701 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglPushMatrix.expect("non-null function pointer")();
    crate::src::sdl::sdl_glimp::qglLoadIdentity.expect("non-null function pointer")();
    crate::src::sdl::sdl_glimp::qglOrtho.expect("non-null function pointer")(
        crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .viewportX as crate::stdlib::GLdouble,
        (crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .viewportX
            + crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .viewportWidth) as crate::stdlib::GLdouble,
        crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .viewportY as crate::stdlib::GLdouble,
        (crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .viewportY
            + crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .viewportHeight) as crate::stdlib::GLdouble,
        -(99999 as i32) as crate::stdlib::GLdouble,
        99999 as i32 as crate::stdlib::GLdouble,
    );
    f = r_activeFlares;
    while !f.is_null() {
        if (*f).frameSceneNum
            == crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .frameSceneNum
            && (*f).inPortal as u32
                == crate::src::renderergl1::tr_backend::backEnd
                    .viewParms
                    .isPortal as u32
            && (*f).drawIntensity != 0.
        {
            RB_RenderFlare(f);
        }
        f = (*f).next
    }
    crate::src::sdl::sdl_glimp::qglPopMatrix.expect("non-null function pointer")();
    crate::src::sdl::sdl_glimp::qglMatrixMode.expect("non-null function pointer")(
        0x1700 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglPopMatrix.expect("non-null function pointer")();
}

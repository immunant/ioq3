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

    //
    // these aren't needed by any of the VMs.  put in another header?
    //
    // bit vector of area visibility
    // print levels from renderer (FIXME: set up for game / cgame?)

    // only print when "developer 1"

    // parameters to the main Error routine

    // pop up the need-cd dialog
    // client disconnected from the server

    // don't kill server

    // print to console and disconnect from game

    // exit the entire game with a popup window

    // font rendering values used by ui and cgame
    // default
    // default

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

    // server browser sources
    // TTimo: AS_MPLAYER is no longer used
    // cinematic states

    // all other conditions, i.e. stop/EOF/abort

    // play

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

pub use crate::stdlib::__int64_t;
pub use crate::stdlib::int64_t;

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::Q_rsqrt;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec2_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::FMV_EOF;
pub use crate::src::qcommon::q_shared::FMV_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_BLT;
pub use crate::src::qcommon::q_shared::FMV_ID_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_WAIT;
pub use crate::src::qcommon::q_shared::FMV_LOOPED;
pub use crate::src::qcommon::q_shared::FMV_PLAY;
pub use crate::src::qcommon::q_shared::PRINT_ALL;
pub use crate::src::qcommon::q_shared::PRINT_DEVELOPER;
pub use crate::src::qcommon::q_shared::PRINT_ERROR;
pub use crate::src::qcommon::q_shared::PRINT_WARNING;
pub use crate::src::renderergl1::tr_shade_calc::q_shared_h::CrossProduct;
pub use crate::src::renderergl1::tr_shade_calc::q_shared_h::VectorLength;
pub use crate::src::renderergl1::tr_shade_calc::q_shared_h::VectorNormalizeFast;
pub use crate::stdlib::GLuint;
pub use crate::tr_public_h::refimport_t;
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

pub use crate::src::renderercommon::tr_noise::R_NoiseGet4f;
pub use crate::src::renderergl1::tr_backend::backEnd;
pub use crate::src::renderergl1::tr_image::R_FogFactor;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_shade::tess;
pub use crate::src::renderergl1::tr_shadows::RB_ProjectionShadowDeform;
pub use crate::src::renderergl1::tr_surface::RB_AddQuadStamp;
pub use crate::src::renderergl1::tr_surface::RB_AddQuadStampExt;

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
pub use crate::tr_local_h::drawSurf_s;
pub use crate::tr_local_h::fogParms_t;
pub use crate::tr_local_h::fogPass_t;
pub use crate::tr_local_h::fog_t;
pub use crate::tr_local_h::frontEndCounters_t;
pub use crate::tr_local_h::genFunc_t;
pub use crate::tr_local_h::glIndex_t;
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
// tr_shade_calc.c

unsafe extern "C" fn TableForFunc(mut func: crate::tr_local_h::genFunc_t) -> *mut f32 {
    match func as u32 {
        1 => return crate::src::renderergl1::tr_main::tr.sinTable.as_mut_ptr(),
        3 => {
            return crate::src::renderergl1::tr_main::tr
                .triangleTable
                .as_mut_ptr()
        }
        2 => {
            return crate::src::renderergl1::tr_main::tr
                .squareTable
                .as_mut_ptr()
        }
        4 => {
            return crate::src::renderergl1::tr_main::tr
                .sawToothTable
                .as_mut_ptr()
        }
        5 => {
            return crate::src::renderergl1::tr_main::tr
                .inverseSawToothTable
                .as_mut_ptr()
        }
        0 | _ => {}
    }
    crate::src::renderergl1::tr_main::ri
        .Error
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::ERR_DROP as i32,
        b"TableForFunc called with invalid function \'%d\' in shader \'%s\'\x00" as *const u8
            as *const libc::c_char,
        func as u32,
        (*crate::src::renderergl1::tr_shade::tess.shader)
            .name
            .as_mut_ptr(),
    );
}
/*
** EvalWaveForm
**
** Evaluates a given waveForm_t, referencing backEnd.refdef.time directly
*/

unsafe extern "C" fn EvalWaveForm(mut wf: *const crate::tr_local_h::waveForm_t) -> f32 {
    let mut table: *mut f32 = 0 as *mut f32;
    table = TableForFunc((*wf).func);
    return (*wf).base
        + *table.offset(
            ((((*wf).phase as f64
                + crate::src::renderergl1::tr_shade::tess.shaderTime * (*wf).frequency as f64)
                * 1024 as i32 as f64) as crate::stdlib::int64_t
                & (1024 as i32 - 1 as i32) as isize) as isize,
        ) * (*wf).amplitude;
}

unsafe extern "C" fn EvalWaveFormClamped(mut wf: *const crate::tr_local_h::waveForm_t) -> f32 {
    let mut glow: f32 = EvalWaveForm(wf);
    if glow < 0 as i32 as f32 {
        return 0 as i32 as f32;
    }
    if glow > 1 as i32 as f32 {
        return 1 as i32 as f32;
    }
    return glow;
}
/*
** RB_CalcStretchTexCoords
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcStretchTexCoords(
    mut wf: *const crate::tr_local_h::waveForm_t,
    mut st: *mut f32,
) {
    let mut p: f32 = 0.;
    let mut tmi: crate::tr_local_h::texModInfo_t = crate::tr_local_h::texModInfo_t {
        type_0: crate::tr_local_h::TMOD_NONE,
        wave: crate::tr_local_h::waveForm_t {
            func: crate::tr_local_h::GF_NONE,
            base: 0.,
            amplitude: 0.,
            phase: 0.,
            frequency: 0.,
        },
        matrix: [[0.; 2]; 2],
        translate: [0.; 2],
        scale: [0.; 2],
        scroll: [0.; 2],
        rotateSpeed: 0.,
    };
    p = 1.0f32 / EvalWaveForm(wf);
    tmi.matrix[0 as i32 as usize][0 as i32 as usize] = p;
    tmi.matrix[1 as i32 as usize][0 as i32 as usize] = 0 as i32 as f32;
    tmi.translate[0 as i32 as usize] = 0.5f32 - 0.5f32 * p;
    tmi.matrix[0 as i32 as usize][1 as i32 as usize] = 0 as i32 as f32;
    tmi.matrix[1 as i32 as usize][1 as i32 as usize] = p;
    tmi.translate[1 as i32 as usize] = 0.5f32 - 0.5f32 * p;
    RB_CalcTransformTexCoords(&mut tmi, st);
}
/*
====================================================================

DEFORMATIONS

====================================================================
*/
/*
========================
RB_CalcDeformVertexes

========================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcDeformVertexes(mut ds: *mut crate::tr_local_h::deformStage_t) {
    let mut i: i32 = 0;
    let mut offset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut scale: f32 = 0.;
    let mut xyz: *mut f32 = crate::src::renderergl1::tr_shade::tess.xyz.as_mut_ptr() as *mut f32;
    let mut normal: *mut f32 =
        crate::src::renderergl1::tr_shade::tess.normal.as_mut_ptr() as *mut f32;
    let mut table: *mut f32 = 0 as *mut f32;
    if (*ds).deformationWave.frequency == 0 as i32 as f32 {
        scale = EvalWaveForm(&mut (*ds).deformationWave);
        i = 0 as i32;
        while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
            offset[0 as i32 as usize] = *normal.offset(0 as i32 as isize) * scale;
            offset[1 as i32 as usize] = *normal.offset(1 as i32 as isize) * scale;
            offset[2 as i32 as usize] = *normal.offset(2 as i32 as isize) * scale;
            *xyz.offset(0 as i32 as isize) += offset[0 as i32 as usize];
            *xyz.offset(1 as i32 as isize) += offset[1 as i32 as usize];
            *xyz.offset(2 as i32 as isize) += offset[2 as i32 as usize];
            i += 1;
            xyz = xyz.offset(4 as i32 as isize);
            normal = normal.offset(4 as i32 as isize)
        }
    } else {
        table = TableForFunc((*ds).deformationWave.func);
        i = 0 as i32;
        while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
            let mut off: f32 = (*xyz.offset(0 as i32 as isize)
                + *xyz.offset(1 as i32 as isize)
                + *xyz.offset(2 as i32 as isize))
                * (*ds).deformationSpread;
            scale = (*ds).deformationWave.base
                + *table.offset(
                    (((((*ds).deformationWave.phase + off) as f64
                        + crate::src::renderergl1::tr_shade::tess.shaderTime
                            * (*ds).deformationWave.frequency as f64)
                        * 1024 as i32 as f64) as crate::stdlib::int64_t
                        & (1024 as i32 - 1 as i32) as isize) as isize,
                ) * (*ds).deformationWave.amplitude;
            offset[0 as i32 as usize] = *normal.offset(0 as i32 as isize) * scale;
            offset[1 as i32 as usize] = *normal.offset(1 as i32 as isize) * scale;
            offset[2 as i32 as usize] = *normal.offset(2 as i32 as isize) * scale;
            *xyz.offset(0 as i32 as isize) += offset[0 as i32 as usize];
            *xyz.offset(1 as i32 as isize) += offset[1 as i32 as usize];
            *xyz.offset(2 as i32 as isize) += offset[2 as i32 as usize];
            i += 1;
            xyz = xyz.offset(4 as i32 as isize);
            normal = normal.offset(4 as i32 as isize)
        }
    };
}
/*
=========================
RB_CalcDeformNormals

Wiggle the normals for wavy environment mapping
=========================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcDeformNormals(mut ds: *mut crate::tr_local_h::deformStage_t) {
    let mut i: i32 = 0;
    let mut scale: f32 = 0.;
    let mut xyz: *mut f32 = crate::src::renderergl1::tr_shade::tess.xyz.as_mut_ptr() as *mut f32;
    let mut normal: *mut f32 =
        crate::src::renderergl1::tr_shade::tess.normal.as_mut_ptr() as *mut f32;
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        scale = 0.98f32;
        scale = crate::src::renderercommon::tr_noise::R_NoiseGet4f(
            *xyz.offset(0 as i32 as isize) * scale,
            *xyz.offset(1 as i32 as isize) * scale,
            *xyz.offset(2 as i32 as isize) * scale,
            crate::src::renderergl1::tr_shade::tess.shaderTime
                * (*ds).deformationWave.frequency as f64,
        );
        *normal.offset(0 as i32 as isize) += (*ds).deformationWave.amplitude * scale;
        scale = 0.98f32;
        scale = crate::src::renderercommon::tr_noise::R_NoiseGet4f(
            100 as i32 as f32 + *xyz.offset(0 as i32 as isize) * scale,
            *xyz.offset(1 as i32 as isize) * scale,
            *xyz.offset(2 as i32 as isize) * scale,
            crate::src::renderergl1::tr_shade::tess.shaderTime
                * (*ds).deformationWave.frequency as f64,
        );
        *normal.offset(1 as i32 as isize) += (*ds).deformationWave.amplitude * scale;
        scale = 0.98f32;
        scale = crate::src::renderercommon::tr_noise::R_NoiseGet4f(
            200 as i32 as f32 + *xyz.offset(0 as i32 as isize) * scale,
            *xyz.offset(1 as i32 as isize) * scale,
            *xyz.offset(2 as i32 as isize) * scale,
            crate::src::renderergl1::tr_shade::tess.shaderTime
                * (*ds).deformationWave.frequency as f64,
        );
        *normal.offset(2 as i32 as isize) += (*ds).deformationWave.amplitude * scale;
        VectorNormalizeFast(normal);
        i += 1;
        xyz = xyz.offset(4 as i32 as isize);
        normal = normal.offset(4 as i32 as isize)
    }
}
/*
========================
RB_CalcBulgeVertexes

========================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcBulgeVertexes(mut ds: *mut crate::tr_local_h::deformStage_t) {
    let mut i: i32 = 0;
    let mut st: *const f32 = crate::src::renderergl1::tr_shade::tess.texCoords[0 as i32 as usize]
        .as_mut_ptr() as *const f32;
    let mut xyz: *mut f32 = crate::src::renderergl1::tr_shade::tess.xyz.as_mut_ptr() as *mut f32;
    let mut normal: *mut f32 =
        crate::src::renderergl1::tr_shade::tess.normal.as_mut_ptr() as *mut f32;
    let mut now: f64 = 0.;
    now = crate::src::renderergl1::tr_backend::backEnd.refdef.time as f64
        * 0.001f64
        * (*ds).bulgeSpeed as f64;
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        let mut off: crate::stdlib::int64_t = 0;
        let mut scale: f32 = 0.;
        off = ((1024 as i32 as f64 / (3.14159265358979323846f64 * 2 as i32 as f64)) as f32 as f64
            * ((*st.offset(0 as i32 as isize) * (*ds).bulgeWidth) as f64 + now))
            as crate::stdlib::int64_t;
        scale = crate::src::renderergl1::tr_main::tr.sinTable
            [(off & (1024 as i32 - 1 as i32) as isize) as usize]
            * (*ds).bulgeHeight;
        *xyz.offset(0 as i32 as isize) += *normal.offset(0 as i32 as isize) * scale;
        *xyz.offset(1 as i32 as isize) += *normal.offset(1 as i32 as isize) * scale;
        *xyz.offset(2 as i32 as isize) += *normal.offset(2 as i32 as isize) * scale;
        i += 1;
        xyz = xyz.offset(4 as i32 as isize);
        st = st.offset(4 as i32 as isize);
        normal = normal.offset(4 as i32 as isize)
    }
}
/*
======================
RB_CalcMoveVertexes

A deformation that can move an entire surface along a wave path
======================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcMoveVertexes(mut ds: *mut crate::tr_local_h::deformStage_t) {
    let mut i: i32 = 0;
    let mut xyz: *mut f32 = 0 as *mut f32;
    let mut table: *mut f32 = 0 as *mut f32;
    let mut scale: f32 = 0.;
    let mut offset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    table = TableForFunc((*ds).deformationWave.func);
    scale = (*ds).deformationWave.base
        + *table.offset(
            ((((*ds).deformationWave.phase as f64
                + crate::src::renderergl1::tr_shade::tess.shaderTime
                    * (*ds).deformationWave.frequency as f64)
                * 1024 as i32 as f64) as crate::stdlib::int64_t
                & (1024 as i32 - 1 as i32) as isize) as isize,
        ) * (*ds).deformationWave.amplitude;
    offset[0 as i32 as usize] = (*ds).moveVector[0 as i32 as usize] * scale;
    offset[1 as i32 as usize] = (*ds).moveVector[1 as i32 as usize] * scale;
    offset[2 as i32 as usize] = (*ds).moveVector[2 as i32 as usize] * scale;
    xyz = crate::src::renderergl1::tr_shade::tess.xyz.as_mut_ptr() as *mut f32;
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        *xyz.offset(0 as i32 as isize) = *xyz.offset(0 as i32 as isize) + offset[0 as i32 as usize];
        *xyz.offset(1 as i32 as isize) = *xyz.offset(1 as i32 as isize) + offset[1 as i32 as usize];
        *xyz.offset(2 as i32 as isize) = *xyz.offset(2 as i32 as isize) + offset[2 as i32 as usize];
        i += 1;
        xyz = xyz.offset(4 as i32 as isize)
    }
}
/*
=============
DeformText

Change a polygon into a bunch of text polygons
=============
*/
#[no_mangle]

pub unsafe extern "C" fn DeformText(mut text: *const libc::c_char) {
    let mut i: i32 = 0;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut width: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut height: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut len: i32 = 0;
    let mut ch: i32 = 0;
    let mut color: [crate::src::qcommon::q_shared::byte; 4] = [0; 4];
    let mut bottom: f32 = 0.;
    let mut top: f32 = 0.;
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    height[0 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    height[1 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    height[2 as i32 as usize] = -(1 as i32) as crate::src::qcommon::q_shared::vec_t;
    CrossProduct(
        crate::src::renderergl1::tr_shade::tess.normal[0 as i32 as usize].as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        height.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        width.as_mut_ptr(),
    );
    // find the midpoint of the box
    mid[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    mid[1 as i32 as usize] = mid[2 as i32 as usize];
    mid[0 as i32 as usize] = mid[1 as i32 as usize];
    bottom = 999999 as i32 as f32;
    top = -(999999 as i32) as f32;
    i = 0 as i32;
    while i < 4 as i32 {
        mid[0 as i32 as usize] = crate::src::renderergl1::tr_shade::tess.xyz[i as usize]
            [0 as i32 as usize]
            + mid[0 as i32 as usize];
        mid[1 as i32 as usize] = crate::src::renderergl1::tr_shade::tess.xyz[i as usize]
            [1 as i32 as usize]
            + mid[1 as i32 as usize];
        mid[2 as i32 as usize] = crate::src::renderergl1::tr_shade::tess.xyz[i as usize]
            [2 as i32 as usize]
            + mid[2 as i32 as usize];
        if crate::src::renderergl1::tr_shade::tess.xyz[i as usize][2 as i32 as usize] < bottom {
            bottom = crate::src::renderergl1::tr_shade::tess.xyz[i as usize][2 as i32 as usize]
        }
        if crate::src::renderergl1::tr_shade::tess.xyz[i as usize][2 as i32 as usize] > top {
            top = crate::src::renderergl1::tr_shade::tess.xyz[i as usize][2 as i32 as usize]
        }
        i += 1
    }
    origin[0 as i32 as usize] = mid[0 as i32 as usize] * 0.25f32;
    origin[1 as i32 as usize] = mid[1 as i32 as usize] * 0.25f32;
    origin[2 as i32 as usize] = mid[2 as i32 as usize] * 0.25f32;
    // determine the individual character size
    height[0 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    height[1 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    height[2 as i32 as usize] = (top - bottom) * 0.5f32;
    width[0 as i32 as usize] = width[0 as i32 as usize] * (height[2 as i32 as usize] * -0.75f32);
    width[1 as i32 as usize] = width[1 as i32 as usize] * (height[2 as i32 as usize] * -0.75f32);
    width[2 as i32 as usize] = width[2 as i32 as usize] * (height[2 as i32 as usize] * -0.75f32);
    // determine the starting position
    len = crate::stdlib::strlen(text) as i32;
    origin[0 as i32 as usize] =
        origin[0 as i32 as usize] + width[0 as i32 as usize] * (len - 1 as i32) as f32;
    origin[1 as i32 as usize] =
        origin[1 as i32 as usize] + width[1 as i32 as usize] * (len - 1 as i32) as f32;
    origin[2 as i32 as usize] =
        origin[2 as i32 as usize] + width[2 as i32 as usize] * (len - 1 as i32) as f32;
    // clear the shader indexes
    crate::src::renderergl1::tr_shade::tess.numIndexes = 0 as i32;
    crate::src::renderergl1::tr_shade::tess.numVertexes = 0 as i32;
    color[3 as i32 as usize] = 255 as i32 as crate::src::qcommon::q_shared::byte;
    color[2 as i32 as usize] = color[3 as i32 as usize];
    color[1 as i32 as usize] = color[2 as i32 as usize];
    color[0 as i32 as usize] = color[1 as i32 as usize];
    // draw each character
    i = 0 as i32;
    while i < len {
        ch = *text.offset(i as isize) as i32;
        ch &= 255 as i32;
        if ch != ' ' as i32 {
            let mut row: i32 = 0;
            let mut col: i32 = 0;
            let mut frow: f32 = 0.;
            let mut fcol: f32 = 0.;
            let mut size: f32 = 0.;
            row = ch >> 4 as i32;
            col = ch & 15 as i32;
            frow = row as f32 * 0.0625f32;
            fcol = col as f32 * 0.0625f32;
            size = 0.0625f32;
            crate::src::renderergl1::tr_surface::RB_AddQuadStampExt(
                origin.as_mut_ptr(),
                width.as_mut_ptr(),
                height.as_mut_ptr(),
                color.as_mut_ptr(),
                fcol,
                frow,
                fcol + size,
                frow + size,
            );
        }
        origin[0 as i32 as usize] =
            origin[0 as i32 as usize] + width[0 as i32 as usize] * -(2 as i32) as f32;
        origin[1 as i32 as usize] =
            origin[1 as i32 as usize] + width[1 as i32 as usize] * -(2 as i32) as f32;
        origin[2 as i32 as usize] =
            origin[2 as i32 as usize] + width[2 as i32 as usize] * -(2 as i32) as f32;
        i += 1
    }
}
/*
==================
GlobalVectorToLocal
==================
*/

unsafe extern "C" fn GlobalVectorToLocal(
    mut in_0: *const crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0 as i32 as isize) = *in_0.offset(0 as i32 as isize)
        * crate::src::renderergl1::tr_backend::backEnd.or.axis[0 as i32 as usize]
            [0 as i32 as usize]
        + *in_0.offset(1 as i32 as isize)
            * crate::src::renderergl1::tr_backend::backEnd.or.axis[0 as i32 as usize]
                [1 as i32 as usize]
        + *in_0.offset(2 as i32 as isize)
            * crate::src::renderergl1::tr_backend::backEnd.or.axis[0 as i32 as usize]
                [2 as i32 as usize];
    *out.offset(1 as i32 as isize) = *in_0.offset(0 as i32 as isize)
        * crate::src::renderergl1::tr_backend::backEnd.or.axis[1 as i32 as usize]
            [0 as i32 as usize]
        + *in_0.offset(1 as i32 as isize)
            * crate::src::renderergl1::tr_backend::backEnd.or.axis[1 as i32 as usize]
                [1 as i32 as usize]
        + *in_0.offset(2 as i32 as isize)
            * crate::src::renderergl1::tr_backend::backEnd.or.axis[1 as i32 as usize]
                [2 as i32 as usize];
    *out.offset(2 as i32 as isize) = *in_0.offset(0 as i32 as isize)
        * crate::src::renderergl1::tr_backend::backEnd.or.axis[2 as i32 as usize]
            [0 as i32 as usize]
        + *in_0.offset(1 as i32 as isize)
            * crate::src::renderergl1::tr_backend::backEnd.or.axis[2 as i32 as usize]
                [1 as i32 as usize]
        + *in_0.offset(2 as i32 as isize)
            * crate::src::renderergl1::tr_backend::backEnd.or.axis[2 as i32 as usize]
                [2 as i32 as usize];
}
/*
=====================
AutospriteDeform

Assuming all the triangles for this shader are independent
quads, rebuild them as forward facing sprites
=====================
*/

unsafe extern "C" fn AutospriteDeform() {
    let mut i: i32 = 0;
    let mut oldVerts: i32 = 0;
    let mut xyz: *mut f32 = 0 as *mut f32;
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut delta: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut radius: f32 = 0.;
    let mut left: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut leftDir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut upDir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if crate::src::renderergl1::tr_shade::tess.numVertexes & 3 as i32 != 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"Autosprite shader %s had odd vertex count\n\x00" as *const u8 as *const libc::c_char,
            (*crate::src::renderergl1::tr_shade::tess.shader)
                .name
                .as_mut_ptr(),
        );
    }
    if crate::src::renderergl1::tr_shade::tess.numIndexes
        != (crate::src::renderergl1::tr_shade::tess.numVertexes >> 2 as i32) * 6 as i32
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"Autosprite shader %s had odd index count\n\x00" as *const u8 as *const libc::c_char,
            (*crate::src::renderergl1::tr_shade::tess.shader)
                .name
                .as_mut_ptr(),
        );
    }
    oldVerts = crate::src::renderergl1::tr_shade::tess.numVertexes;
    crate::src::renderergl1::tr_shade::tess.numVertexes = 0 as i32;
    crate::src::renderergl1::tr_shade::tess.numIndexes = 0 as i32;
    if crate::src::renderergl1::tr_backend::backEnd.currentEntity
        != &mut crate::src::renderergl1::tr_main::tr.worldEntity
            as *mut crate::tr_local_h::trRefEntity_t
    {
        GlobalVectorToLocal(
            crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[1 as i32 as usize]
                .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            leftDir.as_mut_ptr(),
        );
        GlobalVectorToLocal(
            crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[2 as i32 as usize]
                .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            upDir.as_mut_ptr(),
        );
    } else {
        leftDir[0 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[1 as i32 as usize][0 as i32 as usize];
        leftDir[1 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[1 as i32 as usize][1 as i32 as usize];
        leftDir[2 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[1 as i32 as usize][2 as i32 as usize];
        upDir[0 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[2 as i32 as usize][0 as i32 as usize];
        upDir[1 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[2 as i32 as usize][1 as i32 as usize];
        upDir[2 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[2 as i32 as usize][2 as i32 as usize]
    }
    i = 0 as i32;
    while i < oldVerts {
        // find the midpoint
        xyz = crate::src::renderergl1::tr_shade::tess.xyz[i as usize].as_mut_ptr(); // / sqrt(2)
        mid[0 as i32 as usize] = 0.25f32
            * (*xyz.offset(0 as i32 as isize)
                + *xyz.offset(4 as i32 as isize)
                + *xyz.offset(8 as i32 as isize)
                + *xyz.offset(12 as i32 as isize));
        mid[1 as i32 as usize] = 0.25f32
            * (*xyz.offset(1 as i32 as isize)
                + *xyz.offset(5 as i32 as isize)
                + *xyz.offset(9 as i32 as isize)
                + *xyz.offset(13 as i32 as isize));
        mid[2 as i32 as usize] = 0.25f32
            * (*xyz.offset(2 as i32 as isize)
                + *xyz.offset(6 as i32 as isize)
                + *xyz.offset(10 as i32 as isize)
                + *xyz.offset(14 as i32 as isize));
        delta[0 as i32 as usize] = *xyz.offset(0 as i32 as isize) - mid[0 as i32 as usize];
        delta[1 as i32 as usize] = *xyz.offset(1 as i32 as isize) - mid[1 as i32 as usize];
        delta[2 as i32 as usize] = *xyz.offset(2 as i32 as isize) - mid[2 as i32 as usize];
        radius = VectorLength(delta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
            * 0.707f32;
        left[0 as i32 as usize] = leftDir[0 as i32 as usize] * radius;
        left[1 as i32 as usize] = leftDir[1 as i32 as usize] * radius;
        left[2 as i32 as usize] = leftDir[2 as i32 as usize] * radius;
        up[0 as i32 as usize] = upDir[0 as i32 as usize] * radius;
        up[1 as i32 as usize] = upDir[1 as i32 as usize] * radius;
        up[2 as i32 as usize] = upDir[2 as i32 as usize] * radius;
        if crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .isMirror as u64
            != 0
        {
            left[0 as i32 as usize] = crate::src::qcommon::q_math::vec3_origin[0 as i32 as usize]
                - left[0 as i32 as usize];
            left[1 as i32 as usize] = crate::src::qcommon::q_math::vec3_origin[1 as i32 as usize]
                - left[1 as i32 as usize];
            left[2 as i32 as usize] = crate::src::qcommon::q_math::vec3_origin[2 as i32 as usize]
                - left[2 as i32 as usize]
        }
        // compensate for scale in the axes if necessary
        if (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .nonNormalizedAxes as u64
            != 0
        {
            let mut axisLength: f32 = 0.;
            axisLength = VectorLength(
                (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
                    .e
                    .axis[0 as i32 as usize]
                    .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            );
            if axisLength == 0. {
                axisLength = 0 as i32 as f32
            } else {
                axisLength = 1.0f32 / axisLength
            }
            left[0 as i32 as usize] = left[0 as i32 as usize] * axisLength;
            left[1 as i32 as usize] = left[1 as i32 as usize] * axisLength;
            left[2 as i32 as usize] = left[2 as i32 as usize] * axisLength;
            up[0 as i32 as usize] = up[0 as i32 as usize] * axisLength;
            up[1 as i32 as usize] = up[1 as i32 as usize] * axisLength;
            up[2 as i32 as usize] = up[2 as i32 as usize] * axisLength
        }
        crate::src::renderergl1::tr_surface::RB_AddQuadStamp(
            mid.as_mut_ptr(),
            left.as_mut_ptr(),
            up.as_mut_ptr(),
            crate::src::renderergl1::tr_shade::tess.vertexColors[i as usize].as_mut_ptr(),
        );
        i += 4 as i32
    }
}
/*
=====================
Autosprite2Deform

Autosprite2 will pivot a rectangular quad along the center of its long axis
=====================
*/
#[no_mangle]

pub static mut edgeVerts: [[i32; 2]; 6] = [
    [0 as i32, 1 as i32],
    [0 as i32, 2 as i32],
    [0 as i32, 3 as i32],
    [1 as i32, 2 as i32],
    [1 as i32, 3 as i32],
    [2 as i32, 3 as i32],
];

unsafe extern "C" fn Autosprite2Deform() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut indexes: i32 = 0;
    let mut xyz: *mut f32 = 0 as *mut f32;
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if crate::src::renderergl1::tr_shade::tess.numVertexes & 3 as i32 != 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"Autosprite2 shader %s had odd vertex count\x00" as *const u8 as *const libc::c_char,
            (*crate::src::renderergl1::tr_shade::tess.shader)
                .name
                .as_mut_ptr(),
        );
    }
    if crate::src::renderergl1::tr_shade::tess.numIndexes
        != (crate::src::renderergl1::tr_shade::tess.numVertexes >> 2 as i32) * 6 as i32
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"Autosprite2 shader %s had odd index count\x00" as *const u8 as *const libc::c_char,
            (*crate::src::renderergl1::tr_shade::tess.shader)
                .name
                .as_mut_ptr(),
        );
    }
    if crate::src::renderergl1::tr_backend::backEnd.currentEntity
        != &mut crate::src::renderergl1::tr_main::tr.worldEntity
            as *mut crate::tr_local_h::trRefEntity_t
    {
        GlobalVectorToLocal(
            crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[0 as i32 as usize]
                .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            forward.as_mut_ptr(),
        );
    } else {
        forward[0 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[0 as i32 as usize][0 as i32 as usize];
        forward[1 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[0 as i32 as usize][1 as i32 as usize];
        forward[2 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[0 as i32 as usize][2 as i32 as usize]
    }
    // this is a lot of work for two triangles...
    // we could precalculate a lot of it is an issue, but it would mess up
    // the shader abstraction
    i = 0 as i32;
    indexes = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        let mut lengths: [f32; 2] = [0.; 2];
        let mut nums: [i32; 2] = [0; 2];
        let mut mid: [crate::src::qcommon::q_shared::vec3_t; 2] = [[0.; 3]; 2];
        let mut major: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut minor: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut v1: *mut f32 = 0 as *mut f32;
        let mut v2: *mut f32 = 0 as *mut f32;
        // find the midpoint
        xyz = crate::src::renderergl1::tr_shade::tess.xyz[i as usize].as_mut_ptr();
        // identify the two shortest edges
        nums[1 as i32 as usize] = 0 as i32;
        nums[0 as i32 as usize] = nums[1 as i32 as usize];
        lengths[1 as i32 as usize] = 999999 as i32 as f32;
        lengths[0 as i32 as usize] = lengths[1 as i32 as usize];
        j = 0 as i32;
        while j < 6 as i32 {
            let mut l: f32 = 0.;
            let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
            v1 = xyz.offset((4 as i32 * edgeVerts[j as usize][0 as i32 as usize]) as isize);
            v2 = xyz.offset((4 as i32 * edgeVerts[j as usize][1 as i32 as usize]) as isize);
            temp[0 as i32 as usize] = *v1.offset(0 as i32 as isize) - *v2.offset(0 as i32 as isize);
            temp[1 as i32 as usize] = *v1.offset(1 as i32 as isize) - *v2.offset(1 as i32 as isize);
            temp[2 as i32 as usize] = *v1.offset(2 as i32 as isize) - *v2.offset(2 as i32 as isize);
            l = temp[0 as i32 as usize] * temp[0 as i32 as usize]
                + temp[1 as i32 as usize] * temp[1 as i32 as usize]
                + temp[2 as i32 as usize] * temp[2 as i32 as usize];
            if l < lengths[0 as i32 as usize] {
                nums[1 as i32 as usize] = nums[0 as i32 as usize];
                lengths[1 as i32 as usize] = lengths[0 as i32 as usize];
                nums[0 as i32 as usize] = j;
                lengths[0 as i32 as usize] = l
            } else if l < lengths[1 as i32 as usize] {
                nums[1 as i32 as usize] = j;
                lengths[1 as i32 as usize] = l
            }
            j += 1
        }
        j = 0 as i32;
        while j < 2 as i32 {
            v1 = xyz.offset(
                (4 as i32 * edgeVerts[nums[j as usize] as usize][0 as i32 as usize]) as isize,
            );
            v2 = xyz.offset(
                (4 as i32 * edgeVerts[nums[j as usize] as usize][1 as i32 as usize]) as isize,
            );
            mid[j as usize][0 as i32 as usize] =
                0.5f32 * (*v1.offset(0 as i32 as isize) + *v2.offset(0 as i32 as isize));
            mid[j as usize][1 as i32 as usize] =
                0.5f32 * (*v1.offset(1 as i32 as isize) + *v2.offset(1 as i32 as isize));
            mid[j as usize][2 as i32 as usize] =
                0.5f32 * (*v1.offset(2 as i32 as isize) + *v2.offset(2 as i32 as isize));
            j += 1
        }
        // find the vector of the major axis
        major[0 as i32 as usize] =
            mid[1 as i32 as usize][0 as i32 as usize] - mid[0 as i32 as usize][0 as i32 as usize];
        major[1 as i32 as usize] =
            mid[1 as i32 as usize][1 as i32 as usize] - mid[0 as i32 as usize][1 as i32 as usize];
        major[2 as i32 as usize] =
            mid[1 as i32 as usize][2 as i32 as usize] - mid[0 as i32 as usize][2 as i32 as usize];
        // cross this with the view direction to get minor axis
        CrossProduct(
            major.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            forward.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            minor.as_mut_ptr(),
        );
        crate::src::qcommon::q_math::VectorNormalize(minor.as_mut_ptr());
        // re-project the points
        j = 0 as i32;
        while j < 2 as i32 {
            let mut l_0: f32 = 0.;
            v1 = xyz.offset(
                (4 as i32 * edgeVerts[nums[j as usize] as usize][0 as i32 as usize]) as isize,
            );
            v2 = xyz.offset(
                (4 as i32 * edgeVerts[nums[j as usize] as usize][1 as i32 as usize]) as isize,
            );
            l_0 = (0.5f64 * crate::stdlib::sqrt(lengths[j as usize] as f64)) as f32;
            // we need to see which direction this edge
            // is used to determine direction of projection
            k = 0 as i32;
            while k < 5 as i32 {
                if crate::src::renderergl1::tr_shade::tess.indexes[(indexes + k) as usize]
                    == (i + edgeVerts[nums[j as usize] as usize][0 as i32 as usize]) as u32
                    && crate::src::renderergl1::tr_shade::tess.indexes
                        [(indexes + k + 1 as i32) as usize]
                        == (i + edgeVerts[nums[j as usize] as usize][1 as i32 as usize]) as u32
                {
                    break;
                }
                k += 1
            }
            if k == 5 as i32 {
                *v1.offset(0 as i32 as isize) =
                    mid[j as usize][0 as i32 as usize] + minor[0 as i32 as usize] * l_0;
                *v1.offset(1 as i32 as isize) =
                    mid[j as usize][1 as i32 as usize] + minor[1 as i32 as usize] * l_0;
                *v1.offset(2 as i32 as isize) =
                    mid[j as usize][2 as i32 as usize] + minor[2 as i32 as usize] * l_0;
                *v2.offset(0 as i32 as isize) =
                    mid[j as usize][0 as i32 as usize] + minor[0 as i32 as usize] * -l_0;
                *v2.offset(1 as i32 as isize) =
                    mid[j as usize][1 as i32 as usize] + minor[1 as i32 as usize] * -l_0;
                *v2.offset(2 as i32 as isize) =
                    mid[j as usize][2 as i32 as usize] + minor[2 as i32 as usize] * -l_0
            } else {
                *v1.offset(0 as i32 as isize) =
                    mid[j as usize][0 as i32 as usize] + minor[0 as i32 as usize] * -l_0;
                *v1.offset(1 as i32 as isize) =
                    mid[j as usize][1 as i32 as usize] + minor[1 as i32 as usize] * -l_0;
                *v1.offset(2 as i32 as isize) =
                    mid[j as usize][2 as i32 as usize] + minor[2 as i32 as usize] * -l_0;
                *v2.offset(0 as i32 as isize) =
                    mid[j as usize][0 as i32 as usize] + minor[0 as i32 as usize] * l_0;
                *v2.offset(1 as i32 as isize) =
                    mid[j as usize][1 as i32 as usize] + minor[1 as i32 as usize] * l_0;
                *v2.offset(2 as i32 as isize) =
                    mid[j as usize][2 as i32 as usize] + minor[2 as i32 as usize] * l_0
            }
            j += 1
        }
        i += 4 as i32;
        indexes += 6 as i32
    }
}
/*
=====================
RB_DeformTessGeometry

=====================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_DeformTessGeometry() {
    let mut i: i32 = 0;
    let mut ds: *mut crate::tr_local_h::deformStage_t = 0 as *mut crate::tr_local_h::deformStage_t;
    i = 0 as i32;
    while i < (*crate::src::renderergl1::tr_shade::tess.shader).numDeforms {
        ds = &mut *(*crate::src::renderergl1::tr_shade::tess.shader)
            .deforms
            .as_mut_ptr()
            .offset(i as isize) as *mut crate::tr_local_h::deformStage_t;
        match (*ds).deformation as u32 {
            2 => {
                RB_CalcDeformNormals(ds);
            }
            1 => {
                RB_CalcDeformVertexes(ds);
            }
            3 => {
                RB_CalcBulgeVertexes(ds);
            }
            4 => {
                RB_CalcMoveVertexes(ds);
            }
            5 => {
                crate::src::renderergl1::tr_shadows::RB_ProjectionShadowDeform();
            }
            6 => {
                AutospriteDeform();
            }
            7 => {
                Autosprite2Deform();
            }
            8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
                DeformText(
                    crate::src::renderergl1::tr_backend::backEnd.refdef.text[((*ds).deformation
                        as u32)
                        .wrapping_sub(crate::tr_local_h::DEFORM_TEXT0 as i32 as u32)
                        as usize]
                        .as_mut_ptr(),
                );
            }
            0 | _ => {}
        }
        i += 1
    }
}
/*
====================================================================

COLORS

====================================================================
*/
/*
** RB_CalcColorFromEntity
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcColorFromEntity(mut dstColors: *mut u8) {
    let mut i: i32 = 0;
    let mut pColors: *mut i32 = dstColors as *mut i32;
    let mut c: i32 = 0;
    if crate::src::renderergl1::tr_backend::backEnd
        .currentEntity
        .is_null()
    {
        return;
    }
    c = *((*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
        .e
        .shaderRGBA
        .as_mut_ptr() as *mut i32);
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        *pColors = c;
        i += 1;
        pColors = pColors.offset(1)
    }
}
/*
** RB_CalcColorFromOneMinusEntity
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcColorFromOneMinusEntity(mut dstColors: *mut u8) {
    let mut i: i32 = 0; // this trashes alpha, but the AGEN block fixes it
    let mut pColors: *mut i32 = dstColors as *mut i32;
    let mut invModulate: [u8; 4] = [0; 4];
    let mut c: i32 = 0;
    if crate::src::renderergl1::tr_backend::backEnd
        .currentEntity
        .is_null()
    {
        return;
    }
    invModulate[0 as i32 as usize] = (255 as i32
        - (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[0 as i32 as usize] as i32) as u8;
    invModulate[1 as i32 as usize] = (255 as i32
        - (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[1 as i32 as usize] as i32) as u8;
    invModulate[2 as i32 as usize] = (255 as i32
        - (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[2 as i32 as usize] as i32) as u8;
    invModulate[3 as i32 as usize] = (255 as i32
        - (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[3 as i32 as usize] as i32) as u8;
    c = *(invModulate.as_mut_ptr() as *mut i32);
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        *pColors = c;
        i += 1;
        pColors = pColors.offset(1)
    }
}
/*
** RB_CalcAlphaFromEntity
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcAlphaFromEntity(mut dstColors: *mut u8) {
    let mut i: i32 = 0;
    if crate::src::renderergl1::tr_backend::backEnd
        .currentEntity
        .is_null()
    {
        return;
    }
    dstColors = dstColors.offset(3 as i32 as isize);
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        *dstColors = (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .shaderRGBA[3 as i32 as usize];
        i += 1;
        dstColors = dstColors.offset(4 as i32 as isize)
    }
}
/*
** RB_CalcAlphaFromOneMinusEntity
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcAlphaFromOneMinusEntity(mut dstColors: *mut u8) {
    let mut i: i32 = 0;
    if crate::src::renderergl1::tr_backend::backEnd
        .currentEntity
        .is_null()
    {
        return;
    }
    dstColors = dstColors.offset(3 as i32 as isize);
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        *dstColors = (0xff as i32
            - (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
                .e
                .shaderRGBA[3 as i32 as usize] as i32) as u8;
        i += 1;
        dstColors = dstColors.offset(4 as i32 as isize)
    }
}
/*
** RB_CalcWaveColor
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcWaveColor(
    mut wf: *const crate::tr_local_h::waveForm_t,
    mut dstColors: *mut u8,
) {
    let mut i: i32 = 0;
    let mut v: i32 = 0;
    let mut glow: f32 = 0.;
    let mut colors: *mut i32 = dstColors as *mut i32;
    let mut color: [crate::src::qcommon::q_shared::byte; 4] = [0; 4];
    if (*wf).func as u32 == crate::tr_local_h::GF_NOISE as i32 as u32 {
        glow = (*wf).base
            + crate::src::renderercommon::tr_noise::R_NoiseGet4f(
                0 as i32 as f32,
                0 as i32 as f32,
                0 as i32 as f32,
                (crate::src::renderergl1::tr_shade::tess.shaderTime + (*wf).phase as f64)
                    * (*wf).frequency as f64,
            ) * (*wf).amplitude
    } else {
        glow = EvalWaveForm(wf) * crate::src::renderergl1::tr_main::tr.identityLight
    }
    if glow < 0 as i32 as f32 {
        glow = 0 as i32 as f32
    } else if glow > 1 as i32 as f32 {
        glow = 1 as i32 as f32
    }
    v = crate::src::renderergl1::tr_main::ri
        .ftol
        .expect("non-null function pointer")(255 as i32 as f32 * glow) as i32;
    color[2 as i32 as usize] = v as crate::src::qcommon::q_shared::byte;
    color[1 as i32 as usize] = color[2 as i32 as usize];
    color[0 as i32 as usize] = color[1 as i32 as usize];
    color[3 as i32 as usize] = 255 as i32 as crate::src::qcommon::q_shared::byte;
    v = *(color.as_mut_ptr() as *mut i32);
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        *colors = v;
        i += 1;
        colors = colors.offset(1)
    }
}
/*
** RB_CalcWaveAlpha
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcWaveAlpha(
    mut wf: *const crate::tr_local_h::waveForm_t,
    mut dstColors: *mut u8,
) {
    let mut i: i32 = 0;
    let mut v: i32 = 0;
    let mut glow: f32 = 0.;
    glow = EvalWaveFormClamped(wf);
    v = (255 as i32 as f32 * glow) as i32;
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        *dstColors.offset(3 as i32 as isize) = v as u8;
        i += 1;
        dstColors = dstColors.offset(4 as i32 as isize)
    }
}
/*
** RB_CalcModulateColorsByFog
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcModulateColorsByFog(mut colors: *mut u8) {
    let mut i: i32 = 0;
    let mut texCoords: [[f32; 2]; 1000] = [[0.; 2]; 1000];
    // calculate texcoords so we can derive density
    // this is not wasted, because it would only have
    // been previously called if the surface was opaque
    RB_CalcFogTexCoords(texCoords[0 as i32 as usize].as_mut_ptr());
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        let mut f: f32 = (1.0f64
            - crate::src::renderergl1::tr_image::R_FogFactor(
                texCoords[i as usize][0 as i32 as usize],
                texCoords[i as usize][1 as i32 as usize],
            ) as f64) as f32;
        let ref mut fresh3 = *colors.offset(0 as i32 as isize);
        *fresh3 = (*fresh3 as f32 * f) as u8;
        let ref mut fresh4 = *colors.offset(1 as i32 as isize);
        *fresh4 = (*fresh4 as f32 * f) as u8;
        let ref mut fresh5 = *colors.offset(2 as i32 as isize);
        *fresh5 = (*fresh5 as f32 * f) as u8;
        i += 1;
        colors = colors.offset(4 as i32 as isize)
    }
}
/*
** RB_CalcModulateAlphasByFog
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcModulateAlphasByFog(mut colors: *mut u8) {
    let mut i: i32 = 0;
    let mut texCoords: [[f32; 2]; 1000] = [[0.; 2]; 1000];
    // calculate texcoords so we can derive density
    // this is not wasted, because it would only have
    // been previously called if the surface was opaque
    RB_CalcFogTexCoords(texCoords[0 as i32 as usize].as_mut_ptr());
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        let mut f: f32 = (1.0f64
            - crate::src::renderergl1::tr_image::R_FogFactor(
                texCoords[i as usize][0 as i32 as usize],
                texCoords[i as usize][1 as i32 as usize],
            ) as f64) as f32;
        let ref mut fresh6 = *colors.offset(3 as i32 as isize);
        *fresh6 = (*fresh6 as f32 * f) as u8;
        i += 1;
        colors = colors.offset(4 as i32 as isize)
    }
}
/*
** RB_CalcModulateRGBAsByFog
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcModulateRGBAsByFog(mut colors: *mut u8) {
    let mut i: i32 = 0;
    let mut texCoords: [[f32; 2]; 1000] = [
        [0.0f32, 0.],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
    ];
    // calculate texcoords so we can derive density
    // this is not wasted, because it would only have
    // been previously called if the surface was opaque
    RB_CalcFogTexCoords(texCoords[0 as i32 as usize].as_mut_ptr());
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        let mut f: f32 = (1.0f64
            - crate::src::renderergl1::tr_image::R_FogFactor(
                texCoords[i as usize][0 as i32 as usize],
                texCoords[i as usize][1 as i32 as usize],
            ) as f64) as f32;
        let ref mut fresh7 = *colors.offset(0 as i32 as isize);
        *fresh7 = (*fresh7 as f32 * f) as u8;
        let ref mut fresh8 = *colors.offset(1 as i32 as isize);
        *fresh8 = (*fresh8 as f32 * f) as u8;
        let ref mut fresh9 = *colors.offset(2 as i32 as isize);
        *fresh9 = (*fresh9 as f32 * f) as u8;
        let ref mut fresh10 = *colors.offset(3 as i32 as isize);
        *fresh10 = (*fresh10 as f32 * f) as u8;
        i += 1;
        colors = colors.offset(4 as i32 as isize)
    }
}
/*
====================================================================

TEX COORDS

====================================================================
*/
/*
========================
RB_CalcFogTexCoords

To do the clipped fog plane really correctly, we should use
projected textures, but I don't trust the drivers and it
doesn't fit our shader data.
========================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcFogTexCoords(mut st: *mut f32) {
    let mut i: i32 = 0;
    let mut v: *mut f32 = 0 as *mut f32;
    let mut s: f32 = 0.;
    let mut t: f32 = 0.;
    let mut eyeT: f32 = 0.;
    let mut eyeOutside: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut fog: *mut crate::tr_local_h::fog_t = 0 as *mut crate::tr_local_h::fog_t;
    let mut local: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut fogDistanceVector: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut fogDepthVector: crate::src::qcommon::q_shared::vec4_t = [
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
        0 as i32 as crate::src::qcommon::q_shared::vec_t,
    ];
    fog = (*crate::src::renderergl1::tr_main::tr.world)
        .fogs
        .offset(crate::src::renderergl1::tr_shade::tess.fogNum as isize);
    // all fogging distance is based on world Z units
    local[0 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd.or.origin
        [0 as i32 as usize]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[0 as i32 as usize];
    local[1 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd.or.origin
        [1 as i32 as usize]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[1 as i32 as usize];
    local[2 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd.or.origin
        [2 as i32 as usize]
        - crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[2 as i32 as usize];
    fogDistanceVector[0 as i32 as usize] =
        -crate::src::renderergl1::tr_backend::backEnd.or.modelMatrix[2 as i32 as usize];
    fogDistanceVector[1 as i32 as usize] =
        -crate::src::renderergl1::tr_backend::backEnd.or.modelMatrix[6 as i32 as usize];
    fogDistanceVector[2 as i32 as usize] =
        -crate::src::renderergl1::tr_backend::backEnd.or.modelMatrix[10 as i32 as usize];
    fogDistanceVector[3 as i32 as usize] = local[0 as i32 as usize]
        * crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .axis[0 as i32 as usize][0 as i32 as usize]
        + local[1 as i32 as usize]
            * crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[0 as i32 as usize][1 as i32 as usize]
        + local[2 as i32 as usize]
            * crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .axis[0 as i32 as usize][2 as i32 as usize];
    // scale the fog vectors based on the fog's thickness
    fogDistanceVector[0 as i32 as usize] *= (*fog).tcScale;
    fogDistanceVector[1 as i32 as usize] *= (*fog).tcScale;
    fogDistanceVector[2 as i32 as usize] *= (*fog).tcScale;
    fogDistanceVector[3 as i32 as usize] *= (*fog).tcScale;
    // rotate the gradient vector for this orientation
    if (*fog).hasSurface as u64 != 0 {
        fogDepthVector[0 as i32 as usize] = (*fog).surface[0 as i32 as usize]
            * crate::src::renderergl1::tr_backend::backEnd.or.axis[0 as i32 as usize]
                [0 as i32 as usize]
            + (*fog).surface[1 as i32 as usize]
                * crate::src::renderergl1::tr_backend::backEnd.or.axis[0 as i32 as usize]
                    [1 as i32 as usize]
            + (*fog).surface[2 as i32 as usize]
                * crate::src::renderergl1::tr_backend::backEnd.or.axis[0 as i32 as usize]
                    [2 as i32 as usize];
        fogDepthVector[1 as i32 as usize] = (*fog).surface[0 as i32 as usize]
            * crate::src::renderergl1::tr_backend::backEnd.or.axis[1 as i32 as usize]
                [0 as i32 as usize]
            + (*fog).surface[1 as i32 as usize]
                * crate::src::renderergl1::tr_backend::backEnd.or.axis[1 as i32 as usize]
                    [1 as i32 as usize]
            + (*fog).surface[2 as i32 as usize]
                * crate::src::renderergl1::tr_backend::backEnd.or.axis[1 as i32 as usize]
                    [2 as i32 as usize];
        fogDepthVector[2 as i32 as usize] = (*fog).surface[0 as i32 as usize]
            * crate::src::renderergl1::tr_backend::backEnd.or.axis[2 as i32 as usize]
                [0 as i32 as usize]
            + (*fog).surface[1 as i32 as usize]
                * crate::src::renderergl1::tr_backend::backEnd.or.axis[2 as i32 as usize]
                    [1 as i32 as usize]
            + (*fog).surface[2 as i32 as usize]
                * crate::src::renderergl1::tr_backend::backEnd.or.axis[2 as i32 as usize]
                    [2 as i32 as usize];
        fogDepthVector[3 as i32 as usize] = -(*fog).surface[3 as i32 as usize]
            + (crate::src::renderergl1::tr_backend::backEnd.or.origin[0 as i32 as usize]
                * (*fog).surface[0 as i32 as usize]
                + crate::src::renderergl1::tr_backend::backEnd.or.origin[1 as i32 as usize]
                    * (*fog).surface[1 as i32 as usize]
                + crate::src::renderergl1::tr_backend::backEnd.or.origin[2 as i32 as usize]
                    * (*fog).surface[2 as i32 as usize]);
        eyeT = crate::src::renderergl1::tr_backend::backEnd.or.viewOrigin[0 as i32 as usize]
            * fogDepthVector[0 as i32 as usize]
            + crate::src::renderergl1::tr_backend::backEnd.or.viewOrigin[1 as i32 as usize]
                * fogDepthVector[1 as i32 as usize]
            + crate::src::renderergl1::tr_backend::backEnd.or.viewOrigin[2 as i32 as usize]
                * fogDepthVector[2 as i32 as usize]
            + fogDepthVector[3 as i32 as usize]
    } else {
        eyeT = 1 as i32 as f32
        // non-surface fog always has eye inside
    }
    // see if the viewpoint is outside
    // this is needed for clipping distance even for constant fog
    if eyeT < 0 as i32 as f32 {
        eyeOutside = crate::src::qcommon::q_shared::qtrue
    } else {
        eyeOutside = crate::src::qcommon::q_shared::qfalse
    }
    fogDistanceVector[3 as i32 as usize] = (fogDistanceVector[3 as i32 as usize] as f64
        + 1.0f64 / 512 as i32 as f64)
        as crate::src::qcommon::q_shared::vec_t;
    // calculate density for each point
    i = 0 as i32;
    v = crate::src::renderergl1::tr_shade::tess.xyz[0 as i32 as usize].as_mut_ptr();
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        // calculate the length in fog
        s = *v.offset(0 as i32 as isize) * fogDistanceVector[0 as i32 as usize]
            + *v.offset(1 as i32 as isize) * fogDistanceVector[1 as i32 as usize]
            + *v.offset(2 as i32 as isize) * fogDistanceVector[2 as i32 as usize]
            + fogDistanceVector[3 as i32 as usize];
        t = *v.offset(0 as i32 as isize) * fogDepthVector[0 as i32 as usize]
            + *v.offset(1 as i32 as isize) * fogDepthVector[1 as i32 as usize]
            + *v.offset(2 as i32 as isize) * fogDepthVector[2 as i32 as usize]
            + fogDepthVector[3 as i32 as usize];
        // partially clipped fogs use the T axis
        if eyeOutside as u64 != 0 {
            if (t as f64) < 1.0f64 {
                t = (1.0f64 / 32 as i32 as f64) as f32
            // point is outside, so no fogging
            } else {
                t = (1.0f64 / 32 as i32 as f64
                    + 30.0f64 / 32 as i32 as f64 * t as f64 / (t - eyeT) as f64)
                    as f32
                // cut the distance at the fog plane
            }
        } else if t < 0 as i32 as f32 {
            t = (1.0f64 / 32 as i32 as f64) as f32
        // point is outside, so no fogging
        } else {
            t = (31.0f64 / 32 as i32 as f64) as f32
        }
        *st.offset(0 as i32 as isize) = s;
        *st.offset(1 as i32 as isize) = t;
        st = st.offset(2 as i32 as isize);
        i += 1;
        v = v.offset(4 as i32 as isize)
    }
}
/*
** RB_CalcEnvironmentTexCoords
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcEnvironmentTexCoords(mut st: *mut f32) {
    let mut i: i32 = 0;
    let mut v: *mut f32 = 0 as *mut f32;
    let mut normal: *mut f32 = 0 as *mut f32;
    let mut viewer: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut reflected: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d: f32 = 0.;
    v = crate::src::renderergl1::tr_shade::tess.xyz[0 as i32 as usize].as_mut_ptr();
    normal = crate::src::renderergl1::tr_shade::tess.normal[0 as i32 as usize].as_mut_ptr();
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        viewer[0 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd.or.viewOrigin
            [0 as i32 as usize]
            - *v.offset(0 as i32 as isize);
        viewer[1 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd.or.viewOrigin
            [1 as i32 as usize]
            - *v.offset(1 as i32 as isize);
        viewer[2 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd.or.viewOrigin
            [2 as i32 as usize]
            - *v.offset(2 as i32 as isize);
        VectorNormalizeFast(viewer.as_mut_ptr());
        d = *normal.offset(0 as i32 as isize) * viewer[0 as i32 as usize]
            + *normal.offset(1 as i32 as isize) * viewer[1 as i32 as usize]
            + *normal.offset(2 as i32 as isize) * viewer[2 as i32 as usize];
        reflected[0 as i32 as usize] =
            *normal.offset(0 as i32 as isize) * 2 as i32 as f32 * d - viewer[0 as i32 as usize];
        reflected[1 as i32 as usize] =
            *normal.offset(1 as i32 as isize) * 2 as i32 as f32 * d - viewer[1 as i32 as usize];
        reflected[2 as i32 as usize] =
            *normal.offset(2 as i32 as isize) * 2 as i32 as f32 * d - viewer[2 as i32 as usize];
        *st.offset(0 as i32 as isize) =
            (0.5f64 + reflected[1 as i32 as usize] as f64 * 0.5f64) as f32;
        *st.offset(1 as i32 as isize) =
            (0.5f64 - reflected[2 as i32 as usize] as f64 * 0.5f64) as f32;
        i += 1;
        v = v.offset(4 as i32 as isize);
        normal = normal.offset(4 as i32 as isize);
        st = st.offset(2 as i32 as isize)
    }
}
/*
** RB_CalcTurbulentTexCoords
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcTurbulentTexCoords(
    mut wf: *const crate::tr_local_h::waveForm_t,
    mut st: *mut f32,
) {
    let mut i: i32 = 0;
    let mut now: f64 = 0.;
    now = (*wf).phase as f64
        + crate::src::renderergl1::tr_shade::tess.shaderTime * (*wf).frequency as f64;
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        let mut s: f32 = *st.offset(0 as i32 as isize);
        let mut t: f32 = *st.offset(1 as i32 as isize);
        *st.offset(0 as i32 as isize) = s + crate::src::renderergl1::tr_main::tr.sinTable
            [((((crate::src::renderergl1::tr_shade::tess.xyz[i as usize][0 as i32 as usize]
                + crate::src::renderergl1::tr_shade::tess.xyz[i as usize][2 as i32 as usize])
                as f64
                * 1.0f64
                / 128 as i32 as f64
                * 0.125f64
                + now)
                * 1024 as i32 as f64) as crate::stdlib::int64_t
                & (1024 as i32 - 1 as i32) as isize) as usize]
            * (*wf).amplitude;
        *st.offset(1 as i32 as isize) = t + crate::src::renderergl1::tr_main::tr.sinTable
            [(((crate::src::renderergl1::tr_shade::tess.xyz[i as usize][1 as i32 as usize] as f64
                * 1.0f64
                / 128 as i32 as f64
                * 0.125f64
                + now)
                * 1024 as i32 as f64) as crate::stdlib::int64_t
                & (1024 as i32 - 1 as i32) as isize) as usize]
            * (*wf).amplitude;
        i += 1;
        st = st.offset(2 as i32 as isize)
    }
}
/*
** RB_CalcScaleTexCoords
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcScaleTexCoords(mut scale: *const f32, mut st: *mut f32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        *st.offset(0 as i32 as isize) *= *scale.offset(0 as i32 as isize);
        *st.offset(1 as i32 as isize) *= *scale.offset(1 as i32 as isize);
        i += 1;
        st = st.offset(2 as i32 as isize)
    }
}
/*
** RB_CalcScrollTexCoords
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcScrollTexCoords(mut scrollSpeed: *const f32, mut st: *mut f32) {
    let mut i: i32 = 0;
    let mut timeScale: f64 = crate::src::renderergl1::tr_shade::tess.shaderTime;
    let mut adjustedScrollS: f64 = 0.;
    let mut adjustedScrollT: f64 = 0.;
    adjustedScrollS = *scrollSpeed.offset(0 as i32 as isize) as f64 * timeScale;
    adjustedScrollT = *scrollSpeed.offset(1 as i32 as isize) as f64 * timeScale;
    // clamp so coordinates don't continuously get larger, causing problems
    // with hardware limits
    adjustedScrollS = adjustedScrollS - crate::stdlib::floor(adjustedScrollS);
    adjustedScrollT = adjustedScrollT - crate::stdlib::floor(adjustedScrollT);
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        let ref mut fresh11 = *st.offset(0 as i32 as isize);
        *fresh11 = (*fresh11 as f64 + adjustedScrollS) as f32;
        let ref mut fresh12 = *st.offset(1 as i32 as isize);
        *fresh12 = (*fresh12 as f64 + adjustedScrollT) as f32;
        i += 1;
        st = st.offset(2 as i32 as isize)
    }
}
/*
** RB_CalcTransformTexCoords
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcTransformTexCoords(
    mut tmi: *const crate::tr_local_h::texModInfo_t,
    mut st: *mut f32,
) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_shade::tess.numVertexes {
        let mut s: f32 = *st.offset(0 as i32 as isize);
        let mut t: f32 = *st.offset(1 as i32 as isize);
        *st.offset(0 as i32 as isize) = s * (*tmi).matrix[0 as i32 as usize][0 as i32 as usize]
            + t * (*tmi).matrix[1 as i32 as usize][0 as i32 as usize]
            + (*tmi).translate[0 as i32 as usize];
        *st.offset(1 as i32 as isize) = s * (*tmi).matrix[0 as i32 as usize][1 as i32 as usize]
            + t * (*tmi).matrix[1 as i32 as usize][1 as i32 as usize]
            + (*tmi).translate[1 as i32 as usize];
        i += 1;
        st = st.offset(2 as i32 as isize)
    }
}
/*
** RB_CalcRotateTexCoords
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcRotateTexCoords(mut degsPerSecond: f32, mut st: *mut f32) {
    let mut timeScale: f64 = crate::src::renderergl1::tr_shade::tess.shaderTime;
    let mut degs: f64 = 0.;
    let mut index: crate::stdlib::int64_t = 0;
    let mut sinValue: f32 = 0.;
    let mut cosValue: f32 = 0.;
    let mut tmi: crate::tr_local_h::texModInfo_t = crate::tr_local_h::texModInfo_t {
        type_0: crate::tr_local_h::TMOD_NONE,
        wave: crate::tr_local_h::waveForm_t {
            func: crate::tr_local_h::GF_NONE,
            base: 0.,
            amplitude: 0.,
            phase: 0.,
            frequency: 0.,
        },
        matrix: [[0.; 2]; 2],
        translate: [0.; 2],
        scale: [0.; 2],
        scroll: [0.; 2],
        rotateSpeed: 0.,
    };
    degs = -degsPerSecond as f64 * timeScale;
    index = (degs * (1024 as i32 as f32 / 360.0f32) as f64) as crate::stdlib::int64_t;
    sinValue = crate::src::renderergl1::tr_main::tr.sinTable
        [(index & (1024 as i32 - 1 as i32) as isize) as usize];
    cosValue = crate::src::renderergl1::tr_main::tr.sinTable
        [(index + (1024 as i32 / 4 as i32) as isize & (1024 as i32 - 1 as i32) as isize) as usize];
    tmi.matrix[0 as i32 as usize][0 as i32 as usize] = cosValue;
    tmi.matrix[1 as i32 as usize][0 as i32 as usize] = -sinValue;
    tmi.translate[0 as i32 as usize] =
        (0.5f64 - 0.5f64 * cosValue as f64 + 0.5f64 * sinValue as f64) as f32;
    tmi.matrix[0 as i32 as usize][1 as i32 as usize] = sinValue;
    tmi.matrix[1 as i32 as usize][1 as i32 as usize] = cosValue;
    tmi.translate[1 as i32 as usize] =
        (0.5f64 - 0.5f64 * sinValue as f64 - 0.5f64 * cosValue as f64) as f32;
    RB_CalcTransformTexCoords(&mut tmi, st);
}
/*
** RB_CalcSpecularAlpha
**
** Calculates specular coefficient and places it in the alpha channel
*/
#[no_mangle]

pub static mut lightOrigin: crate::src::qcommon::q_shared::vec3_t = [
    -(960 as i32) as crate::src::qcommon::q_shared::vec_t,
    1980 as i32 as crate::src::qcommon::q_shared::vec_t,
    96 as i32 as crate::src::qcommon::q_shared::vec_t,
];
// FIXME: track dynamically
#[no_mangle]

pub unsafe extern "C" fn RB_CalcSpecularAlpha(mut alphas: *mut u8) {
    let mut i: i32 = 0;
    let mut v: *mut f32 = 0 as *mut f32;
    let mut normal: *mut f32 = 0 as *mut f32;
    let mut viewer: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut reflected: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut l: f32 = 0.;
    let mut d: f32 = 0.;
    let mut b: i32 = 0;
    let mut lightDir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut numVertexes: i32 = 0;
    v = crate::src::renderergl1::tr_shade::tess.xyz[0 as i32 as usize].as_mut_ptr();
    normal = crate::src::renderergl1::tr_shade::tess.normal[0 as i32 as usize].as_mut_ptr();
    alphas = alphas.offset(3 as i32 as isize);
    numVertexes = crate::src::renderergl1::tr_shade::tess.numVertexes;
    i = 0 as i32;
    while i < numVertexes {
        let mut ilength: f32 = 0.;
        lightDir[0 as i32 as usize] = lightOrigin[0 as i32 as usize] - *v.offset(0 as i32 as isize);
        lightDir[1 as i32 as usize] = lightOrigin[1 as i32 as usize] - *v.offset(1 as i32 as isize);
        lightDir[2 as i32 as usize] = lightOrigin[2 as i32 as usize] - *v.offset(2 as i32 as isize);
        //		ilength = Q_rsqrt( DotProduct( lightDir, lightDir ) );
        VectorNormalizeFast(lightDir.as_mut_ptr());
        // calculate the specular color
        d = *normal.offset(0 as i32 as isize) * lightDir[0 as i32 as usize]
            + *normal.offset(1 as i32 as isize) * lightDir[1 as i32 as usize]
            + *normal.offset(2 as i32 as isize) * lightDir[2 as i32 as usize];
        //		d *= ilength;
        // we don't optimize for the d < 0 case since this tends to
        // cause visual artifacts such as faceted "snapping"
        reflected[0 as i32 as usize] =
            *normal.offset(0 as i32 as isize) * 2 as i32 as f32 * d - lightDir[0 as i32 as usize];
        reflected[1 as i32 as usize] =
            *normal.offset(1 as i32 as isize) * 2 as i32 as f32 * d - lightDir[1 as i32 as usize];
        reflected[2 as i32 as usize] =
            *normal.offset(2 as i32 as isize) * 2 as i32 as f32 * d - lightDir[2 as i32 as usize];
        viewer[0 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd.or.viewOrigin
            [0 as i32 as usize]
            - *v.offset(0 as i32 as isize);
        viewer[1 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd.or.viewOrigin
            [1 as i32 as usize]
            - *v.offset(1 as i32 as isize);
        viewer[2 as i32 as usize] = crate::src::renderergl1::tr_backend::backEnd.or.viewOrigin
            [2 as i32 as usize]
            - *v.offset(2 as i32 as isize);
        ilength = crate::src::qcommon::q_math::Q_rsqrt(
            viewer[0 as i32 as usize] * viewer[0 as i32 as usize]
                + viewer[1 as i32 as usize] * viewer[1 as i32 as usize]
                + viewer[2 as i32 as usize] * viewer[2 as i32 as usize],
        );
        l = reflected[0 as i32 as usize] * viewer[0 as i32 as usize]
            + reflected[1 as i32 as usize] * viewer[1 as i32 as usize]
            + reflected[2 as i32 as usize] * viewer[2 as i32 as usize];
        l *= ilength;
        if l < 0 as i32 as f32 {
            b = 0 as i32
        } else {
            l = l * l;
            l = l * l;
            b = (l * 255 as i32 as f32) as i32;
            if b > 255 as i32 {
                b = 255 as i32
            }
        }
        *alphas = b as u8;
        i += 1;
        v = v.offset(4 as i32 as isize);
        normal = normal.offset(4 as i32 as isize);
        alphas = alphas.offset(4 as i32 as isize)
    }
}
/*
** RB_CalcDiffuseColor
**
** The basic vertex lighting calc
*/

unsafe extern "C" fn RB_CalcDiffuseColor_scalar(mut colors: *mut u8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut v: *mut f32 = 0 as *mut f32;
    let mut normal: *mut f32 = 0 as *mut f32;
    let mut incoming: f32 = 0.;
    let mut ent: *mut crate::tr_local_h::trRefEntity_t = 0 as *mut crate::tr_local_h::trRefEntity_t;
    let mut ambientLightInt: i32 = 0;
    let mut ambientLight: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lightDir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut directedLight: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut numVertexes: i32 = 0;
    ent = crate::src::renderergl1::tr_backend::backEnd.currentEntity;
    ambientLightInt = (*ent).ambientLightInt;
    ambientLight[0 as i32 as usize] = (*ent).ambientLight[0 as i32 as usize];
    ambientLight[1 as i32 as usize] = (*ent).ambientLight[1 as i32 as usize];
    ambientLight[2 as i32 as usize] = (*ent).ambientLight[2 as i32 as usize];
    directedLight[0 as i32 as usize] = (*ent).directedLight[0 as i32 as usize];
    directedLight[1 as i32 as usize] = (*ent).directedLight[1 as i32 as usize];
    directedLight[2 as i32 as usize] = (*ent).directedLight[2 as i32 as usize];
    lightDir[0 as i32 as usize] = (*ent).lightDir[0 as i32 as usize];
    lightDir[1 as i32 as usize] = (*ent).lightDir[1 as i32 as usize];
    lightDir[2 as i32 as usize] = (*ent).lightDir[2 as i32 as usize];
    v = crate::src::renderergl1::tr_shade::tess.xyz[0 as i32 as usize].as_mut_ptr();
    normal = crate::src::renderergl1::tr_shade::tess.normal[0 as i32 as usize].as_mut_ptr();
    numVertexes = crate::src::renderergl1::tr_shade::tess.numVertexes;
    i = 0 as i32;
    while i < numVertexes {
        incoming = *normal.offset(0 as i32 as isize) * lightDir[0 as i32 as usize]
            + *normal.offset(1 as i32 as isize) * lightDir[1 as i32 as usize]
            + *normal.offset(2 as i32 as isize) * lightDir[2 as i32 as usize];
        if incoming <= 0 as i32 as f32 {
            *(&mut *colors.offset((i * 4 as i32) as isize) as *mut u8 as *mut i32) = ambientLightInt
        } else {
            j = crate::src::renderergl1::tr_main::ri
                .ftol
                .expect("non-null function pointer")(
                ambientLight[0 as i32 as usize] + incoming * directedLight[0 as i32 as usize],
            ) as i32;
            if j > 255 as i32 {
                j = 255 as i32
            }
            *colors.offset((i * 4 as i32 + 0 as i32) as isize) = j as u8;
            j = crate::src::renderergl1::tr_main::ri
                .ftol
                .expect("non-null function pointer")(
                ambientLight[1 as i32 as usize] + incoming * directedLight[1 as i32 as usize],
            ) as i32;
            if j > 255 as i32 {
                j = 255 as i32
            }
            *colors.offset((i * 4 as i32 + 1 as i32) as isize) = j as u8;
            j = crate::src::renderergl1::tr_main::ri
                .ftol
                .expect("non-null function pointer")(
                ambientLight[2 as i32 as usize] + incoming * directedLight[2 as i32 as usize],
            ) as i32;
            if j > 255 as i32 {
                j = 255 as i32
            }
            *colors.offset((i * 4 as i32 + 2 as i32) as isize) = j as u8;
            *colors.offset((i * 4 as i32 + 3 as i32) as isize) = 255 as i32 as u8
        }
        i += 1;
        v = v.offset(4 as i32 as isize);
        normal = normal.offset(4 as i32 as isize)
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
============================================================

SCENE GENERATION

============================================================
*/
/*
=============================================================

UNCOMPRESSING BONES

=============================================================
*/
/*
=============================================================

ANIMATED MODELS

=============================================================
*/
/*
=============================================================
=============================================================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_CalcDiffuseColor(mut colors: *mut u8) {
    RB_CalcDiffuseColor_scalar(colors);
}

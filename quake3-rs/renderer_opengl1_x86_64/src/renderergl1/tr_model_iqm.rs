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

    // __Q_SHARED_H
}

pub use crate::stddef_h::size_t;

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::AddPointToBounds;
pub use crate::src::qcommon::q_math::AxisClear;
pub use crate::src::qcommon::q_math::ClearBounds;
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
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec2_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_strlwr;
pub use crate::src::qcommon::q_shared::Q_strncmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
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
pub use crate::src::renderergl1::tr_model_iqm::q_shared_h::VectorLength;
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

pub use crate::iqm_h::iqmBounds_t;
pub use crate::iqm_h::iqmHeader_t;
pub use crate::iqm_h::iqmJoint_t;
pub use crate::iqm_h::iqmMesh_t;
pub use crate::iqm_h::iqmPose_t;
pub use crate::iqm_h::iqmTriangle_t;
pub use crate::iqm_h::iqmVertexArray_t;
pub use crate::iqm_h::iqmbounds;
pub use crate::iqm_h::iqmheader;
pub use crate::iqm_h::iqmjoint;
pub use crate::iqm_h::iqmmesh;
pub use crate::iqm_h::iqmpose;
pub use crate::iqm_h::iqmtriangle;
pub use crate::iqm_h::iqmvertexarray;
pub use crate::iqm_h::IQM_BLENDINDEXES;
pub use crate::iqm_h::IQM_BLENDWEIGHTS;
pub use crate::iqm_h::IQM_BYTE;
pub use crate::iqm_h::IQM_COLOR;
pub use crate::iqm_h::IQM_CUSTOM;
pub use crate::iqm_h::IQM_DOUBLE;
pub use crate::iqm_h::IQM_FLOAT;
pub use crate::iqm_h::IQM_HALF;
pub use crate::iqm_h::IQM_INT;
pub use crate::iqm_h::IQM_NORMAL;
pub use crate::iqm_h::IQM_POSITION;
pub use crate::iqm_h::IQM_SHORT;
pub use crate::iqm_h::IQM_TANGENT;
pub use crate::iqm_h::IQM_TEXCOORD;
pub use crate::iqm_h::IQM_UBYTE;
pub use crate::iqm_h::IQM_UINT;
pub use crate::iqm_h::IQM_USHORT;
pub use crate::src::renderergl1::tr_backend::backEnd;
pub use crate::src::renderergl1::tr_image::R_GetSkinByHandle;
pub use crate::src::renderergl1::tr_init::r_shadows;
pub use crate::src::renderergl1::tr_light::R_SetupEntityLighting;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_main::R_AddDrawSurf;
pub use crate::src::renderergl1::tr_main::R_CullLocalBox;
pub use crate::src::renderergl1::tr_shade::tess;
pub use crate::src::renderergl1::tr_shader::R_FindShader;
pub use crate::src::renderergl1::tr_shader::R_GetShaderByHandle;
pub use crate::src::renderergl1::tr_surface::RB_CheckOverflow;

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
pub use crate::tr_local_h::iqmData_t;
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
pub use crate::tr_local_h::srfIQModel_s;
pub use crate::tr_local_h::srfIQModel_t;
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
pub use crate::tr_local_h::C2RustUnnamed_119;
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
pub use crate::tr_local_h::SS_ALMOST_NEAREST;
pub use crate::tr_local_h::SS_BAD;
pub use crate::tr_local_h::SS_BANNER;
pub use crate::tr_local_h::SS_BLEND0;
pub use crate::tr_local_h::SS_BLEND1;
pub use crate::tr_local_h::SS_BLEND2;
pub use crate::tr_local_h::SS_BLEND3;
pub use crate::tr_local_h::SS_BLEND6;
pub use crate::tr_local_h::SS_DECAL;
pub use crate::tr_local_h::SS_ENVIRONMENT;
pub use crate::tr_local_h::SS_FOG;
pub use crate::tr_local_h::SS_NEAREST;
pub use crate::tr_local_h::SS_OPAQUE;
pub use crate::tr_local_h::SS_PORTAL;
pub use crate::tr_local_h::SS_SEE_THROUGH;
pub use crate::tr_local_h::SS_STENCIL_SHADOW;
pub use crate::tr_local_h::SS_UNDERWATER;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_128 {
    pub b: *mut crate::src::qcommon::q_shared::byte,
    pub f: *mut f32,
}
/*
===========================================================================
Copyright (C) 2011 Thilo Schulz <thilo@tjps.eu>
Copyright (C) 2011 Matthias Bentrup <matthias.bentrup@googlemail.com>

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
// 3x4 identity matrix

static mut identityMatrix: [f32; 12] = [
    1 as i32 as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    1 as i32 as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    1 as i32 as f32,
    0 as i32 as f32,
];

unsafe extern "C" fn IQM_CheckRange(
    mut header: *mut crate::iqm_h::iqmHeader_t,
    mut offset: i32,
    mut count: i32,
    mut size: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    // return true if the range specified by offset, count and size
    // doesn't fit into the file
    return (count <= 0 as i32
        || offset <= 0 as i32
        || offset as u32 > (*header).filesize
        || offset + count * size < 0 as i32
        || (offset + count * size) as u32 > (*header).filesize) as i32
        as crate::src::qcommon::q_shared::qboolean;
}
// "multiply" 3x4 matrices, these are assumed to be the top 3 rows
// of a 4x4 matrix with the last row = (0 0 0 1)

unsafe extern "C" fn Matrix34Multiply(mut a: *mut f32, mut b: *mut f32, mut out: *mut f32) {
    *out.offset(0 as i32 as isize) = *a.offset(0 as i32 as isize) * *b.offset(0 as i32 as isize)
        + *a.offset(1 as i32 as isize) * *b.offset(4 as i32 as isize)
        + *a.offset(2 as i32 as isize) * *b.offset(8 as i32 as isize);
    *out.offset(1 as i32 as isize) = *a.offset(0 as i32 as isize) * *b.offset(1 as i32 as isize)
        + *a.offset(1 as i32 as isize) * *b.offset(5 as i32 as isize)
        + *a.offset(2 as i32 as isize) * *b.offset(9 as i32 as isize);
    *out.offset(2 as i32 as isize) = *a.offset(0 as i32 as isize) * *b.offset(2 as i32 as isize)
        + *a.offset(1 as i32 as isize) * *b.offset(6 as i32 as isize)
        + *a.offset(2 as i32 as isize) * *b.offset(10 as i32 as isize);
    *out.offset(3 as i32 as isize) = *a.offset(0 as i32 as isize) * *b.offset(3 as i32 as isize)
        + *a.offset(1 as i32 as isize) * *b.offset(7 as i32 as isize)
        + *a.offset(2 as i32 as isize) * *b.offset(11 as i32 as isize)
        + *a.offset(3 as i32 as isize);
    *out.offset(4 as i32 as isize) = *a.offset(4 as i32 as isize) * *b.offset(0 as i32 as isize)
        + *a.offset(5 as i32 as isize) * *b.offset(4 as i32 as isize)
        + *a.offset(6 as i32 as isize) * *b.offset(8 as i32 as isize);
    *out.offset(5 as i32 as isize) = *a.offset(4 as i32 as isize) * *b.offset(1 as i32 as isize)
        + *a.offset(5 as i32 as isize) * *b.offset(5 as i32 as isize)
        + *a.offset(6 as i32 as isize) * *b.offset(9 as i32 as isize);
    *out.offset(6 as i32 as isize) = *a.offset(4 as i32 as isize) * *b.offset(2 as i32 as isize)
        + *a.offset(5 as i32 as isize) * *b.offset(6 as i32 as isize)
        + *a.offset(6 as i32 as isize) * *b.offset(10 as i32 as isize);
    *out.offset(7 as i32 as isize) = *a.offset(4 as i32 as isize) * *b.offset(3 as i32 as isize)
        + *a.offset(5 as i32 as isize) * *b.offset(7 as i32 as isize)
        + *a.offset(6 as i32 as isize) * *b.offset(11 as i32 as isize)
        + *a.offset(7 as i32 as isize);
    *out.offset(8 as i32 as isize) = *a.offset(8 as i32 as isize) * *b.offset(0 as i32 as isize)
        + *a.offset(9 as i32 as isize) * *b.offset(4 as i32 as isize)
        + *a.offset(10 as i32 as isize) * *b.offset(8 as i32 as isize);
    *out.offset(9 as i32 as isize) = *a.offset(8 as i32 as isize) * *b.offset(1 as i32 as isize)
        + *a.offset(9 as i32 as isize) * *b.offset(5 as i32 as isize)
        + *a.offset(10 as i32 as isize) * *b.offset(9 as i32 as isize);
    *out.offset(10 as i32 as isize) = *a.offset(8 as i32 as isize) * *b.offset(2 as i32 as isize)
        + *a.offset(9 as i32 as isize) * *b.offset(6 as i32 as isize)
        + *a.offset(10 as i32 as isize) * *b.offset(10 as i32 as isize);
    *out.offset(11 as i32 as isize) = *a.offset(8 as i32 as isize) * *b.offset(3 as i32 as isize)
        + *a.offset(9 as i32 as isize) * *b.offset(7 as i32 as isize)
        + *a.offset(10 as i32 as isize) * *b.offset(11 as i32 as isize)
        + *a.offset(11 as i32 as isize);
}

unsafe extern "C" fn InterpolateMatrix(
    mut a: *mut f32,
    mut b: *mut f32,
    mut lerp: f32,
    mut mat: *mut f32,
) {
    let mut unLerp: f32 = 1.0f32 - lerp;
    *mat.offset(0 as i32 as isize) =
        *a.offset(0 as i32 as isize) * unLerp + *b.offset(0 as i32 as isize) * lerp;
    *mat.offset(1 as i32 as isize) =
        *a.offset(1 as i32 as isize) * unLerp + *b.offset(1 as i32 as isize) * lerp;
    *mat.offset(2 as i32 as isize) =
        *a.offset(2 as i32 as isize) * unLerp + *b.offset(2 as i32 as isize) * lerp;
    *mat.offset(3 as i32 as isize) =
        *a.offset(3 as i32 as isize) * unLerp + *b.offset(3 as i32 as isize) * lerp;
    *mat.offset(4 as i32 as isize) =
        *a.offset(4 as i32 as isize) * unLerp + *b.offset(4 as i32 as isize) * lerp;
    *mat.offset(5 as i32 as isize) =
        *a.offset(5 as i32 as isize) * unLerp + *b.offset(5 as i32 as isize) * lerp;
    *mat.offset(6 as i32 as isize) =
        *a.offset(6 as i32 as isize) * unLerp + *b.offset(6 as i32 as isize) * lerp;
    *mat.offset(7 as i32 as isize) =
        *a.offset(7 as i32 as isize) * unLerp + *b.offset(7 as i32 as isize) * lerp;
    *mat.offset(8 as i32 as isize) =
        *a.offset(8 as i32 as isize) * unLerp + *b.offset(8 as i32 as isize) * lerp;
    *mat.offset(9 as i32 as isize) =
        *a.offset(9 as i32 as isize) * unLerp + *b.offset(9 as i32 as isize) * lerp;
    *mat.offset(10 as i32 as isize) =
        *a.offset(10 as i32 as isize) * unLerp + *b.offset(10 as i32 as isize) * lerp;
    *mat.offset(11 as i32 as isize) =
        *a.offset(11 as i32 as isize) * unLerp + *b.offset(11 as i32 as isize) * lerp;
}

unsafe extern "C" fn JointToMatrix(
    mut rot: *mut crate::src::qcommon::q_shared::vec_t,
    mut scale: *mut crate::src::qcommon::q_shared::vec_t,
    mut trans: *mut crate::src::qcommon::q_shared::vec_t,
    mut mat: *mut f32,
) {
    let mut xx: f32 = 2.0f32 * *rot.offset(0 as i32 as isize) * *rot.offset(0 as i32 as isize);
    let mut yy: f32 = 2.0f32 * *rot.offset(1 as i32 as isize) * *rot.offset(1 as i32 as isize);
    let mut zz: f32 = 2.0f32 * *rot.offset(2 as i32 as isize) * *rot.offset(2 as i32 as isize);
    let mut xy: f32 = 2.0f32 * *rot.offset(0 as i32 as isize) * *rot.offset(1 as i32 as isize);
    let mut xz: f32 = 2.0f32 * *rot.offset(0 as i32 as isize) * *rot.offset(2 as i32 as isize);
    let mut yz: f32 = 2.0f32 * *rot.offset(1 as i32 as isize) * *rot.offset(2 as i32 as isize);
    let mut wx: f32 = 2.0f32 * *rot.offset(3 as i32 as isize) * *rot.offset(0 as i32 as isize);
    let mut wy: f32 = 2.0f32 * *rot.offset(3 as i32 as isize) * *rot.offset(1 as i32 as isize);
    let mut wz: f32 = 2.0f32 * *rot.offset(3 as i32 as isize) * *rot.offset(2 as i32 as isize);
    *mat.offset(0 as i32 as isize) = *scale.offset(0 as i32 as isize) * (1.0f32 - (yy + zz));
    *mat.offset(1 as i32 as isize) = *scale.offset(0 as i32 as isize) * (xy - wz);
    *mat.offset(2 as i32 as isize) = *scale.offset(0 as i32 as isize) * (xz + wy);
    *mat.offset(3 as i32 as isize) = *trans.offset(0 as i32 as isize);
    *mat.offset(4 as i32 as isize) = *scale.offset(1 as i32 as isize) * (xy + wz);
    *mat.offset(5 as i32 as isize) = *scale.offset(1 as i32 as isize) * (1.0f32 - (xx + zz));
    *mat.offset(6 as i32 as isize) = *scale.offset(1 as i32 as isize) * (yz - wx);
    *mat.offset(7 as i32 as isize) = *trans.offset(1 as i32 as isize);
    *mat.offset(8 as i32 as isize) = *scale.offset(2 as i32 as isize) * (xz - wy);
    *mat.offset(9 as i32 as isize) = *scale.offset(2 as i32 as isize) * (yz + wx);
    *mat.offset(10 as i32 as isize) = *scale.offset(2 as i32 as isize) * (1.0f32 - (xx + yy));
    *mat.offset(11 as i32 as isize) = *trans.offset(2 as i32 as isize);
}

unsafe extern "C" fn Matrix34Invert(mut inMat: *mut f32, mut outMat: *mut f32) {
    let mut trans: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut invSqrLen: f32 = 0.;
    let mut v: *mut f32 = 0 as *mut f32;
    *outMat.offset(0 as i32 as isize) = *inMat.offset(0 as i32 as isize);
    *outMat.offset(1 as i32 as isize) = *inMat.offset(4 as i32 as isize);
    *outMat.offset(2 as i32 as isize) = *inMat.offset(8 as i32 as isize);
    *outMat.offset(4 as i32 as isize) = *inMat.offset(1 as i32 as isize);
    *outMat.offset(5 as i32 as isize) = *inMat.offset(5 as i32 as isize);
    *outMat.offset(6 as i32 as isize) = *inMat.offset(9 as i32 as isize);
    *outMat.offset(8 as i32 as isize) = *inMat.offset(2 as i32 as isize);
    *outMat.offset(9 as i32 as isize) = *inMat.offset(6 as i32 as isize);
    *outMat.offset(10 as i32 as isize) = *inMat.offset(10 as i32 as isize);
    v = outMat.offset(0 as i32 as isize);
    invSqrLen = 1.0f32
        / (*v.offset(0 as i32 as isize) * *v.offset(0 as i32 as isize)
            + *v.offset(1 as i32 as isize) * *v.offset(1 as i32 as isize)
            + *v.offset(2 as i32 as isize) * *v.offset(2 as i32 as isize));
    *v.offset(0 as i32 as isize) = *v.offset(0 as i32 as isize) * invSqrLen;
    *v.offset(1 as i32 as isize) = *v.offset(1 as i32 as isize) * invSqrLen;
    *v.offset(2 as i32 as isize) = *v.offset(2 as i32 as isize) * invSqrLen;
    v = outMat.offset(4 as i32 as isize);
    invSqrLen = 1.0f32
        / (*v.offset(0 as i32 as isize) * *v.offset(0 as i32 as isize)
            + *v.offset(1 as i32 as isize) * *v.offset(1 as i32 as isize)
            + *v.offset(2 as i32 as isize) * *v.offset(2 as i32 as isize));
    *v.offset(0 as i32 as isize) = *v.offset(0 as i32 as isize) * invSqrLen;
    *v.offset(1 as i32 as isize) = *v.offset(1 as i32 as isize) * invSqrLen;
    *v.offset(2 as i32 as isize) = *v.offset(2 as i32 as isize) * invSqrLen;
    v = outMat.offset(8 as i32 as isize);
    invSqrLen = 1.0f32
        / (*v.offset(0 as i32 as isize) * *v.offset(0 as i32 as isize)
            + *v.offset(1 as i32 as isize) * *v.offset(1 as i32 as isize)
            + *v.offset(2 as i32 as isize) * *v.offset(2 as i32 as isize));
    *v.offset(0 as i32 as isize) = *v.offset(0 as i32 as isize) * invSqrLen;
    *v.offset(1 as i32 as isize) = *v.offset(1 as i32 as isize) * invSqrLen;
    *v.offset(2 as i32 as isize) = *v.offset(2 as i32 as isize) * invSqrLen;
    trans[0 as i32 as usize] = *inMat.offset(3 as i32 as isize);
    trans[1 as i32 as usize] = *inMat.offset(7 as i32 as isize);
    trans[2 as i32 as usize] = *inMat.offset(11 as i32 as isize);
    *outMat.offset(3 as i32 as isize) = -(*outMat
        .offset(0 as i32 as isize)
        .offset(0 as i32 as isize)
        * trans[0 as i32 as usize]
        + *outMat.offset(0 as i32 as isize).offset(1 as i32 as isize) * trans[1 as i32 as usize]
        + *outMat.offset(0 as i32 as isize).offset(2 as i32 as isize) * trans[2 as i32 as usize]);
    *outMat.offset(7 as i32 as isize) = -(*outMat
        .offset(4 as i32 as isize)
        .offset(0 as i32 as isize)
        * trans[0 as i32 as usize]
        + *outMat.offset(4 as i32 as isize).offset(1 as i32 as isize) * trans[1 as i32 as usize]
        + *outMat.offset(4 as i32 as isize).offset(2 as i32 as isize) * trans[2 as i32 as usize]);
    *outMat.offset(11 as i32 as isize) = -(*outMat
        .offset(8 as i32 as isize)
        .offset(0 as i32 as isize)
        * trans[0 as i32 as usize]
        + *outMat.offset(8 as i32 as isize).offset(1 as i32 as isize) * trans[1 as i32 as usize]
        + *outMat.offset(8 as i32 as isize).offset(2 as i32 as isize) * trans[2 as i32 as usize]);
}
/*
=================
R_LoadIQM

Load an IQM model and compute the joint matrices for every frame.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LoadIQM(
    mut mod_0: *mut crate::tr_local_h::model_t,
    mut buffer: *mut libc::c_void,
    mut filesize: i32,
    mut mod_name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut header: *mut crate::iqm_h::iqmHeader_t = 0 as *mut crate::iqm_h::iqmHeader_t;
    let mut vertexarray: *mut crate::iqm_h::iqmVertexArray_t =
        0 as *mut crate::iqm_h::iqmVertexArray_t;
    let mut triangle: *mut crate::iqm_h::iqmTriangle_t = 0 as *mut crate::iqm_h::iqmTriangle_t;
    let mut mesh: *mut crate::iqm_h::iqmMesh_t = 0 as *mut crate::iqm_h::iqmMesh_t;
    let mut joint: *mut crate::iqm_h::iqmJoint_t = 0 as *mut crate::iqm_h::iqmJoint_t;
    let mut pose: *mut crate::iqm_h::iqmPose_t = 0 as *mut crate::iqm_h::iqmPose_t;
    let mut bounds: *mut crate::iqm_h::iqmBounds_t = 0 as *mut crate::iqm_h::iqmBounds_t;
    let mut framedata: *mut u16 = 0 as *mut u16;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut jointInvMats: [f32; 1536] = [
        0.0f32, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    ];
    let mut mat: *mut f32 = 0 as *mut f32;
    let mut matInv: *mut f32 = 0 as *mut f32;
    let mut size: crate::stddef_h::size_t = 0;
    let mut joint_names: crate::stddef_h::size_t = 0;
    let mut dataPtr: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut iqmData: *mut crate::tr_local_h::iqmData_t = 0 as *mut crate::tr_local_h::iqmData_t;
    let mut surface: *mut crate::tr_local_h::srfIQModel_t =
        0 as *mut crate::tr_local_h::srfIQModel_t;
    let mut meshName: [libc::c_char; 64] = [0; 64];
    let mut vertexArrayFormat: [i32; 7] = [0; 7];
    let mut allocateInfluences: i32 = 0;
    let mut blendIndexes: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut blendWeights: C2RustUnnamed_128 = C2RustUnnamed_128 {
        b: 0 as *mut crate::src::qcommon::q_shared::byte,
    };
    if (filesize as libc::c_ulong)
        < ::std::mem::size_of::<crate::iqm_h::iqmHeader_t>() as libc::c_ulong
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    header = buffer as *mut crate::iqm_h::iqmHeader_t;
    if crate::src::qcommon::q_shared::Q_strncmp(
        (*header).magic.as_mut_ptr(),
        b"INTERQUAKEMODEL\x00" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as i32,
    ) != 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    (*header).version = (*header).version;
    if (*header).version != 2 as i32 as u32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_LoadIQM: %s is a unsupported IQM version (%d), only version %d is supported.\n\x00"
                as *const u8 as *const libc::c_char,
            mod_name,
            (*header).version,
            2 as i32,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    (*header).filesize = (*header).filesize;
    if (*header).filesize > filesize as u32
        || (*header).filesize > ((16 as i32) << 20 as i32) as u32
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    (*header).flags = (*header).flags;
    (*header).num_text = (*header).num_text;
    (*header).ofs_text = (*header).ofs_text;
    (*header).num_meshes = (*header).num_meshes;
    (*header).ofs_meshes = (*header).ofs_meshes;
    (*header).num_vertexarrays = (*header).num_vertexarrays;
    (*header).num_vertexes = (*header).num_vertexes;
    (*header).ofs_vertexarrays = (*header).ofs_vertexarrays;
    (*header).num_triangles = (*header).num_triangles;
    (*header).ofs_triangles = (*header).ofs_triangles;
    (*header).ofs_adjacency = (*header).ofs_adjacency;
    (*header).num_joints = (*header).num_joints;
    (*header).ofs_joints = (*header).ofs_joints;
    (*header).num_poses = (*header).num_poses;
    (*header).ofs_poses = (*header).ofs_poses;
    (*header).num_anims = (*header).num_anims;
    (*header).ofs_anims = (*header).ofs_anims;
    (*header).num_frames = (*header).num_frames;
    (*header).num_framechannels = (*header).num_framechannels;
    (*header).ofs_frames = (*header).ofs_frames;
    (*header).ofs_bounds = (*header).ofs_bounds;
    (*header).num_comment = (*header).num_comment;
    (*header).ofs_comment = (*header).ofs_comment;
    (*header).num_extensions = (*header).num_extensions;
    (*header).ofs_extensions = (*header).ofs_extensions;
    // check ioq3 joint limit
    if (*header).num_joints > 128 as i32 as u32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_LoadIQM: %s has more than %d joints (%d).\n\x00" as *const u8
                as *const libc::c_char,
            mod_name,
            128 as i32,
            (*header).num_joints,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    i = 0 as i32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[i32; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<i32>() as libc::c_ulong)
    {
        vertexArrayFormat[i as usize] = -(1 as i32);
        i += 1
    }
    blendIndexes = 0 as *mut crate::src::qcommon::q_shared::byte;
    blendWeights.b = 0 as *mut crate::src::qcommon::q_shared::byte;
    allocateInfluences = 0 as i32;
    if (*header).num_meshes != 0 {
        // check and swap vertex arrays
        if IQM_CheckRange(
            header,
            (*header).ofs_vertexarrays as i32,
            (*header).num_vertexarrays as i32,
            ::std::mem::size_of::<crate::iqm_h::iqmVertexArray_t>() as libc::c_ulong as i32,
        ) as u64
            != 0
        {
            return crate::src::qcommon::q_shared::qfalse;
        }
        vertexarray = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_vertexarrays as isize)
            as *mut crate::iqm_h::iqmVertexArray_t;
        i = 0 as i32;
        while (i as u32) < (*header).num_vertexarrays {
            let mut n: i32 = 0;
            let mut intPtr: *mut i32 = 0 as *mut i32;
            if (*vertexarray).size <= 0 as i32 as u32 || (*vertexarray).size > 4 as i32 as u32 {
                return crate::src::qcommon::q_shared::qfalse;
            }
            // total number of values
            n = (*header).num_vertexes.wrapping_mul((*vertexarray).size) as i32;
            match (*vertexarray).format {
                0 | 1 => {
                    // 1 byte, no swapping necessary
                    if IQM_CheckRange(
                        header,
                        (*vertexarray).offset as i32,
                        n,
                        ::std::mem::size_of::<crate::src::qcommon::q_shared::byte>()
                            as libc::c_ulong as i32,
                    ) as u64
                        != 0
                    {
                        return crate::src::qcommon::q_shared::qfalse;
                    }
                }
                4 | 5 | 7 => {
                    // 4-byte swap
                    if IQM_CheckRange(
                        header,
                        (*vertexarray).offset as i32,
                        n,
                        ::std::mem::size_of::<f32>() as libc::c_ulong as i32,
                    ) as u64
                        != 0
                    {
                        return crate::src::qcommon::q_shared::qfalse;
                    }
                    intPtr = (header as *mut crate::src::qcommon::q_shared::byte)
                        .offset((*vertexarray).offset as isize)
                        as *mut i32;
                    j = 0 as i32;
                    while j < n {
                        *intPtr = *intPtr;
                        j += 1;
                        intPtr = intPtr.offset(1)
                    }
                }
                _ => {
                    // not supported
                    return crate::src::qcommon::q_shared::qfalse;
                }
            }
            if ((*vertexarray).type_0 as libc::c_ulong)
                < (::std::mem::size_of::<[i32; 7]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<i32>() as libc::c_ulong)
            {
                vertexArrayFormat[(*vertexarray).type_0 as usize] = (*vertexarray).format as i32
            }
            match (*vertexarray).type_0 {
                0 | 2 => {
                    if (*vertexarray).format != crate::iqm_h::IQM_FLOAT as i32 as u32
                        || (*vertexarray).size != 3 as i32 as u32
                    {
                        return crate::src::qcommon::q_shared::qfalse;
                    }
                }
                3 => {
                    if (*vertexarray).format != crate::iqm_h::IQM_FLOAT as i32 as u32
                        || (*vertexarray).size != 4 as i32 as u32
                    {
                        return crate::src::qcommon::q_shared::qfalse;
                    }
                }
                1 => {
                    if (*vertexarray).format != crate::iqm_h::IQM_FLOAT as i32 as u32
                        || (*vertexarray).size != 2 as i32 as u32
                    {
                        return crate::src::qcommon::q_shared::qfalse;
                    }
                }
                4 => {
                    if (*vertexarray).format != crate::iqm_h::IQM_INT as i32 as u32
                        && (*vertexarray).format != crate::iqm_h::IQM_UBYTE as i32 as u32
                        || (*vertexarray).size != 4 as i32 as u32
                    {
                        return crate::src::qcommon::q_shared::qfalse;
                    }
                    blendIndexes = (header as *mut crate::src::qcommon::q_shared::byte)
                        .offset((*vertexarray).offset as isize)
                }
                5 => {
                    if (*vertexarray).format != crate::iqm_h::IQM_FLOAT as i32 as u32
                        && (*vertexarray).format != crate::iqm_h::IQM_UBYTE as i32 as u32
                        || (*vertexarray).size != 4 as i32 as u32
                    {
                        return crate::src::qcommon::q_shared::qfalse;
                    }
                    if (*vertexarray).format == crate::iqm_h::IQM_FLOAT as i32 as u32 {
                        blendWeights.f = (header as *mut crate::src::qcommon::q_shared::byte)
                            .offset((*vertexarray).offset as isize)
                            as *mut f32
                    } else {
                        blendWeights.b = (header as *mut crate::src::qcommon::q_shared::byte)
                            .offset((*vertexarray).offset as isize)
                    }
                }
                6 => {
                    if (*vertexarray).format != crate::iqm_h::IQM_UBYTE as i32 as u32
                        || (*vertexarray).size != 4 as i32 as u32
                    {
                        return crate::src::qcommon::q_shared::qfalse;
                    }
                }
                _ => {}
            }
            i += 1;
            vertexarray = vertexarray.offset(1)
        }
        // check for required vertex arrays
        if vertexArrayFormat[crate::iqm_h::IQM_POSITION as i32 as usize] == -(1 as i32)
            || vertexArrayFormat[crate::iqm_h::IQM_NORMAL as i32 as usize] == -(1 as i32)
            || vertexArrayFormat[crate::iqm_h::IQM_TEXCOORD as i32 as usize] == -(1 as i32)
        {
            crate::src::renderergl1::tr_main::ri.Printf.expect("non-null function pointer")(crate::src::qcommon::q_shared::PRINT_WARNING as
                                                              i32,
                                                          b"R_LoadIQM: %s is missing IQM_POSITION, IQM_NORMAL, and/or IQM_TEXCOORD array.\n\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          mod_name);
            return crate::src::qcommon::q_shared::qfalse;
        }
        if (*header).num_joints != 0 {
            if vertexArrayFormat[crate::iqm_h::IQM_BLENDINDEXES as i32 as usize] == -(1 as i32)
                || vertexArrayFormat[crate::iqm_h::IQM_BLENDWEIGHTS as i32 as usize] == -(1 as i32)
            {
                crate::src::renderergl1::tr_main::ri.Printf.expect("non-null function pointer")(crate::src::qcommon::q_shared::PRINT_WARNING as
                                                                  i32,
                                                              b"R_LoadIQM: %s is missing IQM_BLENDINDEXES and/or IQM_BLENDWEIGHTS array.\n\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              mod_name);
                return crate::src::qcommon::q_shared::qfalse;
            }
        } else {
            // ignore blend arrays if present
            vertexArrayFormat[crate::iqm_h::IQM_BLENDINDEXES as i32 as usize] = -(1 as i32);
            vertexArrayFormat[crate::iqm_h::IQM_BLENDWEIGHTS as i32 as usize] = -(1 as i32)
        }
        // opengl1 renderer doesn't use tangents
        vertexArrayFormat[crate::iqm_h::IQM_TANGENT as i32 as usize] = -(1 as i32);
        // check and swap triangles
        if IQM_CheckRange(
            header,
            (*header).ofs_triangles as i32,
            (*header).num_triangles as i32,
            ::std::mem::size_of::<crate::iqm_h::iqmTriangle_t>() as libc::c_ulong as i32,
        ) as u64
            != 0
        {
            return crate::src::qcommon::q_shared::qfalse;
        }
        triangle = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_triangles as isize)
            as *mut crate::iqm_h::iqmTriangle_t;
        i = 0 as i32;
        while (i as u32) < (*header).num_triangles {
            (*triangle).vertex[0 as i32 as usize] = (*triangle).vertex[0 as i32 as usize];
            (*triangle).vertex[1 as i32 as usize] = (*triangle).vertex[1 as i32 as usize];
            (*triangle).vertex[2 as i32 as usize] = (*triangle).vertex[2 as i32 as usize];
            if (*triangle).vertex[0 as i32 as usize] > (*header).num_vertexes
                || (*triangle).vertex[1 as i32 as usize] > (*header).num_vertexes
                || (*triangle).vertex[2 as i32 as usize] > (*header).num_vertexes
            {
                return crate::src::qcommon::q_shared::qfalse;
            }
            i += 1;
            triangle = triangle.offset(1)
        }
        // check and swap meshes
        if IQM_CheckRange(
            header,
            (*header).ofs_meshes as i32,
            (*header).num_meshes as i32,
            ::std::mem::size_of::<crate::iqm_h::iqmMesh_t>() as libc::c_ulong as i32,
        ) as u64
            != 0
        {
            return crate::src::qcommon::q_shared::qfalse;
        }
        mesh = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_meshes as isize) as *mut crate::iqm_h::iqmMesh_t;
        i = 0 as i32;
        while (i as u32) < (*header).num_meshes {
            (*mesh).name = (*mesh).name;
            (*mesh).material = (*mesh).material;
            (*mesh).first_vertex = (*mesh).first_vertex;
            (*mesh).num_vertexes = (*mesh).num_vertexes;
            (*mesh).first_triangle = (*mesh).first_triangle;
            (*mesh).num_triangles = (*mesh).num_triangles;
            if (*mesh).name < (*header).num_text {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    meshName.as_mut_ptr(),
                    (header as *mut libc::c_char)
                        .offset((*header).ofs_text as isize)
                        .offset((*mesh).name as isize),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                );
            } else {
                meshName[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char
            }
            // check ioq3 limits
            if (*mesh).num_vertexes >= 1000 as i32 as u32 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"R_LoadIQM: %s has more than %i verts on %s (%i).\n\x00" as *const u8
                        as *const libc::c_char,
                    mod_name,
                    1000 as i32 - 1 as i32,
                    if meshName[0 as i32 as usize] as i32 != 0 {
                        meshName.as_mut_ptr() as *const libc::c_char
                    } else {
                        b"a surface\x00" as *const u8 as *const libc::c_char
                    },
                    (*mesh).num_vertexes,
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            if (*mesh).num_triangles.wrapping_mul(3 as i32 as u32)
                >= (6 as i32 * 1000 as i32) as u32
            {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"R_LoadIQM: %s has more than %i triangles on %s (%i).\n\x00" as *const u8
                        as *const libc::c_char,
                    mod_name,
                    6 as i32 * 1000 as i32 / 3 as i32 - 1 as i32,
                    if meshName[0 as i32 as usize] as i32 != 0 {
                        meshName.as_mut_ptr() as *const libc::c_char
                    } else {
                        b"a surface\x00" as *const u8 as *const libc::c_char
                    },
                    (*mesh).num_triangles,
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            if (*mesh).first_vertex >= (*header).num_vertexes
                || (*mesh).first_vertex.wrapping_add((*mesh).num_vertexes) > (*header).num_vertexes
                || (*mesh).first_triangle >= (*header).num_triangles
                || (*mesh).first_triangle.wrapping_add((*mesh).num_triangles)
                    > (*header).num_triangles
                || (*mesh).name >= (*header).num_text
                || (*mesh).material >= (*header).num_text
            {
                return crate::src::qcommon::q_shared::qfalse;
            }
            // find number of unique blend influences per mesh
            if (*header).num_joints != 0 {
                j = 0 as i32;
                while (j as u32) < (*mesh).num_vertexes {
                    let mut vtx: i32 = (*mesh).first_vertex.wrapping_add(j as u32) as i32;
                    k = 0 as i32;
                    while k < j {
                        let mut influence: i32 = (*mesh).first_vertex.wrapping_add(k as u32) as i32;
                        if !(*(&mut *blendIndexes.offset((4 as i32 * influence) as isize)
                            as *mut crate::src::qcommon::q_shared::byte
                            as *mut i32)
                            != *(&mut *blendIndexes.offset((4 as i32 * vtx) as isize)
                                as *mut crate::src::qcommon::q_shared::byte
                                as *mut i32))
                        {
                            if vertexArrayFormat[crate::iqm_h::IQM_BLENDWEIGHTS as i32 as usize]
                                == crate::iqm_h::IQM_FLOAT as i32
                            {
                                if *blendWeights
                                    .f
                                    .offset((4 as i32 * influence + 0 as i32) as isize)
                                    == *blendWeights.f.offset((4 as i32 * vtx + 0 as i32) as isize)
                                    && *blendWeights
                                        .f
                                        .offset((4 as i32 * influence + 1 as i32) as isize)
                                        == *blendWeights
                                            .f
                                            .offset((4 as i32 * vtx + 1 as i32) as isize)
                                    && *blendWeights
                                        .f
                                        .offset((4 as i32 * influence + 2 as i32) as isize)
                                        == *blendWeights
                                            .f
                                            .offset((4 as i32 * vtx + 2 as i32) as isize)
                                    && *blendWeights
                                        .f
                                        .offset((4 as i32 * influence + 3 as i32) as isize)
                                        == *blendWeights
                                            .f
                                            .offset((4 as i32 * vtx + 3 as i32) as isize)
                                {
                                    break;
                                }
                            } else if *(&mut *blendWeights.b.offset((4 as i32 * influence) as isize)
                                as *mut crate::src::qcommon::q_shared::byte
                                as *mut i32)
                                == *(&mut *blendWeights.b.offset((4 as i32 * vtx) as isize)
                                    as *mut crate::src::qcommon::q_shared::byte
                                    as *mut i32)
                            {
                                break;
                            }
                        }
                        k += 1
                    }
                    if k == j {
                        allocateInfluences += 1
                    }
                    j += 1
                }
            }
            i += 1;
            mesh = mesh.offset(1)
        }
    }
    if (*header).num_poses != (*header).num_joints && (*header).num_poses != 0 as i32 as u32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_LoadIQM: %s has %d poses and %d joints, must have the same number or 0 poses\n\x00"
                as *const u8 as *const libc::c_char,
            mod_name,
            (*header).num_poses,
            (*header).num_joints,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    joint_names = 0 as i32 as crate::stddef_h::size_t;
    if (*header).num_joints != 0 {
        // check and swap joints
        if IQM_CheckRange(
            header,
            (*header).ofs_joints as i32,
            (*header).num_joints as i32,
            ::std::mem::size_of::<crate::iqm_h::iqmJoint_t>() as libc::c_ulong as i32,
        ) as u64
            != 0
        {
            return crate::src::qcommon::q_shared::qfalse;
        }
        joint = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_joints as isize) as *mut crate::iqm_h::iqmJoint_t;
        i = 0 as i32;
        while (i as u32) < (*header).num_joints {
            (*joint).name = (*joint).name;
            (*joint).parent = (*joint).parent;
            (*joint).translate[0 as i32 as usize] = (*joint).translate[0 as i32 as usize];
            (*joint).translate[1 as i32 as usize] = (*joint).translate[1 as i32 as usize];
            (*joint).translate[2 as i32 as usize] = (*joint).translate[2 as i32 as usize];
            (*joint).rotate[0 as i32 as usize] = (*joint).rotate[0 as i32 as usize];
            (*joint).rotate[1 as i32 as usize] = (*joint).rotate[1 as i32 as usize];
            (*joint).rotate[2 as i32 as usize] = (*joint).rotate[2 as i32 as usize];
            (*joint).rotate[3 as i32 as usize] = (*joint).rotate[3 as i32 as usize];
            (*joint).scale[0 as i32 as usize] = (*joint).scale[0 as i32 as usize];
            (*joint).scale[1 as i32 as usize] = (*joint).scale[1 as i32 as usize];
            (*joint).scale[2 as i32 as usize] = (*joint).scale[2 as i32 as usize];
            if (*joint).parent < -(1 as i32)
                || (*joint).parent >= (*header).num_joints as i32
                || (*joint).name >= (*header).num_text as i32 as u32
            {
                return crate::src::qcommon::q_shared::qfalse;
            }
            joint_names = (joint_names as libc::c_ulong).wrapping_add(
                crate::stdlib::strlen(
                    (header as *mut libc::c_char)
                        .offset((*header).ofs_text as isize)
                        .offset((*joint).name as isize),
                )
                .wrapping_add(1 as i32 as libc::c_ulong),
            ) as crate::stddef_h::size_t;
            i += 1;
            joint = joint.offset(1)
        }
    }
    if (*header).num_poses != 0 {
        // check and swap poses
        if IQM_CheckRange(
            header,
            (*header).ofs_poses as i32,
            (*header).num_poses as i32,
            ::std::mem::size_of::<crate::iqm_h::iqmPose_t>() as libc::c_ulong as i32,
        ) as u64
            != 0
        {
            return crate::src::qcommon::q_shared::qfalse;
        }
        pose = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_poses as isize) as *mut crate::iqm_h::iqmPose_t;
        i = 0 as i32;
        while (i as u32) < (*header).num_poses {
            (*pose).parent = (*pose).parent;
            (*pose).mask = (*pose).mask;
            (*pose).channeloffset[0 as i32 as usize] = (*pose).channeloffset[0 as i32 as usize];
            (*pose).channeloffset[1 as i32 as usize] = (*pose).channeloffset[1 as i32 as usize];
            (*pose).channeloffset[2 as i32 as usize] = (*pose).channeloffset[2 as i32 as usize];
            (*pose).channeloffset[3 as i32 as usize] = (*pose).channeloffset[3 as i32 as usize];
            (*pose).channeloffset[4 as i32 as usize] = (*pose).channeloffset[4 as i32 as usize];
            (*pose).channeloffset[5 as i32 as usize] = (*pose).channeloffset[5 as i32 as usize];
            (*pose).channeloffset[6 as i32 as usize] = (*pose).channeloffset[6 as i32 as usize];
            (*pose).channeloffset[7 as i32 as usize] = (*pose).channeloffset[7 as i32 as usize];
            (*pose).channeloffset[8 as i32 as usize] = (*pose).channeloffset[8 as i32 as usize];
            (*pose).channeloffset[9 as i32 as usize] = (*pose).channeloffset[9 as i32 as usize];
            (*pose).channelscale[0 as i32 as usize] = (*pose).channelscale[0 as i32 as usize];
            (*pose).channelscale[1 as i32 as usize] = (*pose).channelscale[1 as i32 as usize];
            (*pose).channelscale[2 as i32 as usize] = (*pose).channelscale[2 as i32 as usize];
            (*pose).channelscale[3 as i32 as usize] = (*pose).channelscale[3 as i32 as usize];
            (*pose).channelscale[4 as i32 as usize] = (*pose).channelscale[4 as i32 as usize];
            (*pose).channelscale[5 as i32 as usize] = (*pose).channelscale[5 as i32 as usize];
            (*pose).channelscale[6 as i32 as usize] = (*pose).channelscale[6 as i32 as usize];
            (*pose).channelscale[7 as i32 as usize] = (*pose).channelscale[7 as i32 as usize];
            (*pose).channelscale[8 as i32 as usize] = (*pose).channelscale[8 as i32 as usize];
            (*pose).channelscale[9 as i32 as usize] = (*pose).channelscale[9 as i32 as usize];
            i += 1;
            pose = pose.offset(1)
        }
    }
    if (*header).ofs_bounds != 0 {
        // check and swap model bounds
        if IQM_CheckRange(
            header,
            (*header).ofs_bounds as i32,
            (*header).num_frames as i32,
            ::std::mem::size_of::<crate::iqm_h::iqmBounds_t>() as libc::c_ulong as i32,
        ) as u64
            != 0
        {
            return crate::src::qcommon::q_shared::qfalse;
        }
        bounds = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_bounds as isize)
            as *mut crate::iqm_h::iqmBounds_t;
        i = 0 as i32;
        while (i as u32) < (*header).num_frames {
            (*bounds).bbmin[0 as i32 as usize] = (*bounds).bbmin[0 as i32 as usize];
            (*bounds).bbmin[1 as i32 as usize] = (*bounds).bbmin[1 as i32 as usize];
            (*bounds).bbmin[2 as i32 as usize] = (*bounds).bbmin[2 as i32 as usize];
            (*bounds).bbmax[0 as i32 as usize] = (*bounds).bbmax[0 as i32 as usize];
            (*bounds).bbmax[1 as i32 as usize] = (*bounds).bbmax[1 as i32 as usize];
            (*bounds).bbmax[2 as i32 as usize] = (*bounds).bbmax[2 as i32 as usize];
            bounds = bounds.offset(1);
            i += 1
        }
    }
    // allocate the model and copy the data
    size = ::std::mem::size_of::<crate::tr_local_h::iqmData_t>() as libc::c_ulong; // surfaces
    if (*header).num_meshes != 0 {
        size = (size as libc::c_ulong).wrapping_add(
            ((*header).num_meshes as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::tr_local_h::srfIQModel_t,
            >() as libc::c_ulong),
        ) as crate::stddef_h::size_t; // triangles
        size = (size as libc::c_ulong).wrapping_add(
            ((*header).num_triangles.wrapping_mul(3 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
        ) as crate::stddef_h::size_t; // positions
        size = (size as libc::c_ulong).wrapping_add(
            ((*header).num_vertexes.wrapping_mul(3 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        ) as crate::stddef_h::size_t; // texcoords
        size = (size as libc::c_ulong).wrapping_add(
            ((*header).num_vertexes.wrapping_mul(2 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        ) as crate::stddef_h::size_t; // normals
        size = (size as libc::c_ulong).wrapping_add(
            ((*header).num_vertexes.wrapping_mul(3 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        ) as crate::stddef_h::size_t;
        if vertexArrayFormat[crate::iqm_h::IQM_TANGENT as i32 as usize] != -(1 as i32) {
            size = (size as libc::c_ulong).wrapping_add(
                ((*header).num_vertexes.wrapping_mul(4 as i32 as u32) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
            ) as crate::stddef_h::size_t
            // tangents
        }
        if vertexArrayFormat[crate::iqm_h::IQM_COLOR as i32 as usize] != -(1 as i32) {
            size = (size as libc::c_ulong).wrapping_add(
                ((*header).num_vertexes.wrapping_mul(4 as i32 as u32) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<crate::src::qcommon::q_shared::byte>()
                        as libc::c_ulong),
            ) as crate::stddef_h::size_t
            // colors
        } // influences
        if allocateInfluences != 0 {
            size = (size as libc::c_ulong).wrapping_add(
                ((*header).num_vertexes as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
            ) as crate::stddef_h::size_t; // influenceBlendIndexes
            size = (size as libc::c_ulong).wrapping_add(
                ((allocateInfluences * 4 as i32) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<crate::src::qcommon::q_shared::byte>()
                        as libc::c_ulong),
            ) as crate::stddef_h::size_t;
            if vertexArrayFormat[crate::iqm_h::IQM_BLENDWEIGHTS as i32 as usize]
                == crate::iqm_h::IQM_UBYTE as i32
            {
                size = (size as libc::c_ulong).wrapping_add(
                    ((allocateInfluences * 4 as i32) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<crate::src::qcommon::q_shared::byte>()
                            as libc::c_ulong),
                ) as crate::stddef_h::size_t
            // influenceBlendWeights
            } else if vertexArrayFormat[crate::iqm_h::IQM_BLENDWEIGHTS as i32 as usize]
                == crate::iqm_h::IQM_FLOAT as i32
            {
                size = (size as libc::c_ulong).wrapping_add(
                    ((allocateInfluences * 4 as i32) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
                ) as crate::stddef_h::size_t
                // influenceBlendWeights
            }
        }
    } // joint names
    if (*header).num_joints != 0 {
        size = (size as libc::c_ulong).wrapping_add(joint_names) as crate::stddef_h::size_t;
        // joint mats
        size = (size as libc::c_ulong).wrapping_add(
            ((*header).num_joints as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
        ) as crate::stddef_h::size_t; // joint parents
        size = (size as libc::c_ulong).wrapping_add(
            ((*header).num_joints.wrapping_mul(12 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        ) as crate::stddef_h::size_t
    }
    if (*header).num_poses != 0 {
        size = (size as libc::c_ulong).wrapping_add(
            ((*header)
                .num_poses
                .wrapping_mul((*header).num_frames)
                .wrapping_mul(12 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        ) as crate::stddef_h::size_t
        // pose mats
    }
    if (*header).ofs_bounds != 0 {
        size = (size as libc::c_ulong).wrapping_add(
            ((*header).num_frames.wrapping_mul(6 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        ) as crate::stddef_h::size_t
    // model bounds
    } else if (*header).num_meshes != 0 && (*header).num_frames == 0 as i32 as u32 {
        size = (size as libc::c_ulong).wrapping_add(
            (6 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        ) as crate::stddef_h::size_t
        // model bounds
    }
    (*mod_0).type_0 = crate::tr_local_h::MOD_IQM;
    iqmData = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        size as i32, crate::src::qcommon::q_shared::h_low
    ) as *mut crate::tr_local_h::iqmData_t;
    (*mod_0).modelData = iqmData as *mut libc::c_void;
    // fill header
    (*iqmData).num_vertexes = if (*header).num_meshes > 0 as i32 as u32 {
        (*header).num_vertexes
    } else {
        0 as i32 as u32
    } as i32; // triangles
    (*iqmData).num_triangles = if (*header).num_meshes > 0 as i32 as u32 {
        (*header).num_triangles
    } else {
        0 as i32 as u32
    } as i32; // positions
    (*iqmData).num_frames = (*header).num_frames as i32; // texcoords
    (*iqmData).num_surfaces = (*header).num_meshes as i32; // normals
    (*iqmData).num_joints = (*header).num_joints as i32;
    (*iqmData).num_poses = (*header).num_poses as i32;
    (*iqmData).blendWeightsType = vertexArrayFormat[crate::iqm_h::IQM_BLENDWEIGHTS as i32 as usize];
    dataPtr = (iqmData as *mut crate::src::qcommon::q_shared::byte)
        .offset(::std::mem::size_of::<crate::tr_local_h::iqmData_t>() as libc::c_ulong as isize);
    if (*header).num_meshes != 0 {
        (*iqmData).surfaces = dataPtr as *mut crate::tr_local_h::srfIQModel_s;
        dataPtr =
            dataPtr.offset(((*header).num_meshes as libc::c_ulong).wrapping_mul(
                ::std::mem::size_of::<crate::tr_local_h::srfIQModel_t>() as libc::c_ulong,
            ) as isize);
        (*iqmData).triangles = dataPtr as *mut i32;
        dataPtr = dataPtr.offset(
            ((*header).num_triangles.wrapping_mul(3 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong) as isize,
        );
        (*iqmData).positions = dataPtr as *mut f32;
        dataPtr = dataPtr.offset(
            ((*header).num_vertexes.wrapping_mul(3 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as isize,
        );
        (*iqmData).texcoords = dataPtr as *mut f32;
        dataPtr = dataPtr.offset(
            ((*header).num_vertexes.wrapping_mul(2 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as isize,
        );
        (*iqmData).normals = dataPtr as *mut f32;
        dataPtr = dataPtr.offset(
            ((*header).num_vertexes.wrapping_mul(3 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as isize,
        );
        if vertexArrayFormat[crate::iqm_h::IQM_TANGENT as i32 as usize] != -(1 as i32) {
            (*iqmData).tangents = dataPtr as *mut f32;
            dataPtr = dataPtr.offset(
                ((*header).num_vertexes.wrapping_mul(4 as i32 as u32) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong)
                    as isize,
            )
            // tangents
        }
        if vertexArrayFormat[crate::iqm_h::IQM_COLOR as i32 as usize] != -(1 as i32) {
            (*iqmData).colors = dataPtr;
            dataPtr = dataPtr.offset(
                ((*header).num_vertexes.wrapping_mul(4 as i32 as u32) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<crate::src::qcommon::q_shared::byte>()
                        as libc::c_ulong) as isize,
            )
            // colors
        } // influences
        if allocateInfluences != 0 {
            (*iqmData).influences = dataPtr as *mut i32; // influenceBlendIndexes
            dataPtr = dataPtr.offset(
                ((*header).num_vertexes as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong)
                    as isize,
            );
            (*iqmData).influenceBlendIndexes = dataPtr;
            dataPtr = dataPtr.offset(
                ((allocateInfluences * 4 as i32) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<crate::src::qcommon::q_shared::byte>()
                        as libc::c_ulong) as isize,
            );
            if vertexArrayFormat[crate::iqm_h::IQM_BLENDWEIGHTS as i32 as usize]
                == crate::iqm_h::IQM_UBYTE as i32
            {
                (*iqmData).influenceBlendWeights.b = dataPtr;
                dataPtr = dataPtr.offset(
                    ((allocateInfluences * 4 as i32) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<crate::src::qcommon::q_shared::byte>()
                            as libc::c_ulong) as isize,
                )
            // influenceBlendWeights
            } else if vertexArrayFormat[crate::iqm_h::IQM_BLENDWEIGHTS as i32 as usize]
                == crate::iqm_h::IQM_FLOAT as i32
            {
                (*iqmData).influenceBlendWeights.f = dataPtr as *mut f32;
                dataPtr = dataPtr.offset(
                    ((allocateInfluences * 4 as i32) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong)
                        as isize,
                )
                // influenceBlendWeights
            }
        }
    }
    if (*header).num_joints != 0 {
        (*iqmData).jointNames = dataPtr as *mut libc::c_char;
        // joint mats
        dataPtr = dataPtr.offset(joint_names as isize); // joint names
        (*iqmData).jointParents = dataPtr as *mut i32; // joint parents
        dataPtr = dataPtr.offset(
            ((*header).num_joints as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong) as isize,
        );
        (*iqmData).jointMats = dataPtr as *mut f32;
        dataPtr = dataPtr.offset(
            ((*header).num_joints.wrapping_mul(12 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as isize,
        )
    }
    if (*header).num_poses != 0 {
        (*iqmData).poseMats = dataPtr as *mut f32;
        dataPtr = dataPtr.offset(
            ((*header)
                .num_poses
                .wrapping_mul((*header).num_frames)
                .wrapping_mul(12 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as isize,
        )
        // pose mats
    }
    if (*header).ofs_bounds != 0 {
        (*iqmData).bounds = dataPtr as *mut f32;
        dataPtr = dataPtr.offset(
            ((*header).num_frames.wrapping_mul(6 as i32 as u32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as isize,
        )
    // model bounds
    } else if (*header).num_meshes != 0 && (*header).num_frames == 0 as i32 as u32 {
        (*iqmData).bounds = dataPtr as *mut f32;
        dataPtr = dataPtr.offset(
            (6 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong)
                as isize,
        )
        // model bounds
    }
    if (*header).num_meshes != 0 {
        // register shaders
        // overwrite the material offset with the shader index
        mesh = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_meshes as isize) as *mut crate::iqm_h::iqmMesh_t; // lowercase the surface name so skin compares are faster
        surface = (*iqmData).surfaces;
        str = (header as *mut libc::c_char).offset((*header).ofs_text as isize);
        i = 0 as i32;
        while (i as u32) < (*header).num_meshes {
            (*surface).surfaceType = crate::tr_local_h::SF_IQM;
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*surface).name.as_mut_ptr(),
                str.offset((*mesh).name as isize),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
            );
            crate::src::qcommon::q_shared::Q_strlwr((*surface).name.as_mut_ptr());
            (*surface).shader = crate::src::renderergl1::tr_shader::R_FindShader(
                str.offset((*mesh).material as isize),
                -(1 as i32),
                crate::src::qcommon::q_shared::qtrue,
            ) as *mut crate::tr_local_h::shader_s;
            if (*(*surface).shader).defaultShader as u64 != 0 {
                (*surface).shader = crate::src::renderergl1::tr_main::tr.defaultShader
            }
            (*surface).data = iqmData;
            (*surface).first_vertex = (*mesh).first_vertex as i32;
            (*surface).num_vertexes = (*mesh).num_vertexes as i32;
            (*surface).first_triangle = (*mesh).first_triangle as i32;
            (*surface).num_triangles = (*mesh).num_triangles as i32;
            i += 1;
            mesh = mesh.offset(1);
            surface = surface.offset(1)
        }
        // copy triangles
        triangle = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_triangles as isize)
            as *mut crate::iqm_h::iqmTriangle_t;
        i = 0 as i32;
        while (i as u32) < (*header).num_triangles {
            *(*iqmData)
                .triangles
                .offset((3 as i32 * i + 0 as i32) as isize) =
                (*triangle).vertex[0 as i32 as usize] as i32;
            *(*iqmData)
                .triangles
                .offset((3 as i32 * i + 1 as i32) as isize) =
                (*triangle).vertex[1 as i32 as usize] as i32;
            *(*iqmData)
                .triangles
                .offset((3 as i32 * i + 2 as i32) as isize) =
                (*triangle).vertex[2 as i32 as usize] as i32;
            i += 1;
            triangle = triangle.offset(1)
        }
        // copy vertexarrays and indexes
        vertexarray = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_vertexarrays as isize)
            as *mut crate::iqm_h::iqmVertexArray_t;
        i = 0 as i32;
        while (i as u32) < (*header).num_vertexarrays {
            let mut n_0: i32 = 0;
            // skip disabled arrays
            if !(((*vertexarray).type_0 as libc::c_ulong)
                < (::std::mem::size_of::<[i32; 7]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<i32>() as libc::c_ulong)
                && vertexArrayFormat[(*vertexarray).type_0 as usize] == -(1 as i32))
            {
                // total number of values
                n_0 = (*header).num_vertexes.wrapping_mul((*vertexarray).size) as i32;
                match (*vertexarray).type_0 {
                    0 => {
                        crate::stdlib::memcpy(
                            (*iqmData).positions as *mut libc::c_void,
                            (header as *mut crate::src::qcommon::q_shared::byte)
                                .offset((*vertexarray).offset as isize)
                                as *const libc::c_void,
                            (n_0 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
                        );
                    }
                    2 => {
                        crate::stdlib::memcpy(
                            (*iqmData).normals as *mut libc::c_void,
                            (header as *mut crate::src::qcommon::q_shared::byte)
                                .offset((*vertexarray).offset as isize)
                                as *const libc::c_void,
                            (n_0 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
                        );
                    }
                    3 => {
                        crate::stdlib::memcpy(
                            (*iqmData).tangents as *mut libc::c_void,
                            (header as *mut crate::src::qcommon::q_shared::byte)
                                .offset((*vertexarray).offset as isize)
                                as *const libc::c_void,
                            (n_0 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
                        );
                    }
                    1 => {
                        crate::stdlib::memcpy(
                            (*iqmData).texcoords as *mut libc::c_void,
                            (header as *mut crate::src::qcommon::q_shared::byte)
                                .offset((*vertexarray).offset as isize)
                                as *const libc::c_void,
                            (n_0 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
                        );
                    }
                    6 => {
                        crate::stdlib::memcpy(
                            (*iqmData).colors as *mut libc::c_void,
                            (header as *mut crate::src::qcommon::q_shared::byte)
                                .offset((*vertexarray).offset as isize)
                                as *const libc::c_void,
                            (n_0 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<
                                    crate::src::qcommon::q_shared::byte,
                                >() as libc::c_ulong),
                        );
                    }
                    4 | 5 | _ => {}
                }
            }
            i += 1;
            vertexarray = vertexarray.offset(1)
        }
        // find unique blend influences per mesh
        if allocateInfluences != 0 {
            let mut vtx_0: i32 = 0;
            let mut influence_0: i32 = 0;
            let mut totalInfluences: i32 = 0 as i32;
            surface = (*iqmData).surfaces;
            i = 0 as i32;
            while (i as u32) < (*header).num_meshes {
                (*surface).first_influence = totalInfluences;
                (*surface).num_influences = 0 as i32;
                j = 0 as i32;
                while j < (*surface).num_vertexes {
                    vtx_0 = (*surface).first_vertex + j;
                    k = 0 as i32;
                    while k < (*surface).num_influences {
                        influence_0 = (*surface).first_influence + k;
                        if !(*(&mut *(*iqmData)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence_0) as isize)
                            as *mut crate::src::qcommon::q_shared::byte
                            as *mut i32)
                            != *(&mut *blendIndexes.offset((4 as i32 * vtx_0) as isize)
                                as *mut crate::src::qcommon::q_shared::byte
                                as *mut i32))
                        {
                            if vertexArrayFormat[crate::iqm_h::IQM_BLENDWEIGHTS as i32 as usize]
                                == crate::iqm_h::IQM_FLOAT as i32
                            {
                                if *(*iqmData)
                                    .influenceBlendWeights
                                    .f
                                    .offset((4 as i32 * influence_0 + 0 as i32) as isize)
                                    == *blendWeights
                                        .f
                                        .offset((4 as i32 * vtx_0 + 0 as i32) as isize)
                                    && *(*iqmData)
                                        .influenceBlendWeights
                                        .f
                                        .offset((4 as i32 * influence_0 + 1 as i32) as isize)
                                        == *blendWeights
                                            .f
                                            .offset((4 as i32 * vtx_0 + 1 as i32) as isize)
                                    && *(*iqmData)
                                        .influenceBlendWeights
                                        .f
                                        .offset((4 as i32 * influence_0 + 2 as i32) as isize)
                                        == *blendWeights
                                            .f
                                            .offset((4 as i32 * vtx_0 + 2 as i32) as isize)
                                    && *(*iqmData)
                                        .influenceBlendWeights
                                        .f
                                        .offset((4 as i32 * influence_0 + 3 as i32) as isize)
                                        == *blendWeights
                                            .f
                                            .offset((4 as i32 * vtx_0 + 3 as i32) as isize)
                                {
                                    break;
                                }
                            } else if *(&mut *(*iqmData)
                                .influenceBlendWeights
                                .b
                                .offset((4 as i32 * influence_0) as isize)
                                as *mut crate::src::qcommon::q_shared::byte
                                as *mut i32)
                                == *(&mut *blendWeights.b.offset((4 as i32 * vtx_0) as isize)
                                    as *mut crate::src::qcommon::q_shared::byte
                                    as *mut i32)
                            {
                                break;
                            }
                        }
                        k += 1
                    }
                    *(*iqmData).influences.offset(vtx_0 as isize) = (*surface).first_influence + k;
                    if k == (*surface).num_influences {
                        influence_0 = (*surface).first_influence + k;
                        *(*iqmData)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence_0 + 0 as i32) as isize) =
                            *blendIndexes.offset((4 as i32 * vtx_0 + 0 as i32) as isize);
                        *(*iqmData)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence_0 + 1 as i32) as isize) =
                            *blendIndexes.offset((4 as i32 * vtx_0 + 1 as i32) as isize);
                        *(*iqmData)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence_0 + 2 as i32) as isize) =
                            *blendIndexes.offset((4 as i32 * vtx_0 + 2 as i32) as isize);
                        *(*iqmData)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence_0 + 3 as i32) as isize) =
                            *blendIndexes.offset((4 as i32 * vtx_0 + 3 as i32) as isize);
                        if vertexArrayFormat[crate::iqm_h::IQM_BLENDWEIGHTS as i32 as usize]
                            == crate::iqm_h::IQM_FLOAT as i32
                        {
                            *(*iqmData)
                                .influenceBlendWeights
                                .f
                                .offset((4 as i32 * influence_0 + 0 as i32) as isize) =
                                *blendWeights
                                    .f
                                    .offset((4 as i32 * vtx_0 + 0 as i32) as isize);
                            *(*iqmData)
                                .influenceBlendWeights
                                .f
                                .offset((4 as i32 * influence_0 + 1 as i32) as isize) =
                                *blendWeights
                                    .f
                                    .offset((4 as i32 * vtx_0 + 1 as i32) as isize);
                            *(*iqmData)
                                .influenceBlendWeights
                                .f
                                .offset((4 as i32 * influence_0 + 2 as i32) as isize) =
                                *blendWeights
                                    .f
                                    .offset((4 as i32 * vtx_0 + 2 as i32) as isize);
                            *(*iqmData)
                                .influenceBlendWeights
                                .f
                                .offset((4 as i32 * influence_0 + 3 as i32) as isize) =
                                *blendWeights
                                    .f
                                    .offset((4 as i32 * vtx_0 + 3 as i32) as isize)
                        } else {
                            *(*iqmData)
                                .influenceBlendWeights
                                .b
                                .offset((4 as i32 * influence_0 + 0 as i32) as isize) =
                                *blendWeights
                                    .b
                                    .offset((4 as i32 * vtx_0 + 0 as i32) as isize);
                            *(*iqmData)
                                .influenceBlendWeights
                                .b
                                .offset((4 as i32 * influence_0 + 1 as i32) as isize) =
                                *blendWeights
                                    .b
                                    .offset((4 as i32 * vtx_0 + 1 as i32) as isize);
                            *(*iqmData)
                                .influenceBlendWeights
                                .b
                                .offset((4 as i32 * influence_0 + 2 as i32) as isize) =
                                *blendWeights
                                    .b
                                    .offset((4 as i32 * vtx_0 + 2 as i32) as isize);
                            *(*iqmData)
                                .influenceBlendWeights
                                .b
                                .offset((4 as i32 * influence_0 + 3 as i32) as isize) =
                                *blendWeights
                                    .b
                                    .offset((4 as i32 * vtx_0 + 3 as i32) as isize)
                        }
                        totalInfluences += 1;
                        (*surface).num_influences += 1
                    }
                    j += 1
                }
                i += 1;
                surface = surface.offset(1)
            }
        }
    }
    if (*header).num_joints != 0 {
        // copy joint names
        str = (*iqmData).jointNames;
        joint = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_joints as isize) as *mut crate::iqm_h::iqmJoint_t;
        i = 0 as i32;
        while (i as u32) < (*header).num_joints {
            let mut name: *mut libc::c_char = (header as *mut libc::c_char)
                .offset((*header).ofs_text as isize)
                .offset((*joint).name as isize);
            let mut len: i32 =
                crate::stdlib::strlen(name).wrapping_add(1 as i32 as libc::c_ulong) as i32;
            crate::stdlib::memcpy(
                str as *mut libc::c_void,
                name as *const libc::c_void,
                len as libc::c_ulong,
            );
            str = str.offset(len as isize);
            i += 1;
            joint = joint.offset(1)
        }
        // copy joint parents
        joint = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_joints as isize) as *mut crate::iqm_h::iqmJoint_t;
        i = 0 as i32;
        while (i as u32) < (*header).num_joints {
            *(*iqmData).jointParents.offset(i as isize) = (*joint).parent;
            i += 1;
            joint = joint.offset(1)
        }
        // calculate joint matrices and their inverses
        // joint inverses are needed only until the pose matrices are calculated
        mat = (*iqmData).jointMats;
        matInv = jointInvMats.as_mut_ptr();
        joint = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_joints as isize) as *mut crate::iqm_h::iqmJoint_t;
        i = 0 as i32;
        while (i as u32) < (*header).num_joints {
            let mut baseFrame: [f32; 12] = [0.; 12];
            let mut invBaseFrame: [f32; 12] = [0.; 12];
            JointToMatrix(
                (*joint).rotate.as_mut_ptr(),
                (*joint).scale.as_mut_ptr(),
                (*joint).translate.as_mut_ptr(),
                baseFrame.as_mut_ptr(),
            );
            Matrix34Invert(baseFrame.as_mut_ptr(), invBaseFrame.as_mut_ptr());
            if (*joint).parent >= 0 as i32 {
                Matrix34Multiply(
                    (*iqmData)
                        .jointMats
                        .offset((12 as i32 * (*joint).parent) as isize),
                    baseFrame.as_mut_ptr(),
                    mat,
                );
                mat = mat.offset(12 as i32 as isize);
                Matrix34Multiply(
                    invBaseFrame.as_mut_ptr(),
                    jointInvMats
                        .as_mut_ptr()
                        .offset((12 as i32 * (*joint).parent) as isize),
                    matInv,
                );
                matInv = matInv.offset(12 as i32 as isize)
            } else {
                crate::stdlib::memcpy(
                    mat as *mut libc::c_void,
                    baseFrame.as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<[f32; 12]>() as libc::c_ulong,
                );
                mat = mat.offset(12 as i32 as isize);
                crate::stdlib::memcpy(
                    matInv as *mut libc::c_void,
                    invBaseFrame.as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<[f32; 12]>() as libc::c_ulong,
                );
                matInv = matInv.offset(12 as i32 as isize)
            }
            i += 1;
            joint = joint.offset(1)
        }
    }
    if (*header).num_poses != 0 {
        // calculate pose matrices
        framedata = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_frames as isize) as *mut u16;
        mat = (*iqmData).poseMats;
        i = 0 as i32;
        while (i as u32) < (*header).num_frames {
            pose = (header as *mut crate::src::qcommon::q_shared::byte)
                .offset((*header).ofs_poses as isize)
                as *mut crate::iqm_h::iqmPose_t;
            j = 0 as i32;
            while (j as u32) < (*header).num_poses {
                let mut translate: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                let mut rotate: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
                let mut scale: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                let mut mat1: [f32; 12] = [0.; 12];
                let mut mat2: [f32; 12] = [0.; 12];
                translate[0 as i32 as usize] = (*pose).channeloffset[0 as i32 as usize];
                if (*pose).mask & 0x1 as i32 as u32 != 0 {
                    let fresh0 = framedata;
                    framedata = framedata.offset(1);
                    translate[0 as i32 as usize] +=
                        *fresh0 as i32 as f32 * (*pose).channelscale[0 as i32 as usize]
                }
                translate[1 as i32 as usize] = (*pose).channeloffset[1 as i32 as usize];
                if (*pose).mask & 0x2 as i32 as u32 != 0 {
                    let fresh1 = framedata;
                    framedata = framedata.offset(1);
                    translate[1 as i32 as usize] +=
                        *fresh1 as i32 as f32 * (*pose).channelscale[1 as i32 as usize]
                }
                translate[2 as i32 as usize] = (*pose).channeloffset[2 as i32 as usize];
                if (*pose).mask & 0x4 as i32 as u32 != 0 {
                    let fresh2 = framedata;
                    framedata = framedata.offset(1);
                    translate[2 as i32 as usize] +=
                        *fresh2 as i32 as f32 * (*pose).channelscale[2 as i32 as usize]
                }
                rotate[0 as i32 as usize] = (*pose).channeloffset[3 as i32 as usize];
                if (*pose).mask & 0x8 as i32 as u32 != 0 {
                    let fresh3 = framedata;
                    framedata = framedata.offset(1);
                    rotate[0 as i32 as usize] +=
                        *fresh3 as i32 as f32 * (*pose).channelscale[3 as i32 as usize]
                }
                rotate[1 as i32 as usize] = (*pose).channeloffset[4 as i32 as usize];
                if (*pose).mask & 0x10 as i32 as u32 != 0 {
                    let fresh4 = framedata;
                    framedata = framedata.offset(1);
                    rotate[1 as i32 as usize] +=
                        *fresh4 as i32 as f32 * (*pose).channelscale[4 as i32 as usize]
                }
                rotate[2 as i32 as usize] = (*pose).channeloffset[5 as i32 as usize];
                if (*pose).mask & 0x20 as i32 as u32 != 0 {
                    let fresh5 = framedata;
                    framedata = framedata.offset(1);
                    rotate[2 as i32 as usize] +=
                        *fresh5 as i32 as f32 * (*pose).channelscale[5 as i32 as usize]
                }
                rotate[3 as i32 as usize] = (*pose).channeloffset[6 as i32 as usize];
                if (*pose).mask & 0x40 as i32 as u32 != 0 {
                    let fresh6 = framedata;
                    framedata = framedata.offset(1);
                    rotate[3 as i32 as usize] +=
                        *fresh6 as i32 as f32 * (*pose).channelscale[6 as i32 as usize]
                }
                scale[0 as i32 as usize] = (*pose).channeloffset[7 as i32 as usize];
                if (*pose).mask & 0x80 as i32 as u32 != 0 {
                    let fresh7 = framedata;
                    framedata = framedata.offset(1);
                    scale[0 as i32 as usize] +=
                        *fresh7 as i32 as f32 * (*pose).channelscale[7 as i32 as usize]
                }
                scale[1 as i32 as usize] = (*pose).channeloffset[8 as i32 as usize];
                if (*pose).mask & 0x100 as i32 as u32 != 0 {
                    let fresh8 = framedata;
                    framedata = framedata.offset(1);
                    scale[1 as i32 as usize] +=
                        *fresh8 as i32 as f32 * (*pose).channelscale[8 as i32 as usize]
                }
                scale[2 as i32 as usize] = (*pose).channeloffset[9 as i32 as usize];
                if (*pose).mask & 0x200 as i32 as u32 != 0 {
                    let fresh9 = framedata;
                    framedata = framedata.offset(1);
                    scale[2 as i32 as usize] +=
                        *fresh9 as i32 as f32 * (*pose).channelscale[9 as i32 as usize]
                }
                // construct transformation matrix
                JointToMatrix(
                    rotate.as_mut_ptr(),
                    scale.as_mut_ptr(),
                    translate.as_mut_ptr(),
                    mat1.as_mut_ptr(),
                );
                if (*pose).parent >= 0 as i32 {
                    Matrix34Multiply(
                        (*iqmData)
                            .jointMats
                            .offset((12 as i32 * (*pose).parent) as isize),
                        mat1.as_mut_ptr(),
                        mat2.as_mut_ptr(),
                    );
                } else {
                    crate::stdlib::memcpy(
                        mat2.as_mut_ptr() as *mut libc::c_void,
                        mat1.as_mut_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<[f32; 12]>() as libc::c_ulong,
                    );
                }
                Matrix34Multiply(
                    mat2.as_mut_ptr(),
                    jointInvMats.as_mut_ptr().offset((12 as i32 * j) as isize),
                    mat,
                );
                mat = mat.offset(12 as i32 as isize);
                j += 1;
                pose = pose.offset(1)
            }
            i += 1
        }
    }
    // copy model bounds
    if (*header).ofs_bounds != 0 {
        mat = (*iqmData).bounds;
        bounds = (header as *mut crate::src::qcommon::q_shared::byte)
            .offset((*header).ofs_bounds as isize)
            as *mut crate::iqm_h::iqmBounds_t;
        i = 0 as i32;
        while (i as u32) < (*header).num_frames {
            *mat.offset(0 as i32 as isize) = (*bounds).bbmin[0 as i32 as usize];
            *mat.offset(1 as i32 as isize) = (*bounds).bbmin[1 as i32 as usize];
            *mat.offset(2 as i32 as isize) = (*bounds).bbmin[2 as i32 as usize];
            *mat.offset(3 as i32 as isize) = (*bounds).bbmax[0 as i32 as usize];
            *mat.offset(4 as i32 as isize) = (*bounds).bbmax[1 as i32 as usize];
            *mat.offset(5 as i32 as isize) = (*bounds).bbmax[2 as i32 as usize];
            mat = mat.offset(6 as i32 as isize);
            bounds = bounds.offset(1);
            i += 1
        }
    } else if (*header).num_meshes != 0 && (*header).num_frames == 0 as i32 as u32 {
        mat = (*iqmData).bounds;
        crate::src::qcommon::q_math::ClearBounds(
            &mut *(*iqmData).bounds.offset(0 as i32 as isize),
            &mut *(*iqmData).bounds.offset(3 as i32 as isize),
        );
        i = 0 as i32;
        while (i as u32) < (*header).num_vertexes {
            crate::src::qcommon::q_math::AddPointToBounds(
                &mut *(*iqmData).positions.offset((i * 3 as i32) as isize) as *mut f32
                    as *const crate::src::qcommon::q_shared::vec_t,
                &mut *(*iqmData).bounds.offset(0 as i32 as isize),
                &mut *(*iqmData).bounds.offset(3 as i32 as isize),
            );
            i += 1
        }
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=============
R_CullIQM
=============
*/

unsafe extern "C" fn R_CullIQM(
    mut data: *mut crate::tr_local_h::iqmData_t,
    mut ent: *mut crate::tr_local_h::trRefEntity_t,
) -> i32 {
    let mut bounds: [crate::src::qcommon::q_shared::vec3_t; 2] = [[0.; 3]; 2];
    let mut oldBounds: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut newBounds: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut i: i32 = 0;
    if (*data).bounds.is_null() {
        crate::src::renderergl1::tr_main::tr.pc.c_box_cull_md3_clip += 1;
        return 1 as i32;
    }
    // compute bounds pointers
    oldBounds = (*data)
        .bounds
        .offset((6 as i32 * (*ent).e.oldframe) as isize);
    newBounds = (*data).bounds.offset((6 as i32 * (*ent).e.frame) as isize);
    // calculate a bounding box in the current coordinate system
    i = 0 as i32;
    while i < 3 as i32 {
        bounds[0 as i32 as usize][i as usize] =
            if *oldBounds.offset(i as isize) < *newBounds.offset(i as isize) {
                *oldBounds.offset(i as isize)
            } else {
                *newBounds.offset(i as isize)
            };
        bounds[1 as i32 as usize][i as usize] = if *oldBounds.offset((i + 3 as i32) as isize)
            > *newBounds.offset((i + 3 as i32) as isize)
        {
            *oldBounds.offset((i + 3 as i32) as isize)
        } else {
            *newBounds.offset((i + 3 as i32) as isize)
        };
        i += 1
    }
    match crate::src::renderergl1::tr_main::R_CullLocalBox(bounds.as_mut_ptr()) {
        0 => {
            crate::src::renderergl1::tr_main::tr.pc.c_box_cull_md3_in += 1;
            return 0 as i32;
        }
        1 => {
            crate::src::renderergl1::tr_main::tr.pc.c_box_cull_md3_clip += 1;
            return 1 as i32;
        }
        2 | _ => {
            crate::src::renderergl1::tr_main::tr.pc.c_box_cull_md3_out += 1;
            return 2 as i32;
        }
    };
}
/*
=================
R_ComputeIQMFogNum

=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_ComputeIQMFogNum(
    mut data: *mut crate::tr_local_h::iqmData_t,
    mut ent: *mut crate::tr_local_h::trRefEntity_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut fog: *mut crate::tr_local_h::fog_t = 0 as *mut crate::tr_local_h::fog_t;
    let mut bounds: *const crate::src::qcommon::q_shared::vec_t =
        0 as *const crate::src::qcommon::q_shared::vec_t;
    let defaultBounds: [crate::src::qcommon::q_shared::vec_t; 6] = [
        -(8 as i32) as crate::src::qcommon::q_shared::vec_t,
        -(8 as i32) as crate::src::qcommon::q_shared::vec_t,
        -(8 as i32) as crate::src::qcommon::q_shared::vec_t,
        8 as i32 as crate::src::qcommon::q_shared::vec_t,
        8 as i32 as crate::src::qcommon::q_shared::vec_t,
        8 as i32 as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut diag: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut center: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut localOrigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut radius: crate::src::qcommon::q_shared::vec_t = 0.;
    if crate::src::renderergl1::tr_main::tr.refdef.rdflags & 0x1 as i32 != 0 {
        return 0 as i32;
    }
    // FIXME: non-normalized axis issues
    if !(*data).bounds.is_null() {
        bounds = (*data).bounds.offset((6 as i32 * (*ent).e.frame) as isize)
    } else {
        bounds = defaultBounds.as_ptr()
    }
    diag[0 as i32 as usize] = *bounds.offset(3 as i32 as isize).offset(0 as i32 as isize)
        - *bounds.offset(0 as i32 as isize);
    diag[1 as i32 as usize] = *bounds.offset(3 as i32 as isize).offset(1 as i32 as isize)
        - *bounds.offset(1 as i32 as isize);
    diag[2 as i32 as usize] = *bounds.offset(3 as i32 as isize).offset(2 as i32 as isize)
        - *bounds.offset(2 as i32 as isize);
    center[0 as i32 as usize] =
        *bounds.offset(0 as i32 as isize) + diag[0 as i32 as usize] * 0.5f32;
    center[1 as i32 as usize] =
        *bounds.offset(1 as i32 as isize) + diag[1 as i32 as usize] * 0.5f32;
    center[2 as i32 as usize] =
        *bounds.offset(2 as i32 as isize) + diag[2 as i32 as usize] * 0.5f32;
    localOrigin[0 as i32 as usize] = (*ent).e.origin[0 as i32 as usize] + center[0 as i32 as usize];
    localOrigin[1 as i32 as usize] = (*ent).e.origin[1 as i32 as usize] + center[1 as i32 as usize];
    localOrigin[2 as i32 as usize] = (*ent).e.origin[2 as i32 as usize] + center[2 as i32 as usize];
    radius =
        0.5f32 * VectorLength(diag.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    i = 1 as i32;
    while i < (*crate::src::renderergl1::tr_main::tr.world).numfogs {
        fog = &mut *(*crate::src::renderergl1::tr_main::tr.world)
            .fogs
            .offset(i as isize) as *mut crate::tr_local_h::fog_t;
        j = 0 as i32;
        while j < 3 as i32 {
            if localOrigin[j as usize] - radius >= (*fog).bounds[1 as i32 as usize][j as usize] {
                break;
            }
            if localOrigin[j as usize] + radius <= (*fog).bounds[0 as i32 as usize][j as usize] {
                break;
            }
            j += 1
        }
        if j == 3 as i32 {
            return i;
        }
        i += 1
    }
    return 0 as i32;
}
/*
=================
R_AddIQMSurfaces

Add all surfaces of this model
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_AddIQMSurfaces(mut ent: *mut crate::tr_local_h::trRefEntity_t) {
    let mut data: *mut crate::tr_local_h::iqmData_t = 0 as *mut crate::tr_local_h::iqmData_t;
    let mut surface: *mut crate::tr_local_h::srfIQModel_t =
        0 as *mut crate::tr_local_h::srfIQModel_t;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut personalModel: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut cull: i32 = 0;
    let mut fogNum: i32 = 0;
    let mut shader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    let mut skin: *mut crate::tr_local_h::skin_t = 0 as *mut crate::tr_local_h::skin_t;
    data = (*crate::src::renderergl1::tr_main::tr.currentModel).modelData
        as *mut crate::tr_local_h::iqmData_t;
    surface = (*data).surfaces;
    // don't add third_person objects if not in a portal
    personalModel = ((*ent).e.renderfx & 0x2 as i32 != 0
        && crate::src::renderergl1::tr_main::tr.viewParms.isPortal as u64 == 0)
        as i32 as crate::src::qcommon::q_shared::qboolean;
    if (*ent).e.renderfx & 0x200 as i32 != 0 {
        (*ent).e.frame %= (*data).num_frames;
        (*ent).e.oldframe %= (*data).num_frames
    }
    //
    // Validate the frames so there is no chance of a crash.
    // This will write directly into the entity structure, so
    // when the surfaces are rendered, they don't need to be
    // range checked again.
    //
    if (*ent).e.frame >= (*data).num_frames
        || (*ent).e.frame < 0 as i32
        || (*ent).e.oldframe >= (*data).num_frames
        || (*ent).e.oldframe < 0 as i32
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
            b"R_AddIQMSurfaces: no such frame %d to %d for \'%s\'\n\x00" as *const u8
                as *const libc::c_char,
            (*ent).e.oldframe,
            (*ent).e.frame,
            (*crate::src::renderergl1::tr_main::tr.currentModel)
                .name
                .as_mut_ptr(),
        );
        (*ent).e.frame = 0 as i32;
        (*ent).e.oldframe = 0 as i32
    }
    //
    // cull the entire model if merged bounding box of both frames
    // is outside the view frustum.
    //
    cull = R_CullIQM(data, ent);
    if cull == 2 as i32 {
        return;
    }
    //
    // set up lighting now that we know we aren't culled
    //
    if personalModel as u64 == 0
        || (*crate::src::renderergl1::tr_init::r_shadows).integer > 1 as i32
    {
        crate::src::renderergl1::tr_light::R_SetupEntityLighting(
            &mut crate::src::renderergl1::tr_main::tr.refdef as *mut _
                as *const crate::tr_local_h::trRefdef_t,
            ent as *mut crate::tr_local_h::trRefEntity_t,
        );
    }
    //
    // see if we are in a fog volume
    //
    fogNum = R_ComputeIQMFogNum(data, ent);
    i = 0 as i32;
    while i < (*data).num_surfaces {
        if (*ent).e.customShader != 0 {
            shader = crate::src::renderergl1::tr_shader::R_GetShaderByHandle((*ent).e.customShader)
                as *mut crate::tr_local_h::shader_s
        } else if (*ent).e.customSkin > 0 as i32
            && (*ent).e.customSkin < crate::src::renderergl1::tr_main::tr.numSkins
        {
            skin = crate::src::renderergl1::tr_image::R_GetSkinByHandle((*ent).e.customSkin)
                as *mut crate::tr_local_h::skin_s;
            shader = crate::src::renderergl1::tr_main::tr.defaultShader;
            j = 0 as i32;
            while j < (*skin).numSurfaces {
                if ::libc::strcmp(
                    (*(*skin).surfaces.offset(j as isize)).name.as_mut_ptr(),
                    (*surface).name.as_mut_ptr(),
                ) == 0
                {
                    shader = (*(*skin).surfaces.offset(j as isize)).shader;
                    break;
                } else {
                    j += 1
                }
            }
        } else {
            shader = (*surface).shader
        }
        // we will add shadows even if the main object isn't visible in the view
        // stencil shadows can't do personal models unless I polyhedron clip
        if personalModel as u64 == 0
            && (*crate::src::renderergl1::tr_init::r_shadows).integer == 2 as i32
            && fogNum == 0 as i32
            && (*ent).e.renderfx & (0x40 as i32 | 0x8 as i32) == 0
            && (*shader).sort == crate::tr_local_h::SS_OPAQUE as i32 as f32
        {
            crate::src::renderergl1::tr_main::R_AddDrawSurf(
                surface as *mut libc::c_void as *mut crate::tr_local_h::surfaceType_t,
                crate::src::renderergl1::tr_main::tr.shadowShader
                    as *mut crate::tr_local_h::shader_s,
                0 as i32,
                0 as i32,
            );
        }
        // projection shadows work fine with personal models
        if (*crate::src::renderergl1::tr_init::r_shadows).integer == 3 as i32
            && fogNum == 0 as i32
            && (*ent).e.renderfx & 0x100 as i32 != 0
            && (*shader).sort == crate::tr_local_h::SS_OPAQUE as i32 as f32
        {
            crate::src::renderergl1::tr_main::R_AddDrawSurf(
                surface as *mut libc::c_void as *mut crate::tr_local_h::surfaceType_t,
                crate::src::renderergl1::tr_main::tr.projectionShadowShader
                    as *mut crate::tr_local_h::shader_s,
                0 as i32,
                0 as i32,
            );
        }
        if personalModel as u64 == 0 {
            crate::src::renderergl1::tr_main::R_AddDrawSurf(
                surface as *mut libc::c_void as *mut crate::tr_local_h::surfaceType_t,
                shader as *mut crate::tr_local_h::shader_s,
                fogNum,
                0 as i32,
            );
        }
        surface = surface.offset(1);
        i += 1
    }
}

unsafe extern "C" fn ComputePoseMats(
    mut data: *mut crate::tr_local_h::iqmData_t,
    mut frame: i32,
    mut oldframe: i32,
    mut backlerp: f32,
    mut mat: *mut f32,
) {
    let mut mat1: *mut f32 = 0 as *mut f32;
    let mut mat2: *mut f32 = 0 as *mut f32;
    let mut joint: *mut i32 = (*data).jointParents;
    let mut i: i32 = 0;
    if oldframe == frame {
        mat1 = (*data)
            .poseMats
            .offset((12 as i32 * (*data).num_poses * frame) as isize);
        i = 0 as i32;
        while i < (*data).num_poses {
            if *joint >= 0 as i32 {
                Matrix34Multiply(
                    mat.offset((12 as i32 * *joint) as isize),
                    mat1.offset((12 as i32 * i) as isize),
                    mat.offset((12 as i32 * i) as isize),
                );
            } else {
                crate::stdlib::memcpy(
                    mat.offset((12 as i32 * i) as isize) as *mut libc::c_void,
                    mat1.offset((12 as i32 * i) as isize) as *const libc::c_void,
                    (12 as i32 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
                );
            }
            i += 1;
            joint = joint.offset(1)
        }
    } else {
        mat1 = (*data)
            .poseMats
            .offset((12 as i32 * (*data).num_poses * frame) as isize);
        mat2 = (*data)
            .poseMats
            .offset((12 as i32 * (*data).num_poses * oldframe) as isize);
        i = 0 as i32;
        while i < (*data).num_poses {
            if *joint >= 0 as i32 {
                let mut tmpMat: [f32; 12] = [0.; 12];
                InterpolateMatrix(
                    mat1.offset((12 as i32 * i) as isize),
                    mat2.offset((12 as i32 * i) as isize),
                    backlerp,
                    tmpMat.as_mut_ptr(),
                );
                Matrix34Multiply(
                    mat.offset((12 as i32 * *joint) as isize),
                    tmpMat.as_mut_ptr(),
                    mat.offset((12 as i32 * i) as isize),
                );
            } else {
                InterpolateMatrix(
                    mat1.offset((12 as i32 * i) as isize),
                    mat2.offset((12 as i32 * i) as isize),
                    backlerp,
                    mat.offset((12 as i32 * i) as isize),
                );
            }
            i += 1;
            joint = joint.offset(1)
        }
    };
}

unsafe extern "C" fn ComputeJointMats(
    mut data: *mut crate::tr_local_h::iqmData_t,
    mut frame: i32,
    mut oldframe: i32,
    mut backlerp: f32,
    mut mat: *mut f32,
) {
    let mut mat1: *mut f32 = 0 as *mut f32;
    let mut i: i32 = 0;
    if (*data).num_poses == 0 as i32 {
        crate::stdlib::memcpy(
            mat as *mut libc::c_void,
            (*data).jointMats as *const libc::c_void,
            (((*data).num_joints * 12 as i32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        );
        return;
    }
    ComputePoseMats(data, frame, oldframe, backlerp, mat);
    i = 0 as i32;
    while i < (*data).num_joints {
        let mut outmat: [f32; 12] = [0.; 12];
        mat1 = mat.offset((12 as i32 * i) as isize);
        crate::stdlib::memcpy(
            outmat.as_mut_ptr() as *mut libc::c_void,
            mat1 as *const libc::c_void,
            ::std::mem::size_of::<[f32; 12]>() as libc::c_ulong,
        );
        Matrix34Multiply(
            outmat.as_mut_ptr(),
            (*data).jointMats.offset((12 as i32 * i) as isize),
            mat1,
        );
        i += 1
    }
}
/*
=================
RB_AddIQMSurfaces

Compute vertices for this model surface
=================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_IQMSurfaceAnim(mut surface: *mut crate::tr_local_h::surfaceType_t) {
    let mut surf: *mut crate::tr_local_h::srfIQModel_t =
        surface as *mut crate::tr_local_h::srfIQModel_t;
    let mut data: *mut crate::tr_local_h::iqmData_t = (*surf).data;
    let mut poseMats: [f32; 1536] = [0.; 1536];
    let mut influenceVtxMat: [f32; 12000] = [0.; 12000];
    let mut influenceNrmMat: [f32; 9000] = [0.; 9000];
    let mut i: i32 = 0;
    let mut xyz: *mut f32 = 0 as *mut f32;
    let mut normal: *mut f32 = 0 as *mut f32;
    let mut texCoords: *mut f32 = 0 as *mut f32;
    let mut color: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut outXYZ: *mut crate::src::qcommon::q_shared::vec4_t =
        0 as *mut crate::src::qcommon::q_shared::vec4_t;
    let mut outNormal: *mut crate::src::qcommon::q_shared::vec4_t =
        0 as *mut crate::src::qcommon::q_shared::vec4_t;
    let mut outTexCoord: *mut [crate::src::qcommon::q_shared::vec2_t; 2] =
        0 as *mut [crate::src::qcommon::q_shared::vec2_t; 2];
    let mut outColor: *mut crate::tr_local_h::color4ub_t = 0 as *mut crate::tr_local_h::color4ub_t;
    let mut frame: i32 = if (*data).num_frames != 0 {
        ((*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .frame)
            % (*data).num_frames
    } else {
        0 as i32
    };
    let mut oldframe: i32 = if (*data).num_frames != 0 {
        ((*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
            .e
            .oldframe)
            % (*data).num_frames
    } else {
        0 as i32
    };
    let mut backlerp: f32 = (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
        .e
        .backlerp;
    let mut tri: *mut i32 = 0 as *mut i32;
    let mut ptr: *mut crate::tr_local_h::glIndex_t = 0 as *mut crate::tr_local_h::glIndex_t;
    let mut base: crate::tr_local_h::glIndex_t = 0;
    if crate::src::renderergl1::tr_shade::tess.numVertexes + (*surf).num_vertexes >= 1000 as i32
        || crate::src::renderergl1::tr_shade::tess.numIndexes + (*surf).num_triangles * 3 as i32
            >= 6 as i32 * 1000 as i32
    {
        crate::src::renderergl1::tr_surface::RB_CheckOverflow(
            (*surf).num_vertexes,
            (*surf).num_triangles * 3 as i32,
        );
    }
    xyz = &mut *(*data)
        .positions
        .offset(((*surf).first_vertex * 3 as i32) as isize) as *mut f32;
    normal = &mut *(*data)
        .normals
        .offset(((*surf).first_vertex * 3 as i32) as isize) as *mut f32;
    texCoords = &mut *(*data)
        .texcoords
        .offset(((*surf).first_vertex * 2 as i32) as isize) as *mut f32;
    if !(*data).colors.is_null() {
        color = &mut *(*data)
            .colors
            .offset(((*surf).first_vertex * 4 as i32) as isize)
            as *mut crate::src::qcommon::q_shared::byte
    } else {
        color = 0 as *mut crate::src::qcommon::q_shared::byte
    }
    outXYZ = &mut *crate::src::renderergl1::tr_shade::tess
        .xyz
        .as_mut_ptr()
        .offset(crate::src::renderergl1::tr_shade::tess.numVertexes as isize)
        as *mut crate::src::qcommon::q_shared::vec4_t;
    outNormal = &mut *crate::src::renderergl1::tr_shade::tess
        .normal
        .as_mut_ptr()
        .offset(crate::src::renderergl1::tr_shade::tess.numVertexes as isize)
        as *mut crate::src::qcommon::q_shared::vec4_t;
    outTexCoord = &mut *crate::src::renderergl1::tr_shade::tess
        .texCoords
        .as_mut_ptr()
        .offset(crate::src::renderergl1::tr_shade::tess.numVertexes as isize)
        as *mut [crate::src::qcommon::q_shared::vec2_t; 2];
    outColor = &mut *crate::src::renderergl1::tr_shade::tess
        .vertexColors
        .as_mut_ptr()
        .offset(crate::src::renderergl1::tr_shade::tess.numVertexes as isize)
        as *mut crate::tr_local_h::color4ub_t;
    if (*data).num_poses > 0 as i32 {
        // compute interpolated joint matrices
        ComputePoseMats(data, frame, oldframe, backlerp, poseMats.as_mut_ptr());
        // compute vertex blend influence matricies
        i = 0 as i32;
        while i < (*surf).num_influences {
            let mut influence: i32 = (*surf).first_influence + i;
            let mut vtxMat: *mut f32 = &mut *influenceVtxMat
                .as_mut_ptr()
                .offset((12 as i32 * i) as isize)
                as *mut f32;
            let mut nrmMat: *mut f32 =
                &mut *influenceNrmMat.as_mut_ptr().offset((9 as i32 * i) as isize) as *mut f32;
            let mut j: i32 = 0;
            let mut blendWeights: [f32; 4] = [0.; 4];
            let mut numWeights: i32 = 0;
            numWeights = 0 as i32;
            while numWeights < 4 as i32 {
                if (*data).blendWeightsType == crate::iqm_h::IQM_FLOAT as i32 {
                    blendWeights[numWeights as usize] = *(*data)
                        .influenceBlendWeights
                        .f
                        .offset((4 as i32 * influence + numWeights) as isize)
                } else {
                    blendWeights[numWeights as usize] = *(*data)
                        .influenceBlendWeights
                        .b
                        .offset((4 as i32 * influence + numWeights) as isize)
                        as f32
                        / 255.0f32
                }
                if blendWeights[numWeights as usize] <= 0.0f32 {
                    break;
                }
                numWeights += 1
            }
            if numWeights == 0 as i32 {
                // no blend joint, use identity matrix.
                *vtxMat.offset(0 as i32 as isize) = identityMatrix[0 as i32 as usize];
                *vtxMat.offset(1 as i32 as isize) = identityMatrix[1 as i32 as usize];
                *vtxMat.offset(2 as i32 as isize) = identityMatrix[2 as i32 as usize];
                *vtxMat.offset(3 as i32 as isize) = identityMatrix[3 as i32 as usize];
                *vtxMat.offset(4 as i32 as isize) = identityMatrix[4 as i32 as usize];
                *vtxMat.offset(5 as i32 as isize) = identityMatrix[5 as i32 as usize];
                *vtxMat.offset(6 as i32 as isize) = identityMatrix[6 as i32 as usize];
                *vtxMat.offset(7 as i32 as isize) = identityMatrix[7 as i32 as usize];
                *vtxMat.offset(8 as i32 as isize) = identityMatrix[8 as i32 as usize];
                *vtxMat.offset(9 as i32 as isize) = identityMatrix[9 as i32 as usize];
                *vtxMat.offset(10 as i32 as isize) = identityMatrix[10 as i32 as usize];
                *vtxMat.offset(11 as i32 as isize) = identityMatrix[11 as i32 as usize]
            } else {
                // compute the vertex matrix by blending the up to
                // four blend weights
                *vtxMat.offset(0 as i32 as isize) = blendWeights[0 as i32 as usize]
                    * poseMats[(12 as i32
                        * *(*data)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence + 0 as i32) as isize)
                            as i32
                        + 0 as i32) as usize];
                *vtxMat.offset(1 as i32 as isize) = blendWeights[0 as i32 as usize]
                    * poseMats[(12 as i32
                        * *(*data)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence + 0 as i32) as isize)
                            as i32
                        + 1 as i32) as usize];
                *vtxMat.offset(2 as i32 as isize) = blendWeights[0 as i32 as usize]
                    * poseMats[(12 as i32
                        * *(*data)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence + 0 as i32) as isize)
                            as i32
                        + 2 as i32) as usize];
                *vtxMat.offset(3 as i32 as isize) = blendWeights[0 as i32 as usize]
                    * poseMats[(12 as i32
                        * *(*data)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence + 0 as i32) as isize)
                            as i32
                        + 3 as i32) as usize];
                *vtxMat.offset(4 as i32 as isize) = blendWeights[0 as i32 as usize]
                    * poseMats[(12 as i32
                        * *(*data)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence + 0 as i32) as isize)
                            as i32
                        + 4 as i32) as usize];
                *vtxMat.offset(5 as i32 as isize) = blendWeights[0 as i32 as usize]
                    * poseMats[(12 as i32
                        * *(*data)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence + 0 as i32) as isize)
                            as i32
                        + 5 as i32) as usize];
                *vtxMat.offset(6 as i32 as isize) = blendWeights[0 as i32 as usize]
                    * poseMats[(12 as i32
                        * *(*data)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence + 0 as i32) as isize)
                            as i32
                        + 6 as i32) as usize];
                *vtxMat.offset(7 as i32 as isize) = blendWeights[0 as i32 as usize]
                    * poseMats[(12 as i32
                        * *(*data)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence + 0 as i32) as isize)
                            as i32
                        + 7 as i32) as usize];
                *vtxMat.offset(8 as i32 as isize) = blendWeights[0 as i32 as usize]
                    * poseMats[(12 as i32
                        * *(*data)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence + 0 as i32) as isize)
                            as i32
                        + 8 as i32) as usize];
                *vtxMat.offset(9 as i32 as isize) = blendWeights[0 as i32 as usize]
                    * poseMats[(12 as i32
                        * *(*data)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence + 0 as i32) as isize)
                            as i32
                        + 9 as i32) as usize];
                *vtxMat.offset(10 as i32 as isize) = blendWeights[0 as i32 as usize]
                    * poseMats[(12 as i32
                        * *(*data)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence + 0 as i32) as isize)
                            as i32
                        + 10 as i32) as usize];
                *vtxMat.offset(11 as i32 as isize) = blendWeights[0 as i32 as usize]
                    * poseMats[(12 as i32
                        * *(*data)
                            .influenceBlendIndexes
                            .offset((4 as i32 * influence + 0 as i32) as isize)
                            as i32
                        + 11 as i32) as usize];
                j = 1 as i32;
                while j < numWeights {
                    *vtxMat.offset(0 as i32 as isize) += blendWeights[j as usize]
                        * poseMats[(12 as i32
                            * *(*data)
                                .influenceBlendIndexes
                                .offset((4 as i32 * influence + j) as isize)
                                as i32
                            + 0 as i32) as usize];
                    *vtxMat.offset(1 as i32 as isize) += blendWeights[j as usize]
                        * poseMats[(12 as i32
                            * *(*data)
                                .influenceBlendIndexes
                                .offset((4 as i32 * influence + j) as isize)
                                as i32
                            + 1 as i32) as usize];
                    *vtxMat.offset(2 as i32 as isize) += blendWeights[j as usize]
                        * poseMats[(12 as i32
                            * *(*data)
                                .influenceBlendIndexes
                                .offset((4 as i32 * influence + j) as isize)
                                as i32
                            + 2 as i32) as usize];
                    *vtxMat.offset(3 as i32 as isize) += blendWeights[j as usize]
                        * poseMats[(12 as i32
                            * *(*data)
                                .influenceBlendIndexes
                                .offset((4 as i32 * influence + j) as isize)
                                as i32
                            + 3 as i32) as usize];
                    *vtxMat.offset(4 as i32 as isize) += blendWeights[j as usize]
                        * poseMats[(12 as i32
                            * *(*data)
                                .influenceBlendIndexes
                                .offset((4 as i32 * influence + j) as isize)
                                as i32
                            + 4 as i32) as usize];
                    *vtxMat.offset(5 as i32 as isize) += blendWeights[j as usize]
                        * poseMats[(12 as i32
                            * *(*data)
                                .influenceBlendIndexes
                                .offset((4 as i32 * influence + j) as isize)
                                as i32
                            + 5 as i32) as usize];
                    *vtxMat.offset(6 as i32 as isize) += blendWeights[j as usize]
                        * poseMats[(12 as i32
                            * *(*data)
                                .influenceBlendIndexes
                                .offset((4 as i32 * influence + j) as isize)
                                as i32
                            + 6 as i32) as usize];
                    *vtxMat.offset(7 as i32 as isize) += blendWeights[j as usize]
                        * poseMats[(12 as i32
                            * *(*data)
                                .influenceBlendIndexes
                                .offset((4 as i32 * influence + j) as isize)
                                as i32
                            + 7 as i32) as usize];
                    *vtxMat.offset(8 as i32 as isize) += blendWeights[j as usize]
                        * poseMats[(12 as i32
                            * *(*data)
                                .influenceBlendIndexes
                                .offset((4 as i32 * influence + j) as isize)
                                as i32
                            + 8 as i32) as usize];
                    *vtxMat.offset(9 as i32 as isize) += blendWeights[j as usize]
                        * poseMats[(12 as i32
                            * *(*data)
                                .influenceBlendIndexes
                                .offset((4 as i32 * influence + j) as isize)
                                as i32
                            + 9 as i32) as usize];
                    *vtxMat.offset(10 as i32 as isize) += blendWeights[j as usize]
                        * poseMats[(12 as i32
                            * *(*data)
                                .influenceBlendIndexes
                                .offset((4 as i32 * influence + j) as isize)
                                as i32
                            + 10 as i32) as usize];
                    *vtxMat.offset(11 as i32 as isize) += blendWeights[j as usize]
                        * poseMats[(12 as i32
                            * *(*data)
                                .influenceBlendIndexes
                                .offset((4 as i32 * influence + j) as isize)
                                as i32
                            + 11 as i32) as usize];
                    j += 1
                }
            }
            // compute the normal matrix as transpose of the adjoint
            // of the vertex matrix
            *nrmMat.offset(0 as i32 as isize) = *vtxMat.offset(5 as i32 as isize)
                * *vtxMat.offset(10 as i32 as isize)
                - *vtxMat.offset(6 as i32 as isize) * *vtxMat.offset(9 as i32 as isize);
            *nrmMat.offset(1 as i32 as isize) = *vtxMat.offset(6 as i32 as isize)
                * *vtxMat.offset(8 as i32 as isize)
                - *vtxMat.offset(4 as i32 as isize) * *vtxMat.offset(10 as i32 as isize);
            *nrmMat.offset(2 as i32 as isize) = *vtxMat.offset(4 as i32 as isize)
                * *vtxMat.offset(9 as i32 as isize)
                - *vtxMat.offset(5 as i32 as isize) * *vtxMat.offset(8 as i32 as isize);
            *nrmMat.offset(3 as i32 as isize) = *vtxMat.offset(2 as i32 as isize)
                * *vtxMat.offset(9 as i32 as isize)
                - *vtxMat.offset(1 as i32 as isize) * *vtxMat.offset(10 as i32 as isize);
            *nrmMat.offset(4 as i32 as isize) = *vtxMat.offset(0 as i32 as isize)
                * *vtxMat.offset(10 as i32 as isize)
                - *vtxMat.offset(2 as i32 as isize) * *vtxMat.offset(8 as i32 as isize);
            *nrmMat.offset(5 as i32 as isize) = *vtxMat.offset(1 as i32 as isize)
                * *vtxMat.offset(8 as i32 as isize)
                - *vtxMat.offset(0 as i32 as isize) * *vtxMat.offset(9 as i32 as isize);
            *nrmMat.offset(6 as i32 as isize) = *vtxMat.offset(1 as i32 as isize)
                * *vtxMat.offset(6 as i32 as isize)
                - *vtxMat.offset(2 as i32 as isize) * *vtxMat.offset(5 as i32 as isize);
            *nrmMat.offset(7 as i32 as isize) = *vtxMat.offset(2 as i32 as isize)
                * *vtxMat.offset(4 as i32 as isize)
                - *vtxMat.offset(0 as i32 as isize) * *vtxMat.offset(6 as i32 as isize);
            *nrmMat.offset(8 as i32 as isize) = *vtxMat.offset(0 as i32 as isize)
                * *vtxMat.offset(5 as i32 as isize)
                - *vtxMat.offset(1 as i32 as isize) * *vtxMat.offset(4 as i32 as isize);
            i += 1
        }
        // transform vertexes and fill other data
        i = 0 as i32;
        while i < (*surf).num_vertexes {
            let mut influence_0: i32 = *(*data)
                .influences
                .offset(((*surf).first_vertex + i) as isize)
                - (*surf).first_influence;
            let mut vtxMat_0: *mut f32 = &mut *influenceVtxMat
                .as_mut_ptr()
                .offset((12 as i32 * influence_0) as isize)
                as *mut f32;
            let mut nrmMat_0: *mut f32 = &mut *influenceNrmMat
                .as_mut_ptr()
                .offset((9 as i32 * influence_0) as isize)
                as *mut f32;
            (*outTexCoord)[0 as i32 as usize][0 as i32 as usize] =
                *texCoords.offset(0 as i32 as isize);
            (*outTexCoord)[0 as i32 as usize][1 as i32 as usize] =
                *texCoords.offset(1 as i32 as isize);
            (*outXYZ)[0 as i32 as usize] = *vtxMat_0.offset(0 as i32 as isize)
                * *xyz.offset(0 as i32 as isize)
                + *vtxMat_0.offset(1 as i32 as isize) * *xyz.offset(1 as i32 as isize)
                + *vtxMat_0.offset(2 as i32 as isize) * *xyz.offset(2 as i32 as isize)
                + *vtxMat_0.offset(3 as i32 as isize);
            (*outXYZ)[1 as i32 as usize] = *vtxMat_0.offset(4 as i32 as isize)
                * *xyz.offset(0 as i32 as isize)
                + *vtxMat_0.offset(5 as i32 as isize) * *xyz.offset(1 as i32 as isize)
                + *vtxMat_0.offset(6 as i32 as isize) * *xyz.offset(2 as i32 as isize)
                + *vtxMat_0.offset(7 as i32 as isize);
            (*outXYZ)[2 as i32 as usize] = *vtxMat_0.offset(8 as i32 as isize)
                * *xyz.offset(0 as i32 as isize)
                + *vtxMat_0.offset(9 as i32 as isize) * *xyz.offset(1 as i32 as isize)
                + *vtxMat_0.offset(10 as i32 as isize) * *xyz.offset(2 as i32 as isize)
                + *vtxMat_0.offset(11 as i32 as isize);
            (*outNormal)[0 as i32 as usize] = *nrmMat_0.offset(0 as i32 as isize)
                * *normal.offset(0 as i32 as isize)
                + *nrmMat_0.offset(1 as i32 as isize) * *normal.offset(1 as i32 as isize)
                + *nrmMat_0.offset(2 as i32 as isize) * *normal.offset(2 as i32 as isize);
            (*outNormal)[1 as i32 as usize] = *nrmMat_0.offset(3 as i32 as isize)
                * *normal.offset(0 as i32 as isize)
                + *nrmMat_0.offset(4 as i32 as isize) * *normal.offset(1 as i32 as isize)
                + *nrmMat_0.offset(5 as i32 as isize) * *normal.offset(2 as i32 as isize);
            (*outNormal)[2 as i32 as usize] = *nrmMat_0.offset(6 as i32 as isize)
                * *normal.offset(0 as i32 as isize)
                + *nrmMat_0.offset(7 as i32 as isize) * *normal.offset(1 as i32 as isize)
                + *nrmMat_0.offset(8 as i32 as isize) * *normal.offset(2 as i32 as isize);
            i += 1;
            xyz = xyz.offset(3 as i32 as isize);
            normal = normal.offset(3 as i32 as isize);
            texCoords = texCoords.offset(2 as i32 as isize);
            outXYZ = outXYZ.offset(1);
            outNormal = outNormal.offset(1);
            outTexCoord = outTexCoord.offset(1)
        }
    } else {
        // copy vertexes and fill other data
        i = 0 as i32;
        while i < (*surf).num_vertexes {
            (*outTexCoord)[0 as i32 as usize][0 as i32 as usize] =
                *texCoords.offset(0 as i32 as isize);
            (*outTexCoord)[0 as i32 as usize][1 as i32 as usize] =
                *texCoords.offset(1 as i32 as isize);
            (*outXYZ)[0 as i32 as usize] = *xyz.offset(0 as i32 as isize);
            (*outXYZ)[1 as i32 as usize] = *xyz.offset(1 as i32 as isize);
            (*outXYZ)[2 as i32 as usize] = *xyz.offset(2 as i32 as isize);
            (*outNormal)[0 as i32 as usize] = *normal.offset(0 as i32 as isize);
            (*outNormal)[1 as i32 as usize] = *normal.offset(1 as i32 as isize);
            (*outNormal)[2 as i32 as usize] = *normal.offset(2 as i32 as isize);
            i += 1;
            xyz = xyz.offset(3 as i32 as isize);
            normal = normal.offset(3 as i32 as isize);
            texCoords = texCoords.offset(2 as i32 as isize);
            outXYZ = outXYZ.offset(1);
            outNormal = outNormal.offset(1);
            outTexCoord = outTexCoord.offset(1)
        }
    }
    if !color.is_null() {
        crate::stdlib::memcpy(
            outColor as *mut libc::c_void,
            color as *const libc::c_void,
            ((*surf).num_vertexes as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::tr_local_h::color4ub_t,
            >() as libc::c_ulong),
        );
    } else {
        crate::stdlib::memset(
            outColor as *mut libc::c_void,
            0 as i32,
            ((*surf).num_vertexes as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::tr_local_h::color4ub_t,
            >() as libc::c_ulong),
        );
    }
    tri = (*data)
        .triangles
        .offset((3 as i32 * (*surf).first_triangle) as isize);
    ptr = &mut *crate::src::renderergl1::tr_shade::tess
        .indexes
        .as_mut_ptr()
        .offset(crate::src::renderergl1::tr_shade::tess.numIndexes as isize)
        as *mut crate::tr_local_h::glIndex_t;
    base = crate::src::renderergl1::tr_shade::tess.numVertexes as crate::tr_local_h::glIndex_t;
    i = 0 as i32;
    while i < (*surf).num_triangles {
        let fresh10 = tri;
        tri = tri.offset(1);
        let fresh11 = ptr;
        ptr = ptr.offset(1);
        *fresh11 = base.wrapping_add((*fresh10 - (*surf).first_vertex) as u32);
        let fresh12 = tri;
        tri = tri.offset(1);
        let fresh13 = ptr;
        ptr = ptr.offset(1);
        *fresh13 = base.wrapping_add((*fresh12 - (*surf).first_vertex) as u32);
        let fresh14 = tri;
        tri = tri.offset(1);
        let fresh15 = ptr;
        ptr = ptr.offset(1);
        *fresh15 = base.wrapping_add((*fresh14 - (*surf).first_vertex) as u32);
        i += 1
    }
    crate::src::renderergl1::tr_shade::tess.numIndexes += 3 as i32 * (*surf).num_triangles;
    crate::src::renderergl1::tr_shade::tess.numVertexes += (*surf).num_vertexes;
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
#[no_mangle]

pub unsafe extern "C" fn R_IQMLerpTag(
    mut tag: *mut crate::src::qcommon::q_shared::orientation_t,
    mut data: *mut crate::tr_local_h::iqmData_t,
    mut startFrame: i32,
    mut endFrame: i32,
    mut frac: f32,
    mut tagName: *const libc::c_char,
) -> i32 {
    let mut jointMats: [f32; 1536] = [0.; 1536];
    let mut joint: i32 = 0;
    let mut names: *mut libc::c_char = (*data).jointNames;
    // get joint number by reading the joint names
    joint = 0 as i32;
    while joint < (*data).num_joints {
        if ::libc::strcmp(tagName, names) == 0 {
            break;
        }
        names = names
            .offset(crate::stdlib::strlen(names).wrapping_add(1 as i32 as libc::c_ulong) as isize);
        joint += 1
    }
    if joint >= (*data).num_joints {
        crate::src::qcommon::q_math::AxisClear((*tag).axis.as_mut_ptr());
        (*tag).origin[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
        (*tag).origin[1 as i32 as usize] = (*tag).origin[2 as i32 as usize];
        (*tag).origin[0 as i32 as usize] = (*tag).origin[1 as i32 as usize];
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    ComputeJointMats(data, startFrame, endFrame, frac, jointMats.as_mut_ptr());
    (*tag).axis[0 as i32 as usize][0 as i32 as usize] =
        jointMats[(12 as i32 * joint + 0 as i32) as usize];
    (*tag).axis[1 as i32 as usize][0 as i32 as usize] =
        jointMats[(12 as i32 * joint + 1 as i32) as usize];
    (*tag).axis[2 as i32 as usize][0 as i32 as usize] =
        jointMats[(12 as i32 * joint + 2 as i32) as usize];
    (*tag).origin[0 as i32 as usize] = jointMats[(12 as i32 * joint + 3 as i32) as usize];
    (*tag).axis[0 as i32 as usize][1 as i32 as usize] =
        jointMats[(12 as i32 * joint + 4 as i32) as usize];
    (*tag).axis[1 as i32 as usize][1 as i32 as usize] =
        jointMats[(12 as i32 * joint + 5 as i32) as usize];
    (*tag).axis[2 as i32 as usize][1 as i32 as usize] =
        jointMats[(12 as i32 * joint + 6 as i32) as usize];
    (*tag).origin[1 as i32 as usize] = jointMats[(12 as i32 * joint + 7 as i32) as usize];
    (*tag).axis[0 as i32 as usize][2 as i32 as usize] =
        jointMats[(12 as i32 * joint + 8 as i32) as usize];
    (*tag).axis[1 as i32 as usize][2 as i32 as usize] =
        jointMats[(12 as i32 * joint + 9 as i32) as usize];
    (*tag).axis[2 as i32 as usize][2 as i32 as usize] =
        jointMats[(12 as i32 * joint + 10 as i32) as usize];
    (*tag).origin[2 as i32 as usize] = jointMats[(12 as i32 * joint + 11 as i32) as usize];
    return crate::src::qcommon::q_shared::qtrue as i32;
}

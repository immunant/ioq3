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

    // __Q_SHARED_H
}

pub use crate::qfiles_h::dbrush_t;
pub use crate::qfiles_h::dbrushside_t;
pub use crate::qfiles_h::dfog_t;
pub use crate::qfiles_h::dheader_t;
pub use crate::qfiles_h::dleaf_t;
pub use crate::qfiles_h::dmodel_t;
pub use crate::qfiles_h::dnode_t;
pub use crate::qfiles_h::dplane_t;
pub use crate::qfiles_h::drawVert_t;
pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::dsurface_t;
pub use crate::qfiles_h::lump_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::qfiles_h::MST_BAD;
pub use crate::qfiles_h::MST_FLARE;
pub use crate::qfiles_h::MST_PATCH;
pub use crate::qfiles_h::MST_PLANAR;
pub use crate::qfiles_h::MST_TRIANGLE_SOUP;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AddPointToBounds;
pub use crate::src::qcommon::q_math::ClearBounds;
pub use crate::src::qcommon::q_math::ColorBytes4;
pub use crate::src::qcommon::q_math::SetPlaneSignbits;
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
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::COM_ParseExt;
pub use crate::src::qcommon::q_shared::COM_SkipPath;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
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
pub use crate::src::renderergl1::tr_bsp::q_shared_h::VectorLength;
pub use crate::stdlib::GLuint;
pub use crate::tr_public_h::refimport_t;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::polyVert_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
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

pub use crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands;
pub use crate::src::renderergl1::tr_curve::R_FreeSurfaceGridMesh;
pub use crate::src::renderergl1::tr_curve::R_GridInsertColumn;
pub use crate::src::renderergl1::tr_curve::R_GridInsertRow;
pub use crate::src::renderergl1::tr_curve::R_SubdividePatchToGrid;
pub use crate::src::renderergl1::tr_image::R_CreateImage;
pub use crate::src::renderergl1::tr_init::glConfig;
pub use crate::src::renderergl1::tr_init::r_fullbright;
pub use crate::src::renderergl1::tr_init::r_lightmap;
pub use crate::src::renderergl1::tr_init::r_mapOverBrightBits;
pub use crate::src::renderergl1::tr_init::r_singleShader;
pub use crate::src::renderergl1::tr_init::r_vertexLight;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_model::R_AllocModel;
pub use crate::src::renderergl1::tr_shader::R_FindShader;
pub use crate::src::renderergl1::tr_shader::R_RemapShader;
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
pub use crate::tr_local_h::srfFlare_s;
pub use crate::tr_local_h::srfFlare_t;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_102 {
    pub b: *mut crate::src::qcommon::q_shared::byte,
    pub v: *mut libc::c_void,
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
// tr_map.c
/*

Loads and prepares a map file for scene rendering.

A single entry point:

void RE_LoadWorldMap( const char *name );

*/

static mut s_worldData: crate::tr_local_h::world_t = crate::tr_local_h::world_t {
    name: [0; 64],
    baseName: [0; 64],
    dataSize: 0,
    numShaders: 0,
    shaders: 0 as *const crate::qfiles_h::dshader_t as *mut crate::qfiles_h::dshader_t,
    bmodels: 0 as *const crate::tr_local_h::bmodel_t as *mut crate::tr_local_h::bmodel_t,
    numplanes: 0,
    planes: 0 as *const crate::src::qcommon::q_shared::cplane_t
        as *mut crate::src::qcommon::q_shared::cplane_t,
    numnodes: 0,
    numDecisionNodes: 0,
    nodes: 0 as *const crate::tr_local_h::mnode_t as *mut crate::tr_local_h::mnode_t,
    numsurfaces: 0,
    surfaces: 0 as *const crate::tr_local_h::msurface_t as *mut crate::tr_local_h::msurface_t,
    nummarksurfaces: 0,
    marksurfaces: 0 as *const *mut crate::tr_local_h::msurface_t
        as *mut *mut crate::tr_local_h::msurface_t,
    numfogs: 0,
    fogs: 0 as *const crate::tr_local_h::fog_t as *mut crate::tr_local_h::fog_t,
    lightGridOrigin: [0.; 3],
    lightGridSize: [0.; 3],
    lightGridInverseSize: [0.; 3],
    lightGridBounds: [0; 3],
    lightGridData: 0 as *const crate::src::qcommon::q_shared::byte
        as *mut crate::src::qcommon::q_shared::byte,
    numClusters: 0,
    clusterBytes: 0,
    vis: 0 as *const crate::src::qcommon::q_shared::byte,
    novis: 0 as *const crate::src::qcommon::q_shared::byte
        as *mut crate::src::qcommon::q_shared::byte,
    entityString: 0 as *const libc::c_char as *mut libc::c_char,
    entityParsePoint: 0 as *const libc::c_char as *mut libc::c_char,
};

static mut fileBase: *mut crate::src::qcommon::q_shared::byte =
    0 as *const crate::src::qcommon::q_shared::byte as *mut crate::src::qcommon::q_shared::byte;
#[no_mangle]

pub static mut c_subdivisions: i32 = 0;
#[no_mangle]

pub static mut c_gridVerts: i32 = 0;
//===============================================================================

unsafe extern "C" fn HSVtoRGB(mut h: f32, mut s: f32, mut v: f32, mut rgb: *mut f32) {
    let mut i: i32 = 0;
    let mut f: f32 = 0.;
    let mut p: f32 = 0.;
    let mut q: f32 = 0.;
    let mut t: f32 = 0.;
    h *= 5 as i32 as f32;
    i = crate::stdlib::floor(h as f64) as i32;
    f = h - i as f32;
    p = v * (1 as i32 as f32 - s);
    q = v * (1 as i32 as f32 - s * f);
    t = v * (1 as i32 as f32 - s * (1 as i32 as f32 - f));
    match i {
        0 => {
            *rgb.offset(0 as i32 as isize) = v;
            *rgb.offset(1 as i32 as isize) = t;
            *rgb.offset(2 as i32 as isize) = p
        }
        1 => {
            *rgb.offset(0 as i32 as isize) = q;
            *rgb.offset(1 as i32 as isize) = v;
            *rgb.offset(2 as i32 as isize) = p
        }
        2 => {
            *rgb.offset(0 as i32 as isize) = p;
            *rgb.offset(1 as i32 as isize) = v;
            *rgb.offset(2 as i32 as isize) = t
        }
        3 => {
            *rgb.offset(0 as i32 as isize) = p;
            *rgb.offset(1 as i32 as isize) = q;
            *rgb.offset(2 as i32 as isize) = v
        }
        4 => {
            *rgb.offset(0 as i32 as isize) = t;
            *rgb.offset(1 as i32 as isize) = p;
            *rgb.offset(2 as i32 as isize) = v
        }
        5 => {
            *rgb.offset(0 as i32 as isize) = v;
            *rgb.offset(1 as i32 as isize) = p;
            *rgb.offset(2 as i32 as isize) = q
        }
        _ => {}
    };
}
/*
===============
R_ColorShiftLightingBytes

===============
*/

unsafe extern "C" fn R_ColorShiftLightingBytes(
    mut in_0: *mut crate::src::qcommon::q_shared::byte,
    mut out: *mut crate::src::qcommon::q_shared::byte,
) {
    let mut shift: i32 = 0;
    let mut r: i32 = 0;
    let mut g: i32 = 0;
    let mut b: i32 = 0;
    // shift the color data based on overbright range
    shift = (*crate::src::renderergl1::tr_init::r_mapOverBrightBits).integer
        - crate::src::renderergl1::tr_main::tr.overbrightBits;
    // shift the data based on overbright range
    r = (*in_0.offset(0 as i32 as isize) as i32) << shift;
    g = (*in_0.offset(1 as i32 as isize) as i32) << shift;
    b = (*in_0.offset(2 as i32 as isize) as i32) << shift;
    // normalize by color instead of saturating to white
    if r | g | b > 255 as i32 {
        let mut max: i32 = 0;
        max = if r > g { r } else { g };
        max = if max > b { max } else { b };
        r = r * 255 as i32 / max;
        g = g * 255 as i32 / max;
        b = b * 255 as i32 / max
    }
    *out.offset(0 as i32 as isize) = r as crate::src::qcommon::q_shared::byte;
    *out.offset(1 as i32 as isize) = g as crate::src::qcommon::q_shared::byte;
    *out.offset(2 as i32 as isize) = b as crate::src::qcommon::q_shared::byte;
    *out.offset(3 as i32 as isize) = *in_0.offset(3 as i32 as isize);
}

unsafe extern "C" fn R_LoadLightmaps(mut l: *mut crate::qfiles_h::lump_t) {
    let mut buf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut buf_p: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut len: i32 = 0;
    let mut image: [crate::src::qcommon::q_shared::byte; 65536] = [0; 65536];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut maxIntensity: f32 = 0 as i32 as f32;
    len = (*l).filelen;
    if len == 0 {
        return;
    }
    buf = fileBase.offset((*l).fileofs as isize);
    // we are about to upload textures
    crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands();
    // create all the lightmaps
    crate::src::renderergl1::tr_main::tr.numLightmaps = len / (128 as i32 * 128 as i32 * 3 as i32);
    if crate::src::renderergl1::tr_main::tr.numLightmaps == 1 as i32 {
        //FIXME: HACK: maps with only one lightmap turn up fullbright for some reason.
        //this avoids this, but isn't the correct solution.
        crate::src::renderergl1::tr_main::tr.numLightmaps += 1
    }
    // if we are in r_vertexLight mode, we don't need the lightmaps at all
    if (*crate::src::renderergl1::tr_init::r_vertexLight).integer != 0
        || crate::src::renderergl1::tr_init::glConfig.hardwareType as u32
            == crate::tr_types_h::GLHW_PERMEDIA2 as i32 as u32
    {
        return;
    }
    crate::src::renderergl1::tr_main::tr.lightmaps =
        crate::src::renderergl1::tr_main::ri
            .Hunk_Alloc
            .expect("non-null function pointer")(
            (crate::src::renderergl1::tr_main::tr.numLightmaps as libc::c_ulong).wrapping_mul(
                ::std::mem::size_of::<*mut crate::tr_common_h::image_t>() as libc::c_ulong,
            ) as i32,
            crate::src::qcommon::q_shared::h_low,
        ) as *mut *mut crate::tr_common_h::image_t;
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_main::tr.numLightmaps {
        // expand the 24 bit on-disk to 32 bit
        buf_p = buf.offset((i * 128 as i32 * 128 as i32 * 3 as i32) as isize);
        if (*crate::src::renderergl1::tr_init::r_lightmap).integer == 2 as i32 {
            // color code by intensity as development tool	(FIXME: check range)
            j = 0 as i32;
            while j < 128 as i32 * 128 as i32 {
                let mut r: f32 = *buf_p.offset((j * 3 as i32 + 0 as i32) as isize) as f32;
                let mut g: f32 = *buf_p.offset((j * 3 as i32 + 1 as i32) as isize) as f32;
                let mut b: f32 = *buf_p.offset((j * 3 as i32 + 2 as i32) as isize) as f32;
                let mut intensity: f32 = 0.;
                let mut out: [f32; 3] = [0.0f64 as f32, 0.0f64 as f32, 0.0f64 as f32];
                intensity = 0.33f32 * r + 0.685f32 * g + 0.063f32 * b;
                if intensity > 255 as i32 as f32 {
                    intensity = 1.0f32
                } else {
                    intensity /= 255.0f32
                }
                if intensity > maxIntensity {
                    maxIntensity = intensity
                }
                HSVtoRGB(intensity, 1.00f64 as f32, 0.50f64 as f32, out.as_mut_ptr());
                image[(j * 4 as i32 + 0 as i32) as usize] = (out[0 as i32 as usize]
                    * 255 as i32 as f32)
                    as crate::src::qcommon::q_shared::byte;
                image[(j * 4 as i32 + 1 as i32) as usize] = (out[1 as i32 as usize]
                    * 255 as i32 as f32)
                    as crate::src::qcommon::q_shared::byte;
                image[(j * 4 as i32 + 2 as i32) as usize] = (out[2 as i32 as usize]
                    * 255 as i32 as f32)
                    as crate::src::qcommon::q_shared::byte;
                image[(j * 4 as i32 + 3 as i32) as usize] =
                    255 as i32 as crate::src::qcommon::q_shared::byte;
                j += 1
            }
        } else {
            j = 0 as i32;
            while j < 128 as i32 * 128 as i32 {
                R_ColorShiftLightingBytes(
                    &mut *buf_p.offset((j * 3 as i32) as isize),
                    &mut *image.as_mut_ptr().offset((j * 4 as i32) as isize),
                );
                image[(j * 4 as i32 + 3 as i32) as usize] =
                    255 as i32 as crate::src::qcommon::q_shared::byte;
                j += 1
            }
        }
        let ref mut fresh0 = *crate::src::renderergl1::tr_main::tr
            .lightmaps
            .offset(i as isize);
        *fresh0 = crate::src::renderergl1::tr_image::R_CreateImage(
            crate::src::qcommon::q_shared::va(
                b"*lightmap%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                i,
            ),
            image.as_mut_ptr(),
            128 as i32,
            128 as i32,
            crate::tr_common_h::IMGTYPE_COLORALPHA,
            (crate::tr_common_h::IMGFLAG_NOLIGHTSCALE as i32
                | crate::tr_common_h::IMGFLAG_NO_COMPRESSION as i32
                | crate::tr_common_h::IMGFLAG_CLAMPTOEDGE as i32)
                as crate::tr_common_h::imgFlags_t,
            0 as i32,
        ) as *mut crate::tr_common_h::image_s;
        i += 1
    }
    if (*crate::src::renderergl1::tr_init::r_lightmap).integer == 2 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Brightest lightmap value: %d\n\x00" as *const u8 as *const libc::c_char,
            (maxIntensity * 255 as i32 as f32) as i32,
        );
    };
}
/*
=================
RE_SetWorldVisData

This is called by the clipmodel subsystem so we can share the 1.8 megs of
space in big maps...
=================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_SetWorldVisData(mut vis: *const crate::src::qcommon::q_shared::byte) {
    crate::src::renderergl1::tr_main::tr.externalVisData = vis;
}
/*
=================
R_LoadVisibility
=================
*/

unsafe extern "C" fn R_LoadVisibility(mut l: *mut crate::qfiles_h::lump_t) {
    let mut len: i32 = 0;
    let mut buf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    len = s_worldData.numClusters + 63 as i32 & !(63 as i32);
    s_worldData.novis = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        len, crate::src::qcommon::q_shared::h_low
    ) as *mut crate::src::qcommon::q_shared::byte;
    crate::stdlib::memset(
        s_worldData.novis as *mut libc::c_void,
        0xff as i32,
        len as libc::c_ulong,
    );
    len = (*l).filelen;
    if len == 0 {
        return;
    }
    buf = fileBase.offset((*l).fileofs as isize);
    s_worldData.numClusters = *(buf as *mut i32).offset(0 as i32 as isize);
    s_worldData.clusterBytes = *(buf as *mut i32).offset(1 as i32 as isize);
    // CM_Load should have given us the vis data to share, so
    // we don't need to allocate another copy
    if !crate::src::renderergl1::tr_main::tr
        .externalVisData
        .is_null()
    {
        s_worldData.vis = crate::src::renderergl1::tr_main::tr.externalVisData
    } else {
        let mut dest: *mut crate::src::qcommon::q_shared::byte =
            0 as *mut crate::src::qcommon::q_shared::byte;
        dest = crate::src::renderergl1::tr_main::ri
            .Hunk_Alloc
            .expect("non-null function pointer")(
            len - 8 as i32,
            crate::src::qcommon::q_shared::h_low,
        ) as *mut crate::src::qcommon::q_shared::byte;
        crate::stdlib::memcpy(
            dest as *mut libc::c_void,
            buf.offset(8 as i32 as isize) as *const libc::c_void,
            (len - 8 as i32) as libc::c_ulong,
        );
        s_worldData.vis = dest
    };
}
//===============================================================================
/*
===============
ShaderForShaderNum
===============
*/

unsafe extern "C" fn ShaderForShaderNum(
    mut shaderNum: i32,
    mut lightmapNum: i32,
) -> *mut crate::tr_local_h::shader_t {
    let mut shader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    let mut dsh: *mut crate::qfiles_h::dshader_t = 0 as *mut crate::qfiles_h::dshader_t;
    let mut _shaderNum: i32 = shaderNum;
    if _shaderNum < 0 as i32 || _shaderNum >= s_worldData.numShaders {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"ShaderForShaderNum: bad num %i\x00" as *const u8 as *const libc::c_char,
            _shaderNum,
        );
    }
    dsh = &mut *s_worldData.shaders.offset(_shaderNum as isize) as *mut crate::qfiles_h::dshader_t;
    if (*crate::src::renderergl1::tr_init::r_vertexLight).integer != 0
        || crate::src::renderergl1::tr_init::glConfig.hardwareType as u32
            == crate::tr_types_h::GLHW_PERMEDIA2 as i32 as u32
    {
        lightmapNum = -(3 as i32)
    }
    if (*crate::src::renderergl1::tr_init::r_fullbright).integer != 0 {
        lightmapNum = -(2 as i32)
    }
    shader = crate::src::renderergl1::tr_shader::R_FindShader(
        (*dsh).shader.as_mut_ptr(),
        lightmapNum,
        crate::src::qcommon::q_shared::qtrue,
    ) as *mut crate::tr_local_h::shader_s;
    // if the shader had errors, just use default shader
    if (*shader).defaultShader as u64 != 0 {
        return crate::src::renderergl1::tr_main::tr.defaultShader;
    }
    return shader;
}
/*
===============
ParseFace
===============
*/

unsafe extern "C" fn ParseFace(
    mut ds: *mut crate::qfiles_h::dsurface_t,
    mut verts: *mut crate::qfiles_h::drawVert_t,
    mut surf: *mut crate::tr_local_h::msurface_t,
    mut indexes: *mut i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cv: *mut crate::tr_local_h::srfSurfaceFace_t =
        0 as *mut crate::tr_local_h::srfSurfaceFace_t;
    let mut numPoints: i32 = 0;
    let mut numIndexes: i32 = 0;
    let mut lightmapNum: i32 = 0;
    let mut sfaceSize: i32 = 0;
    let mut ofsIndexes: i32 = 0;
    lightmapNum = (*ds).lightmapNum;
    // get fog volume
    (*surf).fogIndex = (*ds).fogNum + 1 as i32;
    // get shader value
    (*surf).shader = ShaderForShaderNum((*ds).shaderNum, lightmapNum);
    if (*crate::src::renderergl1::tr_init::r_singleShader).integer != 0
        && (*(*surf).shader).isSky as u64 == 0
    {
        (*surf).shader = crate::src::renderergl1::tr_main::tr.defaultShader
    }
    numPoints = (*ds).numVerts;
    if numPoints > 64 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: MAX_FACE_POINTS exceeded: %i\n\x00" as *const u8 as *const libc::c_char,
            numPoints,
        );
        numPoints = 64 as i32;
        (*surf).shader = crate::src::renderergl1::tr_main::tr.defaultShader
    }
    numIndexes = (*ds).numIndexes;
    // create the srfSurfaceFace_t
    sfaceSize = (40 as libc::c_ulong).wrapping_add(
        (::std::mem::size_of::<[f32; 8]>() as libc::c_ulong)
            .wrapping_mul(numPoints as libc::c_ulong),
    ) as i32;
    ofsIndexes = sfaceSize;
    sfaceSize = (sfaceSize as libc::c_ulong).wrapping_add(
        (::std::mem::size_of::<i32>() as libc::c_ulong).wrapping_mul(numIndexes as libc::c_ulong),
    ) as i32;
    cv = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        sfaceSize, crate::src::qcommon::q_shared::h_low
    ) as *mut crate::tr_local_h::srfSurfaceFace_t;
    (*cv).surfaceType = crate::tr_local_h::SF_FACE;
    (*cv).numPoints = numPoints;
    (*cv).numIndices = numIndexes;
    (*cv).ofsIndices = ofsIndexes;
    verts = verts.offset((*ds).firstVert as isize);
    i = 0 as i32;
    while i < numPoints {
        j = 0 as i32;
        while j < 3 as i32 {
            (*(*cv).points.as_mut_ptr().offset(i as isize))[j as usize] =
                (*verts.offset(i as isize)).xyz[j as usize];
            j += 1
        }
        j = 0 as i32;
        while j < 2 as i32 {
            (*(*cv).points.as_mut_ptr().offset(i as isize))[(3 as i32 + j) as usize] =
                (*verts.offset(i as isize)).st[j as usize];
            (*(*cv).points.as_mut_ptr().offset(i as isize))[(5 as i32 + j) as usize] =
                (*verts.offset(i as isize)).lightmap[j as usize];
            j += 1
        }
        R_ColorShiftLightingBytes(
            (*verts.offset(i as isize)).color.as_mut_ptr(),
            &mut *(*(*cv).points.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(7 as i32 as isize) as *mut f32
                as *mut crate::src::qcommon::q_shared::byte,
        );
        i += 1
    }
    indexes = indexes.offset((*ds).firstIndex as isize);
    i = 0 as i32;
    while i < numIndexes {
        *((cv as *mut crate::src::qcommon::q_shared::byte).offset((*cv).ofsIndices as isize)
            as *mut i32)
            .offset(i as isize) = *indexes.offset(i as isize);
        i += 1
    }
    // take the plane information from the lightmap vector
    i = 0 as i32;
    while i < 3 as i32 {
        (*cv).plane.normal[i as usize] = (*ds).lightmapVecs[2 as i32 as usize][i as usize];
        i += 1
    }
    (*cv).plane.dist = (*(*cv).points.as_mut_ptr().offset(0 as i32 as isize))[0 as i32 as usize]
        * (*cv).plane.normal[0 as i32 as usize]
        + (*(*cv).points.as_mut_ptr().offset(0 as i32 as isize))[1 as i32 as usize]
            * (*cv).plane.normal[1 as i32 as usize]
        + (*(*cv).points.as_mut_ptr().offset(0 as i32 as isize))[2 as i32 as usize]
            * (*cv).plane.normal[2 as i32 as usize];
    crate::src::qcommon::q_math::SetPlaneSignbits(
        &mut (*cv).plane as *mut _ as *mut crate::src::qcommon::q_shared::cplane_s,
    );
    (*cv).plane.type_0 = if (*cv).plane.normal[0 as i32 as usize] as f64 == 1.0f64 {
        0 as i32
    } else if (*cv).plane.normal[1 as i32 as usize] as f64 == 1.0f64 {
        1 as i32
    } else if (*cv).plane.normal[2 as i32 as usize] as f64 == 1.0f64 {
        2 as i32
    } else {
        3 as i32
    } as crate::src::qcommon::q_shared::byte;
    (*surf).data = cv as *mut crate::tr_local_h::surfaceType_t;
}
/*
===============
ParseMesh
===============
*/

unsafe extern "C" fn ParseMesh(
    mut ds: *mut crate::qfiles_h::dsurface_t,
    mut verts: *mut crate::qfiles_h::drawVert_t,
    mut surf: *mut crate::tr_local_h::msurface_t,
) {
    let mut grid: *mut crate::tr_local_h::srfGridMesh_t =
        0 as *mut crate::tr_local_h::srfGridMesh_t;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut numPoints: i32 = 0;
    let mut points: [crate::qfiles_h::drawVert_t; 1024] = [crate::qfiles_h::drawVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        lightmap: [0.; 2],
        normal: [0.; 3],
        color: [0; 4],
    }; 1024];
    let mut lightmapNum: i32 = 0;
    let mut bounds: [crate::src::qcommon::q_shared::vec3_t; 2] = [[0.; 3]; 2];
    let mut tmpVec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    static mut skipData: crate::tr_local_h::surfaceType_t = crate::tr_local_h::SF_SKIP;
    lightmapNum = (*ds).lightmapNum;
    // get fog volume
    (*surf).fogIndex = (*ds).fogNum + 1 as i32;
    // get shader value
    (*surf).shader = ShaderForShaderNum((*ds).shaderNum, lightmapNum);
    if (*crate::src::renderergl1::tr_init::r_singleShader).integer != 0
        && (*(*surf).shader).isSky as u64 == 0
    {
        (*surf).shader = crate::src::renderergl1::tr_main::tr.defaultShader
    }
    // we may have a nodraw surface, because they might still need to
    // be around for movement clipping
    if (*s_worldData.shaders.offset((*ds).shaderNum as isize)).surfaceFlags & 0x80 as i32 != 0 {
        (*surf).data = &mut skipData;
        return;
    }
    width = (*ds).patchWidth;
    height = (*ds).patchHeight;
    verts = verts.offset((*ds).firstVert as isize);
    numPoints = width * height;
    i = 0 as i32;
    while i < numPoints {
        j = 0 as i32;
        while j < 3 as i32 {
            points[i as usize].xyz[j as usize] = (*verts.offset(i as isize)).xyz[j as usize];
            points[i as usize].normal[j as usize] = (*verts.offset(i as isize)).normal[j as usize];
            j += 1
        }
        j = 0 as i32;
        while j < 2 as i32 {
            points[i as usize].st[j as usize] = (*verts.offset(i as isize)).st[j as usize];
            points[i as usize].lightmap[j as usize] =
                (*verts.offset(i as isize)).lightmap[j as usize];
            j += 1
        }
        R_ColorShiftLightingBytes(
            (*verts.offset(i as isize)).color.as_mut_ptr(),
            points[i as usize].color.as_mut_ptr(),
        );
        i += 1
    }
    // pre-tesseleate
    grid = crate::src::renderergl1::tr_curve::R_SubdividePatchToGrid(
        width,
        height,
        points.as_mut_ptr() as *mut crate::qfiles_h::drawVert_t,
    ) as *mut crate::tr_local_h::srfGridMesh_s;
    (*surf).data = grid as *mut crate::tr_local_h::surfaceType_t;
    // copy the level of detail origin, which is the center
    // of the group of all curves that must subdivide the same
    // to avoid cracking
    i = 0 as i32;
    while i < 3 as i32 {
        bounds[0 as i32 as usize][i as usize] = (*ds).lightmapVecs[0 as i32 as usize][i as usize];
        bounds[1 as i32 as usize][i as usize] = (*ds).lightmapVecs[1 as i32 as usize][i as usize];
        i += 1
    }
    bounds[1 as i32 as usize][0 as i32 as usize] =
        bounds[0 as i32 as usize][0 as i32 as usize] + bounds[1 as i32 as usize][0 as i32 as usize];
    bounds[1 as i32 as usize][1 as i32 as usize] =
        bounds[0 as i32 as usize][1 as i32 as usize] + bounds[1 as i32 as usize][1 as i32 as usize];
    bounds[1 as i32 as usize][2 as i32 as usize] =
        bounds[0 as i32 as usize][2 as i32 as usize] + bounds[1 as i32 as usize][2 as i32 as usize];
    (*grid).lodOrigin[0 as i32 as usize] = bounds[1 as i32 as usize][0 as i32 as usize] * 0.5f32;
    (*grid).lodOrigin[1 as i32 as usize] = bounds[1 as i32 as usize][1 as i32 as usize] * 0.5f32;
    (*grid).lodOrigin[2 as i32 as usize] = bounds[1 as i32 as usize][2 as i32 as usize] * 0.5f32;
    tmpVec[0 as i32 as usize] =
        bounds[0 as i32 as usize][0 as i32 as usize] - (*grid).lodOrigin[0 as i32 as usize];
    tmpVec[1 as i32 as usize] =
        bounds[0 as i32 as usize][1 as i32 as usize] - (*grid).lodOrigin[1 as i32 as usize];
    tmpVec[2 as i32 as usize] =
        bounds[0 as i32 as usize][2 as i32 as usize] - (*grid).lodOrigin[2 as i32 as usize];
    (*grid).lodRadius =
        VectorLength(tmpVec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
}
/*
===============
ParseTriSurf
===============
*/

unsafe extern "C" fn ParseTriSurf(
    mut ds: *mut crate::qfiles_h::dsurface_t,
    mut verts: *mut crate::qfiles_h::drawVert_t,
    mut surf: *mut crate::tr_local_h::msurface_t,
    mut indexes: *mut i32,
) {
    let mut tri: *mut crate::tr_local_h::srfTriangles_t =
        0 as *mut crate::tr_local_h::srfTriangles_t;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut numVerts: i32 = 0;
    let mut numIndexes: i32 = 0;
    // get fog volume
    (*surf).fogIndex = (*ds).fogNum + 1 as i32;
    // get shader
    (*surf).shader = ShaderForShaderNum((*ds).shaderNum, -(3 as i32));
    if (*crate::src::renderergl1::tr_init::r_singleShader).integer != 0
        && (*(*surf).shader).isSky as u64 == 0
    {
        (*surf).shader = crate::src::renderergl1::tr_main::tr.defaultShader
    }
    numVerts = (*ds).numVerts;
    numIndexes = (*ds).numIndexes;
    tri = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        (::std::mem::size_of::<crate::tr_local_h::srfTriangles_t>() as libc::c_ulong)
            .wrapping_add(
                (numVerts as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                    crate::qfiles_h::drawVert_t,
                >() as libc::c_ulong),
            )
            .wrapping_add(
                (numIndexes as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
            ) as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::srfTriangles_t;
    (*tri).surfaceType = crate::tr_local_h::SF_TRIANGLES;
    (*tri).numVerts = numVerts;
    (*tri).numIndexes = numIndexes;
    (*tri).verts = tri.offset(1 as i32 as isize) as *mut crate::qfiles_h::drawVert_t;
    (*tri).indexes = (*tri).verts.offset((*tri).numVerts as isize) as *mut i32;
    (*surf).data = tri as *mut crate::tr_local_h::surfaceType_t;
    // copy vertexes
    crate::src::qcommon::q_math::ClearBounds(
        (*tri).bounds[0 as i32 as usize].as_mut_ptr(),
        (*tri).bounds[1 as i32 as usize].as_mut_ptr(),
    );
    verts = verts.offset((*ds).firstVert as isize);
    i = 0 as i32;
    while i < numVerts {
        j = 0 as i32;
        while j < 3 as i32 {
            (*(*tri).verts.offset(i as isize)).xyz[j as usize] =
                (*verts.offset(i as isize)).xyz[j as usize];
            (*(*tri).verts.offset(i as isize)).normal[j as usize] =
                (*verts.offset(i as isize)).normal[j as usize];
            j += 1
        }
        crate::src::qcommon::q_math::AddPointToBounds(
            (*(*tri).verts.offset(i as isize)).xyz.as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            (*tri).bounds[0 as i32 as usize].as_mut_ptr(),
            (*tri).bounds[1 as i32 as usize].as_mut_ptr(),
        );
        j = 0 as i32;
        while j < 2 as i32 {
            (*(*tri).verts.offset(i as isize)).st[j as usize] =
                (*verts.offset(i as isize)).st[j as usize];
            (*(*tri).verts.offset(i as isize)).lightmap[j as usize] =
                (*verts.offset(i as isize)).lightmap[j as usize];
            j += 1
        }
        R_ColorShiftLightingBytes(
            (*verts.offset(i as isize)).color.as_mut_ptr(),
            (*(*tri).verts.offset(i as isize)).color.as_mut_ptr(),
        );
        i += 1
    }
    // copy indexes
    indexes = indexes.offset((*ds).firstIndex as isize);
    i = 0 as i32;
    while i < numIndexes {
        *(*tri).indexes.offset(i as isize) = *indexes.offset(i as isize);
        if *(*tri).indexes.offset(i as isize) < 0 as i32
            || *(*tri).indexes.offset(i as isize) >= numVerts
        {
            crate::src::renderergl1::tr_main::ri
                .Error
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"Bad index in triangle surface\x00" as *const u8 as *const libc::c_char,
            );
        }
        i += 1
    }
}
/*
===============
ParseFlare
===============
*/

unsafe extern "C" fn ParseFlare(
    mut ds: *mut crate::qfiles_h::dsurface_t,
    mut _verts: *mut crate::qfiles_h::drawVert_t,
    mut surf: *mut crate::tr_local_h::msurface_t,
    mut _indexes: *mut i32,
) {
    let mut flare: *mut crate::tr_local_h::srfFlare_t = 0 as *mut crate::tr_local_h::srfFlare_t;
    let mut i: i32 = 0;
    // get fog volume
    (*surf).fogIndex = (*ds).fogNum + 1 as i32;
    // get shader
    (*surf).shader = ShaderForShaderNum((*ds).shaderNum, -(3 as i32));
    if (*crate::src::renderergl1::tr_init::r_singleShader).integer != 0
        && (*(*surf).shader).isSky as u64 == 0
    {
        (*surf).shader = crate::src::renderergl1::tr_main::tr.defaultShader
    }
    flare = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        ::std::mem::size_of::<crate::tr_local_h::srfFlare_t>() as libc::c_ulong as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::srfFlare_t;
    (*flare).surfaceType = crate::tr_local_h::SF_FLARE;
    (*surf).data = flare as *mut crate::tr_local_h::surfaceType_t;
    i = 0 as i32;
    while i < 3 as i32 {
        (*flare).origin[i as usize] = (*ds).lightmapOrigin[i as usize];
        (*flare).color[i as usize] = (*ds).lightmapVecs[0 as i32 as usize][i as usize];
        (*flare).normal[i as usize] = (*ds).lightmapVecs[2 as i32 as usize][i as usize];
        i += 1
    }
}
/*
=================
R_MergedWidthPoints

returns true if there are grid points merged on a width edge
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_MergedWidthPoints(
    mut grid: *mut crate::tr_local_h::srfGridMesh_t,
    mut offset: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 1 as i32;
    while i < (*grid).width - 1 as i32 {
        j = i + 1 as i32;
        while j < (*grid).width - 1 as i32 {
            if !(crate::stdlib::fabs(
                ((*(*grid).verts.as_mut_ptr().offset((i + offset) as isize)).xyz[0 as i32 as usize]
                    - (*(*grid).verts.as_mut_ptr().offset((j + offset) as isize)).xyz
                        [0 as i32 as usize]) as f64,
            ) > 0.1f64)
            {
                if !(crate::stdlib::fabs(
                    ((*(*grid).verts.as_mut_ptr().offset((i + offset) as isize)).xyz
                        [1 as i32 as usize]
                        - (*(*grid).verts.as_mut_ptr().offset((j + offset) as isize)).xyz
                            [1 as i32 as usize]) as f64,
                ) > 0.1f64)
                {
                    if !(crate::stdlib::fabs(
                        ((*(*grid).verts.as_mut_ptr().offset((i + offset) as isize)).xyz
                            [2 as i32 as usize]
                            - (*(*grid).verts.as_mut_ptr().offset((j + offset) as isize)).xyz
                                [2 as i32 as usize]) as f64,
                    ) > 0.1f64)
                    {
                        return crate::src::qcommon::q_shared::qtrue as i32;
                    }
                }
            }
            j += 1
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
=================
R_MergedHeightPoints

returns true if there are grid points merged on a height edge
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_MergedHeightPoints(
    mut grid: *mut crate::tr_local_h::srfGridMesh_t,
    mut offset: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 1 as i32;
    while i < (*grid).height - 1 as i32 {
        j = i + 1 as i32;
        while j < (*grid).height - 1 as i32 {
            if !(crate::stdlib::fabs(
                ((*(*grid)
                    .verts
                    .as_mut_ptr()
                    .offset(((*grid).width * i + offset) as isize))
                .xyz[0 as i32 as usize]
                    - (*(*grid)
                        .verts
                        .as_mut_ptr()
                        .offset(((*grid).width * j + offset) as isize))
                    .xyz[0 as i32 as usize]) as f64,
            ) > 0.1f64)
            {
                if !(crate::stdlib::fabs(
                    ((*(*grid)
                        .verts
                        .as_mut_ptr()
                        .offset(((*grid).width * i + offset) as isize))
                    .xyz[1 as i32 as usize]
                        - (*(*grid)
                            .verts
                            .as_mut_ptr()
                            .offset(((*grid).width * j + offset) as isize))
                        .xyz[1 as i32 as usize]) as f64,
                ) > 0.1f64)
                {
                    if !(crate::stdlib::fabs(
                        ((*(*grid)
                            .verts
                            .as_mut_ptr()
                            .offset(((*grid).width * i + offset) as isize))
                        .xyz[2 as i32 as usize]
                            - (*(*grid)
                                .verts
                                .as_mut_ptr()
                                .offset(((*grid).width * j + offset) as isize))
                            .xyz[2 as i32 as usize]) as f64,
                    ) > 0.1f64)
                    {
                        return crate::src::qcommon::q_shared::qtrue as i32;
                    }
                }
            }
            j += 1
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
=================
R_FixSharedVertexLodError_r

NOTE: never sync LoD through grid edges with merged points!

FIXME: write generalized version that also avoids cracks between a patch and one that meets half way?
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_FixSharedVertexLodError_r(
    mut start: i32,
    mut grid1: *mut crate::tr_local_h::srfGridMesh_t,
) {
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut offset1: i32 = 0;
    let mut offset2: i32 = 0;
    let mut touch: i32 = 0;
    let mut grid2: *mut crate::tr_local_h::srfGridMesh_t =
        0 as *mut crate::tr_local_h::srfGridMesh_t;
    j = start;
    while j < s_worldData.numsurfaces {
        //
        grid2 = (*s_worldData.surfaces.offset(j as isize)).data
            as *mut crate::tr_local_h::srfGridMesh_t;
        // if this surface is not a grid
        if !((*grid2).surfaceType as u32 != crate::tr_local_h::SF_GRID as i32 as u32) {
            // if the LOD errors are already fixed for this patch
            if !((*grid2).lodFixed == 2 as i32) {
                // grids in the same LOD group should have the exact same lod radius
                if !((*grid1).lodRadius != (*grid2).lodRadius) {
                    // grids in the same LOD group should have the exact same lod origin
                    if !((*grid1).lodOrigin[0 as i32 as usize]
                        != (*grid2).lodOrigin[0 as i32 as usize])
                    {
                        if !((*grid1).lodOrigin[1 as i32 as usize]
                            != (*grid2).lodOrigin[1 as i32 as usize])
                        {
                            if !((*grid1).lodOrigin[2 as i32 as usize]
                                != (*grid2).lodOrigin[2 as i32 as usize])
                            {
                                //
                                touch = crate::src::qcommon::q_shared::qfalse as i32;
                                n = 0 as i32;
                                while n < 2 as i32 {
                                    //
                                    if n != 0 {
                                        offset1 = ((*grid1).height - 1 as i32) * (*grid1).width
                                    } else {
                                        offset1 = 0 as i32
                                    }
                                    if !(R_MergedWidthPoints(grid1, offset1) != 0) {
                                        k = 1 as i32;
                                        while k < (*grid1).width - 1 as i32 {
                                            m = 0 as i32;
                                            while m < 2 as i32 {
                                                if m != 0 {
                                                    offset2 = ((*grid2).height - 1 as i32)
                                                        * (*grid2).width
                                                } else {
                                                    offset2 = 0 as i32
                                                }
                                                if !(R_MergedWidthPoints(grid2, offset2) != 0) {
                                                    l = 1 as i32;
                                                    while l < (*grid2).width - 1 as i32 {
                                                        //
                                                        if !(crate::stdlib::fabs(
                                                            ((*(*grid1)
                                                                .verts
                                                                .as_mut_ptr()
                                                                .offset((k + offset1) as isize))
                                                            .xyz
                                                                [0 as i32 as usize]
                                                                - (*(*grid2)
                                                                    .verts
                                                                    .as_mut_ptr()
                                                                    .offset(
                                                                        (l + offset2) as isize,
                                                                    ))
                                                                .xyz
                                                                    [0 as i32 as usize])
                                                                as f64,
                                                        ) > 0.1f64)
                                                        {
                                                            if !(crate::stdlib::fabs(
                                                                ((*(*grid1)
                                                                    .verts
                                                                    .as_mut_ptr()
                                                                    .offset(
                                                                        (k + offset1) as isize,
                                                                    ))
                                                                .xyz
                                                                    [1 as i32 as usize]
                                                                    - (*(*grid2)
                                                                        .verts
                                                                        .as_mut_ptr()
                                                                        .offset(
                                                                            (l + offset2) as isize,
                                                                        ))
                                                                    .xyz
                                                                        [1 as i32 as usize])
                                                                    as f64,
                                                            ) > 0.1f64)
                                                            {
                                                                if !(crate::stdlib::fabs(
                                                                    ((*(*grid1)
                                                                        .verts
                                                                        .as_mut_ptr()
                                                                        .offset(
                                                                            (k + offset1) as isize,
                                                                        ))
                                                                    .xyz
                                                                        [2 as i32 as usize]
                                                                        - (*(*grid2)
                                                                            .verts
                                                                            .as_mut_ptr()
                                                                            .offset(
                                                                                (l + offset2)
                                                                                    as isize,
                                                                            ))
                                                                        .xyz
                                                                            [2 as i32 as usize])
                                                                        as f64,
                                                                ) > 0.1f64)
                                                                {
                                                                    // ok the points are equal and should have the same lod error
                                                                    *(*grid2)
                                                                        .widthLodError
                                                                        .offset(l as isize) =
                                                                        *(*grid1)
                                                                            .widthLodError
                                                                            .offset(k as isize);
                                                                    touch =
                                                                        crate::src::qcommon::q_shared::qtrue
                                                                            as
                                                                            i32
                                                                }
                                                            }
                                                        }
                                                        l += 1
                                                    }
                                                }
                                                m += 1
                                            }
                                            m = 0 as i32;
                                            while m < 2 as i32 {
                                                if m != 0 {
                                                    offset2 = (*grid2).width - 1 as i32
                                                } else {
                                                    offset2 = 0 as i32
                                                }
                                                if !(R_MergedHeightPoints(grid2, offset2) != 0) {
                                                    l = 1 as i32;
                                                    while l < (*grid2).height - 1 as i32 {
                                                        //
                                                        if !(crate::stdlib::fabs(
                                                            ((*(*grid1)
                                                                .verts
                                                                .as_mut_ptr()
                                                                .offset((k + offset1) as isize))
                                                            .xyz
                                                                [0 as i32 as usize]
                                                                - (*(*grid2)
                                                                    .verts
                                                                    .as_mut_ptr()
                                                                    .offset(
                                                                        ((*grid2).width * l
                                                                            + offset2)
                                                                            as isize,
                                                                    ))
                                                                .xyz
                                                                    [0 as i32 as usize])
                                                                as f64,
                                                        ) > 0.1f64)
                                                        {
                                                            if !(crate::stdlib::fabs(
                                                                ((*(*grid1)
                                                                    .verts
                                                                    .as_mut_ptr()
                                                                    .offset(
                                                                        (k + offset1) as isize,
                                                                    ))
                                                                .xyz
                                                                    [1 as i32 as usize]
                                                                    - (*(*grid2)
                                                                        .verts
                                                                        .as_mut_ptr()
                                                                        .offset(
                                                                            ((*grid2).width * l
                                                                                + offset2)
                                                                                as isize,
                                                                        ))
                                                                    .xyz
                                                                        [1 as i32 as usize])
                                                                    as f64,
                                                            ) > 0.1f64)
                                                            {
                                                                if !(crate::stdlib::fabs(
                                                                    ((*(*grid1)
                                                                        .verts
                                                                        .as_mut_ptr()
                                                                        .offset(
                                                                            (k + offset1) as isize,
                                                                        ))
                                                                    .xyz
                                                                        [2 as i32 as usize]
                                                                        - (*(*grid2)
                                                                            .verts
                                                                            .as_mut_ptr()
                                                                            .offset(
                                                                                ((*grid2).width * l
                                                                                    + offset2)
                                                                                    as isize,
                                                                            ))
                                                                        .xyz
                                                                            [2 as i32 as usize])
                                                                        as f64,
                                                                ) > 0.1f64)
                                                                {
                                                                    // ok the points are equal and should have the same lod error
                                                                    *(*grid2)
                                                                        .heightLodError
                                                                        .offset(l as isize) =
                                                                        *(*grid1)
                                                                            .widthLodError
                                                                            .offset(k as isize);
                                                                    touch =
                                                                        crate::src::qcommon::q_shared::qtrue
                                                                            as
                                                                            i32
                                                                }
                                                            }
                                                        }
                                                        l += 1
                                                    }
                                                }
                                                m += 1
                                            }
                                            k += 1
                                        }
                                    }
                                    n += 1
                                }
                                n = 0 as i32;
                                while n < 2 as i32 {
                                    //
                                    if n != 0 {
                                        offset1 = (*grid1).width - 1 as i32
                                    } else {
                                        offset1 = 0 as i32
                                    }
                                    if !(R_MergedHeightPoints(grid1, offset1) != 0) {
                                        k = 1 as i32;
                                        while k < (*grid1).height - 1 as i32 {
                                            m = 0 as i32;
                                            while m < 2 as i32 {
                                                if m != 0 {
                                                    offset2 = ((*grid2).height - 1 as i32)
                                                        * (*grid2).width
                                                } else {
                                                    offset2 = 0 as i32
                                                }
                                                if !(R_MergedWidthPoints(grid2, offset2) != 0) {
                                                    l = 1 as i32;
                                                    while l < (*grid2).width - 1 as i32 {
                                                        //
                                                        if !(crate::stdlib::fabs(
                                                            ((*(*grid1).verts.as_mut_ptr().offset(
                                                                ((*grid1).width * k + offset1)
                                                                    as isize,
                                                            ))
                                                            .xyz
                                                                [0 as i32 as usize]
                                                                - (*(*grid2)
                                                                    .verts
                                                                    .as_mut_ptr()
                                                                    .offset(
                                                                        (l + offset2) as isize,
                                                                    ))
                                                                .xyz
                                                                    [0 as i32 as usize])
                                                                as f64,
                                                        ) > 0.1f64)
                                                        {
                                                            if !(crate::stdlib::fabs(
                                                                ((*(*grid1)
                                                                    .verts
                                                                    .as_mut_ptr()
                                                                    .offset(
                                                                        ((*grid1).width * k
                                                                            + offset1)
                                                                            as isize,
                                                                    ))
                                                                .xyz
                                                                    [1 as i32 as usize]
                                                                    - (*(*grid2)
                                                                        .verts
                                                                        .as_mut_ptr()
                                                                        .offset(
                                                                            (l + offset2) as isize,
                                                                        ))
                                                                    .xyz
                                                                        [1 as i32 as usize])
                                                                    as f64,
                                                            ) > 0.1f64)
                                                            {
                                                                if !(crate::stdlib::fabs(
                                                                    ((*(*grid1)
                                                                        .verts
                                                                        .as_mut_ptr()
                                                                        .offset(
                                                                            ((*grid1).width * k
                                                                                + offset1)
                                                                                as isize,
                                                                        ))
                                                                    .xyz
                                                                        [2 as i32 as usize]
                                                                        - (*(*grid2)
                                                                            .verts
                                                                            .as_mut_ptr()
                                                                            .offset(
                                                                                (l + offset2)
                                                                                    as isize,
                                                                            ))
                                                                        .xyz
                                                                            [2 as i32 as usize])
                                                                        as f64,
                                                                ) > 0.1f64)
                                                                {
                                                                    // ok the points are equal and should have the same lod error
                                                                    *(*grid2)
                                                                        .widthLodError
                                                                        .offset(l as isize) =
                                                                        *(*grid1)
                                                                            .heightLodError
                                                                            .offset(k as isize);
                                                                    touch =
                                                                        crate::src::qcommon::q_shared::qtrue
                                                                            as
                                                                            i32
                                                                }
                                                            }
                                                        }
                                                        l += 1
                                                    }
                                                }
                                                m += 1
                                            }
                                            m = 0 as i32;
                                            while m < 2 as i32 {
                                                if m != 0 {
                                                    offset2 = (*grid2).width - 1 as i32
                                                } else {
                                                    offset2 = 0 as i32
                                                }
                                                if !(R_MergedHeightPoints(grid2, offset2) != 0) {
                                                    l = 1 as i32;
                                                    while l < (*grid2).height - 1 as i32 {
                                                        //
                                                        if !(crate::stdlib::fabs(
                                                            ((*(*grid1).verts.as_mut_ptr().offset(
                                                                ((*grid1).width * k + offset1)
                                                                    as isize,
                                                            ))
                                                            .xyz
                                                                [0 as i32 as usize]
                                                                - (*(*grid2)
                                                                    .verts
                                                                    .as_mut_ptr()
                                                                    .offset(
                                                                        ((*grid2).width * l
                                                                            + offset2)
                                                                            as isize,
                                                                    ))
                                                                .xyz
                                                                    [0 as i32 as usize])
                                                                as f64,
                                                        ) > 0.1f64)
                                                        {
                                                            if !(crate::stdlib::fabs(
                                                                ((*(*grid1)
                                                                    .verts
                                                                    .as_mut_ptr()
                                                                    .offset(
                                                                        ((*grid1).width * k
                                                                            + offset1)
                                                                            as isize,
                                                                    ))
                                                                .xyz
                                                                    [1 as i32 as usize]
                                                                    - (*(*grid2)
                                                                        .verts
                                                                        .as_mut_ptr()
                                                                        .offset(
                                                                            ((*grid2).width * l
                                                                                + offset2)
                                                                                as isize,
                                                                        ))
                                                                    .xyz
                                                                        [1 as i32 as usize])
                                                                    as f64,
                                                            ) > 0.1f64)
                                                            {
                                                                if !(crate::stdlib::fabs(
                                                                    ((*(*grid1)
                                                                        .verts
                                                                        .as_mut_ptr()
                                                                        .offset(
                                                                            ((*grid1).width * k
                                                                                + offset1)
                                                                                as isize,
                                                                        ))
                                                                    .xyz
                                                                        [2 as i32 as usize]
                                                                        - (*(*grid2)
                                                                            .verts
                                                                            .as_mut_ptr()
                                                                            .offset(
                                                                                ((*grid2).width * l
                                                                                    + offset2)
                                                                                    as isize,
                                                                            ))
                                                                        .xyz
                                                                            [2 as i32 as usize])
                                                                        as f64,
                                                                ) > 0.1f64)
                                                                {
                                                                    // ok the points are equal and should have the same lod error
                                                                    *(*grid2)
                                                                        .heightLodError
                                                                        .offset(l as isize) =
                                                                        *(*grid1)
                                                                            .heightLodError
                                                                            .offset(k as isize);
                                                                    touch =
                                                                        crate::src::qcommon::q_shared::qtrue
                                                                            as
                                                                            i32
                                                                }
                                                            }
                                                        }
                                                        l += 1
                                                    }
                                                }
                                                m += 1
                                            }
                                            k += 1
                                        }
                                    }
                                    n += 1
                                }
                                if touch != 0 {
                                    (*grid2).lodFixed = 2 as i32;
                                    R_FixSharedVertexLodError_r(start, grid2);
                                    //NOTE: this would be correct but makes things really slow
                                    //grid2->lodFixed = 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        j += 1
    }
}
/*
=================
R_FixSharedVertexLodError

This function assumes that all patches in one group are nicely stitched together for the highest LoD.
If this is not the case this function will still do its job but won't fix the highest LoD cracks.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_FixSharedVertexLodError() {
    let mut i: i32 = 0;
    let mut grid1: *mut crate::tr_local_h::srfGridMesh_t =
        0 as *mut crate::tr_local_h::srfGridMesh_t;
    i = 0 as i32;
    while i < s_worldData.numsurfaces {
        //
        grid1 = (*s_worldData.surfaces.offset(i as isize)).data
            as *mut crate::tr_local_h::srfGridMesh_t;
        // if this surface is not a grid
        if !((*grid1).surfaceType as u32 != crate::tr_local_h::SF_GRID as i32 as u32) {
            //
            if !((*grid1).lodFixed != 0) {
                //
                (*grid1).lodFixed = 2 as i32;
                // recursively fix other patches in the same LOD group
                R_FixSharedVertexLodError_r(i + 1 as i32, grid1);
            }
        }
        i += 1
    }
}
/*
===============
R_StitchPatches
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_StitchPatches(mut grid1num: i32, mut grid2num: i32) -> i32 {
    let mut v1: *mut f32 = 0 as *mut f32;
    let mut v2: *mut f32 = 0 as *mut f32;
    let mut grid1: *mut crate::tr_local_h::srfGridMesh_t =
        0 as *mut crate::tr_local_h::srfGridMesh_t;
    let mut grid2: *mut crate::tr_local_h::srfGridMesh_t =
        0 as *mut crate::tr_local_h::srfGridMesh_t;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut offset1: i32 = 0;
    let mut offset2: i32 = 0;
    let mut row: i32 = 0;
    let mut column: i32 = 0;
    grid1 = (*s_worldData.surfaces.offset(grid1num as isize)).data
        as *mut crate::tr_local_h::srfGridMesh_t;
    grid2 = (*s_worldData.surfaces.offset(grid2num as isize)).data
        as *mut crate::tr_local_h::srfGridMesh_t;
    n = 0 as i32;
    while n < 2 as i32 {
        //
        if n != 0 {
            offset1 = ((*grid1).height - 1 as i32) * (*grid1).width
        } else {
            offset1 = 0 as i32
        }
        if !(R_MergedWidthPoints(grid1, offset1) != 0) {
            k = 0 as i32;
            while k < (*grid1).width - 2 as i32 {
                m = 0 as i32;
                while m < 2 as i32 {
                    if (*grid2).width >= 65 as i32 {
                        break;
                    }
                    if m != 0 {
                        offset2 = ((*grid2).height - 1 as i32) * (*grid2).width
                    } else {
                        offset2 = 0 as i32
                    }
                    l = 0 as i32;
                    while l < (*grid2).width - 1 as i32 {
                        //
                        v1 = (*(*grid1).verts.as_mut_ptr().offset((k + offset1) as isize))
                            .xyz
                            .as_mut_ptr();
                        v2 = (*(*grid2).verts.as_mut_ptr().offset((l + offset2) as isize))
                            .xyz
                            .as_mut_ptr();
                        if !(crate::stdlib::fabs(
                            (*v1.offset(0 as i32 as isize) - *v2.offset(0 as i32 as isize)) as f64,
                        ) > 0.1f64)
                        {
                            if !(crate::stdlib::fabs(
                                (*v1.offset(1 as i32 as isize) - *v2.offset(1 as i32 as isize))
                                    as f64,
                            ) > 0.1f64)
                            {
                                if !(crate::stdlib::fabs(
                                    (*v1.offset(2 as i32 as isize) - *v2.offset(2 as i32 as isize))
                                        as f64,
                                ) > 0.1f64)
                                {
                                    v1 = (*(*grid1)
                                        .verts
                                        .as_mut_ptr()
                                        .offset((k + 2 as i32 + offset1) as isize))
                                    .xyz
                                    .as_mut_ptr();
                                    v2 = (*(*grid2)
                                        .verts
                                        .as_mut_ptr()
                                        .offset((l + 1 as i32 + offset2) as isize))
                                    .xyz
                                    .as_mut_ptr();
                                    if !(crate::stdlib::fabs(
                                        (*v1.offset(0 as i32 as isize)
                                            - *v2.offset(0 as i32 as isize))
                                            as f64,
                                    ) > 0.1f64)
                                    {
                                        if !(crate::stdlib::fabs(
                                            (*v1.offset(1 as i32 as isize)
                                                - *v2.offset(1 as i32 as isize))
                                                as f64,
                                        ) > 0.1f64)
                                        {
                                            if !(crate::stdlib::fabs(
                                                (*v1.offset(2 as i32 as isize)
                                                    - *v2.offset(2 as i32 as isize))
                                                    as f64,
                                            ) > 0.1f64)
                                            {
                                                //
                                                v1 = (*(*grid2)
                                                    .verts
                                                    .as_mut_ptr()
                                                    .offset((l + offset2) as isize))
                                                .xyz
                                                .as_mut_ptr();
                                                v2 = (*(*grid2)
                                                    .verts
                                                    .as_mut_ptr()
                                                    .offset((l + 1 as i32 + offset2) as isize))
                                                .xyz
                                                .as_mut_ptr();
                                                if !(crate::stdlib::fabs(
                                                    (*v1.offset(0 as i32 as isize)
                                                        - *v2.offset(0 as i32 as isize))
                                                        as f64,
                                                ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(1 as i32 as isize)
                                                            - *v2.offset(1 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(2 as i32 as isize)
                                                            - *v2.offset(2 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64)
                                                {
                                                    //
                                                    //ri.Printf( PRINT_ALL, "found highest LoD crack between two patches\n" );
                                                    // insert column into grid2 right after after column l
                                                    if m != 0 {
                                                        row = (*grid2).height - 1 as i32
                                                    } else {
                                                        row = 0 as i32
                                                    }
                                                    grid2 =
                                                        
                                                        crate::src::renderergl1::tr_curve::R_GridInsertColumn(grid2 as *mut crate::tr_local_h::srfGridMesh_s,
                                                                           l +
                                                                               1
                                                                                   as
                                                                                   i32,
                                                                           row,
                                                                           (*(*grid1).verts.as_mut_ptr().offset((k
                                                                                                                     +
                                                                                                                     1
                                                                                                                         as
                                                                                                                         i32
                                                                                                                     +
                                                                                                                     offset1)
                                                                                                                    as
                                                                                                                    isize)).xyz.as_mut_ptr(),
                                                                           *(*grid1).widthLodError.offset((k
                                                                                                               +
                                                                                                               1
                                                                                                                   as
                                                                                                                   i32)
                                                                                                              as
                                                                                                              isize))
    as *mut crate::tr_local_h::srfGridMesh_s;
                                                    (*grid2).lodStitched =
                                                        crate::src::qcommon::q_shared::qfalse
                                                            as i32;
                                                    let ref mut fresh1 = (*s_worldData
                                                        .surfaces
                                                        .offset(grid2num as isize))
                                                    .data;
                                                    *fresh1 = grid2 as *mut libc::c_void
                                                        as *mut crate::tr_local_h::surfaceType_t;
                                                    return crate::src::qcommon::q_shared::qtrue
                                                        as i32;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        l += 1
                    }
                    m += 1
                }
                m = 0 as i32;
                while m < 2 as i32 {
                    if (*grid2).height >= 65 as i32 {
                        break;
                    }
                    if m != 0 {
                        offset2 = (*grid2).width - 1 as i32
                    } else {
                        offset2 = 0 as i32
                    }
                    l = 0 as i32;
                    while l < (*grid2).height - 1 as i32 {
                        //
                        v1 = (*(*grid1).verts.as_mut_ptr().offset((k + offset1) as isize))
                            .xyz
                            .as_mut_ptr();
                        v2 = (*(*grid2)
                            .verts
                            .as_mut_ptr()
                            .offset(((*grid2).width * l + offset2) as isize))
                        .xyz
                        .as_mut_ptr();
                        if !(crate::stdlib::fabs(
                            (*v1.offset(0 as i32 as isize) - *v2.offset(0 as i32 as isize)) as f64,
                        ) > 0.1f64)
                        {
                            if !(crate::stdlib::fabs(
                                (*v1.offset(1 as i32 as isize) - *v2.offset(1 as i32 as isize))
                                    as f64,
                            ) > 0.1f64)
                            {
                                if !(crate::stdlib::fabs(
                                    (*v1.offset(2 as i32 as isize) - *v2.offset(2 as i32 as isize))
                                        as f64,
                                ) > 0.1f64)
                                {
                                    v1 = (*(*grid1)
                                        .verts
                                        .as_mut_ptr()
                                        .offset((k + 2 as i32 + offset1) as isize))
                                    .xyz
                                    .as_mut_ptr();
                                    v2 = (*(*grid2).verts.as_mut_ptr().offset(
                                        ((*grid2).width * (l + 1 as i32) + offset2) as isize,
                                    ))
                                    .xyz
                                    .as_mut_ptr();
                                    if !(crate::stdlib::fabs(
                                        (*v1.offset(0 as i32 as isize)
                                            - *v2.offset(0 as i32 as isize))
                                            as f64,
                                    ) > 0.1f64)
                                    {
                                        if !(crate::stdlib::fabs(
                                            (*v1.offset(1 as i32 as isize)
                                                - *v2.offset(1 as i32 as isize))
                                                as f64,
                                        ) > 0.1f64)
                                        {
                                            if !(crate::stdlib::fabs(
                                                (*v1.offset(2 as i32 as isize)
                                                    - *v2.offset(2 as i32 as isize))
                                                    as f64,
                                            ) > 0.1f64)
                                            {
                                                //
                                                v1 = (*(*grid2).verts.as_mut_ptr().offset(
                                                    ((*grid2).width * l + offset2) as isize,
                                                ))
                                                .xyz
                                                .as_mut_ptr();
                                                v2 = (*(*grid2).verts.as_mut_ptr().offset(
                                                    ((*grid2).width * (l + 1 as i32) + offset2)
                                                        as isize,
                                                ))
                                                .xyz
                                                .as_mut_ptr();
                                                if !(crate::stdlib::fabs(
                                                    (*v1.offset(0 as i32 as isize)
                                                        - *v2.offset(0 as i32 as isize))
                                                        as f64,
                                                ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(1 as i32 as isize)
                                                            - *v2.offset(1 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(2 as i32 as isize)
                                                            - *v2.offset(2 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64)
                                                {
                                                    //
                                                    //ri.Printf( PRINT_ALL, "found highest LoD crack between two patches\n" );
                                                    // insert row into grid2 right after after row l
                                                    if m != 0 {
                                                        column = (*grid2).width - 1 as i32
                                                    } else {
                                                        column = 0 as i32
                                                    }
                                                    grid2 =
                                                        
                                                        crate::src::renderergl1::tr_curve::R_GridInsertRow(grid2 as *mut crate::tr_local_h::srfGridMesh_s,
                                                                        l +
                                                                            1
                                                                                as
                                                                                i32,
                                                                        column,
                                                                        (*(*grid1).verts.as_mut_ptr().offset((k
                                                                                                                  +
                                                                                                                  1
                                                                                                                      as
                                                                                                                      i32
                                                                                                                  +
                                                                                                                  offset1)
                                                                                                                 as
                                                                                                                 isize)).xyz.as_mut_ptr(),
                                                                        *(*grid1).widthLodError.offset((k
                                                                                                            +
                                                                                                            1
                                                                                                                as
                                                                                                                i32)
                                                                                                           as
                                                                                                           isize))
    as *mut crate::tr_local_h::srfGridMesh_s;
                                                    (*grid2).lodStitched =
                                                        crate::src::qcommon::q_shared::qfalse
                                                            as i32;
                                                    let ref mut fresh2 = (*s_worldData
                                                        .surfaces
                                                        .offset(grid2num as isize))
                                                    .data;
                                                    *fresh2 = grid2 as *mut libc::c_void
                                                        as *mut crate::tr_local_h::surfaceType_t;
                                                    return crate::src::qcommon::q_shared::qtrue
                                                        as i32;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        l += 1
                    }
                    m += 1
                }
                k += 2 as i32
            }
        }
        n += 1
    }
    n = 0 as i32;
    while n < 2 as i32 {
        //
        if n != 0 {
            offset1 = (*grid1).width - 1 as i32
        } else {
            offset1 = 0 as i32
        }
        if !(R_MergedHeightPoints(grid1, offset1) != 0) {
            k = 0 as i32;
            while k < (*grid1).height - 2 as i32 {
                m = 0 as i32;
                while m < 2 as i32 {
                    if (*grid2).width >= 65 as i32 {
                        break;
                    }
                    if m != 0 {
                        offset2 = ((*grid2).height - 1 as i32) * (*grid2).width
                    } else {
                        offset2 = 0 as i32
                    }
                    l = 0 as i32;
                    while l < (*grid2).width - 1 as i32 {
                        //
                        v1 = (*(*grid1)
                            .verts
                            .as_mut_ptr()
                            .offset(((*grid1).width * k + offset1) as isize))
                        .xyz
                        .as_mut_ptr();
                        v2 = (*(*grid2).verts.as_mut_ptr().offset((l + offset2) as isize))
                            .xyz
                            .as_mut_ptr();
                        if !(crate::stdlib::fabs(
                            (*v1.offset(0 as i32 as isize) - *v2.offset(0 as i32 as isize)) as f64,
                        ) > 0.1f64)
                        {
                            if !(crate::stdlib::fabs(
                                (*v1.offset(1 as i32 as isize) - *v2.offset(1 as i32 as isize))
                                    as f64,
                            ) > 0.1f64)
                            {
                                if !(crate::stdlib::fabs(
                                    (*v1.offset(2 as i32 as isize) - *v2.offset(2 as i32 as isize))
                                        as f64,
                                ) > 0.1f64)
                                {
                                    v1 = (*(*grid1).verts.as_mut_ptr().offset(
                                        ((*grid1).width * (k + 2 as i32) + offset1) as isize,
                                    ))
                                    .xyz
                                    .as_mut_ptr();
                                    v2 = (*(*grid2)
                                        .verts
                                        .as_mut_ptr()
                                        .offset((l + 1 as i32 + offset2) as isize))
                                    .xyz
                                    .as_mut_ptr();
                                    if !(crate::stdlib::fabs(
                                        (*v1.offset(0 as i32 as isize)
                                            - *v2.offset(0 as i32 as isize))
                                            as f64,
                                    ) > 0.1f64)
                                    {
                                        if !(crate::stdlib::fabs(
                                            (*v1.offset(1 as i32 as isize)
                                                - *v2.offset(1 as i32 as isize))
                                                as f64,
                                        ) > 0.1f64)
                                        {
                                            if !(crate::stdlib::fabs(
                                                (*v1.offset(2 as i32 as isize)
                                                    - *v2.offset(2 as i32 as isize))
                                                    as f64,
                                            ) > 0.1f64)
                                            {
                                                //
                                                v1 = (*(*grid2)
                                                    .verts
                                                    .as_mut_ptr()
                                                    .offset((l + offset2) as isize))
                                                .xyz
                                                .as_mut_ptr();
                                                v2 = (*(*grid2)
                                                    .verts
                                                    .as_mut_ptr()
                                                    .offset((l + 1 as i32 + offset2) as isize))
                                                .xyz
                                                .as_mut_ptr();
                                                if !(crate::stdlib::fabs(
                                                    (*v1.offset(0 as i32 as isize)
                                                        - *v2.offset(0 as i32 as isize))
                                                        as f64,
                                                ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(1 as i32 as isize)
                                                            - *v2.offset(1 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(2 as i32 as isize)
                                                            - *v2.offset(2 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64)
                                                {
                                                    //
                                                    //ri.Printf( PRINT_ALL, "found highest LoD crack between two patches\n" );
                                                    // insert column into grid2 right after after column l
                                                    if m != 0 {
                                                        row = (*grid2).height - 1 as i32
                                                    } else {
                                                        row = 0 as i32
                                                    }
                                                    grid2 =
                                                        
                                                        crate::src::renderergl1::tr_curve::R_GridInsertColumn(grid2 as *mut crate::tr_local_h::srfGridMesh_s,
                                                                           l +
                                                                               1
                                                                                   as
                                                                                   i32,
                                                                           row,
                                                                           (*(*grid1).verts.as_mut_ptr().offset(((*grid1).width
                                                                                                                     *
                                                                                                                     (k
                                                                                                                          +
                                                                                                                          1
                                                                                                                              as
                                                                                                                              i32)
                                                                                                                     +
                                                                                                                     offset1)
                                                                                                                    as
                                                                                                                    isize)).xyz.as_mut_ptr(),
                                                                           *(*grid1).heightLodError.offset((k
                                                                                                                +
                                                                                                                1
                                                                                                                    as
                                                                                                                    i32)
                                                                                                               as
                                                                                                               isize))
    as *mut crate::tr_local_h::srfGridMesh_s;
                                                    (*grid2).lodStitched =
                                                        crate::src::qcommon::q_shared::qfalse
                                                            as i32;
                                                    let ref mut fresh3 = (*s_worldData
                                                        .surfaces
                                                        .offset(grid2num as isize))
                                                    .data;
                                                    *fresh3 = grid2 as *mut libc::c_void
                                                        as *mut crate::tr_local_h::surfaceType_t;
                                                    return crate::src::qcommon::q_shared::qtrue
                                                        as i32;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        l += 1
                    }
                    m += 1
                }
                m = 0 as i32;
                while m < 2 as i32 {
                    if (*grid2).height >= 65 as i32 {
                        break;
                    }
                    if m != 0 {
                        offset2 = (*grid2).width - 1 as i32
                    } else {
                        offset2 = 0 as i32
                    }
                    l = 0 as i32;
                    while l < (*grid2).height - 1 as i32 {
                        //
                        v1 = (*(*grid1)
                            .verts
                            .as_mut_ptr()
                            .offset(((*grid1).width * k + offset1) as isize))
                        .xyz
                        .as_mut_ptr();
                        v2 = (*(*grid2)
                            .verts
                            .as_mut_ptr()
                            .offset(((*grid2).width * l + offset2) as isize))
                        .xyz
                        .as_mut_ptr();
                        if !(crate::stdlib::fabs(
                            (*v1.offset(0 as i32 as isize) - *v2.offset(0 as i32 as isize)) as f64,
                        ) > 0.1f64)
                        {
                            if !(crate::stdlib::fabs(
                                (*v1.offset(1 as i32 as isize) - *v2.offset(1 as i32 as isize))
                                    as f64,
                            ) > 0.1f64)
                            {
                                if !(crate::stdlib::fabs(
                                    (*v1.offset(2 as i32 as isize) - *v2.offset(2 as i32 as isize))
                                        as f64,
                                ) > 0.1f64)
                                {
                                    v1 = (*(*grid1).verts.as_mut_ptr().offset(
                                        ((*grid1).width * (k + 2 as i32) + offset1) as isize,
                                    ))
                                    .xyz
                                    .as_mut_ptr();
                                    v2 = (*(*grid2).verts.as_mut_ptr().offset(
                                        ((*grid2).width * (l + 1 as i32) + offset2) as isize,
                                    ))
                                    .xyz
                                    .as_mut_ptr();
                                    if !(crate::stdlib::fabs(
                                        (*v1.offset(0 as i32 as isize)
                                            - *v2.offset(0 as i32 as isize))
                                            as f64,
                                    ) > 0.1f64)
                                    {
                                        if !(crate::stdlib::fabs(
                                            (*v1.offset(1 as i32 as isize)
                                                - *v2.offset(1 as i32 as isize))
                                                as f64,
                                        ) > 0.1f64)
                                        {
                                            if !(crate::stdlib::fabs(
                                                (*v1.offset(2 as i32 as isize)
                                                    - *v2.offset(2 as i32 as isize))
                                                    as f64,
                                            ) > 0.1f64)
                                            {
                                                //
                                                v1 = (*(*grid2).verts.as_mut_ptr().offset(
                                                    ((*grid2).width * l + offset2) as isize,
                                                ))
                                                .xyz
                                                .as_mut_ptr();
                                                v2 = (*(*grid2).verts.as_mut_ptr().offset(
                                                    ((*grid2).width * (l + 1 as i32) + offset2)
                                                        as isize,
                                                ))
                                                .xyz
                                                .as_mut_ptr();
                                                if !(crate::stdlib::fabs(
                                                    (*v1.offset(0 as i32 as isize)
                                                        - *v2.offset(0 as i32 as isize))
                                                        as f64,
                                                ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(1 as i32 as isize)
                                                            - *v2.offset(1 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(2 as i32 as isize)
                                                            - *v2.offset(2 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64)
                                                {
                                                    //
                                                    //ri.Printf( PRINT_ALL, "found highest LoD crack between two patches\n" );
                                                    // insert row into grid2 right after after row l
                                                    if m != 0 {
                                                        column = (*grid2).width - 1 as i32
                                                    } else {
                                                        column = 0 as i32
                                                    }
                                                    grid2 =
                                                        
                                                        crate::src::renderergl1::tr_curve::R_GridInsertRow(grid2 as *mut crate::tr_local_h::srfGridMesh_s,
                                                                        l +
                                                                            1
                                                                                as
                                                                                i32,
                                                                        column,
                                                                        (*(*grid1).verts.as_mut_ptr().offset(((*grid1).width
                                                                                                                  *
                                                                                                                  (k
                                                                                                                       +
                                                                                                                       1
                                                                                                                           as
                                                                                                                           i32)
                                                                                                                  +
                                                                                                                  offset1)
                                                                                                                 as
                                                                                                                 isize)).xyz.as_mut_ptr(),
                                                                        *(*grid1).heightLodError.offset((k
                                                                                                             +
                                                                                                             1
                                                                                                                 as
                                                                                                                 i32)
                                                                                                            as
                                                                                                            isize))
    as *mut crate::tr_local_h::srfGridMesh_s;
                                                    (*grid2).lodStitched =
                                                        crate::src::qcommon::q_shared::qfalse
                                                            as i32;
                                                    let ref mut fresh4 = (*s_worldData
                                                        .surfaces
                                                        .offset(grid2num as isize))
                                                    .data;
                                                    *fresh4 = grid2 as *mut libc::c_void
                                                        as *mut crate::tr_local_h::surfaceType_t;
                                                    return crate::src::qcommon::q_shared::qtrue
                                                        as i32;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        l += 1
                    }
                    m += 1
                }
                k += 2 as i32
            }
        }
        n += 1
    }
    n = 0 as i32;
    while n < 2 as i32 {
        //
        if n != 0 {
            offset1 = ((*grid1).height - 1 as i32) * (*grid1).width
        } else {
            offset1 = 0 as i32
        }
        if !(R_MergedWidthPoints(grid1, offset1) != 0) {
            k = (*grid1).width - 1 as i32;
            while k > 1 as i32 {
                m = 0 as i32;
                while m < 2 as i32 {
                    if grid2.is_null() || (*grid2).width >= 65 as i32 {
                        break;
                    }
                    if m != 0 {
                        offset2 = ((*grid2).height - 1 as i32) * (*grid2).width
                    } else {
                        offset2 = 0 as i32
                    }
                    l = 0 as i32;
                    while l < (*grid2).width - 1 as i32 {
                        //
                        v1 = (*(*grid1).verts.as_mut_ptr().offset((k + offset1) as isize))
                            .xyz
                            .as_mut_ptr();
                        v2 = (*(*grid2).verts.as_mut_ptr().offset((l + offset2) as isize))
                            .xyz
                            .as_mut_ptr();
                        if !(crate::stdlib::fabs(
                            (*v1.offset(0 as i32 as isize) - *v2.offset(0 as i32 as isize)) as f64,
                        ) > 0.1f64)
                        {
                            if !(crate::stdlib::fabs(
                                (*v1.offset(1 as i32 as isize) - *v2.offset(1 as i32 as isize))
                                    as f64,
                            ) > 0.1f64)
                            {
                                if !(crate::stdlib::fabs(
                                    (*v1.offset(2 as i32 as isize) - *v2.offset(2 as i32 as isize))
                                        as f64,
                                ) > 0.1f64)
                                {
                                    v1 = (*(*grid1)
                                        .verts
                                        .as_mut_ptr()
                                        .offset((k - 2 as i32 + offset1) as isize))
                                    .xyz
                                    .as_mut_ptr();
                                    v2 = (*(*grid2)
                                        .verts
                                        .as_mut_ptr()
                                        .offset((l + 1 as i32 + offset2) as isize))
                                    .xyz
                                    .as_mut_ptr();
                                    if !(crate::stdlib::fabs(
                                        (*v1.offset(0 as i32 as isize)
                                            - *v2.offset(0 as i32 as isize))
                                            as f64,
                                    ) > 0.1f64)
                                    {
                                        if !(crate::stdlib::fabs(
                                            (*v1.offset(1 as i32 as isize)
                                                - *v2.offset(1 as i32 as isize))
                                                as f64,
                                        ) > 0.1f64)
                                        {
                                            if !(crate::stdlib::fabs(
                                                (*v1.offset(2 as i32 as isize)
                                                    - *v2.offset(2 as i32 as isize))
                                                    as f64,
                                            ) > 0.1f64)
                                            {
                                                //
                                                v1 = (*(*grid2)
                                                    .verts
                                                    .as_mut_ptr()
                                                    .offset((l + offset2) as isize))
                                                .xyz
                                                .as_mut_ptr();
                                                v2 = (*(*grid2)
                                                    .verts
                                                    .as_mut_ptr()
                                                    .offset((l + 1 as i32 + offset2) as isize))
                                                .xyz
                                                .as_mut_ptr();
                                                if !(crate::stdlib::fabs(
                                                    (*v1.offset(0 as i32 as isize)
                                                        - *v2.offset(0 as i32 as isize))
                                                        as f64,
                                                ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(1 as i32 as isize)
                                                            - *v2.offset(1 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(2 as i32 as isize)
                                                            - *v2.offset(2 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64)
                                                {
                                                    //
                                                    //ri.Printf( PRINT_ALL, "found highest LoD crack between two patches\n" );
                                                    // insert column into grid2 right after after column l
                                                    if m != 0 {
                                                        row = (*grid2).height - 1 as i32
                                                    } else {
                                                        row = 0 as i32
                                                    }
                                                    grid2 =
                                                        
                                                        crate::src::renderergl1::tr_curve::R_GridInsertColumn(grid2 as *mut crate::tr_local_h::srfGridMesh_s,
                                                                           l +
                                                                               1
                                                                                   as
                                                                                   i32,
                                                                           row,
                                                                           (*(*grid1).verts.as_mut_ptr().offset((k
                                                                                                                     -
                                                                                                                     1
                                                                                                                         as
                                                                                                                         i32
                                                                                                                     +
                                                                                                                     offset1)
                                                                                                                    as
                                                                                                                    isize)).xyz.as_mut_ptr(),
                                                                           *(*grid1).widthLodError.offset((k
                                                                                                               +
                                                                                                               1
                                                                                                                   as
                                                                                                                   i32)
                                                                                                              as
                                                                                                              isize))
    as *mut crate::tr_local_h::srfGridMesh_s;
                                                    (*grid2).lodStitched =
                                                        crate::src::qcommon::q_shared::qfalse
                                                            as i32;
                                                    let ref mut fresh5 = (*s_worldData
                                                        .surfaces
                                                        .offset(grid2num as isize))
                                                    .data;
                                                    *fresh5 = grid2 as *mut libc::c_void
                                                        as *mut crate::tr_local_h::surfaceType_t;
                                                    return crate::src::qcommon::q_shared::qtrue
                                                        as i32;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        l += 1
                    }
                    m += 1
                }
                m = 0 as i32;
                while m < 2 as i32 {
                    if grid2.is_null() || (*grid2).height >= 65 as i32 {
                        break;
                    }
                    if m != 0 {
                        offset2 = (*grid2).width - 1 as i32
                    } else {
                        offset2 = 0 as i32
                    }
                    l = 0 as i32;
                    while l < (*grid2).height - 1 as i32 {
                        //
                        v1 = (*(*grid1).verts.as_mut_ptr().offset((k + offset1) as isize))
                            .xyz
                            .as_mut_ptr();
                        v2 = (*(*grid2)
                            .verts
                            .as_mut_ptr()
                            .offset(((*grid2).width * l + offset2) as isize))
                        .xyz
                        .as_mut_ptr();
                        if !(crate::stdlib::fabs(
                            (*v1.offset(0 as i32 as isize) - *v2.offset(0 as i32 as isize)) as f64,
                        ) > 0.1f64)
                        {
                            if !(crate::stdlib::fabs(
                                (*v1.offset(1 as i32 as isize) - *v2.offset(1 as i32 as isize))
                                    as f64,
                            ) > 0.1f64)
                            {
                                if !(crate::stdlib::fabs(
                                    (*v1.offset(2 as i32 as isize) - *v2.offset(2 as i32 as isize))
                                        as f64,
                                ) > 0.1f64)
                                {
                                    v1 = (*(*grid1)
                                        .verts
                                        .as_mut_ptr()
                                        .offset((k - 2 as i32 + offset1) as isize))
                                    .xyz
                                    .as_mut_ptr();
                                    v2 = (*(*grid2).verts.as_mut_ptr().offset(
                                        ((*grid2).width * (l + 1 as i32) + offset2) as isize,
                                    ))
                                    .xyz
                                    .as_mut_ptr();
                                    if !(crate::stdlib::fabs(
                                        (*v1.offset(0 as i32 as isize)
                                            - *v2.offset(0 as i32 as isize))
                                            as f64,
                                    ) > 0.1f64)
                                    {
                                        if !(crate::stdlib::fabs(
                                            (*v1.offset(1 as i32 as isize)
                                                - *v2.offset(1 as i32 as isize))
                                                as f64,
                                        ) > 0.1f64)
                                        {
                                            if !(crate::stdlib::fabs(
                                                (*v1.offset(2 as i32 as isize)
                                                    - *v2.offset(2 as i32 as isize))
                                                    as f64,
                                            ) > 0.1f64)
                                            {
                                                //
                                                v1 = (*(*grid2).verts.as_mut_ptr().offset(
                                                    ((*grid2).width * l + offset2) as isize,
                                                ))
                                                .xyz
                                                .as_mut_ptr();
                                                v2 = (*(*grid2).verts.as_mut_ptr().offset(
                                                    ((*grid2).width * (l + 1 as i32) + offset2)
                                                        as isize,
                                                ))
                                                .xyz
                                                .as_mut_ptr();
                                                if !(crate::stdlib::fabs(
                                                    (*v1.offset(0 as i32 as isize)
                                                        - *v2.offset(0 as i32 as isize))
                                                        as f64,
                                                ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(1 as i32 as isize)
                                                            - *v2.offset(1 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(2 as i32 as isize)
                                                            - *v2.offset(2 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64)
                                                {
                                                    //
                                                    //ri.Printf( PRINT_ALL, "found highest LoD crack between two patches\n" );
                                                    // insert row into grid2 right after after row l
                                                    if m != 0 {
                                                        column = (*grid2).width - 1 as i32
                                                    } else {
                                                        column = 0 as i32
                                                    }
                                                    grid2 =
                                                        
                                                        crate::src::renderergl1::tr_curve::R_GridInsertRow(grid2 as *mut crate::tr_local_h::srfGridMesh_s,
                                                                        l +
                                                                            1
                                                                                as
                                                                                i32,
                                                                        column,
                                                                        (*(*grid1).verts.as_mut_ptr().offset((k
                                                                                                                  -
                                                                                                                  1
                                                                                                                      as
                                                                                                                      i32
                                                                                                                  +
                                                                                                                  offset1)
                                                                                                                 as
                                                                                                                 isize)).xyz.as_mut_ptr(),
                                                                        *(*grid1).widthLodError.offset((k
                                                                                                            +
                                                                                                            1
                                                                                                                as
                                                                                                                i32)
                                                                                                           as
                                                                                                           isize))
    as *mut crate::tr_local_h::srfGridMesh_s;
                                                    if grid2.is_null() {
                                                        break;
                                                    }
                                                    (*grid2).lodStitched =
                                                        crate::src::qcommon::q_shared::qfalse
                                                            as i32;
                                                    let ref mut fresh6 = (*s_worldData
                                                        .surfaces
                                                        .offset(grid2num as isize))
                                                    .data;
                                                    *fresh6 = grid2 as *mut libc::c_void
                                                        as *mut crate::tr_local_h::surfaceType_t;
                                                    return crate::src::qcommon::q_shared::qtrue
                                                        as i32;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        l += 1
                    }
                    m += 1
                }
                k -= 2 as i32
            }
        }
        n += 1
    }
    n = 0 as i32;
    while n < 2 as i32 {
        //
        if n != 0 {
            offset1 = (*grid1).width - 1 as i32
        } else {
            offset1 = 0 as i32
        }
        if !(R_MergedHeightPoints(grid1, offset1) != 0) {
            k = (*grid1).height - 1 as i32;
            while k > 1 as i32 {
                m = 0 as i32;
                while m < 2 as i32 {
                    if grid2.is_null() || (*grid2).width >= 65 as i32 {
                        break;
                    }
                    if m != 0 {
                        offset2 = ((*grid2).height - 1 as i32) * (*grid2).width
                    } else {
                        offset2 = 0 as i32
                    }
                    l = 0 as i32;
                    while l < (*grid2).width - 1 as i32 {
                        //
                        v1 = (*(*grid1)
                            .verts
                            .as_mut_ptr()
                            .offset(((*grid1).width * k + offset1) as isize))
                        .xyz
                        .as_mut_ptr();
                        v2 = (*(*grid2).verts.as_mut_ptr().offset((l + offset2) as isize))
                            .xyz
                            .as_mut_ptr();
                        if !(crate::stdlib::fabs(
                            (*v1.offset(0 as i32 as isize) - *v2.offset(0 as i32 as isize)) as f64,
                        ) > 0.1f64)
                        {
                            if !(crate::stdlib::fabs(
                                (*v1.offset(1 as i32 as isize) - *v2.offset(1 as i32 as isize))
                                    as f64,
                            ) > 0.1f64)
                            {
                                if !(crate::stdlib::fabs(
                                    (*v1.offset(2 as i32 as isize) - *v2.offset(2 as i32 as isize))
                                        as f64,
                                ) > 0.1f64)
                                {
                                    v1 = (*(*grid1).verts.as_mut_ptr().offset(
                                        ((*grid1).width * (k - 2 as i32) + offset1) as isize,
                                    ))
                                    .xyz
                                    .as_mut_ptr();
                                    v2 = (*(*grid2)
                                        .verts
                                        .as_mut_ptr()
                                        .offset((l + 1 as i32 + offset2) as isize))
                                    .xyz
                                    .as_mut_ptr();
                                    if !(crate::stdlib::fabs(
                                        (*v1.offset(0 as i32 as isize)
                                            - *v2.offset(0 as i32 as isize))
                                            as f64,
                                    ) > 0.1f64)
                                    {
                                        if !(crate::stdlib::fabs(
                                            (*v1.offset(1 as i32 as isize)
                                                - *v2.offset(1 as i32 as isize))
                                                as f64,
                                        ) > 0.1f64)
                                        {
                                            if !(crate::stdlib::fabs(
                                                (*v1.offset(2 as i32 as isize)
                                                    - *v2.offset(2 as i32 as isize))
                                                    as f64,
                                            ) > 0.1f64)
                                            {
                                                //
                                                v1 = (*(*grid2)
                                                    .verts
                                                    .as_mut_ptr()
                                                    .offset((l + offset2) as isize))
                                                .xyz
                                                .as_mut_ptr();
                                                v2 = (*(*grid2)
                                                    .verts
                                                    .as_mut_ptr()
                                                    .offset((l + 1 as i32 + offset2) as isize))
                                                .xyz
                                                .as_mut_ptr();
                                                if !(crate::stdlib::fabs(
                                                    (*v1.offset(0 as i32 as isize)
                                                        - *v2.offset(0 as i32 as isize))
                                                        as f64,
                                                ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(1 as i32 as isize)
                                                            - *v2.offset(1 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(2 as i32 as isize)
                                                            - *v2.offset(2 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64)
                                                {
                                                    //
                                                    //ri.Printf( PRINT_ALL, "found highest LoD crack between two patches\n" );
                                                    // insert column into grid2 right after after column l
                                                    if m != 0 {
                                                        row = (*grid2).height - 1 as i32
                                                    } else {
                                                        row = 0 as i32
                                                    }
                                                    grid2 =
                                                        
                                                        crate::src::renderergl1::tr_curve::R_GridInsertColumn(grid2 as *mut crate::tr_local_h::srfGridMesh_s,
                                                                           l +
                                                                               1
                                                                                   as
                                                                                   i32,
                                                                           row,
                                                                           (*(*grid1).verts.as_mut_ptr().offset(((*grid1).width
                                                                                                                     *
                                                                                                                     (k
                                                                                                                          -
                                                                                                                          1
                                                                                                                              as
                                                                                                                              i32)
                                                                                                                     +
                                                                                                                     offset1)
                                                                                                                    as
                                                                                                                    isize)).xyz.as_mut_ptr(),
                                                                           *(*grid1).heightLodError.offset((k
                                                                                                                +
                                                                                                                1
                                                                                                                    as
                                                                                                                    i32)
                                                                                                               as
                                                                                                               isize))
    as *mut crate::tr_local_h::srfGridMesh_s;
                                                    (*grid2).lodStitched =
                                                        crate::src::qcommon::q_shared::qfalse
                                                            as i32;
                                                    let ref mut fresh7 = (*s_worldData
                                                        .surfaces
                                                        .offset(grid2num as isize))
                                                    .data;
                                                    *fresh7 = grid2 as *mut libc::c_void
                                                        as *mut crate::tr_local_h::surfaceType_t;
                                                    return crate::src::qcommon::q_shared::qtrue
                                                        as i32;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        l += 1
                    }
                    m += 1
                }
                m = 0 as i32;
                while m < 2 as i32 {
                    if grid2.is_null() || (*grid2).height >= 65 as i32 {
                        break;
                    }
                    if m != 0 {
                        offset2 = (*grid2).width - 1 as i32
                    } else {
                        offset2 = 0 as i32
                    }
                    l = 0 as i32;
                    while l < (*grid2).height - 1 as i32 {
                        //
                        v1 = (*(*grid1)
                            .verts
                            .as_mut_ptr()
                            .offset(((*grid1).width * k + offset1) as isize))
                        .xyz
                        .as_mut_ptr();
                        v2 = (*(*grid2)
                            .verts
                            .as_mut_ptr()
                            .offset(((*grid2).width * l + offset2) as isize))
                        .xyz
                        .as_mut_ptr();
                        if !(crate::stdlib::fabs(
                            (*v1.offset(0 as i32 as isize) - *v2.offset(0 as i32 as isize)) as f64,
                        ) > 0.1f64)
                        {
                            if !(crate::stdlib::fabs(
                                (*v1.offset(1 as i32 as isize) - *v2.offset(1 as i32 as isize))
                                    as f64,
                            ) > 0.1f64)
                            {
                                if !(crate::stdlib::fabs(
                                    (*v1.offset(2 as i32 as isize) - *v2.offset(2 as i32 as isize))
                                        as f64,
                                ) > 0.1f64)
                                {
                                    v1 = (*(*grid1).verts.as_mut_ptr().offset(
                                        ((*grid1).width * (k - 2 as i32) + offset1) as isize,
                                    ))
                                    .xyz
                                    .as_mut_ptr();
                                    v2 = (*(*grid2).verts.as_mut_ptr().offset(
                                        ((*grid2).width * (l + 1 as i32) + offset2) as isize,
                                    ))
                                    .xyz
                                    .as_mut_ptr();
                                    if !(crate::stdlib::fabs(
                                        (*v1.offset(0 as i32 as isize)
                                            - *v2.offset(0 as i32 as isize))
                                            as f64,
                                    ) > 0.1f64)
                                    {
                                        if !(crate::stdlib::fabs(
                                            (*v1.offset(1 as i32 as isize)
                                                - *v2.offset(1 as i32 as isize))
                                                as f64,
                                        ) > 0.1f64)
                                        {
                                            if !(crate::stdlib::fabs(
                                                (*v1.offset(2 as i32 as isize)
                                                    - *v2.offset(2 as i32 as isize))
                                                    as f64,
                                            ) > 0.1f64)
                                            {
                                                //
                                                v1 = (*(*grid2).verts.as_mut_ptr().offset(
                                                    ((*grid2).width * l + offset2) as isize,
                                                ))
                                                .xyz
                                                .as_mut_ptr();
                                                v2 = (*(*grid2).verts.as_mut_ptr().offset(
                                                    ((*grid2).width * (l + 1 as i32) + offset2)
                                                        as isize,
                                                ))
                                                .xyz
                                                .as_mut_ptr();
                                                if !(crate::stdlib::fabs(
                                                    (*v1.offset(0 as i32 as isize)
                                                        - *v2.offset(0 as i32 as isize))
                                                        as f64,
                                                ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(1 as i32 as isize)
                                                            - *v2.offset(1 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64
                                                    && crate::stdlib::fabs(
                                                        (*v1.offset(2 as i32 as isize)
                                                            - *v2.offset(2 as i32 as isize))
                                                            as f64,
                                                    ) < 0.01f64)
                                                {
                                                    //
                                                    //ri.Printf( PRINT_ALL, "found highest LoD crack between two patches\n" );
                                                    // insert row into grid2 right after after row l
                                                    if m != 0 {
                                                        column = (*grid2).width - 1 as i32
                                                    } else {
                                                        column = 0 as i32
                                                    }
                                                    grid2 =
                                                        
                                                        crate::src::renderergl1::tr_curve::R_GridInsertRow(grid2 as *mut crate::tr_local_h::srfGridMesh_s,
                                                                        l +
                                                                            1
                                                                                as
                                                                                i32,
                                                                        column,
                                                                        (*(*grid1).verts.as_mut_ptr().offset(((*grid1).width
                                                                                                                  *
                                                                                                                  (k
                                                                                                                       -
                                                                                                                       1
                                                                                                                           as
                                                                                                                           i32)
                                                                                                                  +
                                                                                                                  offset1)
                                                                                                                 as
                                                                                                                 isize)).xyz.as_mut_ptr(),
                                                                        *(*grid1).heightLodError.offset((k
                                                                                                             +
                                                                                                             1
                                                                                                                 as
                                                                                                                 i32)
                                                                                                            as
                                                                                                            isize))
    as *mut crate::tr_local_h::srfGridMesh_s;
                                                    (*grid2).lodStitched =
                                                        crate::src::qcommon::q_shared::qfalse
                                                            as i32;
                                                    let ref mut fresh8 = (*s_worldData
                                                        .surfaces
                                                        .offset(grid2num as isize))
                                                    .data;
                                                    *fresh8 = grid2 as *mut libc::c_void
                                                        as *mut crate::tr_local_h::surfaceType_t;
                                                    return crate::src::qcommon::q_shared::qtrue
                                                        as i32;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        l += 1
                    }
                    m += 1
                }
                k -= 2 as i32
            }
        }
        n += 1
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
===============
R_TryStitchPatch

This function will try to stitch patches in the same LoD group together for the highest LoD.

Only single missing vertice cracks will be fixed.

Vertices will be joined at the patch side a crack is first found, at the other side
of the patch (on the same row or column) the vertices will not be joined and cracks
might still appear at that side.
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_TryStitchingPatch(mut grid1num: i32) -> i32 {
    let mut j: i32 = 0;
    let mut numstitches: i32 = 0;
    let mut grid1: *mut crate::tr_local_h::srfGridMesh_t =
        0 as *mut crate::tr_local_h::srfGridMesh_t;
    let mut grid2: *mut crate::tr_local_h::srfGridMesh_t =
        0 as *mut crate::tr_local_h::srfGridMesh_t;
    numstitches = 0 as i32;
    grid1 = (*s_worldData.surfaces.offset(grid1num as isize)).data
        as *mut crate::tr_local_h::srfGridMesh_t;
    j = 0 as i32;
    while j < s_worldData.numsurfaces {
        //
        grid2 = (*s_worldData.surfaces.offset(j as isize)).data
            as *mut crate::tr_local_h::srfGridMesh_t;
        // if this surface is not a grid
        if !((*grid2).surfaceType as u32 != crate::tr_local_h::SF_GRID as i32 as u32) {
            // grids in the same LOD group should have the exact same lod radius
            if !((*grid1).lodRadius != (*grid2).lodRadius) {
                // grids in the same LOD group should have the exact same lod origin
                if !((*grid1).lodOrigin[0 as i32 as usize] != (*grid2).lodOrigin[0 as i32 as usize])
                {
                    if !((*grid1).lodOrigin[1 as i32 as usize]
                        != (*grid2).lodOrigin[1 as i32 as usize])
                    {
                        if !((*grid1).lodOrigin[2 as i32 as usize]
                            != (*grid2).lodOrigin[2 as i32 as usize])
                        {
                            //
                            while R_StitchPatches(grid1num, j) != 0 {
                                numstitches += 1
                            }
                        }
                    }
                }
            }
        }
        j += 1
    }
    return numstitches;
}
/*
===============
R_StitchAllPatches
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_StitchAllPatches() {
    let mut i: i32 = 0;
    let mut stitched: i32 = 0;
    let mut numstitches: i32 = 0;
    let mut grid1: *mut crate::tr_local_h::srfGridMesh_t =
        0 as *mut crate::tr_local_h::srfGridMesh_t;
    numstitches = 0 as i32;
    loop {
        stitched = crate::src::qcommon::q_shared::qfalse as i32;
        i = 0 as i32;
        while i < s_worldData.numsurfaces {
            //
            grid1 = (*s_worldData.surfaces.offset(i as isize)).data
                as *mut crate::tr_local_h::srfGridMesh_t;
            // if this surface is not a grid
            if !((*grid1).surfaceType as u32 != crate::tr_local_h::SF_GRID as i32 as u32) {
                //
                if !((*grid1).lodStitched != 0) {
                    //
                    (*grid1).lodStitched = crate::src::qcommon::q_shared::qtrue as i32;
                    stitched = crate::src::qcommon::q_shared::qtrue as i32;
                    //
                    numstitches += R_TryStitchingPatch(i)
                }
            }
            i += 1
        }
        if !(stitched != 0) {
            break;
        }
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"stitched %d LoD cracks\n\x00" as *const u8 as *const libc::c_char,
        numstitches,
    );
}
/*
===============
R_MovePatchSurfacesToHunk
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_MovePatchSurfacesToHunk() {
    let mut i: i32 = 0;
    let mut size: i32 = 0;
    let mut grid: *mut crate::tr_local_h::srfGridMesh_t =
        0 as *mut crate::tr_local_h::srfGridMesh_t;
    let mut hunkgrid: *mut crate::tr_local_h::srfGridMesh_t =
        0 as *mut crate::tr_local_h::srfGridMesh_t;
    i = 0 as i32;
    while i < s_worldData.numsurfaces {
        //
        grid = (*s_worldData.surfaces.offset(i as isize)).data
            as *mut crate::tr_local_h::srfGridMesh_t;
        // if this surface is not a grid
        if !((*grid).surfaceType as u32 != crate::tr_local_h::SF_GRID as i32 as u32) {
            //
            size = (((*grid).width * (*grid).height - 1 as i32) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::qfiles_h::drawVert_t>() as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<crate::tr_local_h::srfGridMesh_t>() as libc::c_ulong
                ) as i32;
            hunkgrid = crate::src::renderergl1::tr_main::ri
                .Hunk_Alloc
                .expect("non-null function pointer")(
                size, crate::src::qcommon::q_shared::h_low
            ) as *mut crate::tr_local_h::srfGridMesh_t;
            crate::stdlib::memcpy(
                hunkgrid as *mut libc::c_void,
                grid as *const libc::c_void,
                size as libc::c_ulong,
            );
            (*hunkgrid).widthLodError = crate::src::renderergl1::tr_main::ri
                .Hunk_Alloc
                .expect("non-null function pointer")(
                (*grid).width * 4 as i32,
                crate::src::qcommon::q_shared::h_low,
            ) as *mut f32;
            crate::stdlib::memcpy(
                (*hunkgrid).widthLodError as *mut libc::c_void,
                (*grid).widthLodError as *const libc::c_void,
                ((*grid).width * 4 as i32) as libc::c_ulong,
            );
            (*hunkgrid).heightLodError = crate::src::renderergl1::tr_main::ri
                .Hunk_Alloc
                .expect("non-null function pointer")(
                (*grid).height * 4 as i32,
                crate::src::qcommon::q_shared::h_low,
            ) as *mut f32;
            crate::stdlib::memcpy(
                (*hunkgrid).heightLodError as *mut libc::c_void,
                (*grid).heightLodError as *const libc::c_void,
                ((*grid).height * 4 as i32) as libc::c_ulong,
            );
            crate::src::renderergl1::tr_curve::R_FreeSurfaceGridMesh(
                grid as *mut crate::tr_local_h::srfGridMesh_s,
            );
            let ref mut fresh9 = (*s_worldData.surfaces.offset(i as isize)).data;
            *fresh9 = hunkgrid as *mut libc::c_void as *mut crate::tr_local_h::surfaceType_t
        }
        i += 1
    }
}
/*
===============
R_LoadSurfaces
===============
*/

unsafe extern "C" fn R_LoadSurfaces(
    mut surfs: *mut crate::qfiles_h::lump_t,
    mut verts: *mut crate::qfiles_h::lump_t,
    mut indexLump: *mut crate::qfiles_h::lump_t,
) {
    let mut in_0: *mut crate::qfiles_h::dsurface_t = 0 as *mut crate::qfiles_h::dsurface_t;
    let mut out: *mut crate::tr_local_h::msurface_t = 0 as *mut crate::tr_local_h::msurface_t;
    let mut dv: *mut crate::qfiles_h::drawVert_t = 0 as *mut crate::qfiles_h::drawVert_t;
    let mut indexes: *mut i32 = 0 as *mut i32;
    let mut count: i32 = 0;
    let mut numFaces: i32 = 0;
    let mut numMeshes: i32 = 0;
    let mut numTriSurfs: i32 = 0;
    let mut numFlares: i32 = 0;
    let mut i: i32 = 0;
    numFaces = 0 as i32;
    numMeshes = 0 as i32;
    numTriSurfs = 0 as i32;
    numFlares = 0 as i32;
    in_0 = fileBase.offset((*surfs).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dsurface_t;
    if ((*surfs).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dsurface_t>() as libc::c_ulong)
        != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadMap: funny lump size in %s\x00" as *const u8 as *const libc::c_char,
            s_worldData.name.as_mut_ptr(),
        );
    }
    count = ((*surfs).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dsurface_t>() as libc::c_ulong)
        as i32;
    dv = fileBase.offset((*verts).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::drawVert_t;
    if ((*verts).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::drawVert_t>() as libc::c_ulong)
        != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadMap: funny lump size in %s\x00" as *const u8 as *const libc::c_char,
            s_worldData.name.as_mut_ptr(),
        );
    }
    indexes = fileBase.offset((*indexLump).fileofs as isize) as *mut libc::c_void as *mut i32;
    if ((*indexLump).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<i32>() as libc::c_ulong)
        != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadMap: funny lump size in %s\x00" as *const u8 as *const libc::c_char,
            s_worldData.name.as_mut_ptr(),
        );
    }
    out = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::tr_local_h::msurface_t>() as libc::c_ulong)
            as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::msurface_t;
    s_worldData.surfaces = out;
    s_worldData.numsurfaces = count;
    i = 0 as i32;
    while i < count {
        match (*in_0).surfaceType {
            2 => {
                ParseMesh(in_0, dv, out);
                numMeshes += 1
            }
            3 => {
                ParseTriSurf(in_0, dv, out, indexes);
                numTriSurfs += 1
            }
            1 => {
                ParseFace(in_0, dv, out, indexes);
                numFaces += 1
            }
            4 => {
                ParseFlare(in_0, dv, out, indexes);
                numFlares += 1
            }
            _ => {
                crate::src::renderergl1::tr_main::ri
                    .Error
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::ERR_DROP as i32,
                    b"Bad surfaceType\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
    R_StitchAllPatches();
    R_FixSharedVertexLodError();
    R_MovePatchSurfacesToHunk();
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"...loaded %d faces, %i meshes, %i trisurfs, %i flares\n\x00" as *const u8
            as *const libc::c_char,
        numFaces,
        numMeshes,
        numTriSurfs,
        numFlares,
    );
}
/*
=================
R_LoadSubmodels
=================
*/

unsafe extern "C" fn R_LoadSubmodels(mut l: *mut crate::qfiles_h::lump_t) {
    let mut in_0: *mut crate::qfiles_h::dmodel_t = 0 as *mut crate::qfiles_h::dmodel_t;
    let mut out: *mut crate::tr_local_h::bmodel_t = 0 as *mut crate::tr_local_h::bmodel_t;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut count: i32 = 0;
    in_0 = fileBase.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dmodel_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dmodel_t>() as libc::c_ulong)
        != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadMap: funny lump size in %s\x00" as *const u8 as *const libc::c_char,
            s_worldData.name.as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dmodel_t>() as libc::c_ulong)
        as i32;
    out = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::tr_local_h::bmodel_t>() as libc::c_ulong)
            as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::bmodel_t;
    s_worldData.bmodels = out;
    i = 0 as i32;
    while i < count {
        let mut model: *mut crate::tr_local_h::model_t = 0 as *mut crate::tr_local_h::model_t;
        model =
            crate::src::renderergl1::tr_model::R_AllocModel() as *mut crate::tr_local_h::model_s;
        // this should never happen
        if model.is_null() {
            crate::src::renderergl1::tr_main::ri
                .Error
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"R_LoadSubmodels: R_AllocModel() failed\x00" as *const u8 as *const libc::c_char,
            );
        }
        (*model).type_0 = crate::tr_local_h::MOD_BRUSH;
        (*model).bmodel = out;
        crate::src::qcommon::q_shared::Com_sprintf(
            (*model).name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
            b"*%d\x00" as *const u8 as *const libc::c_char,
            i,
        );
        j = 0 as i32;
        while j < 3 as i32 {
            (*out).bounds[0 as i32 as usize][j as usize] = (*in_0).mins[j as usize];
            (*out).bounds[1 as i32 as usize][j as usize] = (*in_0).maxs[j as usize];
            j += 1
        }
        (*out).firstSurface = s_worldData.surfaces.offset((*in_0).firstSurface as isize);
        (*out).numSurfaces = (*in_0).numSurfaces;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
}
//==================================================================
/*
=================
R_SetParent
=================
*/

unsafe extern "C" fn R_SetParent(
    mut node: *mut crate::tr_local_h::mnode_t,
    mut parent: *mut crate::tr_local_h::mnode_t,
) {
    (*node).parent = parent;
    if (*node).contents != -(1 as i32) {
        return;
    }
    R_SetParent((*node).children[0 as i32 as usize], node);
    R_SetParent((*node).children[1 as i32 as usize], node);
}
/*
=================
R_LoadNodesAndLeafs
=================
*/

unsafe extern "C" fn R_LoadNodesAndLeafs(
    mut nodeLump: *mut crate::qfiles_h::lump_t,
    mut leafLump: *mut crate::qfiles_h::lump_t,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p: i32 = 0;
    let mut in_0: *mut crate::qfiles_h::dnode_t = 0 as *mut crate::qfiles_h::dnode_t;
    let mut inLeaf: *mut crate::qfiles_h::dleaf_t = 0 as *mut crate::qfiles_h::dleaf_t;
    let mut out: *mut crate::tr_local_h::mnode_t = 0 as *mut crate::tr_local_h::mnode_t;
    let mut numNodes: i32 = 0;
    let mut numLeafs: i32 = 0;
    in_0 = fileBase.offset((*nodeLump).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dnode_t;
    if ((*nodeLump).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dnode_t>() as libc::c_ulong)
        != 0
        || ((*leafLump).filelen as libc::c_ulong)
            .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dleaf_t>() as libc::c_ulong)
            != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadMap: funny lump size in %s\x00" as *const u8 as *const libc::c_char,
            s_worldData.name.as_mut_ptr(),
        );
    }
    numNodes = ((*nodeLump).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dnode_t>() as libc::c_ulong)
        as i32;
    numLeafs = ((*leafLump).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dleaf_t>() as libc::c_ulong)
        as i32;
    out = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        ((numNodes + numLeafs) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::tr_local_h::mnode_t>() as libc::c_ulong)
            as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::mnode_t;
    s_worldData.nodes = out;
    s_worldData.numnodes = numNodes + numLeafs;
    s_worldData.numDecisionNodes = numNodes;
    // load nodes
    i = 0 as i32; // differentiate from leafs
    while i < numNodes {
        j = 0 as i32;
        while j < 3 as i32 {
            (*out).mins[j as usize] =
                (*in_0).mins[j as usize] as crate::src::qcommon::q_shared::vec_t;
            (*out).maxs[j as usize] =
                (*in_0).maxs[j as usize] as crate::src::qcommon::q_shared::vec_t;
            j += 1
        }
        p = (*in_0).planeNum;
        (*out).plane = s_worldData.planes.offset(p as isize);
        (*out).contents = -(1 as i32);
        j = 0 as i32;
        while j < 2 as i32 {
            p = (*in_0).children[j as usize];
            if p >= 0 as i32 {
                (*out).children[j as usize] = s_worldData.nodes.offset(p as isize)
            } else {
                (*out).children[j as usize] = s_worldData
                    .nodes
                    .offset(numNodes as isize)
                    .offset((-(1 as i32) - p) as isize)
            }
            j += 1
        }
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
    // load leafs
    inLeaf = fileBase.offset((*leafLump).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dleaf_t;
    i = 0 as i32;
    while i < numLeafs {
        j = 0 as i32;
        while j < 3 as i32 {
            (*out).mins[j as usize] =
                (*inLeaf).mins[j as usize] as crate::src::qcommon::q_shared::vec_t;
            (*out).maxs[j as usize] =
                (*inLeaf).maxs[j as usize] as crate::src::qcommon::q_shared::vec_t;
            j += 1
        }
        (*out).cluster = (*inLeaf).cluster;
        (*out).area = (*inLeaf).area;
        if (*out).cluster >= s_worldData.numClusters {
            s_worldData.numClusters = (*out).cluster + 1 as i32
        }
        (*out).firstmarksurface = s_worldData
            .marksurfaces
            .offset((*inLeaf).firstLeafSurface as isize);
        (*out).nummarksurfaces = (*inLeaf).numLeafSurfaces;
        i += 1;
        inLeaf = inLeaf.offset(1);
        out = out.offset(1)
    }
    // chain descendants
    R_SetParent(s_worldData.nodes, 0 as *mut crate::tr_local_h::mnode_t);
}
//=============================================================================
/*
=================
R_LoadShaders
=================
*/

unsafe extern "C" fn R_LoadShaders(mut l: *mut crate::qfiles_h::lump_t) {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut in_0: *mut crate::qfiles_h::dshader_t = 0 as *mut crate::qfiles_h::dshader_t;
    let mut out: *mut crate::qfiles_h::dshader_t = 0 as *mut crate::qfiles_h::dshader_t;
    in_0 = fileBase.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dshader_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dshader_t>() as libc::c_ulong)
        != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadMap: funny lump size in %s\x00" as *const u8 as *const libc::c_char,
            s_worldData.name.as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dshader_t>() as libc::c_ulong)
        as i32;
    out = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::qfiles_h::dshader_t>() as libc::c_ulong)
            as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::qfiles_h::dshader_t;
    s_worldData.shaders = out;
    s_worldData.numShaders = count;
    crate::stdlib::memcpy(
        out as *mut libc::c_void,
        in_0 as *const libc::c_void,
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::qfiles_h::dshader_t>() as libc::c_ulong),
    );
    i = 0 as i32;
    while i < count {
        (*out.offset(i as isize)).surfaceFlags = (*out.offset(i as isize)).surfaceFlags;
        (*out.offset(i as isize)).contentFlags = (*out.offset(i as isize)).contentFlags;
        i += 1
    }
}
/*
=================
R_LoadMarksurfaces
=================
*/

unsafe extern "C" fn R_LoadMarksurfaces(mut l: *mut crate::qfiles_h::lump_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut count: i32 = 0;
    let mut in_0: *mut i32 = 0 as *mut i32;
    let mut out: *mut *mut crate::tr_local_h::msurface_t =
        0 as *mut *mut crate::tr_local_h::msurface_t;
    in_0 = fileBase.offset((*l).fileofs as isize) as *mut libc::c_void as *mut i32;
    if ((*l).filelen as libc::c_ulong).wrapping_rem(::std::mem::size_of::<i32>() as libc::c_ulong)
        != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadMap: funny lump size in %s\x00" as *const u8 as *const libc::c_char,
            s_worldData.name.as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<i32>() as libc::c_ulong) as i32;
    out = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        (count as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            *mut crate::tr_local_h::msurface_t,
        >() as libc::c_ulong) as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut *mut crate::tr_local_h::msurface_t;
    s_worldData.marksurfaces = out;
    s_worldData.nummarksurfaces = count;
    i = 0 as i32;
    while i < count {
        j = *in_0.offset(i as isize);
        let ref mut fresh10 = *out.offset(i as isize);
        *fresh10 = s_worldData.surfaces.offset(j as isize);
        i += 1
    }
}
/*
=================
R_LoadPlanes
=================
*/

unsafe extern "C" fn R_LoadPlanes(mut l: *mut crate::qfiles_h::lump_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut out: *mut crate::src::qcommon::q_shared::cplane_t =
        0 as *mut crate::src::qcommon::q_shared::cplane_t;
    let mut in_0: *mut crate::qfiles_h::dplane_t = 0 as *mut crate::qfiles_h::dplane_t;
    let mut count: i32 = 0;
    let mut bits: i32 = 0;
    in_0 = fileBase.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dplane_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dplane_t>() as libc::c_ulong)
        != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadMap: funny lump size in %s\x00" as *const u8 as *const libc::c_char,
            s_worldData.name.as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dplane_t>() as libc::c_ulong)
        as i32;
    out = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        ((count * 2 as i32) as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::src::qcommon::q_shared::cplane_t,
        >() as libc::c_ulong) as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::src::qcommon::q_shared::cplane_t;
    s_worldData.planes = out;
    s_worldData.numplanes = count;
    i = 0 as i32;
    while i < count {
        bits = 0 as i32;
        j = 0 as i32;
        while j < 3 as i32 {
            (*out).normal[j as usize] = (*in_0).normal[j as usize];
            if (*out).normal[j as usize] < 0 as i32 as f32 {
                bits |= (1 as i32) << j
            }
            j += 1
        }
        (*out).dist = (*in_0).dist;
        (*out).type_0 = if (*out).normal[0 as i32 as usize] as f64 == 1.0f64 {
            0 as i32
        } else if (*out).normal[1 as i32 as usize] as f64 == 1.0f64 {
            1 as i32
        } else if (*out).normal[2 as i32 as usize] as f64 == 1.0f64 {
            2 as i32
        } else {
            3 as i32
        } as crate::src::qcommon::q_shared::byte;
        (*out).signbits = bits as crate::src::qcommon::q_shared::byte;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
}
/*
=================
R_LoadFogs

=================
*/

unsafe extern "C" fn R_LoadFogs(
    mut l: *mut crate::qfiles_h::lump_t,
    mut brushesLump: *mut crate::qfiles_h::lump_t,
    mut sidesLump: *mut crate::qfiles_h::lump_t,
) {
    let mut i: i32 = 0;
    let mut out: *mut crate::tr_local_h::fog_t = 0 as *mut crate::tr_local_h::fog_t;
    let mut fogs: *mut crate::qfiles_h::dfog_t = 0 as *mut crate::qfiles_h::dfog_t;
    let mut brushes: *mut crate::qfiles_h::dbrush_t = 0 as *mut crate::qfiles_h::dbrush_t;
    let mut brush: *mut crate::qfiles_h::dbrush_t = 0 as *mut crate::qfiles_h::dbrush_t;
    let mut sides: *mut crate::qfiles_h::dbrushside_t = 0 as *mut crate::qfiles_h::dbrushside_t;
    let mut count: i32 = 0;
    let mut brushesCount: i32 = 0;
    let mut sidesCount: i32 = 0;
    let mut sideNum: i32 = 0;
    let mut planeNum: i32 = 0;
    let mut shader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    let mut d: f32 = 0.;
    let mut firstSide: i32 = 0;
    fogs =
        fileBase.offset((*l).fileofs as isize) as *mut libc::c_void as *mut crate::qfiles_h::dfog_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dfog_t>() as libc::c_ulong)
        != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadMap: funny lump size in %s\x00" as *const u8 as *const libc::c_char,
            s_worldData.name.as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dfog_t>() as libc::c_ulong)
        as i32;
    // create fog structures for them
    s_worldData.numfogs = count + 1 as i32;
    s_worldData.fogs = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        (s_worldData.numfogs as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::tr_local_h::fog_t>() as libc::c_ulong)
            as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::fog_t;
    out = s_worldData.fogs.offset(1 as i32 as isize);
    if count == 0 {
        return;
    }
    brushes = fileBase.offset((*brushesLump).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dbrush_t;
    if ((*brushesLump).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dbrush_t>() as libc::c_ulong)
        != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadMap: funny lump size in %s\x00" as *const u8 as *const libc::c_char,
            s_worldData.name.as_mut_ptr(),
        );
    }
    brushesCount = ((*brushesLump).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dbrush_t>() as libc::c_ulong)
        as i32;
    sides = fileBase.offset((*sidesLump).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dbrushside_t;
    if ((*sidesLump).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dbrushside_t>() as libc::c_ulong)
        != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"LoadMap: funny lump size in %s\x00" as *const u8 as *const libc::c_char,
            s_worldData.name.as_mut_ptr(),
        );
    }
    sidesCount = ((*sidesLump).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dbrushside_t>() as libc::c_ulong)
        as i32;
    i = 0 as i32;
    while i < count {
        (*out).originalBrushNumber = (*fogs).brushNum;
        if (*out).originalBrushNumber as u32 >= brushesCount as u32 {
            crate::src::renderergl1::tr_main::ri
                .Error
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"fog brushNumber out of range\x00" as *const u8 as *const libc::c_char,
            );
        }
        brush = brushes.offset((*out).originalBrushNumber as isize);
        firstSide = (*brush).firstSide;
        if firstSide as u32 > (sidesCount - 6 as i32) as u32 {
            crate::src::renderergl1::tr_main::ri
                .Error
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"fog brush sideNumber out of range\x00" as *const u8 as *const libc::c_char,
            );
        }
        // brushes are always sorted with the axial sides first
        sideNum = firstSide + 0 as i32;
        planeNum = (*sides.offset(sideNum as isize)).planeNum;
        (*out).bounds[0 as i32 as usize][0 as i32 as usize] =
            -(*s_worldData.planes.offset(planeNum as isize)).dist;
        sideNum = firstSide + 1 as i32;
        planeNum = (*sides.offset(sideNum as isize)).planeNum;
        (*out).bounds[1 as i32 as usize][0 as i32 as usize] =
            (*s_worldData.planes.offset(planeNum as isize)).dist;
        sideNum = firstSide + 2 as i32;
        planeNum = (*sides.offset(sideNum as isize)).planeNum;
        (*out).bounds[0 as i32 as usize][1 as i32 as usize] =
            -(*s_worldData.planes.offset(planeNum as isize)).dist;
        sideNum = firstSide + 3 as i32;
        planeNum = (*sides.offset(sideNum as isize)).planeNum;
        (*out).bounds[1 as i32 as usize][1 as i32 as usize] =
            (*s_worldData.planes.offset(planeNum as isize)).dist;
        sideNum = firstSide + 4 as i32;
        planeNum = (*sides.offset(sideNum as isize)).planeNum;
        (*out).bounds[0 as i32 as usize][2 as i32 as usize] =
            -(*s_worldData.planes.offset(planeNum as isize)).dist;
        sideNum = firstSide + 5 as i32;
        planeNum = (*sides.offset(sideNum as isize)).planeNum;
        (*out).bounds[1 as i32 as usize][2 as i32 as usize] =
            (*s_worldData.planes.offset(planeNum as isize)).dist;
        // get information from the shader for fog parameters
        shader = crate::src::renderergl1::tr_shader::R_FindShader(
            (*fogs).shader.as_mut_ptr(),
            -(1 as i32),
            crate::src::qcommon::q_shared::qtrue,
        ) as *mut crate::tr_local_h::shader_s;
        (*out).parms = (*shader).fogParms;
        (*out).colorInt = crate::src::qcommon::q_math::ColorBytes4(
            (*shader).fogParms.color[0 as i32 as usize]
                * crate::src::renderergl1::tr_main::tr.identityLight,
            (*shader).fogParms.color[1 as i32 as usize]
                * crate::src::renderergl1::tr_main::tr.identityLight,
            (*shader).fogParms.color[2 as i32 as usize]
                * crate::src::renderergl1::tr_main::tr.identityLight,
            1.0f64 as f32,
        );
        d = if (*shader).fogParms.depthForOpaque < 1 as i32 as f32 {
            1 as i32 as f32
        } else {
            (*shader).fogParms.depthForOpaque
        };
        (*out).tcScale = 1.0f32 / (d * 8 as i32 as f32);
        // set the gradient vector
        sideNum = (*fogs).visibleSide;
        if sideNum == -(1 as i32) {
            (*out).hasSurface = crate::src::qcommon::q_shared::qfalse
        } else {
            (*out).hasSurface = crate::src::qcommon::q_shared::qtrue;
            planeNum = (*sides.offset((firstSide + sideNum) as isize)).planeNum;
            (*out).surface[0 as i32 as usize] = crate::src::qcommon::q_math::vec3_origin
                [0 as i32 as usize]
                - (*s_worldData.planes.offset(planeNum as isize)).normal[0 as i32 as usize];
            (*out).surface[1 as i32 as usize] = crate::src::qcommon::q_math::vec3_origin
                [1 as i32 as usize]
                - (*s_worldData.planes.offset(planeNum as isize)).normal[1 as i32 as usize];
            (*out).surface[2 as i32 as usize] = crate::src::qcommon::q_math::vec3_origin
                [2 as i32 as usize]
                - (*s_worldData.planes.offset(planeNum as isize)).normal[2 as i32 as usize];
            (*out).surface[3 as i32 as usize] =
                -(*s_worldData.planes.offset(planeNum as isize)).dist
        }
        out = out.offset(1);
        i += 1;
        fogs = fogs.offset(1)
    }
}
/*
================
R_LoadLightGrid

================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LoadLightGrid(mut l: *mut crate::qfiles_h::lump_t) {
    let mut i: i32 = 0;
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut numGridPoints: i32 = 0;
    let mut w: *mut crate::tr_local_h::world_t = 0 as *mut crate::tr_local_h::world_t;
    let mut wMins: *mut f32 = 0 as *mut f32;
    let mut wMaxs: *mut f32 = 0 as *mut f32;
    w = &mut s_worldData;
    (*w).lightGridInverseSize[0 as i32 as usize] = 1.0f32 / (*w).lightGridSize[0 as i32 as usize];
    (*w).lightGridInverseSize[1 as i32 as usize] = 1.0f32 / (*w).lightGridSize[1 as i32 as usize];
    (*w).lightGridInverseSize[2 as i32 as usize] = 1.0f32 / (*w).lightGridSize[2 as i32 as usize];
    wMins = (*(*w).bmodels.offset(0 as i32 as isize)).bounds[0 as i32 as usize].as_mut_ptr();
    wMaxs = (*(*w).bmodels.offset(0 as i32 as isize)).bounds[1 as i32 as usize].as_mut_ptr();
    i = 0 as i32;
    while i < 3 as i32 {
        (*w).lightGridOrigin[i as usize] = ((*w).lightGridSize[i as usize] as f64
            * crate::stdlib::ceil(
                (*wMins.offset(i as isize) / (*w).lightGridSize[i as usize]) as f64,
            )) as crate::src::qcommon::q_shared::vec_t;
        maxs[i as usize] = ((*w).lightGridSize[i as usize] as f64
            * crate::stdlib::floor(
                (*wMaxs.offset(i as isize) / (*w).lightGridSize[i as usize]) as f64,
            )) as crate::src::qcommon::q_shared::vec_t;
        (*w).lightGridBounds[i as usize] = ((maxs[i as usize] - (*w).lightGridOrigin[i as usize])
            / (*w).lightGridSize[i as usize]
            + 1 as i32 as f32) as i32;
        i += 1
    }
    numGridPoints = (*w).lightGridBounds[0 as i32 as usize]
        * (*w).lightGridBounds[1 as i32 as usize]
        * (*w).lightGridBounds[2 as i32 as usize];
    if (*l).filelen != numGridPoints * 8 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"WARNING: light grid mismatch\n\x00" as *const u8 as *const libc::c_char,
        );
        (*w).lightGridData = 0 as *mut crate::src::qcommon::q_shared::byte;
        return;
    }
    (*w).lightGridData = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        (*l).filelen, crate::src::qcommon::q_shared::h_low
    ) as *mut crate::src::qcommon::q_shared::byte;
    crate::stdlib::memcpy(
        (*w).lightGridData as *mut libc::c_void,
        fileBase.offset((*l).fileofs as isize) as *mut libc::c_void,
        (*l).filelen as libc::c_ulong,
    );
    // deal with overbright bits
    i = 0 as i32;
    while i < numGridPoints {
        R_ColorShiftLightingBytes(
            &mut *(*w).lightGridData.offset((i * 8 as i32) as isize),
            &mut *(*w).lightGridData.offset((i * 8 as i32) as isize),
        );
        R_ColorShiftLightingBytes(
            &mut *(*w)
                .lightGridData
                .offset((i * 8 as i32 + 3 as i32) as isize),
            &mut *(*w)
                .lightGridData
                .offset((i * 8 as i32 + 3 as i32) as isize),
        );
        i += 1
    }
}
/*
================
R_LoadEntities
================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LoadEntities(mut l: *mut crate::qfiles_h::lump_t) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut keyname: [libc::c_char; 1024] = [0; 1024];
    let mut value: [libc::c_char; 1024] = [0; 1024];
    let mut w: *mut crate::tr_local_h::world_t = 0 as *mut crate::tr_local_h::world_t;
    w = &mut s_worldData;
    (*w).lightGridSize[0 as i32 as usize] = 64 as i32 as crate::src::qcommon::q_shared::vec_t;
    (*w).lightGridSize[1 as i32 as usize] = 64 as i32 as crate::src::qcommon::q_shared::vec_t;
    (*w).lightGridSize[2 as i32 as usize] = 128 as i32 as crate::src::qcommon::q_shared::vec_t;
    p = fileBase.offset((*l).fileofs as isize) as *mut libc::c_char;
    // store for reference by the cgame
    (*w).entityString = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        (*l).filelen + 1 as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut libc::c_char;
    ::libc::strcpy((*w).entityString, p);
    (*w).entityParsePoint = (*w).entityString;
    token =
        crate::src::qcommon::q_shared::COM_ParseExt(&mut p, crate::src::qcommon::q_shared::qtrue);
    if *token == 0 || *token as i32 != '{' as i32 {
        return;
    }
    loop
    // only parse the world spawn
    // parse key
    {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            &mut p,
            crate::src::qcommon::q_shared::qtrue,
        );
        if *token == 0 || *token as i32 == '}' as i32 {
            break;
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            keyname.as_mut_ptr(),
            token,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        // parse value
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            &mut p,
            crate::src::qcommon::q_shared::qtrue,
        );
        if *token == 0 || *token as i32 == '}' as i32 {
            break;
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            value.as_mut_ptr(),
            token,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        // check for remapping of shaders for vertex lighting
        s = b"vertexremapshader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        if crate::src::qcommon::q_shared::Q_strncmp(
            keyname.as_mut_ptr(),
            s,
            crate::stdlib::strlen(s) as i32,
        ) == 0
        {
            s = ::libc::strchr(value.as_mut_ptr(), ';' as i32);
            if s.is_null() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"WARNING: no semi colon in vertexshaderremap \'%s\'\n\x00" as *const u8
                        as *const libc::c_char,
                    value.as_mut_ptr(),
                );
                break;
            } else {
                let fresh11 = s;
                s = s.offset(1);
                *fresh11 = 0 as i32 as libc::c_char;
                if (*crate::src::renderergl1::tr_init::r_vertexLight).integer != 0 {
                    crate::src::renderergl1::tr_shader::R_RemapShader(
                        value.as_mut_ptr(),
                        s,
                        b"0\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
        } else {
            // check for remapping of shaders
            s = b"remapshader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
            if crate::src::qcommon::q_shared::Q_strncmp(
                keyname.as_mut_ptr(),
                s,
                crate::stdlib::strlen(s) as i32,
            ) == 0
            {
                s = ::libc::strchr(value.as_mut_ptr(), ';' as i32);
                if s.is_null() {
                    crate::src::renderergl1::tr_main::ri
                        .Printf
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                        b"WARNING: no semi colon in shaderremap \'%s\'\n\x00" as *const u8
                            as *const libc::c_char,
                        value.as_mut_ptr(),
                    );
                    break;
                } else {
                    let fresh12 = s;
                    s = s.offset(1);
                    *fresh12 = 0 as i32 as libc::c_char;
                    crate::src::renderergl1::tr_shader::R_RemapShader(
                        value.as_mut_ptr(),
                        s,
                        b"0\x00" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                // check for a different grid size
                if !(crate::src::qcommon::q_shared::Q_stricmp(
                    keyname.as_mut_ptr(),
                    b"gridsize\x00" as *const u8 as *const libc::c_char,
                ) == 0)
                {
                    continue;
                }
                ::libc::sscanf(
                    value.as_mut_ptr(),
                    b"%f %f %f\x00" as *const u8 as *const libc::c_char,
                    &mut *(*w).lightGridSize.as_mut_ptr().offset(0 as i32 as isize)
                        as *mut crate::src::qcommon::q_shared::vec_t,
                    &mut *(*w).lightGridSize.as_mut_ptr().offset(1 as i32 as isize)
                        as *mut crate::src::qcommon::q_shared::vec_t,
                    &mut *(*w).lightGridSize.as_mut_ptr().offset(2 as i32 as isize)
                        as *mut crate::src::qcommon::q_shared::vec_t,
                );
            }
        }
    }
}
/*
=================
R_GetEntityToken
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_GetEntityToken(
    mut buffer: *mut libc::c_char,
    mut size: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    s = crate::src::qcommon::q_shared::COM_Parse(&mut s_worldData.entityParsePoint);
    crate::src::qcommon::q_shared::Q_strncpyz(buffer, s, size);
    if s_worldData.entityParsePoint.is_null() && *s.offset(0 as i32 as isize) == 0 {
        s_worldData.entityParsePoint = s_worldData.entityString;
        return crate::src::qcommon::q_shared::qfalse;
    } else {
        return crate::src::qcommon::q_shared::qtrue;
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
/*
=================
RE_LoadWorldMap

Called directly from cgame
=================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_LoadWorldMap(mut name: *const libc::c_char) {
    let mut i: i32 = 0;
    let mut header: *mut crate::qfiles_h::dheader_t = 0 as *mut crate::qfiles_h::dheader_t;
    let mut buffer: C2RustUnnamed_102 = C2RustUnnamed_102 {
        b: 0 as *mut crate::src::qcommon::q_shared::byte,
    };
    let mut startMarker: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    if crate::src::renderergl1::tr_main::tr.worldMapLoaded as u64 != 0 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"ERROR: attempted to redundantly load world map\x00" as *const u8
                as *const libc::c_char,
        );
    }
    // set default sun direction to be used if it isn't
    // overridden by a shader
    crate::src::renderergl1::tr_main::tr.sunDirection[0 as i32 as usize] = 0.45f32;
    crate::src::renderergl1::tr_main::tr.sunDirection[1 as i32 as usize] = 0.3f32;
    crate::src::renderergl1::tr_main::tr.sunDirection[2 as i32 as usize] = 0.9f32;
    crate::src::qcommon::q_math::VectorNormalize(
        crate::src::renderergl1::tr_main::tr
            .sunDirection
            .as_mut_ptr(),
    );
    crate::src::renderergl1::tr_main::tr.worldMapLoaded = crate::src::qcommon::q_shared::qtrue;
    // load it
    crate::src::renderergl1::tr_main::ri
        .FS_ReadFile
        .expect("non-null function pointer")(name, &mut buffer.v);
    if buffer.b.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"RE_LoadWorldMap: %s not found\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    // clear tr.world so if the level fails to load, the next
    // try will not look at the partially loaded version
    crate::src::renderergl1::tr_main::tr.world = 0 as *mut crate::tr_local_h::world_t;
    crate::stdlib::memset(
        &mut s_worldData as *mut crate::tr_local_h::world_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_local_h::world_t>() as libc::c_ulong,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        s_worldData.name.as_mut_ptr(),
        name,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        s_worldData.baseName.as_mut_ptr(),
        crate::src::qcommon::q_shared::COM_SkipPath(s_worldData.name.as_mut_ptr()),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::COM_StripExtension(
        s_worldData.baseName.as_mut_ptr(),
        s_worldData.baseName.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    startMarker = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        0 as i32, crate::src::qcommon::q_shared::h_low
    ) as *mut crate::src::qcommon::q_shared::byte;
    c_gridVerts = 0 as i32;
    header = buffer.b as *mut crate::qfiles_h::dheader_t;
    fileBase = header as *mut crate::src::qcommon::q_shared::byte;
    i = (*header).version;
    if i != 46 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"RE_LoadWorldMap: %s has wrong version number (%i should be %i)\x00" as *const u8
                as *const libc::c_char,
            name,
            i,
            46 as i32,
        );
    }
    // swap all the lumps
    i = 0 as i32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<crate::qfiles_h::dheader_t>() as libc::c_ulong)
            .wrapping_div(4 as i32 as libc::c_ulong)
    {
        *(header as *mut i32).offset(i as isize) = *(header as *mut i32).offset(i as isize);
        i += 1
    }
    // load into heap
    R_LoadShaders(&mut *(*header).lumps.as_mut_ptr().offset(1 as i32 as isize));
    R_LoadLightmaps(&mut *(*header).lumps.as_mut_ptr().offset(14 as i32 as isize));
    R_LoadPlanes(&mut *(*header).lumps.as_mut_ptr().offset(2 as i32 as isize));
    R_LoadFogs(
        &mut *(*header).lumps.as_mut_ptr().offset(12 as i32 as isize),
        &mut *(*header).lumps.as_mut_ptr().offset(8 as i32 as isize),
        &mut *(*header).lumps.as_mut_ptr().offset(9 as i32 as isize),
    );
    R_LoadSurfaces(
        &mut *(*header).lumps.as_mut_ptr().offset(13 as i32 as isize),
        &mut *(*header).lumps.as_mut_ptr().offset(10 as i32 as isize),
        &mut *(*header).lumps.as_mut_ptr().offset(11 as i32 as isize),
    );
    R_LoadMarksurfaces(&mut *(*header).lumps.as_mut_ptr().offset(5 as i32 as isize));
    R_LoadNodesAndLeafs(
        &mut *(*header).lumps.as_mut_ptr().offset(3 as i32 as isize),
        &mut *(*header).lumps.as_mut_ptr().offset(4 as i32 as isize),
    );
    R_LoadSubmodels(&mut *(*header).lumps.as_mut_ptr().offset(7 as i32 as isize));
    R_LoadVisibility(&mut *(*header).lumps.as_mut_ptr().offset(16 as i32 as isize));
    R_LoadEntities(&mut *(*header).lumps.as_mut_ptr().offset(0 as i32 as isize));
    R_LoadLightGrid(&mut *(*header).lumps.as_mut_ptr().offset(15 as i32 as isize));
    s_worldData.dataSize = (crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        0 as i32, crate::src::qcommon::q_shared::h_low
    ) as *mut crate::src::qcommon::q_shared::byte)
        .offset_from(startMarker) as isize as i32;
    // only set tr.world now that we know the entire level has loaded properly
    crate::src::renderergl1::tr_main::tr.world = &mut s_worldData;
    crate::src::renderergl1::tr_main::ri
        .FS_FreeFile
        .expect("non-null function pointer")(buffer.v);
}

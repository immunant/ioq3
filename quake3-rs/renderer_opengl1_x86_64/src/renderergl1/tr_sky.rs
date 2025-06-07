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

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::PerpendicularVector;
pub use crate::src::qcommon::q_math::Q_acos;
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
pub use crate::src::renderergl1::tr_sky::q_shared_h::CrossProduct;
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

pub use crate::qgl_h::Beginproc;
pub use crate::qgl_h::Color3fproc;
pub use crate::qgl_h::DepthRangeproc;
pub use crate::qgl_h::Endproc;
pub use crate::qgl_h::LoadMatrixfproc;
pub use crate::qgl_h::PopMatrixproc;
pub use crate::qgl_h::PushMatrixproc;
pub use crate::qgl_h::TexCoord2fvproc;
pub use crate::qgl_h::Translatefproc;
pub use crate::qgl_h::Vertex3fvproc;
pub use crate::src::renderergl1::tr_backend::backEnd;
pub use crate::src::renderergl1::tr_backend::GL_Bind;
pub use crate::src::renderergl1::tr_backend::GL_Cull;
pub use crate::src::renderergl1::tr_backend::GL_State;
pub use crate::src::renderergl1::tr_init::r_fastsky;
pub use crate::src::renderergl1::tr_init::r_showsky;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_shade::tess;
pub use crate::src::renderergl1::tr_shade::RB_BeginSurface;
pub use crate::src::renderergl1::tr_shade::RB_EndSurface;
pub use crate::src::renderergl1::tr_shade::RB_StageIteratorGeneric;
pub use crate::src::renderergl1::tr_surface::RB_AddQuadStamp;
pub use crate::src::sdl::sdl_glimp::qglBegin;
pub use crate::src::sdl::sdl_glimp::qglColor3f;
pub use crate::src::sdl::sdl_glimp::qglDepthRange;
pub use crate::src::sdl::sdl_glimp::qglEnd;
pub use crate::src::sdl::sdl_glimp::qglLoadMatrixf;
pub use crate::src::sdl::sdl_glimp::qglPopMatrix;
pub use crate::src::sdl::sdl_glimp::qglPushMatrix;
pub use crate::src::sdl::sdl_glimp::qglTexCoord2fv;
pub use crate::src::sdl::sdl_glimp::qglTranslatef;
pub use crate::src::sdl::sdl_glimp::qglVertex3fv;

pub use crate::stdlib::GLclampd;
pub use crate::stdlib::GLenum;
pub use crate::stdlib::GLfloat;
pub use crate::stdlib::GLuint;
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

static mut s_cloudTexCoords: [[[[libc::c_float; 2]; 9]; 9]; 6] = [[[[0.; 2]; 9]; 9]; 6];

static mut s_cloudTexP: [[[libc::c_float; 9]; 9]; 6] = [[[0.; 9]; 9]; 6];
/*
===================================================================================

POLYGON TO BOX SIDE PROJECTION

===================================================================================
*/

static mut sky_clip: [crate::src::qcommon::q_shared::vec3_t; 6] = [
    [
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        -(1 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        -(1 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ],
    [
        -(1 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ],
];

static mut sky_mins: [[libc::c_float; 6]; 2] = [[0.; 6]; 2];

static mut sky_maxs: [[libc::c_float; 6]; 2] = [[0.; 6]; 2];

static mut sky_min: libc::c_float = 0.;

static mut sky_max: libc::c_float = 0.;
/*
================
AddSkyPolygon
================
*/

unsafe extern "C" fn AddSkyPolygon(
    mut nump: libc::c_int,
    mut vecs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut av: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut axis: libc::c_int = 0;
    let mut vp: *mut libc::c_float = 0 as *mut libc::c_float;
    // s = [0]/[2], t = [1]/[2]
    static mut vec_to_st: [[libc::c_int; 3]; 6] = [
        [-(2 as libc::c_int), 3 as libc::c_int, 1 as libc::c_int],
        [2 as libc::c_int, 3 as libc::c_int, -(1 as libc::c_int)],
        [1 as libc::c_int, 3 as libc::c_int, 2 as libc::c_int],
        [-(1 as libc::c_int), 3 as libc::c_int, -(2 as libc::c_int)],
        [-(2 as libc::c_int), -(1 as libc::c_int), 3 as libc::c_int],
        [-(2 as libc::c_int), 1 as libc::c_int, -(3 as libc::c_int)],
    ];
    // decide which face it maps to
    v[0 as libc::c_int as usize] =
        crate::src::qcommon::q_math::vec3_origin[0 as libc::c_int as usize];
    v[1 as libc::c_int as usize] =
        crate::src::qcommon::q_math::vec3_origin[1 as libc::c_int as usize];
    v[2 as libc::c_int as usize] =
        crate::src::qcommon::q_math::vec3_origin[2 as libc::c_int as usize];
    i = 0 as libc::c_int;
    vp = vecs;
    while i < nump {
        v[0 as libc::c_int as usize] =
            *vp.offset(0 as libc::c_int as isize) + v[0 as libc::c_int as usize];
        v[1 as libc::c_int as usize] =
            *vp.offset(1 as libc::c_int as isize) + v[1 as libc::c_int as usize];
        v[2 as libc::c_int as usize] =
            *vp.offset(2 as libc::c_int as isize) + v[2 as libc::c_int as usize];
        i += 1;
        vp = vp.offset(3 as libc::c_int as isize)
    }
    av[0 as libc::c_int as usize] =
        crate::stdlib::fabs(v[0 as libc::c_int as usize] as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
    av[1 as libc::c_int as usize] =
        crate::stdlib::fabs(v[1 as libc::c_int as usize] as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
    av[2 as libc::c_int as usize] =
        crate::stdlib::fabs(v[2 as libc::c_int as usize] as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
    if av[0 as libc::c_int as usize] > av[1 as libc::c_int as usize]
        && av[0 as libc::c_int as usize] > av[2 as libc::c_int as usize]
    {
        if v[0 as libc::c_int as usize] < 0 as libc::c_int as libc::c_float {
            axis = 1 as libc::c_int
        } else {
            axis = 0 as libc::c_int
        }
    } else if av[1 as libc::c_int as usize] > av[2 as libc::c_int as usize]
        && av[1 as libc::c_int as usize] > av[0 as libc::c_int as usize]
    {
        if v[1 as libc::c_int as usize] < 0 as libc::c_int as libc::c_float {
            axis = 3 as libc::c_int
        } else {
            axis = 2 as libc::c_int
        }
    } else if v[2 as libc::c_int as usize] < 0 as libc::c_int as libc::c_float {
        axis = 5 as libc::c_int
    } else {
        axis = 4 as libc::c_int
    }
    // project new texture coords
    i = 0 as libc::c_int; // don't divide by zero
    while i < nump {
        j = vec_to_st[axis as usize][2 as libc::c_int as usize];
        if j > 0 as libc::c_int {
            dv = *vecs.offset((j - 1 as libc::c_int) as isize)
        } else {
            dv = -*vecs.offset((-j - 1 as libc::c_int) as isize)
        }
        if !((dv as libc::c_double) < 0.001f64) {
            j = vec_to_st[axis as usize][0 as libc::c_int as usize];
            if j < 0 as libc::c_int {
                s = -*vecs.offset((-j - 1 as libc::c_int) as isize) / dv
            } else {
                s = *vecs.offset((j - 1 as libc::c_int) as isize) / dv
            }
            j = vec_to_st[axis as usize][1 as libc::c_int as usize];
            if j < 0 as libc::c_int {
                t = -*vecs.offset((-j - 1 as libc::c_int) as isize) / dv
            } else {
                t = *vecs.offset((j - 1 as libc::c_int) as isize) / dv
            }
            if s < sky_mins[0 as libc::c_int as usize][axis as usize] {
                sky_mins[0 as libc::c_int as usize][axis as usize] = s
            }
            if t < sky_mins[1 as libc::c_int as usize][axis as usize] {
                sky_mins[1 as libc::c_int as usize][axis as usize] = t
            }
            if s > sky_maxs[0 as libc::c_int as usize][axis as usize] {
                sky_maxs[0 as libc::c_int as usize][axis as usize] = s
            }
            if t > sky_maxs[1 as libc::c_int as usize][axis as usize] {
                sky_maxs[1 as libc::c_int as usize][axis as usize] = t
            }
        }
        i += 1;
        vecs = vecs.offset(3 as libc::c_int as isize)
    }
}
/*
================
ClipSkyPolygon
================
*/

unsafe extern "C" fn ClipSkyPolygon(
    mut nump: libc::c_int,
    mut vecs: *mut crate::src::qcommon::q_shared::vec_t,
    mut stage: libc::c_int,
) {
    let mut norm: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut front: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut back: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut d: libc::c_float = 0.;
    let mut e: libc::c_float = 0.;
    let mut dists: [libc::c_float; 64] = [0.; 64];
    let mut sides: [libc::c_int; 64] = [0; 64];
    let mut newv: [[crate::src::qcommon::q_shared::vec3_t; 64]; 2] = [[[0.; 3]; 64]; 2];
    let mut newc: [libc::c_int; 2] = [0; 2];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if nump > 64 as libc::c_int - 2 as libc::c_int {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"ClipSkyPolygon: MAX_CLIP_VERTS\x00" as *const u8 as *const libc::c_char,
        );
    }
    if stage == 6 as libc::c_int {
        // fully clipped, so draw it
        AddSkyPolygon(nump, vecs);
        return;
    }
    back = crate::src::qcommon::q_shared::qfalse;
    front = back;
    norm = sky_clip[stage as usize].as_mut_ptr();
    i = 0 as libc::c_int;
    v = vecs;
    while i < nump {
        d = *v.offset(0 as libc::c_int as isize) * *norm.offset(0 as libc::c_int as isize)
            + *v.offset(1 as libc::c_int as isize) * *norm.offset(1 as libc::c_int as isize)
            + *v.offset(2 as libc::c_int as isize) * *norm.offset(2 as libc::c_int as isize);
        if d > 0.1f32 {
            front = crate::src::qcommon::q_shared::qtrue;
            sides[i as usize] = 0 as libc::c_int
        } else if d < -0.1f32 {
            back = crate::src::qcommon::q_shared::qtrue;
            sides[i as usize] = 1 as libc::c_int
        } else {
            sides[i as usize] = 2 as libc::c_int
        }
        dists[i as usize] = d;
        i += 1;
        v = v.offset(3 as libc::c_int as isize)
    }
    if front as u64 == 0 || back as u64 == 0 {
        // not clipped
        ClipSkyPolygon(nump, vecs, stage + 1 as libc::c_int);
        return;
    }
    // clip it
    sides[i as usize] = sides[0 as libc::c_int as usize];
    dists[i as usize] = dists[0 as libc::c_int as usize];
    *vecs
        .offset((i * 3 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) = *vecs.offset(0 as libc::c_int as isize);
    *vecs
        .offset((i * 3 as libc::c_int) as isize)
        .offset(1 as libc::c_int as isize) = *vecs.offset(1 as libc::c_int as isize);
    *vecs
        .offset((i * 3 as libc::c_int) as isize)
        .offset(2 as libc::c_int as isize) = *vecs.offset(2 as libc::c_int as isize);
    newc[1 as libc::c_int as usize] = 0 as libc::c_int;
    newc[0 as libc::c_int as usize] = newc[1 as libc::c_int as usize];
    i = 0 as libc::c_int;
    v = vecs;
    while i < nump {
        match sides[i as usize] {
            0 => {
                newv[0 as libc::c_int as usize][newc[0 as libc::c_int as usize] as usize]
                    [0 as libc::c_int as usize] = *v.offset(0 as libc::c_int as isize);
                newv[0 as libc::c_int as usize][newc[0 as libc::c_int as usize] as usize]
                    [1 as libc::c_int as usize] = *v.offset(1 as libc::c_int as isize);
                newv[0 as libc::c_int as usize][newc[0 as libc::c_int as usize] as usize]
                    [2 as libc::c_int as usize] = *v.offset(2 as libc::c_int as isize);
                newc[0 as libc::c_int as usize] += 1
            }
            1 => {
                newv[1 as libc::c_int as usize][newc[1 as libc::c_int as usize] as usize]
                    [0 as libc::c_int as usize] = *v.offset(0 as libc::c_int as isize);
                newv[1 as libc::c_int as usize][newc[1 as libc::c_int as usize] as usize]
                    [1 as libc::c_int as usize] = *v.offset(1 as libc::c_int as isize);
                newv[1 as libc::c_int as usize][newc[1 as libc::c_int as usize] as usize]
                    [2 as libc::c_int as usize] = *v.offset(2 as libc::c_int as isize);
                newc[1 as libc::c_int as usize] += 1
            }
            2 => {
                newv[0 as libc::c_int as usize][newc[0 as libc::c_int as usize] as usize]
                    [0 as libc::c_int as usize] = *v.offset(0 as libc::c_int as isize);
                newv[0 as libc::c_int as usize][newc[0 as libc::c_int as usize] as usize]
                    [1 as libc::c_int as usize] = *v.offset(1 as libc::c_int as isize);
                newv[0 as libc::c_int as usize][newc[0 as libc::c_int as usize] as usize]
                    [2 as libc::c_int as usize] = *v.offset(2 as libc::c_int as isize);
                newc[0 as libc::c_int as usize] += 1;
                newv[1 as libc::c_int as usize][newc[1 as libc::c_int as usize] as usize]
                    [0 as libc::c_int as usize] = *v.offset(0 as libc::c_int as isize);
                newv[1 as libc::c_int as usize][newc[1 as libc::c_int as usize] as usize]
                    [1 as libc::c_int as usize] = *v.offset(1 as libc::c_int as isize);
                newv[1 as libc::c_int as usize][newc[1 as libc::c_int as usize] as usize]
                    [2 as libc::c_int as usize] = *v.offset(2 as libc::c_int as isize);
                newc[1 as libc::c_int as usize] += 1
            }
            _ => {}
        }
        if !(sides[i as usize] == 2 as libc::c_int
            || sides[(i + 1 as libc::c_int) as usize] == 2 as libc::c_int
            || sides[(i + 1 as libc::c_int) as usize] == sides[i as usize])
        {
            d = dists[i as usize] / (dists[i as usize] - dists[(i + 1 as libc::c_int) as usize]);
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                e = *v.offset(j as isize)
                    + d * (*v.offset((j + 3 as libc::c_int) as isize) - *v.offset(j as isize));
                newv[0 as libc::c_int as usize][newc[0 as libc::c_int as usize] as usize]
                    [j as usize] = e;
                newv[1 as libc::c_int as usize][newc[1 as libc::c_int as usize] as usize]
                    [j as usize] = e;
                j += 1
            }
            newc[0 as libc::c_int as usize] += 1;
            newc[1 as libc::c_int as usize] += 1
        }
        i += 1;
        v = v.offset(3 as libc::c_int as isize)
    }
    // continue
    ClipSkyPolygon(
        newc[0 as libc::c_int as usize],
        newv[0 as libc::c_int as usize][0 as libc::c_int as usize].as_mut_ptr(),
        stage + 1 as libc::c_int,
    );
    ClipSkyPolygon(
        newc[1 as libc::c_int as usize],
        newv[1 as libc::c_int as usize][0 as libc::c_int as usize].as_mut_ptr(),
        stage + 1 as libc::c_int,
    );
}
/*
==============
ClearSkyBox
==============
*/

unsafe extern "C" fn ClearSkyBox() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        sky_mins[1 as libc::c_int as usize][i as usize] = 9999 as libc::c_int as libc::c_float;
        sky_mins[0 as libc::c_int as usize][i as usize] =
            sky_mins[1 as libc::c_int as usize][i as usize];
        sky_maxs[1 as libc::c_int as usize][i as usize] = -(9999 as libc::c_int) as libc::c_float;
        sky_maxs[0 as libc::c_int as usize][i as usize] =
            sky_maxs[1 as libc::c_int as usize][i as usize];
        i += 1
    }
}
/*
================
RB_ClipSkyPolygons
================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_ClipSkyPolygons(mut input: *mut crate::tr_local_h::shaderCommands_t) {
    let mut p: [crate::src::qcommon::q_shared::vec3_t; 5] = [[0.; 3]; 5]; // need one extra point for clipping
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    ClearSkyBox();
    i = 0 as libc::c_int;
    while i < (*input).numIndexes {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            p[j as usize][0 as libc::c_int as usize] = (*input).xyz
                [(*input).indexes[(i + j) as usize] as usize][0 as libc::c_int as usize]
                - crate::src::renderergl1::tr_backend::backEnd
                    .viewParms
                    .or
                    .origin[0 as libc::c_int as usize];
            p[j as usize][1 as libc::c_int as usize] = (*input).xyz
                [(*input).indexes[(i + j) as usize] as usize][1 as libc::c_int as usize]
                - crate::src::renderergl1::tr_backend::backEnd
                    .viewParms
                    .or
                    .origin[1 as libc::c_int as usize];
            p[j as usize][2 as libc::c_int as usize] = (*input).xyz
                [(*input).indexes[(i + j) as usize] as usize][2 as libc::c_int as usize]
                - crate::src::renderergl1::tr_backend::backEnd
                    .viewParms
                    .or
                    .origin[2 as libc::c_int as usize];
            j += 1
        }
        ClipSkyPolygon(
            3 as libc::c_int,
            p[0 as libc::c_int as usize].as_mut_ptr(),
            0 as libc::c_int,
        );
        i += 3 as libc::c_int
    }
}
/*
===================================================================================

CLOUD VERTEX GENERATION

===================================================================================
*/
/*
** MakeSkyVec
**
** Parms: s, t range from -1 to 1
*/

unsafe extern "C" fn MakeSkyVec(
    mut s: libc::c_float,
    mut t: libc::c_float,
    mut axis: libc::c_int,
    mut outSt: *mut libc::c_float,
    mut outXYZ: *mut crate::src::qcommon::q_shared::vec_t,
) {
    // 1 = s, 2 = t, 3 = 2048
    static mut st_to_vec: [[libc::c_int; 3]; 6] = [
        [3 as libc::c_int, -(1 as libc::c_int), 2 as libc::c_int],
        [-(3 as libc::c_int), 1 as libc::c_int, 2 as libc::c_int],
        [1 as libc::c_int, 3 as libc::c_int, 2 as libc::c_int],
        [-(1 as libc::c_int), -(3 as libc::c_int), 2 as libc::c_int],
        [-(2 as libc::c_int), -(1 as libc::c_int), 3 as libc::c_int],
        [2 as libc::c_int, -(1 as libc::c_int), -(3 as libc::c_int)],
    ]; // div sqrt(3)
    let mut b: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut boxSize: libc::c_float = 0.;
    boxSize = (crate::src::renderergl1::tr_backend::backEnd.viewParms.zFar as libc::c_double
        / 1.75f64) as libc::c_float;
    b[0 as libc::c_int as usize] = s * boxSize;
    b[1 as libc::c_int as usize] = t * boxSize;
    b[2 as libc::c_int as usize] = boxSize;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        k = st_to_vec[axis as usize][j as usize];
        if k < 0 as libc::c_int {
            *outXYZ.offset(j as isize) = -b[(-k - 1 as libc::c_int) as usize]
        } else {
            *outXYZ.offset(j as isize) = b[(k - 1 as libc::c_int) as usize]
        }
        j += 1
    }
    // avoid bilerp seam
    s = ((s + 1 as libc::c_int as libc::c_float) as libc::c_double * 0.5f64) as libc::c_float;
    t = ((t + 1 as libc::c_int as libc::c_float) as libc::c_double * 0.5f64) as libc::c_float;
    if s < sky_min {
        s = sky_min
    } else if s > sky_max {
        s = sky_max
    }
    if t < sky_min {
        t = sky_min
    } else if t > sky_max {
        t = sky_max
    }
    t = (1.0f64 - t as libc::c_double) as libc::c_float;
    if !outSt.is_null() {
        *outSt.offset(0 as libc::c_int as isize) = s;
        *outSt.offset(1 as libc::c_int as isize) = t
    };
}

static mut sky_texorder: [libc::c_int; 6] = [
    0 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
];

static mut s_skyPoints: [[crate::src::qcommon::q_shared::vec3_t; 9]; 9] = [[[0.; 3]; 9]; 9];

static mut s_skyTexCoords: [[[libc::c_float; 2]; 9]; 9] = [[[0.; 2]; 9]; 9];

unsafe extern "C" fn DrawSkySide(
    mut image: *mut crate::tr_common_h::image_s,
    mut mins: *const libc::c_int,
    mut maxs: *const libc::c_int,
) {
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    crate::src::renderergl1::tr_backend::GL_Bind(image as *mut crate::tr_common_h::image_s);
    t = *mins.offset(1 as libc::c_int as isize) + 8 as libc::c_int / 2 as libc::c_int;
    while t < *maxs.offset(1 as libc::c_int as isize) + 8 as libc::c_int / 2 as libc::c_int {
        crate::src::sdl::sdl_glimp::qglBegin.expect("non-null function pointer")(
            0x5 as libc::c_int as crate::stdlib::GLenum,
        );
        s = *mins.offset(0 as libc::c_int as isize) + 8 as libc::c_int / 2 as libc::c_int;
        while s <= *maxs.offset(0 as libc::c_int as isize) + 8 as libc::c_int / 2 as libc::c_int {
            crate::src::sdl::sdl_glimp::qglTexCoord2fv.expect("non-null function pointer")(
                s_skyTexCoords[t as usize][s as usize].as_mut_ptr(),
            );
            crate::src::sdl::sdl_glimp::qglVertex3fv.expect("non-null function pointer")(
                s_skyPoints[t as usize][s as usize].as_mut_ptr(),
            );
            crate::src::sdl::sdl_glimp::qglTexCoord2fv.expect("non-null function pointer")(
                s_skyTexCoords[(t + 1 as libc::c_int) as usize][s as usize].as_mut_ptr(),
            );
            crate::src::sdl::sdl_glimp::qglVertex3fv.expect("non-null function pointer")(
                s_skyPoints[(t + 1 as libc::c_int) as usize][s as usize].as_mut_ptr(),
            );
            s += 1
        }
        crate::src::sdl::sdl_glimp::qglEnd.expect("non-null function pointer")();
        t += 1
    }
}

unsafe extern "C" fn DrawSkyBox(mut shader: *mut crate::tr_local_h::shader_t) {
    let mut i: libc::c_int = 0;
    sky_min = 0 as libc::c_int as libc::c_float;
    sky_max = 1 as libc::c_int as libc::c_float;
    crate::stdlib::memset(
        s_skyTexCoords.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[[[libc::c_float; 2]; 9]; 9]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        let mut sky_mins_subd: [libc::c_int; 2] = [0; 2];
        let mut sky_maxs_subd: [libc::c_int; 2] = [0; 2];
        let mut s: libc::c_int = 0;
        let mut t: libc::c_int = 0;
        sky_mins[0 as libc::c_int as usize][i as usize] = (crate::stdlib::floor(
            (sky_mins[0 as libc::c_int as usize][i as usize]
                * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_double,
        ) / (8 as libc::c_int / 2 as libc::c_int)
            as libc::c_double)
            as libc::c_float;
        sky_mins[1 as libc::c_int as usize][i as usize] = (crate::stdlib::floor(
            (sky_mins[1 as libc::c_int as usize][i as usize]
                * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_double,
        ) / (8 as libc::c_int / 2 as libc::c_int)
            as libc::c_double)
            as libc::c_float;
        sky_maxs[0 as libc::c_int as usize][i as usize] = (crate::stdlib::ceil(
            (sky_maxs[0 as libc::c_int as usize][i as usize]
                * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_double,
        ) / (8 as libc::c_int / 2 as libc::c_int)
            as libc::c_double)
            as libc::c_float;
        sky_maxs[1 as libc::c_int as usize][i as usize] = (crate::stdlib::ceil(
            (sky_maxs[1 as libc::c_int as usize][i as usize]
                * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_double,
        ) / (8 as libc::c_int / 2 as libc::c_int)
            as libc::c_double)
            as libc::c_float;
        if !(sky_mins[0 as libc::c_int as usize][i as usize]
            >= sky_maxs[0 as libc::c_int as usize][i as usize]
            || sky_mins[1 as libc::c_int as usize][i as usize]
                >= sky_maxs[1 as libc::c_int as usize][i as usize])
        {
            sky_mins_subd[0 as libc::c_int as usize] = (sky_mins[0 as libc::c_int as usize]
                [i as usize]
                * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_int;
            sky_mins_subd[1 as libc::c_int as usize] = (sky_mins[1 as libc::c_int as usize]
                [i as usize]
                * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_int;
            sky_maxs_subd[0 as libc::c_int as usize] = (sky_maxs[0 as libc::c_int as usize]
                [i as usize]
                * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_int;
            sky_maxs_subd[1 as libc::c_int as usize] = (sky_maxs[1 as libc::c_int as usize]
                [i as usize]
                * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                as libc::c_int;
            if sky_mins_subd[0 as libc::c_int as usize] < -(8 as libc::c_int / 2 as libc::c_int) {
                sky_mins_subd[0 as libc::c_int as usize] = -(8 as libc::c_int / 2 as libc::c_int)
            } else if sky_mins_subd[0 as libc::c_int as usize] > 8 as libc::c_int / 2 as libc::c_int
            {
                sky_mins_subd[0 as libc::c_int as usize] = 8 as libc::c_int / 2 as libc::c_int
            }
            if sky_mins_subd[1 as libc::c_int as usize] < -(8 as libc::c_int / 2 as libc::c_int) {
                sky_mins_subd[1 as libc::c_int as usize] = -(8 as libc::c_int / 2 as libc::c_int)
            } else if sky_mins_subd[1 as libc::c_int as usize] > 8 as libc::c_int / 2 as libc::c_int
            {
                sky_mins_subd[1 as libc::c_int as usize] = 8 as libc::c_int / 2 as libc::c_int
            }
            if sky_maxs_subd[0 as libc::c_int as usize] < -(8 as libc::c_int / 2 as libc::c_int) {
                sky_maxs_subd[0 as libc::c_int as usize] = -(8 as libc::c_int / 2 as libc::c_int)
            } else if sky_maxs_subd[0 as libc::c_int as usize] > 8 as libc::c_int / 2 as libc::c_int
            {
                sky_maxs_subd[0 as libc::c_int as usize] = 8 as libc::c_int / 2 as libc::c_int
            }
            if sky_maxs_subd[1 as libc::c_int as usize] < -(8 as libc::c_int / 2 as libc::c_int) {
                sky_maxs_subd[1 as libc::c_int as usize] = -(8 as libc::c_int / 2 as libc::c_int)
            } else if sky_maxs_subd[1 as libc::c_int as usize] > 8 as libc::c_int / 2 as libc::c_int
            {
                sky_maxs_subd[1 as libc::c_int as usize] = 8 as libc::c_int / 2 as libc::c_int
            }
            //
            // iterate through the subdivisions
            //
            t = sky_mins_subd[1 as libc::c_int as usize] + 8 as libc::c_int / 2 as libc::c_int;
            while t
                <= sky_maxs_subd[1 as libc::c_int as usize] + 8 as libc::c_int / 2 as libc::c_int
            {
                s = sky_mins_subd[0 as libc::c_int as usize] + 8 as libc::c_int / 2 as libc::c_int;
                while s
                    <= sky_maxs_subd[0 as libc::c_int as usize]
                        + 8 as libc::c_int / 2 as libc::c_int
                {
                    MakeSkyVec(
                        (s - 8 as libc::c_int / 2 as libc::c_int) as libc::c_float
                            / (8 as libc::c_int / 2 as libc::c_int) as libc::c_float,
                        (t - 8 as libc::c_int / 2 as libc::c_int) as libc::c_float
                            / (8 as libc::c_int / 2 as libc::c_int) as libc::c_float,
                        i,
                        s_skyTexCoords[t as usize][s as usize].as_mut_ptr(),
                        s_skyPoints[t as usize][s as usize].as_mut_ptr(),
                    );
                    s += 1
                }
                t += 1
            }
            DrawSkySide(
                (*shader).sky.outerbox[sky_texorder[i as usize] as usize],
                sky_mins_subd.as_mut_ptr() as *const libc::c_int,
                sky_maxs_subd.as_mut_ptr() as *const libc::c_int,
            );
        }
        i += 1
    }
}

unsafe extern "C" fn FillCloudySkySide(
    mut mins: *const libc::c_int,
    mut maxs: *const libc::c_int,
    mut addIndexes: crate::src::qcommon::q_shared::qboolean,
) {
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut vertexStart: libc::c_int = crate::src::renderergl1::tr_shade::tess.numVertexes;
    let mut tHeight: libc::c_int = 0;
    let mut sWidth: libc::c_int = 0;
    tHeight = *maxs.offset(1 as libc::c_int as isize) - *mins.offset(1 as libc::c_int as isize)
        + 1 as libc::c_int;
    sWidth = *maxs.offset(0 as libc::c_int as isize) - *mins.offset(0 as libc::c_int as isize)
        + 1 as libc::c_int;
    t = *mins.offset(1 as libc::c_int as isize) + 8 as libc::c_int / 2 as libc::c_int;
    while t <= *maxs.offset(1 as libc::c_int as isize) + 8 as libc::c_int / 2 as libc::c_int {
        s = *mins.offset(0 as libc::c_int as isize) + 8 as libc::c_int / 2 as libc::c_int;
        while s <= *maxs.offset(0 as libc::c_int as isize) + 8 as libc::c_int / 2 as libc::c_int {
            crate::src::renderergl1::tr_shade::tess.xyz
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize]
                [0 as libc::c_int as usize] = s_skyPoints[t as usize][s as usize]
                [0 as libc::c_int as usize]
                + crate::src::renderergl1::tr_backend::backEnd
                    .viewParms
                    .or
                    .origin[0 as libc::c_int as usize];
            crate::src::renderergl1::tr_shade::tess.xyz
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize]
                [1 as libc::c_int as usize] = s_skyPoints[t as usize][s as usize]
                [1 as libc::c_int as usize]
                + crate::src::renderergl1::tr_backend::backEnd
                    .viewParms
                    .or
                    .origin[1 as libc::c_int as usize];
            crate::src::renderergl1::tr_shade::tess.xyz
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize]
                [2 as libc::c_int as usize] = s_skyPoints[t as usize][s as usize]
                [2 as libc::c_int as usize]
                + crate::src::renderergl1::tr_backend::backEnd
                    .viewParms
                    .or
                    .origin[2 as libc::c_int as usize];
            crate::src::renderergl1::tr_shade::tess.texCoords
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize]
                [0 as libc::c_int as usize][0 as libc::c_int as usize] =
                s_skyTexCoords[t as usize][s as usize][0 as libc::c_int as usize];
            crate::src::renderergl1::tr_shade::tess.texCoords
                [crate::src::renderergl1::tr_shade::tess.numVertexes as usize]
                [0 as libc::c_int as usize][1 as libc::c_int as usize] =
                s_skyTexCoords[t as usize][s as usize][1 as libc::c_int as usize];
            crate::src::renderergl1::tr_shade::tess.numVertexes += 1;
            if crate::src::renderergl1::tr_shade::tess.numVertexes >= 1000 as libc::c_int {
                crate::src::renderergl1::tr_main::ri
                    .Error
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                    b"SHADER_MAX_VERTEXES hit in FillCloudySkySide()\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            s += 1
        }
        t += 1
    }
    // only add indexes for one pass, otherwise it would draw multiple times for each pass
    if addIndexes as u64 != 0 {
        t = 0 as libc::c_int;
        while t < tHeight - 1 as libc::c_int {
            s = 0 as libc::c_int;
            while s < sWidth - 1 as libc::c_int {
                crate::src::renderergl1::tr_shade::tess.indexes
                    [crate::src::renderergl1::tr_shade::tess.numIndexes as usize] =
                    (vertexStart + s + t * sWidth) as crate::tr_local_h::glIndex_t;
                crate::src::renderergl1::tr_shade::tess.numIndexes += 1;
                crate::src::renderergl1::tr_shade::tess.indexes
                    [crate::src::renderergl1::tr_shade::tess.numIndexes as usize] =
                    (vertexStart + s + (t + 1 as libc::c_int) * sWidth)
                        as crate::tr_local_h::glIndex_t;
                crate::src::renderergl1::tr_shade::tess.numIndexes += 1;
                crate::src::renderergl1::tr_shade::tess.indexes
                    [crate::src::renderergl1::tr_shade::tess.numIndexes as usize] =
                    (vertexStart + s + 1 as libc::c_int + t * sWidth)
                        as crate::tr_local_h::glIndex_t;
                crate::src::renderergl1::tr_shade::tess.numIndexes += 1;
                crate::src::renderergl1::tr_shade::tess.indexes
                    [crate::src::renderergl1::tr_shade::tess.numIndexes as usize] =
                    (vertexStart + s + (t + 1 as libc::c_int) * sWidth)
                        as crate::tr_local_h::glIndex_t;
                crate::src::renderergl1::tr_shade::tess.numIndexes += 1;
                crate::src::renderergl1::tr_shade::tess.indexes
                    [crate::src::renderergl1::tr_shade::tess.numIndexes as usize] =
                    (vertexStart + s + 1 as libc::c_int + (t + 1 as libc::c_int) * sWidth)
                        as crate::tr_local_h::glIndex_t;
                crate::src::renderergl1::tr_shade::tess.numIndexes += 1;
                crate::src::renderergl1::tr_shade::tess.indexes
                    [crate::src::renderergl1::tr_shade::tess.numIndexes as usize] =
                    (vertexStart + s + 1 as libc::c_int + t * sWidth)
                        as crate::tr_local_h::glIndex_t;
                crate::src::renderergl1::tr_shade::tess.numIndexes += 1;
                s += 1
            }
            t += 1
        }
    };
}

unsafe extern "C" fn FillCloudBox(
    mut _shader: *const crate::tr_local_h::shader_t,
    mut stage: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        let mut sky_mins_subd: [libc::c_int; 2] = [0; 2];
        let mut sky_maxs_subd: [libc::c_int; 2] = [0; 2];
        let mut s: libc::c_int = 0;
        let mut t: libc::c_int = 0;
        let mut MIN_T: libc::c_float = 0.;
        // FIXME? shader->sky.fullClouds )
        MIN_T = -(8 as libc::c_int / 2 as libc::c_int) as libc::c_float;
        // still don't want to draw the bottom, even if fullClouds
        if !(i == 5 as libc::c_int) {
            sky_mins[0 as libc::c_int as usize][i as usize] =
                (crate::stdlib::floor(
                    (sky_mins[0 as libc::c_int as usize][i as usize]
                        * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                        as libc::c_double,
                ) / (8 as libc::c_int / 2 as libc::c_int) as libc::c_double)
                    as libc::c_float;
            sky_mins[1 as libc::c_int as usize][i as usize] =
                (crate::stdlib::floor(
                    (sky_mins[1 as libc::c_int as usize][i as usize]
                        * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                        as libc::c_double,
                ) / (8 as libc::c_int / 2 as libc::c_int) as libc::c_double)
                    as libc::c_float;
            sky_maxs[0 as libc::c_int as usize][i as usize] =
                (crate::stdlib::ceil(
                    (sky_maxs[0 as libc::c_int as usize][i as usize]
                        * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                        as libc::c_double,
                ) / (8 as libc::c_int / 2 as libc::c_int) as libc::c_double)
                    as libc::c_float;
            sky_maxs[1 as libc::c_int as usize][i as usize] =
                (crate::stdlib::ceil(
                    (sky_maxs[1 as libc::c_int as usize][i as usize]
                        * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float)
                        as libc::c_double,
                ) / (8 as libc::c_int / 2 as libc::c_int) as libc::c_double)
                    as libc::c_float;
            if !(sky_mins[0 as libc::c_int as usize][i as usize]
                >= sky_maxs[0 as libc::c_int as usize][i as usize]
                || sky_mins[1 as libc::c_int as usize][i as usize]
                    >= sky_maxs[1 as libc::c_int as usize][i as usize])
            {
                sky_mins_subd[0 as libc::c_int as usize] = crate::src::renderergl1::tr_main::ri
                    .ftol
                    .expect("non-null function pointer")(
                    sky_mins[0 as libc::c_int as usize][i as usize]
                        * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float,
                ) as libc::c_int;
                sky_mins_subd[1 as libc::c_int as usize] = crate::src::renderergl1::tr_main::ri
                    .ftol
                    .expect("non-null function pointer")(
                    sky_mins[1 as libc::c_int as usize][i as usize]
                        * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float,
                ) as libc::c_int;
                sky_maxs_subd[0 as libc::c_int as usize] = crate::src::renderergl1::tr_main::ri
                    .ftol
                    .expect("non-null function pointer")(
                    sky_maxs[0 as libc::c_int as usize][i as usize]
                        * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float,
                ) as libc::c_int;
                sky_maxs_subd[1 as libc::c_int as usize] = crate::src::renderergl1::tr_main::ri
                    .ftol
                    .expect("non-null function pointer")(
                    sky_maxs[1 as libc::c_int as usize][i as usize]
                        * (8 as libc::c_int / 2 as libc::c_int) as libc::c_float,
                ) as libc::c_int;
                if sky_mins_subd[0 as libc::c_int as usize] < -(8 as libc::c_int / 2 as libc::c_int)
                {
                    sky_mins_subd[0 as libc::c_int as usize] =
                        -(8 as libc::c_int / 2 as libc::c_int)
                } else if sky_mins_subd[0 as libc::c_int as usize]
                    > 8 as libc::c_int / 2 as libc::c_int
                {
                    sky_mins_subd[0 as libc::c_int as usize] = 8 as libc::c_int / 2 as libc::c_int
                }
                if (sky_mins_subd[1 as libc::c_int as usize] as libc::c_float) < MIN_T {
                    sky_mins_subd[1 as libc::c_int as usize] = MIN_T as libc::c_int
                } else if sky_mins_subd[1 as libc::c_int as usize]
                    > 8 as libc::c_int / 2 as libc::c_int
                {
                    sky_mins_subd[1 as libc::c_int as usize] = 8 as libc::c_int / 2 as libc::c_int
                }
                if sky_maxs_subd[0 as libc::c_int as usize] < -(8 as libc::c_int / 2 as libc::c_int)
                {
                    sky_maxs_subd[0 as libc::c_int as usize] =
                        -(8 as libc::c_int / 2 as libc::c_int)
                } else if sky_maxs_subd[0 as libc::c_int as usize]
                    > 8 as libc::c_int / 2 as libc::c_int
                {
                    sky_maxs_subd[0 as libc::c_int as usize] = 8 as libc::c_int / 2 as libc::c_int
                }
                if (sky_maxs_subd[1 as libc::c_int as usize] as libc::c_float) < MIN_T {
                    sky_maxs_subd[1 as libc::c_int as usize] = MIN_T as libc::c_int
                } else if sky_maxs_subd[1 as libc::c_int as usize]
                    > 8 as libc::c_int / 2 as libc::c_int
                {
                    sky_maxs_subd[1 as libc::c_int as usize] = 8 as libc::c_int / 2 as libc::c_int
                }
                //
                // iterate through the subdivisions
                //
                t = sky_mins_subd[1 as libc::c_int as usize] + 8 as libc::c_int / 2 as libc::c_int;
                while t
                    <= sky_maxs_subd[1 as libc::c_int as usize]
                        + 8 as libc::c_int / 2 as libc::c_int
                {
                    s = sky_mins_subd[0 as libc::c_int as usize]
                        + 8 as libc::c_int / 2 as libc::c_int;
                    while s
                        <= sky_maxs_subd[0 as libc::c_int as usize]
                            + 8 as libc::c_int / 2 as libc::c_int
                    {
                        MakeSkyVec(
                            (s - 8 as libc::c_int / 2 as libc::c_int) as libc::c_float
                                / (8 as libc::c_int / 2 as libc::c_int) as libc::c_float,
                            (t - 8 as libc::c_int / 2 as libc::c_int) as libc::c_float
                                / (8 as libc::c_int / 2 as libc::c_int) as libc::c_float,
                            i,
                            0 as *mut libc::c_float,
                            s_skyPoints[t as usize][s as usize].as_mut_ptr(),
                        );
                        s_skyTexCoords[t as usize][s as usize][0 as libc::c_int as usize] =
                            s_cloudTexCoords[i as usize][t as usize][s as usize]
                                [0 as libc::c_int as usize];
                        s_skyTexCoords[t as usize][s as usize][1 as libc::c_int as usize] =
                            s_cloudTexCoords[i as usize][t as usize][s as usize]
                                [1 as libc::c_int as usize];
                        s += 1
                    }
                    t += 1
                }
                // only add indexes for first stage
                FillCloudySkySide(
                    sky_mins_subd.as_mut_ptr() as *const libc::c_int,
                    sky_maxs_subd.as_mut_ptr() as *const libc::c_int,
                    (stage == 0 as libc::c_int) as libc::c_int
                        as crate::src::qcommon::q_shared::qboolean,
                );
            }
        }
        // don't draw clouds beneath you
        i += 1
    }
}
/*
** R_BuildCloudData
*/
#[no_mangle]

pub unsafe extern "C" fn R_BuildCloudData(mut input: *mut crate::tr_local_h::shaderCommands_t) {
    let mut i: libc::c_int = 0; // FIXME: not correct?
    let mut shader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    shader = (*input).shader;
    sky_min = (1.0f64 / 256.0f32 as libc::c_double) as libc::c_float;
    sky_max = (255.0f64 / 256.0f32 as libc::c_double) as libc::c_float;
    // set up for drawing
    crate::src::renderergl1::tr_shade::tess.numIndexes = 0 as libc::c_int;
    crate::src::renderergl1::tr_shade::tess.numVertexes = 0 as libc::c_int;
    if (*shader).sky.cloudHeight != 0. {
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            if (*crate::src::renderergl1::tr_shade::tess
                .xstages
                .offset(i as isize))
            .is_null()
            {
                break;
            }
            FillCloudBox(shader, i);
            i += 1
        }
    };
}
/*
** R_InitSkyTexCoords
** Called when a sky shader is parsed
*/
#[no_mangle]

pub unsafe extern "C" fn R_InitSkyTexCoords(mut heightCloud: libc::c_float) {
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut radiusWorld: libc::c_float = 4096 as libc::c_int as libc::c_float;
    let mut p: libc::c_float = 0.;
    let mut sRad: libc::c_float = 0.;
    let mut tRad: libc::c_float = 0.;
    let mut skyVec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // init zfar so MakeSkyVec works even though
    // a world hasn't been bounded
    crate::src::renderergl1::tr_backend::backEnd.viewParms.zFar =
        1024 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        t = 0 as libc::c_int;
        while t <= 8 as libc::c_int {
            s = 0 as libc::c_int;
            while s <= 8 as libc::c_int {
                // compute vector from view origin to sky side integral point
                MakeSkyVec(
                    (s - 8 as libc::c_int / 2 as libc::c_int) as libc::c_float
                        / (8 as libc::c_int / 2 as libc::c_int) as libc::c_float,
                    (t - 8 as libc::c_int / 2 as libc::c_int) as libc::c_float
                        / (8 as libc::c_int / 2 as libc::c_int) as libc::c_float,
                    i,
                    0 as *mut libc::c_float,
                    skyVec.as_mut_ptr(),
                );
                // compute parametric value 'p' that intersects with cloud layer
                p = ((1.0f32
                    / (2 as libc::c_int as libc::c_float
                        * (skyVec[0 as libc::c_int as usize] * skyVec[0 as libc::c_int as usize]
                            + skyVec[1 as libc::c_int as usize]
                                * skyVec[1 as libc::c_int as usize]
                            + skyVec[2 as libc::c_int as usize]
                                * skyVec[2 as libc::c_int as usize])))
                    as libc::c_double
                    * ((-(2 as libc::c_int) as libc::c_float
                        * skyVec[2 as libc::c_int as usize]
                        * radiusWorld) as libc::c_double
                        + 2 as libc::c_int as libc::c_double
                            * crate::stdlib::sqrt(
                                (skyVec[2 as libc::c_int as usize]
                                    * skyVec[2 as libc::c_int as usize]
                                    * (radiusWorld * radiusWorld)
                                    + 2 as libc::c_int as libc::c_float
                                        * (skyVec[0 as libc::c_int as usize]
                                            * skyVec[0 as libc::c_int as usize])
                                        * radiusWorld
                                        * heightCloud
                                    + skyVec[0 as libc::c_int as usize]
                                        * skyVec[0 as libc::c_int as usize]
                                        * (heightCloud * heightCloud)
                                    + 2 as libc::c_int as libc::c_float
                                        * (skyVec[1 as libc::c_int as usize]
                                            * skyVec[1 as libc::c_int as usize])
                                        * radiusWorld
                                        * heightCloud
                                    + skyVec[1 as libc::c_int as usize]
                                        * skyVec[1 as libc::c_int as usize]
                                        * (heightCloud * heightCloud)
                                    + 2 as libc::c_int as libc::c_float
                                        * (skyVec[2 as libc::c_int as usize]
                                            * skyVec[2 as libc::c_int as usize])
                                        * radiusWorld
                                        * heightCloud
                                    + skyVec[2 as libc::c_int as usize]
                                        * skyVec[2 as libc::c_int as usize]
                                        * (heightCloud * heightCloud))
                                    as libc::c_double,
                            ))) as libc::c_float;
                s_cloudTexP[i as usize][t as usize][s as usize] = p;
                // compute intersection point based on p
                v[0 as libc::c_int as usize] = skyVec[0 as libc::c_int as usize] * p;
                v[1 as libc::c_int as usize] = skyVec[1 as libc::c_int as usize] * p;
                v[2 as libc::c_int as usize] = skyVec[2 as libc::c_int as usize] * p;
                v[2 as libc::c_int as usize] += radiusWorld;
                // compute vector from world origin to intersection point 'v'
                crate::src::qcommon::q_math::VectorNormalize(v.as_mut_ptr());
                sRad = crate::src::qcommon::q_math::Q_acos(v[0 as libc::c_int as usize]);
                tRad = crate::src::qcommon::q_math::Q_acos(v[1 as libc::c_int as usize]);
                s_cloudTexCoords[i as usize][t as usize][s as usize][0 as libc::c_int as usize] =
                    sRad;
                s_cloudTexCoords[i as usize][t as usize][s as usize][1 as libc::c_int as usize] =
                    tRad;
                s += 1
            }
            t += 1
        }
        i += 1
    }
}
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
//======================================================================================
/*
** RB_DrawSun
*/
#[no_mangle]

pub unsafe extern "C" fn RB_DrawSun(
    mut scale: libc::c_float,
    mut shader: *mut crate::tr_local_h::shader_t,
) {
    let mut size: libc::c_float = 0.; // div sqrt(3)
    let mut dist: libc::c_float = 0.;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut sunColor: [crate::src::qcommon::q_shared::byte; 4] = [
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ];
    if crate::src::renderergl1::tr_backend::backEnd.skyRenderedThisView as u64 == 0 {
        return;
    }
    crate::src::sdl::sdl_glimp::qglLoadMatrixf.expect("non-null function pointer")(
        crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .world
            .modelMatrix
            .as_mut_ptr(),
    );
    crate::src::sdl::sdl_glimp::qglTranslatef.expect("non-null function pointer")(
        crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[0 as libc::c_int as usize],
        crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[1 as libc::c_int as usize],
        crate::src::renderergl1::tr_backend::backEnd
            .viewParms
            .or
            .origin[2 as libc::c_int as usize],
    );
    dist = (crate::src::renderergl1::tr_backend::backEnd.viewParms.zFar as libc::c_double / 1.75f64)
        as libc::c_float;
    size = dist * scale;
    origin[0 as libc::c_int as usize] =
        crate::src::renderergl1::tr_main::tr.sunDirection[0 as libc::c_int as usize] * dist;
    origin[1 as libc::c_int as usize] =
        crate::src::renderergl1::tr_main::tr.sunDirection[1 as libc::c_int as usize] * dist;
    origin[2 as libc::c_int as usize] =
        crate::src::renderergl1::tr_main::tr.sunDirection[2 as libc::c_int as usize] * dist;
    crate::src::qcommon::q_math::PerpendicularVector(
        vec1.as_mut_ptr(),
        crate::src::renderergl1::tr_main::tr
            .sunDirection
            .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    );
    CrossProduct(
        crate::src::renderergl1::tr_main::tr
            .sunDirection
            .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        vec1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        vec2.as_mut_ptr(),
    );
    vec1[0 as libc::c_int as usize] = vec1[0 as libc::c_int as usize] * size;
    vec1[1 as libc::c_int as usize] = vec1[1 as libc::c_int as usize] * size;
    vec1[2 as libc::c_int as usize] = vec1[2 as libc::c_int as usize] * size;
    vec2[0 as libc::c_int as usize] = vec2[0 as libc::c_int as usize] * size;
    vec2[1 as libc::c_int as usize] = vec2[1 as libc::c_int as usize] * size;
    vec2[2 as libc::c_int as usize] = vec2[2 as libc::c_int as usize] * size;
    // farthest depth range
    crate::src::sdl::sdl_glimp::qglDepthRange.expect("non-null function pointer")(1.0f64, 1.0f64);
    crate::src::renderergl1::tr_shade::RB_BeginSurface(
        shader as *mut crate::tr_local_h::shader_s,
        0 as libc::c_int,
    );
    crate::src::renderergl1::tr_surface::RB_AddQuadStamp(
        origin.as_mut_ptr(),
        vec1.as_mut_ptr(),
        vec2.as_mut_ptr(),
        sunColor.as_mut_ptr(),
    );
    crate::src::renderergl1::tr_shade::RB_EndSurface();
    // back to normal depth range
    crate::src::sdl::sdl_glimp::qglDepthRange.expect("non-null function pointer")(0.0f64, 1.0f64);
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
================
RB_StageIteratorSky

All of the visible sky triangles are in tess

Other things could be stuck in here, like birds in the sky, etc
================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_StageIteratorSky() {
    if (*crate::src::renderergl1::tr_init::r_fastsky).integer != 0 {
        return;
    }
    // go through all the polygons and project them onto
    // the sky box to see which blocks on each side need
    // to be drawn
    RB_ClipSkyPolygons(&mut crate::src::renderergl1::tr_shade::tess);
    // r_showsky will let all the sky blocks be drawn in
    // front of everything to allow developers to see how
    // much sky is getting sucked in
    if (*crate::src::renderergl1::tr_init::r_showsky).integer != 0 {
        crate::src::sdl::sdl_glimp::qglDepthRange.expect("non-null function pointer")(
            0.0f64, 0.0f64,
        );
    } else {
        crate::src::sdl::sdl_glimp::qglDepthRange.expect("non-null function pointer")(
            1.0f64, 1.0f64,
        );
    }
    // draw the outer skybox
    if !(*crate::src::renderergl1::tr_shade::tess.shader)
        .sky
        .outerbox[0 as libc::c_int as usize]
        .is_null()
        && (*crate::src::renderergl1::tr_shade::tess.shader)
            .sky
            .outerbox[0 as libc::c_int as usize]
            != crate::src::renderergl1::tr_main::tr.defaultImage
    {
        crate::src::sdl::sdl_glimp::qglColor3f.expect("non-null function pointer")(
            crate::src::renderergl1::tr_main::tr.identityLight,
            crate::src::renderergl1::tr_main::tr.identityLight,
            crate::src::renderergl1::tr_main::tr.identityLight,
        );
        crate::src::sdl::sdl_glimp::qglPushMatrix.expect("non-null function pointer")();
        crate::src::renderergl1::tr_backend::GL_State(0 as libc::c_int as libc::c_ulong);
        crate::src::renderergl1::tr_backend::GL_Cull(
            crate::tr_local_h::CT_FRONT_SIDED as libc::c_int,
        );
        crate::src::sdl::sdl_glimp::qglTranslatef.expect("non-null function pointer")(
            crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .origin[0 as libc::c_int as usize],
            crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .origin[1 as libc::c_int as usize],
            crate::src::renderergl1::tr_backend::backEnd
                .viewParms
                .or
                .origin[2 as libc::c_int as usize],
        );
        DrawSkyBox(crate::src::renderergl1::tr_shade::tess.shader);
        crate::src::sdl::sdl_glimp::qglPopMatrix.expect("non-null function pointer")();
    }
    // generate the vertexes for all the clouds, which will be drawn
    // by the generic shader routine
    R_BuildCloudData(&mut crate::src::renderergl1::tr_shade::tess);
    crate::src::renderergl1::tr_shade::RB_StageIteratorGeneric();
    // draw the inner skybox
    // back to normal depth range
    crate::src::sdl::sdl_glimp::qglDepthRange.expect("non-null function pointer")(0.0f64, 1.0f64);
    // note that sky was drawn so we will draw a sun later
    crate::src::renderergl1::tr_backend::backEnd.skyRenderedThisView =
        crate::src::qcommon::q_shared::qtrue;
}

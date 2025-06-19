use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0 as i32 as isize) * *v.offset(0 as i32 as isize)
                + *v.offset(1 as i32 as isize) * *v.offset(1 as i32 as isize)
                + *v.offset(2 as i32 as isize) * *v.offset(2 as i32 as isize))
                as f64,
        ) as crate::src::qcommon::q_shared::vec_t;
    }

    // __Q_SHARED_H
}

pub use crate::stdlib::__int64_t;
pub use crate::stdlib::int64_t;

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::Q_fabs;
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
pub use crate::src::renderergl1::tr_shade::q_shared_h::VectorLength;
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

pub use crate::qgl_h::ArrayElementproc;
pub use crate::qgl_h::Beginproc;
pub use crate::qgl_h::Color3fproc;
pub use crate::qgl_h::Color4ubvproc;
pub use crate::qgl_h::ColorPointerproc;
pub use crate::qgl_h::DepthRangeproc;
pub use crate::qgl_h::DisableClientStateproc;
pub use crate::qgl_h::Disableproc;
pub use crate::qgl_h::DrawElementsproc;
pub use crate::qgl_h::EnableClientStateproc;
pub use crate::qgl_h::Enableproc;
pub use crate::qgl_h::Endproc;
pub use crate::qgl_h::PolygonModeproc;
pub use crate::qgl_h::PolygonOffsetproc;
pub use crate::qgl_h::TexCoord2fvproc;
pub use crate::qgl_h::TexCoordPointerproc;
pub use crate::qgl_h::Vertex3fvproc;
pub use crate::qgl_h::VertexPointerproc;
pub use crate::src::renderergl1::tr_backend::backEnd;
pub use crate::src::renderergl1::tr_backend::GL_Bind;
pub use crate::src::renderergl1::tr_backend::GL_Cull;
pub use crate::src::renderergl1::tr_backend::GL_SelectTexture;
pub use crate::src::renderergl1::tr_backend::GL_State;
pub use crate::src::renderergl1::tr_backend::GL_TexEnv;
pub use crate::src::renderergl1::tr_init::glState;
pub use crate::src::renderergl1::tr_init::r_debugSort;
pub use crate::src::renderergl1::tr_init::r_dlightBacks;
pub use crate::src::renderergl1::tr_init::r_greyscale;
pub use crate::src::renderergl1::tr_init::r_lightmap;
pub use crate::src::renderergl1::tr_init::r_logFile;
pub use crate::src::renderergl1::tr_init::r_offsetFactor;
pub use crate::src::renderergl1::tr_init::r_offsetUnits;
pub use crate::src::renderergl1::tr_init::r_primitives;
pub use crate::src::renderergl1::tr_init::r_shownormals;
pub use crate::src::renderergl1::tr_init::r_showtris;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcAlphaFromEntity;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcAlphaFromOneMinusEntity;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcColorFromEntity;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcColorFromOneMinusEntity;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcDiffuseColor;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcEnvironmentTexCoords;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcFogTexCoords;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcModulateAlphasByFog;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcModulateColorsByFog;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcModulateRGBAsByFog;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcRotateTexCoords;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcScaleTexCoords;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcScrollTexCoords;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcSpecularAlpha;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcStretchTexCoords;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcTransformTexCoords;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcTurbulentTexCoords;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcWaveAlpha;
pub use crate::src::renderergl1::tr_shade_calc::RB_CalcWaveColor;
pub use crate::src::renderergl1::tr_shade_calc::RB_DeformTessGeometry;
pub use crate::src::renderergl1::tr_shadows::RB_ShadowTessEnd;
pub use crate::src::sdl::sdl_glimp::qglArrayElement;
pub use crate::src::sdl::sdl_glimp::qglBegin;
pub use crate::src::sdl::sdl_glimp::qglColor3f;
pub use crate::src::sdl::sdl_glimp::qglColor4ubv;
pub use crate::src::sdl::sdl_glimp::qglColorPointer;
pub use crate::src::sdl::sdl_glimp::qglDepthRange;
pub use crate::src::sdl::sdl_glimp::qglDisable;
pub use crate::src::sdl::sdl_glimp::qglDisableClientState;
pub use crate::src::sdl::sdl_glimp::qglDrawElements;
pub use crate::src::sdl::sdl_glimp::qglEnable;
pub use crate::src::sdl::sdl_glimp::qglEnableClientState;
pub use crate::src::sdl::sdl_glimp::qglEnd;
pub use crate::src::sdl::sdl_glimp::qglLockArraysEXT;
pub use crate::src::sdl::sdl_glimp::qglMultiTexCoord2fARB;
pub use crate::src::sdl::sdl_glimp::qglPolygonMode;
pub use crate::src::sdl::sdl_glimp::qglPolygonOffset;
pub use crate::src::sdl::sdl_glimp::qglTexCoord2fv;
pub use crate::src::sdl::sdl_glimp::qglTexCoordPointer;
pub use crate::src::sdl::sdl_glimp::qglUnlockArraysEXT;
pub use crate::src::sdl::sdl_glimp::qglVertex3fv;
pub use crate::src::sdl::sdl_glimp::qglVertexPointer;
pub use crate::src::sdl::sdl_glimp::GLimp_LogComment;

pub use crate::stdlib::GLclampd;
pub use crate::stdlib::GLenum;
pub use crate::stdlib::GLfloat;
pub use crate::stdlib::GLint;
pub use crate::stdlib::GLsizei;
pub use crate::stdlib::GLubyte;
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
// tr_shade.c
/*

  THIS ENTIRE FILE IS BACK END

  This file deals with applying shaders to surface data in the tess struct.
*/
/*
================
R_ArrayElementDiscrete

This is just for OpenGL conformance testing, it should never be the fastest
================
*/

unsafe extern "C" fn R_ArrayElementDiscrete(mut index: crate::stdlib::GLint) {
    crate::src::sdl::sdl_glimp::qglColor4ubv.expect("non-null function pointer")(
        tess.svars.colors[index as usize].as_mut_ptr(),
    );
    if crate::src::renderergl1::tr_init::glState.currenttmu != 0 {
        crate::src::sdl::sdl_glimp::qglMultiTexCoord2fARB.expect("non-null function pointer")(
            0 as i32 as crate::stdlib::GLenum,
            tess.svars.texcoords[0 as i32 as usize][index as usize]
                [0 as i32 as usize],
            tess.svars.texcoords[0 as i32 as usize][index as usize]
                [1 as i32 as usize],
        );
        crate::src::sdl::sdl_glimp::qglMultiTexCoord2fARB.expect("non-null function pointer")(
            1 as i32 as crate::stdlib::GLenum,
            tess.svars.texcoords[1 as i32 as usize][index as usize]
                [0 as i32 as usize],
            tess.svars.texcoords[1 as i32 as usize][index as usize]
                [1 as i32 as usize],
        );
    } else {
        crate::src::sdl::sdl_glimp::qglTexCoord2fv.expect("non-null function pointer")(
            tess.svars.texcoords[0 as i32 as usize][index as usize].as_mut_ptr(),
        );
    }
    crate::src::sdl::sdl_glimp::qglVertex3fv.expect("non-null function pointer")(
        tess.xyz[index as usize].as_mut_ptr(),
    );
}
/*
===================
R_DrawStripElements

===================
*/

static mut c_vertexes: i32 = 0;
// for seeing how long our average strips are

static mut c_begins: i32 = 0;

unsafe extern "C" fn R_DrawStripElements(
    mut numIndexes: i32,
    mut indexes: *const crate::tr_local_h::glIndex_t,
    mut element: Option<unsafe extern "C" fn(_: crate::stdlib::GLint) -> ()>,
) {
    let mut i: i32 = 0;
    let mut last: [i32; 3] = [
        -(1 as i32),
        -(1 as i32),
        -(1 as i32),
    ];
    let mut even: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    c_begins += 1;
    if numIndexes <= 0 as i32 {
        return;
    }
    crate::src::sdl::sdl_glimp::qglBegin.expect("non-null function pointer")(
        0x5 as i32 as crate::stdlib::GLenum,
    );
    // prime the strip
    element.expect("non-null function pointer")(
        *indexes.offset(0 as i32 as isize) as crate::stdlib::GLint
    );
    element.expect("non-null function pointer")(
        *indexes.offset(1 as i32 as isize) as crate::stdlib::GLint
    );
    element.expect("non-null function pointer")(
        *indexes.offset(2 as i32 as isize) as crate::stdlib::GLint
    );
    c_vertexes += 3 as i32;
    last[0 as i32 as usize] = *indexes.offset(0 as i32 as isize) as i32;
    last[1 as i32 as usize] = *indexes.offset(1 as i32 as isize) as i32;
    last[2 as i32 as usize] = *indexes.offset(2 as i32 as isize) as i32;
    even = crate::src::qcommon::q_shared::qfalse;
    i = 3 as i32;
    while i < numIndexes {
        // odd numbered triangle in potential strip
        if even as u64 == 0 {
            // check previous triangle to see if we're continuing a strip
            if *indexes.offset((i + 0 as i32) as isize)
                == last[2 as i32 as usize] as u32
                && *indexes.offset((i + 1 as i32) as isize)
                    == last[1 as i32 as usize] as u32
            {
                element.expect("non-null function pointer")(
                    *indexes.offset((i + 2 as i32) as isize) as crate::stdlib::GLint,
                );
                c_vertexes += 1;
                even = crate::src::qcommon::q_shared::qtrue
            } else {
                // otherwise we're done with this strip so finish it and start
                // a new one
                crate::src::sdl::sdl_glimp::qglEnd.expect("non-null function pointer")();
                crate::src::sdl::sdl_glimp::qglBegin.expect("non-null function pointer")(
                    0x5 as i32 as crate::stdlib::GLenum,
                );
                c_begins += 1;
                element.expect("non-null function pointer")(
                    *indexes.offset((i + 0 as i32) as isize) as crate::stdlib::GLint,
                );
                element.expect("non-null function pointer")(
                    *indexes.offset((i + 1 as i32) as isize) as crate::stdlib::GLint,
                );
                element.expect("non-null function pointer")(
                    *indexes.offset((i + 2 as i32) as isize) as crate::stdlib::GLint,
                );
                c_vertexes += 3 as i32;
                even = crate::src::qcommon::q_shared::qfalse
            }
        } else if last[2 as i32 as usize] as u32
            == *indexes.offset((i + 1 as i32) as isize)
            && last[0 as i32 as usize] as u32
                == *indexes.offset((i + 0 as i32) as isize)
        {
            element.expect("non-null function pointer")(
                *indexes.offset((i + 2 as i32) as isize) as crate::stdlib::GLint,
            );
            c_vertexes += 1;
            even = crate::src::qcommon::q_shared::qfalse
        } else {
            // check previous triangle to see if we're continuing a strip
            // otherwise we're done with this strip so finish it and start
            // a new one
            crate::src::sdl::sdl_glimp::qglEnd.expect("non-null function pointer")();
            crate::src::sdl::sdl_glimp::qglBegin.expect("non-null function pointer")(
                0x5 as i32 as crate::stdlib::GLenum,
            );
            c_begins += 1;
            element.expect("non-null function pointer")(
                *indexes.offset((i + 0 as i32) as isize) as crate::stdlib::GLint,
            );
            element.expect("non-null function pointer")(
                *indexes.offset((i + 1 as i32) as isize) as crate::stdlib::GLint,
            );
            element.expect("non-null function pointer")(
                *indexes.offset((i + 2 as i32) as isize) as crate::stdlib::GLint,
            );
            c_vertexes += 3 as i32;
            even = crate::src::qcommon::q_shared::qfalse
        }
        // cache the last three vertices
        last[0 as i32 as usize] =
            *indexes.offset((i + 0 as i32) as isize) as i32;
        last[1 as i32 as usize] =
            *indexes.offset((i + 1 as i32) as isize) as i32;
        last[2 as i32 as usize] =
            *indexes.offset((i + 2 as i32) as isize) as i32;
        i += 3 as i32
    }
    crate::src::sdl::sdl_glimp::qglEnd.expect("non-null function pointer")();
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
/*
=============================================================

RENDERER BACK END FUNCTIONS

=============================================================
*/
/*
=============================================================

RENDERER BACK END COMMAND QUEUE

=============================================================
*/
// these are sort of arbitrary limits.
// the limits apply to the sum of all scenes in a frame --
// the main view, all the 3D icons, etc
// all of the information needed by the back end must be
// contained in a backEndData_t
//[MAX_POLYS];
//[MAX_POLYVERTS];
// the second one may not be allocated
/*
==================
R_DrawElements

Optionally performs our own glDrawElements that looks for strip conditions
instead of using the single glDrawElements call that may be inefficient
without compiled vertex arrays.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn R_DrawElements(
    mut numIndexes: i32,
    mut indexes: *const crate::tr_local_h::glIndex_t,
) {
    let mut primitives: i32 = 0;
    primitives = (*crate::src::renderergl1::tr_init::r_primitives).integer;
    // default is to use triangles if compiled vertex arrays are present
    if primitives == 0 as i32 {
        if crate::src::sdl::sdl_glimp::qglLockArraysEXT.is_some() {
            primitives = 2 as i32
        } else {
            primitives = 1 as i32
        }
    }
    if primitives == 2 as i32 {
        crate::src::sdl::sdl_glimp::qglDrawElements.expect("non-null function pointer")(
            0x4 as i32 as crate::stdlib::GLenum,
            numIndexes,
            0x1405 as i32 as crate::stdlib::GLenum,
            indexes as *const libc::c_void,
        );
        return;
    }
    if primitives == 1 as i32 {
        R_DrawStripElements(
            numIndexes,
            indexes,
            crate::src::sdl::sdl_glimp::qglArrayElement,
        );
        return;
    }
    if primitives == 3 as i32 {
        R_DrawStripElements(
            numIndexes,
            indexes,
            Some(R_ArrayElementDiscrete as unsafe extern "C" fn(_: crate::stdlib::GLint) -> ()),
        );
        return;
    };
    // anything else will cause no drawing
}
/*
=============================================================

SURFACE SHADERS

=============================================================
*/
#[no_mangle]

pub static mut tess: crate::tr_local_h::shaderCommands_t = crate::tr_local_h::shaderCommands_t {
    indexes: [0; 6000],
    xyz: [[0.; 4]; 1000],
    normal: [[0.; 4]; 1000],
    texCoords: [[[0.; 2]; 2]; 1000],
    vertexColors: [[0; 4]; 1000],
    vertexDlightBits: [0; 1000],
    svars: crate::tr_local_h::stageVars_t {
        colors: [[0; 4]; 1000],
        texcoords: [[[0.; 2]; 1000]; 2],
    },
    constantColor255: [[0; 4]; 1000],
    shader: 0 as *const crate::tr_local_h::shader_t as *mut crate::tr_local_h::shader_t,
    shaderTime: 0.,
    fogNum: 0,
    dlightBits: 0,
    numIndexes: 0,
    numVertexes: 0,
    numPasses: 0,
    currentStageIteratorFunc: None,
    xstages: 0 as *const *mut crate::tr_local_h::shaderStage_t
        as *mut *mut crate::tr_local_h::shaderStage_t,
};

static mut setArraysOnce: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
/*
=================
R_BindAnimatedImage

=================
*/

unsafe extern "C" fn R_BindAnimatedImage(mut bundle: *mut crate::tr_local_h::textureBundle_t) {
    let mut index: crate::stdlib::int64_t = 0;
    if (*bundle).isVideoMap as u64 != 0 {
        crate::src::renderergl1::tr_main::ri
            .CIN_RunCinematic
            .expect("non-null function pointer")((*bundle).videoMapHandle);
        crate::src::renderergl1::tr_main::ri
            .CIN_UploadCinematic
            .expect("non-null function pointer")((*bundle).videoMapHandle);
        return;
    }
    if (*bundle).numImageAnimations <= 1 as i32 {
        crate::src::renderergl1::tr_backend::GL_Bind(
            (*bundle).image[0 as i32 as usize] as *mut crate::tr_common_h::image_s,
        );
        return;
    }
    // it is necessary to do this messy calc to make sure animations line up
    // exactly with waveforms of the same frequency
    index = (tess.shaderTime
        * (*bundle).imageAnimationSpeed as f64
        * 1024 as i32 as f64) as crate::stdlib::int64_t;
    index >>= 10 as i32;
    if index < 0 as i32 as libc::c_long {
        index = 0 as i32 as crate::stdlib::int64_t
        // may happen with shader time offsets
    }
    // Windows x86 doesn't load renderer DLL with 64 bit modulus
    //index %= bundle->numImageAnimations;
    while index >= (*bundle).numImageAnimations as libc::c_long {
        index -= (*bundle).numImageAnimations as libc::c_long
    }
    crate::src::renderergl1::tr_backend::GL_Bind(
        (*bundle).image[index as usize] as *mut crate::tr_common_h::image_s,
    );
}
/*
================
DrawTris

Draws triangle outlines for debugging
================
*/

unsafe extern "C" fn DrawTris(mut input: *mut crate::tr_local_h::shaderCommands_t) {
    crate::src::renderergl1::tr_backend::GL_Bind(
        crate::src::renderergl1::tr_main::tr.whiteImage as *mut crate::tr_common_h::image_s,
    ); // padded for SIMD
    crate::src::sdl::sdl_glimp::qglColor3f.expect("non-null function pointer")(
        1 as i32 as crate::stdlib::GLfloat,
        1 as i32 as crate::stdlib::GLfloat,
        1 as i32 as crate::stdlib::GLfloat,
    );
    crate::src::renderergl1::tr_backend::GL_State(
        (0x1000 as i32 | 0x100 as i32) as libc::c_ulong,
    );
    crate::src::sdl::sdl_glimp::qglDepthRange.expect("non-null function pointer")(
        0 as i32 as crate::stdlib::GLclampd,
        0 as i32 as crate::stdlib::GLclampd,
    );
    crate::src::sdl::sdl_glimp::qglDisableClientState.expect("non-null function pointer")(
        0x8076 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglDisableClientState.expect("non-null function pointer")(
        0x8078 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglVertexPointer.expect("non-null function pointer")(
        3 as i32,
        0x1406 as i32 as crate::stdlib::GLenum,
        16 as i32,
        (*input).xyz.as_mut_ptr() as *const libc::c_void,
    );
    if crate::src::sdl::sdl_glimp::qglLockArraysEXT.is_some() {
        crate::src::sdl::sdl_glimp::qglLockArraysEXT.expect("non-null function pointer")(
            0 as i32,
            (*input).numVertexes,
        );
        crate::src::sdl::sdl_glimp::GLimp_LogComment(
            b"glLockArraysEXT\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    R_DrawElements((*input).numIndexes, (*input).indexes.as_mut_ptr());
    if crate::src::sdl::sdl_glimp::qglUnlockArraysEXT.is_some() {
        crate::src::sdl::sdl_glimp::qglUnlockArraysEXT.expect("non-null function pointer")();
        crate::src::sdl::sdl_glimp::GLimp_LogComment(
            b"glUnlockArraysEXT\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    crate::src::sdl::sdl_glimp::qglDepthRange.expect("non-null function pointer")(
        0 as i32 as crate::stdlib::GLclampd,
        1 as i32 as crate::stdlib::GLclampd,
    );
}
/*
================
DrawNormals

Draws vertex normals for debugging
================
*/

unsafe extern "C" fn DrawNormals(mut input: *mut crate::tr_local_h::shaderCommands_t) {
    let mut i: i32 = 0; // never occluded
    let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    crate::src::renderergl1::tr_backend::GL_Bind(
        crate::src::renderergl1::tr_main::tr.whiteImage as *mut crate::tr_common_h::image_s,
    );
    crate::src::sdl::sdl_glimp::qglColor3f.expect("non-null function pointer")(
        1 as i32 as crate::stdlib::GLfloat,
        1 as i32 as crate::stdlib::GLfloat,
        1 as i32 as crate::stdlib::GLfloat,
    );
    crate::src::sdl::sdl_glimp::qglDepthRange.expect("non-null function pointer")(
        0 as i32 as crate::stdlib::GLclampd,
        0 as i32 as crate::stdlib::GLclampd,
    );
    crate::src::renderergl1::tr_backend::GL_State(
        (0x1000 as i32 | 0x100 as i32) as libc::c_ulong,
    );
    crate::src::sdl::sdl_glimp::qglBegin.expect("non-null function pointer")(
        0x1 as i32 as crate::stdlib::GLenum,
    );
    i = 0 as i32;
    while i < (*input).numVertexes {
        crate::src::sdl::sdl_glimp::qglVertex3fv.expect("non-null function pointer")(
            (*input).xyz[i as usize].as_mut_ptr(),
        );
        temp[0 as i32 as usize] = (*input).xyz[i as usize][0 as i32 as usize]
            + (*input).normal[i as usize][0 as i32 as usize]
                * 2 as i32 as f32;
        temp[1 as i32 as usize] = (*input).xyz[i as usize][1 as i32 as usize]
            + (*input).normal[i as usize][1 as i32 as usize]
                * 2 as i32 as f32;
        temp[2 as i32 as usize] = (*input).xyz[i as usize][2 as i32 as usize]
            + (*input).normal[i as usize][2 as i32 as usize]
                * 2 as i32 as f32;
        crate::src::sdl::sdl_glimp::qglVertex3fv.expect("non-null function pointer")(
            temp.as_mut_ptr(),
        );
        i += 1
    }
    crate::src::sdl::sdl_glimp::qglEnd.expect("non-null function pointer")();
    crate::src::sdl::sdl_glimp::qglDepthRange.expect("non-null function pointer")(
        0 as i32 as crate::stdlib::GLclampd,
        1 as i32 as crate::stdlib::GLclampd,
    );
}
/*
==============
RB_BeginSurface

We must set some things up before beginning any tesselation,
because a surface may be forced to perform a RB_End due
to overflow.
==============
*/
#[no_mangle]

pub unsafe extern "C" fn RB_BeginSurface(
    mut shader: *mut crate::tr_local_h::shader_t,
    mut fogNum: i32,
) {
    let mut state: *mut crate::tr_local_h::shader_t = if !(*shader).remappedShader.is_null() {
        (*shader).remappedShader
    } else {
        shader
    }; // will be OR'd in by surface functions
    tess.numIndexes = 0 as i32;
    tess.numVertexes = 0 as i32;
    tess.shader = state;
    tess.fogNum = fogNum;
    tess.dlightBits = 0 as i32;
    tess.xstages = (*state).stages.as_mut_ptr();
    tess.numPasses = (*state).numUnfoggedPasses;
    tess.currentStageIteratorFunc = (*state).optimalStageIteratorFunc;
    tess.shaderTime = crate::src::renderergl1::tr_backend::backEnd
        .refdef
        .floatTime
        - (*tess.shader).timeOffset;
    if (*tess.shader).clampTime != 0. && tess.shaderTime >= (*tess.shader).clampTime {
        tess.shaderTime = (*tess.shader).clampTime
    };
}
/*
===================
DrawMultitextured

output = t0 * t1 or t0 + t1

t0 = most upstream according to spec
t1 = most downstream according to spec
===================
*/

unsafe extern "C" fn DrawMultitextured(
    mut input: *mut crate::tr_local_h::shaderCommands_t,
    mut stage: i32,
) {
    let mut pStage: *mut crate::tr_local_h::shaderStage_t =
        0 as *mut crate::tr_local_h::shaderStage_t;
    pStage = *tess.xstages.offset(stage as isize);
    crate::src::renderergl1::tr_backend::GL_State((*pStage).stateBits as libc::c_ulong);
    // this is an ugly hack to work around a GeForce driver
    // bug with multitexture and clip planes
    if crate::src::renderergl1::tr_backend::backEnd
        .viewParms
        .isPortal as u64
        != 0
    {
        crate::src::sdl::sdl_glimp::qglPolygonMode.expect("non-null function pointer")(
            0x408 as i32 as crate::stdlib::GLenum,
            0x1b02 as i32 as crate::stdlib::GLenum,
        );
    }
    //
    // base
    //
    crate::src::renderergl1::tr_backend::GL_SelectTexture(0 as i32);
    crate::src::sdl::sdl_glimp::qglTexCoordPointer.expect("non-null function pointer")(
        2 as i32,
        0x1406 as i32 as crate::stdlib::GLenum,
        0 as i32,
        (*input).svars.texcoords[0 as i32 as usize].as_mut_ptr() as *const libc::c_void,
    );
    R_BindAnimatedImage(
        &mut *(*pStage)
            .bundle
            .as_mut_ptr()
            .offset(0 as i32 as isize),
    );
    //
    // lightmap/secondary pass
    //
    crate::src::renderergl1::tr_backend::GL_SelectTexture(1 as i32);
    crate::src::sdl::sdl_glimp::qglEnable.expect("non-null function pointer")(
        0xde1 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
        0x8078 as i32 as crate::stdlib::GLenum,
    );
    if (*crate::src::renderergl1::tr_init::r_lightmap).integer != 0 {
        crate::src::renderergl1::tr_backend::GL_TexEnv(0x1e01 as i32);
    } else {
        crate::src::renderergl1::tr_backend::GL_TexEnv((*tess.shader).multitextureEnv);
    }
    crate::src::sdl::sdl_glimp::qglTexCoordPointer.expect("non-null function pointer")(
        2 as i32,
        0x1406 as i32 as crate::stdlib::GLenum,
        0 as i32,
        (*input).svars.texcoords[1 as i32 as usize].as_mut_ptr() as *const libc::c_void,
    );
    R_BindAnimatedImage(
        &mut *(*pStage)
            .bundle
            .as_mut_ptr()
            .offset(1 as i32 as isize),
    );
    R_DrawElements((*input).numIndexes, (*input).indexes.as_mut_ptr());
    //
    // disable texturing on TEXTURE1, then select TEXTURE0
    //
    //qglDisableClientState( GL_TEXTURE_COORD_ARRAY );
    crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
        0xde1 as i32 as crate::stdlib::GLenum,
    );
    crate::src::renderergl1::tr_backend::GL_SelectTexture(0 as i32);
}
/*
===================
ProjectDlightTexture

Perform dynamic lighting with another rendering pass
===================
*/

unsafe extern "C" fn ProjectDlightTexture_scalar() {
    let mut i: i32 = 0;
    let mut l: i32 = 0;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut texCoords: *mut f32 = 0 as *mut f32;
    let mut colors: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut clipBits: [crate::src::qcommon::q_shared::byte; 1000] = [0; 1000];
    let mut texCoordsArray: [[f32; 2]; 1000] = [[0.; 2]; 1000];
    let mut colorArray: [[crate::src::qcommon::q_shared::byte; 4]; 1000] = [[0; 4]; 1000];
    let mut hitIndexes: [crate::tr_local_h::glIndex_t; 6000] = [0; 6000];
    let mut numIndexes: i32 = 0;
    let mut scale: f32 = 0.;
    let mut radius: f32 = 0.;
    let mut floatColor: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut modulate: f32 = 0.0f32;
    if crate::src::renderergl1::tr_backend::backEnd
        .refdef
        .num_dlights
        == 0
    {
        return;
    }
    l = 0 as i32;
    while l < crate::src::renderergl1::tr_backend::backEnd
        .refdef
        .num_dlights
    {
        let mut dl: *mut crate::tr_local_h::dlight_t = 0 as *mut crate::tr_local_h::dlight_t;
        if !(tess.dlightBits & (1 as i32) << l == 0) {
            texCoords = texCoordsArray[0 as i32 as usize].as_mut_ptr();
            colors = colorArray[0 as i32 as usize].as_mut_ptr();
            dl = &mut *crate::src::renderergl1::tr_backend::backEnd
                .refdef
                .dlights
                .offset(l as isize) as *mut crate::tr_local_h::dlight_s;
            origin[0 as i32 as usize] = (*dl).transformed[0 as i32 as usize];
            origin[1 as i32 as usize] = (*dl).transformed[1 as i32 as usize];
            origin[2 as i32 as usize] = (*dl).transformed[2 as i32 as usize];
            radius = (*dl).radius;
            scale = 1.0f32 / radius;
            if (*crate::src::renderergl1::tr_init::r_greyscale).integer != 0 {
                let mut luminance: f32 = 0.;
                luminance = (0.2126f32 * (*dl).color[0 as i32 as usize]
                    + 0.7152f32 * (*dl).color[1 as i32 as usize]
                    + 0.0722f32 * (*dl).color[2 as i32 as usize])
                    * 255.0f32;
                floatColor[2 as i32 as usize] = luminance;
                floatColor[1 as i32 as usize] = floatColor[2 as i32 as usize];
                floatColor[0 as i32 as usize] = floatColor[1 as i32 as usize]
            } else if (*crate::src::renderergl1::tr_init::r_greyscale).value != 0. {
                let mut luminance_0: f32 = 0.;
                luminance_0 = (0.2126f32 * (*dl).color[0 as i32 as usize]
                    + 0.7152f32 * (*dl).color[1 as i32 as usize]
                    + 0.0722f32 * (*dl).color[2 as i32 as usize])
                    * 255.0f32;
                floatColor[0 as i32 as usize] = (*dl).color[0 as i32 as usize]
                    * 255.0f32
                    * (1.0f32 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    + luminance_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value;
                floatColor[1 as i32 as usize] = (*dl).color[1 as i32 as usize]
                    * 255.0f32
                    * (1.0f32 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    + luminance_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value;
                floatColor[2 as i32 as usize] = (*dl).color[2 as i32 as usize]
                    * 255.0f32
                    * (1.0f32 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    + luminance_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value
            } else {
                floatColor[0 as i32 as usize] =
                    (*dl).color[0 as i32 as usize] * 255.0f32;
                floatColor[1 as i32 as usize] =
                    (*dl).color[1 as i32 as usize] * 255.0f32;
                floatColor[2 as i32 as usize] =
                    (*dl).color[2 as i32 as usize] * 255.0f32
            }
            i = 0 as i32;
            while i < tess.numVertexes {
                let mut clip: i32 = 0 as i32;
                let mut dist: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                dist[0 as i32 as usize] = origin[0 as i32 as usize]
                    - tess.xyz[i as usize][0 as i32 as usize];
                dist[1 as i32 as usize] = origin[1 as i32 as usize]
                    - tess.xyz[i as usize][1 as i32 as usize];
                dist[2 as i32 as usize] = origin[2 as i32 as usize]
                    - tess.xyz[i as usize][2 as i32 as usize];
                crate::src::renderergl1::tr_backend::backEnd
                    .pc
                    .c_dlightVertexes += 1;
                *texCoords.offset(0 as i32 as isize) =
                    0.5f32 + dist[0 as i32 as usize] * scale;
                *texCoords.offset(1 as i32 as isize) =
                    0.5f32 + dist[1 as i32 as usize] * scale;
                if (*crate::src::renderergl1::tr_init::r_dlightBacks).integer == 0
                    && dist[0 as i32 as usize]
                        * tess.normal[i as usize][0 as i32 as usize]
                        + dist[1 as i32 as usize]
                            * tess.normal[i as usize][1 as i32 as usize]
                        + dist[2 as i32 as usize]
                            * tess.normal[i as usize][2 as i32 as usize]
                        < 0.0f32
                {
                    clip = 63 as i32
                } else {
                    if *texCoords.offset(0 as i32 as isize) < 0.0f32 {
                        clip |= 1 as i32
                    } else if *texCoords.offset(0 as i32 as isize) > 1.0f32 {
                        clip |= 2 as i32
                    }
                    if *texCoords.offset(1 as i32 as isize) < 0.0f32 {
                        clip |= 4 as i32
                    } else if *texCoords.offset(1 as i32 as isize) > 1.0f32 {
                        clip |= 8 as i32
                    }
                    *texCoords.offset(0 as i32 as isize) =
                        *texCoords.offset(0 as i32 as isize);
                    *texCoords.offset(1 as i32 as isize) =
                        *texCoords.offset(1 as i32 as isize);
                    // modulate the strength based on the height and color
                    if dist[2 as i32 as usize] > radius {
                        clip |= 16 as i32;
                        modulate = 0.0f32
                    } else if dist[2 as i32 as usize] < -radius {
                        clip |= 32 as i32;
                        modulate = 0.0f32
                    } else {
                        dist[2 as i32 as usize] =
                            crate::src::qcommon::q_math::Q_fabs(dist[2 as i32 as usize]);
                        if dist[2 as i32 as usize] < radius * 0.5f32 {
                            modulate = 1.0f32
                        } else {
                            modulate = 2.0f32 * (radius - dist[2 as i32 as usize]) * scale
                        }
                    }
                }
                clipBits[i as usize] = clip as crate::src::qcommon::q_shared::byte;
                *colors.offset(0 as i32 as isize) = crate::src::renderergl1::tr_main::ri
                    .ftol
                    .expect("non-null function pointer")(
                    floatColor[0 as i32 as usize] * modulate,
                )
                    as crate::src::qcommon::q_shared::byte;
                *colors.offset(1 as i32 as isize) = crate::src::renderergl1::tr_main::ri
                    .ftol
                    .expect("non-null function pointer")(
                    floatColor[1 as i32 as usize] * modulate,
                )
                    as crate::src::qcommon::q_shared::byte;
                *colors.offset(2 as i32 as isize) = crate::src::renderergl1::tr_main::ri
                    .ftol
                    .expect("non-null function pointer")(
                    floatColor[2 as i32 as usize] * modulate,
                )
                    as crate::src::qcommon::q_shared::byte;
                *colors.offset(3 as i32 as isize) =
                    255 as i32 as crate::src::qcommon::q_shared::byte;
                i += 1;
                texCoords = texCoords.offset(2 as i32 as isize);
                colors = colors.offset(4 as i32 as isize)
            }
            // build a list of triangles that need light
            numIndexes = 0 as i32;
            i = 0 as i32;
            while i < tess.numIndexes {
                let mut a: i32 = 0;
                let mut b: i32 = 0;
                let mut c: i32 = 0;
                a = tess.indexes[i as usize] as i32;
                b = tess.indexes[(i + 1 as i32) as usize] as i32;
                c = tess.indexes[(i + 2 as i32) as usize] as i32;
                if !(clipBits[a as usize] as i32
                    & clipBits[b as usize] as i32
                    & clipBits[c as usize] as i32
                    != 0)
                {
                    hitIndexes[numIndexes as usize] = a as crate::tr_local_h::glIndex_t;
                    hitIndexes[(numIndexes + 1 as i32) as usize] =
                        b as crate::tr_local_h::glIndex_t;
                    hitIndexes[(numIndexes + 2 as i32) as usize] =
                        c as crate::tr_local_h::glIndex_t;
                    numIndexes += 3 as i32
                }
                i += 3 as i32
                // not lighted
            }
            if !(numIndexes == 0) {
                crate::src::sdl::sdl_glimp::qglEnableClientState
                    .expect("non-null function pointer")(
                    0x8078 as i32 as crate::stdlib::GLenum,
                );
                crate::src::sdl::sdl_glimp::qglTexCoordPointer.expect("non-null function pointer")(
                    2 as i32,
                    0x1406 as i32 as crate::stdlib::GLenum,
                    0 as i32,
                    texCoordsArray[0 as i32 as usize].as_mut_ptr() as *const libc::c_void,
                );
                crate::src::sdl::sdl_glimp::qglEnableClientState
                    .expect("non-null function pointer")(
                    0x8076 as i32 as crate::stdlib::GLenum,
                );
                crate::src::sdl::sdl_glimp::qglColorPointer.expect("non-null function pointer")(
                    4 as i32,
                    0x1401 as i32 as crate::stdlib::GLenum,
                    0 as i32,
                    colorArray.as_mut_ptr() as *const libc::c_void,
                );
                crate::src::renderergl1::tr_backend::GL_Bind(
                    crate::src::renderergl1::tr_main::tr.dlightImage
                        as *mut crate::tr_common_h::image_s,
                );
                // include GLS_DEPTHFUNC_EQUAL so alpha tested surfaces don't add light
                // where they aren't rendered
                if (*dl).additive != 0 {
                    crate::src::renderergl1::tr_backend::GL_State(
                        (0x2 as i32 | 0x20 as i32 | 0x20000 as i32)
                            as libc::c_ulong,
                    );
                } else {
                    crate::src::renderergl1::tr_backend::GL_State(
                        (0x3 as i32 | 0x20 as i32 | 0x20000 as i32)
                            as libc::c_ulong,
                    );
                }
                R_DrawElements(numIndexes, hitIndexes.as_mut_ptr());
                crate::src::renderergl1::tr_backend::backEnd
                    .pc
                    .c_totalIndexes += numIndexes;
                crate::src::renderergl1::tr_backend::backEnd
                    .pc
                    .c_dlightIndexes += numIndexes
            }
        }
        l += 1
        // this surface definitely doesn't have any of this light
    }
}

unsafe extern "C" fn ProjectDlightTexture() {
    ProjectDlightTexture_scalar();
}
/*
===================
RB_FogPass

Blends a fog texture on top of everything else
===================
*/

unsafe extern "C" fn RB_FogPass() {
    let mut fog: *mut crate::tr_local_h::fog_t = 0 as *mut crate::tr_local_h::fog_t;
    let mut i: i32 = 0;
    crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
        0x8076 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglColorPointer.expect("non-null function pointer")(
        4 as i32,
        0x1401 as i32 as crate::stdlib::GLenum,
        0 as i32,
        tess.svars.colors.as_mut_ptr() as *const libc::c_void,
    );
    crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
        0x8078 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglTexCoordPointer.expect("non-null function pointer")(
        2 as i32,
        0x1406 as i32 as crate::stdlib::GLenum,
        0 as i32,
        tess.svars.texcoords[0 as i32 as usize].as_mut_ptr() as *const libc::c_void,
    );
    fog = (*crate::src::renderergl1::tr_main::tr.world)
        .fogs
        .offset(tess.fogNum as isize);
    i = 0 as i32;
    while i < tess.numVertexes {
        *(&mut *tess.svars.colors.as_mut_ptr().offset(i as isize)
            as *mut crate::tr_local_h::color4ub_t as *mut i32) =
            (*fog).colorInt as i32;
        i += 1
    }
    crate::src::renderergl1::tr_shade_calc::RB_CalcFogTexCoords(
        tess.svars.texcoords[0 as i32 as usize].as_mut_ptr() as *mut f32,
    );
    crate::src::renderergl1::tr_backend::GL_Bind(
        crate::src::renderergl1::tr_main::tr.fogImage as *mut crate::tr_common_h::image_s,
    );
    if (*tess.shader).fogPass as u32
        == crate::tr_local_h::FP_EQUAL as i32 as u32
    {
        crate::src::renderergl1::tr_backend::GL_State(
            (0x5 as i32 | 0x60 as i32 | 0x20000 as i32) as libc::c_ulong,
        );
    } else {
        crate::src::renderergl1::tr_backend::GL_State(
            (0x5 as i32 | 0x60 as i32) as libc::c_ulong,
        );
    }
    R_DrawElements(tess.numIndexes, tess.indexes.as_mut_ptr());
}
/*
===============
ComputeColors
===============
*/

unsafe extern "C" fn ComputeColors(mut pStage: *mut crate::tr_local_h::shaderStage_t) {
    let mut i: i32 = 0;
    //
    // rgbGen
    //
    match (*pStage).rgbGen as u32 {
        2 => {
            crate::stdlib::memset(
                tess.svars.colors.as_mut_ptr() as *mut libc::c_void,
                0xff as i32,
                (tess.numVertexes * 4 as i32) as libc::c_ulong,
            );
        }
        9 => {
            crate::src::renderergl1::tr_shade_calc::RB_CalcDiffuseColor(
                tess.svars.colors.as_mut_ptr() as *mut u8,
            );
        }
        5 => {
            crate::stdlib::memcpy(
                tess.svars.colors.as_mut_ptr() as *mut libc::c_void,
                tess.vertexColors.as_mut_ptr() as *const libc::c_void,
                (tess.numVertexes as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::tr_local_h::color4ub_t>() as libc::c_ulong
                    ),
            );
        }
        11 => {
            i = 0 as i32;
            while i < tess.numVertexes {
                *(tess.svars.colors[i as usize].as_mut_ptr() as *mut i32) =
                    *((*pStage).constantColor.as_mut_ptr() as *mut i32);
                i += 1
            }
        }
        6 => {
            if crate::src::renderergl1::tr_main::tr.identityLight
                == 1 as i32 as f32
            {
                crate::stdlib::memcpy(
                    tess.svars.colors.as_mut_ptr() as *mut libc::c_void,
                    tess.vertexColors.as_mut_ptr() as *const libc::c_void,
                    (tess.numVertexes as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<crate::tr_local_h::color4ub_t>() as libc::c_ulong
                        ),
                );
            } else {
                i = 0 as i32;
                while i < tess.numVertexes {
                    tess.svars.colors[i as usize][0 as i32 as usize] =
                        (tess.vertexColors[i as usize][0 as i32 as usize] as i32
                            as f32
                            * crate::src::renderergl1::tr_main::tr.identityLight)
                            as crate::src::qcommon::q_shared::byte;
                    tess.svars.colors[i as usize][1 as i32 as usize] =
                        (tess.vertexColors[i as usize][1 as i32 as usize] as i32
                            as f32
                            * crate::src::renderergl1::tr_main::tr.identityLight)
                            as crate::src::qcommon::q_shared::byte;
                    tess.svars.colors[i as usize][2 as i32 as usize] =
                        (tess.vertexColors[i as usize][2 as i32 as usize] as i32
                            as f32
                            * crate::src::renderergl1::tr_main::tr.identityLight)
                            as crate::src::qcommon::q_shared::byte;
                    tess.svars.colors[i as usize][3 as i32 as usize] =
                        tess.vertexColors[i as usize][3 as i32 as usize];
                    i += 1
                }
            }
        }
        7 => {
            if crate::src::renderergl1::tr_main::tr.identityLight
                == 1 as i32 as f32
            {
                i = 0 as i32;
                while i < tess.numVertexes {
                    tess.svars.colors[i as usize][0 as i32 as usize] = (255 as i32
                        - tess.vertexColors[i as usize][0 as i32 as usize] as i32)
                        as crate::src::qcommon::q_shared::byte;
                    tess.svars.colors[i as usize][1 as i32 as usize] = (255 as i32
                        - tess.vertexColors[i as usize][1 as i32 as usize] as i32)
                        as crate::src::qcommon::q_shared::byte;
                    tess.svars.colors[i as usize][2 as i32 as usize] = (255 as i32
                        - tess.vertexColors[i as usize][2 as i32 as usize] as i32)
                        as crate::src::qcommon::q_shared::byte;
                    i += 1
                }
            } else {
                i = 0 as i32;
                while i < tess.numVertexes {
                    tess.svars.colors[i as usize][0 as i32 as usize] = ((255 as i32
                        - tess.vertexColors[i as usize][0 as i32 as usize] as i32)
                        as f32
                        * crate::src::renderergl1::tr_main::tr.identityLight)
                        as crate::src::qcommon::q_shared::byte;
                    tess.svars.colors[i as usize][1 as i32 as usize] = ((255 as i32
                        - tess.vertexColors[i as usize][1 as i32 as usize] as i32)
                        as f32
                        * crate::src::renderergl1::tr_main::tr.identityLight)
                        as crate::src::qcommon::q_shared::byte;
                    tess.svars.colors[i as usize][2 as i32 as usize] = ((255 as i32
                        - tess.vertexColors[i as usize][2 as i32 as usize] as i32)
                        as f32
                        * crate::src::renderergl1::tr_main::tr.identityLight)
                        as crate::src::qcommon::q_shared::byte;
                    i += 1
                }
            }
        }
        10 => {
            let mut fog: *mut crate::tr_local_h::fog_t = 0 as *mut crate::tr_local_h::fog_t;
            fog = (*crate::src::renderergl1::tr_main::tr.world)
                .fogs
                .offset(tess.fogNum as isize);
            i = 0 as i32;
            while i < tess.numVertexes {
                *(&mut *tess.svars.colors.as_mut_ptr().offset(i as isize)
                    as *mut crate::tr_local_h::color4ub_t as *mut i32) =
                    (*fog).colorInt as i32;
                i += 1
            }
        }
        8 => {
            crate::src::renderergl1::tr_shade_calc::RB_CalcWaveColor(
                &mut (*pStage).rgbWave as *mut _ as *const crate::tr_local_h::waveForm_t,
                tess.svars.colors.as_mut_ptr() as *mut u8,
            );
        }
        3 => {
            crate::src::renderergl1::tr_shade_calc::RB_CalcColorFromEntity(
                tess.svars.colors.as_mut_ptr() as *mut u8,
            );
        }
        4 => {
            crate::src::renderergl1::tr_shade_calc::RB_CalcColorFromOneMinusEntity(
                tess.svars.colors.as_mut_ptr() as *mut u8,
            );
        }
        1 | _ => {
            crate::stdlib::memset(
                tess.svars.colors.as_mut_ptr() as *mut libc::c_void,
                crate::src::renderergl1::tr_main::tr.identityLightByte,
                (tess.numVertexes * 4 as i32) as libc::c_ulong,
            );
        }
    }
    //
    // alphaGen
    //
    match (*pStage).alphaGen as u32 {
        0 => {
            if (*pStage).rgbGen as u32
                != crate::tr_local_h::CGEN_IDENTITY as i32 as u32
            {
                if (*pStage).rgbGen as u32
                    == crate::tr_local_h::CGEN_VERTEX as i32 as u32
                    && crate::src::renderergl1::tr_main::tr.identityLight
                        != 1 as i32 as f32
                    || (*pStage).rgbGen as u32
                        != crate::tr_local_h::CGEN_VERTEX as i32 as u32
                {
                    i = 0 as i32;
                    while i < tess.numVertexes {
                        tess.svars.colors[i as usize][3 as i32 as usize] =
                            0xff as i32 as crate::src::qcommon::q_shared::byte;
                        i += 1
                    }
                }
            }
        }
        9 => {
            if (*pStage).rgbGen as u32
                != crate::tr_local_h::CGEN_CONST as i32 as u32
            {
                i = 0 as i32;
                while i < tess.numVertexes {
                    tess.svars.colors[i as usize][3 as i32 as usize] =
                        (*pStage).constantColor[3 as i32 as usize];
                    i += 1
                }
            }
        }
        7 => {
            crate::src::renderergl1::tr_shade_calc::RB_CalcWaveAlpha(
                &mut (*pStage).alphaWave as *mut _ as *const crate::tr_local_h::waveForm_t,
                tess.svars.colors.as_mut_ptr() as *mut u8,
            );
        }
        6 => {
            crate::src::renderergl1::tr_shade_calc::RB_CalcSpecularAlpha(
                tess.svars.colors.as_mut_ptr() as *mut u8,
            );
        }
        2 => {
            crate::src::renderergl1::tr_shade_calc::RB_CalcAlphaFromEntity(
                tess.svars.colors.as_mut_ptr() as *mut u8,
            );
        }
        3 => {
            crate::src::renderergl1::tr_shade_calc::RB_CalcAlphaFromOneMinusEntity(
                tess.svars.colors.as_mut_ptr() as *mut u8,
            );
        }
        4 => {
            if (*pStage).rgbGen as u32
                != crate::tr_local_h::CGEN_VERTEX as i32 as u32
            {
                i = 0 as i32;
                while i < tess.numVertexes {
                    tess.svars.colors[i as usize][3 as i32 as usize] =
                        tess.vertexColors[i as usize][3 as i32 as usize];
                    i += 1
                }
            }
        }
        5 => {
            i = 0 as i32;
            while i < tess.numVertexes {
                tess.svars.colors[i as usize][3 as i32 as usize] = (255 as i32
                    - tess.vertexColors[i as usize][3 as i32 as usize] as i32)
                    as crate::src::qcommon::q_shared::byte;
                i += 1
            }
        }
        8 => {
            let mut alpha: u8 = 0;
            i = 0 as i32;
            while i < tess.numVertexes {
                let mut len: f32 = 0.;
                let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                v[0 as i32 as usize] = tess.xyz[i as usize][0 as i32 as usize]
                    - crate::src::renderergl1::tr_backend::backEnd
                        .viewParms
                        .or
                        .origin[0 as i32 as usize];
                v[1 as i32 as usize] = tess.xyz[i as usize][1 as i32 as usize]
                    - crate::src::renderergl1::tr_backend::backEnd
                        .viewParms
                        .or
                        .origin[1 as i32 as usize];
                v[2 as i32 as usize] = tess.xyz[i as usize][2 as i32 as usize]
                    - crate::src::renderergl1::tr_backend::backEnd
                        .viewParms
                        .or
                        .origin[2 as i32 as usize];
                len = VectorLength(v.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
                len /= (*tess.shader).portalRange;
                if len < 0 as i32 as f32 {
                    alpha = 0 as i32 as u8
                } else if len > 1 as i32 as f32 {
                    alpha = 0xff as i32 as u8
                } else {
                    alpha = (len * 0xff as i32 as f32) as u8
                }
                tess.svars.colors[i as usize][3 as i32 as usize] = alpha;
                i += 1
            }
        }
        1 | _ => {}
    }
    //
    // fog adjustment for colors to fade out as fog increases
    //
    if tess.fogNum != 0 {
        match (*pStage).adjustColorsForFog as u32 {
            1 => {
                crate::src::renderergl1::tr_shade_calc::RB_CalcModulateColorsByFog(
                    tess.svars.colors.as_mut_ptr() as *mut u8,
                );
            }
            3 => {
                crate::src::renderergl1::tr_shade_calc::RB_CalcModulateAlphasByFog(
                    tess.svars.colors.as_mut_ptr() as *mut u8,
                );
            }
            2 => {
                crate::src::renderergl1::tr_shade_calc::RB_CalcModulateRGBAsByFog(
                    tess.svars.colors.as_mut_ptr() as *mut u8,
                );
            }
            0 | _ => {}
        }
    }
    // if in greyscale rendering mode turn all color values into greyscale.
    if (*crate::src::renderergl1::tr_init::r_greyscale).integer != 0 {
        let mut scale: i32 = 0;
        i = 0 as i32;
        while i < tess.numVertexes {
            scale = (0.2126f32
                * tess.svars.colors[i as usize][0 as i32 as usize] as i32
                    as f32
                + 0.7152f32
                    * tess.svars.colors[i as usize][1 as i32 as usize] as i32
                        as f32
                + 0.0722f32
                    * tess.svars.colors[i as usize][2 as i32 as usize] as i32
                        as f32) as i32;
            tess.svars.colors[i as usize][2 as i32 as usize] =
                scale as crate::src::qcommon::q_shared::byte;
            tess.svars.colors[i as usize][1 as i32 as usize] =
                tess.svars.colors[i as usize][2 as i32 as usize];
            tess.svars.colors[i as usize][0 as i32 as usize] =
                tess.svars.colors[i as usize][1 as i32 as usize];
            i += 1
        }
    } else if (*crate::src::renderergl1::tr_init::r_greyscale).value != 0. {
        let mut scale_0: f32 = 0.;
        i = 0 as i32;
        while i < tess.numVertexes {
            scale_0 = 0.2126f32
                * tess.svars.colors[i as usize][0 as i32 as usize] as i32
                    as f32
                + 0.7152f32
                    * tess.svars.colors[i as usize][1 as i32 as usize] as i32
                        as f32
                + 0.0722f32
                    * tess.svars.colors[i as usize][2 as i32 as usize] as i32
                        as f32;
            tess.svars.colors[i as usize][0 as i32 as usize] =
                (tess.svars.colors[i as usize][0 as i32 as usize] as i32
                    as f32
                    * (1.0f32 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    + scale_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    as crate::src::qcommon::q_shared::byte;
            tess.svars.colors[i as usize][1 as i32 as usize] =
                (tess.svars.colors[i as usize][1 as i32 as usize] as i32
                    as f32
                    * (1.0f32 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    + scale_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    as crate::src::qcommon::q_shared::byte;
            tess.svars.colors[i as usize][2 as i32 as usize] =
                (tess.svars.colors[i as usize][2 as i32 as usize] as i32
                    as f32
                    * (1.0f32 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    + scale_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    as crate::src::qcommon::q_shared::byte;
            i += 1
        }
    };
}
/*
===============
ComputeTexCoords
===============
*/

unsafe extern "C" fn ComputeTexCoords(mut pStage: *mut crate::tr_local_h::shaderStage_t) {
    let mut i: i32 = 0;
    let mut b: i32 = 0;
    b = 0 as i32;
    while b < 2 as i32 {
        let mut tm: i32 = 0;
        //
        // generate the texture coordinates
        //
        match (*pStage).bundle[b as usize].tcGen as u32 {
            1 => {
                crate::stdlib::memset(
                    tess.svars.texcoords[b as usize].as_mut_ptr() as *mut libc::c_void,
                    0 as i32,
                    (::std::mem::size_of::<f32>() as libc::c_ulong)
                        .wrapping_mul(2 as i32 as libc::c_ulong)
                        .wrapping_mul(tess.numVertexes as libc::c_ulong),
                );
            }
            3 => {
                i = 0 as i32;
                while i < tess.numVertexes {
                    tess.svars.texcoords[b as usize][i as usize][0 as i32 as usize] = tess
                        .texCoords[i as usize][0 as i32 as usize]
                        [0 as i32 as usize];
                    tess.svars.texcoords[b as usize][i as usize][1 as i32 as usize] = tess
                        .texCoords[i as usize][0 as i32 as usize]
                        [1 as i32 as usize];
                    i += 1
                }
            }
            2 => {
                i = 0 as i32;
                while i < tess.numVertexes {
                    tess.svars.texcoords[b as usize][i as usize][0 as i32 as usize] = tess
                        .texCoords[i as usize][1 as i32 as usize]
                        [0 as i32 as usize];
                    tess.svars.texcoords[b as usize][i as usize][1 as i32 as usize] = tess
                        .texCoords[i as usize][1 as i32 as usize]
                        [1 as i32 as usize];
                    i += 1
                }
            }
            6 => {
                i = 0 as i32;
                while i < tess.numVertexes {
                    tess.svars.texcoords[b as usize][i as usize][0 as i32 as usize] = tess
                        .xyz[i as usize][0 as i32 as usize]
                        * (*pStage).bundle[b as usize].tcGenVectors[0 as i32 as usize]
                            [0 as i32 as usize]
                        + tess.xyz[i as usize][1 as i32 as usize]
                            * (*pStage).bundle[b as usize].tcGenVectors[0 as i32 as usize]
                                [1 as i32 as usize]
                        + tess.xyz[i as usize][2 as i32 as usize]
                            * (*pStage).bundle[b as usize].tcGenVectors[0 as i32 as usize]
                                [2 as i32 as usize];
                    tess.svars.texcoords[b as usize][i as usize][1 as i32 as usize] = tess
                        .xyz[i as usize][0 as i32 as usize]
                        * (*pStage).bundle[b as usize].tcGenVectors[1 as i32 as usize]
                            [0 as i32 as usize]
                        + tess.xyz[i as usize][1 as i32 as usize]
                            * (*pStage).bundle[b as usize].tcGenVectors[1 as i32 as usize]
                                [1 as i32 as usize]
                        + tess.xyz[i as usize][2 as i32 as usize]
                            * (*pStage).bundle[b as usize].tcGenVectors[1 as i32 as usize]
                                [2 as i32 as usize];
                    i += 1
                }
            }
            5 => {
                crate::src::renderergl1::tr_shade_calc::RB_CalcFogTexCoords(
                    tess.svars.texcoords[b as usize].as_mut_ptr() as *mut f32,
                );
            }
            4 => {
                crate::src::renderergl1::tr_shade_calc::RB_CalcEnvironmentTexCoords(
                    tess.svars.texcoords[b as usize].as_mut_ptr() as *mut f32,
                );
            }
            0 => return,
            _ => {}
        }
        //
        // alter texture coordinates
        //
        tm = 0 as i32; // break out of for loop
        while tm < (*pStage).bundle[b as usize].numTexMods {
            match (*(*pStage).bundle[b as usize].texMods.offset(tm as isize)).type_0 as u32
            {
                0 => tm = 4 as i32,
                2 => {
                    crate::src::renderergl1::tr_shade_calc::RB_CalcTurbulentTexCoords(
                        &mut (*(*(*pStage).bundle.as_mut_ptr().offset(b as isize))
                            .texMods
                            .offset(tm as isize))
                        .wave as *mut _
                            as *const crate::tr_local_h::waveForm_t,
                        tess.svars.texcoords[b as usize].as_mut_ptr() as *mut f32,
                    );
                }
                7 => {
                    crate::src::renderergl1::tr_shade_calc::RB_CalcScrollTexCoords(
                        (*crate::src::renderergl1::tr_backend::backEnd.currentEntity)
                            .e
                            .shaderTexCoord
                            .as_mut_ptr() as *const f32,
                        tess.svars.texcoords[b as usize].as_mut_ptr() as *mut f32,
                    );
                }
                3 => {
                    crate::src::renderergl1::tr_shade_calc::RB_CalcScrollTexCoords(
                        (*(*pStage).bundle[b as usize].texMods.offset(tm as isize))
                            .scroll
                            .as_mut_ptr() as *const f32,
                        tess.svars.texcoords[b as usize].as_mut_ptr() as *mut f32,
                    );
                }
                4 => {
                    crate::src::renderergl1::tr_shade_calc::RB_CalcScaleTexCoords(
                        (*(*pStage).bundle[b as usize].texMods.offset(tm as isize))
                            .scale
                            .as_mut_ptr() as *const f32,
                        tess.svars.texcoords[b as usize].as_mut_ptr() as *mut f32,
                    );
                }
                5 => {
                    crate::src::renderergl1::tr_shade_calc::RB_CalcStretchTexCoords(
                        &mut (*(*(*pStage).bundle.as_mut_ptr().offset(b as isize))
                            .texMods
                            .offset(tm as isize))
                        .wave as *mut _
                            as *const crate::tr_local_h::waveForm_t,
                        tess.svars.texcoords[b as usize].as_mut_ptr() as *mut f32,
                    );
                }
                1 => {
                    crate::src::renderergl1::tr_shade_calc::RB_CalcTransformTexCoords(
                        &mut *(*(*pStage).bundle.as_mut_ptr().offset(b as isize))
                            .texMods
                            .offset(tm as isize) as *mut _
                            as *const crate::tr_local_h::texModInfo_t,
                        tess.svars.texcoords[b as usize].as_mut_ptr() as *mut f32,
                    );
                }
                6 => {
                    crate::src::renderergl1::tr_shade_calc::RB_CalcRotateTexCoords(
                        (*(*pStage).bundle[b as usize].texMods.offset(tm as isize)).rotateSpeed,
                        tess.svars.texcoords[b as usize].as_mut_ptr() as *mut f32,
                    );
                }
                _ => {
                    crate::src::renderergl1::tr_main::ri
                        .Error
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::ERR_DROP as i32,
                        b"ERROR: unknown texmod \'%d\' in shader \'%s\'\x00" as *const u8
                            as *const libc::c_char,
                        (*(*pStage).bundle[b as usize].texMods.offset(tm as isize)).type_0
                            as u32,
                        (*tess.shader).name.as_mut_ptr(),
                    );
                }
            }
            tm += 1
        }
        b += 1
    }
}
/*
** RB_IterateStagesGeneric
*/

unsafe extern "C" fn RB_IterateStagesGeneric(mut input: *mut crate::tr_local_h::shaderCommands_t) {
    let mut stage: i32 = 0;
    stage = 0 as i32;
    while stage < 8 as i32 {
        let mut pStage: *mut crate::tr_local_h::shaderStage_t =
            *tess.xstages.offset(stage as isize);
        if pStage.is_null() {
            break;
        }
        ComputeColors(pStage);
        ComputeTexCoords(pStage);
        if setArraysOnce as u64 == 0 {
            crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
                0x8076 as i32 as crate::stdlib::GLenum,
            );
            crate::src::sdl::sdl_glimp::qglColorPointer.expect("non-null function pointer")(
                4 as i32,
                0x1401 as i32 as crate::stdlib::GLenum,
                0 as i32,
                (*input).svars.colors.as_mut_ptr() as *const libc::c_void,
            );
        }
        //
        // do multitexture
        //
        if !(*pStage).bundle[1 as i32 as usize].image[0 as i32 as usize].is_null() {
            DrawMultitextured(input, stage);
        } else {
            if setArraysOnce as u64 == 0 {
                crate::src::sdl::sdl_glimp::qglTexCoordPointer.expect("non-null function pointer")(
                    2 as i32,
                    0x1406 as i32 as crate::stdlib::GLenum,
                    0 as i32,
                    (*input).svars.texcoords[0 as i32 as usize].as_mut_ptr()
                        as *const libc::c_void,
                );
            }
            //
            // set state
            //
            R_BindAnimatedImage(
                &mut *(*pStage)
                    .bundle
                    .as_mut_ptr()
                    .offset(0 as i32 as isize),
            );
            crate::src::renderergl1::tr_backend::GL_State((*pStage).stateBits as libc::c_ulong);
            //
            // draw
            //
            R_DrawElements((*input).numIndexes, (*input).indexes.as_mut_ptr());
        }
        // allow skipping out to show just lightmaps during development
        if (*crate::src::renderergl1::tr_init::r_lightmap).integer != 0
            && ((*pStage).bundle[0 as i32 as usize].isLightmap as u32 != 0
                || (*pStage).bundle[1 as i32 as usize].isLightmap as u32 != 0)
        {
            break;
        }
        stage += 1
    }
}
/*
** RB_StageIteratorGeneric
*/
#[no_mangle]

pub unsafe extern "C" fn RB_StageIteratorGeneric() {
    let mut input: *mut crate::tr_local_h::shaderCommands_t =
        0 as *mut crate::tr_local_h::shaderCommands_t;
    let mut shader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    input = &mut tess;
    shader = (*input).shader;
    crate::src::renderergl1::tr_shade_calc::RB_DeformTessGeometry();
    //
    // log this call
    //
    if (*crate::src::renderergl1::tr_init::r_logFile).integer != 0 {
        // don't just call LogComment, or we will get
        // a call to va() every frame!
        crate::src::sdl::sdl_glimp::GLimp_LogComment(crate::src::qcommon::q_shared::va(
            b"--- RB_StageIteratorGeneric( %s ) ---\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*tess.shader).name.as_mut_ptr(),
        ));
    }
    //
    // set face culling appropriately
    //
    crate::src::renderergl1::tr_backend::GL_Cull((*shader).cullType as i32);
    // set polygon offset if necessary
    if (*shader).polygonOffset as u64 != 0 {
        crate::src::sdl::sdl_glimp::qglEnable.expect("non-null function pointer")(
            0x8037 as i32 as crate::stdlib::GLenum,
        );
        crate::src::sdl::sdl_glimp::qglPolygonOffset.expect("non-null function pointer")(
            (*crate::src::renderergl1::tr_init::r_offsetFactor).value,
            (*crate::src::renderergl1::tr_init::r_offsetUnits).value,
        );
    }
    //
    // if there is only a single pass then we can enable color
    // and texture arrays before we compile, otherwise we need
    // to avoid compiling those arrays since they will change
    // during multipass rendering
    //
    if tess.numPasses > 1 as i32 || (*shader).multitextureEnv != 0 {
        setArraysOnce = crate::src::qcommon::q_shared::qfalse;
        crate::src::sdl::sdl_glimp::qglDisableClientState.expect("non-null function pointer")(
            0x8076 as i32 as crate::stdlib::GLenum,
        );
        crate::src::sdl::sdl_glimp::qglDisableClientState.expect("non-null function pointer")(
            0x8078 as i32 as crate::stdlib::GLenum,
        );
    } else {
        setArraysOnce = crate::src::qcommon::q_shared::qtrue;
        crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
            0x8076 as i32 as crate::stdlib::GLenum,
        );
        crate::src::sdl::sdl_glimp::qglColorPointer.expect("non-null function pointer")(
            4 as i32,
            0x1401 as i32 as crate::stdlib::GLenum,
            0 as i32,
            tess.svars.colors.as_mut_ptr() as *const libc::c_void,
        );
        crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
            0x8078 as i32 as crate::stdlib::GLenum,
        );
        crate::src::sdl::sdl_glimp::qglTexCoordPointer.expect("non-null function pointer")(
            2 as i32,
            0x1406 as i32 as crate::stdlib::GLenum,
            0 as i32,
            tess.svars.texcoords[0 as i32 as usize].as_mut_ptr() as *const libc::c_void,
        );
    }
    //
    // lock XYZ
    //
    crate::src::sdl::sdl_glimp::qglVertexPointer.expect("non-null function pointer")(
        3 as i32,
        0x1406 as i32 as crate::stdlib::GLenum,
        16 as i32,
        (*input).xyz.as_mut_ptr() as *const libc::c_void,
    ); // padded for SIMD
    if crate::src::sdl::sdl_glimp::qglLockArraysEXT.is_some() {
        crate::src::sdl::sdl_glimp::qglLockArraysEXT.expect("non-null function pointer")(
            0 as i32,
            (*input).numVertexes,
        );
        crate::src::sdl::sdl_glimp::GLimp_LogComment(
            b"glLockArraysEXT\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    //
    // enable color and texcoord arrays after the lock if necessary
    //
    if setArraysOnce as u64 == 0 {
        crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
            0x8078 as i32 as crate::stdlib::GLenum,
        );
        crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
            0x8076 as i32 as crate::stdlib::GLenum,
        );
    }
    //
    // call shader function
    //
    RB_IterateStagesGeneric(input);
    //
    // now do any dynamic lighting needed
    //
    if tess.dlightBits != 0
        && (*tess.shader).sort <= crate::tr_local_h::SS_OPAQUE as i32 as f32
        && (*tess.shader).surfaceFlags & (0x20000 as i32 | 0x4 as i32) == 0
    {
        ProjectDlightTexture();
    }
    //
    // now do fog
    //
    if tess.fogNum != 0 && (*tess.shader).fogPass as u32 != 0 {
        RB_FogPass();
    }
    //
    // unlock arrays
    //
    if crate::src::sdl::sdl_glimp::qglUnlockArraysEXT.is_some() {
        crate::src::sdl::sdl_glimp::qglUnlockArraysEXT.expect("non-null function pointer")();
        crate::src::sdl::sdl_glimp::GLimp_LogComment(
            b"glUnlockArraysEXT\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    //
    // reset polygon offset
    //
    if (*shader).polygonOffset as u64 != 0 {
        crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
            0x8037 as i32 as crate::stdlib::GLenum,
        );
    };
}
/*
** RB_StageIteratorVertexLitTexture
*/
#[no_mangle]

pub unsafe extern "C" fn RB_StageIteratorVertexLitTexture() {
    let mut input: *mut crate::tr_local_h::shaderCommands_t =
        0 as *mut crate::tr_local_h::shaderCommands_t;
    let mut shader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    input = &mut tess;
    shader = (*input).shader;
    //
    // compute colors
    //
    crate::src::renderergl1::tr_shade_calc::RB_CalcDiffuseColor(
        tess.svars.colors.as_mut_ptr() as *mut u8
    );
    //
    // log this call
    //
    if (*crate::src::renderergl1::tr_init::r_logFile).integer != 0 {
        // don't just call LogComment, or we will get
        // a call to va() every frame!
        crate::src::sdl::sdl_glimp::GLimp_LogComment(crate::src::qcommon::q_shared::va(
            b"--- RB_StageIteratorVertexLitTexturedUnfogged( %s ) ---\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*tess.shader).name.as_mut_ptr(),
        ));
    }
    //
    // set face culling appropriately
    //
    crate::src::renderergl1::tr_backend::GL_Cull((*shader).cullType as i32);
    //
    // set arrays and lock
    //
    crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
        0x8076 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
        0x8078 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglColorPointer.expect("non-null function pointer")(
        4 as i32,
        0x1401 as i32 as crate::stdlib::GLenum,
        0 as i32,
        tess.svars.colors.as_mut_ptr() as *const libc::c_void,
    );
    crate::src::sdl::sdl_glimp::qglTexCoordPointer.expect("non-null function pointer")(
        2 as i32,
        0x1406 as i32 as crate::stdlib::GLenum,
        16 as i32,
        tess.texCoords[0 as i32 as usize][0 as i32 as usize].as_mut_ptr()
            as *const libc::c_void,
    );
    crate::src::sdl::sdl_glimp::qglVertexPointer.expect("non-null function pointer")(
        3 as i32,
        0x1406 as i32 as crate::stdlib::GLenum,
        16 as i32,
        (*input).xyz.as_mut_ptr() as *const libc::c_void,
    );
    if crate::src::sdl::sdl_glimp::qglLockArraysEXT.is_some() {
        crate::src::sdl::sdl_glimp::qglLockArraysEXT.expect("non-null function pointer")(
            0 as i32,
            (*input).numVertexes,
        );
        crate::src::sdl::sdl_glimp::GLimp_LogComment(
            b"glLockArraysEXT\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    //
    // call special shade routine
    //
    R_BindAnimatedImage(
        &mut *(**tess.xstages.offset(0 as i32 as isize))
            .bundle
            .as_mut_ptr()
            .offset(0 as i32 as isize),
    );
    crate::src::renderergl1::tr_backend::GL_State(
        (**tess.xstages.offset(0 as i32 as isize)).stateBits as libc::c_ulong,
    );
    R_DrawElements((*input).numIndexes, (*input).indexes.as_mut_ptr());
    //
    // now do any dynamic lighting needed
    //
    if tess.dlightBits != 0
        && (*tess.shader).sort <= crate::tr_local_h::SS_OPAQUE as i32 as f32
    {
        ProjectDlightTexture();
    }
    //
    // now do fog
    //
    if tess.fogNum != 0 && (*tess.shader).fogPass as u32 != 0 {
        RB_FogPass();
    }
    //
    // unlock arrays
    //
    if crate::src::sdl::sdl_glimp::qglUnlockArraysEXT.is_some() {
        crate::src::sdl::sdl_glimp::qglUnlockArraysEXT.expect("non-null function pointer")();
        crate::src::sdl::sdl_glimp::GLimp_LogComment(
            b"glUnlockArraysEXT\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    };
}
//define	REPLACE_MODE
#[no_mangle]

pub unsafe extern "C" fn RB_StageIteratorLightmappedMultitexture() {
    let mut input: *mut crate::tr_local_h::shaderCommands_t =
        0 as *mut crate::tr_local_h::shaderCommands_t;
    let mut shader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    input = &mut tess;
    shader = (*input).shader;
    //
    // log this call
    //
    if (*crate::src::renderergl1::tr_init::r_logFile).integer != 0 {
        // don't just call LogComment, or we will get
        // a call to va() every frame!
        crate::src::sdl::sdl_glimp::GLimp_LogComment(crate::src::qcommon::q_shared::va(
            b"--- RB_StageIteratorLightmappedMultitexture( %s ) ---\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*tess.shader).name.as_mut_ptr(),
        ));
    }
    //
    // set face culling appropriately
    //
    crate::src::renderergl1::tr_backend::GL_Cull((*shader).cullType as i32);
    //
    // set color, pointers, and lock
    //
    crate::src::renderergl1::tr_backend::GL_State(0x100 as i32 as libc::c_ulong);
    crate::src::sdl::sdl_glimp::qglVertexPointer.expect("non-null function pointer")(
        3 as i32,
        0x1406 as i32 as crate::stdlib::GLenum,
        16 as i32,
        (*input).xyz.as_mut_ptr() as *const libc::c_void,
    );
    crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
        0x8076 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglColorPointer.expect("non-null function pointer")(
        4 as i32,
        0x1401 as i32 as crate::stdlib::GLenum,
        0 as i32,
        tess.constantColor255.as_mut_ptr() as *const libc::c_void,
    );
    //
    // select base stage
    //
    crate::src::renderergl1::tr_backend::GL_SelectTexture(0 as i32);
    crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
        0x8078 as i32 as crate::stdlib::GLenum,
    );
    R_BindAnimatedImage(
        &mut *(**tess.xstages.offset(0 as i32 as isize))
            .bundle
            .as_mut_ptr()
            .offset(0 as i32 as isize),
    );
    crate::src::sdl::sdl_glimp::qglTexCoordPointer.expect("non-null function pointer")(
        2 as i32,
        0x1406 as i32 as crate::stdlib::GLenum,
        16 as i32,
        tess.texCoords[0 as i32 as usize][0 as i32 as usize].as_mut_ptr()
            as *const libc::c_void,
    );
    //
    // configure second stage
    //
    crate::src::renderergl1::tr_backend::GL_SelectTexture(1 as i32);
    crate::src::sdl::sdl_glimp::qglEnable.expect("non-null function pointer")(
        0xde1 as i32 as crate::stdlib::GLenum,
    );
    if (*crate::src::renderergl1::tr_init::r_lightmap).integer != 0 {
        crate::src::renderergl1::tr_backend::GL_TexEnv(0x1e01 as i32);
    } else {
        crate::src::renderergl1::tr_backend::GL_TexEnv(0x2100 as i32);
    }
    R_BindAnimatedImage(
        &mut *(**tess.xstages.offset(0 as i32 as isize))
            .bundle
            .as_mut_ptr()
            .offset(1 as i32 as isize),
    );
    crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
        0x8078 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglTexCoordPointer.expect("non-null function pointer")(
        2 as i32,
        0x1406 as i32 as crate::stdlib::GLenum,
        16 as i32,
        tess.texCoords[0 as i32 as usize][1 as i32 as usize].as_mut_ptr()
            as *const libc::c_void,
    );
    //
    // lock arrays
    //
    if crate::src::sdl::sdl_glimp::qglLockArraysEXT.is_some() {
        crate::src::sdl::sdl_glimp::qglLockArraysEXT.expect("non-null function pointer")(
            0 as i32,
            (*input).numVertexes,
        );
        crate::src::sdl::sdl_glimp::GLimp_LogComment(
            b"glLockArraysEXT\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    R_DrawElements((*input).numIndexes, (*input).indexes.as_mut_ptr());
    //
    // disable texturing on TEXTURE1, then select TEXTURE0
    //
    crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
        0xde1 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglDisableClientState.expect("non-null function pointer")(
        0x8078 as i32 as crate::stdlib::GLenum,
    );
    crate::src::renderergl1::tr_backend::GL_SelectTexture(0 as i32);
    //
    // now do any dynamic lighting needed
    //
    if tess.dlightBits != 0
        && (*tess.shader).sort <= crate::tr_local_h::SS_OPAQUE as i32 as f32
    {
        ProjectDlightTexture();
    }
    //
    // now do fog
    //
    if tess.fogNum != 0 && (*tess.shader).fogPass as u32 != 0 {
        RB_FogPass();
    }
    //
    // unlock arrays
    //
    if crate::src::sdl::sdl_glimp::qglUnlockArraysEXT.is_some() {
        crate::src::sdl::sdl_glimp::qglUnlockArraysEXT.expect("non-null function pointer")();
        crate::src::sdl::sdl_glimp::GLimp_LogComment(
            b"glUnlockArraysEXT\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
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
** RB_EndSurface
*/
#[no_mangle]

pub unsafe extern "C" fn RB_EndSurface() {
    let mut input: *mut crate::tr_local_h::shaderCommands_t =
        0 as *mut crate::tr_local_h::shaderCommands_t;
    input = &mut tess;
    if (*input).numIndexes == 0 as i32 {
        return;
    }
    if (*input).indexes[(6 as i32 * 1000 as i32 - 1 as i32) as usize]
        != 0 as i32 as u32
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"RB_EndSurface() - SHADER_MAX_INDEXES hit\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*input).xyz[(1000 as i32 - 1 as i32) as usize][0 as i32 as usize]
        != 0 as i32 as f32
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"RB_EndSurface() - SHADER_MAX_VERTEXES hit\x00" as *const u8 as *const libc::c_char,
        );
    }
    if tess.shader == crate::src::renderergl1::tr_main::tr.shadowShader {
        crate::src::renderergl1::tr_shadows::RB_ShadowTessEnd();
        return;
    }
    // for debugging of sort order issues, stop rendering after a given sort value
    if (*crate::src::renderergl1::tr_init::r_debugSort).integer != 0
        && ((*crate::src::renderergl1::tr_init::r_debugSort).integer as f32)
            < (*tess.shader).sort
    {
        return;
    }
    //
    // update performance counters
    //
    crate::src::renderergl1::tr_backend::backEnd.pc.c_shaders += 1;
    crate::src::renderergl1::tr_backend::backEnd.pc.c_vertexes += tess.numVertexes;
    crate::src::renderergl1::tr_backend::backEnd.pc.c_indexes += tess.numIndexes;
    crate::src::renderergl1::tr_backend::backEnd
        .pc
        .c_totalIndexes += tess.numIndexes * tess.numPasses;
    //
    // call off to shader specific tess end function
    //
    tess.currentStageIteratorFunc
        .expect("non-null function pointer")();
    //
    // draw debugging stuff
    //
    if (*crate::src::renderergl1::tr_init::r_showtris).integer != 0 {
        DrawTris(input);
    }
    if (*crate::src::renderergl1::tr_init::r_shownormals).integer != 0 {
        DrawNormals(input);
    }
    // clear shader so we can tell we don't have any unclosed surfaces
    tess.numIndexes = 0 as i32;
    crate::src::sdl::sdl_glimp::GLimp_LogComment(
        b"----------\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}

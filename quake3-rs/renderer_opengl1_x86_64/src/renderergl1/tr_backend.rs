use ::libc;

pub use crate::stdlib::intptr_t;

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
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

pub use crate::qgl_h::AlphaFuncproc;
pub use crate::qgl_h::Beginproc;
pub use crate::qgl_h::BindTextureproc;
pub use crate::qgl_h::BlendFuncproc;
pub use crate::qgl_h::ClearColorproc;
pub use crate::qgl_h::Clearproc;
pub use crate::qgl_h::ClipPlaneproc;
pub use crate::qgl_h::Color3fproc;
pub use crate::qgl_h::ColorMaskproc;
pub use crate::qgl_h::CullFaceproc;
pub use crate::qgl_h::DepthFuncproc;
pub use crate::qgl_h::DepthMaskproc;
pub use crate::qgl_h::DepthRangeproc;
pub use crate::qgl_h::Disableproc;
pub use crate::qgl_h::DrawBufferproc;
pub use crate::qgl_h::Enableproc;
pub use crate::qgl_h::Endproc;
pub use crate::qgl_h::Finishproc;
pub use crate::qgl_h::LoadIdentityproc;
pub use crate::qgl_h::LoadMatrixfproc;
pub use crate::qgl_h::MatrixModeproc;
pub use crate::qgl_h::Orthoproc;
pub use crate::qgl_h::PolygonModeproc;
pub use crate::qgl_h::ReadPixelsproc;
pub use crate::qgl_h::Scissorproc;
pub use crate::qgl_h::TexCoord2fproc;
pub use crate::qgl_h::TexEnvfproc;
pub use crate::qgl_h::TexImage2Dproc;
pub use crate::qgl_h::TexParameterfproc;
pub use crate::qgl_h::TexSubImage2Dproc;
pub use crate::qgl_h::Vertex2fproc;
pub use crate::qgl_h::Viewportproc;
pub use crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands;
pub use crate::src::renderergl1::tr_flares::RB_RenderFlares;
pub use crate::src::renderergl1::tr_init::glConfig;
pub use crate::src::renderergl1::tr_init::glState;
pub use crate::src::renderergl1::tr_init::r_clear;
pub use crate::src::renderergl1::tr_init::r_drawSun;
pub use crate::src::renderergl1::tr_init::r_fastsky;
pub use crate::src::renderergl1::tr_init::r_finish;
pub use crate::src::renderergl1::tr_init::r_measureOverdraw;
pub use crate::src::renderergl1::tr_init::r_nobind;
pub use crate::src::renderergl1::tr_init::r_shadows;
pub use crate::src::renderergl1::tr_init::r_showImages;
pub use crate::src::renderergl1::tr_init::r_speeds;
pub use crate::src::renderergl1::tr_init::r_znear;
pub use crate::src::renderergl1::tr_init::RB_TakeScreenshotCmd;
pub use crate::src::renderergl1::tr_init::RB_TakeVideoFrameCmd;
pub use crate::src::renderergl1::tr_light::R_TransformDlights;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_main::R_DecomposeSort;
pub use crate::src::renderergl1::tr_main::R_RotateForEntity;
pub use crate::src::renderergl1::tr_main::R_SetupProjection;
pub use crate::src::renderergl1::tr_shade::tess;
pub use crate::src::renderergl1::tr_shade::RB_BeginSurface;
pub use crate::src::renderergl1::tr_shade::RB_EndSurface;
pub use crate::src::renderergl1::tr_shadows::RB_ShadowFinish;
pub use crate::src::renderergl1::tr_sky::RB_DrawSun;
pub use crate::src::renderergl1::tr_surface::rb_surfaceTable;
pub use crate::src::renderergl1::tr_surface::RB_CheckOverflow;
pub use crate::src::sdl::sdl_glimp::qglActiveTextureARB;
pub use crate::src::sdl::sdl_glimp::qglAlphaFunc;
pub use crate::src::sdl::sdl_glimp::qglBegin;
pub use crate::src::sdl::sdl_glimp::qglBindTexture;
pub use crate::src::sdl::sdl_glimp::qglBlendFunc;
pub use crate::src::sdl::sdl_glimp::qglClear;
pub use crate::src::sdl::sdl_glimp::qglClearColor;
pub use crate::src::sdl::sdl_glimp::qglClientActiveTextureARB;
pub use crate::src::sdl::sdl_glimp::qglClipPlane;
pub use crate::src::sdl::sdl_glimp::qglColor3f;
pub use crate::src::sdl::sdl_glimp::qglColorMask;
pub use crate::src::sdl::sdl_glimp::qglCullFace;
pub use crate::src::sdl::sdl_glimp::qglDepthFunc;
pub use crate::src::sdl::sdl_glimp::qglDepthMask;
pub use crate::src::sdl::sdl_glimp::qglDepthRange;
pub use crate::src::sdl::sdl_glimp::qglDisable;
pub use crate::src::sdl::sdl_glimp::qglDrawBuffer;
pub use crate::src::sdl::sdl_glimp::qglEnable;
pub use crate::src::sdl::sdl_glimp::qglEnd;
pub use crate::src::sdl::sdl_glimp::qglFinish;
pub use crate::src::sdl::sdl_glimp::qglLoadIdentity;
pub use crate::src::sdl::sdl_glimp::qglLoadMatrixf;
pub use crate::src::sdl::sdl_glimp::qglMatrixMode;
pub use crate::src::sdl::sdl_glimp::qglOrtho;
pub use crate::src::sdl::sdl_glimp::qglPolygonMode;
pub use crate::src::sdl::sdl_glimp::qglReadPixels;
pub use crate::src::sdl::sdl_glimp::qglScissor;
pub use crate::src::sdl::sdl_glimp::qglTexCoord2f;
pub use crate::src::sdl::sdl_glimp::qglTexEnvf;
pub use crate::src::sdl::sdl_glimp::qglTexImage2D;
pub use crate::src::sdl::sdl_glimp::qglTexParameterf;
pub use crate::src::sdl::sdl_glimp::qglTexSubImage2D;
pub use crate::src::sdl::sdl_glimp::qglVertex2f;
pub use crate::src::sdl::sdl_glimp::qglViewport;
pub use crate::src::sdl::sdl_glimp::GLimp_EndFrame;
pub use crate::src::sdl::sdl_glimp::GLimp_LogComment;
pub use crate::stdlib::GLbitfield;
pub use crate::stdlib::GLboolean;
pub use crate::stdlib::GLclampd;
pub use crate::stdlib::GLclampf;
pub use crate::stdlib::GLdouble;
pub use crate::stdlib::GLenum;
pub use crate::stdlib::GLfloat;
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
pub use crate::tr_local_h::backEndData_t;
pub use crate::tr_local_h::backEndState_t;
pub use crate::tr_local_h::bmodel_t;
pub use crate::tr_local_h::clearDepthCommand_t;
pub use crate::tr_local_h::color4ub_t;
pub use crate::tr_local_h::colorGen_t;
pub use crate::tr_local_h::colorMaskCommand_t;
pub use crate::tr_local_h::cullType_t;
pub use crate::tr_local_h::deformStage_t;
pub use crate::tr_local_h::deform_t;
pub use crate::tr_local_h::dlight_s;
pub use crate::tr_local_h::dlight_t;
pub use crate::tr_local_h::drawBufferCommand_t;
pub use crate::tr_local_h::drawSurf_s;
pub use crate::tr_local_h::drawSurf_t;
pub use crate::tr_local_h::drawSurfsCommand_t;
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
pub use crate::tr_local_h::renderCommandList_t;
pub use crate::tr_local_h::setColorCommand_t;
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
pub use crate::tr_local_h::srfPoly_t;
pub use crate::tr_local_h::stageVars;
pub use crate::tr_local_h::stageVars_t;
pub use crate::tr_local_h::stretchPicCommand_t;
pub use crate::tr_local_h::surfaceType_t;
pub use crate::tr_local_h::swapBuffersCommand_t;
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
pub use crate::tr_local_h::RC_CLEARDEPTH;
pub use crate::tr_local_h::RC_COLORMASK;
pub use crate::tr_local_h::RC_DRAW_BUFFER;
pub use crate::tr_local_h::RC_DRAW_SURFS;
pub use crate::tr_local_h::RC_END_OF_LIST;
pub use crate::tr_local_h::RC_SCREENSHOT;
pub use crate::tr_local_h::RC_SET_COLOR;
pub use crate::tr_local_h::RC_STRETCH_PIC;
pub use crate::tr_local_h::RC_SWAP_BUFFERS;
pub use crate::tr_local_h::RC_VIDEOFRAME;
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
#[no_mangle]

pub static mut backEndData: *mut crate::tr_local_h::backEndData_t =
    0 as *const crate::tr_local_h::backEndData_t as *mut crate::tr_local_h::backEndData_t;
#[no_mangle]

pub static mut backEnd: crate::tr_local_h::backEndState_t = crate::tr_local_h::backEndState_t {
    refdef: crate::tr_local_h::trRefdef_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        fov_x: 0.,
        fov_y: 0.,
        vieworg: [0.; 3],
        viewaxis: [[0.; 3]; 3],
        stereoFrame: crate::tr_types_h::STEREO_CENTER,
        time: 0,
        rdflags: 0,
        areamask: [0; 32],
        areamaskModified: crate::src::qcommon::q_shared::qfalse,
        floatTime: 0.,
        text: [[0; 32]; 8],
        num_entities: 0,
        entities: 0 as *const crate::tr_local_h::trRefEntity_t
            as *mut crate::tr_local_h::trRefEntity_t,
        num_dlights: 0,
        dlights: 0 as *const crate::tr_local_h::dlight_s as *mut crate::tr_local_h::dlight_s,
        numPolys: 0,
        polys: 0 as *const crate::tr_local_h::srfPoly_s as *mut crate::tr_local_h::srfPoly_s,
        numDrawSurfs: 0,
        drawSurfs: 0 as *const crate::tr_local_h::drawSurf_s as *mut crate::tr_local_h::drawSurf_s,
    },
    viewParms: crate::tr_local_h::viewParms_t {
        or: crate::tr_local_h::orientationr_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
            viewOrigin: [0.; 3],
            modelMatrix: [0.; 16],
        },
        world: crate::tr_local_h::orientationr_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
            viewOrigin: [0.; 3],
            modelMatrix: [0.; 16],
        },
        pvsOrigin: [0.; 3],
        isPortal: crate::src::qcommon::q_shared::qfalse,
        isMirror: crate::src::qcommon::q_shared::qfalse,
        frameSceneNum: 0,
        frameCount: 0,
        portalPlane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        viewportX: 0,
        viewportY: 0,
        viewportWidth: 0,
        viewportHeight: 0,
        fovX: 0.,
        fovY: 0.,
        projectionMatrix: [0.; 16],
        frustum: [crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        }; 4],
        visBounds: [[0.; 3]; 2],
        zFar: 0.,
        stereoFrame: crate::tr_types_h::STEREO_CENTER,
    },
    or: crate::tr_local_h::orientationr_t {
        origin: [0.; 3],
        axis: [[0.; 3]; 3],
        viewOrigin: [0.; 3],
        modelMatrix: [0.; 16],
    },
    pc: crate::tr_local_h::backEndCounters_t {
        c_surfaces: 0,
        c_shaders: 0,
        c_vertexes: 0,
        c_indexes: 0,
        c_totalIndexes: 0,
        c_overDraw: 0.,
        c_dlightVertexes: 0,
        c_dlightIndexes: 0,
        c_flareAdds: 0,
        c_flareTests: 0,
        c_flareRenders: 0,
        msec: 0,
    },
    isHyperspace: crate::src::qcommon::q_shared::qfalse,
    currentEntity: 0 as *const crate::tr_local_h::trRefEntity_t
        as *mut crate::tr_local_h::trRefEntity_t,
    skyRenderedThisView: crate::src::qcommon::q_shared::qfalse,
    projection2D: crate::src::qcommon::q_shared::qfalse,
    color2D: [0; 4],
    vertexes2D: crate::src::qcommon::q_shared::qfalse,
    entity2D: crate::tr_local_h::trRefEntity_t {
        e: crate::tr_types_h::refEntity_t {
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
        },
        axisLength: 0.,
        needDlights: crate::src::qcommon::q_shared::qfalse,
        lightingCalculated: crate::src::qcommon::q_shared::qfalse,
        lightDir: [0.; 3],
        ambientLight: [0.; 3],
        ambientLightInt: 0,
        directedLight: [0.; 3],
    },
};

static mut s_flipMatrix: [f32; 16] = [
    0 as i32 as f32,
    0 as i32 as f32,
    -(1 as i32) as f32,
    0 as i32 as f32,
    -(1 as i32) as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    1 as i32 as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    0 as i32 as f32,
    1 as i32 as f32,
];
/*
** GL_Bind
*/
#[no_mangle]

pub unsafe extern "C" fn GL_Bind(mut image: *mut crate::tr_common_h::image_t) {
    let mut texnum: i32 = 0;
    if image.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"GL_Bind: NULL image\n\x00" as *const u8 as *const libc::c_char,
        );
        texnum = (*crate::src::renderergl1::tr_main::tr.defaultImage).texnum as i32
    } else {
        texnum = (*image).texnum as i32
    }
    if (*crate::src::renderergl1::tr_init::r_nobind).integer != 0
        && !crate::src::renderergl1::tr_main::tr.dlightImage.is_null()
    {
        // performance evaluation option
        texnum = (*crate::src::renderergl1::tr_main::tr.dlightImage).texnum as i32
    }
    if crate::src::renderergl1::tr_init::glState.currenttextures
        [crate::src::renderergl1::tr_init::glState.currenttmu as usize]
        != texnum
    {
        if !image.is_null() {
            (*image).frameUsed = crate::src::renderergl1::tr_main::tr.frameCount
        }
        crate::src::renderergl1::tr_init::glState.currenttextures
            [crate::src::renderergl1::tr_init::glState.currenttmu as usize] = texnum;
        crate::src::sdl::sdl_glimp::qglBindTexture.expect("non-null function pointer")(
            0xde1 as i32 as crate::stdlib::GLenum,
            texnum as crate::stdlib::GLuint,
        );
    };
}
/*
** GL_SelectTexture
*/
#[no_mangle]

pub unsafe extern "C" fn GL_SelectTexture(mut unit: i32) {
    if crate::src::renderergl1::tr_init::glState.currenttmu == unit {
        return;
    }
    if unit == 0 as i32 {
        crate::src::sdl::sdl_glimp::qglActiveTextureARB.expect("non-null function pointer")(
            0x84c0 as i32 as crate::stdlib::GLenum,
        );
        crate::src::sdl::sdl_glimp::GLimp_LogComment(
            b"glActiveTextureARB( GL_TEXTURE0_ARB )\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        crate::src::sdl::sdl_glimp::qglClientActiveTextureARB.expect("non-null function pointer")(
            0x84c0 as i32 as crate::stdlib::GLenum,
        );
        crate::src::sdl::sdl_glimp::GLimp_LogComment(
            b"glClientActiveTextureARB( GL_TEXTURE0_ARB )\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else if unit == 1 as i32 {
        crate::src::sdl::sdl_glimp::qglActiveTextureARB.expect("non-null function pointer")(
            0x84c1 as i32 as crate::stdlib::GLenum,
        );
        crate::src::sdl::sdl_glimp::GLimp_LogComment(
            b"glActiveTextureARB( GL_TEXTURE1_ARB )\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        crate::src::sdl::sdl_glimp::qglClientActiveTextureARB.expect("non-null function pointer")(
            0x84c1 as i32 as crate::stdlib::GLenum,
        );
        crate::src::sdl::sdl_glimp::GLimp_LogComment(
            b"glClientActiveTextureARB( GL_TEXTURE1_ARB )\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"GL_SelectTexture: unit = %i\x00" as *const u8 as *const libc::c_char,
            unit,
        );
    }
    crate::src::renderergl1::tr_init::glState.currenttmu = unit;
}
/*
** GL_BindMultitexture
*/
#[no_mangle]

pub unsafe extern "C" fn GL_BindMultitexture(
    mut image0: *mut crate::tr_common_h::image_t,
    mut _env0: crate::stdlib::GLuint,
    mut image1: *mut crate::tr_common_h::image_t,
    mut _env1: crate::stdlib::GLuint,
) {
    let mut texnum0: i32 = 0;
    let mut texnum1: i32 = 0;
    texnum0 = (*image0).texnum as i32;
    texnum1 = (*image1).texnum as i32;
    if (*crate::src::renderergl1::tr_init::r_nobind).integer != 0
        && !crate::src::renderergl1::tr_main::tr.dlightImage.is_null()
    {
        // performance evaluation option
        texnum1 = (*crate::src::renderergl1::tr_main::tr.dlightImage).texnum as i32;
        texnum0 = texnum1
    }
    if crate::src::renderergl1::tr_init::glState.currenttextures[1 as i32 as usize]
        != texnum1
    {
        GL_SelectTexture(1 as i32);
        (*image1).frameUsed = crate::src::renderergl1::tr_main::tr.frameCount;
        crate::src::renderergl1::tr_init::glState.currenttextures[1 as i32 as usize] =
            texnum1;
        crate::src::sdl::sdl_glimp::qglBindTexture.expect("non-null function pointer")(
            0xde1 as i32 as crate::stdlib::GLenum,
            texnum1 as crate::stdlib::GLuint,
        );
    }
    if crate::src::renderergl1::tr_init::glState.currenttextures[0 as i32 as usize]
        != texnum0
    {
        GL_SelectTexture(0 as i32);
        (*image0).frameUsed = crate::src::renderergl1::tr_main::tr.frameCount;
        crate::src::renderergl1::tr_init::glState.currenttextures[0 as i32 as usize] =
            texnum0;
        crate::src::sdl::sdl_glimp::qglBindTexture.expect("non-null function pointer")(
            0xde1 as i32 as crate::stdlib::GLenum,
            texnum0 as crate::stdlib::GLuint,
        );
    };
}
/*
** GL_Cull
*/
#[no_mangle]

pub unsafe extern "C" fn GL_Cull(mut cullType: i32) {
    if crate::src::renderergl1::tr_init::glState.faceCulling == cullType {
        return;
    }
    crate::src::renderergl1::tr_init::glState.faceCulling = cullType;
    if cullType == crate::tr_local_h::CT_TWO_SIDED as i32 {
        crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
            0xb44 as i32 as crate::stdlib::GLenum,
        );
    } else {
        let mut cullFront: crate::src::qcommon::q_shared::qboolean =
            crate::src::qcommon::q_shared::qfalse;
        crate::src::sdl::sdl_glimp::qglEnable.expect("non-null function pointer")(
            0xb44 as i32 as crate::stdlib::GLenum,
        );
        cullFront = (cullType == crate::tr_local_h::CT_FRONT_SIDED as i32) as i32
            as crate::src::qcommon::q_shared::qboolean;
        if backEnd.viewParms.isMirror as u64 != 0 {
            cullFront =
                (cullFront as u64 == 0) as i32 as crate::src::qcommon::q_shared::qboolean
        }
        crate::src::sdl::sdl_glimp::qglCullFace.expect("non-null function pointer")(if cullFront
            as u32
            != 0
        {
            0x404 as i32
        } else {
            0x405 as i32
        }
            as crate::stdlib::GLenum);
    };
}
/*
** GL_TexEnv
*/
#[no_mangle]

pub unsafe extern "C" fn GL_TexEnv(mut env: i32) {
    if env
        == crate::src::renderergl1::tr_init::glState.texEnv
            [crate::src::renderergl1::tr_init::glState.currenttmu as usize]
    {
        return;
    }
    crate::src::renderergl1::tr_init::glState.texEnv
        [crate::src::renderergl1::tr_init::glState.currenttmu as usize] = env;
    match env {
        8448 => {
            crate::src::sdl::sdl_glimp::qglTexEnvf.expect("non-null function pointer")(
                0x2300 as i32 as crate::stdlib::GLenum,
                0x2200 as i32 as crate::stdlib::GLenum,
                0x2100 as i32 as crate::stdlib::GLfloat,
            );
        }
        7681 => {
            crate::src::sdl::sdl_glimp::qglTexEnvf.expect("non-null function pointer")(
                0x2300 as i32 as crate::stdlib::GLenum,
                0x2200 as i32 as crate::stdlib::GLenum,
                0x1e01 as i32 as crate::stdlib::GLfloat,
            );
        }
        8449 => {
            crate::src::sdl::sdl_glimp::qglTexEnvf.expect("non-null function pointer")(
                0x2300 as i32 as crate::stdlib::GLenum,
                0x2200 as i32 as crate::stdlib::GLenum,
                0x2101 as i32 as crate::stdlib::GLfloat,
            );
        }
        260 => {
            crate::src::sdl::sdl_glimp::qglTexEnvf.expect("non-null function pointer")(
                0x2300 as i32 as crate::stdlib::GLenum,
                0x2200 as i32 as crate::stdlib::GLenum,
                0x104 as i32 as crate::stdlib::GLfloat,
            );
        }
        _ => {
            crate::src::renderergl1::tr_main::ri
                .Error
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"GL_TexEnv: invalid env \'%d\' passed\x00" as *const u8 as *const libc::c_char,
                env,
            );
        }
    };
}
/*
** GL_State
**
** This routine is responsible for setting the most commonly changed state
** in Q3.
*/
#[no_mangle]

pub unsafe extern "C" fn GL_State(mut stateBits: libc::c_ulong) {
    let mut diff: libc::c_ulong = stateBits ^ crate::src::renderergl1::tr_init::glState.glStateBits;
    if diff == 0 {
        return;
    }
    //
    // check depthFunc bits
    //
    if diff & 0x20000 as i32 as libc::c_ulong != 0 {
        if stateBits & 0x20000 as i32 as libc::c_ulong != 0 {
            crate::src::sdl::sdl_glimp::qglDepthFunc.expect("non-null function pointer")(
                0x202 as i32 as crate::stdlib::GLenum,
            );
        } else {
            crate::src::sdl::sdl_glimp::qglDepthFunc.expect("non-null function pointer")(
                0x203 as i32 as crate::stdlib::GLenum,
            );
        }
    }
    //
    // check blend bits
    //
    if diff & (0xf as i32 | 0xf0 as i32) as libc::c_ulong != 0 {
        let mut srcFactor: crate::stdlib::GLenum = 1 as i32 as crate::stdlib::GLenum;
        let mut dstFactor: crate::stdlib::GLenum = 1 as i32 as crate::stdlib::GLenum;
        if stateBits & (0xf as i32 | 0xf0 as i32) as libc::c_ulong != 0 {
            match stateBits & 0xf as i32 as libc::c_ulong {
                1 => srcFactor = 0 as i32 as crate::stdlib::GLenum,
                2 => srcFactor = 1 as i32 as crate::stdlib::GLenum,
                3 => srcFactor = 0x306 as i32 as crate::stdlib::GLenum,
                4 => srcFactor = 0x307 as i32 as crate::stdlib::GLenum,
                5 => srcFactor = 0x302 as i32 as crate::stdlib::GLenum,
                6 => srcFactor = 0x303 as i32 as crate::stdlib::GLenum,
                7 => srcFactor = 0x304 as i32 as crate::stdlib::GLenum,
                8 => srcFactor = 0x305 as i32 as crate::stdlib::GLenum,
                9 => srcFactor = 0x308 as i32 as crate::stdlib::GLenum,
                _ => {
                    crate::src::renderergl1::tr_main::ri
                        .Error
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::ERR_DROP as i32,
                        b"GL_State: invalid src blend state bits\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            match stateBits & 0xf0 as i32 as libc::c_ulong {
                16 => dstFactor = 0 as i32 as crate::stdlib::GLenum,
                32 => dstFactor = 1 as i32 as crate::stdlib::GLenum,
                48 => dstFactor = 0x300 as i32 as crate::stdlib::GLenum,
                64 => dstFactor = 0x301 as i32 as crate::stdlib::GLenum,
                80 => dstFactor = 0x302 as i32 as crate::stdlib::GLenum,
                96 => dstFactor = 0x303 as i32 as crate::stdlib::GLenum,
                112 => dstFactor = 0x304 as i32 as crate::stdlib::GLenum,
                128 => dstFactor = 0x305 as i32 as crate::stdlib::GLenum,
                _ => {
                    crate::src::renderergl1::tr_main::ri
                        .Error
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::ERR_DROP as i32,
                        b"GL_State: invalid dst blend state bits\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            crate::src::sdl::sdl_glimp::qglEnable.expect("non-null function pointer")(
                0xbe2 as i32 as crate::stdlib::GLenum,
            );
            crate::src::sdl::sdl_glimp::qglBlendFunc.expect("non-null function pointer")(
                srcFactor, dstFactor,
            );
        } else {
            crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
                0xbe2 as i32 as crate::stdlib::GLenum,
            );
        }
    }
    //
    // check depthmask
    //
    if diff & 0x100 as i32 as libc::c_ulong != 0 {
        if stateBits & 0x100 as i32 as libc::c_ulong != 0 {
            crate::src::sdl::sdl_glimp::qglDepthMask.expect("non-null function pointer")(
                1 as i32 as crate::stdlib::GLboolean,
            );
        } else {
            crate::src::sdl::sdl_glimp::qglDepthMask.expect("non-null function pointer")(
                0 as i32 as crate::stdlib::GLboolean,
            );
        }
    }
    //
    // fill/line mode
    //
    if diff & 0x1000 as i32 as libc::c_ulong != 0 {
        if stateBits & 0x1000 as i32 as libc::c_ulong != 0 {
            crate::src::sdl::sdl_glimp::qglPolygonMode.expect("non-null function pointer")(
                0x408 as i32 as crate::stdlib::GLenum,
                0x1b01 as i32 as crate::stdlib::GLenum,
            );
        } else {
            crate::src::sdl::sdl_glimp::qglPolygonMode.expect("non-null function pointer")(
                0x408 as i32 as crate::stdlib::GLenum,
                0x1b02 as i32 as crate::stdlib::GLenum,
            );
        }
    }
    //
    // depthtest
    //
    if diff & 0x10000 as i32 as libc::c_ulong != 0 {
        if stateBits & 0x10000 as i32 as libc::c_ulong != 0 {
            crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
                0xb71 as i32 as crate::stdlib::GLenum,
            );
        } else {
            crate::src::sdl::sdl_glimp::qglEnable.expect("non-null function pointer")(
                0xb71 as i32 as crate::stdlib::GLenum,
            );
        }
    }
    //
    // alpha test
    //
    if diff & 0x70000000 as i32 as libc::c_ulong != 0 {
        match stateBits & 0x70000000 as i32 as libc::c_ulong {
            0 => {
                crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
                    0xbc0 as i32 as crate::stdlib::GLenum,
                );
            }
            268435456 => {
                crate::src::sdl::sdl_glimp::qglEnable.expect("non-null function pointer")(
                    0xbc0 as i32 as crate::stdlib::GLenum,
                );
                crate::src::sdl::sdl_glimp::qglAlphaFunc.expect("non-null function pointer")(
                    0x204 as i32 as crate::stdlib::GLenum,
                    0.0f32,
                );
            }
            536870912 => {
                crate::src::sdl::sdl_glimp::qglEnable.expect("non-null function pointer")(
                    0xbc0 as i32 as crate::stdlib::GLenum,
                );
                crate::src::sdl::sdl_glimp::qglAlphaFunc.expect("non-null function pointer")(
                    0x201 as i32 as crate::stdlib::GLenum,
                    0.5f32,
                );
            }
            1073741824 => {
                crate::src::sdl::sdl_glimp::qglEnable.expect("non-null function pointer")(
                    0xbc0 as i32 as crate::stdlib::GLenum,
                );
                crate::src::sdl::sdl_glimp::qglAlphaFunc.expect("non-null function pointer")(
                    0x206 as i32 as crate::stdlib::GLenum,
                    0.5f32,
                );
            }
            _ => {}
        }
    }
    crate::src::renderergl1::tr_init::glState.glStateBits = stateBits;
}
/*
================
RB_Hyperspace

A player has predicted a teleport, but hasn't arrived yet
================
*/

unsafe extern "C" fn RB_Hyperspace() {
    let mut c: f32 = 0.;
    // ensure hyperspace flag cleared
    c = (backEnd.refdef.time & 255 as i32) as f32 / 255.0f32;
    crate::src::sdl::sdl_glimp::qglClearColor.expect("non-null function pointer")(
        c,
        c,
        c,
        1 as i32 as crate::stdlib::GLclampf,
    );
    crate::src::sdl::sdl_glimp::qglClear.expect("non-null function pointer")(
        0x4000 as i32 as crate::stdlib::GLbitfield,
    );
    backEnd.isHyperspace = crate::src::qcommon::q_shared::qtrue;
}

unsafe extern "C" fn SetViewportAndScissor() {
    crate::src::sdl::sdl_glimp::qglMatrixMode.expect("non-null function pointer")(
        0x1701 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglLoadMatrixf.expect("non-null function pointer")(
        backEnd.viewParms.projectionMatrix.as_mut_ptr(),
    );
    crate::src::sdl::sdl_glimp::qglMatrixMode.expect("non-null function pointer")(
        0x1700 as i32 as crate::stdlib::GLenum,
    );
    // set the window clipping
    crate::src::sdl::sdl_glimp::qglViewport.expect("non-null function pointer")(
        backEnd.viewParms.viewportX,
        backEnd.viewParms.viewportY,
        backEnd.viewParms.viewportWidth,
        backEnd.viewParms.viewportHeight,
    );
    crate::src::sdl::sdl_glimp::qglScissor.expect("non-null function pointer")(
        backEnd.viewParms.viewportX,
        backEnd.viewParms.viewportY,
        backEnd.viewParms.viewportWidth,
        backEnd.viewParms.viewportHeight,
    );
}
/*
=================
RB_BeginDrawingView

Any mirrored or portaled views have already been drawn, so prepare
to actually render the visible surfaces for this view
=================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_BeginDrawingView() {
    let mut clearBits: i32 = 0 as i32;
    // sync with gl if needed
    if (*crate::src::renderergl1::tr_init::r_finish).integer == 1 as i32
        && crate::src::renderergl1::tr_init::glState.finishCalled as u64 == 0
    {
        crate::src::sdl::sdl_glimp::qglFinish.expect("non-null function pointer")();
        crate::src::renderergl1::tr_init::glState.finishCalled =
            crate::src::qcommon::q_shared::qtrue
    }
    if (*crate::src::renderergl1::tr_init::r_finish).integer == 0 as i32 {
        crate::src::renderergl1::tr_init::glState.finishCalled =
            crate::src::qcommon::q_shared::qtrue
    }
    // we will need to change the projection matrix before drawing
    // 2D images again
    backEnd.projection2D = crate::src::qcommon::q_shared::qfalse;
    //
    // set the modelview matrix for the viewer
    //
    SetViewportAndScissor();
    // ensures that depth writes are enabled for the depth clear
    GL_State(0x100 as i32 as libc::c_ulong);
    // clear relevant buffers
    clearBits = 0x100 as i32; // FIXME: only if sky shaders have been used
    if (*crate::src::renderergl1::tr_init::r_measureOverdraw).integer != 0
        || (*crate::src::renderergl1::tr_init::r_shadows).integer == 2 as i32
    {
        clearBits |= 0x400 as i32
    }
    if (*crate::src::renderergl1::tr_init::r_fastsky).integer != 0
        && backEnd.refdef.rdflags & 0x1 as i32 == 0
    {
        clearBits |= 0x4000 as i32;
        crate::src::sdl::sdl_glimp::qglClearColor.expect("non-null function pointer")(
            0.0f32, 0.0f32, 0.0f32, 1.0f32,
        );
        // FIXME: get color of sky
    } // force face culling to set next time
    crate::src::sdl::sdl_glimp::qglClear.expect("non-null function pointer")(
        clearBits as crate::stdlib::GLbitfield,
    );
    if backEnd.refdef.rdflags & 0x4 as i32 != 0 {
        RB_Hyperspace();
        return;
    } else {
        backEnd.isHyperspace = crate::src::qcommon::q_shared::qfalse
    }
    crate::src::renderergl1::tr_init::glState.faceCulling = -(1 as i32);
    // we will only draw a sun if there was sky rendered in this view
    backEnd.skyRenderedThisView = crate::src::qcommon::q_shared::qfalse;
    // clip to the plane of the portal
    if backEnd.viewParms.isPortal as u64 != 0 {
        let mut plane: [f32; 4] = [0.; 4];
        let mut plane2: [crate::stdlib::GLdouble; 4] = [0.; 4];
        plane[0 as i32 as usize] =
            backEnd.viewParms.portalPlane.normal[0 as i32 as usize];
        plane[1 as i32 as usize] =
            backEnd.viewParms.portalPlane.normal[1 as i32 as usize];
        plane[2 as i32 as usize] =
            backEnd.viewParms.portalPlane.normal[2 as i32 as usize];
        plane[3 as i32 as usize] = backEnd.viewParms.portalPlane.dist;
        plane2[0 as i32 as usize] =
            (backEnd.viewParms.or.axis[0 as i32 as usize][0 as i32 as usize]
                * plane[0 as i32 as usize]
                + backEnd.viewParms.or.axis[0 as i32 as usize][1 as i32 as usize]
                    * plane[1 as i32 as usize]
                + backEnd.viewParms.or.axis[0 as i32 as usize][2 as i32 as usize]
                    * plane[2 as i32 as usize]) as crate::stdlib::GLdouble;
        plane2[1 as i32 as usize] =
            (backEnd.viewParms.or.axis[1 as i32 as usize][0 as i32 as usize]
                * plane[0 as i32 as usize]
                + backEnd.viewParms.or.axis[1 as i32 as usize][1 as i32 as usize]
                    * plane[1 as i32 as usize]
                + backEnd.viewParms.or.axis[1 as i32 as usize][2 as i32 as usize]
                    * plane[2 as i32 as usize]) as crate::stdlib::GLdouble;
        plane2[2 as i32 as usize] =
            (backEnd.viewParms.or.axis[2 as i32 as usize][0 as i32 as usize]
                * plane[0 as i32 as usize]
                + backEnd.viewParms.or.axis[2 as i32 as usize][1 as i32 as usize]
                    * plane[1 as i32 as usize]
                + backEnd.viewParms.or.axis[2 as i32 as usize][2 as i32 as usize]
                    * plane[2 as i32 as usize]) as crate::stdlib::GLdouble;
        plane2[3 as i32 as usize] = (plane[0 as i32 as usize]
            * backEnd.viewParms.or.origin[0 as i32 as usize]
            + plane[1 as i32 as usize]
                * backEnd.viewParms.or.origin[1 as i32 as usize]
            + plane[2 as i32 as usize]
                * backEnd.viewParms.or.origin[2 as i32 as usize]
            - plane[3 as i32 as usize])
            as crate::stdlib::GLdouble;
        crate::src::sdl::sdl_glimp::qglLoadMatrixf.expect("non-null function pointer")(
            s_flipMatrix.as_mut_ptr(),
        );
        crate::src::sdl::sdl_glimp::qglClipPlane.expect("non-null function pointer")(
            0x3000 as i32 as crate::stdlib::GLenum,
            plane2.as_mut_ptr(),
        );
        crate::src::sdl::sdl_glimp::qglEnable.expect("non-null function pointer")(
            0x3000 as i32 as crate::stdlib::GLenum,
        );
    } else {
        crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
            0x3000 as i32 as crate::stdlib::GLenum,
        );
    };
}
/*
==================
RB_RenderDrawSurfList
==================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_RenderDrawSurfList(
    mut drawSurfs: *mut crate::tr_local_h::drawSurf_t,
    mut numDrawSurfs: i32,
) {
    let mut shader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    let mut oldShader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    let mut fogNum: i32 = 0;
    let mut oldFogNum: i32 = 0;
    let mut entityNum: i32 = 0;
    let mut oldEntityNum: i32 = 0;
    let mut dlighted: i32 = 0;
    let mut oldDlighted: i32 = 0;
    let mut depthRange: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut oldDepthRange: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut isCrosshair: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut wasCrosshair: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut i: i32 = 0;
    let mut drawSurf: *mut crate::tr_local_h::drawSurf_t = 0 as *mut crate::tr_local_h::drawSurf_t;
    let mut oldSort: i32 = 0;
    let mut originalTime: f64 = 0.;
    // save original time for entity shader offsets
    originalTime = backEnd.refdef.floatTime;
    // clear the z buffer, set the modelview, etc
    RB_BeginDrawingView();
    // draw everything
    oldEntityNum = -(1 as i32);
    backEnd.currentEntity = &mut crate::src::renderergl1::tr_main::tr.worldEntity;
    oldShader = 0 as *mut crate::tr_local_h::shader_t;
    oldFogNum = -(1 as i32);
    oldDepthRange = crate::src::qcommon::q_shared::qfalse;
    wasCrosshair = crate::src::qcommon::q_shared::qfalse;
    oldDlighted = crate::src::qcommon::q_shared::qfalse as i32;
    oldSort = -(1 as i32);
    depthRange = crate::src::qcommon::q_shared::qfalse;
    backEnd.pc.c_surfaces += numDrawSurfs;
    i = 0 as i32;
    drawSurf = drawSurfs;
    while i < numDrawSurfs {
        if (*drawSurf).sort == oldSort as u32 {
            // fast path, same as previous sort
            crate::src::renderergl1::tr_surface::rb_surfaceTable[*(*drawSurf).surface as usize]
                .expect("non-null function pointer")(
                (*drawSurf).surface as *mut libc::c_void
            );
        } else {
            oldSort = (*drawSurf).sort as i32;
            crate::src::renderergl1::tr_main::R_DecomposeSort(
                (*drawSurf).sort,
                &mut entityNum,
                &mut shader as *mut _ as *mut *mut crate::tr_local_h::shader_s,
                &mut fogNum,
                &mut dlighted,
            );
            //
            // change the tess parameters if needed
            // a "entityMergable" shader is a shader that can have surfaces from separate
            // entities merged into a single batch, like smoke and blood puff sprites
            if !shader.is_null()
                && (shader != oldShader
                    || fogNum != oldFogNum
                    || dlighted != oldDlighted
                    || entityNum != oldEntityNum && (*shader).entityMergable as u64 == 0)
            {
                if !oldShader.is_null() {
                    crate::src::renderergl1::tr_shade::RB_EndSurface();
                }
                crate::src::renderergl1::tr_shade::RB_BeginSurface(
                    shader as *mut crate::tr_local_h::shader_s,
                    fogNum,
                );
                oldShader = shader;
                oldFogNum = fogNum;
                oldDlighted = dlighted
            }
            //
            // change the modelview matrix if needed
            //
            if entityNum != oldEntityNum {
                isCrosshair = crate::src::qcommon::q_shared::qfalse;
                depthRange = isCrosshair;
                if entityNum != ((1 as i32) << 10 as i32) - 1 as i32 {
                    backEnd.currentEntity = &mut *backEnd.refdef.entities.offset(entityNum as isize)
                        as *mut crate::tr_local_h::trRefEntity_t;
                    // FIXME: e.shaderTime must be passed as int to avoid fp-precision loss issues
                    backEnd.refdef.floatTime =
                        originalTime - (*backEnd.currentEntity).e.shaderTime as f64;
                    // we have to reset the shaderTime as well otherwise image animations start
                    // from the wrong frame
                    crate::src::renderergl1::tr_shade::tess.shaderTime = backEnd.refdef.floatTime
                        - (*crate::src::renderergl1::tr_shade::tess.shader).timeOffset;
                    // set up the transformation matrix
                    crate::src::renderergl1::tr_main::R_RotateForEntity(
                        backEnd.currentEntity as *const crate::tr_local_h::trRefEntity_t,
                        &mut backEnd.viewParms as *mut _ as *const crate::tr_local_h::viewParms_t,
                        &mut backEnd.or as *mut _ as *mut crate::tr_local_h::orientationr_t,
                    );
                    // set up the dynamic lighting if needed
                    if (*backEnd.currentEntity).needDlights as u64 != 0 {
                        crate::src::renderergl1::tr_light::R_TransformDlights(
                            backEnd.refdef.num_dlights,
                            backEnd.refdef.dlights as *mut crate::tr_local_h::dlight_s,
                            &mut backEnd.or as *mut _ as *mut crate::tr_local_h::orientationr_t,
                        );
                    }
                    if (*backEnd.currentEntity).e.renderfx & 0x8 as i32 != 0 {
                        // hack the depth range to prevent view model from poking into walls
                        depthRange = crate::src::qcommon::q_shared::qtrue;
                        if (*backEnd.currentEntity).e.renderfx & 0x10 as i32 != 0 {
                            isCrosshair = crate::src::qcommon::q_shared::qtrue
                        }
                    }
                } else {
                    backEnd.currentEntity = &mut crate::src::renderergl1::tr_main::tr.worldEntity;
                    backEnd.refdef.floatTime = originalTime;
                    backEnd.or = backEnd.viewParms.world;
                    // we have to reset the shaderTime as well otherwise image animations on
                    // the world (like water) continue with the wrong frame
                    crate::src::renderergl1::tr_shade::tess.shaderTime = backEnd.refdef.floatTime
                        - (*crate::src::renderergl1::tr_shade::tess.shader).timeOffset;
                    crate::src::renderergl1::tr_light::R_TransformDlights(
                        backEnd.refdef.num_dlights,
                        backEnd.refdef.dlights as *mut crate::tr_local_h::dlight_s,
                        &mut backEnd.or as *mut _ as *mut crate::tr_local_h::orientationr_t,
                    );
                }
                crate::src::sdl::sdl_glimp::qglLoadMatrixf.expect("non-null function pointer")(
                    backEnd.or.modelMatrix.as_mut_ptr(),
                );
                //
                // change depthrange. Also change projection matrix so first person weapon does not look like coming
                // out of the screen.
                //
                if oldDepthRange as u32 != depthRange as u32
                    || wasCrosshair as u32 != isCrosshair as u32
                {
                    if depthRange as u64 != 0 {
                        if backEnd.viewParms.stereoFrame as u32
                            != crate::tr_types_h::STEREO_CENTER as i32 as u32
                        {
                            if isCrosshair as u64 != 0 {
                                if oldDepthRange as u64 != 0 {
                                    // was not a crosshair but now is, change back proj matrix
                                    crate::src::sdl::sdl_glimp::qglMatrixMode
                                        .expect("non-null function pointer")(
                                        0x1701 as i32 as crate::stdlib::GLenum,
                                    );
                                    crate::src::sdl::sdl_glimp::qglLoadMatrixf
                                        .expect("non-null function pointer")(
                                        backEnd.viewParms.projectionMatrix.as_mut_ptr(),
                                    );
                                    crate::src::sdl::sdl_glimp::qglMatrixMode
                                        .expect("non-null function pointer")(
                                        0x1700 as i32 as crate::stdlib::GLenum,
                                    );
                                }
                            } else {
                                let mut temp: crate::tr_local_h::viewParms_t = backEnd.viewParms;
                                crate::src::renderergl1::tr_main::R_SetupProjection(
                                    &mut temp as *mut _ as *mut crate::tr_local_h::viewParms_t,
                                    (*crate::src::renderergl1::tr_init::r_znear).value,
                                    crate::src::qcommon::q_shared::qfalse,
                                );
                                crate::src::sdl::sdl_glimp::qglMatrixMode
                                    .expect("non-null function pointer")(
                                    0x1701 as i32 as crate::stdlib::GLenum,
                                );
                                crate::src::sdl::sdl_glimp::qglLoadMatrixf
                                    .expect("non-null function pointer")(
                                    temp.projectionMatrix.as_mut_ptr(),
                                );
                                crate::src::sdl::sdl_glimp::qglMatrixMode
                                    .expect("non-null function pointer")(
                                    0x1700 as i32 as crate::stdlib::GLenum,
                                );
                            }
                        }
                        if oldDepthRange as u64 == 0 {
                            crate::src::sdl::sdl_glimp::qglDepthRange
                                .expect("non-null function pointer")(
                                0 as i32 as crate::stdlib::GLclampd,
                                0.3f64,
                            );
                        }
                    } else {
                        if wasCrosshair as u64 == 0
                            && backEnd.viewParms.stereoFrame as u32
                                != crate::tr_types_h::STEREO_CENTER as i32 as u32
                        {
                            crate::src::sdl::sdl_glimp::qglMatrixMode
                                .expect("non-null function pointer")(
                                0x1701 as i32 as crate::stdlib::GLenum,
                            );
                            crate::src::sdl::sdl_glimp::qglLoadMatrixf
                                .expect("non-null function pointer")(
                                backEnd.viewParms.projectionMatrix.as_mut_ptr(),
                            );
                            crate::src::sdl::sdl_glimp::qglMatrixMode
                                .expect("non-null function pointer")(
                                0x1700 as i32 as crate::stdlib::GLenum,
                            );
                        }
                        crate::src::sdl::sdl_glimp::qglDepthRange
                            .expect("non-null function pointer")(
                            0 as i32 as crate::stdlib::GLclampd,
                            1 as i32 as crate::stdlib::GLclampd,
                        );
                    }
                    oldDepthRange = depthRange;
                    wasCrosshair = isCrosshair
                }
                oldEntityNum = entityNum
            }
            // add the triangles for this surface
            crate::src::renderergl1::tr_surface::rb_surfaceTable[*(*drawSurf).surface as usize]
                .expect("non-null function pointer")(
                (*drawSurf).surface as *mut libc::c_void
            );
        }
        i += 1;
        drawSurf = drawSurf.offset(1)
    }
    backEnd.refdef.floatTime = originalTime;
    // draw the contents of the last shader batch
    if !oldShader.is_null() {
        crate::src::renderergl1::tr_shade::RB_EndSurface();
    }
    // go back to the world modelview matrix
    crate::src::sdl::sdl_glimp::qglLoadMatrixf.expect("non-null function pointer")(
        backEnd.viewParms.world.modelMatrix.as_mut_ptr(),
    );
    if depthRange as u64 != 0 {
        crate::src::sdl::sdl_glimp::qglDepthRange.expect("non-null function pointer")(
            0 as i32 as crate::stdlib::GLclampd,
            1 as i32 as crate::stdlib::GLclampd,
        );
    }
    if (*crate::src::renderergl1::tr_init::r_drawSun).integer != 0 {
        crate::src::renderergl1::tr_sky::RB_DrawSun(
            0.1f64 as f32,
            crate::src::renderergl1::tr_main::tr.sunShader as *mut crate::tr_local_h::shader_s,
        );
    }
    // darken down any stencil shadows
    crate::src::renderergl1::tr_shadows::RB_ShadowFinish();
    // add light flares on lights that aren't obscured
    crate::src::renderergl1::tr_flares::RB_RenderFlares();
}
/*
============================================================================

RENDER BACK END FUNCTIONS

============================================================================
*/
/*
================
RB_SetGL2D

================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_SetGL2D() {
    backEnd.projection2D = crate::src::qcommon::q_shared::qtrue;
    // set 2D virtual screen size
    crate::src::sdl::sdl_glimp::qglViewport.expect("non-null function pointer")(
        0 as i32,
        0 as i32,
        crate::src::renderergl1::tr_init::glConfig.vidWidth,
        crate::src::renderergl1::tr_init::glConfig.vidHeight,
    );
    crate::src::sdl::sdl_glimp::qglScissor.expect("non-null function pointer")(
        0 as i32,
        0 as i32,
        crate::src::renderergl1::tr_init::glConfig.vidWidth,
        crate::src::renderergl1::tr_init::glConfig.vidHeight,
    );
    crate::src::sdl::sdl_glimp::qglMatrixMode.expect("non-null function pointer")(
        0x1701 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglLoadIdentity.expect("non-null function pointer")();
    crate::src::sdl::sdl_glimp::qglOrtho.expect("non-null function pointer")(
        0 as i32 as crate::stdlib::GLdouble,
        crate::src::renderergl1::tr_init::glConfig.vidWidth as crate::stdlib::GLdouble,
        crate::src::renderergl1::tr_init::glConfig.vidHeight as crate::stdlib::GLdouble,
        0 as i32 as crate::stdlib::GLdouble,
        0 as i32 as crate::stdlib::GLdouble,
        1 as i32 as crate::stdlib::GLdouble,
    );
    crate::src::sdl::sdl_glimp::qglMatrixMode.expect("non-null function pointer")(
        0x1700 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglLoadIdentity.expect("non-null function pointer")();
    GL_State((0x10000 as i32 | 0x5 as i32 | 0x60 as i32) as libc::c_ulong);
    GL_Cull(crate::tr_local_h::CT_TWO_SIDED as i32);
    crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
        0x3000 as i32 as crate::stdlib::GLenum,
    );
    // set time for 2D shaders
    backEnd.refdef.time = crate::src::renderergl1::tr_main::ri
        .Milliseconds
        .expect("non-null function pointer")();
    backEnd.refdef.floatTime = backEnd.refdef.time as f64 * 0.001f64;
}
/*
=============
RE_StretchRaw

FIXME: not exactly backend
Stretches a raw 32 bit power of 2 bitmap image over the given screen rectangle.
Used for cinematics.
=============
*/
#[no_mangle]

pub unsafe extern "C" fn RE_StretchRaw(
    mut x: i32,
    mut y: i32,
    mut w: i32,
    mut h: i32,
    mut cols: i32,
    mut rows: i32,
    mut data: *const crate::src::qcommon::q_shared::byte,
    mut client: i32,
    mut dirty: crate::src::qcommon::q_shared::qboolean,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    if crate::src::renderergl1::tr_main::tr.registered as u64 == 0 {
        return;
    }
    crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands();
    if crate::src::renderergl1::tr_shade::tess.numIndexes != 0 {
        crate::src::renderergl1::tr_shade::RB_EndSurface();
    }
    // we definitely want to sync every frame for the cinematics
    crate::src::sdl::sdl_glimp::qglFinish.expect("non-null function pointer")();
    start = 0 as i32;
    if (*crate::src::renderergl1::tr_init::r_speeds).integer != 0 {
        start = crate::src::renderergl1::tr_main::ri
            .Milliseconds
            .expect("non-null function pointer")()
    }
    // make sure rows and cols are powers of 2
    i = 0 as i32;
    while (1 as i32) << i < cols {
        i += 1
    }
    j = 0 as i32;
    while (1 as i32) << j < rows {
        j += 1
    }
    if (1 as i32) << i != cols || (1 as i32) << j != rows {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Draw_StretchRaw: size not a power of 2: %i by %i\x00" as *const u8
                as *const libc::c_char,
            cols,
            rows,
        );
    }
    RE_UploadCinematic(w, h, cols, rows, data, client, dirty);
    GL_Bind(crate::src::renderergl1::tr_main::tr.scratchImage[client as usize]);
    if (*crate::src::renderergl1::tr_init::r_speeds).integer != 0 {
        end = crate::src::renderergl1::tr_main::ri
            .Milliseconds
            .expect("non-null function pointer")();
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"qglTexSubImage2D %i, %i: %i msec\n\x00" as *const u8 as *const libc::c_char,
            cols,
            rows,
            end - start,
        );
    }
    RB_SetGL2D();
    crate::src::sdl::sdl_glimp::qglColor3f.expect("non-null function pointer")(
        crate::src::renderergl1::tr_main::tr.identityLight,
        crate::src::renderergl1::tr_main::tr.identityLight,
        crate::src::renderergl1::tr_main::tr.identityLight,
    );
    crate::src::sdl::sdl_glimp::qglBegin.expect("non-null function pointer")(
        0x7 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglTexCoord2f.expect("non-null function pointer")(
        0.5f32 / cols as f32,
        0.5f32 / rows as f32,
    );
    crate::src::sdl::sdl_glimp::qglVertex2f.expect("non-null function pointer")(
        x as crate::stdlib::GLfloat,
        y as crate::stdlib::GLfloat,
    );
    crate::src::sdl::sdl_glimp::qglTexCoord2f.expect("non-null function pointer")(
        (cols as f32 - 0.5f32) / cols as f32,
        0.5f32 / rows as f32,
    );
    crate::src::sdl::sdl_glimp::qglVertex2f.expect("non-null function pointer")(
        (x + w) as crate::stdlib::GLfloat,
        y as crate::stdlib::GLfloat,
    );
    crate::src::sdl::sdl_glimp::qglTexCoord2f.expect("non-null function pointer")(
        (cols as f32 - 0.5f32) / cols as f32,
        (rows as f32 - 0.5f32) / rows as f32,
    );
    crate::src::sdl::sdl_glimp::qglVertex2f.expect("non-null function pointer")(
        (x + w) as crate::stdlib::GLfloat,
        (y + h) as crate::stdlib::GLfloat,
    );
    crate::src::sdl::sdl_glimp::qglTexCoord2f.expect("non-null function pointer")(
        0.5f32 / cols as f32,
        (rows as f32 - 0.5f32) / rows as f32,
    );
    crate::src::sdl::sdl_glimp::qglVertex2f.expect("non-null function pointer")(
        x as crate::stdlib::GLfloat,
        (y + h) as crate::stdlib::GLfloat,
    );
    crate::src::sdl::sdl_glimp::qglEnd.expect("non-null function pointer")();
}
#[no_mangle]

pub unsafe extern "C" fn RE_UploadCinematic(
    mut _w: i32,
    mut _h: i32,
    mut cols: i32,
    mut rows: i32,
    mut data: *const crate::src::qcommon::q_shared::byte,
    mut client: i32,
    mut dirty: crate::src::qcommon::q_shared::qboolean,
) {
    GL_Bind(crate::src::renderergl1::tr_main::tr.scratchImage[client as usize]);
    // if the scratchImage isn't in the format we want, specify it as a new texture
    if cols != (*crate::src::renderergl1::tr_main::tr.scratchImage[client as usize]).width
        || rows != (*crate::src::renderergl1::tr_main::tr.scratchImage[client as usize]).height
    {
        (*crate::src::renderergl1::tr_main::tr.scratchImage[client as usize]).uploadWidth = cols;
        (*crate::src::renderergl1::tr_main::tr.scratchImage[client as usize]).width =
            (*crate::src::renderergl1::tr_main::tr.scratchImage[client as usize]).uploadWidth;
        (*crate::src::renderergl1::tr_main::tr.scratchImage[client as usize]).uploadHeight = rows;
        (*crate::src::renderergl1::tr_main::tr.scratchImage[client as usize]).height =
            (*crate::src::renderergl1::tr_main::tr.scratchImage[client as usize]).uploadHeight;
        crate::src::sdl::sdl_glimp::qglTexImage2D.expect("non-null function pointer")(
            0xde1 as i32 as crate::stdlib::GLenum,
            0 as i32,
            0x8051 as i32,
            cols,
            rows,
            0 as i32,
            0x1908 as i32 as crate::stdlib::GLenum,
            0x1401 as i32 as crate::stdlib::GLenum,
            data as *const libc::c_void,
        );
        crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
            0xde1 as i32 as crate::stdlib::GLenum,
            0x2801 as i32 as crate::stdlib::GLenum,
            0x2601 as i32 as crate::stdlib::GLfloat,
        );
        crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
            0xde1 as i32 as crate::stdlib::GLenum,
            0x2800 as i32 as crate::stdlib::GLenum,
            0x2601 as i32 as crate::stdlib::GLfloat,
        );
        crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
            0xde1 as i32 as crate::stdlib::GLenum,
            0x2802 as i32 as crate::stdlib::GLenum,
            0x812f as i32 as crate::stdlib::GLfloat,
        );
        crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
            0xde1 as i32 as crate::stdlib::GLenum,
            0x2803 as i32 as crate::stdlib::GLenum,
            0x812f as i32 as crate::stdlib::GLfloat,
        );
    } else if dirty as u64 != 0 {
        // otherwise, just subimage upload it so that drivers can tell we are going to be changing
        // it and don't try and do a texture compression
        crate::src::sdl::sdl_glimp::qglTexSubImage2D.expect("non-null function pointer")(
            0xde1 as i32 as crate::stdlib::GLenum,
            0 as i32,
            0 as i32,
            0 as i32,
            cols,
            rows,
            0x1908 as i32 as crate::stdlib::GLenum,
            0x1401 as i32 as crate::stdlib::GLenum,
            data as *const libc::c_void,
        );
    };
}
/*
=============
RB_SetColor

=============
*/
#[no_mangle]

pub unsafe extern "C" fn RB_SetColor(mut data: *const libc::c_void) -> *const libc::c_void {
    let mut cmd: *const crate::tr_local_h::setColorCommand_t =
        0 as *const crate::tr_local_h::setColorCommand_t;
    cmd = data as *const crate::tr_local_h::setColorCommand_t;
    backEnd.color2D[0 as i32 as usize] = ((*cmd).color[0 as i32 as usize]
        * 255 as i32 as f32)
        as crate::src::qcommon::q_shared::byte;
    backEnd.color2D[1 as i32 as usize] = ((*cmd).color[1 as i32 as usize]
        * 255 as i32 as f32)
        as crate::src::qcommon::q_shared::byte;
    backEnd.color2D[2 as i32 as usize] = ((*cmd).color[2 as i32 as usize]
        * 255 as i32 as f32)
        as crate::src::qcommon::q_shared::byte;
    backEnd.color2D[3 as i32 as usize] = ((*cmd).color[3 as i32 as usize]
        * 255 as i32 as f32)
        as crate::src::qcommon::q_shared::byte;
    return cmd.offset(1 as i32 as isize) as *const libc::c_void;
}
/*
=============
RB_StretchPic
=============
*/
#[no_mangle]

pub unsafe extern "C" fn RB_StretchPic(mut data: *const libc::c_void) -> *const libc::c_void {
    let mut cmd: *const crate::tr_local_h::stretchPicCommand_t =
        0 as *const crate::tr_local_h::stretchPicCommand_t;
    let mut shader: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    let mut numVerts: i32 = 0;
    let mut numIndexes: i32 = 0;
    cmd = data as *const crate::tr_local_h::stretchPicCommand_t;
    if backEnd.projection2D as u64 == 0 {
        RB_SetGL2D();
    }
    shader = (*cmd).shader;
    if shader != crate::src::renderergl1::tr_shade::tess.shader {
        if crate::src::renderergl1::tr_shade::tess.numIndexes != 0 {
            crate::src::renderergl1::tr_shade::RB_EndSurface();
        }
        backEnd.currentEntity = &mut backEnd.entity2D;
        crate::src::renderergl1::tr_shade::RB_BeginSurface(
            shader as *mut crate::tr_local_h::shader_s,
            0 as i32,
        );
    }
    if crate::src::renderergl1::tr_shade::tess.numVertexes + 4 as i32 >= 1000 as i32
        || crate::src::renderergl1::tr_shade::tess.numIndexes + 6 as i32
            >= 6 as i32 * 1000 as i32
    {
        crate::src::renderergl1::tr_surface::RB_CheckOverflow(4 as i32, 6 as i32);
    }
    numVerts = crate::src::renderergl1::tr_shade::tess.numVertexes;
    numIndexes = crate::src::renderergl1::tr_shade::tess.numIndexes;
    crate::src::renderergl1::tr_shade::tess.numVertexes += 4 as i32;
    crate::src::renderergl1::tr_shade::tess.numIndexes += 6 as i32;
    crate::src::renderergl1::tr_shade::tess.indexes[numIndexes as usize] =
        (numVerts + 3 as i32) as crate::tr_local_h::glIndex_t;
    crate::src::renderergl1::tr_shade::tess.indexes[(numIndexes + 1 as i32) as usize] =
        (numVerts + 0 as i32) as crate::tr_local_h::glIndex_t;
    crate::src::renderergl1::tr_shade::tess.indexes[(numIndexes + 2 as i32) as usize] =
        (numVerts + 2 as i32) as crate::tr_local_h::glIndex_t;
    crate::src::renderergl1::tr_shade::tess.indexes[(numIndexes + 3 as i32) as usize] =
        (numVerts + 2 as i32) as crate::tr_local_h::glIndex_t;
    crate::src::renderergl1::tr_shade::tess.indexes[(numIndexes + 4 as i32) as usize] =
        (numVerts + 0 as i32) as crate::tr_local_h::glIndex_t;
    crate::src::renderergl1::tr_shade::tess.indexes[(numIndexes + 5 as i32) as usize] =
        (numVerts + 1 as i32) as crate::tr_local_h::glIndex_t;
    let ref mut fresh0 = *(crate::src::renderergl1::tr_shade::tess.vertexColors
        [(numVerts + 3 as i32) as usize]
        .as_mut_ptr() as *mut i32);
    *fresh0 = *(backEnd.color2D.as_mut_ptr() as *mut i32);
    let ref mut fresh1 = *(crate::src::renderergl1::tr_shade::tess.vertexColors
        [(numVerts + 2 as i32) as usize]
        .as_mut_ptr() as *mut i32);
    *fresh1 = *fresh0;
    let ref mut fresh2 = *(crate::src::renderergl1::tr_shade::tess.vertexColors
        [(numVerts + 1 as i32) as usize]
        .as_mut_ptr() as *mut i32);
    *fresh2 = *fresh1;
    *(crate::src::renderergl1::tr_shade::tess.vertexColors[numVerts as usize].as_mut_ptr()
        as *mut i32) = *fresh2;
    crate::src::renderergl1::tr_shade::tess.xyz[numVerts as usize][0 as i32 as usize] =
        (*cmd).x;
    crate::src::renderergl1::tr_shade::tess.xyz[numVerts as usize][1 as i32 as usize] =
        (*cmd).y;
    crate::src::renderergl1::tr_shade::tess.xyz[numVerts as usize][2 as i32 as usize] =
        0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::renderergl1::tr_shade::tess.texCoords[numVerts as usize]
        [0 as i32 as usize][0 as i32 as usize] = (*cmd).s1;
    crate::src::renderergl1::tr_shade::tess.texCoords[numVerts as usize]
        [0 as i32 as usize][1 as i32 as usize] = (*cmd).t1;
    crate::src::renderergl1::tr_shade::tess.xyz[(numVerts + 1 as i32) as usize]
        [0 as i32 as usize] = (*cmd).x + (*cmd).w;
    crate::src::renderergl1::tr_shade::tess.xyz[(numVerts + 1 as i32) as usize]
        [1 as i32 as usize] = (*cmd).y;
    crate::src::renderergl1::tr_shade::tess.xyz[(numVerts + 1 as i32) as usize]
        [2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::renderergl1::tr_shade::tess.texCoords[(numVerts + 1 as i32) as usize]
        [0 as i32 as usize][0 as i32 as usize] = (*cmd).s2;
    crate::src::renderergl1::tr_shade::tess.texCoords[(numVerts + 1 as i32) as usize]
        [0 as i32 as usize][1 as i32 as usize] = (*cmd).t1;
    crate::src::renderergl1::tr_shade::tess.xyz[(numVerts + 2 as i32) as usize]
        [0 as i32 as usize] = (*cmd).x + (*cmd).w;
    crate::src::renderergl1::tr_shade::tess.xyz[(numVerts + 2 as i32) as usize]
        [1 as i32 as usize] = (*cmd).y + (*cmd).h;
    crate::src::renderergl1::tr_shade::tess.xyz[(numVerts + 2 as i32) as usize]
        [2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::renderergl1::tr_shade::tess.texCoords[(numVerts + 2 as i32) as usize]
        [0 as i32 as usize][0 as i32 as usize] = (*cmd).s2;
    crate::src::renderergl1::tr_shade::tess.texCoords[(numVerts + 2 as i32) as usize]
        [0 as i32 as usize][1 as i32 as usize] = (*cmd).t2;
    crate::src::renderergl1::tr_shade::tess.xyz[(numVerts + 3 as i32) as usize]
        [0 as i32 as usize] = (*cmd).x;
    crate::src::renderergl1::tr_shade::tess.xyz[(numVerts + 3 as i32) as usize]
        [1 as i32 as usize] = (*cmd).y + (*cmd).h;
    crate::src::renderergl1::tr_shade::tess.xyz[(numVerts + 3 as i32) as usize]
        [2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    crate::src::renderergl1::tr_shade::tess.texCoords[(numVerts + 3 as i32) as usize]
        [0 as i32 as usize][0 as i32 as usize] = (*cmd).s1;
    crate::src::renderergl1::tr_shade::tess.texCoords[(numVerts + 3 as i32) as usize]
        [0 as i32 as usize][1 as i32 as usize] = (*cmd).t2;
    return cmd.offset(1 as i32 as isize) as *const libc::c_void;
}
/*
=============
RB_DrawSurfs

=============
*/
#[no_mangle]

pub unsafe extern "C" fn RB_DrawSurfs(mut data: *const libc::c_void) -> *const libc::c_void {
    let mut cmd: *const crate::tr_local_h::drawSurfsCommand_t =
        0 as *const crate::tr_local_h::drawSurfsCommand_t;
    // finish any 2D drawing if needed
    if crate::src::renderergl1::tr_shade::tess.numIndexes != 0 {
        crate::src::renderergl1::tr_shade::RB_EndSurface();
    }
    cmd = data as *const crate::tr_local_h::drawSurfsCommand_t;
    backEnd.refdef = (*cmd).refdef;
    backEnd.viewParms = (*cmd).viewParms;
    RB_RenderDrawSurfList((*cmd).drawSurfs, (*cmd).numDrawSurfs);
    return cmd.offset(1 as i32 as isize) as *const libc::c_void;
}
/*
=============
RB_DrawBuffer

=============
*/
#[no_mangle]

pub unsafe extern "C" fn RB_DrawBuffer(mut data: *const libc::c_void) -> *const libc::c_void {
    let mut cmd: *const crate::tr_local_h::drawBufferCommand_t =
        0 as *const crate::tr_local_h::drawBufferCommand_t;
    cmd = data as *const crate::tr_local_h::drawBufferCommand_t;
    crate::src::sdl::sdl_glimp::qglDrawBuffer.expect("non-null function pointer")(
        (*cmd).buffer as crate::stdlib::GLenum,
    );
    // clear screen for debugging
    if (*crate::src::renderergl1::tr_init::r_clear).integer != 0 {
        crate::src::sdl::sdl_glimp::qglClearColor.expect("non-null function pointer")(
            1 as i32 as crate::stdlib::GLclampf,
            0 as i32 as crate::stdlib::GLclampf,
            0.5f64 as crate::stdlib::GLclampf,
            1 as i32 as crate::stdlib::GLclampf,
        );
        crate::src::sdl::sdl_glimp::qglClear.expect("non-null function pointer")(
            (0x4000 as i32 | 0x100 as i32) as crate::stdlib::GLbitfield,
        );
    }
    return cmd.offset(1 as i32 as isize) as *const libc::c_void;
}
/*
===============
RB_ShowImages

Draw all the images to the screen, on top of whatever
was there.  This is used to test for texture thrashing.

Also called by RE_EndRegistration
===============
*/
#[no_mangle]

pub unsafe extern "C" fn RB_ShowImages() {
    let mut i: i32 = 0;
    let mut image: *mut crate::tr_common_h::image_t = 0 as *mut crate::tr_common_h::image_t;
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    let mut w: f32 = 0.;
    let mut h: f32 = 0.;
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    if backEnd.projection2D as u64 == 0 {
        RB_SetGL2D();
    }
    crate::src::sdl::sdl_glimp::qglClear.expect("non-null function pointer")(
        0x4000 as i32 as crate::stdlib::GLbitfield,
    );
    crate::src::sdl::sdl_glimp::qglFinish.expect("non-null function pointer")();
    start = crate::src::renderergl1::tr_main::ri
        .Milliseconds
        .expect("non-null function pointer")();
    i = 0 as i32;
    while i < crate::src::renderergl1::tr_main::tr.numImages {
        image = crate::src::renderergl1::tr_main::tr.images[i as usize];
        w = (crate::src::renderergl1::tr_init::glConfig.vidWidth / 20 as i32)
            as f32;
        h = (crate::src::renderergl1::tr_init::glConfig.vidHeight / 15 as i32)
            as f32;
        x = (i % 20 as i32) as f32 * w;
        y = (i / 20 as i32) as f32 * h;
        // show in proportional size in mode 2
        if (*crate::src::renderergl1::tr_init::r_showImages).integer == 2 as i32 {
            w *= (*image).uploadWidth as f32 / 512.0f32;
            h *= (*image).uploadHeight as f32 / 512.0f32
        }
        GL_Bind(image);
        crate::src::sdl::sdl_glimp::qglBegin.expect("non-null function pointer")(
            0x7 as i32 as crate::stdlib::GLenum,
        );
        crate::src::sdl::sdl_glimp::qglTexCoord2f.expect("non-null function pointer")(
            0 as i32 as crate::stdlib::GLfloat,
            0 as i32 as crate::stdlib::GLfloat,
        );
        crate::src::sdl::sdl_glimp::qglVertex2f.expect("non-null function pointer")(x, y);
        crate::src::sdl::sdl_glimp::qglTexCoord2f.expect("non-null function pointer")(
            1 as i32 as crate::stdlib::GLfloat,
            0 as i32 as crate::stdlib::GLfloat,
        );
        crate::src::sdl::sdl_glimp::qglVertex2f.expect("non-null function pointer")(x + w, y);
        crate::src::sdl::sdl_glimp::qglTexCoord2f.expect("non-null function pointer")(
            1 as i32 as crate::stdlib::GLfloat,
            1 as i32 as crate::stdlib::GLfloat,
        );
        crate::src::sdl::sdl_glimp::qglVertex2f.expect("non-null function pointer")(x + w, y + h);
        crate::src::sdl::sdl_glimp::qglTexCoord2f.expect("non-null function pointer")(
            0 as i32 as crate::stdlib::GLfloat,
            1 as i32 as crate::stdlib::GLfloat,
        );
        crate::src::sdl::sdl_glimp::qglVertex2f.expect("non-null function pointer")(x, y + h);
        crate::src::sdl::sdl_glimp::qglEnd.expect("non-null function pointer")();
        i += 1
    }
    crate::src::sdl::sdl_glimp::qglFinish.expect("non-null function pointer")();
    end = crate::src::renderergl1::tr_main::ri
        .Milliseconds
        .expect("non-null function pointer")();
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"%i msec to draw all images\n\x00" as *const u8 as *const libc::c_char,
        end - start,
    );
}
/*
=============
RB_ColorMask

=============
*/
#[no_mangle]

pub unsafe extern "C" fn RB_ColorMask(mut data: *const libc::c_void) -> *const libc::c_void {
    let mut cmd: *const crate::tr_local_h::colorMaskCommand_t =
        data as *const crate::tr_local_h::colorMaskCommand_t;
    crate::src::sdl::sdl_glimp::qglColorMask.expect("non-null function pointer")(
        (*cmd).rgba[0 as i32 as usize],
        (*cmd).rgba[1 as i32 as usize],
        (*cmd).rgba[2 as i32 as usize],
        (*cmd).rgba[3 as i32 as usize],
    );
    return cmd.offset(1 as i32 as isize) as *const libc::c_void;
}
/*
=============
RB_ClearDepth

=============
*/
#[no_mangle]

pub unsafe extern "C" fn RB_ClearDepth(mut data: *const libc::c_void) -> *const libc::c_void {
    let mut cmd: *const crate::tr_local_h::clearDepthCommand_t =
        data as *const crate::tr_local_h::clearDepthCommand_t;
    if crate::src::renderergl1::tr_shade::tess.numIndexes != 0 {
        crate::src::renderergl1::tr_shade::RB_EndSurface();
    }
    // texture swapping test
    if (*crate::src::renderergl1::tr_init::r_showImages).integer != 0 {
        RB_ShowImages();
    }
    crate::src::sdl::sdl_glimp::qglClear.expect("non-null function pointer")(
        0x100 as i32 as crate::stdlib::GLbitfield,
    );
    return cmd.offset(1 as i32 as isize) as *const libc::c_void;
}
/*
=============
RB_SwapBuffers

=============
*/
#[no_mangle]

pub unsafe extern "C" fn RB_SwapBuffers(mut data: *const libc::c_void) -> *const libc::c_void {
    let mut cmd: *const crate::tr_local_h::swapBuffersCommand_t =
        0 as *const crate::tr_local_h::swapBuffersCommand_t;
    // finish any 2D drawing if needed
    if crate::src::renderergl1::tr_shade::tess.numIndexes != 0 {
        crate::src::renderergl1::tr_shade::RB_EndSurface();
    }
    // texture swapping test
    if (*crate::src::renderergl1::tr_init::r_showImages).integer != 0 {
        RB_ShowImages();
    }
    cmd = data as *const crate::tr_local_h::swapBuffersCommand_t;
    // we measure overdraw by reading back the stencil buffer and
    // counting up the number of increments that have happened
    if (*crate::src::renderergl1::tr_init::r_measureOverdraw).integer != 0 {
        let mut i: i32 = 0;
        let mut sum: libc::c_long = 0 as i32 as libc::c_long;
        let mut stencilReadback: *mut u8 = 0 as *mut u8;
        stencilReadback = crate::src::renderergl1::tr_main::ri
            .Hunk_AllocateTempMemory
            .expect("non-null function pointer")(
            crate::src::renderergl1::tr_init::glConfig.vidWidth
                * crate::src::renderergl1::tr_init::glConfig.vidHeight,
        ) as *mut u8;
        crate::src::sdl::sdl_glimp::qglReadPixels.expect("non-null function pointer")(
            0 as i32,
            0 as i32,
            crate::src::renderergl1::tr_init::glConfig.vidWidth,
            crate::src::renderergl1::tr_init::glConfig.vidHeight,
            0x1901 as i32 as crate::stdlib::GLenum,
            0x1401 as i32 as crate::stdlib::GLenum,
            stencilReadback as *mut libc::c_void,
        );
        i = 0 as i32;
        while i < crate::src::renderergl1::tr_init::glConfig.vidWidth
            * crate::src::renderergl1::tr_init::glConfig.vidHeight
        {
            sum += *stencilReadback.offset(i as isize) as libc::c_long;
            i += 1
        }
        backEnd.pc.c_overDraw += sum as f32;
        crate::src::renderergl1::tr_main::ri
            .Hunk_FreeTempMemory
            .expect("non-null function pointer")(stencilReadback as *mut libc::c_void);
    }
    if crate::src::renderergl1::tr_init::glState.finishCalled as u64 == 0 {
        crate::src::sdl::sdl_glimp::qglFinish.expect("non-null function pointer")();
    }
    crate::src::sdl::sdl_glimp::GLimp_LogComment(
        b"***************** RB_SwapBuffers *****************\n\n\n\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::sdl::sdl_glimp::GLimp_EndFrame();
    backEnd.projection2D = crate::src::qcommon::q_shared::qfalse;
    return cmd.offset(1 as i32 as isize) as *const libc::c_void;
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
/*
=============================================================

RENDERER BACK END FUNCTIONS

=============================================================
*/
/*
====================
RB_ExecuteRenderCommands
====================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_ExecuteRenderCommands(mut data: *const libc::c_void) {
    let mut t1: i32 = 0;
    let mut t2: i32 = 0;
    t1 = crate::src::renderergl1::tr_main::ri
        .Milliseconds
        .expect("non-null function pointer")();
    loop {
        data = ((data as crate::stdlib::intptr_t as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as i32 as libc::c_ulong)
            & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as i32 as libc::c_ulong))
            as *mut libc::c_void;
        match *(data as *const i32) {
            1 => data = RB_SetColor(data),
            2 => data = RB_StretchPic(data),
            3 => data = RB_DrawSurfs(data),
            4 => data = RB_DrawBuffer(data),
            5 => data = RB_SwapBuffers(data),
            6 => data = crate::src::renderergl1::tr_init::RB_TakeScreenshotCmd(data),
            7 => data = crate::src::renderergl1::tr_init::RB_TakeVideoFrameCmd(data),
            8 => data = RB_ColorMask(data),
            9 => data = RB_ClearDepth(data),
            0 | _ => {
                // stop rendering
                t2 = crate::src::renderergl1::tr_main::ri
                    .Milliseconds
                    .expect("non-null function pointer")();
                backEnd.pc.msec = t2 - t1;
                return;
            }
        }
    }
}
